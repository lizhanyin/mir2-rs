//! 登录场景 - 使用 bevy_extended_ui
//!
//! 背景图片从 ChrSel.wzl 库加载：
//! - index=22: 登录背景图片
//!
//! UI 图片从 Prguse.wzl 库加载：
//! - index=60: 登录表单背景图
//! - index=61, 64, 53: 按钮按下时的图片
//!
//! 登录成功后的动画在 LoginSuccess 场景中播放

use bevy::prelude::*;
use bevy::window::Window;
use bevy_extended_ui::html::{HtmlEvent, HtmlSource, HtmlSubmit};
use bevy_extended_ui::io::HtmlAsset;
use bevy_extended_ui::registry::UiRegistry;
use bevy_extended_ui_macros::html_fn;

use crate::core::{GameConfig, GameState, SceneState};
use crate::resource::load_bevy_images_by_indices_from_path;

/// 登录场景插件
pub struct LoginScenePlugin;

impl Plugin for LoginScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SceneState::Login), setup_login)
            .add_systems(OnExit(SceneState::Login), cleanup_login)
            .add_systems(Update, resize_background.run_if(in_state(SceneState::Login)))
            .init_resource::<LoginUiImages>();
    }
}

/// 登录界面标记
#[derive(Component)]
struct LoginScreen;

/// 登录背景标记
#[derive(Component)]
struct LoginBackground {
    /// 原始图片宽度
    original_width: f32,
    /// 原始图片高度
    original_height: f32,
}

/// 登录表单背景标记
#[derive(Component)]
struct LoginFormBackground;

/// 登录场景相机
#[derive(Component)]
struct LoginCamera;

/// 登录背景图片索引 (ChrSel.wzl)
const LOGIN_BG_INDEX: usize = 22;

/// 库名称
const CHRSEL_LIBRARY: &str = "data/ChrSel";
const PRGUSE_LIBRARY: &str = "data/Prguse";

/// Prguse.wzl 中的 UI 图片索引
const UI_FORM_BG_INDEX: usize = 60;    // 登录表单背景
const UI_BTN_LOGIN_INDEX: usize = 61;  // 登录按钮按下
const UI_BTN_REG_INDEX: usize = 64;    // 注册按钮按下
const UI_BTN_EXIT_INDEX: usize = 53;   // 退出按钮按下

/// 登录 UI 图片资源
#[derive(Resource, Default)]
pub struct LoginUiImages {
    /// 登录表单背景
    pub form_bg: Option<Handle<Image>>,
    /// 登录按钮按下
    pub btn_login: Option<Handle<Image>>,
    /// 注册按钮按下
    pub btn_register: Option<Handle<Image>>,
    /// 退出按钮按下
    pub btn_exit: Option<Handle<Image>>,
}

fn setup_login(
    mut reg: ResMut<UiRegistry>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    config: Res<GameConfig>,
    mut images: ResMut<Assets<Image>>,
    mut ui_images: ResMut<LoginUiImages>,
) {
    // 加载登录页面 HTML
    let handle: Handle<HtmlAsset> = asset_server.load("ui/login.html");
    reg.add_and_use("login_page".to_string(), HtmlSource::from_handle(handle));

    tracing::debug!("加载登录背景资源: {:?}", config.resource_path);

    // ========== 加载 ChrSel.wzl 背景图片 ==========
    let lib_path = config.resource_path.join(format!("{}.wzl", CHRSEL_LIBRARY));
    tracing::debug!("ChrSel 库文件路径: {:?}, 存在: {}", lib_path, lib_path.exists());

    match load_bevy_images_by_indices_from_path(
        &config.resource_path,
        CHRSEL_LIBRARY,
        &[LOGIN_BG_INDEX],
    ) {
        Ok(bg_images) => {
            if let Some(Some(bevy_image)) = bg_images.into_iter().next() {
                let img_width = bevy_image.texture_descriptor.size.width as f32;
                let img_height = bevy_image.texture_descriptor.size.height as f32;

                tracing::debug!("背景图片尺寸: {}x{}", img_width, img_height);

                let image_handle = images.add(bevy_image);

                // 计算初始缩放以适应窗口
                let scale_x = config.screen_width / img_width;
                let scale_y = config.screen_height / img_height;
                let scale = scale_x.max(scale_y); // 使用较大的缩放比例以填满屏幕

                // 生成背景精灵
                commands.spawn((
                    Sprite::from_image(image_handle),
                    Transform::from_xyz(0.0, 0.0, -1.0)
                        .with_scale(Vec3::new(scale, scale, 1.0)),
                    LoginBackground {
                        original_width: img_width,
                        original_height: img_height,
                    },
                ));

                tracing::debug!("登录背景图片加载成功 (index={})", LOGIN_BG_INDEX);
            }
        }
        Err(e) => {
            tracing::error!("无法加载登录背景图片: {:?}", e);
        }
    }

    // ========== 加载 Prguse.wzl UI 图片 ==========
    let ui_indices = [UI_FORM_BG_INDEX, UI_BTN_LOGIN_INDEX, UI_BTN_REG_INDEX, UI_BTN_EXIT_INDEX];

    match load_bevy_images_by_indices_from_path(
        &config.resource_path,
        PRGUSE_LIBRARY,
        &ui_indices,
    ) {
        Ok(ui_image_list) => {
            tracing::debug!("加载 {} 张 UI 图片", ui_image_list.len());

            // 按顺序分配图片 handle 并生成精灵
            if let Some(Some(img)) = ui_image_list.get(0) {
                tracing::debug!(
                    "表单背景尺寸: {}x{}",
                    img.texture_descriptor.size.width,
                    img.texture_descriptor.size.height
                );
                let handle = images.add(img.clone());
                ui_images.form_bg = Some(handle.clone());

                // 生成表单背景精灵（放在屏幕中央）
                commands.spawn((
                    Sprite::from_image(handle),
                    Transform::from_xyz(0.0, 0.0, 0.0), // z=0 在主背景(z=-1)之上
                    LoginFormBackground,
                ));

                tracing::debug!("表单背景加载成功 (index={})", UI_FORM_BG_INDEX);
            }
            if let Some(Some(img)) = ui_image_list.get(1) {
                ui_images.btn_login = Some(images.add(img.clone()));
                tracing::debug!("登录按钮图片加载成功 (index={})", UI_BTN_LOGIN_INDEX);
            }
            if let Some(Some(img)) = ui_image_list.get(2) {
                ui_images.btn_register = Some(images.add(img.clone()));
                tracing::debug!("注册按钮图片加载成功 (index={})", UI_BTN_REG_INDEX);
            }
            if let Some(Some(img)) = ui_image_list.get(3) {
                ui_images.btn_exit = Some(images.add(img.clone()));
                tracing::debug!("退出按钮图片加载成功 (index={})", UI_BTN_EXIT_INDEX);
            }
        }
        Err(e) => {
            tracing::error!("无法加载 Prguse UI 图片: {:?}", e);
        }
    }

    // 标记登录界面已加载
    commands.spawn(LoginScreen);

    // 生成 2D 相机来渲染背景精灵（order=-1 确保在 UI 相机之前渲染）
    commands.spawn((
        Camera2d,
        Camera {
            order: -1, // 确保在 UI 相机之前渲染
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        LoginCamera,
    ));

    tracing::info!("进入登录场景");
}

fn cleanup_login(
    mut commands: Commands,
    query: Query<Entity, With<LoginScreen>>,
    bg_query: Query<Entity, With<LoginBackground>>,
    form_bg_query: Query<Entity, With<LoginFormBackground>>,
    camera_query: Query<Entity, With<LoginCamera>>,
    mut reg: ResMut<UiRegistry>,
) {
    // 移除 UI
    reg.remove("login_page");

    // 移除登录界面标记实体
    for entity in query.iter() {
        commands.entity(entity).try_despawn();
    }

    // 移除背景精灵
    for entity in bg_query.iter() {
        commands.entity(entity).try_despawn();
    }

    // 移除表单背景精灵
    for entity in form_bg_query.iter() {
        commands.entity(entity).try_despawn();
    }

    // 移除相机
    for entity in camera_query.iter() {
        commands.entity(entity).try_despawn();
    }

    tracing::info!("退出登录场景");
}

/// 窗口大小变化时调整背景图大小
fn resize_background(
    window_query: Query<&Window, Changed<Window>>,
    mut bg_query: Query<(&LoginBackground, &mut Transform)>,
) {
    // 检查窗口大小是否变化
    if let Ok(window) = window_query.single() {
        let window_width = window.width();
        let window_height = window.height();

        for (bg, mut transform) in bg_query.iter_mut() {
            // 计算缩放比例以填满窗口
            let scale_x = window_width / bg.original_width;
            let scale_y = window_height / bg.original_height;
            let scale = scale_x.max(scale_y); // 使用较大的缩放比例以填满屏幕

            transform.scale = Vec3::new(scale, scale, 1.0);
            tracing::debug!("调整背景大小: 窗口={}x{}, 缩放={}", window_width, window_height, scale);
        }
    }
}

/// 处理登录表单提交
#[html_fn("on_login")]
fn on_login(
    In(event): In<HtmlSubmit>,
    mut next_state: ResMut<NextState<SceneState>>,
    mut game_state: ResMut<GameState>,
) {
    let username = event.data.get("username").cloned().unwrap_or_default();
    let password = event.data.get("password").cloned().unwrap_or_default();

    tracing::info!("登录请求: username='{}', password='{}'", username, password);

    // 模拟登录验证
    if !username.is_empty() && !password.is_empty() {
        game_state.is_logged_in = true;
        game_state.player_name = username.clone();
        game_state.player_id = 1001;

        tracing::info!("登录成功，玩家: {}", username);
        // 先切换到登录成功动画场景
        next_state.set(SceneState::LoginSuccess);
    } else {
        tracing::warn!("登录失败：用户名或密码为空");
    }
}

/// 处理注册链接点击
#[html_fn("on_register")]
fn on_register(In(_event): In<HtmlEvent>) {
    tracing::info!("点击注册账号");
    // TODO: 实现注册功能
}

/// 处理忘记密码链接点击
#[html_fn("on_forgot")]
fn on_forgot(In(_event): In<HtmlEvent>) {
    tracing::info!("点击忘记密码");
    // TODO: 实现找回密码功能
}

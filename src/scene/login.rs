//! 登录场景 - 使用 bevy_extended_ui
//!
//! 背景图片从 ChrSel.wzl 库加载：
//! - index=22: 登录背景图片 -> 缓存 key: ChrSel_22.png
//!
//! UI 图片从 Prguse.wzl 库加载：
//! - index=60: 登录表单背景图 -> 缓存 key: Prguse_60.png
//! - index=62: 登录按钮按下 -> 缓存 key: Prguse_62.png
//! - index=61: 注册按钮按下 -> 缓存 key: Prguse_61.png
//! - index=53: 修改密码按钮按下 -> 缓存 key: Prguse_53.png
//! - index=64: 退出按钮按下 -> 缓存 key: Prguse_64.png
//!
//! 图片通过 CSS background-image 使用，例如：
//! background-image: url("ChrSel_22.png");
//!
//! 登录成功后的动画在 LoginSuccess 场景中播放

use bevy::prelude::*;
use bevy_extended_ui::ImageCache;
use bevy_extended_ui::html::{HtmlEvent, HtmlSource, HtmlSubmit};
use bevy_extended_ui::io::HtmlAsset;
use bevy_extended_ui::registry::UiRegistry;
use bevy_extended_ui_macros::html_fn;

use crate::core::{GameConfig, GameState, SceneState};
use crate::resource::{load_image_to_cache, load_images_to_cache};

/// 登录场景插件
pub struct LoginScenePlugin;

impl Plugin for LoginScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SceneState::Login), setup_login)
            .add_systems(OnExit(SceneState::Login), cleanup_login)
            .init_resource::<LoginUiImages>();
    }
}

/// 登录界面标记
#[derive(Component)]
struct LoginScreen;

/// 登录背景图片索引 (ChrSel.wzl)
const LOGIN_BG_INDEX: usize = 22;

/// 库名称
const CHRSEL_LIBRARY: &str = "data/ChrSel";
const PRGUSE_LIBRARY: &str = "data/Prguse";

/// Prguse.wzl 中的 UI 图片索引
const UI_FORM_BG_INDEX: usize = 60; // 登录表单背景
const UI_BTN_LOGIN_INDEX: usize = 62; // 登录按钮按下
const UI_BTN_REG_INDEX: usize = 61; // 注册按钮按下
const UI_BTN_CHANGE_PWD_INDEX: usize = 53; // 修改密码按钮按下
const UI_BTN_EXIT_INDEX: usize = 64; // 退出按钮按下

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
    mut image_cache: ResMut<ImageCache>,
) {
    // 加载登录页面 HTML
    let handle: Handle<HtmlAsset> = asset_server.load("ui/login.html");
    reg.add_and_use("login_page".to_string(), HtmlSource::from_handle(handle));

    tracing::debug!("加载登录场景资源: {:?}", config.resource_path);

    // ========== 加载 ChrSel.wzl 背景图片到 ImageCache ==========
    // CSS 可通过 background-image: url("ChrSel_22.png") 使用
    let lib_path = config.resource_path.join(format!("{}.wzl", CHRSEL_LIBRARY));
    tracing::debug!(
        "ChrSel 库文件路径: {:?}, 存在: {}",
        lib_path,
        lib_path.exists()
    );

    match load_image_to_cache(
        &config.resource_path,
        CHRSEL_LIBRARY,
        LOGIN_BG_INDEX,
        &mut image_cache,
        &mut images,
    ) {
        Ok(_handle) => {
            tracing::debug!(
                "背景图片加载到缓存 (index={}), key: ChrSel_{}.png",
                LOGIN_BG_INDEX,
                LOGIN_BG_INDEX
            );
        }
        Err(e) => {
            tracing::error!("无法加载登录背景图片: {:?}", e);
        }
    }

    // ========== 加载 Prguse.wzl UI 图片到 ImageCache ==========
    let ui_indices = [
        UI_FORM_BG_INDEX,
        UI_BTN_LOGIN_INDEX,
        UI_BTN_REG_INDEX,
        UI_BTN_EXIT_INDEX,
    ];

    match load_images_to_cache(
        &config.resource_path,
        PRGUSE_LIBRARY,
        &ui_indices,
        &mut image_cache,
        &mut images,
    ) {
        Ok(ui_handle_list) => {
            tracing::debug!("加载 {} 张 UI 图片到缓存", ui_handle_list.len());

            // 保存 handle 供后续使用（如按钮交互）
            if let Some(Some(handle)) = ui_handle_list.first() {
                ui_images.form_bg = Some(handle.clone());
                tracing::debug!(
                    "表单背景缓存成功 (index={}), key: Prguse_{}.png",
                    UI_FORM_BG_INDEX,
                    UI_FORM_BG_INDEX
                );
            }
            if let Some(Some(handle)) = ui_handle_list.get(1) {
                ui_images.btn_login = Some(handle.clone());
                tracing::debug!(
                    "登录按钮图片缓存成功 (index={}), key: Prguse_{}.png",
                    UI_BTN_LOGIN_INDEX,
                    UI_BTN_LOGIN_INDEX
                );
            }
            if let Some(Some(handle)) = ui_handle_list.get(2) {
                ui_images.btn_register = Some(handle.clone());
                tracing::debug!(
                    "注册按钮图片缓存成功 (index={}), key: Prguse_{}.png",
                    UI_BTN_REG_INDEX,
                    UI_BTN_REG_INDEX
                );
            }
            if let Some(Some(handle)) = ui_handle_list.get(3) {
                ui_images.btn_exit = Some(handle.clone());
                tracing::debug!(
                    "退出按钮图片缓存成功 (index={}), key: Prguse_{}.png",
                    UI_BTN_EXIT_INDEX,
                    UI_BTN_EXIT_INDEX
                );
            }
        }
        Err(e) => {
            tracing::error!("无法加载 Prguse UI 图片: {:?}", e);
        }
    }

    // 标记登录界面已加载
    commands.spawn(LoginScreen);

    tracing::info!("进入登录场景，ImageCache 大小: {}", image_cache.map.len());
}

fn cleanup_login(
    mut commands: Commands,
    query: Query<Entity, With<LoginScreen>>,
    mut reg: ResMut<UiRegistry>,
) {
    // 移除 UI
    reg.remove("login_page");

    // 移除登录界面标记实体
    for entity in query.iter() {
        commands.entity(entity).try_despawn();
    }

    tracing::info!("退出登录场景");
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

// ========== 关闭按钮事件 ==========

/// 关闭按钮点击事件 - 切换图片并退出
#[html_fn("on_close")]
fn on_close(
    In(event): In<HtmlEvent>,
    mut query: Query<&mut bevy_extended_ui::styles::CssClass>,
    mut app_exit: bevy::ecs::message::MessageWriter<AppExit>,
) {
    let target = event.target();
    tracing::info!("关闭按钮点击事件触发, target entity: {:?}", target);

    // 添加 active class 切换图片
    match query.get_mut(target) {
        Ok(mut css_class) => {
            tracing::info!("当前 class 列表: {:?}", css_class.0);
            crate::ui::effects::add_class(&mut css_class, crate::ui::effects::CLASS_ACTIVE);
            tracing::info!("添加 active class 后: {:?}", css_class.0);
        }
        Err(e) => {
            tracing::warn!("无法获取 CssClass 组件: {:?}", e);
        }
    }

    tracing::info!("点击关闭按钮，退出游戏");
    app_exit.write(AppExit::Success);
}

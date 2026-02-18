//! 登录场景 - 使用 bevy_extended_ui
//!
//! 背景图片从 ChrSel.wzl 库加载：
//! - index=22: 登录背景图片
//!
//! 登录成功后的动画在 LoginSuccess 场景中播放

use bevy::prelude::*;
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
            .add_systems(OnExit(SceneState::Login), cleanup_login);
    }
}

/// 登录界面标记
#[derive(Component)]
struct LoginScreen;

/// 登录背景标记
#[derive(Component)]
struct LoginBackground;

/// 登录背景图片索引
const LOGIN_BG_INDEX: usize = 22;

/// 库名称
const LIBRARY_NAME: &str = "data/ChrSel";

fn setup_login(
    mut reg: ResMut<UiRegistry>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    config: Res<GameConfig>,
    mut images: ResMut<Assets<Image>>,
) {
    // 加载登录页面 HTML
    let handle: Handle<HtmlAsset> = asset_server.load("ui/login.html");
    reg.add_and_use("login_page".to_string(), HtmlSource::from_handle(handle));

    tracing::info!("加载登录背景资源: {:?}", config.resource_path);

    // 加载背景图片 (index=22)
    if let Ok(bg_images) = load_bevy_images_by_indices_from_path(
        &config.resource_path,
        LIBRARY_NAME,
        &[LOGIN_BG_INDEX],
    ) {
        if let Some(Some(bevy_image)) = bg_images.into_iter().next() {
            let image_handle = images.add(bevy_image);

            // 生成背景精灵
            commands.spawn((
                Sprite::from_image(image_handle),
                Transform::from_xyz(0.0, 0.0, -1.0),
                LoginBackground,
            ));

            tracing::info!("登录背景图片加载成功 (index={})", LOGIN_BG_INDEX);
        }
    } else {
        tracing::warn!("无法加载登录背景图片 (index={})", LOGIN_BG_INDEX);
    }

    // 标记登录界面已加载
    commands.spawn(LoginScreen);

    tracing::info!("进入登录场景");
}

fn cleanup_login(
    mut commands: Commands,
    query: Query<Entity, With<LoginScreen>>,
    bg_query: Query<Entity, With<LoginBackground>>,
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

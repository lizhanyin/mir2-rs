//! Mir2-RS - 传奇2 客户端入口
//!
//! 基于 Bevy 游戏引擎的《传奇2》客户端

use bevy::prelude::*;
use mir2_rs::core::{ConfigPlugin, GameStatePlugin};
use mir2_rs::game::GamePlugin;
use mir2_rs::render::RenderPlugin;
use mir2_rs::resource::ResourcePlugin;
use mir2_rs::scene::ScenePlugin;
use mir2_rs::ui::UiPlugin;

fn main() {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("mir2_rs=debug".parse().unwrap()),
        )
        .init();

    tracing::info!("传奇世界 - Mir2-RS 启动中...");

    // 创建 Bevy 应用
    App::new()
        // 添加默认插件
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "传奇世界 - Mir2-RS".to_string(),
                resolution: (1024, 768).into(),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        // 添加自定义插件
        .add_plugins((
            ConfigPlugin,
            GameStatePlugin,
            ResourcePlugin,
            ScenePlugin,
            GamePlugin,
            RenderPlugin,
            UiPlugin,
        ))
        // 运行应用
        .run();
}

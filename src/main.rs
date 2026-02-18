  //! Mir2-RS - 传奇2 客户端入口
//!
//! 基于 Bevy 游戏引擎的《传奇2》客户端

use bevy::{log::LogPlugin, prelude::*};
use bevy_extended_ui::{ExtendedUiConfiguration, ExtendedUiPlugin};
use mir2_rs::core::{GameConfig, GameStatePlugin};
use mir2_rs::game::GamePlugin;
use mir2_rs::render::RenderPlugin;
use mir2_rs::resource::ResourcePlugin;
use mir2_rs::scene::ScenePlugin;
use mir2_rs::ui::UiPlugin;

fn main() {
    // 解析配置（从命令行参数和环境变量）
    let config = GameConfig::from_args();

    // 显示资源目录
    println!("========================================");
    println!("热血传奇 - Mir2-RS");
    println!("========================================");
    println!("游戏资源目录: {}", config.resource_path.display());
    println!("窗口大小: {}x{}", config.screen_width, config.screen_height);
    println!("全屏模式: {}", config.fullscreen);
    println!("========================================");

    // 创建 Bevy 应用
    App::new()
        // 添加默认插件，配置日志级别
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "热血传奇 - Mir2-RS".to_string(),
                resolution: (config.screen_width as u32, config.screen_height as u32).into(),
                resizable: true,
                ..default()
            }),
            ..default()
        }).set(LogPlugin {
            level: bevy::log::Level::DEBUG,
            filter: "mir2_rs=debug,bevy_extended_ui=info,wgpu=error,naga=error".to_string(),
            ..default()
        }))
        // 配置 bevy_extended_ui
        .insert_resource(ExtendedUiConfiguration {
            assets_path: "assets/ui/".to_string(),
            language_path: "assets/lang/".to_string(),
            ..default()
        })
        // 添加 bevy_extended_ui 插件
        .add_plugins(ExtendedUiPlugin)
        // 添加自定义插件（直接使用上面解析的配置）
        .insert_resource(config)
        .add_plugins((
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

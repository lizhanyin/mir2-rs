//! Mir2-RS - 传奇2 客户端入口
//!
//! 基于 Bevy 游戏引擎的《传奇2》客户端

use bevy::asset::{AssetMetaCheck, AssetPlugin};
use bevy::{log::LogPlugin, prelude::*};
use bevy_extended_ui::{ExtendedUiConfiguration, ExtendedUiPlugin};
use mir2_rs::core::{GameConfig, GameStatePlugin};
use mir2_rs::game::GamePlugin;
use mir2_rs::render::RenderPlugin;
use mir2_rs::resource::ResourcePlugin;
use mir2_rs::scene::ScenePlugin;
use mir2_rs::ui::UiPlugin;
use std::path::PathBuf;

fn main() {
    // 确保 assets 目录可访问（Debug 模式下切换工作目录）
    let assets_path = get_assets_path();

    // 解析配置（从命令行参数和环境变量）
    let config = GameConfig::from_args();

    // 构建 bevy_extended_ui 的绝对路径
    let ui_assets_path = assets_path.join("ui").to_string_lossy().to_string();
    let lang_assets_path = assets_path.join("lang").to_string_lossy().to_string();
    // bevy asset 路径
    let bevy_asset_path = assets_path.to_string_lossy().to_string();

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
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "热血传奇 - Mir2-RS".to_string(),
                        resolution: (config.screen_width as u32, config.screen_height as u32)
                            .into(),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(LogPlugin {
                    level: bevy::log::Level::DEBUG,
                    filter: "mir2_rs=debug,bevy_extended_ui=info,wgpu=error,naga=error".to_string(),
                    ..default()
                })
                .set(AssetPlugin {
                    file_path: bevy_asset_path,
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                }),
        )
        // 配置 bevy_extended_ui（使用绝对路径）
        .insert_resource(ExtendedUiConfiguration {
            assets_path: ui_assets_path,
            language_path: lang_assets_path,
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

/// 获取 assets 目录的绝对路径
/// Debug 模式下可能需要从项目根目录查找
fn get_assets_path() -> PathBuf {
    // 首先检查当前目录
    let current_dir = std::env::current_dir().unwrap_or_default();
    let assets_in_current = current_dir.join("assets");
    if assets_in_current.exists() {
        return assets_in_current;
    }

    // 检查可执行文件所在目录的上级目录（target/debug -> 项目根）
    if let Ok(exe_path) = std::env::current_exe()
        && let Some(exe_dir) = exe_path.parent()
    {
        // target/debug -> target -> 项目根
        if let Some(target_dir) = exe_dir.parent()
            && let Some(project_root) = target_dir.parent()
        {
            let assets_in_root = project_root.join("assets");
            if assets_in_root.exists() {
                // 切换工作目录到项目根
                let _ = std::env::set_current_dir(project_root);
                return assets_in_root;
            }
        }
    }

    // 默认返回当前目录下的 assets
    PathBuf::from("assets")
}

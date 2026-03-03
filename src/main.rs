//! Mir2-RS - 传奇2 客户端入口
//!
//! 基于 Bevy 游戏引擎的《传奇2》客户端

use bevy::asset::{AssetMetaCheck, AssetPlugin};
use bevy::prelude::*;
use bevy::window::{EnabledButtons, PrimaryWindow};
use bevy_extended_ui::{ExtendedUiConfiguration, ExtendedUiPlugin};
use mir2_rs::core::{GameConfig, GameStatePlugin};
use mir2_rs::game::GamePlugin;
use mir2_rs::render::RenderPlugin;
use mir2_rs::resource::ResourcePlugin;
use mir2_rs::scene::ScenePlugin;
use mir2_rs::ui::UiPlugin;
use std::path::PathBuf;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

/// 目标宽高比 4:3
const ASPECT_RATIO: f32 = 4.0 / 3.0;

fn main() {
    // 初始化日志（输出到文件和控制台）
    init_logger();

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
    tracing::info!("========================================");
    tracing::info!("热血传奇");
    tracing::info!("========================================");
    tracing::info!("游戏资源目录: {}", config.resource_path.display());
    tracing::info!("窗口大小: {}x{}", config.screen_width, config.screen_height);
    tracing::info!("全屏模式: {}", config.fullscreen);
    tracing::info!("========================================");

    // 创建 Bevy 应用
    App::new()
        // 添加默认插件（不包含 LogPlugin，使用自定义日志）
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "mir2".to_string(),
                        resolution: (config.screen_width as u32, config.screen_height as u32)
                            .into(),
                        resizable: true,
                        resize_constraints: WindowResizeConstraints {
                            min_width: 800.0,
                            min_height: 600.0,
                            ..default()
                        },
                        // 禁用最大化按钮，保持 4:3 比例
                        enabled_buttons: EnabledButtons {
                            minimize: true,
                            maximize: false,
                            close: true,
                        },
                        ..default()
                    }),
                    ..default()
                })
                // .set(LogPlugin {
                //     level: bevy::log::Level::DEBUG,
                //     filter: "mir2_rs=debug,bevy_extended_ui=info,wgpu=error,naga=error".to_string(),
                //     ..default()
                // })
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
        // 添加窗口比例保持系统
        .add_systems(Update, maintain_aspect_ratio)
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

/// 初始化日志系统（输出到文件和控制台）
fn init_logger() {
    use tracing_appender::{non_blocking, rolling};

    // 获取日志目录
    let log_dir = get_log_dir();

    // 创建日志文件写入器（按日期滚动）
    let file_appender = rolling::daily(&log_dir, "mir2-rs.log");
    let (non_blocking_file, _guard) = non_blocking(file_appender);

    // 配置日志过滤器
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        EnvFilter::new("mir2_rs=debug,bevy_extended_ui=info,wgpu=error,naga=error")
    });

    // 初始化订阅器（同时输出到控制台和文件）
    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer().with_writer(std::io::stdout))
        .with(
            tracing_subscriber::fmt::layer()
                .with_ansi(false)
                .with_writer(non_blocking_file),
        )
        .init();

    // tracing::info!("日志目录: {}", log_dir.display());
}

/// 获取日志目录路径
fn get_log_dir() -> PathBuf {
    // 优先使用可执行文件所在目录的 logs 子目录
    if let Ok(exe_path) = std::env::current_exe()
        && let Some(exe_dir) = exe_path.parent()
    {
        let logs_dir = exe_dir.join("logs");
        if std::fs::create_dir_all(&logs_dir).is_err() {
            return exe_dir.to_path_buf();
        }
        return logs_dir;
    }

    // 默认使用当前目录
    PathBuf::from(".")
}

/// 保持窗口宽高比为 4:3
/// 使用 changed 查询过滤器只在窗口大小变化时运行
fn maintain_aspect_ratio(mut windows: Query<&mut Window, (With<PrimaryWindow>, Changed<Window>)>) {
    for mut window in windows.iter_mut() {
        let width = window.width();
        let height = window.height();

        // 计算当前宽高比
        let current_aspect = width / height;

        // 如果宽高比不是 4:3，则调整
        if (current_aspect - ASPECT_RATIO).abs() > 0.01 {
            // 根据宽度计算高度（保持 4:3）
            let new_height = width / ASPECT_RATIO;

            // 设置新的分辨率
            window.resolution.set(width, new_height);

            tracing::debug!(
                "调整窗口比例 4:3: {:.0}x{:.0} -> {:.0}x{:.0}",
                width,
                height,
                width,
                new_height
            );
        }
    }
}

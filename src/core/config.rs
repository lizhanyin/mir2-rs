//! 游戏配置模块

use bevy::prelude::*;
use std::path::PathBuf;

/// 默认游戏资源目录
const DEFAULT_RESOURCE_PATH: &str = r"E:\Game\Online\Legend of mir";

/// 游戏配置资源
#[derive(Resource, Clone)]
pub struct GameConfig {
    /// 游戏资源目录
    pub resource_path: PathBuf,
    /// 屏幕宽度
    pub screen_width: f32,
    /// 屏幕高度
    pub screen_height: f32,
    /// 是否全屏
    pub fullscreen: bool,
    /// 帧率限制
    pub fps_limit: Option<u32>,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            resource_path: PathBuf::from(DEFAULT_RESOURCE_PATH),
            screen_width: 800.0,
            screen_height: 600.0,
            fullscreen: false,
            fps_limit: Some(60),
        }
    }
}

impl GameConfig {
    /// 创建新的配置
    pub fn new(resource_path: PathBuf) -> Self {
        Self {
            resource_path,
            ..Default::default()
        }
    }

    /// 从命令行参数和环境变量创建配置
    /// 优先级: 命令行参数 > 环境变量 > 默认值
    pub fn from_args() -> Self {
        let mut config = Self::default();

        // 1. 从环境变量读取
        if let Ok(path) = std::env::var("MIR2_RESOURCE_PATH") {
            config.resource_path = PathBuf::from(path);
        }

        // 2. 从命令行参数读取 (覆盖环境变量)
        let args: Vec<String> = std::env::args().collect();
        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "--resource-path" | "-r" => {
                    if i + 1 < args.len() {
                        config.resource_path = PathBuf::from(&args[i + 1]);
                        i += 1;
                    }
                }
                "--fullscreen" | "-f" => {
                    config.fullscreen = true;
                }
                "--width" | "-w" => {
                    if i + 1 < args.len() {
                        if let Ok(width) = args[i + 1].parse::<f32>() {
                            config.screen_width = width;
                            i += 1;
                        }
                    }
                }
                "--height" | "-h" => {
                    if i + 1 < args.len() {
                        if let Ok(height) = args[i + 1].parse::<f32>() {
                            config.screen_height = height;
                            i += 1;
                        }
                    }
                }
                "--help" => {
                    print_help();
                    std::process::exit(0);
                }
                _ => {}
            }
            i += 1;
        }

        config
    }

    /// 获取库文件路径
    pub fn get_library_path(&self, name: &str) -> PathBuf {
        self.resource_path.join(name)
    }

    /// 获取地图文件路径
    pub fn get_map_path(&self, map_name: &str) -> PathBuf {
        self.resource_path.join("Map").join(map_name)
    }
}

/// 打印帮助信息
fn print_help() {
    println!("热血传奇 - Mir2-RS");
    println!();
    println!("用法: mir2-rs [选项]");
    println!();
    println!("选项:");
    println!("  -r, --resource-path <PATH>  游戏资源目录路径");
    println!("                              (也可通过环境变量 MIR2_RESOURCE_PATH 设置)");
    println!("  -f, --fullscreen            全屏模式");
    println!("  -w, --width <WIDTH>         窗口宽度");
    println!("  -h, --height <HEIGHT>       窗口高度");
    println!("      --help                  显示帮助信息");
    println!();
    println!("示例:");
    println!("  mir2-rs -r \"E:\\Game\\Online\\Legend of mir\"");
    println!("  mir2-rs --resource-path \"/path/to/resources\" --fullscreen");
    println!("  MIR2_RESOURCE_PATH=/path/to/resources mir2-rs");
}

/// 配置插件
pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameConfig::from_args());
    }
}

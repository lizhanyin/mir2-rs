//! 游戏配置模块

use bevy::prelude::*;
use std::path::PathBuf;

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
            resource_path: PathBuf::from(r"E:\Game\Online\Legend of mir"),
            screen_width: 1024.0,
            screen_height: 768.0,
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

    /// 获取库文件路径
    pub fn get_library_path(&self, name: &str) -> PathBuf {
        self.resource_path.join(name)
    }

    /// 获取地图文件路径
    pub fn get_map_path(&self, map_name: &str) -> PathBuf {
        self.resource_path.join("Map").join(map_name)
    }
}

/// 配置插件
pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameConfig::default());
    }
}

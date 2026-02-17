//! 游戏状态模块

use bevy::prelude::*;

/// 场景类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, States, Default)]
pub enum SceneState {
    #[default]
    Loading,
    Login,
    SelectChar,
    Game,
}

/// 游戏状态资源
#[derive(Resource, Default)]
pub struct GameState {
    /// 当前玩家名称
    pub player_name: String,
    /// 是否已登录
    pub is_logged_in: bool,
    /// 当前地图名称
    pub current_map: String,
    /// 玩家ID
    pub player_id: u32,
}

/// 连接状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ConnectionState {
    #[default]
    Disconnected,
    Connecting,
    Connected,
    Login,
    SelectChar,
    Playing,
}

/// 游戏状态插件
pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SceneState>()
            .init_resource::<GameState>();
    }
}

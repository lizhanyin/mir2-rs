//! 游戏逻辑模块

pub mod actor;
pub mod map;

use bevy::prelude::*;

/// 游戏逻辑插件
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((actor::ActorPlugin, map::MapPlugin));
    }
}

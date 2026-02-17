//! 场景系统模块

pub mod game;
pub mod loading;
pub mod login;
pub mod select_char;

use bevy::prelude::*;

/// 场景插件
pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((loading::LoadingScenePlugin, login::LoginScenePlugin,
                        select_char::SelectCharScenePlugin, game::GameScenePlugin));
    }
}

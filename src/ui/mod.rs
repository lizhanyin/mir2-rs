//! UI 系统模块

pub mod effects;

use bevy::prelude::*;
pub use effects::*;

/// UI 插件
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiEffectsPlugin);
    }
}

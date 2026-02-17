//! 资源管理模块

pub mod cache;
pub mod loader;

use bevy::prelude::*;

pub use cache::*;
pub use loader::*;

/// 资源管理插件
pub struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ResourceCache::new())
            .init_resource::<ResourceLoader>();
    }
}

//! 资源管理模块

pub mod bevy_image;
pub mod cache;
pub mod formats;
pub mod image;
pub mod loader;

use bevy::prelude::*;

pub use bevy_image::*;
pub use cache::*;
pub use formats::{LibraryInfo, LibraryLoader, LibraryType};
pub use loader::*;

/// 资源管理插件
pub struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ResourceCache::new())
            .init_resource::<ResourceLoader>();
    }
}

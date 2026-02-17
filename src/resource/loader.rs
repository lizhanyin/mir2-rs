//! 资源加载器

use crate::core::GameConfig;
use crate::formats::LibraryLoader;
use bevy::prelude::*;

/// 资源加载器
#[derive(Resource, Default)]
pub struct ResourceLoader {
    /// 已加载的库路径
    loaded_libraries: Vec<String>,
}

impl ResourceLoader {
    /// 加载库文件
    pub fn load_library(
        &mut self,
        config: &GameConfig,
        library_name: &str,
    ) -> Option<(String, LibraryLoader)> {
        let path = config.get_library_path(library_name);

        // 尝试不同的扩展名
        let extensions = [".wzl", ".Lib", ".wil"];

        for ext in &extensions {
            let full_path = path.with_extension(ext.trim_start_matches('.'));
            if full_path.exists() {
                if let Ok((info, loader)) = LibraryLoader::load(&full_path) {
                    self.loaded_libraries.push(info.base_path.clone());
                    tracing::info!("加载库文件: {} ({} 张图像)", library_name, info.image_count);
                    return Some((info.base_path, loader));
                }
            }
        }

        tracing::warn!("无法加载库文件: {}", library_name);
        None
    }

    /// 获取已加载的库列表
    pub fn get_loaded_libraries(&self) -> &[String] {
        &self.loaded_libraries
    }
}

/// 资源加载系统
pub fn resource_loading_system(
    _loader: ResMut<ResourceLoader>,
    _config: Res<GameConfig>,
) {
    // 这里可以预加载常用资源
    // 例如: UI图标、常用特效等
}

/// 启动时加载核心资源
pub fn load_core_resources(
    mut loader: ResMut<ResourceLoader>,
    config: Res<GameConfig>,
) {
    // 加载核心UI资源
    let core_libraries = [
        "FrmMain",
        "Prguse",
        "Prguse2",
    ];

    for lib_name in &core_libraries {
        loader.load_library(&config, lib_name);
    }

    tracing::info!("核心资源加载完成");
}

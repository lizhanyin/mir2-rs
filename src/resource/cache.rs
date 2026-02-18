//! 资源缓存

use super::formats::{LibraryInfo, LibraryLoader};
use bevy::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// 资源缓存
#[derive(Resource)]
pub struct ResourceCache {
    /// 已加载的库文件
    libraries: HashMap<String, Arc<Mutex<LibraryLoader>>>,
    /// 最大缓存数量
    max_cache_size: usize,
}

impl ResourceCache {
    /// 创建新的资源缓存
    pub fn new() -> Self {
        Self {
            libraries: HashMap::new(),
            max_cache_size: 50,
        }
    }

    /// 设置最大缓存大小
    pub fn with_max_size(mut self, size: usize) -> Self {
        self.max_cache_size = size;
        self
    }

    /// 获取或加载库文件
    pub fn get_or_load(&mut self, path: &str) -> Option<Arc<Mutex<LibraryLoader>>> {
        if let Some(loader) = self.libraries.get(path) {
            return Some(loader.clone());
        }

        // 检查缓存大小
        if self.libraries.len() >= self.max_cache_size {
            // 简单策略：移除第一个
            if let Some(first_key) = self.libraries.keys().next().cloned() {
                self.libraries.remove(&first_key);
            }
        }

        // 加载新库
        let lib_path = std::path::Path::new(path);
        if let Ok((_info, loader)) = LibraryLoader::load(lib_path) {
            let loader = Arc::new(Mutex::new(loader));
            self.libraries.insert(path.to_string(), loader.clone());
            Some(loader)
        } else {
            None
        }
    }

    /// 预加载库文件
    pub fn preload(&mut self, paths: &[&str]) -> Vec<LibraryInfo> {
        let mut infos = Vec::new();

        for path in paths {
            let lib_path = std::path::Path::new(path);
            if let Ok((info, loader)) = LibraryLoader::load(lib_path) {
                let loader = Arc::new(Mutex::new(loader));
                self.libraries.insert(path.to_string(), loader);
                infos.push(info);
            }
        }

        infos
    }

    /// 清除缓存
    pub fn clear(&mut self) {
        self.libraries.clear();
    }

    /// 获取缓存大小
    pub fn size(&self) -> usize {
        self.libraries.len()
    }
}

impl Default for ResourceCache {
    fn default() -> Self {
        Self::new()
    }
}

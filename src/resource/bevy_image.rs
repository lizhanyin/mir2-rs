//! Bevy 图像转换工具
//!
//! 将 wzl/lib 库中的图像转换为 Bevy 可用的 Image 资源

use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy_extended_ui::ImageCache;
use image::RgbaImage;
use std::path::Path;

use super::formats::{LibraryLoader, load_images_by_indices_from_path, load_images_from_path};
use crate::error::Result;

/// 将 RgbaImage 转换为 Bevy Image
pub fn rgba_image_to_bevy(rgba_image: &RgbaImage) -> Option<Image> {
    let width = rgba_image.width();
    let height = rgba_image.height();

    if width == 0 || height == 0 {
        return None;
    }

    // 创建 Bevy Image
    let mut image = Image::new_fill(
        Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        rgba_image.as_raw(),
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    );

    // 设置纹理描述符
    image.texture_descriptor.usage = bevy::render::render_resource::TextureUsages::TEXTURE_BINDING
        | bevy::render::render_resource::TextureUsages::COPY_DST
        | bevy::render::render_resource::TextureUsages::RENDER_ATTACHMENT;

    Some(image)
}

/// 从库加载器获取指定索引的图像并转换为 Bevy Image
pub fn load_image_from_library(loader: &mut LibraryLoader, index: usize) -> Result<Option<Image>> {
    let preview = loader.get_preview(index)?;

    if let Some(rgba_image) = preview {
        Ok(rgba_image_to_bevy(&rgba_image))
    } else {
        Ok(None)
    }
}

/// 批量加载图像并转换为 Bevy Image
pub fn load_images_from_library(
    loader: &mut LibraryLoader,
    indices: &[usize],
) -> Result<Vec<Option<Image>>> {
    let mut images = Vec::with_capacity(indices.len());

    for &index in indices {
        let image = load_image_from_library(loader, index)?;
        images.push(image);
    }

    Ok(images)
}

/// 从资源路径加载指定范围的图像并转换为 Bevy Image
///
/// # 参数
/// - `resource_path`: 资源根目录
/// - `library_name`: 库文件名（不含扩展名）
/// - `start`: 起始索引（包含）
/// - `end`: 结束索引（不包含）
///
/// # 返回
/// 返回 Vec<Option<Image>>
pub fn load_bevy_images_from_path(
    resource_path: &Path,
    library_name: &str,
    start: usize,
    end: usize,
) -> Result<Vec<Option<Image>>> {
    let (_info, previews) = load_images_from_path(resource_path, library_name, start, end)?;

    let images: Vec<Option<Image>> = previews
        .into_iter()
        .map(|preview| preview.and_then(|p| rgba_image_to_bevy(&p)))
        .collect();

    Ok(images)
}

/// 从资源路径加载指定索引列表的图像并转换为 Bevy Image
///
/// # 参数
/// - `resource_path`: 资源根目录
/// - `library_name`: 库文件名（不含扩展名）
/// - `indices`: 索引列表
///
/// # 返回
/// 返回 Vec<Option<Image>>
pub fn load_bevy_images_by_indices_from_path(
    resource_path: &Path,
    library_name: &str,
    indices: &[usize],
) -> Result<Vec<Option<Image>>> {
    let (_info, previews) = load_images_by_indices_from_path(resource_path, library_name, indices)?;

    let images: Vec<Option<Image>> = previews
        .into_iter()
        .map(|preview| preview.and_then(|p| rgba_image_to_bevy(&p)))
        .collect();

    Ok(images)
}

/// 帧动画资源
#[derive(Resource)]
pub struct FrameAnimation {
    /// 动画帧纹理
    pub frames: Vec<Handle<Image>>,
    /// 当前帧索引
    pub current_frame: usize,
    /// 帧率 (FPS)
    pub fps: f32,
    /// 累计时间
    pub timer: f32,
}

impl FrameAnimation {
    /// 创建新的帧动画
    pub fn new(frames: Vec<Handle<Image>>, fps: f32) -> Self {
        Self {
            frames,
            current_frame: 0,
            fps,
            timer: 0.0,
        }
    }

    /// 更新动画
    pub fn update(&mut self, delta: f32) {
        if self.frames.is_empty() {
            return;
        }

        self.timer += delta;
        let frame_time = 1.0 / self.fps;

        while self.timer >= frame_time {
            self.timer -= frame_time;
            self.current_frame = (self.current_frame + 1) % self.frames.len();
        }
    }

    /// 获取当前帧
    pub fn current_frame_handle(&self) -> Option<Handle<Image>> {
        self.frames.get(self.current_frame).cloned()
    }
}

/// 帧动画组件
#[derive(Component)]
pub struct AnimatedSprite {
    /// 动画名称
    pub animation_name: String,
}

/// 帧动画系统
pub fn frame_animation_system(
    time: Res<Time>,
    mut animations: ResMut<FrameAnimation>,
    mut query: Query<&mut Sprite, With<AnimatedSprite>>,
) {
    animations.update(time.delta_secs());

    // 更新所有动画精灵的纹理
    if let Some(current_handle) = animations.current_frame_handle() {
        for mut sprite in query.iter_mut() {
            sprite.image = current_handle.clone();
        }
    }
}

// ==================== ImageCache 缓存函数 ====================
// 使用 bevy_extended_ui::ImageCache，缓存 key 格式为 "{library_name}_{index}.png"
// 这样可以在 CSS 中通过 background-image: url("ChrSel_22.png") 引用

/// 生成 ImageCache 缓存 key
///
/// # 参数
/// - `library_name`: 库文件名（如 "data/ChrSel"）
/// - `index`: 图像索引
///
/// # 返回
/// 格式为 "{library_name}_{index}.png" 的 key
pub fn make_cache_key(library_name: &str, index: usize) -> String {
    // 提取文件名（去掉路径和扩展名）
    let name = Path::new(library_name)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(library_name);
    format!("{}_{}.png", name, index)
}

/// 从资源路径加载单张图像并注入到 ImageCache
///
/// # 参数
/// - `resource_path`: 资源根目录
/// - `library_name`: 库文件名（如 "data/ChrSel"）
/// - `index`: 图像索引
/// - `image_cache`: bevy_extended_ui 的 ImageCache
/// - `images`: Bevy 图像资源管理器
///
/// # 返回
/// 返回图像 Handle
pub fn load_image_to_cache(
    resource_path: &Path,
    library_name: &str,
    index: usize,
    image_cache: &mut ImageCache,
    images: &mut Assets<Image>,
) -> Result<Handle<Image>> {
    let key = make_cache_key(library_name, index);

    // 检查缓存
    if let Some(handle) = image_cache.map.get(&key) {
        tracing::trace!("从缓存获取图像: {}", key);
        return Ok(handle.clone());
    }

    // 加载图像
    let bevy_images = load_bevy_images_by_indices_from_path(resource_path, library_name, &[index])?;

    if let Some(Some(image)) = bevy_images.into_iter().next() {
        let handle = images.add(image);
        image_cache.map.insert(key.clone(), handle.clone());
        tracing::debug!("加载并缓存图像: {}", key);
        Ok(handle)
    } else {
        Err(crate::error::LibraryError::FileNotFound(format!(
            "图像未找到: {}",
            key
        )))
    }
}

/// 批量加载图像并注入到 ImageCache
///
/// # 参数
/// - `resource_path`: 资源根目录
/// - `library_name`: 库文件名（如 "data/ChrSel"）
/// - `indices`: 索引列表
/// - `image_cache`: bevy_extended_ui 的 ImageCache
/// - `images`: Bevy 图像资源管理器
///
/// # 返回
/// 返回 Vec<Option<Handle<Image>>>，与输入 indices 顺序对应
pub fn load_images_to_cache(
    resource_path: &Path,
    library_name: &str,
    indices: &[usize],
    image_cache: &mut ImageCache,
    images: &mut Assets<Image>,
) -> Result<Vec<Option<Handle<Image>>>> {
    let mut results = Vec::with_capacity(indices.len());
    let mut uncached_indices = Vec::new();
    let mut uncached_positions = Vec::new();

    // 第一步：检查哪些图像已经在缓存中
    for (pos, &index) in indices.iter().enumerate() {
        let key = make_cache_key(library_name, index);
        if let Some(handle) = image_cache.map.get(&key) {
            results.push(Some(handle.clone()));
        } else {
            results.push(None);
            uncached_indices.push(index);
            uncached_positions.push(pos);
        }
    }

    // 第二步：批量加载未缓存的图像
    if !uncached_indices.is_empty() {
        let loaded_images =
            load_bevy_images_by_indices_from_path(resource_path, library_name, &uncached_indices)?;

        // 第三步：缓存并填充结果
        for (i, pos) in uncached_positions.into_iter().enumerate() {
            if let Some(Some(image)) = loaded_images.get(i) {
                let handle = images.add(image.clone());
                let key = make_cache_key(library_name, uncached_indices[i]);
                image_cache.map.insert(key.clone(), handle.clone());
                results[pos] = Some(handle);
                tracing::debug!("加载并缓存图像: {}", key);
            }
        }
    }

    tracing::debug!(
        "批量加载图像完成: {} 共 {} 张，缓存命中 {} 张",
        library_name,
        indices.len(),
        results.iter().filter(|r| r.is_some()).count()
    );

    Ok(results)
}

/// 从指定范围加载图像并注入到 ImageCache
///
/// # 参数
/// - `resource_path`: 资源根目录
/// - `library_name`: 库文件名（如 "data/ChrSel"）
/// - `start`: 起始索引（包含）
/// - `end`: 结束索引（不包含）
/// - `image_cache`: bevy_extended_ui 的 ImageCache
/// - `images`: Bevy 图像资源管理器
///
/// # 返回
/// 返回 Vec<Option<Handle<Image>>>
pub fn load_images_range_to_cache(
    resource_path: &Path,
    library_name: &str,
    start: usize,
    end: usize,
    image_cache: &mut ImageCache,
    images: &mut Assets<Image>,
) -> Result<Vec<Option<Handle<Image>>>> {
    let indices: Vec<usize> = (start..end).collect();
    load_images_to_cache(resource_path, library_name, &indices, image_cache, images)
}

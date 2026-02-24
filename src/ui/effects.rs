//! UI 交互效果模块
//!
//! 提供可复用的按钮悬停、点击等交互效果
//!
//! ## 使用方法
//!
//! 在 login.rs 或其他场景中导入并使用：
//! ```rust
//! use crate::ui::effects::*;
//! ```
//!
//! 在 HTML 中添加事件：
//! ```html
//! <button onmouseenter="btn_hover_enter" onmouseleave="btn_hover_leave">按钮</button>
//! ```

use bevy::prelude::*;
use bevy_extended_ui::html::{HtmlClick, HtmlMouseOut, HtmlMouseOver};
use bevy_extended_ui::ImageCache;
use bevy_extended_ui_macros::html_fn;

/// 悬停效果配置
/// 存储按钮的悬停状态和图片信息
#[derive(Resource, Default)]
pub struct HoverEffects {
    /// 当前悬停的实体
    pub hovered_entity: Option<Entity>,
}

/// UI 效果插件
pub struct UiEffectsPlugin;

impl Plugin for UiEffectsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HoverEffects>();
    }
}

// ========== 通用事件处理函数（可在 HTML 中直接使用）==========

/// 通用鼠标进入事件 - 记录悬停状态
/// 使用方法: onmouseenter="btn_hover_enter"
#[html_fn("btn_hover_enter")]
pub fn btn_hover_enter(In(event): In<HtmlMouseOver>, mut hover: ResMut<HoverEffects>) {
    hover.hovered_entity = Some(event.entity);
    tracing::debug!("鼠标进入按钮: {:?}", event.entity);
}

/// 通用鼠标离开事件 - 清除悬停状态
/// 使用方法: onmouseleave="btn_hover_leave"
#[html_fn("btn_hover_leave")]
pub fn btn_hover_leave(In(_event): In<HtmlMouseOut>, mut hover: ResMut<HoverEffects>) {
    hover.hovered_entity = None;
    tracing::debug!("鼠标离开按钮");
}

/// 通用点击事件 - 记录点击
/// 使用方法: onclick="btn_click"
#[html_fn("btn_click")]
pub fn btn_click(In(event): In<HtmlClick>) {
    tracing::debug!("按钮点击: {:?}", event.entity);
}

// ========== 按钮 ID 标记组件 ==========

/// 关闭按钮标记
#[derive(Component)]
pub struct CloseButton;

/// 登录按钮标记
#[derive(Component)]
pub struct LoginButton;

/// 注册按钮标记
#[derive(Component)]
pub struct RegisterButton;

// ========== 辅助函数 ==========

/// 从 ImageCache 获取图片句柄
pub fn get_cached_image(image_cache: &ImageCache, key: &str) -> Option<Handle<Image>> {
    image_cache.map.get(key).cloned()
}

/// 检查图片是否在缓存中
pub fn has_cached_image(image_cache: &ImageCache, key: &str) -> bool {
    image_cache.map.contains_key(key)
}

/// 获取缓存中的图片数量
pub fn cache_size(image_cache: &ImageCache) -> usize {
    image_cache.map.len()
}

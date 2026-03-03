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
//!
//! ## CSS 状态 class
//!
//! 通过动态添加/移除 class 来切换按钮状态：
//! - `hover` - 悬停状态
//! - `active` - 按下状态
//!
//! CSS 示例：
//! ```css
//! .btn-close { /* 默认状态 */ }
//! .btn-close.hover { /* 悬停状态 */ }
//! .btn-close.active { /* 按下状态 */ }
//! ```

use bevy::prelude::*;
use bevy_extended_ui::ImageCache;
use bevy_extended_ui::html::{HtmlClick, HtmlDragStart, HtmlDragStop, HtmlMouseOut, HtmlMouseOver};
// use bevy_extended_ui::html::{
//     HtmlClick, HtmlDragStart, HtmlDragStop, HtmlMouseDown, HtmlMouseOut, HtmlMouseOver, HtmlMouseUp,
// };
use bevy_extended_ui::styles::CssClass;
use bevy_extended_ui_macros::html_fn;

/// 悬停状态 class 名
pub const CLASS_HOVER: &str = "btn-close-hover";
/// 激活/按下状态 class 名
pub const CLASS_ACTIVE: &str = "btn-close-active";

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

// ========== Class 操作辅助函数 ==========

/// 添加 class 到元素
pub fn add_class(css_class: &mut CssClass, class_name: &str) {
    if !css_class.0.iter().any(|c| c == class_name) {
        css_class.0.push(class_name.to_string());
    }
}

/// 从元素移除 class
pub fn remove_class(css_class: &mut CssClass, class_name: &str) {
    css_class.0.retain(|c| c != class_name);
}

/// 检查元素是否有指定 class
pub fn has_class(css_class: &CssClass, class_name: &str) -> bool {
    css_class.0.iter().any(|c| c == class_name)
}

/// 切换 class（有则移除，无则添加）
pub fn toggle_class(css_class: &mut CssClass, class_name: &str) {
    if has_class(css_class, class_name) {
        remove_class(css_class, class_name);
    } else {
        add_class(css_class, class_name);
    }
}

// ========== 通用事件处理函数（可在 HTML 中直接使用）==========

/// 通用鼠标进入事件 - 添加 hover class
/// 使用方法: onmouseenter="btn_hover_enter"
#[html_fn("btn_hover_enter")]
pub fn btn_hover_enter(In(event): In<HtmlMouseOver>, mut query: Query<&mut CssClass>) {
    if let Ok(mut css_class) = query.get_mut(event.entity) {
        add_class(&mut css_class, CLASS_HOVER);
        tracing::debug!("鼠标进入按钮: {:?}, 添加 hover class", event.entity);
    }
}

/// 通用鼠标离开事件 - 只移除 hover class
/// 注意：不移除 active class，因为用户可能还在按住鼠标按键
/// active class 会在 btn_release (mouseup) 或 on_drag_stop (dragstop) 时移除
/// 使用方法: onmouseleave="btn_hover_leave"
#[html_fn("btn_hover_leave")]
pub fn btn_hover_leave(In(event): In<HtmlMouseOut>, mut query: Query<&mut CssClass>) {
    if let Ok(mut css_class) = query.get_mut(event.entity) {
        remove_class(&mut css_class, CLASS_HOVER);
        // 不移除 CLASS_ACTIVE，因为用户可能还在按住鼠标
        tracing::debug!(
            "鼠标离开按钮: {:?}, 移除 hover class (保留 active class)",
            event.entity
        );
    }
}

/// 通用点击事件 - 切换 active class
/// 使用方法: onclick="btn_click"
#[html_fn("btn_click")]
pub fn btn_click(In(event): In<HtmlClick>, mut query: Query<&mut CssClass>) {
    if let Ok(mut css_class) = query.get_mut(event.entity) {
        toggle_class(&mut css_class, CLASS_ACTIVE);
        tracing::debug!("按钮点击: {:?}, 切换 active class", event.entity);
    }
}

/// 通用鼠标按下事件 - 添加 active class
/// 使用方法: onmousedown="btn_press"
// #[html_fn("btn_press")]
// pub fn btn_press(In(event): In<HtmlMouseDown>, mut query: Query<&mut CssClass>) {
//     if let Ok(mut css_class) = query.get_mut(event.entity) {
//         add_class(&mut css_class, CLASS_ACTIVE);
//         tracing::info!(
//             "鼠标按下事件触发: entity={:?}, position={:?}, inner_position={:?}",
//             event.entity,
//             event.position,
//             event.inner_position
//         );
//     }
// }

/// 通用鼠标释放事件 - 移除 active class
/// 使用方法: onmouseup="btn_release"
// #[html_fn("btn_release")]
// pub fn btn_release(In(event): In<HtmlMouseUp>, mut query: Query<&mut CssClass>) {
//     if let Ok(mut css_class) = query.get_mut(event.entity) {
//         remove_class(&mut css_class, CLASS_ACTIVE);
//         tracing::info!(
//             "鼠标释放事件触发: entity={:?}, position={:?}, inner_position={:?}",
//             event.entity,
//             event.position,
//             event.inner_position
//         );
//     }
// }

// ========== 拖拽事件（仅日志，不实现拖拽功能）==========

/// 拖拽开始事件 - 仅记录日志
/// 使用方法: ondragstart="on_drag_start"
#[html_fn("on_drag_start")]
pub fn on_drag_start(In(event): In<HtmlDragStart>) {
    tracing::info!(
        "拖拽开始事件触发: entity={:?}, position={:?}",
        event.entity,
        event.position
    );
}

/// 拖拽结束事件 - 移除 active class
/// 使用方法: ondragstop="on_drag_stop"
#[html_fn("on_drag_stop")]
pub fn on_drag_stop(In(event): In<HtmlDragStop>, mut query: Query<&mut CssClass>) {
    if let Ok(mut css_class) = query.get_mut(event.entity) {
        remove_class(&mut css_class, CLASS_ACTIVE);
        tracing::info!(
            "拖拽结束事件触发: entity={:?}, position={:?}, 已移除 active class",
            event.entity,
            event.position
        );
    }
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

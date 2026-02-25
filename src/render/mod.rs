//! 渲染系统模块

pub mod sprite_animation;

use bevy::prelude::*;
pub use sprite_animation::*;

/// 渲染插件
pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app // 初始化动画库
            .init_resource::<AnimationLibrary>()
            // 注册动画消息
            .add_message::<AnimationEvent>()
            .add_message::<AnimationCommand>()
            // 注册类型（用于反射和调试）
            .register_type::<AnimationAction>()
            .register_type::<AnimationDirection>()
            .register_type::<ItemEffectType>()
            .register_type::<SceneAnimation>()
            .register_type::<MonsterAnimation>()
            // 固定时间步长动画更新
            .add_systems(FixedUpdate, fixed_animation_update)
            // 一次性动画更新
            .add_systems(Update, oneshot_animation_update)
            // 动画命令处理
            .add_systems(Update, animation_command_handler);
    }
}

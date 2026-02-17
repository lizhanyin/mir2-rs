//! 角色系统模块

use bevy::prelude::*;

/// 角色插件
pub struct ActorPlugin;

impl Plugin for ActorPlugin {
    fn build(&self, _app: &mut App) {
        // TODO: 添加角色系统
    }
}

/// 角色类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActorType {
    Player,
    Npc,
    Monster,
}

/// 角色组件
#[derive(Component)]
pub struct Actor {
    pub id: u32,
    pub name: String,
    pub actor_type: ActorType,
}

/// 方向 (0-7, 对应8个方向)
#[derive(Debug, Clone, Copy, Default)]
pub struct Direction(pub u8);

impl Direction {
    pub fn to_vec2(&self) -> Vec2 {
        match self.0 {
            0 => Vec2::new(0.0, 1.0),   // 上
            1 => Vec2::new(1.0, 1.0),   // 右上
            2 => Vec2::new(1.0, 0.0),   // 右
            3 => Vec2::new(1.0, -1.0),  // 右下
            4 => Vec2::new(0.0, -1.0),  // 下
            5 => Vec2::new(-1.0, -1.0), // 左下
            6 => Vec2::new(-1.0, 0.0),  // 左
            7 => Vec2::new(-1.0, 1.0),  // 左上
            _ => Vec2::ZERO,
        }
    }
}

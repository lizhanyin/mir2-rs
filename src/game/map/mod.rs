//! 地图系统模块

use bevy::prelude::*;

/// 地图插件
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, _app: &mut App) {
        // TODO: 添加地图系统
    }
}

/// 地图块类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    Walkable,
    Blocked,
    Water,
}

/// 地图块组件
#[derive(Component)]
pub struct MapTile {
    pub x: i32,
    pub y: i32,
    pub tile_type: TileType,
}

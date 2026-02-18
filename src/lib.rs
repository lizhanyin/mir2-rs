//! Mir2-RS - 传奇2 客户端 Rust 实现
//!
//! 基于 Bevy 游戏引擎的《传奇2》客户端重构项目。

pub mod core;
pub mod error;
pub mod game;
pub mod network;
pub mod render;
pub mod resource;
pub mod scene;
pub mod ui;

// 重新导出常用类型
pub use error::{GameError, GameResult, LibraryError, Result};
pub use resource::{LibraryInfo, LibraryLoader, LibraryType};

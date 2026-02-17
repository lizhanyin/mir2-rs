//! 错误处理模块

use thiserror::Error;

/// 库文件错误类型
#[derive(Error, Debug)]
pub enum LibraryError {
    #[error("文件未找到: {0}")]
    FileNotFound(String),

    #[error("无效的文件格式")]
    InvalidFormat,

    #[error("不支持的版本: {0}")]
    UnsupportedVersion(i32),

    #[error("索引越界: {0}")]
    IndexOutOfBounds(usize),

    #[error("无效的图像数据")]
    InvalidImageData,

    #[error("解析错误: {0}")]
    ParseError(String),

    #[error("IO错误: {0}")]
    IoError(#[from] std::io::Error),

    #[error("图像错误: {0}")]
    ImageError(#[from] image::ImageError),

    #[error("压缩错误: {0}")]
    CompressionError(String),
}

/// 游戏错误类型
#[derive(Error, Debug)]
pub enum GameError {
    #[error("资源加载失败: {0}")]
    ResourceLoadFailed(String),

    #[error("场景切换失败: {0}")]
    SceneTransitionFailed(String),

    #[error("网络错误: {0}")]
    NetworkError(String),

    #[error("配置错误: {0}")]
    ConfigError(String),

    #[error("库错误: {0}")]
    LibraryError(#[from] LibraryError),

    #[error("IO错误: {0}")]
    IoError(#[from] std::io::Error),
}

/// 库操作结果类型
pub type Result<T> = std::result::Result<T, LibraryError>;

/// 游戏操作结果类型
pub type GameResult<T> = std::result::Result<T, GameError>;

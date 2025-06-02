//! 核心抽象层
//!
//! 提供主题系统的基础类型定义和核心抽象，包括：
//! - 基础类型定义 (Size, ColorType)
//! - 颜色核心抽象 (RgbColor, ColorPalette)
//! - 动画核心抽象 (Duration, Easing)
//! - 主题配置抽象

pub mod color;
pub mod config;
pub mod motion;
pub mod types;

// 重新导出核心类型
pub use color::*;
pub use config::*;
pub use motion::*;
pub use types::*;

//! Typography 组件模块
//!
//! 提供完整的文本排版系统，包含 Title、Text、Paragraph 和 Link 四个子组件。
//! 支持多种文本样式、交互功能和可访问性特性。

pub mod component;
pub mod styles;
pub mod types;

#[cfg(test)]
pub mod tests;

pub use component::*;
pub use types::*;
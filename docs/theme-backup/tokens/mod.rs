//! Ant Design 特定的设计令牌定义
//!
//! 本模块包含了 Ant Design 设计系统的具体令牌值，包括：
//! - 颜色令牌（主色、功能色、文本色等）
//! - 字体令牌（字号、字重、行高等）
//! - 间距令牌（内边距、外边距、间隙等）
//! - 尺寸令牌（组件尺寸、边框半径等）
//! - 动画令牌（缓动函数、持续时间等）

pub mod animation_presets;
pub mod ant_design_tokens;
pub mod color_presets;
pub mod theme_presets;

pub use animation_presets::*;
pub use ant_design_tokens::*;
pub use color_presets::*;
pub use theme_presets::*;

// 为了兼容 css-in-rust，提供 AntDesignTokenValues 别名
pub use ant_design_tokens::AntDesignTokens as AntDesignTokenValues;

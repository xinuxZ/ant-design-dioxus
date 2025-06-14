//! Typography 组件模块
//!
//! 提供文本排版相关的组件，包括标题、段落、文本和链接等。
//! 支持多种文本样式、交互功能、国际化和主题切换等特性。

pub mod component;
pub mod i18n;
pub mod rtl;
pub mod styles;
pub mod text_search;
pub mod text_selection;
pub mod theme_enhanced;
pub mod types;

// #[cfg(test)]
// pub mod tests;

pub use component::*;
pub use i18n::*;
pub use styles::*;
pub use theme_enhanced::*;
pub use types::*;

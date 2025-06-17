//! Tooltip 组件模块
//!
//! 提供简洁的文字提示气泡框，用于鼠标悬停时显示帮助信息。
//!
//! ## 功能特性
//! - 12种位置选项
//! - 多种触发方式（hover、focus、click、contextMenu）
//! - 自动边界检测和位置调整
//! - 延迟显示/隐藏
//! - 主题适配和自定义样式
//! - 完整的可访问性支持
//!
//! ## 基本用法
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::tooltip::Tooltip;
//!
//! #[component]
//! fn App() -> Element {
//!     rsx! {
//!         Tooltip {
//!             title: "提示文本",
//!             placement: TooltipPlacement::Top,
//!             button { "悬停显示提示" }
//!         }
//!     }
//! }
//! ```

pub mod components;
pub mod types;
pub mod styles;
pub mod hooks;
pub mod utils;
pub mod tests;

// 重新导出主要的公共 API
pub use components::*;
pub use types::*;
pub use styles::*;
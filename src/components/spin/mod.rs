//! # Spin 组件
//!
//! 用于页面和区块的加载中状态显示组件。
//!
//! ## 特性
//!
//! - 🔄 **流畅动画**：基于 CSS 的高性能旋转动画
//! - 📏 **多种尺寸**：支持 small、default、large 三种尺寸
//! - ⏱️ **延迟显示**：防止短暂加载状态的闪烁
//! - 🎨 **自定义指示器**：支持自定义加载指示器
//! - 📦 **内容包装**：可包装其他组件添加加载状态
//! - 🌙 **主题支持**：完整的亮色/暗色主题适配
//! - ♿ **可访问性**：符合 WCAG 2.1 AA 标准
//!
//! ## 使用示例
//!
//! ### 基础用法
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::Spin;
//!
//! #[component]
//! fn App() -> Element {
//!     rsx! {
//!         Spin {}
//!     }
//! }
//! ```
//!
//! ### 包装内容
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::{Spin, Alert, AlertType};
//!
//! #[component]
//! fn App() -> Element {
//!     let mut loading = use_signal(|| true);
//!
//!     rsx! {
//!         Spin {
//!             spinning: loading(),
//!             tip: "Loading...",
//!             Alert {
//!                 message: "Alert message title",
//!                 description: "Further details about the context of this alert.",
//!                 alert_type: AlertType::Info,
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! ### 不同尺寸
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::{Spin, SpinSize, Space};
//!
//! #[component]
//! fn App() -> Element {
//!     rsx! {
//!         Space {
//!             Spin { size: SpinSize::Small }
//!             Spin { size: SpinSize::Default }
//!             Spin { size: SpinSize::Large }
//!         }
//!     }
//! }
//! ```
//!
//! ### 延迟显示
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::Spin;
//!
//! #[component]
//! fn App() -> Element {
//!     let mut loading = use_signal(|| true);
//!
//!     rsx! {
//!         Spin {
//!             spinning: loading(),
//!             delay: 500, // 500ms 后才显示加载状态
//!             tip: "Loading...",
//!             div {
//!                 "Content here"
//!             }
//!         }
//!     }
//! }
//! ```

pub mod component;
pub mod styles;
pub mod types;
pub mod utils;

// 重新导出主要类型和组件
pub use component::*;
pub use types::*;

// 便捷构造函数
pub use component::{
    spin_large,
    spin_small,
    spin_with_tip,
    // spin_delayed,
};

// 样式相关导出
pub use styles::*;

// 工具函数导出
pub use utils::{create_spin_state, validate_spin_props};

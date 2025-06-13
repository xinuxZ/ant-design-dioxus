//!
//! Alert 组件
//!
//! 警告提示，展示需要关注的信息。
//!
//! ## 引入
//!
//! ```rust
//! use ant_design_dioxus::Alert;
//! ```
//!
//! ## 示例
//!
//! ```rust
//! use ant_design_dioxus::Alert;
//! use dioxus::prelude::*;
//!
//! #[component]
//! pub fn AlertExample() -> Element {
//!     rsx! {
//!         Alert {
//!             message: "Success Text".to_string(),
//!             alert_type: AlertType::Success,
//!             show_icon: true
//!         }
//!     }
//! }
//! ```

pub mod component;
pub mod styles;
pub mod tests;
pub mod types;

pub use component::*;
pub use types::*;

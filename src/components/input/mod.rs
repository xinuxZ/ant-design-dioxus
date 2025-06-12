//! # Input 输入框
//!
//! 通过鼠标或键盘输入内容，是最基础的表单域的包装。
//!
//! ## 何时使用
//!
//! - 需要用户输入表单域内容时。
//! - 提供组合型输入框，带搜索的输入框，还可以进行大小选择。
//!
//! ## 基本用法
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::Input;
//!
//! #[component]
//! fn App() -> Element {
//!     let mut value = use_signal(|| String::new());
//!
//!     rsx! {
//!         Input {
//!             value: value.read().clone(),
//!             placeholder: "请输入内容",
//!             on_change: move |val| value.set(val),
//!         }
//!     }
//! }
//! ```

mod component;
pub mod styles;
mod types;

pub use component::*;
pub use types::*;

// 组件已通过#[component]宏自动导出
// 无需重新导出

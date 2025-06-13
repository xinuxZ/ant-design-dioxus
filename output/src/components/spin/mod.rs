//! Spin 加载中组件
//!
//! 用于页面和区块的加载中状态。
//!
//! ## 何时使用
//!
//! 页面局部处于等待异步数据或正在渲染过程时，合适的加载动效会有效缓解用户的焦虑。
//!
//! ## 代码演示
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::Spin;
//!
//! #[component]
//! fn App() -> Element {
//!     rsx! {
//!         Spin {
//!             tip: "加载中...",
//!             div {
//!                 style: "height: 200px; background: #f0f0f0; padding: 20px;",
//!                 "内容区域"
//!             }
//!         }
//!     }
//! }
//! ```

mod component;
mod styles;
mod types;

pub use component::*;
pub use styles::*;
pub use types::*;
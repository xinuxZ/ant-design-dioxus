//!
//! Divider 分割线组件
//!
//! 区隔内容的分割线。
//!
//! ## 何时使用
//!
//! - 对不同章节的文本段落进行分割。
//! - 对行内文字/链接进行分割，例如表格的操作列。
//!
//! ## 示例
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::Divider;
//!
//! fn app() -> Element {
//!     rsx! {
//!         div {
//!             p { "Lorem ipsum dolor sit amet, consectetur adipiscing elit." }
//!             Divider {}
//!             p { "Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua." }
//!         }
//!     }
//! }
//! ```

mod component;
mod types;
mod styles;
mod migrated_styles;

#[cfg(test)]
mod tests;

#[cfg(feature = "examples")]
pub mod examples;

pub use component::Divider;
pub use types::*;
pub use styles::*;

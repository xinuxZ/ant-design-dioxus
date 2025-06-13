//!
//! Tag 标签组件
//!
//! 进行标记和分类的小标签。
//!
//! ## 何时使用
//!
//! - 用于标记事物的属性和维度。
//! - 进行分类。
//!
//! ## 示例
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::Tag;
//!
//! fn app() -> Element {
//!     rsx! {
//!         div {
//!             Tag { "Tag 1" }
//!             Tag { color: "blue", "Tag 2" }
//!             Tag { closable: true, "Closable Tag" }
//!         }
//!     }
//! }
//! ```

mod component;
mod types;
mod styles;

#[cfg(test)]
mod tests;

#[cfg(feature = "examples")]
pub mod examples;

pub use component::Tag;
pub use types::*;
pub use styles::*;
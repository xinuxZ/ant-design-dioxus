//! # Divider 分割线组件
//!
//! 分割线组件用于分隔不同内容区域，支持水平和垂直方向，可以嵌入文本内容。
//!
//! ## 特性
//!
//! - **多种方向**：支持水平和垂直分割线
//! - **文本嵌入**：支持在分割线中嵌入文本，可控制位置
//! - **样式变体**：支持实线、虚线、点线三种样式
//! - **文本样式**：支持标题样式和朴素文本样式
//! - **尺寸控制**：支持不同尺寸的间距
//! - **主题支持**：完整的主题系统和暗色模式
//! - **可访问性**：语义化HTML和ARIA支持
//!
//! ## 基础用法
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::Divider;
//!
//! #[component]
//! fn App() -> Element {
//!     rsx! {
//!         div {
//!             p { "第一段内容" }
//!             Divider {}
//!             p { "第二段内容" }
//!             Divider { dashed: true }
//!             p { "第三段内容" }
//!         }
//!     }
//! }
//! ```
//!
//! ## 带文本分割线
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::{Divider, DividerOrientation};
//!
//! #[component]
//! fn TextDivider() -> Element {
//!     rsx! {
//!         div {
//!             p { "内容区域" }
//!             Divider { "标题文本" }
//!             p { "内容区域" }
//!             Divider { orientation: DividerOrientation::Left, "左对齐文本" }
//!             p { "内容区域" }
//!             Divider { orientation: DividerOrientation::Right, "右对齐文本" }
//!             p { "内容区域" }
//!         }
//!     }
//! }
//! ```
//!
//! ## 垂直分割线
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::{Divider, DividerType};
//!
//! #[component]
//! fn VerticalDivider() -> Element {
//!     rsx! {
//!         span {
//!             "文本"
//!             Divider { r#type: DividerType::Vertical }
//!             a { href: "#", "链接" }
//!             Divider { r#type: DividerType::Vertical }
//!             a { href: "#", "链接" }
//!         }
//!     }
//! }
//! ```
//!
//! ## 样式变体
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::{Divider, DividerVariant};
//!
//! #[component]
//! fn VariantDivider() -> Element {
//!     rsx! {
//!         div {
//!             Divider { variant: DividerVariant::Solid, "实线" }
//!             Divider { variant: DividerVariant::Dashed, "虚线" }
//!             Divider { variant: DividerVariant::Dotted, "点线" }
//!         }
//!     }
//! }
//! ```

pub mod component;
pub mod styles;
pub mod types;
pub mod utils;

#[cfg(test)]
mod tests;

// 重新导出主要类型和组件
pub use component::*;
pub use types::*;

// // 便捷构造函数
// pub use utils::{
//     divider_horizontal,
//     divider_vertical,
//     divider_with_text,
//     divider_dashed,
//     divider_dotted,
//     divider_plain,
// };

// 样式相关
pub use styles::{generate_divider_stylesheet, DividerStyleGenerator};

// 主题和配置
pub use types::{
    get_default_divider_theme, set_default_divider_theme, DividerConfig, DividerTheme,
};

// Hooks
// pub use component::use_divider_theme;

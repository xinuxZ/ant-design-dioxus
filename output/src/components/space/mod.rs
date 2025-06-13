//! Space 间距组件
//!
//! Space 组件用于设置组件之间的间距，避免组件紧贴在一起，拉开统一的空间。
//! 适用于需要在某个方向上保持统一间距的场景，支持水平、垂直方向的间距设置。
//!
//! ## 特性
//!
//! - 🎯 **灵活布局**: 支持水平和垂直方向的间距设置
//! - 📏 **多种尺寸**: 提供小、中、大三种预设尺寸，支持自定义尺寸
//! - 🎨 **对齐控制**: 支持起始、结束、居中、基线四种对齐方式
//! - 🔄 **自动换行**: 水平方向支持自动换行功能
//! - ✂️ **分割元素**: 支持在元素间添加分割线或自定义分割元素
//! - 🎭 **主题适配**: 完整的主题系统支持和CSS-in-Rust实现
//! - 🔧 **高度可定制**: 支持自定义类名、样式和前缀
//!
//! ## 何时使用
//!
//! - 避免组件紧贴在一起，拉开统一的空间
//! - 在某组件的某个方向上，保持统一的间距
//! - 需要在元素间添加分割线或分割元素
//! - 需要控制元素的对齐方式和换行行为
//!
//! ## 基础用法
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::{Space, SpaceSize, SpaceDirection};
//!
//! fn app() -> Element {
//!     rsx! {
//!         // 基础水平间距
//!         Space {
//!             Button { "按钮1" }
//!             Button { "按钮2" }
//!             Button { "按钮3" }
//!         }
//!         
//!         // 垂直间距
//!         Space {
//!             direction: SpaceDirection::Vertical,
//!             size: SpaceSize::Large,
//!             Card { "卡片1" }
//!             Card { "卡片2" }
//!             Card { "卡片3" }
//!         }
//!     }
//! }
//! ```
//!
//! ## 高级用法
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::{Space, SpaceSize, SpaceAlign};
//!
//! fn advanced_space() -> Element {
//!     rsx! {
//!         // 自定义尺寸和对齐
//!         Space {
//!             size: SpaceSize::Custom(20),
//!             align: SpaceAlign::Center,
//!             wrap: true,
//!             Button { "按钮1" }
//!             Button { "按钮2" }
//!             Button { "按钮3" }
//!         }
//!         
//!         // 带分割线
//!         Space {
//!             split: rsx! { Divider { type: DividerType::Vertical } },
//!             Link { "链接1" }
//!             Link { "链接2" }
//!             Link { "链接3" }
//!         }
//!     }
//! }
//! ```

// 公共模块
pub mod component;
pub mod styles;
pub mod types;

// 测试模块
#[cfg(test)]
mod tests;

// 重新导出公共API
pub use component::Space;
pub use types::*;

// 重新导出样式相关类型（供高级用户使用）
pub use styles::{SpaceStyleGenerator, SpaceStyles};
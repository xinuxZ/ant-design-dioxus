//! # Space 组件
//!
//! Space 组件用于设置组件之间的间距，避免组件紧贴在一起，提供统一的空间布局。
//! 它为内联元素设置间距，会为每个子元素添加包装元素以实现内联对齐。
//!
//! ## 特性
//!
//! - 🎯 **间距控制** - 支持预设和自定义间距大小
//! - 📐 **方向布局** - 支持水平和垂直方向布局
//! - 🎨 **对齐方式** - 多种对齐选项（start、end、center、baseline）
//! - 📱 **响应式** - 支持不同方向的响应式间距
//! - 🔄 **自动换行** - 水平布局时支持自动换行
//! - ✂️ **分割元素** - 支持在子元素间插入分割线
//! - 🎯 **紧凑模式** - Space.Compact 支持表单组件紧密连接
//! - 🎨 **主题定制** - 完整的主题系统支持
//!
//! ## 使用示例
//!
//! ### 基础用法
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::Space;
//!
//! #[component]
//! fn App() -> Element {
//!     rsx! {
//!         Space {
//!             Button { "按钮1" }
//!             Button { "按钮2" }
//!             Button { "按钮3" }
//!         }
//!     }
//! }
//! ```
//!
//! ### 垂直布局
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::{Space, SpaceDirection};
//!
//! #[component]
//! fn App() -> Element {
//!     rsx! {
//!         Space {
//!             direction: SpaceDirection::Vertical,
//!             size: "large",
//!             Card { "卡片1" }
//!             Card { "卡片2" }
//!             Card { "卡片3" }
//!         }
//!     }
//! }
//! ```
//!
//! ### 紧凑模式
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::SpaceCompact;
//!
//! #[component]
//! fn App() -> Element {
//!     rsx! {
//!         SpaceCompact {
//!             Button { "按钮1" }
//!             Button { "按钮2" }
//!             Input { placeholder: "输入框" }
//!         }
//!     }
//! }
//! ```
//!
//! ### 响应式间距
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::{Space, SpaceSizeConfig};
//!
//! #[component]
//! fn App() -> Element {
//!     rsx! {
//!         Space {
//!             size: SpaceSizeConfig::Array([16, 24]),
//!             wrap: true,
//!             Tag { "标签1" }
//!             Tag { "标签2" }
//!             Tag { "标签3" }
//!         }
//!     }
//! }
//! ```

pub mod component;
pub mod styles;
pub mod types;
pub mod utils;

// 重新导出主要组件和类型
pub use component::{Space, SpaceCompact};

// 为了支持 Space.Compact 语法，我们需要为 Space 添加关联的 Compact 组件
// impl Space {
//     /// Space.Compact 组件 - 紧凑模式的间距组件
//     ///
//     /// 这是 Space 组件的关联组件，用于实现类似 Space.Compact 的语法
//     pub fn Compact(props: SpaceCompactProps) -> Element {
//         SpaceCompact(props)
//     }
// }

pub use types::{
    CompactSize, SpaceAlign, SpaceCompactProps, SpaceDirection, SpaceProps, SpaceSize,
    SpaceSizeConfig, SpaceState, SpaceTheme,
};

pub use styles::{
    generate_nested_space_compact_styles, generate_space_compact_styles, generate_space_styles,
    get_space_class_name, get_space_compact_class_name,
};

pub use utils::{
    calculate_space_size, detect_space_compact_nesting_level,
    get_nested_space_compact_container_class, get_space_compact_container_class,
    get_space_container_class, get_space_gap_value, merge_space_theme, should_wrap_children,
    validate_space_compact_nesting, validate_space_config,
};

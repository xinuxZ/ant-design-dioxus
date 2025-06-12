//! Button 组件模块
//!
//! 提供完整的 Button 和 ButtonGroup 组件实现，包括：
//! - 多种按钮类型（Primary、Default、Dashed、Text、Link）
//! - 多种尺寸（Large、Middle、Small）
//! - 多种形状（Default、Circle、Round）
//! - 状态支持（危险、幽灵、加载、禁用、块级）
//! - 按钮组功能
//!
//! # 示例
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::{Button, ButtonGroup, ButtonType, ButtonSize};
//!
//! fn app() -> Element {
//!     rsx! {
//!         div {
//!             // 基础按钮
//!             Button {
//!                 button_type: ButtonType::Primary,
//!                 "主要按钮"
//!             }
//!             
//!             // 按钮组
//!             ButtonGroup {
//!                 size: ButtonGroupSize::Large,
//!                 Button { "按钮1" }
//!                 Button { "按钮2" }
//!                 Button { "按钮3" }
//!             }
//!         }
//!     }
//! }
//! ```

mod component;
mod types;
mod styles;
mod migrated_styles;

// 导出组件
pub use component::{Button, ButtonGroup};

// 导出类型
pub use types::{
    ButtonType, ButtonSize, ButtonShape, ButtonHtmlType, ButtonGroupSize,
    ButtonProps, ButtonGroupProps,
};

// 导出样式
pub use styles::{
    ButtonStyles, VariantStyles, StateStyles, ButtonStyleGenerator,
    generate_button_styles, generate_button_group_styles,
};

// 导出迁移的样式（向后兼容）
pub use migrated_styles::*;

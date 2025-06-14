//! # Flex 组件
//!
//! Flex 组件是基于 CSS Flexbox 的现代布局容器，提供强大的对齐和分布能力。
//! 与 Space 组件不同，Flex 用于设置块级元素的布局，不添加包装元素，
//! 适用于垂直或水平方向的子元素布局。
//!
//! ## 特性
//!
//! - 🎯 **灵活布局**: 基于 CSS Flexbox 的强大布局能力
//! - 📐 **多种对齐**: 支持主轴和交叉轴的多种对齐方式
//! - 📏 **间距控制**: 灵活的间距设置，支持预设值和自定义值
//! - 🔄 **换行控制**: 支持自动换行和换行方向控制
//! - 📱 **响应式**: 支持响应式间距和布局配置
//! - 🎨 **主题支持**: 完整的主题定制和暗色模式支持
//! - ♿ **可访问性**: 遵循 WCAG 2.1 标准的可访问性实现
//!
//! ## 基础用法
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::{Flex, FlexGap};
//!
//! fn app() -> Element {
//!     rsx! {
//!         // 水平布局
//!         Flex {
//!             gap: FlexGap::Middle,
//!             div { "Item 1" }
//!             div { "Item 2" }
//!             div { "Item 3" }
//!         }
//!     }
//! }
//! ```
//!
//! ## 垂直布局
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::{Flex, FlexGap};
//!
//! fn vertical_layout() -> Element {
//!     rsx! {
//!         Flex {
//!             vertical: true,
//!             gap: FlexGap::Large,
//!             div { "Item A" }
//!             div { "Item B" }
//!             div { "Item C" }
//!         }
//!     }
//! }
//! ```
//!
//! ## 对齐控制
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::{Flex, FlexJustify, FlexAlign};
//!
//! fn alignment_demo() -> Element {
//!     rsx! {
//!         // 居中对齐
//!         Flex {
//!             justify: FlexJustify::Center,
//!             align: FlexAlign::Center,
//!             style: "height: 200px;",
//!             div { "Centered Content" }
//!         }
//!
//!         // 两端对齐
//!         Flex {
//!             justify: FlexJustify::SpaceBetween,
//!             div { "Left" }
//!             div { "Center" }
//!             div { "Right" }
//!         }
//!     }
//! }
//! ```
//!
//! ## 响应式布局
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::{Flex, FlexWrap, FlexGap};
//!
//! fn responsive_demo() -> Element {
//!     rsx! {
//!         Flex {
//!             wrap: FlexWrap::Wrap,
//!             gap: FlexGap::Array(["16px".to_string(), "24px".to_string()]),
//!
//!             for i in 1..=6 {
//!                 div {
//!                     key: "{i}",
//!                     style: "flex: 1 1 200px; padding: 16px;",
//!                     "Item {i}"
//!                 }
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

#[cfg(test)]
mod tests;

// 重新导出样式和工具函数
pub use styles::{
    generate_compact_theme_styles, generate_dark_theme_styles, generate_flex_container_styles,
    generate_flex_item_styles, generate_grid_styles, generate_layout_styles,
    generate_responsive_styles, get_cross_axis_property, get_flex_container_class,
    get_flex_item_class, get_main_axis_property, get_performance_optimized_styles,
};

pub use utils::{
    calculate_flex_basis, calculate_gap_value, calculate_item_size, calculate_theme_hash,
    check_performance_mode, create_default_config, create_flex_config, estimate_memory_usage,
    format_flex_value, generate_cache_key, generate_css_variables, get_optimal_render_strategy,
    get_responsive_config, is_responsive_gap, merge_flex_theme, optimize_flex_performance,
    should_wrap, validate_flex_props,
};

// 便捷构造函数
pub use component::{
    flex_around, flex_between, flex_center, flex_column, flex_evenly, flex_horizontal, flex_row,
    flex_vertical, flex_wrap,
};

// 高阶组件
pub use component::{FlexContainer, FlexGrid, FlexItem, FlexLayout};

// 主题和配置
pub use component::{
    get_default_flex_theme, set_default_flex_theme, use_flex_theme, FlexThemeProvider,
};

// Hooks
pub use component::{use_flex_layout, use_responsive_flex};

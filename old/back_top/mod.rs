//! # BackTop 组件
//!
//! BackTop（回到顶部）组件提供了一个浮动按钮，当页面内容很长时，
//! 用户可以通过点击该按钮快速回到页面顶部。
//!
//! ## 特性
//!
//! - 🎯 **智能显示**: 基于滚动位置自动显示/隐藏
//! - 🎨 **自定义样式**: 支持完全自定义按钮内容和样式
//! - 🎪 **滚动目标**: 支持监听指定容器的滚动事件
//! - ⚡ **性能优化**: 滚动事件节流和动画帧优化
//! - 🌙 **主题支持**: 内置暗色模式和自定义主题
//! - ♿ **可访问性**: 完整的键盘导航和屏幕阅读器支持
//! - 📱 **响应式**: 移动端友好的触摸交互
//!
//! ## 基础用法
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::BackTop;
//!
//! fn App() -> Element {
//!     rsx! {
//!         div {
//!             style: "height: 2000px; padding: 20px;",
//!             h1 { "长页面内容" }
//!             p { "滚动到底部查看 BackTop 按钮" }
//!
//!             BackTop {}
//!         }
//!     }
//! }
//! ```
//!
//! ## 自定义内容
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::BackTop;
//!
//! fn CustomBackTop() -> Element {
//!     rsx! {
//!         div {
//!             style: "height: 2000px;",
//!
//!             BackTop {
//!                 visibility_height: 300,
//!                 bottom: 100,
//!                 right: 100,
//!                 div {
//!                     style: "
//!                         height: 40px;
//!                         width: 40px;
//!                         line-height: 40px;
//!                         border-radius: 4px;
//!                         background-color: #1088e9;
//!                         color: #fff;
//!                         text-align: center;
//!                         font-size: 20px;
//!                     ",
//!                     "UP"
//!                 }
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! ## 自定义滚动容器
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::BackTop;
//!
//! fn ScrollableContainer() -> Element {
//!     let container_ref = use_signal(|| None::<web_sys::Element>);
//!
//!     rsx! {
//!         div {
//!             id: "scrollable-container",
//!             style: "height: 400px; overflow-y: auto;",
//!             onmounted: move |event| {
//!                 if let Ok(element) = event.data().downcast::<web_sys::Element>() {
//!                     container_ref.set(Some(element.clone()));
//!                 }
//!             },
//!
//!             div {
//!                 style: "height: 2000px; padding: 20px;",
//!                 "容器内的长内容"
//!             }
//!
//!             BackTop {
//!                 target: move || container_ref.read().clone(),
//!                 visibility_height: 200,
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! ## 高级用法
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::{BackTop, BackTopTheme};
//!
//! fn AdvancedBackTop() -> Element {
//!     let custom_theme = BackTopTheme {
//!         background_color: "#722ed1".to_string(),
//!         hover_background_color: "#531dab".to_string(),
//!         icon_color: "#ffffff".to_string(),
//!         hover_icon_color: "#f0f0f0".to_string(),
//!         border_radius: "50%".to_string(),
//!         box_shadow: "0 4px 12px rgba(114, 46, 209, 0.3)".to_string(),
//!         hover_box_shadow: "0 6px 16px rgba(114, 46, 209, 0.4)".to_string(),
//!         transition: "all 0.3s cubic-bezier(0.4, 0, 0.2, 1)".to_string(),
//!         z_index: 1000,
//!     };
//!
//!     rsx! {
//!         div {
//!             style: "height: 3000px; padding: 20px;",
//!
//!             BackTop {
//!                 visibility_height: 500,
//!                 duration: 600,
//!                 bottom: 80,
//!                 right: 80,
//!                 theme: custom_theme,
//!                 on_click: move |_| {
//!                     println!("BackTop clicked!");
//!                 },
//!             }
//!         }
//!     }
//! }
//! ```

pub mod component;
// pub mod hooks;
pub mod styles;
pub mod types;
pub mod utils;

#[cfg(test)]
mod tests;

// 重新导出核心类型和组件
pub use component::*;
// pub use hooks::*;
pub use styles::*;
pub use types::*;
pub use utils::*;

// 便捷构造函数
pub use component::{
    // back_top,
    custom_back_top,
    positioned_back_top,
    target_back_top,
    // themed_back_top,
    // container_back_top,
};

// 样式生成器
pub use styles::{generate_back_top_stylesheet, BackTopStyleGenerator};

// Hooks
// pub use hooks::{
//     use_back_top_animation, use_back_top_scroll, use_back_top_theme, use_back_top_visibility,
// };

// 工具函数
pub use utils::{
    calculate_scroll_position, create_back_top_config, smooth_scroll_to_top,
    validate_back_top_props,
};

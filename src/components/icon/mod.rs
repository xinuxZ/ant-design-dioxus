//! Icon 组件模块
//!
//! 提供图标显示功能，支持多种主题、尺寸和交互状态。
//! 兼容 Ant Design 的图标系统，支持自定义 SVG 图标和图标字体。
//!
//! # 特性
//!
//! - 多种图标主题：outlined、filled、two-tone
//! - 灵活的尺寸控制：预设尺寸和自定义尺寸
//! - 旋转和动画效果：静态旋转和旋转动画
//! - 交互状态：可点击、禁用状态
//! - 自定义图标：支持 SVG 字符串和图标字体
//! - 双色图标：支持自定义双色图标的颜色
//!
//! # 使用示例
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::{Icon, IconTheme, IconSize, IconType};
//!
//! fn app() -> Element {
//!     rsx! {
//!         // 基础图标
//!         Icon {
//!             icon_type: IconType::Home,
//!             theme: IconTheme::Outlined,
//!         }
//!
//!         // 大尺寸填充图标
//!         Icon {
//!             icon_type: IconType::User,
//!             theme: IconTheme::Filled,
//!             size: IconSize::Large,
//!         }
//!
//!         // 可点击的旋转图标
//!         Icon {
//!             icon_type: IconType::Setting,
//!             spin: true,
//!             on_click: |_| {
//!                 // 处理点击事件
//!             },
//!         }
//!
//!         // 双色图标
//!         Icon {
//!             icon_type: IconType::Heart,
//!             theme: IconTheme::TwoTone,
//!             two_tone_color: "#ff4d4f",
//!         }
//!     }
//! }
//! ```

// 子模块声明
mod component;
mod styles;
mod types;
mod utils;

// 公共导出
pub use component::*;
pub use styles::class_names;
pub use types::*;
pub use utils::{get_global_icon, register_global_icon, validate_icon_name, IconLibrary, SvgIcon};

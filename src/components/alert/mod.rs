//! # Alert 警告提示组件
//!
//! 警告提示，展现需要关注的信息。
//!
//! ## 何时使用
//!
//! - 当某个页面需要向用户显示警告的信息时
//! - 非浮层的静态展现形式，始终展现，不会自动消失，用户可以点击关闭
//!
//! ## 特性
//!
//! - 🎨 **四种类型**：支持 success、info、warning、error 四种警告类型
//! - 🔧 **可关闭**：支持显示关闭按钮，点击可关闭警告提示
//! - 🎯 **图标显示**：支持显示辅助图标，让信息类型更加醒目
//! - 📝 **辅助文字**：支持显示详细的辅助性文字介绍
//! - ⚡ **动画效果**：平滑、自然的关闭动画
//! - 🎛️ **自定义操作**：支持在右上角自定义操作项
//! - 🎪 **顶部公告**：支持页面顶部通告形式
//! - 🌈 **主题定制**：支持自定义主题和样式
//! - ♿ **无障碍**：完整的键盘导航和屏幕阅读器支持
//! - 📱 **响应式**：支持移动端和桌面端适配
//!
//! ## 基础用法
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::{Alert, AlertType};
//!
//! fn app() -> Element {
//!     rsx! {
//!         Alert {
//!             message: "Success Text",
//!             alert_type: AlertType::Success,
//!         }
//!     }
//! }
//! ```
//!
//! ## 四种样式
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::{Alert, AlertType};
//!
//! fn app() -> Element {
//!     rsx! {
//!         Alert {
//!             message: "Success Text",
//!             alert_type: AlertType::Success,
//!         }
//!         Alert {
//!             message: "Info Text",
//!             alert_type: AlertType::Info,
//!         }
//!         Alert {
//!             message: "Warning Text",
//!             alert_type: AlertType::Warning,
//!         }
//!         Alert {
//!             message: "Error Text",
//!             alert_type: AlertType::Error,
//!         }
//!     }
//! }
//! ```
//!
//! ## 可关闭的警告提示
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::{Alert, AlertType};
//!
//! fn app() -> Element {
//!     rsx! {
//!         Alert {
//!             message: "Warning Text",
//!             alert_type: AlertType::Warning,
//!             closable: true,
//!             on_close: move |_| {
//!                 println!("Alert closed!");
//!             },
//!         }
//!     }
//! }
//! ```
//!
//! ## 含有图标
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::{Alert, AlertType};
//!
//! fn app() -> Element {
//!     rsx! {
//!         Alert {
//!             message: "Success Tips",
//!             alert_type: AlertType::Success,
//!             show_icon: true,
//!         }
//!         Alert {
//!             message: "Informational Notes",
//!             alert_type: AlertType::Info,
//!             show_icon: true,
//!         }
//!         Alert {
//!             message: "Warning",
//!             alert_type: AlertType::Warning,
//!             show_icon: true,
//!             closable: true,
//!         }
//!         Alert {
//!             message: "Error",
//!             alert_type: AlertType::Error,
//!             show_icon: true,
//!         }
//!     }
//! }
//! ```
//!
//! ## 含有辅助性文字介绍
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::{Alert, AlertType};
//!
//! fn app() -> Element {
//!     rsx! {
//!         Alert {
//!             message: "Success Tips",
//!             description: "Detailed description and advice about successful copywriting.",
//!             alert_type: AlertType::Success,
//!             show_icon: true,
//!         }
//!         Alert {
//!             message: "Informational Notes",
//!             description: "Additional description and information about copywriting.",
//!             alert_type: AlertType::Info,
//!             show_icon: true,
//!         }
//!     }
//! }
//! ```
//!
//! ## 自定义操作项
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::{Alert, AlertType, Button, ButtonType, ButtonSize};
//!
//! fn app() -> Element {
//!     rsx! {
//!         Alert {
//!             message: "Success Tips",
//!             alert_type: AlertType::Success,
//!             show_icon: true,
//!             closable: true,
//!             action: rsx! {
//!                 Button {
//!                     size: ButtonSize::Small,
//!                     button_type: Some(ButtonType::Text),
//!                     "UNDO"
//!                 }
//!             },
//!         }
//!     }
//! }
//! ```
//!
//! ## 顶部公告
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::{Alert, AlertType};
//!
//! fn app() -> Element {
//!     rsx! {
//!         Alert {
//!             message: "Warning",
//!             banner: true,
//!             closable: true,
//!         }
//!     }
//! }
//! ```
//!
//! ## 平滑地卸载
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::{Alert, AlertType};
//!
//! fn app() -> Element {
//!     let mut visible = use_signal(|| true);
//!
//!     rsx! {
//!         if visible() {
//!             Alert {
//!                 message: "Alert Message Text",
//!                 alert_type: AlertType::Success,
//!                 closable: true,
//!                 after_close: move |_| {
//!                     visible.set(false);
//!                 },
//!             }
//!         }
//!         p { "placeholder text here" }
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
pub use utils::*;

#[cfg(test)]
mod tests;

// 便捷构造函数
pub use component::{
    action_alert, banner_alert, closable_alert, error_alert, icon_alert, info_alert, success_alert,
    warning_alert,
};

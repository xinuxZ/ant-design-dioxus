//! QRCode 二维码组件
//!
//! 用于生成二维码的组件。
//!
//! ## 何时使用
//!
//! - 当需要将链接转换为二维码时使用。
//! - 适用于需要通过扫码进行信息传递的场景。
//!
//! ## 代码演示
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::components::qr_code::{QRCode, QRCodeSize, QRCodeErrorLevel, QRCodeType};
//!
//! #[component]
//! fn App() -> Element {
//!     rsx! {
//!         div {
//!             style: "padding: 20px;",
//!
//!             // 基础用法
//!             QRCode {
//!                 value: "https://ant-design-dioxus.rs",
//!             }
//!
//!             // 自定义样式
//!             QRCode {
//!                 value: "https://ant-design-dioxus.rs",
//!                 size: QRCodeSize::Custom(200),
//!                 color: "#1677FF",
//!                 bg_color: "#f5f5f5",
//!                 error_level: QRCodeErrorLevel::H,
//!             }
//!
//!             // 带图标的二维码
//!             QRCode {
//!                 value: "https://ant-design-dioxus.rs",
//!                 icon: Some("https://example.com/icon.png".to_string()),
//!                 icon_size: 40,
//!             }
//!         }
//!     }
//! }
//! ```

mod component;
mod style_generator;
mod styles;
mod types;
mod utils;

pub use component::QRCode;
pub use types::{QRCodeErrorLevel, QRCodeProps, QRCodeSize, QRCodeStatus, QRCodeType};

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
//! use ant_design_dioxus::components::qr_code::{QRCode, QRCodeErrorLevel, QRCodeType};
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
//!                 size: 200,
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
mod styles;
mod types;
mod utils;

#[cfg(test)]
mod tests;

pub use component::QRCode;
pub use types::{QRCodeErrorLevel, QRCodeProps, QRCodeStatus, QRCodeType};

/// QRCode 组件的便捷构造函数
impl QRCodeProps {
    /// 创建一个默认的 QRCode 组件
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }

    /// 设置渲染类型
    pub fn r#type(mut self, qr_type: QRCodeType) -> Self {
        self.r#type = qr_type;
        self
    }

    /// 设置图标
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// 设置二维码大小
    pub fn size(mut self, size: u32) -> Self {
        self.size = size;
        self
    }

    /// 设置图标大小
    pub fn icon_size(mut self, icon_size: u32) -> Self {
        self.icon_size = icon_size;
        self
    }

    /// 设置二维码颜色
    pub fn color(mut self, color: impl Into<String>) -> Self {
        self.color = color.into();
        self
    }

    /// 设置二维码背景色
    pub fn bg_color(mut self, bg_color: impl Into<String>) -> Self {
        self.bg_color = bg_color.into();
        self
    }

    /// 设置是否有边框
    pub fn bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }

    /// 设置纠错级别
    pub fn error_level(mut self, error_level: QRCodeErrorLevel) -> Self {
        self.error_level = error_level;
        self
    }

    /// 设置二维码状态
    pub fn status(mut self, status: QRCodeStatus) -> Self {
        self.status = status;
        self
    }

    /// 设置自定义样式类名
    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.class = Some(class.into());
        self
    }

    /// 设置自定义样式
    pub fn style(mut self, style: impl Into<String>) -> Self {
        self.style = Some(style.into());
        self
    }
}

impl Default for QRCodeProps {
    fn default() -> Self {
        Self {
            value: String::new(),
            r#type: QRCodeType::default(),
            icon: None,
            size: 160,
            icon_size: 40,
            color: "#000000".to_string(),
            bg_color: "transparent".to_string(),
            bordered: true,
            error_level: QRCodeErrorLevel::default(),
            status: QRCodeStatus::default(),
            status_render: None,
            on_refresh: None,
            class: None,
            style: None,
            children: None,
        }
    }
}

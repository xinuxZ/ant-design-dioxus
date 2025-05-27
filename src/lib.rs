//! # Ant Design Dioxus
//!
//! An enterprise-class UI design language and Dioxus components implementation.
//!
//! ## Quick Start
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::prelude::*;
//!
//! fn App() -> Element {
//!     rsx! {
//!         ConfigProvider {
//!             Button {
//!                 button_type: ButtonType::Primary,
//!                 "Hello Ant Design Dioxus!"
//!             }
//!         }
//!     }
//! }
//! ```

// 核心模块
pub mod config_provider;
pub mod locale;
pub mod theme;
pub mod utils;

// 组件模块
pub mod components;

// 预导入模块
pub mod prelude {
    //! 常用类型和组件的预导入模块

    // 重新导出 Dioxus 核心类型
    pub use dioxus::prelude::*;

    // 配置和主题
    pub use crate::config_provider::{ConfigProvider, ConfigProviderProps};
    pub use crate::theme::{Theme, ThemeConfig, ThemeProvider};

    // 工具类型
    pub use crate::utils::class_names::*;
    pub use crate::utils::color::*;
    pub use crate::utils::motion::Direction as MotionDirection;
    pub use crate::utils::responsive::*;
    pub use crate::utils::size::*;
    pub use crate::utils::{is_chinese_char, is_two_cn_char};

    // 组件（当前为空，后续会添加）
    pub use crate::components::*;
}

// 版本信息
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
}

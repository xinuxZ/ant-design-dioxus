//! Ant Design Dioxus
//!
//! 一个基于 Dioxus 框架的 Ant Design 组件库实现
//!
//! ## 特性
//!
//! - 🎨 企业级 UI 设计语言
//! - 📦 开箱即用的高质量 Rust 组件
//! - 🛡️ 使用 Rust 编写，类型安全
//! - ⚡ 基于 Dioxus 的高性能渲染
//! - 🌍 国际化语言支持
//! - 🎭 主题定制能力
//! - 📱 响应式设计
//!
//! ## 快速开始
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::prelude::*;
//!
//! fn app() -> Element {
//!     rsx! {
//!         ConfigProvider {
//!             theme: "light",
//!             locale: "zh_CN",
//!             // 你的应用内容
//!         }
//!     }
//! }
//! ```

// 引用依赖以避免未使用警告
#[allow(unused_imports)]
use getrandom as _;
#[allow(unused_imports)]
use once_cell as _;
#[allow(unused_imports)]
use serde_json as _;
#[allow(unused_imports)]
use web_sys as _;

// 引用dev-dependencies以避免未使用警告
#[cfg(test)]
extern crate wasm_bindgen_test;

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

    // 重新导出 Dioxus 核心类型（避免命名冲突）
    pub use dioxus::prelude::{component, rsx, Element, EventHandler, MouseEvent, Props};

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

// 重新导出主要组件
pub use components::button::{Button, ButtonHtmlType, ButtonShape, ButtonSize, ButtonType};

// 重新导出主要模块（避免名称冲突）
pub use config_provider::ConfigProvider;
pub use theme::ThemeProvider;

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

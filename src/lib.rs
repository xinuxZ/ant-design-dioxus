//! # Ant Design Dioxus
//!
//! 基于 Dioxus 的 Ant Design 组件库
//!
//! ## 特性
//!
//! - 完整实现 Ant Design 设计规范
//! - 提供 60+ 高质量组件
//! - 支持主题定制
//! - 支持国际化
//! - 支持服务端渲染
//! - 支持移动端适配
//!
//! ## 使用方式
//!
//! ```rust
//! use ant_design_dioxus::prelude::*;
//!
//! #[component]
//! fn App() -> Element {
//!     rsx! {
//!         ConfigProvider {
//!             theme: Some(ThemeBuilder::new().add_token("colorPrimary", "#1890ff").build()),
//!             locale: Some(Locale::ZhCN),
//!             div {
//!                 Button { "按钮" }
//!                 Input { placeholder: "请输入" }
//!             }
//!         }
//!     }
//! }
//! ```

// 引用依赖以避免未使用警告
#[allow(unused_imports)]
use css_in_rust_macros as _;
use log as _;
#[allow(unused_imports)]
use once_cell as _;
#[allow(unused_imports)]
use serde_json as _;
#[allow(unused_imports)]
use web_sys as _;

// 引用dev-dependencies以避免未使用警告
// #[cfg(test)]
// extern crate wasm_bindgen_test;

// 核心模块
pub mod config_provider;
pub mod locale;
pub mod prelude;
pub mod shared;
pub mod theme;
pub mod utils;

// 组件模块
pub mod components;
pub mod hooks;

// 预导入模块已移至 prelude.rs 文件
// 该模块提供了更清晰的组件导出结构，避免命名冲突

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

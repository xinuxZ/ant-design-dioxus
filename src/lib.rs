//! # Ant Design Dioxus
//!
//! 一个基于 Dioxus 的 Ant Design 组件库实现
//!
//! ## 特性
//!
//! - 🎨 **企业级 UI 设计语言**：遵循 Ant Design 设计规范
//! - 🛡️ **类型安全**：使用 Rust 的类型系统确保组件安全
//! - 🎯 **开箱即用**：高质量的 Dioxus 组件
//! - 🌈 **主题定制**：支持主题定制和暗色模式
//! - 🌍 **国际化**：内置国际化支持
//! - ⚡ **高性能**：基于 Dioxus 的高性能渲染
//!
//! ## 快速开始
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

pub mod components;
pub mod config_provider;
pub mod hooks;
pub mod locale;
pub mod prelude;
pub mod shared;
pub mod theme;
pub mod utils;

// 重新导出核心依赖
pub use dioxus;
pub use dioxus::prelude::*;

// 版本信息
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// 组件库初始化函数
pub fn init() {
    // 初始化日志
    #[cfg(feature = "web")]
    {
        console_log::init_with_level(log::Level::Info).ok();
    }
    
    // 初始化主题系统
    // theme::init_theme_system();
    
    // 初始化国际化
    // locale::init_locale_system();
}

/// 错误类型定义
#[derive(Debug, Clone, PartialEq)]
pub enum AntDesignError {
    /// 主题相关错误
    ThemeError(String),
    /// 国际化相关错误
    LocaleError(String),
    /// 组件配置错误
    ConfigError(String),
    /// 其他错误
    Other(String),
}

impl std::fmt::Display for AntDesignError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AntDesignError::ThemeError(msg) => write!(f, "Theme error: {}", msg),
            AntDesignError::LocaleError(msg) => write!(f, "Locale error: {}", msg),
            AntDesignError::ConfigError(msg) => write!(f, "Config error: {}", msg),
            AntDesignError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for AntDesignError {}

/// 结果类型别名
pub type Result<T> = std::result::Result<T, AntDesignError>;
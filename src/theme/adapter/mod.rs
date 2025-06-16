//! 主题适配层模块
//!
//! 适配层连接核心层和框架层，提供通用的适配接口和扩展功能。
//! 包括样式注入、服务端渲染支持、框架适配器和主题提供者扩展。
//!
//! # 主要组件
//!
//! - **框架适配器** (frameworks)：为不同框架提供适配，如 Dioxus 和 React
//! - **样式注入器** (injection)：负责将生成的 CSS 注入到不同平台
//! - **主题提供者** (provider)：扩展核心主题提供者，添加高级功能
//! - **SSR 支持** (ssr)：提供服务端渲染支持和客户端水合
//!
//! # Examples
//!
//! ```
//! use css_in_rust::theme::adapter::{ThemeProviderAdapter, StyleInjector, DioxusAdapter, SsrSupport};
//! use css_in_rust::theme::core::manager::ThemeManager;
//! use std::sync::Arc;
//!
//! // 创建主题管理器
//! let manager = Arc::new(ThemeManager::default());
//!
//! // 创建主题提供者适配器
//! let provider_adapter = ThemeProviderAdapter::default();
//!
//! // 创建样式注入器
//! let style_injector = StyleInjector::new(":root");
//!
//! // 创建 Dioxus 适配器
//! let dioxus_adapter = DioxusAdapter::new(provider_adapter);
//!
//! // 创建 SSR 支持
//! let ssr_support = SsrSupport::default();
//! ```

pub mod frameworks;
pub mod injection;
pub mod provider;
pub mod ssr;

// Re-exports
pub use frameworks::DioxusAdapter;
pub use injection::StyleInjector;
pub use provider::ThemeProviderAdapter;
pub use ssr::SsrSupport;

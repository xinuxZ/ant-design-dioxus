//! # Prelude
//!
//! 常用类型和组件的重新导出，方便用户使用

// 重新导出 Dioxus 核心
pub use dioxus::prelude::*;

// 重新导出组件
pub use crate::components::*;

// 重新导出配置提供者
pub use crate::config_provider::*;

// 重新导出主题系统
pub use crate::theme::*;

// 重新导出国际化
pub use crate::locale::*;

// 重新导出工具函数
pub use crate::utils::*;

// 重新导出 hooks
pub use crate::hooks::*;

// 重新导出共享类型
pub use crate::shared::*;

// 重新导出错误类型
pub use crate::{AntDesignError, Result};
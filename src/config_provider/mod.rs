//! 全局配置提供者模块
//!
//! 提供全局配置上下文和组件，用于管理应用级别的配置

use std::collections::HashMap;
use std::rc::Rc;

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::locale::Locale;
use crate::theme::ThemeConfig;

// 导出子模块
pub mod builder;
pub mod component_config;
pub mod config_utils;
pub mod hooks;
pub mod popup_config;
pub mod security_config;

// 重新导出主要类型
pub use builder::*;
pub use component_config::*;
pub use config_utils::*;
pub use hooks::{use_config, ConfigProvider, ConfigProviderProps, GlobalConfigContext};
pub use popup_config::*;
pub use security_config::*;
pub use virtual_scroll_config::*;

#[cfg(test)]
mod tests;
pub mod virtual_scroll_config;

/// 文本方向
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    /// 从左到右
    Ltr,
    /// 从右到左
    Rtl,
}

impl Default for Direction {
    fn default() -> Self {
        Self::Ltr
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Ltr => write!(f, "ltr"),
            Direction::Rtl => write!(f, "rtl"),
        }
    }
}

/// 组件尺寸
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComponentSize {
    /// 小尺寸
    Small,
    /// 中等尺寸
    Middle,
    /// 大尺寸
    Large,
}

impl Default for ComponentSize {
    fn default() -> Self {
        Self::Middle
    }
}

impl std::fmt::Display for ComponentSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComponentSize::Small => write!(f, "small"),
            ComponentSize::Middle => write!(f, "middle"),
            ComponentSize::Large => write!(f, "large"),
        }
    }
}

/// 组件尺寸配置
#[derive(Debug, Clone, PartialEq)]
pub struct ComponentSizeConfig {
    /// 默认组件尺寸
    pub default_size: ComponentSize,
}

impl Default for ComponentSizeConfig {
    fn default() -> Self {
        Self {
            default_size: ComponentSize::Middle,
        }
    }
}

/// 表单配置
#[derive(Debug, Clone, PartialEq)]
pub struct FormConfig {
    /// 是否显示验证状态图标
    pub validate_messages: HashMap<String, String>,
    /// 是否显示必填标记
    pub required_mark: bool,
    /// 是否显示冒号
    pub colon: bool,
    /// 标签对齐方式
    pub label_align: String,
    /// 标签宽度
    pub label_width: Option<String>,
}

impl Default for FormConfig {
    fn default() -> Self {
        Self {
            validate_messages: HashMap::new(),
            required_mark: true,
            colon: true,
            label_align: "right".to_string(),
            label_width: None,
        }
    }
}

/// 主题配置
#[derive(Debug, Clone, PartialEq)]
pub struct ThemeProviderConfig {
    /// 主题配置
    pub theme: ThemeConfig,
    /// 是否启用紧凑模式
    pub compact: bool,
}

impl Default for ThemeProviderConfig {
    fn default() -> Self {
        Self {
            theme: ThemeConfig::default(),
            compact: false,
        }
    }
}

/// 语言配置
#[derive(Debug, Clone, PartialEq)]
pub struct LocaleProviderConfig {
    /// 语言
    pub locale: Locale,
}

impl Default for LocaleProviderConfig {
    fn default() -> Self {
        Self {
            locale: Locale::ZhCN,
        }
    }
}

/// 全局配置
#[derive(Debug, Clone, PartialEq)]
pub struct GlobalConfig {
    /// 主题配置
    pub theme: ThemeProviderConfig,
    /// 语言配置
    pub locale: LocaleProviderConfig,
    /// 组件尺寸配置
    pub component_size: ComponentSizeConfig,
    /// 方向
    pub direction: Direction,
    /// CSS 类名前缀
    pub prefix_cls: String,
    /// 按钮中是否自动插入空格
    pub auto_insert_space_in_button: bool,
    /// 表单配置
    pub form: FormConfig,
    /// 获取弹出容器的函数
    pub get_popup_container: Option<fn() -> Element>,
    /// 获取目标容器的函数
    pub get_target_container: Option<fn() -> Element>,
}

impl Default for GlobalConfig {
    fn default() -> Self {
        Self {
            theme: ThemeProviderConfig::default(),
            locale: LocaleProviderConfig::default(),
            component_size: ComponentSizeConfig::default(),
            direction: Direction::Ltr,
            prefix_cls: "ant".to_string(),
            auto_insert_space_in_button: true,
            form: FormConfig::default(),
            get_popup_container: None,
            get_target_container: None,
        }
    }
}

/// 配置上下文
#[derive(Debug, Clone, PartialEq)]
pub struct ConfigContext {
    /// 全局配置
    pub config: GlobalConfig,
}

// ConfigProviderProps 已移动到 hooks.rs 模块中

// ConfigProvider 组件已移动到 hooks.rs 模块中

// Hooks函数已移动到hooks.rs模块中，通过重新导出提供访问

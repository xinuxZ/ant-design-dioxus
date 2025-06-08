//! 主题系统模块
//!
//! 提供 Ant Design 主题系统的实现，包括主题配置、令牌系统、算法和工具。
//! 基于 css-in-rust 的主题系统构建，专为 Dioxus 框架优化。

use dioxus::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

// 重新导出 css-in-rust 的核心功能
pub use css_in_rust::theme::{
    core::{
        cache::component_cache::ComponentStyleCache,
        css::{variables::CssVariableManager, CssGenerator},
        manager::ThemeManager,
        token::{
            definitions::{ColorValue, DimensionValue, ThemeVariant, TokenValue},
            resolver::TokenResolver,
            simple_system::TokenSystem,
        },
    },
    theme_types::{Theme, ThemeMode},
};

// 导出桥接器
pub use css_in_rust::theme_bridge::ThemeBridge as CssThemeBridge;

// 主题模块
pub mod algorithm;
pub mod core;
pub mod hooks;
pub mod presets;
pub mod provider;
pub mod tokens;

// 重新导出常用类型和函数
pub use algorithm::*;
pub use core::types::*;
pub use hooks::*;
pub use presets::*;
pub use provider::*;
pub use tokens::*;

/// 主题配置
#[derive(Debug, Clone, PartialEq)]
pub struct ThemeConfig {
    /// 主题实例
    pub theme: Theme,
    /// 是否启用紧凑模式
    pub compact: bool,
    /// 自定义令牌
    pub token: HashMap<String, String>,
    /// 组件令牌
    pub components: HashMap<String, HashMap<String, String>>,
    /// 算法配置
    pub algorithm: Vec<Algorithm>,
}

/// 主题算法
#[derive(Debug, Clone, PartialEq)]
pub enum Algorithm {
    /// 默认算法
    Default,
    /// 暗色算法
    Dark,
    /// 紧凑算法
    Compact,
    /// 自定义算法
    Custom(String),
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
            compact: false,
            token: HashMap::new(),
            components: HashMap::new(),
            algorithm: vec![Algorithm::Default],
        }
    }
}

impl ThemeConfig {
    /// 创建新的主题配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 创建亮色主题
    pub fn light() -> Self {
        Self {
            theme: Theme::new("light").with_mode(ThemeMode::Light),
            compact: false,
            token: HashMap::new(),
            components: HashMap::new(),
            algorithm: vec![Algorithm::Default],
        }
    }

    /// 创建暗色主题
    pub fn dark() -> Self {
        Self {
            theme: Theme::new("dark").with_mode(ThemeMode::Dark),
            compact: false,
            token: HashMap::new(),
            components: HashMap::new(),
            algorithm: vec![Algorithm::Default, Algorithm::Dark],
        }
    }

    /// 创建紧凑主题
    pub fn compact() -> Self {
        Self {
            theme: Theme::new("compact").with_mode(ThemeMode::Light),
            compact: true,
            token: HashMap::new(),
            components: HashMap::new(),
            algorithm: vec![Algorithm::Default, Algorithm::Compact],
        }
    }

    /// 创建紧凑暗色主题
    pub fn compact_dark() -> Self {
        Self {
            theme: Theme::new("compact-dark").with_mode(ThemeMode::Dark),
            compact: true,
            token: HashMap::new(),
            components: HashMap::new(),
            algorithm: vec![Algorithm::Default, Algorithm::Dark, Algorithm::Compact],
        }
    }

    /// 设置主色
    pub fn with_primary_color(mut self, color: impl Into<String>) -> Self {
        self.token.insert("colorPrimary".to_string(), color.into());
        self
    }

    /// 设置成功色
    pub fn with_success_color(mut self, color: impl Into<String>) -> Self {
        self.token.insert("colorSuccess".to_string(), color.into());
        self
    }

    /// 设置警告色
    pub fn with_warning_color(mut self, color: impl Into<String>) -> Self {
        self.token.insert("colorWarning".to_string(), color.into());
        self
    }

    /// 设置错误色
    pub fn with_error_color(mut self, color: impl Into<String>) -> Self {
        self.token.insert("colorError".to_string(), color.into());
        self
    }

    /// 设置信息色
    pub fn with_info_color(mut self, color: impl Into<String>) -> Self {
        self.token.insert("colorInfo".to_string(), color.into());
        self
    }

    /// 设置边框圆角
    pub fn with_border_radius(mut self, radius: i32) -> Self {
        self.token
            .insert("borderRadius".to_string(), radius.to_string());
        self
    }

    /// 设置基础字体大小
    pub fn with_font_size(mut self, size: i32) -> Self {
        self.token.insert("fontSize".to_string(), size.to_string());
        self
    }

    /// 设置线框模式
    pub fn with_wireframe(mut self, wireframe: bool) -> Self {
        self.token
            .insert("wireframe".to_string(), wireframe.to_string());
        self
    }

    /// 设置组件令牌
    pub fn with_component_token(
        mut self,
        component: impl Into<String>,
        token: HashMap<String, String>,
    ) -> Self {
        self.components.insert(component.into(), token);
        self
    }

    /// 设置自定义令牌
    pub fn with_token(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.token.insert(key.into(), value.into());
        self
    }

    /// 设置算法
    pub fn with_algorithm(mut self, algorithm: Vec<Algorithm>) -> Self {
        self.algorithm = algorithm;
        self
    }

    /// 添加算法
    pub fn add_algorithm(mut self, algorithm: Algorithm) -> Self {
        self.algorithm.push(algorithm);
        self
    }

    /// 生成CSS变量
    pub fn to_css_variables(&self) -> String {
        // 基础实现，实际应用中需要更复杂的逻辑
        let mut css = String::from(":root {\n");

        // 添加基础令牌
        for (key, value) in &self.token {
            css.push_str(&format!("  --{}: {};\n", key, value));
        }

        // 根据算法生成派生令牌
        if self.algorithm.contains(&Algorithm::Dark) {
            css.push_str("  --color-bg-container: #141414;\n");
            css.push_str("  --color-text-base: rgba(255, 255, 255, 0.85);\n");
        } else {
            css.push_str("  --color-bg-container: #ffffff;\n");
            css.push_str("  --color-text-base: rgba(0, 0, 0, 0.85);\n");
        }

        // 紧凑模式
        if self.algorithm.contains(&Algorithm::Compact) {
            css.push_str("  --size-unit: 4px;\n");
            css.push_str("  --size-step: 4px;\n");
        } else {
            css.push_str("  --size-unit: 8px;\n");
            css.push_str("  --size-step: 8px;\n");
        }

        css.push_str("}\n");
        css
    }
}

//! 主题系统模块
//!
//! 提供 Ant Design 主题系统的实现，包括主题配置、令牌系统、算法和工具。
//! 基于 css-in-rust 的主题系统构建，专为 Dioxus 框架优化。

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
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
    theme_types::{Theme as CssTheme, ThemeMode},
};

// 导出桥接器
pub use css_in_rust::theme_bridge::ThemeBridge as CssThemeBridge;

/// 主题类型
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Theme {
    /// 亮色主题
    Light,
    /// 暗色主题
    Dark,
    /// 紧凑主题
    Compact,
    /// 自定义主题
    Custom,
}

impl Default for Theme {
    fn default() -> Self {
        Self::Light
    }
}

// 主题模块
pub mod algorithm;
pub mod color_utils;
pub mod core;
pub mod css_vars;
pub mod hooks;
pub mod presets;
pub mod provider;
pub mod tokens;

// 重新导出常用类型和函数
pub use algorithm::*;
pub use color_utils::*;
pub use core::types::*;
pub use css_vars::*;
pub use hooks::*;
pub use presets::*;
pub use provider::*;
pub use tokens::*;

/// 主题配置
///
/// 包含主题相关的所有配置项
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ThemeConfig {
    /// 主题对象
    pub theme: CssTheme,
    /// 当前主题类型
    pub theme_type: Theme,
    /// 是否启用紧凑模式
    pub compact: bool,
    /// 主题令牌
    pub token: HashMap<String, String>,
    /// 组件令牌
    pub components: HashMap<String, HashMap<String, String>>,
    /// CSS变量选项
    pub css_vars: css_vars::CssVariablesOptions,
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
            theme: CssTheme::default(),
            theme_type: Theme::Light,
            compact: false,
            token: HashMap::new(),
            components: HashMap::new(),
            css_vars: css_vars::CssVariablesOptions::default(),
        }
    }
}

impl ThemeConfig {
    /// 创建新的主题配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置主题
    pub fn theme(mut self, theme: Theme) -> Self {
        match theme {
            Theme::Light => {
                self.theme.mode = ThemeMode::Light;
                self.compact = false;
            }
            Theme::Dark => {
                self.theme.mode = ThemeMode::Dark;
                self.compact = false;
            }
            Theme::Compact => {
                self.theme.mode = ThemeMode::Light;
                self.compact = true;
            }
            Theme::Custom => {
                // 不做特殊处理，保持当前设置
            }
        }
        self
    }

    /// 设置紧凑模式
    pub fn compact(mut self, compact: bool) -> Self {
        self.compact = compact;
        self
    }

    /// 设置令牌
    pub fn token(mut self, key: &str, value: &str) -> Self {
        self.token.insert(key.to_string(), value.to_string());
        self
    }

    /// 设置组件令牌
    pub fn component_token(mut self, component: &str, key: &str, value: &str) -> Self {
        self.components
            .entry(component.to_string())
            .or_insert_with(HashMap::new)
            .insert(key.to_string(), value.to_string());
        self
    }

    /// 启用CSS变量
    pub fn enable_css_vars(mut self, enabled: bool) -> Self {
        self.css_vars.enabled = enabled;
        self
    }

    /// 设置CSS变量前缀
    pub fn css_vars_prefix(mut self, prefix: &str) -> Self {
        self.css_vars.prefix = prefix.to_string();
        self
    }

    /// 设置CSS变量根选择器
    pub fn css_vars_root_selector(mut self, selector: &str) -> Self {
        self.css_vars.root_selector = selector.to_string();
        self
    }

    /// 生成并注入CSS变量
    pub fn inject_css_vars(&self) -> String {
        let seed = SeedToken::default();
        let map = MapToken::default();
        let alias = AliasToken::default();

        let css = css_vars::generate_css_variables(&seed, &map, &alias, &self.css_vars);
        css_vars::inject_css_variables(&css);
        css
    }

    /// 创建亮色主题配置
    pub fn light() -> Self {
        let mut theme = CssTheme::default();
        theme.mode = ThemeMode::Light;
        Self {
            theme,
            theme_type: Theme::Light,
            compact: false,
            token: HashMap::new(),
            components: HashMap::new(),
            css_vars: css_vars::CssVariablesOptions::default(),
        }
    }

    /// 创建暗色主题配置
    pub fn dark() -> Self {
        let mut theme = CssTheme::default();
        theme.mode = ThemeMode::Dark;
        Self {
            theme,
            theme_type: Theme::Dark,
            compact: false,
            token: HashMap::new(),
            components: HashMap::new(),
            css_vars: css_vars::CssVariablesOptions::default(),
        }
    }

    /// 创建紧凑主题配置
    pub fn compact() -> Self {
        let mut config = Self::light();
        config.theme_type = Theme::Compact;
        config.compact = true;
        config
    }

    /// 创建暗色紧凑主题配置
    pub fn compact_dark() -> Self {
        let mut config = Self::dark();
        config.theme_type = Theme::Compact;
        config.compact = true;
        config
    }

    /// 设置主题主色
    pub fn with_primary_color(mut self, color: &str) -> Self {
        self.token
            .insert("colorPrimary".to_string(), color.to_string());
        self
    }

    /// 设置成功色
    pub fn with_success_color(mut self, color: &str) -> Self {
        self.token
            .insert("colorSuccess".to_string(), color.to_string());
        self
    }

    /// 设置警告色
    pub fn with_warning_color(mut self, color: &str) -> Self {
        self.token
            .insert("colorWarning".to_string(), color.to_string());
        self
    }

    /// 设置错误色
    pub fn with_error_color(mut self, color: &str) -> Self {
        self.token
            .insert("colorError".to_string(), color.to_string());
        self
    }

    /// 设置信息色
    pub fn with_info_color(mut self, color: &str) -> Self {
        self.token
            .insert("colorInfo".to_string(), color.to_string());
        self
    }

    /// 设置边框圆角
    pub fn with_border_radius(mut self, radius: f32) -> Self {
        self.token
            .insert("borderRadius".to_string(), radius.to_string());
        self
    }

    /// 设置字体大小
    pub fn with_font_size(mut self, size: f32) -> Self {
        self.token.insert("fontSize".to_string(), size.to_string());
        self
    }

    /// 设置线框模式
    pub fn with_wireframe(mut self, wireframe: bool) -> Self {
        self.token
            .insert("wireframe".to_string(), wireframe.to_string());
        self
    }

    /// 获取主题令牌
    pub fn get_token(&self, key: &str) -> Option<&String> {
        self.token.get(key)
    }

    /// 获取组件令牌
    pub fn get_component_token(&self, component: &str, key: &str) -> Option<&String> {
        self.components
            .get(component)
            .and_then(|tokens| tokens.get(key))
    }

    /// 合并主题配置
    pub fn merge(&mut self, other: &ThemeConfig) {
        self.compact = other.compact;

        // 合并主题令牌
        for (key, value) in &other.token {
            self.token.insert(key.clone(), value.clone());
        }

        // 合并组件令牌
        for (component, tokens) in &other.components {
            let component_tokens = self
                .components
                .entry(component.clone())
                .or_insert_with(HashMap::new);

            for (key, value) in tokens {
                component_tokens.insert(key.clone(), value.clone());
            }
        }
    }
}

/// 主题构建器
///
/// 用于链式构建主题配置
#[derive(Debug, Clone)]
pub struct ThemeBuilder {
    /// 主题配置
    config: ThemeConfig,
}

impl ThemeBuilder {
    /// 创建新的主题构建器
    pub fn new() -> Self {
        Self {
            config: ThemeConfig::default(),
        }
    }

    /// 设置主题
    pub fn theme(mut self, theme: CssTheme) -> Self {
        self.config.theme = theme;
        self
    }

    /// 设置紧凑模式
    pub fn compact(mut self, compact: bool) -> Self {
        self.config.compact = compact;
        self
    }

    /// 设置主题令牌
    pub fn token(mut self, token: HashMap<String, String>) -> Self {
        self.config.token = token;
        self
    }

    /// 添加主题令牌
    pub fn add_token(mut self, key: &str, value: &str) -> Self {
        self.config.token.insert(key.to_string(), value.to_string());
        self
    }

    /// 设置组件令牌
    pub fn components(mut self, components: HashMap<String, HashMap<String, String>>) -> Self {
        self.config.components = components;
        self
    }

    /// 添加组件令牌
    pub fn add_component_token(mut self, component: &str, key: &str, value: &str) -> Self {
        let component_tokens = self
            .config
            .components
            .entry(component.to_string())
            .or_insert_with(HashMap::new);

        component_tokens.insert(key.to_string(), value.to_string());
        self
    }

    /// 构建主题配置
    pub fn build(self) -> ThemeConfig {
        self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_config_default() {
        let config = ThemeConfig::default();
        assert!(!config.compact);
        assert!(config.token.is_empty());
        assert!(config.components.is_empty());
    }

    #[test]
    fn test_theme_config_with_compact() {
        let config = ThemeConfig::default().with_compact(true);
        assert!(config.compact);
    }

    #[test]
    fn test_theme_config_with_token() {
        let mut token = HashMap::new();
        token.insert("primary".to_string(), "#1890ff".to_string());
        token.insert("success".to_string(), "#52c41a".to_string());

        let config = ThemeConfig::default().with_token(token.clone());
        assert_eq!(config.token, token);
    }

    #[test]
    fn test_theme_config_with_components() {
        let mut button = HashMap::new();
        button.insert("height".to_string(), "32px".to_string());
        button.insert("padding".to_string(), "0 15px".to_string());

        let mut input = HashMap::new();
        input.insert("height".to_string(), "32px".to_string());
        input.insert("padding".to_string(), "4px 11px".to_string());

        let mut components = HashMap::new();
        components.insert("Button".to_string(), button);
        components.insert("Input".to_string(), input);

        let config = ThemeConfig::default().with_components(components.clone());
        assert_eq!(config.components, components);
    }

    #[test]
    fn test_theme_config_get_token() {
        let mut token = HashMap::new();
        token.insert("primary".to_string(), "#1890ff".to_string());
        token.insert("success".to_string(), "#52c41a".to_string());

        let config = ThemeConfig::default().with_token(token);
        assert_eq!(config.get_token("primary"), Some(&"#1890ff".to_string()));
        assert_eq!(config.get_token("success"), Some(&"#52c41a".to_string()));
        assert_eq!(config.get_token("warning"), None);
    }

    #[test]
    fn test_theme_config_get_component_token() {
        let mut button = HashMap::new();
        button.insert("height".to_string(), "32px".to_string());
        button.insert("padding".to_string(), "0 15px".to_string());

        let mut components = HashMap::new();
        components.insert("Button".to_string(), button);

        let config = ThemeConfig::default().with_components(components);
        assert_eq!(
            config.get_component_token("Button", "height"),
            Some(&"32px".to_string())
        );
        assert_eq!(
            config.get_component_token("Button", "padding"),
            Some(&"0 15px".to_string())
        );
        assert_eq!(config.get_component_token("Button", "color"), None);
        assert_eq!(config.get_component_token("Input", "height"), None);
    }

    #[test]
    fn test_theme_config_merge() {
        let mut base_token = HashMap::new();
        base_token.insert("primary".to_string(), "#1890ff".to_string());
        base_token.insert("success".to_string(), "#52c41a".to_string());

        let mut base_button = HashMap::new();
        base_button.insert("height".to_string(), "32px".to_string());
        base_button.insert("padding".to_string(), "0 15px".to_string());

        let mut base_components = HashMap::new();
        base_components.insert("Button".to_string(), base_button);

        let mut base = ThemeConfig::default()
            .with_token(base_token)
            .with_components(base_components);

        let mut override_token = HashMap::new();
        override_token.insert("primary".to_string(), "#722ed1".to_string());
        override_token.insert("warning".to_string(), "#faad14".to_string());

        let mut override_button = HashMap::new();
        override_button.insert("height".to_string(), "40px".to_string());
        override_button.insert("borderRadius".to_string(), "4px".to_string());

        let mut override_input = HashMap::new();
        override_input.insert("height".to_string(), "40px".to_string());

        let mut override_components = HashMap::new();
        override_components.insert("Button".to_string(), override_button);
        override_components.insert("Input".to_string(), override_input);

        let override_config = ThemeConfig::default()
            .with_compact(true)
            .with_token(override_token)
            .with_components(override_components);

        base.merge(&override_config);

        assert!(base.compact);
        assert_eq!(base.get_token("primary"), Some(&"#722ed1".to_string()));
        assert_eq!(base.get_token("success"), Some(&"#52c41a".to_string()));
        assert_eq!(base.get_token("warning"), Some(&"#faad14".to_string()));
        assert_eq!(
            base.get_component_token("Button", "height"),
            Some(&"40px".to_string())
        );
        assert_eq!(
            base.get_component_token("Button", "padding"),
            Some(&"0 15px".to_string())
        );
        assert_eq!(
            base.get_component_token("Button", "borderRadius"),
            Some(&"4px".to_string())
        );
        assert_eq!(
            base.get_component_token("Input", "height"),
            Some(&"40px".to_string())
        );
    }

    #[test]
    fn test_theme_builder() {
        let config = ThemeBuilder::new()
            .compact(true)
            .add_token("primary", "#1890ff")
            .add_token("success", "#52c41a")
            .add_component_token("Button", "height", "32px")
            .add_component_token("Button", "padding", "0 15px")
            .add_component_token("Input", "height", "32px")
            .build();

        assert!(config.compact);
        assert_eq!(config.get_token("primary"), Some(&"#1890ff".to_string()));
        assert_eq!(config.get_token("success"), Some(&"#52c41a".to_string()));
        assert_eq!(
            config.get_component_token("Button", "height"),
            Some(&"32px".to_string())
        );
        assert_eq!(
            config.get_component_token("Button", "padding"),
            Some(&"0 15px".to_string())
        );
        assert_eq!(
            config.get_component_token("Input", "height"),
            Some(&"32px".to_string())
        );
    }
}

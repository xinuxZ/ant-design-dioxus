//! 主题配置核心抽象
//!
//! 提供主题配置的基础接口和抽象，包括：
//! - 主题配置 trait
//! - 设计令牌接口
//! - CSS 生成接口

use super::{ColorType, Duration, Easing, RgbColor, Size, SpaceSize};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 设计令牌 trait
///
/// 定义设计令牌的基础接口
pub trait DesignTokens {
    /// 获取颜色令牌
    fn get_color(&self, color_type: ColorType) -> Option<RgbColor>;

    /// 获取尺寸令牌
    fn get_size(&self, size: Size) -> Option<u32>;

    /// 获取间距令牌
    fn get_space(&self, space: SpaceSize) -> Option<u32>;

    /// 获取动画持续时间令牌
    fn get_duration(&self, duration: Duration) -> Option<u32>;

    /// 获取缓动函数令牌
    fn get_easing(&self, easing: &Easing) -> Option<String>;

    /// 获取所有令牌的 CSS 变量映射
    fn to_css_variables(&self) -> HashMap<String, String>;
}

/// 主题配置 trait
///
/// 定义主题配置的基础接口
pub trait ThemeConfig: DesignTokens {
    /// 获取主题名称
    fn name(&self) -> &str;

    /// 获取主题变体
    fn variant(&self) -> &str;

    /// 是否为深色主题
    fn is_dark(&self) -> bool;

    /// 生成完整的 CSS 样式
    fn generate_css(&self) -> String;

    /// 生成 CSS 变量定义
    fn generate_css_variables(&self) -> HashMap<String, String> {
        let variables = self.to_css_variables();
        // let mut css = String::new();

        variables
        // css.push_str(":root {\n");
        // for (key, value) in variables {
        //     css.push_str(&format!("  {}: {};\n", key, value));
        // }
        // css.push_str("}\n");

        // css
    }
}

/// CSS 生成器 trait
///
/// 定义 CSS 生成的接口
pub trait CssGenerator {
    /// 生成组件样式
    fn generate_component_css(&self, component: &str) -> String;

    /// 生成全局样式
    fn generate_global_css(&self) -> String;

    /// 生成主题样式
    fn generate_theme_css(&self, theme_name: &str) -> String;
}

/// 主题提供者 trait
///
/// 定义主题提供者的接口
pub trait ThemeProvider {
    type Theme: ThemeConfig;

    /// 获取当前主题
    fn current_theme(&self) -> &Self::Theme;

    /// 切换主题
    fn switch_theme(&mut self, theme: Self::Theme);

    /// 获取所有可用主题
    fn available_themes(&self) -> Vec<&Self::Theme>;

    /// 根据名称获取主题
    fn get_theme_by_name(&self, name: &str) -> Option<&Self::Theme>;
}

/// 基础主题配置结构
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BaseThemeConfig {
    /// 主题名称
    pub name: String,
    /// 主题变体
    pub variant: String,
    /// 是否为深色主题
    pub is_dark: bool,
    /// 自定义属性
    pub custom_properties: HashMap<String, String>,
}

impl BaseThemeConfig {
    /// 创建新的基础主题配置
    pub fn new(name: String, variant: String, is_dark: bool) -> Self {
        Self {
            name,
            variant,
            is_dark,
            custom_properties: HashMap::new(),
        }
    }

    /// 添加自定义属性
    pub fn with_property(mut self, key: String, value: String) -> Self {
        self.custom_properties.insert(key, value);
        self
    }

    /// 获取自定义属性
    pub fn get_property(&self, key: &str) -> Option<&String> {
        self.custom_properties.get(key)
    }
}

/// 令牌解析器 trait
///
/// 定义令牌解析的接口
pub trait TokenResolver {
    /// 解析颜色令牌
    fn resolve_color_token(&self, token: &str) -> Option<RgbColor>;

    /// 解析尺寸令牌
    fn resolve_size_token(&self, token: &str) -> Option<u32>;

    /// 解析间距令牌
    fn resolve_space_token(&self, token: &str) -> Option<u32>;

    /// 解析动画令牌
    fn resolve_animation_token(&self, token: &str) -> Option<String>;
}

/// 主题变更监听器 trait
pub trait ThemeChangeListener {
    /// 主题变更时的回调
    fn on_theme_changed(&self, old_theme: &str, new_theme: &str);
}

/// 主题验证器 trait
pub trait ThemeValidator {
    /// 验证主题配置
    fn validate_theme<T: ThemeConfig>(&self, theme: &T) -> Result<(), Vec<String>>;

    /// 验证颜色对比度
    fn validate_color_contrast(&self, foreground: &RgbColor, background: &RgbColor) -> bool;

    /// 验证可访问性
    fn validate_accessibility<T: ThemeConfig>(&self, theme: &T) -> Result<(), Vec<String>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_theme_config() {
        let config = BaseThemeConfig::new("test".to_string(), "light".to_string(), false);

        assert_eq!(config.name, "test");
        assert_eq!(config.variant, "light");
        assert!(!config.is_dark);
    }

    #[test]
    fn test_base_theme_config_with_property() {
        let config = BaseThemeConfig::new("test".to_string(), "light".to_string(), false)
            .with_property("custom".to_string(), "value".to_string());

        assert_eq!(config.get_property("custom"), Some(&"value".to_string()));
        assert_eq!(config.get_property("nonexistent"), None);
    }
}

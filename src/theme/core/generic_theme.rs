//! 通用主题引擎
//!
//! 提供基于泛型的主题引擎实现，支持不同的颜色配置系统

use super::{
    config::{DesignTokens, ThemeConfigInterface},
    motion::{AnimationConfig, Duration},
    types::{Size, SpaceSize},
    ColorConfig, ColorPalette, ColorType, Easing, RgbColor,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::marker::PhantomData;

/// 通用主题引擎
///
/// 使用泛型参数和 trait bound 支持不同的颜色配置实现
#[derive(Debug, Clone, PartialEq)]
pub struct GenericThemeEngine<C>
where
    C: ColorConfig + Clone + PartialEq,
{
    /// 主题名称
    pub name: String,
    /// 是否为暗色主题
    pub dark: bool,
    /// 是否为紧凑主题
    pub compact: bool,
    /// 颜色配置
    pub colors: C,
    /// 字体配置类型
    pub typography_type: String,
    /// 间距配置类型
    pub spacing_type: String,
    /// 尺寸配置类型
    pub sizing_type: String,
    /// 动画配置类型
    pub animations_type: String,
    /// 自定义属性
    pub properties: HashMap<String, String>,
}

impl<C> GenericThemeEngine<C>
where
    C: ColorConfig + Clone + PartialEq,
{
    /// 创建新的主题引擎
    pub fn new(
        name: String,
        dark: bool,
        compact: bool,
        colors: C,
        typography_type: String,
        spacing_type: String,
        sizing_type: String,
        animations_type: String,
    ) -> Self {
        Self {
            name,
            dark,
            compact,
            colors,
            typography_type,
            spacing_type,
            sizing_type,
            animations_type,
            properties: HashMap::new(),
        }
    }

    /// 创建浅色主题
    pub fn light(
        name: String,
        typography_type: String,
        spacing_type: String,
        sizing_type: String,
        animations_type: String,
    ) -> Self {
        Self::new(
            name,
            false,
            false,
            C::light(),
            typography_type,
            spacing_type,
            sizing_type,
            animations_type,
        )
    }

    /// 创建深色主题
    pub fn dark(
        name: String,
        typography_type: String,
        spacing_type: String,
        sizing_type: String,
        animations_type: String,
    ) -> Self {
        Self::new(
            name,
            true,
            false,
            C::dark(),
            typography_type,
            spacing_type,
            sizing_type,
            animations_type,
        )
    }

    /// 创建紧凑主题
    pub fn compact(
        name: String,
        dark: bool,
        typography_type: String,
        spacing_type: String,
        sizing_type: String,
        animations_type: String,
    ) -> Self {
        let colors = if dark { C::dark() } else { C::light() };
        Self::new(
            name,
            dark,
            true,
            colors,
            typography_type,
            spacing_type,
            sizing_type,
            animations_type,
        )
    }

    /// 设置自定义属性
    pub fn with_property(mut self, key: &str, value: &str) -> Self {
        self.properties.insert(key.to_string(), value.to_string());
        self
    }

    /// 获取自定义属性
    pub fn get_property(&self, key: &str) -> Option<&String> {
        self.properties.get(key)
    }

    /// 获取颜色配置
    pub fn get_color(&self, color_type: ColorType) -> Option<RgbColor> {
        self.colors.get_color(color_type)
    }

    /// 获取颜色调色板
    pub fn get_palette(&self, color_type: ColorType) -> Option<&ColorPalette> {
        self.colors.get_palette(color_type)
    }

    /// 是否为深色主题
    pub fn is_dark_theme(&self) -> bool {
        self.dark
    }

    /// 获取主题名称
    pub fn get_theme_name(&self) -> &str {
        &self.name
    }
}

impl<C> DesignTokens for GenericThemeEngine<C>
where
    C: ColorConfig + Clone + PartialEq,
{
    fn get_color(&self, color_type: ColorType) -> Option<RgbColor> {
        self.colors.get_color(color_type)
    }

    fn get_size(&self, _size: Size) -> Option<u32> {
        // 这里需要实现尺寸获取逻辑
        // 由于我们没有实现通用的尺寸系统，暂时返回默认值
        Some(8)
    }

    fn get_space(&self, space: SpaceSize) -> Option<u32> {
        // 这里需要实现间距获取逻辑
        // 由于我们没有实现通用的间距系统，暂时返回默认值
        match space {
            SpaceSize::XSmall => Some(4),
            SpaceSize::Small => Some(8),
            SpaceSize::Medium => Some(16),
            SpaceSize::Large => Some(24),
            SpaceSize::XLarge => Some(32),
            SpaceSize::XXLarge => Some(48),
        }
    }

    fn get_duration(&self, duration: Duration) -> Option<u32> {
        // 这里需要实现动画持续时间获取逻辑
        // 由于我们没有实现通用的动画系统，暂时返回默认值
        match duration {
            Duration::Shortest => Some(100),
            Duration::Shorter => Some(200),
            Duration::Short => Some(300),
            Duration::Standard => Some(400),
            Duration::Long => Some(500),
            Duration::Longer => Some(600),
            Duration::Longest => Some(800),
        }
    }

    fn get_easing(&self, easing: &Easing) -> Option<String> {
        // 这里需要实现缓动函数获取逻辑
        // 由于我们没有实现通用的动画系统，暂时返回默认值
        match easing {
            Easing::Linear => Some("linear".to_string()),
            Easing::EaseIn => Some("cubic-bezier(0.4, 0, 1, 1)".to_string()),
            Easing::EaseOut => Some("cubic-bezier(0, 0, 0.2, 1)".to_string()),
            Easing::EaseInOut => Some("cubic-bezier(0.4, 0, 0.2, 1)".to_string()),
            Easing::Sharp => Some("cubic-bezier(0.4, 0, 0.6, 1)".to_string()),
            Easing::Custom(_) => Some("cubic-bezier(0.4, 0, 0.2, 1)".to_string()),
        }
    }

    fn to_css_variables(&self) -> HashMap<String, String> {
        // 合并各个子模块的 CSS 变量
        let mut variables = HashMap::new();

        // 添加颜色变量
        variables.extend(self.colors.to_css_variables());

        // 添加主题级别的变量
        variables.insert("--theme-name".to_string(), self.name.clone());
        variables.insert(
            "--theme-dark".to_string(),
            if self.dark { "1" } else { "0" }.to_string(),
        );
        variables.insert(
            "--theme-compact".to_string(),
            if self.compact { "1" } else { "0" }.to_string(),
        );

        // 添加自定义属性
        for (key, value) in &self.properties {
            variables.insert(format!("--theme-{}", key), value.clone());
        }

        variables
    }
}

impl<C> ThemeConfigInterface for GenericThemeEngine<C>
where
    C: ColorConfig + Clone + PartialEq,
{
    fn name(&self) -> &str {
        &self.name
    }

    fn variant(&self) -> &str {
        if self.dark {
            "dark"
        } else {
            "light"
        }
    }

    fn is_dark(&self) -> bool {
        self.dark
    }

    fn generate_css_variables(&self) -> HashMap<String, String> {
        self.to_css_variables()
    }

    fn generate_css(&self) -> String {
        let mut css = String::new();

        // 生成 CSS 变量
        let variables: HashMap<String, String> = self.generate_css_variables();
        css.push_str(":root {\n");
        for (key, value) in variables {
            css.push_str(&format!("  {}: {};\n", key, value));
        }
        css.push_str("}\n\n");

        // 生成颜色样式
        css.push_str(&self.colors.generate_css());

        css
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::ant_design::AntDesignColors;

    #[test]
    fn test_generic_theme_with_ant_design_colors() {
        let theme = GenericThemeEngine::new(
            "Test Theme".to_string(),
            false,
            false,
            AntDesignColors::light(),
            "default".to_string(),
            "default".to_string(),
            "default".to_string(),
            "default".to_string(),
        );

        assert_eq!(theme.name, "Test Theme");
        assert_eq!(theme.dark, false);
        assert!(theme.get_color(ColorType::Primary).is_some());

        let css = theme.generate_css();
        assert!(css.contains(":root"));
        assert!(css.contains("--ant-primary-color"));
    }

    #[test]
    fn test_theme_properties() {
        let theme = GenericThemeEngine::new(
            "Test Theme".to_string(),
            false,
            false,
            AntDesignColors::light(),
            "default".to_string(),
            "default".to_string(),
            "default".to_string(),
            "default".to_string(),
        )
        .with_property("custom-prop", "custom-value");

        assert_eq!(
            theme.get_property("custom-prop"),
            Some(&"custom-value".to_string())
        );

        let variables = theme.to_css_variables();
        assert_eq!(
            variables.get("--theme-custom-prop"),
            Some(&"custom-value".to_string())
        );
    }
}

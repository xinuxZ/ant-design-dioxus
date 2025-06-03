//! Ant Design 主题实现
//!
//! 实现 Ant Design 设计语言的主题系统，包括：
//! - 颜色系统
//! - 字体排版
//! - 间距系统
//! - 尺寸规范
//! - 动画效果
use crate::theme::core::{
    config::{DesignTokens, ThemeConfig},
    motion::{AnimationConfig, Duration},
    types::{Size, SpaceSize},
    ColorType, Easing, RgbColor,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// 子模块
pub mod animations;
pub mod colors;
pub mod sizing;
pub mod spacing;
pub mod typography;

// 重新导出核心类型
pub use animations::AntDesignAnimations;
pub use colors::AntDesignColors;
pub use sizing::AntDesignSizing;
pub use spacing::AntDesignSpacing;
pub use typography::AntDesignTypography;

/// Ant Design 主题引擎
///
/// 职责：主题引擎层，包含完整的设计系统组件
/// - 实现复杂的CSS生成和样式计算
/// - 管理typography、spacing、sizing、animations等子系统
/// - 提供高级的主题操作API
/// - 实现DesignTokens和ThemeConfig traits
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AntDesignThemeEngine {
    /// 主题名称
    pub name: String,
    /// 是否为暗色主题
    pub dark: bool,
    /// 是否为紧凑主题
    pub compact: bool,
    /// 颜色配置
    pub colors: AntDesignColors,
    /// 字体配置
    pub typography: AntDesignTypography,
    /// 间距配置
    pub spacing: AntDesignSpacing,
    /// 尺寸配置
    pub sizing: AntDesignSizing,
    /// 动画配置
    pub animations: AntDesignAnimations,
}

impl AntDesignThemeEngine {
    /// 创建浅色主题
    pub fn light() -> Self {
        Self {
            name: "Ant Design Light".to_string(),
            dark: false,
            compact: false,
            colors: AntDesignColors::light(),
            typography: AntDesignTypography::default(),
            spacing: AntDesignSpacing::default(),
            sizing: AntDesignSizing::default(),
            animations: AntDesignAnimations::default(),
        }
    }

    /// 创建深色主题
    pub fn dark() -> Self {
        Self {
            name: "Ant Design Dark".to_string(),
            dark: true,
            compact: false,
            colors: AntDesignColors::dark(),
            typography: AntDesignTypography::default(),
            spacing: AntDesignSpacing::default(),
            sizing: AntDesignSizing::default(),
            animations: AntDesignAnimations::default(),
        }
    }

    /// 创建紧凑主题
    pub fn compact() -> Self {
        Self {
            name: "Ant Design Compact".to_string(),
            dark: false,
            compact: true,
            colors: AntDesignColors::light(),
            typography: AntDesignTypography::compact(),
            spacing: AntDesignSpacing::compact(),
            sizing: AntDesignSizing::compact(),
            animations: AntDesignAnimations::default(),
        }
    }

    /// 获取颜色配置
    pub fn get_color(&self, color_type: ColorType) -> Option<RgbColor> {
        self.colors.get_color(color_type)
    }

    /// 获取间距值
    pub fn get_spacing(&self, space: SpaceSize) -> u32 {
        self.spacing.get_space(space).unwrap()
    }

    /// 获取尺寸值
    pub fn get_size(&self, size: Size) -> Option<u32> {
        self.sizing.get_size(size)
    }

    /// 获取动画预设
    pub fn get_animation(&self, name: &str) -> Option<&AnimationConfig> {
        self.animations.get_preset(name)
    }

    pub fn is_dark_theme(&self) -> bool {
        self.dark
    }

    pub fn get_theme_name(&self) -> &str {
        &self.name
    }
}

impl DesignTokens for AntDesignThemeEngine {
    fn get_color(&self, color_type: ColorType) -> Option<RgbColor> {
        self.colors.get_color(color_type)
    }

    fn get_size(&self, size: Size) -> Option<u32> {
        Some(self.sizing.get_border_radius(size))
    }

    fn get_space(&self, space: SpaceSize) -> Option<u32> {
        self.spacing.get_space(space)
    }

    fn get_duration(&self, duration: Duration) -> Option<u32> {
        self.animations.get_duration(&duration)
    }

    fn get_easing(&self, easing: &Easing) -> Option<String> {
        self.animations.get_easing(easing)
    }

    fn to_css_variables(&self) -> HashMap<String, String> {
        let mut variables = HashMap::new();

        // 合并各个子模块的 CSS 变量
        variables.extend(self.colors.to_css_variables());
        variables.extend(self.typography.to_css_variables());
        variables.extend(self.spacing.to_css_variables());
        variables.extend(self.sizing.to_css_variables());
        variables.extend(self.animations.to_css_variables());

        variables
    }
}

impl ThemeConfig for AntDesignThemeEngine {
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
        let mut variables = HashMap::new();

        // 合并各个子模块的 CSS 变量
        variables.extend(self.colors.to_css_variables());
        variables.extend(self.typography.to_css_variables());
        variables.extend(self.spacing.to_css_variables());
        variables.extend(self.sizing.to_css_variables());
        variables.extend(self.animations.to_css_variables());

        // 添加主题级别的变量
        variables.insert("--ant-theme-name".to_string(), self.name.clone());
        variables.insert(
            "--ant-theme-dark".to_string(),
            if self.dark { "1" } else { "0" }.to_string(),
        );
        variables.insert(
            "--ant-theme-compact".to_string(),
            if self.compact { "1" } else { "0" }.to_string(),
        );

        variables
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

        // 生成组件样式
        css.push_str(&self.colors.generate_css());
        css.push_str(&self.typography.generate_css());
        css.push_str(&self.spacing.generate_css());
        css.push_str(&self.sizing.generate_css());
        css.push_str(&self.animations.generate_css_classes());
        css.push_str(&self.animations.generate_keyframes());

        css
    }
}

/// Ant Design 主题引擎工厂
///
/// 职责：创建和管理主题引擎实例
/// - 提供完整的主题引擎预设
/// - 支持复杂的自定义主题创建
/// - 管理主题引擎的生命周期
pub struct AntDesignThemeEngineFactory;

impl AntDesignThemeEngineFactory {
    /// 获取所有预设主题
    pub fn all() -> Vec<AntDesignThemeEngine> {
        vec![
            AntDesignThemeEngine::light(),
            AntDesignThemeEngine::dark(),
            AntDesignThemeEngine::compact(),
        ]
    }

    /// 获取默认主题（浅色）
    pub fn default() -> AntDesignThemeEngine {
        AntDesignThemeEngine::light()
    }

    /// 根据名称获取主题
    pub fn get_by_name(name: &str) -> Option<AntDesignThemeEngine> {
        match name {
            "light" => Some(AntDesignThemeEngine::light()),
            "dark" => Some(AntDesignThemeEngine::dark()),
            "compact" => Some(AntDesignThemeEngine::compact()),
            _ => None,
        }
    }

    /// 获取主题名称列表
    pub fn get_theme_names() -> Vec<&'static str> {
        vec!["light", "dark", "compact"]
    }

    /// 创建自定义主题
    pub fn create_custom_theme(
        name: String,
        dark: bool,
        compact: bool,
        colors: Option<AntDesignColors>,
        typography: Option<AntDesignTypography>,
        spacing: Option<AntDesignSpacing>,
        sizing: Option<AntDesignSizing>,
        animations: Option<AntDesignAnimations>,
    ) -> AntDesignThemeEngine {
        AntDesignThemeEngine {
            name,
            dark,
            compact,
            colors: colors.unwrap_or_else(|| {
                if dark {
                    AntDesignColors::dark()
                } else {
                    AntDesignColors::light()
                }
            }),
            typography: typography.unwrap_or_else(|| {
                if compact {
                    AntDesignTypography::compact()
                } else {
                    AntDesignTypography::default()
                }
            }),
            spacing: spacing.unwrap_or_else(|| {
                if compact {
                    AntDesignSpacing::compact()
                } else {
                    AntDesignSpacing::default()
                }
            }),
            sizing: sizing.unwrap_or_else(|| {
                if compact {
                    AntDesignSizing::compact()
                } else {
                    AntDesignSizing::default()
                }
            }),
            animations: animations.unwrap_or_else(AntDesignAnimations::default),
        }
    }
}

impl Default for AntDesignThemeEngine {
    fn default() -> Self {
        Self::light()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::core::ColorType;

    #[test]
    fn test_ant_design_theme_creation() {
        let light = AntDesignThemeEngine::light();
        assert_eq!(light.get_theme_name(), "Ant Design Light");
        assert!(!light.is_dark_theme());

        let dark = AntDesignThemeEngine::dark();
        assert_eq!(dark.get_theme_name(), "Ant Design Dark");
        assert!(dark.is_dark_theme());

        let compact = AntDesignThemeEngine::compact();
        assert_eq!(compact.get_theme_name(), "Ant Design Compact");
        assert!(!compact.is_dark_theme());
    }

    #[test]
    fn test_theme_presets() {
        let all_themes = AntDesignThemeEngineFactory::all();
        assert_eq!(all_themes.len(), 3);

        let default_theme = AntDesignThemeEngineFactory::default();
        assert!(!default_theme.is_dark_theme());

        let light_theme = AntDesignThemeEngineFactory::get_by_name("light").unwrap();
        assert!(!light_theme.is_dark_theme());

        assert!(AntDesignThemeEngineFactory::get_by_name("nonexistent").is_none());
    }

    #[test]
    fn test_design_tokens() {
        let theme = AntDesignThemeEngine::light();

        // 测试颜色令牌
        assert!(theme.get_color(ColorType::Primary).is_some());

        // 测试尺寸令牌
        assert!(theme.get_size(Size::Middle).is_some());

        // 测试 CSS 变量生成
        let variables = theme.generate_css_variables();
        assert!(!variables.is_empty());
    }

    #[test]
    fn test_css_generation() {
        let theme = AntDesignThemeEngine::light();
        let css = theme.generate_css();

        assert!(css.contains(":root"));
        assert!(!css.is_empty());
    }

    #[test]
    fn test_custom_theme() {
        let custom = AntDesignThemeEngineFactory::create_custom_theme(
            "Custom Theme".to_string(),
            false,
            true,
            None,
            None,
            None,
            None,
            None,
        );

        assert_eq!(custom.name, "Custom Theme");
        assert!(!custom.dark);
        assert!(custom.compact);
    }

    #[test]
    fn test_design_tokens_trait() {
        let theme = AntDesignThemeEngine::light();

        // 测试 DesignTokens trait 方法
        assert!(theme.get_color(ColorType::Primary).is_some());
        assert_eq!(theme.get_spacing(SpaceSize::Middle), 16);
        assert!(theme.get_size(Size::Middle) > Some(0));
        // assert!(theme.get_border_radius(Size::Middle) > 0);
    }

    #[test]
    fn test_theme_config_trait() {
        let theme = AntDesignThemeEngine::light();

        // 测试 ThemeConfig trait 方法
        let variables: HashMap<String, String> = theme.generate_css_variables();
        assert!(!variables.is_empty());

        let css = theme.generate_css();
        assert!(css.contains(":root"));

        assert!(!theme.is_dark_theme());
        assert_eq!(theme.get_theme_name(), "Ant Design Light");
    }
}

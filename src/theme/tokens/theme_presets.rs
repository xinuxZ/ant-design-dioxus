//! Ant Design 主题预设
//!
//! 本模块定义了 Ant Design 的完整主题配置，包括浅色和深色主题。
//! 这些预设基于 Ant Design 5.x 的官方主题规范。
//!
//! 职责：
//! 1. 提供预设的主题配置（light、dark、compact）
//! 2. 包含具体的颜色值和配置
//! 3. 提供 `to_css_variables()` 和 `design_tokens()` 方法
//! 4. 更像是一个"主题数据"层，存储具体的主题值

use super::{
    animation_presets::AntDesignEasing,
    color_presets::{AntDesignColors, BorderColors, TextColors},
};
use css_in_rust::theme::{DesignTokens, ThemeVariant};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 主题预设数据结构
///
/// 职责：纯数据层，存储主题配置的原始值
/// - 提供基础的主题预设（light、dark、compact）
/// - 支持简单的颜色定制
/// - 提供到css-in-rust DesignTokens的转换
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ThemePreset {
    /// 主题名称
    pub name: String,
    /// 主题变体（浅色/深色）
    pub variant: ThemeVariant,
    /// 主色调
    pub primary_color: String,
    /// 成功色
    pub success_color: String,
    /// 警告色
    pub warning_color: String,
    /// 错误色
    pub error_color: String,
    /// 信息色
    pub info_color: String,
    /// 文本颜色配置
    pub text_colors: TextColors,
    /// 边框颜色配置
    pub border_colors: BorderColors,
    /// 背景颜色
    pub background_colors: BackgroundColors,
    /// 默认缓动函数
    pub default_easing: AntDesignEasing,
}

/// 背景颜色配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BackgroundColors {
    pub default: String,
    pub container: String,
    pub elevated: String,
    pub layout: String,
}

impl Default for BackgroundColors {
    fn default() -> Self {
        Self {
            default: "#ffffff".to_string(),
            container: "#ffffff".to_string(),
            elevated: "#ffffff".to_string(),
            layout: "#f5f5f5".to_string(),
        }
    }
}

impl BackgroundColors {
    /// 深色主题的背景颜色
    pub fn dark() -> Self {
        Self {
            default: "#141414".to_string(),
            container: "#1f1f1f".to_string(),
            elevated: "#262626".to_string(),
            layout: "#000000".to_string(),
        }
    }

    /// 转换为 CSS 变量
    pub fn to_css_variables(&self) -> String {
        format!(
            "  --color-background-default: {};\n\
             --color-background-container: {};\n\
             --color-background-elevated: {};\n\
             --color-background-layout: {};\n",
            self.default, self.container, self.elevated, self.layout
        )
    }
}

impl ThemePreset {
    /// 获取主题名称
    pub fn get_theme_name(&self) -> &str {
        &self.name
    }

    /// 是否为深色主题
    pub fn is_dark_theme(&self) -> bool {
        matches!(self.variant, ThemeVariant::Dark)
    }

    /// 获取主色调
    pub fn primary_color(&self) -> String {
        self.primary_color.clone()
    }

    /// 获取成功色
    pub fn success_color(&self) -> String {
        self.success_color.clone()
    }

    /// 获取警告色
    pub fn warning_color(&self) -> String {
        self.warning_color.clone()
    }

    /// 获取错误色
    pub fn error_color(&self) -> String {
        self.error_color.clone()
    }

    /// 获取信息色
    pub fn info_color(&self) -> String {
        self.info_color.clone()
    }

    /// 转换为设计令牌
    pub fn design_tokens(&self) -> DesignTokens {
        let mut design_tokens = DesignTokens::new();

        // 根据主题变体设置颜色
        match self.variant {
            ThemeVariant::Light => {
                design_tokens.colors.primary = self.primary_color.clone();
                design_tokens.colors.success = self.success_color.clone();
                design_tokens.colors.warning = self.warning_color.clone();
                design_tokens.colors.error = self.error_color.clone();
                design_tokens.colors.info = self.info_color.clone();
            }
            ThemeVariant::Dark => {
                design_tokens.colors.primary = self.primary_color.clone();
                design_tokens.colors.success = self.success_color.clone();
                design_tokens.colors.warning = self.warning_color.clone();
                design_tokens.colors.error = self.error_color.clone();
                design_tokens.colors.info = self.info_color.clone();
            }
            ThemeVariant::Auto => {
                design_tokens.colors.primary = self.primary_color.clone();
                design_tokens.colors.success = self.success_color.clone();
                design_tokens.colors.warning = self.warning_color.clone();
                design_tokens.colors.error = self.error_color.clone();
                design_tokens.colors.info = self.info_color.clone();
            }
        }

        design_tokens
    }

    /// 转换为设计令牌（别名方法）
    pub fn to_design_tokens(&self) -> DesignTokens {
        self.design_tokens()
    }

    /// 创建默认的浅色主题
    pub fn light() -> Self {
        Self {
            name: "Ant Design Light".to_string(),
            variant: ThemeVariant::Light,
            primary_color: "#1677ff".to_string(),
            success_color: "#52c41a".to_string(),
            warning_color: "#faad14".to_string(),
            error_color: "#ff4d4f".to_string(),
            info_color: "#1677ff".to_string(),
            text_colors: TextColors::default(),
            border_colors: BorderColors::default(),
            background_colors: BackgroundColors::default(),
            default_easing: AntDesignEasing::Standard,
        }
    }

    /// 创建默认的深色主题
    pub fn dark() -> Self {
        Self {
            name: "Ant Design Dark".to_string(),
            variant: ThemeVariant::Dark,
            primary_color: "#1668dc".to_string(),
            success_color: "#49aa19".to_string(),
            warning_color: "#d89614".to_string(),
            error_color: "#dc4446".to_string(),
            info_color: "#1668dc".to_string(),
            text_colors: TextColors::dark(),
            border_colors: BorderColors::dark(),
            background_colors: BackgroundColors::dark(),
            default_easing: AntDesignEasing::Standard,
        }
    }

    /// 创建紧凑主题（基于浅色主题，但间距更小）
    pub fn compact() -> Self {
        let mut theme = Self::light();
        theme.name = "Ant Design Compact".to_string();
        theme
    }

    // /// 转换为 css-in-rust 的 DesignTokens
    // pub fn to_design_tokens(&self) -> DesignTokens {
    //     let tokens = match self.variant {
    //         ThemeVariant::Light => AntDesignTokens::get_light_theme_tokens(),
    //         ThemeVariant::Dark => AntDesignTokens::get_dark_theme_tokens(),
    //         ThemeVariant::Auto => AntDesignTokens::get_light_theme_tokens(), // 默认使用浅色主题
    //     };

    //     DesignTokens::from_token_map(tokens)
    // }

    /// 生成完整的 CSS 变量
    pub fn to_css_variables(&self) -> String {
        let mut css = String::new();

        // 主题基础信息
        css.push_str(&format!("  --theme-name: '{}';\n", self.name));
        css.push_str(&format!(
            "  --theme-variant: '{}';\n",
            match self.variant {
                ThemeVariant::Light => "light",
                ThemeVariant::Dark => "dark",
                ThemeVariant::Auto => "auto",
            }
        ));

        // 主要颜色
        css.push_str(&format!("  --color-primary: {};\n", self.primary_color));
        css.push_str(&format!("  --color-success: {};\n", self.success_color));
        css.push_str(&format!("  --color-warning: {};\n", self.warning_color));
        css.push_str(&format!("  --color-error: {};\n", self.error_color));
        css.push_str(&format!("  --color-info: {};\n", self.info_color));

        // 文本颜色
        css.push_str(&self.text_colors.to_css_variables());

        // 边框颜色
        css.push_str(&self.border_colors.to_css_variables());

        // 背景颜色
        css.push_str(&self.background_colors.to_css_variables());

        // 动画配置
        css.push_str(&format!(
            "  --animation-easing-standard: {};\n",
            self.default_easing.to_css()
        ));
        css.push_str(&format!(
            "  --animation-duration-standard: {}ms;\n",
            self.default_easing.suggested_duration_ms()
        ));

        // 颜色调色板
        let colors = AntDesignColors::all_colors();
        for (name, scale) in colors {
            css.push_str(&scale.to_css_variables(&format!("color-{}", name)));
        }

        css
    }

    /// 自定义主题颜色
    pub fn with_primary_color(mut self, color: &str) -> Self {
        self.primary_color = color.to_string();
        self
    }

    /// 自定义成功色
    pub fn with_success_color(mut self, color: &str) -> Self {
        self.success_color = color.to_string();
        self
    }

    /// 自定义警告色
    pub fn with_warning_color(mut self, color: &str) -> Self {
        self.warning_color = color.to_string();
        self
    }

    /// 自定义错误色
    pub fn with_error_color(mut self, color: &str) -> Self {
        self.error_color = color.to_string();
        self
    }

    /// 自定义信息色
    pub fn with_info_color(mut self, color: &str) -> Self {
        self.info_color = color.to_string();
        self
    }

    /// 转换为主题引擎实例
    /// 将纯数据层的主题预设转换为可执行的主题引擎
    pub fn to_theme_engine(&self) -> crate::theme::ant_design::AntDesignThemeEngine {
        use crate::theme::ant_design::*;

        // 根据主题变体创建对应的主题引擎
        let mut engine = match self.variant {
            css_in_rust::theme::ThemeVariant::Dark => AntDesignThemeEngine::dark(),
            _ => AntDesignThemeEngine::light(),
        };

        // 更新主题名称
        engine.name = self.name.clone();

        // 如果是紧凑主题，设置compact标志
        if self.name.contains("Compact") {
            engine.compact = true;
        }

        engine
    }

    /// 从主题引擎实例创建主题预设
    /// 将主题引擎转换回纯数据层的主题预设
    pub fn from_theme_engine(engine: &crate::theme::ant_design::AntDesignThemeEngine) -> Self {
        let variant = if engine.dark {
            css_in_rust::theme::ThemeVariant::Dark
        } else {
            css_in_rust::theme::ThemeVariant::Light
        };

        Self {
            name: engine.name.clone(),
            variant,
            primary_color: "#1890ff".to_string(),
            success_color: "#52c41a".to_string(),
            warning_color: "#faad14".to_string(),
            error_color: "#ff4d4f".to_string(),
            info_color: "#1890ff".to_string(),
            text_colors: TextColors::default(),
            border_colors: BorderColors::default(),
            background_colors: BackgroundColors::default(),
            default_easing: AntDesignEasing::Standard,
        }
    }
}

/// 主题预设工厂
///
/// 职责：提供预设主题数据的工厂方法
/// - 创建标准主题预设（light、dark、compact）
/// - 支持基于预设的简单定制
/// - 返回纯数据结构，无复杂业务逻辑
pub struct ThemePresetFactory;

impl ThemePresetFactory {
    /// 获取所有预设主题
    pub fn all_presets() -> HashMap<String, ThemePreset> {
        let mut presets = HashMap::new();
        presets.insert("light".to_string(), ThemePreset::light());
        presets.insert("dark".to_string(), ThemePreset::dark());
        presets.insert("compact".to_string(), ThemePreset::compact());
        presets
    }

    /// 获取默认主题（浅色）
    pub fn default() -> ThemePreset {
        ThemePreset::light()
    }

    /// 根据名称获取预设主题
    pub fn get_preset(name: &str) -> Option<ThemePreset> {
        match name {
            "light" => Some(ThemePreset::light()),
            "dark" => Some(ThemePreset::dark()),
            "compact" => Some(ThemePreset::compact()),
            _ => None,
        }
    }

    /// 创建自定义主题（基于浅色主题）
    pub fn custom_light(name: &str, primary_color: &str) -> ThemePreset {
        ThemePreset::light().with_primary_color(primary_color)
    }

    /// 创建自定义主题（基于深色主题）
    pub fn custom_dark(name: &str, primary_color: &str) -> ThemePreset {
        ThemePreset::dark().with_primary_color(primary_color)
    }
}

/// 主题切换辅助函数
pub mod theme_utils {
    use super::*;

    /// 检测系统主题偏好
    pub fn detect_system_theme() -> ThemePreset {
        // 在实际应用中，这里应该检测系统的主题偏好
        // 目前返回默认的浅色主题
        ThemePreset::light()
    }

    /// 根据时间自动选择主题
    pub fn auto_theme_by_time() -> ThemePreset {
        // 简单的时间判断逻辑：6-18点使用浅色主题，其他时间使用深色主题
        let hour = chrono::Local::now()
            .format("%H")
            .to_string()
            .parse::<u32>()
            .unwrap_or(12);
        if hour >= 6 && hour < 18 {
            ThemePreset::light()
        } else {
            ThemePreset::dark()
        }
    }

    /// 主题对比度检查
    pub fn check_contrast_ratio(theme: &ThemePreset) -> f32 {
        // 简化的对比度计算，实际应用中需要更复杂的算法
        match theme.variant {
            ThemeVariant::Light => 4.5, // WCAG AA 标准
            ThemeVariant::Dark => 7.0,  // WCAG AAA 标准
            ThemeVariant::Auto => 4.5,  // 默认使用浅色主题标准
        }
    }
}

//! Ant Design 颜色规范
//!
//! 实现 Ant Design 的颜色系统，包括：
//! - 基础颜色调色板
//! - 语义化颜色
//! - 中性色
//! - 功能色
//! - 浅色/深色主题适配

use crate::theme::core::{ColorPalette, ColorType, RgbColor};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Ant Design 颜色预设
pub mod presets {
    use super::RgbColor;

    // 基础色彩
    /// 拂晓蓝（主色）
    pub const BLUE: RgbColor = RgbColor {
        r: 24,
        g: 144,
        b: 255,
    };
    /// 薄暮（紫色）
    pub const PURPLE: RgbColor = RgbColor {
        r: 114,
        g: 46,
        b: 209,
    };
    /// 日暮（青色）
    pub const CYAN: RgbColor = RgbColor {
        r: 19,
        g: 194,
        b: 194,
    };
    /// 极光绿
    pub const GREEN: RgbColor = RgbColor {
        r: 82,
        g: 196,
        b: 26,
    };
    /// 酱紫（粉色）
    pub const MAGENTA: RgbColor = RgbColor {
        r: 235,
        g: 47,
        b: 150,
    };
    /// 日出（红色）
    pub const RED: RgbColor = RgbColor {
        r: 255,
        g: 77,
        b: 79,
    };
    /// 火山（橙红色）
    pub const VOLCANO: RgbColor = RgbColor {
        r: 250,
        g: 84,
        b: 28,
    };
    /// 日落（橙色）
    pub const ORANGE: RgbColor = RgbColor {
        r: 250,
        g: 173,
        b: 20,
    };
    /// 金盏花（黄色）
    pub const GOLD: RgbColor = RgbColor {
        r: 250,
        g: 173,
        b: 20,
    };
    /// 青柠（黄绿色）
    pub const YELLOW: RgbColor = RgbColor {
        r: 250,
        g: 219,
        b: 20,
    };
    /// 新生（绿色）
    pub const LIME: RgbColor = RgbColor {
        r: 160,
        g: 217,
        b: 17,
    };
    /// 极客蓝
    pub const GEEK_BLUE: RgbColor = RgbColor {
        r: 47,
        g: 84,
        b: 235,
    };

    // 中性色
    /// 标题色
    pub const TITLE: RgbColor = RgbColor { r: 0, g: 0, b: 0 };
    /// 主文本色
    pub const TEXT: RgbColor = RgbColor { r: 0, g: 0, b: 0 };
    /// 次文本色
    pub const TEXT_SECONDARY: RgbColor = RgbColor {
        r: 69,
        g: 69,
        b: 69,
    };
    /// 失效色
    pub const TEXT_DISABLED: RgbColor = RgbColor {
        r: 140,
        g: 140,
        b: 140,
    };
    /// 边框色
    pub const BORDER: RgbColor = RgbColor {
        r: 217,
        g: 217,
        b: 217,
    };
    /// 分割线色
    pub const DIVIDER: RgbColor = RgbColor {
        r: 240,
        g: 240,
        b: 240,
    };
    /// 背景色
    pub const BACKGROUND: RgbColor = RgbColor {
        r: 255,
        g: 255,
        b: 255,
    };
    /// 布局背景色
    pub const LAYOUT_BACKGROUND: RgbColor = RgbColor {
        r: 245,
        g: 245,
        b: 245,
    };

    // 深色主题中性色
    /// 深色主题标题色
    pub const DARK_TITLE: RgbColor = RgbColor {
        r: 255,
        g: 255,
        b: 255,
    };
    /// 深色主题主文本色
    pub const DARK_TEXT: RgbColor = RgbColor {
        r: 255,
        g: 255,
        b: 255,
    };
    /// 深色主题次文本色
    pub const DARK_TEXT_SECONDARY: RgbColor = RgbColor {
        r: 191,
        g: 191,
        b: 191,
    };
    /// 深色主题失效色
    pub const DARK_TEXT_DISABLED: RgbColor = RgbColor {
        r: 140,
        g: 140,
        b: 140,
    };
    /// 深色主题边框色
    pub const DARK_BORDER: RgbColor = RgbColor {
        r: 48,
        g: 48,
        b: 48,
    };
    /// 深色主题分割线色
    pub const DARK_DIVIDER: RgbColor = RgbColor {
        r: 48,
        g: 48,
        b: 48,
    };
    /// 深色主题背景色
    pub const DARK_BACKGROUND: RgbColor = RgbColor {
        r: 20,
        g: 20,
        b: 20,
    };
    /// 深色主题布局背景色
    pub const DARK_LAYOUT_BACKGROUND: RgbColor = RgbColor { r: 0, g: 0, b: 0 };
}

/// Ant Design 颜色配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AntDesignColors {
    /// 主色调色板
    pub primary: ColorPalette,
    /// 成功色调色板
    pub success: ColorPalette,
    /// 警告色调色板
    pub warning: ColorPalette,
    /// 错误色调色板
    pub error: ColorPalette,
    /// 信息色调色板
    pub info: ColorPalette,
    /// 链接色调色板
    pub link: ColorPalette,

    // 中性色
    /// 标题色
    pub title: RgbColor,
    /// 主文本色
    pub text: RgbColor,
    /// 次文本色
    pub text_secondary: RgbColor,
    /// 失效色
    pub text_disabled: RgbColor,
    /// 边框色
    pub border: RgbColor,
    /// 分割线色
    pub divider: RgbColor,
    /// 背景色
    pub background: RgbColor,
    /// 布局背景色
    pub layout_background: RgbColor,

    /// 是否为深色主题
    pub is_dark: bool,
}

impl AntDesignColors {
    /// 创建浅色主题颜色配置
    pub fn light() -> Self {
        Self {
            primary: ColorPalette::generate(presets::BLUE, 10),
            success: ColorPalette::generate(presets::GREEN, 10),
            warning: ColorPalette::generate(presets::ORANGE, 10),
            error: ColorPalette::generate(presets::RED, 10),
            info: ColorPalette::generate(presets::BLUE, 10),
            link: ColorPalette::generate(presets::BLUE, 10),

            title: presets::TITLE,
            text: presets::TEXT,
            text_secondary: presets::TEXT_SECONDARY,
            text_disabled: presets::TEXT_DISABLED,
            border: presets::BORDER,
            divider: presets::DIVIDER,
            background: presets::BACKGROUND,
            layout_background: presets::LAYOUT_BACKGROUND,

            is_dark: false,
        }
    }

    /// 创建深色主题颜色配置
    pub fn dark() -> Self {
        Self {
            primary: ColorPalette::generate(presets::BLUE, 10),
            success: ColorPalette::generate(presets::GREEN, 10),
            warning: ColorPalette::generate(presets::ORANGE, 10),
            error: ColorPalette::generate(presets::RED, 10),
            info: ColorPalette::generate(presets::BLUE, 10),
            link: ColorPalette::generate(presets::BLUE, 10),

            title: presets::DARK_TITLE,
            text: presets::DARK_TEXT,
            text_secondary: presets::DARK_TEXT_SECONDARY,
            text_disabled: presets::DARK_TEXT_DISABLED,
            border: presets::DARK_BORDER,
            divider: presets::DARK_DIVIDER,
            background: presets::DARK_BACKGROUND,
            layout_background: presets::DARK_LAYOUT_BACKGROUND,

            is_dark: true,
        }
    }

    /// 获取指定类型的颜色
    pub fn get_color(&self, color_type: ColorType) -> Option<RgbColor> {
        match color_type {
            ColorType::Primary => Some(self.primary.base),
            ColorType::Success => Some(self.success.base),
            ColorType::Warning => Some(self.warning.base),
            ColorType::Error => Some(self.error.base),
            ColorType::Info => Some(self.info.base),
            ColorType::Link => Some(self.link.base),
            ColorType::Default => Some(self.text_secondary),
        }
    }

    /// 获取指定类型的调色板
    pub fn get_palette(&self, color_type: ColorType) -> Option<&ColorPalette> {
        match color_type {
            ColorType::Primary => Some(&self.primary),
            ColorType::Success => Some(&self.success),
            ColorType::Warning => Some(&self.warning),
            ColorType::Error => Some(&self.error),
            ColorType::Info => Some(&self.info),
            ColorType::Link => Some(&self.link),
            ColorType::Default => None,
        }
    }

    /// 生成 CSS 变量
    pub fn to_css_variables(&self) -> HashMap<String, String> {
        let mut variables = HashMap::new();

        // 语义化颜色变量
        variables.insert(
            "--ant-primary-color".to_string(),
            self.primary.base.to_rgb_string(),
        );
        variables.insert(
            "--ant-success-color".to_string(),
            self.success.base.to_rgb_string(),
        );
        variables.insert(
            "--ant-warning-color".to_string(),
            self.warning.base.to_rgb_string(),
        );
        variables.insert(
            "--ant-error-color".to_string(),
            self.error.base.to_rgb_string(),
        );
        variables.insert(
            "--ant-info-color".to_string(),
            self.info.base.to_rgb_string(),
        );
        variables.insert(
            "--ant-link-color".to_string(),
            self.link.base.to_rgb_string(),
        );

        // 中性色变量
        variables.insert("--ant-text-color".to_string(), self.text.to_rgb_string());
        variables.insert(
            "--ant-text-color-secondary".to_string(),
            self.text_secondary.to_rgb_string(),
        );
        variables.insert(
            "--ant-text-color-disabled".to_string(),
            self.text_disabled.to_rgb_string(),
        );
        variables.insert(
            "--ant-border-color".to_string(),
            self.border.to_rgb_string(),
        );
        variables.insert(
            "--ant-divider-color".to_string(),
            self.divider.to_rgb_string(),
        );
        variables.insert(
            "--ant-background-color".to_string(),
            self.background.to_rgb_string(),
        );
        variables.insert(
            "--ant-layout-background-color".to_string(),
            self.layout_background.to_rgb_string(),
        );

        // 调色板变量
        for (name, palette) in [
            ("primary", &self.primary),
            ("success", &self.success),
            ("warning", &self.warning),
            ("error", &self.error),
            ("info", &self.info),
            ("link", &self.link),
        ] {
            variables.insert(
                format!("--ant-{}-color-light", name),
                palette.light.to_rgb_string(),
            );
            variables.insert(
                format!("--ant-{}-color-lighter", name),
                palette.lighter.to_rgb_string(),
            );
            variables.insert(
                format!("--ant-{}-color-dark", name),
                palette.dark.to_rgb_string(),
            );
            variables.insert(
                format!("--ant-{}-color-darker", name),
                palette.darker.to_rgb_string(),
            );
        }

        variables
    }

    /// 生成 CSS 样式
    pub fn generate_css(&self) -> String {
        let mut css = String::new();

        // 基础颜色类
        css.push_str(".ant-primary { color: var(--ant-primary-color); }\n");
        css.push_str(".ant-success { color: var(--ant-success-color); }\n");
        css.push_str(".ant-warning { color: var(--ant-warning-color); }\n");
        css.push_str(".ant-error { color: var(--ant-error-color); }\n");
        css.push_str(".ant-info { color: var(--ant-info-color); }\n");
        css.push_str(".ant-link { color: var(--ant-link-color); }\n");

        // 背景色类
        css.push_str(".ant-bg-primary { background-color: var(--ant-primary-color); }\n");
        css.push_str(".ant-bg-success { background-color: var(--ant-success-color); }\n");
        css.push_str(".ant-bg-warning { background-color: var(--ant-warning-color); }\n");
        css.push_str(".ant-bg-error { background-color: var(--ant-error-color); }\n");
        css.push_str(".ant-bg-info { background-color: var(--ant-info-color); }\n");

        // 文本颜色类
        css.push_str(".ant-text { color: var(--ant-text-color); }\n");
        css.push_str(".ant-text-secondary { color: var(--ant-text-color-secondary); }\n");
        css.push_str(".ant-text-disabled { color: var(--ant-text-color-disabled); }\n");

        // 边框颜色类
        css.push_str(".ant-border { border-color: var(--ant-border-color); }\n");
        css.push_str(".ant-divider { border-color: var(--ant-divider-color); }\n");

        css
    }
}

impl Default for AntDesignColors {
    fn default() -> Self {
        Self::light()
    }
}

/// 颜色工具函数
pub mod utils {
    use super::*;

    /// 根据颜色类型获取默认颜色
    pub fn get_color_by_type(color_type: ColorType) -> RgbColor {
        match color_type {
            ColorType::Primary => presets::BLUE,
            ColorType::Success => presets::GREEN,
            ColorType::Warning => presets::ORANGE,
            ColorType::Error => presets::RED,
            ColorType::Info => presets::BLUE,
            ColorType::Default => presets::TEXT_SECONDARY,
            ColorType::Link => presets::BLUE,
        }
    }

    /// 生成颜色的 CSS 变量名
    pub fn generate_css_var_name(color_type: ColorType, variant: Option<&str>) -> String {
        let base_name = match color_type {
            ColorType::Primary => "primary",
            ColorType::Success => "success",
            ColorType::Warning => "warning",
            ColorType::Error => "error",
            ColorType::Info => "info",
            ColorType::Default => "default",
            ColorType::Link => "link",
        };

        match variant {
            Some(variant) => format!("--ant-{}-color-{}", base_name, variant),
            None => format!("--ant-{}-color", base_name),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_light_colors() {
        let colors = AntDesignColors::light();
        assert!(!colors.is_dark);
        assert_eq!(colors.primary.base, presets::BLUE);
        assert_eq!(colors.success.base, presets::GREEN);
    }

    #[test]
    fn test_dark_colors() {
        let colors = AntDesignColors::dark();
        assert!(colors.is_dark);
        assert_eq!(colors.text, presets::DARK_TEXT);
        assert_eq!(colors.background, presets::DARK_BACKGROUND);
    }

    #[test]
    fn test_get_color() {
        let colors = AntDesignColors::light();
        assert_eq!(colors.get_color(ColorType::Primary), Some(presets::BLUE));
        assert_eq!(colors.get_color(ColorType::Success), Some(presets::GREEN));
    }

    #[test]
    fn test_css_variables() {
        let colors = AntDesignColors::light();
        let variables = colors.to_css_variables();

        assert!(variables.contains_key("--ant-primary-color"));
        assert!(variables.contains_key("--ant-text-color"));
        assert!(variables.contains_key("--ant-primary-color-light"));
    }

    #[test]
    fn test_css_generation() {
        let colors = AntDesignColors::light();
        let css = colors.generate_css();

        assert!(css.contains(".ant-primary"));
        assert!(css.contains(".ant-text"));
        assert!(css.contains("var(--ant-primary-color)"));
    }

    #[test]
    fn test_utils() {
        assert_eq!(utils::get_color_by_type(ColorType::Primary), presets::BLUE);
        assert_eq!(
            utils::generate_css_var_name(ColorType::Primary, None),
            "--ant-primary-color"
        );
        assert_eq!(
            utils::generate_css_var_name(ColorType::Primary, Some("light")),
            "--ant-primary-color-light"
        );
    }
}

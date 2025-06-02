//! Theme system for Ant Design Dioxus components
//!
//! This module provides theme support and CSS integration for Ant Design components.
//! It includes Ant Design specific design tokens, color presets, animation configurations,
//! and theme presets that work seamlessly with the css-in-rust foundation.

pub mod css_bridge;
pub mod hooks;
pub mod tokens;

pub use css_bridge::*;
pub use hooks::*;
pub use tokens::*;

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

use crate::utils::color::{presets as color_presets, ColorPalette, ColorType, RgbColor};
use crate::utils::motion::{Duration, Easing};
use crate::utils::size::Size;

/// 主题类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

impl fmt::Display for Theme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Theme::Light => write!(f, "light"),
            Theme::Dark => write!(f, "dark"),
            Theme::Compact => write!(f, "compact"),
            Theme::Custom => write!(f, "custom"),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Theme::Light
    }
}

/// 颜色主题配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ColorTheme {
    /// 主色
    pub primary: ColorPalette,
    /// 成功色
    pub success: ColorPalette,
    /// 警告色
    pub warning: ColorPalette,
    /// 错误色
    pub error: ColorPalette,
    /// 信息色
    pub info: ColorPalette,
    /// 文本颜色
    pub text: TextColors,
    /// 背景颜色
    pub background: BackgroundColors,
    /// 边框颜色
    pub border: BorderColors,
    /// 填充颜色
    pub fill: FillColors,
}

/// 文本颜色配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct TextColors {
    /// 主要文本
    pub primary: RgbColor,
    /// 次要文本
    pub secondary: RgbColor,
    /// 禁用文本
    pub disabled: RgbColor,
    /// 反色文本
    pub inverse: RgbColor,
}

/// 背景颜色配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct BackgroundColors {
    /// 主背景
    pub primary: RgbColor,
    /// 次背景
    pub secondary: RgbColor,
    /// 容器背景
    pub container: RgbColor,
    /// 悬浮背景
    pub elevated: RgbColor,
    /// 布局背景
    pub layout: RgbColor,
}

/// 边框颜色配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct BorderColors {
    /// 基础边框
    pub base: RgbColor,
    /// 分割线
    pub split: RgbColor,
    /// 次要边框
    pub secondary: RgbColor,
}

/// 填充颜色配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct FillColors {
    /// 基础填充
    pub base: RgbColor,
    /// 次要填充
    pub secondary: RgbColor,
    /// 三级填充
    pub tertiary: RgbColor,
    /// 四级填充
    pub quaternary: RgbColor,
}

/// 尺寸主题配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SizeTheme {
    /// 基础单位
    pub unit: u32,
    /// 字体大小
    pub font_sizes: FontSizes,
    /// 行高
    pub line_heights: LineHeights,
    /// 边框圆角
    pub border_radius: BorderRadius,
    /// 间距
    pub spacing: Spacing,
    /// 阴影
    pub shadows: Shadows,
}

/// 字体大小配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct FontSizes {
    /// 超小字体
    pub xs: u32,
    /// 小字体
    pub sm: u32,
    /// 基础字体
    pub base: u32,
    /// 大字体
    pub lg: u32,
    /// 超大字体
    pub xl: u32,
    /// 标题字体
    pub heading: HashMap<u32, u32>, // h1-h6
}

/// 行高配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct LineHeights {
    /// 基础行高
    pub base: f32,
    /// 标题行高
    pub heading: f32,
    /// 紧凑行高
    pub sm: f32,
}

/// 边框圆角配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct BorderRadius {
    /// 超小圆角
    pub xs: u32,
    /// 小圆角
    pub sm: u32,
    /// 基础圆角
    pub base: u32,
    /// 大圆角
    pub lg: u32,
    /// 超大圆角
    pub xl: u32,
}

/// 间距配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Spacing {
    /// 超小间距
    pub xs: u32,
    /// 小间距
    pub sm: u32,
    /// 基础间距
    pub base: u32,
    /// 大间距
    pub lg: u32,
    /// 超大间距
    pub xl: u32,
    /// 超超大间距
    pub xxl: u32,
}

/// 阴影配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Shadows {
    /// 基础阴影
    pub base: String,
    /// 小阴影
    pub sm: String,
    /// 大阴影
    pub lg: String,
    /// 超大阴影
    pub xl: String,
}

/// 动画主题配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MotionTheme {
    /// 动画持续时间
    pub durations: MotionDurations,
    /// 缓动函数
    pub easings: MotionEasings,
}

/// 动画持续时间配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MotionDurations {
    /// 快速
    pub fast: Duration,
    /// 中等
    pub mid: Duration,
    /// 慢速
    pub slow: Duration,
}

/// 缓动函数配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MotionEasings {
    /// 标准缓动
    pub standard: Easing,
    /// 强调缓动
    pub emphasized: Easing,
    /// 减速缓动
    pub decelerated: Easing,
    /// 加速缓动
    pub accelerated: Easing,
}

/// 主题配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThemeConfig {
    /// 主题类型
    pub theme: Theme,
    /// 颜色主题
    pub colors: ColorTheme,
    /// 尺寸主题
    pub sizes: SizeTheme,
    /// 动画主题
    pub motion: MotionTheme,
    /// 自定义 CSS 变量
    pub custom_vars: HashMap<String, String>,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self::light()
    }
}

impl ThemeConfig {
    /// 创建亮色主题
    pub fn light() -> Self {
        let mut font_sizes = HashMap::new();
        font_sizes.insert(1, 38);
        font_sizes.insert(2, 30);
        font_sizes.insert(3, 24);
        font_sizes.insert(4, 20);
        font_sizes.insert(5, 16);
        font_sizes.insert(6, 14);

        Self {
            theme: Theme::Light,
            colors: ColorTheme {
                primary: ColorPalette::generate(color_presets::BLUE, 10),
                success: ColorPalette::generate(color_presets::GREEN, 10),
                warning: ColorPalette::generate(color_presets::ORANGE, 10),
                error: ColorPalette::generate(color_presets::RED, 10),
                info: ColorPalette::generate(color_presets::BLUE, 10),
                text: TextColors {
                    primary: RgbColor::new(0, 0, 0),
                    secondary: RgbColor::new(102, 102, 102),
                    disabled: RgbColor::new(191, 191, 191),
                    inverse: RgbColor::new(255, 255, 255),
                },
                background: BackgroundColors {
                    primary: RgbColor::new(255, 255, 255),
                    secondary: RgbColor::new(250, 250, 250),
                    container: RgbColor::new(255, 255, 255),
                    elevated: RgbColor::new(255, 255, 255),
                    layout: RgbColor::new(240, 242, 245),
                },
                border: BorderColors {
                    base: RgbColor::new(217, 217, 217),
                    split: RgbColor::new(240, 240, 240),
                    secondary: RgbColor::new(242, 242, 242),
                },
                fill: FillColors {
                    base: RgbColor::new(0, 0, 0),
                    secondary: RgbColor::new(242, 242, 242),
                    tertiary: RgbColor::new(250, 250, 250),
                    quaternary: RgbColor::new(255, 255, 255),
                },
            },
            sizes: SizeTheme {
                unit: 4,
                font_sizes: FontSizes {
                    xs: 12,
                    sm: 14,
                    base: 14,
                    lg: 16,
                    xl: 20,
                    heading: font_sizes,
                },
                line_heights: LineHeights {
                    base: 1.5715,
                    heading: 1.35,
                    sm: 1.66,
                },
                border_radius: BorderRadius {
                    xs: 2,
                    sm: 4,
                    base: 6,
                    lg: 8,
                    xl: 16,
                },
                spacing: Spacing {
                    xs: 4,
                    sm: 8,
                    base: 16,
                    lg: 24,
                    xl: 32,
                    xxl: 48,
                },
                shadows: Shadows {
                    base: "0 2px 8px rgba(0, 0, 0, 0.15)".to_string(),
                    sm: "0 1px 2px rgba(0, 0, 0, 0.03), 0 1px 6px -1px rgba(0, 0, 0, 0.02), 0 2px 4px rgba(0, 0, 0, 0.02)".to_string(),
                    lg: "0 4px 12px rgba(0, 0, 0, 0.15)".to_string(),
                    xl: "0 6px 16px 0 rgba(0, 0, 0, 0.08), 0 3px 6px -4px rgba(0, 0, 0, 0.12), 0 9px 28px 8px rgba(0, 0, 0, 0.05)".to_string(),
                },
            },
            motion: MotionTheme {
                durations: MotionDurations {
                    fast: Duration::Fast,
                    mid: Duration::Mid,
                    slow: Duration::Slow,
                },
                easings: MotionEasings {
                    standard: Easing::EaseInOut,
                    emphasized: Easing::EaseOutBack,
                    decelerated: Easing::EaseOut,
                    accelerated: Easing::EaseIn,
                },
            },
            custom_vars: HashMap::new(),
        }
    }

    /// 创建暗色主题
    pub fn dark() -> Self {
        let mut theme = Self::light();
        theme.theme = Theme::Dark;

        // 调整暗色主题的颜色
        theme.colors.text = TextColors {
            primary: RgbColor::new(255, 255, 255),
            secondary: RgbColor::new(191, 191, 191),
            disabled: RgbColor::new(102, 102, 102),
            inverse: RgbColor::new(0, 0, 0),
        };

        theme.colors.background = BackgroundColors {
            primary: RgbColor::new(20, 20, 20),
            secondary: RgbColor::new(30, 30, 30),
            container: RgbColor::new(36, 36, 36),
            elevated: RgbColor::new(48, 48, 48),
            layout: RgbColor::new(16, 16, 16),
        };

        theme.colors.border = BorderColors {
            base: RgbColor::new(64, 64, 64),
            split: RgbColor::new(48, 48, 48),
            secondary: RgbColor::new(32, 32, 32),
        };

        theme.colors.fill = FillColors {
            base: RgbColor::new(255, 255, 255),
            secondary: RgbColor::new(48, 48, 48),
            tertiary: RgbColor::new(32, 32, 32),
            quaternary: RgbColor::new(20, 20, 20),
        };

        theme
    }

    /// 创建紧凑主题
    pub fn compact() -> Self {
        let mut theme = Self::light();
        theme.theme = Theme::Compact;

        // 调整紧凑主题的尺寸
        theme.sizes.spacing = Spacing {
            xs: 2,
            sm: 4,
            base: 8,
            lg: 12,
            xl: 16,
            xxl: 24,
        };

        theme.sizes.font_sizes.xs = 10;
        theme.sizes.font_sizes.sm = 12;
        theme.sizes.font_sizes.base = 12;
        theme.sizes.font_sizes.lg = 14;
        theme.sizes.font_sizes.xl = 16;

        theme
    }

    /// 生成 CSS 变量
    pub fn generate_css_vars(&self) -> String {
        let mut css = String::new();

        // 颜色变量
        css.push_str(&format!(":root {{\n"));

        // 主色系
        css.push_str(&format!(
            "  --ant-primary-color: {};\n",
            self.colors.primary.base.to_hex()
        ));
        css.push_str(&format!(
            "  --ant-primary-color-hover: {};\n",
            self.colors.primary.light.to_hex()
        ));
        css.push_str(&format!(
            "  --ant-primary-color-active: {};\n",
            self.colors.primary.dark.to_hex()
        ));

        // 功能色系
        css.push_str(&format!(
            "  --ant-success-color: {};\n",
            self.colors.success.base.to_hex()
        ));
        css.push_str(&format!(
            "  --ant-success-color-hover: {};\n",
            self.colors.success.light.to_hex()
        ));
        css.push_str(&format!(
            "  --ant-success-color-active: {};\n",
            self.colors.success.dark.to_hex()
        ));
        css.push_str(&format!(
            "  --ant-warning-color: {};\n",
            self.colors.warning.base.to_hex()
        ));
        css.push_str(&format!(
            "  --ant-warning-color-hover: {};\n",
            self.colors.warning.light.to_hex()
        ));
        css.push_str(&format!(
            "  --ant-warning-color-active: {};\n",
            self.colors.warning.dark.to_hex()
        ));
        css.push_str(&format!(
            "  --ant-error-color: {};\n",
            self.colors.error.base.to_hex()
        ));
        css.push_str(&format!(
            "  --ant-error-color-hover: {};\n",
            self.colors.error.light.to_hex()
        ));
        css.push_str(&format!(
            "  --ant-error-color-active: {};\n",
            self.colors.error.dark.to_hex()
        ));
        css.push_str(&format!(
            "  --ant-info-color: {};\n",
            self.colors.info.base.to_hex()
        ));
        css.push_str(&format!(
            "  --ant-info-color-hover: {};\n",
            self.colors.info.light.to_hex()
        ));
        css.push_str(&format!(
            "  --ant-info-color-active: {};\n",
            self.colors.info.dark.to_hex()
        ));

        // 文本颜色
        css.push_str(&format!(
            "  --ant-text-color: {};\n",
            self.colors.text.primary.to_hex()
        ));
        css.push_str(&format!(
            "  --ant-text-color-secondary: {};\n",
            self.colors.text.secondary.to_hex()
        ));
        css.push_str(&format!(
            "  --ant-text-color-disabled: {};\n",
            self.colors.text.disabled.to_hex()
        ));
        css.push_str(&format!(
            "  --ant-text-color-inverse: {};\n",
            self.colors.text.inverse.to_hex()
        ));

        // 背景颜色
        css.push_str(&format!(
            "  --ant-background-color: {};\n",
            self.colors.background.primary.to_hex()
        ));
        css.push_str(&format!(
            "  --ant-background-color-secondary: {};\n",
            self.colors.background.secondary.to_hex()
        ));
        css.push_str(&format!(
            "  --ant-background-color-container: {};\n",
            self.colors.background.container.to_hex()
        ));
        css.push_str(&format!(
            "  --ant-background-color-elevated: {};\n",
            self.colors.background.elevated.to_hex()
        ));
        css.push_str(&format!(
            "  --ant-background-color-layout: {};\n",
            self.colors.background.layout.to_hex()
        ));

        // 边框颜色
        css.push_str(&format!(
            "  --ant-border-color: {};\n",
            self.colors.border.base.to_hex()
        ));
        css.push_str(&format!(
            "  --ant-border-color-split: {};\n",
            self.colors.border.split.to_hex()
        ));
        css.push_str(&format!(
            "  --ant-border-color-secondary: {};\n",
            self.colors.border.secondary.to_hex()
        ));

        // 填充颜色
        css.push_str(&format!(
            "  --ant-fill-color: {};\n",
            self.colors.fill.base.to_hex()
        ));
        css.push_str(&format!(
            "  --ant-fill-color-secondary: {};\n",
            self.colors.fill.secondary.to_hex()
        ));
        css.push_str(&format!(
            "  --ant-fill-color-tertiary: {};\n",
            self.colors.fill.tertiary.to_hex()
        ));
        css.push_str(&format!(
            "  --ant-fill-color-quaternary: {};\n",
            self.colors.fill.quaternary.to_hex()
        ));

        // 尺寸变量
        css.push_str(&format!(
            "  --ant-border-radius-xs: {}px;\n",
            self.sizes.border_radius.xs
        ));
        css.push_str(&format!(
            "  --ant-border-radius-sm: {}px;\n",
            self.sizes.border_radius.sm
        ));
        css.push_str(&format!(
            "  --ant-border-radius: {}px;\n",
            self.sizes.border_radius.base
        ));
        css.push_str(&format!(
            "  --ant-border-radius-lg: {}px;\n",
            self.sizes.border_radius.lg
        ));
        css.push_str(&format!(
            "  --ant-border-radius-xl: {}px;\n",
            self.sizes.border_radius.xl
        ));

        // 字体大小
        css.push_str(&format!(
            "  --ant-font-size-xs: {}px;\n",
            self.sizes.font_sizes.xs
        ));
        css.push_str(&format!(
            "  --ant-font-size-sm: {}px;\n",
            self.sizes.font_sizes.sm
        ));
        css.push_str(&format!(
            "  --ant-font-size: {}px;\n",
            self.sizes.font_sizes.base
        ));
        css.push_str(&format!(
            "  --ant-font-size-lg: {}px;\n",
            self.sizes.font_sizes.lg
        ));
        css.push_str(&format!(
            "  --ant-font-size-xl: {}px;\n",
            self.sizes.font_sizes.xl
        ));

        // 行高
        css.push_str(&format!(
            "  --ant-line-height: {};\n",
            self.sizes.line_heights.base
        ));
        css.push_str(&format!(
            "  --ant-line-height-heading: {};\n",
            self.sizes.line_heights.heading
        ));
        css.push_str(&format!(
            "  --ant-line-height-sm: {};\n",
            self.sizes.line_heights.sm
        ));

        // 间距变量
        css.push_str(&format!(
            "  --ant-spacing-xs: {}px;\n",
            self.sizes.spacing.xs
        ));
        css.push_str(&format!(
            "  --ant-spacing-sm: {}px;\n",
            self.sizes.spacing.sm
        ));
        css.push_str(&format!(
            "  --ant-spacing: {}px;\n",
            self.sizes.spacing.base
        ));
        css.push_str(&format!(
            "  --ant-spacing-lg: {}px;\n",
            self.sizes.spacing.lg
        ));
        css.push_str(&format!(
            "  --ant-spacing-xl: {}px;\n",
            self.sizes.spacing.xl
        ));
        css.push_str(&format!(
            "  --ant-spacing-xxl: {}px;\n",
            self.sizes.spacing.xxl
        ));

        // 阴影变量
        css.push_str(&format!("  --ant-shadow: {};\n", self.sizes.shadows.base));
        css.push_str(&format!("  --ant-shadow-sm: {};\n", self.sizes.shadows.sm));
        css.push_str(&format!("  --ant-shadow-lg: {};\n", self.sizes.shadows.lg));
        css.push_str(&format!("  --ant-shadow-xl: {};\n", self.sizes.shadows.xl));

        // 动画变量
        css.push_str(&format!(
            "  --ant-duration-fast: {};\n",
            self.motion.durations.fast
        ));
        css.push_str(&format!(
            "  --ant-duration-mid: {};\n",
            self.motion.durations.mid
        ));
        css.push_str(&format!(
            "  --ant-duration-slow: {};\n",
            self.motion.durations.slow
        ));
        css.push_str(&format!(
            "  --ant-ease-standard: {};\n",
            self.motion.easings.standard
        ));
        css.push_str(&format!(
            "  --ant-ease-emphasized: {};\n",
            self.motion.easings.emphasized
        ));
        css.push_str(&format!(
            "  --ant-ease-decelerated: {};\n",
            self.motion.easings.decelerated
        ));
        css.push_str(&format!(
            "  --ant-ease-accelerated: {};\n",
            self.motion.easings.accelerated
        ));

        // 自定义变量
        for (key, value) in &self.custom_vars {
            css.push_str(&format!("  --{}: {};\n", key, value));
        }

        css.push_str("}\n");
        css
    }

    /// 获取颜色值
    pub fn get_color(&self, color_type: ColorType) -> &ColorPalette {
        match color_type {
            ColorType::Primary => &self.colors.primary,
            ColorType::Success => &self.colors.success,
            ColorType::Warning => &self.colors.warning,
            ColorType::Error => &self.colors.error,
            ColorType::Info => &self.colors.info,
            _ => &self.colors.primary,
        }
    }

    /// 获取字体大小
    pub fn get_font_size(&self, size: Size) -> u32 {
        match size {
            Size::Small => self.sizes.font_sizes.sm,
            Size::Middle => self.sizes.font_sizes.base,
            Size::Large => self.sizes.font_sizes.lg,
        }
    }

    /// 获取间距
    pub fn get_spacing(&self, size: Size) -> u32 {
        match size {
            Size::Small => self.sizes.spacing.sm,
            Size::Middle => self.sizes.spacing.base,
            Size::Large => self.sizes.spacing.lg,
        }
    }

    /// 生成主题色调色板
    pub fn generate_color_palette(&self, base_color: RgbColor, theme: Theme) -> ColorPalette {
        match theme {
            Theme::Dark => self.generate_dark_palette(base_color),
            Theme::Light => self.generate_light_palette(base_color),
            Theme::Compact => self.generate_compact_palette(base_color),
            _ => self.generate_light_palette(base_color),
        }
    }

    /// 生成浅色主题调色板
    fn generate_light_palette(&self, base_color: RgbColor) -> ColorPalette {
        ColorPalette::generate(base_color, 10)
    }

    /// 生成深色主题调色板
    fn generate_dark_palette(&self, base_color: RgbColor) -> ColorPalette {
        ColorPalette::generate(base_color, 10)
    }

    /// 生成紧凑主题调色板
    fn generate_compact_palette(&self, base_color: RgbColor) -> ColorPalette {
        ColorPalette::generate(base_color, 10)
    }

    /// 创建自定义主题
    pub fn create_custom_theme(primary_color: RgbColor, theme_type: Theme) -> Self {
        let mut config = Self::default();
        config.colors.primary = config.generate_color_palette(primary_color, theme_type);

        // 根据主题类型调整其他颜色
        match theme_type {
            Theme::Dark => {
                config.colors.text.primary = RgbColor::new(255, 255, 255);
                config.colors.text.secondary = RgbColor::new(191, 191, 191);
                config.colors.text.disabled = RgbColor::new(64, 64, 64);
                config.colors.background.primary = RgbColor::new(20, 20, 20);
                config.colors.background.secondary = RgbColor::new(30, 30, 30);
                config.colors.background.container = RgbColor::new(40, 40, 40);
            }
            Theme::Light => {
                config.colors.text.primary = RgbColor::new(0, 0, 0);
                config.colors.text.secondary = RgbColor::new(102, 102, 102);
                config.colors.text.disabled = RgbColor::new(191, 191, 191);
                config.colors.background.primary = RgbColor::new(255, 255, 255);
                config.colors.background.secondary = RgbColor::new(250, 250, 250);
                config.colors.background.container = RgbColor::new(245, 245, 245);
            }
            Theme::Compact => {
                // 紧凑主题使用较小的尺寸
                config.sizes.spacing.xs = 2;
                config.sizes.spacing.sm = 4;
                config.sizes.spacing.base = 8;
                config.sizes.spacing.lg = 12;
                config.sizes.spacing.xl = 16;
                config.sizes.spacing.xxl = 20;

                config.sizes.font_sizes.xs = 10;
                config.sizes.font_sizes.sm = 12;
                config.sizes.font_sizes.base = 13;
                config.sizes.font_sizes.lg = 14;
                config.sizes.font_sizes.xl = 16;
            }
            _ => {}
        }

        config
    }

    /// 获取主题令牌值
    pub fn get_token(&self, token: &str) -> Option<String> {
        match token {
            "primary-color" => Some(self.colors.primary.base.to_hex()),
            "primary-color-hover" => Some(self.colors.primary.light.to_hex()),
            "primary-color-active" => Some(self.colors.primary.dark.to_hex()),
            "text-color" => Some(self.colors.text.primary.to_hex()),
            "text-color-secondary" => Some(self.colors.text.secondary.to_hex()),
            "background-color" => Some(self.colors.background.primary.to_hex()),
            "border-radius" => Some(format!("{}px", self.sizes.border_radius.base)),
            "font-size" => Some(format!("{}px", self.sizes.font_sizes.base)),
            "spacing" => Some(format!("{}px", self.sizes.spacing.base)),
            _ => self.custom_vars.get(token).cloned(),
        }
    }

    /// 设置自定义令牌
    pub fn set_token(&mut self, token: String, value: String) {
        self.custom_vars.insert(token, value);
    }

    /// 获取边框圆角
    pub fn get_border_radius(&self, size: Size) -> u32 {
        match size {
            Size::Small => self.sizes.border_radius.sm,
            Size::Middle => self.sizes.border_radius.base,
            Size::Large => self.sizes.border_radius.lg,
        }
    }
}

/// 使用主题令牌的 Hook
///
/// 返回一个函数，用于获取主题令牌值
pub fn use_theme_token() -> impl Fn(&str) -> Option<String> {
    let theme_signal = use_context::<Signal<ThemeConfig>>();
    move |token: &str| {
        let config = theme_signal.read();
        config.get_token(token)
    }
}

/// 使用主题切换的 Hook
///
/// 返回当前主题和切换主题的函数
pub fn use_theme_switch() -> (Theme, impl FnMut(Theme)) {
    let mut theme_signal = use_context::<Signal<ThemeConfig>>();
    let current_theme = theme_signal.read().theme;

    let switch_theme = move |new_theme: Theme| {
        let mut config = theme_signal.write();
        config.theme = new_theme;
        // 根据新主题重新生成颜色配置
        match new_theme {
            Theme::Dark => {
                config.colors.text.primary = RgbColor::new(255, 255, 255);
                config.colors.text.secondary = RgbColor::new(191, 191, 191);
                config.colors.background.primary = RgbColor::new(20, 20, 20);
                config.colors.background.secondary = RgbColor::new(30, 30, 30);
            }
            Theme::Light => {
                config.colors.text.primary = RgbColor::new(0, 0, 0);
                config.colors.text.secondary = RgbColor::new(102, 102, 102);
                config.colors.background.primary = RgbColor::new(255, 255, 255);
                config.colors.background.secondary = RgbColor::new(250, 250, 250);
            }
            _ => {}
        }
    };

    (current_theme, switch_theme)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_display() {
        assert_eq!(Theme::Light.to_string(), "light");
        assert_eq!(Theme::Dark.to_string(), "dark");
        assert_eq!(Theme::Compact.to_string(), "compact");
    }

    #[test]
    fn test_theme_default() {
        assert_eq!(Theme::default(), Theme::Light);
    }

    #[test]
    fn test_theme_config_light() {
        let theme = ThemeConfig::light();
        assert_eq!(theme.theme, Theme::Light);
        assert_eq!(theme.colors.text.primary, RgbColor::new(0, 0, 0));
        assert_eq!(
            theme.colors.background.primary,
            RgbColor::new(255, 255, 255)
        );
    }

    #[test]
    fn test_theme_config_dark() {
        let theme = ThemeConfig::dark();
        assert_eq!(theme.theme, Theme::Dark);
        assert_eq!(theme.colors.text.primary, RgbColor::new(255, 255, 255));
        assert_eq!(theme.colors.background.primary, RgbColor::new(20, 20, 20));
    }

    #[test]
    fn test_theme_config_compact() {
        let theme = ThemeConfig::compact();
        assert_eq!(theme.theme, Theme::Compact);
        assert_eq!(theme.sizes.spacing.base, 8);
        assert_eq!(theme.sizes.font_sizes.base, 12);
    }

    #[test]
    fn test_get_color() {
        let theme = ThemeConfig::light();
        let primary = theme.get_color(ColorType::Primary);
        assert_eq!(primary.base, color_presets::BLUE);
    }

    #[test]
    fn test_get_font_size() {
        let theme = ThemeConfig::light();
        assert_eq!(theme.get_font_size(Size::Small), 14);
        assert_eq!(theme.get_font_size(Size::Middle), 14);
        assert_eq!(theme.get_font_size(Size::Large), 16);
    }

    #[test]
    fn test_generate_css_vars() {
        let theme = ThemeConfig::light();
        let css = theme.generate_css_vars();

        assert!(css.contains(":root {"));
        assert!(css.contains("--ant-primary-color:"));
        assert!(css.contains("--ant-text-color:"));
        assert!(css.contains("--ant-background-color:"));
        assert!(css.contains("--ant-font-size:"));
        assert!(css.contains("}"));
    }
}

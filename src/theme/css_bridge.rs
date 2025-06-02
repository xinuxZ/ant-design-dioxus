//! CSS-in-Rust 桥接模块
//!
//! 提供 ant-design-dioxus 主题系统与 css-in-rust 的桥接功能

use css_in_rust::theme::{DesignTokens, Theme as CssTheme, ThemeMode, ThemeProvider};
// 注意：css! 宏需要在启用 proc-macro 特性时才可用
use std::collections::HashMap;

use crate::theme::{
    BackgroundColors, BorderColors, ColorTheme, FillColors, ShadowConfig, SizeConfig, TextColors,
    Theme, ThemeConfig, TypographyConfig,
};
use crate::utils::color::{ColorPalette, RgbColor};

/// CSS-in-Rust 主题桥接器
#[derive(Debug, Clone)]
pub struct CssThemeBridge {
    /// CSS-in-Rust 主题实例
    css_theme: CssTheme,
    /// 原始主题配置
    theme_config: ThemeConfig,
}

impl CssThemeBridge {
    /// 创建新的主题桥接器
    pub fn new(theme_config: ThemeConfig) -> Self {
        let css_theme = Self::convert_to_css_theme(&theme_config);
        Self {
            css_theme,
            theme_config,
        }
    }

    /// 获取 CSS-in-Rust 主题
    pub fn css_theme(&self) -> &CssTheme {
        &self.css_theme
    }

    /// 获取原始主题配置
    pub fn theme_config(&self) -> &ThemeConfig {
        &self.theme_config
    }

    /// 更新主题配置
    pub fn update_theme(&mut self, theme_config: ThemeConfig) {
        self.theme_config = theme_config.clone();
        self.css_theme = Self::convert_to_css_theme(&theme_config);
    }

    /// 转换为 CSS-in-Rust 主题
    fn convert_to_css_theme(theme_config: &ThemeConfig) -> CssTheme {
        let theme_mode = match theme_config.theme {
            Theme::Dark => ThemeMode::Dark,
            _ => ThemeMode::Light,
        };

        let mut design_tokens = DesignTokens::default();

        // 转换颜色令牌
        Self::convert_color_tokens(&mut design_tokens, &theme_config.colors);

        // 转换尺寸令牌
        Self::convert_size_tokens(&mut design_tokens, &theme_config.sizes);

        // 转换排版令牌
        Self::convert_typography_tokens(&mut design_tokens, &theme_config.typography);

        // 转换阴影令牌
        Self::convert_shadow_tokens(&mut design_tokens, &theme_config.shadow);

        let custom_variables = HashMap::new();

        CssTheme {
            name: format!("{}-theme", theme_config.theme),
            tokens: design_tokens,
            custom_variables,
            mode: theme_mode,
        }
    }

    /// 转换颜色令牌
    fn convert_color_tokens(tokens: &mut DesignTokens, color: &ColorTheme) {
        // 设置主色调
        tokens.set_color("primary", &color.primary.base().to_hex());
        tokens.set_color("success", &color.success.base().to_hex());
        tokens.set_color("warning", &color.warning.base().to_hex());
        tokens.set_color("error", &color.error.base().to_hex());
        tokens.set_color("info", &color.info.base().to_hex());

        // 设置文本颜色
        tokens.set_color("text-primary", &color.text.primary.to_hex());
        tokens.set_color("text-secondary", &color.text.secondary.to_hex());
        tokens.set_color("text-disabled", &color.text.disabled.to_hex());
        tokens.set_color("text-inverse", &color.text.inverse.to_hex());

        // 设置背景颜色
        tokens.set_color("bg-primary", &color.background.primary.to_hex());
        tokens.set_color("bg-secondary", &color.background.secondary.to_hex());
        tokens.set_color("bg-container", &color.background.container.to_hex());
        tokens.set_color("bg-elevated", &color.background.elevated.to_hex());
        tokens.set_color("bg-layout", &color.background.layout.to_hex());

        // 设置边框颜色
        tokens.set_color("border-base", &color.border.base.to_hex());
        tokens.set_color("border-split", &color.border.split.to_hex());

        // 设置填充颜色
        tokens.set_color("fill-primary", &color.fill.primary.to_hex());
        tokens.set_color("fill-secondary", &color.fill.secondary.to_hex());
        tokens.set_color("fill-tertiary", &color.fill.tertiary.to_hex());
        tokens.set_color("fill-quaternary", &color.fill.quaternary.to_hex());
    }

    /// 转换尺寸令牌
    fn convert_size_tokens(tokens: &mut DesignTokens, size: &SizeConfig) {
        // 设置边框圆角
        tokens.set_size("border-radius-xs", &format!("{}px", size.border_radius.xs));
        tokens.set_size("border-radius-sm", &format!("{}px", size.border_radius.sm));
        tokens.set_size(
            "border-radius-base",
            &format!("{}px", size.border_radius.base),
        );
        tokens.set_size("border-radius-lg", &format!("{}px", size.border_radius.lg));

        // 设置间距
        tokens.set_size("spacing-xs", &format!("{}px", size.spacing.xs));
        tokens.set_size("spacing-sm", &format!("{}px", size.spacing.sm));
        tokens.set_size("spacing-base", &format!("{}px", size.spacing.base));
        tokens.set_size("spacing-lg", &format!("{}px", size.spacing.lg));
        tokens.set_size("spacing-xl", &format!("{}px", size.spacing.xl));
    }

    /// 转换排版令牌
    fn convert_typography_tokens(tokens: &mut DesignTokens, typography: &TypographyConfig) {
        // 设置字体家族
        tokens.set_font_family("base", &typography.font_family.base);
        tokens.set_font_family("code", &typography.font_family.code);

        // 设置字体大小
        tokens.set_size("font-size-xs", &format!("{}px", typography.font_size.xs));
        tokens.set_size("font-size-sm", &format!("{}px", typography.font_size.sm));
        tokens.set_size(
            "font-size-base",
            &format!("{}px", typography.font_size.base),
        );
        tokens.set_size("font-size-lg", &format!("{}px", typography.font_size.lg));
        tokens.set_size("font-size-xl", &format!("{}px", typography.font_size.xl));

        // 设置行高
        tokens.set_number("line-height-base", typography.line_height.base);
        tokens.set_number("line-height-sm", typography.line_height.sm);
        tokens.set_number("line-height-lg", typography.line_height.lg);
    }

    /// 转换阴影令牌
    fn convert_shadow_tokens(tokens: &mut DesignTokens, shadow: &ShadowConfig) {
        tokens.set_shadow("box-shadow-sm", &shadow.box_shadow.sm);
        tokens.set_shadow("box-shadow-base", &shadow.box_shadow.base);
        tokens.set_shadow("box-shadow-lg", &shadow.box_shadow.lg);
    }

    /// 生成 CSS 类名
    pub fn generate_css_class(&self, styles: &str) -> String {
        // 注意：这里需要实际的 CSS-in-Rust 处理逻辑
        // 目前返回原始样式字符串作为占位符
        format!("css-{}", styles.replace(' ', "-").replace(':', "-"))
    }

    /// 创建主题提供者
    pub fn create_theme_provider(&self) -> ThemeProvider {
        ThemeProvider::new(self.css_theme.clone())
    }
}

/// 创建默认的 CSS 主题桥接器
pub fn create_default_css_theme_bridge() -> CssThemeBridge {
    let theme_config = ThemeConfig::default();
    CssThemeBridge::new(theme_config)
}

/// 为指定主题创建 CSS 主题桥接器
pub fn create_css_theme_bridge_for_theme(theme: Theme) -> CssThemeBridge {
    let mut theme_config = ThemeConfig::default();
    theme_config.theme = theme;

    // 根据主题类型调整配置
    match theme {
        Theme::Dark => {
            // 调整为暗色主题的颜色
            theme_config.colors.background.primary = RgbColor::new(20, 20, 20);
            theme_config.colors.background.secondary = RgbColor::new(30, 30, 30);
            theme_config.colors.text.primary = RgbColor::new(255, 255, 255);
            theme_config.colors.text.secondary = RgbColor::new(200, 200, 200);
        }
        Theme::Compact => {
            // 调整为紧凑主题的尺寸
            theme_config.sizes.spacing.xs = 2;
            theme_config.sizes.spacing.sm = 4;
            theme_config.sizes.spacing.base = 6;
            theme_config.sizes.spacing.lg = 8;
            theme_config.sizes.spacing.xl = 12;
        }
        _ => {}
    }

    CssThemeBridge::new(theme_config)
}

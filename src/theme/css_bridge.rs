//! CSS-in-Rust 桥接模块
//!
//! 提供 ant-design-dioxus 主题系统与 css-in-rust 的桥接功能

use css_in_rust::runtime::StyleInjector;
use css_in_rust::theme::{DesignTokens, Theme as CssTheme, ThemeMode, ThemeProvider};
// 注意：css! 宏需要在启用 proc-macro 特性时才可用
use std::collections::HashMap;

use crate::theme::{ColorTheme, SizeTheme, Theme, ThemeConfig};
use crate::utils::color::RgbColor;

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

    /// 注入全局主题变量到 DOM
    ///
    /// 将主题配置转换为 CSS 自定义属性并注入到页面中
    /// 这些变量可以在组件样式中通过 var(--variable-name) 使用
    pub fn inject_theme_variables(&self) {
        let css_vars = self.generate_css_variables();
        let injector = StyleInjector::new();
        injector
            .inject_style(&css_vars, "ant-design-theme-vars")
            .ok();
    }

    /// 生成 CSS 自定义属性
    ///
    /// 根据当前主题配置生成完整的 CSS 变量定义
    /// 包括颜色、尺寸、字体等所有设计令牌
    fn generate_css_variables(&self) -> String {
        let colors = &self.theme_config.colors;
        let sizes = &self.theme_config.sizes;

        format!(
            r#":root {{
                /* 主色调 */
                --ant-primary-color: {};
                --ant-primary-color-hover: {};
                --ant-primary-color-active: {};

                /* 功能色 */
                --ant-success-color: {};
                --ant-warning-color: {};
                --ant-error-color: {};
                --ant-info-color: {};

                /* 文本颜色 */
                --ant-text-color: {};
                --ant-text-color-secondary: {};
                --ant-text-color-disabled: {};

                /* 背景颜色 */
                --ant-bg-color: {};
                --ant-bg-color-container: {};

                /* 边框 */
                --ant-border-color: {};
                --ant-border-radius: {}px;
                --ant-border-radius-sm: {}px;
                --ant-border-radius-lg: {}px;

                /* 字体 */
                --ant-font-size-base: {}px;
                --ant-font-size-sm: {}px;
                --ant-font-size-lg: {}px;
                --ant-line-height-base: {};

                /* 间距 */
                --ant-padding-xs: {}px;
                --ant-padding-sm: {}px;
                --ant-padding-md: {}px;
                --ant-padding-lg: {}px;
                --ant-padding-xl: {}px;
            }}

            /* 暗色主题变量覆盖 */
            [data-theme="dark"] {{
                --ant-text-color: #ffffff;
                --ant-text-color-secondary: rgba(255, 255, 255, 0.65);
                --ant-text-color-disabled: rgba(255, 255, 255, 0.25);
                --ant-bg-color: #141414;
                --ant-bg-color-container: #1f1f1f;
                --ant-border-color: #424242;
            }}

            /* 紧凑主题变量覆盖 */
            [data-theme="compact"] {{
                --ant-padding-xs: 2px;
                --ant-padding-sm: 4px;
                --ant-padding-md: 8px;
                --ant-padding-lg: 12px;
                --ant-padding-xl: 16px;
                --ant-font-size-base: 12px;
                --ant-border-radius: 4px;
            }}"#,
            colors.primary.base.to_hex(),
            colors
                .primary
                .variants
                .get(5)
                .unwrap_or(&colors.primary.base)
                .to_hex(),
            colors
                .primary
                .variants
                .get(7)
                .unwrap_or(&colors.primary.base)
                .to_hex(),
            colors.success.base.to_hex(),
            colors.warning.base.to_hex(),
            colors.error.base.to_hex(),
            colors.info.base.to_hex(),
            colors.text.primary.to_hex(),
            colors.text.secondary.to_hex(),
            colors.text.disabled.to_hex(),
            colors.background.primary.to_hex(),
            colors.background.container.to_hex(),
            colors.border.base.to_hex(),
            sizes.border_radius.base,
            sizes.border_radius.sm,
            sizes.border_radius.lg,
            sizes.font_sizes.base,
            sizes.font_sizes.sm,
            sizes.font_sizes.lg,
            sizes.line_heights.base,
            sizes.spacing.xs,
            sizes.spacing.sm,
            sizes.spacing.base,
            sizes.spacing.lg,
            sizes.spacing.xl
        )
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
        Self::convert_typography_tokens(&mut design_tokens, &theme_config.sizes);

        // 转换阴影令牌
        Self::convert_shadow_tokens(&mut design_tokens, &theme_config.sizes);

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
        tokens.colors.primary = color.primary.base.to_hex();
        tokens.colors.success = color.success.base.to_hex();
        tokens.colors.warning = color.warning.base.to_hex();
        tokens.colors.error = color.error.base.to_hex();
        tokens.colors.info = color.info.base.to_hex();

        // 设置文本颜色
        tokens.colors.text.primary = color.text.primary.to_hex();
        tokens.colors.text.secondary = color.text.secondary.to_hex();
        tokens.colors.text.disabled = color.text.disabled.to_hex();
        tokens.colors.text.inverse = color.text.inverse.to_hex();

        // 设置背景颜色
        tokens.colors.background.primary = color.background.primary.to_hex();
        tokens.colors.background.secondary = color.background.secondary.to_hex();
        tokens.colors.background.tertiary = color.background.container.to_hex();
        tokens.colors.background.inverse = color.background.elevated.to_hex();

        // 设置边框颜色
        tokens.colors.border.primary = color.border.base.to_hex();
        tokens.colors.border.secondary = color.border.split.to_hex();
        tokens.colors.border.inverse = color.border.base.to_hex();
    }

    /// 转换尺寸令牌
    fn convert_size_tokens(tokens: &mut DesignTokens, size: &SizeTheme) {
        // 设置边框圆角
        tokens.borders.radius.sm = format!("{}px", size.border_radius.xs);
        tokens.borders.radius.md = format!("{}px", size.border_radius.sm);
        tokens.borders.radius.lg = format!("{}px", size.border_radius.base);
        tokens.borders.radius.xl = format!("{}px", size.border_radius.lg);

        // 设置间距
        tokens.spacing.xs = format!("{}px", size.spacing.xs);
        tokens.spacing.sm = format!("{}px", size.spacing.sm);
        tokens.spacing.md = format!("{}px", size.spacing.base);
        tokens.spacing.lg = format!("{}px", size.spacing.lg);
        tokens.spacing.xl = format!("{}px", size.spacing.xl);
    }

    /// 转换排版令牌
    fn convert_typography_tokens(tokens: &mut DesignTokens, size_theme: &SizeTheme) {
        // 设置字体家族 - 使用默认值
        tokens.typography.font_family.sans = "system-ui, -apple-system, sans-serif".to_string();
        tokens.typography.font_family.mono = "ui-monospace, monospace".to_string();

        // 设置字体大小
        tokens.typography.font_size.xs = format!("{}px", size_theme.font_sizes.xs);
        tokens.typography.font_size.sm = format!("{}px", size_theme.font_sizes.sm);
        tokens.typography.font_size.md = format!("{}px", size_theme.font_sizes.base);
        tokens.typography.font_size.lg = format!("{}px", size_theme.font_sizes.lg);
        tokens.typography.font_size.xl = format!("{}px", size_theme.font_sizes.xl);

        // 设置行高
        tokens.typography.line_height.normal = size_theme.line_heights.base.to_string();
        tokens.typography.line_height.tight = size_theme.line_heights.sm.to_string();
        tokens.typography.line_height.relaxed = size_theme.line_heights.heading.to_string();
    }

    /// 转换阴影令牌
    fn convert_shadow_tokens(tokens: &mut DesignTokens, size_theme: &SizeTheme) {
        tokens.shadows.sm = size_theme.shadows.sm.clone();
        tokens.shadows.md = size_theme.shadows.base.clone();
        tokens.shadows.lg = size_theme.shadows.lg.clone();
    }

    /// 生成 CSS 类名
    pub fn generate_css_class(&self, styles: &str) -> String {
        // 注意：这里需要实际的 CSS-in-Rust 处理逻辑
        // 目前返回原始样式字符串作为占位符
        format!("css-{}", styles.replace(' ', "-").replace(':', "-"))
    }

    /// 创建主题提供者
    pub fn create_theme_provider(&self) -> ThemeProvider {
        ThemeProvider::new()
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

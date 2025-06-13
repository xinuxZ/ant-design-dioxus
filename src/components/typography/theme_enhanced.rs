//!
//! Typography组件的增强主题支持
//! 提供动态主题切换、主题令牌和样式生成功能

use crate::theme::{use_theme, Theme, ThemeConfig};
use css_in_rust::css;
use std::collections::HashMap;

/// Typography主题令牌
#[derive(Debug, Clone, PartialEq)]
pub struct TypographyThemeTokens {
    /// 主色调
    pub primary_color: String,
    /// 文本颜色
    pub text_color: String,
    /// 次要文本颜色
    pub text_color_secondary: String,
    /// 禁用文本颜色
    pub text_color_disabled: String,
    /// 成功颜色
    pub success_color: String,
    /// 警告颜色
    pub warning_color: String,
    /// 错误颜色
    pub error_color: String,
    /// 链接颜色
    pub link_color: String,
    /// 链接悬停颜色
    pub link_hover_color: String,
    /// 背景颜色
    pub background_color: String,
    /// 边框颜色
    pub border_color: String,
    /// 代码背景颜色
    pub code_bg_color: String,
    /// 标记背景颜色
    pub mark_bg_color: String,
    /// 键盘背景颜色
    pub keyboard_bg_color: String,
    /// 字体大小
    pub font_size: String,
    /// 行高
    pub line_height: String,
    /// 字体族
    pub font_family: String,
    /// 边框半径
    pub border_radius: String,
    /// 阴影
    pub box_shadow: String,
}

impl Default for TypographyThemeTokens {
    fn default() -> Self {
        Self {
            primary_color: "#1677ff".to_string(),
            text_color: "rgba(0, 0, 0, 0.88)".to_string(),
            text_color_secondary: "rgba(0, 0, 0, 0.65)".to_string(),
            text_color_disabled: "rgba(0, 0, 0, 0.25)".to_string(),
            success_color: "#52c41a".to_string(),
            warning_color: "#faad14".to_string(),
            error_color: "#ff4d4f".to_string(),
            link_color: "#1677ff".to_string(),
            link_hover_color: "#69b1ff".to_string(),
            background_color: "#ffffff".to_string(),
            border_color: "#d9d9d9".to_string(),
            code_bg_color: "rgba(0, 0, 0, 0.06)".to_string(),
            mark_bg_color: "#ffe58f".to_string(),
            keyboard_bg_color: "#fafafa".to_string(),
            font_size: "14px".to_string(),
            line_height: "1.5715".to_string(),
            font_family: "-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Noto Sans', sans-serif".to_string(),
            border_radius: "6px".to_string(),
            box_shadow: "0 2px 8px rgba(0, 0, 0, 0.15)".to_string(),
        }
    }
}

/// 根据主题类型生成Typography主题令牌
pub fn get_typography_theme_tokens(theme: &Theme) -> TypographyThemeTokens {
    match theme {
        Theme::Light => TypographyThemeTokens::default(),
        Theme::Dark => TypographyThemeTokens {
            text_color: "rgba(255, 255, 255, 0.85)".to_string(),
            text_color_secondary: "rgba(255, 255, 255, 0.65)".to_string(),
            text_color_disabled: "rgba(255, 255, 255, 0.25)".to_string(),
            background_color: "#141414".to_string(),
            border_color: "#424242".to_string(),
            code_bg_color: "rgba(255, 255, 255, 0.06)".to_string(),
            keyboard_bg_color: "#1f1f1f".to_string(),
            ..TypographyThemeTokens::default()
        },
        Theme::Compact => TypographyThemeTokens {
            font_size: "12px".to_string(),
            line_height: "1.5".to_string(),
            border_radius: "4px".to_string(),
            ..TypographyThemeTokens::default()
        },
        Theme::Custom => TypographyThemeTokens::default(),
    }
}

/// Typography主题样式生成器
#[derive(Clone)]
pub struct TypographyThemeStyleGenerator {
    tokens: TypographyThemeTokens,
    theme_type: Theme,
}

impl TypographyThemeStyleGenerator {
    /// 创建新的主题样式生成器
    pub fn new(theme_type: Theme) -> Self {
        let tokens = get_typography_theme_tokens(&theme_type);
        Self {
            tokens,
            theme_type,
        }
    }

    /// 从主题配置创建
    pub fn from_config(config: &ThemeConfig) -> Self {
        let theme_type = config.theme_type.clone();
        let mut tokens = get_typography_theme_tokens(&theme_type);
        
        // 应用自定义令牌
        if let Some(primary) = config.token.get("colorPrimary") {
            tokens.primary_color = primary.clone();
            tokens.link_color = primary.clone();
        }
        
        if let Some(text) = config.token.get("colorText") {
            tokens.text_color = text.clone();
        }
        
        if let Some(bg) = config.token.get("colorBgContainer") {
            tokens.background_color = bg.clone();
        }
        
        Self {
            tokens,
            theme_type,
        }
    }

    /// 生成CSS变量
    pub fn generate_css_variables(&self) -> String {
        format!(
            r#"
            --ant-typography-primary-color: {};
            --ant-typography-text-color: {};
            --ant-typography-text-color-secondary: {};
            --ant-typography-text-color-disabled: {};
            --ant-typography-success-color: {};
            --ant-typography-warning-color: {};
            --ant-typography-error-color: {};
            --ant-typography-link-color: {};
            --ant-typography-link-hover-color: {};
            --ant-typography-background-color: {};
            --ant-typography-border-color: {};
            --ant-typography-code-bg-color: {};
            --ant-typography-mark-bg-color: {};
            --ant-typography-keyboard-bg-color: {};
            --ant-typography-font-size: {};
            --ant-typography-line-height: {};
            --ant-typography-font-family: {};
            --ant-typography-border-radius: {};
            --ant-typography-box-shadow: {};
            "#,
            self.tokens.primary_color,
            self.tokens.text_color,
            self.tokens.text_color_secondary,
            self.tokens.text_color_disabled,
            self.tokens.success_color,
            self.tokens.warning_color,
            self.tokens.error_color,
            self.tokens.link_color,
            self.tokens.link_hover_color,
            self.tokens.background_color,
            self.tokens.border_color,
            self.tokens.code_bg_color,
            self.tokens.mark_bg_color,
            self.tokens.keyboard_bg_color,
            self.tokens.font_size,
            self.tokens.line_height,
            self.tokens.font_family,
            self.tokens.border_radius,
            self.tokens.box_shadow,
        )
    }

    /// 生成主题相关的CSS类
    pub fn generate_theme_classes(&self) -> String {
        let base_class = match self.theme_type {
            Theme::Light => "ant-typography-light",
            Theme::Dark => "ant-typography-dark",
            Theme::Compact => "ant-typography-compact",
            Theme::Custom => "ant-typography-custom",
        };

        format!(
            r#"
            .{} {{
                {}
            }}
            
            .{} .ant-typography-copy-btn {{
                color: var(--ant-typography-text-color-secondary);
                background: transparent;
                border: 1px solid var(--ant-typography-border-color);
                border-radius: var(--ant-typography-border-radius);
            }}
            
            .{} .ant-typography-copy-btn:hover {{
                color: var(--ant-typography-primary-color);
                border-color: var(--ant-typography-primary-color);
            }}
            
            .{} .ant-typography-edit-btn {{
                color: var(--ant-typography-text-color-secondary);
                background: transparent;
                border: 1px solid var(--ant-typography-border-color);
                border-radius: var(--ant-typography-border-radius);
            }}
            
            .{} .ant-typography-edit-btn:hover {{
                color: var(--ant-typography-primary-color);
                border-color: var(--ant-typography-primary-color);
            }}
            
            .{} .ant-typography-edit-input {{
                color: var(--ant-typography-text-color);
                background: var(--ant-typography-background-color);
                border: 1px solid var(--ant-typography-border-color);
                border-radius: var(--ant-typography-border-radius);
                font-family: var(--ant-typography-font-family);
                font-size: var(--ant-typography-font-size);
                line-height: var(--ant-typography-line-height);
            }}
            
            .{} .ant-typography-edit-input:focus {{
                border-color: var(--ant-typography-primary-color);
                box-shadow: 0 0 0 2px rgba(22, 119, 255, 0.2);
                outline: none;
            }}
            "#,
            base_class,
            self.generate_css_variables(),
            base_class,
            base_class,
            base_class,
            base_class,
            base_class,
            base_class,
        )
    }

    /// 获取主题令牌
    pub fn get_tokens(&self) -> &TypographyThemeTokens {
        &self.tokens
    }

    /// 获取主题类型
    pub fn get_theme_type(&self) -> &Theme {
        &self.theme_type
    }
}

/// Typography主题Hook
pub fn use_typography_theme() -> TypographyThemeStyleGenerator {
    let theme_context = use_theme();
    TypographyThemeStyleGenerator::from_config(&theme_context.config)
}

/// 主题切换Hook
pub fn use_typography_theme_switch() -> impl Fn(Theme) {
    let theme_context = use_theme();
    let switch_theme = theme_context.switch_theme.clone();
    
    move |new_theme: Theme| {
        let mut new_config = ThemeConfig::default();
        new_config.theme_type = new_theme;
        // 这里需要调用switch_theme，但由于Arc<dyn FnMut>的限制，需要特殊处理
        // 实际使用时可能需要通过其他方式实现
    }
}

/// 生成响应式主题样式
pub fn generate_responsive_theme_styles(tokens: &TypographyThemeTokens) -> String {
    format!(
        r#"
        @media (max-width: 768px) {{
            .ant-typography {{
                font-size: calc({} * 0.9);
                line-height: calc({} * 0.95);
            }}
        }}
        
        @media (max-width: 480px) {{
            .ant-typography {{
                font-size: calc({} * 0.8);
                line-height: calc({} * 0.9);
            }}
        }}
        
        @media (prefers-reduced-motion: reduce) {{
            .ant-typography * {{
                transition: none !important;
                animation: none !important;
            }}
        }}
        
        @media (prefers-color-scheme: dark) {{
            .ant-typography:not([data-theme]) {{
                color: rgba(255, 255, 255, 0.85);
                background: #141414;
            }}
        }}
        "#,
        tokens.font_size,
        tokens.line_height,
        tokens.font_size,
        tokens.line_height,
    )
}
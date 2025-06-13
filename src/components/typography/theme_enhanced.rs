//!
//! Typography组件的主题系统
//! 基于Ant Design v5的Design Token架构设计
//! 支持Seed Token、Map Token、Alias Token三层架构

use crate::theme::{use_theme, Theme, ThemeConfig};
use css_in_rust::css;
use std::collections::HashMap;

/// Seed Tokens - 基础设计令牌
/// 这些是最基础的设计变量，影响范围最大
#[derive(Debug, Clone, PartialEq)]
pub struct TypographySeedTokens {
    /// 主色调 - 影响链接、强调等
    pub color_primary: String,
    /// 成功色
    pub color_success: String,
    /// 警告色
    pub color_warning: String,
    /// 错误色
    pub color_error: String,
    /// 信息色
    pub color_info: String,
    /// 字体族
    pub font_family: String,
    /// 基础字体大小
    pub font_size: f32,
    /// 基础行高
    pub line_height: f32,
    /// 基础边框半径
    pub border_radius: f32,
}

impl Default for TypographySeedTokens {
    fn default() -> Self {
        Self {
            color_primary: "#1677ff".to_string(),
            color_success: "#52c41a".to_string(),
            color_warning: "#faad14".to_string(),
            color_error: "#ff4d4f".to_string(),
            color_info: "#1677ff".to_string(),
            font_family: "-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Noto Sans', sans-serif".to_string(),
            font_size: 14.0,
            line_height: 1.5715,
            border_radius: 6.0,
        }
    }
}

/// Map Tokens - 映射令牌
/// 从Seed Token通过算法派生的中间层令牌
#[derive(Debug, Clone, PartialEq)]
pub struct TypographyMapTokens {
    // 颜色相关
    pub color_text: String,
    pub color_text_secondary: String,
    pub color_text_tertiary: String,
    pub color_text_quaternary: String,
    pub color_text_disabled: String,
    pub color_bg_container: String,
    pub color_bg_elevated: String,
    pub color_border: String,
    pub color_border_secondary: String,
    
    // 字体相关
    pub font_size_sm: f32,
    pub font_size_lg: f32,
    pub font_size_xl: f32,
    pub font_size_heading_1: f32,
    pub font_size_heading_2: f32,
    pub font_size_heading_3: f32,
    pub font_size_heading_4: f32,
    pub font_size_heading_5: f32,
    
    // 行高相关
    pub line_height_sm: f32,
    pub line_height_lg: f32,
    pub line_height_heading: f32,
    
    // 间距相关
    pub margin_xs: f32,
    pub margin_sm: f32,
    pub margin_md: f32,
    pub margin_lg: f32,
    pub margin_xl: f32,
    
    // 其他
    pub border_radius_sm: f32,
    pub border_radius_lg: f32,
}

/// Alias Tokens - 别名令牌
/// Typography组件特有的语义化令牌
#[derive(Debug, Clone, PartialEq)]
pub struct TypographyAliasTokens {
    // Typography特有的颜色
    pub color_link: String,
    pub color_link_hover: String,
    pub color_link_active: String,
    pub color_text_heading: String,
    pub color_text_label: String,
    pub color_text_description: String,
    
    // 特殊元素颜色
    pub color_code_bg: String,
    pub color_mark_bg: String,
    pub color_kbd_bg: String,
    pub color_kbd_border: String,
    
    // Typography特有的字体
    pub font_family_code: String,
    
    // Typography特有的尺寸
    pub title_margin_top: f32,
    pub title_margin_bottom: f32,
    pub paragraph_margin_bottom: f32,
}

/// Typography完整的Design Token集合
#[derive(Debug, Clone, PartialEq)]
pub struct TypographyDesignTokens {
    pub seed: TypographySeedTokens,
    pub map: TypographyMapTokens,
    pub alias: TypographyAliasTokens,
}

/// 主题算法 - 用于从Seed Token生成Map Token
pub trait ThemeAlgorithm {
    fn generate_map_tokens(&self, seed: &TypographySeedTokens) -> TypographyMapTokens;
    fn generate_alias_tokens(&self, seed: &TypographySeedTokens, map: &TypographyMapTokens) -> TypographyAliasTokens;
}

/// 默认主题算法
pub struct DefaultAlgorithm;

impl ThemeAlgorithm for DefaultAlgorithm {
    fn generate_map_tokens(&self, seed: &TypographySeedTokens) -> TypographyMapTokens {
        TypographyMapTokens {
            color_text: "rgba(0, 0, 0, 0.88)".to_string(),
            color_text_secondary: "rgba(0, 0, 0, 0.65)".to_string(),
            color_text_tertiary: "rgba(0, 0, 0, 0.45)".to_string(),
            color_text_quaternary: "rgba(0, 0, 0, 0.25)".to_string(),
            color_text_disabled: "rgba(0, 0, 0, 0.25)".to_string(),
            color_bg_container: "#ffffff".to_string(),
            color_bg_elevated: "#ffffff".to_string(),
            color_border: "#d9d9d9".to_string(),
            color_border_secondary: "#f0f0f0".to_string(),
            
            font_size_sm: seed.font_size * 0.875,
            font_size_lg: seed.font_size * 1.125,
            font_size_xl: seed.font_size * 1.25,
            font_size_heading_1: seed.font_size * 2.5,
            font_size_heading_2: seed.font_size * 2.0,
            font_size_heading_3: seed.font_size * 1.75,
            font_size_heading_4: seed.font_size * 1.5,
            font_size_heading_5: seed.font_size * 1.25,
            
            line_height_sm: 1.5,
            line_height_lg: 1.5715,
            line_height_heading: 1.35,
            
            margin_xs: 4.0,
            margin_sm: 8.0,
            margin_md: 16.0,
            margin_lg: 24.0,
            margin_xl: 32.0,
            
            border_radius_sm: seed.border_radius * 0.67,
            border_radius_lg: seed.border_radius * 1.33,
        }
    }
    
    fn generate_alias_tokens(&self, seed: &TypographySeedTokens, map: &TypographyMapTokens) -> TypographyAliasTokens {
        TypographyAliasTokens {
            color_link: seed.color_primary.clone(),
            color_link_hover: lighten_color(&seed.color_primary, 0.2),
            color_link_active: darken_color(&seed.color_primary, 0.1),
            color_text_heading: map.color_text.clone(),
            color_text_label: map.color_text.clone(),
            color_text_description: map.color_text_secondary.clone(),
            
            color_code_bg: "rgba(0, 0, 0, 0.06)".to_string(),
            color_mark_bg: "#ffe58f".to_string(),
            color_kbd_bg: "#fafafa".to_string(),
            color_kbd_border: map.color_border.clone(),
            
            font_family_code: "'SFMono-Regular', Consolas, 'Liberation Mono', Menlo, Courier, monospace".to_string(),
            
            title_margin_top: map.margin_lg,
            title_margin_bottom: map.margin_md,
            paragraph_margin_bottom: map.margin_md,
        }
    }
}

/// 暗色主题算法
pub struct DarkAlgorithm;

impl ThemeAlgorithm for DarkAlgorithm {
    fn generate_map_tokens(&self, seed: &TypographySeedTokens) -> TypographyMapTokens {
        let mut tokens = DefaultAlgorithm.generate_map_tokens(seed);
        
        // 暗色主题特有的颜色调整
        tokens.color_text = "rgba(255, 255, 255, 0.85)".to_string();
        tokens.color_text_secondary = "rgba(255, 255, 255, 0.65)".to_string();
        tokens.color_text_tertiary = "rgba(255, 255, 255, 0.45)".to_string();
        tokens.color_text_quaternary = "rgba(255, 255, 255, 0.25)".to_string();
        tokens.color_text_disabled = "rgba(255, 255, 255, 0.25)".to_string();
        tokens.color_bg_container = "#141414".to_string();
        tokens.color_bg_elevated = "#1f1f1f".to_string();
        tokens.color_border = "#424242".to_string();
        tokens.color_border_secondary = "#303030".to_string();
        
        tokens
    }
    
    fn generate_alias_tokens(&self, seed: &TypographySeedTokens, map: &TypographyMapTokens) -> TypographyAliasTokens {
        let mut tokens = DefaultAlgorithm.generate_alias_tokens(seed, map);
        
        // 暗色主题特有的调整
        tokens.color_code_bg = "rgba(255, 255, 255, 0.06)".to_string();
        tokens.color_kbd_bg = "#1f1f1f".to_string();
        
        tokens
    }
}

/// 紧凑主题算法
pub struct CompactAlgorithm;

impl ThemeAlgorithm for CompactAlgorithm {
    fn generate_map_tokens(&self, seed: &TypographySeedTokens) -> TypographyMapTokens {
        let mut tokens = DefaultAlgorithm.generate_map_tokens(seed);
        
        // 紧凑主题的尺寸调整
        tokens.font_size_sm = seed.font_size * 0.75;
        tokens.font_size_lg = seed.font_size;
        tokens.font_size_xl = seed.font_size * 1.125;
        tokens.font_size_heading_1 = seed.font_size * 2.0;
        tokens.font_size_heading_2 = seed.font_size * 1.75;
        tokens.font_size_heading_3 = seed.font_size * 1.5;
        tokens.font_size_heading_4 = seed.font_size * 1.25;
        tokens.font_size_heading_5 = seed.font_size * 1.125;
        
        tokens.line_height_sm = 1.4;
        tokens.line_height_lg = 1.5;
        tokens.line_height_heading = 1.3;
        
        tokens.margin_xs = 2.0;
        tokens.margin_sm = 4.0;
        tokens.margin_md = 8.0;
        tokens.margin_lg = 12.0;
        tokens.margin_xl = 16.0;
        
        tokens.border_radius_sm = seed.border_radius * 0.5;
        tokens.border_radius_lg = seed.border_radius;
        
        tokens
    }
    
    fn generate_alias_tokens(&self, seed: &TypographySeedTokens, map: &TypographyMapTokens) -> TypographyAliasTokens {
        let mut tokens = DefaultAlgorithm.generate_alias_tokens(seed, map);
        
        // 紧凑主题的间距调整
        tokens.title_margin_top = map.margin_md;
        tokens.title_margin_bottom = map.margin_sm;
        tokens.paragraph_margin_bottom = map.margin_sm;
        
        tokens
    }
}

// 颜色工具函数
fn lighten_color(color: &str, amount: f32) -> String {
    // 简化实现，实际应该解析颜色并调整亮度
    if color == "#1677ff" {
        "#69b1ff".to_string()
    } else {
        color.to_string()
    }
}

fn darken_color(color: &str, amount: f32) -> String {
    // 简化实现，实际应该解析颜色并调整亮度
    if color == "#1677ff" {
        "#0958d9".to_string()
    } else {
        color.to_string()
    }
}

/// Typography主题提供者
/// 负责生成完整的Design Token集合和样式
#[derive(Clone)]
pub struct TypographyThemeProvider {
    design_tokens: TypographyDesignTokens,
    theme_type: Theme,
}

impl TypographyThemeProvider {
    /// 创建新的主题提供者
    pub fn new(theme_type: Theme) -> Self {
        let seed = TypographySeedTokens::default();
        let algorithm: Box<dyn ThemeAlgorithm> = match theme_type {
            Theme::Light => Box::new(DefaultAlgorithm),
            Theme::Dark => Box::new(DarkAlgorithm),
            Theme::Compact => Box::new(CompactAlgorithm),
            Theme::Custom => Box::new(DefaultAlgorithm),
        };
        
        let map = algorithm.generate_map_tokens(&seed);
        let alias = algorithm.generate_alias_tokens(&seed, &map);
        
        Self {
            design_tokens: TypographyDesignTokens { seed, map, alias },
            theme_type,
        }
    }

    /// 从主题配置和自定义Seed Token创建
    pub fn from_config(config: &ThemeConfig) -> Self {
        let theme_type = config.theme_type.clone();
        let mut seed = TypographySeedTokens::default();
        
        // 应用自定义Seed Token
        if let Some(primary) = config.token.get("colorPrimary") {
            seed.color_primary = primary.clone();
        }
        if let Some(success) = config.token.get("colorSuccess") {
            seed.color_success = success.clone();
        }
        if let Some(warning) = config.token.get("colorWarning") {
            seed.color_warning = warning.clone();
        }
        if let Some(error) = config.token.get("colorError") {
            seed.color_error = error.clone();
        }
        if let Some(font_size) = config.token.get("fontSize") {
            if let Ok(size) = font_size.parse::<f32>() {
                seed.font_size = size;
            }
        }
        if let Some(line_height) = config.token.get("lineHeight") {
            if let Ok(height) = line_height.parse::<f32>() {
                seed.line_height = height;
            }
        }
        if let Some(border_radius) = config.token.get("borderRadius") {
            if let Ok(radius) = border_radius.parse::<f32>() {
                seed.border_radius = radius;
            }
        }
        
        let algorithm: Box<dyn ThemeAlgorithm> = match theme_type {
            Theme::Light => Box::new(DefaultAlgorithm),
            Theme::Dark => Box::new(DarkAlgorithm),
            Theme::Compact => Box::new(CompactAlgorithm),
            Theme::Custom => Box::new(DefaultAlgorithm),
        };
        
        let map = algorithm.generate_map_tokens(&seed);
        let alias = algorithm.generate_alias_tokens(&seed, &map);
        
        Self {
            design_tokens: TypographyDesignTokens { seed, map, alias },
            theme_type,
        }
    }

    /// 生成CSS变量
    pub fn generate_css_variables(&self) -> String {
        let seed = &self.design_tokens.seed;
        let map = &self.design_tokens.map;
        let alias = &self.design_tokens.alias;
        
        format!(
            r#"
            /* Seed Tokens */
            --ant-color-primary: {};
            --ant-color-success: {};
            --ant-color-warning: {};
            --ant-color-error: {};
            --ant-color-info: {};
            --ant-font-family: {};
            --ant-font-size: {}px;
            --ant-line-height: {};
            --ant-border-radius: {}px;
            
            /* Map Tokens */
            --ant-color-text: {};
            --ant-color-text-secondary: {};
            --ant-color-text-tertiary: {};
            --ant-color-text-quaternary: {};
            --ant-color-text-disabled: {};
            --ant-color-bg-container: {};
            --ant-color-bg-elevated: {};
            --ant-color-border: {};
            --ant-color-border-secondary: {};
            --ant-font-size-sm: {}px;
            --ant-font-size-lg: {}px;
            --ant-font-size-xl: {}px;
            --ant-font-size-heading-1: {}px;
            --ant-font-size-heading-2: {}px;
            --ant-font-size-heading-3: {}px;
            --ant-font-size-heading-4: {}px;
            --ant-font-size-heading-5: {}px;
            --ant-line-height-sm: {};
            --ant-line-height-lg: {};
            --ant-line-height-heading: {};
            --ant-margin-xs: {}px;
            --ant-margin-sm: {}px;
            --ant-margin-md: {}px;
            --ant-margin-lg: {}px;
            --ant-margin-xl: {}px;
            --ant-border-radius-sm: {}px;
            --ant-border-radius-lg: {}px;
            
            /* Alias Tokens */
            --ant-color-link: {};
            --ant-color-link-hover: {};
            --ant-color-link-active: {};
            --ant-color-text-heading: {};
            --ant-color-text-label: {};
            --ant-color-text-description: {};
            --ant-color-code-bg: {};
            --ant-color-mark-bg: {};
            --ant-color-kbd-bg: {};
            --ant-color-kbd-border: {};
            --ant-font-family-code: {};
            --ant-title-margin-top: {}px;
            --ant-title-margin-bottom: {}px;
            --ant-paragraph-margin-bottom: {}px;
            "#,
            seed.color_primary,
            seed.color_success,
            seed.color_warning,
            seed.color_error,
            seed.color_info,
            seed.font_family,
            seed.font_size,
            seed.line_height,
            seed.border_radius,
            
            map.color_text,
            map.color_text_secondary,
            map.color_text_tertiary,
            map.color_text_quaternary,
            map.color_text_disabled,
            map.color_bg_container,
            map.color_bg_elevated,
            map.color_border,
            map.color_border_secondary,
            map.font_size_sm,
            map.font_size_lg,
            map.font_size_xl,
            map.font_size_heading_1,
            map.font_size_heading_2,
            map.font_size_heading_3,
            map.font_size_heading_4,
            map.font_size_heading_5,
            map.line_height_sm,
            map.line_height_lg,
            map.line_height_heading,
            map.margin_xs,
            map.margin_sm,
            map.margin_md,
            map.margin_lg,
            map.margin_xl,
            map.border_radius_sm,
            map.border_radius_lg,
            
            alias.color_link,
            alias.color_link_hover,
            alias.color_link_active,
            alias.color_text_heading,
            alias.color_text_label,
            alias.color_text_description,
            alias.color_code_bg,
            alias.color_mark_bg,
            alias.color_kbd_bg,
            alias.color_kbd_border,
            alias.font_family_code,
            alias.title_margin_top,
            alias.title_margin_bottom,
            alias.paragraph_margin_bottom,
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

    /// 生成主题相关的CSS类
    pub fn generate_theme_styles(&self) -> String {
        format!(
            r#"
            .ant-typography {{
                color: var(--ant-color-text);
                font-family: var(--ant-font-family);
                font-size: var(--ant-font-size);
                line-height: var(--ant-line-height);
            }}
            
            .ant-typography h1 {{
                color: var(--ant-color-text-heading);
                font-size: var(--ant-font-size-heading-1);
                line-height: var(--ant-line-height-heading);
                font-weight: 600;
                margin-top: var(--ant-title-margin-top);
                margin-bottom: var(--ant-title-margin-bottom);
            }}
            
            .ant-typography h2 {{
                color: var(--ant-color-text-heading);
                font-size: var(--ant-font-size-heading-2);
                line-height: var(--ant-line-height-heading);
                font-weight: 600;
                margin-top: var(--ant-title-margin-top);
                margin-bottom: var(--ant-title-margin-bottom);
            }}
            
            .ant-typography h3 {{
                color: var(--ant-color-text-heading);
                font-size: var(--ant-font-size-heading-3);
                line-height: var(--ant-line-height-heading);
                font-weight: 600;
                margin-top: var(--ant-title-margin-top);
                margin-bottom: var(--ant-title-margin-bottom);
            }}
            
            .ant-typography h4 {{
                color: var(--ant-color-text-heading);
                font-size: var(--ant-font-size-heading-4);
                line-height: var(--ant-line-height-heading);
                font-weight: 600;
                margin-top: var(--ant-title-margin-top);
                margin-bottom: var(--ant-title-margin-bottom);
            }}
            
            .ant-typography h5 {{
                color: var(--ant-color-text-heading);
                font-size: var(--ant-font-size-heading-5);
                line-height: var(--ant-line-height-heading);
                font-weight: 600;
                margin-top: var(--ant-title-margin-top);
                margin-bottom: var(--ant-title-margin-bottom);
            }}
            
            .ant-typography h6 {{
                color: var(--ant-color-text-heading);
                font-size: var(--ant-font-size);
                line-height: var(--ant-line-height-heading);
                font-weight: 600;
                margin-top: var(--ant-title-margin-top);
                margin-bottom: var(--ant-title-margin-bottom);
            }}
            
            .ant-typography p {{
                margin-bottom: var(--ant-paragraph-margin-bottom);
            }}
            
            .ant-typography a {{
                color: var(--ant-color-link);
                text-decoration: none;
                transition: color 0.3s;
            }}
            
            .ant-typography a:hover {{
                color: var(--ant-color-link-hover);
            }}
            
            .ant-typography a:active {{
                color: var(--ant-color-link-active);
            }}
            
            .ant-typography code {{
                background-color: var(--ant-color-code-bg);
                border-radius: var(--ant-border-radius-sm);
                padding: 0.2em 0.4em;
                font-family: var(--ant-font-family-code);
                font-size: 0.85em;
            }}
            
            .ant-typography mark {{
                background-color: var(--ant-color-mark-bg);
                padding: 0.2em;
            }}
            
            .ant-typography kbd {{
                background-color: var(--ant-color-kbd-bg);
                border: 1px solid var(--ant-color-kbd-border);
                border-radius: var(--ant-border-radius-sm);
                padding: 0.2em 0.4em;
                font-family: var(--ant-font-family-code);
                font-size: 0.85em;
                box-shadow: 0 1px 0 var(--ant-color-kbd-border), 0 0 0 2px #fff inset;
            }}
            
            .ant-typography.ant-typography-secondary {{
                color: var(--ant-color-text-secondary);
            }}
            
            .ant-typography.ant-typography-disabled {{
                color: var(--ant-color-text-disabled);
                cursor: not-allowed;
            }}
            
            .ant-typography.ant-typography-success {{
                color: var(--ant-color-success);
            }}
            
            .ant-typography.ant-typography-warning {{
                color: var(--ant-color-warning);
            }}
            
            .ant-typography.ant-typography-danger {{
                color: var(--ant-color-error);
            }}
            "#
        )
    }

    /// 获取设计令牌
    pub fn get_design_tokens(&self) -> &TypographyDesignTokens {
        &self.design_tokens
    }

    /// 获取主题类型
    pub fn get_theme_type(&self) -> &Theme {
        &self.theme_type
    }
}

/// Typography主题Hook
pub fn use_typography_theme() -> TypographyThemeProvider {
    let theme_context = use_theme();
    TypographyThemeProvider::from_config(&theme_context.config)
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
pub fn generate_responsive_theme_styles(design_tokens: &TypographyDesignTokens) -> String {
    format!(
        r#"
        @media (max-width: 768px) {{
            .ant-typography {{
                font-size: calc({}px * 0.9);
                line-height: calc({} * 0.95);
            }}
        }}
        
        @media (max-width: 480px) {{
            .ant-typography {{
                font-size: calc({}px * 0.8);
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
        design_tokens.seed.font_size,
        design_tokens.seed.line_height,
        design_tokens.seed.font_size,
        design_tokens.seed.line_height,
    )
}
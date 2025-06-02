//! Ant Design 字体规范
//!
//! 实现 Ant Design 的字体系统，包括：
//! - 字体族
//! - 字体大小
//! - 行高
//! - 字重
//! - 字间距

use crate::theme::core::Size;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 字体族枚举
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FontFamily {
    /// 系统默认字体
    System,
    /// 等宽字体
    Monospace,
    /// 自定义字体
    Custom(String),
}

impl FontFamily {
    /// 转换为 CSS 字体族字符串
    pub fn to_css(&self) -> String {
        match self {
            FontFamily::System => "-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Noto Sans', sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol', 'Noto Color Emoji'".to_string(),
            FontFamily::Monospace => "'SFMono-Regular', Consolas, 'Liberation Mono', Menlo, Courier, monospace".to_string(),
            FontFamily::Custom(font) => font.clone(),
        }
    }
}

/// 字体大小枚举
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FontSize {
    /// 12px - 辅助文字
    XSmall,
    /// 14px - 标准文字
    Small,
    /// 16px - 小标题
    Base,
    /// 18px - 标题
    Large,
    /// 20px - 主标题
    XLarge,
    /// 24px - 大标题
    XXLarge,
    /// 30px - 特大标题
    XXXLarge,
    /// 38px - 超大标题
    Huge,
    /// 自定义大小
    Custom(u32),
}

impl FontSize {
    /// 获取像素值
    pub fn to_pixels(&self) -> u32 {
        match self {
            FontSize::XSmall => 12,
            FontSize::Small => 14,
            FontSize::Base => 16,
            FontSize::Large => 18,
            FontSize::XLarge => 20,
            FontSize::XXLarge => 24,
            FontSize::XXXLarge => 30,
            FontSize::Huge => 38,
            FontSize::Custom(size) => *size,
        }
    }

    /// 转换为 CSS 字符串
    pub fn to_css(&self) -> String {
        format!("{}px", self.to_pixels())
    }

    /// 从尺寸获取对应的字体大小
    pub fn from_size(size: Size) -> Self {
        match size {
            Size::Small => FontSize::Small,
            Size::Middle => FontSize::Base,
            Size::Large => FontSize::Large,
        }
    }
}

/// 字体重量枚举
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FontWeight {
    /// 100
    Thin,
    /// 200
    ExtraLight,
    /// 300
    Light,
    /// 400 - 正常
    Normal,
    /// 500 - 中等
    Medium,
    /// 600 - 半粗
    SemiBold,
    /// 700 - 粗体
    Bold,
    /// 800 - 特粗
    ExtraBold,
    /// 900 - 黑体
    Black,
    /// 自定义重量
    Custom(u32),
}

impl FontWeight {
    /// 获取数值
    pub fn to_number(&self) -> u32 {
        match self {
            FontWeight::Thin => 100,
            FontWeight::ExtraLight => 200,
            FontWeight::Light => 300,
            FontWeight::Normal => 400,
            FontWeight::Medium => 500,
            FontWeight::SemiBold => 600,
            FontWeight::Bold => 700,
            FontWeight::ExtraBold => 800,
            FontWeight::Black => 900,
            FontWeight::Custom(weight) => *weight,
        }
    }

    /// 转换为 CSS 字符串
    pub fn to_css(&self) -> String {
        self.to_number().to_string()
    }
}

/// 行高枚举
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LineHeight {
    /// 1.0 - 紧密
    Tight,
    /// 1.2 - 较紧
    Snug,
    /// 1.5 - 正常
    Normal,
    /// 1.6 - 宽松
    Relaxed,
    /// 2.0 - 很宽松
    Loose,
    /// 自定义行高
    Custom(f32),
}

impl LineHeight {
    /// 获取数值
    pub fn to_number(&self) -> f32 {
        match self {
            LineHeight::Tight => 1.0,
            LineHeight::Snug => 1.2,
            LineHeight::Normal => 1.5,
            LineHeight::Relaxed => 1.6,
            LineHeight::Loose => 2.0,
            LineHeight::Custom(height) => *height,
        }
    }

    /// 转换为 CSS 字符串
    pub fn to_css(&self) -> String {
        self.to_number().to_string()
    }
}

/// 字体配置结构
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FontConfig {
    /// 字体族
    pub family: FontFamily,
    /// 字体大小
    pub size: FontSize,
    /// 字体重量
    pub weight: FontWeight,
    /// 行高
    pub line_height: LineHeight,
    /// 字间距（em）
    pub letter_spacing: Option<f32>,
}

impl FontConfig {
    /// 创建新的字体配置
    pub fn new(
        family: FontFamily,
        size: FontSize,
        weight: FontWeight,
        line_height: LineHeight,
    ) -> Self {
        Self {
            family,
            size,
            weight,
            line_height,
            letter_spacing: None,
        }
    }

    /// 设置字间距
    pub fn with_letter_spacing(mut self, spacing: f32) -> Self {
        self.letter_spacing = Some(spacing);
        self
    }

    /// 生成 CSS 样式
    pub fn to_css(&self) -> String {
        let mut css = format!(
            "font-family: {}; font-size: {}; font-weight: {}; line-height: {};",
            self.family.to_css(),
            self.size.to_css(),
            self.weight.to_css(),
            self.line_height.to_css()
        );

        if let Some(spacing) = self.letter_spacing {
            css.push_str(&format!(" letter-spacing: {}em;", spacing));
        }

        css
    }
}

/// Ant Design 字体规范
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AntDesignTypography {
    /// 基础字体族
    pub base_font_family: FontFamily,
    /// 等宽字体族
    pub code_font_family: FontFamily,

    // 标题字体配置
    /// H1 标题
    pub h1: FontConfig,
    /// H2 标题
    pub h2: FontConfig,
    /// H3 标题
    pub h3: FontConfig,
    /// H4 标题
    pub h4: FontConfig,
    /// H5 标题
    pub h5: FontConfig,
    /// H6 标题
    pub h6: FontConfig,

    // 正文字体配置
    /// 大号正文
    pub body_large: FontConfig,
    /// 标准正文
    pub body: FontConfig,
    /// 小号正文
    pub body_small: FontConfig,
    /// 说明文字
    pub caption: FontConfig,

    // 特殊字体配置
    /// 代码字体
    pub code: FontConfig,
    /// 链接字体
    pub link: FontConfig,
}

impl AntDesignTypography {
    /// 创建默认字体规范
    pub fn default() -> Self {
        let base_family = FontFamily::System;
        let code_family = FontFamily::Monospace;

        Self {
            base_font_family: base_family.clone(),
            code_font_family: code_family.clone(),

            // 标题配置
            h1: FontConfig::new(
                base_family.clone(),
                FontSize::Huge,
                FontWeight::SemiBold,
                LineHeight::Snug,
            ),
            h2: FontConfig::new(
                base_family.clone(),
                FontSize::XXXLarge,
                FontWeight::SemiBold,
                LineHeight::Snug,
            ),
            h3: FontConfig::new(
                base_family.clone(),
                FontSize::XXLarge,
                FontWeight::SemiBold,
                LineHeight::Normal,
            ),
            h4: FontConfig::new(
                base_family.clone(),
                FontSize::XLarge,
                FontWeight::SemiBold,
                LineHeight::Normal,
            ),
            h5: FontConfig::new(
                base_family.clone(),
                FontSize::Large,
                FontWeight::SemiBold,
                LineHeight::Normal,
            ),
            h6: FontConfig::new(
                base_family.clone(),
                FontSize::Base,
                FontWeight::SemiBold,
                LineHeight::Normal,
            ),

            // 正文配置
            body_large: FontConfig::new(
                base_family.clone(),
                FontSize::Base,
                FontWeight::Normal,
                LineHeight::Relaxed,
            ),
            body: FontConfig::new(
                base_family.clone(),
                FontSize::Small,
                FontWeight::Normal,
                LineHeight::Normal,
            ),
            body_small: FontConfig::new(
                base_family.clone(),
                FontSize::XSmall,
                FontWeight::Normal,
                LineHeight::Normal,
            ),
            caption: FontConfig::new(
                base_family.clone(),
                FontSize::XSmall,
                FontWeight::Normal,
                LineHeight::Normal,
            ),

            // 特殊配置
            code: FontConfig::new(
                code_family,
                FontSize::Small,
                FontWeight::Normal,
                LineHeight::Normal,
            ),
            link: FontConfig::new(
                base_family,
                FontSize::Small,
                FontWeight::Normal,
                LineHeight::Normal,
            ),
        }
    }

    /// 创建紧凑字体规范
    pub fn compact() -> Self {
        let mut typography = Self::default();

        // 减小字体大小
        typography.h1.size = FontSize::XXXLarge;
        typography.h2.size = FontSize::XXLarge;
        typography.h3.size = FontSize::XLarge;
        typography.h4.size = FontSize::Large;
        typography.h5.size = FontSize::Base;
        typography.h6.size = FontSize::Small;

        typography.body_large.size = FontSize::Small;
        typography.body.size = FontSize::XSmall;
        typography.body_small.size = FontSize::Custom(10);
        typography.caption.size = FontSize::Custom(10);

        // 调整行高
        typography.h1.line_height = LineHeight::Tight;
        typography.h2.line_height = LineHeight::Tight;
        typography.h3.line_height = LineHeight::Snug;

        typography
    }

    /// 获取指定尺寸的字体配置
    pub fn get_font_config(&self, size: Size) -> &FontConfig {
        match size {
            Size::Small => &self.body_small,
            Size::Middle => &self.body,
            Size::Large => &self.body_large,
        }
    }

    /// 生成 CSS 变量
    pub fn to_css_variables(&self) -> HashMap<String, String> {
        let mut variables = HashMap::new();

        // 字体族变量
        variables.insert(
            "--ant-font-family".to_string(),
            self.base_font_family.to_css(),
        );
        variables.insert(
            "--ant-font-family-code".to_string(),
            self.code_font_family.to_css(),
        );

        // 字体大小变量
        variables.insert(
            "--ant-font-size-sm".to_string(),
            self.body_small.size.to_css(),
        );
        variables.insert("--ant-font-size-base".to_string(), self.body.size.to_css());
        variables.insert(
            "--ant-font-size-lg".to_string(),
            self.body_large.size.to_css(),
        );
        variables.insert("--ant-font-size-xl".to_string(), FontSize::XLarge.to_css());

        // 标题字体大小变量
        variables.insert("--ant-font-size-h1".to_string(), self.h1.size.to_css());
        variables.insert("--ant-font-size-h2".to_string(), self.h2.size.to_css());
        variables.insert("--ant-font-size-h3".to_string(), self.h3.size.to_css());
        variables.insert("--ant-font-size-h4".to_string(), self.h4.size.to_css());
        variables.insert("--ant-font-size-h5".to_string(), self.h5.size.to_css());
        variables.insert("--ant-font-size-h6".to_string(), self.h6.size.to_css());

        // 行高变量
        variables.insert(
            "--ant-line-height-base".to_string(),
            self.body.line_height.to_css(),
        );
        variables.insert(
            "--ant-line-height-sm".to_string(),
            LineHeight::Snug.to_css(),
        );
        variables.insert(
            "--ant-line-height-lg".to_string(),
            LineHeight::Relaxed.to_css(),
        );

        variables
    }

    /// 生成 CSS 样式
    pub fn generate_css(&self) -> String {
        let mut css = String::new();

        // 基础字体样式
        css.push_str("body { ");
        css.push_str(&self.body.to_css());
        css.push_str(" }\n");

        // 标题样式
        css.push_str(&format!("h1 {{ {} }}\n", self.h1.to_css()));
        css.push_str(&format!("h2 {{ {} }}\n", self.h2.to_css()));
        css.push_str(&format!("h3 {{ {} }}\n", self.h3.to_css()));
        css.push_str(&format!("h4 {{ {} }}\n", self.h4.to_css()));
        css.push_str(&format!("h5 {{ {} }}\n", self.h5.to_css()));
        css.push_str(&format!("h6 {{ {} }}\n", self.h6.to_css()));

        // 代码样式
        css.push_str(&format!("code, pre {{ {} }}\n", self.code.to_css()));

        // 链接样式
        css.push_str(&format!("a {{ {} }}\n", self.link.to_css()));

        // 工具类
        css.push_str(&format!(
            ".ant-typography-caption {{ {} }}\n",
            self.caption.to_css()
        ));
        css.push_str(&format!(
            ".ant-typography-body-large {{ {} }}\n",
            self.body_large.to_css()
        ));
        css.push_str(&format!(
            ".ant-typography-body-small {{ {} }}\n",
            self.body_small.to_css()
        ));

        css
    }
}

impl Default for AntDesignTypography {
    fn default() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_font_family() {
        let system = FontFamily::System;
        assert!(system.to_css().contains("system"));

        let mono = FontFamily::Monospace;
        assert!(mono.to_css().contains("monospace"));

        let custom = FontFamily::Custom("Arial".to_string());
        assert_eq!(custom.to_css(), "Arial");
    }

    #[test]
    fn test_font_size() {
        assert_eq!(FontSize::Base.to_pixels(), 16);
        assert_eq!(FontSize::Small.to_css(), "14px");
        assert_eq!(FontSize::Custom(20).to_pixels(), 20);

        assert_eq!(FontSize::from_size(Size::Small), FontSize::Small);
        assert_eq!(FontSize::from_size(Size::Middle), FontSize::Base);
        assert_eq!(FontSize::from_size(Size::Large), FontSize::Large);
    }

    #[test]
    fn test_font_weight() {
        assert_eq!(FontWeight::Normal.to_number(), 400);
        assert_eq!(FontWeight::Bold.to_css(), "700");
        assert_eq!(FontWeight::Custom(550).to_number(), 550);
    }

    #[test]
    fn test_line_height() {
        assert_eq!(LineHeight::Normal.to_number(), 1.5);
        assert_eq!(LineHeight::Tight.to_css(), "1");
        assert_eq!(LineHeight::Custom(1.8).to_number(), 1.8);
    }

    #[test]
    fn test_font_config() {
        let config = FontConfig::new(
            FontFamily::System,
            FontSize::Base,
            FontWeight::Normal,
            LineHeight::Normal,
        )
        .with_letter_spacing(0.1);

        let css = config.to_css();
        assert!(css.contains("font-family:"));
        assert!(css.contains("font-size: 16px"));
        assert!(css.contains("font-weight: 400"));
        assert!(css.contains("line-height: 1.5"));
        assert!(css.contains("letter-spacing: 0.1em"));
    }

    #[test]
    fn test_ant_design_typography() {
        let typography = AntDesignTypography::default();

        assert_eq!(typography.h1.size, FontSize::Huge);
        assert_eq!(typography.body.size, FontSize::Small);

        let compact = AntDesignTypography::compact();
        assert_eq!(compact.h1.size, FontSize::XXXLarge);

        let config = typography.get_font_config(Size::Middle);
        assert_eq!(config.size, FontSize::Small);
    }

    #[test]
    fn test_css_generation() {
        let typography = AntDesignTypography::default();

        let variables = typography.to_css_variables();
        assert!(variables.contains_key("--ant-font-family"));
        assert!(variables.contains_key("--ant-font-size-base"));

        let css = typography.generate_css();
        assert!(css.contains("body {"));
        assert!(css.contains("h1 {"));
        assert!(css.contains("code, pre {"));
    }
}

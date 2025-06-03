//! 颜色核心抽象
//!
//! 提供颜色系统的基础类型和核心功能，包括：
//! - RGB/HSL 颜色表示
//! - 颜色转换和操作
//! - 颜色调色板生成
//! - 颜色工具函数

use serde::{Deserialize, Serialize};
use std::fmt;

/// 颜色类型枚举
///
/// 定义组件中常用的颜色类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ColorType {
    /// 主色
    Primary,
    /// 成功色
    Success,
    /// 警告色
    Warning,
    /// 错误色
    Error,
    /// 信息色
    Info,
    /// 默认色
    Default,
    /// 链接色
    Link,
}

impl fmt::Display for ColorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ColorType::Primary => write!(f, "primary"),
            ColorType::Success => write!(f, "success"),
            ColorType::Warning => write!(f, "warning"),
            ColorType::Error => write!(f, "error"),
            ColorType::Info => write!(f, "info"),
            ColorType::Default => write!(f, "default"),
            ColorType::Link => write!(f, "link"),
        }
    }
}

/// RGB 颜色结构
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RgbColor {
    /// 创建新的 RGB 颜色
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// 从十六进制字符串创建颜色
    pub fn from_hex(hex: &str) -> Option<Self> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return None;
        }

        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;

        Some(Self::new(r, g, b))
    }

    /// 转换为十六进制字符串
    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }

    /// 转换为 CSS rgb() 字符串
    pub fn to_rgb_string(&self) -> String {
        format!("rgb({}, {}, {})", self.r, self.g, self.b)
    }

    /// 转换为 HSL 颜色
    pub fn to_hsl(&self) -> HslColor {
        let r = self.r as f32 / 255.0;
        let g = self.g as f32 / 255.0;
        let b = self.b as f32 / 255.0;

        let max = r.max(g.max(b));
        let min = r.min(g.min(b));
        let delta = max - min;

        let lightness = (max + min) / 2.0;

        let (hue, saturation) = if delta == 0.0 {
            (0.0, 0.0)
        } else {
            let saturation = if lightness < 0.5 {
                delta / (max + min)
            } else {
                delta / (2.0 - max - min)
            };

            let hue = if max == r {
                ((g - b) / delta + if g < b { 6.0 } else { 0.0 }) / 6.0
            } else if max == g {
                ((b - r) / delta + 2.0) / 6.0
            } else {
                ((r - g) / delta + 4.0) / 6.0
            };

            (hue * 360.0, saturation)
        };

        HslColor::new(hue, saturation, lightness)
    }

    /// 调整亮度
    pub fn adjust_brightness(&self, factor: f32) -> Self {
        let factor = factor.clamp(0.0, 2.0);
        let r = ((self.r as f32 * factor).round() as u8).min(255);
        let g = ((self.g as f32 * factor).round() as u8).min(255);
        let b = ((self.b as f32 * factor).round() as u8).min(255);
        Self::new(r, g, b)
    }

    /// 混合两个颜色
    pub fn mix(&self, other: &Self, ratio: f32) -> Self {
        let ratio = ratio.clamp(0.0, 1.0);
        let inv_ratio = 1.0 - ratio;

        let r = (self.r as f32 * inv_ratio + other.r as f32 * ratio).round() as u8;
        let g = (self.g as f32 * inv_ratio + other.g as f32 * ratio).round() as u8;
        let b = (self.b as f32 * inv_ratio + other.b as f32 * ratio).round() as u8;

        Self::new(r, g, b)
    }
}

/// HSL 颜色结构
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct HslColor {
    /// 色相 (0-360)
    pub h: f32,
    /// 饱和度 (0-1)
    pub s: f32,
    /// 亮度 (0-1)
    pub l: f32,
}

impl HslColor {
    /// 创建新的 HSL 颜色
    pub fn new(h: f32, s: f32, l: f32) -> Self {
        Self {
            h: h % 360.0,
            s: s.clamp(0.0, 1.0),
            l: l.clamp(0.0, 1.0),
        }
    }

    /// 转换为 RGB 颜色
    pub fn to_rgb(&self) -> RgbColor {
        let c = (1.0 - (2.0 * self.l - 1.0).abs()) * self.s;
        let x = c * (1.0 - ((self.h / 60.0) % 2.0 - 1.0).abs());
        let m = self.l - c / 2.0;

        let (r_prime, g_prime, b_prime) = match self.h as u32 {
            0..=59 => (c, x, 0.0),
            60..=119 => (x, c, 0.0),
            120..=179 => (0.0, c, x),
            180..=239 => (0.0, x, c),
            240..=299 => (x, 0.0, c),
            _ => (c, 0.0, x),
        };

        let r = ((r_prime + m) * 255.0).round() as u8;
        let g = ((g_prime + m) * 255.0).round() as u8;
        let b = ((b_prime + m) * 255.0).round() as u8;

        RgbColor::new(r, g, b)
    }

    /// 转换为 CSS hsl() 字符串
    pub fn to_hsl_string(&self) -> String {
        format!(
            "hsl({:.0}, {:.0}%, {:.0}%)",
            self.h,
            self.s * 100.0,
            self.l * 100.0
        )
    }

    /// 调整亮度
    pub fn adjust_lightness(&self, delta: f32) -> Self {
        Self::new(self.h, self.s, (self.l + delta).clamp(0.0, 1.0))
    }

    /// 调整饱和度
    pub fn adjust_saturation(&self, delta: f32) -> Self {
        Self::new(self.h, (self.s + delta).clamp(0.0, 1.0), self.l)
    }
}

/// 颜色调色板
///
/// 包含一个颜色的不同深浅变化
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ColorPalette {
    /// 基础颜色
    pub base: RgbColor,
    /// 浅色变体
    pub light: RgbColor,
    /// 更浅色变体
    pub lighter: RgbColor,
    /// 深色变体
    pub dark: RgbColor,
    /// 更深色变体
    pub darker: RgbColor,
    /// 颜色变化（从浅到深）
    pub variants: Vec<RgbColor>,
}

impl ColorPalette {
    /// 从基础颜色生成调色板
    pub fn generate(base_color: RgbColor, steps: usize) -> Self {
        let mut variants = Vec::with_capacity(steps);
        let hsl = base_color.to_hsl();

        for i in 0..steps {
            let factor = i as f32 / (steps - 1) as f32;
            // 从浅到深：调整亮度从高到低
            let lightness = 0.95 - factor * 0.85;
            let variant_hsl = HslColor::new(hsl.h, hsl.s, lightness);
            variants.push(variant_hsl.to_rgb());
        }

        // 生成浅色和深色变体
        let light = Self::lighten_color(&base_color, 0.2);
        let lighter = Self::lighten_color(&base_color, 0.4);
        let dark = Self::darken_color(&base_color, 0.2);
        let darker = Self::darken_color(&base_color, 0.4);

        Self {
            base: base_color,
            light,
            lighter,
            dark,
            darker,
            variants,
        }
    }

    /// 使颜色变浅
    fn lighten_color(color: &RgbColor, factor: f32) -> RgbColor {
        let hsl = color.to_hsl();
        let new_lightness = (hsl.l + factor).min(1.0);
        HslColor::new(hsl.h, hsl.s, new_lightness).to_rgb()
    }

    /// 使颜色变深
    fn darken_color(color: &RgbColor, factor: f32) -> RgbColor {
        let hsl = color.to_hsl();
        let new_lightness = (hsl.l - factor).max(0.0);
        HslColor::new(hsl.h, hsl.s, new_lightness).to_rgb()
    }

    /// 获取特定索引的颜色
    pub fn get_variant(&self, index: usize) -> Option<&RgbColor> {
        self.variants.get(index)
    }

    /// 获取最浅的颜色
    pub fn get_lightest(&self) -> Option<&RgbColor> {
        self.variants.first()
    }

    /// 获取最深的颜色
    pub fn get_darkest(&self) -> Option<&RgbColor> {
        self.variants.last()
    }
}

/// 颜色工具函数
pub mod utils {
    use super::*;

    /// 判断颜色是否为深色
    pub fn is_dark_color(color: &RgbColor) -> bool {
        // 使用相对亮度公式
        let luminance = 0.299 * color.r as f32 + 0.587 * color.g as f32 + 0.114 * color.b as f32;
        luminance < 128.0
    }

    /// 获取对比色（黑色或白色）
    pub fn get_contrast_color(color: &RgbColor) -> RgbColor {
        if is_dark_color(color) {
            RgbColor::new(255, 255, 255) // 白色
        } else {
            RgbColor::new(0, 0, 0) // 黑色
        }
    }

    /// 生成颜色的 CSS 变量名
    pub fn generate_css_var_name(
        prefix: &str,
        color_type: ColorType,
        variant: Option<u8>,
    ) -> String {
        match variant {
            Some(v) => format!("--{}-{}-{}", prefix, color_type, v),
            None => format!("--{}-{}", prefix, color_type),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::utils::*;
    use super::*;

    #[test]
    fn test_rgb_color_creation() {
        let color = RgbColor::new(255, 128, 64);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 64);
    }

    #[test]
    fn test_rgb_from_hex() {
        let color = RgbColor::from_hex("#FF8040").unwrap();
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 64);

        let color2 = RgbColor::from_hex("FF8040").unwrap();
        assert_eq!(color, color2);

        assert!(RgbColor::from_hex("invalid").is_none());
    }

    #[test]
    fn test_color_palette() {
        let base = RgbColor::new(100, 150, 200);
        let palette = ColorPalette::generate(base, 5);

        assert_eq!(palette.variants.len(), 5);
        assert!(palette.get_lightest().is_some());
        assert!(palette.get_darkest().is_some());
    }

    #[test]
    fn test_is_dark_color() {
        let dark = RgbColor::new(50, 50, 50);
        let light = RgbColor::new(200, 200, 200);

        assert!(is_dark_color(&dark));
        assert!(!is_dark_color(&light));
    }

    #[test]
    fn test_generate_css_var_name() {
        assert_eq!(
            generate_css_var_name("ant", ColorType::Primary, None),
            "--ant-primary"
        );
        assert_eq!(
            generate_css_var_name("ant", ColorType::Success, Some(5)),
            "--ant-success-5"
        );
    }
}

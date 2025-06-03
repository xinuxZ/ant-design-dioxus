//! 颜色系统模块
//!
//! 提供颜色定义、转换和主题色彩管理功能

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
    ///
    /// # Arguments
    ///
    /// * `hex` - 十六进制颜色字符串（如 "#FF0000" 或 "FF0000"）
    ///
    /// # Returns
    ///
    /// RGB 颜色，如果解析失败则返回 None
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
    ///
    /// # Arguments
    ///
    /// * `factor` - 亮度调整因子（0.0-2.0，1.0为原始亮度）
    pub fn adjust_brightness(&self, factor: f32) -> Self {
        let factor = factor.clamp(0.0, 2.0);
        let r = ((self.r as f32 * factor).round() as u8).min(255);
        let g = ((self.g as f32 * factor).round() as u8).min(255);
        let b = ((self.b as f32 * factor).round() as u8).min(255);
        Self::new(r, g, b)
    }

    /// 混合两个颜色
    ///
    /// # Arguments
    ///
    /// * `other` - 另一个颜色
    /// * `ratio` - 混合比例（0.0-1.0，0.0为完全是self，1.0为完全是other）
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
    ///
    /// # Arguments
    ///
    /// * `base_color` - 基础颜色
    /// * `steps` - 生成的变化数量
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

/// 预定义的 Ant Design 颜色
pub mod presets {
    use super::RgbColor;

    /// 蓝色（主色）
    pub const BLUE: RgbColor = RgbColor {
        r: 24,
        g: 144,
        b: 255,
    };
    /// 绿色（成功色）
    pub const GREEN: RgbColor = RgbColor {
        r: 82,
        g: 196,
        b: 26,
    };
    /// 红色（错误色）
    pub const RED: RgbColor = RgbColor {
        r: 255,
        g: 77,
        b: 79,
    };
    /// 橙色（警告色）
    pub const ORANGE: RgbColor = RgbColor {
        r: 250,
        g: 173,
        b: 20,
    };
    /// 灰色（默认色）
    pub const GRAY: RgbColor = RgbColor {
        r: 140,
        g: 140,
        b: 140,
    };
    /// 紫色
    pub const PURPLE: RgbColor = RgbColor {
        r: 114,
        g: 46,
        b: 209,
    };
    /// 青色
    pub const CYAN: RgbColor = RgbColor {
        r: 19,
        g: 194,
        b: 194,
    };
    /// 粉色
    pub const MAGENTA: RgbColor = RgbColor {
        r: 235,
        g: 47,
        b: 150,
    };
    /// 黄色
    pub const YELLOW: RgbColor = RgbColor {
        r: 250,
        g: 219,
        b: 20,
    };
    /// 火山色
    pub const VOLCANO: RgbColor = RgbColor {
        r: 250,
        g: 84,
        b: 28,
    };
    /// 极光绿
    pub const GEEK_BLUE: RgbColor = RgbColor {
        r: 47,
        g: 84,
        b: 235,
    };
    /// 酱紫
    pub const GOLD: RgbColor = RgbColor {
        r: 250,
        g: 173,
        b: 20,
    };
    /// 日暮
    pub const LIME: RgbColor = RgbColor {
        r: 160,
        g: 217,
        b: 17,
    };
}

/// 获取颜色类型对应的默认颜色
pub fn get_color_by_type(color_type: ColorType) -> RgbColor {
    match color_type {
        ColorType::Primary => presets::BLUE,
        ColorType::Success => presets::GREEN,
        ColorType::Warning => presets::ORANGE,
        ColorType::Error => presets::RED,
        ColorType::Info => presets::BLUE,
        ColorType::Default => presets::GRAY,
        ColorType::Link => presets::BLUE,
    }
}

/// 生成颜色的 CSS 变量名
pub fn generate_css_var_name(color_type: ColorType, variant: Option<u8>) -> String {
    match variant {
        Some(v) => format!("--ant-{}-{}", color_type, v),
        None => format!("--ant-{}", color_type),
    }
}

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

#[cfg(test)]
mod tests {
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
    fn test_rgb_to_hex() {
        let color = RgbColor::new(255, 128, 64);
        assert_eq!(color.to_hex(), "#FF8040");
    }

    #[test]
    fn test_rgb_to_rgb_string() {
        let color = RgbColor::new(255, 128, 64);
        assert_eq!(color.to_rgb_string(), "rgb(255, 128, 64)");
    }

    #[test]
    fn test_adjust_brightness() {
        let color = RgbColor::new(100, 100, 100);
        let brighter = color.adjust_brightness(1.5);
        assert!(brighter.r > color.r);

        let darker = color.adjust_brightness(0.5);
        assert!(darker.r < color.r);
    }

    #[test]
    fn test_color_mix() {
        let red = RgbColor::new(255, 0, 0);
        let blue = RgbColor::new(0, 0, 255);
        let purple = red.mix(&blue, 0.5);

        assert_eq!(purple.r, 128);
        assert_eq!(purple.g, 0);
        assert_eq!(purple.b, 128);
    }

    #[test]
    fn test_hsl_color() {
        let hsl = HslColor::new(240.0, 1.0, 0.5); // 纯蓝色
        let rgb = hsl.to_rgb();

        assert_eq!(rgb.r, 0);
        assert_eq!(rgb.g, 0);
        assert_eq!(rgb.b, 255);
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
    fn test_color_type_display() {
        assert_eq!(ColorType::Primary.to_string(), "primary");
        assert_eq!(ColorType::Success.to_string(), "success");
    }

    #[test]
    fn test_is_dark_color() {
        let dark = RgbColor::new(50, 50, 50);
        let light = RgbColor::new(200, 200, 200);

        assert!(is_dark_color(&dark));
        assert!(!is_dark_color(&light));
    }

    #[test]
    fn test_get_contrast_color() {
        let dark = RgbColor::new(50, 50, 50);
        let light = RgbColor::new(200, 200, 200);

        let contrast_dark = get_contrast_color(&dark);
        let contrast_light = get_contrast_color(&light);

        assert_eq!(contrast_dark, RgbColor::new(255, 255, 255));
        assert_eq!(contrast_light, RgbColor::new(0, 0, 0));
    }

    #[test]
    fn test_generate_css_var_name() {
        assert_eq!(
            generate_css_var_name(ColorType::Primary, None),
            "--ant-primary"
        );
        assert_eq!(
            generate_css_var_name(ColorType::Success, Some(5)),
            "--ant-success-5"
        );
    }
}

//! 颜色工具模块
//!
//! 提供颜色操作和转换的工具函数

use std::fmt;

/// RGB颜色表示
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rgb({}, {}, {})", self.r, self.g, self.b)
    }
}

/// HSL颜色表示
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hsl {
    pub h: f32, // 0-360
    pub s: f32, // 0-100
    pub l: f32, // 0-100
}

impl fmt::Display for Hsl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "hsl({}, {}%, {}%)", self.h, self.s, self.l)
    }
}

/// RGBA颜色表示
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f32, // 0-1
}

impl fmt::Display for Rgba {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rgba({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }
}

/// HSLA颜色表示
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hsla {
    pub h: f32, // 0-360
    pub s: f32, // 0-100
    pub l: f32, // 0-100
    pub a: f32, // 0-1
}

impl fmt::Display for Hsla {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "hsla({}, {}%, {}%, {})", self.h, self.s, self.l, self.a)
    }
}

/// 解析十六进制颜色为RGB
pub fn hex_to_rgb(hex: &str) -> Result<Rgb, String> {
    let hex = hex.trim_start_matches('#');

    if hex.len() != 6 && hex.len() != 3 {
        return Err(format!("Invalid hex color: {}", hex));
    }

    if hex.len() == 3 {
        // 处理缩写形式 #RGB
        let r = u8::from_str_radix(&hex[0..1].repeat(2), 16)
            .map_err(|_| format!("Invalid hex color: {}", hex))?;
        let g = u8::from_str_radix(&hex[1..2].repeat(2), 16)
            .map_err(|_| format!("Invalid hex color: {}", hex))?;
        let b = u8::from_str_radix(&hex[2..3].repeat(2), 16)
            .map_err(|_| format!("Invalid hex color: {}", hex))?;

        Ok(Rgb { r, g, b })
    } else {
        // 处理标准形式 #RRGGBB
        let r = u8::from_str_radix(&hex[0..2], 16)
            .map_err(|_| format!("Invalid hex color: {}", hex))?;
        let g = u8::from_str_radix(&hex[2..4], 16)
            .map_err(|_| format!("Invalid hex color: {}", hex))?;
        let b = u8::from_str_radix(&hex[4..6], 16)
            .map_err(|_| format!("Invalid hex color: {}", hex))?;

        Ok(Rgb { r, g, b })
    }
}

/// 将RGB转换为十六进制字符串
pub fn rgb_to_hex(rgb: Rgb) -> String {
    format!("#{:02x}{:02x}{:02x}", rgb.r, rgb.g, rgb.b)
}

/// 将RGB转换为HSL
pub fn rgb_to_hsl(rgb: Rgb) -> Hsl {
    let r = rgb.r as f32 / 255.0;
    let g = rgb.g as f32 / 255.0;
    let b = rgb.b as f32 / 255.0;

    let max = r.max(g.max(b));
    let min = r.min(g.min(b));
    let delta = max - min;

    let mut h = 0.0;
    let mut s = 0.0;
    let l = (max + min) / 2.0;

    if delta != 0.0 {
        s = if l < 0.5 {
            delta / (max + min)
        } else {
            delta / (2.0 - max - min)
        };

        h = if r == max {
            (g - b) / delta + (if g < b { 6.0 } else { 0.0 })
        } else if g == max {
            (b - r) / delta + 2.0
        } else {
            (r - g) / delta + 4.0
        };

        h *= 60.0;
    }

    Hsl {
        h,
        s: s * 100.0,
        l: l * 100.0,
    }
}

/// 将HSL转换为RGB
pub fn hsl_to_rgb(hsl: Hsl) -> Rgb {
    let h = hsl.h / 360.0;
    let s = hsl.s / 100.0;
    let l = hsl.l / 100.0;

    if s == 0.0 {
        // 灰度
        let v = (l * 255.0) as u8;
        return Rgb { r: v, g: v, b: v };
    }

    let q = if l < 0.5 {
        l * (1.0 + s)
    } else {
        l + s - l * s
    };

    let p = 2.0 * l - q;

    let r = hue_to_rgb(p, q, h + 1.0 / 3.0);
    let g = hue_to_rgb(p, q, h);
    let b = hue_to_rgb(p, q, h - 1.0 / 3.0);

    Rgb {
        r: (r * 255.0) as u8,
        g: (g * 255.0) as u8,
        b: (b * 255.0) as u8,
    }
}

/// 辅助函数，将色相转换为RGB分量
fn hue_to_rgb(p: f32, q: f32, mut t: f32) -> f32 {
    if t < 0.0 {
        t += 1.0;
    }
    if t > 1.0 {
        t -= 1.0;
    }

    if t < 1.0 / 6.0 {
        return p + (q - p) * 6.0 * t;
    }
    if t < 1.0 / 2.0 {
        return q;
    }
    if t < 2.0 / 3.0 {
        return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
    }

    p
}

/// 增亮颜色
pub fn lighten(hex: &str, amount: f32) -> Result<String, String> {
    let rgb = hex_to_rgb(hex)?;
    let mut hsl = rgb_to_hsl(rgb);

    hsl.l = (hsl.l + amount * 100.0).min(100.0);

    let rgb = hsl_to_rgb(hsl);
    Ok(rgb_to_hex(rgb))
}

/// 变暗颜色
pub fn darken(hex: &str, amount: f32) -> Result<String, String> {
    let rgb = hex_to_rgb(hex)?;
    let mut hsl = rgb_to_hsl(rgb);

    hsl.l = (hsl.l - amount * 100.0).max(0.0);

    let rgb = hsl_to_rgb(hsl);
    Ok(rgb_to_hex(rgb))
}

/// 设置透明度
pub fn set_alpha(hex: &str, alpha: f32) -> Result<String, String> {
    let rgb = hex_to_rgb(hex)?;

    Ok(format!("rgba({}, {}, {}, {})", rgb.r, rgb.g, rgb.b, alpha))
}

/// 调整饱和度
pub fn adjust_saturation(hex: &str, amount: f32) -> Result<String, String> {
    let rgb = hex_to_rgb(hex)?;
    let mut hsl = rgb_to_hsl(rgb);

    hsl.s = (hsl.s + amount * 100.0).min(100.0).max(0.0);

    let rgb = hsl_to_rgb(hsl);
    Ok(rgb_to_hex(rgb))
}

/// 生成颜色色板
pub fn generate_palette(base_color: &str, steps: usize) -> Result<Vec<String>, String> {
    let base_rgb = hex_to_rgb(base_color)?;
    let base_hsl = rgb_to_hsl(base_rgb);

    let mut palette = Vec::with_capacity(steps);

    // 计算亮色部分
    let light_step = (100.0 - base_hsl.l) / ((steps / 2) as f32);
    for i in 0..(steps / 2) {
        let l = base_hsl.l + light_step * (i as f32);
        let hsl = Hsl {
            h: base_hsl.h,
            s: base_hsl.s,
            l,
        };
        palette.push(rgb_to_hex(hsl_to_rgb(hsl)));
    }

    // 添加基础色
    palette.push(base_color.to_string());

    // 计算暗色部分
    let dark_step = base_hsl.l / ((steps / 2) as f32);
    for i in 1..=(steps / 2) {
        let l = base_hsl.l - dark_step * (i as f32);
        let hsl = Hsl {
            h: base_hsl.h,
            s: base_hsl.s,
            l,
        };
        palette.push(rgb_to_hex(hsl_to_rgb(hsl)));
    }

    Ok(palette)
}

/// 生成Ant Design风格的颜色色板
///
/// 基于给定的基础颜色生成10个色阶的调色板，符合Ant Design的色彩系统规则
///
/// # 参数
///
/// * `base_color` - 基础颜色（HEX格式）
///
/// # 返回值
///
/// 包含10个色阶的调色板（从浅到深）
pub fn generate_ant_palette(base_color: &str) -> Result<Vec<String>, String> {
    let base_rgb = hex_to_rgb(base_color)?;
    let base_hsl = rgb_to_hsl(base_rgb);

    let mut palette = Vec::with_capacity(10);

    // 生成浅色部分 (1-5)
    // 算法: 固定色相，减少饱和度，增加亮度
    let brightest_l = 97.5; // 最亮的亮度
    let brightest_s = base_hsl.s * 0.15; // 最亮色的饱和度降低到原始饱和度的15%

    for i in 0..5 {
        let factor = (5 - i) as f32 / 5.0;

        // 亮度: 线性增加到最亮
        let l = base_hsl.l + (brightest_l - base_hsl.l) * factor;

        // 饱和度: 线性减少到最低
        let s = base_hsl.s - (base_hsl.s - brightest_s) * factor;

        let hsl = Hsl {
            h: base_hsl.h,
            s: s.max(0.0).min(100.0),
            l: l.max(0.0).min(100.0),
        };

        palette.push(rgb_to_hex(hsl_to_rgb(hsl)));
    }

    // 添加基础色 (6)
    palette.push(base_color.to_string());

    // 生成深色部分 (7-10)
    // 算法: 固定色相，增加饱和度，减少亮度
    let darkest_l = base_hsl.l * 0.25; // 最暗的亮度是原始亮度的25%
    let darkest_s = (base_hsl.s + (100.0 - base_hsl.s) * 0.4).min(100.0); // 增加饱和度但不超过100

    for i in 0..4 {
        let factor = (i + 1) as f32 / 4.0;

        // 亮度: 线性减少到最暗
        let l = base_hsl.l - (base_hsl.l - darkest_l) * factor;

        // 饱和度: 线性增加饱和度
        let s = base_hsl.s + (darkest_s - base_hsl.s) * factor;

        let hsl = Hsl {
            h: base_hsl.h,
            s: s.max(0.0).min(100.0),
            l: l.max(0.0).min(100.0),
        };

        palette.push(rgb_to_hex(hsl_to_rgb(hsl)));
    }

    Ok(palette)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_rgb() {
        assert_eq!(hex_to_rgb("#ff0000").unwrap(), Rgb { r: 255, g: 0, b: 0 });
        assert_eq!(hex_to_rgb("#00ff00").unwrap(), Rgb { r: 0, g: 255, b: 0 });
        assert_eq!(hex_to_rgb("#0000ff").unwrap(), Rgb { r: 0, g: 0, b: 255 });
        assert_eq!(hex_to_rgb("#f00").unwrap(), Rgb { r: 255, g: 0, b: 0 });
    }

    #[test]
    fn test_rgb_to_hex() {
        assert_eq!(rgb_to_hex(Rgb { r: 255, g: 0, b: 0 }), "#ff0000");
        assert_eq!(rgb_to_hex(Rgb { r: 0, g: 255, b: 0 }), "#00ff00");
        assert_eq!(rgb_to_hex(Rgb { r: 0, g: 0, b: 255 }), "#0000ff");
    }

    #[test]
    fn test_lighten_darken() {
        let color = "#1890ff";
        let lighter = lighten(color, 0.2).unwrap();
        let darker = darken(color, 0.2).unwrap();

        assert_ne!(lighter, color);
        assert_ne!(darker, color);
    }
}

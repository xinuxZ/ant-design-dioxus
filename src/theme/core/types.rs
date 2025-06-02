//! 核心类型定义
//!
//! 提供主题系统的基础类型定义，包括：
//! - 组件尺寸类型
//! - 间距尺寸类型
//! - 尺寸配置抽象

use serde::{Deserialize, Serialize};
use std::fmt;

/// 组件尺寸枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Size {
    /// 小尺寸
    Small,
    /// 中等尺寸（默认）
    Middle,
    /// 大尺寸
    Large,
}

impl Default for Size {
    fn default() -> Self {
        Size::Middle
    }
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Size::Small => write!(f, "small"),
            Size::Middle => write!(f, "middle"),
            Size::Large => write!(f, "large"),
        }
    }
}

impl From<&str> for Size {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "small" | "sm" => Size::Small,
            "large" | "lg" => Size::Large,
            _ => Size::Middle,
        }
    }
}

impl From<String> for Size {
    fn from(s: String) -> Self {
        Size::from(s.as_str())
    }
}

/// 间距尺寸枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SpaceSize {
    /// 无间距
    None,
    /// 超小间距
    XSmall,
    /// 小间距
    Small,
    /// 中等间距
    Middle,
    /// 大间距
    Large,
    /// 超大间距
    XLarge,
    /// 自定义间距（像素值）
    Custom(u32),
}

impl Default for SpaceSize {
    fn default() -> Self {
        SpaceSize::Small
    }
}

impl fmt::Display for SpaceSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SpaceSize::None => write!(f, "0"),
            SpaceSize::XSmall => write!(f, "4px"),
            SpaceSize::Small => write!(f, "8px"),
            SpaceSize::Middle => write!(f, "16px"),
            SpaceSize::Large => write!(f, "24px"),
            SpaceSize::XLarge => write!(f, "32px"),
            SpaceSize::Custom(px) => write!(f, "{}px", px),
        }
    }
}

impl From<u32> for SpaceSize {
    fn from(px: u32) -> Self {
        match px {
            0 => SpaceSize::None,
            4 => SpaceSize::XSmall,
            8 => SpaceSize::Small,
            16 => SpaceSize::Middle,
            24 => SpaceSize::Large,
            32 => SpaceSize::XLarge,
            _ => SpaceSize::Custom(px),
        }
    }
}

/// 尺寸配置结构体
///
/// 包含组件尺寸的完整配置信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SizeConfig {
    /// 高度
    pub height: u32,
    /// 水平内边距
    pub padding_horizontal: u32,
    /// 垂直内边距
    pub padding_vertical: u32,
    /// 字体大小
    pub font_size: u32,
    /// 边框圆角
    pub border_radius: u32,
}

impl SizeConfig {
    /// 创建新的尺寸配置
    pub fn new(
        height: u32,
        padding_horizontal: u32,
        padding_vertical: u32,
        font_size: u32,
        border_radius: u32,
    ) -> Self {
        Self {
            height,
            padding_horizontal,
            padding_vertical,
            font_size,
            border_radius,
        }
    }

    /// 获取 CSS 样式字符串
    pub fn to_css(&self) -> String {
        format!(
            "height: {}px; padding: {}px {}px; font-size: {}px; border-radius: {}px;",
            self.height,
            self.padding_vertical,
            self.padding_horizontal,
            self.font_size,
            self.border_radius
        )
    }

    /// 获取 CSS 变量映射
    pub fn to_css_vars(&self, prefix: &str) -> Vec<(String, String)> {
        vec![
            (format!("--{}-height", prefix), format!("{}px", self.height)),
            (
                format!("--{}-padding-horizontal", prefix),
                format!("{}px", self.padding_horizontal),
            ),
            (
                format!("--{}-padding-vertical", prefix),
                format!("{}px", self.padding_vertical),
            ),
            (
                format!("--{}-font-size", prefix),
                format!("{}px", self.font_size),
            ),
            (
                format!("--{}-border-radius", prefix),
                format!("{}px", self.border_radius),
            ),
        ]
    }
}

/// 尺寸工具函数
pub mod utils {
    use super::*;

    /// 获取标准尺寸对应的像素值
    pub fn get_standard_size_pixels(size: Size) -> u32 {
        match size {
            Size::Small => 24,
            Size::Middle => 32,
            Size::Large => 40,
        }
    }

    /// 获取标准尺寸对应的内边距
    pub fn get_standard_size_padding(size: Size) -> (u32, u32) {
        match size {
            Size::Small => (7, 4),
            Size::Middle => (15, 8),
            Size::Large => (15, 12),
        }
    }

    /// 获取标准尺寸对应的字体大小
    pub fn get_standard_size_font_size(size: Size) -> u32 {
        match size {
            Size::Small => 12,
            Size::Middle => 14,
            Size::Large => 16,
        }
    }

    /// 获取标准尺寸对应的边框圆角
    pub fn get_standard_size_border_radius(size: Size) -> u32 {
        match size {
            Size::Small => 4,
            Size::Middle => 6,
            Size::Large => 8,
        }
    }

    /// 创建标准尺寸配置
    pub fn create_standard_size_config(size: Size) -> SizeConfig {
        let height = get_standard_size_pixels(size);
        let (padding_horizontal, padding_vertical) = get_standard_size_padding(size);
        let font_size = get_standard_size_font_size(size);
        let border_radius = get_standard_size_border_radius(size);

        SizeConfig::new(
            height,
            padding_horizontal,
            padding_vertical,
            font_size,
            border_radius,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::utils::*;
    use super::*;

    #[test]
    fn test_size_display() {
        assert_eq!(Size::Small.to_string(), "small");
        assert_eq!(Size::Middle.to_string(), "middle");
        assert_eq!(Size::Large.to_string(), "large");
    }

    #[test]
    fn test_size_from_str() {
        assert_eq!(Size::from("small"), Size::Small);
        assert_eq!(Size::from("large"), Size::Large);
        assert_eq!(Size::from("unknown"), Size::Middle);
    }

    #[test]
    fn test_space_size_display() {
        assert_eq!(SpaceSize::None.to_string(), "0");
        assert_eq!(SpaceSize::Small.to_string(), "8px");
        assert_eq!(SpaceSize::Custom(20).to_string(), "20px");
    }

    #[test]
    fn test_space_size_from_u32() {
        assert_eq!(SpaceSize::from(0), SpaceSize::None);
        assert_eq!(SpaceSize::from(8), SpaceSize::Small);
        assert_eq!(SpaceSize::from(20), SpaceSize::Custom(20));
    }

    #[test]
    fn test_size_config() {
        let config = create_standard_size_config(Size::Middle);
        assert_eq!(config.height, 32);
        assert_eq!(config.padding_horizontal, 15);
        assert_eq!(config.padding_vertical, 8);
        assert_eq!(config.font_size, 14);
        assert_eq!(config.border_radius, 6);
    }

    #[test]
    fn test_size_config_to_css() {
        let config = create_standard_size_config(Size::Middle);
        let css = config.to_css();
        assert!(css.contains("height: 32px"));
        assert!(css.contains("padding: 8px 15px"));
        assert!(css.contains("font-size: 14px"));
        assert!(css.contains("border-radius: 6px"));
    }

    #[test]
    fn test_size_config_to_css_vars() {
        let config = create_standard_size_config(Size::Middle);
        let vars = config.to_css_vars("button");

        assert!(vars.contains(&("--button-height".to_string(), "32px".to_string())));
        assert!(vars.contains(&("--button-font-size".to_string(), "14px".to_string())));
    }
}

//! 尺寸系统模块
//!
//! 定义组件库中使用的各种尺寸类型和相关工具函数

use serde::{Deserialize, Serialize};
use std::fmt;

/// 组件尺寸枚举
///
/// 定义组件的标准尺寸规格
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
///
/// 定义组件间距的标准规格
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

/// 获取尺寸对应的像素值
///
/// # Arguments
///
/// * `size` - 组件尺寸
/// * `component_type` - 组件类型（用于不同组件的尺寸差异）
///
/// # Returns
///
/// 对应的像素值
pub fn get_size_pixels(size: Size, component_type: &str) -> u32 {
    match component_type {
        "button" => match size {
            Size::Small => 24,
            Size::Middle => 32,
            Size::Large => 40,
        },
        "input" => match size {
            Size::Small => 24,
            Size::Middle => 32,
            Size::Large => 40,
        },
        "select" => match size {
            Size::Small => 24,
            Size::Middle => 32,
            Size::Large => 40,
        },
        _ => match size {
            Size::Small => 24,
            Size::Middle => 32,
            Size::Large => 40,
        },
    }
}

/// 获取尺寸对应的内边距
///
/// # Arguments
///
/// * `size` - 组件尺寸
///
/// # Returns
///
/// (水平内边距, 垂直内边距) 的元组
pub fn get_size_padding(size: Size) -> (u32, u32) {
    match size {
        Size::Small => (7, 4),
        Size::Middle => (15, 8),
        Size::Large => (15, 12),
    }
}

/// 获取尺寸对应的字体大小
///
/// # Arguments
///
/// * `size` - 组件尺寸
///
/// # Returns
///
/// 字体大小（像素值）
pub fn get_size_font_size(size: Size) -> u32 {
    match size {
        Size::Small => 12,
        Size::Middle => 14,
        Size::Large => 16,
    }
}

/// 获取尺寸对应的边框圆角
///
/// # Arguments
///
/// * `size` - 组件尺寸
///
/// # Returns
///
/// 边框圆角值（像素值）
pub fn get_size_border_radius(size: Size) -> u32 {
    match size {
        Size::Small => 4,
        Size::Middle => 6,
        Size::Large => 8,
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
    /// 根据尺寸和组件类型创建配置
    ///
    /// # Arguments
    ///
    /// * `size` - 组件尺寸
    /// * `component_type` - 组件类型
    ///
    /// # Returns
    ///
    /// 尺寸配置实例
    pub fn new(size: Size, component_type: &str) -> Self {
        let height = get_size_pixels(size, component_type);
        let (padding_horizontal, padding_vertical) = get_size_padding(size);
        let font_size = get_size_font_size(size);
        let border_radius = get_size_border_radius(size);

        Self {
            height,
            padding_horizontal,
            padding_vertical,
            font_size,
            border_radius,
        }
    }

    /// 获取 CSS 样式字符串
    ///
    /// # Returns
    ///
    /// CSS 样式字符串
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
}

#[cfg(test)]
mod tests {
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
    fn test_get_size_pixels() {
        assert_eq!(get_size_pixels(Size::Small, "button"), 24);
        assert_eq!(get_size_pixels(Size::Middle, "button"), 32);
        assert_eq!(get_size_pixels(Size::Large, "button"), 40);
    }

    #[test]
    fn test_get_size_padding() {
        assert_eq!(get_size_padding(Size::Small), (7, 4));
        assert_eq!(get_size_padding(Size::Middle), (15, 8));
        assert_eq!(get_size_padding(Size::Large), (15, 12));
    }

    #[test]
    fn test_size_config() {
        let config = SizeConfig::new(Size::Middle, "button");
        assert_eq!(config.height, 32);
        assert_eq!(config.padding_horizontal, 15);
        assert_eq!(config.padding_vertical, 8);
        assert_eq!(config.font_size, 14);
        assert_eq!(config.border_radius, 6);
    }

    #[test]
    fn test_size_config_to_css() {
        let config = SizeConfig::new(Size::Middle, "button");
        let css = config.to_css();
        assert!(css.contains("height: 32px"));
        assert!(css.contains("padding: 8px 15px"));
        assert!(css.contains("font-size: 14px"));
        assert!(css.contains("border-radius: 6px"));
    }
}

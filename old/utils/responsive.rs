//! 响应式断点系统模块
//!
//! 提供响应式设计所需的断点定义和工具函数

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// 响应式断点枚举
///
/// 定义标准的响应式断点
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum Breakpoint {
    /// 超小屏幕 < 576px
    XS,
    /// 小屏幕 ≥ 576px
    SM,
    /// 中等屏幕 ≥ 768px
    MD,
    /// 大屏幕 ≥ 992px
    LG,
    /// 超大屏幕 ≥ 1200px
    XL,
    /// 超超大屏幕 ≥ 1600px
    XXL,
}

impl fmt::Display for Breakpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Breakpoint::XS => write!(f, "xs"),
            Breakpoint::SM => write!(f, "sm"),
            Breakpoint::MD => write!(f, "md"),
            Breakpoint::LG => write!(f, "lg"),
            Breakpoint::XL => write!(f, "xl"),
            Breakpoint::XXL => write!(f, "xxl"),
        }
    }
}

impl From<&str> for Breakpoint {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "xs" => Breakpoint::XS,
            "sm" => Breakpoint::SM,
            "md" => Breakpoint::MD,
            "lg" => Breakpoint::LG,
            "xl" => Breakpoint::XL,
            "xxl" => Breakpoint::XXL,
            _ => Breakpoint::MD, // 默认中等屏幕
        }
    }
}

/// 获取断点对应的最小宽度
///
/// # Arguments
///
/// * `breakpoint` - 断点类型
///
/// # Returns
///
/// 最小宽度（像素值）
pub fn get_breakpoint_min_width(breakpoint: Breakpoint) -> u32 {
    match breakpoint {
        Breakpoint::XS => 0,
        Breakpoint::SM => 576,
        Breakpoint::MD => 768,
        Breakpoint::LG => 992,
        Breakpoint::XL => 1200,
        Breakpoint::XXL => 1600,
    }
}

/// 获取断点对应的最大宽度
///
/// # Arguments
///
/// * `breakpoint` - 断点类型
///
/// # Returns
///
/// 最大宽度（像素值），如果是最大断点则返回 None
pub fn get_breakpoint_max_width(breakpoint: Breakpoint) -> Option<u32> {
    match breakpoint {
        Breakpoint::XS => Some(575),
        Breakpoint::SM => Some(767),
        Breakpoint::MD => Some(991),
        Breakpoint::LG => Some(1199),
        Breakpoint::XL => Some(1599),
        Breakpoint::XXL => None,
    }
}

/// 响应式值类型
///
/// 用于定义在不同断点下的不同值
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponsiveValue<T> {
    /// 各断点对应的值
    pub values: HashMap<Breakpoint, T>,
    /// 默认值（当没有匹配的断点时使用）
    pub default: T,
}

impl<T> ResponsiveValue<T>
where
    T: Clone,
{
    /// 创建新的响应式值
    ///
    /// # Arguments
    ///
    /// * `default` - 默认值
    ///
    /// # Returns
    ///
    /// 新的响应式值实例
    pub fn new(default: T) -> Self {
        Self {
            values: HashMap::new(),
            default,
        }
    }

    /// 设置特定断点的值
    ///
    /// # Arguments
    ///
    /// * `breakpoint` - 断点类型
    /// * `value` - 对应的值
    ///
    /// # Returns
    ///
    /// 返回 self 以支持链式调用
    pub fn set(mut self, breakpoint: Breakpoint, value: T) -> Self {
        self.values.insert(breakpoint, value);
        self
    }

    /// 根据当前屏幕宽度获取对应的值
    ///
    /// # Arguments
    ///
    /// * `screen_width` - 当前屏幕宽度
    ///
    /// # Returns
    ///
    /// 对应的值
    pub fn get_value_for_width(&self, screen_width: u32) -> &T {
        let current_breakpoint = get_current_breakpoint(screen_width);

        // 从当前断点开始，向下查找最近的有值的断点
        let breakpoints = [
            Breakpoint::XXL,
            Breakpoint::XL,
            Breakpoint::LG,
            Breakpoint::MD,
            Breakpoint::SM,
            Breakpoint::XS,
        ];

        for &bp in &breakpoints {
            if bp <= current_breakpoint {
                if let Some(value) = self.values.get(&bp) {
                    return value;
                }
            }
        }

        &self.default
    }

    /// 获取特定断点的值
    ///
    /// # Arguments
    ///
    /// * `breakpoint` - 断点类型
    ///
    /// # Returns
    ///
    /// 对应的值，如果没有则返回默认值
    pub fn get_value_for_breakpoint(&self, breakpoint: Breakpoint) -> &T {
        self.values.get(&breakpoint).unwrap_or(&self.default)
    }
}

/// 根据屏幕宽度获取当前断点
///
/// # Arguments
///
/// * `width` - 屏幕宽度
///
/// # Returns
///
/// 当前断点
pub fn get_current_breakpoint(width: u32) -> Breakpoint {
    if width >= 1600 {
        Breakpoint::XXL
    } else if width >= 1200 {
        Breakpoint::XL
    } else if width >= 992 {
        Breakpoint::LG
    } else if width >= 768 {
        Breakpoint::MD
    } else if width >= 576 {
        Breakpoint::SM
    } else {
        Breakpoint::XS
    }
}

/// 生成媒体查询字符串
///
/// # Arguments
///
/// * `breakpoint` - 断点类型
/// * `direction` - 查询方向（"min" 或 "max"）
///
/// # Returns
///
/// 媒体查询字符串
pub fn generate_media_query(breakpoint: Breakpoint, direction: &str) -> String {
    match direction {
        "min" => {
            let min_width = get_breakpoint_min_width(breakpoint);
            if min_width > 0 {
                format!("@media (min-width: {}px)", min_width)
            } else {
                String::new()
            }
        }
        "max" => {
            if let Some(max_width) = get_breakpoint_max_width(breakpoint) {
                format!("@media (max-width: {}px)", max_width)
            } else {
                String::new()
            }
        }
        _ => String::new(),
    }
}

/// 生成断点范围的媒体查询
///
/// # Arguments
///
/// * `min_breakpoint` - 最小断点
/// * `max_breakpoint` - 最大断点
///
/// # Returns
///
/// 媒体查询字符串
pub fn generate_range_media_query(
    min_breakpoint: Breakpoint,
    max_breakpoint: Breakpoint,
) -> String {
    let min_width = get_breakpoint_min_width(min_breakpoint);

    if let Some(max_width) = get_breakpoint_max_width(max_breakpoint) {
        if min_width > 0 {
            format!(
                "@media (min-width: {}px) and (max-width: {}px)",
                min_width, max_width
            )
        } else {
            format!("@media (max-width: {}px)", max_width)
        }
    } else {
        format!("@media (min-width: {}px)", min_width)
    }
}

/// 栅格系统配置
///
/// 定义栅格系统的基本参数
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GridConfig {
    /// 总列数
    pub columns: u32,
    /// 间距大小
    pub gutter: ResponsiveValue<u32>,
    /// 容器最大宽度
    pub container_max_width: ResponsiveValue<Option<u32>>,
}

impl Default for GridConfig {
    fn default() -> Self {
        Self {
            columns: 24, // Ant Design 使用 24 列栅格
            gutter: ResponsiveValue::new(0)
                .set(Breakpoint::XS, 8)
                .set(Breakpoint::SM, 16)
                .set(Breakpoint::MD, 24)
                .set(Breakpoint::LG, 32),
            container_max_width: ResponsiveValue::new(None)
                .set(Breakpoint::SM, Some(540))
                .set(Breakpoint::MD, Some(720))
                .set(Breakpoint::LG, Some(960))
                .set(Breakpoint::XL, Some(1140))
                .set(Breakpoint::XXL, Some(1320)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breakpoint_display() {
        assert_eq!(Breakpoint::XS.to_string(), "xs");
        assert_eq!(Breakpoint::MD.to_string(), "md");
        assert_eq!(Breakpoint::XXL.to_string(), "xxl");
    }

    #[test]
    fn test_breakpoint_from_str() {
        assert_eq!(Breakpoint::from("xs"), Breakpoint::XS);
        assert_eq!(Breakpoint::from("lg"), Breakpoint::LG);
        assert_eq!(Breakpoint::from("unknown"), Breakpoint::MD);
    }

    #[test]
    fn test_get_breakpoint_min_width() {
        assert_eq!(get_breakpoint_min_width(Breakpoint::XS), 0);
        assert_eq!(get_breakpoint_min_width(Breakpoint::SM), 576);
        assert_eq!(get_breakpoint_min_width(Breakpoint::XXL), 1600);
    }

    #[test]
    fn test_get_current_breakpoint() {
        assert_eq!(get_current_breakpoint(500), Breakpoint::XS);
        assert_eq!(get_current_breakpoint(600), Breakpoint::SM);
        assert_eq!(get_current_breakpoint(800), Breakpoint::MD);
        assert_eq!(get_current_breakpoint(1000), Breakpoint::LG);
        assert_eq!(get_current_breakpoint(1300), Breakpoint::XL);
        assert_eq!(get_current_breakpoint(1700), Breakpoint::XXL);
    }

    #[test]
    fn test_responsive_value() {
        let responsive = ResponsiveValue::new(12)
            .set(Breakpoint::SM, 16)
            .set(Breakpoint::MD, 20)
            .set(Breakpoint::LG, 24);

        assert_eq!(*responsive.get_value_for_width(500), 12); // XS -> default
        assert_eq!(*responsive.get_value_for_width(600), 16); // SM
        assert_eq!(*responsive.get_value_for_width(800), 20); // MD
        assert_eq!(*responsive.get_value_for_width(1000), 24); // LG
        assert_eq!(*responsive.get_value_for_width(1300), 24); // XL -> LG value
    }

    #[test]
    fn test_generate_media_query() {
        assert_eq!(
            generate_media_query(Breakpoint::SM, "min"),
            "@media (min-width: 576px)"
        );
        assert_eq!(
            generate_media_query(Breakpoint::MD, "max"),
            "@media (max-width: 991px)"
        );
        assert_eq!(generate_media_query(Breakpoint::XS, "min"), "");
    }

    #[test]
    fn test_generate_range_media_query() {
        assert_eq!(
            generate_range_media_query(Breakpoint::SM, Breakpoint::LG),
            "@media (min-width: 576px) and (max-width: 1199px)"
        );
    }

    #[test]
    fn test_grid_config_default() {
        let config = GridConfig::default();
        assert_eq!(config.columns, 24);
        assert_eq!(*config.gutter.get_value_for_breakpoint(Breakpoint::SM), 16);
    }
}

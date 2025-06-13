//! Space 组件的类型定义
//!
//! 定义了 Space 组件使用的所有类型，包括方向、对齐方式、尺寸等。

use dioxus::prelude::*;

/// 间距方向
#[derive(Debug, Clone, PartialEq, Default)]
pub enum SpaceDirection {
    /// 水平方向
    #[default]
    Horizontal,
    /// 垂直方向
    Vertical,
}

impl SpaceDirection {
    /// 转换为CSS类名
    pub fn to_class(&self) -> &'static str {
        match self {
            SpaceDirection::Horizontal => "horizontal",
            SpaceDirection::Vertical => "vertical",
        }
    }
}

/// 对齐方式
#[derive(Debug, Clone, PartialEq, Default)]
pub enum SpaceAlign {
    /// 起始对齐
    #[default]
    Start,
    /// 结束对齐
    End,
    /// 居中对齐
    Center,
    /// 基线对齐
    Baseline,
}

impl SpaceAlign {
    /// 转换为CSS类名
    pub fn to_class(&self) -> &'static str {
        match self {
            SpaceAlign::Start => "start",
            SpaceAlign::End => "end",
            SpaceAlign::Center => "center",
            SpaceAlign::Baseline => "baseline",
        }
    }
}

/// 间距大小
#[derive(Debug, Clone, PartialEq, Default)]
pub enum SpaceSize {
    /// 小间距 (8px)
    Small,
    /// 中等间距 (16px)
    #[default]
    Middle,
    /// 大间距 (24px)
    Large,
    /// 自定义间距
    Custom(u32),
}

impl SpaceSize {
    /// 转换为CSS类名
    pub fn to_class(&self) -> &'static str {
        match self {
            SpaceSize::Small => "small",
            SpaceSize::Middle => "middle",
            SpaceSize::Large => "large",
            SpaceSize::Custom(_) => "custom",
        }
    }
    
    /// 获取间距值（像素）
    pub fn to_pixels(&self) -> u32 {
        match self {
            SpaceSize::Small => 8,
            SpaceSize::Middle => 16,
            SpaceSize::Large => 24,
            SpaceSize::Custom(size) => *size,
        }
    }
}

/// Space 组件的属性
#[derive(Props, Clone, PartialEq)]
pub struct SpaceProps {
    /// 子元素
    pub children: Element,
    
    /// 间距方向
    #[props(default)]
    pub direction: SpaceDirection,
    
    /// 间距大小
    #[props(default)]
    pub size: SpaceSize,
    
    /// 对齐方式
    #[props(default)]
    pub align: SpaceAlign,
    
    /// 是否自动换行，仅在 horizontal 时有效
    #[props(default = false)]
    pub wrap: bool,
    
    /// 设置拆分元素
    #[props(default)]
    pub split: Option<Element>,
    
    /// 自定义CSS类名
    #[props(default)]
    pub class: Option<String>,
    
    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,
    
    /// 自定义前缀类名
    #[props(default)]
    pub prefix_cls: Option<String>,
}
//! Space 组件类型定义
//!
//! 定义了 Space 组件及其子组件的所有属性结构体、枚举类型和配置选项。

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Space 主组件属性
#[derive(Props, Clone, PartialEq)]
pub struct SpaceProps {
    /// 子元素
    #[props(default)]
    pub children: Element,

    /// 对齐方式
    #[props(default)]
    pub align: Option<SpaceAlign>,

    /// 自定义类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,

    /// 间距方向
    #[props(default)]
    pub direction: Option<SpaceDirection>,

    /// 间距大小
    #[props(default)]
    pub size: Option<SpaceSizeConfig>,

    /// 分割元素
    #[props(default)]
    pub split: Option<Element>,

    /// 是否自动换行（仅水平方向有效）
    #[props(default)]
    pub wrap: Option<bool>,

    /// 主题配置
    #[props(default)]
    pub theme: Option<SpaceTheme>,

    /// 点击事件
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
}

/// Space.Compact 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct SpaceCompactProps {
    /// 子元素
    #[props(default)]
    pub children: Element,

    /// 自定义类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,

    /// 是否适应父容器宽度
    #[props(default)]
    pub block: Option<bool>,

    /// 布局方向
    #[props(default)]
    pub direction: Option<SpaceDirection>,

    /// 子组件尺寸
    #[props(default)]
    pub size: Option<CompactSize>,

    /// 主题配置
    #[props(default)]
    pub theme: Option<SpaceTheme>,

    /// 点击事件
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
}

/// 间距方向枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpaceDirection {
    /// 水平方向
    Horizontal,
    /// 垂直方向
    Vertical,
}

/// 对齐方式枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpaceAlign {
    /// 起始对齐
    Start,
    /// 结束对齐
    End,
    /// 居中对齐
    Center,
    /// 基线对齐
    Baseline,
}

/// 间距大小枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpaceSize {
    /// 小间距 (8px)
    Small,
    /// 中等间距 (16px)
    Middle,
    /// 大间距 (24px)
    Large,
    /// 自定义数值间距
    Custom(u32),
}

/// 间距大小配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SpaceSizeConfig {
    /// 单一尺寸
    Single(SpaceSize),
    /// 数组形式 [水平间距, 垂直间距]
    Array([u32; 2]),
    /// 字符串形式（预设值）
    String(String),
}

/// 紧凑模式尺寸枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompactSize {
    /// 大尺寸
    Large,
    /// 中等尺寸
    Middle,
    /// 小尺寸
    Small,
}

/// Space 主题配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpaceTheme {
    /// 小间距值
    pub space_small: u32,
    /// 中等间距值
    pub space_middle: u32,
    /// 大间距值
    pub space_large: u32,
    /// 紧凑模式边框颜色
    pub compact_border_color: String,
    /// 紧凑模式边框宽度
    pub compact_border_width: u32,
    /// 紧凑模式圆角
    pub compact_border_radius: u32,
    /// 自定义CSS变量
    pub css_vars: HashMap<String, String>,
}

/// Space 组件状态
#[derive(Debug, Clone, PartialEq)]
pub struct SpaceState {
    /// 是否已初始化
    pub initialized: bool,
    /// 当前方向
    pub current_direction: SpaceDirection,
    /// 当前间距值
    pub current_gap: u32,
    /// 是否换行
    pub is_wrapped: bool,
    /// 子元素数量
    pub children_count: usize,
}

impl Default for SpaceProps {
    fn default() -> Self {
        SpaceProps::new()
    }
}

impl Default for SpaceCompactProps {
    fn default() -> Self {
        SpaceCompactProps::new()
    }
}
// 默认实现
impl Default for SpaceDirection {
    fn default() -> Self {
        Self::Horizontal
    }
}

impl Default for SpaceAlign {
    fn default() -> Self {
        Self::Start
    }
}

impl Default for SpaceSize {
    fn default() -> Self {
        Self::Small
    }
}

impl Default for SpaceSizeConfig {
    fn default() -> Self {
        Self::Single(SpaceSize::Small)
    }
}

impl Default for CompactSize {
    fn default() -> Self {
        Self::Middle
    }
}

impl Default for SpaceTheme {
    fn default() -> Self {
        let mut css_vars = HashMap::new();
        css_vars.insert("--ant-space-small".to_string(), "8px".to_string());
        css_vars.insert("--ant-space-middle".to_string(), "16px".to_string());
        css_vars.insert("--ant-space-large".to_string(), "24px".to_string());

        Self {
            space_small: 8,
            space_middle: 16,
            space_large: 24,
            compact_border_color: "#d9d9d9".to_string(),
            compact_border_width: 1,
            compact_border_radius: 6,
            css_vars,
        }
    }
}

impl Default for SpaceState {
    fn default() -> Self {
        Self {
            initialized: false,
            current_direction: SpaceDirection::Horizontal,
            current_gap: 8,
            is_wrapped: false,
            children_count: 0,
        }
    }
}

// 类型转换实现
impl From<&str> for SpaceSize {
    fn from(s: &str) -> Self {
        match s {
            "small" => Self::Small,
            "middle" => Self::Middle,
            "large" => Self::Large,
            _ => {
                if let Ok(num) = s.parse::<u32>() {
                    Self::Custom(num)
                } else {
                    Self::Small
                }
            }
        }
    }
}

impl From<u32> for SpaceSize {
    fn from(num: u32) -> Self {
        match num {
            8 => Self::Small,
            16 => Self::Middle,
            24 => Self::Large,
            _ => Self::Custom(num),
        }
    }
}

impl From<&str> for SpaceDirection {
    fn from(s: &str) -> Self {
        match s {
            "vertical" => Self::Vertical,
            "horizontal" | _ => Self::Horizontal,
        }
    }
}

impl From<&str> for SpaceAlign {
    fn from(s: &str) -> Self {
        match s {
            "start" => Self::Start,
            "end" => Self::End,
            "center" => Self::Center,
            "baseline" => Self::Baseline,
            _ => Self::Start,
        }
    }
}

impl From<&str> for CompactSize {
    fn from(s: &str) -> Self {
        match s {
            "large" => Self::Large,
            "small" => Self::Small,
            "middle" | _ => Self::Middle,
        }
    }
}

// 显示特性实现
impl std::fmt::Display for SpaceDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Horizontal => write!(f, "horizontal"),
            Self::Vertical => write!(f, "vertical"),
        }
    }
}

impl std::fmt::Display for SpaceAlign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Start => write!(f, "start"),
            Self::End => write!(f, "end"),
            Self::Center => write!(f, "center"),
            Self::Baseline => write!(f, "baseline"),
        }
    }
}

impl std::fmt::Display for SpaceSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Small => write!(f, "small"),
            Self::Middle => write!(f, "middle"),
            Self::Large => write!(f, "large"),
            Self::Custom(num) => write!(f, "{}px", num),
        }
    }
}

impl std::fmt::Display for CompactSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Large => write!(f, "large"),
            Self::Middle => write!(f, "middle"),
            Self::Small => write!(f, "small"),
        }
    }
}

// 便捷构造函数
impl SpaceProps {
    /// 创建新的 SpaceProps
    pub fn new() -> Self {
        Self {
            children: Ok(VNode::default()),
            align: None,
            class: None,
            style: None,
            direction: None,
            size: None,
            split: None,
            wrap: None,
            theme: None,
            onclick: None,
        }
    }

    /// 设置方向
    pub fn direction(mut self, direction: SpaceDirection) -> Self {
        self.direction = Some(direction);
        self
    }

    /// 设置间距大小
    pub fn size(mut self, size: SpaceSizeConfig) -> Self {
        self.size = Some(size);
        self
    }

    /// 设置对齐方式
    pub fn align(mut self, align: SpaceAlign) -> Self {
        self.align = Some(align);
        self
    }

    /// 设置是否换行
    pub fn wrap(mut self, wrap: bool) -> Self {
        self.wrap = Some(wrap);
        self
    }

    /// 设置自定义类名
    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.class = Some(class.into());
        self
    }

    /// 创建水平方向的 Space
    pub fn horizontal() -> Self {
        Self {
            direction: Some(SpaceDirection::Horizontal),
            ..Default::default()
        }
    }

    /// 创建垂直方向的 Space
    pub fn vertical() -> Self {
        Self {
            direction: Some(SpaceDirection::Vertical),
            ..Default::default()
        }
    }

    /// 设置分割元素
    pub fn split(mut self, split: Element) -> Self {
        self.split = Some(split);
        self
    }

    /// 设置自定义主题
    pub fn theme(mut self, theme: SpaceTheme) -> Self {
        self.theme = Some(theme);
        self
    }

    /// 设置自定义样式
    pub fn style(mut self, style: impl Into<String>) -> Self {
        self.style = Some(style.into());
        self
    }
}

impl SpaceCompactProps {
    /// 创建新的 SpaceCompactProps
    pub fn new() -> Self {
        Self {
            children: Ok(VNode::default()),
            class: None,
            style: None,
            block: None,
            direction: None,
            size: None,
            theme: None,
            onclick: None,
        }
    }

    /// 创建水平方向的 Space.Compact
    pub fn horizontal() -> Self {
        Self {
            direction: Some(SpaceDirection::Horizontal),
            ..Default::default()
        }
    }

    /// 创建垂直方向的 Space.Compact
    pub fn vertical() -> Self {
        Self {
            direction: Some(SpaceDirection::Vertical),
            ..Default::default()
        }
    }

    /// 设置是否块级
    pub fn block(mut self, block: bool) -> Self {
        self.block = Some(block);
        self
    }

    /// 设置方向
    pub fn direction(mut self, direction: SpaceDirection) -> Self {
        self.direction = Some(direction);
        self
    }

    /// 设置尺寸
    pub fn size(mut self, size: CompactSize) -> Self {
        self.size = Some(size);
        self
    }

    /// 设置自定义类名
    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.class = Some(class.into());
        self
    }
}

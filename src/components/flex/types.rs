//! Flex 组件的类型定义
//!
//! 包含 Flex 组件的所有类型定义，包括 Props、枚举类型、主题配置等。

use dioxus::prelude::*;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Flex 组件的 Props
#[derive(Props, Clone, PartialEq)]
pub struct FlexProps {
    /// 是否垂直布局
    #[props(default = false)]
    pub vertical: bool,

    /// 换行方式
    #[props(default = FlexWrap::NoWrap)]
    pub wrap: FlexWrap,

    /// 主轴对齐方式
    #[props(default = FlexJustify::Normal)]
    pub justify: FlexJustify,

    /// 交叉轴对齐方式
    #[props(default = FlexAlign::Normal)]
    pub align: FlexAlign,

    /// flex 简写属性
    #[props(optional)]
    pub flex: Option<String>,

    /// 间距设置
    #[props(optional)]
    pub gap: Option<FlexGap>,

    /// 自定义类名
    #[props(optional)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(optional)]
    pub style: Option<String>,

    /// 自定义组件类型
    #[props(default = "div")]
    pub component: &'static str,

    /// 子元素
    #[props(optional)]
    pub children: Element,
}

/// Flex 项目组件
#[derive(Props, Clone, PartialEq)]
pub struct FlexItemProps {
    /// flex 属性
    #[props(optional)]
    pub flex: Option<String>,

    /// flex-grow
    #[props(optional)]
    pub flex_grow: Option<f32>,

    /// flex-shrink
    #[props(optional)]
    pub flex_shrink: Option<f32>,

    /// flex-basis
    #[props(optional)]
    pub flex_basis: Option<String>,

    /// align-self
    #[props(optional)]
    pub align_self: Option<FlexAlign>,

    /// order
    #[props(optional)]
    pub order: Option<i32>,

    /// 自定义类名
    #[props(optional)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(optional)]
    pub style: Option<String>,

    /// 子元素
    #[props(optional)]
    pub children: Element,
}

/// Flex 容器组件 (高阶组件)
#[derive(Props, Clone, PartialEq)]
pub struct FlexContainerProps {
    /// Flex 配置
    pub config: FlexConfig,

    /// 自定义类名
    #[props(optional)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(optional)]
    pub style: Option<String>,

    /// 子元素
    #[props(optional)]
    pub children: Element,
}

/// Flex 网格组件
#[derive(Props, Clone, PartialEq)]
pub struct FlexGridProps {
    /// 列数
    #[props(default = 3)]
    pub cols: usize,

    /// 间距
    #[props(default = FlexGap::Middle)]
    pub gap: FlexGap,

    /// 自定义类名
    #[props(optional)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(optional)]
    pub style: Option<String>,

    /// 子元素
    #[props(optional)]
    pub children: Element,
}

/// Flex 布局组件
#[derive(Props, Clone, PartialEq)]
pub struct FlexLayoutProps {
    /// 布局区域
    #[props(default = vec!["header".to_string(), "content".to_string(), "footer".to_string()])]
    pub areas: Vec<String>,

    /// 自定义类名
    #[props(optional)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(optional)]
    pub style: Option<String>,

    /// 子元素
    #[props(optional)]
    pub children: Element,
}

/// 换行方式枚举
#[derive(Clone, PartialEq, Debug, Default)]
pub enum FlexWrap {
    /// 不换行
    #[default]
    NoWrap,
    /// 换行
    Wrap,
    /// 反向换行
    WrapReverse,
}

/// 主轴对齐方式枚举
#[derive(Clone, PartialEq, Debug, Default)]
pub enum FlexJustify {
    /// 默认对齐
    #[default]
    Normal,
    /// 起始对齐
    FlexStart,
    /// 居中对齐
    Center,
    /// 结束对齐
    FlexEnd,
    /// 两端对齐
    SpaceBetween,
    /// 环绕对齐
    SpaceAround,
    /// 均匀对齐
    SpaceEvenly,
}

/// 交叉轴对齐方式枚举
#[derive(Clone, PartialEq, Debug, Default)]
pub enum FlexAlign {
    /// 默认对齐
    #[default]
    Normal,
    /// 起始对齐
    FlexStart,
    /// 居中对齐
    Center,
    /// 结束对齐
    FlexEnd,
    /// 拉伸对齐
    Stretch,
    /// 基线对齐
    Baseline,
}

/// 间距设置枚举
#[derive(Clone, PartialEq, Debug)]
pub enum FlexGap {
    /// 小间距 (8px)
    Small,
    /// 中等间距 (16px)
    Middle,
    /// 大间距 (24px)
    Large,
    /// 自定义间距
    Custom(String),
    /// 双轴间距 [horizontal, vertical]
    Array([String; 2]),
}

/// Flex 主题配置
#[derive(Clone, PartialEq, Debug)]
pub struct FlexTheme {
    /// 小间距值
    pub gap_small: String,
    /// 中等间距值
    pub gap_middle: String,
    /// 大间距值
    pub gap_large: String,
    /// 默认字体大小
    pub font_size: String,
    /// 默认行高
    pub line_height: String,
    /// 默认颜色
    pub color: String,
    /// 背景颜色
    pub background_color: String,
    /// 边框颜色
    pub border_color: String,
    /// 边框半径
    pub border_radius: String,
    /// 阴影
    pub box_shadow: String,
    /// 过渡动画
    pub transition: String,
    /// 响应式断点
    pub breakpoints: HashMap<String, String>,
}

/// Flex 配置
#[derive(Clone, PartialEq, Debug)]
pub struct FlexConfig {
    /// 是否垂直布局
    pub vertical: bool,
    /// 换行方式
    pub wrap: FlexWrap,
    /// 主轴对齐方式
    pub justify: FlexJustify,
    /// 交叉轴对齐方式
    pub align: FlexAlign,
    /// flex 属性
    pub flex: Option<String>,
    /// 间距设置
    pub gap: Option<FlexGap>,
    /// 组件类型
    pub component: String,
    /// 是否有子元素
    pub has_children: bool,
}

/// Flex 项目配置
#[derive(Clone, PartialEq, Debug)]
pub struct FlexItemConfig {
    /// flex 属性
    pub flex: Option<String>,
    /// flex-grow
    pub flex_grow: Option<f32>,
    /// flex-shrink
    pub flex_shrink: Option<f32>,
    /// flex-basis
    pub flex_basis: Option<String>,
    /// align-self
    pub align_self: Option<FlexAlign>,
    /// order
    pub order: Option<i32>,
}

/// 响应式配置
#[derive(Clone, PartialEq, Debug)]
pub struct ResponsiveConfig {
    /// 断点配置
    pub breakpoints: HashMap<String, FlexConfig>,
    /// 当前断点
    pub current_breakpoint: String,
    /// 是否启用响应式
    pub enabled: bool,
}

/// 性能配置
#[derive(Clone, PartialEq, Debug)]
pub struct PerformanceConfig {
    /// 是否启用虚拟化
    pub virtualization: bool,
    /// 最大渲染项目数
    pub max_render_items: usize,
    /// 是否启用懒加载
    pub lazy_loading: bool,
    /// 内存限制 (MB)
    pub memory_limit: usize,
}

// 实现 Display trait
impl Display for FlexWrap {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            FlexWrap::NoWrap => write!(f, "nowrap"),
            FlexWrap::Wrap => write!(f, "wrap"),
            FlexWrap::WrapReverse => write!(f, "wrap-reverse"),
        }
    }
}

impl Display for FlexJustify {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            FlexJustify::Normal => write!(f, "normal"),
            FlexJustify::FlexStart => write!(f, "flex-start"),
            FlexJustify::Center => write!(f, "center"),
            FlexJustify::FlexEnd => write!(f, "flex-end"),
            FlexJustify::SpaceBetween => write!(f, "space-between"),
            FlexJustify::SpaceAround => write!(f, "space-around"),
            FlexJustify::SpaceEvenly => write!(f, "space-evenly"),
        }
    }
}

impl Display for FlexAlign {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            FlexAlign::Normal => write!(f, "normal"),
            FlexAlign::FlexStart => write!(f, "flex-start"),
            FlexAlign::Center => write!(f, "center"),
            FlexAlign::FlexEnd => write!(f, "flex-end"),
            FlexAlign::Stretch => write!(f, "stretch"),
            FlexAlign::Baseline => write!(f, "baseline"),
        }
    }
}

impl Display for FlexGap {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            FlexGap::Small => write!(f, "small"),
            FlexGap::Middle => write!(f, "middle"),
            FlexGap::Large => write!(f, "large"),
            FlexGap::Custom(value) => write!(f, "{}", value),
            FlexGap::Array([h, v]) => write!(f, "{} {}", h, v),
        }
    }
}

// 实现 From trait 用于字符串转换
impl From<&str> for FlexWrap {
    fn from(value: &str) -> Self {
        match value {
            "wrap" => FlexWrap::Wrap,
            "wrap-reverse" => FlexWrap::WrapReverse,
            _ => FlexWrap::NoWrap,
        }
    }
}

impl From<&str> for FlexJustify {
    fn from(value: &str) -> Self {
        match value {
            "flex-start" => FlexJustify::FlexStart,
            "center" => FlexJustify::Center,
            "flex-end" => FlexJustify::FlexEnd,
            "space-between" => FlexJustify::SpaceBetween,
            "space-around" => FlexJustify::SpaceAround,
            "space-evenly" => FlexJustify::SpaceEvenly,
            _ => FlexJustify::Normal,
        }
    }
}

impl From<&str> for FlexAlign {
    fn from(value: &str) -> Self {
        match value {
            "flex-start" => FlexAlign::FlexStart,
            "center" => FlexAlign::Center,
            "flex-end" => FlexAlign::FlexEnd,
            "stretch" => FlexAlign::Stretch,
            "baseline" => FlexAlign::Baseline,
            _ => FlexAlign::Normal,
        }
    }
}

impl From<&str> for FlexGap {
    fn from(value: &str) -> Self {
        match value {
            "small" => FlexGap::Small,
            "middle" => FlexGap::Middle,
            "large" => FlexGap::Large,
            _ => FlexGap::Custom(value.to_string()),
        }
    }
}

/// impl merge for FlexTheme
impl FlexTheme {
    pub fn merge(self, other: &Self) -> Self {
        FlexTheme {
            gap_small: other.gap_small.clone(),
            gap_middle: other.gap_middle.clone(),
            gap_large: other.gap_large.clone(),
            font_size: other.font_size.clone(),
            line_height: other.line_height.clone(),
            color: other.color.clone(),
            background_color: other.background_color.clone(),
            border_color: other.border_color.clone(),
            border_radius: other.border_radius.clone(),
            box_shadow: other.box_shadow.clone(),
            transition: other.transition.clone(),
            breakpoints: other.breakpoints.clone(),
        }
    }
}

// 实现 Default trait
impl Default for FlexTheme {
    fn default() -> Self {
        let mut breakpoints = HashMap::new();
        breakpoints.insert("xs".to_string(), "480px".to_string());
        breakpoints.insert("sm".to_string(), "576px".to_string());
        breakpoints.insert("md".to_string(), "768px".to_string());
        breakpoints.insert("lg".to_string(), "992px".to_string());
        breakpoints.insert("xl".to_string(), "1200px".to_string());
        breakpoints.insert("xxl".to_string(), "1600px".to_string());

        Self {
            gap_small: "8px".to_string(),
            gap_middle: "16px".to_string(),
            gap_large: "24px".to_string(),
            font_size: "14px".to_string(),
            line_height: "1.5715".to_string(),
            color: "rgba(0, 0, 0, 0.88)".to_string(),
            background_color: "#ffffff".to_string(),
            border_color: "#d9d9d9".to_string(),
            border_radius: "6px".to_string(),
            box_shadow: "0 2px 8px rgba(0, 0, 0, 0.15)".to_string(),
            transition: "all 0.2s cubic-bezier(0.645, 0.045, 0.355, 1)".to_string(),
            breakpoints,
        }
    }
}

impl Default for FlexConfig {
    fn default() -> Self {
        Self {
            vertical: false,
            wrap: FlexWrap::NoWrap,
            justify: FlexJustify::Normal,
            align: FlexAlign::Normal,
            flex: None,
            gap: None,
            component: "div".to_string(),
            has_children: false,
        }
    }
}

impl Default for FlexItemConfig {
    fn default() -> Self {
        Self {
            flex: None,
            flex_grow: None,
            flex_shrink: None,
            flex_basis: None,
            align_self: None,
            order: None,
        }
    }
}

impl Default for ResponsiveConfig {
    fn default() -> Self {
        Self {
            breakpoints: HashMap::new(),
            current_breakpoint: "md".to_string(),
            enabled: false,
        }
    }
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            virtualization: false,
            max_render_items: 1000,
            lazy_loading: false,
            memory_limit: 100, // 100MB
        }
    }
}

// 便捷构造函数
impl FlexProps {
    /// 创建新的 FlexProps
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置垂直布局
    pub fn vertical(mut self, vertical: bool) -> Self {
        self.vertical = vertical;
        self
    }

    /// 设置换行方式
    pub fn wrap(mut self, wrap: FlexWrap) -> Self {
        self.wrap = wrap;
        self
    }

    /// 设置主轴对齐方式
    pub fn justify(mut self, justify: FlexJustify) -> Self {
        self.justify = justify;
        self
    }

    /// 设置交叉轴对齐方式
    pub fn align(mut self, align: FlexAlign) -> Self {
        self.align = align;
        self
    }

    /// 设置 flex 属性
    pub fn flex(mut self, flex: impl Into<String>) -> Self {
        self.flex = Some(flex.into());
        self
    }

    /// 设置间距
    pub fn gap(mut self, gap: FlexGap) -> Self {
        self.gap = Some(gap);
        self
    }

    /// 设置自定义类名
    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.class = Some(class.into());
        self
    }

    /// 设置自定义样式
    pub fn style(mut self, style: impl Into<String>) -> Self {
        self.style = Some(style.into());
        self
    }

    /// 设置组件类型
    pub fn component(mut self, component: &'static str) -> Self {
        self.component = component;
        self
    }
}

impl Default for FlexProps {
    fn default() -> Self {
        Self {
            vertical: false,
            wrap: FlexWrap::NoWrap,
            justify: FlexJustify::Normal,
            align: FlexAlign::Normal,
            flex: None,
            gap: None,
            class: None,
            style: None,
            component: "div",
            children: Ok(VNode::default()),
        }
    }
}

// 全局默认主题
static mut DEFAULT_FLEX_THEME: Option<FlexTheme> = None;

/// 设置全局默认 Flex 主题
pub fn set_default_flex_theme(theme: FlexTheme) {
    unsafe {
        DEFAULT_FLEX_THEME = Some(theme);
    }
}

/// 获取全局默认 Flex 主题
pub fn get_default_flex_theme() -> FlexTheme {
    unsafe { DEFAULT_FLEX_THEME.clone().unwrap_or_default() }
}

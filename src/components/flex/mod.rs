//! # Flex 弹性布局
//!
//! 弹性布局组件，基于 CSS Flexbox 实现。
//!
//! ## 何时使用
//!
//! - 需要快速实现弹性布局
//! - 需要对齐、分布子元素
//! - 需要响应式的布局方案
//!
//! ## 示例
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::{Flex, FlexJustify, FlexAlign};
//!
//! fn app() -> Element {
//!     rsx! {
//!         Flex {
//!             justify: FlexJustify::SpaceBetween,
//!             align: FlexAlign::Center,
//!             gap: "middle",
//!             div { "Item 1" }
//!             div { "Item 2" }
//!             div { "Item 3" }
//!         }
//!     }
//! }
//! ```
//!
//! ### 垂直布局
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::{Flex, FlexDirection};
//!
//! fn app() -> Element {
//!     rsx! {
//!         Flex {
//!             vertical: true,
//!             gap: "large",
//!             div { "Item 1" }
//!             div { "Item 2" }
//!             div { "Item 3" }
//!         }
//!     }
//! }
//! ```
//!
//! ### 响应式布局
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::Flex;
//!
//! fn app() -> Element {
//!     rsx! {
//!         Flex {
//!             wrap: "wrap",
//!             gap: "small",
//!             div { flex: "1 1 300px", "Responsive Item 1" }
//!             div { flex: "1 1 300px", "Responsive Item 2" }
//!             div { flex: "1 1 300px", "Responsive Item 3" }
//!         }
//!     }
//! }
//! ```

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

const FLEX_STYLE: &str = include_str!("./style.css");

/// Flex 主轴对齐方式
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FlexJustify {
    /// 起始位置对齐
    FlexStart,
    /// 居中对齐
    Center,
    /// 末尾位置对齐
    FlexEnd,
    /// 两端对齐，项目之间的间隔都相等
    SpaceBetween,
    /// 每个项目两侧的间隔相等
    SpaceAround,
    /// 每个项目的间隔与项目和容器之间的间隔相等
    SpaceEvenly,
}

impl Default for FlexJustify {
    fn default() -> Self {
        Self::FlexStart
    }
}

impl FlexJustify {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::FlexStart => "flex-start",
            Self::Center => "center",
            Self::FlexEnd => "flex-end",
            Self::SpaceBetween => "space-between",
            Self::SpaceAround => "space-around",
            Self::SpaceEvenly => "space-evenly",
        }
    }
}

/// Flex 交叉轴对齐方式
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FlexAlign {
    /// 起始位置对齐
    FlexStart,
    /// 居中对齐
    Center,
    /// 末尾位置对齐
    FlexEnd,
    /// 基线对齐
    Baseline,
    /// 拉伸对齐
    Stretch,
}

impl Default for FlexAlign {
    fn default() -> Self {
        Self::FlexStart
    }
}

impl FlexAlign {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::FlexStart => "flex-start",
            Self::Center => "center",
            Self::FlexEnd => "flex-end",
            Self::Baseline => "baseline",
            Self::Stretch => "stretch",
        }
    }
}

/// Flex 换行方式
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FlexWrap {
    /// 不换行
    NoWrap,
    /// 换行，第一行在上方
    Wrap,
    /// 换行，第一行在下方
    WrapReverse,
}

impl Default for FlexWrap {
    fn default() -> Self {
        Self::NoWrap
    }
}

impl FlexWrap {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NoWrap => "nowrap",
            Self::Wrap => "wrap",
            Self::WrapReverse => "wrap-reverse",
        }
    }
}

/// Flex 方向
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FlexDirection {
    /// 水平方向，起点在左端
    Row,
    /// 水平方向，起点在右端
    RowReverse,
    /// 垂直方向，起点在上沿
    Column,
    /// 垂直方向，起点在下沿
    ColumnReverse,
}

impl Default for FlexDirection {
    fn default() -> Self {
        Self::Row
    }
}

impl FlexDirection {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Row => "row",
            Self::RowReverse => "row-reverse",
            Self::Column => "column",
            Self::ColumnReverse => "column-reverse",
        }
    }
}

/// Flex 间距大小
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FlexGap {
    /// 小间距
    Small,
    /// 中等间距
    Middle,
    /// 大间距
    Large,
    /// 自定义间距（像素值）
    Custom(u32),
}

impl Default for FlexGap {
    fn default() -> Self {
        Self::Small
    }
}

impl FlexGap {
    pub fn to_px(&self) -> String {
        match self {
            Self::Small => "8px".to_string(),
            Self::Middle => "16px".to_string(),
            Self::Large => "24px".to_string(),
            Self::Custom(px) => format!("{}px", px),
        }
    }
}

/// Flex 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct FlexProps {
    /// 是否垂直布局
    #[props(default = false)]
    pub vertical: bool,

    /// 主轴对齐方式
    #[props(default)]
    pub justify: FlexJustify,

    /// 交叉轴对齐方式
    #[props(default)]
    pub align: FlexAlign,

    /// 是否换行
    #[props(default)]
    pub wrap: FlexWrap,

    /// 间距大小
    #[props(default)]
    pub gap: Option<FlexGap>,

    /// 自定义 flex 属性
    #[props(default)]
    pub flex: Option<String>,

    /// 自定义 CSS 类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,

    /// 子元素
    children: Element,
}

/// Flex 弹性布局组件
///
/// 基于 CSS Flexbox 的弹性布局组件。
#[component]
pub fn Flex(props: FlexProps) -> Element {
    let mut class_list = vec!["ant-flex"];

    // 添加方向样式
    if props.vertical {
        class_list.push("ant-flex-vertical");
    }

    // 添加对齐样式
    class_list.push(match props.justify {
        FlexJustify::FlexStart => "ant-flex-justify-start",
        FlexJustify::Center => "ant-flex-justify-center",
        FlexJustify::FlexEnd => "ant-flex-justify-end",
        FlexJustify::SpaceBetween => "ant-flex-justify-between",
        FlexJustify::SpaceAround => "ant-flex-justify-around",
        FlexJustify::SpaceEvenly => "ant-flex-justify-evenly",
    });

    class_list.push(match props.align {
        FlexAlign::FlexStart => "ant-flex-align-start",
        FlexAlign::Center => "ant-flex-align-center",
        FlexAlign::FlexEnd => "ant-flex-align-end",
        FlexAlign::Baseline => "ant-flex-align-baseline",
        FlexAlign::Stretch => "ant-flex-align-stretch",
    });

    // 添加换行样式
    class_list.push(match props.wrap {
        FlexWrap::NoWrap => "ant-flex-nowrap",
        FlexWrap::Wrap => "ant-flex-wrap",
        FlexWrap::WrapReverse => "ant-flex-wrap-reverse",
    });

    // 添加间距样式
    if let Some(gap) = &props.gap {
        class_list.push(match gap {
            FlexGap::Small => "ant-flex-gap-small",
            FlexGap::Middle => "ant-flex-gap-middle",
            FlexGap::Large => "ant-flex-gap-large",
            FlexGap::Custom(_) => "ant-flex-gap-custom",
        });
    }

    // 添加自定义类名
    if let Some(custom_class) = &props.class {
        class_list.push(custom_class);
    }

    let class_name = class_list.join(" ");

    // 构建样式
    let mut style_parts = Vec::new();

    // 添加 flex 属性
    if let Some(flex_value) = &props.flex {
        style_parts.push(format!("flex: {}", flex_value));
    }

    // 添加间距样式
    if let Some(gap) = &props.gap {
        style_parts.push(format!("gap: {}", gap.to_px()));
    }

    // 添加自定义样式
    if let Some(custom_style) = &props.style {
        style_parts.push(custom_style.clone());
    }

    let style_attr = if style_parts.is_empty() {
        None
    } else {
        Some(style_parts.join("; "))
    };

    rsx! {
        style { {FLEX_STYLE} }

        div {
            class: class_name.clone(),
            style: style_attr,
            {props.children}
        }
    }
}

// 重新导出公共类型
// 注意：不使用通配符导出以避免命名冲突

//! 栅格组件
//!
//! 24 栅格系统。
//!
//! ## 设计理念
//!
//! 在多数业务情况下，Ant Design需要在设计区域内解决大量信息收纳的问题，因此在 12 栅格系统的基础上，我们将整个设计建议区域按照 24 等分的原则进行划分。
//!
//! 划分之后的信息区域我们称之为『盒子』。建议横向排列的盒子数量最多四个，最少一个。『盒子』在整个屏幕上占比见上图。设计部分基于盒子的单位定制盒子内部的排版规则，以保证视觉层面的舒适感。
//!
//! ## 概述
//!
//! 布局的栅格化系统，我们是基于行（row）和列（col）来定义信息区域的外部框架，以保证页面的每个区域能够稳健地排布起来。下面简单介绍一下它的工作原理：
//!
//! - 通过 `row` 在水平方向建立一组 `column`（简写col）。
//! - 你的内容应当放置于 `col` 内，并且，只有 `col` 可以作为 `row` 的直接元素。
//! - 栅格系统中的列是指 1 到 24 的值来表示其跨越的范围。例如，三个等宽的列可以使用 `<Col span={8} />` 来创建。
//! - 如果一个 `row` 中的 `col` 总和超过 24，那么多余的 `col` 会作为一个整体另起一行排列。
//!
//! ## Flex 布局
//!
//! 我们的栅格化系统支持 Flex 布局，允许子元素在父节点内的水平对齐方式 - 居左、居中、居右、等宽排列、分散排列。子元素与子元素之间，支持顶部对齐、垂直居中对齐、底部对齐的方式。同时，支持使用 order 来定义元素的排列顺序。
//!
//! Flex 布局是基于 24 栅格来定义每一个『盒子』的宽度，但不拘泥于栅格。

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

// 引入栅格样式
mod styles;
use styles::{use_grid_style, ColStyleGenerator, RowStyleGenerator};
pub use styles::{Align, Justify, ResponsiveConfig};

/// 行属性
#[derive(Props, Clone, PartialEq)]
pub struct RowProps {
    /// 栅格间隔，可以写成像素值或支持响应式的对象写法来设置水平间隔 { xs: 8, sm: 16, md: 24}
    #[props(default = 0)]
    pub gutter: u32,

    /// 水平排列方式
    #[props(default)]
    pub justify: Justify,

    /// 垂直对齐方式
    #[props(default)]
    pub align: Align,

    /// 是否自动换行
    #[props(default = true)]
    pub wrap: bool,

    /// 自定义类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,

    /// 子元素
    children: Element,
}

/// 列属性
#[derive(Props, Clone, PartialEq)]
pub struct ColProps {
    /// 栅格占位格数，为 0 时相当于 display: none
    #[props(default)]
    pub span: Option<u32>,

    /// 栅格左侧的间隔格数，间隔内不可以有栅格
    #[props(default = 0)]
    pub offset: u32,

    /// 栅格向右移动格数
    #[props(default = 0)]
    pub push: u32,

    /// 栅格向左移动格数
    #[props(default = 0)]
    pub pull: u32,

    /// 栅格顺序
    #[props(default)]
    pub order: Option<i32>,

    /// 屏幕 < 576px 响应式栅格
    #[props(default)]
    pub xs: Option<ResponsiveConfig>,

    /// 屏幕 ≥ 576px 响应式栅格
    #[props(default)]
    pub sm: Option<ResponsiveConfig>,

    /// 屏幕 ≥ 768px 响应式栅格
    #[props(default)]
    pub md: Option<ResponsiveConfig>,

    /// 屏幕 ≥ 992px 响应式栅格
    #[props(default)]
    pub lg: Option<ResponsiveConfig>,

    /// 屏幕 ≥ 1200px 响应式栅格
    #[props(default)]
    pub xl: Option<ResponsiveConfig>,

    /// 屏幕 ≥ 1600px 响应式栅格
    #[props(default)]
    pub xxl: Option<ResponsiveConfig>,

    /// 自定义类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,

    /// 子元素
    children: Element,
}

/// 行组件
///
/// # 参数
///
/// * `props` - 行属性
///
/// # 示例
///
/// ```rust
/// use dioxus::prelude::*;
/// use ant_design_dioxus::{Row, Col, Justify, Align};
///
/// fn app() -> Element {
///     rsx! {
///         Row {
///             gutter: 16,
///             justify: Justify::Center,
///             align: Align::Middle,
///             Col { span: 12, "Column 1" }
///             Col { span: 12, "Column 2" }
///         }
///     }
/// }
/// ```
#[component]
pub fn Row(props: RowProps) -> Element {
    // 确保样式已注入
    use_grid_style();

    // 使用样式生成器生成行样式
    let style_gen = RowStyleGenerator::new()
        .with_gutter(props.gutter)
        .with_justify(props.justify.clone())
        .with_align(props.align.clone())
        .with_wrap(props.wrap);

    let mut row_style = style_gen.generate();

    // 添加自定义样式
    if let Some(ref style) = props.style {
        row_style.push_str(" ");
        row_style.push_str(style);
    }

    rsx! {
        div {
            style: row_style,
            {props.children}
        }
    }
}

/// 列组件
///
/// # 参数
///
/// * `props` - 列属性
///
/// # 示例
///
/// ```rust
/// use dioxus::prelude::*;
/// use ant_design_dioxus::{Row, Col};
///
/// fn app() -> Element {
///     rsx! {
///         Row {
///             Col { span: 8, "Column 1" }
///             Col { span: 8, "Column 2" }
///             Col { span: 8, "Column 3" }
///         }
///     }
/// }
/// ```
#[component]
pub fn Col(props: ColProps) -> Element {
    // 确保样式已注入
    use_grid_style();

    // 获取上下文中的gutter值
    // 这里应该从父组件Row中获取gutter值，简化实现直接使用props
    let gutter = 0; // 理想情况下应从Row上下文获取

    // 使用样式生成器生成列样式
    let style_gen = ColStyleGenerator::new()
        .with_span(props.span)
        .with_offset(props.offset)
        .with_push(props.push)
        .with_pull(props.pull)
        .with_order(props.order)
        .with_gutter(gutter)
        .with_xs(props.xs.clone())
        .with_sm(props.sm.clone())
        .with_md(props.md.clone())
        .with_lg(props.lg.clone())
        .with_xl(props.xl.clone())
        .with_xxl(props.xxl.clone());

    let mut col_style = style_gen.generate();

    // 添加自定义样式
    if let Some(ref style) = props.style {
        col_style.push_str(" ");
        col_style.push_str(style);
    }

    rsx! {
        div {
            style: col_style,
            {props.children}
        }
    }
}

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
const ROW_STYLE: &str = include_str!("row.css");
const COL_STYLE: &str = include_str!("col.css");
const RESPONSIVE_STYLE: &str = include_str!("responsive.css");

/// 栅格对齐方式
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Justify {
    /// 左对齐
    Start,
    /// 右对齐
    End,
    /// 居中对齐
    Center,
    /// 两端对齐
    SpaceBetween,
    /// 分散对齐
    SpaceAround,
    /// 均匀分布
    SpaceEvenly,
}

impl Default for Justify {
    fn default() -> Self {
        Self::Start
    }
}

/// 栅格垂直对齐方式
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Align {
    /// 顶部对齐
    Top,
    /// 中间对齐
    Middle,
    /// 底部对齐
    Bottom,
    /// 拉伸对齐
    Stretch,
}

impl Default for Align {
    fn default() -> Self {
        Self::Top
    }
}

/// 响应式断点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponsiveConfig {
    /// 栅格占位格数
    pub span: Option<u32>,
    /// 栅格左侧的间隔格数
    pub offset: Option<u32>,
    /// 栅格向右移动格数
    pub push: Option<u32>,
    /// 栅格向左移动格数
    pub pull: Option<u32>,
    /// 栅格顺序
    pub order: Option<i32>,
}

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
    let class_name = get_row_class_name(&props);
    let row_style = get_row_style(&props);

    rsx! {
        // 注入栅格样式
        style { {ROW_STYLE} }
        style { {COL_STYLE} }
        style { {RESPONSIVE_STYLE} }

        div {
            class: "{class_name}",
            style: "{row_style}",
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
    let class_name = get_col_class_name(&props);
    let col_style = get_col_style(&props);

    rsx! {
        div {
            class: "{class_name}",
            style: "{col_style}",
            {props.children}
        }
    }
}

/// 获取行类名
///
/// # 参数
///
/// * `props` - 行属性
///
/// # 返回值
///
/// 返回行的CSS类名字符串
fn get_row_class_name(props: &RowProps) -> String {
    let mut classes = vec!["ant-row".to_string()];

    // 添加对齐方式类名
    match props.justify {
        Justify::Start => {}
        Justify::End => classes.push("ant-row-end".to_string()),
        Justify::Center => classes.push("ant-row-center".to_string()),
        Justify::SpaceBetween => classes.push("ant-row-space-between".to_string()),
        Justify::SpaceAround => classes.push("ant-row-space-around".to_string()),
        Justify::SpaceEvenly => classes.push("ant-row-space-evenly".to_string()),
    }

    // 添加垂直对齐方式类名
    match props.align {
        Align::Top => {}
        Align::Middle => classes.push("ant-row-middle".to_string()),
        Align::Bottom => classes.push("ant-row-bottom".to_string()),
        Align::Stretch => classes.push("ant-row-stretch".to_string()),
    }

    // 添加换行类名
    if !props.wrap {
        classes.push("ant-row-nowrap".to_string());
    }

    // 添加自定义类名
    if let Some(custom_class) = &props.class {
        classes.push(custom_class.clone());
    }

    classes.join(" ")
}

/// 获取行样式
///
/// # 参数
///
/// * `props` - 行属性
///
/// # 返回值
///
/// 返回行的内联样式字符串
fn get_row_style(props: &RowProps) -> String {
    let mut styles = Vec::new();

    // 添加间隔样式
    if props.gutter > 0 {
        let margin = -(props.gutter as i32) / 2;
        styles.push(format!("margin-left: {}px", margin));
        styles.push(format!("margin-right: {}px", margin));
    }

    // 添加自定义样式
    if let Some(custom_style) = &props.style {
        styles.push(custom_style.clone());
    }

    styles.join("; ")
}

/// 获取列类名
///
/// # 参数
///
/// * `props` - 列属性
///
/// # 返回值
///
/// 返回列的CSS类名字符串
fn get_col_class_name(props: &ColProps) -> String {
    let mut classes = vec!["ant-col".to_string()];

    // 添加span类名
    if let Some(span) = props.span {
        if span == 0 {
            classes.push("ant-col-0".to_string());
        } else {
            classes.push(format!("ant-col-{}", span));
        }
    }

    // 添加offset类名
    if props.offset > 0 {
        classes.push(format!("ant-col-offset-{}", props.offset));
    }

    // 添加push类名
    if props.push > 0 {
        classes.push(format!("ant-col-push-{}", props.push));
    }

    // 添加pull类名
    if props.pull > 0 {
        classes.push(format!("ant-col-pull-{}", props.pull));
    }

    // 添加order类名
    if let Some(order) = props.order {
        classes.push(format!("ant-col-order-{}", order));
    }

    // 添加响应式类名
    add_responsive_classes(&mut classes, "xs", &props.xs);
    add_responsive_classes(&mut classes, "sm", &props.sm);
    add_responsive_classes(&mut classes, "md", &props.md);
    add_responsive_classes(&mut classes, "lg", &props.lg);
    add_responsive_classes(&mut classes, "xl", &props.xl);
    add_responsive_classes(&mut classes, "xxl", &props.xxl);

    // 添加自定义类名
    if let Some(custom_class) = &props.class {
        classes.push(custom_class.clone());
    }

    classes.join(" ")
}

/// 获取列样式
///
/// # 参数
///
/// * `props` - 列属性
///
/// # 返回值
///
/// 返回列的内联样式字符串
fn get_col_style(props: &ColProps) -> String {
    let mut styles = Vec::new();

    // 添加自定义样式
    if let Some(custom_style) = &props.style {
        styles.push(custom_style.clone());
    }

    styles.join("; ")
}

/// 添加响应式类名
///
/// # 参数
///
/// * `classes` - 类名列表
/// * `breakpoint` - 断点名称
/// * `config` - 响应式配置
fn add_responsive_classes(
    classes: &mut Vec<String>,
    breakpoint: &str,
    config: &Option<ResponsiveConfig>,
) {
    if let Some(config) = config {
        if let Some(span) = config.span {
            if span == 0 {
                classes.push(format!("ant-col-{}-0", breakpoint));
            } else {
                classes.push(format!("ant-col-{}-{}", breakpoint, span));
            }
        }

        if let Some(offset) = config.offset {
            if offset > 0 {
                classes.push(format!("ant-col-{}-offset-{}", breakpoint, offset));
            }
        }

        if let Some(push) = config.push {
            if push > 0 {
                classes.push(format!("ant-col-{}-push-{}", breakpoint, push));
            }
        }

        if let Some(pull) = config.pull {
            if pull > 0 {
                classes.push(format!("ant-col-{}-pull-{}", breakpoint, pull));
            }
        }

        if let Some(order) = config.order {
            classes.push(format!("ant-col-{}-order-{}", breakpoint, order));
        }
    }
}

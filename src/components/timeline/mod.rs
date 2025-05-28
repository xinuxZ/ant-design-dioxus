//! Timeline 时间轴组件
//!
//! 垂直展示的时间流信息。
//!
//! ## 何时使用
//!
//! - 当有一系列信息需按时间排列时，可正序和倒序。
//! - 需要有一条时间轴进行视觉上的串联时。

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

/// Timeline 时间轴组件属性
#[derive(Props, Clone, PartialEq)]
pub struct TimelineProps {
    /// 子元素
    children: Element,
    /// 自定义类名
    #[props(default = "".to_string())]
    class: String,
    /// 自定义样式
    #[props(default = "".to_string())]
    style: String,
    /// 指定最后一个幽灵节点是否存在或内容
    #[props(default = false)]
    pending: bool,
    /// 当最后一个幽灵节点存在時，指定其时间或内容
    #[props(default = "".to_string())]
    pending_dot: String,
    /// 通过设置 mode 可以改变时间轴和内容的相对位置
    #[props(default = TimelineMode::Left)]
    mode: TimelineMode,
    /// 是否倒序
    #[props(default = false)]
    reverse: bool,
}

/// Timeline 时间轴模式
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum TimelineMode {
    /// 左侧
    Left,
    /// 右侧
    Right,
    /// 交替
    Alternate,
}

/// Timeline 时间轴组件
#[component]
pub fn Timeline(props: TimelineProps) -> Element {
    let class = format!(
        "ant-timeline {} {}",
        match props.mode {
            TimelineMode::Left => "ant-timeline-left",
            TimelineMode::Right => "ant-timeline-right",
            TimelineMode::Alternate => "ant-timeline-alternate",
        },
        if props.reverse {
            "ant-timeline-reverse"
        } else {
            ""
        }
    );

    let class = if !props.class.is_empty() {
        format!("{} {}", class, props.class)
    } else {
        class
    };

    rsx! {
        div {
            class: class.clone(),
            style: props.style.clone(),
            {props.children}
            if props.pending {
                TimelineItem {
                    dot: props.pending_dot.clone(),
                    class: "ant-timeline-item-pending",
                    "Loading..."
                }
            }
        }
        style { {include_str!("./style.css")} }
    }
}

/// TimelineItem 时间轴项属性
#[derive(Props, Clone, PartialEq)]
pub struct TimelineItemProps {
    /// 子元素
    children: Element,
    /// 自定义类名
    #[props(default = "".to_string())]
    class: String,
    /// 自定义样式
    #[props(default = "".to_string())]
    style: String,
    /// 自定义时间轴点
    #[props(default = "".to_string())]
    dot: String,
    /// 指定圆圈颜色
    #[props(default = TimelineItemColor::Blue)]
    color: TimelineItemColor,
    /// 设置标签
    #[props(default = "".to_string())]
    label: String,
    /// 时间轴项位置
    #[props(default = None)]
    position: Option<TimelineItemPosition>,
}

/// Timeline 项颜色
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum TimelineItemColor {
    /// 蓝色
    Blue,
    /// 红色
    Red,
    /// 绿色
    Green,
    /// 灰色
    Gray,
    /// 自定义颜色
    Custom(String),
}

/// Timeline 项位置
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum TimelineItemPosition {
    /// 左侧
    Left,
    /// 右侧
    Right,
}

/// TimelineItem 时间轴项组件
#[component]
pub fn TimelineItem(props: TimelineItemProps) -> Element {
    let color_class = match &props.color {
        TimelineItemColor::Blue => "ant-timeline-item-blue",
        TimelineItemColor::Red => "ant-timeline-item-red",
        TimelineItemColor::Green => "ant-timeline-item-green",
        TimelineItemColor::Gray => "ant-timeline-item-gray",
        TimelineItemColor::Custom(_) => "ant-timeline-item-custom",
    };

    let position_class = match &props.position {
        Some(TimelineItemPosition::Left) => "ant-timeline-item-left",
        Some(TimelineItemPosition::Right) => "ant-timeline-item-right",
        None => "",
    };

    let class = format!(
        "ant-timeline-item {} {} {}",
        color_class, position_class, props.class
    )
    .trim()
    .to_string();

    let dot_style = if let TimelineItemColor::Custom(color) = &props.color {
        format!("border-color: {}; color: {};", color, color)
    } else {
        String::new()
    };

    rsx! {
        div {
            class: class.clone(),
            style: props.style.clone(),
            div {
                class: "ant-timeline-item-tail"
            }
            div {
                class: "ant-timeline-item-head",
                div {
                    class: "ant-timeline-item-head-custom",
                    style: dot_style.clone(),
                    if !props.dot.is_empty() {
                        span { {props.dot.clone()} }
                    } else {
                        span { class: "ant-timeline-item-dot" }
                    }
                }
            }
            div {
                class: "ant-timeline-item-content",
                if !props.label.is_empty() {
                    div {
                        class: "ant-timeline-item-label",
                        {props.label.clone()}
                    }
                }
                div {
                    class: "ant-timeline-item-description",
                    {props.children}
                }
            }
        }
    }
}

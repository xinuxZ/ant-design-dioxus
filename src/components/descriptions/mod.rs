//! Descriptions 描述列表组件
//!
//! 成组展示多个只读字段。
//!
//! ## 何时使用
//!
//! 常见于详情页的信息展示。

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

const DESC_STYLE: &str = include_str!("./style.css");

/// Descriptions 描述列表组件属性
#[derive(Props, Clone, PartialEq)]
pub struct DescriptionsProps {
    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 描述列表的标题，显示在最顶部
    #[props(default)]
    pub title: Option<String>,

    /// 操作区域，显示在右上角
    #[props(default)]
    pub extra: Option<Element>,

    /// 是否展示边框
    #[props(default = false)]
    pub bordered: bool,

    /// 一行的 DescriptionItems 数量，可以写成像素值或支持响应式的对象写法
    #[props(default = 3)]
    pub column: u32,

    /// 设置列表的大小
    #[props(default = DescriptionsSize::Default)]
    pub size: DescriptionsSize,

    /// 表格布局
    #[props(default = DescriptionsLayout::Horizontal)]
    pub layout: DescriptionsLayout,

    /// 配置 Descriptions.Item 的 colon 的默认值
    #[props(default = true)]
    pub colon: bool,

    /// 子元素
    pub children: Element,
}

/// 描述列表大小
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum DescriptionsSize {
    Default,
    Middle,
    Small,
}

impl Default for DescriptionsSize {
    fn default() -> Self {
        Self::Default
    }
}

/// 描述列表布局
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum DescriptionsLayout {
    Horizontal,
    Vertical,
}

impl Default for DescriptionsLayout {
    fn default() -> Self {
        Self::Horizontal
    }
}

/// Descriptions 描述列表组件
#[component]
pub fn Descriptions(props: DescriptionsProps) -> Element {
    let class_name = format!(
        "ant-descriptions {} {} {} {}",
        if props.bordered {
            "ant-descriptions-bordered"
        } else {
            ""
        },
        match props.size {
            DescriptionsSize::Default => "",
            DescriptionsSize::Middle => "ant-descriptions-middle",
            DescriptionsSize::Small => "ant-descriptions-small",
        },
        match props.layout {
            DescriptionsLayout::Horizontal => "",
            DescriptionsLayout::Vertical => "ant-descriptions-vertical",
        },
        props.class
    );

    rsx! {
        style { {DESC_STYLE} }

        div {
            class: class_name.clone(),
            style: props.style.clone(),

            // 标题和操作区域
            if props.title.is_some() || props.extra.is_some() {
                div {
                    class: "ant-descriptions-header",

                    if let Some(title) = &props.title {
                        div {
                            class: "ant-descriptions-title",
                            {title.clone()}
                        }
                    }

                    if let Some(extra) = &props.extra {
                        div {
                            class: "ant-descriptions-extra",

                            {extra.clone()}
                        }
                    }
                }
            }

            // 描述列表内容
            div {
                class: "ant-descriptions-view",

                table {
                    class: "ant-descriptions-table",

                    tbody {
                        {props.children}
                    }
                }
            }
        }

        style { {include_str!("./style.css")} }
    }
}

/// DescriptionsItem 描述列表项属性
#[derive(Props, Clone, PartialEq)]
pub struct DescriptionsItemProps {
    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 内容的描述
    pub label: String,

    /// 包含列的数量
    #[props(default = 1)]
    pub span: u32,

    /// 自定义标签样式
    #[props(default)]
    pub label_style: Option<String>,

    /// 自定义内容样式
    #[props(default)]
    pub content_style: Option<String>,

    /// 子元素
    pub children: Element,
}

/// DescriptionsItem 描述列表项组件
#[component]
pub fn DescriptionsItem(props: DescriptionsItemProps) -> Element {
    let class_name = format!("ant-descriptions-item {}", props.class);

    rsx! {
        tr {
            class: class_name.clone(),
            style: props.style.clone(),

            td {
                class: "ant-descriptions-item-label",
                style: props.label_style.as_ref().map_or("", |s| s.as_str()).to_string(),
                colspan: if props.span > 1 { props.span } else { 1 },

                span {
                    class: "ant-descriptions-item-label-content",
                    {props.label.clone()}
                }

                span {
                    class: "ant-descriptions-item-label-colon",
                    ":"
                }
            }

            td {
                class: "ant-descriptions-item-content",
                style: props.content_style.as_ref().map_or("", |s| s.as_str()).to_string(),
                colspan: if props.span > 1 { props.span } else { 1 },

                {props.children}
            }
        }
    }
}

/// DescriptionsRow 描述列表行属性
#[derive(Props, Clone, PartialEq)]
pub struct DescriptionsRowProps {
    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 子元素
    pub children: Element,
}

/// DescriptionsRow 描述列表行组件
#[component]
pub fn DescriptionsRow(props: DescriptionsRowProps) -> Element {
    let class_name = format!("ant-descriptions-row {}", props.class);

    rsx! {
        tr {
            class: class_name.clone(),
            style: props.style.clone(),

            {props.children}
        }
    }
}

//! List 列表组件
//!
//! 通用列表。
//!
//! ## 何时使用
//!
//! 最基础的列表展示，可承载文字、列表、图片、段落，常用于后台数据展示页面。

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

mod styles;
use styles::use_list_style;

/// 列表尺寸
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ListSize {
    /// 默认尺寸
    Default,
    /// 大尺寸
    Large,
    /// 小尺寸
    Small,
}

impl Default for ListSize {
    fn default() -> Self {
        Self::Default
    }
}

/// 列表布局
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ListLayout {
    /// 水平布局
    Horizontal,
    /// 垂直布局
    Vertical,
}

impl Default for ListLayout {
    fn default() -> Self {
        Self::Vertical
    }
}

/// List 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct ListProps {
    /// 是否有边框
    #[props(default = false)]
    pub bordered: bool,

    /// 列表头部
    #[props(default)]
    pub header: Option<Element>,

    /// 列表底部
    #[props(default)]
    pub footer: Option<Element>,

    /// 列表操作组，根据 itemLayout 的不同, 位置在卡片底部或者最右侧
    #[props(default)]
    pub actions: Option<Element>,

    /// 设置 List.Item 布局, 设置成 vertical 则竖直样式显示, 默认横排
    #[props(default)]
    pub item_layout: ListLayout,

    /// 当卡片内容还在加载中时，可以用 loading 展示一个占位
    #[props(default = false)]
    pub loading: bool,

    /// list 的尺寸
    #[props(default)]
    pub size: ListSize,

    /// 是否展示分割线
    #[props(default = true)]
    pub split: bool,

    /// 自定义 CSS 类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,

    /// 加载状态的骨架屏数量
    #[props(default = 3)]
    pub loading_count: usize,

    /// 子元素
    children: Element,
}

/// List 组件
///
/// 通用列表，可承载文字、列表、图片、段落等内容
///
/// # Props
/// - `bordered`: 是否有边框，默认为 false
/// - `header`: 列表头部
/// - `footer`: 列表底部
/// - `actions`: 列表操作组
/// - `item_layout`: 设置 List.Item 布局，默认为 Vertical
/// - `loading`: 当卡片内容还在加载中时的占位状态，默认为 false
/// - `size`: 列表尺寸，默认为 Default
/// - `split`: 是否展示分割线，默认为 true
/// - `class`: 自定义 CSS 类名
/// - `style`: 自定义样式
/// - `children`: 子元素
#[component]
pub fn List(props: ListProps) -> Element {
    // 获取样式
    let style_class = use_list_style();
    let mut class_list = vec![&style_class, "ant-list"];

    // 添加尺寸类
    match props.size {
        ListSize::Large => class_list.push("ant-list-lg"),
        ListSize::Small => class_list.push("ant-list-sm"),
        ListSize::Default => {}
    }

    // 添加边框类
    if props.bordered {
        class_list.push("ant-list-bordered");
    }

    // 添加布局类
    match props.item_layout {
        ListLayout::Horizontal => class_list.push("ant-list-horizontal"),
        ListLayout::Vertical => class_list.push("ant-list-vertical"),
    }

    // 添加分割线类
    if !props.split {
        class_list.push("ant-list-split");
    }

    // 添加加载类
    if props.loading {
        class_list.push("ant-list-loading");
    }

    // 添加自定义类
    if let Some(custom_class) = &props.class {
        class_list.push(custom_class);
    }

    let class_str = class_list.join(" ");

    rsx! {
        div {
            class: class_str.clone(),
            style: props.style.unwrap_or_default(),

            // 列表头部
            if let Some(header) = &props.header {
                div { class: "ant-list-header", {header} }
            }

            // 列表主体
            div { class: "ant-list-items",
                if props.loading {
                    // 加载状态的骨架屏
                    div { class: "ant-list-loading-content",
                        for i in 0..props.loading_count {
                            div {
                                class: "ant-list-item ant-list-item-loading",
                                key: i.to_string(),
                                div { class: "ant-list-item-meta",
                                    div { class: "ant-list-item-meta-avatar",
                                        div { class: "ant-list-loading-block ant-list-loading-avatar" }
                                    }
                                    div { class: "ant-list-item-meta-content",
                                        div { class: "ant-list-loading-block ant-list-loading-title" }
                                        div { class: "ant-list-loading-block ant-list-loading-description" }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    {props.children}
                }
            }

            // 列表底部
            if let Some(footer) = &props.footer {
                div { class: "ant-list-footer", {footer} }
            }
        }
    }
}

/// ListItem 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct ListItemProps {
    /// 列表操作组，根据 itemLayout 的不同, 位置在卡片底部或者最右侧
    #[props(default)]
    pub actions: Option<Element>,

    /// 额外内容, 通常用在 itemLayout 为 vertical 的情况下, 展示右侧内容; horizontal 展示在列表元素最右侧
    #[props(default)]
    pub extra: Option<Element>,

    /// 自定义 CSS 类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,

    /// 子元素
    children: Element,
}

/// ListItem 组件
///
/// 列表项组件
///
/// # Props
/// - `actions`: 列表操作组
/// - `extra`: 额外内容
/// - `class`: 自定义 CSS 类名
/// - `style`: 自定义样式
/// - `children`: 子元素
#[component]
pub fn ListItem(props: ListItemProps) -> Element {
    let mut class_list = vec!["ant-list-item"];

    if let Some(custom_class) = &props.class {
        class_list.push(custom_class);
    }

    let class_str = class_list.join(" ");

    rsx! {
        div {
            class: class_str.clone(),
            style: props.style.unwrap_or_default(),

            div { class: "ant-list-item-main",
                {props.children}

                // 操作组
                if let Some(actions) = &props.actions {
                    ul { class: "ant-list-item-action",
                        {actions}
                    }
                }
            }

            // 额外内容
            if let Some(extra) = &props.extra {
                div { class: "ant-list-item-extra", {extra} }
            }
        }
    }
}

/// ListItemMeta 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct ListItemMetaProps {
    /// 列表元素的图标
    #[props(default)]
    pub avatar: Option<Element>,

    /// 列表元素的标题
    #[props(default)]
    pub title: Option<String>,

    /// 列表元素的描述内容
    #[props(default)]
    pub description: Option<String>,

    /// 自定义 CSS 类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,
}

/// ListItemMeta 组件
///
/// 列表项元信息组件
///
/// # Props
/// - `avatar`: 列表元素的图标
/// - `title`: 列表元素的标题
/// - `description`: 列表元素的描述内容
/// - `class`: 自定义 CSS 类名
/// - `style`: 自定义样式
#[component]
pub fn ListItemMeta(props: ListItemMetaProps) -> Element {
    let mut class_list = vec!["ant-list-item-meta"];

    if let Some(custom_class) = &props.class {
        class_list.push(custom_class);
    }

    let class_str = class_list.join(" ");

    rsx! {
        div {
            class: class_str.clone(),
            style: props.style.unwrap_or_default(),

            if let Some(avatar) = &props.avatar {
                div { class: "ant-list-item-meta-avatar", {avatar} }
            }

            div { class: "ant-list-item-meta-content",
                if let Some(title) = &props.title {
                    h4 { class: "ant-list-item-meta-title", {title.clone()} }
                }
                if let Some(description) = &props.description {
                    div { class: "ant-list-item-meta-description", {description.clone()} }
                }
            }
        }
    }
}

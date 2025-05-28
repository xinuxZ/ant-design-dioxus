//! Card 卡片组件
//!
//! 通用卡片容器。
//!
//! ## 何时使用
//!
//! 最基础的卡片容器，可承载文字、列表、图片、段落，常用于后台概览页面。

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

// 引入卡片样式
const CARD_STYLE: &str = include_str!("style.css");

/// 卡片尺寸
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CardSize {
    /// 默认尺寸
    Default,
    /// 小尺寸
    Small,
}

impl Default for CardSize {
    fn default() -> Self {
        Self::Default
    }
}

/// 卡片类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CardType {
    /// 默认卡片
    Default,
    /// 内部卡片
    Inner,
}

impl Default for CardType {
    fn default() -> Self {
        Self::Default
    }
}

/// Card 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct CardProps {
    /// 卡片标题
    #[props(default)]
    pub title: Option<String>,

    /// 卡片右上角的操作区域
    #[props(default)]
    pub extra: Option<Element>,

    /// 是否有边框
    #[props(default = true)]
    pub bordered: bool,

    /// 鼠标悬停时可浮起
    #[props(default = false)]
    pub hoverable: bool,

    /// 当卡片内容还在加载中时，可以用 loading 展示一个占位
    #[props(default = false)]
    pub loading: bool,

    /// card 的尺寸
    #[props(default)]
    pub size: CardSize,

    /// 卡片类型
    #[props(default)]
    pub card_type: CardType,

    /// 卡片操作组，位置在卡片底部
    #[props(default)]
    pub actions: Option<Element>,

    /// 自定义 CSS 类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,

    /// 子元素
    children: Element,
}

/// Card 组件
///
/// 通用卡片容器，可承载文字、列表、图片、段落等内容
///
/// # Props
/// - `title`: 卡片标题
/// - `extra`: 卡片右上角的操作区域
/// - `bordered`: 是否有边框，默认为 true
/// - `hoverable`: 鼠标悬停时可浮起，默认为 false
/// - `loading`: 当卡片内容还在加载中时的占位状态，默认为 false
/// - `size`: 卡片尺寸，默认为 Default
/// - `card_type`: 卡片类型，默认为 Default
/// - `actions`: 卡片操作组，位置在卡片底部
/// - `class`: 自定义 CSS 类名
/// - `style`: 自定义样式
/// - `children`: 子元素
#[component]
pub fn Card(props: CardProps) -> Element {
    let mut class_list = vec!["ant-card"];

    // 添加尺寸类
    match props.size {
        CardSize::Small => class_list.push("ant-card-small"),
        CardSize::Default => {}
    }

    // 添加类型类
    match props.card_type {
        CardType::Inner => class_list.push("ant-card-type-inner"),
        CardType::Default => {}
    }

    // 添加边框类
    if !props.bordered {
        class_list.push("ant-card-bordered");
    }

    // 添加悬停类
    if props.hoverable {
        class_list.push("ant-card-hoverable");
    }

    // 添加加载类
    if props.loading {
        class_list.push("ant-card-loading");
    }

    // 添加自定义类
    if let Some(custom_class) = &props.class {
        class_list.push(custom_class);
    }

    let class_str = class_list.join(" ");

    rsx! {
        style { {CARD_STYLE} }
        div {
            class: "{class_str}",
            style: props.style.unwrap_or_default(),

            // 卡片头部
            if props.title.is_some() || props.extra.is_some() {
                div { class: "ant-card-head",
                    div { class: "ant-card-head-wrapper",
                        if let Some(title) = &props.title {
                            div { class: "ant-card-head-title", "{title}" }
                        }
                        if let Some(extra) = &props.extra {
                            div { class: "ant-card-extra", {extra} }
                        }
                    }
                }
            }

            // 卡片主体
            div { class: "ant-card-body",
                if props.loading {
                    // 加载状态的骨架屏
                    div { class: "ant-card-loading-content",
                        div { class: "ant-card-loading-block" }
                        div { class: "ant-card-loading-block" }
                        div { class: "ant-card-loading-block" }
                    }
                } else {
                    {props.children}
                }
            }

            // 卡片操作区域
            if let Some(actions) = &props.actions {
                ul { class: "ant-card-actions",
                    {actions}
                }
            }
        }
    }
}

/// Card.Meta 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct CardMetaProps {
    /// 头像/图标
    #[props(default)]
    pub avatar: Option<Element>,

    /// 标题内容
    #[props(default)]
    pub title: Option<String>,

    /// 描述内容
    #[props(default)]
    pub description: Option<String>,

    /// 自定义 CSS 类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,
}

/// Card.Meta 组件
///
/// 卡片元信息组件，通常用于展示头像、标题和描述
///
/// # Props
/// - `avatar`: 头像/图标
/// - `title`: 标题内容
/// - `description`: 描述内容
/// - `class`: 自定义 CSS 类名
/// - `style`: 自定义样式
#[component]
pub fn CardMeta(props: CardMetaProps) -> Element {
    let mut class_list = vec!["ant-card-meta"];

    if let Some(custom_class) = &props.class {
        class_list.push(custom_class);
    }

    let class_str = class_list.join(" ");

    rsx! {
        div {
            class: "{class_str}",
            style: props.style.unwrap_or_default(),

            div { class: "ant-card-meta-detail",
                if let Some(avatar) = &props.avatar {
                    div { class: "ant-card-meta-avatar", {avatar} }
                }

                div { class: "ant-card-meta-content",
                    if let Some(title) = &props.title {
                        div { class: "ant-card-meta-title", "{title}" }
                    }
                    if let Some(description) = &props.description {
                        div { class: "ant-card-meta-description", "{description}" }
                    }
                }
            }
        }
    }
}

/// Card.Grid 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct CardGridProps {
    /// 是否可悬停
    #[props(default = true)]
    pub hoverable: bool,

    /// 自定义 CSS 类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,

    /// 子元素
    children: Element,
}

/// Card.Grid 组件
///
/// 卡片网格组件，用于在卡片内部创建网格布局
///
/// # Props
/// - `hoverable`: 是否可悬停，默认为 true
/// - `class`: 自定义 CSS 类名
/// - `style`: 自定义样式
/// - `children`: 子元素
#[component]
pub fn CardGrid(props: CardGridProps) -> Element {
    let mut class_list = vec!["ant-card-grid"];

    if !props.hoverable {
        class_list.push("ant-card-grid-hoverable");
    }

    if let Some(custom_class) = &props.class {
        class_list.push(custom_class);
    }

    let class_str = class_list.join(" ");

    rsx! {
        div {
            class: "{class_str}",
            style: props.style.unwrap_or_default(),
            {props.children}
        }
    }
}

//! Card 卡片组件
//!
//! 通用卡片容器。
//!
//! ## 何时使用
//!
//! 最基础的卡片容器，可承载文字、列表、图片、段落，常用于后台概览页面。

mod styles;

use self::styles::*;
use css_in_rust::css;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

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

impl CardSize {
    /// 获取卡片尺寸对应的CSS样式
    pub fn to_css(&self) -> String {
        let style = match self {
            CardSize::Default => css!("").to_string(),
            CardSize::Small => css!(
                r#".ant-card-small .ant-card-head {
                    min-height: 38px;
                    padding: 0 12px;
                    font-size: 14px;
                }
                .ant-card-small .ant-card-body {
                    padding: 12px;
                }"#
            )
            .to_string(),
        };

        style
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

impl CardType {
    /// 获取卡片类型对应的CSS样式
    pub fn to_css(&self) -> String {
        let style = match self {
            CardType::Default => css! { "" }.to_string(),
            CardType::Inner => css! {
                ".ant-card-type-inner .ant-card-head {
                    padding: 0 24px;
                    background: #fafafa;
                }
                .ant-card-type-inner .ant-card-head-title {
                    padding: 16px 0;
                    font-size: 14px;
                }
                .ant-card-type-inner .ant-card-body {
                    padding: 16px 24px;
                }
                .ant-card-type-inner .ant-card-extra {
                    padding: 17.5px 0;
                }"
            }
            .to_string(),
        };

        style.to_string()
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
        style { {get_card_base_css()} }
        style { {props.size.to_css()} }
        style { {props.card_type.to_css()} }
        div {
            class: format!("{} {}", get_card_wrapper_css(), class_str),
            style: props.style.unwrap_or_default(),

            // 卡片头部
            if props.title.is_some() || props.extra.is_some() {
                div { class: get_card_head_css(),
                    div { class: get_card_head_wrapper_css(),
                        if let Some(title) = &props.title {
                            div { class: get_card_head_title_css(), {title.clone()} }
                        }
                        if let Some(extra) = &props.extra {
                            div { class: get_card_extra_css(), {extra} }
                        }
                    }
                }
            }

            // 卡片主体
            div { class: get_card_body_css(),
                if props.loading {
                    // 加载状态的骨架屏
                    div { class: get_card_loading_content_css(),
                        div { class: get_card_loading_block_css() }
                        div { class: get_card_loading_block_css() }
                        div { class: get_card_loading_block_css() }
                    }
                } else {
                    {props.children}
                }
            }

            // 卡片操作区域
            if let Some(actions) = &props.actions {
                ul { class: get_card_actions_css(),
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
            class: format!("{} {}", get_card_meta_css(), class_str),
            style: props.style.unwrap_or_default(),

            div { class: get_card_meta_detail_css(),
                if let Some(avatar) = &props.avatar {
                    div { class: get_card_meta_avatar_css(), {avatar} }
                }

                div { class: get_card_meta_content_css(),
                    if let Some(title) = &props.title {
                        div { class: get_card_meta_title_css(), {title.clone()} }
                    }
                    if let Some(description) = &props.description {
                        div { class: get_card_meta_description_css(), {description.clone()} }
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
            class: format!("{} {}", get_card_grid_css(), class_str),
            style: props.style.unwrap_or_default(),
            {props.children}
        }
    }
}

// CSS-in-Rust 辅助函数

/// 获取卡片基础CSS样式
fn get_card_base_css() -> String {
    css! {
        ".ant-card {
            position: relative;
            background: #fff;
            border-radius: 8px;
            border: 1px solid #f0f0f0;
            font-size: 14px;
            font-variant: tabular-nums;
            line-height: 1.5715;
            list-style: none;
            font-feature-settings: 'tnum';
        }
        .ant-card-bordered {
            border: 0;
        }
        .ant-card-hoverable {
            cursor: pointer;
            transition: box-shadow 0.3s, border-color 0.3s;
        }
        .ant-card-hoverable:hover {
            border-color: transparent;
            box-shadow: 0 1px 2px -2px rgba(0, 0, 0, 0.16), 0 3px 6px 0 rgba(0, 0, 0, 0.12), 0 5px 12px 4px rgba(0, 0, 0, 0.09);
        }
        .ant-card-loading {
            overflow: hidden;
        }"
    }.to_string()
}

/// 获取卡片包装器CSS样式
fn get_card_wrapper_css() -> String {
    css! {
        "ant-card"
    }
    .to_string()
}

/// 获取卡片头部CSS样式
fn get_card_head_css() -> String {
    css! {
        "ant-card-head"
    }
    .to_string()
}

/// 获取卡片头部包装器CSS样式
fn get_card_head_wrapper_css() -> String {
    css! {
        "ant-card-head-wrapper"
    }
    .to_string()
}

/// 获取卡片头部标题CSS样式
fn get_card_head_title_css() -> String {
    css! {
        "ant-card-head-title"
    }
    .to_string()
}

/// 获取卡片额外操作区域CSS样式
fn get_card_extra_css() -> String {
    css! {
        "ant-card-extra"
    }
    .to_string()
}

/// 获取卡片主体CSS样式
fn get_card_body_css() -> String {
    css! {
        "ant-card-body"
    }
    .to_string()
}

/// 获取卡片加载内容CSS样式
fn get_card_loading_content_css() -> String {
    css! {
        "ant-card-loading-content"
    }
    .to_string()
}

/// 获取卡片加载块CSS样式
fn get_card_loading_block_css() -> String {
    css! {
        "ant-card-loading-block"
    }
    .to_string()
}

/// 获取卡片操作区域CSS样式
fn get_card_actions_css() -> String {
    css! {
        "ant-card-actions"
    }
    .to_string()
}

/// 获取卡片元信息CSS样式
fn get_card_meta_css() -> String {
    css! {
        "ant-card-meta"
    }
    .to_string()
}

/// 获取卡片元信息详情CSS样式
fn get_card_meta_detail_css() -> String {
    css! {
        "ant-card-meta-detail"
    }
    .to_string()
}

/// 获取卡片元信息头像CSS样式
fn get_card_meta_avatar_css() -> String {
    css! {
        "ant-card-meta-avatar"
    }
    .to_string()
}

/// 获取卡片元信息内容CSS样式
fn get_card_meta_content_css() -> String {
    css! {
        "ant-card-meta-content"
    }
    .to_string()
}

/// 获取卡片元信息标题CSS样式
fn get_card_meta_title_css() -> String {
    css! {
        "ant-card-meta-title"
    }
    .to_string()
}

/// 获取卡片元信息描述CSS样式
fn get_card_meta_description_css() -> String {
    css! {
        "ant-card-meta-description"
    }
    .to_string()
}

/// 获取卡片网格CSS样式
fn get_card_grid_css() -> String {
    css! {
        "ant-card-grid"
    }
    .to_string()
}

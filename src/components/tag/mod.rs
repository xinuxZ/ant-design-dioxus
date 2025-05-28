//! # Tag 标签
//!
//! 进行标记和分类的小标签。
//!
//! ## 何时使用
//!
//! - 用于标记事物的属性和维度。
//! - 进行分类。

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

// 引入标签样式
const TAG_STYLE: &str = include_str!("style.css");

/// 标签颜色类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TagColor {
    /// 默认颜色
    Default,
    /// 主色
    Primary,
    /// 成功色
    Success,
    /// 警告色
    Warning,
    /// 错误色
    Error,
    /// 信息色
    Info,
    /// 自定义颜色
    Custom(String),
}

impl Default for TagColor {
    fn default() -> Self {
        Self::Default
    }
}

/// 标签尺寸
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TagSize {
    /// 小尺寸
    Small,
    /// 默认尺寸
    Default,
    /// 大尺寸
    Large,
}

impl Default for TagSize {
    fn default() -> Self {
        Self::Default
    }
}

/// 标签属性
#[derive(Props, Clone, PartialEq)]
pub struct TagProps {
    /// 标签颜色
    #[props(default)]
    pub color: TagColor,

    /// 标签尺寸
    #[props(default)]
    pub size: TagSize,

    /// 是否可关闭
    #[props(default = false)]
    pub closable: bool,

    /// 是否显示边框
    #[props(default = true)]
    pub bordered: bool,

    /// 自定义类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,

    /// 点击事件处理器
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,

    /// 关闭事件处理器
    #[props(default)]
    pub onclose: Option<EventHandler<MouseEvent>>,

    /// 子元素
    children: Element,
}

/// 标签组件
///
/// # 参数
///
/// * `props` - 标签属性
///
/// # 示例
///
/// ```rust
/// use dioxus::prelude::*;
/// use ant_design_dioxus::Tag;
///
/// fn app() -> Element {
///     rsx! {
///         Tag {
///             color: TagColor::Primary,
///             "Primary Tag"
///         }
///     }
/// }
/// ```
#[component]
pub fn Tag(props: TagProps) -> Element {
    let class_name = get_tag_class_name(&props);
    let tag_style = get_tag_style(&props);

    rsx! {
        // 注入标签样式
        style { {TAG_STYLE} }

        span {
            class: "{class_name}",
            style: "{tag_style}",
            onclick: move |evt| {
                if let Some(handler) = &props.onclick {
                    handler.call(evt);
                }
            },

            // 标签内容
            span {
                class: "ant-tag-content",
                {props.children}
            }

            // 关闭按钮
            if props.closable {
                span {
                    class: "ant-tag-close-icon",
                    onclick: move |evt| {
                        evt.stop_propagation();
                        if let Some(handler) = &props.onclose {
                            handler.call(evt);
                        }
                    },
                    "×"
                }
            }
        }
    }
}

/// 获取标签的 CSS 类名
///
/// # 参数
///
/// * `props` - 标签属性
///
/// # 返回值
///
/// 返回标签的完整 CSS 类名字符串
fn get_tag_class_name(props: &TagProps) -> String {
    let mut classes = vec!["ant-tag"];

    // 标签颜色
    match &props.color {
        TagColor::Default => {}
        TagColor::Primary => classes.push("ant-tag-blue"),
        TagColor::Success => classes.push("ant-tag-green"),
        TagColor::Warning => classes.push("ant-tag-orange"),
        TagColor::Error => classes.push("ant-tag-red"),
        TagColor::Info => classes.push("ant-tag-cyan"),
        TagColor::Custom(_) => classes.push("ant-tag-custom"),
    }

    // 标签尺寸
    match props.size {
        TagSize::Small => classes.push("ant-tag-sm"),
        TagSize::Default => {}
        TagSize::Large => classes.push("ant-tag-lg"),
    }

    // 是否有边框
    if !props.bordered {
        classes.push("ant-tag-borderless");
    }

    // 是否可关闭
    if props.closable {
        classes.push("ant-tag-has-color");
    }

    // 添加自定义类名
    let mut class_string = classes.join(" ");
    if let Some(custom_class) = &props.class {
        class_string.push(' ');
        class_string.push_str(custom_class);
    }

    class_string
}

/// 获取标签的内联样式
///
/// # 参数
///
/// * `props` - 标签属性
///
/// # 返回值
///
/// 返回标签的内联样式字符串
fn get_tag_style(props: &TagProps) -> String {
    let mut styles = Vec::new();

    // 自定义颜色
    if let TagColor::Custom(color) = &props.color {
        styles.push(format!(
            "background-color: {}; border-color: {};",
            color, color
        ));
    }

    // 添加自定义样式
    if let Some(custom_style) = &props.style {
        styles.push(custom_style.clone());
    }

    styles.join(" ")
}

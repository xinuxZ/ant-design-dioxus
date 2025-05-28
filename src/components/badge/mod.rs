//! # Badge 徽标数
//!
//! 图标右上角的圆形徽标数字。
//!
//! ## 何时使用
//!
//! 一般出现在通知图标或头像的右上角，用于显示需要处理的消息条数，通过醒目视觉形式吸引用户处理。

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

// 引入徽标样式
const BADGE_STYLE: &str = include_str!("style.css");

/// 徽标状态类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BadgeStatus {
    /// 成功状态
    Success,
    /// 处理中状态
    Processing,
    /// 默认状态
    Default,
    /// 错误状态
    Error,
    /// 警告状态
    Warning,
}

impl Default for BadgeStatus {
    fn default() -> Self {
        Self::Default
    }
}

/// 徽标尺寸
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BadgeSize {
    /// 默认尺寸
    Default,
    /// 小尺寸
    Small,
}

impl Default for BadgeSize {
    fn default() -> Self {
        Self::Default
    }
}

/// Badge 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct BadgeProps {
    /// 展示的数字，大于 overflowCount 时显示为 `${overflowCount}+`，为 0 时隐藏
    #[props(default)]
    pub count: Option<i32>,

    /// 展示封顶的数字值
    #[props(default = 99)]
    pub overflow_count: i32,

    /// 不展示数字，只有一个小红点
    #[props(default = false)]
    pub dot: bool,

    /// 设置状态点的位置偏移
    #[props(default)]
    pub offset: Option<(i32, i32)>,

    /// 设置鼠标放在状态点上时显示的文字
    #[props(default)]
    pub title: Option<String>,

    /// 当数值为 0 时，是否展示 Badge
    #[props(default = false)]
    pub show_zero: bool,

    /// 设置状态点的状态
    #[props(default)]
    pub status: Option<BadgeStatus>,

    /// 在设置了 status 的前提下有效，设置状态点的文本
    #[props(default)]
    pub text: Option<String>,

    /// 自定义小圆点的颜色
    #[props(default)]
    pub color: Option<String>,

    /// 徽标尺寸
    #[props(default)]
    pub size: BadgeSize,

    /// 自定义类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,

    /// 子元素
    children: Element,
}

/// Badge 组件
#[component]
pub fn Badge(props: BadgeProps) -> Element {
    let class_name = get_badge_class_name(&props);
    let badge_style = get_badge_style(&props);

    // 判断是否显示徽标
    let should_show_badge = should_show_badge(&props);

    rsx! {
        // 注入徽标样式
        style { {BADGE_STYLE} }

        if props.children.is_ok() {
            // 有子元素时，作为包装器
            span {
                class: class_name.clone(),
                style: badge_style.clone(),

                // 子元素
                {props.children.clone()}

                // 徽标内容
                if should_show_badge {
                    {render_badge_content(&props)}
                }
            }
        } else {
            // 无子元素时，仅显示状态点
            if let Some(status) = &props.status {
                span {
                    class: "ant-badge ant-badge-status",
                    style: badge_style.clone(),

                    span {
                        class: "ant-badge-status-dot ant-badge-status-{get_status_class(status)}",
                        style: get_status_dot_style(&props),
                    }

                    if let Some(text) = &props.text {
                        span {
                            class: "ant-badge-status-text",
                            {text.clone()}
                        }
                    }
                }
            }
        }
    }
}

/// 获取徽标类名
fn get_badge_class_name(props: &BadgeProps) -> String {
    let mut classes = vec!["ant-badge"];

    if props.children.is_ok() {
        classes.push("ant-badge-not-a-wrapper");
    }

    if props.dot {
        classes.push("ant-badge-dot");
    }

    if matches!(props.size, BadgeSize::Small) {
        classes.push("ant-badge-small");
    }

    if let Some(class) = &props.class {
        classes.push(class);
    }

    classes.join(" ")
}

/// 获取徽标样式
fn get_badge_style(props: &BadgeProps) -> String {
    let mut styles = Vec::new();

    // 添加自定义样式
    if let Some(custom_style) = &props.style {
        styles.push(custom_style.clone());
    }

    styles.join("; ")
}

/// 判断是否应该显示徽标
fn should_show_badge(props: &BadgeProps) -> bool {
    if props.dot {
        return true;
    }

    if let Some(count) = props.count {
        return count > 0 || (count == 0 && props.show_zero);
    }

    false
}

/// 渲染徽标内容
fn render_badge_content(props: &BadgeProps) -> Element {
    if props.dot {
        // 小红点模式
        rsx! {
            sup {
                class: "ant-badge-dot",
                style: get_badge_sup_style(props),
                title: props.title.as_deref().unwrap_or_default(),
            }
        }
    } else if let Some(count) = props.count {
        if count > 0 || (count == 0 && props.show_zero) {
            // 数字模式
            let display_count = if count > props.overflow_count {
                format!("{}+", props.overflow_count)
            } else {
                count.to_string()
            };

            rsx! {
                sup {
                    class: "ant-badge-count",
                    style: get_badge_sup_style(props),
                    title: props.title.as_deref().unwrap_or(&display_count),

                    span {
                       class: "ant-badge-count-content",
                        {display_count.clone()}
                    }
                }
            }
        } else {
            rsx! { span {} }
        }
    } else {
        rsx! { span {} }
    }
}

/// 获取徽标上标样式
fn get_badge_sup_style(props: &BadgeProps) -> String {
    let mut styles = Vec::new();

    // 自定义颜色
    if let Some(color) = &props.color {
        styles.push(format!("background-color: {}", color));
    }

    // 位置偏移
    if let Some((x, y)) = props.offset {
        styles.push(format!("transform: translate({}px, {}px)", x, y));
    }

    styles.join("; ")
}

/// 获取状态点样式
fn get_status_dot_style(props: &BadgeProps) -> String {
    let mut styles = Vec::new();

    // 自定义颜色
    if let Some(color) = &props.color {
        styles.push(format!("background-color: {}", color));
    }

    styles.join("; ")
}

/// 获取状态类名
fn get_status_class(status: &BadgeStatus) -> &'static str {
    match status {
        BadgeStatus::Success => "success",
        BadgeStatus::Processing => "processing",
        BadgeStatus::Default => "default",
        BadgeStatus::Error => "error",
        BadgeStatus::Warning => "warning",
    }
}

//! Result 结果
//!
//! 用于反馈一系列操作任务的处理结果。
//!
//! ## 何时使用
//!
//! 当有重要操作需告知用户处理结果，且反馈内容较为复杂时使用。
//!
//! ## 代码演示
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::{Result, ResultStatus, Button};
//!
//! #[component]
//! fn App() -> Element {
//!     rsx! {
//!         Result {
//!             status: ResultStatus::Success,
//!             title: "成功提交",
//!             sub_title: "您的申请已经成功提交，我们会在1-2个工作日内处理。",
//!             extra: rsx! {
//!                 Button { "返回首页" }
//!                 Button { button_type: ButtonType::Primary, "查看详情" }
//!             }
//!         }
//!     }
//! }
//! ```

use dioxus::prelude::*;

const RESULT_STYLES: &str = include_str!("./style.css");

/// Result 状态类型
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ResultStatus {
    /// 成功
    Success,
    /// 错误
    Error,
    /// 信息
    Info,
    /// 警告
    Warning,
    /// 404
    NotFound,
    /// 403
    Forbidden,
    /// 500
    ServerError,
}

impl Default for ResultStatus {
    fn default() -> Self {
        Self::Info
    }
}

impl ResultStatus {
    /// 获取状态对应的CSS类名
    pub fn to_class(&self) -> &'static str {
        match self {
            ResultStatus::Success => "ant-result-success",
            ResultStatus::Error => "ant-result-error",
            ResultStatus::Info => "ant-result-info",
            ResultStatus::Warning => "ant-result-warning",
            ResultStatus::NotFound => "ant-result-404",
            ResultStatus::Forbidden => "ant-result-403",
            ResultStatus::ServerError => "ant-result-500",
        }
    }

    /// 获取默认图标
    pub fn default_icon(&self) -> &'static str {
        match self {
            ResultStatus::Success => "✓",
            ResultStatus::Error => "✕",
            ResultStatus::Info => "ℹ",
            ResultStatus::Warning => "⚠",
            ResultStatus::NotFound => "404",
            ResultStatus::Forbidden => "403",
            ResultStatus::ServerError => "500",
        }
    }

    /// 获取默认标题
    pub fn default_title(&self) -> &'static str {
        match self {
            ResultStatus::Success => "成功",
            ResultStatus::Error => "错误",
            ResultStatus::Info => "信息",
            ResultStatus::Warning => "警告",
            ResultStatus::NotFound => "404",
            ResultStatus::Forbidden => "403",
            ResultStatus::ServerError => "500",
        }
    }

    /// 获取默认副标题
    pub fn default_sub_title(&self) -> &'static str {
        match self {
            ResultStatus::Success => "操作成功完成",
            ResultStatus::Error => "操作失败，请重试",
            ResultStatus::Info => "这是一条信息",
            ResultStatus::Warning => "请注意相关事项",
            ResultStatus::NotFound => "抱歉，您访问的页面不存在",
            ResultStatus::Forbidden => "抱歉，您无权访问该页面",
            ResultStatus::ServerError => "抱歉，服务器出错了",
        }
    }
}

/// Result 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct ResultProps {
    /// 结果的状态
    #[props(default)]
    pub status: ResultStatus,

    /// 标题
    #[props(default)]
    pub title: Option<String>,

    /// 副标题
    #[props(default)]
    pub sub_title: Option<String>,

    /// 自定义图标
    #[props(default)]
    pub icon: Option<String>,

    /// 操作区
    #[props(default)]
    pub extra: Option<Element>,

    /// 自定义类名
    #[props(default)]
    pub class_name: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,

    /// 子元素
    #[props(default)]
    pub children: Option<Element>,
}

/// Result 结果组件
///
/// 用于反馈一系列操作任务的处理结果
#[component]
pub fn Result(props: ResultProps) -> Element {
    let result_class = {
        let mut classes = vec!["ant-result"];
        classes.push(props.status.to_class());

        if let Some(class_name) = &props.class_name {
            classes.push(class_name);
        }

        classes.join(" ")
    };

    let icon = props
        .icon
        .as_deref()
        .unwrap_or_else(|| props.status.default_icon());
    let title = props
        .title
        .as_deref()
        .unwrap_or_else(|| props.status.default_title());
    let sub_title = props
        .sub_title
        .as_deref()
        .unwrap_or_else(|| props.status.default_sub_title());

    rsx! {
        style { {RESULT_STYLES} }
        div {
            class: "{result_class}",
            style: props.style.as_deref().unwrap_or(""),

            // 图标区域
            div {
                class: "ant-result-icon",
                span {
                    class: "ant-result-icon-content",
                    "{icon}"
                }
            }

            // 标题区域
            div {
                class: "ant-result-title",
                "{title}"
            }

            // 副标题区域
            div {
                class: "ant-result-subtitle",
                "{sub_title}"
            }

            // 自定义内容
            if let Some(children) = &props.children {
                div {
                    class: "ant-result-content",
                    {children}
                }
            }

            // 操作区域
            if let Some(extra) = &props.extra {
                div {
                    class: "ant-result-extra",
                    {extra}
                }
            }
        }
    }
}

/// 成功结果组件
#[component]
pub fn SuccessResult(
    title: Option<String>,
    sub_title: Option<String>,
    extra: Option<Element>,
) -> Element {
    rsx! {
        Result {
            status: ResultStatus::Success,
            title: title,
            sub_title: sub_title,
            extra: extra,
        }
    }
}

/// 错误结果组件
#[component]
pub fn ErrorResult(
    title: Option<String>,
    sub_title: Option<String>,
    extra: Option<Element>,
) -> Element {
    rsx! {
        Result {
            status: ResultStatus::Error,
            title: title,
            sub_title: sub_title,
            extra: extra,
        }
    }
}

/// 404 结果组件
#[component]
pub fn NotFoundResult(
    title: Option<String>,
    sub_title: Option<String>,
    extra: Option<Element>,
) -> Element {
    rsx! {
        Result {
            status: ResultStatus::NotFound,
            title: title,
            sub_title: sub_title,
            extra: extra,
        }
    }
}

/// 403 结果组件
#[component]
pub fn ForbiddenResult(
    title: Option<String>,
    sub_title: Option<String>,
    extra: Option<Element>,
) -> Element {
    rsx! {
        Result {
            status: ResultStatus::Forbidden,
            title: title,
            sub_title: sub_title,
            extra: extra,
        }
    }
}

/// 500 结果组件
#[component]
pub fn ServerErrorResult(
    title: Option<String>,
    sub_title: Option<String>,
    extra: Option<Element>,
) -> Element {
    rsx! {
        Result {
            status: ResultStatus::ServerError,
            title: title,
            sub_title: sub_title,
            extra: extra,
        }
    }
}

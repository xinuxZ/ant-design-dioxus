//! Alert 警告提示组件
//!
//! 警告提示，展现需要关注的信息。
//!
//! ## 何时使用
//!
//! - 当某个页面需要向用户显示警告的信息时。
//! - 非浮层的静态展现形式，始终展现，不会自动消失，用户可以点击关闭。
//!
//! ## 代码演示
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::{Alert, AlertType};
//!
//! #[component]
//! fn App() -> Element {
//!     rsx! {
//!         Alert {
//!             message: "成功提示的文案",
//!             alert_type: AlertType::Success,
//!             closable: true
//!         }
//!     }
//! }
//! ```

use dioxus::prelude::*;

/// Alert 组件类型
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlertType {
    /// 成功提示
    Success,
    /// 消息通知
    Info,
    /// 警告提示
    Warning,
    /// 错误提示
    Error,
}

impl Default for AlertType {
    fn default() -> Self {
        Self::Info
    }
}

impl AlertType {
    /// 获取类型对应的CSS类名
    pub fn to_class(&self) -> &'static str {
        match self {
            AlertType::Success => "ant-alert-success",
            AlertType::Info => "ant-alert-info",
            AlertType::Warning => "ant-alert-warning",
            AlertType::Error => "ant-alert-error",
        }
    }

    /// 获取默认图标
    pub fn default_icon(&self) -> &'static str {
        match self {
            AlertType::Success => "✓",
            AlertType::Info => "ℹ",
            AlertType::Warning => "⚠",
            AlertType::Error => "✕",
        }
    }
}

/// Alert 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct AlertProps {
    /// 指定警告提示的样式
    #[props(default)]
    pub alert_type: AlertType,

    /// 警告提示内容
    pub message: String,

    /// 警告提示的辅助性文字介绍
    #[props(default)]
    pub description: Option<String>,

    /// 是否显示辅助图标
    #[props(default = true)]
    pub show_icon: bool,

    /// 自定义图标，show_icon 为 true 时有效
    #[props(default)]
    pub icon: Option<Element>,

    /// 默认不显示关闭按钮
    #[props(default = false)]
    pub closable: bool,

    /// 自定义关闭按钮
    #[props(default)]
    pub close_text: Option<String>,

    /// 关闭时触发的回调函数
    #[props(default)]
    pub on_close: Option<EventHandler<()>>,

    /// 是否用作顶部公告
    #[props(default = false)]
    pub banner: bool,

    /// 自定义样式类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,

    /// 额外的操作
    #[props(default)]
    pub action: Option<Element>,
}

/// Alert 警告提示组件
///
/// 用于页面中展示重要的提示信息
#[component]
pub fn Alert(props: AlertProps) -> Element {
    let mut visible = use_signal(|| true);

    let handle_close = move |_| {
        visible.set(false);
        if let Some(on_close) = &props.on_close {
            on_close.call(());
        }
    };

    if !visible() {
        return rsx! { div {} };
    }

    let alert_class = {
        let mut classes = vec!["ant-alert"];

        classes.push(props.alert_type.to_class());

        if props.show_icon {
            classes.push("ant-alert-with-icon");
        }

        if props.description.is_some() {
            classes.push("ant-alert-with-description");
        }

        if props.banner {
            classes.push("ant-alert-banner");
        }

        if props.closable {
            classes.push("ant-alert-closable");
        }

        if let Some(ref class) = props.class {
            classes.push(class);
        }

        classes.join(" ")
    };

    rsx! {
        div {
            class: alert_class,
            style: props.style,
            role: "alert",

            if props.show_icon {
                div {
                    class: "ant-alert-icon",
                    if let Some(icon) = props.icon {
                        {icon}
                    } else {
                        span {
                            class: "ant-alert-icon-default",
                            {props.alert_type.default_icon()}
                        }
                    }
                }
            }

            div {
                class: "ant-alert-content",

                div {
                    class: "ant-alert-message",
                    {props.message.clone()}
                }

                if let Some(ref description) = props.description {
                    div {
                        class: "ant-alert-description",
                        {description.clone()}
                    }
                }
            }

            if let Some(action) = props.action {
                div {
                    class: "ant-alert-action",
                    {action}
                }
            }

            if props.closable {
                button {
                    class: "ant-alert-close-icon",
                    r#type: "button",
                    onclick: handle_close,
                    "aria-label": "Close",

                    if let Some(ref close_text) = props.close_text {
                        span {
                            class: "ant-alert-close-text",
                            {close_text.clone()}
                        }
                    } else {
                        span {
                            class: "ant-alert-close-icon-default",
                            "×"
                        }
                    }
                }
            }
        }
    }
}

// 组件已通过#[component]宏自动导出
// 无需重新导出

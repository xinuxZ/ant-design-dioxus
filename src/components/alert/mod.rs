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

mod styles;
pub use styles::AlertType;
use styles::{use_alert_style, AlertStyleGenerator};

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

    // 确保样式已注入
    use_alert_style();

    let handle_close = move |_| {
        visible.set(false);
        if let Some(on_close) = &props.on_close {
            on_close.call(());
        }
    };

    if !visible() {
        return rsx! { div {} };
    }

    // 使用AlertStyleGenerator生成样式
    let style_gen = AlertStyleGenerator::new()
        .with_type(props.alert_type)
        .with_icon(props.show_icon)
        .with_description(props.description.is_some())
        .with_closable(props.closable)
        .with_banner(props.banner);

    // 获取生成的样式字符串，其中已包含类名和基础样式
    let mut alert_style = style_gen.generate();

    // 添加自定义样式
    if let Some(ref style) = props.style {
        alert_style.push_str(" ");
        alert_style.push_str(style);
    }

    rsx! {
        div {
            // 不再单独设置class，因为样式字符串中已包含类名
            style: alert_style,
            role: "alert",

            if props.show_icon {
                div {
                    class: "ant-alert-icon",
                    if let Some(icon) = props.icon {
                        {icon}
                    } else {
                        span {
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

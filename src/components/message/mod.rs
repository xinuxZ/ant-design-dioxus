//! Message 全局提示
//!
//! 全局展示操作反馈信息。
//!
//! ## 何时使用
//!
//! - 可提供成功、警告和错误等反馈信息。
//! - 顶部居中显示并自动消失，是一种不打断用户操作的轻量级提示方式。
//!
//! ## 代码演示
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::{Message, MessageType};
//!
//! #[component]
//! fn App() -> Element {
//!     rsx! {
//!         Message {
//!             message_type: MessageType::Success,
//!             content: "这是一条成功提示",
//!             duration: 3.0
//!         }
//!     }
//! }
//! ```

use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;

const MESSAGE_STYLES: &str = include_str!("./style.css");

/// Message 组件类型
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MessageType {
    /// 成功消息
    Success,
    /// 错误消息
    Error,
    /// 警告消息
    Warning,
    /// 信息消息
    Info,
    /// 加载消息
    Loading,
}

impl Default for MessageType {
    fn default() -> Self {
        Self::Info
    }
}

impl MessageType {
    /// 获取消息类型对应的CSS类名
    pub fn to_class(&self) -> &'static str {
        match self {
            MessageType::Success => "ant-message-success",
            MessageType::Error => "ant-message-error",
            MessageType::Warning => "ant-message-warning",
            MessageType::Info => "ant-message-info",
            MessageType::Loading => "ant-message-loading",
        }
    }

    /// 获取消息类型对应的图标
    pub fn to_icon(&self) -> &'static str {
        match self {
            MessageType::Success => "✓",
            MessageType::Error => "✕",
            MessageType::Warning => "⚠",
            MessageType::Info => "ℹ",
            MessageType::Loading => "⟳",
        }
    }

    /// 获取默认持续时间（秒）
    pub fn default_duration(&self) -> f64 {
        match self {
            MessageType::Loading => 0.0, // 加载消息不自动消失
            MessageType::Error => 4.5,   // 错误消息显示更长时间
            _ => 3.0,                    // 其他消息默认3秒
        }
    }
}

/// Message 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct MessageProps {
    /// 消息类型
    #[props(default)]
    pub message_type: MessageType,

    /// 消息内容
    pub content: String,

    /// 自动关闭的延时，单位秒。设为 0 时不自动关闭
    #[props(default)]
    pub duration: Option<f64>,

    /// 关闭时触发的回调函数
    #[props(default)]
    pub on_close: Option<EventHandler<()>>,

    /// 自定义图标
    #[props(default)]
    pub icon: Option<Element>,

    /// 消息的唯一标识，用于更新消息内容
    #[props(default)]
    pub r#key: Option<String>,

    /// 自定义样式类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,

    /// 是否显示消息
    #[props(default = true)]
    pub visible: bool,
}

/// Message 全局提示组件
///
/// 全局展示操作反馈信息，顶部居中显示并自动消失
#[component]
pub fn Message(props: MessageProps) -> Element {
    let mut visible = use_signal(|| props.visible);

    // 处理自动关闭逻辑
    use_effect(move || {
        if props.visible {
            visible.set(true);

            let duration = props
                .duration
                .unwrap_or_else(|| props.message_type.default_duration());

            if duration > 0.0 {
                let on_close = props.on_close.clone();
                spawn(async move {
                    TimeoutFuture::new((duration * 1000.0) as u32).await;
                    visible.set(false);

                    // 延迟一点时间等待动画完成后触发回调
                    TimeoutFuture::new(300).await;
                    if let Some(callback) = on_close {
                        callback.call(());
                    }
                });
            }
        } else {
            visible.set(false);
        }
    });

    let message_class = {
        let mut classes = vec!["ant-message-notice"];
        classes.push(props.message_type.to_class());

        if let Some(class) = &props.class {
            classes.push(class);
        }

        classes.join(" ")
    };

    let container_class = {
        let mut classes = vec!["ant-message"];
        if !visible() {
            classes.push("ant-message-hidden");
        }
        classes.join(" ")
    };

    rsx! {
        style { {MESSAGE_STYLES} }
        if visible() {
            div {
                class: "{container_class}",
                div {
                    class: "ant-message-notice-wrapper",
                    div {
                        class: "{message_class}",
                        style: props.style.as_deref().unwrap_or(""),
                        div {
                            class: "ant-message-notice-content",
                            span {
                                class: "ant-message-custom-content",
                                span {
                                    class: "ant-message-icon",
                                    if let Some(icon) = &props.icon {
                                        {icon}
                                    } else {
                                        span {
                                            class: match props.message_type {
                                                MessageType::Loading => "ant-message-icon-loading",
                                                _ => "ant-message-icon-default",
                                            },
                                            "{props.message_type.to_icon()}"
                                        }
                                    }
                                }
                                span {
                                    class: "ant-message-content-text",
                                    "{props.content}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Message 容器组件
///
/// 用于管理多个消息的显示
#[derive(Props, Clone, PartialEq)]
pub struct MessageContainerProps {
    /// 消息列表
    pub messages: Vec<MessageProps>,

    /// 距离顶部的距离
    #[props(default = 24)]
    pub top: i32,

    /// 最大显示数量
    #[props(default)]
    pub max_count: Option<usize>,
}

/// Message 容器组件
#[component]
pub fn MessageContainer(props: MessageContainerProps) -> Element {
    let container_style = format!("top: {}px;", props.top);

    let messages_to_show = if let Some(max_count) = props.max_count {
        if props.messages.len() > max_count {
            &props.messages[props.messages.len() - max_count..]
        } else {
            &props.messages
        }
    } else {
        &props.messages
    };

    rsx! {
        div {
            class: "ant-message-container",
            style: "{container_style}",
            for (_index, message) in messages_to_show.iter().enumerate() {
                Message {
                    key: message.key.clone().unwrap_or_else(|| format!("message-{}", index)),
                    message_type: message.message_type,
                    content: message.content.clone(),
                    duration: message.duration,
                    on_close: message.on_close.clone(),
                    icon: message.icon.clone(),
                    class: message.class.clone(),
                    style: message.style.clone(),
                    visible: message.visible,
                }
            }
        }
    }
}

/// 全局消息配置
#[derive(Debug, Clone)]
pub struct MessageConfig {
    /// 距离顶部的距离
    pub top: i32,
    /// 默认自动关闭延时，单位秒
    pub duration: f64,
    /// 最大显示数量
    pub max_count: Option<usize>,
    /// 获取弹出层的容器
    pub get_container: Option<fn() -> String>,
}

impl Default for MessageConfig {
    fn default() -> Self {
        Self {
            top: 24,
            duration: 3.0,
            max_count: None,
            get_container: None,
        }
    }
}

/// 全局消息管理器
#[derive(Debug, Clone)]
pub struct MessageManager {
    config: MessageConfig,
}

impl MessageManager {
    /// 创建新的消息管理器
    pub fn new() -> Self {
        Self {
            config: MessageConfig::default(),
        }
    }

    /// 配置全局设置
    pub fn config(&mut self, config: MessageConfig) {
        self.config = config;
    }

    /// 显示成功消息
    pub fn success(&self, content: &str, duration: Option<f64>) -> MessageProps {
        MessageProps {
            message_type: MessageType::Success,
            content: content.to_string(),
            duration: duration.or(Some(self.config.duration)),
            on_close: None,
            icon: None,
            key: None,
            class: None,
            style: None,
            visible: true,
        }
    }

    /// 显示错误消息
    pub fn error(&self, content: &str, duration: Option<f64>) -> MessageProps {
        MessageProps {
            message_type: MessageType::Error,
            content: content.to_string(),
            duration: duration.or(Some(self.config.duration)),
            on_close: None,
            icon: None,
            key: None,
            class: None,
            style: None,
            visible: true,
        }
    }

    /// 显示警告消息
    pub fn warning(&self, content: &str, duration: Option<f64>) -> MessageProps {
        MessageProps {
            message_type: MessageType::Warning,
            content: content.to_string(),
            duration: duration.or(Some(self.config.duration)),
            on_close: None,
            icon: None,
            key: None,
            class: None,
            style: None,
            visible: true,
        }
    }

    /// 显示信息消息
    pub fn info(&self, content: &str, duration: Option<f64>) -> MessageProps {
        MessageProps {
            message_type: MessageType::Info,
            content: content.to_string(),
            duration: duration.or(Some(self.config.duration)),
            on_close: None,
            icon: None,
            key: None,
            class: None,
            style: None,
            visible: true,
        }
    }

    /// 显示加载消息
    pub fn loading(&self, content: &str, duration: Option<f64>) -> MessageProps {
        MessageProps {
            message_type: MessageType::Loading,
            content: content.to_string(),
            duration: duration.or(Some(0.0)), // 加载消息默认不自动关闭
            on_close: None,
            icon: None,
            key: None,
            class: None,
            style: None,
            visible: true,
        }
    }
}

impl Default for MessageManager {
    fn default() -> Self {
        Self::new()
    }
}

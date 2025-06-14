//! Popconfirm 气泡确认框组件
//!
//! 点击元素，弹出气泡式的确认框。
//!
//! ## 何时使用
//!
//! 目标元素的操作需要用户进一步的确认时，在目标元素附近弹出浮层提示，询问用户。
//! 和 confirm 弹出的全屏居中模态对话框相比，交互形式更轻量。

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

const POPCONFIRM_STYLES: &str = include_str!("./style.css");

/// Popconfirm 气泡确认框组件属性
#[derive(Props, Clone, PartialEq)]
pub struct PopconfirmProps {
    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 确认框标题
    pub title: String,

    /// 确认框描述
    #[props(default)]
    pub description: Option<String>,

    /// 气泡框位置
    #[props(default = PopconfirmPlacement::Top)]
    pub placement: PopconfirmPlacement,

    /// 触发行为
    #[props(default = PopconfirmTrigger::Click)]
    pub trigger: PopconfirmTrigger,

    /// 是否显示箭头
    #[props(default = true)]
    pub arrow: bool,

    /// 是否默认显示
    #[props(default = false)]
    pub open: bool,

    /// 显示隐藏的回调
    #[props(default)]
    pub on_open_change: Option<EventHandler<bool>>,

    /// 确认按钮文字
    #[props(default = "确定".to_string())]
    pub ok_text: String,

    /// 取消按钮文字
    #[props(default = "取消".to_string())]
    pub cancel_text: String,

    /// 自定义图标
    #[props(default)]
    pub icon: Option<Element>,

    /// 是否显示取消按钮
    #[props(default = true)]
    pub show_cancel: bool,

    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,

    /// 点击确认回调
    #[props(default)]
    pub on_confirm: Option<EventHandler<()>>,

    /// 点击取消回调
    #[props(default)]
    pub on_cancel: Option<EventHandler<()>>,

    /// 卡片类名
    #[props(default)]
    pub overlay_class_name: Option<String>,

    /// 子元素
    pub children: Element,
}

/// 气泡框位置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PopconfirmPlacement {
    Top,
    Left,
    Right,
    Bottom,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    LeftTop,
    LeftBottom,
    RightTop,
    RightBottom,
}

impl Default for PopconfirmPlacement {
    fn default() -> Self {
        Self::Top
    }
}

impl PopconfirmPlacement {
    fn to_class(&self) -> &'static str {
        match self {
            PopconfirmPlacement::Top => "ant-popover-placement-top",
            PopconfirmPlacement::Left => "ant-popover-placement-left",
            PopconfirmPlacement::Right => "ant-popover-placement-right",
            PopconfirmPlacement::Bottom => "ant-popover-placement-bottom",
            PopconfirmPlacement::TopLeft => "ant-popover-placement-topLeft",
            PopconfirmPlacement::TopRight => "ant-popover-placement-topRight",
            PopconfirmPlacement::BottomLeft => "ant-popover-placement-bottomLeft",
            PopconfirmPlacement::BottomRight => "ant-popover-placement-bottomRight",
            PopconfirmPlacement::LeftTop => "ant-popover-placement-leftTop",
            PopconfirmPlacement::LeftBottom => "ant-popover-placement-leftBottom",
            PopconfirmPlacement::RightTop => "ant-popover-placement-rightTop",
            PopconfirmPlacement::RightBottom => "ant-popover-placement-rightBottom",
        }
    }
}

/// 触发行为
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PopconfirmTrigger {
    Hover,
    Focus,
    Click,
    ContextMenu,
}

impl Default for PopconfirmTrigger {
    fn default() -> Self {
        Self::Click
    }
}

/// Popconfirm 气泡确认框组件
#[component]
pub fn Popconfirm(props: PopconfirmProps) -> Element {
    // 使用 use_signal 管理可见性状态
    let mut visible = use_signal(|| props.open);

    // 复制需要在闭包中使用的 props
    let title = props.title.clone();
    let description = props.description.clone();
    let ok_text = props.ok_text.clone();
    let cancel_text = props.cancel_text.clone();
    let show_cancel = props.show_cancel;
    let disabled = props.disabled;
    let placement = props.placement.clone();
    let arrow = props.arrow;
    let class = props.class.clone();
    let style = props.style.clone();
    let overlay_class_name = props.overlay_class_name.as_ref().map(|s| s.clone());

    // 处理点击触发器
    let handle_trigger_click = move |_| {
        if !disabled {
            let new_visible = !visible();
            visible.set(new_visible);
            if let Some(on_open_change) = &props.on_open_change {
                on_open_change.call(new_visible);
            }
        }
    };

    // 处理确认按钮点击
    let handle_confirm = move |_| {
        if let Some(on_confirm) = &props.on_confirm {
            on_confirm.call(());
        }
        visible.set(false);
        if let Some(on_open_change) = &props.on_open_change {
            on_open_change.call(false);
        }
    };

    // 处理取消按钮点击
    let handle_cancel = move |_| {
        if let Some(on_cancel) = &props.on_cancel {
            on_cancel.call(());
        }
        visible.set(false);
        if let Some(on_open_change) = &props.on_open_change {
            on_open_change.call(false);
        }
    };

    // 计算类名
    let popover_class = format!(
        "ant-popover ant-popconfirm {} {}",
        placement.to_class(),
        if visible() { "ant-popover-open" } else { "" }
    );

    let overlay_class = format!("ant-popover-content",);

    rsx! {
        style { {POPCONFIRM_STYLES} }
        div {
            class: "ant-popconfirm-wrapper",
            style: "{style}",

            // 触发器元素
            div {
                class: "ant-popconfirm-trigger",
                onclick: handle_trigger_click,
                {props.children}
            }

            // 气泡确认框
            if visible() {
                div {
                    class: "{popover_class} {class}",
                    div {
                        class: "{overlay_class} {overlay_class_name.clone().unwrap_or_default()}",

                        // 箭头
                        if arrow {
                            div {
                                class: "ant-popover-arrow",
                                div {
                                    class: "ant-popover-arrow-content"
                                }
                            }
                        }

                        // 内容区域
                        div {
                            class: "ant-popover-inner",
                            role: "tooltip",

                            div {
                                class: "ant-popconfirm-inner-content",

                                div {
                                    class: "ant-popconfirm-message",

                                    // 图标
                                    if let Some(icon) = &props.icon {
                                        div {
                                            class: "ant-popconfirm-message-icon",
                                            {icon}
                                        }
                                    } else {
                                        div {
                                            class: "ant-popconfirm-message-icon",
                                            span {
                                                class: "anticon anticon-exclamation-circle",
                                                "⚠️"
                                            }
                                        }
                                    }

                                    // 标题和描述
                                    div {
                                        class: "ant-popconfirm-message-title",
                                        "{title}"

                                        if let Some(desc) = &description {
                                            div {
                                                class: "ant-popconfirm-description",
                                                "{desc}"
                                            }
                                        }
                                    }
                                }

                                // 按钮组
                                div {
                                    class: "ant-popconfirm-buttons",

                                    if show_cancel {
                                        button {
                                            class: "ant-btn ant-btn-sm",
                                            onclick: handle_cancel,
                                            "{cancel_text}"
                                        }
                                    }

                                    button {
                                        class: "ant-btn ant-btn-primary ant-btn-sm",
                                        onclick: handle_confirm,
                                        "{ok_text}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

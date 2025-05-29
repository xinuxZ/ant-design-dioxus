//! Popconfirm 气泡确认框组件
//!
//! 点击元素，弹出气泡式的确认框。
//!
//! ## 何时使用
//!
//! 目标元素的操作需要用户进一步的确认时，在目标元素附近弹出浮层提示，询问用户。
//! 和 confirm 弹出的全屏居中模态对话框相比，交互形式更轻量。

use crate::components::button::{Button, ButtonSize, ButtonType};
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

    /// 确认按钮类型
    #[props(default = ButtonType::Primary)]
    pub ok_type: ButtonType,

    /// 确认按钮属性
    #[props(default)]
    pub ok_button_props: Option<String>,

    /// 取消按钮属性
    #[props(default)]
    pub cancel_button_props: Option<String>,

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
    pub on_confirm: Option<EventHandler<MouseEvent>>,

    /// 点击取消回调
    #[props(default)]
    pub on_cancel: Option<EventHandler<MouseEvent>>,

    /// 点击浮层回调
    #[props(default)]
    pub on_popup_click: Option<EventHandler<MouseEvent>>,

    /// 卡片类名
    #[props(default)]
    pub overlay_class_name: Option<String>,

    /// 卡片样式
    #[props(default)]
    pub overlay_style: Option<String>,

    /// 浮层渲染父节点，默认渲染到 body 上
    #[props(default)]
    pub get_popup_container: Option<String>,

    /// 鼠标移入后延时多少才显示，单位：秒
    #[props(default = 0.1)]
    pub mouse_enter_delay: f64,

    /// 鼠标移出后延时多少才隐藏，单位：秒
    #[props(default = 0.1)]
    pub mouse_leave_delay: f64,

    /// 子元素
    pub children: Element,
}

/// 气泡框位置
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PopconfirmPlacement {
    Top,
    TopLeft,
    TopRight,
    Bottom,
    BottomLeft,
    BottomRight,
    Left,
    LeftTop,
    LeftBottom,
    Right,
    RightTop,
    RightBottom,
}

impl Default for PopconfirmPlacement {
    fn default() -> Self {
        Self::Top
    }
}

/// 触发行为
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PopconfirmTrigger {
    Hover,
    Click,
    Focus,
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
    let mut visible = use_signal(|| props.open);
    let mut mouse_enter_timer = use_signal(|| None::<gloo_timers::callback::Timeout>);
    let mut mouse_leave_timer = use_signal(|| None::<gloo_timers::callback::Timeout>);

    // 显示确认框
    let mut show_popconfirm = move || {
        if props.disabled {
            return;
        }

        // 清除隐藏定时器
        if let Some(timer) = mouse_leave_timer.take() {
            timer.cancel();
        }

        if !visible() {
            let delay = (props.mouse_enter_delay * 1000.0) as u32;
            if delay > 0 {
                let visible_clone = visible.clone();
                let on_open_change_clone = props.on_open_change.clone();
                let timer = gloo_timers::callback::Timeout::new(delay, move || {
                    visible_clone.set(true);
                    if let Some(on_open_change) = &on_open_change_clone {
                        on_open_change.call(true);
                    }
                });
                mouse_enter_timer.set(Some(timer));
            } else {
                visible.set(true);
                if let Some(on_open_change) = &props.on_open_change {
                    on_open_change.call(true);
                }
            }
        }
    };

    // 隐藏确认框
    let hide_popconfirm = move || {
        // 清除显示定时器
        if let Some(timer) = mouse_enter_timer.take() {
            timer.cancel();
        }

        if visible() {
            let delay = (props.mouse_leave_delay * 1000.0) as u32;
            if delay > 0 {
                let visible_clone = visible.clone();
                let on_open_change_clone = props.on_open_change.clone();
                let timer = gloo_timers::callback::Timeout::new(delay, move || {
                    visible_clone.set(false);
                    if let Some(on_open_change) = &on_open_change_clone {
                        on_open_change.call(false);
                    }
                });
                mouse_leave_timer.set(Some(timer));
            } else {
                visible.set(false);
                if let Some(on_open_change) = &props.on_open_change {
                    on_open_change.call(false);
                }
            }
        }
    };

    // 切换显示状态
    let toggle_popconfirm = move || {
        if props.disabled {
            return;
        }

        let new_visible = !visible();
        visible.set(new_visible);
        if let Some(on_open_change) = &props.on_open_change {
            on_open_change.call(new_visible);
        }
    };

    // 确认按钮点击事件
    let handle_confirm = move |event: MouseEvent| {
        if let Some(on_confirm) = &props.on_confirm {
            on_confirm.call(event);
        }
        visible.set(false);
        if let Some(on_open_change) = &props.on_open_change {
            on_open_change.call(false);
        }
    };

    // 取消按钮点击事件
    let handle_cancel = move |event: MouseEvent| {
        if let Some(on_cancel) = &props.on_cancel {
            on_cancel.call(event);
        }
        visible.set(false);
        if let Some(on_open_change) = &props.on_open_change {
            on_open_change.call(false);
        }
    };

    let placement_class = match props.placement {
        PopconfirmPlacement::Top => "ant-popover-placement-top",
        PopconfirmPlacement::TopLeft => "ant-popover-placement-topLeft",
        PopconfirmPlacement::TopRight => "ant-popover-placement-topRight",
        PopconfirmPlacement::Bottom => "ant-popover-placement-bottom",
        PopconfirmPlacement::BottomLeft => "ant-popover-placement-bottomLeft",
        PopconfirmPlacement::BottomRight => "ant-popover-placement-bottomRight",
        PopconfirmPlacement::Left => "ant-popover-placement-left",
        PopconfirmPlacement::LeftTop => "ant-popover-placement-leftTop",
        PopconfirmPlacement::LeftBottom => "ant-popover-placement-leftBottom",
        PopconfirmPlacement::Right => "ant-popover-placement-right",
        PopconfirmPlacement::RightTop => "ant-popover-placement-rightTop",
        PopconfirmPlacement::RightBottom => "ant-popover-placement-rightBottom",
    };

    let class_name = format!("ant-popconfirm-wrapper {}", props.class);

    let overlay_class_name = format!(
        "ant-popover ant-popconfirm {} {} {}",
        placement_class,
        if visible() {
            "ant-popover-open"
        } else {
            "ant-popover-hidden"
        },
        props.overlay_class_name.as_ref().map_or("", |s| s.as_str())
    );

    rsx! {
        style { {POPCONFIRM_STYLES} }

        div {
            class: class_name,
            style: props.style,

            // 触发元素
            div {
                class: "ant-popconfirm-trigger",
                onmouseenter: move |_| {
                    if props.trigger == PopconfirmTrigger::Hover {
                        show_popconfirm();
                    }
                },
                onmouseleave: move |_| {
                    if props.trigger == PopconfirmTrigger::Hover {
                        hide_popconfirm();
                    }
                },
                onclick: move |_| {
                    if props.trigger == PopconfirmTrigger::Click {
                        toggle_popconfirm();
                    }
                },
                onfocus: move |_| {
                    if props.trigger == PopconfirmTrigger::Focus {
                        show_popconfirm();
                    }
                },
                onblur: move |_| {
                    if props.trigger == PopconfirmTrigger::Focus {
                        hide_popconfirm();
                    }
                },
                oncontextmenu: move |_| {
                    if props.trigger == PopconfirmTrigger::ContextMenu {
                        toggle_popconfirm();
                    }
                },

                {props.children}
            }

            // 确认框
            if visible() {
                div {
                    class: overlay_class_name,
                    style: props.overlay_style.as_deref().unwrap_or_default(),
                    onclick: move |event| {
                        if let Some(on_popup_click) = &props.on_popup_click {
                            on_popup_click.call(event);
                        }
                    },
                    onmouseenter: move |_| {
                        if props.trigger == PopconfirmTrigger::Hover {
                            // 清除隐藏定时器
                            if let Some(timer) = mouse_leave_timer.take() {
                                timer.cancel();
                            }
                        }
                    },
                    onmouseleave: move |_| {
                        if props.trigger == PopconfirmTrigger::Hover {
                            hide_popconfirm();
                        }
                    },

                    div {
                        class: "ant-popover-content",

                        // 箭头
                        if props.arrow {
                            div {
                                class: "ant-popover-arrow",

                                div {
                                    class: "ant-popover-arrow-content"
                                }
                            }
                        }

                        div {
                            class: "ant-popover-inner ant-popconfirm-inner",

                            div {
                                class: "ant-popconfirm-inner-content",

                                div {
                                    class: "ant-popconfirm-message",

                                    // 图标
                                    div {
                                        class: "ant-popconfirm-message-icon",
                                        if let Some(icon) = props.icon {
                                            {icon}
                                        } else {
                                            // 默认感叹号图标
                                            svg {
                                                class: "ant-popconfirm-icon",
                                                width: "16",
                                                height: "16",
                                                viewBox: "0 0 1024 1024",
                                                fill: "#faad14",
                                                path {
                                                    d: "M512 64C264.6 64 64 264.6 64 512s200.6 448 448 448 448-200.6 448-448S759.4 64 512 64zm-32 232c0-4.4 3.6-8 8-8h48c4.4 0 8 3.6 8 8v272c0 4.4-3.6 8-8 8h-48c-4.4 0-8-3.6-8-8V296zm32 440a48.01 48.01 0 0 1 0-96 48.01 48.01 0 0 1 0 96z"
                                                }
                                            }
                                        }
                                    }

                                    // 标题和描述
                                    div {
                                        class: "ant-popconfirm-message-title",
                                        {props.title}

                                        if let Some(description) = &props.description {
                                            div {
                                                class: "ant-popconfirm-description",
                                                {description}
                                            }
                                        }
                                    }
                                }

                                // 按钮组
                                div {
                                    class: "ant-popconfirm-buttons",

                                    if props.show_cancel {
                                        Button {
                                            class: "ant-popconfirm-cancel",
                                            size: ButtonSize::Small,
                                            onclick: handle_cancel,
                                            {props.cancel_text}
                                        }
                                    }

                                    Button {
                                        class: "ant-popconfirm-ok",
                                        button_type: props.ok_type,
                                        size: ButtonSize::Small,
                                        onclick: handle_confirm,
                                        {props.ok_text}
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

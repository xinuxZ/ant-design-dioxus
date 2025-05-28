//! Popover 气泡卡片组件
//!
//! 点击/鼠标移入元素，弹出气泡式的卡片浮层。
//!
//! ## 何时使用
//!
//! 当目标元素有进一步的描述和相关操作时，可以收纳到卡片中，根据用户的操作行为进行展现。
//! 和 Tooltip 的区别是，用户可以对浮层上的元素进行操作，因此它可以承载更复杂的内容，比如链接或按钮等。

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

/// Popover 气泡卡片组件属性
#[derive(Props, Clone, PartialEq)]
pub struct PopoverProps {
    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 卡片标题
    #[props(default)]
    pub title: Option<String>,

    /// 卡片内容
    pub content: Element,

    /// 气泡框位置
    #[props(default = PopoverPlacement::Top)]
    pub placement: PopoverPlacement,

    /// 触发行为
    #[props(default = PopoverTrigger::Hover)]
    pub trigger: PopoverTrigger,

    /// 是否显示箭头
    #[props(default = true)]
    pub arrow: bool,

    /// 是否默认显示
    #[props(default = false)]
    pub open: bool,

    /// 显示隐藏的回调
    #[props(default)]
    pub on_open_change: Option<EventHandler<bool>>,

    /// 卡片类名
    #[props(default)]
    pub overlay_class_name: Option<String>,

    /// 卡片样式
    #[props(default)]
    pub overlay_style: Option<String>,

    /// 浮层渲染父节点，默认渲染到 body 上
    #[props(default)]
    pub get_popup_container: Option<String>,

    /// 鼠标移入后延时多少才显示 Tooltip，单位：秒
    #[props(default = 0.1)]
    pub mouse_enter_delay: f64,

    /// 鼠标移出后延时多少才隐藏 Tooltip，单位：秒
    #[props(default = 0.1)]
    pub mouse_leave_delay: f64,

    /// 子元素
    pub children: Element,
}

/// 气泡框位置
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PopoverPlacement {
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

impl Default for PopoverPlacement {
    fn default() -> Self {
        Self::Top
    }
}

/// 触发行为
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PopoverTrigger {
    Hover,
    Click,
    Focus,
    ContextMenu,
}

impl Default for PopoverTrigger {
    fn default() -> Self {
        Self::Hover
    }
}

/// Popover 气泡卡片组件
#[component]
pub fn Popover(props: PopoverProps) -> Element {
    let mut visible = use_signal(|| props.open);
    let mut mouse_enter_timer = use_signal(|| None::<gloo_timers::callback::Timeout>);
    let mut mouse_leave_timer = use_signal(|| None::<gloo_timers::callback::Timeout>);

    // 显示气泡卡片
    let mut show_popover = move || {
        // 清除隐藏定时器
        if let Some(timer) = mouse_leave_timer.take() {
            timer.cancel();
        }

        if !visible() {
            let delay = (props.mouse_enter_delay * 1000.0) as u32;
            if delay > 0 {
                let timer = gloo_timers::callback::Timeout::new(delay, move || {
                    visible.set(true);
                    if let Some(on_open_change) = &props.on_open_change {
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

    // 隐藏气泡卡片
    let mut hide_popover = move || {
        // 清除显示定时器
        if let Some(timer) = mouse_enter_timer.take() {
            timer.cancel();
        }

        if visible() {
            let delay = (props.mouse_leave_delay * 1000.0) as u32;
            if delay > 0 {
                let timer = gloo_timers::callback::Timeout::new(delay, move || {
                    visible.set(false);
                    if let Some(on_open_change) = &props.on_open_change {
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
    let mut toggle_popover = move || {
        let new_visible = !visible();
        visible.set(new_visible);
        if let Some(on_open_change) = &props.on_open_change {
            on_open_change.call(new_visible);
        }
    };

    let placement_class = match props.placement {
        PopoverPlacement::Top => "ant-popover-placement-top",
        PopoverPlacement::TopLeft => "ant-popover-placement-topLeft",
        PopoverPlacement::TopRight => "ant-popover-placement-topRight",
        PopoverPlacement::Bottom => "ant-popover-placement-bottom",
        PopoverPlacement::BottomLeft => "ant-popover-placement-bottomLeft",
        PopoverPlacement::BottomRight => "ant-popover-placement-bottomRight",
        PopoverPlacement::Left => "ant-popover-placement-left",
        PopoverPlacement::LeftTop => "ant-popover-placement-leftTop",
        PopoverPlacement::LeftBottom => "ant-popover-placement-leftBottom",
        PopoverPlacement::Right => "ant-popover-placement-right",
        PopoverPlacement::RightTop => "ant-popover-placement-rightTop",
        PopoverPlacement::RightBottom => "ant-popover-placement-rightBottom",
    };

    let class_name = format!("ant-popover-wrapper {}", props.class);

    let overlay_class_name = format!(
        "ant-popover {} {} {}",
        placement_class,
        if visible() {
            "ant-popover-open"
        } else {
            "ant-popover-hidden"
        },
        props.overlay_class_name.as_ref().map_or("", |s| s.as_str())
    );

    rsx! {
        div {
            class: class_name.clone(),
            style: props.style.clone(),

            // 触发元素
            div {
                class: "ant-popover-trigger",
                onmouseenter: move |_| {
                    if props.trigger == PopoverTrigger::Hover {
                        show_popover();
                    }
                },
                onmouseleave: move |_| {
                    if props.trigger == PopoverTrigger::Hover {
                        hide_popover();
                    }
                },
                onclick: move |_| {
                    if props.trigger == PopoverTrigger::Click {
                        toggle_popover();
                    }
                },
                onfocus: move |_| {
                    if props.trigger == PopoverTrigger::Focus {
                        show_popover();
                    }
                },
                onblur: move |_| {
                    if props.trigger == PopoverTrigger::Focus {
                        hide_popover();
                    }
                },
                oncontextmenu: move |_| {
                    if props.trigger == PopoverTrigger::ContextMenu {
                        toggle_popover();
                    }
                },

                {props.children}
            }

            // 气泡卡片
            if visible() {
                div {
                    class: overlay_class_name.clone(),
                    style: props.overlay_style.as_deref().unwrap_or_default().to_string(),
                    onmouseenter: move |_| {
                        if props.trigger == PopoverTrigger::Hover {
                            // 清除隐藏定时器
                            if let Some(timer) = mouse_leave_timer.take() {
                                timer.cancel();
                            }
                        }
                    },
                    onmouseleave: move |_| {
                        if props.trigger == PopoverTrigger::Hover {
                            hide_popover();
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
                            class: "ant-popover-inner",

                            // 标题
                            if let Some(title) = &props.title {
                                div {
                                    class: "ant-popover-title",
                                    {title.clone()}
                                }
                            }

                            // 内容
                            div {
                                class: "ant-popover-inner-content",

                                {props.content}
                            }
                        }
                    }
                }
            }
        }

        style { {include_str!("./style.css")} }
    }
}

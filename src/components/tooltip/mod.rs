//! Tooltip 文字提示组件
//!
//! 简单的文字提示气泡框。
//!
//! ## 何时使用
//!
//! 鼠标移入则显示提示，移出消失，气泡浮层不承载复杂文本和操作。
//!
//! 可用来代替系统默认的 `title` 提示，提供一个 `按钮/文字/操作` 的文案解释。
//!
//! ## 代码演示
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::Tooltip;
//!
//! #[component]
//! fn App() -> Element {
//!     rsx! {
//!         Tooltip {
//!             title: "提示文字",
//!             span { "鼠标移入" }
//!         }
//!     }
//! }
//! ```

use dioxus::prelude::*;

/// Tooltip 触发方式
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TooltipTrigger {
    /// 鼠标移入时触发
    Hover,
    /// 鼠标点击时触发
    Click,
    /// 获得焦点时触发
    Focus,
    /// 手动触发
    Manual,
}

/// Tooltip 弹出位置
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TooltipPlacement {
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

/// Tooltip 主题
#[derive(Debug, Clone, PartialEq)]
pub enum TooltipColor {
    /// 默认主题
    Default,
    /// 自定义颜色
    Custom(String),
}

/// Tooltip 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct TooltipProps {
    /// 提示文字
    pub title: String,
    /// 气泡框位置
    #[props(default = TooltipPlacement::Top)]
    pub placement: TooltipPlacement,
    /// 触发行为
    #[props(default = TooltipTrigger::Hover)]
    pub trigger: TooltipTrigger,
    /// 背景颜色
    #[props(default = TooltipColor::Default)]
    pub color: TooltipColor,
    /// 鼠标移入后延时多少才显示 Tooltip，单位：秒
    #[props(default = 0.1)]
    pub mouse_enter_delay: f64,
    /// 鼠标移出后延时多少才隐藏 Tooltip，单位：秒
    #[props(default = 0.1)]
    pub mouse_leave_delay: f64,
    /// 关闭后是否销毁 Tooltip
    #[props(default = false)]
    pub destroy_tooltip_on_hide: bool,
    /// 是否显示箭头
    #[props(default = true)]
    pub arrow: bool,
    /// 用于手动控制浮层显隐
    pub open: Option<bool>,
    /// 显示隐藏的回调
    pub on_open_change: Option<EventHandler<bool>>,
    /// 设置 Tooltip 的 z-index
    #[props(default = 1070)]
    pub z_index: i32,
    /// 子元素
    pub children: Element,
    /// 自定义样式类名
    #[props(default = String::new())]
    pub class: String,
    /// 自定义内联样式
    #[props(default = String::new())]
    pub style: String,
}

/// Tooltip 文字提示组件
///
/// 简单的文字提示气泡框。
#[component]
pub fn Tooltip(props: TooltipProps) -> Element {
    let visible = use_signal(|| props.open.unwrap_or(false));
    let enter_timer = use_signal(|| None::<i32>);
    let leave_timer = use_signal(|| None::<i32>);

    // 处理显示状态变化
    use_effect({
        let mut visible = visible.clone();
        move || {
            if let Some(open) = props.open {
                visible.set(open);
            }
        }
    });

    // 显示 Tooltip
    let mut show_tooltip = {
        let mut visible = visible.clone();
        let mut enter_timer = enter_timer.clone();
        let mut leave_timer = leave_timer.clone();
        let on_open_change = props.on_open_change.clone();
        let mouse_enter_delay = props.mouse_enter_delay;
        move || {
            // 清理定时器
            if let Some(_timer_id) = enter_timer() {
                // clearTimeout(_timer_id);
            }
            if let Some(_timer_id) = leave_timer() {
                // clearTimeout(_timer_id);
            }
            enter_timer.set(None);
            leave_timer.set(None);

            if mouse_enter_delay > 0.0 {
                // 在实际应用中，这里应该设置定时器
                visible.set(true);
                if let Some(handler) = &on_open_change {
                    handler.call(true);
                }
            } else {
                visible.set(true);
                if let Some(handler) = &on_open_change {
                    handler.call(true);
                }
            }
        }
    };

    // 隐藏 Tooltip
    let mut hide_tooltip = {
        let mut visible = visible.clone();
        let mut enter_timer = enter_timer.clone();
        let mut leave_timer = leave_timer.clone();
        let on_open_change = props.on_open_change.clone();
        let mouse_leave_delay = props.mouse_leave_delay;
        move || {
            // 清理定时器
            if let Some(_timer_id) = enter_timer() {
                // clearTimeout(_timer_id);
            }
            if let Some(_timer_id) = leave_timer() {
                // clearTimeout(_timer_id);
            }
            enter_timer.set(None);
            leave_timer.set(None);

            if mouse_leave_delay > 0.0 {
                // 在实际应用中，这里应该设置定时器
                visible.set(false);
                if let Some(handler) = &on_open_change {
                    handler.call(false);
                }
            } else {
                visible.set(false);
                if let Some(handler) = &on_open_change {
                    handler.call(false);
                }
            }
        }
    };

    // 切换显示状态
    let mut toggle_tooltip = {
        let mut visible = visible.clone();
        let mut enter_timer = enter_timer.clone();
        let mut leave_timer = leave_timer.clone();
        let on_open_change = props.on_open_change.clone();
        let mouse_enter_delay = props.mouse_enter_delay;
        let mouse_leave_delay = props.mouse_leave_delay;
        move || {
            if visible() {
                // 隐藏逻辑
                if let Some(_timer_id) = enter_timer() {
                    // clearTimeout(_timer_id);
                }
                if let Some(_timer_id) = leave_timer() {
                    // clearTimeout(_timer_id);
                }
                enter_timer.set(None);
                leave_timer.set(None);

                if mouse_leave_delay > 0.0 {
                    visible.set(false);
                    if let Some(handler) = &on_open_change {
                        handler.call(false);
                    }
                } else {
                    visible.set(false);
                    if let Some(handler) = &on_open_change {
                        handler.call(false);
                    }
                }
            } else {
                // 显示逻辑
                if let Some(_timer_id) = enter_timer() {
                    // clearTimeout(_timer_id);
                }
                if let Some(_timer_id) = leave_timer() {
                    // clearTimeout(_timer_id);
                }
                enter_timer.set(None);
                leave_timer.set(None);

                if mouse_enter_delay > 0.0 {
                    visible.set(true);
                    if let Some(handler) = &on_open_change {
                        handler.call(true);
                    }
                } else {
                    visible.set(true);
                    if let Some(handler) = &on_open_change {
                        handler.call(true);
                    }
                }
            }
        }
    };

    // 获取位置类名
    let placement_class = match props.placement {
        TooltipPlacement::Top => "ant-tooltip-placement-top",
        TooltipPlacement::TopLeft => "ant-tooltip-placement-topLeft",
        TooltipPlacement::TopRight => "ant-tooltip-placement-topRight",
        TooltipPlacement::Bottom => "ant-tooltip-placement-bottom",
        TooltipPlacement::BottomLeft => "ant-tooltip-placement-bottomLeft",
        TooltipPlacement::BottomRight => "ant-tooltip-placement-bottomRight",
        TooltipPlacement::Left => "ant-tooltip-placement-left",
        TooltipPlacement::LeftTop => "ant-tooltip-placement-leftTop",
        TooltipPlacement::LeftBottom => "ant-tooltip-placement-leftBottom",
        TooltipPlacement::Right => "ant-tooltip-placement-right",
        TooltipPlacement::RightTop => "ant-tooltip-placement-rightTop",
        TooltipPlacement::RightBottom => "ant-tooltip-placement-rightBottom",
    };

    // 获取背景颜色样式
    let color_style = match &props.color {
        TooltipColor::Default => String::new(),
        TooltipColor::Custom(color) => format!("background-color: {};", color),
    };

    rsx! {
        div {
            class: format!("ant-tooltip-wrapper {}", props.class),
            style: props.style.clone(),
            position: "relative",
            display: "inline-block",

            // 触发元素
            div {
                class: "ant-tooltip-trigger",
                onmouseenter: move |_| {
                    if matches!(props.trigger, TooltipTrigger::Hover) && props.open.is_none() {
                        show_tooltip();
                    }
                },
                onmouseleave: move |_| {
                    if matches!(props.trigger, TooltipTrigger::Hover) && props.open.is_none() {
                        hide_tooltip();
                    }
                },
                onclick: move |_| {
                    if matches!(props.trigger, TooltipTrigger::Click) && props.open.is_none() {
                        toggle_tooltip();
                    }
                },
                onfocus: move |_| {
                    if matches!(props.trigger, TooltipTrigger::Focus) && props.open.is_none() {
                        show_tooltip();
                    }
                },
                onblur: move |_| {
                    if matches!(props.trigger, TooltipTrigger::Focus) && props.open.is_none() {
                        hide_tooltip();
                    }
                },
                {props.children}
            }

            // Tooltip 内容
            if visible() {
                div {
                    class: format!("ant-tooltip {}", placement_class),
                    style: format!("z-index: {};", props.z_index),
                    role: "tooltip",

                    div {
                        class: "ant-tooltip-content",

                        // 箭头
                        if props.arrow {
                            div {
                                class: "ant-tooltip-arrow",
                                style: color_style.clone(),
                            }
                        }

                        // 内容区域
                        div {
                            class: "ant-tooltip-inner",
                            style: color_style.clone(),
                            {props.title.clone()}
                        }
                    }
                }
            }
        }
    }
}

//! Popconfirm 气泡确认框组件使用示例

use crate::components::{
    button::{Button, ButtonType},
    popconfirm::{Popconfirm, PopconfirmPlacement, PopconfirmTrigger},
};
use dioxus::prelude::*;

/// Popconfirm 基础示例
#[component]
pub fn PopconfirmBasicExample() -> Element {
    rsx! {
        div {
            style: "padding: 20px;",

            h3 { "基础用法" }
            p { "最简单的用法。" }

            Popconfirm {
                title: "确定要删除这个任务吗？".to_string(),
                on_confirm: move |_| {
                    web_sys::console::log_1(&"确认删除".into());
                },
                on_cancel: move |_| {
                    web_sys::console::log_1(&"取消删除".into());
                },

                Button {
                    button_type: ButtonType::Primary,
                    "删除"
                }
            }
        }
    }
}

/// Popconfirm 位置示例
#[component]
pub fn PopconfirmPlacementExample() -> Element {
    rsx! {
        div {
            style: "padding: 20px;",

            h3 { "位置" }
            p { "位置有十二个方向。" }

            div {
                style: "display: flex; flex-wrap: wrap; gap: 8px;",

                Popconfirm {
                    title: "确定要删除吗？".to_string(),
                    placement: PopconfirmPlacement::TopLeft,

                    Button { "TL" }
                }

                Popconfirm {
                    title: "确定要删除吗？".to_string(),
                    placement: PopconfirmPlacement::Top,

                    Button { "Top" }
                }

                Popconfirm {
                    title: "确定要删除吗？".to_string(),
                    placement: PopconfirmPlacement::TopRight,

                    Button { "TR" }
                }

                Popconfirm {
                    title: "确定要删除吗？".to_string(),
                    placement: PopconfirmPlacement::LeftTop,

                    Button { "LT" }
                }

                Popconfirm {
                    title: "确定要删除吗？".to_string(),
                    placement: PopconfirmPlacement::Left,

                    Button { "Left" }
                }

                Popconfirm {
                    title: "确定要删除吗？".to_string(),
                    placement: PopconfirmPlacement::LeftBottom,

                    Button { "LB" }
                }

                Popconfirm {
                    title: "确定要删除吗？".to_string(),
                    placement: PopconfirmPlacement::RightTop,

                    Button { "RT" }
                }

                Popconfirm {
                    title: "确定要删除吗？".to_string(),
                    placement: PopconfirmPlacement::Right,

                    Button { "Right" }
                }

                Popconfirm {
                    title: "确定要删除吗？".to_string(),
                    placement: PopconfirmPlacement::RightBottom,

                    Button { "RB" }
                }

                Popconfirm {
                    title: "确定要删除吗？".to_string(),
                    placement: PopconfirmPlacement::BottomLeft,

                    Button { "BL" }
                }

                Popconfirm {
                    title: "确定要删除吗？".to_string(),
                    placement: PopconfirmPlacement::Bottom,

                    Button { "Bottom" }
                }

                Popconfirm {
                    title: "确定要删除吗？".to_string(),
                    placement: PopconfirmPlacement::BottomRight,

                    Button { "BR" }
                }
            }
        }
    }
}

/// Popconfirm 自定义按钮文字示例
#[component]
pub fn PopconfirmCustomTextExample() -> Element {
    rsx! {
        div {
            style: "padding: 20px;",

            h3 { "自定义按钮文字" }
            p { "设置 ok_text 和 cancel_text 以自定义按钮文字。" }

            Popconfirm {
                title: "确定要删除这个任务吗？".to_string(),
                ok_text: "是的".to_string(),
                cancel_text: "不了".to_string(),
                on_confirm: move |_| {
                    web_sys::console::log_1(&"确认删除".into());
                },
                on_cancel: move |_| {
                    web_sys::console::log_1(&"取消删除".into());
                },

                Button {
                    button_type: ButtonType::Primary,
                    "删除"
                }
            }
        }
    }
}

/// Popconfirm 条件触发示例
#[component]
pub fn PopconfirmConditionalExample() -> Element {
    let mut condition = use_signal(|| true);

    rsx! {
        div {
            style: "padding: 20px;",

            h3 { "条件触发" }
            p { "可以判断是否需要弹出。" }

            div {
                style: "margin-bottom: 16px;",

                label {
                    input {
                        r#type: "checkbox",
                        checked: condition(),
                        onchange: move |event| {
                            condition.set(event.checked());
                        }
                    }
                    " 是否弹出确认框"
                }
            }

            Popconfirm {
                title: "确定要删除这个任务吗？".to_string(),
                disabled: !condition(),
                on_confirm: move |_| {
                    web_sys::console::log_1(&"确认删除".into());
                },

                Button {
                    button_type: ButtonType::Primary,
                    "删除"
                }
            }
        }
    }
}

/// Popconfirm 异步关闭示例
#[component]
pub fn PopconfirmAsyncExample() -> Element {
    let mut loading = use_signal(|| false);

    rsx! {
        div {
            style: "padding: 20px;",

            h3 { "异步关闭" }
            p { "点击确定后异步关闭气泡确认框，例如提交表单。" }

            Popconfirm {
                title: "确定要提交吗？".to_string(),
                on_confirm: move |_| {
                    loading.set(true);
                    // 模拟异步操作
                    let loading_clone = loading.clone();
                    spawn(async move {
                        gloo_timers::future::TimeoutFuture::new(2000).await;
                        loading_clone.set(false);
                        web_sys::console::log_1(&"提交成功".into());
                    });
                },

                Button {
                    button_type: ButtonType::Primary,
                    loading: loading(),
                    "提交"
                }
            }
        }
    }
}

/// Popconfirm 自定义图标示例
#[component]
pub fn PopconfirmCustomIconExample() -> Element {
    rsx! {
        div {
            style: "padding: 20px;",

            h3 { "自定义图标" }
            p { "使用自定义图标。" }

            Popconfirm {
                title: "确定要删除吗？".to_string(),
                icon: rsx! {
                    svg {
                        width: "16",
                        height: "16",
                        viewBox: "0 0 1024 1024",
                        fill: "#ff4d4f",
                        path {
                            d: "M512 64C264.6 64 64 264.6 64 512s200.6 448 448 448 448-200.6 448-448S759.4 64 512 64zm165.4 618.2l-66-.3L512 563.4l-99.3 118.4-66.1.3c-4.4 0-8-3.5-8-8 0-1.9.7-3.7 1.9-5.2l130.1-155L340.5 359a8.32 8.32 0 0 1-1.9-5.2c0-4.4 3.6-8 8-8l66.1.3L512 464.6l99.3-118.4 66-.3c4.4 0 8 3.5 8 8 0 1.9-.7 3.7-1.9 5.2L553.5 514l130 155c1.2 1.5 1.9 3.3 1.9 5.2 0 4.4-3.6 8-8 8z"
                        }
                    }
                },
                on_confirm: move |_| {
                    web_sys::console::log_1(&"确认删除".into());
                },

                Button {
                    button_type: ButtonType::Danger,
                    "删除"
                }
            }
        }
    }
}

/// Popconfirm 触发方式示例
#[component]
pub fn PopconfirmTriggerExample() -> Element {
    rsx! {
        div {
            style: "padding: 20px;",

            h3 { "触发方式" }
            p { "可以通过 hover、click、focus 等方式触发。" }

            div {
                style: "display: flex; gap: 16px;",

                Popconfirm {
                    title: "确定要删除吗？".to_string(),
                    trigger: PopconfirmTrigger::Hover,

                    Button { "Hover 触发" }
                }

                Popconfirm {
                    title: "确定要删除吗？".to_string(),
                    trigger: PopconfirmTrigger::Click,

                    Button { "Click 触发" }
                }

                Popconfirm {
                    title: "确定要删除吗？".to_string(),
                    trigger: PopconfirmTrigger::Focus,

                    Button { "Focus 触发" }
                }
            }
        }
    }
}

/// Popconfirm 完整示例
#[component]
pub fn PopconfirmExample() -> Element {
    rsx! {
        div {
            style: "padding: 20px; max-width: 1200px;",

            h2 { "Popconfirm 气泡确认框" }
            p { "点击元素，弹出气泡式的确认框。" }

            PopconfirmBasicExample {}
            PopconfirmPlacementExample {}
            PopconfirmCustomTextExample {}
            PopconfirmConditionalExample {}
            PopconfirmAsyncExample {}
            PopconfirmCustomIconExample {}
            PopconfirmTriggerExample {}
        }
    }
}

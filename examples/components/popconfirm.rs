#![allow(non_snake_case)]
//!
//! 展示 Popconfirm 组件的各种用法和样式

use crate::common::*;
use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

/// Popconfirm 组件演示
#[component]
pub fn PopconfirmDemo() -> Element {
    let mut visible = use_signal(|| false);
    let mut condition = use_signal(|| true);

    rsx! {
        div {
            class: "component-showcase",

            h1 {
                style: "margin: 0 0 16px 0; color: #262626; font-size: 28px;",
                "Popconfirm 气泡确认框"
            }

            p {
                style: "margin: 0 0 24px 0; color: #666; font-size: 14px;",
                "点击元素，弹出气泡式的确认框。"
            }

            // 基础用法
            DemoSection {
                title: "基础用法",
                description: "最简单的用法。",

                div {
                    style: "display: flex; gap: 8px; flex-wrap: wrap;",

                    Popconfirm {
                        title: "Are you sure to delete this task?",
                        on_confirm: move |_| {
                            message::success("Clicked on Yes.");
                        },
                        on_cancel: move |_| {
                            message::error("Clicked on No.");
                        },
                        ok_text: "Yes",
                        cancel_text: "No",

                        Button {
                            danger: true,
                            "Delete"
                        }
                    }
                }
            }

            // 国际化
            DemoSection {
                title: "国际化",
                description: "使用 okText 和 cancelText 自定义按钮文字。",

                div {
                    style: "display: flex; gap: 8px; flex-wrap: wrap;",

                    Popconfirm {
                        title: "Are you sure?",
                        ok_text: "确定",
                        cancel_text: "取消",
                        on_confirm: move |_| {
                            message::info("确定");
                        },
                        on_cancel: move |_| {
                            message::info("取消");
                        },

                        Button {
                            "国际化"
                        }
                    }
                }
            }

            // 位置
            DemoSection {
                title: "位置",
                description: "位置有十二个方向。如需箭头指向目标元素中心，可以设置 arrowPointAtCenter。",

                div {
                    style: "display: flex; gap: 8px; flex-wrap: wrap;",

                    Popconfirm {
                        placement: PopconfirmPlacement::TopLeft,
                        title: "Are you sure?",
                        ok_text: "Yes",
                        cancel_text: "No",

                        Button {
                            "TL"
                        }
                    }

                    Popconfirm {
                        placement: PopconfirmPlacement::Top,
                        title: "Are you sure?",
                        ok_text: "Yes",
                        cancel_text: "No",

                        Button {
                            "Top"
                        }
                    }

                    Popconfirm {
                        placement: PopconfirmPlacement::TopRight,
                        title: "Are you sure?",
                        ok_text: "Yes",
                        cancel_text: "No",

                        Button {
                            "TR"
                        }
                    }

                    Popconfirm {
                        placement: PopconfirmPlacement::LeftTop,
                        title: "Are you sure?",
                        ok_text: "Yes",
                        cancel_text: "No",

                        Button {
                            "LT"
                        }
                    }

                    Popconfirm {
                        placement: PopconfirmPlacement::Left,
                        title: "Are you sure?",
                        ok_text: "Yes",
                        cancel_text: "No",

                        Button {
                            "Left"
                        }
                    }

                    Popconfirm {
                        placement: PopconfirmPlacement::LeftBottom,
                        title: "Are you sure?",
                        ok_text: "Yes",
                        cancel_text: "No",

                        Button {
                            "LB"
                        }
                    }

                    Popconfirm {
                        placement: PopconfirmPlacement::RightTop,
                        title: "Are you sure?",
                        ok_text: "Yes",
                        cancel_text: "No",

                        Button {
                            "RT"
                        }
                    }

                    Popconfirm {
                        placement: PopconfirmPlacement::Right,
                        title: "Are you sure?",
                        ok_text: "Yes",
                        cancel_text: "No",

                        Button {
                            "Right"
                        }
                    }

                    Popconfirm {
                        placement: PopconfirmPlacement::RightBottom,
                        title: "Are you sure?",
                        ok_text: "Yes",
                        cancel_text: "No",

                        Button {
                            "RB"
                        }
                    }

                    Popconfirm {
                        placement: PopconfirmPlacement::BottomLeft,
                        title: "Are you sure?",
                        ok_text: "Yes",
                        cancel_text: "No",

                        Button {
                            "BL"
                        }
                    }

                    Popconfirm {
                        placement: PopconfirmPlacement::Bottom,
                        title: "Are you sure?",
                        ok_text: "Yes",
                        cancel_text: "No",

                        Button {
                            "Bottom"
                        }
                    }

                    Popconfirm {
                        placement: PopconfirmPlacement::BottomRight,
                        title: "Are you sure?",
                        ok_text: "Yes",
                        cancel_text: "No",

                        Button {
                            "BR"
                        }
                    }
                }
            }

            // 条件触发
            DemoSection {
                title: "条件触发",
                description: "可以判断是否需要弹出。",

                div {
                    style: "display: flex; gap: 8px; flex-wrap: wrap; align-items: center;",

                    Popconfirm {
                        title: "Are you sure?",
                        disabled: !condition(),
                        on_confirm: move |_| {
                            message::success("Next time will not popup if disabled is true");
                        },

                        Button {
                            button_type: ButtonType::Primary,
                            "Conditional Trigger"
                        }
                    }

                    span {
                        style: "margin-left: 8px;",
                        "Whether directly execute: "
                    }

                    Switch {
                        checked: condition(),
                        onchange: move |checked| {
                            condition.set(checked);
                        }
                    }
                }
            }

            // 自定义 Icon
            DemoSection {
                title: "自定义 Icon",
                description: "使用自定义的 Icon。",

                div {
                    style: "display: flex; gap: 8px; flex-wrap: wrap;",

                    Popconfirm {
                        title: "Are you sure?",
                        icon: rsx! {
                            Icon {
                                icon_type: "question-circle-outlined".to_string(),
                                style: "color: red;"
                            }
                        },

                        Button {
                            "Delete"
                        }
                    }
                }
            }

            // 异步关闭
            DemoSection {
                title: "异步关闭",
                description: "点击确定后异步关闭 Popconfirm，例如提交表单。",

                div {
                    style: "display: flex; gap: 8px; flex-wrap: wrap;",

                    Popconfirm {
                        title: "Title",
                        open: visible(),
                        on_confirm: move |_| {
                            // 模拟异步操作
                            spawn(async move {
                                gloo_timers::future::TimeoutFuture::new(3000).await;
                                visible.set(false);
                                message::success("Async operation completed");
                            });
                        },
                        on_cancel: move |_| {
                            visible.set(false);
                        },
                        on_open_change: move |open: bool| {
                            if !open {
                                visible.set(false);
                            }
                        },

                        Button {
                            button_type: ButtonType::Primary,
                            onclick: move |_| {
                                visible.set(true);
                            },
                            "Open Popconfirm with async logic"
                        }
                    }
                }
            }

            // API 文档
            ApiDocumentation {
                component_name: "Popconfirm",
                props: vec![
                    PropDoc {
                        name: "cancel_text".to_string(),
                        prop_type: "String".to_string(),
                        default: "取消".to_string(),
                        description: "取消按钮文字".to_string(),
                    },
                    PropDoc {
                        name: "ok_text".to_string(),
                        prop_type: "String".to_string(),
                        default: "确定".to_string(),
                        description: "确认按钮文字".to_string(),
                    },
                    PropDoc {
                        name: "ok_type".to_string(),
                        prop_type: "ButtonType".to_string(),
                        default: "Primary".to_string(),
                        description: "确认按钮类型".to_string(),
                    },
                    PropDoc {
                        name: "title".to_string(),
                        prop_type: "String | Element".to_string(),
                        default: "-".to_string(),
                        description: "确认框的描述".to_string(),
                    },
                    PropDoc {
                        name: "on_cancel".to_string(),
                        prop_type: "Function".to_string(),
                        default: "-".to_string(),
                        description: "点击取消的回调".to_string(),
                    },
                    PropDoc {
                        name: "on_confirm".to_string(),
                        prop_type: "Function".to_string(),
                        default: "-".to_string(),
                        description: "点击确认的回调".to_string(),
                    },
                    PropDoc {
                        name: "icon".to_string(),
                        prop_type: "Element".to_string(),
                        default: "<ExclamationCircleOutlined />".to_string(),
                        description: "自定义弹出气泡 Icon 图标".to_string(),
                    },
                    PropDoc {
                        name: "disabled".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "点击 Popconfirm 子元素是否弹出气泡确认框".to_string(),
                    },
                    PropDoc {
                        name: "open".to_string(),
                        prop_type: "bool".to_string(),
                        default: "-".to_string(),
                        description: "用于手动控制浮层显隐".to_string(),
                    },
                    PropDoc {
                        name: "placement".to_string(),
                        prop_type: "String".to_string(),
                        default: "top".to_string(),
                        description: "气泡框位置，可选 top left right bottom topLeft topRight bottomLeft bottomRight leftTop leftBottom rightTop rightBottom".to_string(),
                    },
                ]
            }
        }
    }
}

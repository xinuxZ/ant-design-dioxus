#![allow(non_snake_case)]
//!
//! 展示 Notification 组件的各种用法和样式

use crate::common::*;
use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

/// Notification 组件演示
#[component]
pub fn NotificationDemo() -> Element {
    rsx! {
        div {
            class: "component-showcase",

            h1 {
                style: "margin: 0 0 16px 0; color: #262626; font-size: 28px;",
                "Notification 通知提醒框"
            }

            p {
                style: "margin: 0 0 24px 0; color: #666; font-size: 14px;",
                "全局展示通知提醒信息。"
            }

            // 基础用法
            DemoSection {
                title: "基础用法",
                description: "最简单的用法，4.5 秒后自动关闭。",

                div {
                    style: "display: flex; gap: 8px; flex-wrap: wrap;",

                    Button {
                        button_type: ButtonType::Primary,
                        onclick: move |_| {
                            notification::open(NotificationConfig {
                                message: "Notification Title".to_string(),
                                description: "This is the content of the notification. This is the content of the notification. This is the content of the notification.".to_string(),
                                ..Default::default()
                            });
                        },
                        "Open the notification box"
                    }
                }
            }

            // 带有图标的通知提醒框
            DemoSection {
                title: "带有图标的通知提醒框",
                description: "通知提醒框左侧有图标。",

                div {
                    style: "display: flex; gap: 8px; flex-wrap: wrap;",

                    Button {
                        onclick: move |_| {
                            notification::success(NotificationConfig {
                                message: "Notification Title".to_string(),
                                description: "This is the content of the notification. This is the content of the notification. This is the content of the notification.".to_string(),
                                ..Default::default()
                            });
                        },
                        "Success"
                    }

                    Button {
                        onclick: move |_| {
                            notification::info(NotificationConfig {
                                message: "Notification Title".to_string(),
                                description: "This is the content of the notification. This is the content of the notification. This is the content of the notification.".to_string(),
                                ..Default::default()
                            });
                        },
                        "Info"
                    }

                    Button {
                        onclick: move |_| {
                            notification::warning(NotificationConfig {
                                message: "Notification Title".to_string(),
                                description: "This is the content of the notification. This is the content of the notification. This is the content of the notification.".to_string(),
                                ..Default::default()
                            });
                        },
                        "Warning"
                    }

                    Button {
                        onclick: move |_| {
                            notification::error(NotificationConfig {
                                message: "Notification Title".to_string(),
                                description: "This is the content of the notification. This is the content of the notification. This is the content of the notification.".to_string(),
                                ..Default::default()
                            });
                        },
                        "Error"
                    }
                }
            }

            // 自定义按钮
            DemoSection {
                title: "自定义按钮",
                description: "自定义关闭按钮的样式和文字。",

                div {
                    style: "display: flex; gap: 8px; flex-wrap: wrap;",

                    Button {
                        button_type: ButtonType::Primary,
                        onclick: move |_| {
                            let key = format!("open{}", js_sys::Date::now());
                            notification::open(NotificationConfig {
                                message: "Notification Title".to_string(),
                                description: "A function will be be called after the notification is closed (automatically after the 'duration' time of manually).".to_string(),
                                btn: Some(rsx! {
                                    Button {
                                        button_type: ButtonType::Primary,
                                        size: ButtonSize::Small,
                                        onclick: move |_| {
                                            notification::close(&key);
                                        },
                                        "Confirm"
                                    }
                                }),
                                key: key.clone(),
                                on_close: Some(Box::new(|| {
                                    web_sys::console::log_1(&"Notification was closed.".into());
                                })),
                                ..Default::default()
                            });
                        },
                        "Open the notification box"
                    }
                }
            }

            // 自定义图标
            DemoSection {
                title: "自定义图标",
                description: "图标可以被自定义。",

                div {
                    style: "display: flex; gap: 8px; flex-wrap: wrap;",

                    Button {
                        button_type: ButtonType::Primary,
                        onclick: move |_| {
                            notification::open(NotificationConfig {
                                message: "Notification Title".to_string(),
                                description: "This is the content of the notification. This is the content of the notification. This is the content of the notification.".to_string(),
                                icon: Some(rsx! {
                                    Icon {
                                        icon_type: "smile-outlined".to_string(),
                                        style: "color: #108ee9;"
                                    }
                                }),
                                ..Default::default()
                            });
                        },
                        "Open the notification box"
                    }
                }
            }

            // 自定义样式
            DemoSection {
                title: "自定义样式",
                description: "使用 style 和 className 来定义样式。",

                div {
                    style: "display: flex; gap: 8px; flex-wrap: wrap;",

                    Button {
                        button_type: ButtonType::Primary,
                        onclick: move |_| {
                            notification::open(NotificationConfig {
                                message: "Notification Title".to_string(),
                                description: "This is the content of the notification. This is the content of the notification. This is the content of the notification.".to_string(),
                                class_name: "custom-class".to_string(),
                                style: "width: 600px; margin-left: 335px - 600px;".to_string(),
                                ..Default::default()
                            });
                        },
                        "Open the notification box"
                    }
                }
            }

            // 位置
            DemoSection {
                title: "位置",
                description: "可以设置通知从右上角、右下角、左下角、左上角弹出。",

                div {
                    style: "display: flex; gap: 8px; flex-wrap: wrap;",

                    Button {
                        onclick: move |_| {
                            notification::open_with_placement(NotificationConfig {
                                message: "Notification Title".to_string(),
                                description: "This is the content of the notification.".to_string(),
                                ..Default::default()
                            }, "topLeft");
                        },
                        "topLeft"
                    }

                    Button {
                        onclick: move |_| {
                            notification::open_with_placement(NotificationConfig {
                                message: "Notification Title".to_string(),
                                description: "This is the content of the notification.".to_string(),
                                ..Default::default()
                            }, "topRight");
                        },
                        "topRight"
                    }

                    Button {
                        onclick: move |_| {
                            notification::open_with_placement(NotificationConfig {
                                message: "Notification Title".to_string(),
                                description: "This is the content of the notification.".to_string(),
                                ..Default::default()
                            }, "bottomLeft");
                        },
                        "bottomLeft"
                    }

                    Button {
                        onclick: move |_| {
                            notification::open_with_placement(NotificationConfig {
                                message: "Notification Title".to_string(),
                                description: "This is the content of the notification.".to_string(),
                                ..Default::default()
                            }, "bottomRight");
                        },
                        "bottomRight"
                    }
                }
            }

            // API 文档
            ApiDocumentation {
                component_name: "Notification",
                props: vec![
                    PropDoc {
                        name: "message".to_string(),
                        prop_type: "String".to_string(),
                        default: "-".to_string(),
                        description: "通知提醒标题，必选".to_string(),
                    },
                    PropDoc {
                        name: "description".to_string(),
                        prop_type: "String".to_string(),
                        default: "-".to_string(),
                        description: "通知提醒内容，必选".to_string(),
                    },
                    PropDoc {
                        name: "btn".to_string(),
                        prop_type: "Element".to_string(),
                        default: "-".to_string(),
                        description: "自定义关闭按钮".to_string(),
                    },
                    PropDoc {
                        name: "class_name".to_string(),
                        prop_type: "String".to_string(),
                        default: "-".to_string(),
                        description: "自定义 CSS class".to_string(),
                    },
                    PropDoc {
                        name: "duration".to_string(),
                        prop_type: "f64".to_string(),
                        default: "4.5".to_string(),
                        description: "默认 4.5 秒后自动关闭，配置为 null 则不自动关闭".to_string(),
                    },
                    PropDoc {
                        name: "icon".to_string(),
                        prop_type: "Element".to_string(),
                        default: "-".to_string(),
                        description: "自定义图标".to_string(),
                    },
                    PropDoc {
                        name: "key".to_string(),
                        prop_type: "String".to_string(),
                        default: "-".to_string(),
                        description: "当前通知唯一标志".to_string(),
                    },
                    PropDoc {
                        name: "style".to_string(),
                        prop_type: "String".to_string(),
                        default: "-".to_string(),
                        description: "自定义内联样式".to_string(),
                    },
                    PropDoc {
                        name: "on_close".to_string(),
                        prop_type: "Function".to_string(),
                        default: "-".to_string(),
                        description: "点击默认关闭按钮时触发的回调函数".to_string(),
                    },
                ]
            }
        }
    }
}

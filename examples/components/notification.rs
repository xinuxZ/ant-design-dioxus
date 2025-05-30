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
                            let notification_item = NotificationItem::new(
                                NotificationType::Info,
                                "Notification Title",
                                Some("This is the content of the notification. This is the content of the notification. This is the content of the notification.")
                            );
                            notification::open(notification_item);
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
                            notification::success(
                                "Notification Title",
                                "This is the content of the notification. This is the content of the notification. This is the content of the notification."
                            );
                        },
                        "Success"
                    }

                    Button {
                        onclick: move |_| {
                            notification::info(
                                "Notification Title",
                                "This is the content of the notification. This is the content of the notification. This is the content of the notification."
                            );
                        },
                        "Info"
                    }

                    Button {
                        onclick: move |_| {
                            notification::warning(
                                "Notification Title",
                                "This is the content of the notification. This is the content of the notification. This is the content of the notification."
                            );
                        },
                        "Warning"
                    }

                    Button {
                        onclick: move |_| {
                            notification::error(
                                "Notification Title",
                                "This is the content of the notification. This is the content of the notification. This is the content of the notification."
                            );
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
                            let notification_item = NotificationItem::new(
                                NotificationType::Info,
                                "Notification Title",
                                Some("A function will be be called after the notification is closed (automatically after the 'duration' time of manually).")
                            );
                            notification::open(notification_item);
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
                            let notification_item = NotificationItem::new(
                                NotificationType::Info,
                                "Notification Title",
                                Some("This is the content of the notification. This is the content of the notification. This is the content of the notification.")
                            ).with_icon("😊");
                            notification::open(notification_item);
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
                            let notification_item = NotificationItem::new(
                                NotificationType::Info,
                                "Notification Title",
                                Some("This is the content of the notification. This is the content of the notification. This is the content of the notification.")
                            ).with_class_name("custom-class")
                             .with_style("width: 600px; margin-left: 335px - 600px;");
                            notification::open(notification_item);
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
                        button_type: ButtonType::Primary,
                        onclick: move |_| {
                            notification::config(NotificationConfig {
                                placement: NotificationPlacement::TopLeft,
                                ..Default::default()
                            });
                            let notification_item = NotificationItem::new(
                                NotificationType::Info,
                                "Notification Title",
                                Some("This is the content of the notification. This is the content of the notification. This is the content of the notification.")
                            );
                            notification::open(notification_item);
                        },
                        "topLeft"
                    }

                    Button {
                        button_type: ButtonType::Primary,
                        onclick: move |_| {
                            notification::config(NotificationConfig {
                                placement: NotificationPlacement::TopRight,
                                ..Default::default()
                            });
                            let notification_item = NotificationItem::new(
                                NotificationType::Info,
                                "Notification Title",
                                Some("This is the content of the notification.")
                            );
                            notification::open(notification_item);
                        },
                        "topRight"
                    }

                    Button {
                        button_type: ButtonType::Primary,
                        onclick: move |_| {
                            notification::config(NotificationConfig {
                                placement: NotificationPlacement::BottomLeft,
                                ..Default::default()
                            });
                            let notification_item = NotificationItem::new(
                                NotificationType::Info,
                                "Notification Title",
                                Some("This is the content of the notification.")
                            );
                            notification::open(notification_item);
                        },
                        "bottomLeft"
                    }

                    Button {
                        button_type: ButtonType::Primary,
                        onclick: move |_| {
                            notification::config(NotificationConfig {
                                placement: NotificationPlacement::BottomRight,
                                ..Default::default()
                            });
                            let notification_item = NotificationItem::new(
                                NotificationType::Info,
                                "Notification Title",
                                Some("This is the content of the notification.")
                            );
                            notification::open(notification_item);
                        },
                        "bottomRight"
                    }
                }
            }

            // API 文档
            ApiDocumentation {
                component_name: "Notification",
                props: vec![
                    // NotificationItem 属性
                    PropDoc {
                        name: "title".to_string(),
                        prop_type: "&str".to_string(),
                        default: "-".to_string(),
                        description: "通知提醒标题，必选".to_string(),
                    },
                    PropDoc {
                        name: "description".to_string(),
                        prop_type: "Option<&str>".to_string(),
                        default: "None".to_string(),
                        description: "通知提醒内容，可选".to_string(),
                    },
                    PropDoc {
                        name: "notification_type".to_string(),
                        prop_type: "NotificationType".to_string(),
                        default: "NotificationType::Info".to_string(),
                        description: "通知类型，可选值：Info、Success、Warning、Error".to_string(),
                    },
                    PropDoc {
                        name: "duration".to_string(),
                        prop_type: "Option<f64>".to_string(),
                        default: "Some(4.5)".to_string(),
                        description: "默认 4.5 秒后自动关闭，配置为 None 则不自动关闭".to_string(),
                    },
                    PropDoc {
                        name: "icon".to_string(),
                        prop_type: "Option<&str>".to_string(),
                        default: "None".to_string(),
                        description: "自定义图标".to_string(),
                    },
                    PropDoc {
                        name: "class_name".to_string(),
                        prop_type: "Option<&str>".to_string(),
                        default: "None".to_string(),
                        description: "自定义 CSS class".to_string(),
                    },
                    PropDoc {
                        name: "style".to_string(),
                        prop_type: "Option<&str>".to_string(),
                        default: "None".to_string(),
                        description: "自定义内联样式".to_string(),
                    },
                    // NotificationConfig 属性
                    PropDoc {
                        name: "placement".to_string(),
                        prop_type: "NotificationPlacement".to_string(),
                        default: "NotificationPlacement::TopRight".to_string(),
                        description: "弹出位置，可选值：TopLeft、TopRight、BottomLeft、BottomRight".to_string(),
                    },
                    PropDoc {
                        name: "top".to_string(),
                        prop_type: "Option<i32>".to_string(),
                        default: "None".to_string(),
                        description: "消息从顶部弹出时，距离顶部的位置，单位像素".to_string(),
                    },
                    PropDoc {
                        name: "bottom".to_string(),
                        prop_type: "Option<i32>".to_string(),
                        default: "None".to_string(),
                        description: "消息从底部弹出时，距离底部的位置，单位像素".to_string(),
                    },
                    PropDoc {
                        name: "max_count".to_string(),
                        prop_type: "Option<usize>".to_string(),
                        default: "None".to_string(),
                        description: "最大显示数，超过限制时，最早的消息会被自动关闭".to_string(),
                    },
                    PropDoc {
                        name: "rtl".to_string(),
                        prop_type: "Option<bool>".to_string(),
                        default: "None".to_string(),
                        description: "是否开启 RTL 模式".to_string(),
                    },
                    PropDoc {
                        name: "stack".to_string(),
                        prop_type: "Option<bool>".to_string(),
                        default: "None".to_string(),
                        description: "是否堆叠显示".to_string(),
                    },
                ]
            }
        }
    }
}

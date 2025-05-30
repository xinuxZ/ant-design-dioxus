#![allow(non_snake_case)]
//!
//! 展示 Message 组件的各种用法和样式

use crate::common::*;
use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

/// Message 组件演示
#[component]
pub fn MessageDemo() -> Element {
    rsx! {
        div {
            class: "component-showcase",

            h1 {
                style: "margin: 0 0 16px 0; color: #262626; font-size: 28px;",
                "Message 全局提示"
            }

            p {
                style: "margin: 0 0 24px 0; color: #666; font-size: 14px;",
                "全局展示操作反馈信息。"
            }

            // 基础用法
            DemoSection {
                title: "基础用法",
                description: "信息提醒反馈。",

                div {
                    style: "display: flex; gap: 8px; flex-wrap: wrap;",

                    Button {
                        onclick: move |_| {
                            MessageManager::new().info("This is a normal message", Some(1000 as f64));
                        },
                        "Display normal message"
                    }
                }
            }

            // 其他提示类型
            DemoSection {
                title: "其他提示类型",
                description: "包括成功、失败、警告。",

                div {
                    style: "display: flex; gap: 8px; flex-wrap: wrap;",

                    Button {
                        onclick: move |_| {
                            MessageManager::new().success("This is a success message", Some(1000 as f64));
                        },
                        "Success"
                    }

                    Button {
                        onclick: move |_| {
                            MessageManager::new().error("This is an error message", Some(1000 as f64));
                        },
                        "Error"
                    }

                    Button {
                        onclick: move |_| {
                            MessageManager::new().warning("This is a warning message", Some(1000 as f64));
                        },
                        "Warning"
                    }
                }
            }

            // 修改延时
            DemoSection {
                title: "修改延时",
                description: "自定义时长 10s，默认时长为 3s。",

                div {
                    style: "display: flex; gap: 8px; flex-wrap: wrap;",

                    Button {
                        onclick: move |_| {
                            MessageManager::new().success("This is a prompt message for success, and it will disappear in 10 seconds", Some(1000 as f64));
                        },
                        "Customized display duration"
                    }
                }
            }

            // 加载中
            // DemoSection {
            //     title: "加载中",
            //     description: "进行全局 loading，异步自行移除。",

            //     div {
            //         style: "display: flex; gap: 8px; flex-wrap: wrap;",

            //         Button {
            //             onclick: move |_| {
            //                 let hide = MessageManager::new().loading("Action in progress..", Some(1000 as f64));
            //                 // 模拟异步操作
            //                 spawn(async move {
            //                     gloo_timers::future::TimeoutFuture::new(2000).await;
            //                     hide();
            //                     MessageManager::new().success("Loading finished", Some(1000 as f64));
            //                 });
            //             },
            //             "Display a loading indicator"
            //         }
            //     }
            // }

            // Promise 接口
            DemoSection {
                title: "Promise 接口",
                description: "可以通过 then 接口在关闭后运行 callback 。以上用例将在每个 message 将要结束时通过 then 显示新的 message 。",

                div {
                    style: "display: flex; gap: 8px; flex-wrap: wrap;",

                    Button {
                        onclick: move |_| {
                            MessageManager::new().loading("Action in progress..", Some(1000 as f64));
                                // .then(|| MessageManager::new().success("Loading finished", Some(1000 as f64)))
                                // .then(|| MessageManager::new().info("Loading finished is finished", Some(1000 as f64)));
                        },
                        "Display sequential messages"
                    }
                }
            }

            // 自定义样式
            // DemoSection {
            //     title: "自定义样式",
            //     description: "使用 style 和 className 来定义样式。",

            //     div {
            //         style: "display: flex; gap: 8px; flex-wrap: wrap;",

            //         Button {
            //             onclick: move |_| {
            //                 MessageManager::new().config(MessageConfig {
            //                     duration: Some(3000 as f64),
            //                     max_count: Some(3),
            //                     rtl: Some(false),
            //                     top: None,
            //                     prefix_cls: Some("custom-message".to_string()),
            //                 }).succcess( "This is a custom styled message".to_string(), Some(3000 as f64));
            //             },
            //             "Customized style"
            //         }
            //     }
            // }

            // 更新消息内容
            // DemoSection {
            //     title: "更新消息内容",
            //     description: "可以通过唯一的 key 来更新内容。",

            //     div {
            //         style: "display: flex; gap: 8px; flex-wrap: wrap;",

            //         Button {
            //             onclick: move |_| {
            //                 let key = "updatable";
            //                 MessageManager::new().loading_with_key("Loading...", key);

            //                 spawn(async move {
            //                     gloo_timers::future::TimeoutFuture::new(1000).await;
            //                     MessageManager::new().success_with_key("Loaded!", key);
            //                 });
            //             },
            //             "Open the message box"
            //         }
            //     }
            // }

            // API 文档
            ApiDocumentation {
                component_name: "Message",
                props: vec![
                    PropDoc {
                        name: "content".to_string(),
                        prop_type: "String".to_string(),
                        default: "-".to_string(),
                        description: "提示内容".to_string(),
                    },
                    PropDoc {
                        name: "duration".to_string(),
                        prop_type: "u32".to_string(),
                        default: "3".to_string(),
                        description: "自动关闭的延时，单位秒。设为 0 时不自动关闭".to_string(),
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
                        description: "当前提示的唯一标志".to_string(),
                    },
                    PropDoc {
                        name: "style".to_string(),
                        prop_type: "String".to_string(),
                        default: "-".to_string(),
                        description: "自定义内联样式".to_string(),
                    },
                    PropDoc {
                        name: "class_name".to_string(),
                        prop_type: "String".to_string(),
                        default: "-".to_string(),
                        description: "自定义 CSS class".to_string(),
                    },
                    PropDoc {
                        name: "on_close".to_string(),
                        prop_type: "Function".to_string(),
                        default: "-".to_string(),
                        description: "关闭时触发的回调函数".to_string(),
                    },
                ]
            }
        }
    }
}

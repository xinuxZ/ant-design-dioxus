#![allow(non_snake_case)]
//!
//! 展示 Modal 组件的各种用法和样式

use crate::common::*;
use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

/// Modal 组件演示
#[component]
pub fn ModalDemo() -> Element {
    let mut visible = use_signal(|| false);
    let mut confirm_loading = use_signal(|| false);
    let mut modal_text = use_signal(|| "Content of the modal".to_string());

    rsx! {
        div {
            class: "component-showcase",

            h1 {
                style: "margin: 0 0 16px 0; color: #262626; font-size: 28px;",
                "Modal 对话框"
            }

            p {
                style: "margin: 0 0 24px 0; color: #666; font-size: 14px;",
                "模态对话框。"
            }

            // 基础用法
            DemoSection {
                title: "基础用法",
                description: "第一个对话框。",

                div {
                    style: "display: flex; gap: 8px; flex-wrap: wrap;",

                    Button {
                        button_type: ButtonType::Primary,
                        onclick: move |_| {
                            visible.set(true);
                        },
                        "Open Modal"
                    }

                    Modal {
                        title: "Basic Modal",
                        open: visible(),
                        on_ok: move |_| {
                            visible.set(false);
                        },
                        on_cancel: move |_| {
                            visible.set(false);
                        },

                        p { "Some contents..." }
                        p { "Some contents..." }
                        p { "Some contents..." }
                    }
                }
            }

            // 异步关闭
            DemoSection {
                title: "异步关闭",
                description: "点击确定后异步关闭对话框，例如提交表单。",

                div {
                    style: "display: flex; gap: 8px; flex-wrap: wrap;",

                    Button {
                        button_type: ButtonType::Primary,
                        onclick: move |_| {
                            modal_text.set("Content of the modal".to_string());
                            visible.set(true);
                        },
                        "Open Modal with async logic"
                    }

                    Modal {
                        title: "Title",
                        open: visible(),
                        confirm_loading: confirm_loading(),
                        on_ok: move |_| {
                            modal_text.set("The modal will be closed after two seconds".to_string());
                            confirm_loading.set(true);

                            spawn(async move {
                                gloo_timers::future::TimeoutFuture::new(2000).await;
                                visible.set(false);
                                confirm_loading.set(false);
                            });
                        },
                        on_cancel: move |_| {
                            visible.set(false);
                        },

                        p { "{modal_text()}" }
                    }
                }
            }

            // 自定义页脚
            DemoSection {
                title: "自定义页脚",
                description: "更复杂的例子，自定义了页脚的按钮，点击提交后进入 loading 状态。",

                div {
                    style: "display: flex; gap: 8px; flex-wrap: wrap;",

                    Button {
                        button_type: ButtonType::Primary,
                        onclick: move |_| {
                            visible.set(true);
                        },
                        "Open Modal with customized footer"
                    }

                    Modal {
                        title: "Title",
                        open: visible(),
                        footer: rsx! {
                            div {
                                style: "text-align: right;",

                                Button {
                                    onclick: move |_| {
                                        visible.set(false);
                                    },
                                    "Return"
                                }

                                Button {
                                    button_type: ButtonType::Primary,
                                    loading: confirm_loading(),
                                    onclick: move |_| {
                                        confirm_loading.set(true);

                                        spawn(async move {
                                            gloo_timers::future::TimeoutFuture::new(2000).await;
                                            visible.set(false);
                                            confirm_loading.set(false);
                                        });
                                    },
                                    "Submit"
                                }
                            }
                        },

                        p { "Some contents..." }
                        p { "Some contents..." }
                        p { "Some contents..." }
                        p { "Some contents..." }
                        p { "Some contents..." }
                    }
                }
            }

            // 信息提示
            DemoSection {
                title: "信息提示",
                description: "各种类型的信息提示，只提供一个按钮用于关闭。",

                div {
                    style: "display: flex; gap: 8px; flex-wrap: wrap;",

                    Button {
                        onclick: move |_| {
                            Modal::info(ModalConfig {
                                title: "This is a notification message".to_string(),
                                content: "some messages...some messages...".to_string(),
                                ..Default::default()
                            });
                        },
                        "Info"
                    }

                    Button {
                        onclick: move |_| {
                            Modal::success(ModalConfig {
                                title: "This is a success message".to_string(),
                                content: "some messages...some messages...".to_string(),
                                ..Default::default()
                            });
                        },
                        "Success"
                    }

                    Button {
                        onclick: move |_| {
                            Modal::error(ModalConfig {
                                title: "This is an error message".to_string(),
                                content: "some messages...some messages...".to_string(),
                                ..Default::default()
                            });
                        },
                        "Error"
                    }

                    Button {
                        onclick: move |_| {
                            Modal::warning(ModalConfig {
                                title: "This is a warning message".to_string(),
                                content: "some messages...some messages...".to_string(),
                                ..Default::default()
                            });
                        },
                        "Warning"
                    }
                }
            }

            // 确认对话框
            DemoSection {
                title: "确认对话框",
                description: "使用 confirm() 可以快捷地弹出确认框。onCancel/onOk 返回 promise 可以延迟关闭。",

                div {
                    style: "display: flex; gap: 8px; flex-wrap: wrap;",

                    Button {
                        onclick: move |_| {
                            Modal::confirm(ModalConfig {
                                title: "Do you Want to delete these items?".to_string(),
                                content: "Some descriptions".to_string(),
                                on_ok: Some(Box::new(|| {
                                    web_sys::console::log_1(&"OK clicked".into());
                                })),
                                on_cancel: Some(Box::new(|| {
                                    web_sys::console::log_1(&"Cancel clicked".into());
                                })),
                                ..Default::default()
                            });
                        },
                        "Confirm"
                    }

                    Button {
                        onclick: move |_| {
                            Modal::confirm(ModalConfig {
                                title: "Do you Want to delete these items?".to_string(),
                                content: "When clicked the OK button, this dialog will be closed after 1 second".to_string(),
                                on_ok: Some(Box::new(|| {
                                    spawn(async {
                                        gloo_timers::future::TimeoutFuture::new(1000).await;
                                        web_sys::console::log_1(&"Async operation completed".into());
                                    });
                                })),
                                ..Default::default()
                            });
                        },
                        "With async logic"
                    }
                }
            }

            // API 文档
            ApiDocumentation {
                component_name: "Modal",
                props: vec![
                    PropDoc {
                        name: "title".to_string(),
                        prop_type: "String".to_string(),
                        default: "-".to_string(),
                        description: "标题".to_string(),
                    },
                    PropDoc {
                        name: "open".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "对话框是否可见".to_string(),
                    },
                    PropDoc {
                        name: "confirm_loading".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "确定按钮 loading".to_string(),
                    },
                    PropDoc {
                        name: "centered".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "垂直居中展示 Modal".to_string(),
                    },
                    PropDoc {
                        name: "closable".to_string(),
                        prop_type: "bool".to_string(),
                        default: "true".to_string(),
                        description: "是否显示右上角的关闭按钮".to_string(),
                    },
                    PropDoc {
                        name: "destroy_on_close".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "关闭时销毁 Modal 里的子元素".to_string(),
                    },
                    PropDoc {
                        name: "footer".to_string(),
                        prop_type: "Element".to_string(),
                        default: "-".to_string(),
                        description: "底部内容，当不需要默认底部按钮时，可以设为 footer={null}".to_string(),
                    },
                    PropDoc {
                        name: "mask".to_string(),
                        prop_type: "bool".to_string(),
                        default: "true".to_string(),
                        description: "是否展示遮罩".to_string(),
                    },
                    PropDoc {
                        name: "mask_closable".to_string(),
                        prop_type: "bool".to_string(),
                        default: "true".to_string(),
                        description: "点击蒙层是否允许关闭".to_string(),
                    },
                ]
            }
        }
    }
}

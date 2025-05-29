#![allow(non_snake_case)]
//!
//! 展示 Result 组件的各种用法和样式

use crate::common::*;
use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

/// Result 组件演示
#[component]
pub fn ResultDemo() -> Element {
    rsx! {
        div {
            class: "component-showcase",

            h1 {
                style: "margin: 0 0 16px 0; color: #262626; font-size: 28px;",
                "Result 结果"
            }

            p {
                style: "margin: 0 0 24px 0; color: #666; font-size: 14px;",
                "用于反馈一系列操作任务的处理结果。"
            }

            // 成功
            DemoSection {
                title: "成功",
                description: "成功的结果。",

                div {
                    style: "padding: 24px; background: #fafafa; border-radius: 6px;",

                    Result {
                        status: ResultStatus::Success,
                        title: "Successfully Purchased Cloud Server ECS!",
                        sub_title: "Order number: 2017182818828182881 Cloud server configuration takes 1-5 minutes, please wait.",
                        extra: rsx! {
                            div {
                                style: "display: flex; gap: 8px; justify-content: center;",
                                Button {
                                    button_type: ButtonType::Primary,
                                    "Go Console"
                                }
                                Button {
                                    "Buy Again"
                                }
                            }
                        }
                    }
                }
            }

            // 信息提示
            DemoSection {
                title: "信息提示",
                description: "展示处理结果。",

                div {
                    style: "padding: 24px; background: #fafafa; border-radius: 6px;",

                    Result {
                        title: "Your operation has been executed",
                        extra: rsx! {
                            Button {
                                button_type: ButtonType::Primary,
                                "Go Console"
                            }
                        }
                    }
                }
            }

            // 警告
            DemoSection {
                title: "警告",
                description: "警告类型的结果。",

                div {
                    style: "padding: 24px; background: #fafafa; border-radius: 6px;",

                    Result {
                        status: ResultStatus::Warning,
                        title: "There are some problems with your operation.",
                        extra: rsx! {
                            Button {
                                button_type: ButtonType::Primary,
                                "Go Console"
                            }
                        }
                    }
                }
            }

            // 403
            DemoSection {
                title: "403",
                description: "你没有此页面的访问权限。",

                div {
                    style: "padding: 24px; background: #fafafa; border-radius: 6px;",

                    Result {
                        status: ResultStatus::Forbidden,
                        title: "403",
                        sub_title: "Sorry, you are not authorized to access this page.",
                        extra: rsx! {
                            Button {
                                button_type: ButtonType::Primary,
                                "Back Home"
                            }
                        }
                    }
                }
            }

            // 404
            DemoSection {
                title: "404",
                description: "此页面未找到。",

                div {
                    style: "padding: 24px; background: #fafafa; border-radius: 6px;",

                    Result {
                        status: ResultStatus::NotFound,
                        title: "404",
                        sub_title: "Sorry, the page you visited does not exist.",
                        extra: rsx! {
                            Button {
                                button_type: ButtonType::Primary,
                                "Back Home"
                            }
                        }
                    }
                }
            }

            // 500
            DemoSection {
                title: "500",
                description: "服务器发生了错误。",

                div {
                    style: "padding: 24px; background: #fafafa; border-radius: 6px;",

                    Result {
                        status: ResultStatus::ServerError,
                        title: "500",
                        sub_title: "Sorry, something went wrong.",
                        extra: rsx! {
                            Button {
                                button_type: ButtonType::Primary,
                                "Back Home"
                            }
                        }
                    }
                }
            }

            // 错误
            DemoSection {
                title: "错误",
                description: "复杂的错误反馈。",

                div {
                    style: "padding: 24px; background: #fafafa; border-radius: 6px;",

                    Result {
                        status: ResultStatus::Error,
                        title: "Submission Failed",
                        sub_title: "Please check and modify the following information before resubmitting.",
                        extra: rsx! {
                            div {
                                style: "display: flex; gap: 8px; justify-content: center;",
                                Button {
                                    button_type: ButtonType::Primary,
                                    "Go Console"
                                }
                                Button {
                                    "Buy Again"
                                }
                            }
                        },
                        children: rsx! {
                            div {
                                style: "background: #fafafa; padding: 24px; border-radius: 6px; margin-top: 16px;",
                                h4 {
                                    style: "margin: 0 0 16px 0; color: #262626;",
                                    "The content you submitted has the following error:"
                                }
                                div {
                                    style: "color: #666;",
                                    p {
                                        style: "margin: 8px 0;",
                                        Icon {
                                            icon_type: IconType::CloseCircleOutlined,
                                            style: "color: #ff4d4f; margin-right: 8px;"
                                        }
                                        "Your account has been frozen. "
                                        a {
                                            href: "#",
                                            "Thaw immediately >"
                                        }
                                    }
                                    p {
                                        style: "margin: 8px 0;",
                                        Icon {
                                            icon_type: IconType::CloseCircleOutlined,
                                            style: "color: #ff4d4f; margin-right: 8px;"
                                        }
                                        "Your account is not yet eligible to apply. "
                                        a {
                                            href: "#",
                                            "Apply Unlock >"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // 自定义 icon
            DemoSection {
                title: "自定义 icon",
                description: "自定义 icon。",

                div {
                    style: "padding: 24px; background: #fafafa; border-radius: 6px;",

                    Result {
                        icon: rsx! {
                            Icon {
                                icon_type: IconType::SmileOutlined,
                                style: "color: #1890ff; font-size: 72px;"
                            }
                        },
                        title: "Great, we have done all the operations!",
                        extra: rsx! {
                            Button {
                                button_type: ButtonType::Primary,
                                "Next"
                            }
                        }
                    }
                }
            }

            // API 文档
            ApiDocumentation {
                component_name: "Result",
                props: vec![
                    PropDoc {
                        name: "extra".to_string(),
                        prop_type: "Element".to_string(),
                        default: "-".to_string(),
                        description: "操作区".to_string(),
                    },
                    PropDoc {
                        name: "icon".to_string(),
                        prop_type: "Element".to_string(),
                        default: "-".to_string(),
                        description: "自定义 icon".to_string(),
                    },
                    PropDoc {
                        name: "status".to_string(),
                        prop_type: "String".to_string(),
                        default: "info".to_string(),
                        description: "结果的状态，决定图标和颜色，可选：success | error | info | warning | 404 | 403 | 500".to_string(),
                    },
                    PropDoc {
                        name: "sub_title".to_string(),
                        prop_type: "String | Element".to_string(),
                        default: "-".to_string(),
                        description: "subTitle 文字".to_string(),
                    },
                    PropDoc {
                        name: "title".to_string(),
                        prop_type: "String | Element".to_string(),
                        default: "-".to_string(),
                        description: "title 文字".to_string(),
                    },
                ]
            }
        }
    }
}

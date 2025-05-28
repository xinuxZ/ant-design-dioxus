//! Alert 组件示例
//!
//! 展示 Alert 组件的各种用法，包括不同类型、可关闭、带描述、自定义图标等。

use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn AlertDemo() -> Element {
    let mut show_closable = use_signal(|| true);
    let mut show_custom = use_signal(|| true);

    rsx! {
        // style { include_str!("../src/components/alert/style.css") }

        div {
            style: "padding: 20px; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;",

            h1 { "Alert 组件示例" }

            // 基本用法
            section {
                style: "margin-bottom: 40px;",
                h2 { "基本用法" }
                p { "最简单的用法，适用于简短的警告提示。" }

                Alert {
                    message: "这是一条成功提示",
                    alert_type: AlertType::Success
                }
            }

            // 四种样式
            section {
                style: "margin-bottom: 40px;",
                h2 { "四种样式" }
                p { "共有四种样式 success、info、warning、error。" }

                Alert {
                    message: "Success Text",
                    alert_type: AlertType::Success
                }

                Alert {
                    message: "Info Text",
                    alert_type: AlertType::Info
                }

                Alert {
                    message: "Warning Text",
                    alert_type: AlertType::Warning
                }

                Alert {
                    message: "Error Text",
                    alert_type: AlertType::Error
                }
            }

            // 可关闭的警告提示
            section {
                style: "margin-bottom: 40px;",
                h2 { "可关闭的警告提示" }
                p { "显示关闭按钮，点击可关闭警告提示。" }

                if show_closable() {
                    Alert {
                        message: "Warning Text Warning Text Warning TextW arning Text Warning Text Warning TextWarning Text",
                        alert_type: AlertType::Warning,
                        closable: true,
                        on_close: move |_| show_closable.set(false)
                    }
                }

                button {
                    style: "margin-top: 10px; padding: 8px 16px; background: #1677ff; color: white; border: none; border-radius: 4px; cursor: pointer;",
                    onclick: move |_| show_closable.set(true),
                    "重新显示"
                }
            }

            // 含有辅助性文字介绍
            section {
                style: "margin-bottom: 40px;",
                h2 { "含有辅助性文字介绍" }
                p { "含有辅助性文字介绍的警告提示。" }

                Alert {
                    message: "Success Tips",
                    description: "Detailed description and advice about successful copywriting.",
                    alert_type: AlertType::Success,
                    show_icon: true
                }

                Alert {
                    message: "Informational Notes",
                    description: "Additional description and information about copywriting.",
                    alert_type: AlertType::Info,
                    show_icon: true
                }

                Alert {
                    message: "Warning",
                    description: "This is a warning notice about copywriting.",
                    alert_type: AlertType::Warning,
                    show_icon: true,
                    closable: true
                }

                Alert {
                    message: "Error",
                    description: "This is an error message about copywriting.",
                    alert_type: AlertType::Error,
                    show_icon: true
                }
            }

            // 图标
            section {
                style: "margin-bottom: 40px;",
                h2 { "图标" }
                p { "可口的图标让信息类型更加醒目。" }

                Alert {
                    message: "Success Tips",
                    alert_type: AlertType::Success,
                    show_icon: true
                }

                Alert {
                    message: "Informational Notes",
                    alert_type: AlertType::Info,
                    show_icon: true
                }

                Alert {
                    message: "Warning",
                    alert_type: AlertType::Warning,
                    show_icon: true
                }

                Alert {
                    message: "Error",
                    alert_type: AlertType::Error,
                    show_icon: true
                }
            }

            // 自定义关闭
            section {
                style: "margin-bottom: 40px;",
                h2 { "自定义关闭" }
                p { "可以自定义关闭，自定义的文字会替换原先的关闭 Icon。" }

                if show_custom() {
                    Alert {
                        message: "Info Text",
                        alert_type: AlertType::Info,
                        closable: true,
                        close_text: "Close Now",
                        on_close: move |_| show_custom.set(false)
                    }
                }

                button {
                    style: "margin-top: 10px; padding: 8px 16px; background: #1677ff; color: white; border: none; border-radius: 4px; cursor: pointer;",
                    onclick: move |_| show_custom.set(true),
                    "重新显示"
                }
            }

            // 顶部公告
            section {
                style: "margin-bottom: 40px;",
                h2 { "顶部公告" }
                p { "页面顶部通告形式，默认有图标且 type 为 warning。" }

                Alert {
                    message: "Warning text",
                    banner: true,
                    closable: true
                }

                Alert {
                    message: "Very long warning text warning text text text text text text text",
                    banner: true,
                    closable: true
                }

                Alert {
                    message: "Warning text without icon",
                    banner: true,
                    show_icon: false
                }

                Alert {
                    message: "Error text",
                    banner: true,
                    alert_type: AlertType::Error
                }
            }

            // 操作
            section {
                style: "margin-bottom: 40px;",
                h2 { "操作" }
                p { "可以在右上角自定义操作项。" }

                Alert {
                    message: "Success Tips",
                    alert_type: AlertType::Success,
                    action: rsx! {
                        button {
                            style: "padding: 4px 8px; background: transparent; border: 1px solid #52c41a; color: #52c41a; border-radius: 4px; cursor: pointer; font-size: 12px;",
                            "UNDO"
                        }
                    },
                    closable: true
                }

                Alert {
                    message: "Error Text",
                    description: "Error Description Error Description Error Description Error Description",
                    alert_type: AlertType::Error,
                    action: rsx! {
                        button {
                            style: "padding: 4px 8px; background: #ff4d4f; border: none; color: white; border-radius: 4px; cursor: pointer; font-size: 12px; margin-right: 8px;",
                            "Detail"
                        }
                    }
                }
            }

            // 自定义图标
            section {
                style: "margin-bottom: 40px;",
                h2 { "自定义图标" }
                p { "可口的图标让信息类型更加醒目。" }

                Alert {
                    message: "showIcon = false",
                    alert_type: AlertType::Success,
                    show_icon: false
                }

                Alert {
                    message: "Success Tips",
                    alert_type: AlertType::Success,
                    show_icon: true,
                    icon: rsx! {
                        span {
                            style: "color: #52c41a; font-size: 16px;",
                            "😊"
                        }
                    }
                }

                Alert {
                    message: "Informational Notes",
                    alert_type: AlertType::Info,
                    show_icon: true,
                    icon: rsx! {
                        span {
                            style: "color: #1677ff; font-size: 16px;",
                            "💡"
                        }
                    }
                }

                Alert {
                    message: "Warning",
                    alert_type: AlertType::Warning,
                    show_icon: true,
                    icon: rsx! {
                        span {
                            style: "color: #faad14; font-size: 16px;",
                            "🔔"
                        }
                    }
                }

                Alert {
                    message: "Error",
                    alert_type: AlertType::Error,
                    show_icon: true,
                    icon: rsx! {
                        span {
                            style: "color: #ff4d4f; font-size: 16px;",
                            "🚫"
                        }
                    }
                }
            }

            // 实际应用场景
            section {
                h2 { "实际应用场景" }
                p { "在实际项目中的应用示例。" }

                div {
                    style: "display: grid; gap: 20px; margin-top: 20px;",

                    // 表单验证错误
                    div {
                        h4 { "表单验证" }
                        Alert {
                            message: "表单验证失败",
                            description: "请检查以下字段：用户名不能为空，密码长度至少8位，邮箱格式不正确。",
                            alert_type: AlertType::Error,
                            show_icon: true,
                            closable: true
                        }
                    }

                    // 系统通知
                    div {
                        h4 { "系统通知" }
                        Alert {
                            message: "系统维护通知",
                            description: "系统将于今晚 23:00-01:00 进行维护升级，期间可能影响部分功能使用，请提前做好相关准备。",
                            alert_type: AlertType::Warning,
                            show_icon: true,
                            banner: true
                        }
                    }

                    // 操作成功
                    div {
                        h4 { "操作反馈" }
                        Alert {
                            message: "保存成功",
                            description: "您的设置已成功保存，新的配置将在下次登录时生效。",
                            alert_type: AlertType::Success,
                            show_icon: true,
                            action: rsx! {
                                button {
                                    style: "padding: 4px 12px; background: transparent; border: 1px solid #52c41a; color: #52c41a; border-radius: 4px; cursor: pointer; font-size: 12px;",
                                    "查看详情"
                                }
                            }
                        }
                    }

                    // 信息提示
                    div {
                        h4 { "信息提示" }
                        Alert {
                            message: "新功能上线",
                            description: "我们刚刚发布了全新的数据分析功能，现在您可以更直观地查看业务数据趋势。",
                            alert_type: AlertType::Info,
                            show_icon: true,
                            action: rsx! {
                                div {
                                    style: "display: flex; gap: 8px;",
                                    button {
                                        style: "padding: 4px 12px; background: #1677ff; border: none; color: white; border-radius: 4px; cursor: pointer; font-size: 12px;",
                                        "立即体验"
                                    }
                                    button {
                                        style: "padding: 4px 12px; background: transparent; border: 1px solid #1677ff; color: #1677ff; border-radius: 4px; cursor: pointer; font-size: 12px;",
                                        "稍后提醒"
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

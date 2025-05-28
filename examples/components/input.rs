//! Input 输入框组件示例
//!
//! 演示 Input 组件的各种用法和功能

use dioxus::prelude::*;

// 引入组件样式
// const STYLE: &str = include_str!("../src/components/input/style.css");

// 引入 Input 组件
use ant_design_dioxus::prelude::*;

#[component]
pub fn InputDemo() -> Element {
    // 基础用法状态
    let mut basic_value = use_signal(String::new);

    // 尺寸变体状态
    let mut large_value = use_signal(String::new);
    let mut middle_value = use_signal(String::new);
    let mut small_value = use_signal(String::new);

    // 状态变体
    let mut normal_value = use_signal(String::new);
    let mut error_value = use_signal(|| "错误状态".to_string());
    let mut warning_value = use_signal(|| "警告状态".to_string());

    // 功能演示
    let mut disabled_value = use_signal(|| "禁用状态".to_string());
    let mut readonly_value = use_signal(|| "只读状态".to_string());
    let mut clear_value = use_signal(|| "可清除内容".to_string());
    let mut count_value = use_signal(String::new);

    // 前缀后缀
    let mut prefix_value = use_signal(String::new);
    let mut suffix_value = use_signal(String::new);

    // 输入框组
    let mut addon_before_value = use_signal(String::new);
    let mut addon_after_value = use_signal(String::new);

    // 密码输入
    let mut password_value = use_signal(String::new);
    let mut show_password = use_signal(|| false);

    // 搜索框
    let mut search_value = use_signal(String::new);

    // 表单场景
    let mut username = use_signal(String::new);
    let mut email = use_signal(String::new);
    let mut phone = use_signal(String::new);

    rsx! {
        // style { {STYLE} }

        div {
            style: "padding: 24px; max-width: 800px; margin: 0 auto; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;",

            h1 {
                style: "color: #262626; margin-bottom: 24px;",
                "Input 输入框组件示例"
            }

            // 基础用法
            section {
                style: "margin-bottom: 32px;",

                h2 {
                    style: "color: #262626; margin-bottom: 16px; font-size: 18px;",
                    "基础用法"
                }

                div {
                    style: "margin-bottom: 16px;",

                    Input {
                        value: basic_value(),
                        placeholder: "请输入内容",
                        on_change: move |v| basic_value.set(v)
                    }
                }

                p {
                    style: "color: #666; font-size: 14px;",
                    "当前输入值: {basic_value()}"
                }
            }

            // 尺寸变体
            section {
                style: "margin-bottom: 32px;",

                h2 {
                    style: "color: #262626; margin-bottom: 16px; font-size: 18px;",
                    "三种尺寸"
                }

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    div {
                        Input {
                            size: InputSize::Large,
                            value: large_value(),
                            placeholder: "大尺寸输入框",
                            on_change: move |v| large_value.set(v)
                        }
                    }

                    div {
                        Input {
                            size: InputSize::Middle,
                            value: middle_value(),
                            placeholder: "中等尺寸输入框（默认）",
                            on_change: move |v| middle_value.set(v)
                        }
                    }

                    div {
                        Input {
                            size: InputSize::Small,
                            value: small_value(),
                            placeholder: "小尺寸输入框",
                            on_change: move |v| small_value.set(v)
                        }
                    }
                }
            }

            // 状态变体
            section {
                style: "margin-bottom: 32px;",

                h2 {
                    style: "color: #262626; margin-bottom: 16px; font-size: 18px;",
                    "输入框状态"
                }

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    div {
                        Input {
                            status: InputStatus::Normal,
                            value: normal_value(),
                            placeholder: "正常状态",
                            on_change: move |v| normal_value.set(v)
                        }
                    }

                    div {
                        Input {
                            status: InputStatus::Error,
                            value: error_value(),
                            placeholder: "错误状态",
                            on_change: move |v| error_value.set(v)
                        }
                    }

                    div {
                        Input {
                            status: InputStatus::Warning,
                            value: warning_value(),
                            placeholder: "警告状态",
                            on_change: move |v| warning_value.set(v)
                        }
                    }
                }
            }

            // 功能演示
            section {
                style: "margin-bottom: 32px;",

                h2 {
                    style: "color: #262626; margin-bottom: 16px; font-size: 18px;",
                    "功能演示"
                }

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    div {
                        Input {
                            value: disabled_value(),
                            placeholder: "禁用状态",
                            disabled: true,
                            on_change: move |v| disabled_value.set(v)
                        }
                    }

                    div {
                        Input {
                            value: readonly_value(),
                            placeholder: "只读状态",
                            readonly: true,
                            on_change: move |v| readonly_value.set(v)
                        }
                    }

                    div {
                        Input {
                            value: clear_value(),
                            placeholder: "可清除内容",
                            allow_clear: true,
                            on_change: move |v| clear_value.set(v),
                            on_clear: move |_| {
                                println!("输入框已清除");
                            }
                        }
                    }

                    div {
                        Input {
                            value: count_value(),
                            placeholder: "显示字数统计",
                            show_count: true,
                            max_length: Some(50),
                            on_change: move |v| count_value.set(v)
                        }
                    }
                }
            }

            // 前缀和后缀
            section {
                style: "margin-bottom: 32px;",

                h2 {
                    style: "color: #262626; margin-bottom: 16px; font-size: 18px;",
                    "前缀和后缀"
                }

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    div {
                        Input {
                            value: prefix_value(),
                            placeholder: "带前缀的输入框",
                            prefix: rsx! {
                                span {
                                    style: "color: #1890ff;",
                                    "👤"
                                }
                            },
                            on_change: move |v| prefix_value.set(v)
                        }
                    }

                    div {
                        Input {
                            value: suffix_value(),
                            placeholder: "带后缀的输入框",
                            suffix: rsx! {
                                span {
                                    style: "color: #1890ff; cursor: pointer;",
                                    "🔍"
                                }
                            },
                            on_change: move |v| suffix_value.set(v)
                        }
                    }
                }
            }

            // 输入框组
            section {
                style: "margin-bottom: 32px;",

                h2 {
                    style: "color: #262626; margin-bottom: 16px; font-size: 18px;",
                    "输入框组合"
                }

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    div {
                        Input {
                            value: addon_before_value(),
                            placeholder: "带前置标签",
                            addon_before: rsx! {
                                span {
                                    style: "color: #666;",
                                    "Http://"
                                }
                            },
                            on_change: move |v| addon_before_value.set(v)
                        }
                    }

                    div {
                        Input {
                            value: addon_after_value(),
                            placeholder: "带后置标签",
                            addon_after: rsx! {
                                span {
                                    style: "color: #666;",
                                    ".com"
                                }
                            },
                            on_change: move |v| addon_after_value.set(v)
                        }
                    }
                }
            }

            // 密码输入
            section {
                style: "margin-bottom: 32px;",

                h2 {
                    style: "color: #262626; margin-bottom: 16px; font-size: 18px;",
                    "密码输入"
                }

                div {
                    Input {
                        input_type: if show_password() { "text" } else { "password" },
                        value: password_value(),
                        placeholder: "请输入密码",
                        suffix: rsx! {
                            span {
                                style: "color: #1890ff; cursor: pointer;",
                                onclick: move |_| show_password.set(!show_password()),
                                if show_password() { "🙈" } else { "👁" }
                            }
                        },
                        on_change: move |v| password_value.set(v)
                    }
                }
            }

            // 搜索框
            section {
                style: "margin-bottom: 32px;",

                h2 {
                    style: "color: #262626; margin-bottom: 16px; font-size: 18px;",
                    "搜索框"
                }

                div {
                    Input {
                        value: search_value(),
                        placeholder: "搜索内容",
                        suffix: rsx! {
                            button {
                                style: "background: #1890ff; color: white; border: none; padding: 4px 8px; border-radius: 4px; cursor: pointer;",
                                onclick: move |_| {
                                    println!("搜索: {}", search_value());
                                },
                                "搜索"
                            }
                        },
                        on_change: move |v| search_value.set(v),
                        on_press_enter: move |_| {
                            println!("按回车搜索: {}", search_value());
                        }
                    }
                }
            }

            // 表单场景
            section {
                style: "margin-bottom: 32px;",

                h2 {
                    style: "color: #262626; margin-bottom: 16px; font-size: 18px;",
                    "表单场景"
                }

                div {
                    style: "display: flex; flex-direction: column; gap: 16px; max-width: 400px;",

                    div {
                        label {
                            style: "display: block; margin-bottom: 4px; color: #262626; font-weight: 500;",
                            "用户名"
                        }
                        Input {
                            value: username(),
                            placeholder: "请输入用户名",
                            prefix: rsx! {
                                span {
                                    style: "color: #1890ff;",
                                    "👤"
                                }
                            },
                            on_change: move |v| username.set(v)
                        }
                    }

                    div {
                        label {
                            style: "display: block; margin-bottom: 4px; color: #262626; font-weight: 500;",
                            "邮箱地址"
                        }
                        Input {
                            input_type: "email",
                            value: email(),
                            placeholder: "请输入邮箱地址",
                            prefix: rsx! {
                                span {
                                    style: "color: #1890ff;",
                                    "📧"
                                }
                            },
                            on_change: move |v| email.set(v)
                        }
                    }

                    div {
                        label {
                            style: "display: block; margin-bottom: 4px; color: #262626; font-weight: 500;",
                            "手机号码"
                        }
                        Input {
                            input_type: "tel",
                            value: phone(),
                            placeholder: "请输入手机号码",
                            prefix: rsx! {
                                span {
                                    style: "color: #1890ff;",
                                    "📱"
                                }
                            },
                            max_length: Some(11),
                            show_count: true,
                            on_change: move |v| phone.set(v)
                        }
                    }

                    button {
                        style: "background: #1890ff; color: white; border: none; padding: 8px 16px; border-radius: 6px; cursor: pointer; margin-top: 16px;",
                        onclick: move |_| {
                            println!("提交表单: 用户名={}, 邮箱={}, 手机={}", username(), email(), phone());
                        },
                        "提交"
                    }
                }
            }

            // 无边框样式
            section {
                style: "margin-bottom: 32px;",

                h2 {
                    style: "color: #262626; margin-bottom: 16px; font-size: 18px;",
                    "无边框样式"
                }

                div {
                    style: "background: #f5f5f5; padding: 16px; border-radius: 8px;",

                    Input {
                        value: basic_value(),
                        placeholder: "无边框输入框",
                        bordered: false,
                        on_change: move |v| basic_value.set(v)
                    }
                }
            }
        }
    }
}

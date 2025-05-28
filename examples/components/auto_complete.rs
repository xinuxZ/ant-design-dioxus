//!
//! 展示 AutoComplete 组件的各种用法和样式

use crate::common::*;
use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

/// AutoComplete 组件演示
#[component]
pub fn AutoCompleteDemo() -> Element {
    let mut value = use_signal(|| String::new());
    let mut custom_value = use_signal(|| String::new());
    let mut search_value = use_signal(|| String::new());
    let mut selected_option = use_signal(|| String::new());

    // 基础选项数据
    let basic_options = vec![
        AutoCompleteOption::new("apple", "苹果"),
        AutoCompleteOption::new("banana", "香蕉"),
        AutoCompleteOption::new("orange", "橙子"),
        AutoCompleteOption::new("grape", "葡萄"),
        AutoCompleteOption::new("watermelon", "西瓜"),
        AutoCompleteOption::new("strawberry", "草莓"),
    ];

    // 邮箱选项数据
    let email_options = vec![
        AutoCompleteOption::new("@gmail.com", "@gmail.com"),
        AutoCompleteOption::new("@163.com", "@163.com"),
        AutoCompleteOption::new("@qq.com", "@qq.com"),
        AutoCompleteOption::new("@outlook.com", "@outlook.com"),
        AutoCompleteOption::new("@hotmail.com", "@hotmail.com"),
    ];

    // 动态过滤选项
    let filtered_options = basic_options
        .iter()
        .filter(|option| {
            search_value().is_empty()
                || option
                    .label
                    .to_lowercase()
                    .contains(&search_value().to_lowercase())
                || option
                    .value
                    .to_lowercase()
                    .contains(&search_value().to_lowercase())
        })
        .cloned()
        .collect::<Vec<_>>();

    rsx! {
        div {
            class: "component-showcase",

            h1 {
                style: "margin: 0 0 16px 0; color: #262626; font-size: 28px;",
                "AutoComplete 自动完成"
            }

            p {
                style: "margin: 0 0 24px 0; color: #666; font-size: 14px;",
                "输入框自动完成功能。需要一个输入框而不是选择器，需要输入建议/辅助提示。"
            }

            // 基础用法
            DemoSection {
                title: "基础用法",
                description: "基本使用，通过 options 设置自动完成的数据源。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    AutoComplete {
                        value: value(),
                        placeholder: "请输入水果名称",
                        options: basic_options.clone(),
                        on_change: move |val: String| {
                            value.set(val.clone());
                            web_sys::console::log_1(&format!("输入值变化: {}", val).into());
                        },
                        on_select: move |option: AutoCompleteOption| {
                            selected_option.set(option.value.clone());
                            web_sys::console::log_1(&format!("选择了: {}", option.value).into());
                        }
                    }

                    div {
                        style: "color: #666; font-size: 14px;",
                        "当前输入: {value}"
                    }

                    if !selected_option().is_empty() {
                        div {
                            style: "color: #1890ff; font-size: 14px;",
                            "最后选择: {selected_option}"
                        }
                    }
                }
            }

            // 自定义输入
            DemoSection {
                title: "自定义输入",
                description: "可以自定义输入。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    AutoComplete {
                        value: custom_value(),
                        placeholder: "请输入任意内容",
                        options: vec![
                            AutoCompleteOption::new("option1", "选项 1"),
                            AutoCompleteOption::new("option2", "选项 2"),
                            AutoCompleteOption::new("option3", "选项 3"),
                        ],
                        allow_clear: true,
                        on_change: move |val: String| {
                            custom_value.set(val);
                        }
                    }

                    div {
                        style: "color: #666; font-size: 14px;",
                        "自定义输入值: {custom_value}"
                    }
                }
            }

            // 不同尺寸
            DemoSection {
                title: "三种尺寸",
                description: "我们为 AutoComplete 输入框定义了三种尺寸（大、默认、小），高度分别为 40px、32px 和 24px。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    div {
                        "大尺寸:"
                        AutoComplete {
                            placeholder: "大尺寸",
                            size: AutoCompleteSize::Large,
                            options: basic_options.clone(),
                            style: "margin-left: 8px; width: 200px;"
                        }
                    }

                    div {
                        "默认尺寸:"
                        AutoComplete {
                            placeholder: "默认尺寸",
                            size: AutoCompleteSize::Middle,
                            options: basic_options.clone(),
                            style: "margin-left: 8px; width: 200px;"
                        }
                    }

                    div {
                        "小尺寸:"
                        AutoComplete {
                            placeholder: "小尺寸",
                            size: AutoCompleteSize::Small,
                            options: basic_options.clone(),
                            style: "margin-left: 8px; width: 200px;"
                        }
                    }
                }
            }

            // 查找模式
            DemoSection {
                title: "查找模式",
                description: "根据输入项进行查找。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    AutoComplete {
                        value: search_value(),
                        placeholder: "请输入搜索内容",
                        options: filtered_options,
                        on_change: move |val: String| {
                            search_value.set(val);
                        },
                        on_search: move |val: String| {
                            web_sys::console::log_1(&format!("搜索: {}", val).into());
                        }
                    }

                    div {
                        style: "color: #666; font-size: 14px;",
                        "搜索关键词: {search_value}"
                    }
                }
            }

            // 状态
            DemoSection {
                title: "状态",
                description: "使用 status 为 AutoComplete 添加状态。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    div {
                        "错误状态:"
                        AutoComplete {
                            placeholder: "错误状态",
                            status: AutoCompleteStatus::Error,
                            options: basic_options.clone(),
                            style: "margin-left: 8px; width: 200px;"
                        }
                    }

                    div {
                        "警告状态:"
                        AutoComplete {
                            placeholder: "警告状态",
                            status: AutoCompleteStatus::Warning,
                            options: basic_options.clone(),
                            style: "margin-left: 8px; width: 200px;"
                        }
                    }
                }
            }

            // 禁用状态
            DemoSection {
                title: "禁用状态",
                description: "禁用状态。",

                AutoComplete {
                    placeholder: "禁用状态",
                    disabled: true,
                    options: basic_options.clone(),
                    style: "width: 200px;"
                }
            }

            // 邮箱后缀
            DemoSection {
                title: "邮箱后缀",
                description: "邮箱输入建议。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    AutoComplete {
                        placeholder: "请输入邮箱",
                        options: email_options,
                        on_change: move |val: String| {
                            web_sys::console::log_1(&format!("邮箱输入: {}", val).into());
                        }
                    }

                    div {
                        style: "color: #666; font-size: 12px;",
                        "提示: 输入 @ 符号后会显示邮箱后缀建议"
                    }
                }
            }

            // 带清除按钮
            DemoSection {
                title: "带清除按钮",
                description: "带移除图标的输入框，点击图标删除所有内容。",

                AutoComplete {
                    placeholder: "可清除的输入框",
                    allow_clear: true,
                    options: basic_options.clone(),
                    style: "width: 200px;"
                }
            }
        }
    }
}

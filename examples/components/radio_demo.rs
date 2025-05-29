use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn RadioDemo() -> Element {
    let mut basic_checked = use_signal(|| false);
    let mut group_value = use_signal(|| "apple".to_string());
    let mut size_value = use_signal(|| "medium".to_string());
    let mut disabled_value = use_signal(|| "option1".to_string());

    rsx! {
        div {
            style: "padding: 20px; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;",

            h1 { "Radio 单选框组件示例" }

            // 基本用法
            section {
                style: "margin-bottom: 32px;",
                h2 { "基本用法" }
                div {
                    style: "margin-bottom: 16px;",
                    Radio {
                        checked: basic_checked(),
                        onchange: move |checked| {
                            basic_checked.set(checked);
                        },
                        "Radio"
                    }
                }
                p { "当前状态: {basic_checked()}" }
            }

            // RadioGroup 基本用法
            section {
                style: "margin-bottom: 32px;",
                h2 { "RadioGroup 基本用法" }
                div {
                    style: "margin-bottom: 16px;",
                    RadioGroup {
                        value: group_value(),
                        onchange: move |value| {
                            group_value.set(value);
                        },
                        Radio { value: "apple", "Apple" }
                        Radio { value: "pear", "Pear" }
                        Radio { value: "orange", "Orange" }
                    }
                }
                p { "选中的值: {group_value()}" }
            }

            // 不同尺寸
            section {
                style: "margin-bottom: 32px;",
                h2 { "不同尺寸" }
                div {
                    style: "margin-bottom: 16px;",
                    h3 { "Large" }
                    RadioGroup {
                        value: size_value(),
                        size: "large",
                        onchange: move |value| {
                            size_value.set(value);
                        },
                        Radio { value: "large1", "Large Option 1" }
                        Radio { value: "large2", "Large Option 2" }
                    }
                }
                div {
                    style: "margin-bottom: 16px;",
                    h3 { "Medium (默认)" }
                    RadioGroup {
                        value: size_value(),
                        onchange: move |value| {
                            size_value.set(value);
                        },
                        Radio { value: "medium", "Medium Option 1" }
                        Radio { value: "medium2", "Medium Option 2" }
                    }
                }
                div {
                    style: "margin-bottom: 16px;",
                    h3 { "Small" }
                    RadioGroup {
                        value: size_value(),
                        size: "small",
                        onchange: move |value| {
                            size_value.set(value);
                        },
                        Radio { value: "small1", "Small Option 1" }
                        Radio { value: "small2", "Small Option 2" }
                    }
                }
                p { "选中的值: {size_value()}" }
            }

            // 禁用状态
            section {
                style: "margin-bottom: 32px;",
                h2 { "禁用状态" }
                div {
                    style: "margin-bottom: 16px;",
                    RadioGroup {
                        value: disabled_value(),
                        onchange: move |value| {
                            disabled_value.set(value);
                        },
                        Radio { value: "option1", "Normal Option" }
                        Radio { value: "option2", disabled: true, "Disabled Option" }
                        Radio { value: "option3", "Another Normal Option" }
                    }
                }
                p { "选中的值: {disabled_value()}" }
            }

            // 使用 options 属性
            section {
                style: "margin-bottom: 32px;",
                h2 { "使用 options 属性" }
                div {
                    style: "margin-bottom: 16px;",
                    RadioGroup {
                        value: group_value(),
                        options: vec![
                            RadioOption { label: "选项A".to_string(), value: "a".to_string(), disabled: Some(false) },
                            RadioOption { label: "选项B".to_string(), value: "b".to_string(), disabled: Some(false) },
                            RadioOption { label: "禁用选项".to_string(), value: "c".to_string(), disabled: Some(true) },
                        ],
                        onchange: move |value| {
                            group_value.set(value);
                        },
                    }
                }
                p { "选中的值: {group_value()}" }
            }
        }
    }
}

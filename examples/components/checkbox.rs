//! Checkbox 复选框组件示例
//!
//! 展示 Checkbox 组件的各种用法和功能

use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn CheckboxDemo() -> Element {
    rsx! {
        // style { {include_str!("../src/components/checkbox/style.css")} }

        div {
            style: "padding: 24px; max-width: 1200px; margin: 0 auto;",

            h1 { "Checkbox 复选框示例" }

            // 基本用法
            BasicExample {}

            // 尺寸变体
            SizeExample {}

            // 禁用状态
            DisabledExample {}

            // 不确定状态
            IndeterminateExample {}

            // 复选框组
            GroupExample {}

            // 全选功能
            CheckAllExample {}

            // 网格布局
            GridExample {}

            // 卡片样式
            CardExample {}

            // 按钮样式
            ButtonExample {}
        }
    }
}

#[component]
fn BasicExample() -> Element {
    let mut checked = use_signal(|| false);
    let mut checked2 = use_signal(|| true);

    rsx! {
        div {
            style: "margin-bottom: 32px;",

            h2 { "基本用法" }
            p { "简单的复选框。" }

            div {
                style: "display: flex; gap: 16px; flex-wrap: wrap;",

                Checkbox {
                    checked: checked(),
                    on_change: move |c| {
                        checked.set(c);
                        println!("复选框1状态: {}", c);
                    },
                    "复选框"
                }

                Checkbox {
                    checked: checked2(),
                    on_change: move |c| {
                        checked2.set(c);
                        println!("复选框2状态: {}", c);
                    },
                    "默认选中"
                }

                Checkbox {
                    "无状态复选框"
                }
            }

            div {
                style: "margin-top: 16px; padding: 8px; background: #f5f5f5; border-radius: 4px;",
                "状态: 复选框1={checked()}, 复选框2={checked2()}"
            }
        }
    }
}

#[component]
fn SizeExample() -> Element {
    let mut large_checked = use_signal(|| false);
    let mut middle_checked = use_signal(|| false);
    let mut small_checked = use_signal(|| false);

    rsx! {
        div {
            style: "margin-bottom: 32px;",

            h2 { "不同尺寸" }
            p { "三种尺寸的复选框：大、中、小。" }

            div {
                style: "display: flex; gap: 16px; align-items: center; flex-wrap: wrap;",

                div {
                    "大尺寸:"
                    Checkbox {
                        checked: large_checked(),
                        size: CheckboxSize::Large,
                        on_change: move |c| large_checked.set(c),
                        style: "margin-left: 8px;",
                        "Large"
                    }
                }

                div {
                    "中等尺寸:"
                    Checkbox {
                        checked: middle_checked(),
                        size: CheckboxSize::Middle,
                        on_change: move |c| middle_checked.set(c),
                        style: "margin-left: 8px;",
                        "Middle"
                    }
                }

                div {
                    "小尺寸:"
                    Checkbox {
                        checked: small_checked(),
                        size: CheckboxSize::Small,
                        on_change: move |c| small_checked.set(c),
                        style: "margin-left: 8px;",
                        "Small"
                    }
                }
            }
        }
    }
}

#[component]
fn DisabledExample() -> Element {
    rsx! {
        div {
            style: "margin-bottom: 32px;",

            h2 { "禁用状态" }
            p { "禁用的复选框。" }

            div {
                style: "display: flex; gap: 16px; flex-wrap: wrap;",

                Checkbox {
                    disabled: true,
                    "禁用未选中"
                }

                Checkbox {
                    checked: true,
                    disabled: true,
                    "禁用已选中"
                }

                Checkbox {
                    indeterminate: true,
                    disabled: true,
                    "禁用不确定"
                }
            }
        }
    }
}

#[component]
fn IndeterminateExample() -> Element {
    let mut indeterminate = use_signal(|| true);
    let mut checked = use_signal(|| false);

    rsx! {
        div {
            style: "margin-bottom: 32px;",

            h2 { "不确定状态" }
            p { "在实现全选效果时，你可能会用到 indeterminate 属性。" }

            div {
                style: "display: flex; gap: 16px; align-items: center; flex-wrap: wrap;",

                Checkbox {
                    indeterminate: indeterminate(),
                    checked: checked(),
                    on_change: move |c| {
                        checked.set(c);
                        indeterminate.set(false);
                        println!("不确定状态复选框: {}", c);
                    },
                    "不确定状态"
                }

                button {
                    onclick: move |_| {
                        indeterminate.set(true);
                        checked.set(false);
                    },
                    "设为不确定"
                }

                button {
                    onclick: move |_| {
                        indeterminate.set(false);
                        checked.set(true);
                    },
                    "设为选中"
                }

                button {
                    onclick: move |_| {
                        indeterminate.set(false);
                        checked.set(false);
                    },
                    "设为未选中"
                }
            }
        }
    }
}

#[component]
fn GroupExample() -> Element {
    let values = use_signal(|| vec!["apple".to_string(), "orange".to_string()]);

    let options = vec![
        CheckboxOption::new("苹果", "apple"),
        CheckboxOption::new("香蕉", "banana"),
        CheckboxOption::new("橙子", "orange"),
        CheckboxOption::new("梨子", "pear").disabled(true),
    ];

    rsx! {
    div {
        style: "margin-bottom: 32px;",

        h2 { "复选框组" }
        p { "方便的从数组生成复选框组。" }

        div {
            style: "display: flex; gap: 32px; align-items: flex-start; flex-wrap: wrap;",

            div {
                h4 { "使用选项数组" }
                CheckboxGroup {
                    value: values(),
                    options: options,
                    // on_change: move |v| {
                    //     values.set(v.clone());
                    //     println!("选中的水果: {:?}", v);
                    }
                }
            }

            div {
                h4 { "手动组合" }
                CheckboxGroup {
                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",

                        Checkbox { value: "red", "红色" }
                        Checkbox { value: "green", "绿色" }
                        Checkbox { value: "blue", "蓝色" }
                        Checkbox { value: "yellow", disabled: true, "黄色（禁用）" }
                    }
                }
            }

            // div {
            //     style: "padding: 8px; background: #f5f5f5; border-radius: 4px; max-width: 200px;",
            //     "已选择的水果: {values().join(", ")}"
            // }
        }
    }
}

#[component]
fn CheckAllExample() -> Element {
    let mut check_all = use_signal(|| false);
    let mut indeterminate = use_signal(|| false);
    let mut checked_list = use_signal(|| vec!["Apple".to_string()]);

    let plain_options = vec!["Apple", "Pear", "Orange"];

    let plain_options01 = plain_options.clone();
    let on_check_all_change = move |checked: bool| {
        if checked {
            checked_list.set(plain_options01.iter().map(|s| s.to_string()).collect());
        } else {
            checked_list.set(vec![]);
        }
        check_all.set(checked);
        indeterminate.set(false);
    };

    let plain_options0q = plain_options.clone();
    let on_change = move |list: Vec<String>| {
        checked_list.set(list.clone());
        check_all.set(list.len() == plain_options0q.len());
        indeterminate.set(list.len() > 0 && list.len() < plain_options0q.len());
    };

    rsx! {
        div {
            style: "margin-bottom: 32px;",

            h2 { "全选" }
            p { "在实现全选效果时，你可能会用到 indeterminate 属性。" }

            div {
                style: "border: 1px solid #d9d9d9; border-radius: 6px; padding: 16px;",

                div {
                    style: "border-bottom: 1px solid #f0f0f0; padding-bottom: 8px; margin-bottom: 8px;",

                    CheckAll {
                        checked: check_all(),
                        indeterminate: indeterminate(),
                        on_change: on_check_all_change,
                        "全选"
                    }
                }

                CheckboxGroup {
                    value: checked_list(),
                    options: plain_options.iter().map(|&s| CheckboxOption::new(s, s)).collect(),
                    on_change: on_change
                }

                // div {
                //     style: "margin-top: 16px; padding: 8px; background: #f5f5f5; border-radius: 4px;",
                //     "已选择: {checked_list().join(", ")}"
                // }
            }
        }
    }
}

#[component]
fn GridExample() -> Element {
    let values = use_signal(|| vec![]);

    let options = vec![
        CheckboxOption::new("选项A", "a"),
        CheckboxOption::new("选项B", "b"),
        CheckboxOption::new("选项C", "c"),
        CheckboxOption::new("选项D", "d"),
        CheckboxOption::new("选项E", "e"),
        CheckboxOption::new("选项F", "f"),
    ];

    rsx! {
        div {
            style: "margin-bottom: 32px;",

            h2 { "网格布局" }
            p { "使用网格布局的复选框组。" }

            CheckboxGroup {
                value: values(),
                options: options,
                class: "ant-checkbox-group-grid",
                // on_change: move |v| {
                //     values.set(v.clone());
                //     println!("网格选择: {:?}", v);
                // }
            }

            // div {
            //     style: "margin-top: 16px; padding: 8px; background: #f5f5f5; border-radius: 4px;",
            //     "已选择: {values().join(", ")}"
            // }
        }
    }
}

#[component]
fn CardExample() -> Element {
    // let values = use_signal(|| vec![]);

    rsx! {
        div {
            style: "margin-bottom: 32px;",

            h2 { "卡片样式" }
            p { "卡片样式的复选框，适用于选择卡片等场景。" }

            div {
                style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 16px;",

                Checkbox {
                    class: "ant-checkbox-card",
                    value: "card1",

                    div {
                        style: "margin-left: 8px;",
                        h4 { style: "margin: 0 0 4px 0;", "基础套餐" }
                        p { style: "margin: 0; color: #666; font-size: 12px;", "包含基本功能" }
                    }
                }

                Checkbox {
                    class: "ant-checkbox-card",
                    value: "card2",

                    div {
                        style: "margin-left: 8px;",
                        h4 { style: "margin: 0 0 4px 0;", "高级套餐" }
                        p { style: "margin: 0; color: #666; font-size: 12px;", "包含高级功能" }
                    }
                }

                Checkbox {
                    class: "ant-checkbox-card",
                    value: "card3",

                    div {
                        style: "margin-left: 8px;",
                        h4 { style: "margin: 0 0 4px 0;", "企业套餐" }
                        p { style: "margin: 0; color: #666; font-size: 12px;", "包含企业级功能" }
                    }
                }
            }
        }
    }
}

#[component]
fn ButtonExample() -> Element {
    let values = use_signal(|| vec!["medium".to_string()]);

    rsx! {
        div {
            style: "margin-bottom: 32px;",

            h2 { "按钮样式" }
            p { "按钮样式的复选框组，类似于按钮组的效果。" }

            div {
                h4 { "尺寸选择" }
                div {
                    style: "display: flex; margin-bottom: 16px;",

                    Checkbox {
                        class: "ant-checkbox-button",
                        value: "small",
                        "Small"
                    }
                    Checkbox {
                        class: "ant-checkbox-button",
                        value: "medium",
                        checked: values().contains(&"medium".to_string()),
                        "Medium"
                    }
                    Checkbox {
                        class: "ant-checkbox-button",
                        value: "large",
                        "Large"
                    }
                }

                h4 { "功能选择" }
                div {
                    style: "display: flex; margin-bottom: 16px;",

                    Checkbox {
                        class: "ant-checkbox-button",
                        value: "wifi",
                        "WiFi"
                    }
                    Checkbox {
                        class: "ant-checkbox-button",
                        value: "bluetooth",
                        "蓝牙"
                    }
                    Checkbox {
                        class: "ant-checkbox-button",
                        value: "gps",
                        "GPS"
                    }
                    Checkbox {
                        class: "ant-checkbox-button",
                        value: "nfc",
                        disabled: true,
                        "NFC (禁用)"
                    }
                }
            }
        }
    }
}

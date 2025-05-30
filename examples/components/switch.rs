use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn SwitchDemo() -> Element {
    let mut basic_checked = use_signal(|| false);
    let mut disabled_checked = use_signal(|| true);
    let mut loading_checked = use_signal(|| false);
    let mut text_checked = use_signal(|| true);
    let mut size_checked = use_signal(|| false);
    let mut disabled_switch = use_signal(|| true);

    rsx! {
        // style {
        //     r#"
        //     .custom-switch {
        //         /* 自定义开关样式示例 */
        //     }
        //     "#
        // }

        div {
            style: "padding: 20px; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;",

            h1 { "Switch 开关组件示例" }

            // 基本用法
            section {
                style: "margin-bottom: 32px;",
                h2 { "基本用法" }
                div {
                    style: "margin-bottom: 16px;",
                    Switch {
                        checked: basic_checked(),
                        onchange: move |checked| {
                            basic_checked.set(checked);
                        },
                    }
                }
                p { "当前状态: {basic_checked()}" }
            }

            // 不同尺寸
            section {
                style: "margin-bottom: 32px;",
                h2 { "不同尺寸" }
                div {
                    style: "display: flex; align-items: center; gap: 16px; margin-bottom: 16px;",
                    Switch {
                        checked: size_checked(),
                        onchange: move |checked| {
                            size_checked.set(checked);
                        },
                    }
                    Switch {
                        size: SwitchSize::Small,
                        checked: size_checked(),
                        onchange: move |checked| {
                            size_checked.set(checked);
                        },
                    }
                }
                p { "当前状态: {size_checked()}" }
            }

            // 带文字和图标
            section {
                style: "margin-bottom: 32px;",
                h2 { "带文字和图标" }
                div {
                    style: "display: flex; flex-direction: column; gap: 16px; margin-bottom: 16px;",
                    Switch {
                        checked: text_checked(),
                        checked_children: "开".to_string(),
                        unchecked_children: "关".to_string(),
                        onchange: move |checked| {
                            text_checked.set(checked);
                        },
                    }
                    Switch {
                        checked: text_checked(),
                        checked_children: "1".to_string(),
                        unchecked_children: "0".to_string(),
                        onchange: move |checked| {
                            text_checked.set(checked);
                        },
                    }
                    Switch {
                        checked: text_checked(),
                        checked_children: "✓".to_string(),
                        unchecked_children: "✕".to_string(),
                        onchange: move |checked| {
                            text_checked.set(checked);
                        },
                    }
                }
                p { "当前状态: {text_checked()}" }
            }

            // 禁用状态
            section {
                style: "margin-bottom: 32px;",
                h2 { "禁用状态" }
                div {
                    style: "display: flex; flex-direction: column; gap: 16px; margin-bottom: 16px;",
                    div {
                        style: "display: flex; align-items: center; gap: 16px;",
                        Switch {
                            disabled: true,
                            checked: disabled_checked(),
                        }
                        button {
                            onclick: move |_| {
                                disabled_switch.set(!disabled_switch());
                            },
                            "Toggle disabled"
                        }
                    }
                    Switch {
                        disabled: disabled_switch(),
                        checked: disabled_checked(),
                        onchange: move |checked| {
                            disabled_checked.set(checked);
                        },
                    }
                }
                p { "禁用状态: {disabled_switch()}, 开关状态: {disabled_checked()}" }
            }

            // 加载中状态
            section {
                style: "margin-bottom: 32px;",
                h2 { "加载中状态" }
                div {
                    style: "display: flex; flex-direction: column; gap: 16px; margin-bottom: 16px;",
                    Switch {
                        loading: true,
                        checked: loading_checked(),
                    }
                    Switch {
                        size: SwitchSize::Small,
                        loading: true,
                        checked: loading_checked(),
                    }
                }
                p { "加载状态下无法切换" }
            }

            // 自动聚焦
            section {
                style: "margin-bottom: 32px;",
                h2 { "自动聚焦" }
                div {
                    style: "margin-bottom: 16px;",
                    Switch {
                        auto_focus: true,
                        checked: basic_checked(),
                        onchange: move |checked| {
                            basic_checked.set(checked);
                        },
                    }
                }
                p { "页面加载时自动聚焦到此开关" }
            }

            // 自定义样式
            section {
                style: "margin-bottom: 32px;",
                h2 { "自定义样式" }
                div {
                    style: "margin-bottom: 16px;",
                    Switch {
                        checked: basic_checked(),
                        class: "custom-switch".to_string(),
                        style: "margin-right: 16px;".to_string(),
                        onchange: move |checked| {
                            basic_checked.set(checked);
                        },
                    }
                }
                p { "可以通过 class 和 style 属性自定义样式" }
            }
        }

    }
}

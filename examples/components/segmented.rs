use crate::common::demo_section::DemoSection;
use ant_design_dioxus::components::segmented::Segmented;
use dioxus::prelude::*;

/// Segmented组件示例
#[component]
pub fn SegmentedDemo() -> Element {
    rsx! {
        div { class: "demo-container",
            h1 { "Segmented 分段控制器" }
            p { "分段控制器。自 antd@4.20.0 版本开始提供该组件。" }

            DemoSection {
                title: "基本使用",
                description: "最简单的用法。",
                code: r#"rsx! {
    Segmented {
        options: vec!["Daily", "Weekly", "Monthly", "Quarterly", "Yearly"]
    }
}"#,
                demo: rsx! {
                    Segmented {
                        class: "demo-segmented",
                        style: "background: #f5f5f5; border-radius: 6px; padding: 2px; display: inline-flex;",
                        div { style: "display: flex;",
                            button { style: "padding: 4px 16px; border: none; background: #1677ff; color: white; border-radius: 4px; cursor: pointer; margin-right: 2px;", "Daily" }
                            button { style: "padding: 4px 16px; border: none; background: transparent; color: #666; border-radius: 4px; cursor: pointer; margin-right: 2px;", "Weekly" }
                            button { style: "padding: 4px 16px; border: none; background: transparent; color: #666; border-radius: 4px; cursor: pointer; margin-right: 2px;", "Monthly" }
                            button { style: "padding: 4px 16px; border: none; background: transparent; color: #666; border-radius: 4px; cursor: pointer; margin-right: 2px;", "Quarterly" }
                            button { style: "padding: 4px 16px; border: none; background: transparent; color: #666; border-radius: 4px; cursor: pointer;", "Yearly" }
                        }
                    }
                }
            }

            DemoSection {
                title: "带图标的选项",
                description: "为选项添加图标。",
                code: r#"rsx! {
    Segmented {
        class: "segmented-with-icons",
        options: vec![
            { icon: "📊", label: "List" },
            { icon: "🗂️", label: "Kanban" },
            { icon: "📅", label: "Calendar" }
        ]
    }
}"#,
                demo: rsx! {
                    Segmented {
                        class: "segmented-with-icons",
                        style: "background: #f5f5f5; border-radius: 6px; padding: 2px; display: inline-flex;",
                        div { style: "display: flex;",
                            button {
                                style: "padding: 8px 16px; border: none; background: #1677ff; color: white; border-radius: 4px; cursor: pointer; margin-right: 2px; display: flex; align-items: center; gap: 6px;",
                                span { "📊" }
                                span { "List" }
                            }
                            button {
                                style: "padding: 8px 16px; border: none; background: transparent; color: #666; border-radius: 4px; cursor: pointer; margin-right: 2px; display: flex; align-items: center; gap: 6px;",
                                span { "🗂️" }
                                span { "Kanban" }
                            }
                            button {
                                style: "padding: 8px 16px; border: none; background: transparent; color: #666; border-radius: 4px; cursor: pointer; display: flex; align-items: center; gap: 6px;",
                                span { "📅" }
                                span { "Calendar" }
                            }
                        }
                    }
                }
            }

            DemoSection {
                title: "不同尺寸",
                description: "三种尺寸的分段控制器。",
                code: r#"rsx! {
    div { style: "display: flex; flex-direction: column; gap: 16px;",
        Segmented { class: "segmented-large", options: vec!["Large", "Size"] }
        Segmented { class: "segmented-default", options: vec!["Default", "Size"] }
        Segmented { class: "segmented-small", options: vec!["Small", "Size"] }
    }
}"#,
                demo: rsx! {
                    div { style: "display: flex; flex-direction: column; gap: 16px;",
                        Segmented {
                            class: "segmented-large",
                            style: "background: #f5f5f5; border-radius: 8px; padding: 3px; display: inline-flex;",
                            div { style: "display: flex;",
                                button { style: "padding: 8px 20px; border: none; background: #1677ff; color: white; border-radius: 6px; cursor: pointer; margin-right: 3px; font-size: 14px;", "Large" }
                                button { style: "padding: 8px 20px; border: none; background: transparent; color: #666; border-radius: 6px; cursor: pointer; font-size: 14px;", "Size" }
                            }
                        }
                        Segmented {
                            class: "segmented-default",
                            style: "background: #f5f5f5; border-radius: 6px; padding: 2px; display: inline-flex;",
                            div { style: "display: flex;",
                                button { style: "padding: 4px 16px; border: none; background: #1677ff; color: white; border-radius: 4px; cursor: pointer; margin-right: 2px;", "Default" }
                                button { style: "padding: 4px 16px; border: none; background: transparent; color: #666; border-radius: 4px; cursor: pointer;", "Size" }
                            }
                        }
                        Segmented {
                            class: "segmented-small",
                            style: "background: #f5f5f5; border-radius: 4px; padding: 1px; display: inline-flex;",
                            div { style: "display: flex;",
                                button { style: "padding: 2px 12px; border: none; background: #1677ff; color: white; border-radius: 3px; cursor: pointer; margin-right: 1px; font-size: 12px;", "Small" }
                                button { style: "padding: 2px 12px; border: none; background: transparent; color: #666; border-radius: 3px; cursor: pointer; font-size: 12px;", "Size" }
                            }
                        }
                    }
                }
            }

            DemoSection {
                title: "禁用状态",
                description: "禁用状态的分段控制器。",
                code: r#"rsx! {
    Segmented {
        class: "segmented-disabled",
        disabled: true,
        options: vec!["Disabled", "Segmented"]
    }
}"#,
                demo: rsx! {
                    Segmented {
                        class: "segmented-disabled",
                        style: "background: #f5f5f5; border-radius: 6px; padding: 2px; display: inline-flex; opacity: 0.6;",
                        div { style: "display: flex;",
                            button { style: "padding: 4px 16px; border: none; background: #d9d9d9; color: #00000040; border-radius: 4px; cursor: not-allowed; margin-right: 2px;", "Disabled" }
                            button { style: "padding: 4px 16px; border: none; background: transparent; color: #00000040; border-radius: 4px; cursor: not-allowed;", "Segmented" }
                        }
                    }
                }
            }

            DemoSection {
                title: "动态数据",
                description: "动态改变选项数据。",
                code: r#"rsx! {
    Segmented {
        class: "segmented-dynamic",
        options: vec!["Option 1", "Option 2", "Option 3", "Option 4"]
    }
}"#,
                demo: rsx! {
                    Segmented {
                        class: "segmented-dynamic",
                        style: "background: #f5f5f5; border-radius: 6px; padding: 2px; display: inline-flex;",
                        div { style: "display: flex;",
                            button { style: "padding: 4px 12px; border: none; background: #1677ff; color: white; border-radius: 4px; cursor: pointer; margin-right: 2px;", "Option 1" }
                            button { style: "padding: 4px 12px; border: none; background: transparent; color: #666; border-radius: 4px; cursor: pointer; margin-right: 2px;", "Option 2" }
                            button { style: "padding: 4px 12px; border: none; background: transparent; color: #666; border-radius: 4px; cursor: pointer; margin-right: 2px;", "Option 3" }
                            button { style: "padding: 4px 12px; border: none; background: transparent; color: #666; border-radius: 4px; cursor: pointer;", "Option 4" }
                        }
                    }
                }
            }
        }
    }
}

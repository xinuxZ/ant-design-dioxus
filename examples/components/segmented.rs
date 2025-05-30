use crate::common::demo_section::DemoSection;
use ant_design_dioxus::prelude::*;
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

                div {
                    Segmented {
                        class: Some("demo-segmented".to_string()),
                        style: Some("background: #f5f5f5; border-radius: 6px; padding: 2px; display: inline-flex;".to_string()),
                        options: vec![
                            SegmentedOption::String("Daily".to_string()),
                            SegmentedOption::String("Weekly".to_string()),
                            SegmentedOption::String("Monthly".to_string()),
                            SegmentedOption::String("Quarterly".to_string()),
                            SegmentedOption::String("Yearly".to_string()),
                        ],
                        default_value: Some("Daily".to_string())
                    }
                }
            }

            DemoSection {
                title: "带图标的选项",
                description: "为选项添加图标。",

                div {
                    Segmented {
                        class: Some("segmented-with-icons".to_string()),
                        style: Some("background: #f5f5f5; border-radius: 6px; padding: 2px; display: inline-flex;".to_string()),
                        options: vec![
                            SegmentedOption::Item(SegmentedItem {
                                label: Some(rsx!("Map")),
                                value: "map".to_string(),
                                icon: Some(rsx!(Icon { icon_type: "user".to_string(), style: "margin-right: 4px;" })),
                                disabled: false,
                                class_name: None,
                            }),
                            SegmentedOption::Item(SegmentedItem {
                                label: Some(rsx!("Transit")),
                                value: "transit".to_string(),
                                icon: Some(rsx!(Icon { icon_type: "app".to_string(), style: "margin-right: 4px;" })),
                                disabled: false,
                                class_name: None,
                            }),
                            SegmentedOption::Item(SegmentedItem {
                                label: Some(rsx!("Satellite")),
                                value: "satellite".to_string(),
                                icon: Some(rsx!(Icon { icon_type: "compass".to_string(), style: "margin-right: 4px;" })),
                                disabled: false,
                                class_name: None,
                            }),
                        ],
                        default_value: Some("map".to_string())
                    }
                }
            }

            DemoSection {
                title: "不同尺寸",
                description: "三种尺寸的分段控制器。",
                div {
                    div { style: "display: flex; flex-direction: column; gap: 16px;",
                        Segmented {
                            class: Some("segmented-large".to_string()),
                            style: Some("background: #f5f5f5; border-radius: 8px; padding: 3px; display: inline-flex;".to_string()),
                            options: vec![
                                SegmentedOption::String("Daily".to_string()),
                            ],
                            // div { style: "display: flex;",
                            //     button { style: "padding: 8px 20px; border: none; background: #1677ff; color: white; border-radius: 6px; cursor: pointer; margin-right: 3px; font-size: 14px;", "Large" }
                            //     button { style: "padding: 8px 20px; border: none; background: transparent; color: #666; border-radius: 6px; cursor: pointer; font-size: 14px;", "Size" }
                            // }
                        }
                        Segmented {
                            class: Some("segmented-default".to_string()),
                            style: Some("background: #f5f5f5; border-radius: 6px; padding: 2px; display: inline-flex;".to_string()),
                            options: vec![
                                SegmentedOption::String("Daily".to_string()),
                            ],
                            // div { style: "display: flex;",
                            //     button { style: "padding: 4px 16px; border: none; background: #1677ff; color: white; border-radius: 4px; cursor: pointer; margin-right: 2px;", "Default" }
                            //     button { style: "padding: 4px 16px; border: none; background: transparent; color: #666; border-radius: 4px; cursor: pointer;", "Size" }
                            // }
                        }
                        Segmented {
                            class: Some("segmented-small".to_string()),
                            style: Some("background: #f5f5f5; border-radius: 4px; padding: 1px; display: inline-flex;".to_string()),
                            options: vec![
                                SegmentedOption::String("Daily".to_string()),
                            ],
                            // div { style: "display: flex;",
                            //     button { style: "padding: 2px 12px; border: none; background: #1677ff; color: white; border-radius: 3px; cursor: pointer; margin-right: 1px; font-size: 12px;", "Small" }
                            //     button { style: "padding: 2px 12px; border: none; background: transparent; color: #666; border-radius: 3px; cursor: pointer; font-size: 12px;", "Size" }
                            // }
                        }
                    }
                }
            }

            DemoSection {
                title: "禁用状态",
                description: "禁用状态的分段控制器。",

                div {
                    Segmented {
                        class: Some("segmented-disabled".to_string()),
                        style: Some("background: #f5f5f5; border-radius: 6px; padding: 2px; display: inline-flex; opacity: 0.6;".to_string()),
                        options: vec![
                            SegmentedOption::String("Daily".to_string()),
                        ],
                        // div { style: "display: flex;",
                        //     button { style: "padding: 4px 16px; border: none; background: #d9d9d9; color: #00000040; border-radius: 4px; cursor: not-allowed; margin-right: 2px;", "Disabled" }
                        //     button { style: "padding: 4px 16px; border: none; background: transparent; color: #00000040; border-radius: 4px; cursor: not-allowed;", "Segmented" }
                        // }
                    }
                }
            }

            DemoSection {
                title: "动态数据",
                description: "动态改变选项数据。",

                div {
                    Segmented {
                        class: Some("segmented-dynamic".to_string()),
                        style: Some("background: #f5f5f5; border-radius: 6px; padding: 2px; display: inline-flex;".to_string()),
                        options: vec![
                            SegmentedOption::String("Daily".to_string()),
                        ],
                        // div { style: "display: flex;",
                        //     button { style: "padding: 4px 12px; border: none; background: #1677ff; color: white; border-radius: 4px; cursor: pointer; margin-right: 2px;", "Option 1" }
                        //     button { style: "padding: 4px 12px; border: none; background: transparent; color: #666; border-radius: 4px; cursor: pointer; margin-right: 2px;", "Option 2" }
                        //     button { style: "padding: 4px 12px; border: none; background: transparent; color: #666; border-radius: 4px; cursor: pointer; margin-right: 2px;", "Option 3" }
                        //     button { style: "padding: 4px 12px; border: none; background: transparent; color: #666; border-radius: 4px; cursor: pointer;", "Option 4" }
                        // }
                    }
                }
            }
        }
    }
}

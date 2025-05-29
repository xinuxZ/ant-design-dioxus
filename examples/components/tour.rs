use crate::common::demo_section::DemoSection;
use ant_design_dioxus::components::tour::Tour;
use dioxus::prelude::*;

/// Tour组件示例
#[component]
pub fn TourDemo() -> Element {
    rsx! {
        div { class: "demo-container",
            h1 { "Tour 漫游式引导" }
            p { "用于分步引导用户了解产品功能的气泡组件。" }

            DemoSection {
                title: "基本使用",
                description: "最简单的用法。",

                div {
                    div { style: "position: relative; padding: 24px;",
                        button {
                            id: "tour-target-1",
                            style: "padding: 8px 16px; background: #1677ff; color: white; border: none; border-radius: 6px; cursor: pointer;",
                            "开始引导"
                        }
                        Tour {
                            class: "demo-tour",
                            style: "position: absolute; top: 60px; left: 0; background: white; border: 1px solid #d9d9d9; border-radius: 8px; padding: 16px; box-shadow: 0 4px 12px rgba(0,0,0,0.1);",
                            div {
                                h4 { style: "margin: 0 0 8px 0;", "欢迎使用" }
                                p { style: "margin: 0 0 12px 0; color: #666;", "这是一个引导提示" }
                                div { style: "display: flex; gap: 8px;",
                                    button { style: "padding: 4px 12px; border: 1px solid #d9d9d9; background: white; border-radius: 4px; cursor: pointer;", "跳过" }
                                    button { style: "padding: 4px 12px; background: #1677ff; color: white; border: none; border-radius: 4px; cursor: pointer;", "下一步" }
                                }
                            }
                        }
                    }
                }
            }

            DemoSection {
                title: "自定义样式",
                description: "可以自定义引导框的样式。",
                div {
                    div { style: "position: relative; padding: 24px;",
                        button {
                            id: "tour-target-2",
                            style: "padding: 8px 16px; background: #52c41a; color: white; border: none; border-radius: 6px; cursor: pointer;",
                            "功能按钮"
                        }
                        Tour {
                            class: "custom-tour",
                            style: "position: absolute; top: 60px; left: 0; background: #f6ffed; border: 1px solid #b7eb8f; border-radius: 8px; padding: 16px; box-shadow: 0 4px 12px rgba(0,0,0,0.1);",
                            div {
                                h4 { style: "margin: 0 0 8px 0; color: #389e0d;", "功能介绍" }
                                p { style: "margin: 0 0 12px 0; color: #666;", "这个按钮可以执行特定功能" }
                                div { style: "display: flex; gap: 8px;",
                                    button { style: "padding: 4px 12px; border: 1px solid #b7eb8f; background: white; border-radius: 4px; cursor: pointer;", "知道了" }
                                }
                            }
                        }
                    }
                }
            }

            DemoSection {
                title: "多步骤引导",
                description: "支持多个步骤的引导流程。",

                div {
                    div { style: "position: relative; padding: 24px;",
                        div { style: "display: flex; gap: 16px; margin-bottom: 20px;",
                            button {
                                id: "step-1",
                                style: "padding: 8px 16px; background: #1677ff; color: white; border: none; border-radius: 6px; cursor: pointer;",
                                "步骤1"
                            }
                            button {
                                id: "step-2",
                                style: "padding: 8px 16px; background: #722ed1; color: white; border: none; border-radius: 6px; cursor: pointer;",
                                "步骤2"
                            }
                            button {
                                id: "step-3",
                                style: "padding: 8px 16px; background: #fa541c; color: white; border: none; border-radius: 6px; cursor: pointer;",
                                "步骤3"
                            }
                        }
                        Tour {
                            class: "multi-step-tour",
                            style: "position: absolute; top: 80px; left: 0; background: white; border: 1px solid #d9d9d9; border-radius: 8px; padding: 16px; box-shadow: 0 4px 12px rgba(0,0,0,0.1);",
                            div {
                                div { style: "display: flex; align-items: center; margin-bottom: 12px;",
                                    span { style: "background: #1677ff; color: white; width: 20px; height: 20px; border-radius: 50%; display: flex; align-items: center; justify-content: center; font-size: 12px; margin-right: 8px;", "1" }
                                    h4 { style: "margin: 0;", "第一步" }
                                }
                                p { style: "margin: 0 0 12px 0; color: #666;", "这是多步骤引导的第一步" }
                                div { style: "display: flex; justify-content: space-between; align-items: center;",
                                    span { style: "font-size: 12px; color: #999;", "1 / 3" }
                                    div { style: "display: flex; gap: 8px;",
                                        button { style: "padding: 4px 12px; border: 1px solid #d9d9d9; background: white; border-radius: 4px; cursor: pointer;", "跳过" }
                                        button { style: "padding: 4px 12px; background: #1677ff; color: white; border: none; border-radius: 4px; cursor: pointer;", "下一步" }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            DemoSection {
                title: "带遮罩的引导",
                description: "可以添加遮罩层突出引导目标。",

                div {
                    div { style: "position: relative; padding: 24px; background: rgba(0,0,0,0.05); border-radius: 8px;",
                        button {
                            id: "tour-target-mask",
                            style: "padding: 8px 16px; background: #fa8c16; color: white; border: none; border-radius: 6px; cursor: pointer; position: relative; z-index: 1002;",
                            "重要功能"
                        }
                        Tour {
                            class: "tour-with-mask",
                            style: "position: absolute; top: 60px; left: 24px; background: white; border: 1px solid #d9d9d9; border-radius: 8px; padding: 16px; box-shadow: 0 4px 12px rgba(0,0,0,0.2); z-index: 1001;",
                            div {
                                h4 { style: "margin: 0 0 8px 0; color: #fa8c16;", "重要提示" }
                                p { style: "margin: 0 0 12px 0; color: #666;", "这是一个重要的功能按钮，请注意使用" }
                                div { style: "display: flex; gap: 8px;",
                                    button { style: "padding: 4px 12px; background: #fa8c16; color: white; border: none; border-radius: 4px; cursor: pointer;", "我知道了" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

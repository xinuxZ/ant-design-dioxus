use crate::common::demo_section::DemoSection;
use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

/// Tour组件示例
#[component]
pub fn TourDemo() -> Element {
    // 基本使用示例的状态
    let mut basic_tour_open = use_signal(|| false);

    // 自定义样式示例的状态
    let mut custom_tour_open = use_signal(|| false);

    // 多步骤引导示例的状态
    let mut multi_step_tour_open = use_signal(|| false);
    let mut multi_step_current = use_signal(|| 0);

    // 带遮罩的引导示例的状态
    let mut masked_tour_open = use_signal(|| false);

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
                            onclick: move |_| basic_tour_open.set(true),
                            "开始引导"
                        }

                        // 基本使用示例的Tour组件
                        Tour {
                            open: *basic_tour_open.read(),
                            on_close: move |_| basic_tour_open.set(false),
                            steps: vec![
                                TourStep {
                                    title: Some("欢迎使用".into()),
                                    description: Some("这是一个引导提示".into()),
                                    target: Some("#tour-target-1".into()),
                                    ..Default::default()
                                }
                            ],
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
                            onclick: move |_| custom_tour_open.set(true),
                            "功能按钮"
                        }

                        // 自定义样式示例的Tour组件
                        Tour {
                            open: *custom_tour_open.read(),
                            on_close: move |_| custom_tour_open.set(false),
                            class: Some("custom-tour".into()),
                            style: Some("background: #f6ffed; border: 1px solid #b7eb8f;".into()),
                            steps: vec![
                                TourStep {
                                    title: Some("功能介绍".into()),
                                    description: Some("这个按钮可以执行特定功能".into()),
                                    target: Some("#tour-target-2".into()),
                                    ..Default::default()
                                }
                            ],
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
                                onclick: move |_| {
                                    multi_step_current.set(0);
                                    multi_step_tour_open.set(true);
                                },
                                "步骤1"
                            }
                            button {
                                id: "step-2",
                                style: "padding: 8px 16px; background: #722ed1; color: white; border: none; border-radius: 6px; cursor: pointer;",
                                onclick: move |_| {
                                    multi_step_current.set(1);
                                    multi_step_tour_open.set(true);
                                },
                                "步骤2"
                            }
                            button {
                                id: "step-3",
                                style: "padding: 8px 16px; background: #fa541c; color: white; border: none; border-radius: 6px; cursor: pointer;",
                                onclick: move |_| {
                                    multi_step_current.set(2);
                                    multi_step_tour_open.set(true);
                                },
                                "步骤3"
                            }
                        }

                        // 多步骤引导示例的Tour组件
                        Tour {
                            open: *multi_step_tour_open.read(),
                            current: *multi_step_current.read(),
                            on_close: move |_| multi_step_tour_open.set(false),
                            on_change: move |step| multi_step_current.set(step),
                            steps: vec![
                                TourStep {
                                    title: Some("第一步".into()),
                                    description: Some("这是多步骤引导的第一步".into()),
                                    target: Some("#step-1".into()),
                                    ..Default::default()
                                },
                                TourStep {
                                    title: Some("第二步".into()),
                                    description: Some("这是多步骤引导的第二步".into()),
                                    target: Some("#step-2".into()),
                                    ..Default::default()
                                },
                                TourStep {
                                    title: Some("第三步".into()),
                                    description: Some("这是多步骤引导的第三步".into()),
                                    target: Some("#step-3".into()),
                                    ..Default::default()
                                }
                            ],
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
                            onclick: move |_| masked_tour_open.set(true),
                            "重要功能"
                        }

                        // 带遮罩的引导示例的Tour组件
                        Tour {
                            open: *masked_tour_open.read(),
                            on_close: move |_| masked_tour_open.set(false),
                            mask: MaskConfig {
                                show: true,
                                color: Some("rgba(0, 0, 0, 0.5)".into()),
                                ..Default::default()
                            },
                            steps: vec![
                                TourStep {
                                    title: Some("重要提示".into()),
                                    description: Some("这是一个重要的功能按钮，请注意使用".into()),
                                    target: Some("#tour-target-mask".into()),
                                    ..Default::default()
                                }
                            ],
                        }
                    }
                }
            }
        }
    }
}

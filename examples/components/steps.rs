#![allow(non_snake_case)]
//!
//! 展示 Steps 组件的各种用法和样式

use crate::common::*;
use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

/// Steps 组件演示
#[component]
pub fn StepsDemo() -> Element {
    let mut current = use_signal(|| 0);
    let mut current_vertical = use_signal(|| 0);
    let mut current_error = use_signal(|| 1);

    rsx! {
        div {
            class: "component-showcase",

            h1 {
                style: "margin: 0 0 16px 0; color: #262626; font-size: 28px;",
                "Steps 步骤条"
            }

            p {
                style: "margin: 0 0 24px 0; color: #666; font-size: 14px;",
                "引导用户按照流程完成任务的导航条。"
            }

            // 基础用法
            DemoSection {
                title: "基础用法",
                description: "简单的步骤条。",

                div {
                    style: "padding: 24px; background: #fafafa; border-radius: 6px;",

                    Steps {
                        current: current(),
                        items: vec![
                            StepItem {
                                title: "Finished".to_string(),
                                description: Some("This is a description.".to_string()),
                                                                subtitle: None,
                                icon: None,
                                status: None,
                                disabled: false,
                            },
                            StepItem {
                                title: "In Progress".to_string(),
                                description: Some("This is a description.".to_string()),
                                // sub_title: "Left 00:00:08".to_string(),
                                                                subtitle: None,
                                icon: None,
                                status: None,
                                disabled: false,
                            },
                            StepItem {
                                title: "Waiting".to_string(),
                                description: Some("This is a description.".to_string()),
                                                                subtitle: None,
                                icon: None,
                                status: None,
                                disabled: false,
                            },
                        ]
                    }
                }
            }

            // 迷你版
            DemoSection {
                title: "迷你版",
                description: "迷你版的步骤条，通过设置 size=\"small\" 启用。",

                div {
                    style: "padding: 24px; background: #fafafa; border-radius: 6px;",

                    Steps {
                        size: StepsSize::Small,
                        current: 1,
                        items: vec![
                            StepItem {
                                title: "Finished".to_string(),
                                description: Some("This is a description.".to_string()),
                                                                subtitle: None,
                                icon: None,
                                status: None,
                                disabled: false,
                            },
                            StepItem {
                                title: "In Progress".to_string(),
                                description: Some("This is a description.".to_string()),
                                                                subtitle: None,
                                icon: None,
                                status: None,
                                disabled: false,
                            },
                            StepItem {
                                title: "Waiting".to_string(),
                                description: Some("This is a description.".to_string()),
                                subtitle: None,
                                icon: None,
                                status: None,
                                disabled: false,
                            },
                        ]
                    }
                }
            }

            // 带图标的步骤条
            DemoSection {
                title: "带图标的步骤条",
                description: "通过设置 Steps.Step 的 icon 属性，可以启用自定义图标。",

                div {
                    style: "padding: 24px; background: #fafafa; border-radius: 6px;",

                    Steps {
                        items: vec![
                            StepItem {
                                title: "Login".to_string(),
                                description: Some("This is a description.".to_string()),
                                status: Some(StepStatus::Finish),
                                icon: Some(rsx!{
                                    Icon {
                                        icon_type: IconType::UserOutlined
                                    }
                                }),
                                subtitle: None,
                                disabled: false,
                            },
                            StepItem {
                                title: "Verification".to_string(),
                                description: Some("This is a description.".to_string()),
                                status: Some(StepStatus::Finish),
                                icon: Some(rsx! {
                                    Icon {
                                        icon_type: IconType::SolutionOutlined
                                    }
                                }),
                                subtitle: None,
                                disabled: false,
                            },
                            StepItem {
                                title: "Pay".to_string(),
                                description: Some("This is a description.".to_string()),
                                status: Some(StepStatus::Finish),
                                icon: Some(rsx! {
                                    Icon {
                                        icon_type: IconType::LoadingOutlined
                                    }
                                }),
                                subtitle: None,
                                disabled: false,
                            },
                            StepItem {
                                title: "Done".to_string(),
                                description: Some("This is a description.".to_string()),
                                status: Some(StepStatus::Wait),
                                icon: Some(rsx! {
                                    Icon {
                                        icon_type: IconType::SmileOutlined
                                    }
                                }),
                                subtitle: None,
                                disabled: false,
                            },
                        ]
                    }
                }
            }

            // 切换步骤
            DemoSection {
                title: "切换步骤",
                description: "通常配合内容及按钮使用，表示一个流程的处理进度。",

                div {
                    style: "padding: 24px; background: #fafafa; border-radius: 6px;",

                    Steps {
                        current: current(),
                        items: vec![
                            StepItem {
                                title: "First".to_string(),
                                description: Some("This is a description.".to_string()),
                                subtitle: None,
                                icon: None,
                                status: None,
                                disabled: false,
                            },
                            StepItem {
                                title: "Second".to_string(),
                                description: Some("This is a description.".to_string()),
                                subtitle: None,
                                icon: None,
                                status: None,
                                disabled: false,
                            },
                            StepItem {
                                title: "Last".to_string(),
                                description: Some("This is a description.".to_string()),
                                subtitle: None,
                                icon: None,
                                status: None,
                                disabled: false,
                            },
                        ]
                    }

                    div {
                        style: "margin-top: 24px; padding: 24px; background: white; border: 1px solid #d9d9d9; border-radius: 6px;",
                        match current() {
                            0 => rsx! { "First-content" },
                            1 => rsx! { "Second-content" },
                            2 => rsx! { "Last-content" },
                            _ => rsx! { "" }
                        }
                    }

                    div {
                        style: "margin-top: 24px; display: flex; gap: 8px;",
                        if current() < 2 {
                                Button {
                                    button_type: ButtonType::Primary,
                                    onclick: move |_| {
                                        current.set(current() + 1);
                                    },
                                    "Next"
                                }
                        }
                        if current() > 0 {
                                Button {
                                    onclick: move |_| {
                                        current.set(current() - 1);
                                    },
                                    "Previous"
                                }
                        }
                        if current() == 2 {
                                Button {
                                    button_type: ButtonType::Primary,
                                    onclick: move |_| {
                                        MessageManager::new().success("Processing complete!", Some(1000 as f64));
                                    },
                                    "Done"
                                }
                        }
                    }
                }
            }

            // 竖直方向的步骤条
            DemoSection {
                title: "竖直方向的步骤条",
                description: "简单的竖直方向的步骤条。",

                div {
                    style: "padding: 24px; background: #fafafa; border-radius: 6px;",

                    Steps {
                        direction: StepsDirection::Vertical,
                        current: current_vertical(),
                        items: vec![
                            StepItem {
                                title: "Finished".to_string(),
                                description: Some("This is a description.".to_string()),
                                                                subtitle: None,
                                icon: None,
                                status: None,
                                disabled: false,
                            },
                            StepItem {
                                title: "In Progress".to_string(),
                                description: Some("This is a description.".to_string()),
                                                                subtitle: None,
                                icon: None,
                                status: None,
                                disabled: false,
                            },
                            StepItem {
                                title: "Waiting".to_string(),
                                description: Some("This is a description.".to_string()),
                                                                subtitle: None,
                                icon: None,
                                status: None,
                                disabled: false,
                            },
                        ]
                    }
                }
            }

            // 竖直方向的小型步骤条
            DemoSection {
                title: "竖直方向的小型步骤条",
                description: "简单的竖直方向的小型步骤条。",

                div {
                    style: "padding: 24px; background: #fafafa; border-radius: 6px;",

                    Steps {
                        direction: StepsDirection::Vertical,
                        size: StepsSize::Small,
                        current: 1,
                        items: vec![
                            StepItem {
                                title: "Finished".to_string(),
                                description: Some("This is a description.".to_string()),
                                                                subtitle: None,
                                icon: None,
                                status: None,
                                disabled: false,
                            },
                            StepItem {
                                title: "In Progress".to_string(),
                                description: Some("This is a description.".to_string()),
                                                                subtitle: None,
                                icon: None,
                                status: None,
                                disabled: false,
                            },
                            StepItem {
                                title: "Waiting".to_string(),
                                description: Some("This is a description.".to_string()),
                                                                subtitle: None,
                                icon: None,
                                status: None,
                                disabled: false,
                            },
                        ]
                    }
                }
            }

            // 步骤运行错误
            DemoSection {
                title: "步骤运行错误",
                description: "使用 Steps 的 status 属性来指定当前步骤的状态。",

                div {
                    style: "padding: 24px; background: #fafafa; border-radius: 6px;",

                    Steps {
                        current: current_error(),
                        status: StepStatus::Error,
                        items: vec![
                            StepItem {
                                title: "Finished".to_string(),
                                description: Some("This is a description".to_string()),
                                                                subtitle: None,
                                icon: None,
                                status: None,
                                disabled: false,
                            },
                            StepItem {
                                title: "In Process".to_string(),
                                description: Some("This is a description".to_string()),
                                                                subtitle: None,
                                icon: None,
                                status: None,
                                disabled: false,
                            },
                            StepItem {
                                title: "Waiting".to_string(),
                                description: Some("This is a description".to_string()),
                                                                subtitle: None,
                                icon: None,
                                status: None,
                                disabled: false,
                            },
                        ]
                    }
                }
            }

            // 点状步骤条
            DemoSection {
                title: "点状步骤条",
                description: "包含步骤点的进度条。",

                div {
                    style: "padding: 24px; background: #fafafa; border-radius: 6px;",

                    Steps {
                        progress_dot: true,
                        current: 1,
                        items: vec![
                            StepItem {
                                title: "Finished".to_string(),
                                description: Some("You can hover on the dot.".to_string()),
                                                                subtitle: None,
                                icon: None,
                                status: None,
                                disabled: false,
                            },
                            StepItem {
                                title: "In Progress".to_string(),
                                description: Some("You can hover on the dot.".to_string()),
                                                                subtitle: None,
                                icon: None,
                                status: None,
                                disabled: false,
                            },
                            StepItem {
                                title: "Waiting".to_string(),
                                description: Some("You can hover on the dot.".to_string()),
                                                                subtitle: None,
                                icon: None,
                                status: None,
                                disabled: false,
                            },
                            StepItem {
                                title: "Waiting".to_string(),
                                description: Some("You can hover on the dot.".to_string()),
                                                                subtitle: None,
                                icon: None,
                                status: None,
                                disabled: false,
                            },
                        ]
                    }
                }
            }

            // 可点击
            DemoSection {
                title: "可点击",
                description: "设置 onChange 后，Steps 变为可点击状态。",

                div {
                    style: "padding: 24px; background: #fafafa; border-radius: 6px;",

                    Steps {
                        current: current(),
                        on_change: move |step| {
                            current.set(step);
                        },
                        items: vec![
                            StepItem {
                                title: "Step 1".to_string(),
                                description: Some("This is a description.".to_string()),
                                                                subtitle: None,
                                icon: None,
                                status: None,
                                disabled: false,
                            },
                            StepItem {
                                title: "Step 2".to_string(),
                                description: Some("This is a description.".to_string()),
                                                                subtitle: None,
                                icon: None,
                                status: None,
                                disabled: false,
                            },
                            StepItem {
                                title: "Step 3".to_string(),
                                description: Some("This is a description.".to_string()),
                                                                subtitle: None,
                                icon: None,
                                status: None,
                                disabled: false,
                            },
                        ]
                    }

                    div {
                        style: "margin-top: 24px; padding: 24px; background: white; border: 1px solid #d9d9d9; border-radius: 6px;",
                        "Content of step {current() + 1}"
                    }
                }
            }

            // API 文档
            ApiDocumentation {
                component_name: "Steps",
                props: vec![
                    PropDoc {
                        name: "current".to_string(),
                        prop_type: "i32".to_string(),
                        default: "0".to_string(),
                        description: "指定当前步骤，从 0 开始记数。在子 Step 元素中，可以通过 status 属性覆盖状态".to_string(),
                    },
                    PropDoc {
                        name: "direction".to_string(),
                        prop_type: "String".to_string(),
                        default: "horizontal".to_string(),
                        description: "指定步骤条方向。目前支持水平（horizontal）和竖直（vertical）两种方向".to_string(),
                    },
                    PropDoc {
                        name: "label_placement".to_string(),
                        prop_type: "String".to_string(),
                        default: "horizontal".to_string(),
                        description: "指定标签放置位置，默认水平放图标右侧，可选 vertical 放图标下方".to_string(),
                    },
                    PropDoc {
                        name: "progress_dot".to_string(),
                        prop_type: "bool | Function".to_string(),
                        default: "false".to_string(),
                        description: "点状步骤条，可以设置为一个 function，labelPlacement 将强制为 vertical".to_string(),
                    },
                    PropDoc {
                        name: "responsive".to_string(),
                        prop_type: "bool".to_string(),
                        default: "true".to_string(),
                        description: "当屏幕宽度小于 532px 时自动变为垂直模式".to_string(),
                    },
                    PropDoc {
                        name: "size".to_string(),
                        prop_type: "String".to_string(),
                        default: "default".to_string(),
                        description: "指定大小，目前支持普通（default）和迷你（small）".to_string(),
                    },
                    PropDoc {
                        name: "status".to_string(),
                        prop_type: "String".to_string(),
                        default: "process".to_string(),
                        description: "指定当前步骤的状态，可选 wait process finish error".to_string(),
                    },
                    PropDoc {
                        name: "type".to_string(),
                        prop_type: "String".to_string(),
                        default: "default".to_string(),
                        description: "步骤条类型，有 default 和 navigation 两种".to_string(),
                    },
                    PropDoc {
                        name: "on_change".to_string(),
                        prop_type: "Function".to_string(),
                        default: "-".to_string(),
                        description: "点击切换步骤时触发".to_string(),
                    },
                    PropDoc {
                        name: "items".to_string(),
                        prop_type: "Vec<StepItem>".to_string(),
                        default: "-".to_string(),
                        description: "配置选项卡内容".to_string(),
                    },
                ]
            }

            // Steps.Step API
            ApiDocumentation {
                component_name: "Steps.Step",
                props: vec![
                    PropDoc {
                        name: "description".to_string(),
                        prop_type: "String | Element".to_string(),
                        default: "-".to_string(),
                        description: "步骤的详情描述，可选".to_string(),
                    },
                    PropDoc {
                        name: "disabled".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "禁用点击".to_string(),
                    },
                    PropDoc {
                        name: "icon".to_string(),
                        prop_type: "Element".to_string(),
                        default: "-".to_string(),
                        description: "步骤图标的类型，可选".to_string(),
                    },
                    PropDoc {
                        name: "status".to_string(),
                        prop_type: "String".to_string(),
                        default: "wait".to_string(),
                        description: "指定状态。当不配置该属性时，会使用 Steps 的 current 来自动指定状态。可选：wait process finish error".to_string(),
                    },
                    PropDoc {
                        name: "sub_title".to_string(),
                        prop_type: "String | Element".to_string(),
                        default: "-".to_string(),
                        description: "子标题".to_string(),
                    },
                    PropDoc {
                        name: "title".to_string(),
                        prop_type: "String | Element".to_string(),
                        default: "-".to_string(),
                        description: "标题".to_string(),
                    },
                ]
            }
        }
    }
}

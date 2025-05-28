//!
//!
//! 展示 Tooltip 组件的各种用法和样式

use crate::common::*;
use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

/// Tooltip 组件演示
#[component]
pub fn TooltipDemo() -> Element {
    rsx! {
        div { class: "tooltip-demo",
            h1 { "Tooltip 文字提示" }
            p { "简单的文字提示气泡框。" }

            DemoSection {
                title: "基本用法",
                description: "最简单的用法。",

                div {
                    Tooltip {
                        title: "提示文字".to_string(),
                        Button {
                            "鼠标移入"
                        }
                    }
                }
            }

            DemoSection {
                title: "位置",
                description: "位置有 12 个方向。",

                div {
                    style: "display: flex; gap: 8px; flex-wrap: wrap;",
                    Tooltip {
                        title: "提示文字".to_string(),
                        placement: TooltipPlacement::Top,
                        Button {
                            "上"
                        }
                    }
                    Tooltip {
                        title: "提示文字".to_string(),
                        placement: TooltipPlacement::Left,
                        Button {
                            "左"
                        }
                    }
                    Tooltip {
                        title: "提示文字".to_string(),
                        placement: TooltipPlacement::Right,
                        Button {
                            "右"
                        }
                    }
                    Tooltip {
                        title: "提示文字".to_string(),
                        placement: TooltipPlacement::Bottom,
                        Button {
                            "下"
                        }
                    }
                }
            }

            DemoSection {
                title: "触发方式",
                description: "鼠标移入、聚集、点击。",

                div {
                    Space {
                        Tooltip {
                            title: "提示文字".to_string(),
                            trigger: TooltipTrigger::Hover,
                            Button { "鼠标移入" }
                        }
                        Tooltip {
                            title: "提示文字".to_string(),
                            trigger: TooltipTrigger::Focus,
                            Button { "聚焦" }
                        }
                        Tooltip {
                            title: "提示文字".to_string(),
                            trigger: TooltipTrigger::Click,
                            Button { "点击" }
                        }
                    }
                }
            }

            DemoSection {
                title: "多彩文字提示",
                description: "我们添加了多种预设色彩的文字提示样式，用作不同场景使用。",

                div {
                    Space {
                        direction: SpaceDirection::Vertical,
                        div {
                            h4 { "预设颜色" }
                            Space {
                                Tooltip {
                                    title: "提示文字".to_string(),
                                    color: TooltipColor::Custom(String::from("Pink")),
                                    Button { "Pink" }
                                }
                                Tooltip {
                                    title: "提示文字".to_string(),
                                    color: TooltipColor::Custom(String::from("Red")),
                                    Button { "Red" }
                                }
                                Tooltip {
                                    title: "提示文字".to_string(),
                                    color: TooltipColor::Custom(String::from("Yellow")),
                                    Button { "Yellow" }
                                }
                                Tooltip {
                                    title: "提示文字".to_string(),
                                    color: TooltipColor::Custom(String::from("Orange")),
                                    Button { "Orange" }
                                }
                                Tooltip {
                                    title: "提示文字".to_string(),
                                    color: TooltipColor::Custom(String::from("Cyan")),
                                    Button { "Cyan" }
                                }
                                Tooltip {
                                    title: "提示文字".to_string(),
                                    color: TooltipColor::Custom(String::from("Green")),
                                    Button { "Green" }
                                }
                                Tooltip {
                                    title: "提示文字".to_string(),
                                    color: TooltipColor::Custom(String::from("Blue")),
                                    Button { "Blue" }
                                }
                                Tooltip {
                                    title: "提示文字".to_string(),
                                    color: TooltipColor::Custom(String::from("Purple")),
                                    Button { "Purple" }
                                }
                                Tooltip {
                                    title: "提示文字".to_string(),
                                    color: TooltipColor::Custom(String::from("Geekblue")),
                                    Button { "Geekblue" }
                                }
                                Tooltip {
                                    title: "提示文字".to_string(),
                                    color: TooltipColor::Custom(String::from("Magenta")),
                                    Button { "Magenta" }
                                }
                                Tooltip {
                                    title: "提示文字".to_string(),
                                    color: TooltipColor::Custom(String::from("Volcano")),
                                    Button { "Volcano" }
                                }
                                Tooltip {
                                    title: "提示文字".to_string(),
                                    color: TooltipColor::Custom(String::from("Gold")),
                                    Button { "Gold" }
                                }
                                Tooltip {
                                    title: "提示文字".to_string(),
                                    color: TooltipColor::Custom(String::from("Lime")),
                                    Button { "Lime" }
                                }
                            }
                        }
                        div {
                            h4 { "自定义颜色" }
                            Space {
                                Tooltip {
                                    title: "提示文字".to_string(),
                                    color: TooltipColor::Custom("#f50".to_string()),
                                    Button { "#f50" }
                                }
                                Tooltip {
                                    title: "提示文字".to_string(),
                                    color: TooltipColor::Custom("#2db7f5".to_string()),
                                    Button { "#2db7f5" }
                                }
                                Tooltip {
                                    title: "提示文字".to_string(),
                                    color: TooltipColor::Custom("#87d068".to_string()),
                                    Button { "#87d068" }
                                }
                                Tooltip {
                                    title: "提示文字".to_string(),
                                    color: TooltipColor::Custom("#108ee9".to_string()),
                                    Button { "#108ee9" }
                                }
                            }
                        }
                    }
                }
            }

            DemoSection {
                title: "延时",
                description: "鼠标移入后延时出现，鼠标移出后延时消失。",

                div {
                    Tooltip {
                        title: "提示文字".to_string(),
                        mouse_enter_delay: 1.0,
                        mouse_leave_delay: 1.0,
                        Button { "延时1秒显示" }
                    }
                }
            }
        }
    }
}

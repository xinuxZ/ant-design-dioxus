//! Space 组件演示
//!
//! 展示 Space 组件的各种用法，包括间距设置、方向控制、对齐方式等

use crate::common::*;
use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

/// Space 组件演示
#[component]
pub fn SpaceDemo() -> Element {
    rsx! {
        div {
            class: "component-showcase",

            h1 {
                style: "margin: 0 0 16px 0; color: #262626; font-size: 28px;",
                "Space 间距"
            }

            p {
                style: "margin: 0 0 24px 0; color: #666; font-size: 14px;",
                "设置组件之间的间距。"
            }

            // 基础用法
            DemoSection {
                title: "基础用法",
                description: "相邻组件水平间距。",

                div {
                    style: "border: 1px solid #d9d9d9; padding: 16px; border-radius: 6px;",
                    Space {
                        Button {
                            button_type: ButtonType::Primary,
                            "Primary"
                        }
                        Button {
                            "Default"
                        }
                        Button {
                            button_type: ButtonType::Dashed,
                            "Dashed"
                        }
                        Button {
                            button_type: ButtonType::Link,
                            "Link"
                        }
                    }
                }
            }

            // 垂直间距
            DemoSection {
                title: "垂直间距",
                description: "相邻组件垂直间距。",

                div {
                    style: "border: 1px solid #d9d9d9; padding: 16px; border-radius: 6px;",
                    Space {
                        direction: SpaceDirection::Vertical,
                        Button {
                            button_type: ButtonType::Primary,
                            "Primary"
                        }
                        Button {
                            "Default"
                        }
                        Button {
                            button_type: ButtonType::Dashed,
                            "Dashed"
                        }
                        Button {
                            button_type: ButtonType::Link,
                            "Link"
                        }
                    }
                }
            }

            // 间距大小
            DemoSection {
                title: "间距大小",
                description: "间距预设大小，可以设置为 small、middle、large 或自定义数值。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    div {
                        h4 { "Small" }
                        div {
                            style: "border: 1px solid #d9d9d9; padding: 16px; border-radius: 6px;",
                            Space {
                                size: SpaceSize::Small,
                                Button {
                                    "Button"
                                }
                                Button {
                                    "Button"
                                }
                                Button {
                                    "Button"
                                }
                            }
                        }
                    }

                    div {
                        h4 { "Middle" }
                        div {
                            style: "border: 1px solid #d9d9d9; padding: 16px; border-radius: 6px;",
                            Space {
                                size: SpaceSize::Middle,
                                Button {
                                    "Button"
                                }
                                Button {
                                    "Button"
                                }
                                Button {
                                    "Button"
                                }
                            }
                        }
                    }

                    div {
                        h4 { "Large" }
                        div {
                            style: "border: 1px solid #d9d9d9; padding: 16px; border-radius: 6px;",
                            Space {
                                size: SpaceSize::Large,
                                Button {
                                    "Button"
                                }
                                Button {
                                    "Button"
                                }
                                Button {
                                    "Button"
                                }
                            }
                        }
                    }

                    div {
                        h4 { "自定义大小 (32px)" }
                        div {
                            style: "border: 1px solid #d9d9d9; padding: 16px; border-radius: 6px;",
                            Space {
                                size: SpaceSize::Custom(32),
                                Button {
                                    "Button"
                                }
                                Button {
                                    "Button"
                                }
                                Button {
                                    "Button"
                                }
                            }
                        }
                    }
                }
            }

            // 对齐方式
            DemoSection {
                title: "对齐方式",
                description: "设置对齐模式。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    div {
                        h4 { "start" }
                        div {
                            style: "border: 1px solid #d9d9d9; padding: 16px; border-radius: 6px; height: 80px;",
                            Space {
                                align: SpaceAlign::Start,
                                Button {
                                    "Button"
                                }
                                div {
                                    style: "padding: 16px; background: #f0f0f0; border-radius: 6px; height: 60px;",
                                    "Block"
                                }
                                Button {
                                    "Button"
                                }
                            }
                        }
                    }

                    div {
                        h4 { "center" }
                        div {
                            style: "border: 1px solid #d9d9d9; padding: 16px; border-radius: 6px; height: 80px;",
                            Space {
                                align: SpaceAlign::Center,
                                Button {
                                    "Button"
                                }
                                div {
                                    style: "padding: 16px; background: #f0f0f0; border-radius: 6px; height: 60px;",
                                    "Block"
                                }
                                Button {
                                    "Button"
                                }
                            }
                        }
                    }

                    div {
                        h4 { "end" }
                        div {
                            style: "border: 1px solid #d9d9d9; padding: 16px; border-radius: 6px; height: 80px;",
                            Space {
                                align: SpaceAlign::End,
                                Button {
                                    "Button"
                                }
                                div {
                                    style: "padding: 16px; background: #f0f0f0; border-radius: 6px; height: 60px;",
                                    "Block"
                                }
                                Button {
                                    "Button"
                                }
                            }
                        }
                    }

                    div {
                        h4 { "baseline" }
                        div {
                            style: "border: 1px solid #d9d9d9; padding: 16px; border-radius: 6px; height: 80px;",
                            Space {
                                align: SpaceAlign::Baseline,
                                Button {
                                    "Button"
                                }
                                div {
                                    style: "padding: 16px; background: #f0f0f0; border-radius: 6px; height: 60px; font-size: 20px;",
                                    "Block"
                                }
                                Button {
                                    "Button"
                                }
                            }
                        }
                    }
                }
            }

            // 自动换行
            DemoSection {
                title: "自动换行",
                description: "自动换行。",

                div {
                    style: "border: 1px solid #d9d9d9; padding: 16px; border-radius: 6px; width: 300px;",
                    Space {
                        wrap: true,
                        for i in 0..20 {
                            Button {
                                key: "{i}",
                                "Button {i + 1}"
                            }
                        }
                    }
                }
            }

            // 分隔符
            DemoSection {
                title: "分隔符",
                description: "相邻组件分隔符。",

                div {
                    style: "border: 1px solid #d9d9d9; padding: 16px; border-radius: 6px;",
                    Space {
                        split: rsx! {
                            Divider {
                                r#type: DividerType::Vertical
                            }
                        },
                        a {
                            href: "#",
                            style: "color: #1677ff; text-decoration: none;",
                            "Link"
                        }
                        a {
                            href: "#",
                            style: "color: #1677ff; text-decoration: none;",
                            "Link"
                        }
                        a {
                            href: "#",
                            style: "color: #1677ff; text-decoration: none;",
                            "Link"
                        }
                    }
                }
            }

            // 紧凑模式
            DemoSection {
                title: "紧凑模式",
                description: "紧凑模式下的间距。",

                div {
                    style: "border: 1px solid #d9d9d9; padding: 16px; border-radius: 6px;",
                    Space {
                        size: SpaceSize::Custom(4),
                        Button {
                            size: ButtonSize::Small,
                            "Small"
                        }
                        Button {
                            size: ButtonSize::Small,
                            "Small"
                        }
                        Button {
                            size: ButtonSize::Small,
                            "Small"
                        }
                    }
                }
            }
        }
    }
}

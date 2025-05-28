//! Space 组件示例
//!
//! 展示 Space 间距组件的各种用法

use crate::common::{ApiDocumentation, DemoSection, PropDoc};
use dioxus::prelude::*;

/// Space 组件演示页面
#[component]
pub fn SpaceDemo() -> Element {
    rsx! {
        div {
            class: "space-demo",
            style: "padding: 24px;",

            h1 {
                style: "margin: 0 0 24px 0; color: #262626; font-size: 28px;",
                "Space 间距"
            }

            p {
                style: "margin: 0 0 32px 0; color: #666; font-size: 14px;",
                "设置组件之间的间距。"
            }

            // 基础用法
            DemoSection {
                title: "基础用法".to_string(),
                description: "相邻组件水平间距。".to_string(),
                div {
                    style: "display: flex; gap: 8px; align-items: center;",

                    button {
                        style: "padding: 4px 15px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer;",
                        "Button"
                    }
                    button {
                        style: "padding: 4px 15px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer;",
                        "Button"
                    }
                    button {
                        style: "padding: 4px 15px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer;",
                        "Button"
                    }
                }
            }

            // 垂直间距
            DemoSection {
                title: "垂直间距".to_string(),
                description: "相邻组件垂直间距。".to_string(),
                div {
                    style: "display: flex; flex-direction: column; gap: 8px; align-items: flex-start;",

                    button {
                        style: "padding: 4px 15px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer;",
                        "Button"
                    }
                    button {
                        style: "padding: 4px 15px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer;",
                        "Button"
                    }
                    button {
                        style: "padding: 4px 15px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer;",
                        "Button"
                    }
                }
            }

            // 间距大小
            DemoSection {
                title: "间距大小".to_string(),
                description: "间距预设大小。".to_string(),
                div {
                    style: "display: flex; flex-direction: column; gap: 24px;",

                    div {
                        h4 { style: "margin: 0 0 8px 0; color: #666;", "Small" }
                        div {
                            style: "display: flex; gap: 4px; align-items: center;",
                            button {
                                style: "padding: 4px 15px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer;",
                                "Button"
                            }
                            button {
                                style: "padding: 4px 15px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer;",
                                "Button"
                            }
                            button {
                                style: "padding: 4px 15px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer;",
                                "Button"
                            }
                        }
                    }

                    div {
                        h4 { style: "margin: 0 0 8px 0; color: #666;", "Middle" }
                        div {
                            style: "display: flex; gap: 8px; align-items: center;",
                            button {
                                style: "padding: 4px 15px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer;",
                                "Button"
                            }
                            button {
                                style: "padding: 4px 15px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer;",
                                "Button"
                            }
                            button {
                                style: "padding: 4px 15px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer;",
                                "Button"
                            }
                        }
                    }

                    div {
                        h4 { style: "margin: 0 0 8px 0; color: #666;", "Large" }
                        div {
                            style: "display: flex; gap: 16px; align-items: center;",
                            button {
                                style: "padding: 4px 15px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer;",
                                "Button"
                            }
                            button {
                                style: "padding: 4px 15px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer;",
                                "Button"
                            }
                            button {
                                style: "padding: 4px 15px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer;",
                                "Button"
                            }
                        }
                    }
                }
            }

            // 对齐
            DemoSection {
                title: "对齐".to_string(),
                description: "设置对齐模式。".to_string(),
                div {
                    style: "display: flex; flex-direction: column; gap: 24px;",

                    div {
                        h4 { style: "margin: 0 0 8px 0; color: #666;", "start" }
                        div {
                            style: "display: flex; gap: 8px; align-items: flex-start; background: #f5f5f5; padding: 16px;",
                            button {
                                style: "padding: 4px 15px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer; height: 32px;",
                                "Button"
                            }
                            button {
                                style: "padding: 4px 15px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer; height: 40px;",
                                "Button"
                            }
                            button {
                                style: "padding: 4px 15px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer; height: 48px;",
                                "Button"
                            }
                        }
                    }

                    div {
                        h4 { style: "margin: 0 0 8px 0; color: #666;", "center" }
                        div {
                            style: "display: flex; gap: 8px; align-items: center; background: #f5f5f5; padding: 16px;",
                            button {
                                style: "padding: 4px 15px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer; height: 32px;",
                                "Button"
                            }
                            button {
                                style: "padding: 4px 15px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer; height: 40px;",
                                "Button"
                            }
                            button {
                                style: "padding: 4px 15px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer; height: 48px;",
                                "Button"
                            }
                        }
                    }

                    div {
                        h4 { style: "margin: 0 0 8px 0; color: #666;", "end" }
                        div {
                            style: "display: flex; gap: 8px; align-items: flex-end; background: #f5f5f5; padding: 16px;",
                            button {
                                style: "padding: 4px 15px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer; height: 32px;",
                                "Button"
                            }
                            button {
                                style: "padding: 4px 15px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer; height: 40px;",
                                "Button"
                            }
                            button {
                                style: "padding: 4px 15px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer; height: 48px;",
                                "Button"
                            }
                        }
                    }

                    div {
                        h4 { style: "margin: 0 0 8px 0; color: #666;", "baseline" }
                        div {
                            style: "display: flex; gap: 8px; align-items: baseline; background: #f5f5f5; padding: 16px;",
                            button {
                                style: "padding: 4px 15px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer; height: 32px;",
                                "Button"
                            }
                            button {
                                style: "padding: 4px 15px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer; height: 40px;",
                                "Button"
                            }
                            button {
                                style: "padding: 4px 15px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer; height: 48px;",
                                "Button"
                            }
                        }
                    }
                }
            }

            // 自动换行
            DemoSection {
                title: "自动换行".to_string(),
                description: "自动换行。".to_string(),
                div {
                    style: "display: flex; gap: 8px; flex-wrap: wrap; width: 300px; background: #f5f5f5; padding: 16px;",

                    for i in 1..=20 {
                        button {
                            style: "padding: 4px 15px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer;",
                            "Button {i}"
                        }
                    }
                }
            }

            // 分隔符
            DemoSection {
                title: "分隔符".to_string(),
                description: "相邻组件分隔符。".to_string(),
                div {
                    style: "display: flex; align-items: center;",

                    span { "Link" }
                    span { style: "margin: 0 8px; color: #d9d9d9;", "|" }
                    span { "Link" }
                    span { style: "margin: 0 8px; color: #d9d9d9;", "|" }
                    span { "Link" }
                }
            }

            // 紧凑模式
            DemoSection {
                title: "紧凑模式".to_string(),
                description: "紧凑模式下，间距会变小。".to_string(),
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    div {
                        h4 { style: "margin: 0 0 8px 0; color: #666;", "Normal" }
                        div {
                            style: "display: flex; gap: 8px; align-items: center;",
                            button {
                                style: "padding: 4px 15px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer;",
                                "Button"
                            }
                            button {
                                style: "padding: 4px 15px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer;",
                                "Button"
                            }
                            button {
                                style: "padding: 4px 15px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer;",
                                "Button"
                            }
                        }
                    }

                    div {
                        h4 { style: "margin: 0 0 8px 0; color: #666;", "Compact" }
                        div {
                            style: "display: flex; gap: 4px; align-items: center;",
                            button {
                                style: "padding: 4px 15px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer;",
                                "Button"
                            }
                            button {
                                style: "padding: 4px 15px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer;",
                                "Button"
                            }
                            button {
                                style: "padding: 4px 15px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer;",
                                "Button"
                            }
                        }
                    }
                }
            }

            // API 文档
            ApiDocumentation {
                component_name: "Space".to_string(),
                props: vec![
                    PropDoc {
                        name: "align".to_string(),
                        prop_type: "String".to_string(),
                        default: "".to_string(),
                        description: "对齐方式：start | end | center | baseline".to_string(),
                    },
                    PropDoc {
                        name: "direction".to_string(),
                        prop_type: "String".to_string(),
                        default: "horizontal".to_string(),
                        description: "间距方向：vertical | horizontal".to_string(),
                    },
                    PropDoc {
                        name: "size".to_string(),
                        prop_type: "String | Number".to_string(),
                        default: "small".to_string(),
                        description: "间距大小：small | middle | large | number".to_string(),
                    },
                    PropDoc {
                        name: "split".to_string(),
                        prop_type: "Element".to_string(),
                        default: "-".to_string(),
                        description: "设置拆分".to_string(),
                    },
                    PropDoc {
                        name: "wrap".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "是否自动换行，仅在 horizontal 时有效".to_string(),
                    },
                ]
            }
        }
    }
}

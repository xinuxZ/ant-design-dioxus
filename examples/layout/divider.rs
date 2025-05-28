//! Divider 组件示例
//!
//! 展示 Divider 分割线组件的各种用法

use crate::common::{ApiDocumentation, DemoSection, PropDoc};
use dioxus::prelude::*;

/// Divider 组件演示页面
#[component]
pub fn DividerDemo() -> Element {
    rsx! {
        div {
            class: "divider-demo",
            style: "padding: 24px;",

            h1 {
                style: "margin: 0 0 24px 0; color: #262626; font-size: 28px;",
                "Divider 分割线"
            }

            p {
                style: "margin: 0 0 32px 0; color: #666; font-size: 14px;",
                "区隔内容的分割线。"
            }

            // 水平分割线
            DemoSection {
                title: "水平分割线".to_string(),
                description: "默认为水平分割线，可在中间加入文字。".to_string(),
                div {
                    p {
                        style: "margin: 0;",
                        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed nonne merninisti licere mihi ista probare, quae sunt a te dicta? Refert tamen, quo modo."
                    }
                    hr {
                        style: "border: none; border-top: 1px solid #f0f0f0; margin: 16px 0;"
                    }
                    p {
                        style: "margin: 0;",
                        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed nonne merninisti licere mihi ista probare, quae sunt a te dicta? Refert tamen, quo modo."
                    }
                    div {
                        style: "position: relative; margin: 16px 0; text-align: center;",
                        hr {
                            style: "border: none; border-top: 1px solid #f0f0f0; margin: 0;"
                        }
                        span {
                            style: "position: absolute; top: -8px; left: 50%; transform: translateX(-50%); background: white; padding: 0 16px; color: #666; font-size: 14px;",
                            "Text"
                        }
                    }
                    p {
                        style: "margin: 0;",
                        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed nonne merninisti licere mihi ista probare, quae sunt a te dicta? Refert tamen, quo modo."
                    }
                }
            }

            // 带文字的分割线
            DemoSection {
                title: "带文字的分割线".to_string(),
                description: "分割线中带有文字，可以用 orientation 指定文字位置。".to_string(),
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    div {
                        h4 { style: "margin: 0 0 8px 0; color: #666;", "Left" }
                        div {
                            style: "position: relative; text-align: left;",
                            hr {
                                style: "border: none; border-top: 1px solid #f0f0f0; margin: 0;"
                            }
                            span {
                                style: "position: absolute; top: -8px; left: 0; background: white; padding: 0 16px 0 0; color: #666; font-size: 14px;",
                                "Left Text"
                            }
                        }
                    }

                    div {
                        h4 { style: "margin: 0 0 8px 0; color: #666;", "Center" }
                        div {
                            style: "position: relative; text-align: center;",
                            hr {
                                style: "border: none; border-top: 1px solid #f0f0f0; margin: 0;"
                            }
                            span {
                                style: "position: absolute; top: -8px; left: 50%; transform: translateX(-50%); background: white; padding: 0 16px; color: #666; font-size: 14px;",
                                "Center Text"
                            }
                        }
                    }

                    div {
                        h4 { style: "margin: 0 0 8px 0; color: #666;", "Right" }
                        div {
                            style: "position: relative; text-align: right;",
                            hr {
                                style: "border: none; border-top: 1px solid #f0f0f0; margin: 0;"
                            }
                            span {
                                style: "position: absolute; top: -8px; right: 0; background: white; padding: 0 0 0 16px; color: #666; font-size: 14px;",
                                "Right Text"
                            }
                        }
                    }
                }
            }

            // 垂直分割线
            DemoSection {
                title: "垂直分割线".to_string(),
                description: "使用 type=\"vertical\" 设置为行内的垂直分割线。".to_string(),
                div {
                    style: "display: flex; align-items: center;",

                    span { "Text" }
                    span {
                        style: "margin: 0 16px; border-left: 1px solid #f0f0f0; height: 16px;"
                    }
                    a {
                        href: "#",
                        style: "color: #1890ff; text-decoration: none;",
                        "Link"
                    }
                    span {
                        style: "margin: 0 16px; border-left: 1px solid #f0f0f0; height: 16px;"
                    }
                    a {
                        href: "#",
                        style: "color: #1890ff; text-decoration: none;",
                        "Link"
                    }
                }
            }

            // 标题样式
            DemoSection {
                title: "标题样式".to_string(),
                description: "修改文字样式。".to_string(),
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    div {
                        style: "position: relative; text-align: center;",
                        hr {
                            style: "border: none; border-top: 1px solid #f0f0f0; margin: 0;"
                        }
                        span {
                            style: "position: absolute; top: -12px; left: 50%; transform: translateX(-50%); background: white; padding: 0 16px; color: #262626; font-size: 16px; font-weight: 500;",
                            "Title"
                        }
                    }

                    div {
                        style: "position: relative; text-align: center;",
                        hr {
                            style: "border: none; border-top: 1px solid #f0f0f0; margin: 0;"
                        }
                        span {
                            style: "position: absolute; top: -8px; left: 50%; transform: translateX(-50%); background: white; padding: 0 16px; color: #1890ff; font-size: 14px;",
                            "Link Text"
                        }
                    }
                }
            }

            // 虚线
            DemoSection {
                title: "虚线".to_string(),
                description: "使用 dashed 的样式。".to_string(),
                div {
                    hr {
                        style: "border: none; border-top: 1px dashed #f0f0f0; margin: 16px 0;"
                    }
                    div {
                        style: "position: relative; margin: 16px 0; text-align: center;",
                        hr {
                            style: "border: none; border-top: 1px dashed #f0f0f0; margin: 0;"
                        }
                        span {
                            style: "position: absolute; top: -8px; left: 50%; transform: translateX(-50%); background: white; padding: 0 16px; color: #666; font-size: 14px;",
                            "Text"
                        }
                    }
                }
            }

            // 分割文字使用正文样式
            DemoSection {
                title: "分割文字使用正文样式".to_string(),
                description: "使用 plain 可以设置为更轻量的分割文字样式。".to_string(),
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    div {
                        h4 { style: "margin: 0 0 8px 0; color: #666;", "Normal" }
                        div {
                            style: "position: relative; text-align: center;",
                            hr {
                                style: "border: none; border-top: 1px solid #f0f0f0; margin: 0;"
                            }
                            span {
                                style: "position: absolute; top: -8px; left: 50%; transform: translateX(-50%); background: white; padding: 0 16px; color: #666; font-size: 14px; font-weight: 500;",
                                "Text"
                            }
                        }
                    }

                    div {
                        h4 { style: "margin: 0 0 8px 0; color: #666;", "Plain" }
                        div {
                            style: "position: relative; text-align: center;",
                            hr {
                                style: "border: none; border-top: 1px solid #f0f0f0; margin: 0;"
                            }
                            span {
                                style: "position: absolute; top: -8px; left: 50%; transform: translateX(-50%); background: white; padding: 0 16px; color: #666; font-size: 14px; font-weight: normal;",
                                "Text"
                            }
                        }
                    }
                }
            }

            // 自定义样式
            DemoSection {
                title: "自定义样式".to_string(),
                description: "自定义分割线的样式。".to_string(),
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    // 粗线
                    hr {
                        style: "border: none; border-top: 3px solid #1890ff; margin: 16px 0;"
                    }

                    // 渐变线
                    hr {
                        style: "border: none; height: 2px; background: linear-gradient(to right, #1890ff, #52c41a); margin: 16px 0;"
                    }

                    // 点状线
                    hr {
                        style: "border: none; border-top: 2px dotted #ff4d4f; margin: 16px 0;"
                    }
                }
            }

            // API 文档
            ApiDocumentation {
                component_name: "Divider".to_string(),
                props: vec![
                    PropDoc {
                        name: "children".to_string(),
                        prop_type: "Element".to_string(),
                        default: "-".to_string(),
                        description: "嵌套的标题".to_string(),
                    },
                    PropDoc {
                        name: "class_name".to_string(),
                        prop_type: "String".to_string(),
                        default: "".to_string(),
                        description: "分割线样式类".to_string(),
                    },
                    PropDoc {
                        name: "dashed".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "是否虚线".to_string(),
                    },
                    PropDoc {
                        name: "orientation".to_string(),
                        prop_type: "String".to_string(),
                        default: "center".to_string(),
                        description: "分割线标题的位置：left | right | center".to_string(),
                    },
                    PropDoc {
                        name: "orientation_margin".to_string(),
                        prop_type: "String | Number".to_string(),
                        default: "".to_string(),
                        description: "标题和最近 left/right 边框之间的距离".to_string(),
                    },
                    PropDoc {
                        name: "plain".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "文字是否显示为普通正文样式".to_string(),
                    },
                    PropDoc {
                        name: "style".to_string(),
                        prop_type: "String".to_string(),
                        default: "".to_string(),
                        description: "分割线样式对象".to_string(),
                    },
                    PropDoc {
                        name: "div_type".to_string(),
                        prop_type: "String".to_string(),
                        default: "horizontal".to_string(),
                        description: "水平还是垂直类型：horizontal | vertical".to_string(),
                    },
                ]
            }
        }
    }
}

//!
//! 展示 Card 组件的各种用法和样式

use crate::common::*;
use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

/// Card 组件演示
#[component]
pub fn CardDemo() -> Element {
    rsx! {
        div {
            class: "component-showcase",

            h1 {
                style: "margin: 0 0 16px 0; color: #262626; font-size: 28px;",
                "Card 卡片"
            }

            p {
                style: "margin: 0 0 24px 0; color: #666; font-size: 14px;",
                "通用卡片容器。"
            }

            // 基础用法
            DemoSection {
                title: "基础用法",
                description: "包含标题、内容、操作区域。",

                div {
                    style: "display: flex; gap: 16px; flex-wrap: wrap;",

                    Card {
                        title: "Card title",
                        style: "width: 300px;",
                        p { "Card content" }
                        p { "Card content" }
                        p { "Card content" }
                    }
                }
            }

            // 无边框
            DemoSection {
                title: "无边框",
                description: "在灰色背景上使用无边框的卡片。",

                div {
                    style: "background: #ececec; padding: 30px;",

                    Card {
                        title: "Card title",
                        bordered: false,
                        style: "width: 300px;",
                        p { "Card content" }
                        p { "Card content" }
                        p { "Card content" }
                    }
                }
            }

            // 简洁卡片
            DemoSection {
                title: "简洁卡片",
                description: "只有内容区域的简洁卡片。",

                div {
                    style: "display: flex; gap: 16px; flex-wrap: wrap;",

                    Card {
                        style: "width: 300px;",
                        p { "Card content" }
                        p { "Card content" }
                        p { "Card content" }
                    }
                }
            }

            // 更灵活的内容展示
            DemoSection {
                title: "更灵活的内容展示",
                description: "可以调整默认边距，设定宽度。",

                div {
                    style: "display: flex; gap: 16px; flex-wrap: wrap;",

                    Card {
                        hoverable: true,
                        style: "width: 240px;",
                        // cover: rsx! {
                        //     img {
                        //         alt: "example",
                        //         src: "https://gw.alipayobjects.com/zos/rmsportal/JiqGstEfoWAOHiTxclqi.png",
                        //         style: "width: 100%; height: 200px; object-fit: cover;"
                        //     }
                        // },
                        div {
                            style: "padding: 16px;",
                            h3 { style: "margin: 0 0 8px 0;", "Card title" }
                            p { style: "margin: 0; color: #666;", "This is the description" }
                        }
                    }
                }
            }

            // 预加载的卡片
            DemoSection {
                title: "预加载的卡片",
                description: "数据读入前会有文本块样式。",

                div {
                    style: "display: flex; gap: 16px; flex-wrap: wrap;",

                    Card {
                        style: "width: 300px; margin-top: 16px;",
                        loading: true,
                        div {
                            style: "padding: 16px;",
                            "Card content"
                        }
                    }

                    Card {
                        style: "width: 300px; margin-top: 16px;",
                        title: "Card title",
                        loading: true,
                        div {
                            style: "padding: 16px;",
                            "Card content"
                        }
                    }
                }
            }

            // API 文档
            ApiDocumentation {
                component_name: "Card",
                props: vec![
                    PropDoc {
                        name: "title".to_string(),
                        prop_type: "String".to_string(),
                        default: "-".to_string(),
                        description: "卡片标题".to_string(),
                    },
                    PropDoc {
                        name: "bordered".to_string(),
                        prop_type: "bool".to_string(),
                        default: "true".to_string(),
                        description: "是否有边框".to_string(),
                    },
                    PropDoc {
                        name: "hoverable".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "鼠标移过时可浮起".to_string(),
                    },
                    PropDoc {
                        name: "loading".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "当卡片内容还在加载中时，可以用 loading 展示一个占位".to_string(),
                    },
                    PropDoc {
                        name: "size".to_string(),
                        prop_type: "CardSize".to_string(),
                        default: "CardSize::Default".to_string(),
                        description: "card 的尺寸".to_string(),
                    },
                ]
            }
        }
    }
}

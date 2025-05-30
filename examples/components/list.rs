#![allow(non_snake_case)]
//!
//! 展示 List 组件的各种用法和样式

use crate::common::*;
use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

/// List 组件演示
#[component]
pub fn ListDemo() -> Element {
    let data = vec![
        "Racing car sprays burning fuel into crowd.",
        "Japanese princess to wed commoner.",
        "Australian walks 100km after outback crash.",
        "Man charged over missing wedding girl.",
        "Los Angeles battles huge wildfires.",
    ];

    rsx! {
        div {
            class: "component-showcase",

            h1 {
                style: "margin: 0 0 16px 0; color: #262626; font-size: 28px;",
                "List 列表"
            }

            p {
                style: "margin: 0 0 24px 0; color: #666; font-size: 14px;",
                "通用列表。"
            }

            // 基础列表
            DemoSection {
                title: "基础列表",
                description: "基础列表。",

                div {
                    style: "width: 100%;",

                    List {
                        header: rsx! { div { "Header" } },
                        footer: rsx! { div { "Footer" } },
                        bordered: true,

                        for item in data.iter() {
                            ListItem {
                                "{item}"
                            }
                        }
                    }
                }
            }

            // 基础列表 - 大号
            DemoSection {
                title: "基础列表 - 大号",
                description: "大号列表。",

                div {
                    style: "width: 100%;",

                    List {
                        size: ListSize::Large,
                        header: rsx! { div { "Header" } },
                        footer: rsx! { div { "Footer" } },
                        bordered: true,

                        for item in data.iter() {
                            ListItem {
                                "{item}"
                            }
                        }
                    }
                }
            }

            // 基础列表 - 小号
            DemoSection {
                title: "基础列表 - 小号",
                description: "小号列表。",

                div {
                    style: "width: 100%;",

                    List {
                        size: ListSize::Small,
                        header: rsx! { div { "Header" } },
                        footer: rsx! { div { "Footer" } },
                        bordered: true,

                        for item in data.iter() {
                            ListItem {
                                "{item}"
                            }
                        }
                    }
                }
            }

            // 竖排列表样式
            DemoSection {
                title: "竖排列表样式",
                description: "通过设置 itemLayout 属性为 vertical 可实现竖排列表样式。",

                div {
                    style: "width: 100%;",

                    List {
                        item_layout: ListLayout::Vertical,
                        size: ListSize::Large,

                        for item in ["Ant Design Title 1", "Ant Design Title 2", "Ant Design Title 3", "Ant Design Title 4"].iter() {
                            ListItem {
                                key: "{item}",
                                actions: rsx! {
                                    div {
                                        a { "edit" }
                                        a { "more" }
                                    }
                                },
                                extra: rsx! {
                                    img {
                                        width: "272",
                                        alt: "logo",
                                        src: "https://gw.alipayobjects.com/zos/rmsportal/mqaQswcyDLcXyDKnZfES.png"
                                    }
                                },

                                ListItemMeta {
                                    avatar: rsx! {
                                        Avatar {
                                            src: "https://joeschmoe.io/api/v1/random"
                                        }
                                    },
                                    title: "href-a-wait-complete".to_string(), //rsx! { a { href: "https://ant.design", "{item}" } },
                                    description: "Ant Design, a design language for background applications, is refined by Ant UED Team."
                                }

                                "We supply a series of design principles, practical patterns and high quality design resources (Sketch and Axure), to help people create their product prototypes beautifully and efficiently."
                            }
                        }
                    }
                }
            }

            // 栅格列表
            DemoSection {
                title: "栅格列表",
                description: "可以通过设置 List 的 grid 属性来实现栅格列表，column 可设置期望显示的列数。",

                div {
                    style: "width: 100%;",

                    List {
                        style: "display: grid; grid-template-columns: repeat(4, 1fr); gap: 16px;",

                        for item in [
                            "Racing car sprays burning fuel into crowd.",
                            "Japanese princess to wed commoner.",
                            "Australian walks 100km after outback crash.",
                            "Man charged over missing wedding girl.",
                            "Los Angeles battles huge wildfires.",
                            "British woman dies in Greek island fall.",
                            "Clairvoyant claims to predict lottery.",
                            "Safety meeting ends in accident.",
                        ].iter() {
                            ListItem {
                                Card {
                                    title: "Card title",
                                    "{item}"
                                }
                            }
                        }
                    }
                }
            }

            // API 文档
            ApiDocumentation {
                component_name: "List",
                props: vec![
                    PropDoc {
                        name: "bordered".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "是否展示边框".to_string(),
                    },
                    PropDoc {
                        name: "footer".to_string(),
                        prop_type: "Element".to_string(),
                        default: "-".to_string(),
                        description: "列表底部".to_string(),
                    },
                    PropDoc {
                        name: "grid".to_string(),
                        prop_type: "Element".to_string(),
                        default: "-".to_string(),
                        description: "列表栅格配置".to_string(),
                    },
                    PropDoc {
                        name: "header".to_string(),
                        prop_type: "Element".to_string(),
                        default: "-".to_string(),
                        description: "列表头部".to_string(),
                    },
                    PropDoc {
                        name: "item_layout".to_string(),
                        prop_type: "String".to_string(),
                        default: "horizontal".to_string(),
                        description: "设置 List.Item 布局, 设置成 vertical 则竖直样式显示, 默认横排".to_string(),
                    },
                    PropDoc {
                        name: "loading".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "当卡片内容还在加载中时，可以用 loading 展示一个占位".to_string(),
                    },
                    PropDoc {
                        name: "size".to_string(),
                        prop_type: "String".to_string(),
                        default: "default".to_string(),
                        description: "list 的尺寸".to_string(),
                    },
                ]
            }
        }
    }
}

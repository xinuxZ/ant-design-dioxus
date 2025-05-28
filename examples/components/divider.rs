//! Divider 组件演示
//!
//! 展示 Divider 组件的各种用法，包括水平分割线、垂直分割线、带文字的分割线等

use crate::common::*;
use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

// 重命名 Link 组件以避免冲突
use ant_design_dioxus::prelude::Link as AntLink;

/// Divider 组件演示
#[component]
pub fn DividerDemo() -> Element {
    rsx! {
        div {
            class: "component-showcase",

            h1 {
                style: "margin: 0 0 16px 0; color: #262626; font-size: 28px;",
                "Divider 分割线"
            }

            p {
                style: "margin: 0 0 24px 0; color: #666; font-size: 14px;",
                "区隔内容的分割线。"
            }

            // 基础用法
            DemoSection {
                title: "基础用法",
                description: "默认为水平分割线，可在中间加入文字。",

                div {
                    p {
                        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed nonne merninisti licere mihi ista probare, quae sunt a te dicta? Refert tamen, quo modo."
                    }
                    Divider {}
                    p {
                        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed nonne merninisti licere mihi ista probare, quae sunt a te dicta? Refert tamen, quo modo."
                    }
                    Divider {
                        "With Text"
                    }
                    p {
                        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed nonne merninisti licere mihi ista probare, quae sunt a te dicta? Refert tamen, quo modo."
                    }
                }
            }

            // 带文字的分割线
            DemoSection {
                title: "带文字的分割线",
                description: "分割线中带有文字，可以用 orientation 指定文字位置。",

                div {
                    p {
                        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed nonne merninisti licere mihi ista probare, quae sunt a te dicta? Refert tamen, quo modo."
                    }
                    Divider {
                        orientation: DividerOrientation::Left,
                        "Left Text"
                    }
                    p {
                        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed nonne merninisti licere mihi ista probare, quae sunt a te dicta? Refert tamen, quo modo."
                    }
                    Divider {
                        orientation: DividerOrientation::Right,
                        "Right Text"
                    }
                    p {
                        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed nonne merninisti licere mihi ista probare, quae sunt a te dicta? Refert tamen, quo modo."
                    }
                }
            }

            // 垂直分割线
            DemoSection {
                title: "垂直分割线",
                description: "使用 type=\"vertical\" 设置为行内的垂直分割线。",

                div {
                    style: "display: flex; align-items: center; gap: 0;",
                    "Text"
                    Divider { r#type: DividerType::Vertical }
                    AntLink { href: "#", "Link" }
                    Divider { r#type: DividerType::Vertical }
                    AntLink { href: "#", "Link" }
                }
            }

            // 分割线样式
            DemoSection {
                title: "分割线样式",
                description: "使用 dashed 可以设置为虚线。",

                div {
                    p {
                        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed nonne merninisti licere mihi ista probare, quae sunt a te dicta? Refert tamen, quo modo."
                    }
                    Divider {
                        dashed: true,
                    }
                    p {
                        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed nonne merninisti licere mihi ista probare, quae sunt a te dicta? Refert tamen, quo modo."
                    }
                    Divider {
                        dashed: true,
                        "Dashed Text"
                    }
                    p {
                        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed nonne merninisti licere mihi ista probare, quae sunt a te dicta? Refert tamen, quo modo."
                    }
                }
            }

            // 分割线颜色
            DemoSection {
                title: "分割线颜色",
                description: "可以自定义分割线的颜色。",

                div {
                    p {
                        "Lorem ipsum dolor sit amet, consectetur adipiscing elit."
                    }
                    Divider {
                        style: "border-color: #ff4d4f;",
                        "Red Divider"
                    }
                    p {
                        "Lorem ipsum dolor sit amet, consectetur adipiscing elit."
                    }
                    Divider {
                        style: "border-color: #52c41a;",
                        "Green Divider"
                    }
                    p {
                        "Lorem ipsum dolor sit amet, consectetur adipiscing elit."
                    }
                }
            }
        }
    }
}

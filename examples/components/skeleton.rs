#![allow(non_snake_case)]
//!
//! 展示 Skeleton 组件的各种用法和样式

use crate::common::*;
use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

/// Skeleton 组件演示
#[component]
pub fn SkeletonDemo() -> Element {
    let mut loading = use_signal(|| false);
    let mut active = use_signal(|| false);

    rsx! {
        div {
            class: "component-showcase",

            h1 {
                style: "margin: 0 0 16px 0; color: #262626; font-size: 28px;",
                "Skeleton 骨架屏"
            }

            p {
                style: "margin: 0 0 24px 0; color: #666; font-size: 14px;",
                "在需要等待加载内容的位置提供一个占位图形组合。"
            }

            // 基础用法
            DemoSection {
                title: "基础用法",
                description: "最简单的占位效果。",

                div {
                    style: "padding: 24px; background: #fafafa; border-radius: 6px;",

                    Skeleton {}
                }
            }

            // 复杂的组合
            DemoSection {
                title: "复杂的组合",
                description: "更复杂的组合。",

                div {
                    style: "padding: 24px; background: #fafafa; border-radius: 6px;",

                    Skeleton {
                        avatar: true,
                        paragraph: SkeletonParagraph {
                            rows: 4,
                        }
                    }
                }
            }

            // 动画效果
            DemoSection {
                title: "动画效果",
                description: "显示动画效果。",

                div {
                    style: "display: flex; gap: 16px; align-items: center; margin-bottom: 16px;",
                    Switch {
                        checked: active(),
                        on_change: move |checked| {
                            active.set(checked);
                        }
                    }
                    span { "Active" }
                }

                div {
                    style: "padding: 24px; background: #fafafa; border-radius: 6px;",

                    Skeleton {
                        active: active(),
                        avatar: true,
                        paragraph: SkeletonParagraph {
                            rows: 4,
                        }
                    }
                }
            }

            // 包含子组件
            DemoSection {
                title: "包含子组件",
                description: "加载占位图包含子组件。",

                div {
                    style: "display: flex; gap: 16px; align-items: center; margin-bottom: 16px;",
                    Switch {
                        checked: loading(),
                        on_change: move |checked| {
                            loading.set(checked);
                        }
                    }
                    span { "Loading" }
                }

                div {
                    style: "padding: 24px; background: #fafafa; border-radius: 6px;",

                    Skeleton {
                        loading: loading(),
                        avatar: true,
                        active: true,

                        div {
                            h4 {
                                style: "margin: 0 0 16px 0;",
                                "Ant Design, a design language"
                            }
                            p {
                                style: "margin: 0;",
                                "We supply a series of design principles, practical patterns and high quality design resources (Sketch and Axure), to help people create their product prototypes beautifully and efficiently."
                            }
                        }
                    }
                }
            }

            // 列表
            DemoSection {
                title: "列表",
                description: "在列表组件中使用加载占位符。",

                div {
                    style: "padding: 24px; background: #fafafa; border-radius: 6px;",

                    List {
                        item_layout: ListLayout::Horizontal,

                        for i in 0..3 {
                            ListItem {
                                Skeleton {
                                    avatar: true,
                                    active: true,
                                    title: false,
                                    loading: true
                                }
                            }
                        }
                    }
                }
            }

            // 骨架按钮/头像/输入框
            DemoSection {
                title: "骨架按钮/头像/输入框",
                description: "骨架按钮、头像、输入框。",

                div {
                    style: "padding: 24px; background: #fafafa; border-radius: 6px;",

                    div {
                        style: "display: flex; gap: 16px; align-items: center; margin-bottom: 16px;",
                        Skeleton::Button {
                            active: true
                        }
                        Skeleton::Avatar {
                            active: true
                        }
                        Skeleton::Input {
                            active: true
                        }
                    }

                    div {
                        style: "display: flex; gap: 16px; align-items: center; margin-bottom: 16px;",
                        Skeleton::Button {
                            active: true,
                            size: "large"
                        }
                        Skeleton::Avatar {
                            active: true,
                            size: "large"
                        }
                        Skeleton::Input {
                            active: true,
                            size: "large"
                        }
                    }

                    div {
                        style: "display: flex; gap: 16px; align-items: center;",
                        Skeleton::Button {
                            active: true,
                            size: "small"
                        }
                        Skeleton::Avatar {
                            active: true,
                            size: "small"
                        }
                        Skeleton::Input {
                            active: true,
                            size: "small"
                        }
                    }
                }
            }

            // 骨架图片
            DemoSection {
                title: "骨架图片",
                description: "骨架图片。",

                div {
                    style: "padding: 24px; background: #fafafa; border-radius: 6px;",

                    Skeleton::Image {
                        active: true
                    }
                }
            }

            // API 文档
            ApiDocumentation {
                component_name: "Skeleton",
                props: vec![
                    PropDoc {
                        name: "active".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "是否展示动画效果".to_string(),
                    },
                    PropDoc {
                        name: "avatar".to_string(),
                        prop_type: "bool | SkeletonAvatar".to_string(),
                        default: "false".to_string(),
                        description: "是否显示头像占位图".to_string(),
                    },
                    PropDoc {
                        name: "loading".to_string(),
                        prop_type: "bool".to_string(),
                        default: "true".to_string(),
                        description: "为 true 时，显示占位图。反之则直接展示子组件".to_string(),
                    },
                    PropDoc {
                        name: "paragraph".to_string(),
                        prop_type: "bool | SkeletonParagraph".to_string(),
                        default: "true".to_string(),
                        description: "是否显示段落占位图".to_string(),
                    },
                    PropDoc {
                        name: "round".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "为 true 时，段落和标题显示圆角".to_string(),
                    },
                    PropDoc {
                        name: "title".to_string(),
                        prop_type: "bool | SkeletonTitle".to_string(),
                        default: "true".to_string(),
                        description: "是否显示标题占位图".to_string(),
                    },
                ]
            }

            // SkeletonAvatar API
            ApiDocumentation {
                component_name: "SkeletonAvatar",
                props: vec![
                    PropDoc {
                        name: "active".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "是否展示动画效果".to_string(),
                    },
                    PropDoc {
                        name: "shape".to_string(),
                        prop_type: "String".to_string(),
                        default: "circle".to_string(),
                        description: "指定头像的形状，可选：circle | square".to_string(),
                    },
                    PropDoc {
                        name: "size".to_string(),
                        prop_type: "String | i32".to_string(),
                        default: "default".to_string(),
                        description: "设置头像占位图的大小，可选：large | small | default 或数字".to_string(),
                    },
                ]
            }

            // SkeletonTitle API
            ApiDocumentation {
                component_name: "SkeletonTitle",
                props: vec![
                    PropDoc {
                        name: "width".to_string(),
                        prop_type: "String | i32".to_string(),
                        default: "-".to_string(),
                        description: "设置标题占位图的宽度".to_string(),
                    },
                ]
            }

            // SkeletonParagraph API
            ApiDocumentation {
                component_name: "SkeletonParagraph",
                props: vec![
                    PropDoc {
                        name: "rows".to_string(),
                        prop_type: "i32".to_string(),
                        default: "3".to_string(),
                        description: "设置段落占位图的行数".to_string(),
                    },
                    PropDoc {
                        name: "width".to_string(),
                        prop_type: "String | i32 | Vec<String | i32>".to_string(),
                        default: "-".to_string(),
                        description: "设置段落占位图的宽度，若为数组时则为对应的每行宽度，反之则是最后一行的宽度".to_string(),
                    },
                ]
            }

            // SkeletonButton API
            ApiDocumentation {
                component_name: "SkeletonButton",
                props: vec![
                    PropDoc {
                        name: "active".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "是否展示动画效果".to_string(),
                    },
                    PropDoc {
                        name: "block".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "将按钮宽度调整为其父宽度的选项".to_string(),
                    },
                    PropDoc {
                        name: "shape".to_string(),
                        prop_type: "String".to_string(),
                        default: "default".to_string(),
                        description: "指定按钮的形状，可选：circle | round | default".to_string(),
                    },
                    PropDoc {
                        name: "size".to_string(),
                        prop_type: "String".to_string(),
                        default: "default".to_string(),
                        description: "设置按钮的大小，可选：large | small | default".to_string(),
                    },
                ]
            }

            // SkeletonInput API
            ApiDocumentation {
                component_name: "SkeletonInput",
                props: vec![
                    PropDoc {
                        name: "active".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "是否展示动画效果".to_string(),
                    },
                    PropDoc {
                        name: "size".to_string(),
                        prop_type: "String".to_string(),
                        default: "default".to_string(),
                        description: "设置输入框的大小，可选：large | small | default".to_string(),
                    },
                ]
            }
        }
    }
}

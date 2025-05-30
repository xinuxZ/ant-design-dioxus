//! Grid 组件演示
//!
//! 展示 24 栅格系统的基本用法、响应式布局、对齐方式等功能

use crate::common::*;
use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

/// Grid 组件演示
#[component]
pub fn GridDemo() -> Element {
    rsx! {
        div {
            class: "component-showcase",

            h1 {
                style: "margin: 0 0 16px 0; color: #262626; font-size: 28px;",
                "Grid 栅格"
            }

            p {
                style: "margin: 0 0 24px 0; color: #666; font-size: 14px;",
                "24 栅格系统。"
            }

            // 基础栅格
            DemoSection {
                title: "基础栅格",
                description: "使用 Row 和 Col 组件，通过 span 属性设置列的宽度。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    Row {
                        gutter: 16,
                        Col {
                            span: 24,
                            div {
                                style: "background: #0092ff; color: white; text-align: center; padding: 16px 0; border-radius: 4px;",
                                "col-24"
                            }
                        }
                    }

                    Row {
                        gutter: 16,
                        Col {
                            span: 12,
                            div {
                                style: "background: #0092ff; color: white; text-align: center; padding: 16px 0; border-radius: 4px;",
                                "col-12"
                            }
                        }
                        Col {
                            span: 12,
                            div {
                                style: "background: #096dd9; color: white; text-align: center; padding: 16px 0; border-radius: 4px;",
                                "col-12"
                            }
                        }
                    }

                    Row {
                        gutter: 16,
                        Col {
                            span: 8,
                            div {
                                style: "background: #0092ff; color: white; text-align: center; padding: 16px 0; border-radius: 4px;",
                                "col-8"
                            }
                        }
                        Col {
                            span: 8,
                            div {
                                style: "background: #096dd9; color: white; text-align: center; padding: 16px 0; border-radius: 4px;",
                                "col-8"
                            }
                        }
                        Col {
                            span: 8,
                            div {
                                style: "background: #0092ff; color: white; text-align: center; padding: 16px 0; border-radius: 4px;",
                                "col-8"
                            }
                        }
                    }

                    Row {
                        gutter: 16,
                        Col {
                            span: 6,
                            div {
                                style: "background: #0092ff; color: white; text-align: center; padding: 16px 0; border-radius: 4px;",
                                "col-6"
                            }
                        }
                        Col {
                            span: 6,
                            div {
                                style: "background: #096dd9; color: white; text-align: center; padding: 16px 0; border-radius: 4px;",
                                "col-6"
                            }
                        }
                        Col {
                            span: 6,
                            div {
                                style: "background: #0092ff; color: white; text-align: center; padding: 16px 0; border-radius: 4px;",
                                "col-6"
                            }
                        }
                        Col {
                            span: 6,
                            div {
                                style: "background: #096dd9; color: white; text-align: center; padding: 16px 0; border-radius: 4px;",
                                "col-6"
                            }
                        }
                    }
                }
            }

            // 区间间隔
            DemoSection {
                title: "区间间隔",
                description: "栅格常常需要和间隔进行配合，你可以使用 Row 的 gutter 属性。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    Row {
                        gutter: 16,
                        Col {
                            span: 6,
                            div {
                                style: "background: #0092ff; color: white; text-align: center; padding: 16px 0; border-radius: 4px;",
                                "col-6"
                            }
                        }
                        Col {
                            span: 6,
                            div {
                                style: "background: #096dd9; color: white; text-align: center; padding: 16px 0; border-radius: 4px;",
                                "col-6"
                            }
                        }
                        Col {
                            span: 6,
                            div {
                                style: "background: #0092ff; color: white; text-align: center; padding: 16px 0; border-radius: 4px;",
                                "col-6"
                            }
                        }
                        Col {
                            span: 6,
                            div {
                                style: "background: #096dd9; color: white; text-align: center; padding: 16px 0; border-radius: 4px;",
                                "col-6"
                            }
                        }
                    }

                    Row {
                        gutter: 32,
                        Col {
                            span: 6,
                            div {
                                style: "background: #0092ff; color: white; text-align: center; padding: 16px 0; border-radius: 4px;",
                                "col-6"
                            }
                        }
                        Col {
                            span: 6,
                            div {
                                style: "background: #096dd9; color: white; text-align: center; padding: 16px 0; border-radius: 4px;",
                                "col-6"
                            }
                        }
                        Col {
                            span: 6,
                            div {
                                style: "background: #0092ff; color: white; text-align: center; padding: 16px 0; border-radius: 4px;",
                                "col-6"
                            }
                        }
                        Col {
                            span: 6,
                            div {
                                style: "background: #096dd9; color: white; text-align: center; padding: 16px 0; border-radius: 4px;",
                                "col-6"
                            }
                        }
                    }
                }
            }

            // 左右偏移
            DemoSection {
                title: "左右偏移",
                description: "列偏移。使用 offset 可以将列向右侧偏。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    Row {
                        Col {
                            span: 8,
                            div {
                                style: "background: #0092ff; color: white; text-align: center; padding: 16px 0; border-radius: 4px;",
                                "col-8"
                            }
                        }
                        Col {
                            span: 8,
                            offset: 8,
                            div {
                                style: "background: #096dd9; color: white; text-align: center; padding: 16px 0; border-radius: 4px;",
                                "col-8 col-offset-8"
                            }
                        }
                    }

                    Row {
                        Col {
                            span: 6,
                            offset: 6,
                            div {
                                style: "background: #0092ff; color: white; text-align: center; padding: 16px 0; border-radius: 4px;",
                                "col-6 col-offset-6"
                            }
                        }
                        Col {
                            span: 6,
                            offset: 6,
                            div {
                                style: "background: #096dd9; color: white; text-align: center; padding: 16px 0; border-radius: 4px;",
                                "col-6 col-offset-6"
                            }
                        }
                    }
                }
            }

            // Flex 布局
            DemoSection {
                title: "Flex 布局",
                description: "使用 Row 的 justify 和 align 属性来定义其子元素的排版方式。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    div {
                        p { "子元素向左排列" }
                        Row {
                            justify: GridJustify::Start,
                            style: "background: #f5f5f5; min-height: 60px;",
                            Col {
                                span: 4,
                                div {
                                    style: "background: #0092ff; color: white; text-align: center; padding: 16px 0; border-radius: 4px;",
                                    "col-4"
                                }
                            }
                            Col {
                                span: 4,
                                div {
                                    style: "background: #096dd9; color: white; text-align: center; padding: 16px 0; border-radius: 4px;",
                                    "col-4"
                                }
                            }
                            Col {
                                span: 4,
                                div {
                                    style: "background: #0092ff; color: white; text-align: center; padding: 16px 0; border-radius: 4px;",
                                    "col-4"
                                }
                            }
                        }
                    }

                    div {
                        p { "子元素向中间排列" }
                        Row {
                            justify: GridJustify::Center,
                            style: "background: #f5f5f5; min-height: 60px;",
                            Col {
                                span: 4,
                                div {
                                    style: "background: #0092ff; color: white; text-align: center; padding: 16px 0; border-radius: 4px;",
                                    "col-4"
                                }
                            }
                            Col {
                                span: 4,
                                div {
                                    style: "background: #096dd9; color: white; text-align: center; padding: 16px 0; border-radius: 4px;",
                                    "col-4"
                                }
                            }
                            Col {
                                span: 4,
                                div {
                                    style: "background: #0092ff; color: white; text-align: center; padding: 16px 0; border-radius: 4px;",
                                    "col-4"
                                }
                            }
                        }
                    }

                    div {
                        p { "子元素向右排列" }
                        Row {
                            justify: GridJustify::End,
                            style: "background: #f5f5f5; min-height: 60px;",
                            Col {
                                span: 4,
                                div {
                                    style: "background: #0092ff; color: white; text-align: center; padding: 16px 0; border-radius: 4px;",
                                    "col-4"
                                }
                            }
                            Col {
                                span: 4,
                                div {
                                    style: "background: #096dd9; color: white; text-align: center; padding: 16px 0; border-radius: 4px;",
                                    "col-4"
                                }
                            }
                            Col {
                                span: 4,
                                div {
                                    style: "background: #0092ff; color: white; text-align: center; padding: 16px 0; border-radius: 4px;",
                                    "col-4"
                                }
                            }
                        }
                    }
                }
            }

            // API 文档
            ApiDocumentation {
                component_name: "Grid",
                props: vec![
                    PropDoc {
                        name: "span".to_string(),
                        prop_type: "u8".to_string(),
                        default: "24".to_string(),
                        description: "栅格占位格数，为 0 时相当于 display: none".to_string(),
                    },
                    PropDoc {
                        name: "offset".to_string(),
                        prop_type: "u8".to_string(),
                        default: "0".to_string(),
                        description: "栅格左侧的间隔格数，间隔内不可以有栅格".to_string(),
                    },
                    PropDoc {
                        name: "order".to_string(),
                        prop_type: "i32".to_string(),
                        default: "0".to_string(),
                        description: "栅格顺序".to_string(),
                    },
                    PropDoc {
                        name: "gutter".to_string(),
                        prop_type: "u32".to_string(),
                        default: "0".to_string(),
                        description: "栅格间隔，可以写成像素值".to_string(),
                    },
                    PropDoc {
                        name: "justify".to_string(),
                        prop_type: "Justify".to_string(),
                        default: "Justify::Start".to_string(),
                        description: "flex 布局下的水平排列方式".to_string(),
                    },
                    PropDoc {
                        name: "align".to_string(),
                        prop_type: "Align".to_string(),
                        default: "Align::Top".to_string(),
                        description: "flex 布局下的垂直对齐方式".to_string(),
                    },
                ]
            }
        }
    }
}

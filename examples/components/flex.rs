//! Flex 组件演示
//!
//! 展示 Flex 组件的各种用法，包括弹性布局、对齐方式、间距设置等

use crate::common::*;
use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

/// Flex 组件演示
#[component]
pub fn FlexDemo() -> Element {
    rsx! {
        div {
            class: "component-showcase",

            h1 {
                style: "margin: 0 0 16px 0; color: #262626; font-size: 28px;",
                "Flex 弹性布局"
            }

            p {
                style: "margin: 0 0 24px 0; color: #666; font-size: 14px;",
                "弹性布局组件，基于 CSS Flexbox 实现。"
            }

            // 基础用法
            DemoSection {
                title: "基础用法",
                description: "最简单的用法。",

                div {
                    style: "border: 1px solid #d9d9d9; padding: 16px; border-radius: 6px;",
                    Flex {
                        gap: FlexGap::Middle,
                        div {
                            style: "padding: 16px; background: #1677ff; color: white; border-radius: 6px;",
                            "Item 1"
                        }
                        div {
                            style: "padding: 16px; background: #1677ff; color: white; border-radius: 6px;",
                            "Item 2"
                        }
                        div {
                            style: "padding: 16px; background: #1677ff; color: white; border-radius: 6px;",
                            "Item 3"
                        }
                    }
                }
            }

            // 水平对齐
            DemoSection {
                title: "水平对齐",
                description: "通过 justify 来设置主轴上的对齐方式。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    div {
                        h4 { "justify: flex-start (默认)" }
                        div {
                            style: "border: 1px solid #d9d9d9; padding: 16px; border-radius: 6px;",
                            Flex {
                                justify: FlexJustify::FlexStart,
                                gap: FlexGap::Small,
                                div {
                                    style: "padding: 16px; background: #1677ff; color: white; border-radius: 6px;",
                                    "Item 1"
                                }
                                div {
                                    style: "padding: 16px; background: #1677ff; color: white; border-radius: 6px;",
                                    "Item 2"
                                }
                                div {
                                    style: "padding: 16px; background: #1677ff; color: white; border-radius: 6px;",
                                    "Item 3"
                                }
                            }
                        }
                    }

                    div {
                        h4 { "justify: center" }
                        div {
                            style: "border: 1px solid #d9d9d9; padding: 16px; border-radius: 6px;",
                            Flex {
                                justify: FlexJustify::Center,
                                gap: FlexGap::Small,
                                div {
                                    style: "padding: 16px; background: #1677ff; color: white; border-radius: 6px;",
                                    "Item 1"
                                }
                                div {
                                    style: "padding: 16px; background: #1677ff; color: white; border-radius: 6px;",
                                    "Item 2"
                                }
                                div {
                                    style: "padding: 16px; background: #1677ff; color: white; border-radius: 6px;",
                                    "Item 3"
                                }
                            }
                        }
                    }

                    div {
                        h4 { "justify: space-between" }
                        div {
                            style: "border: 1px solid #d9d9d9; padding: 16px; border-radius: 6px;",
                            Flex {
                                justify: FlexJustify::SpaceBetween,
                                div {
                                    style: "padding: 16px; background: #1677ff; color: white; border-radius: 6px;",
                                    "Item 1"
                                }
                                div {
                                    style: "padding: 16px; background: #1677ff; color: white; border-radius: 6px;",
                                    "Item 2"
                                }
                                div {
                                    style: "padding: 16px; background: #1677ff; color: white; border-radius: 6px;",
                                    "Item 3"
                                }
                            }
                        }
                    }
                }
            }

            // 垂直对齐
            DemoSection {
                title: "垂直对齐",
                description: "通过 align 来设置交叉轴上的对齐方式。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    div {
                        h4 { "align: flex-start" }
                        div {
                            style: "border: 1px solid #d9d9d9; padding: 16px; border-radius: 6px; height: 120px;",
                            Flex {
                                align: FlexAlign::FlexStart,
                                gap: FlexGap::Small,
                                div {
                                    style: "padding: 16px; background: #1677ff; color: white; border-radius: 6px;",
                                    "Item 1"
                                }
                                div {
                                    style: "padding: 16px; background: #1677ff; color: white; border-radius: 6px; height: 60px;",
                                    "Item 2"
                                }
                                div {
                                    style: "padding: 16px; background: #1677ff; color: white; border-radius: 6px;",
                                    "Item 3"
                                }
                            }
                        }
                    }

                    div {
                        h4 { "align: center" }
                        div {
                            style: "border: 1px solid #d9d9d9; padding: 16px; border-radius: 6px; height: 120px;",
                            Flex {
                                align: FlexAlign::Center,
                                gap: FlexGap::Small,
                                div {
                                    style: "padding: 16px; background: #1677ff; color: white; border-radius: 6px;",
                                    "Item 1"
                                }
                                div {
                                    style: "padding: 16px; background: #1677ff; color: white; border-radius: 6px; height: 60px;",
                                    "Item 2"
                                }
                                div {
                                    style: "padding: 16px; background: #1677ff; color: white; border-radius: 6px;",
                                    "Item 3"
                                }
                            }
                        }
                    }
                }
            }

            // 垂直布局
            DemoSection {
                title: "垂直布局",
                description: "通过 vertical 设置垂直方向布局。",

                div {
                    style: "border: 1px solid #d9d9d9; padding: 16px; border-radius: 6px; width: 200px;",
                    Flex {
                        vertical: true,
                        gap: FlexGap::Middle,
                        div {
                            style: "padding: 16px; background: #1677ff; color: white; border-radius: 6px; text-align: center;",
                            "Item 1"
                        }
                        div {
                            style: "padding: 16px; background: #1677ff; color: white; border-radius: 6px; text-align: center;",
                            "Item 2"
                        }
                        div {
                            style: "padding: 16px; background: #1677ff; color: white; border-radius: 6px; text-align: center;",
                            "Item 3"
                        }
                    }
                }
            }

            // 间距设置
            DemoSection {
                title: "间距设置",
                description: "通过 gap 来设置元素之间的间距。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    div {
                        h4 { "gap: small" }
                        div {
                            style: "border: 1px solid #d9d9d9; padding: 16px; border-radius: 6px;",
                            Flex {
                                gap: FlexGap::Small,
                                div {
                                    style: "padding: 16px; background: #1677ff; color: white; border-radius: 6px;",
                                    "Item"
                                }
                                div {
                                    style: "padding: 16px; background: #1677ff; color: white; border-radius: 6px;",
                                    "Item"
                                }
                                div {
                                    style: "padding: 16px; background: #1677ff; color: white; border-radius: 6px;",
                                    "Item"
                                }
                            }
                        }
                    }

                    div {
                        h4 { "gap: middle" }
                        div {
                            style: "border: 1px solid #d9d9d9; padding: 16px; border-radius: 6px;",
                            Flex {
                                gap: FlexGap::Middle,
                                div {
                                    style: "padding: 16px; background: #1677ff; color: white; border-radius: 6px;",
                                    "Item"
                                }
                                div {
                                    style: "padding: 16px; background: #1677ff; color: white; border-radius: 6px;",
                                    "Item"
                                }
                                div {
                                    style: "padding: 16px; background: #1677ff; color: white; border-radius: 6px;",
                                    "Item"
                                }
                            }
                        }
                    }

                    div {
                        h4 { "gap: large" }
                        div {
                            style: "border: 1px solid #d9d9d9; padding: 16px; border-radius: 6px;",
                            Flex {
                                gap: FlexGap::Large,
                                div {
                                    style: "padding: 16px; background: #1677ff; color: white; border-radius: 6px;",
                                    "Item"
                                }
                                div {
                                    style: "padding: 16px; background: #1677ff; color: white; border-radius: 6px;",
                                    "Item"
                                }
                                div {
                                    style: "padding: 16px; background: #1677ff; color: white; border-radius: 6px;",
                                    "Item"
                                }
                            }
                        }
                    }
                }
            }

            // 自动换行
            DemoSection {
                title: "自动换行",
                description: "通过 wrap 设置自动换行。",

                div {
                    style: "border: 1px solid #d9d9d9; padding: 16px; border-radius: 6px;",
                    Flex {
                        wrap: FlexWrap::Wrap,
                        gap: FlexGap::Small,
                        div {
                            style: "padding: 16px; background: #1677ff; color: white; border-radius: 6px; width: 200px;",
                            "Item 1"
                        }
                        div {
                            style: "padding: 16px; background: #1677ff; color: white; border-radius: 6px; width: 200px;",
                            "Item 2"
                        }
                        div {
                            style: "padding: 16px; background: #1677ff; color: white; border-radius: 6px; width: 200px;",
                            "Item 3"
                        }
                        div {
                            style: "padding: 16px; background: #1677ff; color: white; border-radius: 6px; width: 200px;",
                            "Item 4"
                        }
                        div {
                            style: "padding: 16px; background: #1677ff; color: white; border-radius: 6px; width: 200px;",
                            "Item 5"
                        }
                    }
                }
            }
        }
    }
}

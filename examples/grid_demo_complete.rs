//! Grid 栅格系统完整演示
//!
//! 展示 Ant Design 24 栅格系统的各种功能：
//! - 基础栅格
//! - 区块间隔
//! - 左右偏移
//! - 栅格排序
//! - 排版
//! - 对齐
//! - 响应式布局

use ant_design_dioxus::components::grid::Align;
use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        div {
            style: "padding: 24px; background: #f0f2f5; min-height: 100vh;",

            h1 { "Grid 栅格系统演示" }

            // 基础栅格
            section {
                style: "margin-bottom: 32px;",
                h2 { "基础栅格" }
                p { "从堆叠到水平排列。" }
                p { "使用单一的一组 Row 和 Col 栅格组件，就可以创建一个基本的栅格系统，所有列（Col）必须放在 Row 内。" }

                div { style: "margin-bottom: 16px;",
                    Row {
                        Col { span: 24,
                            div {
                                style: "background: #0092ff; color: white; text-align: center; padding: 16px 0;",
                                "col-24"
                            }
                        }
                    }
                }

                div { style: "margin-bottom: 16px;",
                    Row {
                        Col { span: 12,
                            div {
                                style: "background: #0092ff; color: white; text-align: center; padding: 16px 0;",
                                "col-12"
                            }
                        }
                        Col { span: 12,
                            div {
                                style: "background: #096dd9; color: white; text-align: center; padding: 16px 0;",
                                "col-12"
                            }
                        }
                    }
                }

                div { style: "margin-bottom: 16px;",
                    Row {
                        Col { span: 8,
                            div {
                                style: "background: #0092ff; color: white; text-align: center; padding: 16px 0;",
                                "col-8"
                            }
                        }
                        Col { span: 8,
                            div {
                                style: "background: #096dd9; color: white; text-align: center; padding: 16px 0;",
                                "col-8"
                            }
                        }
                        Col { span: 8,
                            div {
                                style: "background: #0092ff; color: white; text-align: center; padding: 16px 0;",
                                "col-8"
                            }
                        }
                    }
                }

                div { style: "margin-bottom: 16px;",
                    Row {
                        Col { span: 6,
                            div {
                                style: "background: #0092ff; color: white; text-align: center; padding: 16px 0;",
                                "col-6"
                            }
                        }
                        Col { span: 6,
                            div {
                                style: "background: #096dd9; color: white; text-align: center; padding: 16px 0;",
                                "col-6"
                            }
                        }
                        Col { span: 6,
                            div {
                                style: "background: #0092ff; color: white; text-align: center; padding: 16px 0;",
                                "col-6"
                            }
                        }
                        Col { span: 6,
                            div {
                                style: "background: #096dd9; color: white; text-align: center; padding: 16px 0;",
                                "col-6"
                            }
                        }
                    }
                }
            }

            // 区块间隔
            section {
                style: "margin-bottom: 32px;",
                h2 { "区块间隔" }
                p { "栅格常常需要和间隔进行配合，你可以使用 Row 的 gutter 属性，我们推荐使用 (16+8n)px 作为栅格间隔(n 是自然数)。" }

                div { style: "margin-bottom: 16px;",
                    Row { gutter: 16,
                        Col { span: 6,
                            div {
                                style: "background: #0092ff; color: white; text-align: center; padding: 16px 0;",
                                "col-6"
                            }
                        }
                        Col { span: 6,
                            div {
                                style: "background: #0092ff; color: white; text-align: center; padding: 16px 0;",
                                "col-6"
                            }
                        }
                        Col { span: 6,
                            div {
                                style: "background: #0092ff; color: white; text-align: center; padding: 16px 0;",
                                "col-6"
                            }
                        }
                        Col { span: 6,
                            div {
                                style: "background: #0092ff; color: white; text-align: center; padding: 16px 0;",
                                "col-6"
                            }
                        }
                    }
                }
            }

            // 左右偏移
            section {
                style: "margin-bottom: 32px;",
                h2 { "左右偏移" }
                p { "列偏移。" }
                p { "使用 offset 可以将列向右侧偏。例如，offset={4} 将元素向右侧偏移了 4 个列（column）的宽度。" }

                div { style: "margin-bottom: 16px;",
                    Row {
                        Col { span: 8,
                            div {
                                style: "background: #0092ff; color: white; text-align: center; padding: 16px 0;",
                                "col-8"
                            }
                        }
                        Col { span: 8, offset: 8,
                            div {
                                style: "background: #0092ff; color: white; text-align: center; padding: 16px 0;",
                                "col-8 col-offset-8"
                            }
                        }
                    }
                }

                div { style: "margin-bottom: 16px;",
                    Row {
                        Col { span: 6, offset: 6,
                            div {
                                style: "background: #0092ff; color: white; text-align: center; padding: 16px 0;",
                                "col-6 col-offset-6"
                            }
                        }
                        Col { span: 6, offset: 6,
                            div {
                                style: "background: #0092ff; color: white; text-align: center; padding: 16px 0;",
                                "col-6 col-offset-6"
                            }
                        }
                    }
                }

                div { style: "margin-bottom: 16px;",
                    Row {
                        Col { span: 12, offset: 6,
                            div {
                                style: "background: #0092ff; color: white; text-align: center; padding: 16px 0;",
                                "col-12 col-offset-6"
                            }
                        }
                    }
                }
            }

            // 栅格排序
            section {
                style: "margin-bottom: 32px;",
                h2 { "栅格排序" }
                p { "列排序。" }
                p { "通过使用 push 和 pull 类就可以很容易的改变列（column）的顺序。" }

                div { style: "margin-bottom: 16px;",
                    Row {
                        Col { span: 18, push: 6,
                            div {
                                style: "background: #0092ff; color: white; text-align: center; padding: 16px 0;",
                                "col-18 col-push-6"
                            }
                        }
                        Col { span: 6, pull: 18,
                            div {
                                style: "background: #096dd9; color: white; text-align: center; padding: 16px 0;",
                                "col-6 col-pull-18"
                            }
                        }
                    }
                }
            }

            // Flex 布局
            section {
                style: "margin-bottom: 32px;",
                h2 { "Flex 布局" }
                p { "Flex 子元素垂直对齐。" }

                div { style: "margin-bottom: 16px;",
                    p { "Align Top" }
                    Row { justify: Justify::Center, align: Align::Top,
                        style: "background: rgba(128, 128, 128, 0.08);",
                        Col { span: 4,
                            div {
                                style: "background: #0092ff; color: white; text-align: center; padding: 40px 0;",
                                "col-4"
                            }
                        }
                        Col { span: 4,
                            div {
                                style: "background: #096dd9; color: white; text-align: center; padding: 20px 0;",
                                "col-4"
                            }
                        }
                        Col { span: 4,
                            div {
                                style: "background: #0092ff; color: white; text-align: center; padding: 40px 0;",
                                "col-4"
                            }
                        }
                        Col { span: 4,
                            div {
                                style: "background: #096dd9; color: white; text-align: center; padding: 20px 0;",
                                "col-4"
                            }
                        }
                    }
                }

                div { style: "margin-bottom: 16px;",
                    p { "Align Middle" }
                    Row { justify: Justify::SpaceAround, align: Align::Middle,
                        style: "background: rgba(128, 128, 128, 0.08);",
                        Col { span: 4,
                            div {
                                style: "background: #0092ff; color: white; text-align: center; padding: 40px 0;",
                                "col-4"
                            }
                        }
                        Col { span: 4,
                            div {
                                style: "background: #096dd9; color: white; text-align: center; padding: 20px 0;",
                                "col-4"
                            }
                        }
                        Col { span: 4,
                            div {
                                style: "background: #0092ff; color: white; text-align: center; padding: 40px 0;",
                                "col-4"
                            }
                        }
                        Col { span: 4,
                            div {
                                style: "background: #096dd9; color: white; text-align: center; padding: 20px 0;",
                                "col-4"
                            }
                        }
                    }
                }

                div { style: "margin-bottom: 16px;",
                    p { "Align Bottom" }
                    Row { justify: Justify::SpaceBetween, align: Align::Bottom,
                        style: "background: rgba(128, 128, 128, 0.08);",
                        Col { span: 4,
                            div {
                                style: "background: #0092ff; color: white; text-align: center; padding: 40px 0;",
                                "col-4"
                            }
                        }
                        Col { span: 4,
                            div {
                                style: "background: #096dd9; color: white; text-align: center; padding: 20px 0;",
                                "col-4"
                            }
                        }
                        Col { span: 4,
                            div {
                                style: "background: #0092ff; color: white; text-align: center; padding: 40px 0;",
                                "col-4"
                            }
                        }
                        Col { span: 4,
                            div {
                                style: "background: #096dd9; color: white; text-align: center; padding: 20px 0;",
                                "col-4"
                            }
                        }
                    }
                }
            }

            // 响应式布局
            section {
                style: "margin-bottom: 32px;",
                h2 { "响应式布局" }
                p { "参照 Bootstrap 的 响应式设计，预设六个响应尺寸：xs sm md lg xl xxl。" }

                div { style: "margin-bottom: 16px;",
                    Row {
                        Col { span: Some(6),
                            div {
                                style: "background: #0092ff; color: white; text-align: center; padding: 16px 0;",
                                "Responsive Col"
                            }
                        }
                        Col { span: Some(6),
                            div {
                                style: "background: #096dd9; color: white; text-align: center; padding: 16px 0;",
                                "Responsive Col"
                            }
                        }
                        Col { span: Some(6),
                            div {
                                style: "background: #0092ff; color: white; text-align: center; padding: 16px 0;",
                                "Responsive Col"
                            }
                        }
                        Col { span: Some(6),
                            div {
                                style: "background: #096dd9; color: white; text-align: center; padding: 16px 0;",
                                "Responsive Col"
                            }
                        }
                    }
                }
            }
        }
    }
}

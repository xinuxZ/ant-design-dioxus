//! Grid 栅格组件演示
//!
//! 展示 24 栅格系统的基本用法、响应式布局、对齐方式等功能

use ant_design_dioxus::components::grid::Align as GridAlign;
use ant_design_dioxus::prelude::*;
use dioxus::prelude::{rsx, Element};

// 引用未使用的依赖以避免编译错误
#[allow(unused_imports)]
use once_cell as _;
#[allow(unused_imports)]
use serde as _;
#[allow(unused_imports)]
use serde_json as _;
#[allow(unused_imports)]
use wasm_bindgen_test as _;
#[allow(unused_imports)]
use web_sys as _;

fn main() {
    dioxus::launch(app);
}

/// 应用主组件
///
/// 展示栅格系统的各种使用场景
fn app() -> Element {
    rsx! {
        div {
            style: "padding: 20px; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;",

            h1 { "Grid 栅格系统演示" }

            // 基础栅格
            section {
                style: "margin-bottom: 40px;",
                h2 { "基础栅格" }
                p { "使用 Row 和 Col 组件，通过 span 属性设置列的宽度" }

                Row {
                    gutter: 16,
                    Col {
                        span: 24,
                        div {
                            style: "background: #0092ff; color: white; text-align: center; padding: 16px 0;",
                            "col-24"
                        }
                    }
                }

                br {}

                Row {
                    gutter: 16,
                    Col {
                        span: 12,
                        div {
                            style: "background: #0092ff; color: white; text-align: center; padding: 16px 0;",
                            "col-12"
                        }
                    }
                    Col {
                        span: 12,
                        div {
                            style: "background: #096dd9; color: white; text-align: center; padding: 16px 0;",
                            "col-12"
                        }
                    }
                }

                br {}

                Row {
                    gutter: 16,
                    Col {
                        span: 8,
                        div {
                            style: "background: #0092ff; color: white; text-align: center; padding: 16px 0;",
                            "col-8"
                        }
                    }
                    Col {
                        span: 8,
                        div {
                            style: "background: #096dd9; color: white; text-align: center; padding: 16px 0;",
                            "col-8"
                        }
                    }
                    Col {
                        span: 8,
                        div {
                            style: "background: #0092ff; color: white; text-align: center; padding: 16px 0;",
                            "col-8"
                        }
                    }
                }

                br {}

                Row {
                    gutter: 16,
                    Col {
                        span: 6,
                        div {
                            style: "background: #0092ff; color: white; text-align: center; padding: 16px 0;",
                            "col-6"
                        }
                    }
                    Col {
                        span: 6,
                        div {
                            style: "background: #096dd9; color: white; text-align: center; padding: 16px 0;",
                            "col-6"
                        }
                    }
                    Col {
                        span: 6,
                        div {
                            style: "background: #0092ff; color: white; text-align: center; padding: 16px 0;",
                            "col-6"
                        }
                    }
                    Col {
                        span: 6,
                        div {
                            style: "background: #096dd9; color: white; text-align: center; padding: 16px 0;",
                            "col-6"
                        }
                    }
                }
            }

            // 区块间隔
            section {
                style: "margin-bottom: 40px;",
                h2 { "区块间隔" }
                p { "栅格常常需要和间隔进行配合，你可以使用 Row 的 gutter 属性" }

                Row {
                    gutter: 16,
                    Col {
                        span: 6,
                        div {
                            style: "background: #0092ff; color: white; text-align: center; padding: 16px 0;",
                            "col-6"
                        }
                    }
                    Col {
                        span: 6,
                        div {
                            style: "background: #0092ff; color: white; text-align: center; padding: 16px 0;",
                            "col-6"
                        }
                    }
                    Col {
                        span: 6,
                        div {
                            style: "background: #0092ff; color: white; text-align: center; padding: 16px 0;",
                            "col-6"
                        }
                    }
                    Col {
                        span: 6,
                        div {
                            style: "background: #0092ff; color: white; text-align: center; padding: 16px 0;",
                            "col-6"
                        }
                    }
                }

                br {}

                Row {
                    gutter: 32,
                    Col {
                        span: 6,
                        div {
                            style: "background: #096dd9; color: white; text-align: center; padding: 16px 0;",
                            "col-6"
                        }
                    }
                    Col {
                        span: 6,
                        div {
                            style: "background: #096dd9; color: white; text-align: center; padding: 16px 0;",
                            "col-6"
                        }
                    }
                    Col {
                        span: 6,
                        div {
                            style: "background: #096dd9; color: white; text-align: center; padding: 16px 0;",
                            "col-6"
                        }
                    }
                    Col {
                        span: 6,
                        div {
                            style: "background: #096dd9; color: white; text-align: center; padding: 16px 0;",
                            "col-6"
                        }
                    }
                }
            }

            // 左右偏移
            section {
                style: "margin-bottom: 40px;",
                h2 { "左右偏移" }
                p { "列偏移。使用 offset 可以将列向右侧偏移" }

                Row {
                    Col {
                        span: 8,
                        div {
                            style: "background: #0092ff; color: white; text-align: center; padding: 16px 0;",
                            "col-8"
                        }
                    }
                    Col {
                        span: 8,
                        offset: 8,
                        div {
                            style: "background: #096dd9; color: white; text-align: center; padding: 16px 0;",
                            "col-8 col-offset-8"
                        }
                    }
                }

                br {}

                Row {
                    Col {
                        span: 6,
                        offset: 6,
                        div {
                            style: "background: #0092ff; color: white; text-align: center; padding: 16px 0;",
                            "col-6 col-offset-6"
                        }
                    }
                    Col {
                        span: 6,
                        offset: 6,
                        div {
                            style: "background: #096dd9; color: white; text-align: center; padding: 16px 0;",
                            "col-6 col-offset-6"
                        }
                    }
                }
            }

            // 栅格排序
            section {
                style: "margin-bottom: 40px;",
                h2 { "栅格排序" }
                p { "列排序。通过使用 push 和 pull 类就可以很容易的改变列（column）的顺序" }

                Row {
                    Col {
                        span: 18,
                        push: 6,
                        div {
                            style: "background: #0092ff; color: white; text-align: center; padding: 16px 0;",
                            "col-18 col-push-6"
                        }
                    }
                    Col {
                        span: 6,
                        pull: 18,
                        div {
                            style: "background: #096dd9; color: white; text-align: center; padding: 16px 0;",
                            "col-6 col-pull-18"
                        }
                    }
                }
            }

            // Flex 布局
            section {
                style: "margin-bottom: 40px;",
                h2 { "Flex 布局" }
                p { "Flex 子元素垂直对齐" }

                div {
                    style: "background: #f5f5f5; padding: 20px; margin-bottom: 16px;",
                    p { "Align Top" }
                    Row {
                        justify: Justify::Center,
                        align: GridAlign::Top,
                        Col {
                            span: 4,
                            div {
                                style: "background: #0092ff; color: white; text-align: center; padding: 40px 0;",
                                "col-4"
                            }
                        }
                        Col {
                            span: 4,
                            div {
                                style: "background: #096dd9; color: white; text-align: center; padding: 30px 0;",
                                "col-4"
                            }
                        }
                        Col {
                            span: 4,
                            div {
                                style: "background: #0092ff; color: white; text-align: center; padding: 50px 0;",
                                "col-4"
                            }
                        }
                        Col {
                            span: 4,
                            div {
                                style: "background: #096dd9; color: white; text-align: center; padding: 20px 0;",
                                "col-4"
                            }
                        }
                    }
                }

                div {
                    style: "background: #f5f5f5; padding: 20px; margin-bottom: 16px;",
                    p { "Align Middle" }
                    Row {
                        justify: Justify::SpaceAround,
                        align: GridAlign::Middle,
                        Col {
                            span: 4,
                            div {
                                style: "background: #0092ff; color: white; text-align: center; padding: 40px 0;",
                                "col-4"
                            }
                        }
                        Col {
                            span: 4,
                            div {
                                style: "background: #096dd9; color: white; text-align: center; padding: 30px 0;",
                                "col-4"
                            }
                        }
                        Col {
                            span: 4,
                            div {
                                style: "background: #0092ff; color: white; text-align: center; padding: 50px 0;",
                                "col-4"
                            }
                        }
                        Col {
                            span: 4,
                            div {
                                style: "background: #096dd9; color: white; text-align: center; padding: 20px 0;",
                                "col-4"
                            }
                        }
                    }
                }

                div {
                    style: "background: #f5f5f5; padding: 20px;",
                    p { "Align Bottom" }
                    Row {
                        justify: Justify::SpaceBetween,
                        align: GridAlign::Bottom,
                        Col {
                            span: 4,
                            div {
                                style: "background: #0092ff; color: white; text-align: center; padding: 40px 0;",
                                "col-4"
                            }
                        }
                        Col {
                            span: 4,
                            div {
                                style: "background: #096dd9; color: white; text-align: center; padding: 30px 0;",
                                "col-4"
                            }
                        }
                        Col {
                            span: 4,
                            div {
                                style: "background: #0092ff; color: white; text-align: center; padding: 50px 0;",
                                "col-4"
                            }
                        }
                        Col {
                            span: 4,
                            div {
                                style: "background: #096dd9; color: white; text-align: center; padding: 20px 0;",
                                "col-4"
                            }
                        }
                    }
                }
            }
        }
    }
}

//! Grid 组件示例
//!
//! 展示 Grid 栅格布局组件的各种用法

use crate::common::{ApiDocumentation, DemoSection, PropDoc};
use dioxus::prelude::*;

/// Grid 组件演示页面
#[component]
pub fn GridDemo() -> Element {
    rsx! {
        div {
            class: "grid-demo",
            style: "padding: 24px;",

            h1 {
                style: "margin: 0 0 24px 0; color: #262626; font-size: 28px;",
                "Grid 栅格"
            }

            p {
                style: "margin: 0 0 32px 0; color: #666; font-size: 14px;",
                "24 栅格系统。"
            }

            // 基础栅格
            DemoSection {
                title: "基础栅格".to_string(),
                description: "从堆叠到水平排列。使用单一的一组 Row 和 Col 栅格组件，就可以创建一个基本的栅格系统，所有列（Col）必须放在 Row 内。".to_string(),
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    // 全宽
                    div {
                        class: "demo-row",
                        style: "display: flex; margin-bottom: 16px;",
                        div {
                            class: "demo-col",
                            style: "background: #1890ff; color: white; text-align: center; padding: 16px 0; width: 100%;",
                            "col-24"
                        }
                    }

                    // 二等分
                    div {
                        class: "demo-row",
                        style: "display: flex; gap: 0; margin-bottom: 16px;",
                        div {
                            class: "demo-col",
                            style: "background: #1890ff; color: white; text-align: center; padding: 16px 0; width: 50%;",
                            "col-12"
                        }
                        div {
                            class: "demo-col",
                            style: "background: #40a9ff; color: white; text-align: center; padding: 16px 0; width: 50%;",
                            "col-12"
                        }
                    }

                    // 三等分
                    div {
                        class: "demo-row",
                        style: "display: flex; gap: 0; margin-bottom: 16px;",
                        div {
                            class: "demo-col",
                            style: "background: #1890ff; color: white; text-align: center; padding: 16px 0; width: 33.333333%;",
                            "col-8"
                        }
                        div {
                            class: "demo-col",
                            style: "background: #40a9ff; color: white; text-align: center; padding: 16px 0; width: 33.333333%;",
                            "col-8"
                        }
                        div {
                            class: "demo-col",
                            style: "background: #1890ff; color: white; text-align: center; padding: 16px 0; width: 33.333333%;",
                            "col-8"
                        }
                    }

                    // 四等分
                    div {
                        class: "demo-row",
                        style: "display: flex; gap: 0;",
                        div {
                            class: "demo-col",
                            style: "background: #1890ff; color: white; text-align: center; padding: 16px 0; width: 25%;",
                            "col-6"
                        }
                        div {
                            class: "demo-col",
                            style: "background: #40a9ff; color: white; text-align: center; padding: 16px 0; width: 25%;",
                            "col-6"
                        }
                        div {
                            class: "demo-col",
                            style: "background: #1890ff; color: white; text-align: center; padding: 16px 0; width: 25%;",
                            "col-6"
                        }
                        div {
                            class: "demo-col",
                            style: "background: #40a9ff; color: white; text-align: center; padding: 16px 0; width: 25%;",
                            "col-6"
                        }
                    }
                }
            }

            // 区间间隔
            DemoSection {
                title: "区间间隔".to_string(),
                description: "栅格常常需要和间隔进行配合，你可以使用 Row 的 gutter 属性，我们推荐使用 (16+8n)px 作为栅格间隔(n 是自然数)。".to_string(),
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    // 无间隔
                    div {
                        h4 { style: "margin: 0 0 8px 0; color: #666;", "Gutter = 0" }
                        div {
                            class: "demo-row",
                            style: "display: flex; gap: 0;",
                            div {
                                class: "demo-col",
                                style: "background: #1890ff; color: white; text-align: center; padding: 16px 0; width: 25%;",
                                "col-6"
                            }
                            div {
                                class: "demo-col",
                                style: "background: #40a9ff; color: white; text-align: center; padding: 16px 0; width: 25%;",
                                "col-6"
                            }
                            div {
                                class: "demo-col",
                                style: "background: #1890ff; color: white; text-align: center; padding: 16px 0; width: 25%;",
                                "col-6"
                            }
                            div {
                                class: "demo-col",
                                style: "background: #40a9ff; color: white; text-align: center; padding: 16px 0; width: 25%;",
                                "col-6"
                            }
                        }
                    }

                    // 16px 间隔
                    div {
                        h4 { style: "margin: 0 0 8px 0; color: #666;", "Gutter = 16" }
                        div {
                            class: "demo-row",
                            style: "display: flex; gap: 16px;",
                            div {
                                class: "demo-col",
                                style: "background: #1890ff; color: white; text-align: center; padding: 16px 0; flex: 1;",
                                "col-6"
                            }
                            div {
                                class: "demo-col",
                                style: "background: #40a9ff; color: white; text-align: center; padding: 16px 0; flex: 1;",
                                "col-6"
                            }
                            div {
                                class: "demo-col",
                                style: "background: #1890ff; color: white; text-align: center; padding: 16px 0; flex: 1;",
                                "col-6"
                            }
                            div {
                                class: "demo-col",
                                style: "background: #40a9ff; color: white; text-align: center; padding: 16px 0; flex: 1;",
                                "col-6"
                            }
                        }
                    }
                }
            }

            // 左右偏移
            DemoSection {
                title: "左右偏移".to_string(),
                description: "列偏移。使用 offset 可以将列向右侧偏。例如，offset={4} 将元素向右侧偏移了 4 个列（column）的宽度。".to_string(),
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    div {
                        class: "demo-row",
                        style: "display: flex; gap: 0;",
                        div {
                            class: "demo-col",
                            style: "background: #1890ff; color: white; text-align: center; padding: 16px 0; width: 33.333333%;",
                            "col-8"
                        }
                        div {
                            class: "demo-col",
                            style: "background: #40a9ff; color: white; text-align: center; padding: 16px 0; width: 33.333333%; margin-left: 33.333333%;",
                            "col-8 offset-8"
                        }
                    }

                    div {
                        class: "demo-row",
                        style: "display: flex; gap: 0;",
                        div {
                            class: "demo-col",
                            style: "background: #1890ff; color: white; text-align: center; padding: 16px 0; width: 25%;",
                            "col-6"
                        }
                        div {
                            class: "demo-col",
                            style: "background: #40a9ff; color: white; text-align: center; padding: 16px 0; width: 25%; margin-left: 25%;",
                            "col-6 offset-6"
                        }
                    }

                    div {
                        class: "demo-row",
                        style: "display: flex; gap: 0;",
                        div {
                            class: "demo-col",
                            style: "background: #40a9ff; color: white; text-align: center; padding: 16px 0; width: 50%; margin-left: 25%;",
                            "col-12 offset-6"
                        }
                    }
                }
            }

            // 栅格排序
            DemoSection {
                title: "栅格排序".to_string(),
                description: "列排序。通过使用 push 和 pull 类就可以很容易的改变列（column）的顺序。".to_string(),
                div {
                    class: "demo-row",
                    style: "display: flex; gap: 0;",
                    div {
                        class: "demo-col",
                        style: "background: #1890ff; color: white; text-align: center; padding: 16px 0; width: 75%; order: 2;",
                        "col-18 push-6"
                    }
                    div {
                        class: "demo-col",
                        style: "background: #40a9ff; color: white; text-align: center; padding: 16px 0; width: 25%; order: 1;",
                        "col-6 pull-18"
                    }
                }
            }

            // Flex 布局
            DemoSection {
                title: "Flex 布局".to_string(),
                description: "Flex 子元素垂直对齐。".to_string(),
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    div {
                        h4 { style: "margin: 0 0 8px 0; color: #666;", "Align Top" }
                        div {
                            class: "demo-row",
                            style: "display: flex; align-items: flex-start; background: #f5f5f5; padding: 8px;",
                            div {
                                class: "demo-col",
                                style: "background: #1890ff; color: white; text-align: center; padding: 16px 0; width: 25%; height: 100px;",
                                "col-6"
                            }
                            div {
                                class: "demo-col",
                                style: "background: #40a9ff; color: white; text-align: center; padding: 16px 0; width: 25%; height: 50px;",
                                "col-6"
                            }
                            div {
                                class: "demo-col",
                                style: "background: #1890ff; color: white; text-align: center; padding: 16px 0; width: 25%; height: 120px;",
                                "col-6"
                            }
                            div {
                                class: "demo-col",
                                style: "background: #40a9ff; color: white; text-align: center; padding: 16px 0; width: 25%; height: 80px;",
                                "col-6"
                            }
                        }
                    }

                    div {
                        h4 { style: "margin: 0 0 8px 0; color: #666;", "Align Center" }
                        div {
                            class: "demo-row",
                            style: "display: flex; align-items: center; background: #f5f5f5; padding: 8px;",
                            div {
                                class: "demo-col",
                                style: "background: #1890ff; color: white; text-align: center; padding: 16px 0; width: 25%; height: 100px;",
                                "col-6"
                            }
                            div {
                                class: "demo-col",
                                style: "background: #40a9ff; color: white; text-align: center; padding: 16px 0; width: 25%; height: 50px;",
                                "col-6"
                            }
                            div {
                                class: "demo-col",
                                style: "background: #1890ff; color: white; text-align: center; padding: 16px 0; width: 25%; height: 120px;",
                                "col-6"
                            }
                            div {
                                class: "demo-col",
                                style: "background: #40a9ff; color: white; text-align: center; padding: 16px 0; width: 25%; height: 80px;",
                                "col-6"
                            }
                        }
                    }

                    div {
                        h4 { style: "margin: 0 0 8px 0; color: #666;", "Align Bottom" }
                        div {
                            class: "demo-row",
                            style: "display: flex; align-items: flex-end; background: #f5f5f5; padding: 8px;",
                            div {
                                class: "demo-col",
                                style: "background: #1890ff; color: white; text-align: center; padding: 16px 0; width: 25%; height: 100px;",
                                "col-6"
                            }
                            div {
                                class: "demo-col",
                                style: "background: #40a9ff; color: white; text-align: center; padding: 16px 0; width: 25%; height: 50px;",
                                "col-6"
                            }
                            div {
                                class: "demo-col",
                                style: "background: #1890ff; color: white; text-align: center; padding: 16px 0; width: 25%; height: 120px;",
                                "col-6"
                            }
                            div {
                                class: "demo-col",
                                style: "background: #40a9ff; color: white; text-align: center; padding: 16px 0; width: 25%; height: 80px;",
                                "col-6"
                            }
                        }
                    }
                }
            }

            // API 文档
            ApiDocumentation {
                component_name: "Row".to_string(),
                props: vec![
                    PropDoc {
                        name: "gutter".to_string(),
                        prop_type: "u32".to_string(),
                        default: "0".to_string(),
                        description: "栅格间隔，可以写成像素值或支持响应式的对象写法来设置水平间隔".to_string(),
                    },
                    PropDoc {
                        name: "justify".to_string(),
                        prop_type: "Justify".to_string(),
                        default: "Start".to_string(),
                        description: "flex 布局下的水平排列方式".to_string(),
                    },
                    PropDoc {
                        name: "align".to_string(),
                        prop_type: "Align".to_string(),
                        default: "Top".to_string(),
                        description: "flex 布局下的垂直对齐方式".to_string(),
                    },
                ]
            }

            ApiDocumentation {
                component_name: "Col".to_string(),
                props: vec![
                    PropDoc {
                        name: "span".to_string(),
                        prop_type: "u32".to_string(),
                        default: "24".to_string(),
                        description: "栅格占位格数，为 0 时相当于 display: none".to_string(),
                    },
                    PropDoc {
                        name: "offset".to_string(),
                        prop_type: "u32".to_string(),
                        default: "0".to_string(),
                        description: "栅格左侧的间隔格数，间隔内不可以有栅格".to_string(),
                    },
                    PropDoc {
                        name: "push".to_string(),
                        prop_type: "u32".to_string(),
                        default: "0".to_string(),
                        description: "栅格向右移动格数".to_string(),
                    },
                    PropDoc {
                        name: "pull".to_string(),
                        prop_type: "u32".to_string(),
                        default: "0".to_string(),
                        description: "栅格向左移动格数".to_string(),
                    },
                    PropDoc {
                        name: "order".to_string(),
                        prop_type: "i32".to_string(),
                        default: "0".to_string(),
                        description: "栅格顺序".to_string(),
                    },
                ]
            }
        }
    }
}

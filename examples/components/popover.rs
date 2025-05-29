#![allow(non_snake_case)]
//!
//! 展示 Popover 组件的各种用法和样式

use crate::common::*;
use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

/// Popover 组件演示
#[component]
pub fn PopoverDemo() -> Element {
    let mut clicked = use_signal(|| false);
    let mut hovered = use_signal(|| false);

    rsx! {
        div {
            class: "component-showcase",

            h1 {
                style: "margin: 0 0 16px 0; color: #262626; font-size: 28px;",
                "Popover 气泡卡片"
            }

            p {
                style: "margin: 0 0 24px 0; color: #666; font-size: 14px;",
                "点击/鼠标移入元素，弹出气泡式的卡片浮层。"
            }

            // 基础用法
            DemoSection {
                title: "基础用法",
                description: "最简单的用法，浮层的大小由内容区域决定。",

                div {
                    style: "display: flex; gap: 8px; flex-wrap: wrap;",

                    Popover {
                        content: rsx! {
                            div {
                                p { "Content" }
                                p { "Content" }
                            }
                        },
                        title: "Title",

                        Button {
                            button_type: ButtonType::Primary,
                            "Hover me"
                        }
                    }
                }
            }

            // 三种触发方式
            DemoSection {
                title: "三种触发方式",
                description: "鼠标移入、聚焦、点击。",

                div {
                    style: "display: flex; gap: 8px; flex-wrap: wrap;",

                    Popover {
                        content: rsx! {
                            div {
                                p { "Content" }
                                p { "Content" }
                            }
                        },
                        title: "Title",
                        trigger: PopoverTrigger::Hover,

                        Button {
                            "Hover me"
                        }
                    }

                    Popover {
                        content: rsx! {
                            div {
                                p { "Content" }
                                p { "Content" }
                            }
                        },
                        title: "Title",
                        trigger: PopoverTrigger::Focus,

                        Button {
                            "Focus me"
                        }
                    }

                    Popover {
                        content: rsx! {
                            div {
                                p { "Content" }
                                p { "Content" }
                            }
                        },
                        title: "Title",
                        trigger: PopoverTrigger::Click,

                        Button {
                            "Click me"
                        }
                    }
                }
            }

            // 位置
            DemoSection {
                title: "位置",
                description: "位置有十二个方向。",

                div {
                    style: "display: flex; gap: 8px; flex-wrap: wrap;",

                    Popover {
                        placement: PopoverPlacement::TopLeft,
                        title: "Title",
                        content: rsx! {
                            div {
                                p { "Content" }
                                p { "Content" }
                            }
                        },

                        Button {
                            "TL"
                        }
                    }

                    Popover {
                        placement: PopoverPlacement::Top,
                        title: "Title",
                        content: rsx! {
                            div {
                                p { "Content" }
                                p { "Content" }
                            }
                        },

                        Button {
                            "Top"
                        }
                    }

                    Popover {
                        placement: PopoverPlacement::TopRight,
                        title: "Title",
                        content: rsx! {
                            div {
                                p { "Content" }
                                p { "Content" }
                            }
                        },

                        Button {
                            "TR"
                        }
                    }

                    Popover {
                        placement: PopoverPlacement::LeftTop,
                        title: "Title",
                        content: rsx! {
                            div {
                                p { "Content" }
                                p { "Content" }
                            }
                        },

                        Button {
                            "LT"
                        }
                    }

                    Popover {
                        placement: PopoverPlacement::Left,
                        title: "Title",
                        content: rsx! {
                            div {
                                p { "Content" }
                                p { "Content" }
                            }
                        },

                        Button {
                            "Left"
                        }
                    }

                    Popover {
                        placement: PopoverPlacement::LeftBottom,
                        title: "Title",
                        content: rsx! {
                            div {
                                p { "Content" }
                                p { "Content" }
                            }
                        },

                        Button {
                            "LB"
                        }
                    }

                    Popover {
                        placement: PopoverPlacement::RightTop,
                        title: "Title",
                        content: rsx! {
                            div {
                                p { "Content" }
                                p { "Content" }
                            }
                        },

                        Button {
                            "RT"
                        }
                    }

                    Popover {
                        placement: PopoverPlacement::Right,
                        title: "Title",
                        content: rsx! {
                            div {
                                p { "Content" }
                                p { "Content" }
                            }
                        },

                        Button {
                            "Right"
                        }
                    }

                    Popover {
                        placement: PopoverPlacement::RightBottom,
                        title: "Title",
                        content: rsx! {
                            div {
                                p { "Content" }
                                p { "Content" }
                            }
                        },

                        Button {
                            "RB"
                        }
                    }

                    Popover {
                        placement: PopoverPlacement::BottomLeft,
                        title: "Title",
                        content: rsx! {
                            div {
                                p { "Content" }
                                p { "Content" }
                            }
                        },

                        Button {
                            "BL"
                        }
                    }

                    Popover {
                        placement: PopoverPlacement::Bottom,
                        title: "Title",
                        content: rsx! {
                            div {
                                p { "Content" }
                                p { "Content" }
                            }
                        },

                        Button {
                            "Bottom"
                        }
                    }

                    Popover {
                        placement: PopoverPlacement::BottomRight,
                        title: "Title",
                        content: rsx! {
                            div {
                                p { "Content" }
                                p { "Content" }
                            }
                        },

                        Button {
                            "BR"
                        }
                    }
                }
            }

            // 箭头指向
            DemoSection {
                title: "箭头指向",
                description: "设置了 arrowPointAtCenter 的气泡卡片。",

                div {
                    style: "display: flex; gap: 8px; flex-wrap: wrap;",

                    Popover {
                        placement: PopoverPlacement::TopLeft,
                        title: "Title",
                        content: rsx! {
                            div {
                                p { "Content" }
                                p { "Content" }
                            }
                        },

                        Button {
                            "Align edge / 边缘对齐"
                        }
                    }

                    Popover {
                        placement: PopoverPlacement::TopLeft,
                        title: "Title",
                        content: rsx! {
                            div {
                                p { "Content" }
                                p { "Content" }
                            }
                        },
                        // arrow_point_at_center: true,

                        Button {
                            "Arrow points to center / 箭头指向中心"
                        }
                    }
                }
            }

            // 从浮层内关闭
            DemoSection {
                title: "从浮层内关闭",
                description: "使用 visible 属性控制浮层显示。",

                div {
                    style: "display: flex; gap: 8px; flex-wrap: wrap;",

                    Popover {
                        content: rsx! {
                            div {
                                p { "Content" }
                                p { "Content" }
                                Button {
                                    button_type: ButtonType::Primary,
                                    size: ButtonSize::Small,
                                    onclick: move |_| {
                                        clicked.set(false);
                                    },
                                    "Close"
                                }
                            }
                        },
                        title: "Title",
                        trigger: PopoverTrigger::Click,
                        open: clicked(),
                        on_open_change: move |open| {
                            clicked.set(open);
                        },

                        Button {
                            button_type: ButtonType::Primary,
                            "Click me"
                        }
                    }
                }
            }

            // 悬停点击弹出窗口
            DemoSection {
                title: "悬停点击弹出窗口",
                description: "以下示例显示如何创建可悬停和可点击的弹出窗口。",

                div {
                    style: "display: flex; gap: 8px; flex-wrap: wrap;",

                    Popover {
                        style: "width: 500px;",
                        content: rsx! {
                            div {
                                p { "This is hover content." }
                            }
                        },
                        title: "Hover title",
                        trigger: PopoverTrigger::Hover,

                        Button {
                            // onmouseenter: move |_| {
                            //     hovered.set(true);
                            // },
                            // onmouseleave: move |_| {
                            //     hovered.set(false);
                            // },
                            "Hover me"
                        }
                    }

                    Popover {
                        content: rsx! {
                            div {
                                p { "This is click content." }
                                Button {
                                    button_type: ButtonType::Primary,
                                    size: ButtonSize::Small,
                                    onclick: move |_| {
                                        clicked.set(false);
                                    },
                                    "Close"
                                }
                            }
                        },
                        title: "Click title",
                        trigger: PopoverTrigger::Click,
                        open: clicked(),
                        on_open_change: move |open| {
                            clicked.set(open);
                        },

                        Button {
                            "Click me"
                        }
                    }
                }
            }

            // API 文档
            ApiDocumentation {
                component_name: "Popover",
                props: vec![
                    PropDoc {
                        name: "content".to_string(),
                        prop_type: "String | Element".to_string(),
                        default: "-".to_string(),
                        description: "卡片内容".to_string(),
                    },
                    PropDoc {
                        name: "title".to_string(),
                        prop_type: "String | Element".to_string(),
                        default: "-".to_string(),
                        description: "卡片标题".to_string(),
                    },
                    PropDoc {
                        name: "arrow_point_at_center".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "箭头是否指向目标元素中心".to_string(),
                    },
                    PropDoc {
                        name: "auto_adjust_overflow".to_string(),
                        prop_type: "bool".to_string(),
                        default: "true".to_string(),
                        description: "气泡被遮挡时自动调整位置".to_string(),
                    },
                    PropDoc {
                        name: "color".to_string(),
                        prop_type: "String".to_string(),
                        default: "-".to_string(),
                        description: "背景颜色".to_string(),
                    },
                    PropDoc {
                        name: "default_open".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "默认是否显隐".to_string(),
                    },
                    PropDoc {
                        name: "destroy_tooltip_on_hide".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "关闭后是否销毁 Tooltip".to_string(),
                    },
                    PropDoc {
                        name: "get_popup_container".to_string(),
                        prop_type: "Function".to_string(),
                        default: "() => document.body".to_string(),
                        description: "浮层渲染父节点，默认渲染到 body 上".to_string(),
                    },
                    PropDoc {
                        name: "mouse_enter_delay".to_string(),
                        prop_type: "f64".to_string(),
                        default: "0.1".to_string(),
                        description: "鼠标移入后延时多少才显示 Tooltip，单位：秒".to_string(),
                    },
                    PropDoc {
                        name: "mouse_leave_delay".to_string(),
                        prop_type: "f64".to_string(),
                        default: "0.1".to_string(),
                        description: "鼠标移出后延时多少才隐藏 Tooltip，单位：秒".to_string(),
                    },
                    PropDoc {
                        name: "overlay_class_name".to_string(),
                        prop_type: "String".to_string(),
                        default: "-".to_string(),
                        description: "卡片类名".to_string(),
                    },
                    PropDoc {
                        name: "overlay_style".to_string(),
                        prop_type: "String".to_string(),
                        default: "-".to_string(),
                        description: "卡片样式".to_string(),
                    },
                    PropDoc {
                        name: "placement".to_string(),
                        prop_type: "String".to_string(),
                        default: "top".to_string(),
                        description: "气泡框位置，可选 top left right bottom topLeft topRight bottomLeft bottomRight leftTop leftBottom rightTop rightBottom".to_string(),
                    },
                    PropDoc {
                        name: "trigger".to_string(),
                        prop_type: "String".to_string(),
                        default: "hover".to_string(),
                        description: "触发行为，可选 hover/focus/click/contextMenu".to_string(),
                    },
                    PropDoc {
                        name: "open".to_string(),
                        prop_type: "bool".to_string(),
                        default: "-".to_string(),
                        description: "用于手动控制浮层显隐".to_string(),
                    },
                    PropDoc {
                        name: "z_index".to_string(),
                        prop_type: "i32".to_string(),
                        default: "-".to_string(),
                        description: "设置浮层的 z-index".to_string(),
                    },
                    PropDoc {
                        name: "on_open_change".to_string(),
                        prop_type: "Function".to_string(),
                        default: "-".to_string(),
                        description: "显示隐藏的回调".to_string(),
                    },
                ]
            }
        }
    }
}

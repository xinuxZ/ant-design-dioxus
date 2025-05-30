#![allow(non_snake_case)]
//!
//! 展示 Timeline 组件的各种用法和样式

use crate::common::*;
use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

/// Timeline 组件演示
#[component]
pub fn TimelineDemo() -> Element {
    let mut reverse = use_signal(|| false);
    let mut mode = use_signal(|| "left".to_string());

    rsx! {
        div {
            class: "component-showcase",

            h1 {
                style: "margin: 0 0 16px 0; color: #262626; font-size: 28px;",
                "Timeline 时间轴"
            }

            p {
                style: "margin: 0 0 24px 0; color: #666; font-size: 14px;",
                "垂直展示的时间流信息。"
            }

            // 基础用法
            DemoSection {
                title: "基础用法",
                description: "基本的时间轴。",

                div {
                    style: "padding: 24px; background: #fafafa; border-radius: 6px;",

                    Timeline {
                        TimelineItem {
                            "Create a services site 2015-09-01"
                        }
                        TimelineItem {
                            "Solve initial network problems 2015-09-01"
                        }
                        TimelineItem {
                            "Technical testing 2015-09-01"
                        }
                        TimelineItem {
                            "Network problems being solved 2015-09-01"
                        }
                    }
                }
            }

            // 圆圈颜色
            DemoSection {
                title: "圆圈颜色",
                description: "圆圈颜色，绿色用于已完成、成功状态，红色表示告警或错误状态，蓝色可表示正在进行或其他默认状态，灰色表示未完成或禁用状态。",

                div {
                    style: "padding: 24px; background: #fafafa; border-radius: 6px;",

                    Timeline {
                        TimelineItem {
                            color: TimelineItemColor::Red,
                            "Create a services site 2015-09-01"
                        }
                        TimelineItem {
                            color: TimelineItemColor::Red,
                            "Create a services site 2015-09-01"
                        }
                        TimelineItem {
                            color: TimelineItemColor::Red,
                            p { "Solve initial network problems 2015-09-01" }
                            p { "Network problems being solved 2015-09-01" }
                        }
                        TimelineItem {
                            p { "Technical testing 2015-09-01" }
                            p { "Network problems being solved 2015-09-01" }
                        }
                        TimelineItem {
                            color: TimelineItemColor::Gray,
                            p { "Technical testing 2015-09-01" }
                            p { "Network problems being solved 2015-09-01" }
                        }
                        TimelineItem {
                            color: TimelineItemColor::Gray,
                            p { "Technical testing 2015-09-01" }
                            p { "Network problems being solved 2015-09-01" }
                        }
                    }
                }
            }

            // 最后一个及排序
            DemoSection {
                title: "最后一个及排序",
                description: "当任务状态正在发生，还在记录过程中，可用幽灵节点来表示当前的时间节点，当 pending 为真值时展示幽灵节点，如果 pending 是 React 元素可用于定制该节点内容，同时 pendingDot 将可以用于定制其轴点。reverse 属性用于控制节点排序，为 false 时按正序排列，为 true 时按倒序排列。",

                div {
                    style: "padding: 24px; background: #fafafa; border-radius: 6px;",

                    div {
                        style: "margin-bottom: 16px;",
                        Button {
                            onclick: move |_| {
                                reverse.set(!reverse());
                            },
                            "Toggle Reverse"
                        }
                    }

                    Timeline {
                        pending: Some(rsx! { "Recording..." }).is_some(),
                        reverse: reverse(),
                        TimelineItem {
                            "Create a services site 2015-09-01"
                        }
                        TimelineItem {
                            "Solve initial network problems 2015-09-01"
                        }
                        TimelineItem {
                            "Technical testing 2015-09-01"
                        }
                    }
                }
            }

            // 自定义时间轴点
            DemoSection {
                title: "自定义时间轴点",
                description: "可以设置为图标或其他自定义元素。",

                div {
                    style: "padding: 24px; background: #fafafa; border-radius: 6px;",

                    Timeline {
                        TimelineItem {
                            "Create a services site 2015-09-01"
                        }
                        TimelineItem {
                            "Solve initial network problems 2015-09-01"
                        }
                        TimelineItem {
                            dot: "clock-circle-outlined".to_string(),
                            color: TimelineItemColor::Red,
                            "Technical testing 2015-09-01"
                        }
                        TimelineItem {
                            "Network problems being solved 2015-09-01"
                        }
                    }
                }
            }

            // 右侧时间轴点
            DemoSection {
                title: "右侧时间轴点",
                description: "时间轴点可以在内容的右边。",

                div {
                    style: "padding: 24px; background: #fafafa; border-radius: 6px;",

                    Timeline {
                        mode: TimelineMode::Left,
                        TimelineItem {
                            "Create a services site 2015-09-01"
                        }
                        TimelineItem {
                            "Solve initial network problems 2015-09-01"
                        }
                        TimelineItem {
                            dot: "clock-circle-outlined".to_string(),
                            color: TimelineItemColor::Red,
                            "Technical testing 2015-09-01"
                        }
                        TimelineItem {
                            "Network problems being solved 2015-09-01"
                        }
                    }
                }
            }

            // 交替展现
            DemoSection {
                title: "交替展现",
                description: "内容在时间轴两侧轮流出现。",

                div {
                    style: "padding: 24px; background: #fafafa; border-radius: 6px;",

                    Timeline {
                        mode: TimelineMode::Alternate,
                        TimelineItem {
                            "Create a services site 2015-09-01"
                        }
                        TimelineItem {
                            color: TimelineItemColor::Green,
                            "Solve initial network problems 2015-09-01"
                        }
                        TimelineItem {
                            dot: "clock-circle-outlined".to_string(),
                            "Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo."
                        }
                        TimelineItem {
                            color: TimelineItemColor::Red,
                            "Network problems being solved 2015-09-01"
                        }
                        TimelineItem {
                            "Create a services site 2015-09-01"
                        }
                        TimelineItem {
                            dot: "clock-circle-outlined".to_string(),
                            "Technical testing 2015-09-01"
                        }
                    }
                }
            }

            // 自定义时间轴点 - 标签
            DemoSection {
                title: "标签",
                description: "使用 label 标签单独展示时间。",

                div {
                    style: "padding: 24px; background: #fafafa; border-radius: 6px;",

                    Timeline {
                        mode: TimelineMode::Left,
                        TimelineItem {
                            label: "2015-09-01".to_string(),
                            "Create a services"
                        }
                        TimelineItem {
                            label: "2015-09-01 09:12:11".to_string(),
                            "Solve initial network problems"
                        }
                        TimelineItem {
                            "Technical testing"
                        }
                        TimelineItem {
                            label: "2015-09-01 09:12:11".to_string(),
                            "Network problems being solved"
                        }
                    }
                }
            }

            // 同样支持右侧标签
            DemoSection {
                title: "右侧标签",
                description: "标签也可以在右侧展示。",

                div {
                    style: "padding: 24px; background: #fafafa; border-radius: 6px;",

                    div {
                        style: "margin-bottom: 16px;",
                        RadioGroup {
                            value: mode(),
                            on_change: move |e| {
                                mode.set(e);
                            },
                            Radio {
                                value: "left",
                                "Left"
                            }
                            Radio {
                                value: "right",
                                "Right"
                            }
                            Radio {
                                value: "alternate",
                                "Alternate"
                            }
                        }
                    }

                    Timeline {
                        mode: match mode().as_str() {
                            "left" => TimelineMode::Left,
                            "right" => TimelineMode::Right,
                            "alternate" => TimelineMode::Alternate,
                            _ => TimelineMode::Left,
                        },
                        TimelineItem {
                            label:  "2015-09-01".to_string(),
                            "Create a services"
                        }
                        TimelineItem {
                            label:  "2015-09-01 09:12:11".to_string(),
                            "Solve initial network problems"
                        }
                        TimelineItem {
                            "Technical testing"
                        }
                        TimelineItem {
                            label: "2015-09-01 09:12:11".to_string(),
                            "Network problems being solved"
                        }
                    }
                }
            }

            // API 文档
            ApiDocumentation {
                component_name: "Timeline",
                props: vec![
                    PropDoc {
                        name: "mode".to_string(),
                        prop_type: "String".to_string(),
                        default: "-".to_string(),
                        description: "通过设置 mode 可以改变时间轴和内容的相对位置".to_string(),
                    },
                    PropDoc {
                        name: "pending".to_string(),
                        prop_type: "bool | Element".to_string(),
                        default: "false".to_string(),
                        description: "指定最后一个幽灵节点是否存在或内容".to_string(),
                    },
                    PropDoc {
                        name: "pending_dot".to_string(),
                        prop_type: "Element".to_string(),
                        default: "<LoadingOutlined />".to_string(),
                        description: "当最后一个幽灵节点存在時，指定其时间图点".to_string(),
                    },
                    PropDoc {
                        name: "reverse".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "节点排序".to_string(),
                    },
                    PropDoc {
                        name: "items".to_string(),
                        prop_type: "Vec<TimelineItem>".to_string(),
                        default: "-".to_string(),
                        description: "选项配置".to_string(),
                    },
                ]
            }

            // Timeline.Item API
            ApiDocumentation {
                component_name: "Timeline.Item",
                props: vec![
                    PropDoc {
                        name: "color".to_string(),
                        prop_type: "String".to_string(),
                        default: "blue".to_string(),
                        description: "指定圆圈颜色 blue、red、green、gray，或自定义的色值".to_string(),
                    },
                    PropDoc {
                        name: "dot".to_string(),
                        prop_type: "Element".to_string(),
                        default: "-".to_string(),
                        description: "自定义时间轴点".to_string(),
                    },
                    PropDoc {
                        name: "label".to_string(),
                        prop_type: "Element".to_string(),
                        default: "-".to_string(),
                        description: "设置标签".to_string(),
                    },
                    PropDoc {
                        name: "position".to_string(),
                        prop_type: "String".to_string(),
                        default: "-".to_string(),
                        description: "自定义节点位置".to_string(),
                    },
                ]
            }
        }
    }
}

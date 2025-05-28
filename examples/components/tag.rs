//! # Tag 组件演示
//!
//! 展示 Tag 组件的各种用法和样式

use crate::common::*;
use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

/// Tag 组件演示
#[component]
pub fn TagDemo() -> Element {
    let mut visible_tags = use_signal(|| vec![true, true, true, true, true]);

    rsx! {
        div {
            class: "component-showcase",

            h1 {
                style: "margin: 0 0 16px 0; color: #262626; font-size: 28px;",
                "Tag 标签"
            }

            p {
                style: "margin: 0 0 24px 0; color: #666; font-size: 14px;",
                "进行标记和分类的小标签。"
            }

            // 基础用法
            DemoSection {
                title: "基础用法",
                description: "基础标签的用法，可以通过添加 closable 变为可关闭标签。",

                div {
                    style: "display: flex; gap: 8px; flex-wrap: wrap; align-items: center;",

                    Tag { "Tag 1" }
                    Tag {
                        closable: true,
                        onclose: move |_| {
                            web_sys::console::log_1(&"Tag closed!".into());
                        },
                        "Tag 2"
                    }
                    Tag {
                        closable: true,
                        "Closable Tag"
                    }
                }
            }

            // 多彩标签
            DemoSection {
                title: "多彩标签",
                description: "我们添加了多种预设色彩的标签样式，用作不同场景使用。",

                div {
                    style: "display: flex; gap: 8px; flex-wrap: wrap; align-items: center;",

                    Tag {
                        color: TagColor::Primary,
                        "Primary"
                    }
                    Tag {
                        color: TagColor::Success,
                        "Success"
                    }
                    Tag {
                        color: TagColor::Warning,
                        "Warning"
                    }
                    Tag {
                        color: TagColor::Error,
                        "Error"
                    }
                    Tag {
                        color: TagColor::Info,
                        "Info"
                    }
                }
            }

            // 自定义颜色
            DemoSection {
                title: "自定义颜色",
                description: "可以通过 color 属性自定义标签颜色。",

                div {
                    style: "display: flex; gap: 8px; flex-wrap: wrap; align-items: center;",

                    Tag {
                        color: TagColor::Custom("#f50".to_string()),
                        "#f50"
                    }
                    Tag {
                        color: TagColor::Custom("#2db7f5".to_string()),
                        "#2db7f5"
                    }
                    Tag {
                        color: TagColor::Custom("#87d068".to_string()),
                        "#87d068"
                    }
                    Tag {
                        color: TagColor::Custom("#108ee9".to_string()),
                        "#108ee9"
                    }
                }
            }

            // 不同尺寸
            DemoSection {
                title: "三种尺寸",
                description: "标签有大、中、小三种尺寸。",

                div {
                    style: "display: flex; gap: 8px; align-items: center;",

                    Tag {
                        size: TagSize::Small,
                        color: TagColor::Primary,
                        "Small"
                    }
                    Tag {
                        size: TagSize::Default,
                        color: TagColor::Primary,
                        "Default"
                    }
                    Tag {
                        size: TagSize::Large,
                        color: TagColor::Primary,
                        "Large"
                    }
                }
            }

            // 无边框
            DemoSection {
                title: "无边框",
                description: "通过 bordered 属性控制是否显示边框。",

                div {
                    style: "display: flex; gap: 8px; flex-wrap: wrap; align-items: center;",

                    Tag {
                        color: TagColor::Primary,
                        bordered: false,
                        "Primary"
                    }
                    Tag {
                        color: TagColor::Success,
                        bordered: false,
                        "Success"
                    }
                    Tag {
                        color: TagColor::Warning,
                        bordered: false,
                        "Warning"
                    }
                    Tag {
                        color: TagColor::Error,
                        bordered: false,
                        "Error"
                    }
                }
            }

            // 动态添加和删除
            DemoSection {
                title: "动态添加和删除",
                description: "用数组生成一组标签，可以动态添加和删除。",

                div {
                    style: "display: flex; gap: 8px; flex-wrap: wrap; align-items: center;",

                    for (index, visible) in visible_tags.read().iter().enumerate() {
                        if *visible {
                            Tag {
                                key: "{index}",
                                closable: true,
                                color: match index {
                                    0 => TagColor::Primary,
                                    1 => TagColor::Success,
                                    2 => TagColor::Warning,
                                    3 => TagColor::Error,
                                    4 => TagColor::Info,
                                    _ => TagColor::Default,
                                },
                                onclose: move |_| {
                                    visible_tags.with_mut(|tags| {
                                        if index < tags.len() {
                                            tags[index] = false;
                                        }
                                    });
                                },
                                "Tag {index + 1}"
                            }
                        }
                    }

                    Button {
                        button_type: ButtonType::Dashed,
                        size: ButtonSize::Small,
                        onclick: move |_| {
                            visible_tags.with_mut(|tags| {
                                for tag in tags.iter_mut() {
                                    *tag = true;
                                }
                            });
                        },
                        "+ Add Tag"
                    }
                }
            }

            // API 文档
            ApiDocumentation {
                component_name: "Tag",
                props: vec![
                    PropDoc {
                        name: "color".to_string(),
                        prop_type: "TagColor".to_string(),
                        default: "TagColor::Default".to_string(),
                        description: "标签颜色".to_string(),
                    },
                    PropDoc {
                        name: "size".to_string(),
                        prop_type: "TagSize".to_string(),
                        default: "TagSize::Default".to_string(),
                        description: "标签尺寸".to_string(),
                    },
                    PropDoc {
                        name: "closable".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "是否可关闭".to_string(),
                    },
                    PropDoc {
                        name: "bordered".to_string(),
                        prop_type: "bool".to_string(),
                        default: "true".to_string(),
                        description: "是否显示边框".to_string(),
                    },
                    PropDoc {
                        name: "class".to_string(),
                        prop_type: "Option<String>".to_string(),
                        default: "None".to_string(),
                        description: "自定义类名".to_string(),
                    },
                    PropDoc {
                        name: "style".to_string(),
                        prop_type: "Option<String>".to_string(),
                        default: "None".to_string(),
                        description: "自定义样式".to_string(),
                    },
                    PropDoc {
                        name: "onclick".to_string(),
                        prop_type: "Option<EventHandler<MouseEvent>>".to_string(),
                        default: "None".to_string(),
                        description: "点击事件".to_string(),
                    },
                    PropDoc {
                        name: "onclose".to_string(),
                        prop_type: "Option<EventHandler<MouseEvent>>".to_string(),
                        default: "None".to_string(),
                        description: "关闭事件".to_string(),
                    },
                ]
            }
        }
    }
}

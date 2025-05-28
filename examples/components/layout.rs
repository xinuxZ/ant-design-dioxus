//! Layout 组件演示
//!
//! 展示 Layout、Header、Sider、Content、Footer 组件的各种用法

use crate::common::*;
use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

/// Layout 组件演示
#[component]
pub fn LayoutDemo() -> Element {
    rsx! {
        div {
            class: "component-showcase",

            h1 {
                style: "margin: 0 0 16px 0; color: #262626; font-size: 28px;",
                "Layout 布局"
            }

            p {
                style: "margin: 0 0 24px 0; color: #666; font-size: 14px;",
                "协助进行页面级整体布局。"
            }

            // 基础布局
            DemoSection {
                title: "基础布局",
                description: "最基本的『上-中-下』布局。",

                div {
                    style: "height: 300px; border: 1px solid #d9d9d9;",
                    Layout {
                        Header {
                            style: "background: #7dbcea; color: white; text-align: center; line-height: 64px;",
                            "Header"
                        }
                        Content {
                            style: "background: #108ee9; color: white; text-align: center; line-height: 120px;",
                            "Content"
                        }
                        Footer {
                            style: "background: #7dbcea; color: white; text-align: center; line-height: 64px;",
                            "Footer"
                        }
                    }
                }
            }

            // 上中下布局
            DemoSection {
                title: "上中下布局",
                description: "最常见的网站布局。",

                div {
                    style: "height: 300px; border: 1px solid #d9d9d9;",
                    Layout {
                        Header {
                            style: "background: #108ee9; color: white; text-align: center; line-height: 64px;",
                            "Header"
                        }
                        Layout {
                            style: "padding: 24px 0;",
                            Content {
                                style: "background: #fff; margin: 0 16px; padding: 24px; text-align: center; border: 1px solid #d9d9d9;",
                                "Content"
                            }
                        }
                        Footer {
                            style: "background: #108ee9; color: white; text-align: center; line-height: 64px;",
                            "Footer"
                        }
                    }
                }
            }

            // 侧边栏布局
            DemoSection {
                title: "侧边栏布局",
                description: "拥有侧边栏的布局。",

                div {
                    style: "height: 400px; border: 1px solid #d9d9d9;",
                    Layout {
                        has_sider: true,
                        Sider {
                            width: 200,
                            style: "background: #3ba0e9; color: white; text-align: center; line-height: 120px;",
                            "Sider"
                        }
                        Layout {
                            Header {
                                style: "background: #7dbcea; color: white; text-align: center; line-height: 64px;",
                                "Header"
                            }
                            Content {
                                style: "background: #108ee9; color: white; text-align: center; line-height: 120px; margin: 24px 16px 0;",
                                "Content"
                            }
                            Footer {
                                style: "background: #7dbcea; color: white; text-align: center; line-height: 64px;",
                                "Footer"
                            }
                        }
                    }
                }
            }

            // 可折叠侧边栏
            CollapsibleSiderDemo {}

            // 右侧边栏
            DemoSection {
                title: "右侧边栏",
                description: "侧边栏在右边的布局。",

                div {
                    style: "height: 400px; border: 1px solid #d9d9d9;",
                    Layout {
                        has_sider: true,
                        Layout {
                            Header {
                                style: "background: #7dbcea; color: white; text-align: center; line-height: 64px;",
                                "Header"
                            }
                            Content {
                                style: "background: #108ee9; color: white; text-align: center; line-height: 120px; margin: 24px 16px 0;",
                                "Content"
                            }
                            Footer {
                                style: "background: #7dbcea; color: white; text-align: center; line-height: 64px;",
                                "Footer"
                            }
                        }
                        Sider {
                            width: 200,
                            style: "background: #3ba0e9; color: white; text-align: center; line-height: 120px;",
                            "Sider"
                        }
                    }
                }
            }

            // API 文档
            ApiDocumentation {
                component_name: "Layout",
                props: vec![
                    PropDoc {
                        name: "has_sider".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "表示子元素里有 Sider，一般不用指定。可用于服务端渲染时避免样式闪动".to_string(),
                    },
                    PropDoc {
                        name: "width".to_string(),
                        prop_type: "u32".to_string(),
                        default: "200".to_string(),
                        description: "宽度 (仅 Sider 组件)".to_string(),
                    },
                    PropDoc {
                        name: "collapsed".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "当前收起状态 (仅 Sider 组件)".to_string(),
                    },
                    PropDoc {
                        name: "collapsible".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "是否可收起 (仅 Sider 组件)".to_string(),
                    },
                    PropDoc {
                        name: "collapsed_width".to_string(),
                        prop_type: "u32".to_string(),
                        default: "80".to_string(),
                        description: "收缩宽度，设置为 0 会出现特殊 trigger (仅 Sider 组件)".to_string(),
                    },
                ]
            }
        }
    }
}

/// 可折叠侧边栏演示
#[component]
fn CollapsibleSiderDemo() -> Element {
    let mut collapsed = use_signal(|| false);

    rsx! {
        DemoSection {
            title: "可折叠侧边栏",
            description: "侧边栏支持折叠功能。",

            div {
                style: "margin-bottom: 16px;",
                Button {
                    button_type: ButtonType::Primary,
                    onclick: move |_| {
                        collapsed.set(if *collapsed.read() { false } else { true });
                    },
                    if *collapsed.read() { "展开" } else { "折叠" }
                }
            }

            div {
                style: "height: 400px; border: 1px solid #d9d9d9;",
                Layout {
                    has_sider: true,
                    Sider {
                        width: if *collapsed.read() { 80 } else { 200 },
                        collapsed: *collapsed.read(),
                        collapsible: true,
                        style: "background: #3ba0e9; color: white; text-align: center; line-height: 120px;",
                        if *collapsed.read() { "S" } else { "Sider" }
                    }
                    Layout {
                        Header {
                            style: "background: #7dbcea; color: white; text-align: center; line-height: 64px;",
                            "Header"
                        }
                        Content {
                            style: "background: #108ee9; color: white; text-align: center; line-height: 120px; margin: 24px 16px 0;",
                            "Content"
                        }
                        Footer {
                            style: "background: #7dbcea; color: white; text-align: center; line-height: 64px;",
                            "Footer"
                        }
                    }
                }
            }
        }
    }
}

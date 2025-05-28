//! Layout 组件示例
//!
//! 展示 Layout 布局组件的各种用法

use crate::common::{ApiDocumentation, DemoSection, PropDoc};
use dioxus::prelude::*;

/// Layout 组件演示页面
#[component]
pub fn LayoutDemo() -> Element {
    rsx! {
        div {
            class: "layout-demo",
            style: "padding: 24px;",

            h1 {
                style: "margin: 0 0 24px 0; color: #262626; font-size: 28px;",
                "Layout 布局"
            }

            p {
                style: "margin: 0 0 32px 0; color: #666; font-size: 14px;",
                "协助进行页面级整体布局。"
            }

            // 基础布局
            DemoSection {
                title: "基础布局".to_string(),
                description: "典型的页面布局。".to_string(),
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    // Layout 1: Header + Content + Footer
                    div {
                        class: "layout-demo-1",
                        style: "border: 1px solid #d9d9d9;",

                        div {
                            class: "header",
                            style: "background: #7dbcea; color: white; text-align: center; padding: 24px 0;",
                            "Header"
                        }
                        div {
                            class: "content",
                            style: "background: #108ee9; color: white; text-align: center; padding: 24px 0; min-height: 120px;",
                            "Content"
                        }
                        div {
                            class: "footer",
                            style: "background: #7dbcea; color: white; text-align: center; padding: 24px 0;",
                            "Footer"
                        }
                    }

                    // Layout 2: Header + Sider + Content + Footer
                    div {
                        class: "layout-demo-2",
                        style: "border: 1px solid #d9d9d9;",

                        div {
                            class: "header",
                            style: "background: #7dbcea; color: white; text-align: center; padding: 24px 0;",
                            "Header"
                        }
                        div {
                            class: "layout-body",
                            style: "display: flex;",
                            div {
                                class: "sider",
                                style: "background: #3ba0e9; color: white; text-align: center; padding: 24px 0; width: 200px;",
                                "Sider"
                            }
                            div {
                                class: "content",
                                style: "background: #108ee9; color: white; text-align: center; padding: 24px 0; flex: 1; min-height: 120px;",
                                "Content"
                            }
                        }
                        div {
                            class: "footer",
                            style: "background: #7dbcea; color: white; text-align: center; padding: 24px 0;",
                            "Footer"
                        }
                    }

                    // Layout 3: Sider + Header + Content + Footer
                    div {
                        class: "layout-demo-3",
                        style: "border: 1px solid #d9d9d9; display: flex;",

                        div {
                            class: "sider",
                            style: "background: #3ba0e9; color: white; text-align: center; padding: 24px 0; width: 200px;",
                            "Sider"
                        }
                        div {
                            class: "layout-main",
                            style: "flex: 1;",
                            div {
                                class: "header",
                                style: "background: #7dbcea; color: white; text-align: center; padding: 24px 0;",
                                "Header"
                            }
                            div {
                                class: "content",
                                style: "background: #108ee9; color: white; text-align: center; padding: 24px 0; min-height: 120px;",
                                "Content"
                            }
                            div {
                                class: "footer",
                                style: "background: #7dbcea; color: white; text-align: center; padding: 24px 0;",
                                "Footer"
                            }
                        }
                    }
                }
            }

            // 侧边布局
            DemoSection {
                title: "侧边布局".to_string(),
                description: "侧边两列式布局。页面横向空间有限时，侧边导航可收起。".to_string(),
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    // 左侧导航布局
                    div {
                        class: "sider-layout",
                        style: "border: 1px solid #d9d9d9; display: flex; min-height: 200px;",

                        div {
                            class: "sider",
                            style: "background: #404040; color: white; width: 200px; display: flex; flex-direction: column;",
                            div {
                                class: "logo",
                                style: "background: #002140; text-align: center; padding: 16px 0; font-weight: bold;",
                                "LOGO"
                            }
                            div {
                                class: "menu",
                                style: "flex: 1; padding: 16px;",
                                div { style: "margin-bottom: 8px; padding: 8px; background: rgba(255,255,255,0.1); border-radius: 4px;", "Navigation One" }
                                div { style: "margin-bottom: 8px; padding: 8px; background: rgba(255,255,255,0.1); border-radius: 4px;", "Navigation Two" }
                                div { style: "margin-bottom: 8px; padding: 8px; background: rgba(255,255,255,0.1); border-radius: 4px;", "Navigation Three" }
                            }
                        }
                        div {
                            class: "layout-main",
                            style: "flex: 1; display: flex; flex-direction: column;",
                            div {
                                class: "header",
                                style: "background: white; padding: 0 24px; border-bottom: 1px solid #f0f0f0; display: flex; align-items: center; height: 64px;",
                                "Header"
                            }
                            div {
                                class: "content",
                                style: "background: #f0f2f5; padding: 24px; flex: 1;",
                                div {
                                    style: "background: white; padding: 24px; border-radius: 6px; min-height: 120px;",
                                    "Content"
                                }
                            }
                        }
                    }
                }
            }

            // 顶部-侧边布局
            DemoSection {
                title: "顶部-侧边布局".to_string(),
                description: "拥有顶部导航及侧边栏的页面，多用于展示类网站。".to_string(),
                div {
                    class: "top-sider-layout",
                    style: "border: 1px solid #d9d9d9;",

                    div {
                        class: "header",
                        style: "background: #001529; color: white; padding: 0 24px; display: flex; align-items: center; height: 64px;",
                        div {
                            class: "logo",
                            style: "color: white; font-weight: bold; margin-right: 24px;",
                            "Ant Design"
                        }
                        div {
                            class: "nav",
                            style: "display: flex; gap: 24px;",
                            span { style: "color: rgba(255,255,255,0.65); cursor: pointer;", "nav 1" }
                            span { style: "color: rgba(255,255,255,0.65); cursor: pointer;", "nav 2" }
                            span { style: "color: rgba(255,255,255,0.65); cursor: pointer;", "nav 3" }
                        }
                    }
                    div {
                        class: "layout-body",
                        style: "display: flex;",
                        div {
                            class: "sider",
                            style: "background: #f0f0f0; width: 200px; padding: 16px;",
                            div { style: "margin-bottom: 8px; padding: 8px; background: white; border-radius: 4px; cursor: pointer;", "option1" }
                            div { style: "margin-bottom: 8px; padding: 8px; background: white; border-radius: 4px; cursor: pointer;", "option2" }
                            div { style: "margin-bottom: 8px; padding: 8px; background: white; border-radius: 4px; cursor: pointer;", "option3" }
                            div { style: "margin-bottom: 8px; padding: 8px; background: white; border-radius: 4px; cursor: pointer;", "option4" }
                        }
                        div {
                            class: "content",
                            style: "background: #f0f2f5; padding: 24px; flex: 1;",
                            div {
                                style: "background: white; padding: 24px; border-radius: 6px; min-height: 200px;",
                                "Content"
                            }
                        }
                    }
                }
            }

            // 响应式布局
            DemoSection {
                title: "响应式布局".to_string(),
                description: "Layout.Sider 支持响应式布局。".to_string(),
                div {
                    class: "responsive-layout",
                    style: "border: 1px solid #d9d9d9; display: flex; min-height: 200px;",

                    div {
                        class: "sider-responsive",
                        style: "background: #404040; color: white; width: 200px; transition: width 0.2s;",
                        div {
                            class: "trigger",
                            style: "background: #002140; text-align: center; padding: 16px 0; cursor: pointer;",
                            "≡"
                        }
                        div {
                            class: "menu",
                            style: "padding: 16px;",
                            div { style: "margin-bottom: 8px; padding: 8px; background: rgba(255,255,255,0.1); border-radius: 4px;", "nav 1" }
                            div { style: "margin-bottom: 8px; padding: 8px; background: rgba(255,255,255,0.1); border-radius: 4px;", "nav 2" }
                            div { style: "margin-bottom: 8px; padding: 8px; background: rgba(255,255,255,0.1); border-radius: 4px;", "nav 3" }
                        }
                    }
                    div {
                        class: "layout-main",
                        style: "flex: 1; display: flex; flex-direction: column;",
                        div {
                            class: "header",
                            style: "background: white; padding: 0 24px; border-bottom: 1px solid #f0f0f0; display: flex; align-items: center; height: 64px;",
                            "Header"
                        }
                        div {
                            class: "content",
                            style: "background: #f0f2f5; padding: 24px; flex: 1;",
                            div {
                                style: "background: white; padding: 24px; border-radius: 6px; min-height: 120px;",
                                "Content"
                            }
                        }
                    }
                }
            }

            // API 文档
            ApiDocumentation {
                component_name: "Layout".to_string(),
                props: vec![
                    PropDoc {
                        name: "class".to_string(),
                        prop_type: "String".to_string(),
                        default: "".to_string(),
                        description: "容器 class".to_string(),
                    },
                    PropDoc {
                        name: "style".to_string(),
                        prop_type: "String".to_string(),
                        default: "".to_string(),
                        description: "指定样式".to_string(),
                    },
                ]
            }

            ApiDocumentation {
                component_name: "Layout.Header".to_string(),
                props: vec![
                    PropDoc {
                        name: "class".to_string(),
                        prop_type: "String".to_string(),
                        default: "".to_string(),
                        description: "容器 class".to_string(),
                    },
                    PropDoc {
                        name: "style".to_string(),
                        prop_type: "String".to_string(),
                        default: "".to_string(),
                        description: "指定样式".to_string(),
                    },
                ]
            }

            ApiDocumentation {
                component_name: "Layout.Sider".to_string(),
                props: vec![
                    PropDoc {
                        name: "collapsed".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "当前收起状态".to_string(),
                    },
                    PropDoc {
                        name: "collapsible".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "是否可收起".to_string(),
                    },
                    PropDoc {
                        name: "width".to_string(),
                        prop_type: "u32".to_string(),
                        default: "200".to_string(),
                        description: "宽度".to_string(),
                    },
                    PropDoc {
                        name: "collapsed_width".to_string(),
                        prop_type: "u32".to_string(),
                        default: "80".to_string(),
                        description: "收缩宽度，设置为 0 会出现特殊 trigger".to_string(),
                    },
                ]
            }

            ApiDocumentation {
                component_name: "Layout.Content".to_string(),
                props: vec![
                    PropDoc {
                        name: "class".to_string(),
                        prop_type: "String".to_string(),
                        default: "".to_string(),
                        description: "容器 class".to_string(),
                    },
                    PropDoc {
                        name: "style".to_string(),
                        prop_type: "String".to_string(),
                        default: "".to_string(),
                        description: "指定样式".to_string(),
                    },
                ]
            }

            ApiDocumentation {
                component_name: "Layout.Footer".to_string(),
                props: vec![
                    PropDoc {
                        name: "class".to_string(),
                        prop_type: "String".to_string(),
                        default: "".to_string(),
                        description: "容器 class".to_string(),
                    },
                    PropDoc {
                        name: "style".to_string(),
                        prop_type: "String".to_string(),
                        default: "".to_string(),
                        description: "指定样式".to_string(),
                    },
                ]
            }
        }
    }
}

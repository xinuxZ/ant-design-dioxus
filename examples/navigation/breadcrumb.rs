//! Breadcrumb 组件示例
//!
//! 展示 Breadcrumb 面包屑导航组件的各种用法

use crate::common::{ApiDocumentation, DemoSection, PropDoc};
use dioxus::prelude::*;

/// Breadcrumb 组件演示页面
#[component]
pub fn BreadcrumbDemo() -> Element {
    rsx! {
        div {
            class: "breadcrumb-demo",
            style: "padding: 24px;",

            h1 {
                style: "margin: 0 0 24px 0; color: #262626; font-size: 28px;",
                "Breadcrumb 面包屑"
            }

            p {
                style: "margin: 0 0 32px 0; color: #666; font-size: 14px;",
                "显示当前页面在系统层级结构中的位置，并能向上返回。"
            }

            // 基础用法
            DemoSection {
                title: "基础用法".to_string(),
                description: "最简单的用法。".to_string(),
                nav {
                    style: "display: flex; align-items: center; font-size: 14px;",

                    a {
                        href: "#",
                        style: "color: #666; text-decoration: none;",
                        "Home"
                    }
                    span {
                        style: "margin: 0 8px; color: #d9d9d9;",
                        "/"
                    }
                    a {
                        href: "#",
                        style: "color: #666; text-decoration: none;",
                        "Application Center"
                    }
                    span {
                        style: "margin: 0 8px; color: #d9d9d9;",
                        "/"
                    }
                    span {
                        style: "color: #666;",
                        "Application List"
                    }
                }
            }

            // 带图标的
            DemoSection {
                title: "带图标的".to_string(),
                description: "图标放在文字前面。".to_string(),
                nav {
                    style: "display: flex; align-items: center; font-size: 14px;",

                    a {
                        href: "#",
                        style: "color: #666; text-decoration: none; display: flex; align-items: center; gap: 4px;",
                        span { "🏠" }
                        "Home"
                    }
                    span {
                        style: "margin: 0 8px; color: #d9d9d9;",
                        "/"
                    }
                    a {
                        href: "#",
                        style: "color: #666; text-decoration: none; display: flex; align-items: center; gap: 4px;",
                        span { "👤" }
                        "Application Center"
                    }
                    span {
                        style: "margin: 0 8px; color: #d9d9d9;",
                        "/"
                    }
                    span {
                        style: "color: #666; display: flex; align-items: center; gap: 4px;",
                        span { "📋" }
                        "Application List"
                    }
                }
            }

            // 自定义分隔符
            DemoSection {
                title: "自定义分隔符".to_string(),
                description: "使用 separator=\">\" 可以自定义分隔符。".to_string(),
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    nav {
                        style: "display: flex; align-items: center; font-size: 14px;",

                        a {
                            href: "#",
                            style: "color: #666; text-decoration: none;",
                            "Home"
                        }
                        span {
                            style: "margin: 0 8px; color: #d9d9d9;",
                            ">"
                        }
                        a {
                            href: "#",
                            style: "color: #666; text-decoration: none;",
                            "Application Center"
                        }
                        span {
                            style: "margin: 0 8px; color: #d9d9d9;",
                            ">"
                        }
                        span {
                            style: "color: #666;",
                            "Application List"
                        }
                    }

                    nav {
                        style: "display: flex; align-items: center; font-size: 14px;",

                        a {
                            href: "#",
                            style: "color: #666; text-decoration: none;",
                            "Home"
                        }
                        span {
                            style: "margin: 0 8px; color: #d9d9d9;",
                            "→"
                        }
                        a {
                            href: "#",
                            style: "color: #666; text-decoration: none;",
                            "Application Center"
                        }
                        span {
                            style: "margin: 0 8px; color: #d9d9d9;",
                            "→"
                        }
                        span {
                            style: "color: #666;",
                            "Application List"
                        }
                    }

                    nav {
                        style: "display: flex; align-items: center; font-size: 14px;",

                        a {
                            href: "#",
                            style: "color: #666; text-decoration: none;",
                            "Home"
                        }
                        span {
                            style: "margin: 0 8px; color: #d9d9d9;",
                            "•"
                        }
                        a {
                            href: "#",
                            style: "color: #666; text-decoration: none;",
                            "Application Center"
                        }
                        span {
                            style: "margin: 0 8px; color: #d9d9d9;",
                            "•"
                        }
                        span {
                            style: "color: #666;",
                            "Application List"
                        }
                    }
                }
            }

            // 带下拉菜单的面包屑
            DemoSection {
                title: "带下拉菜单的面包屑".to_string(),
                description: "面包屑支持下拉菜单。".to_string(),
                nav {
                    style: "display: flex; align-items: center; font-size: 14px;",

                    a {
                        href: "#",
                        style: "color: #666; text-decoration: none;",
                        "Home"
                    }
                    span {
                        style: "margin: 0 8px; color: #d9d9d9;",
                        "/"
                    }
                    div {
                        style: "position: relative; display: inline-block;",

                        button {
                            style: "background: none; border: none; color: #666; cursor: pointer; display: flex; align-items: center; gap: 4px;",
                            "Application Center"
                            span { style: "font-size: 12px;", "▼" }
                        }

                        // 下拉菜单（这里只是示例，实际需要状态控制）
                        div {
                            style: "position: absolute; top: 100%; left: 0; background: white; border: 1px solid #f0f0f0; border-radius: 6px; box-shadow: 0 2px 8px rgba(0,0,0,0.1); z-index: 1000; min-width: 120px; display: none;",

                            a {
                                href: "#",
                                style: "display: block; padding: 8px 12px; color: #666; text-decoration: none;",
                                "General"
                            }
                            a {
                                href: "#",
                                style: "display: block; padding: 8px 12px; color: #666; text-decoration: none;",
                                "Layout"
                            }
                            a {
                                href: "#",
                                style: "display: block; padding: 8px 12px; color: #666; text-decoration: none;",
                                "Navigation"
                            }
                        }
                    }
                    span {
                        style: "margin: 0 8px; color: #d9d9d9;",
                        "/"
                    }
                    span {
                        style: "color: #666;",
                        "Application List"
                    }
                }
            }

            // 其它组件配合使用
            DemoSection {
                title: "其它组件配合使用".to_string(),
                description: "和 PageHeader 组件一起使用，来完成页面的路径导航功能。".to_string(),
                div {
                    style: "border: 1px solid #f0f0f0; border-radius: 6px; padding: 16px; background: #fafafa;",

                    // 页面头部
                    div {
                        style: "margin-bottom: 16px;",

                        nav {
                            style: "display: flex; align-items: center; font-size: 14px; margin-bottom: 8px;",

                            a {
                                href: "#",
                                style: "color: #666; text-decoration: none;",
                                "Home"
                            }
                            span {
                                style: "margin: 0 8px; color: #d9d9d9;",
                                "/"
                            }
                            a {
                                href: "#",
                                style: "color: #666; text-decoration: none;",
                                "List"
                            }
                            span {
                                style: "margin: 0 8px; color: #d9d9d9;",
                                "/"
                            }
                            span {
                                style: "color: #666;",
                                "App"
                            }
                        }

                        h2 {
                            style: "margin: 0; color: #262626; font-size: 20px;",
                            "Title"
                        }
                    }

                    // 页面内容
                    div {
                        style: "background: white; padding: 24px; border-radius: 6px;",
                        p {
                            style: "margin: 0; color: #666;",
                            "Content"
                        }
                    }
                }
            }

            // 配置分隔符
            DemoSection {
                title: "配置分隔符".to_string(),
                description: "使用 Breadcrumb.Separator 可以自定义分隔符。".to_string(),
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    nav {
                        style: "display: flex; align-items: center; font-size: 14px;",

                        a {
                            href: "#",
                            style: "color: #666; text-decoration: none;",
                            "Location"
                        }
                        span {
                            style: "margin: 0 8px; color: #1890ff; font-weight: bold;",
                            ":"
                        }
                        a {
                            href: "#",
                            style: "color: #666; text-decoration: none;",
                            "Application Center"
                        }
                        span {
                            style: "margin: 0 8px; color: #1890ff; font-weight: bold;",
                            ":"
                        }
                        span {
                            style: "color: #666;",
                            "Application List"
                        }
                    }

                    nav {
                        style: "display: flex; align-items: center; font-size: 14px;",

                        a {
                            href: "#",
                            style: "color: #666; text-decoration: none;",
                            "Location"
                        }
                        span {
                            style: "margin: 0 8px; color: #52c41a; font-size: 16px;",
                            "★"
                        }
                        a {
                            href: "#",
                            style: "color: #666; text-decoration: none;",
                            "Application Center"
                        }
                        span {
                            style: "margin: 0 8px; color: #52c41a; font-size: 16px;",
                            "★"
                        }
                        span {
                            style: "color: #666;",
                            "Application List"
                        }
                    }
                }
            }

            // API 文档
            ApiDocumentation {
                component_name: "Breadcrumb".to_string(),
                props: vec![
                    PropDoc {
                        name: "item_render".to_string(),
                        prop_type: "Function".to_string(),
                        default: "-".to_string(),
                        description: "自定义链接函数，和 react-router 配置使用".to_string(),
                    },
                    PropDoc {
                        name: "params".to_string(),
                        prop_type: "Object".to_string(),
                        default: "-".to_string(),
                        description: "路由的参数".to_string(),
                    },
                    PropDoc {
                        name: "routes".to_string(),
                        prop_type: "Vec<Route>".to_string(),
                        default: "-".to_string(),
                        description: "router 的路由栈信息".to_string(),
                    },
                    PropDoc {
                        name: "separator".to_string(),
                        prop_type: "String | Element".to_string(),
                        default: "/".to_string(),
                        description: "分隔符自定义".to_string(),
                    },
                ]
            }

            ApiDocumentation {
                component_name: "Breadcrumb.Item".to_string(),
                props: vec![
                    PropDoc {
                        name: "class_name".to_string(),
                        prop_type: "String".to_string(),
                        default: "-".to_string(),
                        description: "自定义类名".to_string(),
                    },
                    PropDoc {
                        name: "drop_down_props".to_string(),
                        prop_type: "Object".to_string(),
                        default: "-".to_string(),
                        description: "弹出下拉菜单的自定义配置".to_string(),
                    },
                    PropDoc {
                        name: "href".to_string(),
                        prop_type: "String".to_string(),
                        default: "-".to_string(),
                        description: "链接的目的地".to_string(),
                    },
                    PropDoc {
                        name: "menu".to_string(),
                        prop_type: "Object".to_string(),
                        default: "-".to_string(),
                        description: "菜单配置项".to_string(),
                    },
                    PropDoc {
                        name: "onclick".to_string(),
                        prop_type: "Function".to_string(),
                        default: "-".to_string(),
                        description: "单击事件".to_string(),
                    },
                ]
            }

            ApiDocumentation {
                component_name: "Breadcrumb.Separator".to_string(),
                props: vec![
                    PropDoc {
                        name: "children".to_string(),
                        prop_type: "Element".to_string(),
                        default: "/".to_string(),
                        description: "要显示的分隔符".to_string(),
                    },
                ]
            }
        }
    }
}

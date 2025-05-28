//!
//!
//! 展示 Breadcrumb 组件的各种用法和样式

use crate::common::*;
use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

/// Breadcrumb 组件演示
#[component]
pub fn BreadcrumbDemo() -> Element {
    let breadcrumb_data = vec![
        BreadcrumbItemData {
            href: Some("/".to_string()),
            title: "首页".to_string(),
            icon: None,
            disabled: false,
            onclick: None,
        },
        BreadcrumbItemData {
            href: Some("/application".to_string()),
            title: "应用列表".to_string(),
            icon: None,
            disabled: false,
            onclick: None,
        },
        BreadcrumbItemData {
            href: None,
            title: "某应用".to_string(),
            icon: None,
            disabled: false,
            onclick: None,
        },
    ];

    let breadcrumb_with_icon = vec![
        BreadcrumbItemData {
            href: Some("/".to_string()),
            title: "首页".to_string(),
            icon: Some("🏠".to_string()),
            disabled: false,
            onclick: None,
        },
        BreadcrumbItemData {
            href: Some("/application".to_string()),
            title: "应用列表".to_string(),
            icon: Some("📱".to_string()),
            disabled: false,
            onclick: None,
        },
        BreadcrumbItemData {
            href: None,
            title: "某应用".to_string(),
            icon: Some("⚙️".to_string()),
            disabled: false,
            onclick: None,
        },
    ];

    rsx! {
        div { class: "breadcrumb-demo",
            h1 { "Breadcrumb 面包屑" }
            p { "显示当前页面在系统层级结构中的位置，并能向上返回。" }

            DemoSection {
                title: "基本用法",
                description: "最简单的用法。",

                div {
                    Breadcrumb {
                        separator: "/".to_string(),
                        BreadcrumbItem {
                            href: "/".to_string(),
                            "首页"
                        }
                        BreadcrumbItem {
                            href: "/application".to_string(),
                            "应用列表"
                        }
                        BreadcrumbItem {
                            "某应用"
                        }
                    }
                }
            }

            DemoSection {
                title: "带图标的",
                description: "图标放在文字前面。",

                div {
                    Breadcrumb {
                        separator: "/".to_string(),
                        BreadcrumbItem {
                            href: "/".to_string(),
                            icon: "🏠".to_string(),
                            "首页"
                        }
                        BreadcrumbItem {
                            href: "/application".to_string(),
                            icon: "📱".to_string(),
                            "应用列表"
                        }
                        BreadcrumbItem {
                            icon: "⚙️".to_string(),
                            "某应用"
                        }
                    }
                }
            }

            DemoSection {
                title: "分隔符",
                description: "使用 separator=\">\"> 可以自定义分隔符。",

                div {
                    Space {
                        direction: SpaceDirection::Vertical,
                        Breadcrumb {
                            separator: ">".to_string(),
                            BreadcrumbItem {
                                href: "/".to_string(),
                                "首页"
                            }
                            BreadcrumbItem {
                                href: "/application".to_string(),
                                "应用列表"
                            }
                            BreadcrumbItem {
                                "某应用"
                            }
                        }
                        Breadcrumb {
                            separator: "→".to_string(),
                            BreadcrumbItem {
                                href: "/".to_string(),
                                "首页"
                            }
                            BreadcrumbItem {
                                href: "/application".to_string(),
                                "应用列表"
                            }
                            BreadcrumbItem {
                                "某应用"
                            }
                        }
                        Breadcrumb {
                            separator: "|".to_string(),
                            BreadcrumbItem {
                                href: "/".to_string(),
                                "首页"
                            }
                            BreadcrumbItem {
                                href: "/application".to_string(),
                                "应用列表"
                            }
                            BreadcrumbItem {
                                "某应用"
                            }
                        }
                    }
                }
            }

            DemoSection {
                title: "带下拉菜单的面包屑",
                description: "面包屑支持下拉菜单。",

                div {
                    Breadcrumb {
                        separator: "/".to_string(),
                        BreadcrumbItem {
                            href: "/".to_string(),
                            "首页"
                        }
                        BreadcrumbItem {
                            onclick: move |_| {
                                println!("点击了应用中心");
                            },
                            "应用中心"
                        }
                        BreadcrumbItem {
                            onclick: move |_| {
                                println!("点击了应用列表");
                            },
                            "应用列表"
                        }
                        BreadcrumbItem {
                            "某应用"
                        }
                    }
                }
            }

            DemoSection {
                title: "数据驱动",
                description: "通过 items 属性设置路由。",

                div {
                    Breadcrumb {
                        separator: "/".to_string(),
                        items: breadcrumb_data.clone(),
                    }
                }
            }

            DemoSection {
                title: "带图标的数据驱动",
                description: "通过 items 属性设置带图标的路由。",

                div {
                    Breadcrumb {
                        separator: "/".to_string(),
                        items: breadcrumb_with_icon.clone(),
                    }
                }
            }

            DemoSection {
                title: "禁用状态",
                description: "面包屑项可以被禁用。",

                div {
                    Breadcrumb {
                        separator: "/".to_string(),
                        BreadcrumbItem {
                            href: "/".to_string(),
                            "首页"
                        }
                        BreadcrumbItem {
                            disabled: true,
                            "禁用项"
                        }
                        BreadcrumbItem {
                            "当前页"
                        }
                    }
                }
            }
        }
    }
}

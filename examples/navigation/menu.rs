//! Menu 组件示例
//!
//! 展示 Menu 导航菜单组件的各种用法

use crate::common::{ApiDocumentation, DemoSection, PropDoc};
use dioxus::prelude::*;

/// Menu 组件演示页面
#[component]
pub fn MenuDemo() -> Element {
    let mut current = use_signal(|| "mail".to_string());
    let mut collapsed = use_signal(|| false);
    let mut theme = use_signal(|| "light".to_string());
    let mut mode = use_signal(|| "horizontal".to_string());

    rsx! {
        div {
            class: "menu-demo",
            style: "padding: 24px;",

            h1 {
                style: "margin: 0 0 24px 0; color: #262626; font-size: 28px;",
                "Menu 导航菜单"
            }

            p {
                style: "margin: 0 0 32px 0; color: #666; font-size: 14px;",
                "为页面和功能提供导航的菜单列表。"
            }

            // 顶部导航
            DemoSection {
                title: "顶部导航".to_string(),
                description: "水平的顶部导航菜单。".to_string(),
                div {
                    style: "border-bottom: 1px solid #f0f0f0;",
                    nav {
                        style: "display: flex; background: white;",

                        a {
                            href: "#",
                            style: if current() == "mail" {
                                "padding: 12px 20px; color: #1890ff; border-bottom: 2px solid #1890ff; text-decoration: none;"
                            } else {
                                "padding: 12px 20px; color: #666; text-decoration: none; border-bottom: 2px solid transparent;"
                            },
                            onclick: move |_| current.set("mail".to_string()),
                            "📧 Mail"
                        }
                        a {
                            href: "#",
                            style: if current() == "app" {
                                "padding: 12px 20px; color: #1890ff; border-bottom: 2px solid #1890ff; text-decoration: none;"
                            } else {
                                "padding: 12px 20px; color: #666; text-decoration: none; border-bottom: 2px solid transparent;"
                            },
                            onclick: move |_| current.set("app".to_string()),
                            "📱 App"
                        }
                        a {
                            href: "#",
                            style: if current() == "setting" {
                                "padding: 12px 20px; color: #1890ff; border-bottom: 2px solid #1890ff; text-decoration: none;"
                            } else {
                                "padding: 12px 20px; color: #666; text-decoration: none; border-bottom: 2px solid transparent;"
                            },
                            onclick: move |_| current.set("setting".to_string()),
                            "⚙️ Setting"
                        }
                    }
                }
            }

            // 内嵌菜单
            DemoSection {
                title: "内嵌菜单".to_string(),
                description: "垂直菜单，子菜单内嵌在菜单区域。".to_string(),
                div {
                    style: "width: 256px; border: 1px solid #f0f0f0;",
                    nav {
                        style: "background: white;",

                        div {
                            style: "padding: 8px 0;",

                            // 邮件组
                            div {
                                a {
                                    href: "#",
                                    style: "display: block; padding: 8px 24px; color: #666; text-decoration: none; background: #e6f7ff;",
                                    "📧 邮件组"
                                }
                                div {
                                    style: "padding-left: 24px; background: #fafafa;",
                                    a {
                                        href: "#",
                                        style: "display: block; padding: 8px 24px; color: #666; text-decoration: none;",
                                        "选项1"
                                    }
                                    a {
                                        href: "#",
                                        style: "display: block; padding: 8px 24px; color: #666; text-decoration: none;",
                                        "选项2"
                                    }
                                }
                            }

                            // 导航组
                            div {
                                a {
                                    href: "#",
                                    style: "display: block; padding: 8px 24px; color: #666; text-decoration: none;",
                                    "🧭 导航组"
                                }
                                div {
                                    style: "padding-left: 24px; background: #fafafa;",
                                    a {
                                        href: "#",
                                        style: "display: block; padding: 8px 24px; color: #666; text-decoration: none;",
                                        "选项3"
                                    }
                                    a {
                                        href: "#",
                                        style: "display: block; padding: 8px 24px; color: #666; text-decoration: none;",
                                        "选项4"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // 缩起内嵌菜单
            DemoSection {
                title: "缩起内嵌菜单".to_string(),
                description: "内嵌菜单可以被缩起/展开。".to_string(),
                div {
                    style: "display: flex; gap: 16px;",

                    button {
                        style: "padding: 4px 15px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer;",
                        onclick: move |_| collapsed.set(!collapsed()),
                        if collapsed() { "展开" } else { "收起" }
                    }

                    div {
                        style: if collapsed() {
                            "width: 80px; border: 1px solid #f0f0f0; transition: width 0.3s;"
                        } else {
                            "width: 256px; border: 1px solid #f0f0f0; transition: width 0.3s;"
                        },
                        nav {
                            style: "background: white;",

                            div {
                                style: "padding: 8px 0;",

                                a {
                                    href: "#",
                                    style: "display: block; padding: 8px 24px; color: #666; text-decoration: none;",
                                    if collapsed() { "📧" } else { "📧 邮件" }
                                }
                                a {
                                    href: "#",
                                    style: "display: block; padding: 8px 24px; color: #666; text-decoration: none;",
                                    if collapsed() { "📱" } else { "📱 应用" }
                                }
                                a {
                                    href: "#",
                                    style: "display: block; padding: 8px 24px; color: #666; text-decoration: none;",
                                    if collapsed() { "⚙️" } else { "⚙️ 设置" }
                                }
                            }
                        }
                    }
                }
            }

            // 主题
            DemoSection {
                title: "主题".to_string(),
                description: "内建了两套主题 light 和 dark，默认 light。".to_string(),
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    div {
                        style: "display: flex; gap: 8px;",
                        button {
                            style: if theme() == "light" {
                                "padding: 4px 15px; border: 1px solid #1890ff; border-radius: 6px; background: #1890ff; color: white; cursor: pointer;"
                            } else {
                                "padding: 4px 15px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer;"
                            },
                            onclick: move |_| theme.set("light".to_string()),
                            "Light"
                        }
                        button {
                            style: if theme() == "dark" {
                                "padding: 4px 15px; border: 1px solid #1890ff; border-radius: 6px; background: #1890ff; color: white; cursor: pointer;"
                            } else {
                                "padding: 4px 15px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer;"
                            },
                            onclick: move |_| theme.set("dark".to_string()),
                            "Dark"
                        }
                    }

                    div {
                        style: "width: 256px; border: 1px solid #f0f0f0;",
                        nav {
                            style: if theme() == "dark" {
                                "background: #001529;"
                            } else {
                                "background: white;"
                            },

                            div {
                                style: "padding: 8px 0;",

                                a {
                                    href: "#",
                                    style: if theme() == "dark" {
                                        "display: block; padding: 8px 24px; color: #fff; text-decoration: none;"
                                    } else {
                                        "display: block; padding: 8px 24px; color: #666; text-decoration: none;"
                                    },
                                    "📧 邮件"
                                }
                                a {
                                    href: "#",
                                    style: if theme() == "dark" {
                                        "display: block; padding: 8px 24px; color: #fff; text-decoration: none;"
                                    } else {
                                        "display: block; padding: 8px 24px; color: #666; text-decoration: none;"
                                    },
                                    "📱 应用"
                                }
                                a {
                                    href: "#",
                                    style: if theme() == "dark" {
                                        "display: block; padding: 8px 24px; color: #fff; text-decoration: none;"
                                    } else {
                                        "display: block; padding: 8px 24px; color: #666; text-decoration: none;"
                                    },
                                    "⚙️ 设置"
                                }
                            }
                        }
                    }
                }
            }

            // 切换菜单类型
            DemoSection {
                title: "切换菜单类型".to_string(),
                description: "展示动态切换模式。".to_string(),
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    div {
                        style: "display: flex; gap: 8px;",
                        button {
                            style: if mode() == "horizontal" {
                                "padding: 4px 15px; border: 1px solid #1890ff; border-radius: 6px; background: #1890ff; color: white; cursor: pointer;"
                            } else {
                                "padding: 4px 15px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer;"
                            },
                            onclick: move |_| mode.set("horizontal".to_string()),
                            "Horizontal"
                        }
                        button {
                            style: if mode() == "vertical" {
                                "padding: 4px 15px; border: 1px solid #1890ff; border-radius: 6px; background: #1890ff; color: white; cursor: pointer;"
                            } else {
                                "padding: 4px 15px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer;"
                            },
                            onclick: move |_| mode.set("vertical".to_string()),
                            "Vertical"
                        }
                        button {
                            style: if mode() == "inline" {
                                "padding: 4px 15px; border: 1px solid #1890ff; border-radius: 6px; background: #1890ff; color: white; cursor: pointer;"
                            } else {
                                "padding: 4px 15px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer;"
                            },
                            onclick: move |_| mode.set("inline".to_string()),
                            "Inline"
                        }
                    }

                    if mode() == "horizontal" {
                        div {
                            style: "border-bottom: 1px solid #f0f0f0;",
                            nav {
                                style: "display: flex; background: white;",

                                a {
                                    href: "#",
                                    style: "padding: 12px 20px; color: #1890ff; border-bottom: 2px solid #1890ff; text-decoration: none;",
                                    "📧 Mail"
                                }
                                a {
                                    href: "#",
                                    style: "padding: 12px 20px; color: #666; text-decoration: none; border-bottom: 2px solid transparent;",
                                    "📱 App"
                                }
                                a {
                                    href: "#",
                                    style: "padding: 12px 20px; color: #666; text-decoration: none; border-bottom: 2px solid transparent;",
                                    "⚙️ Setting"
                                }
                            }
                        }
                    } else {
                        div {
                            style: "width: 256px; border: 1px solid #f0f0f0;",
                            nav {
                                style: "background: white;",

                                div {
                                    style: "padding: 8px 0;",

                                    a {
                                        href: "#",
                                        style: "display: block; padding: 8px 24px; color: #1890ff; text-decoration: none; background: #e6f7ff;",
                                        "📧 邮件"
                                    }
                                    a {
                                        href: "#",
                                        style: "display: block; padding: 8px 24px; color: #666; text-decoration: none;",
                                        "📱 应用"
                                    }
                                    a {
                                        href: "#",
                                        style: "display: block; padding: 8px 24px; color: #666; text-decoration: none;",
                                        "⚙️ 设置"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // API 文档
            ApiDocumentation {
                component_name: "Menu".to_string(),
                props: vec![
                    PropDoc {
                        name: "default_open_keys".to_string(),
                        prop_type: "Vec<String>".to_string(),
                        default: "[]".to_string(),
                        description: "初始展开的 SubMenu 菜单项 key 数组".to_string(),
                    },
                    PropDoc {
                        name: "default_selected_keys".to_string(),
                        prop_type: "Vec<String>".to_string(),
                        default: "[]".to_string(),
                        description: "初始选中的菜单项 key 数组".to_string(),
                    },
                    PropDoc {
                        name: "expand_icon".to_string(),
                        prop_type: "Element".to_string(),
                        default: "-".to_string(),
                        description: "自定义展开图标".to_string(),
                    },
                    PropDoc {
                        name: "force_sub_menu_render".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "在子菜单展示之前就渲染进 DOM".to_string(),
                    },
                    PropDoc {
                        name: "inline_collapsed".to_string(),
                        prop_type: "bool".to_string(),
                        default: "-".to_string(),
                        description: "inline 时菜单是否收起状态".to_string(),
                    },
                    PropDoc {
                        name: "inline_indent".to_string(),
                        prop_type: "i32".to_string(),
                        default: "24".to_string(),
                        description: "inline 模式的菜单缩进宽度".to_string(),
                    },
                    PropDoc {
                        name: "mode".to_string(),
                        prop_type: "String".to_string(),
                        default: "vertical".to_string(),
                        description: "菜单类型：vertical | horizontal | inline".to_string(),
                    },
                    PropDoc {
                        name: "multiple".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "是否允许多选".to_string(),
                    },
                    PropDoc {
                        name: "open_keys".to_string(),
                        prop_type: "Vec<String>".to_string(),
                        default: "-".to_string(),
                        description: "当前展开的 SubMenu 菜单项 key 数组".to_string(),
                    },
                    PropDoc {
                        name: "selectable".to_string(),
                        prop_type: "bool".to_string(),
                        default: "true".to_string(),
                        description: "是否允许选中".to_string(),
                    },
                    PropDoc {
                        name: "selected_keys".to_string(),
                        prop_type: "Vec<String>".to_string(),
                        default: "-".to_string(),
                        description: "当前选中的菜单项 key 数组".to_string(),
                    },
                    PropDoc {
                        name: "style".to_string(),
                        prop_type: "String".to_string(),
                        default: "".to_string(),
                        description: "根节点样式".to_string(),
                    },
                    PropDoc {
                        name: "sub_menu_close_delay".to_string(),
                        prop_type: "f64".to_string(),
                        default: "0.1".to_string(),
                        description: "用户鼠标离开子菜单后关闭延时，单位：秒".to_string(),
                    },
                    PropDoc {
                        name: "sub_menu_open_delay".to_string(),
                        prop_type: "f64".to_string(),
                        default: "0".to_string(),
                        description: "用户鼠标进入子菜单后开启延时，单位：秒".to_string(),
                    },
                    PropDoc {
                        name: "theme".to_string(),
                        prop_type: "String".to_string(),
                        default: "light".to_string(),
                        description: "主题颜色：light | dark".to_string(),
                    },
                    PropDoc {
                        name: "trigger_sub_menu_action".to_string(),
                        prop_type: "String".to_string(),
                        default: "hover".to_string(),
                        description: "SubMenu 展开/关闭的触发行为：hover | click".to_string(),
                    },
                ]
            }
        }
    }
}

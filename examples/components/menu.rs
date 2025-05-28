//!
//!
//! 展示 Menu 组件的各种用法和样式

use crate::common::*;
use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

/// Menu 组件演示
#[component]
pub fn MenuDemo() -> Element {
    let selected_keys = use_signal(|| vec!["1".to_string()]);
    let open_keys = use_signal(|| vec!["sub1".to_string()]);

    let menu_data = vec![
        MenuItemData {
            key: "1".to_string(),
            title: "导航一".to_string(),
            is_group: false,
            href: None,
            group_title: None,
            icon: Some("📧".to_string()),
            disabled: false,
            children: None,
        },
        MenuItemData {
            key: "2".to_string(),
            title: "导航二".to_string(),
            is_group: false,
            href: None,
            group_title: None,
            icon: Some("📅".to_string()),
            disabled: false,
            children: None,
        },
        MenuItemData {
            key: "sub1".to_string(),
            title: "导航 - 子菜单".to_string(),
            is_group: false,
            href: None,
            group_title: None,
            icon: Some("⚙️".to_string()),
            disabled: false,
            children: Some(vec![
                MenuItemData {
                    key: "3".to_string(),
                    title: "选项3".to_string(),
                    is_group: false,
                    href: None,
                    group_title: None,
                    icon: None,
                    disabled: false,
                    children: None,
                },
                MenuItemData {
                    key: "4".to_string(),
                    title: "选项4".to_string(),
                    is_group: false,
                    href: None,
                    group_title: None,
                    icon: None,
                    disabled: false,
                    children: None,
                },
                MenuItemData {
                    key: "sub1-2".to_string(),
                    title: "子菜单".to_string(),
                    is_group: false,
                    href: None,
                    group_title: None,
                    icon: None,
                    disabled: false,
                    children: Some(vec![
                        MenuItemData {
                            key: "5".to_string(),
                            title: "选项5".to_string(),
                            is_group: false,
                            href: None,
                            group_title: None,
                            icon: None,
                            disabled: false,
                            children: None,
                        },
                        MenuItemData {
                            key: "6".to_string(),
                            title: "选项6".to_string(),
                            is_group: false,
                            href: None,
                            group_title: None,
                            icon: None,
                            disabled: false,
                            children: None,
                        },
                    ]),
                },
            ]),
        },
        MenuItemData {
            key: "sub2".to_string(),
            title: "导航 - 子菜单2".to_string(),
            is_group: false,
            href: None,
            group_title: None,
            icon: Some("📱".to_string()),
            disabled: false,
            children: Some(vec![
                MenuItemData {
                    key: "7".to_string(),
                    title: "选项7".to_string(),
                    is_group: false,
                    href: None,
                    group_title: None,
                    icon: None,
                    disabled: false,
                    children: None,
                },
                MenuItemData {
                    key: "8".to_string(),
                    title: "选项8".to_string(),
                    is_group: false,
                    href: None,
                    group_title: None,
                    icon: None,
                    disabled: false,
                    children: None,
                },
            ]),
        },
    ];

    rsx! {
        div {
            class: "menu-demo",
            h1 { "Menu 导航菜单" }
            p { "为页面和功能提供导航的菜单列表。" }

            DemoSection {
                title: "顶部导航",
                description: "水平的顶部导航菜单。",

                div {
                    Menu {
                        mode: MenuMode::Horizontal,
                        selected_keys: vec!["mail".to_string()],
                        style: "line-height: 64px;".to_string(),
                        MenuItem {
                            menu_key: "mail".to_string(),
                            icon: "📧".to_string(),
                            "导航一"
                        }
                        MenuItem {
                            menu_key: "app".to_string(),
                            icon: "📱".to_string(),
                            disabled: true,
                            "导航二(禁用)"
                        }
                        SubMenu {
                            menu_key: "SubMenu".to_string(),
                            icon: "⚙️".to_string(),
                            title: "导航 - 子菜单".to_string(),
                            MenuGroup {
                                title: "分组1".to_string(),
                                menu_key: "setting:1".to_string(),
                                MenuItem {
                                    menu_key: "setting:1".to_string(),
                                    "选项1"
                                }
                                MenuItem {
                                    menu_key: "setting:2".to_string(),
                                    "选项2"
                                }
                            }
                            MenuGroup {
                                title: "分组2".to_string(),
                                menu_key: "setting:2".to_string(),
                                MenuItem {
                                    menu_key: "setting:3".to_string(),
                                    "选项3"
                                }
                                MenuItem {
                                    menu_key: "setting:4".to_string(),
                                    "选项4"
                                }
                            }
                        }
                        MenuItem {
                            menu_key: "alipay".to_string(),
                            "导航四"
                        }
                    }
                }
            }

            DemoSection {
                title: "内嵌菜单",
                description: "垂直菜单，子菜单内嵌在菜单区域。",

                div {
                    Menu {
                        mode: MenuMode::Inline,
                        default_selected_keys: vec!["1".to_string()],
                        default_open_keys: vec!["sub1".to_string()],
                        style: "width: 256px;".to_string(),
                        MenuItem {
                            menu_key: "1".to_string(),
                            icon: "📧".to_string(),
                            "导航一"
                        }
                        MenuItem {
                            menu_key: "2".to_string(),
                            icon: "📅".to_string(),
                            "导航二"
                        }
                        MenuItem {
                            menu_key: "3".to_string(),
                            icon: "📱".to_string(),
                            "导航三"
                        }
                        SubMenu {
                            menu_key: "sub1".to_string(),
                            icon: "⚙️".to_string(),
                            title: "导航 - 子菜单".to_string(),
                            MenuItem {
                                menu_key: "5".to_string(),
                                "选项5"
                            }
                            MenuItem {
                                menu_key: "6".to_string(),
                                "选项6"
                            }
                            MenuItem {
                                menu_key: "7".to_string(),
                                "选项7"
                            }
                            MenuItem {
                                menu_key: "8".to_string(),
                                "选项8"
                            }
                        }
                        SubMenu {
                            menu_key: "sub2".to_string(),
                            icon: "📱".to_string(),
                            title: "导航 - 子菜单2".to_string(),
                            MenuItem {
                                menu_key: "9".to_string(),
                                "选项9"
                            }
                            MenuItem {
                                menu_key: "10".to_string(),
                                "选项10"
                            }
                            SubMenu {
                                menu_key: "sub3".to_string(),
                                title: "子菜单".to_string(),
                                MenuItem {
                                    menu_key: "11".to_string(),
                                    "选项11"
                                }
                                MenuItem {
                                    menu_key: "12".to_string(),
                                    "选项12"
                                }
                            }
                        }
                    }
                }
            }

            DemoSection {
                title: "垂直菜单",
                description: "子菜单是弹出的形式。",

                div {
                    Menu {
                        mode: MenuMode::Vertical,
                        style: "width: 256px;".to_string(),
                        MenuItem {
                            menu_key: "1".to_string(),
                            icon: "📧".to_string(),
                            "导航一"
                        }
                        MenuItem {
                            menu_key: "2".to_string(),
                            icon: "📅".to_string(),
                            "导航二"
                        }
                        SubMenu {
                            menu_key: "sub1".to_string(),
                            icon: "⚙️".to_string(),
                            title: "导航 - 子菜单".to_string(),
                            MenuItem {
                                menu_key: "3".to_string(),
                                "选项3"
                            }
                            MenuItem {
                                menu_key: "4".to_string(),
                                "选项4"
                            }
                            SubMenu {
                                menu_key: "sub1-2".to_string(),
                                title: "子菜单".to_string(),
                                MenuItem {
                                    menu_key: "5".to_string(),
                                    "选项5"
                                }
                                MenuItem {
                                    menu_key: "6".to_string(),
                                    "选项6"
                                }
                            }
                        }
                        SubMenu {
                            menu_key: "sub2".to_string(),
                            icon: "📱".to_string(),
                            title: "导航 - 子菜单2".to_string(),
                            MenuItem {
                                menu_key: "7".to_string(),
                                "选项7"
                            }
                            MenuItem {
                                menu_key: "8".to_string(),
                                "选项8"
                            }
                        }
                    }
                }
            }

            DemoSection {
                title: "主题",
                description: "内建了两套主题 light|dark，默认 light。",

                div {
                    Space {
                        direction: SpaceDirection::Vertical,
                        size: SpaceSize::Large,
                        div {
                            h4 { "Light Theme" }
                            Menu {
                                mode: MenuMode::Horizontal,
                                theme: MenuTheme::Light,
                                default_selected_keys: vec!["1".to_string()],
                                MenuItem {
                                    menu_key: "1".to_string(),
                                    icon: "📧".to_string(),
                                    "导航一"
                                }
                                MenuItem {
                                    menu_key: "2".to_string(),
                                    icon: "📅".to_string(),
                                    "导航二"
                                }
                                MenuItem {
                                    menu_key: "3".to_string(),
                                    icon: "📱".to_string(),
                                    "导航三"
                                }
                            }
                        }
                        div {
                            h4 { "Dark Theme" }
                            Menu {
                                mode: MenuMode::Horizontal,
                                theme: MenuTheme::Dark,
                                default_selected_keys: vec!["1".to_string()],
                                MenuItem {
                                    menu_key: "1".to_string(),
                                    icon: "📧".to_string(),
                                    "导航一"
                                }
                                MenuItem {
                                    menu_key: "2".to_string(),
                                    icon: "📅".to_string(),
                                    "导航二"
                                }
                                MenuItem {
                                    menu_key: "3".to_string(),
                                    icon: "📱".to_string(),
                                    "导航三"
                                }
                            }
                        }
                    }
                }
            }

            DemoSection {
                title: "受控的菜单",
                description: "用 selectedKeys 和 openKeys 对菜单进行受控。",

                div {
                    div {
                        Menu {
                            mode: MenuMode::Inline,
                            selected_keys: selected_keys.read().clone(),
                            open_keys: open_keys.read().clone(),
                            style: "width: 256px;".to_string(),
                            // on_click: move |key: String| {
                            //     selected_keys.set(vec![key]);
                            // },
                            // on_open_change: move |keys: Vec<String>| {
                            //     open_keys.set(keys);
                            // },
                            MenuItem {
                                menu_key: "1".to_string(),
                                icon: "📧".to_string(),
                                "导航一"
                            }
                            MenuItem {
                                menu_key: "2".to_string(),
                                icon: "📅".to_string(),
                                "导航二"
                            }
                            SubMenu {
                                menu_key: "sub1".to_string(),
                                icon: "⚙️".to_string(),
                                title: "导航 - 子菜单".to_string(),
                                MenuItem {
                                    menu_key: "3".to_string(),
                                    "选项3"
                                }
                                MenuItem {
                                    menu_key: "4".to_string(),
                                    "选项4"
                                }
                            }
                        }
                    }
                }
            }

            DemoSection {
                title: "数据驱动菜单",
                description: "使用 items 属性定义菜单内容。",

                div {
                    Menu {
                        mode: MenuMode::Inline,
                        style: "width: 256px;".to_string(),
                        items: menu_data.clone(),
                    }
                }
            }
        }
    }
}

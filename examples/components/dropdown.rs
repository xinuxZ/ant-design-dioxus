//!
//! 展示 Dropdown 组件的各种用法和样式

use crate::common::*;
use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

/// Dropdown 组件演示
#[component]
pub fn DropdownDemo() -> Element {
    let mut clicked_item = use_signal(|| String::new());
    let mut visible_state = use_signal(|| false);
    let mut context_menu_item = use_signal(|| String::new());

    // 基础菜单项
    let basic_menu_items = vec![
        DropdownMenuItem::new("1", "第一个菜单项"),
        DropdownMenuItem::new("2", "第二个菜单项"),
        DropdownMenuItem::new("3", "第三个菜单项").disabled(true),
        DropdownMenuItem::new("4", "危险操作").with_divider(),
    ];

    // 带图标的菜单项
    let icon_menu_items = vec![
        DropdownMenuItem::new("user", "用户信息").with_icon("👤"),
        DropdownMenuItem::new("settings", "设置").with_icon("⚙️"),
        DropdownMenuItem::new("logout", "退出登录")
            .with_icon("🚪")
            .with_divider(),
    ];

    // 多级菜单项
    let nested_menu_items = vec![
        DropdownMenuItem::new("file", "文件").with_children(vec![
            DropdownMenuItem::new("new", "新建"),
            DropdownMenuItem::new("open", "打开"),
            DropdownMenuItem::new("save", "保存"),
        ]),
        DropdownMenuItem::new("edit", "编辑").with_children(vec![
            DropdownMenuItem::new("copy", "复制"),
            DropdownMenuItem::new("paste", "粘贴"),
            DropdownMenuItem::new("delete", "删除").disabled(true),
        ]),
        DropdownMenuItem::new("help", "帮助"),
    ];

    rsx! {
        div {
            class: "component-showcase",

            h1 {
                style: "margin: 0 0 16px 0; color: #262626; font-size: 28px;",
                "Dropdown 下拉菜单"
            }

            p {
                style: "margin: 0 0 24px 0; color: #666; font-size: 14px;",
                "向下弹出的列表。当页面上的操作命令过多时，用此组件可以收纳操作元素。点击或移入触点，会出现一个下拉菜单。"
            }

            // 基础用法
            DemoSection {
                title: "基础用法",
                description: "最简单的下拉菜单。",

                div {
                    style: "display: flex; gap: 16px; align-items: center;",

                    Dropdown {
                        menu_items: basic_menu_items.clone(),
                        on_menu_click: move |key| {
                            clicked_item.set(format!("点击了菜单项: {}", key));
                        },

                        Button {
                            "悬停我"
                        }
                    }

                    // div {
                    //     style: "color: #666; font-size: 14px;",
                    //     "{clicked_item}"
                    // }
                }
            }

            // 弹出位置
            DemoSection {
                title: "弹出位置",
                description: "支持 6 个弹出位置。",

                div {
                    style: "display: grid; grid-template-columns: repeat(3, 1fr); gap: 16px; max-width: 600px;",

                    Dropdown {
                        menu_items: basic_menu_items.clone(),
                        placement: DropdownPlacement::TopLeft,

                        Button {
                            "上左"
                        }
                    }

                    Dropdown {
                        menu_items: basic_menu_items.clone(),
                        placement: DropdownPlacement::Top,

                        Button {
                            "上中"
                        }
                    }

                    Dropdown {
                        menu_items: basic_menu_items.clone(),
                        placement: DropdownPlacement::TopRight,

                        Button {
                            "上右"
                        }
                    }

                    Dropdown {
                        menu_items: basic_menu_items.clone(),
                        placement: DropdownPlacement::BottomLeft,

                        Button {
                            "下左"
                        }
                    }

                    Dropdown {
                        menu_items: basic_menu_items.clone(),
                        placement: DropdownPlacement::Bottom,

                        Button {
                            "下中"
                        }
                    }

                    Dropdown {
                        menu_items: basic_menu_items.clone(),
                        placement: DropdownPlacement::BottomRight,

                        Button {
                            "下右"
                        }
                    }
                }
            }

            // 触发方式
            DemoSection {
                title: "触发方式",
                description: "默认是移入触发菜单，可以点击触发。",

                div {
                    style: "display: flex; gap: 16px;",

                    Dropdown {
                        menu_items: basic_menu_items.clone(),
                        trigger: DropdownTrigger::Hover,

                        Button {
                            "悬停触发"
                        }
                    }

                    Dropdown {
                        menu_items: basic_menu_items.clone(),
                        trigger: DropdownTrigger::Click,

                        Button {
                            "点击触发"
                        }
                    }

                    Dropdown {
                        menu_items: basic_menu_items.clone(),
                        trigger: DropdownTrigger::ContextMenu,
                        on_menu_click: move |key| {
                            context_menu_item.set(format!("右键菜单: {}", key));
                        },

                        div {
                            style: "padding: 16px; border: 1px dashed #d9d9d9; border-radius: 6px; cursor: context-menu;",
                            "右键点击这里"
                        }
                    }
                }

                if !context_menu_item().is_empty() {
                    div {
                        style: "margin-top: 16px; color: #666; font-size: 14px;",
                        "{context_menu_item}"
                    }
                }
            }

            // 带图标的菜单
            DemoSection {
                title: "带图标的菜单",
                description: "菜单项可以带图标。",

                div {
                    style: "display: flex; gap: 16px;",

                    Dropdown {
                        menu_items: icon_menu_items.clone(),

                        Button {
                            button_type: ButtonType::Primary,
                            "用户菜单"
                        }
                    }
                }
            }

            // 多级菜单
            DemoSection {
                title: "多级菜单",
                description: "传入的菜单里有多个层级。",

                div {
                    style: "display: flex; gap: 16px;",

                    Dropdown {
                        menu_items: nested_menu_items.clone(),

                        Button {
                            "多级菜单"
                        }
                    }
                }
            }

            // 禁用
            DemoSection {
                title: "禁用",
                description: "菜单可以被禁用。",

                div {
                    style: "display: flex; gap: 16px;",

                    Dropdown {
                        menu_items: basic_menu_items.clone(),
                        disabled: true,

                        Button {
                            disabled: true,
                            "禁用的下拉菜单"
                        }
                    }
                }
            }

            // 箭头指向
            DemoSection {
                title: "箭头指向",
                description: "可以展示一个箭头。",

                div {
                    style: "display: flex; gap: 16px;",

                    Dropdown {
                        menu_items: basic_menu_items.clone(),
                        arrow: true,

                        Button {
                            "带箭头"
                        }
                    }

                    Dropdown {
                        menu_items: basic_menu_items.clone(),
                        arrow: true,
                        placement: DropdownPlacement::TopLeft,

                        Button {
                            "上方带箭头"
                        }
                    }
                }
            }

            // 自定义下拉内容
            DemoSection {
                title: "自定义下拉内容",
                description: "使用 overlay 自定义下拉内容。",

                div {
                    style: "display: flex; gap: 16px;",

                    Dropdown {
                        overlay: rsx! {
                            div {
                                style: "padding: 16px; background: white; border-radius: 6px; box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15); min-width: 200px;",

                                h4 {
                                    style: "margin: 0 0 12px 0; color: #262626;",
                                    "自定义内容"
                                }

                                p {
                                    style: "margin: 0 0 12px 0; color: #666; font-size: 14px;",
                                    "这是一个自定义的下拉内容，可以包含任何元素。"
                                }

                                div {
                                    style: "display: flex; gap: 8px;",

                                    Button {
                                        button_type: ButtonType::Primary,
                                        size: ButtonSize::Small,
                                        "确定"
                                    }

                                    Button {
                                        size: ButtonSize::Small,
                                        "取消"
                                    }
                                }
                            }
                        },

                        Button {
                            "自定义内容"
                        }
                    }
                }
            }

            // 不自动关闭
            DemoSection {
                title: "点击不关闭",
                description: "设置 close_on_select 为 false，点击菜单项后不自动关闭。",

                div {
                    style: "display: flex; gap: 16px;",

                    Dropdown {
                        menu_items: basic_menu_items.clone(),
                        close_on_select: false,

                        Button {
                            "点击不关闭"
                        }
                    }
                }
            }

            // 事件监听
            DemoSection {
                title: "事件监听",
                description: "监听下拉菜单的显示/隐藏状态。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    Dropdown {
                        menu_items: basic_menu_items.clone(),
                        on_visible_change: move |visible| {
                            visible_state.set(visible);
                        },

                        Button {
                            "监听状态变化"
                        }
                    }

                    // div {
                    //     style: "color: #666; font-size: 14px;",
                    //     "下拉菜单状态: {if visible_state() { "显示" } else { "隐藏" }}"
                    // }
                }
            }

            // 自定义样式
            DemoSection {
                title: "自定义样式",
                description: "自定义下拉菜单的样式。",

                div {
                    style: "display: flex; gap: 16px;",

                    Dropdown {
                        menu_items: basic_menu_items.clone(),
                        overlay_class_name: "custom-dropdown",
                        overlay_style: "border: 2px solid #1890ff; border-radius: 8px;",

                        Button {
                            button_type: ButtonType::Primary,
                            "自定义样式"
                        }
                    }

                    Dropdown {
                        menu_items: icon_menu_items.clone(),
                        overlay_style: "background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white;",

                        Button {
                            "渐变背景"
                        }
                    }
                }
            }

            // 不同触发元素
            DemoSection {
                title: "不同触发元素",
                description: "下拉菜单可以由不同的元素触发。",

                div {
                    style: "display: flex; gap: 16px; align-items: center;",

                    Dropdown {
                        menu_items: basic_menu_items.clone(),

                        a {
                            href: "#",
                            style: "color: #1890ff; text-decoration: none;",
                            "链接触发"
                        }
                    }

                    Dropdown {
                        menu_items: basic_menu_items.clone(),

                        span {
                            style: "padding: 8px 12px; background: #f0f0f0; border-radius: 4px; cursor: pointer;",
                            "文本触发"
                        }
                    }

                    Dropdown {
                        menu_items: basic_menu_items.clone(),

                        div {
                            style: "width: 40px; height: 40px; background: #1890ff; border-radius: 50%; display: flex; align-items: center; justify-content: center; color: white; cursor: pointer;",
                            "⚙️"
                        }
                    }
                }
            }
        }
    }
}

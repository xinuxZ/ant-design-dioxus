//! Dropdown 组件示例
//!
//! 展示 Dropdown 下拉菜单组件的各种用法

use crate::common::{ApiDocumentation, DemoSection, PropDoc};
use dioxus::prelude::*;

/// Dropdown 组件演示页面
#[component]
pub fn DropdownDemo() -> Element {
    let mut basic_visible = use_signal(|| false);
    let mut placement_visible = use_signal(|| false);
    let mut trigger_visible = use_signal(|| false);
    let mut disabled_visible = use_signal(|| false);
    let mut arrow_visible = use_signal(|| false);
    let mut button_visible = use_signal(|| false);

    rsx! {
        div {
            class: "dropdown-demo",
            style: "padding: 24px;",

            h1 {
                style: "margin: 0 0 24px 0; color: #262626; font-size: 28px;",
                "Dropdown 下拉菜单"
            }

            p {
                style: "margin: 0 0 32px 0; color: #666; font-size: 14px;",
                "向下弹出的列表。"
            }

            // 基础下拉菜单
            DemoSection {
                title: "基础下拉菜单".to_string(),
                description: "最简单的下拉菜单。".to_string(),
                div {
                    style: "position: relative; display: inline-block;",

                    button {
                        style: "padding: 4px 15px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer; display: flex; align-items: center; gap: 8px;",
                        onclick: move |_| basic_visible.set(!basic_visible()),
                        "Hover me"
                        span {
                            style: "font-size: 12px; transition: transform 0.2s;",
                            "▼"
                        }
                    }

                    if basic_visible() {
                        div {
                            style: "position: absolute; top: 100%; left: 0; z-index: 1000; min-width: 120px; background: white; border: 1px solid #d9d9d9; border-radius: 6px; box-shadow: 0 3px 6px -4px rgba(0,0,0,0.12), 0 6px 16px 0 rgba(0,0,0,0.08), 0 9px 28px 8px rgba(0,0,0,0.05); margin-top: 4px;",

                            div {
                                style: "padding: 5px 12px; cursor: pointer; font-size: 14px; color: #262626;",
                                onmouseover: |evt| {
                                    if let Some(element) = evt.target().dyn_into::<web_sys::HtmlElement>().ok() {
                                        let _ = element.style().set_property("background-color", "#f5f5f5");
                                    }
                                },
                                onmouseout: |evt| {
                                    if let Some(element) = evt.target().dyn_into::<web_sys::HtmlElement>().ok() {
                                        let _ = element.style().set_property("background-color", "transparent");
                                    }
                                },
                                onclick: move |_| basic_visible.set(false),
                                "1st menu item"
                            }
                            div {
                                style: "padding: 5px 12px; cursor: pointer; font-size: 14px; color: #262626;",
                                onmouseover: |evt| {
                                    if let Some(element) = evt.target().dyn_into::<web_sys::HtmlElement>().ok() {
                                        let _ = element.style().set_property("background-color", "#f5f5f5");
                                    }
                                },
                                onmouseout: |evt| {
                                    if let Some(element) = evt.target().dyn_into::<web_sys::HtmlElement>().ok() {
                                        let _ = element.style().set_property("background-color", "transparent");
                                    }
                                },
                                onclick: move |_| basic_visible.set(false),
                                "2nd menu item"
                            }
                            div {
                                style: "padding: 5px 12px; cursor: pointer; font-size: 14px; color: #262626;",
                                onmouseover: |evt| {
                                    if let Some(element) = evt.target().dyn_into::<web_sys::HtmlElement>().ok() {
                                        let _ = element.style().set_property("background-color", "#f5f5f5");
                                    }
                                },
                                onmouseout: |evt| {
                                    if let Some(element) = evt.target().dyn_into::<web_sys::HtmlElement>().ok() {
                                        let _ = element.style().set_property("background-color", "transparent");
                                    }
                                },
                                onclick: move |_| basic_visible.set(false),
                                "3rd menu item"
                            }
                            hr {
                                style: "margin: 4px 0; border: none; border-top: 1px solid #f0f0f0;"
                            }
                            div {
                                style: "padding: 5px 12px; cursor: pointer; font-size: 14px; color: #ff4d4f;",
                                onmouseover: |evt| {
                                    if let Some(element) = evt.target().dyn_into::<web_sys::HtmlElement>().ok() {
                                        let _ = element.style().set_property("background-color", "#fff2f0");
                                    }
                                },
                                onmouseout: |evt| {
                                    if let Some(element) = evt.target().dyn_into::<web_sys::HtmlElement>().ok() {
                                        let _ = element.style().set_property("background-color", "transparent");
                                    }
                                },
                                onclick: move |_| basic_visible.set(false),
                                "Dangerous item"
                            }
                        }
                    }
                }
            }

            // 弹出位置
            DemoSection {
                title: "弹出位置".to_string(),
                description: "支持 6 个弹出位置。".to_string(),
                div {
                    style: "display: flex; gap: 16px; flex-wrap: wrap;",

                    // Bottom Left
                    div {
                        style: "position: relative; display: inline-block;",

                        button {
                            style: "padding: 4px 15px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer;",
                            onclick: move |_| placement_visible.set(!placement_visible()),
                            "Bottom Left"
                        }

                        if placement_visible() {
                            div {
                                style: "position: absolute; top: 100%; left: 0; z-index: 1000; min-width: 120px; background: white; border: 1px solid #d9d9d9; border-radius: 6px; box-shadow: 0 3px 6px -4px rgba(0,0,0,0.12), 0 6px 16px 0 rgba(0,0,0,0.08), 0 9px 28px 8px rgba(0,0,0,0.05); margin-top: 4px;",

                                div {
                                    style: "padding: 5px 12px; cursor: pointer; font-size: 14px; color: #262626;",
                                    "Bottom Left"
                                }
                            }
                        }
                    }

                    // Bottom Center
                    div {
                        style: "position: relative; display: inline-block;",

                        button {
                            style: "padding: 4px 15px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer;",
                            "Bottom Center"
                        }
                    }

                    // Bottom Right
                    div {
                        style: "position: relative; display: inline-block;",

                        button {
                            style: "padding: 4px 15px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer;",
                            "Bottom Right"
                        }
                    }

                    // Top Left
                    div {
                        style: "position: relative; display: inline-block;",

                        button {
                            style: "padding: 4px 15px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer;",
                            "Top Left"
                        }
                    }

                    // Top Center
                    div {
                        style: "position: relative; display: inline-block;",

                        button {
                            style: "padding: 4px 15px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer;",
                            "Top Center"
                        }
                    }

                    // Top Right
                    div {
                        style: "position: relative; display: inline-block;",

                        button {
                            style: "padding: 4px 15px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer;",
                            "Top Right"
                        }
                    }
                }
            }

            // 触发方式
            DemoSection {
                title: "触发方式".to_string(),
                description: "默认是移入触发菜单，可以点击触发。".to_string(),
                div {
                    style: "display: flex; gap: 16px;",

                    // Hover trigger
                    div {
                        style: "position: relative; display: inline-block;",

                        button {
                            style: "padding: 4px 15px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer;",
                            onmouseenter: move |_| trigger_visible.set(true),
                            onmouseleave: move |_| trigger_visible.set(false),
                            "Hover me"
                        }

                        if trigger_visible() {
                            div {
                                style: "position: absolute; top: 100%; left: 0; z-index: 1000; min-width: 120px; background: white; border: 1px solid #d9d9d9; border-radius: 6px; box-shadow: 0 3px 6px -4px rgba(0,0,0,0.12), 0 6px 16px 0 rgba(0,0,0,0.08), 0 9px 28px 8px rgba(0,0,0,0.05); margin-top: 4px;",
                                onmouseenter: move |_| trigger_visible.set(true),
                                onmouseleave: move |_| trigger_visible.set(false),

                                div {
                                    style: "padding: 5px 12px; cursor: pointer; font-size: 14px; color: #262626;",
                                    "1st menu item"
                                }
                                div {
                                    style: "padding: 5px 12px; cursor: pointer; font-size: 14px; color: #262626;",
                                    "2nd menu item"
                                }
                                div {
                                    style: "padding: 5px 12px; cursor: pointer; font-size: 14px; color: #262626;",
                                    "3rd menu item"
                                }
                            }
                        }
                    }

                    // Click trigger
                    div {
                        style: "position: relative; display: inline-block;",

                        button {
                            style: "padding: 4px 15px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer;",
                            "Click me"
                        }
                    }
                }
            }

            // 禁用状态
            DemoSection {
                title: "禁用状态".to_string(),
                description: "菜单项禁用。".to_string(),
                div {
                    style: "position: relative; display: inline-block;",

                    button {
                        style: "padding: 4px 15px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer;",
                        onclick: move |_| disabled_visible.set(!disabled_visible()),
                        "Hover me"
                    }

                    if disabled_visible() {
                        div {
                            style: "position: absolute; top: 100%; left: 0; z-index: 1000; min-width: 120px; background: white; border: 1px solid #d9d9d9; border-radius: 6px; box-shadow: 0 3px 6px -4px rgba(0,0,0,0.12), 0 6px 16px 0 rgba(0,0,0,0.08), 0 9px 28px 8px rgba(0,0,0,0.05); margin-top: 4px;",

                            div {
                                style: "padding: 5px 12px; cursor: pointer; font-size: 14px; color: #262626;",
                                "1st menu item"
                            }
                            div {
                                style: "padding: 5px 12px; cursor: not-allowed; font-size: 14px; color: #bfbfbf;",
                                "2nd menu item (disabled)"
                            }
                            div {
                                style: "padding: 5px 12px; cursor: pointer; font-size: 14px; color: #262626;",
                                "3rd menu item"
                            }
                        }
                    }
                }
            }

            // 带下拉框的按钮
            DemoSection {
                title: "带下拉框的按钮".to_string(),
                description: "左边是按钮，右边是额外的相关功能菜单。可设置 icon 属性来修改右边的图标。".to_string(),
                div {
                    style: "display: flex; gap: 16px;",

                    // Button with dropdown
                    div {
                        style: "position: relative; display: inline-flex;",

                        button {
                            style: "padding: 4px 15px; border: 1px solid #1890ff; border-radius: 6px 0 0 6px; background: #1890ff; color: white; cursor: pointer; border-right: none;",
                            "Button"
                        }

                        button {
                            style: "padding: 4px 8px; border: 1px solid #1890ff; border-radius: 0 6px 6px 0; background: #1890ff; color: white; cursor: pointer; border-left: 1px solid rgba(255,255,255,0.3);",
                            onclick: move |_| button_visible.set(!button_visible()),
                            "▼"
                        }

                        if button_visible() {
                            div {
                                style: "position: absolute; top: 100%; right: 0; z-index: 1000; min-width: 120px; background: white; border: 1px solid #d9d9d9; border-radius: 6px; box-shadow: 0 3px 6px -4px rgba(0,0,0,0.12), 0 6px 16px 0 rgba(0,0,0,0.08), 0 9px 28px 8px rgba(0,0,0,0.05); margin-top: 4px;",

                                div {
                                    style: "padding: 5px 12px; cursor: pointer; font-size: 14px; color: #262626;",
                                    "1st menu item"
                                }
                                div {
                                    style: "padding: 5px 12px; cursor: pointer; font-size: 14px; color: #262626;",
                                    "2nd menu item"
                                }
                                div {
                                    style: "padding: 5px 12px; cursor: pointer; font-size: 14px; color: #262626;",
                                    "3rd menu item"
                                }
                            }
                        }
                    }

                    // Danger button with dropdown
                    div {
                        style: "position: relative; display: inline-flex;",

                        button {
                            style: "padding: 4px 15px; border: 1px solid #ff4d4f; border-radius: 6px 0 0 6px; background: #ff4d4f; color: white; cursor: pointer; border-right: none;",
                            "Danger"
                        }

                        button {
                            style: "padding: 4px 8px; border: 1px solid #ff4d4f; border-radius: 0 6px 6px 0; background: #ff4d4f; color: white; cursor: pointer; border-left: 1px solid rgba(255,255,255,0.3);",
                            "▼"
                        }
                    }
                }
            }

            // 多级菜单
            DemoSection {
                title: "多级菜单".to_string(),
                description: "传入的菜单里有多个层级。".to_string(),
                div {
                    style: "position: relative; display: inline-block;",

                    button {
                        style: "padding: 4px 15px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer;",
                        "Cascading menu"
                    }
                }
            }

            // 菜单隐藏方式
            DemoSection {
                title: "菜单隐藏方式".to_string(),
                description: "默认点击关闭菜单，可以通过 clickToClose 属性来配置。".to_string(),
                div {
                    style: "position: relative; display: inline-block;",

                    button {
                        style: "padding: 4px 15px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer;",
                        "Click and close"
                    }
                }
            }

            // 触发事件
            DemoSection {
                title: "触发事件".to_string(),
                description: "点击菜单项后会触发事件，用户可以通过相应的菜单项 key 进行不同的操作。".to_string(),
                div {
                    style: "position: relative; display: inline-block;",

                    button {
                        style: "padding: 4px 15px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer;",
                        "Click menu item"
                    }
                }
            }

            // API 文档
            ApiDocumentation {
                component_name: "Dropdown".to_string(),
                props: vec![
                    PropDoc {
                        name: "arrow".to_string(),
                        prop_type: "bool | Object".to_string(),
                        default: "false".to_string(),
                        description: "下拉框箭头是否显示".to_string(),
                    },
                    PropDoc {
                        name: "auto_adjust_overflow".to_string(),
                        prop_type: "bool".to_string(),
                        default: "true".to_string(),
                        description: "下拉框被遮挡时自动调整位置".to_string(),
                    },
                    PropDoc {
                        name: "auto_focus".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "打开后自动获取焦点".to_string(),
                    },
                    PropDoc {
                        name: "disabled".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "菜单是否禁用".to_string(),
                    },
                    PropDoc {
                        name: "destroy_popup_on_hide".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "关闭后是否销毁 Dropdown".to_string(),
                    },
                    PropDoc {
                        name: "drop_down_render".to_string(),
                        prop_type: "Function".to_string(),
                        default: "-".to_string(),
                        description: "自定义下拉框内容".to_string(),
                    },
                    PropDoc {
                        name: "get_popup_container".to_string(),
                        prop_type: "Function".to_string(),
                        default: "() => document.body".to_string(),
                        description: "菜单渲染父节点".to_string(),
                    },
                    PropDoc {
                        name: "menu".to_string(),
                        prop_type: "Menu".to_string(),
                        default: "-".to_string(),
                        description: "菜单配置项".to_string(),
                    },
                    PropDoc {
                        name: "open".to_string(),
                        prop_type: "bool".to_string(),
                        default: "-".to_string(),
                        description: "菜单是否显示，小于 4.23.0 使用 visible".to_string(),
                    },
                    PropDoc {
                        name: "overlay".to_string(),
                        prop_type: "Element | Function".to_string(),
                        default: "-".to_string(),
                        description: "菜单".to_string(),
                    },
                    PropDoc {
                        name: "overlay_class_name".to_string(),
                        prop_type: "String".to_string(),
                        default: "-".to_string(),
                        description: "下拉根元素的类名称".to_string(),
                    },
                    PropDoc {
                        name: "overlay_style".to_string(),
                        prop_type: "Object".to_string(),
                        default: "-".to_string(),
                        description: "下拉根元素的样式".to_string(),
                    },
                    PropDoc {
                        name: "placement".to_string(),
                        prop_type: "String".to_string(),
                        default: "bottomLeft".to_string(),
                        description: "菜单弹出位置".to_string(),
                    },
                    PropDoc {
                        name: "trigger".to_string(),
                        prop_type: "Vec<String>".to_string(),
                        default: "['hover']".to_string(),
                        description: "触发下拉的行为".to_string(),
                    },
                    PropDoc {
                        name: "on_open_change".to_string(),
                        prop_type: "Function".to_string(),
                        default: "-".to_string(),
                        description: "菜单显示状态改变时调用".to_string(),
                    },
                ]
            }
        }
    }
}

#![allow(non_snake_case)]
//!
//! 展示 Tabs 组件的各种用法和样式

use crate::common::*;
use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

/// Tabs 组件演示
#[component]
pub fn TabsDemo() -> Element {
    let mut active_key = use_signal(|| "1".to_string());
    let mut active_key_position = use_signal(|| "1".to_string());
    let mut active_key_size = use_signal(|| "1".to_string());
    let mut active_key_card = use_signal(|| "1".to_string());
    let mut active_key_editable = use_signal(|| "1".to_string());
    let mut tab_items = use_signal(|| {
        vec![
            TabItem {
                key: "1".to_string(),
                label: "Tab 1".to_string(),
                children: rsx! { "Content of Tab Pane 1" },
                closable: Some(false),
                ..Default::default()
            },
            TabItem {
                key: "2".to_string(),
                label: "Tab 2".to_string(),
                children: rsx! { "Content of Tab Pane 2" },
                ..Default::default()
            },
            TabItem {
                key: "3".to_string(),
                label: "Tab 3".to_string(),
                children: rsx! { "Content of Tab Pane 3" },
                ..Default::default()
            },
        ]
    });

    rsx! {
        div {
            class: "component-showcase",

            h1 {
                style: "margin: 0 0 16px 0; color: #262626; font-size: 28px;",
                "Tabs 标签页"
            }

            p {
                style: "margin: 0 0 24px 0; color: #666; font-size: 14px;",
                "选项卡切换组件。"
            }

            // 基础用法
            DemoSection {
                title: "基础用法",
                description: "默认选中第一项。",

                div {
                    style: "padding: 24px; background: #fafafa; border-radius: 6px;",

                    Tabs {
                        active_key: active_key(),
                        on_change: move |key| {
                            active_key.set(key);
                        },
                        items: vec![
                            TabItem {
                                key: "1".to_string(),
                                label: "Tab 1".to_string(),
                                children: rsx! {
                                    div {
                                        style: "padding: 16px;",
                                        "Content of Tab Pane 1"
                                    }
                                },
                                ..Default::default()
                            },
                            TabItem {
                                key: "2".to_string(),
                                label: "Tab 2".to_string(),
                                children: rsx! {
                                    div {
                                        style: "padding: 16px;",
                                        "Content of Tab Pane 2"
                                    }
                                },
                                ..Default::default()
                            },
                            TabItem {
                                key: "3".to_string(),
                                label: "Tab 3".to_string(),
                                children: rsx! {
                                    div {
                                        style: "padding: 16px;",
                                        "Content of Tab Pane 3"
                                    }
                                },
                                ..Default::default()
                            },
                        ]
                    }
                }
            }

            // 禁用
            DemoSection {
                title: "禁用",
                description: "禁用某一项。",

                div {
                    style: "padding: 24px; background: #fafafa; border-radius: 6px;",

                    Tabs {
                        default_active_key: "1",
                        items: vec![
                            TabItem {
                                key: "1".to_string(),
                                label: "Tab 1".to_string(),
                                children: rsx! {
                                    div {
                                        style: "padding: 16px;",
                                        "Tab 1"
                                    }
                                },
                                ..Default::default()
                            },
                            TabItem {
                                key: "2".to_string(),
                                label: "Tab 2".to_string(),
                                disabled: true,
                                children: rsx! {
                                    div {
                                        style: "padding: 16px;",
                                        "Tab 2"
                                    }
                                },
                                ..Default::default()
                            },
                            TabItem {
                                key: "3".to_string(),
                                label: "Tab 3".to_string(),
                                children: rsx! {
                                    div {
                                        style: "padding: 16px;",
                                        "Tab 3"
                                    }
                                },
                                ..Default::default()
                            },
                        ]
                    }
                }
            }

            // 图标
            DemoSection {
                title: "图标",
                description: "有图标的标签。",

                div {
                    style: "padding: 24px; background: #fafafa; border-radius: 6px;",

                    Tabs {
                        default_active_key: "2",
                        items: vec![
                            TabItem {
                                key: "1".to_string(),
                                label: "Tab 1".to_string(),
                                icon: Some(rsx! {
                                    Icon {
                                        icon_type: "apple-outlined".to_string()
                                    }
                                }),
                                children: rsx! {
                                    div {
                                        style: "padding: 16px;",
                                        "Tab 1"
                                    }
                                },
                                ..Default::default()
                            },
                            TabItem {
                                key: "2".to_string(),
                                label: "Tab 2".to_string(),
                                icon: Some(rsx! {
                                    Icon {
                                        icon_type: "android-outlined".to_string()
                                    }
                                }),
                                children: rsx! {
                                    div {
                                        style: "padding: 16px;",
                                        "Tab 2"
                                    }
                                },
                                ..Default::default()
                            },
                        ]
                    }
                }
            }

            // 滑动
            DemoSection {
                title: "滑动",
                description: "可以左右、上下滑动，容纳更多标签。",

                div {
                    style: "padding: 24px; background: #fafafa; border-radius: 6px;",

                    Tabs {
                        default_active_key: "1",
                        items: (1..=30).map(|i| {
                            TabItem {
                                key: i.to_string(),
                                label: format!("Tab-{}", i),
                                children: rsx! {
                                    div {
                                        style: "padding: 16px;",
                                        "Content of tab {i}"
                                    }
                                },
                                ..Default::default()
                            }
                        }).collect()
                    }
                }
            }

            // 附加内容
            DemoSection {
                title: "附加内容",
                description: "可以在页签右边添加附加操作。",

                div {
                    style: "padding: 24px; background: #fafafa; border-radius: 6px;",

                    Tabs {
                        default_active_key: "1",
                        tab_bar_extra_content: Some(rsx! {
                            Button {
                                "Extra Action"
                            }
                        }),
                        items: vec![
                            TabItem {
                                key: "1".to_string(),
                                label: "Tab 1".to_string(),
                                children: rsx! {
                                    div {
                                        style: "padding: 16px;",
                                        "Content of Tab Pane 1"
                                    }
                                },
                                ..Default::default()
                            },
                            TabItem {
                                key: "2".to_string(),
                                label: "Tab 2".to_string(),
                                children: rsx! {
                                    div {
                                        style: "padding: 16px;",
                                        "Content of Tab Pane 2"
                                    }
                                },
                                ..Default::default()
                            },
                            TabItem {
                                key: "3".to_string(),
                                label: "Tab 3".to_string(),
                                children: rsx! {
                                    div {
                                        style: "padding: 16px;",
                                        "Content of Tab Pane 3"
                                    }
                                },
                                ..Default::default()
                            },
                        ]
                    }
                }
            }

            // 大小
            DemoSection {
                title: "大小",
                description: "大号页签用在页头区域，小号用在弹出框等较狭窄的容器内。",

                div {
                    style: "padding: 24px; background: #fafafa; border-radius: 6px;",

                    div {
                        style: "margin-bottom: 16px;",
                        Radio::Group {
                            value: active_key_size(),
                            on_change: move |e| {
                                active_key_size.set(e);
                            },
                            Radio {
                                value: "small",
                                "Small"
                            }
                            Radio {
                                value: "middle",
                                "Middle"
                            }
                            Radio {
                                value: "large",
                                "Large"
                            }
                        }
                    }

                    Tabs {
                        default_active_key: "2",
                        size: match active_key_size().as_str() {
                            "small" => "small",
                            "large" => "large",
                            _ => "middle"
                        },
                        items: vec![
                            TabItem {
                                key: "1".to_string(),
                                label: "Tab 1".to_string(),
                                children: rsx! {
                                    div {
                                        style: "padding: 16px;",
                                        "Content of Tab Pane 1"
                                    }
                                },
                                ..Default::default()
                            },
                            TabItem {
                                key: "2".to_string(),
                                label: "Tab 2".to_string(),
                                children: rsx! {
                                    div {
                                        style: "padding: 16px;",
                                        "Content of Tab Pane 2"
                                    }
                                },
                                ..Default::default()
                            },
                            TabItem {
                                key: "3".to_string(),
                                label: "Tab 3".to_string(),
                                children: rsx! {
                                    div {
                                        style: "padding: 16px;",
                                        "Content of Tab Pane 3"
                                    }
                                },
                                ..Default::default()
                            },
                        ]
                    }
                }
            }

            // 位置
            DemoSection {
                title: "位置",
                description: "有四个位置，tabPosition=\"left|right|top|bottom\"。在移动端下，left|right 会自动切换成 top。",

                div {
                    style: "padding: 24px; background: #fafafa; border-radius: 6px;",

                    div {
                        style: "margin-bottom: 16px;",
                        Radio::Group {
                            value: active_key_position(),
                            on_change: move |e| {
                                active_key_position.set(e);
                            },
                            Radio {
                                value: "top",
                                "top"
                            }
                            Radio {
                                value: "bottom",
                                "bottom"
                            }
                            Radio {
                                value: "left",
                                "left"
                            }
                            Radio {
                                value: "right",
                                "right"
                            }
                        }
                    }

                    Tabs {
                        default_active_key: "1",
                        tab_position: active_key_position(),
                        style: "height: 220px;",
                        items: vec![
                            TabItem {
                                key: "1".to_string(),
                                label: "Tab 1".to_string(),
                                children: rsx! {
                                    div {
                                        style: "padding: 16px;",
                                        "Content of Tab 1"
                                    }
                                },
                                ..Default::default()
                            },
                            TabItem {
                                key: "2".to_string(),
                                label: "Tab 2".to_string(),
                                children: rsx! {
                                    div {
                                        style: "padding: 16px;",
                                        "Content of Tab 2"
                                    }
                                },
                                ..Default::default()
                            },
                            TabItem {
                                key: "3".to_string(),
                                label: "Tab 3".to_string(),
                                children: rsx! {
                                    div {
                                        style: "padding: 16px;",
                                        "Content of Tab 3"
                                    }
                                },
                                ..Default::default()
                            },
                        ]
                    }
                }
            }

            // 卡片式页签
            DemoSection {
                title: "卡片式页签",
                description: "另一种样式的页签，不提供对应的垂直样式。",

                div {
                    style: "padding: 24px; background: #fafafa; border-radius: 6px;",

                    Tabs {
                        tab_type: TabsType::Card,
                        active_key: active_key_card(),
                        on_change: move |key| {
                            active_key_card.set(key);
                        },
                        items: vec![
                            TabItem {
                                key: "1".to_string(),
                                label: "Tab 1".to_string(),
                                children: rsx! {
                                    div {
                                        style: "padding: 16px;",
                                        "Content of Tab Pane 1"
                                    }
                                },
                                ..Default::default()
                            },
                            TabItem {
                                key: "2".to_string(),
                                label: "Tab 2".to_string(),
                                children: rsx! {
                                    div {
                                        style: "padding: 16px;",
                                        "Content of Tab Pane 2"
                                    }
                                },
                                ..Default::default()
                            },
                            TabItem {
                                key: "3".to_string(),
                                label: "Tab 3".to_string(),
                                children: rsx! {
                                    div {
                                        style: "padding: 16px;",
                                        "Content of Tab Pane 3"
                                    }
                                },
                                ..Default::default()
                            },
                        ]
                    }
                }
            }

            // 新增和关闭页签
            DemoSection {
                title: "新增和关闭页签",
                description: "只有卡片样式的页签支持新增和关闭选项。使用 closable={false} 禁止关闭。",

                div {
                    style: "padding: 24px; background: #fafafa; border-radius: 6px;",

                    Tabs {
                        tab_type: TabsType::EditableCard,
                        active_key: active_key_editable(),
                        on_change: move |key| {
                            active_key_editable.set(key);
                        },
                        on_edit: move |target_key, action| {
                            if action == "add" {
                                let new_key = (tab_items().len() + 1).to_string();
                                let mut items = tab_items();
                                items.push(TabItem {
                                    key: new_key.clone(),
                                    label: format!("New Tab {}", new_key),
                                    children: rsx! {
                                        div {
                                            style: "padding: 16px;",
                                            "Content of new Tab {new_key}"
                                        }
                                    },
                                    ..Default::default()
                                });
                                tab_items.set(items);
                                active_key_editable.set(new_key);
                            } else if action == "remove" {
                                let mut items = tab_items();
                                items.retain(|item| item.key != target_key);
                                tab_items.set(items);
                                if active_key_editable() == target_key && !tab_items().is_empty() {
                                    active_key_editable.set(tab_items()[0].key.clone());
                                }
                            }
                        },
                        items: tab_items()
                    }
                }
            }

            // 卡片式页签容器
            DemoSection {
                title: "卡片式页签容器",
                description: "用于容器顶部，需要一点额外的样式覆盖。",

                div {
                    style: "padding: 24px; background: #fafafa; border-radius: 6px;",

                    div {
                        class: "card-container",
                        Tabs {
                            tab_type: TabsType::Card,
                            default_active_key: "1",
                            items: vec![
                                TabItem {
                                    key: "1".to_string(),
                                    label: "Tab Title 1".to_string(),
                                    children: rsx! {
                                        div {
                                            style: "padding: 16px; background: white; min-height: 120px;",
                                            p { "Content of Tab Pane 1" }
                                            p { "Content of Tab Pane 1" }
                                            p { "Content of Tab Pane 1" }
                                        }
                                    },
                                    ..Default::default()
                                },
                                TabItem {
                                    key: "2".to_string(),
                                    label: "Tab Title 2".to_string(),
                                    children: rsx! {
                                        div {
                                            style: "padding: 16px; background: white; min-height: 120px;",
                                            p { "Content of Tab Pane 2" }
                                            p { "Content of Tab Pane 2" }
                                            p { "Content of Tab Pane 2" }
                                        }
                                    },
                                    ..Default::default()
                                },
                                TabItem {
                                    key: "3".to_string(),
                                    label: "Tab Title 3".to_string(),
                                    children: rsx! {
                                        div {
                                            style: "padding: 16px; background: white; min-height: 120px;",
                                            p { "Content of Tab Pane 3" }
                                            p { "Content of Tab Pane 3" }
                                            p { "Content of Tab Pane 3" }
                                        }
                                    },
                                    ..Default::default()
                                },
                            ]
                        }
                    }
                }
            }

            // API 文档
            ApiDocumentation {
                component_name: "Tabs",
                props: vec![
                    PropDoc {
                        name: "active_key".to_string(),
                        prop_type: "String".to_string(),
                        default: "-".to_string(),
                        description: "当前激活 tab 面板的 key".to_string(),
                    },
                    PropDoc {
                        name: "add_icon".to_string(),
                        prop_type: "Element".to_string(),
                        default: "-".to_string(),
                        description: "自定义添加按钮".to_string(),
                    },
                    PropDoc {
                        name: "animated".to_string(),
                        prop_type: "bool | Object".to_string(),
                        default: "{ inkBar: true, tabPane: false }".to_string(),
                        description: "是否使用动画切换 Tabs，在 tabPosition=top|bottom 时有效".to_string(),
                    },
                    PropDoc {
                        name: "centered".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "标签居中展示".to_string(),
                    },
                    PropDoc {
                        name: "default_active_key".to_string(),
                        prop_type: "String".to_string(),
                        default: "第一个面板".to_string(),
                        description: "初始化选中面板的 key，如果没有设置 activeKey".to_string(),
                    },
                    PropDoc {
                        name: "hide_add".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "是否隐藏加号图标，在 type=\"editable-card\" 时有效".to_string(),
                    },
                    PropDoc {
                        name: "more_icon".to_string(),
                        prop_type: "Element".to_string(),
                        default: "<EllipsisOutlined />".to_string(),
                        description: "自定义折叠 icon".to_string(),
                    },
                    PropDoc {
                        name: "popover_props".to_string(),
                        prop_type: "Object".to_string(),
                        default: "-".to_string(),
                        description: "更多菜单的 Popover 配置".to_string(),
                    },
                    PropDoc {
                        name: "size".to_string(),
                        prop_type: "String".to_string(),
                        default: "middle".to_string(),
                        description: "大小，提供 large middle 和 small 三种大小".to_string(),
                    },
                    PropDoc {
                        name: "tab_bar_extra_content".to_string(),
                        prop_type: "Element | Object".to_string(),
                        default: "-".to_string(),
                        description: "tab bar 上额外的元素".to_string(),
                    },
                    PropDoc {
                        name: "tab_bar_gutter".to_string(),
                        prop_type: "i32".to_string(),
                        default: "-".to_string(),
                        description: "tabs 之间的间隙".to_string(),
                    },
                    PropDoc {
                        name: "tab_bar_style".to_string(),
                        prop_type: "Object".to_string(),
                        default: "-".to_string(),
                        description: "tab bar 的样式对象".to_string(),
                    },
                    PropDoc {
                        name: "tab_position".to_string(),
                        prop_type: "String".to_string(),
                        default: "top".to_string(),
                        description: "页签位置，可选值有 top right bottom left".to_string(),
                    },
                    PropDoc {
                        name: "destroy_inactive_tab_pane".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "被隐藏时是否销毁 DOM 结构".to_string(),
                    },
                    PropDoc {
                        name: "tab_type".to_string(),
                        prop_type: "String".to_string(),
                        default: "line".to_string(),
                        description: "页签的基本样式，可选 line、card editable-card 类型".to_string(),
                    },
                    PropDoc {
                        name: "on_change".to_string(),
                        prop_type: "Function".to_string(),
                        default: "-".to_string(),
                        description: "切换面板的回调".to_string(),
                    },
                    PropDoc {
                        name: "on_edit".to_string(),
                        prop_type: "Function".to_string(),
                        default: "-".to_string(),
                        description: "新增和删除页签的回调，在 type=\"editable-card\" 时有效".to_string(),
                    },
                    PropDoc {
                        name: "on_tab_click".to_string(),
                        prop_type: "Function".to_string(),
                        default: "-".to_string(),
                        description: "tab 被点击的回调".to_string(),
                    },
                    PropDoc {
                        name: "on_tab_scroll".to_string(),
                        prop_type: "Function".to_string(),
                        default: "-".to_string(),
                        description: "tab 滚动时触发".to_string(),
                    },
                    PropDoc {
                        name: "items".to_string(),
                        prop_type: "Vec<TabItem>".to_string(),
                        default: "-".to_string(),
                        description: "配置选项卡内容".to_string(),
                    },
                ]
            }

            // TabPane API
            ApiDocumentation {
                component_name: "TabPane",
                props: vec![
                    PropDoc {
                        name: "closable".to_string(),
                        prop_type: "bool".to_string(),
                        default: "true".to_string(),
                        description: "标签是否可关闭，在 type=\"editable-card\" 时有效".to_string(),
                    },
                    PropDoc {
                        name: "destroy_inactive_tab_pane".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "被隐藏时是否销毁 DOM 结构".to_string(),
                    },
                    PropDoc {
                        name: "disabled".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "禁用某一项".to_string(),
                    },
                    PropDoc {
                        name: "force_render".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "被隐藏时是否渲染 DOM 结构".to_string(),
                    },
                    PropDoc {
                        name: "key".to_string(),
                        prop_type: "String".to_string(),
                        default: "-".to_string(),
                        description: "对应 activeKey".to_string(),
                    },
                    PropDoc {
                        name: "label".to_string(),
                        prop_type: "String | Element".to_string(),
                        default: "-".to_string(),
                        description: "选项卡头显示文字".to_string(),
                    },
                    PropDoc {
                        name: "icon".to_string(),
                        prop_type: "Element".to_string(),
                        default: "-".to_string(),
                        description: "选项卡头显示图标".to_string(),
                    },
                ]
            }
        }
    }
}

#![allow(non_snake_case)]
//!
//! 展示 Transfer 组件的各种用法和样式

use crate::common::*;
use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

/// Transfer 组件演示
#[component]
pub fn TransferDemo() -> Element {
    let mut target_keys = use_signal(|| vec!["a".to_string(), "b".to_string()]);
    let mut selected_keys = use_signal(|| vec!["a".to_string(), "c".to_string()]);
    let mut disabled = use_signal(|| false);
    let mut show_search = use_signal(|| false);
    let mut one_way = use_signal(|| false);

    let mock_data = vec![
        TransferItem {
            key: "a".to_string(),
            title: "content1".to_string(),
            description: Some("description of content1".to_string()),
            disabled: false,
            data: None,
        },
        TransferItem {
            key: "b".to_string(),
            title: "content2".to_string(),
            description: Some("description of content2".to_string()),
            disabled: false,
            data: None,
        },
        TransferItem {
            key: "c".to_string(),
            title: "content3".to_string(),
            description: Some("description of content3".to_string()),
            disabled: false,
            data: None,
        },
        TransferItem {
            key: "d".to_string(),
            title: "content4".to_string(),
            description: "description of content4".to_string(),
            disabled: false,
            data: None,
        },
        TransferItem {
            key: "e".to_string(),
            title: "content5".to_string(),
            description: Some("description of content5".to_string()),
            disabled: false,
            data: None,
        },
        TransferItem {
            key: "f".to_string(),
            title: "content6".to_string(),
            description: Some("description of content6".to_string()),
            disabled: false,
            data: None,
        },
    ];

    let large_data: Vec<TransferItem> = (0..20)
        .map(|i| TransferItem {
            key: i.to_string(),
            title: format!("content{}", i + 1),
            description: Some(format!("description of content{}", i + 1)),
            disabled: i % 3 < 1,
            data: None,
        })
        .collect();

    rsx! {
        div {
            class: "component-showcase",

            h1 {
                style: "margin: 0 0 16px 0; color: #262626; font-size: 28px;",
                "Transfer 穿梭框"
            }

            p {
                style: "margin: 0 0 24px 0; color: #666; font-size: 14px;",
                "双栏穿梭选择框。"
            }

            // 基础用法
            DemoSection {
                title: "基础用法",
                description: "最基本的用法，展示了 dataSource、targetKeys、每行的渲染函数 render 以及回调函数 onChange、onSelectChange、onScroll 的用法。",

                div {
                    style: "padding: 24px; background: #fafafa; border-radius: 6px;",

                    Transfer {
                        data_source: mock_data.clone(),
                        titles: vec!["Source".to_string(), "Target".to_string()],
                        target_keys: target_keys(),
                        selected_keys: selected_keys(),
                        on_change: move |next_target_keys, direction, move_keys| {
                            target_keys.set(next_target_keys);
                        },
                        on_select_change: move |source_selected_keys, target_selected_keys| {
                            selected_keys.set([source_selected_keys, target_selected_keys].concat());
                        },
                        render: |item| {
                            rsx! {
                                span {
                                    "{item.title}"
                                }
                            }
                        }
                    }
                }
            }

            // 带搜索框
            DemoSection {
                title: "带搜索框",
                description: "带搜索框的穿梭框，可以自定义搜索函数。",

                div {
                    style: "padding: 24px; background: #fafafa; border-radius: 6px;",

                    div {
                        style: "margin-bottom: 16px;",
                        Switch {
                            checked: show_search(),
                            onchange: move |checked| {
                                show_search.set(checked);
                            },
                            "Show Search"
                        }
                    }

                    Transfer {
                        data_source: mock_data.clone(),
                        show_search: show_search(),
                        filter_option: |input_value, item| {
                            item.title.contains(input_value)
                        },
                        target_keys: target_keys(),
                        on_change: move |next_target_keys, direction, move_keys| {
                            target_keys.set(next_target_keys);
                        },
                        render: |item| {
                            rsx! {
                                span {
                                    "{item.title} - {item.description}"
                                }
                            }
                        }
                    }
                }
            }

            // 高级用法
            DemoSection {
                title: "高级用法",
                description: "穿梭框高级用法，可配置操作文案，可定制宽高，可对底部进行自定义渲染。",

                div {
                    style: "padding: 24px; background: #fafafa; border-radius: 6px;",

                    Transfer {
                        data_source: large_data.clone(),
                        list_style: "width: 300px; height: 300px;",
                        operations: vec!["to right".to_string(), "to left".to_string()],
                        target_keys: target_keys(),
                        on_change: move |next_target_keys, direction, move_keys| {
                            target_keys.set(next_target_keys);
                        },
                        render: |item| {
                            rsx! {
                                span {
                                    class: "custom-item",
                                    "{item.title} - {item.description}"
                                }
                            }
                        },
                        footer: |props| {
                            rsx! {
                                Button {
                                    size: "small",
                                    style: "float: right; margin: 5px;",
                                    onclick: move |_| {
                                        // Reload data
                                    },
                                    "reload"
                                }
                                Button {
                                    size: "small",
                                    style: "float: right; margin: 5px;",
                                    onclick: move |_| {
                                        // Clear data
                                    },
                                    "clear"
                                }
                            }
                        }
                    }
                }
            }

            // 自定义渲染行数据
            DemoSection {
                title: "自定义渲染行数据",
                description: "自定义渲染每一个 Transfer Item，可用于渲染复杂数据。",

                div {
                    style: "padding: 24px; background: #fafafa; border-radius: 6px;",

                    Transfer {
                        data_source: mock_data.clone(),
                        target_keys: target_keys(),
                        on_change: move |next_target_keys, direction, move_keys| {
                            target_keys.set(next_target_keys);
                        },
                        render: |item| {
                            rsx! {
                                div {
                                    style: "display: flex; justify-content: space-between; align-items: center;",
                                    div {
                                        div {
                                            style: "font-weight: bold;",
                                            "{item.title}"
                                        }
                                        div {
                                            style: "color: #999; font-size: 12px;",
                                            "{item.description}"
                                        }
                                    }
                                    Icon {
                                        icon_type: IconType::RightOutlined
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // 分页
            DemoSection {
                title: "分页",
                description: "大数据下使用分页。",

                div {
                    style: "padding: 24px; background: #fafafa; border-radius: 6px;",

                    Transfer {
                        data_source: large_data.clone(),
                        target_keys: target_keys(),
                        pagination: true,
                        on_change: move |next_target_keys, direction, move_keys| {
                            target_keys.set(next_target_keys);
                        },
                        render: |item| {
                            rsx! {
                                span {
                                    "{item.title}"
                                }
                            }
                        }
                    }
                }
            }

            // 单向样式
            DemoSection {
                title: "单向样式",
                description: "通过 oneWay 开启单向样式，通过 operations 设置操作文案。",

                div {
                    style: "padding: 24px; background: #fafafa; border-radius: 6px;",

                    div {
                        style: "margin-bottom: 16px;",
                        Switch {
                            checked: one_way(),
                            onchange: move |checked| {
                                one_way.set(checked);
                            },
                            "One Way"
                        }
                    }

                    Transfer {
                        data_source: mock_data.clone(),
                        target_keys: target_keys(),
                        one_way: one_way(),
                        operations: if one_way() {
                            vec!["to right".to_string()]
                        } else {
                            vec!["to right".to_string(), "to left".to_string()]
                        },
                        on_change: move |next_target_keys, direction, move_keys| {
                            target_keys.set(next_target_keys);
                        },
                        render: |item| {
                            rsx! {
                                span {
                                    "{item.title}"
                                }
                            }
                        }
                    }
                }
            }

            // 表格穿梭框
            DemoSection {
                title: "表格穿梭框",
                description: "使用 Table 组件作为自定义渲染列表。",

                div {
                    style: "padding: 24px; background: #fafafa; border-radius: 6px;",

                    Transfer {
                        data_source: mock_data.clone(),
                        target_keys: target_keys(),
                        on_change: move |next_target_keys, direction, move_keys| {
                            target_keys.set(next_target_keys);
                        },
                        children: |props| {
                            rsx! {
                                Table {
                                    row_selection: TableRowSelection {
                                        checkbox_props: |record| {
                                            CheckboxProps {
                                                disabled: record.disabled,
                                                name: record.title.clone(),
                                            }
                                        },
                                        on_select: props.on_item_select,
                                        on_select_all: props.on_item_select_all,
                                        selected_row_keys: props.selected_keys,
                                    },
                                    columns: vec![
                                        TableColumn {
                                            title: "Name".to_string(),
                                            dataindex: "title".to_string(),
                                            key: "title".to_string(),
                                        },
                                        TableColumn {
                                            title: "Description".to_string(),
                                            dataindex: "description".to_string(),
                                            key: "description".to_string(),
                                        },
                                    ],
                                    data_source: props.data_source,
                                    size: "small",
                                    style: "pointer-events: none;",
                                    on_row_click: |record| {
                                        TableRowProps {
                                            onclick: move |_| {
                                                if !record.disabled {
                                                    props.on_item_select(record.key.clone(), !props.selected_keys.contains(&record.key));
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // 树穿梭框
            DemoSection {
                title: "树穿梭框",
                description: "使用 Tree 组件作为自定义渲染列表。",

                div {
                    style: "padding: 24px; background: #fafafa; border-radius: 6px;",

                    Transfer {
                        data_source: mock_data.clone(),
                        target_keys: target_keys(),
                        on_change: move |next_target_keys, direction, move_keys| {
                            target_keys.set(next_target_keys);
                        },
                        children: |props| {
                            rsx! {
                                Tree {
                                    tree_data: props.data_source.iter().map(|item| {
                                        TreeNode {
                                            title: item.title.clone(),
                                            key: item.key.clone(),
                                            disabled: item.disabled,
                                        }
                                    }).collect(),
                                    checkable: true,
                                    default_checked_keys: props.selected_keys,
                                    on_check: props.on_item_select_all,
                                }
                            }
                        }
                    }
                }
            }

            // API 文档
            ApiDocumentation {
                component_name: "Transfer",
                props: vec![
                    PropDoc {
                        name: "data_source".to_string(),
                        prop_type: "Vec<TransferItem>".to_string(),
                        default: "[]".to_string(),
                        description: "数据源，其中的数据将会被渲染到左边一栏中，targetKeys 中指定的除外。".to_string(),
                    },
                    PropDoc {
                        name: "disabled".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "是否禁用".to_string(),
                    },
                    PropDoc {
                        name: "filter_option".to_string(),
                        prop_type: "Function".to_string(),
                        default: "-".to_string(),
                        description: "接收 inputValue option 两个参数，当 option 符合筛选条件时，应返回 true，反之则返回 false。".to_string(),
                    },
                    PropDoc {
                        name: "footer".to_string(),
                        prop_type: "Function".to_string(),
                        default: "-".to_string(),
                        description: "底部渲染函数".to_string(),
                    },
                    PropDoc {
                        name: "list_style".to_string(),
                        prop_type: "Object | Function".to_string(),
                        default: "-".to_string(),
                        description: "两个穿梭框的自定义样式".to_string(),
                    },
                    PropDoc {
                        name: "locale".to_string(),
                        prop_type: "Object".to_string(),
                        default: "{ itemUnit: 'item', itemsUnit: 'items', searchPlaceholder: 'Search here', notFoundContent: 'Not Found' }".to_string(),
                        description: "各种语言".to_string(),
                    },
                    PropDoc {
                        name: "one_way".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "展示为单向样式".to_string(),
                    },
                    PropDoc {
                        name: "operations".to_string(),
                        prop_type: "Vec<String>".to_string(),
                        default: "['>', '<']".to_string(),
                        description: "操作文案集合，顺序从上至下".to_string(),
                    },
                    PropDoc {
                        name: "pagination".to_string(),
                        prop_type: "bool | Object".to_string(),
                        default: "false".to_string(),
                        description: "使用分页样式，自定义渲染列表下无效".to_string(),
                    },
                    PropDoc {
                        name: "render".to_string(),
                        prop_type: "Function".to_string(),
                        default: "-".to_string(),
                        description: "每行数据渲染函数，该函数的入参为 dataSource 中的项，返回值为 ReactElement。或者返回一个普通对象，其中 label 字段为 ReactElement，value 字段为 title".to_string(),
                    },
                    PropDoc {
                        name: "selected_keys".to_string(),
                        prop_type: "Vec<String>".to_string(),
                        default: "[]".to_string(),
                        description: "设置哪些项应该被选中".to_string(),
                    },
                    PropDoc {
                        name: "show_search".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "是否显示搜索框".to_string(),
                    },
                    PropDoc {
                        name: "show_select_all".to_string(),
                        prop_type: "bool".to_string(),
                        default: "true".to_string(),
                        description: "是否展示全选勾选框".to_string(),
                    },
                    PropDoc {
                        name: "target_keys".to_string(),
                        prop_type: "Vec<String>".to_string(),
                        default: "[]".to_string(),
                        description: "显示在右侧框数据的 key 集合".to_string(),
                    },
                    PropDoc {
                        name: "titles".to_string(),
                        prop_type: "Vec<String>".to_string(),
                        default: "['', '']".to_string(),
                        description: "标题集合，顺序从左至右".to_string(),
                    },
                    PropDoc {
                        name: "on_change".to_string(),
                        prop_type: "Function".to_string(),
                        default: "-".to_string(),
                        description: "选项在两栏之间转移时的回调函数".to_string(),
                    },
                    PropDoc {
                        name: "on_scroll".to_string(),
                        prop_type: "Function".to_string(),
                        default: "-".to_string(),
                        description: "选项列表滚动时的回调函数".to_string(),
                    },
                    PropDoc {
                        name: "on_search".to_string(),
                        prop_type: "Function".to_string(),
                        default: "-".to_string(),
                        description: "搜索框内容时改变时的回调函数".to_string(),
                    },
                    PropDoc {
                        name: "on_select_change".to_string(),
                        prop_type: "Function".to_string(),
                        default: "-".to_string(),
                        description: "选中项发生改变时的回调函数".to_string(),
                    },
                ]
            }

            // Render Props
            ApiDocumentation {
                component_name: "Render Props",
                props: vec![
                    PropDoc {
                        name: "direction".to_string(),
                        prop_type: "String".to_string(),
                        default: "-".to_string(),
                        description: "渲染列表的方向 left | right".to_string(),
                    },
                    PropDoc {
                        name: "disabled".to_string(),
                        prop_type: "bool".to_string(),
                        default: "-".to_string(),
                        description: "是否禁用列表".to_string(),
                    },
                    PropDoc {
                        name: "filtered_items".to_string(),
                        prop_type: "Vec<TransferItem>".to_string(),
                        default: "-".to_string(),
                        description: "过滤后的数据".to_string(),
                    },
                    PropDoc {
                        name: "on_item_select".to_string(),
                        prop_type: "Function".to_string(),
                        default: "-".to_string(),
                        description: "勾选条目".to_string(),
                    },
                    PropDoc {
                        name: "on_item_select_all".to_string(),
                        prop_type: "Function".to_string(),
                        default: "-".to_string(),
                        description: "勾选一组条目".to_string(),
                    },
                    PropDoc {
                        name: "selected_keys".to_string(),
                        prop_type: "Vec<String>".to_string(),
                        default: "-".to_string(),
                        description: "选中的条目".to_string(),
                    },
                ]
            }
        }
    }
}

//!
//!
//! 展示 Table 组件的各种用法和样式

use crate::common::*;
use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Debug)]
struct UserData {
    key: String,
    name: String,
    age: u32,
    address: String,
    tags: Vec<String>,
}

/// Table 组件演示
#[component]
pub fn TableDemo() -> Element {
    let data = use_signal(|| {
        vec![
            UserData {
                key: "1".to_string(),
                name: "John Brown".to_string(),
                age: 32,
                address: "New York No. 1 Lake Park".to_string(),
                tags: vec!["nice".to_string(), "developer".to_string()],
            },
            UserData {
                key: "2".to_string(),
                name: "Jim Green".to_string(),
                age: 42,
                address: "London No. 1 Lake Park".to_string(),
                tags: vec!["loser".to_string()],
            },
            UserData {
                key: "3".to_string(),
                name: "Joe Black".to_string(),
                age: 32,
                address: "Sidney No. 1 Lake Park".to_string(),
                tags: vec!["cool".to_string(), "teacher".to_string()],
            },
        ]
    });

    let columns = vec![
        TableColumn {
            title: "Name".to_string(),
            dataindex: "name".to_string(),
            key: "name".to_string(),
            width: Some("150px".to_string()),
            align: Some(TableAlign::Left),
            sortable: true,
            fixed: None,
            show_sorter_tooltip: true,
        },
        TableColumn {
            title: "Age".to_string(),
            dataindex: "age".to_string(),
            key: "age".to_string(),
            width: Some("80px".to_string()),
            align: Some(TableAlign::Center),
            sortable: true,
            fixed: None,
            show_sorter_tooltip: true,
        },
        TableColumn {
            title: "Address".to_string(),
            dataindex: "address".to_string(),
            key: "address".to_string(),
            width: None,
            align: Some(TableAlign::Left),
            sortable: false,
            fixed: None,
            show_sorter_tooltip: true,
        },
        TableColumn {
            title: "Tags".to_string(),
            dataindex: "tags".to_string(),
            key: "tags".to_string(),
            width: Some("200px".to_string()),
            align: Some(TableAlign::Center),
            sortable: false,
            fixed: None,
            show_sorter_tooltip: true,
        },
        TableColumn {
            title: "Action".to_string(),
            dataindex: "action".to_string(),
            key: "action".to_string(),
            width: Some("150px".to_string()),
            align: Some(TableAlign::Center),
            sortable: false,
            fixed: None,
            show_sorter_tooltip: true,
        },
    ];

    let selected_rows = use_signal(|| Vec::<String>::new());
    let current_page = use_signal(|| 1);
    let page_size = use_signal(|| 10);

    rsx! {
        div { class: "table-demo",
            h1 { "Table 表格" }
            p { "展示行列数据的表格组件。" }

            DemoSection {
                title: "基础表格",
                description: "简单的表格，最后一列是各种操作。",

                div {
                    Table {
                        columns: columns.clone(),
                        data_source: data.read().clone(),
                        size: TableSize::Default,
                        loading: false,
                        scroll_x: false,
                        scroll_y: false,
                        show_header: true,
                        on_row_click: move |record: UserData| {
                            println!("Row clicked: {:?}", record);
                        },
                        on_sort_change: move |column: String, direction: String| {
                            println!("Sort changed: {} {}", column, direction);
                        },
                        row_selection: None,
                        pagination: None,
                    }
                }
            }


            DemoSection {
                title: "可选择",
                description: "第一列是联动的选择框。",

                div {
                    p { "已选择 {selected_rows.read().len()} 项" }
                    Table {
                        columns: columns.clone(),
                        data_source: data.read().clone(),
                        size: TableSize::Default,
                        loading: false,
                        scroll_x: false,
                        scroll_y: false,
                        show_header: true,
                        on_row_click: move |record: UserData| {
                            println!("Row clicked: {:?}", record);
                        },
                        on_sort_change: move |column: String, direction: String| {
                            println!("Sort changed: {} {}", column, direction);
                        },
                        row_selection: Some(TableRowSelection {
                            selected_row_keys: selected_rows.read().clone(),
                            on_select: move |key: String, selected: bool| {
                                let mut keys = selected_rows.read().clone();
                                if selected {
                                    if !keys.contains(&key) {
                                        keys.push(key);
                                    }
                                } else {
                                    keys.retain(|k| k != &key);
                                }
                                selected_rows.set(keys);
                            },
                            on_select_all: move |selected: bool| {
                                if selected {
                                    let all_keys: Vec<String> = data.read().iter().map(|item| item.key.clone()).collect();
                                    selected_rows.set(all_keys);
                                } else {
                                    selected_rows.set(Vec::new());
                                }
                            },
                            get_checkbox_props: None,
                        }),
                        pagination: None,
                    }
                }
            }

            DemoSection {
                title: "带分页的表格",
                description: "表格支持分页功能。",

                div {
                    Table {
                        columns: columns.clone(),
                        // data_source: data.read().clone(),
                        size: TableSize::Default,
                        striped: false,
                        loading: false,
                        scroll_x: None,
                        scroll_y: None,
                        sticky_header: false,
                        show_header: true,
                        table_layout: "auto".to_string(),
                        empty_text: "暂无数据".to_string(),
                        on_row_click: move |record: UserData| {
                            println!("Row clicked: {:?}", record);
                        },
                        on_sort_change: move |column: String, direction: String| {
                            println!("Sort changed: {} {}", column, direction);
                        },
                        row_selection: None,
                        pagination: Some(TablePagination {
                            current: current_page.read().clone(),
                            page_size: page_size.read().clone(),
                            total: 50,
                            show_size_changer: true,
                            show_quick_jumper: true,
                            show_total: true,
                            on_change: move |page: u32, size: u32| {
                                current_page.set(page);
                                page_size.set(size);
                            },
                        }),
                    }
                }
            }

            DemoSection {
                title: "紧凑型表格",
                description: "两种紧凑型的表格。",

                div {
                    Space {
                        direction: SpaceDirection::Vertical,
                        size: SpaceSize::Large,
                        h4 { "Middle size" }
                        Table {
                            columns: columns.clone(),
                            // data_source: data.read().clone(),
                            size: TableSize::Middle,
                            hover: true,
                            striped: false,
                            loading: false,
                            scroll_x: None,
                            scroll_y: None,
                            sticky_header: false,
                            show_header: true,
                            table_layout: "auto".to_string(),
                            empty_text: "暂无数据".to_string(),
                            on_row_click: move |record: UserData| {},
                            on_sort_change: move |column: String, direction: String| {},
                            row_selection: None,
                            pagination: None,
                        }
                        h4 { "Small size" }
                        Table {
                            columns: columns.clone(),
                            // data_source: data.read().clone(),
                            size: TableSize::Small,
                            hover: true,
                            striped: false,
                            loading: false,
                            scroll_x: None,
                            scroll_y: None,
                            sticky_header: false,
                            show_header: true,
                            table_layout: "auto".to_string(),
                            empty_text: "暂无数据".to_string(),
                            on_row_click: move |record: UserData| {},
                            on_sort_change: move |column: String, direction: String| {},
                            row_selection: None,
                            pagination: None,
                        }
                    }
                }
            }
        }
    }
}

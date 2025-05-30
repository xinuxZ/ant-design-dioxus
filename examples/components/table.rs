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

/// 将UserData转换为HashMap<String, String>
fn user_data_to_hashmap(user: &UserData) -> HashMap<String, String> {
    let mut map = HashMap::new();
    map.insert("key".to_string(), user.key.clone());
    map.insert("name".to_string(), user.name.clone());
    map.insert("age".to_string(), user.age.to_string());
    map.insert("address".to_string(), user.address.clone());
    map.insert("tags".to_string(), user.tags.join(", "));
    map
}

/// Table 组件演示
#[component]
pub fn TableDemo() -> Element {
    let user_data = use_signal(|| {
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

    // 将UserData转换为HashMap<String, String>
    let data = use_memo(move || {
        user_data
            .read()
            .iter()
            .map(|item| user_data_to_hashmap(item))
            .collect::<Vec<_>>()
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
    let selected_keys = use_signal(|| Vec::<String>::new());

    // 获取所有用户的key
    let all_keys = use_memo(move || {
        user_data
            .read()
            .iter()
            .map(|item| item.key.clone())
            .collect::<Vec<_>>()
    });

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
                        // 暂时注释掉on_row_click以解决类型不匹配问题
                        // on_row_click: Some(|key| {
                        //     println!("Row clicked: {}", key);
                        // }),
                        // 暂时注释掉on_sort_change以解决类型不匹配问题
                        // on_sort_change: Some(move |(column, order): (String, String)| {
                        //     println!("Sort changed: {} {}", column, order);
                        // }),
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
                        // 暂时注释掉on_row_click以解决类型不匹配问题
                        // on_row_click: Some(|key| {
                        //     println!("Row clicked: {}", key);
                        // }),
                        // 暂时注释掉on_sort_change以解决类型不匹配问题
                        // on_sort_change: Some(move |(column, order): (String, String)| {
                        //     println!("Sort changed: {} {}", column, order);
                        // }),
                        row_selection: Some(TableRowSelection {
                            selected_row_keys: selected_rows.read().clone(),
                            checkbox_props: None,
                            column_width: None,
                            column_title: None,
                            fixed: false,
                            selection_type: SelectionType::Checkbox,
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
                        data_source: data.read().clone(),
                        size: TableSize::Default,
                        loading: false,
                        scroll_x: false,
                        scroll_y: false,
                        show_header: true,
                        // 暂时注释掉on_row_click以解决类型不匹配问题
                        // on_row_click: Some(|key| {
                        //     println!("Row clicked: {}", key);
                        // }),
                        // 暂时注释掉on_sort_change以解决类型不匹配问题
                        // on_sort_change: Some(move |(column, order): (String, String)| {
                        //     println!("Sort changed: {} {}", column, order);
                        // }),
                        row_selection: None,
                        pagination: Some(TablePagination {
                            current: current_page.read().clone(),
                            page_size: page_size.read().clone(),
                            total: 50,
                            show_size_changer: true,
                            show_quick_jumper: true,
                            page_size_options: vec!["10条".to_string(), "20条".to_string()],
                            // on_change: move |page: u32, size: u32| {
                            //     current_page.set(page.try_into().unwrap());
                            //     page_size.set(size);
                            // },
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
                            data_source: data.read().clone(),
                            size: TableSize::Middle,
                            loading: false,
                            scroll_x: false,
                            scroll_y: false,
                            show_header: true,
                            empty_text: "暂无数据".to_string(),
                            on_row_click: move |record: UserData| {},
                            on_sort_change: move |column: String, direction: String| {},
                            row_selection: None,
                            pagination: None,
                        }
                        h4 { "Small size" }
                        Table {
                            columns: columns.clone(),
                            data_source: data.read().clone(),
                            size: TableSize::Small,
                            loading: false,
                            scroll_x: false,
                            scroll_y: false,
                            show_header: true,
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

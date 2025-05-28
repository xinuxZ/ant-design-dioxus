//! Table 表格组件
//!
//! 展示行列数据的表格组件。
//!
//! ## 何时使用
//!
//! - 当有大量结构化的数据需要展现时；
//! - 当需要对数据进行排序、搜索、分页、自定义操作等复杂行为时。
//!
//! ## 代码演示
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::Table;
//!
//! #[component]
//! fn App() -> Element {
//!     let columns = vec![
//!         TableColumn {
//!             title: "姓名".to_string(),
//!             dataindex: "name".to_string(),
//!             key: "name".to_string(),
//!             ..Default::default()
//!         },
//!         TableColumn {
//!             title: "年龄".to_string(),
//!             dataindex: "age".to_string(),
//!             key: "age".to_string(),
//!             ..Default::default()
//!         },
//!     ];
//!
//!     rsx! {
//!         Table {
//!             columns: columns,
//!             data_source: vec![],
//!         }
//!     }
//! }
//! ```

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 表格列配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TableColumn {
    /// 列标题
    pub title: String,
    /// 列数据在数据项中对应的路径
    pub dataindex: String,
    /// React 需要的 key，如果已经设置了唯一的 dataIndex，可以忽略这个属性
    pub key: String,
    /// 列宽度
    pub width: Option<String>,
    /// 设置列的对齐方式
    pub align: Option<TableAlign>,
    /// 是否支持排序
    pub sortable: bool,
    /// 是否固定列
    pub fixed: Option<TableFixed>,
    /// 表头是否显示下一次排序的 tooltip 提示
    pub show_sorter_tooltip: bool,
}

impl Default for TableColumn {
    fn default() -> Self {
        Self {
            title: String::new(),
            dataindex: String::new(),
            key: String::new(),
            width: None,
            align: None,
            sortable: false,
            fixed: None,
            show_sorter_tooltip: true,
        }
    }
}

/// 表格对齐方式
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TableAlign {
    Left,
    Center,
    Right,
}

/// 表格固定列
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TableFixed {
    Left,
    Right,
}

/// 表格尺寸
#[derive(Debug, Clone, PartialEq)]
pub enum TableSize {
    Default,
    Middle,
    Small,
}

/// 表格行选择配置
#[derive(Debug, Clone, PartialEq)]
pub struct TableRowSelection {
    /// 选择框的默认属性配置
    pub checkbox_props: Option<HashMap<String, String>>,
    /// 自定义选择项参考
    pub column_width: Option<String>,
    /// 选择框列的标题
    pub column_title: Option<String>,
    /// 固定选择列
    pub fixed: bool,
    /// 指定选中项的 key 数组
    pub selected_row_keys: Vec<String>,
    /// 选择框的类型
    pub selection_type: SelectionType,
}

/// 选择框类型
#[derive(Debug, Clone, PartialEq)]
pub enum SelectionType {
    Checkbox,
    Radio,
}

/// 表格分页配置
#[derive(Debug, Clone, PartialEq)]
pub struct TablePagination {
    /// 当前页数
    pub current: usize,
    /// 每页条数
    pub page_size: usize,
    /// 数据总数
    pub total: usize,
    /// 是否显示快速跳转至某页
    pub show_quick_jumper: bool,
    /// 是否展示 pageSize 切换器
    pub show_size_changer: bool,
    /// 指定每页可以显示多少条
    pub page_size_options: Vec<String>,
}

/// Table 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct TableProps {
    /// 表格列的配置描述
    pub columns: Vec<TableColumn>,
    /// 数据数组
    pub data_source: Vec<HashMap<String, String>>,
    /// 表格是否可滚动
    #[props(default = false)]
    pub scroll_x: bool,
    /// 表格是否可滚动
    #[props(default = false)]
    pub scroll_y: bool,
    /// 表格大小
    #[props(default = TableSize::Default)]
    pub size: TableSize,
    /// 表格标题
    pub title: Option<String>,
    /// 表格尾部
    pub footer: Option<String>,
    /// 是否显示表头
    #[props(default = true)]
    pub show_header: bool,
    /// 表格行是否可选择
    pub row_selection: Option<TableRowSelection>,
    /// 分页器，参考配置项或 pagination 文档
    pub pagination: Option<TablePagination>,
    /// 页面是否加载中
    #[props(default = false)]
    pub loading: bool,
    /// 表格行 key 的取值
    #[props(default = "key".to_string())]
    pub row_key: String,
    /// 设置表格的总体样式
    #[props(default = String::new())]
    pub class: String,
    /// 设置表格的内联样式
    #[props(default = String::new())]
    pub style: String,
    /// 行点击事件回调
    pub on_row_click: Option<EventHandler<String>>,
    /// 列排序事件回调
    pub on_sort_change: Option<EventHandler<(String, String)>>,
}

/// Table 表格组件
///
/// 展示行列数据的表格组件。
#[component]
pub fn Table(props: TableProps) -> Element {
    let table_class = format!(
        "ant-table ant-table-{} {}",
        match props.size {
            TableSize::Default => "default",
            TableSize::Middle => "middle",
            TableSize::Small => "small",
        },
        props.class
    );

    let scroll_class = if props.scroll_x || props.scroll_y {
        " ant-table-scroll"
    } else {
        ""
    };

    rsx! {
        div {
            class: "ant-table-wrapper",
            style: props.style.clone(),

            // 表格标题
            if let Some(title) = &props.title {
                div {
                    class: "ant-table-title",
                    {title.clone()}
                }
            }

            // 表格容器
            div {
                class: "ant-table-container",

                // 表格内容
                div {
                    class: format!("{table_class}{scroll_class}"),

                    // 表格主体
                    table {
                        class: "ant-table-table",

                        // 表头
                        if props.show_header {
                            thead {
                                class: "ant-table-thead",
                                tr {
                                    class: "ant-table-row",
                                    key: index.to_string(),

                                    // 行选择列
                                    if props.row_selection.is_some() {
                                        th {
                                            class: "ant-table-cell ant-table-selection-column",
                                            input {
                                                r#type: "checkbox",
                                                class: "ant-checkbox-input",
                                            }
                                        }
                                    }

                                    // 数据列
                                    for column in &props.columns {
                                        th {
                                            class: format!(
                                                "ant-table-cell {}",
                                                if let Some(align) = &column.align {
                                                    match align {
                                                        TableAlign::Left => "ant-table-cell-left",
                                                        TableAlign::Center => "ant-table-cell-center",
                                                        TableAlign::Right => "ant-table-cell-right",
                                                    }
                                                } else {
                                                    ""
                                                }
                                            ),
                                            key: column.key.clone(),
                                            style: if let Some(width) = &column.width {
                                                format!("width: {}", width)
                                            } else {
                                                String::new()
                                            },

                                            div {
                                                class: "ant-table-column-title",
                                                {column.title.clone()}
                                            }

                                            // 排序图标
                                            if column.sortable {
                                                div {
                                                    class: "ant-table-column-sorters",
                                                    span {
                                                        class: "ant-table-column-sorter",
                                                        span {
                                                            class: "ant-table-column-sorter-up",
                                                            "▲"
                                                        }
                                                        span {
                                                            class: "ant-table-column-sorter-down",
                                                            "▼"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // 表格主体
                        tbody {
                            class: "ant-table-tbody",

                            // 数据行
                            for (_index, row) in props.data_source.iter().enumerate() {
                                tr {
                                    class: "ant-table-row",
                                    key: index.to_string(),
                                    onclick: {
                                         let handler = props.on_row_click.clone();
                                         let key_value = row.get(&props.row_key).cloned();
                                         move |_| {
                                             if let Some(handler) = &handler {
                                                 if let Some(key) = &key_value {
                                                     handler.call(key.clone());
                                                 }
                                             }
                                         }
                                     },

                                    // 行选择列
                                    if props.row_selection.is_some() {
                                        td {
                                            class: "ant-table-cell ant-table-selection-column",
                                            input {
                                                r#type: "checkbox",
                                                class: "ant-checkbox-input",
                                            }
                                        }
                                    }

                                    // 数据列
                                    for column in &props.columns {
                                        td {
                                            class: format!(
                                                "ant-table-cell {}",
                                                if let Some(align) = &column.align {
                                                    match align {
                                                        TableAlign::Left => "ant-table-cell-left",
                                                        TableAlign::Center => "ant-table-cell-center",
                                                        TableAlign::Right => "ant-table-cell-right",
                                                    }
                                                } else {
                                                    ""
                                                }
                                            ),
                                            key: column.key.clone(),

                                            {row.get(&column.dataindex).cloned().unwrap_or_default()}
                                        }
                                    }
                                }
                            }

                            // 空数据提示
                            if props.data_source.is_empty() {
                                tr {
                                    class: "ant-table-placeholder",
                                    td {
                                        class: "ant-table-cell",
                                        colspan: format!("{}", props.columns.len() + if props.row_selection.is_some() { 1 } else { 0 }),
                                        div {
                                            class: "ant-empty ant-empty-normal",
                                            div {
                                                class: "ant-empty-image",
                                                "📄"
                                            }
                                            div {
                                                class: "ant-empty-description",
                                                "暂无数据"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // 表格尾部
            if let Some(footer) = &props.footer {
                div {
                    class: "ant-table-footer",
                    {footer.clone()}
                }
            }

            // 分页器
            if let Some(pagination) = &props.pagination {
                div {
                    class: "ant-table-pagination ant-pagination",
                    div {
                        class: "ant-pagination-total-text",
                        {format!("共 {} 条", pagination.total)}
                    }
                    ul {
                        class: "ant-pagination-list",
                        li {
                            class: "ant-pagination-prev",
                            "‹"
                        }
                        li {
                            class: "ant-pagination-item ant-pagination-item-active",
                            {pagination.current.to_string()}
                        }
                        li {
                            class: "ant-pagination-next",
                            "›"
                        }
                    }
                }
            }
        }
    }
}

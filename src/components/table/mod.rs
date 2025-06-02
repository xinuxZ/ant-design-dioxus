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

use css_in_rust_macros::css;
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

// CSS-in-Rust 辅助函数

/// 获取表格包装器样式
fn get_table_wrapper_css() -> String {
    css!(
        r#"
        box-sizing: border-box;
        margin: 0;
        padding: 0;
        color: var(--ant-text-color);
        font-size: var(--ant-font-size-base);
        line-height: var(--ant-line-height-base);
        list-style: none;
        font-family: inherit;
        position: relative;
        clear: both;
        max-width: 100%;
        background: var(--ant-bg-color-container);
        border-radius: var(--ant-border-radius);
    "#
    )
}

/// 获取表格基础样式
fn get_table_base_css() -> String {
    css!(
        r#"
        box-sizing: border-box;
        margin: 0;
        padding: 0;
        color: var(--ant-text-color);
        font-size: var(--ant-font-size-base);
        line-height: var(--ant-line-height-base);
        list-style: none;
        font-family: inherit;
        position: relative;
        z-index: 0;
        clear: both;
        font-size: var(--ant-font-size-base);
        background: var(--ant-bg-color-container);
        border-radius: var(--ant-border-radius-lg) var(--ant-border-radius-lg) 0 0;
    "#
    )
}

/// 获取表格滚动样式
fn get_table_scroll_css() -> String {
    css!(
        r#"
        overflow: auto;

        .ant-table-table {
            min-width: 100%;
        }
    "#
    )
}

/// 获取表格标题样式
fn get_table_title_css() -> String {
    css!(
        r#"
        padding: 16px 16px 0 16px;
        position: relative;
        top: 1px;
        border-radius: var(--ant-border-radius-lg) var(--ant-border-radius-lg) 0 0;
    "#
    )
}

/// 获取表格容器样式
fn get_table_container_css() -> String {
    css!(
        r#"
        position: relative;
    "#
    )
}

/// 获取表格table样式
fn get_table_table_css() -> String {
    css!(
        r#"
        width: 100%;
        text-align: left;
        border-radius: 0 0 var(--ant-border-radius-lg) var(--ant-border-radius-lg);
        border-collapse: separate;
        border-spacing: 0;
    "#
    )
}

/// 获取表格表头样式
fn get_table_thead_css() -> String {
    css!(
        r#"

    "#
    )
}

/// 获取表格行样式
fn get_table_row_css() -> String {
    css!(
        r#"

        &:hover > td {
            background: var(--ant-bg-color-hover);
        }
    "#
    )
}

/// 获取表格单元格样式
fn get_table_cell_css() -> String {
    css!(
        r#"
        position: relative;
        padding: 16px;
        overflow-wrap: break-word;
        border-bottom: 1px solid var(--ant-border-color-split);
        transition: background 0.3s;

        &:last-child {
            border-right: 0;
        }
    "#
    )
}

/// 获取表格选择列样式
fn get_table_selection_column_css() -> String {
    css!(
        r#"
        width: 32px;
        min-width: 32px;
        text-align: center;
    "#
    )
}

/// 获取复选框输入样式
fn get_checkbox_input_css() -> String {
    css!(
        r#"
        position: absolute;
        top: 50%;
        left: 50%;
        z-index: 1;
        width: 16px;
        height: 16px;
        margin-top: -8px;
        margin-left: -8px;
        cursor: pointer;
        opacity: 0;
    "#
    )
}

/// 获取表格列标题样式
fn get_table_column_title_css() -> String {
    css!(
        r#"
        position: relative;
        z-index: 1;
        flex: 1 1 auto;
    "#
    )
}

/// 获取表格列排序器样式
fn get_table_column_sorters_css() -> String {
    css!(
        r#"
        display: flex;
        flex: auto;
        align-items: center;
        justify-content: space-between;
        cursor: pointer;

        &:hover {
            background: var(--ant-bg-color-hover);
        }
    "#
    )
}

/// 获取表格列排序器样式
fn get_table_column_sorter_css() -> String {
    css!(
        r#"
        margin-left: 4px;
        color: var(--ant-text-color-tertiary);
        font-size: 0;
        transition: all 0.3s;
    "#
    )
}

/// 获取表格列排序器向上样式
fn get_table_column_sorter_up_css() -> String {
    css!(
        r#"
        height: 0;
        line-height: 0;

        &:hover {
            color: var(--ant-text-color-secondary);
        }
    "#
    )
}

/// 获取表格列排序器向下样式
fn get_table_column_sorter_down_css() -> String {
    css!(
        r#"
        height: 0;
        line-height: 0;

        &:hover {
            color: var(--ant-text-color-secondary);
        }
    "#
    )
}

/// 获取表格主体样式
fn get_table_tbody_css() -> String {
    css!(
        r#"

    "#
    )
}

/// 获取表格占位符样式
fn get_table_placeholder_css() -> String {
    css!(
        r#"

        &:hover > td {
            background: var(--ant-bg-color-container);
        }
    "#
    )
}

/// 获取空状态样式
fn get_empty_css() -> String {
    css!(
        r#"
        margin: 32px 0;
        color: var(--ant-text-color-disabled);
        font-size: var(--ant-font-size-base);
        line-height: var(--ant-line-height-base);
        text-align: center;
    "#
    )
}

/// 获取空状态图片样式
fn get_empty_image_css() -> String {
    css!(
        r#"
        height: 40px;
        margin-bottom: 8px;
        opacity: 1;
    "#
    )
}

/// 获取空状态描述样式
fn get_empty_description_css() -> String {
    css!(
        r#"
        color: var(--ant-text-color-disabled);
    "#
    )
}

/// 获取表格尾部样式
fn get_table_footer_css() -> String {
    css!(
        r#"
        padding: 16px;
        color: var(--ant-text-color);
        background: var(--ant-bg-color-container);
    "#
    )
}

/// 获取表格分页样式
fn get_table_pagination_css() -> String {
    css!(
        r#"
        display: flex;
        flex-wrap: wrap;
        row-gap: 8px;
        justify-content: flex-end;
        margin: 16px 0;
    "#
    )
}

/// 获取分页样式
fn get_pagination_css() -> String {
    css!(
        r#"
        box-sizing: border-box;
        margin: 0;
        padding: 0;
        color: var(--ant-text-color);
        font-size: var(--ant-font-size-base);
        line-height: var(--ant-line-height-base);
        list-style: none;
        font-family: inherit;
        display: flex;
        flex-wrap: wrap;
        align-items: center;
    "#
    )
}

/// 获取分页总数文本样式
fn get_pagination_total_text_css() -> String {
    css!(
        r#"
        display: inline-block;
        margin-right: 8px;
        color: var(--ant-text-color);
        font-size: var(--ant-font-size-base);
        line-height: var(--ant-line-height-base);
    "#
    )
}

/// 获取分页列表样式
fn get_pagination_list_css() -> String {
    css!(
        r#"
        display: flex;
        align-items: center;
        margin: 0;
        padding: 0;
        list-style: none;
    "#
    )
}

/// 获取分页项样式
fn get_pagination_item_css() -> String {
    css!(
        r#"
        display: inline-block;
        min-width: 32px;
        height: 32px;
        margin-right: 8px;
        font-family: inherit;
        line-height: 30px;
        text-align: center;
        vertical-align: middle;
        list-style: none;
        background-color: var(--ant-bg-color-container);
        border: 1px solid var(--ant-border-color);
        border-radius: var(--ant-border-radius);
        outline: 0;
        cursor: pointer;
        user-select: none;

        &:hover {
            border-color: var(--ant-primary-color);
            color: var(--ant-primary-color);
        }
    "#
    )
}

/// 获取分页激活项样式
fn get_pagination_item_active_css() -> String {
    css!(
        r#"
        font-weight: 600;
        background: var(--ant-primary-color);
        border-color: var(--ant-primary-color);
        color: var(--ant-text-color-inverse);

        &:hover {
            background: var(--ant-primary-color-hover);
            border-color: var(--ant-primary-color-hover);
            color: var(--ant-text-color-inverse);
        }
    "#
    )
}

/// 获取分页上一页样式
fn get_pagination_prev_css() -> String {
    get_pagination_item_css()
}

/// 获取分页下一页样式
fn get_pagination_next_css() -> String {
    get_pagination_item_css()
}

/// 表格对齐方式
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TableAlign {
    Left,
    Center,
    Right,
}

impl TableAlign {
    pub fn to_css(&self) -> String {
        match self {
            TableAlign::Left => css!(
                r#"
                text-align: left;
            "#
            ),
            TableAlign::Center => css!(
                r#"
                text-align: center;
            "#
            ),
            TableAlign::Right => css!(
                r#"
                text-align: right;
            "#
            ),
        }
    }
}

// CSS-in-Rust 辅助函数

/// 获取表格包装器样式
fn get_table_wrapper_css() -> String {
    css!(
        r#"
        box-sizing: border-box;
        margin: 0;
        padding: 0;
        color: var(--ant-text-color);
        font-size: var(--ant-font-size-base);
        line-height: var(--ant-line-height-base);
        list-style: none;
        font-family: inherit;
        position: relative;
        clear: both;
        max-width: 100%;
        background: var(--ant-bg-color-container);
        border-radius: var(--ant-border-radius);
    "#
    )
}

/// 获取表格基础样式
fn get_table_base_css() -> String {
    css!(
        r#"
        box-sizing: border-box;
        margin: 0;
        padding: 0;
        color: var(--ant-text-color);
        font-size: var(--ant-font-size-base);
        line-height: var(--ant-line-height-base);
        list-style: none;
        font-family: inherit;
        position: relative;
        z-index: 0;
        clear: both;
        font-size: var(--ant-font-size-base);
        background: var(--ant-bg-color-container);
        border-radius: var(--ant-border-radius-lg) var(--ant-border-radius-lg) 0 0;
    "#
    )
}

/// 获取表格滚动样式
fn get_table_scroll_css() -> String {
    css!(
        r#"
        overflow: auto;

        .ant-table-table {
            min-width: 100%;
        }
    "#
    )
}

/// 获取表格标题样式
fn get_table_title_css() -> String {
    css!(
        r#"
        padding: 16px 16px 0 16px;
        position: relative;
        top: 1px;
        border-radius: var(--ant-border-radius-lg) var(--ant-border-radius-lg) 0 0;
    "#
    )
}

/// 获取表格容器样式
fn get_table_container_css() -> String {
    css!(
        r#"
        position: relative;
    "#
    )
}

/// 获取表格table样式
fn get_table_table_css() -> String {
    css!(
        r#"
        width: 100%;
        text-align: left;
        border-radius: 0 0 var(--ant-border-radius-lg) var(--ant-border-radius-lg);
        border-collapse: separate;
        border-spacing: 0;
    "#
    )
}

/// 获取表格表头样式
fn get_table_thead_css() -> String {
    css!(
        r#"

    "#
    )
}

/// 获取表格行样式
fn get_table_row_css() -> String {
    css!(
        r#"

        &:hover > td {
            background: var(--ant-bg-color-hover);
        }
    "#
    )
}

/// 获取表格单元格样式
fn get_table_cell_css() -> String {
    css!(
        r#"
        position: relative;
        padding: 16px;
        overflow-wrap: break-word;
        border-bottom: 1px solid var(--ant-border-color-split);
        transition: background 0.3s;

        &:last-child {
            border-right: 0;
        }
    "#
    )
}

/// 获取表格选择列样式
fn get_table_selection_column_css() -> String {
    css!(
        r#"
        width: 32px;
        min-width: 32px;
        text-align: center;
    "#
    )
}

/// 获取复选框输入样式
fn get_checkbox_input_css() -> String {
    css!(
        r#"
        position: absolute;
        top: 50%;
        left: 50%;
        z-index: 1;
        width: 16px;
        height: 16px;
        margin-top: -8px;
        margin-left: -8px;
        cursor: pointer;
        opacity: 0;
    "#
    )
}

/// 获取表格列标题样式
fn get_table_column_title_css() -> String {
    css!(
        r#"
        position: relative;
        z-index: 1;
        flex: 1 1 auto;
    "#
    )
}

/// 获取表格列排序器样式
fn get_table_column_sorters_css() -> String {
    css!(
        r#"
        display: flex;
        flex: auto;
        align-items: center;
        justify-content: space-between;
        cursor: pointer;

        &:hover {
            background: var(--ant-bg-color-hover);
        }
    "#
    )
}

/// 获取表格列排序器样式
fn get_table_column_sorter_css() -> String {
    css!(
        r#"
        margin-left: 4px;
        color: var(--ant-text-color-tertiary);
        font-size: 0;
        transition: all 0.3s;
    "#
    )
}

/// 获取表格列排序器向上样式
fn get_table_column_sorter_up_css() -> String {
    css!(
        r#"
        height: 0;
        line-height: 0;

        &:hover {
            color: var(--ant-text-color-secondary);
        }
    "#
    )
}

/// 获取表格列排序器向下样式
fn get_table_column_sorter_down_css() -> String {
    css!(
        r#"
        height: 0;
        line-height: 0;

        &:hover {
            color: var(--ant-text-color-secondary);
        }
    "#
    )
}

/// 获取表格主体样式
fn get_table_tbody_css() -> String {
    css!(
        r#"

    "#
    )
}

/// 获取表格占位符样式
fn get_table_placeholder_css() -> String {
    css!(
        r#"

        &:hover > td {
            background: var(--ant-bg-color-container);
        }
    "#
    )
}

/// 获取空状态样式
fn get_empty_css() -> String {
    css!(
        r#"
        margin: 32px 0;
        color: var(--ant-text-color-disabled);
        font-size: var(--ant-font-size-base);
        line-height: var(--ant-line-height-base);
        text-align: center;
    "#
    )
}

/// 获取空状态图片样式
fn get_empty_image_css() -> String {
    css!(
        r#"
        height: 40px;
        margin-bottom: 8px;
        opacity: 1;
    "#
    )
}

/// 获取空状态描述样式
fn get_empty_description_css() -> String {
    css!(
        r#"
        color: var(--ant-text-color-disabled);
    "#
    )
}

/// 获取表格尾部样式
fn get_table_footer_css() -> String {
    css!(
        r#"
        padding: 16px;
        color: var(--ant-text-color);
        background: var(--ant-bg-color-container);
    "#
    )
}

/// 获取表格分页样式
fn get_table_pagination_css() -> String {
    css!(
        r#"
        display: flex;
        flex-wrap: wrap;
        row-gap: 8px;
        justify-content: flex-end;
        margin: 16px 0;
    "#
    )
}

/// 获取分页样式
fn get_pagination_css() -> String {
    css!(
        r#"
        box-sizing: border-box;
        margin: 0;
        padding: 0;
        color: var(--ant-text-color);
        font-size: var(--ant-font-size-base);
        line-height: var(--ant-line-height-base);
        list-style: none;
        font-family: inherit;
        display: flex;
        flex-wrap: wrap;
        align-items: center;
    "#
    )
}

/// 获取分页总数文本样式
fn get_pagination_total_text_css() -> String {
    css!(
        r#"
        display: inline-block;
        margin-right: 8px;
        color: var(--ant-text-color);
        font-size: var(--ant-font-size-base);
        line-height: var(--ant-line-height-base);
    "#
    )
}

/// 获取分页列表样式
fn get_pagination_list_css() -> String {
    css!(
        r#"
        display: flex;
        align-items: center;
        margin: 0;
        padding: 0;
        list-style: none;
    "#
    )
}

/// 获取分页项样式
fn get_pagination_item_css() -> String {
    css!(
        r#"
        display: inline-block;
        min-width: 32px;
        height: 32px;
        margin-right: 8px;
        font-family: inherit;
        line-height: 30px;
        text-align: center;
        vertical-align: middle;
        list-style: none;
        background-color: var(--ant-bg-color-container);
        border: 1px solid var(--ant-border-color);
        border-radius: var(--ant-border-radius);
        outline: 0;
        cursor: pointer;
        user-select: none;

        &:hover {
            border-color: var(--ant-primary-color);
            color: var(--ant-primary-color);
        }
    "#
    )
}

/// 获取分页激活项样式
fn get_pagination_item_active_css() -> String {
    css!(
        r#"
        font-weight: 600;
        background: var(--ant-primary-color);
        border-color: var(--ant-primary-color);
        color: var(--ant-text-color-inverse);

        &:hover {
            background: var(--ant-primary-color-hover);
            border-color: var(--ant-primary-color-hover);
            color: var(--ant-text-color-inverse);
        }
    "#
    )
}

/// 获取分页上一页样式
fn get_pagination_prev_css() -> String {
    get_pagination_item_css()
}

/// 获取分页下一页样式
fn get_pagination_next_css() -> String {
    get_pagination_item_css()
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

impl TableSize {
    pub fn to_css(&self) -> String {
        match self {
            TableSize::Default => css!(
                r#"
                .ant-table-cell {
                    padding: 16px;
                }
            "#
            ),
            TableSize::Middle => css!(
                r#"
                .ant-table-cell {
                    padding: 12px 8px;
                }
            "#
            ),
            TableSize::Small => css!(
                r#"
                .ant-table-cell {
                    padding: 8px;
                }
                .ant-table-title {
                    padding: 8px 0;
                }
                .ant-table-footer {
                    padding: 8px 0;
                }
            "#
            ),
        }
    }
}

// CSS-in-Rust 辅助函数

/// 获取表格包装器样式
fn get_table_wrapper_css() -> String {
    css!(
        r#"
        box-sizing: border-box;
        margin: 0;
        padding: 0;
        color: var(--ant-text-color);
        font-size: var(--ant-font-size-base);
        line-height: var(--ant-line-height-base);
        list-style: none;
        font-family: inherit;
        position: relative;
        clear: both;
        max-width: 100%;
        background: var(--ant-bg-color-container);
        border-radius: var(--ant-border-radius);
    "#
    )
}

/// 获取表格基础样式
fn get_table_base_css() -> String {
    css!(
        r#"
        box-sizing: border-box;
        margin: 0;
        padding: 0;
        color: var(--ant-text-color);
        font-size: var(--ant-font-size-base);
        line-height: var(--ant-line-height-base);
        list-style: none;
        font-family: inherit;
        position: relative;
        z-index: 0;
        clear: both;
        font-size: var(--ant-font-size-base);
        background: var(--ant-bg-color-container);
        border-radius: var(--ant-border-radius-lg) var(--ant-border-radius-lg) 0 0;
    "#
    )
}

/// 获取表格滚动样式
fn get_table_scroll_css() -> String {
    css!(
        r#"
        overflow: auto;

        .ant-table-table {
            min-width: 100%;
        }
    "#
    )
}

/// 获取表格标题样式
fn get_table_title_css() -> String {
    css!(
        r#"
        padding: 16px 16px 0 16px;
        position: relative;
        top: 1px;
        border-radius: var(--ant-border-radius-lg) var(--ant-border-radius-lg) 0 0;
    "#
    )
}

/// 获取表格容器样式
fn get_table_container_css() -> String {
    css!(
        r#"
        position: relative;
    "#
    )
}

/// 获取表格table样式
fn get_table_table_css() -> String {
    css!(
        r#"
        width: 100%;
        text-align: left;
        border-radius: 0 0 var(--ant-border-radius-lg) var(--ant-border-radius-lg);
        border-collapse: separate;
        border-spacing: 0;
    "#
    )
}

/// 获取表格表头样式
fn get_table_thead_css() -> String {
    css!(
        r#"

    "#
    )
}

/// 获取表格行样式
fn get_table_row_css() -> String {
    css!(
        r#"

        &:hover > td {
            background: var(--ant-bg-color-hover);
        }
    "#
    )
}

/// 获取表格单元格样式
fn get_table_cell_css() -> String {
    css!(
        r#"
        position: relative;
        padding: 16px;
        overflow-wrap: break-word;
        border-bottom: 1px solid var(--ant-border-color-split);
        transition: background 0.3s;

        &:last-child {
            border-right: 0;
        }
    "#
    )
}

/// 获取表格选择列样式
fn get_table_selection_column_css() -> String {
    css!(
        r#"
        width: 32px;
        min-width: 32px;
        text-align: center;
    "#
    )
}

/// 获取复选框输入样式
fn get_checkbox_input_css() -> String {
    css!(
        r#"
        position: absolute;
        top: 50%;
        left: 50%;
        z-index: 1;
        width: 16px;
        height: 16px;
        margin-top: -8px;
        margin-left: -8px;
        cursor: pointer;
        opacity: 0;
    "#
    )
}

/// 获取表格列标题样式
fn get_table_column_title_css() -> String {
    css!(
        r#"
        position: relative;
        z-index: 1;
        flex: 1 1 auto;
    "#
    )
}

/// 获取表格列排序器样式
fn get_table_column_sorters_css() -> String {
    css!(
        r#"
        display: flex;
        flex: auto;
        align-items: center;
        justify-content: space-between;
        cursor: pointer;

        &:hover {
            background: var(--ant-bg-color-hover);
        }
    "#
    )
}

/// 获取表格列排序器样式
fn get_table_column_sorter_css() -> String {
    css!(
        r#"
        margin-left: 4px;
        color: var(--ant-text-color-tertiary);
        font-size: 0;
        transition: all 0.3s;
    "#
    )
}

/// 获取表格列排序器向上样式
fn get_table_column_sorter_up_css() -> String {
    css!(
        r#"
        height: 0;
        line-height: 0;

        &:hover {
            color: var(--ant-text-color-secondary);
        }
    "#
    )
}

/// 获取表格列排序器向下样式
fn get_table_column_sorter_down_css() -> String {
    css!(
        r#"
        height: 0;
        line-height: 0;

        &:hover {
            color: var(--ant-text-color-secondary);
        }
    "#
    )
}

/// 获取表格主体样式
fn get_table_tbody_css() -> String {
    css!(
        r#"

    "#
    )
}

/// 获取表格占位符样式
fn get_table_placeholder_css() -> String {
    css!(
        r#"

        &:hover > td {
            background: var(--ant-bg-color-container);
        }
    "#
    )
}

/// 获取空状态样式
fn get_empty_css() -> String {
    css!(
        r#"
        margin: 32px 0;
        color: var(--ant-text-color-disabled);
        font-size: var(--ant-font-size-base);
        line-height: var(--ant-line-height-base);
        text-align: center;
    "#
    )
}

/// 获取空状态图片样式
fn get_empty_image_css() -> String {
    css!(
        r#"
        height: 40px;
        margin-bottom: 8px;
        opacity: 1;
    "#
    )
}

/// 获取空状态描述样式
fn get_empty_description_css() -> String {
    css!(
        r#"
        color: var(--ant-text-color-disabled);
    "#
    )
}

/// 获取表格尾部样式
fn get_table_footer_css() -> String {
    css!(
        r#"
        padding: 16px;
        color: var(--ant-text-color);
        background: var(--ant-bg-color-container);
    "#
    )
}

/// 获取表格分页样式
fn get_table_pagination_css() -> String {
    css!(
        r#"
        display: flex;
        flex-wrap: wrap;
        row-gap: 8px;
        justify-content: flex-end;
        margin: 16px 0;
    "#
    )
}

/// 获取分页样式
fn get_pagination_css() -> String {
    css!(
        r#"
        box-sizing: border-box;
        margin: 0;
        padding: 0;
        color: var(--ant-text-color);
        font-size: var(--ant-font-size-base);
        line-height: var(--ant-line-height-base);
        list-style: none;
        font-family: inherit;
        display: flex;
        flex-wrap: wrap;
        align-items: center;
    "#
    )
}

/// 获取分页总数文本样式
fn get_pagination_total_text_css() -> String {
    css!(
        r#"
        display: inline-block;
        margin-right: 8px;
        color: var(--ant-text-color);
        font-size: var(--ant-font-size-base);
        line-height: var(--ant-line-height-base);
    "#
    )
}

/// 获取分页列表样式
fn get_pagination_list_css() -> String {
    css!(
        r#"
        display: flex;
        align-items: center;
        margin: 0;
        padding: 0;
        list-style: none;
    "#
    )
}

/// 获取分页项样式
fn get_pagination_item_css() -> String {
    css!(
        r#"
        display: inline-block;
        min-width: 32px;
        height: 32px;
        margin-right: 8px;
        font-family: inherit;
        line-height: 30px;
        text-align: center;
        vertical-align: middle;
        list-style: none;
        background-color: var(--ant-bg-color-container);
        border: 1px solid var(--ant-border-color);
        border-radius: var(--ant-border-radius);
        outline: 0;
        cursor: pointer;
        user-select: none;

        &:hover {
            border-color: var(--ant-primary-color);
            color: var(--ant-primary-color);
        }
    "#
    )
}

/// 获取分页激活项样式
fn get_pagination_item_active_css() -> String {
    css!(
        r#"
        font-weight: 600;
        background: var(--ant-primary-color);
        border-color: var(--ant-primary-color);
        color: var(--ant-text-color-inverse);

        &:hover {
            background: var(--ant-primary-color-hover);
            border-color: var(--ant-primary-color-hover);
            color: var(--ant-text-color-inverse);
        }
    "#
    )
}

/// 获取分页上一页样式
fn get_pagination_prev_css() -> String {
    get_pagination_item_css()
}

/// 获取分页下一页样式
fn get_pagination_next_css() -> String {
    get_pagination_item_css()
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
    // 获取表格基础样式
    let table_base_class = get_table_base_css();
    let table_size_class = props.size.to_css();
    let table_scroll_class = if props.scroll_x || props.scroll_y {
        get_table_scroll_css()
    } else {
        String::new()
    };

    rsx! {

        div {
            class: format!("{} {}", get_table_wrapper_css(), props.class),
            style: props.style.clone(),

            // 表格标题
            if let Some(title) = &props.title {
                div {
                    class: get_table_title_css(),
                    {title.clone()}
                }
            }

            // 表格容器
            div {
                class: get_table_container_css(),

                // 表格内容
                div {
                    class: format!("{} {} {}", table_base_class, table_size_class, table_scroll_class),

                    // 表格主体
                    table {
                        class: get_table_table_css(),

                        // 表头
                        if props.show_header {
                            thead {
                                class: get_table_thead_css(),
                                tr {
                                    class: get_table_row_css(),
                                    key: "header",

                                    // 行选择列
                                    if props.row_selection.is_some() {
                                        th {
                                            class: format!("{} {}", get_table_cell_css(), get_table_selection_column_css()),
                                            input {
                                                r#type: "checkbox",
                                                class: get_checkbox_input_css(),
                                            }
                                        }
                                    }

                                    // 数据列
                                    for column in &props.columns {
                                        th {
                                            class: format!(
                                                "{} {}",
                                                get_table_cell_css(),
                                                if let Some(align) = &column.align {
                                                    align.to_css()
                                                } else {
                                                    String::new()
                                                }
                                            ),
                                            key: column.key.clone(),
                                            style: if let Some(width) = &column.width {
                                                format!("width: {}", width)
                                            } else {
                                                String::new()
                                            },

                                            div {
                                                class: get_table_column_title_css(),
                                                {column.title.clone()}
                                            }

                                            // 排序图标
                                            if column.sortable {
                                                div {
                                                    class: get_table_column_sorters_css(),
                                                    span {
                                                        class: get_table_column_sorter_css(),
                                                        span {
                                                            class: get_table_column_sorter_up_css(),
                                                            "▲"
                                                        }
                                                        span {
                                                            class: get_table_column_sorter_down_css(),
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
                            class: get_table_tbody_css(),

                            // 数据行
                            for (index, row) in props.data_source.iter().enumerate() {
                                tr {
                                    class: get_table_row_css(),
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
                                            class: format!("{} {}", get_table_cell_css(), get_table_selection_column_css()),
                                            input {
                                                r#type: "checkbox",
                                                class: get_checkbox_input_css(),
                                            }
                                        }
                                    }

                                    // 数据列
                                    for column in &props.columns {
                                        td {
                                            class: format!(
                                                "{} {}",
                                                get_table_cell_css(),
                                                if let Some(align) = &column.align {
                                                    align.to_css()
                                                } else {
                                                    String::new()
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
                                    class: get_table_placeholder_css(),
                                    td {
                                        class: get_table_cell_css(),
                                        colspan: format!("{}", props.columns.len() + if props.row_selection.is_some() { 1 } else { 0 }),
                                        div {
                                            class: get_empty_css(),
                                            div {
                                                class: get_empty_image_css(),
                                                "📄"
                                            }
                                            div {
                                                class: get_empty_description_css(),
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
                    class: get_table_footer_css(),
                    {footer.clone()}
                }
            }

            // 分页器
            if let Some(pagination) = &props.pagination {
                div {
                    class: format!("{} {}", get_table_pagination_css(), get_pagination_css()),
                    div {
                        class: get_pagination_total_text_css(),
                        {format!("共 {} 条", pagination.total)}
                    }
                    ul {
                        class: get_pagination_list_css(),
                        li {
                            class: get_pagination_prev_css(),
                            "‹"
                        }
                        li {
                            class: format!("{} {}", get_pagination_item_css(), get_pagination_item_active_css()),
                            {pagination.current.to_string()}
                        }
                        li {
                            class: get_pagination_next_css(),
                            "›"
                        }
                    }
                }
            }
        }
    }
}

// CSS-in-Rust 辅助函数

/// 获取表格包装器样式
fn get_table_wrapper_css() -> String {
    css!(
        r#"
        box-sizing: border-box;
        margin: 0;
        padding: 0;
        color: var(--ant-text-color);
        font-size: var(--ant-font-size-base);
        line-height: var(--ant-line-height-base);
        list-style: none;
        font-family: inherit;
        position: relative;
        clear: both;
        max-width: 100%;
        background: var(--ant-bg-color-container);
        border-radius: var(--ant-border-radius);
    "#
    )
}

/// 获取表格基础样式
fn get_table_base_css() -> String {
    css!(
        r#"
        box-sizing: border-box;
        margin: 0;
        padding: 0;
        color: var(--ant-text-color);
        font-size: var(--ant-font-size-base);
        line-height: var(--ant-line-height-base);
        list-style: none;
        font-family: inherit;
        position: relative;
        z-index: 0;
        clear: both;
        font-size: var(--ant-font-size-base);
        background: var(--ant-bg-color-container);
        border-radius: var(--ant-border-radius-lg) var(--ant-border-radius-lg) 0 0;
    "#
    )
}

/// 获取表格滚动样式
fn get_table_scroll_css() -> String {
    css!(
        r#"
        overflow: auto;

        .ant-table-table {
            min-width: 100%;
        }
    "#
    )
}

/// 获取表格标题样式
fn get_table_title_css() -> String {
    css!(
        r#"
        padding: 16px 16px 0 16px;
        position: relative;
        top: 1px;
        border-radius: var(--ant-border-radius-lg) var(--ant-border-radius-lg) 0 0;
    "#
    )
}

/// 获取表格容器样式
fn get_table_container_css() -> String {
    css!(
        r#"
        position: relative;
    "#
    )
}

/// 获取表格table样式
fn get_table_table_css() -> String {
    css!(
        r#"
        width: 100%;
        text-align: left;
        border-radius: 0 0 var(--ant-border-radius-lg) var(--ant-border-radius-lg);
        border-collapse: separate;
        border-spacing: 0;
    "#
    )
}

/// 获取表格表头样式
fn get_table_thead_css() -> String {
    css!(
        r#"

    "#
    )
}

/// 获取表格行样式
fn get_table_row_css() -> String {
    css!(
        r#"

        &:hover > td {
            background: var(--ant-bg-color-hover);
        }
    "#
    )
}

/// 获取表格单元格样式
fn get_table_cell_css() -> String {
    css!(
        r#"
        position: relative;
        padding: 16px;
        overflow-wrap: break-word;
        border-bottom: 1px solid var(--ant-border-color-split);
        transition: background 0.3s;

        &:last-child {
            border-right: 0;
        }
    "#
    )
}

/// 获取表格选择列样式
fn get_table_selection_column_css() -> String {
    css!(
        r#"
        width: 32px;
        min-width: 32px;
        text-align: center;
    "#
    )
}

/// 获取复选框输入样式
fn get_checkbox_input_css() -> String {
    css!(
        r#"
        position: absolute;
        top: 50%;
        left: 50%;
        z-index: 1;
        width: 16px;
        height: 16px;
        margin-top: -8px;
        margin-left: -8px;
        cursor: pointer;
        opacity: 0;
    "#
    )
}

/// 获取表格列标题样式
fn get_table_column_title_css() -> String {
    css!(
        r#"
        position: relative;
        z-index: 1;
        flex: 1 1 auto;
    "#
    )
}

/// 获取表格列排序器样式
fn get_table_column_sorters_css() -> String {
    css!(
        r#"
        display: flex;
        flex: auto;
        align-items: center;
        justify-content: space-between;
        cursor: pointer;

        &:hover {
            background: var(--ant-bg-color-hover);
        }
    "#
    )
}

/// 获取表格列排序器样式
fn get_table_column_sorter_css() -> String {
    css!(
        r#"
        margin-left: 4px;
        color: var(--ant-text-color-tertiary);
        font-size: 0;
        transition: all 0.3s;
    "#
    )
}

/// 获取表格列排序器向上样式
fn get_table_column_sorter_up_css() -> String {
    css!(
        r#"
        height: 0;
        line-height: 0;

        &:hover {
            color: var(--ant-text-color-secondary);
        }
    "#
    )
}

/// 获取表格列排序器向下样式
fn get_table_column_sorter_down_css() -> String {
    css!(
        r#"
        height: 0;
        line-height: 0;

        &:hover {
            color: var(--ant-text-color-secondary);
        }
    "#
    )
}

/// 获取表格主体样式
fn get_table_tbody_css() -> String {
    css!(
        r#"

    "#
    )
}

/// 获取表格占位符样式
fn get_table_placeholder_css() -> String {
    css!(
        r#"

        &:hover > td {
            background: var(--ant-bg-color-container);
        }
    "#
    )
}

/// 获取空状态样式
fn get_empty_css() -> String {
    css!(
        r#"
        margin: 32px 0;
        color: var(--ant-text-color-disabled);
        font-size: var(--ant-font-size-base);
        line-height: var(--ant-line-height-base);
        text-align: center;
    "#
    )
}

/// 获取空状态图片样式
fn get_empty_image_css() -> String {
    css!(
        r#"
        height: 40px;
        margin-bottom: 8px;
        opacity: 1;
    "#
    )
}

/// 获取空状态描述样式
fn get_empty_description_css() -> String {
    css!(
        r#"
        color: var(--ant-text-color-disabled);
    "#
    )
}

/// 获取表格尾部样式
fn get_table_footer_css() -> String {
    css!(
        r#"
        padding: 16px;
        color: var(--ant-text-color);
        background: var(--ant-bg-color-container);
    "#
    )
}

/// 获取表格分页样式
fn get_table_pagination_css() -> String {
    css!(
        r#"
        display: flex;
        flex-wrap: wrap;
        row-gap: 8px;
        justify-content: flex-end;
        margin: 16px 0;
    "#
    )
}

/// 获取分页样式
fn get_pagination_css() -> String {
    css!(
        r#"
        box-sizing: border-box;
        margin: 0;
        padding: 0;
        color: var(--ant-text-color);
        font-size: var(--ant-font-size-base);
        line-height: var(--ant-line-height-base);
        list-style: none;
        font-family: inherit;
        display: flex;
        flex-wrap: wrap;
        align-items: center;
    "#
    )
}

/// 获取分页总数文本样式
fn get_pagination_total_text_css() -> String {
    css!(
        r#"
        display: inline-block;
        margin-right: 8px;
        color: var(--ant-text-color);
        font-size: var(--ant-font-size-base);
        line-height: var(--ant-line-height-base);
    "#
    )
}

/// 获取分页列表样式
fn get_pagination_list_css() -> String {
    css!(
        r#"
        display: flex;
        align-items: center;
        margin: 0;
        padding: 0;
        list-style: none;
    "#
    )
}

/// 获取分页项样式
fn get_pagination_item_css() -> String {
    css!(
        r#"
        display: inline-block;
        min-width: 32px;
        height: 32px;
        margin-right: 8px;
        font-family: inherit;
        line-height: 30px;
        text-align: center;
        vertical-align: middle;
        list-style: none;
        background-color: var(--ant-bg-color-container);
        border: 1px solid var(--ant-border-color);
        border-radius: var(--ant-border-radius);
        outline: 0;
        cursor: pointer;
        user-select: none;

        &:hover {
            border-color: var(--ant-primary-color);
            color: var(--ant-primary-color);
        }
    "#
    )
}

/// 获取分页激活项样式
fn get_pagination_item_active_css() -> String {
    css!(
        r#"
        font-weight: 600;
        background: var(--ant-primary-color);
        border-color: var(--ant-primary-color);
        color: var(--ant-text-color-inverse);

        &:hover {
            background: var(--ant-primary-color-hover);
            border-color: var(--ant-primary-color-hover);
            color: var(--ant-text-color-inverse);
        }
    "#
    )
}

/// 获取分页上一页样式
fn get_pagination_prev_css() -> String {
    get_pagination_item_css()
}

/// 获取分页下一页样式
fn get_pagination_next_css() -> String {
    get_pagination_item_css()
}

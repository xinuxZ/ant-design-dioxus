//! Table组件样式模块
//!
//! 本模块包含Table组件的所有样式定义，从组件逻辑中分离出来，
//! 提高代码的可维护性和复用性。

use crate::shared::styles::mixins::*;
use crate::shared::styles::tokens::DesignToken;
use css_in_rust::css;

/// 表格尺寸枚举
#[derive(Debug, Clone, PartialEq)]
pub enum TableSize {
    Default,
    Middle,
    Small,
}

/// 表格选择类型枚举
#[derive(Debug, Clone, PartialEq)]
pub enum TableSelectionType {
    Checkbox,
    Radio,
}

/// 表格样式生成器
pub struct TableStyleGenerator {
    pub size: TableSize,
    pub bordered: bool,
    pub show_header: bool,
    pub loading: bool,
    pub sticky: bool,
}

impl TableStyleGenerator {
    /// 创建新的表格样式生成器
    pub fn new() -> Self {
        Self {
            size: TableSize::Default,
            bordered: false,
            show_header: true,
            loading: false,
            sticky: false,
        }
    }

    /// 设置表格尺寸
    pub fn with_size(mut self, size: TableSize) -> Self {
        self.size = size;
        self
    }

    /// 设置边框状态
    pub fn with_bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }

    /// 设置表头显示状态
    pub fn with_show_header(mut self, show_header: bool) -> Self {
        self.show_header = show_header;
        self
    }

    /// 设置加载状态
    pub fn with_loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    /// 设置粘性表头
    pub fn with_sticky(mut self, sticky: bool) -> Self {
        self.sticky = sticky;
        self
    }

    /// 生成完整的表格样式类名
    pub fn generate(&self) -> String {
        let mut classes = vec![self.base_style(), self.size_style()];

        if self.bordered {
            classes.push(self.bordered_style());
        }

        if !self.show_header {
            classes.push(self.no_header_style());
        }

        if self.loading {
            classes.push(self.loading_style());
        }

        if self.sticky {
            classes.push(self.sticky_style());
        }

        classes.join(" ")
    }

    /// 基础表格样式
    fn base_style(&self) -> String {
        format!(
            "{} {}",
            table_style(),
            css!(
                r#"
            position: relative;
            "#
            )
        )
    }

    /// 表格尺寸样式
    fn size_style(&self) -> String {
        match self.size {
            TableSize::Small => css!(
                r#"
                & .ant-table-thead > tr > th,
                & .ant-table-tbody > tr > td,
                & .ant-table tfoot > tr > th,
                & .ant-table tfoot > tr > td {
                    padding: 8px;
                }
                "#
            )
            .to_string(),
            TableSize::Middle => css!(
                r#"
                & .ant-table-thead > tr > th,
                & .ant-table-tbody > tr > td,
                & .ant-table tfoot > tr > th,
                & .ant-table tfoot > tr > td {
                    padding: 12px 8px;
                }
                "#
            )
            .to_string(),
            TableSize::Default => css!(
                r#"
                & .ant-table-thead > tr > th,
                & .ant-table-tbody > tr > td,
                & .ant-table tfoot > tr > th,
                & .ant-table tfoot > tr > td {
                    padding: 16px;
                }
                "#
            )
            .to_string(),
        }
    }

    /// 边框样式
    fn bordered_style(&self) -> String {
        css!(
            r#"
            & .ant-table-container {
                border: 1px solid var(--ant-border-color-base);
                border-right: 0;
                border-bottom: 0;
            }

            & .ant-table-thead > tr > th,
            & .ant-table-tbody > tr > td,
            & .ant-table tfoot > tr > th,
            & .ant-table tfoot > tr > td {
                border-right: 1px solid var(--ant-border-color-base);
            }

            & .ant-table-placeholder .ant-table-cell {
                border-right: 1px solid var(--ant-border-color-base);
                border-bottom: 1px solid var(--ant-border-color-base);
            }
            "#
        )
        .to_string()
    }

    /// 无表头样式
    fn no_header_style(&self) -> String {
        css!(
            r#"
            & .ant-table-thead {
                display: none;
            }
            "#
        )
        .to_string()
    }

    /// 加载状态样式
    fn loading_style(&self) -> String {
        loading_state()
    }

    /// 粘性表头样式
    fn sticky_style(&self) -> String {
        css!(
            r#"
            & .ant-table-thead > tr > th {
                position: sticky;
                top: 0;
                z-index: 1;
            }
            "#
        )
        .to_string()
    }
}

/// 表格容器样式
pub fn table_container_style() -> String {
    css!(
        r#"
        position: relative;
        clear: both;
        max-width: 100%;
        "#
    )
    .to_string()
}

/// 表格内容样式
pub fn table_content_style() -> String {
    css!(
        r#"
        position: relative;
        overflow: hidden;
        "#
    )
    .to_string()
}

/// 表格主体样式
pub fn table_body_style() -> String {
    css!(
        r#"
        overflow: auto;
        "#
    )
    .to_string()
}

/// 表格样式
pub fn table_style() -> String {
    css!(
        r#"
        width: 100%;
        text-align: left;
        border-radius: var(--ant-border-radius) var(--ant-border-radius) 0 0;
        border-collapse: separate;
        border-spacing: 0;
        "#
    )
    .to_string()
}

/// 表格表头样式
pub fn table_thead_style() -> String {
    css!(
        r#"
        & > tr > th {
            position: relative;
            color: var(--ant-text-color);
            font-weight: 500;
            text-align: left;
            background: var(--ant-bg-color-container);
            border-bottom: 1px solid var(--ant-border-color-split);
            transition: background var(--ant-motion-duration-mid) var(--ant-motion-ease-in-out);
        }

        & > tr:not(:last-child) > th[colspan] {
            border-bottom: 0;
        }
        "#
    )
    .to_string()
}

/// 表格表体样式
pub fn table_tbody_style() -> String {
    css!(
        r#"
        & > tr > td {
            border-bottom: 1px solid var(--ant-border-color-split);
            transition: background var(--ant-motion-duration-mid) var(--ant-motion-ease-in-out);
        }

        & > tr:hover > td {
            background: var(--ant-bg-color-container-hover);
        }

        & > tr.ant-table-row-selected > td {
            background: var(--ant-primary-color-bg);
        }

        & > tr.ant-table-row-selected:hover > td {
            background: var(--ant-primary-color-bg-hover);
        }
        "#
    )
    .to_string()
}

/// 表格单元格样式
pub fn table_cell_style() -> String {
    format!(
        "{} {}",
        text_ellipsis(),
        css!(
            r#"
        position: relative;
        padding: 16px;
        overflow-wrap: break-word;
        "#
        )
    )
}

/// 表格排序列样式
pub fn table_column_sorter_style() -> String {
    css!(
        r#"
        margin-left: 4px;
        display: inline-flex;
        flex-direction: column;
        align-items: center;

        & .ant-table-column-sorter-up,
        & .ant-table-column-sorter-down {
            height: 1em;
            line-height: 1em;
            display: block;
            width: 1em;
            cursor: pointer;

            &.active {
                color: var(--ant-primary-color);
            }
        }
        "#
    )
    .to_string()
}

/// 表格筛选器样式
pub fn table_filter_trigger_style() -> String {
    css!(
        r#"
        position: relative;
        display: inline-block;
        margin-left: 4px;
        color: var(--ant-text-color-secondary);
        font-size: 12px;
        cursor: pointer;
        transition: all var(--ant-motion-duration-mid) var(--ant-motion-ease-in-out);

        &:hover {
            color: var(--ant-text-color-secondary-hover);
        }

        &.active {
            color: var(--ant-primary-color);
        }
        "#
    )
    .to_string()
}

/// 表格选择列样式
pub fn table_selection_column_style() -> String {
    css!(
        r#"
        width: 32px;
        min-width: 32px;
        text-align: center;
        "#
    )
    .to_string()
}

/// 表格展开图标样式
pub fn table_row_expand_icon_style() -> String {
    css!(
        r#"
        color: var(--ant-primary-color);
        text-decoration: none;
        cursor: pointer;
        transition: color var(--ant-motion-duration-mid) var(--ant-motion-ease-in-out);

        &:focus,
        &:hover,
        &:active {
            color: var(--ant-primary-color-hover);
        }

        &[aria-expanded="true"] {
            transform: rotate(90deg);
        }
        "#
    )
    .to_string()
}

/// 表格展开行样式
pub fn table_expanded_row_style() -> String {
    css!(
        r#"
        & > td {
            border-bottom: 1px solid var(--ant-border-color-split);
        }

        &:hover > td {
            background: var(--ant-bg-color-container);
        }
        "#
    )
    .to_string()
}

/// 表格空状态样式
pub fn table_placeholder_style() -> String {
    css!(
        r#"
        position: relative;
        z-index: 1;
        padding: 16px 12px;
        color: var(--ant-text-color-disabled);
        font-size: var(--ant-font-size-base);
        text-align: center;
        background: var(--ant-bg-color-container);
        border-bottom: 1px solid var(--ant-border-color-split);
        border-radius: 0 0 var(--ant-border-radius) var(--ant-border-radius);
        "#
    )
    .to_string()
}

/// 表格分页样式
pub fn table_pagination_style() -> String {
    css!(
        r#"
        margin: 16px 0;
        "#
    )
    .to_string()
}

/// 表格固定列样式
pub fn table_fixed_left_style() -> String {
    css!(
        r#"
        position: sticky;
        left: 0;
        z-index: 1;
        "#
    )
    .to_string()
}

pub fn table_fixed_right_style() -> String {
    css!(
        r#"
        position: sticky;
        right: 0;
        z-index: 1;
        "#
    )
    .to_string()
}

/// 表格滚动条样式
pub fn table_scroll_style() -> String {
    format!(
        "{} {}",
        custom_scrollbar("8px", "#f5f5f5", "#d9d9d9"),
        css!(
            r#"
        overflow: auto;
        "#
        )
    )
}

/// 表格加载遮罩样式
pub fn table_loading_style() -> String {
    css!(
        r#"
        position: absolute;
        top: 0;
        right: 0;
        bottom: 0;
        left: 0;
        z-index: 1000;
        display: flex;
        align-items: center;
        justify-content: center;
        background: rgba(255, 255, 255, 0.65);
        "#
    )
    .to_string()
}

//! Select 选择器组件样式

use crate::theme::AliasToken;
use css_in_rust::css;

/// Select 选择器样式生成器
pub struct SelectStyleGenerator {
    /// 尺寸
    pub size: SelectSize,
    /// 状态
    pub status: SelectStatus,
    /// 是否禁用
    pub disabled: bool,
    /// 是否多选
    pub multiple: bool,
    /// 是否支持搜索
    pub show_search: bool,
    /// 是否有边框
    pub bordered: bool,
    /// 主题令牌
    pub token: AliasToken,
}

/// Select 选择器尺寸
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SelectSize {
    /// 大尺寸
    Large,
    /// 中等尺寸（默认）
    Middle,
    /// 小尺寸
    Small,
}

impl Default for SelectSize {
    fn default() -> Self {
        Self::Middle
    }
}

/// Select 选择器状态
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SelectStatus {
    /// 正常状态
    Normal,
    /// 错误状态
    Error,
    /// 警告状态
    Warning,
}

impl Default for SelectStatus {
    fn default() -> Self {
        Self::Normal
    }
}

impl From<super::SelectSize> for SelectSize {
    fn from(size: super::SelectSize) -> Self {
        match size {
            super::SelectSize::Large => Self::Large,
            super::SelectSize::Middle => Self::Middle,
            super::SelectSize::Small => Self::Small,
        }
    }
}

impl From<super::SelectStatus> for SelectStatus {
    fn from(status: super::SelectStatus) -> Self {
        match status {
            super::SelectStatus::Normal => Self::Normal,
            super::SelectStatus::Error => Self::Error,
            super::SelectStatus::Warning => Self::Warning,
        }
    }
}

impl Default for SelectStyleGenerator {
    fn default() -> Self {
        Self {
            size: SelectSize::default(),
            status: SelectStatus::default(),
            disabled: false,
            multiple: false,
            show_search: false,
            bordered: true,
            token: AliasToken::default(),
        }
    }
}

impl SelectStyleGenerator {
    /// 创建新实例
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置尺寸
    pub fn with_size(mut self, size: SelectSize) -> Self {
        self.size = size;
        self
    }

    /// 设置状态
    pub fn with_status(mut self, status: SelectStatus) -> Self {
        self.status = status;
        self
    }

    /// 设置是否禁用
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// 设置是否多选
    pub fn with_multiple(mut self, multiple: bool) -> Self {
        self.multiple = multiple;
        self
    }

    /// 设置是否支持搜索
    pub fn with_show_search(mut self, show_search: bool) -> Self {
        self.show_search = show_search;
        self
    }

    /// 设置是否有边框
    pub fn with_bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }

    /// 设置主题令牌
    pub fn with_token(mut self, token: AliasToken) -> Self {
        self.token = token;
        self
    }

    /// 生成样式类名
    pub fn generate(&self) -> String {
        let mut classes = vec!["ant-select"];

        // 尺寸
        match self.size {
            SelectSize::Large => classes.push("ant-select-lg"),
            SelectSize::Middle => {}
            SelectSize::Small => classes.push("ant-select-sm"),
        }

        // 状态
        match self.status {
            SelectStatus::Normal => {}
            SelectStatus::Error => classes.push("ant-select-status-error"),
            SelectStatus::Warning => classes.push("ant-select-status-warning"),
        }

        // 禁用
        if self.disabled {
            classes.push("ant-select-disabled");
        }

        // 多选
        if self.multiple {
            classes.push("ant-select-multiple");
        }

        // 搜索
        if self.show_search {
            classes.push("ant-select-show-search");
        }

        // 无边框
        if !self.bordered {
            classes.push("ant-select-borderless");
        }

        classes.join(" ")
    }

    /// 生成 CSS 样式
    pub fn generate_css(&self) -> String {
        css!(
            r#"
            .ant-select {
                position: relative;
                display: inline-block;
                min-width: 120px;
                font-size: ${font_size}px;
                line-height: ${line_height};
                color: ${color_text};
                background-color: ${color_bg_container};
                border: 1px solid ${color_border};
                border-radius: ${border_radius}px;
                transition: all 0.2s cubic-bezier(0.645, 0.045, 0.355, 1);
                cursor: pointer;
            }

            .ant-select:hover:not(.ant-select-disabled) {
                border-color: ${color_primary_hover};
            }

            .ant-select-focused:not(.ant-select-disabled) {
                border-color: ${color_primary};
                box-shadow: 0 0 0 2px ${color_primary_hover_outline};
                outline: 0;
            }

            .ant-select-open:not(.ant-select-disabled) {
                border-color: ${color_primary};
            }

            .ant-select-selector {
                position: relative;
                display: flex;
                flex-wrap: wrap;
                align-items: center;
                padding: ${padding_xs}px ${padding_sm}px;
                min-height: ${height_base}px;
                outline: none;
            }

            .ant-select-selection-item {
                flex: 1;
                overflow: hidden;
                white-space: nowrap;
                text-overflow: ellipsis;
                font-weight: 400;
                color: ${color_text};
            }

            .ant-select-selection-placeholder {
                flex: 1;
                color: ${color_text_placeholder};
                pointer-events: none;
            }

            .ant-select-selection-search {
                position: relative;
                flex: 1;
                max-width: 100%;
            }

            .ant-select-selection-search-input {
                width: 100%;
                min-width: 4px;
                padding: 0;
                background: transparent;
                border: none;
                outline: none;
                appearance: none;
                font-size: ${font_size}px;
                color: ${color_text};
            }

            .ant-select-selection-search-input::placeholder {
                color: ${color_text_placeholder};
            }

            .ant-select-multiple .ant-select-selector {
                padding: ${padding_xs / 2}px ${padding_xs}px;
                min-height: ${height_base}px;
                display: flex;
                flex-wrap: wrap;
            }

            .ant-select-multiple .ant-select-selection-item {
                position: relative;
                display: flex;
                align-items: center;
                margin-top: ${margin_xs / 2}px;
                margin-bottom: ${margin_xs / 2}px;
                margin-right: ${margin_xs}px;
                padding: 0 ${padding_xs}px 0 ${padding_xs}px;
                line-height: ${height_base - padding_xs * 2}px;
                background: ${color_bg_container_disabled};
                border: 1px solid ${color_split};
                border-radius: ${border_radius}px;
                cursor: default;
                transition: font-size 0.3s, line-height 0.3s, height 0.3s;
                user-select: none;
                flex: none;
                max-width: 100%;
            }

            .ant-select-multiple .ant-select-selection-item-content {
                display: inline-block;
                margin-right: ${margin_xs}px;
                overflow: hidden;
                white-space: nowrap;
                text-overflow: ellipsis;
            }

            .ant-select-multiple .ant-select-selection-item-remove {
                color: ${color_text_secondary};
                font-style: normal;
                line-height: 0;
                text-align: center;
                text-transform: none;
                vertical-align: -0.125em;
                text-rendering: optimizeLegibility;
                -webkit-font-smoothing: antialiased;
                -moz-osx-font-smoothing: grayscale;
                display: inline-block;
                font-size: 10px;
                font-weight: bold;
                cursor: pointer;
            }

            .ant-select-multiple .ant-select-selection-item-remove:hover {
                color: ${color_text};
            }

            .ant-select-multiple .ant-select-selection-search {
                position: relative;
                margin-left: ${margin_xs / 2}px;
            }

            .ant-select-multiple .ant-select-selection-search-input {
                min-width: 4px;
                margin-left: ${margin_xs / 2}px;
            }

            .ant-select-allow-clear .ant-select-clear {
                position: absolute;
                top: 50%;
                right: ${padding_sm}px;
                z-index: 1;
                display: inline-block;
                width: 12px;
                height: 12px;
                margin-top: -6px;
                color: ${color_text_secondary};
                font-size: 12px;
                font-style: normal;
                line-height: 1;
                text-align: center;
                text-transform: none;
                background: ${color_bg_container};
                cursor: pointer;
                opacity: 0;
                transition: color 0.3s ease, opacity 0.15s ease;
                text-rendering: auto;
            }

            .ant-select-allow-clear:hover .ant-select-clear {
                opacity: 1;
            }

            .ant-select-allow-clear .ant-select-clear:hover {
                color: ${color_text};
            }

            .ant-select-arrow {
                position: absolute;
                top: 50%;
                right: ${padding_sm}px;
                width: 12px;
                height: 12px;
                margin-top: -6px;
                color: ${color_text_secondary};
                font-size: 12px;
                line-height: 1;
                text-align: center;
                pointer-events: none;
            }

            .ant-select-dropdown {
                position: absolute;
                z-index: ${z_index_popup};
                margin: 0;
                padding: ${padding_xs}px 0;
                color: ${color_text};
                font-size: ${font_size}px;
                line-height: ${line_height};
                list-style: none;
                background-color: ${color_bg_container};
                border-radius: ${border_radius}px;
                outline: none;
                box-shadow: ${box_shadow};
            }

            .ant-select-dropdown-hidden {
                display: none;
            }

            .ant-select-item {
                position: relative;
                display: block;
                min-height: ${height_base}px;
                padding: ${padding_xs}px ${padding_sm}px;
                color: ${color_text};
                font-weight: normal;
                line-height: ${line_height};
                cursor: pointer;
                transition: background 0.3s ease;
            }

            .ant-select-item-option-active:not(.ant-select-item-option-disabled) {
                background-color: ${color_bg_text_hover};
            }

            .ant-select-item-option-selected:not(.ant-select-item-option-disabled) {
                color: ${color_text};
                font-weight: 600;
                background-color: ${color_bg_text_active};
            }

            .ant-select-item-option-disabled {
                color: ${color_text_disabled};
                cursor: not-allowed;
            }

            .ant-select-item-empty {
                color: ${color_text_secondary};
                text-align: center;
            }

            .ant-select-item-group {
                color: ${color_text_secondary};
                font-size: ${font_size}px;
                padding: ${padding_xs}px ${padding_sm}px;
                font-weight: bold;
            }

            .ant-select-lg {
                font-size: ${font_size_lg}px;
            }

            .ant-select-lg .ant-select-selector {
                padding: ${padding_sm}px ${padding_md}px;
                min-height: ${height_lg}px;
            }

            .ant-select-sm {
                font-size: ${font_size_sm}px;
            }

            .ant-select-sm .ant-select-selector {
                padding: ${padding_xs / 2}px ${padding_xs}px;
                min-height: ${height_sm}px;
            }

            .ant-select-status-error.ant-select:not(.ant-select-disabled):not(.ant-select-customize-input) .ant-select-selector {
                background-color: ${color_bg_container};
                border-color: ${color_error} !important;
            }

            .ant-select-status-error.ant-select:not(.ant-select-disabled):not(.ant-select-customize-input).ant-select-focused .ant-select-selector {
                box-shadow: 0 0 0 2px ${color_error_hover_outline};
            }

            .ant-select-status-warning.ant-select:not(.ant-select-disabled):not(.ant-select-customize-input) .ant-select-selector {
                background-color: ${color_bg_container};
                border-color: ${color_warning} !important;
            }

            .ant-select-status-warning.ant-select:not(.ant-select-disabled):not(.ant-select-customize-input).ant-select-focused .ant-select-selector {
                box-shadow: 0 0 0 2px ${color_warning_hover_outline};
            }

            .ant-select-disabled {
                color: ${color_text_disabled};
                background: ${color_bg_container_disabled};
                cursor: not-allowed;
            }

            .ant-select-disabled .ant-select-selector {
                color: ${color_text_disabled};
                background: ${color_bg_container_disabled};
                cursor: not-allowed;
            }

            .ant-select-disabled .ant-select-arrow {
                color: ${color_text_disabled};
            }

            .ant-select-borderless .ant-select-selector {
                background-color: transparent !important;
                border-color: transparent !important;
                box-shadow: none !important;
            }

            .ant-select-show-search.ant-select:not(.ant-select-customize-input) .ant-select-selector {
                cursor: text;
            }

            .ant-select-show-search.ant-select:not(.ant-select-customize-input) .ant-select-selector input {
                cursor: auto;
            }

            .ant-select-rtl {
                direction: rtl;
            }

            .ant-select-rtl .ant-select-arrow {
                right: auto;
                left: ${padding_sm}px;
            }

            @media (max-width: 575px) {
                .ant-select {
                    width: 100%;
                }
            }

            @keyframes antSelectDropdownSlideUpIn {
                0% {
                    opacity: 0;
                    transform: scaleY(0.8);
                }
                100% {
                    opacity: 1;
                    transform: scaleY(1);
                }
            }

            @keyframes antSelectDropdownSlideUpOut {
                0% {
                    opacity: 1;
                    transform: scaleY(1);
                }
                100% {
                    opacity: 0;
                    transform: scaleY(0.8);
                }
            }

            @media (prefers-contrast: high) {
                .ant-select {
                    border: 1px solid black;
                }
                .ant-select-focused {
                    outline: 2px solid black;
                }
            }

            @media (prefers-color-scheme: dark) {
                .ant-select {
                    background-color: ${color_bg_container_dark};
                    color: ${color_text_dark};
                }
                .ant-select-selection-item {
                    color: ${color_text_dark};
                }
                .ant-select-dropdown {
                    background-color: ${color_bg_container_dark};
                    color: ${color_text_dark};
                }
                .ant-select-item {
                    color: ${color_text_dark};
                }
                .ant-select-item-option-active:not(.ant-select-item-option-disabled) {
                    background-color: ${color_bg_text_hover_dark};
                }
                .ant-select-item-option-selected:not(.ant-select-item-option-disabled) {
                    background-color: ${color_bg_text_active_dark};
                }
            }
            "#,
            font_size = self.token.font_size,
            line_height = self.token.line_height,
            color_text = self.token.color_text,
            color_bg_container = self.token.color_bg_container,
            color_border = self.token.color_border,
            border_radius = self.token.border_radius,
            color_primary_hover = self.token.color_primary_hover,
            color_primary = self.token.color_primary,
            color_primary_hover_outline = self.token.color_primary_hover_outline,
            padding_xs = self.token.padding_xs,
            padding_sm = self.token.padding_sm,
            height_base = self.token.height_base,
            color_text_placeholder = self.token.color_text_placeholder,
            margin_xs = self.token.margin_xs,
            color_bg_container_disabled = self.token.color_bg_container_disabled,
            color_split = self.token.color_split,
            color_text_secondary = self.token.color_text_secondary,
            z_index_popup = self.token.z_index_popup,
            box_shadow = self.token.box_shadow,
            color_bg_text_hover = self.token.color_bg_text_hover,
            color_bg_text_active = self.token.color_bg_text_active,
            color_text_disabled = self.token.color_text_disabled,
            font_size_lg = self.token.font_size_lg,
            padding_md = self.token.padding_md,
            height_lg = self.token.height_lg,
            font_size_sm = self.token.font_size_sm,
            height_sm = self.token.height_sm,
            color_error = self.token.color_error,
            color_error_hover_outline = self.token.color_error_hover_outline,
            color_warning = self.token.color_warning,
            color_warning_hover_outline = self.token.color_warning_hover_outline,
            color_bg_container_dark = "#1f1f1f",
            color_text_dark = "rgba(255, 255, 255, 0.85)",
            color_bg_text_hover_dark = "rgba(255, 255, 255, 0.08)",
            color_bg_text_active_dark = "rgba(255, 255, 255, 0.12)"
        ).to_string()
    }
}

/// 生成 Select 样式
pub fn generate_select_style(
    size: SelectSize,
    status: SelectStatus,
    disabled: bool,
    multiple: bool,
    show_search: bool,
    bordered: bool,
) -> String {
    SelectStyleGenerator::new()
        .with_size(size)
        .with_status(status)
        .with_disabled(disabled)
        .with_multiple(multiple)
        .with_show_search(show_search)
        .with_bordered(bordered)
        .generate()
}

/// 生成 Select CSS 样式
pub fn generate_select_css(
    size: SelectSize,
    status: SelectStatus,
    disabled: bool,
    multiple: bool,
    show_search: bool,
    bordered: bool,
) -> String {
    SelectStyleGenerator::new()
        .with_size(size)
        .with_status(status)
        .with_disabled(disabled)
        .with_multiple(multiple)
        .with_show_search(show_search)
        .with_bordered(bordered)
        .generate_css()
}

/// 默认 Select 样式
pub fn default_select_style() -> String {
    SelectStyleGenerator::new().generate()
}

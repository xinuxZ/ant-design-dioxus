//! Input组件样式模块
//!
//! 本模块包含Input组件的所有样式定义，从组件逻辑中分离出来，
//! 提高代码的可维护性和复用性。

use crate::shared::styles::mixins::*;
use crate::shared::styles::tokens::DesignToken;
use css_in_rust::css;

/// 输入框尺寸枚举
#[derive(Debug, Clone, PartialEq)]
pub enum InputSize {
    Large,
    Middle,
    Small,
}

/// 输入框状态枚举
#[derive(Debug, Clone, PartialEq)]
pub enum InputStatus {
    Default,
    Error,
    Warning,
}

/// 输入框样式生成器
pub struct InputStyleGenerator {
    pub size: InputSize,
    pub status: InputStatus,
    pub disabled: bool,
    pub bordered: bool,
    pub allow_clear: bool,
    pub prefix: bool,
    pub suffix: bool,
}

impl InputStyleGenerator {
    /// 创建新的输入框样式生成器
    pub fn new() -> Self {
        Self {
            size: InputSize::Middle,
            status: InputStatus::Default,
            disabled: false,
            bordered: true,
            allow_clear: false,
            prefix: false,
            suffix: false,
        }
    }

    /// 设置输入框尺寸
    pub fn with_size(mut self, size: InputSize) -> Self {
        self.size = size;
        self
    }

    /// 设置输入框状态
    pub fn with_status(mut self, status: InputStatus) -> Self {
        self.status = status;
        self
    }

    /// 设置禁用状态
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// 设置边框状态
    pub fn with_bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }

    /// 设置清除按钮
    pub fn with_allow_clear(mut self, allow_clear: bool) -> Self {
        self.allow_clear = allow_clear;
        self
    }

    /// 设置前缀
    pub fn with_prefix(mut self, prefix: bool) -> Self {
        self.prefix = prefix;
        self
    }

    /// 设置后缀
    pub fn with_suffix(mut self, suffix: bool) -> Self {
        self.suffix = suffix;
        self
    }

    /// 生成完整的输入框样式类名
    pub fn generate(&self) -> String {
        let mut classes = vec![self.base_style(), self.size_style(), self.status_style()];

        if self.disabled {
            classes.push(self.disabled_style());
        }

        if !self.bordered {
            classes.push(self.borderless_style());
        }

        classes.join(" ")
    }

    /// 基础输入框样式
    fn base_style(&self) -> String {
        format!(
            "{} {}",
            input_style(),
            css!(
                r#"
            position: relative;
            display: inline-flex;
            align-items: center;
            width: 100%;
            min-width: 0;

            input {
                flex: 1;
                min-width: 0;
                border: none;
                outline: none;
                background: transparent;
                color: inherit;
                font-size: inherit;
                line-height: inherit;
            }
            "#
            )
        )
    }

    /// 输入框尺寸样式
    fn size_style(&self) -> String {
        match self.size {
            InputSize::Large => css!(
                r#"
                height: 40px;
                padding: 6px 11px;
                font-size: 16px;
                "#
            ),
            InputSize::Small => css!(
                r#"
                height: 24px;
                padding: 0px 7px;
                font-size: var(--ant-font-size-sm);
                "#
            ),
            InputSize::Middle => css!(
                r#"
                height: 32px;
                padding: 4px 11px;
                font-size: var(--ant-font-size-base);
                "#
            ),
        }
    }

    /// 输入框状态样式
    fn status_style(&self) -> String {
        match self.status {
            InputStatus::Error => css!(
                r#"
                border-color: var(--ant-error-color);

                &:hover {
                    border-color: var(--ant-error-color);
                }

                &:focus {
                    border-color: var(--ant-error-color);
                    box-shadow: 0 0 0 2px rgba(255, 77, 79, 0.2);
                }
                "#
            ),
            InputStatus::Warning => css!(
                r#"
                border-color: var(--ant-warning-color);

                &:hover {
                    border-color: var(--ant-warning-color);
                }

                &:focus {
                    border-color: var(--ant-warning-color);
                    box-shadow: 0 0 0 2px rgba(250, 173, 20, 0.2);
                }
                "#
            ),
            InputStatus::Default => String::new(),
        }
    }

    /// 禁用状态样式
    fn disabled_style(&self) -> String {
        format!(
            "{} {}",
            disabled_state(),
            css!(
                r#"
            color: var(--ant-text-color-disabled);
            background-color: var(--ant-bg-color-container);
            border-color: var(--ant-border-color);
            "#
            )
        )
    }

    /// 无边框样式
    fn borderless_style(&self) -> String {
        css!(
            r#"
            border: none;
            box-shadow: none;

            &:hover,
            &:focus {
                border: none;
                box-shadow: none;
            }
            "#
        )
    }
}

/// 输入框前缀样式
pub fn input_prefix_style() -> String {
    css!(
        r#"
        margin-right: 4px;
        color: var(--ant-text-color-secondary);
        "#
    )
}

/// 输入框后缀样式
pub fn input_suffix_style() -> String {
    css!(
        r#"
        margin-left: 4px;
        color: var(--ant-text-color-secondary);
        "#
    )
}

/// 输入框清除按钮样式
pub fn input_clear_style() -> String {
    format!(
        "{} {}",
        reset_button(),
        css!(
            r#"
        margin-left: 4px;
        color: var(--ant-text-color-secondary);
        font-size: 12px;
        cursor: pointer;
        transition: color var(--ant-motion-duration-mid) var(--ant-motion-ease-in-out);

        &:hover {
            color: var(--ant-text-color);
        }
        "#
        )
    )
}

/// 输入框组样式
pub fn input_group_style() -> String {
    css!(
        r#"
        position: relative;
        display: table;
        width: 100%;
        border-collapse: separate;
        border-spacing: 0;

        & > * {
            position: relative;
            z-index: 1;
            display: table-cell;
            vertical-align: middle;
        }

        & > *:not(:first-child):not(:last-child) {
            border-radius: 0;
        }

        & > *:first-child:not(:last-child) {
            border-top-right-radius: 0;
            border-bottom-right-radius: 0;
        }

        & > *:last-child:not(:first-child) {
            border-top-left-radius: 0;
            border-bottom-left-radius: 0;
        }

        & > *:not(:first-child) {
            margin-left: -1px;
        }
        "#
    )
}

/// 输入框组前置标签样式
pub fn input_group_addon_style() -> String {
    css!(
        r#"
        padding: 0 11px;
        color: var(--ant-text-color);
        font-weight: normal;
        font-size: var(--ant-font-size-base);
        text-align: center;
        background-color: var(--ant-bg-color-container);
        border: 1px solid var(--ant-border-color);
        border-radius: var(--ant-border-radius);
        white-space: nowrap;
        "#
    )
}

/// 文本域样式
pub fn textarea_style() -> String {
    format!(
        "{} {}",
        input_style(),
        css!(
            r#"
        max-width: 100%;
        height: auto;
        min-height: 32px;
        line-height: 1.5715;
        vertical-align: bottom;
        transition: all var(--ant-motion-duration-mid) var(--ant-motion-ease-in-out), height 0s;
        resize: vertical;

        &.ant-input-disabled {
            resize: none;
        }
        "#
        )
    )
}

/// 搜索框样式
pub fn search_input_style() -> String {
    css!(
        r#"
        & .ant-input-search-button {
            border-top-left-radius: 0;
            border-bottom-left-radius: 0;
            border-left: 0;
        }

        & .ant-input-search-button:not(.ant-btn-primary) {
            color: var(--ant-text-color-secondary);
        }

        & .ant-input-search-button:not(.ant-btn-primary):hover {
            border-color: var(--ant-primary-color-hover);
            color: var(--ant-primary-color-hover);
        }
        "#
    )
}

/// 密码输入框样式
pub fn password_input_style() -> String {
    css!(
        r#"
        & .ant-input-password-icon {
            color: var(--ant-text-color-secondary);
            cursor: pointer;
            transition: all var(--ant-motion-duration-mid) var(--ant-motion-ease-in-out);
        }

        & .ant-input-password-icon:hover {
            color: var(--ant-text-color);
        }
        "#
    )
}

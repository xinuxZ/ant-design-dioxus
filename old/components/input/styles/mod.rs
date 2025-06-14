//! Input组件样式模块
//!
//! 本模块包含Input组件的所有样式定义，从组件逻辑中分离出来，
//! 提高代码的可维护性和复用性。

use crate::theme::AliasToken;
use css_in_rust::css;

/// 输入框尺寸枚举
#[derive(Debug, Clone, PartialEq)]
pub enum InputSize {
    Large,
    Middle,
    Small,
}

impl Default for InputSize {
    fn default() -> Self {
        Self::Middle
    }
}

/// 输入框状态枚举
#[derive(Debug, Clone, PartialEq)]
pub enum InputStatus {
    Default,
    Error,
    Warning,
}

impl Default for InputStatus {
    fn default() -> Self {
        Self::Default
    }
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
    pub show_count: bool,
    pub addon_before: bool,
    pub addon_after: bool,
    pub focused: bool,
    pub token: AliasToken,
}

impl Default for InputStyleGenerator {
    fn default() -> Self {
        Self {
            size: InputSize::default(),
            status: InputStatus::default(),
            disabled: false,
            bordered: true,
            allow_clear: false,
            prefix: false,
            suffix: false,
            show_count: false,
            addon_before: false,
            addon_after: false,
            focused: false,
            token: AliasToken::default(),
        }
    }
}

impl InputStyleGenerator {
    /// 创建新的输入框样式生成器
    pub fn new() -> Self {
        Self::default()
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

    /// 设置字数统计
    pub fn with_show_count(mut self, show_count: bool) -> Self {
        self.show_count = show_count;
        self
    }

    /// 设置前置标签
    pub fn with_addon_before(mut self, addon_before: bool) -> Self {
        self.addon_before = addon_before;
        self
    }

    /// 设置后置标签
    pub fn with_addon_after(mut self, addon_after: bool) -> Self {
        self.addon_after = addon_after;
        self
    }

    /// 设置焦点状态
    pub fn with_focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    /// 设置主题令牌
    pub fn with_token(mut self, token: AliasToken) -> Self {
        self.token = token;
        self
    }

    /// 生成 CSS 样式
    pub fn generate_css(&self) -> String {
        let mut css = String::new();

        // 基础样式
        css.push_str(&self.generate_base_style());

        // 尺寸样式
        css.push_str(&self.generate_size_style());

        // 状态样式
        css.push_str(&self.generate_status_style());

        // 禁用样式
        if self.disabled {
            css.push_str(&self.generate_disabled_style());
        }

        // 无边框样式
        if !self.bordered {
            css.push_str(&self.generate_borderless_style());
        }

        // 前缀后缀样式
        if self.prefix || self.suffix {
            css.push_str(&self.generate_affix_style());
        }

        // 清除按钮样式
        if self.allow_clear {
            css.push_str(&self.generate_clear_style());
        }

        // 字数统计样式
        if self.show_count {
            css.push_str(&self.generate_count_style());
        }

        // 输入组样式
        if self.addon_before || self.addon_after {
            css.push_str(&self.generate_group_style());
        }

        // 响应式样式
        css.push_str(&self.generate_responsive_style());

        // 高对比度样式
        css.push_str(&self.generate_high_contrast_style());

        // 暗色主题样式
        css.push_str(&self.generate_dark_theme_style());

        css
    }

    /// 生成基础样式
    fn generate_base_style(&self) -> String {
        let token = &self.token;

        css!(
            r#"
            .ant-input {
                box-sizing: border-box;
                margin: 0;
                padding: 4px 11px;
                color: rgba(0, 0, 0, 0.88);
                font-size: 14px;
                line-height: 1.5714285714285714;
                list-style: none;
                font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Noto Sans', sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol', 'Noto Color Emoji';
                position: relative;
                display: inline-block;
                width: 100%;
                min-width: 0;
                background-color: #ffffff;
                background-image: none;
                border-width: 1px;
                border-style: solid;
                border-color: #d9d9d9;
                border-radius: 6px;
                transition: all 0.2s;
            }

            .ant-input:placeholder-shown {
                text-overflow: ellipsis;
            }

            .ant-input::placeholder {
                color: rgba(0, 0, 0, 0.25);
                user-select: none;
            }

            .ant-input:hover {
                border-color: #4096ff;
                border-inline-end-width: 1px;
            }

            .ant-input:focus,
            .ant-input-focused {
                border-color: #4096ff;
                box-shadow: 0 0 0 2px rgba(5, 145, 255, 0.1);
                border-inline-end-width: 1px;
                outline: 0;
            }
            "#
        )
    }

    /// 生成尺寸样式
    fn generate_size_style(&self) -> String {
        match self.size {
            InputSize::Large => css!(
                r#"
                .ant-input-lg {
                    padding: 6.5px 11px;
                    font-size: 16px;
                }

                .ant-input-affix-wrapper.ant-input-lg {
                    padding: 6.5px 11px;
                    font-size: 16px;
                }
                "#
            ),
            InputSize::Small => css!(
                r#"
                .ant-input-sm {
                    padding: 0px 7px;
                    font-size: 14px;
                }

                .ant-input-affix-wrapper.ant-input-sm {
                    padding: 0px 7px;
                    font-size: 14px;
                }
                "#
            ),
            InputSize::Middle => String::new(),
        }
    }

    /// 生成状态样式
    fn generate_status_style(&self) -> String {
        match self.status {
            InputStatus::Error => css!(
                r#"
                .ant-input-status-error {
                    border-color: #ff4d4f;
                }

                .ant-input-status-error:hover {
                    border-color: #ff4d4f;
                    border-inline-end-width: 1px;
                }

                .ant-input-status-error:focus,
                .ant-input-status-error.ant-input-focused {
                    border-color: #ff4d4f;
                    box-shadow: 0 0 0 2px rgba(255, 77, 79, 0.1);
                    border-inline-end-width: 1px;
                    outline: 0;
                }

                .ant-input-affix-wrapper.ant-input-status-error {
                    border-color: #ff4d4f;
                }

                .ant-input-affix-wrapper.ant-input-status-error:hover {
                    border-color: #ff4d4f;
                    border-inline-end-width: 1px;
                }

                .ant-input-affix-wrapper.ant-input-status-error:focus,
                .ant-input-affix-wrapper.ant-input-status-error.ant-input-affix-wrapper-focused {
                    border-color: #ff4d4f;
                    box-shadow: 0 0 0 2px rgba(255, 77, 79, 0.1);
                    border-inline-end-width: 1px;
                    outline: 0;
                }
                "#
            ),
            InputStatus::Warning => css!(
                r#"
                .ant-input-status-warning {
                    border-color: #faad14;
                }

                .ant-input-status-warning:hover {
                    border-color: #faad14;
                    border-inline-end-width: 1px;
                }

                .ant-input-status-warning:focus,
                .ant-input-status-warning.ant-input-focused {
                    border-color: #faad14;
                    box-shadow: 0 0 0 2px rgba(250, 173, 20, 0.1);
                    border-inline-end-width: 1px;
                    outline: 0;
                }

                .ant-input-affix-wrapper.ant-input-status-warning {
                    border-color: #faad14;
                }

                .ant-input-affix-wrapper.ant-input-status-warning:hover {
                    border-color: #faad14;
                    border-inline-end-width: 1px;
                }

                .ant-input-affix-wrapper.ant-input-status-warning:focus,
                .ant-input-affix-wrapper.ant-input-status-warning.ant-input-affix-wrapper-focused {
                    border-color: #faad14;
                    box-shadow: 0 0 0 2px rgba(250, 173, 20, 0.1);
                    border-inline-end-width: 1px;
                    outline: 0;
                }
                "#
            ),
            InputStatus::Default => String::new(),
        }
    }

    /// 生成禁用样式
    fn generate_disabled_style(&self) -> String {
        css!(
            r#"
            .ant-input-disabled {
                color: rgba(0, 0, 0, 0.25);
                background-color: rgba(0, 0, 0, 0.04);
                border-color: #d9d9d9;
                box-shadow: none;
                cursor: not-allowed;
                opacity: 1;
            }

            .ant-input-disabled:hover {
                border-color: #d9d9d9;
                border-inline-end-width: 1px;
            }

            .ant-input[disabled] {
                color: rgba(0, 0, 0, 0.25);
                background-color: rgba(0, 0, 0, 0.04);
                border-color: #d9d9d9;
                box-shadow: none;
                cursor: not-allowed;
                opacity: 1;
            }

            .ant-input[disabled]:hover {
                border-color: #d9d9d9;
                border-inline-end-width: 1px;
            }

            .ant-input-affix-wrapper-disabled {
                color: rgba(0, 0, 0, 0.25);
                background-color: rgba(0, 0, 0, 0.04);
                border-color: #d9d9d9;
                box-shadow: none;
                cursor: not-allowed;
                opacity: 1;
            }

            .ant-input-affix-wrapper-disabled:hover {
                border-color: #d9d9d9;
                border-inline-end-width: 1px;
            }
            "#
        )
    }

    /// 生成无边框样式
    fn generate_borderless_style(&self) -> String {
        css!(
            r#"
            .ant-input-borderless,
            .ant-input-borderless:hover,
            .ant-input-borderless:focus,
            .ant-input-borderless-focused,
            .ant-input-borderless-disabled,
            .ant-input-borderless[disabled] {
                background-color: transparent;
                border: none;
                box-shadow: none;
            }

            .ant-input-affix-wrapper-borderless,
            .ant-input-affix-wrapper-borderless:hover,
            .ant-input-affix-wrapper-borderless:focus,
            .ant-input-affix-wrapper-borderless-focused,
            .ant-input-affix-wrapper-borderless-disabled {
                background-color: transparent;
                border: none;
                box-shadow: none;
            }
            "#
        )
    }

    /// 生成前缀后缀样式
    fn generate_affix_style(&self) -> String {
        css!(
            r#"
            .ant-input-affix-wrapper {
                box-sizing: border-box;
                margin: 0;
                padding: 0;
                color: rgba(0, 0, 0, 0.88);
                font-size: 14px;
                line-height: 1.5714285714285714;
                list-style: none;
                font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Noto Sans', sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol', 'Noto Color Emoji';
                position: relative;
                display: inline-flex;
                width: 100%;
                min-width: 0;
                padding: 4px 11px;
                background-color: #ffffff;
                background-image: none;
                border-width: 1px;
                border-style: solid;
                border-color: #d9d9d9;
                border-radius: 6px;
                transition: all 0.2s;
            }

            .ant-input-affix-wrapper>input.ant-input {
                padding: 0;
                font-size: inherit;
                border: none;
                border-radius: 0;
                outline: none;
                background-color: transparent;
                color: inherit;
            }

            .ant-input-affix-wrapper>input.ant-input:focus {
                box-shadow: none !important;
            }

            .ant-input-affix-wrapper:hover {
                border-color: #4096ff;
                border-inline-end-width: 1px;
            }

            .ant-input-affix-wrapper:focus,
            .ant-input-affix-wrapper-focused {
                border-color: #4096ff;
                box-shadow: 0 0 0 2px rgba(5, 145, 255, 0.1);
                border-inline-end-width: 1px;
                outline: 0;
            }

            .ant-input-prefix {
                margin-right: 4px;
                display: flex;
                flex: none;
                align-items: center;
            }

            .ant-input-suffix {
                margin-left: 4px;
                display: flex;
                flex: none;
                align-items: center;
            }
            "#
        )
    }

    /// 生成清除按钮样式
    fn generate_clear_style(&self) -> String {
        css!(
            r#"
            .ant-input-clear-icon {
                margin: 0;
                color: rgba(0, 0, 0, 0.25);
                font-size: 12px;
                vertical-align: -1px;
                cursor: pointer;
                transition: color 0.3s;
                display: flex;
                align-items: center;
                justify-content: center;
            }

            .ant-input-clear-icon:hover {
                color: rgba(0, 0, 0, 0.45);
            }

            .ant-input-affix-wrapper:hover .ant-input-clear-icon {
                opacity: 1;
            }
            "#
        )
    }

    /// 生成字数统计样式
    fn generate_count_style(&self) -> String {
        css!(
            r#"
            .ant-input-show-count-suffix {
                color: rgba(0, 0, 0, 0.45);
                white-space: nowrap;
                pointer-events: none;
                margin-left: 4px;
            }
            "#
        )
    }

    /// 生成输入组样式
    fn generate_group_style(&self) -> String {
        css!(
            r#"
            .ant-input-group-wrapper {
                display: inline-block;
                width: 100%;
                text-align: start;
                vertical-align: top;
            }

            .ant-input-group {
                box-sizing: border-box;
                margin: 0;
                padding: 0;
                color: rgba(0, 0, 0, 0.88);
                font-size: 14px;
                line-height: 1.5714285714285714;
                list-style: none;
                font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Noto Sans', sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol', 'Noto Color Emoji';
                position: relative;
                display: table;
                width: 100%;
                border-collapse: separate;
                border-spacing: 0;
            }

            .ant-input-group>.ant-input:first-child,
            .ant-input-group .ant-input-affix-wrapper:first-child {
                border-top-right-radius: 0;
                border-bottom-right-radius: 0;
            }

            .ant-input-group>.ant-input:last-child,
            .ant-input-group .ant-input-affix-wrapper:last-child {
                border-top-left-radius: 0;
                border-bottom-left-radius: 0;
            }

            .ant-input-group-addon {
                position: relative;
                padding: 0 11px;
                color: rgba(0, 0, 0, 0.88);
                font-weight: normal;
                font-size: 14px;
                text-align: center;
                background-color: rgba(0, 0, 0, 0.02);
                border: 1px solid #d9d9d9;
                border-radius: 6px;
                transition: all 0.2s;
                display: table-cell;
                width: 1px;
                white-space: nowrap;
                vertical-align: middle;
            }

            .ant-input-group-addon:first-child {
                border-right: 0;
                border-top-right-radius: 0;
                border-bottom-right-radius: 0;
            }

            .ant-input-group-addon:last-child {
                border-left: 0;
                border-top-left-radius: 0;
                border-bottom-left-radius: 0;
            }

            .ant-input-group>.ant-input {
                float: left;
                width: 100%;
                margin-bottom: 0;
                text-align: inherit;
            }

            .ant-input-group>.ant-input:not(:first-child):not(:last-child) {
                border-radius: 0;
            }

            .ant-input-group .ant-input-affix-wrapper {
                float: left;
                width: 100%;
                margin-bottom: 0;
            }

            .ant-input-group .ant-input-affix-wrapper:not(:first-child) {
                border-top-left-radius: 0;
                border-bottom-left-radius: 0;
            }

            .ant-input-group .ant-input-affix-wrapper:not(:last-child) {
                border-top-right-radius: 0;
                border-bottom-right-radius: 0;
            }
            "#
        )
    }

    /// 生成响应式样式
    fn generate_responsive_style(&self) -> String {
        css!(
            r#"
            @media (max-width: 575px) {
                .ant-input-affix-wrapper.ant-input-lg {
                    padding: 6.5px 11px;
                }

                .ant-input-affix-wrapper.ant-input-sm {
                    padding: 0px 7px;
                }
            }
            "#
        )
    }

    /// 生成高对比度样式
    fn generate_high_contrast_style(&self) -> String {
        css!(
            r#"
            @media (prefers-contrast: high) {
                .ant-input {
                    border: 1px solid transparent;
                }

                .ant-input:focus,
                .ant-input-focused {
                    outline: 2px solid currentcolor;
                }
            }
            "#
        )
    }

    /// 生成暗色主题样式
    fn generate_dark_theme_style(&self) -> String {
        css!(
            r#"
            @media (prefers-color-scheme: dark) {
                .ant-input {
                    background-color: #141414;
                    border-color: #424242;
                    color: rgba(255, 255, 255, 0.85);
                }

                .ant-input::placeholder {
                    color: rgba(255, 255, 255, 0.3);
                }

                .ant-input:hover {
                    border-color: #165996;
                }

                .ant-input:focus,
                .ant-input-focused {
                    border-color: #177ddc;
                    box-shadow: 0 0 0 2px rgba(23, 125, 220, 0.2);
                }

                .ant-input-disabled {
                    background-color: rgba(255, 255, 255, 0.08);
                    color: rgba(255, 255, 255, 0.3);
                }

                .ant-input-affix-wrapper {
                    background-color: #141414;
                    border-color: #424242;
                    color: rgba(255, 255, 255, 0.85);
                }

                .ant-input-group-addon {
                    background-color: rgba(255, 255, 255, 0.08);
                    border-color: #424242;
                    color: rgba(255, 255, 255, 0.85);
                }

                .ant-input-clear-icon {
                    color: rgba(255, 255, 255, 0.3);
                }

                .ant-input-clear-icon:hover {
                    color: rgba(255, 255, 255, 0.45);
                }
            }
            "#
        )
    }
}

/// 生成输入框样式
pub fn generate_input_style(
    size: InputSize,
    status: InputStatus,
    disabled: bool,
    bordered: bool,
    allow_clear: bool,
    prefix: bool,
    suffix: bool,
    show_count: bool,
    addon_before: bool,
    addon_after: bool,
    focused: bool,
) -> String {
    InputStyleGenerator::new()
        .with_size(size)
        .with_status(status)
        .with_disabled(disabled)
        .with_bordered(bordered)
        .with_allow_clear(allow_clear)
        .with_prefix(prefix)
        .with_suffix(suffix)
        .with_show_count(show_count)
        .with_addon_before(addon_before)
        .with_addon_after(addon_after)
        .with_focused(focused)
        .generate_css()
}

/// 默认输入框样式
pub fn default_input_style() -> String {
    generate_input_style(
        InputSize::default(),
        InputStatus::default(),
        false,
        true,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
    )
}

/// 生成文本域样式
pub fn generate_textarea_style(
    size: InputSize,
    status: InputStatus,
    disabled: bool,
    bordered: bool,
    show_count: bool,
    resize: bool,
    auto_size: bool,
    focused: bool,
) -> String {
    let mut generator = InputStyleGenerator::new()
        .with_size(size)
        .with_status(status)
        .with_disabled(disabled)
        .with_bordered(bordered)
        .with_show_count(show_count)
        .with_focused(focused);

    let mut css = generator.generate_css();
    
    // 添加文本域特定样式
    css.push_str(&css!(
        r#"
        .ant-input {
            min-height: 32px;
            line-height: 1.5714285714285714;
            vertical-align: bottom;
            transition: all 0.2s, height 0s;
        }
        
        .ant-input.ant-input-lg {
            min-height: 40px;
            padding: 6.5px 11px;
            font-size: 16px;
        }
        
        .ant-input.ant-input-sm {
            min-height: 24px;
            padding: 0px 7px;
        }
        "#
    ));
    
    if !resize {
        css.push_str(&css!(
            r#"
            .ant-input {
                resize: none;
            }
            "#
        ));
    }
    
    if auto_size {
        css.push_str(&css!(
            r#"
            .ant-input {
                overflow: hidden;
                resize: none;
            }
            "#
        ));
    }
    
    css
}

/// 生成搜索框样式
pub fn generate_search_style(
    size: InputSize,
    status: InputStatus,
    disabled: bool,
    bordered: bool,
    loading: bool,
    focused: bool,
) -> String {
    let mut generator = InputStyleGenerator::new()
        .with_size(size)
        .with_status(status)
        .with_disabled(disabled)
        .with_bordered(bordered)
        .with_suffix(true)
        .with_focused(focused);

    let mut css = generator.generate_css();
    
    // 添加搜索框特定样式
    css.push_str(&css!(
        r#"
        .ant-input-search {
            position: relative;
        }
        
        .ant-input-search .ant-input-suffix {
            padding-inline-start: 8px;
        }
        
        .ant-input-search .ant-input-search-button {
            border-start-start-radius: 0;
            border-end-start-radius: 0;
            border-inline-start: 0;
        }
        
        .ant-input-search .ant-input-search-button:not(.ant-btn-primary) {
            color: rgba(0, 0, 0, 0.45);
        }
        
        .ant-input-search .ant-input-search-button:not(.ant-btn-primary):hover {
            border-inline-start: 0;
        }
        
        .ant-input-search-icon {
            color: rgba(0, 0, 0, 0.45);
            cursor: pointer;
            transition: all 0.3s;
        }
        
        .ant-input-search-icon:hover {
            color: rgba(0, 0, 0, 0.88);
        }
        "#
    ));
    
    if loading {
        css.push_str(&css!(
            r#"
            .ant-input-search-loading {
                color: #1890ff;
            }
            "#
        ));
    }
    
    css
}

/// 生成密码框样式
pub fn generate_password_style(
    size: InputSize,
    status: InputStatus,
    disabled: bool,
    bordered: bool,
    visibility_toggle: bool,
    focused: bool,
) -> String {
    let mut generator = InputStyleGenerator::new()
        .with_size(size)
        .with_status(status)
        .with_disabled(disabled)
        .with_bordered(bordered)
        .with_suffix(visibility_toggle)
        .with_focused(focused);

    let mut css = generator.generate_css();
    
    // 添加密码框特定样式
    css.push_str(&css!(
        r#"
        .ant-input-password {
            position: relative;
        }
        
        .ant-input-password-icon {
            color: rgba(0, 0, 0, 0.45);
            cursor: pointer;
            transition: all 0.3s;
        }
        
        .ant-input-password-icon:hover {
            color: rgba(0, 0, 0, 0.88);
        }
        "#
    ));
    
    css
}

/// 生成OTP输入框样式
pub fn generate_otp_style(
    size: InputSize,
    status: InputStatus,
    disabled: bool,
    length: usize,
    focused_index: Option<usize>,
) -> String {
    let mut generator = InputStyleGenerator::new()
        .with_size(size)
        .with_status(status)
        .with_disabled(disabled)
        .with_bordered(true);

    let mut css = generator.generate_css();
    
    // 添加OTP特定样式
    css.push_str(&css!(
        r#"
        .ant-input-otp {
            display: inline-flex;
            gap: 8px;
        }
        
        .ant-input-otp .ant-input {
            width: 40px;
            text-align: center;
            padding-inline: 0;
        }
        
        .ant-input-otp .ant-input.ant-input-lg {
            width: 48px;
        }
        
        .ant-input-otp .ant-input.ant-input-sm {
            width: 32px;
        }
        
        .ant-input-otp .ant-input:focus {
            z-index: 1;
        }
        "#
    ));
    
    // 为每个输入框添加特定样式
    for i in 0..length {
        if Some(i) == focused_index {
            css.push_str(&css!(
                format!(
                    r#"
                    .ant-input-otp .ant-input:nth-child({}) {{
                        border-color: #4096ff;
                        box-shadow: 0 0 0 2px rgba(5, 145, 255, 0.1);
                        z-index: 1;
                    }}
                    "#,
                    i + 1
                )
            ));
        }
    }
    
    css
}

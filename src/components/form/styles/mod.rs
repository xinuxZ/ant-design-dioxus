//! Form 表单组件样式模块
//!
//! 本模块包含 Form 组件的所有样式定义，从组件逻辑中分离出来，
//! 提高代码的可维护性和复用性。

use css_in_rust::css;
use serde::{Deserialize, Serialize};

/// 表单布局类型
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum FormLayout {
    /// 水平布局
    Horizontal,
    /// 垂直布局
    Vertical,
    /// 内联布局
    Inline,
}

impl Default for FormLayout {
    fn default() -> Self {
        FormLayout::Horizontal
    }
}

/// 表单尺寸
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum FormSize {
    /// 小尺寸
    Small,
    /// 中等尺寸
    Middle,
    /// 大尺寸
    Large,
}

impl Default for FormSize {
    fn default() -> Self {
        FormSize::Middle
    }
}

/// 标签对齐方式
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum LabelAlign {
    /// 左对齐
    Left,
    /// 右对齐
    Right,
}

impl Default for LabelAlign {
    fn default() -> Self {
        LabelAlign::Right
    }
}

/// 校验状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ValidateStatus {
    /// 成功
    Success,
    /// 警告
    Warning,
    /// 错误
    Error,
    /// 校验中
    Validating,
}

/// 表单样式生成器
pub struct FormStyleGenerator {
    /// 表单布局
    pub layout: FormLayout,
    /// 表单尺寸
    pub size: FormSize,
    /// 标签对齐方式
    pub label_align: LabelAlign,
    /// 是否禁用
    pub disabled: bool,
}

impl FormStyleGenerator {
    /// 创建新的表单样式生成器
    pub fn new() -> Self {
        Self {
            layout: FormLayout::default(),
            size: FormSize::default(),
            label_align: LabelAlign::default(),
            disabled: false,
        }
    }

    /// 设置表单布局
    pub fn with_layout(mut self, layout: FormLayout) -> Self {
        self.layout = layout;
        self
    }

    /// 设置表单尺寸
    pub fn with_size(mut self, size: FormSize) -> Self {
        self.size = size;
        self
    }

    /// 设置标签对齐方式
    pub fn with_label_align(mut self, label_align: LabelAlign) -> Self {
        self.label_align = label_align;
        self
    }

    /// 设置是否禁用
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// 生成完整的表单样式
    pub fn generate(&self) -> String {
        let mut classes = vec![self.base_style()];

        // 布局样式
        match self.layout {
            FormLayout::Horizontal => classes.push(self.horizontal_layout_style()),
            FormLayout::Vertical => classes.push(self.vertical_layout_style()),
            FormLayout::Inline => classes.push(self.inline_layout_style()),
        }

        // 尺寸样式
        match self.size {
            FormSize::Small => classes.push(self.small_size_style()),
            FormSize::Middle => {}
            FormSize::Large => classes.push(self.large_size_style()),
        }

        // 禁用样式
        if self.disabled {
            classes.push(self.disabled_style());
        }

        // 标签对齐样式
        match self.label_align {
            LabelAlign::Left => classes.push(self.label_align_left_style()),
            LabelAlign::Right => classes.push(self.label_align_right_style()),
        }

        classes.join(" ")
    }

    /// 基础样式
    fn base_style(&self) -> String {
        css!(
            r#"
            box-sizing: border-box;
            margin: 0;
            padding: 0;
            color: rgba(0, 0, 0, 0.88);
            font-size: 14px;
            line-height: 1.5714285714285714;
            list-style: none;
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Noto Sans', sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol', 'Noto Color Emoji';
            "#
        )
    }

    /// 水平布局样式
    fn horizontal_layout_style(&self) -> String {
        css!(
            r#"
            &.ant-form-horizontal .ant-form-item-label {
                text-align: right;
            }

            &.ant-form-horizontal .ant-form-item-label label {
                height: 32px;
                line-height: 32px;
            }
            "#
        )
    }

    /// 垂直布局样式
    fn vertical_layout_style(&self) -> String {
        css!(
            r#"
            &.ant-form-vertical .ant-form-item-label {
                padding: 0 0 8px;
            }

            &.ant-form-vertical .ant-form-item-label label {
                height: auto;
                line-height: 1.5714285714285714;
            }
            "#
        )
    }

    /// 内联布局样式
    fn inline_layout_style(&self) -> String {
        css!(
            r#"
            &.ant-form-inline {
                display: flex;
                flex-wrap: wrap;
            }

            &.ant-form-inline .ant-form-item {
                flex: none;
                margin-right: 16px;
                margin-bottom: 0;
            }

            &.ant-form-inline .ant-form-item-with-help {
                margin-bottom: 24px;
            }

            &.ant-form-inline .ant-form-item>.ant-form-item-label,
            &.ant-form-inline .ant-form-item>.ant-form-item-control {
                display: inline-block;
                vertical-align: top;
            }

            &.ant-form-inline .ant-form-item>.ant-form-item-label {
                padding-right: 8px;
                white-space: nowrap;
            }

            &.ant-form-inline .ant-form-item>.ant-form-item-control {
                flex: 1 1 auto;
                min-width: 0;
            }
            "#
        )
    }

    /// 小尺寸样式
    fn small_size_style(&self) -> String {
        css!(
            r#"
            &.ant-form-small .ant-form-item-label>label {
                height: 24px;
            }

            &.ant-form-small .ant-form-item-control-input {
                min-height: 24px;
            }
            "#
        )
    }

    /// 大尺寸样式
    fn large_size_style(&self) -> String {
        css!(
            r#"
            &.ant-form-large .ant-form-item-label>label {
                height: 40px;
            }

            &.ant-form-large .ant-form-item-control-input {
                min-height: 40px;
            }
            "#
        )
    }

    /// 禁用样式
    fn disabled_style(&self) -> String {
        css!(
            r#"
            &.ant-form-disabled .ant-form-item-label>label {
                color: rgba(0, 0, 0, 0.25);
                cursor: not-allowed;
            }

            &.ant-form-disabled .ant-input,
            &.ant-form-disabled .ant-input:hover,
            &.ant-form-disabled .ant-input:focus,
            &.ant-form-disabled .ant-input-affix-wrapper,
            &.ant-form-disabled .ant-input-affix-wrapper:hover,
            &.ant-form-disabled .ant-input-affix-wrapper:focus {
                color: rgba(0, 0, 0, 0.25);
                background-color: rgba(0, 0, 0, 0.04);
                border-color: #d9d9d9;
                box-shadow: none;
                cursor: not-allowed;
                opacity: 1;
            }
            "#
        )
    }

    /// 标签左对齐样式
    fn label_align_left_style(&self) -> String {
        css!(
            r#"
            .ant-form-item-label {
                text-align: left;
            }
            "#
        )
    }

    /// 标签右对齐样式
    fn label_align_right_style(&self) -> String {
        css!(
            r#"
            .ant-form-item-label {
                text-align: right;
            }
            "#
        )
    }
}

/// 表单项样式生成器
pub struct FormItemStyleGenerator {
    /// 校验状态
    pub status: Option<ValidateStatus>,
    /// 是否必填
    pub required: bool,
    /// 是否显示冒号
    pub colon: bool,
}

impl FormItemStyleGenerator {
    /// 创建新的表单项样式生成器
    pub fn new() -> Self {
        Self {
            status: None,
            required: false,
            colon: true,
        }
    }

    /// 设置校验状态
    pub fn with_status(mut self, status: Option<ValidateStatus>) -> Self {
        self.status = status;
        self
    }

    /// 设置是否必填
    pub fn with_required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    /// 设置是否显示冒号
    pub fn with_colon(mut self, colon: bool) -> Self {
        self.colon = colon;
        self
    }

    /// 生成完整的表单项样式
    pub fn generate(&self) -> String {
        let mut classes = vec![self.base_style()];

        // 状态样式
        if let Some(status) = &self.status {
            match status {
                ValidateStatus::Success => classes.push(self.success_style()),
                ValidateStatus::Warning => classes.push(self.warning_style()),
                ValidateStatus::Error => classes.push(self.error_style()),
                ValidateStatus::Validating => classes.push(self.validating_style()),
            }
        }

        // 必填样式
        if self.required {
            classes.push(self.required_style());
        }

        // 冒号样式
        if !self.colon {
            classes.push(self.no_colon_style());
        }

        classes.join(" ")
    }

    /// 基础样式
    fn base_style(&self) -> String {
        css!(
            r#"
            margin-bottom: 24px;
            vertical-align: top;

            .ant-form-item-row {
                display: flex;
                flex-flow: row wrap;
                min-width: 0;
            }

            .ant-form-item-label {
                flex-grow: 0;
                overflow: hidden;
                white-space: nowrap;
                vertical-align: top;
            }

            .ant-form-item-label>label {
                position: relative;
                display: inline-flex;
                align-items: center;
                max-width: 100%;
                height: 32px;
                color: rgba(0, 0, 0, 0.88);
                font-size: 14px;
                cursor: default;
            }

            .ant-form-item-label>label::after {
                content: ':';
                position: relative;
                top: -0.5px;
                margin: 0 8px 0 2px;
            }

            .ant-form-item-control {
                display: flex;
                flex-direction: column;
                flex-grow: 1;
                min-width: 0;
            }

            .ant-form-item-control-input {
                position: relative;
                display: flex;
                align-items: center;
                min-height: 32px;
            }

            .ant-form-item-control-input-content {
                flex: auto;
                max-width: 100%;
            }

            .ant-form-item-explain {
                clear: both;
                min-height: 24px;
                margin-top: 4px;
                color: rgba(0, 0, 0, 0.45);
                font-size: 14px;
                line-height: 1.5714285714285714;
                transition: color 0.3s cubic-bezier(0.215, 0.61, 0.355, 1);
            }

            .ant-form-item-extra {
                clear: both;
                min-height: 24px;
                margin-top: 4px;
                color: rgba(0, 0, 0, 0.45);
                font-size: 14px;
                line-height: 1.5714285714285714;
                transition: color 0.3s cubic-bezier(0.215, 0.61, 0.355, 1);
            }
            "#
        )
    }

    /// 成功状态样式
    fn success_style(&self) -> String {
        css!(
            r#"
            &.ant-form-item-has-success .ant-input,
            &.ant-form-item-has-success .ant-input:hover {
                border-color: #52c41a;
            }

            &.ant-form-item-has-success .ant-input:focus,
            &.ant-form-item-has-success .ant-input-focused {
                border-color: #73d13d;
                box-shadow: 0 0 0 2px rgba(82, 196, 26, 0.2);
                border-right-width: 1px;
                outline: 0;
            }

            .ant-form-item-explain-success {
                color: #52c41a;
            }
            "#
        )
    }

    /// 警告状态样式
    fn warning_style(&self) -> String {
        css!(
            r#"
            &.ant-form-item-has-warning .ant-input,
            &.ant-form-item-has-warning .ant-input:hover {
                border-color: #faad14;
            }

            &.ant-form-item-has-warning .ant-input:focus,
            &.ant-form-item-has-warning .ant-input-focused {
                border-color: #ffc53d;
                box-shadow: 0 0 0 2px rgba(250, 173, 20, 0.2);
                border-right-width: 1px;
                outline: 0;
            }

            .ant-form-item-explain-warning {
                color: #faad14;
            }
            "#
        )
    }

    /// 错误状态样式
    fn error_style(&self) -> String {
        css!(
            r#"
            &.ant-form-item-has-error .ant-form-item-control-input {
                border-color: #ff4d4f;
            }

            &.ant-form-item-has-error .ant-input,
            &.ant-form-item-has-error .ant-input:hover {
                border-color: #ff4d4f;
            }

            &.ant-form-item-has-error .ant-input:focus,
            &.ant-form-item-has-error .ant-input-focused {
                border-color: #ff7875;
                box-shadow: 0 0 0 2px rgba(255, 77, 79, 0.2);
                border-right-width: 1px;
                outline: 0;
            }

            .ant-form-item-explain-error {
                color: #ff4d4f;
            }
            "#
        )
    }

    /// 校验中状态样式
    fn validating_style(&self) -> String {
        css!(
            r#"
            &.ant-form-item-is-validating .ant-form-item-control-input::after {
                position: absolute;
                top: 50%;
                right: 8px;
                z-index: 1;
                width: 14px;
                height: 14px;
                margin-top: -7px;
                color: #1890ff;
                line-height: 1;
                text-align: center;
                content: '';
                animation: loadingCircle 1s infinite linear;
            }

            @keyframes loadingCircle {
                100% {
                    transform: rotate(360deg);
                }
            }
            "#
        )
    }

    /// 必填样式
    fn required_style(&self) -> String {
        css!(
            r#"
            .ant-form-item-label>label.ant-form-item-required:not(.ant-form-item-required-mark-optional)::before {
                display: inline-block;
                margin-right: 4px;
                color: #ff4d4f;
                font-size: 14px;
                font-family: SimSun, sans-serif;
                line-height: 1;
                content: '*';
            }
            "#
        )
    }

    /// 无冒号样式
    fn no_colon_style(&self) -> String {
        css!(
            r#"
            .ant-form-item-label>label.ant-form-item-no-colon::after {
                content: '';
            }
            "#
        )
    }
}

/// 响应式样式
pub fn form_responsive_style() -> String {
    css!(
        r#"
        @media (max-width: 575px) {
            .ant-form-horizontal .ant-form-item-label {
                padding: 0 0 8px;
                text-align: left;
            }

            .ant-form-horizontal .ant-form-item-row {
                flex-direction: column;
            }

            .ant-form-horizontal .ant-form-item-label,
            .ant-form-horizontal .ant-form-item-control {
                flex: 0 0 auto;
                max-width: 100%;
                width: 100%;
            }

            .ant-form-inline {
                flex-direction: column;
            }

            .ant-form-inline .ant-form-item {
                margin-right: 0;
                margin-bottom: 24px;
                width: 100%;
            }
        }
        "#
    )
}

/// 暗色主题样式
pub fn form_dark_theme_style() -> String {
    css!(
        r#"
        @media (prefers-color-scheme: dark) {
            .ant-form {
                color: rgba(255, 255, 255, 0.85);
            }

            .ant-form-item-label>label {
                color: rgba(255, 255, 255, 0.85);
            }

            .ant-form-item-explain {
                color: rgba(255, 255, 255, 0.45);
            }

            .ant-form-item-extra {
                color: rgba(255, 255, 255, 0.45);
            }

            .ant-form-disabled .ant-form-item-label>label {
                color: rgba(255, 255, 255, 0.25);
            }

            .ant-form-disabled .ant-input,
            .ant-form-disabled .ant-input:hover,
            .ant-form-disabled .ant-input:focus,
            .ant-form-disabled .ant-input-affix-wrapper,
            .ant-form-disabled .ant-input-affix-wrapper:hover,
            .ant-form-disabled .ant-input-affix-wrapper:focus {
                color: rgba(255, 255, 255, 0.25);
                background-color: rgba(255, 255, 255, 0.08);
                border-color: #434343;
            }
        }
        "#
    )
}

/// 高对比度样式
pub fn form_high_contrast_style() -> String {
    css!(
        r#"
        @media (prefers-contrast: high) {
            .ant-form-item-has-error .ant-input,
            .ant-form-item-has-error .ant-input:hover,
            .ant-form-item-has-error .ant-input:focus {
                border-color: #000;
                border-width: 2px;
            }

            .ant-form-item-has-success .ant-input,
            .ant-form-item-has-success .ant-input:hover,
            .ant-form-item-has-success .ant-input:focus {
                border-color: #000;
                border-width: 2px;
            }

            .ant-form-item-has-warning .ant-input,
            .ant-form-item-has-warning .ant-input:hover,
            .ant-form-item-has-warning .ant-input:focus {
                border-color: #000;
                border-width: 2px;
            }
        }
        "#
    )
}

/// 减少动画样式
pub fn form_reduced_motion_style() -> String {
    css!(
        r#"
        @media (prefers-reduced-motion: reduce) {
            .ant-form-item-explain {
                transition: none;
            }

            .ant-form-item-extra {
                transition: none;
            }

            .ant-form-item-is-validating .ant-form-item-control-input::after {
                animation: none;
            }
        }
        "#
    )
}

/// 可访问性样式
pub fn form_a11y_style() -> String {
    css!(
        r#"
        .ant-form-item-label>label:focus-visible {
            outline: 2px solid #1890ff;
            outline-offset: 1px;
        }
        "#
    )
}

/// 打印样式
pub fn form_print_style() -> String {
    css!(
        r#"
        @media print {
            .ant-form {
                color: #000;
            }

            .ant-form-item-explain,
            .ant-form-item-extra {
                display: none;
            }

            .ant-form-item-is-validating .ant-form-item-control-input::after {
                display: none;
            }
        }
        "#
    )
}

/// 生成完整的表单样式
pub fn generate_form_style(
    layout: FormLayout,
    size: FormSize,
    label_align: LabelAlign,
    disabled: bool,
) -> String {
    format!(
        "{} {} {} {} {} {} {}",
        FormStyleGenerator::new()
            .with_layout(layout)
            .with_size(size)
            .with_label_align(label_align)
            .with_disabled(disabled)
            .generate(),
        form_responsive_style(),
        form_dark_theme_style(),
        form_high_contrast_style(),
        form_reduced_motion_style(),
        form_a11y_style(),
        form_print_style(),
    )
}

/// 生成表单项样式
pub fn generate_form_item_style(
    status: Option<ValidateStatus>,
    required: bool,
    colon: bool,
) -> String {
    FormItemStyleGenerator::new()
        .with_status(status)
        .with_required(required)
        .with_colon(colon)
        .generate()
}

//! Form 表单组件样式模块
//!
//! 本模块包含 Form 组件的所有样式定义，从组件逻辑中分离出来，
//! 提高代码的可维护性和复用性。

use crate::theme::AliasToken;
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
    /// 主题令牌
    pub token: AliasToken,
}

impl FormStyleGenerator {
    /// 创建新的表单样式生成器
    pub fn new() -> Self {
        Self {
            layout: FormLayout::default(),
            size: FormSize::default(),
            label_align: LabelAlign::default(),
            disabled: false,
            token: AliasToken::default(),
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

    /// 设置主题令牌
    pub fn with_token(mut self, token: AliasToken) -> Self {
        self.token = token;
        self
    }

    /// 生成完整的表单样式类名
    pub fn generate(&self) -> String {
        let mut classes = vec!["ant-form".to_string()];

        // 布局样式
        match self.layout {
            FormLayout::Horizontal => classes.push("ant-form-horizontal".to_string()),
            FormLayout::Vertical => classes.push("ant-form-vertical".to_string()),
            FormLayout::Inline => classes.push("ant-form-inline".to_string()),
        }

        // 尺寸样式
        match self.size {
            FormSize::Small => classes.push("ant-form-small".to_string()),
            FormSize::Middle => {}
            FormSize::Large => classes.push("ant-form-large".to_string()),
        }

        // 禁用样式
        if self.disabled {
            classes.push("ant-form-disabled".to_string());
        }

        // 标签对齐样式
        match self.label_align {
            LabelAlign::Left => classes.push("ant-form-label-left".to_string()),
            LabelAlign::Right => classes.push("ant-form-label-right".to_string()),
        }

        classes.join(" ")
    }

    /// 生成 CSS 样式
    pub fn generate_css(&self) -> String {
        css!(
            r#"
            .ant-form {
                box-sizing: border-box;
                margin: 0;
                padding: 0;
                color: ${color_text};
                font-size: ${font_size}px;
                line-height: ${line_height};
                list-style: none;
                font-family: ${font_family};
            }

            .ant-form-horizontal .ant-form-item-label {
                flex-grow: 0;
                overflow: hidden;
                white-space: nowrap;
                text-align: right;
                vertical-align: middle;
            }

            .ant-form-horizontal .ant-form-item-label > label {
                height: ${height}px;
                margin: 0;
                line-height: ${height}px;
            }

            .ant-form-vertical .ant-form-item-label {
                padding: 0 0 ${padding_xs}px;
            }

            .ant-form-vertical .ant-form-item-label > label {
                height: auto;
                margin: 0;
            }

            .ant-form-inline {
                display: flex;
                flex-wrap: wrap;
            }

            .ant-form-inline .ant-form-item {
                flex: none;
                margin-right: ${margin_lg}px;
                margin-bottom: 0;
            }

            .ant-form-inline .ant-form-item-with-help {
                margin-bottom: ${margin_sm}px;
            }

            .ant-form-inline .ant-form-item > .ant-form-item-label,
            .ant-form-inline .ant-form-item > .ant-form-item-control {
                display: inline-block;
                vertical-align: top;
            }

            .ant-form-small .ant-form-item-label > label {
                height: ${height_sm}px;
                line-height: ${height_sm}px;
            }

            .ant-form-small .ant-form-item {
                margin-bottom: ${margin_xs}px;
            }

            .ant-form-large .ant-form-item-label > label {
                height: ${height_lg}px;
                line-height: ${height_lg}px;
            }

            .ant-form-large .ant-form-item {
                margin-bottom: ${margin_md}px;
            }

            .ant-form-disabled .ant-form-item-label > label {
                color: ${color_text_disabled};
            }

            .ant-form-disabled .ant-form-item-control-input {
                cursor: not-allowed;
            }

            .ant-form-disabled .ant-form-item-control-input-content {
                color: ${color_text_disabled};
                background-color: ${color_bg_container_disabled};
                border-color: ${color_border};
                box-shadow: none;
                cursor: not-allowed;
                opacity: 1;
            }

            .ant-form-label-left .ant-form-item-label {
                text-align: left;
            }

            .ant-form-label-right .ant-form-item-label {
                text-align: right;
            }

            .ant-form-item {
                box-sizing: border-box;
                margin: 0;
                padding: 0;
                color: ${color_text};
                font-size: ${font_size}px;
                line-height: ${line_height};
                list-style: none;
                margin-bottom: ${margin_md}px;
                vertical-align: top;
            }

            .ant-form-item-label {
                display: inline-block;
                flex-grow: 0;
                overflow: hidden;
                white-space: nowrap;
                text-align: right;
                vertical-align: middle;
            }

            .ant-form-item-label > label {
                position: relative;
                display: inline-flex;
                align-items: center;
                height: ${height}px;
                color: ${color_text};
                font-size: ${font_size}px;
            }

            .ant-form-item-label > label::after {
                content: ':';
                position: relative;
                margin-block: 0;
                margin-inline-start: 2px;
                margin-inline-end: 8px;
            }

            .ant-form-item-label > label.ant-form-item-no-colon::after {
                content: ' ';
            }

            .ant-form-item-control {
                display: flex;
                flex-direction: column;
                flex-grow: 1;
            }

            .ant-form-item-control-input {
                position: relative;
                display: flex;
                align-items: center;
                min-height: ${height}px;
            }

            .ant-form-item-control-input-content {
                flex: auto;
                max-width: 100%;
            }

            .ant-form-item-explain,
            .ant-form-item-extra {
                clear: both;
                color: ${color_text_secondary};
                font-size: ${font_size_sm}px;
                line-height: ${line_height_sm};
            }

            .ant-form-item-explain {
                min-height: ${font_size_sm * line_height_sm}px;
            }

            .ant-form-item-with-help .ant-form-item-explain {
                height: auto;
                opacity: 1;
            }

            .ant-form-item-has-feedback .ant-form-item-control {
                padding-right: 24px;
            }

            .ant-form-item-feedback-icon {
                position: absolute;
                top: 50%;
                right: 0;
                display: flex;
                align-items: center;
                height: 20px;
                margin-top: -10px;
                font-size: ${font_size}px;
                line-height: 1;
            }

            .ant-form-item-has-success.ant-form-item-has-feedback .ant-form-item-feedback-icon {
                color: ${color_success};
            }

            .ant-form-item-has-warning.ant-form-item-has-feedback .ant-form-item-feedback-icon {
                color: ${color_warning};
            }

            .ant-form-item-has-error.ant-form-item-has-feedback .ant-form-item-feedback-icon {
                color: ${color_error};
            }

            .ant-form-item-has-validating.ant-form-item-has-feedback .ant-form-item-feedback-icon {
                color: ${color_primary};
            }

            .ant-form-item-has-success .ant-input,
            .ant-form-item-has-success .ant-input-affix-wrapper {
                border-color: ${color_success};
            }

            .ant-form-item-has-success .ant-input:hover,
            .ant-form-item-has-success .ant-input-affix-wrapper:hover {
                border-color: ${color_success_hover};
            }

            .ant-form-item-has-success .ant-input:focus,
            .ant-form-item-has-success .ant-input-affix-wrapper:focus,
            .ant-form-item-has-success .ant-input-focused,
            .ant-form-item-has-success .ant-input-affix-wrapper-focused {
                border-color: ${color_success};
                box-shadow: 0 0 0 2px ${color_success_outline};
            }

            .ant-form-item-has-warning .ant-input,
            .ant-form-item-has-warning .ant-input-affix-wrapper {
                border-color: ${color_warning};
            }

            .ant-form-item-has-warning .ant-input:hover,
            .ant-form-item-has-warning .ant-input-affix-wrapper:hover {
                border-color: ${color_warning_hover};
            }

            .ant-form-item-has-warning .ant-input:focus,
            .ant-form-item-has-warning .ant-input-affix-wrapper:focus,
            .ant-form-item-has-warning .ant-input-focused,
            .ant-form-item-has-warning .ant-input-affix-wrapper-focused {
                border-color: ${color_warning};
                box-shadow: 0 0 0 2px ${color_warning_outline};
            }

            .ant-form-item-has-error .ant-input,
            .ant-form-item-has-error .ant-input-affix-wrapper {
                border-color: ${color_error};
            }

            .ant-form-item-has-error .ant-input:hover,
            .ant-form-item-has-error .ant-input-affix-wrapper:hover {
                border-color: ${color_error_hover};
            }

            .ant-form-item-has-error .ant-input:focus,
            .ant-form-item-has-error .ant-input-affix-wrapper:focus,
            .ant-form-item-has-error .ant-input-focused,
            .ant-form-item-has-error .ant-input-affix-wrapper-focused {
                border-color: ${color_error};
                box-shadow: 0 0 0 2px ${color_error_outline};
            }

            .ant-form-item-required > .ant-form-item-label > label::before {
                display: inline-block;
                margin-right: 4px;
                color: ${color_error};
                font-size: ${font_size_sm}px;
                font-family: SimSun, sans-serif;
                line-height: 1;
                content: '*';
            }
            "#,
            color_text = self.token.color_text,
            font_size = self.token.font_size,
            line_height = self.token.line_height,
            font_family = self.token.font_family,
            height = self.token.height_base,
            padding_xs = self.token.padding_xs,
            margin_lg = self.token.margin_lg,
            margin_sm = self.token.margin_sm,
            height_sm = self.token.height_sm,
            margin_xs = self.token.margin_xs,
            height_lg = self.token.height_lg,
            margin_md = self.token.margin_md,
            color_text_disabled = self.token.color_text_disabled,
            color_bg_container_disabled = self.token.color_bg_container_disabled,
            color_border = self.token.color_border,
            color_text_secondary = self.token.color_text_secondary,
            font_size_sm = self.token.font_size_sm,
            line_height_sm = self.token.line_height_sm,
            color_success = self.token.color_success,
            color_warning = self.token.color_warning,
            color_error = self.token.color_error,
            color_primary = self.token.color_primary,
            color_success_hover = self.token.color_success_hover,
            color_success_outline = self.token.color_success_outline,
            color_warning_hover = self.token.color_warning_hover,
            color_warning_outline = self.token.color_warning_outline,
            color_error_hover = self.token.color_error_hover,
            color_error_outline = self.token.color_error_outline
        )
        .to_string()
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
    /// 主题令牌
    pub token: AliasToken,
}

impl FormItemStyleGenerator {
    /// 创建新的表单项样式生成器
    pub fn new() -> Self {
        Self {
            status: None,
            required: false,
            colon: true,
            token: AliasToken::default(),
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

    /// 设置主题令牌
    pub fn with_token(mut self, token: AliasToken) -> Self {
        self.token = token;
        self
    }

    /// 生成完整的表单项样式类名
    pub fn generate(&self) -> String {
        let mut classes = vec!["ant-form-item".to_string()];

        // 状态样式
        if let Some(status) = &self.status {
            match status {
                ValidateStatus::Success => classes.push("ant-form-item-has-success".to_string()),
                ValidateStatus::Warning => classes.push("ant-form-item-has-warning".to_string()),
                ValidateStatus::Error => classes.push("ant-form-item-has-error".to_string()),
                ValidateStatus::Validating => {
                    classes.push("ant-form-item-has-validating".to_string())
                }
            }
            classes.push("ant-form-item-has-feedback".to_string());
        }

        // 必填样式
        if self.required {
            classes.push("ant-form-item-required".to_string());
        }

        // 冒号样式
        if !self.colon {
            classes.push("ant-form-item-no-colon".to_string());
        }

        classes.join(" ")
    }
}

/// 生成表单样式
pub fn generate_form_style(
    layout: FormLayout,
    size: FormSize,
    label_align: LabelAlign,
    disabled: bool,
) -> String {
    FormStyleGenerator::new()
        .with_layout(layout)
        .with_size(size)
        .with_label_align(label_align)
        .with_disabled(disabled)
        .generate()
}

/// 生成表单 CSS 样式
pub fn generate_form_css(
    layout: FormLayout,
    size: FormSize,
    label_align: LabelAlign,
    disabled: bool,
) -> String {
    FormStyleGenerator::new()
        .with_layout(layout)
        .with_size(size)
        .with_label_align(label_align)
        .with_disabled(disabled)
        .generate_css()
}

/// 默认表单样式
pub fn default_form_style() -> String {
    FormStyleGenerator::new().generate()
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

/// 默认表单项样式
pub fn default_form_item_style() -> String {
    FormItemStyleGenerator::new().generate()
}

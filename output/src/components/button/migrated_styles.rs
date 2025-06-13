//! button 组件样式
//!
//! 此文件由样式迁移工具自动生成，请勿手动修改

use css_in_rust::css;

/// button 组件样式
#[derive(Debug, Clone, PartialEq)]
pub struct ButtonStyles {
    pub base: String,
    pub variants: VariantStyles,
    pub states: StateStyles,
}

/// 变体样式
#[derive(Debug, Clone, PartialEq)]
pub struct VariantStyles {
    pub type_text: String,
    pub type_primary: String,
    pub type_link: String,
    pub type_dashed: String,
}

/// 状态样式
#[derive(Debug, Clone, PartialEq)]
pub struct StateStyles {
    pub hover: String,
    pub disabled: String,
    pub focus: String,
    pub active: String,
}

/// button 样式生成器
#[derive(Debug, Clone)]
pub struct MigratedButtonStyleGenerator {
    // 在这里添加样式生成器的字段
}

impl MigratedButtonStyleGenerator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn generate(&self) -> ButtonStyles {
        generate_button_styles()
    }
}

/// 生成 button 组件样式
pub fn generate_button_styles() -> ButtonStyles {
    let base = css!(
        r#"
        .ant-btn {
            line-height: 1.5714285714285714;
            border: 1px solid transparent;
            outline: none;
            background-image: none;
            border-radius: 6px;
            color: rgba(0, 0, 0, 0.88);
            position: relative;
            touch-action: manipulation;
            white-space: nowrap;
            text-decoration: none;
            box-shadow: 0 2px 0 rgba(0, 0, 0, 0.02);
            cursor: pointer;
            user-select: none;
            font-weight: 400;
            transition: all 0.2s cubic-bezier(0.645, 0.045, 0.355, 1);
            text-align: center;
            height: 32px;
            padding: 4px 15px;
            display: inline-block;
            font-size: 14px;
            border-color: #d9d9d9;
            background: #ffffff;
        }
        .ant-btn-lg {
            padding: 6.4px 15px;
            font-size: 16px;
            height: 40px;
            border-radius: 8px;
        }
        .ant-btn-sm {
            height: 24px;
            font-size: 14px;
            padding: 0px 7px;
            border-radius: 4px;
        }
        .ant-btn-circle {
            border-radius: 50%;
            min-width: 32px;
            padding-left: 0;
            padding-right: 0;
        }
        .ant-btn-circle.ant-btn-lg {
            min-width: 40px;
        }
        .ant-btn-circle.ant-btn-sm {
            min-width: 24px;
        }
        .ant-btn-round {
            border-radius: 32px;
        }
        .ant-btn-round.ant-btn-lg {
            border-radius: 40px;
        }
        .ant-btn-round.ant-btn-sm {
            border-radius: 24px;
        }
        .ant-btn-dangerous {
            color: #ff4d4f;
            border-color: #ff4d4f;
        }
        .ant-btn-background-ghost {
            border-color: #ffffff;
            color: #ffffff;
            background: transparent;
        }
        .ant-btn-loading {
            position: relative;
            pointer-events: none;
        }
        .ant-btn-loading-icon {
            animation: loadingCircle 1s infinite linear;
            margin-right: 8px;
            display: inline-block;
        }
        .ant-btn-loading-icon::before {
            display: inline-block;
            content: "⟳";
        }
        0% {
            transform: rotate(0deg);
        }
        100% {
            transform: rotate(360deg);
        }
        .ant-btn-block {
            width: 100%;
        }
        .ant-btn-group {
            position: relative;
            display: inline-flex;
            vertical-align: middle;
        }
        .ant-btn-group>.ant-btn {
            position: relative;
            z-index: 1;
        }
        .ant-btn-group>.ant-btn:not(:first-child):not(:last-child) {
            border-radius: 0;
            border-left-width: 0;
        }
        .ant-btn-group>.ant-btn:first-child:not(:last-child) {
            border-top-right-radius: 0;
            border-bottom-right-radius: 0;
            border-right-width: 0;
        }
        .ant-btn-group>.ant-btn:last-child:not(:first-child) {
            border-top-left-radius: 0;
            border-left-width: 0;
            border-bottom-left-radius: 0;
        }
        .ant-btn-group>.ant-btn:only-child {
            border-radius: 6px;
        }
        .ant-btn-group-lg>.ant-btn {
            height: 40px;
            padding: 6.4px 15px;
            border-radius: 8px;
            font-size: 16px;
        }
        .ant-btn-group-lg>.ant-btn:first-child:not(:last-child) {
            border-bottom-right-radius: 0;
            border-top-right-radius: 0;
        }
        .ant-btn-group-lg>.ant-btn:last-child:not(:first-child) {
            border-bottom-left-radius: 0;
            border-top-left-radius: 0;
        }
        .ant-btn-group-lg>.ant-btn:only-child {
            border-radius: 8px;
        }
        .ant-btn-group-sm>.ant-btn {
            font-size: 14px;
            padding: 0px 7px;
            border-radius: 4px;
            height: 24px;
        }
        .ant-btn-group-sm>.ant-btn:first-child:not(:last-child) {
            border-bottom-right-radius: 0;
            border-top-right-radius: 0;
        }
        .ant-btn-group-sm>.ant-btn:last-child:not(:first-child) {
            border-bottom-left-radius: 0;
            border-top-left-radius: 0;
        }
        .ant-btn-group-sm>.ant-btn:only-child {
            border-radius: 4px;
        }
        .ant-btn-group>.ant-btn-dangerous:not(:first-child) {
            border-left-color: #d9363e;
        }
        .ant-btn-group>.ant-btn-dangerous:not(:last-child) {
            border-right-color: #d9363e;
        }
        .ant-btn-content {
            display: inline-block;
        }
        .ant-btn+.ant-btn {
            margin-left: 8px;
        }
        [dir="rtl"] .ant-btn+.ant-btn {
            margin-left: 0;
            margin-right: 8px;
        }
        [dir="rtl"] .ant-btn-loading-icon {
            margin-left: 8px;
            margin-right: 0;
        }
    "#
    );

    let variants = VariantStyles {
        type_text: css!(
            r#"
        .ant-btn-text {
            box-shadow: none;
            border-color: transparent;
            background: transparent;
            color: rgba(0, 0, 0, 0.88);
        }
        "#
        ),
        type_primary: css!(
            r#"
        .ant-btn-primary {
            color: #ffffff;
            background: #1677ff;
            border-color: #1677ff;
            box-shadow: 0 2px 0 rgba(5, 145, 255, 0.1);
        }
        .ant-btn-primary.ant-btn-dangerous {
            border-color: #ff4d4f;
            box-shadow: 0 2px 0 rgba(255, 77, 79, 0.1);
            color: #ffffff;
            background: #ff4d4f;
        }
        .ant-btn-primary.ant-btn-background-ghost {
            border-color: #1677ff;
            color: #1677ff;
            background: transparent;
        }
        .ant-btn-group>.ant-btn-primary:not(:first-child) {
            border-left-color: #0958d9;
        }
        .ant-btn-group>.ant-btn-primary:not(:last-child) {
            border-right-color: #0958d9;
        }
        "#
        ),
        type_link: css!(
            r#"
        .ant-btn-link {
            background: transparent;
            border-color: transparent;
            box-shadow: none;
            color: #1677ff;
        }
        "#
        ),
        type_dashed: css!(
            r#"
        .ant-btn-dashed {
            border-color: #d9d9d9;
            background: #ffffff;
            color: rgba(0, 0, 0, 0.88);
            border-style: dashed;
        }
        "#
        ),
    };

    let states = StateStyles {
        hover: css!(
            r#"
        .ant-btn:hover {
            border-color: #4096ff;
            color: #4096ff;
            background: #ffffff;
        }
        .ant-btn-primary:hover {
            background: #4096ff;
            color: #ffffff;
            border-color: #4096ff;
        }
        .ant-btn-dashed:hover {
            background: #ffffff;
            border-color: #4096ff;
            color: #4096ff;
        }
        .ant-btn-text:hover {
            border-color: transparent;
            color: rgba(0, 0, 0, 0.88);
            background: rgba(0, 0, 0, 0.06);
        }
        .ant-btn-link:hover {
            background: transparent;
            color: #4096ff;
            border-color: transparent;
        }
        .ant-btn-dangerous:hover {
            border-color: #ff7875;
            color: #ff7875;
        }
        .ant-btn-primary.ant-btn-dangerous:hover {
            background: #ff7875;
            color: #ffffff;
            border-color: #ff7875;
        }
        .ant-btn-background-ghost:hover {
            background: transparent;
            border-color: rgba(255, 255, 255, 0.8);
            color: rgba(255, 255, 255, 0.8);
        }
        .ant-btn-primary.ant-btn-background-ghost:hover {
            border-color: #4096ff;
            color: #4096ff;
            background: transparent;
        }
        .ant-btn:disabled:hover,
.ant-btn-disabled:hover {
            border-color: #d9d9d9;
            color: rgba(0, 0, 0, 0.25);
            background: rgba(0, 0, 0, 0.04);
        }
        .ant-btn-group>.ant-btn:hover,
.ant-btn-group>.ant-btn:focus,
.ant-btn-group>.ant-btn:active {
            z-index: 2;
        }
        "#
        ),
        disabled: css!(
            r#"
        .ant-btn:disabled,
.ant-btn-disabled {
            background: rgba(0, 0, 0, 0.04);
            border-color: #d9d9d9;
            box-shadow: none;
            color: rgba(0, 0, 0, 0.25);
            cursor: not-allowed;
        }
        .ant-btn-primary:disabled,
.ant-btn-primary.ant-btn-disabled {
            color: rgba(0, 0, 0, 0.25);
            background: rgba(0, 0, 0, 0.04);
            border-color: #d9d9d9;
        }
        .ant-btn-text:disabled,
.ant-btn-text.ant-btn-disabled {
            background: transparent;
            color: rgba(0, 0, 0, 0.25);
            border-color: transparent;
        }
        .ant-btn-link:disabled,
.ant-btn-link.ant-btn-disabled {
            background: transparent;
            border-color: transparent;
            color: rgba(0, 0, 0, 0.25);
        }
        .ant-btn-group>.ant-btn:disabled {
            z-index: 0;
        }
        "#
        ),
        focus: css!(
            r#"
        .ant-btn:focus {
            box-shadow: 0 0 0 2px rgba(5, 145, 255, 0.1);
            color: #4096ff;
            background: #ffffff;
            border-color: #4096ff;
            outline: 0;
        }
        .ant-btn-primary:focus {
            outline: 0;
            color: #ffffff;
            background: #4096ff;
            border-color: #4096ff;
            box-shadow: 0 0 0 2px rgba(5, 145, 255, 0.1);
        }
        .ant-btn-dashed:focus {
            color: #4096ff;
            outline: 0;
            box-shadow: 0 0 0 2px rgba(5, 145, 255, 0.1);
            border-color: #4096ff;
            background: #ffffff;
        }
        .ant-btn-text:focus {
            background: rgba(0, 0, 0, 0.06);
            outline: 0;
            color: rgba(0, 0, 0, 0.88);
            box-shadow: 0 0 0 2px rgba(5, 145, 255, 0.1);
            border-color: transparent;
        }
        .ant-btn-link:focus {
            border-color: transparent;
            color: #4096ff;
            background: transparent;
            outline: 0;
            box-shadow: 0 0 0 2px rgba(5, 145, 255, 0.1);
        }
        .ant-btn-dangerous:focus {
            color: #ff7875;
            box-shadow: 0 0 0 2px rgba(255, 77, 79, 0.1);
            border-color: #ff7875;
            outline: 0;
        }
        .ant-btn-primary.ant-btn-dangerous:focus {
            outline: 0;
            border-color: #ff7875;
            color: #ffffff;
            background: #ff7875;
            box-shadow: 0 0 0 2px rgba(255, 77, 79, 0.1);
        }
        .ant-btn-background-ghost:focus {
            outline: 0;
            border-color: rgba(255, 255, 255, 0.8);
            color: rgba(255, 255, 255, 0.8);
            box-shadow: 0 0 0 2px rgba(255, 255, 255, 0.1);
            background: transparent;
        }
        .ant-btn-primary.ant-btn-background-ghost:focus {
            outline: 0;
            background: transparent;
            box-shadow: 0 0 0 2px rgba(5, 145, 255, 0.1);
            color: #4096ff;
            border-color: #4096ff;
        }
        "#
        ),
        active: css!(
            r#"
        .ant-btn:active {
            border-color: #0958d9;
            background: #ffffff;
            color: #0958d9;
        }
        .ant-btn-primary:active {
            border-color: #0958d9;
            color: #ffffff;
            background: #0958d9;
        }
        .ant-btn-dashed:active {
            color: #0958d9;
            background: #ffffff;
            border-color: #0958d9;
        }
        .ant-btn-text:active {
            background: rgba(0, 0, 0, 0.15);
            border-color: transparent;
            color: rgba(0, 0, 0, 0.88);
        }
        .ant-btn-link:active {
            border-color: transparent;
            color: #0958d9;
            background: transparent;
        }
        .ant-btn-dangerous:active {
            color: #d9363e;
            border-color: #d9363e;
        }
        .ant-btn-primary.ant-btn-dangerous:active {
            color: #ffffff;
            background: #d9363e;
            border-color: #d9363e;
        }
        .ant-btn-background-ghost:active {
            border-color: rgba(255, 255, 255, 0.65);
            background: transparent;
            color: rgba(255, 255, 255, 0.65);
        }
        .ant-btn-primary.ant-btn-background-ghost:active {
            color: #0958d9;
            border-color: #0958d9;
            background: transparent;
        }
        "#
        ),
    };

    ButtonStyles {
        base,
        variants,
        states,
    }
}

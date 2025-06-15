mod base;
mod color_variant;

pub use base::*;
pub use color_variant::*;

use css_in_rust::css;

/// 按钮样式
pub fn button_styles() -> String {
    // 基础样式
    let base_styles = button_base_styles();

    // 合并样式
    css!(
        r#"
        {base_styles}

        /* 按钮波纹效果样式 */
        .ant-btn-ripple {
            position: absolute;
            background-color: rgba(0, 0, 0, 0.15);
            border-radius: 50%;
            transform: scale(0);
            opacity: 1;
            pointer-events: none;
            transition: transform 0.6s, opacity 0.6s;
        }

        .ant-btn-ripple.active {
            transform: scale(1);
            opacity: 0;
        }
        "#
    )
    .to_string()
}

/// 组合所有按钮组样式
pub fn button_group_styles() -> String {
    // TODO: 实现按钮组样式
    css!(
        r#"
        .ant-btn-group {
            position: relative;
            display: inline-flex;
        }

        .ant-btn-group > .ant-btn {
            position: relative;
            z-index: 1;
        }

        .ant-btn-group > .ant-btn:hover,
        .ant-btn-group > .ant-btn:focus,
        .ant-btn-group > .ant-btn:active {
            z-index: 2;
        }

        .ant-btn-group > .ant-btn:not(:first-child):not(:last-child) {
            border-radius: 0;
        }

        .ant-btn-group > .ant-btn:first-child:not(:last-child) {
            border-top-right-radius: 0;
            border-bottom-right-radius: 0;
        }

        .ant-btn-group > .ant-btn:last-child:not(:first-child) {
            border-top-left-radius: 0;
            border-bottom-left-radius: 0;
        }

        .ant-btn-group > .ant-btn + .ant-btn {
            margin-left: -1px;
        }
        "#
    )
    .to_string()
}

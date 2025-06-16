//! 按钮组件样式模块
//!
//! 本模块包含Button组件的所有样式定义，从组件逻辑中分离出来，
//! 提高代码的可维护性和复用性。

mod base;
mod color_variant;
mod generator;

pub use base::*;
pub use color_variant::*;
pub use generator::*;

use css_in_rust::css;

/// 按钮波纹效果样式
pub fn button_ripple_styles() -> String {
    css!(
        r#"
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

/// 按钮加载图标样式
pub fn button_loading_icon_style() -> String {
    css!(
        r#"
        .ant-btn-loading-icon {
            display: inline-block;
            margin-right: 8px;
            animation: loadingCircle 1s infinite linear;
        }

        @keyframes loadingCircle {
            100% {
                transform: rotate(360deg);
            }
        }
        "#
    )
    .to_string()
}

/// 按钮内容样式
pub fn button_content_style() -> String {
    css!(
        r#"
        .ant-btn-content {
            display: inline-block;
        }
        "#
    )
    .to_string()
}

/// 按钮组样式
pub fn button_group_style() -> String {
    css!(
        r#"
        .ant-btn-group {
            position: relative;
            display: inline-block;
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

        .ant-btn-group > .ant-btn:not(:first-child) {
            margin-left: -1px;
        }
        "#
    )
    .to_string()
}

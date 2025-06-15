mod base;
mod color_variant;

pub use base::{generate_button_base_styles, generate_custom_color_class, generate_variant_class};

pub use color_variant::{generate_button_color_styles, generate_button_variant_styles};

use css_in_rust::css;

/// 按钮样式
pub fn button_styles() -> String {
    // 基础样式
    let base_styles = generate_button_base_styles();

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
}

/// 组合所有按钮组样式
pub fn button_group_styles() -> String {
    base::button_group_styles()
}

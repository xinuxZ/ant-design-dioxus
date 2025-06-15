mod base;

pub use base::*;

use css_in_rust::css;

/// 组合所有按钮样式
pub fn button_styles() -> String {
    let styles = vec![
        base_style(),
        type_style(),
        size_style(),
        shape_style(),
        icon_style(),
        loading_style(),
        group_style(),
    ];

    styles.join("\n")
}

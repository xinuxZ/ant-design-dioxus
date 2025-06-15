mod base;
mod color_variant;

pub use base::*;
pub use color_variant::*;

/// 组合所有按钮样式
pub fn button_styles() -> String {
    base::button_styles()
}

/// 组合所有按钮组样式
pub fn button_group_styles() -> String {
    base::button_group_styles()
}

//! Button 组件样式模块
//!
//! 包含 Button 组件的所有样式定义和生成逻辑

mod base;

// 明确导出 base 模块的内容
pub use base::{
    ButtonStyleGenerator as BaseButtonStyleGenerator,
    generate_button_styles as base_generate_button_styles,
    generate_button_group_styles,
    ButtonStyles as BaseButtonStyles,
    VariantStyles as BaseVariantStyles,
    StateStyles as BaseStateStyles,
};

// 重新导出迁移的样式，使用别名避免冲突
pub use super::migrated_styles::{
    MigratedButtonStyleGenerator,
    generate_button_styles,
    ButtonStyles,
    VariantStyles,
    StateStyles,
};

// 为了向后兼容，默认使用迁移版本的 ButtonStyleGenerator
pub use super::migrated_styles::MigratedButtonStyleGenerator as ButtonStyleGenerator;

//! 主题钩子模块
//!
//! 提供主题相关的钩子函数，方便在组件中使用主题功能

use dioxus::prelude::*;
use std::collections::HashMap;

use super::core::types::Size;
use super::provider::use_theme;
use super::AntThemeType;

/// 使用暗色模式的 Hook
///
/// 检查当前主题是否为暗色模式
///
/// # 返回值
///
/// 如果是暗色模式则返回 true，否则返回 false
pub fn use_dark_mode() -> bool {
    let theme_context = use_theme();
    matches!(theme_context.config.theme_type, AntThemeType::Dark)
}

/// 使用紧凑模式的 Hook
///
/// 检查当前主题是否为紧凑模式
///
/// # 返回值
///
/// 如果是紧凑模式则返回 true，否则返回 false
pub fn use_compact_mode() -> bool {
    let theme_context = use_theme();
    theme_context.config.compact
}

/// 使用组件样式的 Hook
///
/// 获取组件的样式类名
///
/// # 参数
///
/// * `component` - 组件名称
/// * `variants` - 组件变体
///
/// # 返回值
///
/// 组件的样式类名
pub fn use_component_style(component: &str, variants: Option<HashMap<String, String>>) -> String {
    let theme_context = use_theme();
    let dark_mode = use_dark_mode();
    let compact_mode = use_compact_mode();

    let mut class_names = vec![format!("ant-{}", component)];

    if dark_mode {
        class_names.push(format!("ant-{}-dark", component));
    }

    if compact_mode {
        class_names.push(format!("ant-{}-compact", component));
    }

    if let Some(vars) = variants {
        for (key, value) in vars {
            class_names.push(format!("ant-{}-{}-{}", component, key, value));
        }
    }

    class_names.join(" ")
}

/// 使用组件尺寸的 Hook
///
/// 获取当前主题的组件尺寸
///
/// # 返回值
///
/// 组件尺寸枚举
pub fn use_component_size() -> Size {
    // 这里应该从配置提供者中获取，简化实现
    Size::Middle
}

/// 使用响应式的 Hook
///
/// 根据屏幕尺寸返回不同的值
///
/// # 参数
///
/// * `xs` - 超小屏幕的值
/// * `sm` - 小屏幕的值
/// * `md` - 中等屏幕的值
/// * `lg` - 大屏幕的值
/// * `xl` - 超大屏幕的值
/// * `xxl` - 超超大屏幕的值
///
/// # 返回值
///
/// 根据当前屏幕尺寸选择的值
pub fn use_responsive<T: Clone>(
    xs: T,
    sm: Option<T>,
    md: Option<T>,
    lg: Option<T>,
    xl: Option<T>,
    xxl: Option<T>,
) -> T {
    // 这里应该根据实际屏幕尺寸返回不同的值，简化实现
    xs
}

/// 使用主题CSS变量的 Hook
///
/// 获取CSS变量名
///
/// # 参数
///
/// * `token_name` - 令牌名称
///
/// # 返回值
///
/// CSS变量名（包含 -- 前缀）
pub fn use_css_var_name(token_name: &str) -> String {
    format!("--{}", token_name)
}

/// 使用主题令牌样式的 Hook
///
/// 生成包含主题令牌的样式字符串
///
/// # 参数
///
/// * `styles` - 样式映射
///
/// # 返回值
///
/// 样式字符串
pub fn use_token_style(styles: HashMap<&str, &str>) -> String {
    let theme_context = use_theme();

    let mut style_str = String::new();

    for (prop, token_name) in styles {
        if let Some(value) = theme_context.config.token.get(token_name) {
            style_str.push_str(&format!("{}: {}; ", prop, value));
        }
    }

    style_str
}

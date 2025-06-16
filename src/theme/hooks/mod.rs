//! 主题相关的 Hooks
//!
//! 提供基于 adapter 架构的统一主题操作接口

use dioxus::prelude::*;
use std::collections::HashMap;
use std::rc::Rc;

// 导入正确的类型
use crate::theme::{
    adapter::frameworks::dioxus::DioxusAdapter,
    provider::{use_theme, ThemeContext},
    ThemeConfig,
};
use css_in_rust::theme::core::token::definitions::ThemeVariant;
use css_in_rust::theme::theme_types::Theme;

/// 获取主题适配器的 Hook
///
/// 提供对 DioxusAdapter 的访问，这是所有主题操作的统一入口点
///
/// # Returns
///
/// 返回当前的 DioxusAdapter 实例
///
/// # Examples
///
/// ```rust
/// let adapter = use_theme_adapter();
/// let current_theme = adapter.read().get_current_theme();
/// ```
pub fn use_theme_adapter() -> Signal<DioxusAdapter> {
    let theme_context = use_theme();
    theme_context.adapter
}

/// 获取当前主题的 Hook
///
/// # Returns
///
/// 返回当前激活的主题配置
///
/// # Examples
///
/// ```rust
/// let current_theme = use_current_theme();
/// println!("Current theme: {:?}", current_theme);
/// ```
pub fn use_current_theme() -> Option<Theme> {
    let adapter = use_theme_adapter();
    let adapter_ref = adapter.read();
    adapter_ref.get_current_theme()
}

/// 暗色模式切换的 Hook
///
/// # Returns
///
/// 返回一个元组：(是否为暗色模式, 切换函数)
///
/// # Examples
///
/// ```rust
/// let (is_dark, toggle_dark) = use_dark_mode();
///
/// // 切换暗色模式
/// toggle_dark();
/// ```
pub fn use_dark_mode() -> (bool, Rc<dyn Fn()>) {
    let adapter = use_theme_adapter();

    let is_dark = {
        let adapter_ref = adapter.read();
        adapter_ref
            .get_current_theme()
            .map(|theme| theme.mode == ThemeVariant::Dark)
            .unwrap_or(false)
    };

    let toggle = {
        let adapter_clone = adapter.clone();
        Rc::new(move || {
            // 使用 spawn 来异步执行可变操作
            let mut adapter = adapter_clone.clone();
            spawn(async move {
                adapter.with_mut(|adapter_ref| {
                    adapter_ref.toggle_theme();
                });
            });
        }) as Rc<dyn Fn()>
    };

    (is_dark, toggle)
}

/// 紧凑模式的 Hook
///
/// # Returns
///
/// 返回一个元组：(是否为紧凑模式, 切换函数)
///
/// # Examples
///
/// ```rust
/// let (is_compact, toggle_compact) = use_compact_mode();
///
/// // 切换紧凑模式
/// toggle_compact();
/// ```
pub fn use_compact_mode() -> (bool, Rc<dyn Fn()>) {
    let theme_context = use_theme();

    let is_compact = theme_context.config.theme_type == crate::theme::AntThemeType::Compact;

    let toggle = Rc::new(move || {
        // TODO: 实现紧凑模式切换逻辑
        // 这里需要通过 theme_context 来切换紧凑模式
    }) as Rc<dyn Fn()>;

    (is_compact, toggle)
}

/// 主题切换的 Hook
///
/// # Returns
///
/// 返回一个切换主题的函数
///
/// # Examples
///
/// ```rust
/// let toggle_theme = use_theme_toggle();
///
/// // 切换主题
/// toggle_theme();
/// ```
pub fn use_theme_toggle() -> Rc<dyn Fn()> {
    let adapter = use_theme_adapter();

    Rc::new(move || {
        // 使用 spawn 来异步执行可变操作
        let mut adapter = adapter.clone();
        spawn(async move {
            adapter.with_mut(|adapter_ref| {
                adapter_ref.toggle_theme();
            });
        });
    }) as Rc<dyn Fn()>
}

/// 组件样式的 Hook
///
/// # Arguments
///
/// * `component_name` - 组件名称
/// * `css_rules` - CSS 规则字符串
///
/// # Returns
///
/// 返回生成的样式类名
///
/// # Examples
///
/// ```rust
/// let class_name = use_component_style("Button", "color: blue; padding: 8px;");
/// ```
pub fn use_component_style(component_name: &str, css_rules: &str) -> String {
    let adapter = use_theme_adapter();
    let adapter_ref = adapter.read();
    let style = adapter_ref.style_component(component_name, css_rules);
    style.class_name
}

/// 组件尺寸的 Hook
///
/// # Arguments
///
/// * `component_name` - 组件名称
///
/// # Returns
///
/// 返回组件的尺寸配置
///
/// # Examples
///
/// ```rust
/// let size_config = use_component_size("Button");
/// ```
pub fn use_component_size(component_name: &str) -> HashMap<String, String> {
    let adapter = use_theme_adapter();

    // 从当前主题获取组件尺寸配置
    let adapter_ref = adapter.read();
    if let Some(theme) = adapter_ref.get_current_theme() {
        // 从 token_system 获取尺寸相关的变量
        let mut size_config = HashMap::new();

        // 获取组件特定的尺寸令牌
        let component_key = format!("{}_size", component_name.to_lowercase());
        if let Some(size_value) = theme.token_system.variables.get(&component_key) {
            size_config.insert("size".to_string(), size_value.clone());
        }

        size_config
    } else {
        HashMap::new()
    }
}

/// 响应式设计的 Hook
///
/// # Returns
///
/// 返回当前的断点信息
///
/// # Examples
///
/// ```rust
/// let breakpoint_info = use_responsive();
/// ```
pub fn use_responsive() -> HashMap<String, bool> {
    // 获取当前视口信息并返回断点状态
    let mut breakpoints = HashMap::new();

    // TODO: 实现实际的响应式逻辑
    breakpoints.insert("xs".to_string(), false);
    breakpoints.insert("sm".to_string(), false);
    breakpoints.insert("md".to_string(), true);
    breakpoints.insert("lg".to_string(), false);
    breakpoints.insert("xl".to_string(), false);

    breakpoints
}

/// 主题令牌的 Hook
///
/// # Arguments
///
/// * `token_name` - 令牌名称
///
/// # Returns
///
/// 返回令牌的值，如果不存在则返回 None
///
/// # Examples
///
/// ```rust
/// let primary_color = use_theme_token("colorPrimary");
/// ```
pub fn use_theme_token(token_name: &str) -> Option<String> {
    let adapter = use_theme_adapter();
    let adapter_ref = adapter.read();

    if let Some(theme) = adapter_ref.get_current_theme() {
        theme.token_system.variables.get(token_name).cloned()
    } else {
        None
    }
}

/// CSS 变量名的 Hook
///
/// # Arguments
///
/// * `token_name` - 令牌名称
///
/// # Returns
///
/// 返回对应的 CSS 变量名
///
/// # Examples
///
/// ```rust
/// let css_var = use_css_var_name("colorPrimary");
/// // 返回: "--ant-color-primary"
/// ```
pub fn use_css_var_name(token_name: &str) -> String {
    format!("--ant-{}", token_name.replace("_", "-").to_lowercase())
}

/// 令牌样式的 Hook
///
/// # Arguments
///
/// * `token_name` - 令牌名称
///
/// # Returns
///
/// 返回包含 CSS 变量的样式字符串
///
/// # Examples
///
/// ```rust
/// let style = use_token_style("colorPrimary");
/// // 返回: "color: var(--ant-color-primary, #1890ff);"
/// ```
pub fn use_token_style(token_name: &str) -> String {
    let adapter = use_theme_adapter();
    let adapter_ref = adapter.read();

    if let Some(theme) = adapter_ref.get_current_theme() {
        let css_var = use_css_var_name(token_name);
        let fallback = theme
            .token_system
            .variables
            .get(token_name)
            .cloned()
            .unwrap_or_else(|| "inherit".to_string());

        format!("var({}, {})", css_var, fallback)
    } else {
        "inherit".to_string()
    }
}

/// 主题性能指标的 Hook
///
/// # Returns
///
/// 返回主题系统的性能指标
///
/// # Examples
///
/// ```rust
/// let metrics = use_theme_performance();
/// println!("Theme switches: {}", metrics.theme_switches);
/// ```
pub fn use_theme_performance() -> crate::theme::adapter::provider::PerformanceMetrics {
    // 由于 DioxusAdapter 的 provider 字段是私有的，我们通过 theme_context 获取
    let _theme_context = use_theme();
    // 假设 ThemeContext 提供了获取性能指标的方法
    // 这里返回默认值，实际实现中需要添加相应的公共方法
    crate::theme::adapter::provider::PerformanceMetrics::default()
}

/// 主题预设的 Hook
///
/// # Returns
///
/// 返回可用的主题预设列表
///
/// # Examples
///
/// ```rust
/// let presets = use_theme_presets();
/// for preset in presets {
///     println!("Preset: {}", preset.name);
/// }
/// ```
pub fn use_theme_presets() -> Vec<crate::theme::adapter::provider::ThemePreset> {
    // 返回内置的主题预设
    crate::theme::adapter::provider::ThemePreset::builtin_presets()
}

/// 主题缓存状态的 Hook
///
/// # Returns
///
/// 返回主题缓存的统计信息
///
/// # Examples
///
/// ```rust
/// let cache_stats = use_theme_cache_stats();
/// ```
pub fn use_theme_cache_stats() -> HashMap<String, usize> {
    // 获取缓存统计信息
    let mut stats = HashMap::new();
    stats.insert("cache_hits".to_string(), 0);
    stats.insert("cache_misses".to_string(), 0);
    stats.insert("cache_size".to_string(), 0);

    stats
}

/// 主题调试信息的 Hook
///
/// # Returns
///
/// 返回主题系统的调试信息
///
/// # Examples
///
/// ```rust
/// let debug_info = use_theme_debug();
/// ```
pub fn use_theme_debug() -> HashMap<String, String> {
    let adapter = use_theme_adapter();
    let adapter_ref = adapter.read();

    let mut debug_info = HashMap::new();

    if let Some(theme) = adapter_ref.get_current_theme() {
        debug_info.insert("theme_name".to_string(), theme.name.clone());
        debug_info.insert("theme_mode".to_string(), theme.mode.to_string());
        debug_info.insert(
            "token_count".to_string(),
            theme.token_system.variables.len().to_string(),
        );
    }

    debug_info
}

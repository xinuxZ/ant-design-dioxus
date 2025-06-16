//! 主题提供者模块
//!
//! 提供主题上下文和组件，用于在应用中注入和管理主题

use crate::{
    theme::{
        adapter::{frameworks::dioxus::DioxusAdapter, provider::ThemeProviderAdapter},
        ThemeVariant,
    },
    ThemeConfig,
};
use dioxus::prelude::*;
use std::sync::Arc;

/// 主题上下文
#[derive(Clone)]
pub struct ThemeContext {
    /// 当前主题配置
    pub config: ThemeConfig,
    /// 主题适配器
    pub adapter: Signal<DioxusAdapter>,
}

impl PartialEq for ThemeContext {
    fn eq(&self, other: &Self) -> bool {
        self.config == other.config
        // 忽略 adapter 字段比较
    }
}

impl ThemeContext {
    /// 切换主题
    pub fn switch_theme(&self, new_config: ThemeConfig) {
        // 更新配置
        // TODO: 实现主题切换逻辑
        // 这里可以通过适配器应用新的主题配置
    }

    /// 获取当前主题配置
    pub fn get_config(&self) -> &ThemeConfig {
        &self.config
    }
}

/// 主题提供者组件的属性
#[derive(Props, Clone, PartialEq)]
pub struct ThemeProviderProps {
    /// 主题配置
    pub theme: Option<ThemeConfig>,
    /// 主题配置（别名）
    pub config: Option<ThemeConfig>,
    /// 子组件
    pub children: Element,
}

/// 主题提供者组件
///
/// 为应用提供主题上下文，支持主题切换和CSS变量注入
#[component]
pub fn ThemeProvider(props: ThemeProviderProps) -> Element {
    // 创建适配器实例
    let mut adapter = use_signal(|| DioxusAdapter::new(ThemeProviderAdapter::default()));

    // 获取或使用默认主题配置
    let theme_config = props.config.clone()
        .or(props.theme.clone())
        .unwrap_or_default();
    
    // 克隆一份用于 use_effect
    let theme_config_for_effect = theme_config.clone();
    
    // 使用适配器处理主题
    use_effect(move || {
        adapter.write().apply_theme(&theme_config_for_effect);
    });
    
    // 提供主题上下文
    use_context_provider(|| ThemeContext {
        config: theme_config.clone(),
        adapter: adapter.clone(),
    });

    rsx! {
        {props.children}
    }
}

/// 使用主题的 Hook
///
/// 获取当前的主题上下文
pub fn use_theme() -> ThemeContext {
    use_context::<ThemeContext>()
}

/// 使用主题切换的 Hook
///
/// 获取主题切换函数
pub fn use_theme_switch() -> Arc<dyn Fn(ThemeConfig) + 'static> {
    let theme_context = use_theme();
    Arc::new(move |config| {
        theme_context.switch_theme(config);
    })
}

/// 使用主题令牌的 Hook
///
/// 获取特定主题令牌的值
///
/// # 参数
///
/// * `token_name` - 令牌名称
///
/// # 返回值
///
/// 令牌值，如果不存在则返回空字符串
pub fn use_theme_token(token_name: &str) -> String {
    let theme_context = use_theme();
    theme_context
        .config
        .token
        .get(token_name)
        .cloned()
        .unwrap_or_default()
}

/// 使用组件令牌的 Hook
///
/// 获取特定组件的主题令牌
///
/// # 参数
///
/// * `component` - 组件名称
/// * `token_name` - 令牌名称
///
/// # 返回值
///
/// 令牌值，如果不存在则返回空字符串
pub fn use_component_token(component: &str, token_name: &str) -> String {
    let theme_context = use_theme();
    theme_context
        .config
        .components
        .get(component)
        .and_then(|tokens| tokens.get(token_name))
        .cloned()
        .unwrap_or_default()
}

/// 使用CSS变量的 Hook
///
/// 获取CSS变量名称
///
/// # 参数
///
/// * `token_name` - 令牌名称
///
/// # 返回值
///
/// CSS变量名称
pub fn use_css_var_name(token_name: &str) -> String {
    let theme_context = use_theme();
    let prefix = &theme_context.config.css_vars.prefix;
    format!("--{}-{}", prefix, token_name)
}

/// 使用CSS变量值的 Hook
///
/// 获取CSS变量值引用
///
/// # 参数
///
/// * `token_name` - 令牌名称
///
/// # 返回值
///
/// CSS变量值引用
pub fn use_css_var_value(token_name: &str) -> String {
    let var_name = use_css_var_name(token_name);
    format!("var({})", var_name)
}

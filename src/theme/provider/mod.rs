//! 主题提供者模块
//!
//! 提供主题上下文和组件，用于在应用中注入和管理主题

use css_in_rust::theme_bridge::ThemeBridge;
use dioxus::prelude::*;
use std::sync::Arc;

use super::ThemeConfig;

/// 主题上下文
#[derive(Clone)]
pub struct ThemeContext {
    /// 主题配置
    pub config: ThemeConfig,
    /// 主题切换函数
    pub switch_theme: Arc<dyn FnMut(ThemeConfig) + 'static>,
}

impl PartialEq for ThemeContext {
    fn eq(&self, other: &Self) -> bool {
        self.config == other.config
        // 忽略 switch_theme 字段比较
    }
}

impl ThemeContext {
    /// 创建新的主题上下文
    pub fn new(config: ThemeConfig, switch_theme: impl FnMut(ThemeConfig) + 'static) -> Self {
        Self {
            config,
            switch_theme: Arc::new(switch_theme),
        }
    }
}

/// 主题提供者属性
#[derive(Props, Clone, PartialEq)]
pub struct ThemeProviderProps {
    /// 子组件
    pub children: Element,
    /// 主题配置
    #[props(into)]
    pub config: Signal<ThemeConfig>,
}

/// 主题提供者组件
///
/// 为子组件提供主题上下文
#[component]
pub fn ThemeProvider(props: ThemeProviderProps) -> Element {
    let theme_config = props.config;

    // 初始化主题桥接器
    let mut bridge = use_signal(|| {
        ThemeBridge::new(
            theme_config.read().theme.clone(),
            css_in_rust::theme::core::css::variables::InjectionStrategy::Replace,
            true,
        )
    });

    // 创建主题切换函数
    let switch_theme = {
        let mut theme_config = theme_config.clone();
        let mut bridge = bridge.clone();

        move |new_config: ThemeConfig| {
            // 更新主题配置
            theme_config.set(new_config.clone());

            // 更新主题桥接器
            bridge.write().set_theme(new_config.theme.clone());
        }
    };

    // 创建主题上下文
    let theme_context = ThemeContext::new(theme_config.read().clone(), switch_theme);

    // 注入主题变量
    use_effect(move || {
        bridge.write().sync_theme_variables();
    });

    // 提供主题上下文
    use_context_provider(|| theme_context);

    rsx! {
        div {
            class: "ant-theme-provider",
            "data-theme": format!("{:?}", theme_config.read().theme.mode),
            "data-compact": theme_config.read().compact.to_string(),
            {props.children}
        }
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
pub fn use_theme_switch() -> Arc<dyn FnMut(ThemeConfig) + 'static> {
    let theme_context = use_theme();
    theme_context.switch_theme
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

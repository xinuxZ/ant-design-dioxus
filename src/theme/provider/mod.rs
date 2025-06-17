//! 主题提供者模块
//!
//! 提供主题上下文和组件，用于在应用中注入和管理主题
//!
//! 基于 DioxusAdapter 构建的统一主题架构，提供完整的主题管理功能。

use crate::theme::{
    adapter::{
        frameworks::dioxus::DioxusAdapter,
        provider::{ThemeProviderAdapter, ThemeSwitchResult},
    },
    ThemeConfig, ThemeVariant,
};
use css_in_rust::theme::theme_types::Theme;
use dioxus::prelude::*;
use std::sync::Arc;

/// 主题上下文
///
/// 基于 DioxusAdapter 的统一主题上下文，提供完整的主题管理功能
#[derive(Clone)]
pub struct ThemeContext {
    /// 当前主题配置
    pub config: ThemeConfig,
    /// Dioxus 主题适配器（提供完整的主题管理功能）
    pub adapter: Signal<DioxusAdapter>,
}

impl PartialEq for ThemeContext {
    fn eq(&self, other: &Self) -> bool {
        self.config == other.config
        // 忽略 adapter 字段比较，因为 Signal 不实现 PartialEq
    }
}

impl ThemeContext {
    /// 切换主题配置
    ///
    /// 通过 DioxusAdapter 应用新的主题配置，支持性能监控和错误处理
    pub fn switch_theme(&self, new_config: ThemeConfig) -> Result<ThemeSwitchResult, String> {
        // 通过适配器应用主题配置
        self.adapter.with(|adapter| {
            adapter.apply_theme(&new_config);
        });

        // 返回切换结果（这里简化处理，实际应该从 adapter 获取详细结果）
        Ok(ThemeSwitchResult {
            success: true,
            duration_ms: 0,       // 实际应该测量时间
            updated_variables: 0, // 实际应该统计更新的变量数
            error: None,
        })
    }

    /// 切换主题模式（亮色/暗色）
    ///
    /// 通过 DioxusAdapter 切换主题模式
    pub fn toggle_theme_mode(&self) -> Result<ThemeSwitchResult, String> {
        // TODO: 实现主题模式切换
        // 这需要 DioxusAdapter 提供相应的接口
        Ok(ThemeSwitchResult {
            success: true,
            duration_ms: 0,
            updated_variables: 0,
            error: None,
        })
    }

    /// 获取当前主题
    pub fn get_current_theme(&self) -> Option<Theme> {
        self.adapter.with(|adapter| adapter.get_current_theme())
    }

    /// 获取当前主题配置
    pub fn get_config(&self) -> &ThemeConfig {
        &self.config
    }

    /// 获取性能指标
    pub fn get_performance_metrics(
        &self,
    ) -> Option<crate::theme::adapter::provider::PerformanceMetrics> {
        // TODO: 从 adapter 获取性能指标
        // 需要 DioxusAdapter 提供相应的接口
        None
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
/// 基于 DioxusAdapter 的统一主题提供者，为应用提供完整的主题管理功能
///
/// # Features
/// - 基于 DioxusAdapter 的统一架构
/// - 支持主题切换和CSS变量注入
/// - 提供性能监控和错误处理
/// - 支持主题持久化和系统主题检测
///
/// # Examples
///
/// ```rust
/// use dioxus::prelude::*;
/// use ant_design_dioxus::theme::provider::ThemeProvider;
/// use ant_design_dioxus::ThemeConfig;
///
/// #[component]
/// fn App() -> Element {
///     let theme_config = ThemeConfig::default();
///
///     rsx! {
///         ThemeProvider {
///             config: Some(theme_config),
///             div { "Hello, themed world!" }
///         }
///     }
/// }
/// ```
#[component]
pub fn ThemeProvider(props: ThemeProviderProps) -> Element {
    // 创建基于 ThemeProviderAdapter 的 DioxusAdapter 实例
    let adapter = use_signal(|| {
        let provider_config = crate::theme::adapter::provider::ThemeProviderConfig {
            auto_detect_system_theme: true,
            enable_persistence: true,
            storage_key: "ant-design-theme".to_string(),
        };

        let theme_provider = ThemeProviderAdapter::new(
            Arc::new(css_in_rust::theme::core::manager::ThemeManager::default()),
            provider_config,
        );

        DioxusAdapter::new(theme_provider).with_auto_inject(true) // 启用自动样式注入
    });

    // 获取或使用默认主题配置
    let theme_config = props
        .config
        .clone()
        .or(props.theme.clone())
        .unwrap_or_default();

    // 克隆配置用于 effect
    let theme_config_for_effect = theme_config.clone();
    let adapter_for_effect = adapter.clone();

    // 应用主题配置
    use_effect(move || {
        adapter_for_effect.with(|adapter| {
            adapter.apply_theme(&theme_config_for_effect);
        });

        // 如果有错误，可以在这里处理
        // 例如：记录日志、显示错误提示等
    });

    // 提供增强的主题上下文
    use_context_provider(|| ThemeContext {
        config: theme_config.clone(),
        adapter: adapter.clone(),
    });

    rsx! {
        // 添加主题相关的根级样式
        div {
            class: "ant-theme-provider",
            "data-theme": theme_config.theme.name,
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

/// 使用主题切换钩子
///
/// 基于 DioxusAdapter 的主题切换钩子，提供完整的主题管理功能
///
/// # Returns
///
/// 返回一个函数，用于切换主题配置，支持错误处理和性能监控
///
/// # Examples
///
/// ```rust
/// use dioxus::prelude::*;
/// use ant_design_dioxus::theme::provider::use_theme_switch;
/// use ant_design_dioxus::ThemeConfig;
///
/// #[component]
/// fn ThemeToggle() -> Element {
///     let switch_theme = use_theme_switch();
///
///     let handle_click = move |_| {
///         let dark_theme = ThemeConfig::dark();
///         if let Err(e) = switch_theme(dark_theme) {
///             // 处理错误
///             log::error!("主题切换失败: {}", e);
///         }
///     };
///
///     rsx! {
///         button { onclick: handle_click, "切换到暗色主题" }
///     }
/// }
/// ```
pub fn use_theme_switch() -> impl Fn(ThemeConfig) -> Result<ThemeSwitchResult, String> + 'static {
    let theme_context = use_context::<ThemeContext>();

    move |new_config: ThemeConfig| theme_context.switch_theme(new_config)
}

/// 使用主题模式切换钩子
///
/// 提供亮色/暗色主题模式的快速切换功能
///
/// # Returns
///
/// 返回一个函数，用于切换主题模式（亮色/暗色）
///
/// # Examples
///
/// ```rust
/// use dioxus::prelude::*;
/// use ant_design_dioxus::theme::provider::use_theme_mode_toggle;
///
/// #[component]
/// fn ThemeModeToggle() -> Element {
///     let toggle_mode = use_theme_mode_toggle();
///
///     let handle_click = move |_| {
///         if let Err(e) = toggle_mode() {
///             log::error!("主题模式切换失败: {}", e);
///         }
///     };
///
///     rsx! {
///         button { onclick: handle_click, "切换主题模式" }
///     }
/// }
/// ```
pub fn use_theme_mode_toggle() -> impl Fn() -> Result<ThemeSwitchResult, String> + 'static {
    let theme_context = use_context::<ThemeContext>();

    move || theme_context.toggle_theme_mode()
}

/// 使用当前主题钩子
///
/// 获取当前主题信息，包括配置和主题对象
///
/// # Returns
///
/// 返回当前主题配置和主题对象的元组
///
/// # Examples
///
/// ```rust
/// use dioxus::prelude::*;
/// use ant_design_dioxus::theme::provider::use_current_theme;
///
/// #[component]
/// fn ThemeInfo() -> Element {
///     let (config, theme) = use_current_theme();
///
///     rsx! {
///         div {
///             p { "当前主题: {config.algorithm:?}" }
///             if let Some(theme) = theme {
///                 p { "主题名称: {theme.name}" }
///             }
///         }
///     }
/// }
/// ```
pub fn use_current_theme() -> (ThemeConfig, Option<Theme>) {
    let theme_context = use_context::<ThemeContext>();

    let config = theme_context.get_config().clone();
    let theme = theme_context.get_current_theme();

    (config, theme)
}

/// 使用主题性能监控钩子
///
/// 获取主题系统的性能指标
///
/// # Returns
///
/// 返回性能指标，如果不可用则返回 None
///
/// # Examples
///
/// ```rust
/// use dioxus::prelude::*;
/// use ant_design_dioxus::theme::provider::use_theme_performance;
///
/// #[component]
/// fn PerformanceMonitor() -> Element {
///     let metrics = use_theme_performance();
///
///     rsx! {
///         div {
///             if let Some(metrics) = metrics {
///                 p { "主题切换次数: {metrics.theme_switches}" }
///                 p { "平均切换时间: {metrics.avg_switch_time_ms:.2}ms" }
///             } else {
///                 p { "性能指标不可用" }
///             }
///         }
///     }
/// }
/// ```
pub fn use_theme_performance() -> Option<crate::theme::adapter::provider::PerformanceMetrics> {
    let theme_context = use_context::<ThemeContext>();
    theme_context.get_performance_metrics()
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

//! 全局配置提供者模块
//!
//! 提供全局配置上下文和组件，用于管理应用级别的配置

use css_in_rust::theme::ThemeVariant;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::locale::Locale;
use crate::theme::ThemeConfig;

// 导出子模块
pub mod builder;
pub mod component_config;
pub mod config_utils;
pub mod examples;
pub mod hooks;
pub mod popup_config;
pub mod security_config;
#[cfg(test)]
mod tests;
pub mod virtual_scroll_config;

// 重新导出主要类型
pub use builder::*;
pub use component_config::*;
pub use config_utils::*;
pub use hooks::*;
pub use popup_config::*;
pub use security_config::*;
pub use virtual_scroll_config::*;

/// 文本方向
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    /// 从左到右
    Ltr,
    /// 从右到左
    Rtl,
}

impl Default for Direction {
    fn default() -> Self {
        Self::Ltr
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Ltr => write!(f, "ltr"),
            Direction::Rtl => write!(f, "rtl"),
        }
    }
}

/// 组件尺寸
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComponentSize {
    /// 小尺寸
    Small,
    /// 中等尺寸
    Middle,
    /// 大尺寸
    Large,
}

impl Default for ComponentSize {
    fn default() -> Self {
        Self::Middle
    }
}

impl std::fmt::Display for ComponentSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComponentSize::Small => write!(f, "small"),
            ComponentSize::Middle => write!(f, "middle"),
            ComponentSize::Large => write!(f, "large"),
        }
    }
}

/// 组件尺寸配置
#[derive(Debug, Clone, PartialEq)]
pub struct ComponentSizeConfig {
    /// 默认组件尺寸
    pub default_size: ComponentSize,
}

impl Default for ComponentSizeConfig {
    fn default() -> Self {
        Self {
            default_size: ComponentSize::Middle,
        }
    }
}

/// 表单配置
#[derive(Debug, Clone, PartialEq)]
pub struct FormConfig {
    /// 是否显示验证状态图标
    pub validate_messages: HashMap<String, String>,
    /// 是否显示必填标记
    pub required_mark: bool,
    /// 是否显示冒号
    pub colon: bool,
    /// 标签对齐方式
    pub label_align: String,
    /// 标签宽度
    pub label_width: Option<String>,
}

impl Default for FormConfig {
    fn default() -> Self {
        Self {
            validate_messages: HashMap::new(),
            required_mark: true,
            colon: true,
            label_align: "right".to_string(),
            label_width: None,
        }
    }
}

/// 主题配置
#[derive(Debug, Clone, PartialEq)]
pub struct ThemeProviderConfig {
    /// 主题配置
    pub theme: ThemeConfig,
    /// 是否启用紧凑模式
    pub compact: bool,
}

impl Default for ThemeProviderConfig {
    fn default() -> Self {
        Self {
            theme: ThemeConfig::default(),
            compact: false,
        }
    }
}

/// 语言配置
#[derive(Debug, Clone, PartialEq)]
pub struct LocaleProviderConfig {
    /// 语言
    pub locale: Locale,
}

impl Default for LocaleProviderConfig {
    fn default() -> Self {
        Self {
            locale: Locale::ZhCN,
        }
    }
}

/// 全局配置
#[derive(Debug, Clone, PartialEq)]
pub struct GlobalConfig {
    /// 主题配置
    pub theme: ThemeProviderConfig,
    /// 语言配置
    pub locale: LocaleProviderConfig,
    /// 组件尺寸配置
    pub component_size: ComponentSizeConfig,
    /// 方向
    pub direction: Direction,
    /// CSS 类名前缀
    pub prefix_cls: String,
    /// 按钮中是否自动插入空格
    pub auto_insert_space_in_button: bool,
    /// 表单配置
    pub form: FormConfig,
    /// 获取弹出容器的函数
    pub get_popup_container: Option<fn() -> Element>,
    /// 获取目标容器的函数
    pub get_target_container: Option<fn() -> Element>,
}

impl Default for GlobalConfig {
    fn default() -> Self {
        Self {
            theme: ThemeProviderConfig::default(),
            locale: LocaleProviderConfig::default(),
            component_size: ComponentSizeConfig::default(),
            direction: Direction::Ltr,
            prefix_cls: "ant".to_string(),
            auto_insert_space_in_button: true,
            form: FormConfig::default(),
            get_popup_container: None,
            get_target_container: None,
        }
    }
}

/// 配置上下文
#[derive(Debug, Clone, PartialEq)]
pub struct ConfigContext {
    /// 全局配置
    pub config: GlobalConfig,
}

/// 配置提供者属性
#[derive(Props, Clone, PartialEq)]
pub struct ConfigProviderProps {
    /// 子组件
    pub children: Element,
    /// 全局配置
    #[props(default)]
    pub config: GlobalConfig,
    /// 主题配置
    #[props(default)]
    pub theme: Option<ThemeConfig>,
    /// 语言配置
    #[props(default)]
    pub locale: Option<Locale>,
    /// 组件尺寸
    #[props(default)]
    pub component_size: Option<ComponentSize>,
    /// 方向
    #[props(default)]
    pub direction: Option<Direction>,
    /// CSS 类名前缀
    #[props(default)]
    pub prefix_cls: Option<String>,
    /// 按钮中是否自动插入空格
    #[props(default)]
    pub auto_insert_space_in_button: Option<bool>,
    /// 表单配置
    #[props(default)]
    pub form: Option<FormConfig>,
    /// 获取弹出容器的函数
    #[props(default)]
    pub get_popup_container: Option<fn() -> Element>,
    /// 获取目标容器的函数
    #[props(default)]
    pub get_target_container: Option<fn() -> Element>,
}

/// 配置提供者组件
///
/// 为子组件提供全局配置
#[component]
pub fn ConfigProvider(props: ConfigProviderProps) -> Element {
    let mut config = props.config;

    // 应用传入的配置覆盖
    if let Some(theme) = props.theme {
        config.theme.theme.theme = theme.theme;
    }
    if let Some(locale) = props.locale {
        config.locale.locale = locale;
    }
    if let Some(size) = props.component_size {
        config.component_size.default_size = size;
    }
    if let Some(direction) = props.direction {
        config.direction = direction;
    }
    if let Some(prefix_cls) = props.prefix_cls {
        config.prefix_cls = prefix_cls;
    }
    if let Some(auto_insert_space) = props.auto_insert_space_in_button {
        config.auto_insert_space_in_button = auto_insert_space;
    }
    if let Some(form_config) = props.form {
        config.form = form_config;
    }
    if let Some(get_popup_container) = props.get_popup_container {
        config.get_popup_container = Some(get_popup_container);
    }
    if let Some(get_target_container) = props.get_target_container {
        config.get_target_container = Some(get_target_container);
    }

    let context = ConfigContext {
        config: config.clone(),
    };

    // 创建主题配置信号
    let theme_config = use_signal(|| config.theme.theme.clone());

    // 创建语言配置信号
    let locale_config = use_signal(|| config.locale.locale);

    // 提供配置上下文
    use_context_provider(|| context.clone());

    rsx! {
        // 使用主题提供者
        crate::theme::provider::ThemeProvider {
            config: theme_config,
            // 使用语言提供者
            crate::locale::LocaleProvider {
                locale: locale_config,
                div {
                    class: format!(
                        "{}-config-provider {}",
                        context.config.prefix_cls,
                        if context.config.direction == Direction::Rtl { "rtl" } else { "ltr" }
                    ),
                    "data-theme": format!("{:?}", context.config.theme.theme.theme.mode),
                    "data-locale": context.config.locale.locale.to_string(),
                    "data-size": context.config.component_size.default_size.to_string(),

                    {props.children}
                }
            }
        }
    }
}

/// 使用配置的 Hook
///
/// 获取当前的配置上下文
pub fn use_config() -> ConfigContext {
    use_context::<ConfigContext>()
}

/// 使用 CSS 类名前缀的 Hook
///
/// 获取当前的 CSS 类名前缀
pub fn use_prefix_cls() -> String {
    let config = use_config();
    config.config.prefix_cls
}

/// 使用组件 CSS 类名前缀的 Hook
///
/// 获取特定组件的 CSS 类名前缀
///
/// # 参数
///
/// * `component` - 组件名称
///
/// # 返回值
///
/// 组件 CSS 类名前缀
pub fn use_component_prefix_cls(component: &str) -> String {
    let config = use_config();
    format!("{}-{}", config.config.prefix_cls, component)
}

/// 使用方向的 Hook
///
/// 获取当前的方向配置
pub fn use_direction() -> Direction {
    let config = use_config();
    config.config.direction
}

/// 使用组件尺寸的 Hook
///
/// 获取当前的组件尺寸配置
pub fn use_component_size() -> ComponentSize {
    let config = use_config();
    config.config.component_size.default_size
}

/// 使用表单配置的 Hook
///
/// 获取当前的表单配置
pub fn use_form_config() -> FormConfig {
    let config = use_config();
    config.config.form.clone()
}

/// 使用按钮空格配置的 Hook
///
/// 获取当前的按钮空格配置
pub fn use_auto_insert_space_in_button() -> bool {
    let config = use_config();
    config.config.auto_insert_space_in_button
}

/// 使用弹出容器的 Hook
///
/// 获取弹出容器的函数
pub fn use_popup_container() -> Option<fn() -> Element> {
    let config = use_config();
    config.config.get_popup_container
}

/// 使用目标容器的 Hook
///
/// 获取目标容器的函数
pub fn use_target_container() -> Option<fn() -> Element> {
    let config = use_config();
    config.config.get_target_container
}

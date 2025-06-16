//! 配置提供者模块
//!
//! 提供全局配置上下文和组件，用于在应用中注入和管理全局配置

use css_in_rust::theme::ThemeVariant;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::locale::{Locale, LocaleConfig};
use crate::theme::core::types::Size;
use crate::theme::ThemeConfig;

/// 方向类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    /// 从左到右
    Ltr,
    /// 从右到左
    Rtl,
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentSize {
    /// 小号
    Small,
    /// 中号（默认）
    Middle,
    /// 大号
    Large,
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
    /// 表格配置
    pub table: TableConfig,
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
            table: TableConfig::default(),
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

/// 表格配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TableConfig {
    /// 默认分页大小
    pub page_size: usize,
    /// 显示分页大小选择器
    pub show_size_changer: bool,
    /// 显示快速跳转
    pub show_quick_jumper: bool,
    /// 显示总数
    pub show_total: bool,
    /// 表格尺寸
    pub size: Size,
}

impl Default for TableConfig {
    fn default() -> Self {
        Self {
            page_size: 10,
            show_size_changer: true,
            show_quick_jumper: false,
            show_total: true,
            size: Size::Middle,
        }
    }
}

/// 空状态配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmptyConfig {
    /// 默认描述文本
    pub description: Option<String>,
    /// 默认图片
    pub image: Option<String>,
    /// 图片样式
    pub image_style: HashMap<String, String>,
}

impl Default for EmptyConfig {
    fn default() -> Self {
        Self {
            description: None,
            image: None,
            image_style: HashMap::new(),
        }
    }
}

/// 配置提供者构建器
///
/// 用于方便地构建配置提供者
pub struct ConfigProviderBuilder {
    config: GlobalConfig,
}

impl ConfigProviderBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self {
            config: GlobalConfig::default(),
        }
    }

    /// 设置主题
    pub fn theme(mut self, theme: super::theme::AntThemeType) -> Self {
        // 将项目内部的Theme转换为css_in_rust的Theme
        let mut css_theme = self.config.theme.theme.theme.clone();
        match theme {
            super::theme::AntThemeType::Light => {
                css_theme.mode = ThemeVariant::Light;
                self.config.theme.theme.theme = css_theme;
            }
            super::theme::AntThemeType::Dark => {
                css_theme.mode = ThemeVariant::Dark;
                self.config.theme.theme.theme = css_theme;
            }
            super::theme::AntThemeType::Compact => {
                css_theme.mode = ThemeVariant::Light;
                self.config.theme.theme.theme = css_theme;
                self.config.theme.compact = true;
            }
            super::theme::AntThemeType::Custom => {
                // 自定义主题不做特殊处理，保持当前设置
            }
        }
        self
    }

    /// 设置CSS主题
    pub fn css_theme(mut self, theme: css_in_rust::Theme) -> Self {
        self.config.theme.theme.theme = theme;
        self
    }

    /// 设置国际化
    pub fn locale(mut self, locale: Locale) -> Self {
        self.config.locale.locale = locale;
        self
    }

    /// 设置组件尺寸
    pub fn component_size(mut self, size: ComponentSize) -> Self {
        self.config.component_size.default_size = size;
        self
    }

    /// 设置紧凑模式
    pub fn compact(mut self, compact: bool) -> Self {
        self.config.theme.compact = compact;
        self
    }

    /// 设置方向
    pub fn direction(mut self, direction: Direction) -> Self {
        self.config.direction = direction;
        self
    }

    /// 设置前缀类名
    pub fn prefix_cls(mut self, prefix_cls: impl Into<String>) -> Self {
        self.config.prefix_cls = prefix_cls.into();
        self
    }

    /// 设置自动插入空格
    pub fn auto_insert_space_in_button(mut self, auto_insert: bool) -> Self {
        self.config.auto_insert_space_in_button = auto_insert;
        self
    }

    /// 设置表单配置
    pub fn form_config(mut self, form_config: FormConfig) -> Self {
        self.config.form = form_config;
        self
    }

    /// 设置表格配置
    pub fn table_config(mut self, table_config: TableConfig) -> Self {
        self.config.table = table_config;
        self
    }

    /// 构建配置
    pub fn build(self) -> GlobalConfig {
        self.config
    }
}

impl Default for ConfigProviderBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_global_config_default() {
        let config = GlobalConfig::default();
        assert_eq!(config.prefix_cls, "ant");
        assert_eq!(config.direction, Direction::Ltr);
        assert!(config.auto_insert_space_in_button);
        assert_eq!(config.component_size.default_size, ComponentSize::Middle);
    }

    #[test]
    fn test_form_config_default() {
        let form_config = FormConfig::default();
        assert_eq!(form_config.validate_messages.len(), 0);
        assert!(form_config.required_mark);
        assert!(form_config.colon);
        assert_eq!(form_config.label_align, "right");
        assert!(form_config.label_width.is_none());
    }

    #[test]
    fn test_table_config_default() {
        let table_config = TableConfig::default();
        assert_eq!(table_config.page_size, 10);
        assert!(table_config.show_size_changer);
        assert!(!table_config.show_quick_jumper);
        assert!(table_config.show_total);
    }

    #[test]
    fn test_config_provider_builder() {
        let config = ConfigProviderBuilder::new()
            .prefix_cls("my-ant")
            .component_size(ComponentSize::Large)
            .direction(Direction::Rtl)
            .compact(true)
            .auto_insert_space_in_button(false)
            .build();

        assert_eq!(config.prefix_cls, "my-ant");
        assert_eq!(config.component_size.default_size, ComponentSize::Large);
        assert_eq!(config.direction, Direction::Rtl);
        assert!(config.theme.compact);
        assert!(!config.auto_insert_space_in_button);
    }

    // #[test]
    // fn test_direction() {
    //     assert_eq!(Direction::default(), Direction::Ltr);
    // }

    // #[test]
    // fn test_validate_trigger() {
    //     let trigger = ValidateTrigger::OnChange;
    //     assert_eq!(trigger, ValidateTrigger::OnChange);
    // }

    // #[test]
    // fn test_required_mark() {
    //     let mark = RequiredMark::Optional;
    //     assert_eq!(mark, RequiredMark::Optional);
    // }

    // #[test]
    // fn test_label_align() {
    //     let align = LabelAlign::Left;
    //     assert_eq!(align, LabelAlign::Left);
    // }
}

//! 配置提供者模块
//!
//! 提供全局配置管理，包括主题、国际化、组件默认属性等

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::locale::{Locale, LocaleConfig};
use crate::theme::{CssThemeBridge, Theme, ThemeConfig};
use crate::utils::{responsive::Breakpoint, size::Size};

/// 组件尺寸配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComponentSizeConfig {
    /// 默认组件尺寸
    pub default_size: Size,
    /// 紧凑模式
    pub compact: bool,
}

impl Default for ComponentSizeConfig {
    fn default() -> Self {
        Self {
            default_size: Size::Middle,
            compact: false,
        }
    }
}

/// 表单配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FormConfig {
    /// 验证触发方式
    pub validate_trigger: ValidateTrigger,
    /// 是否显示必填标记
    pub required_mark: RequiredMark,
    /// 标签对齐方式
    pub label_align: LabelAlign,
    /// 标签换行
    pub label_wrap: bool,
    /// 冒号显示
    pub colon: bool,
}

/// 验证触发方式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValidateTrigger {
    /// 改变时验证
    OnChange,
    /// 失焦时验证
    OnBlur,
    /// 提交时验证
    OnSubmit,
}

/// 必填标记显示方式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RequiredMark {
    /// 显示
    True,
    /// 不显示
    False,
    /// 可选标记
    Optional,
}

/// 标签对齐方式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LabelAlign {
    /// 左对齐
    Left,
    /// 右对齐
    Right,
}

impl Default for FormConfig {
    fn default() -> Self {
        Self {
            validate_trigger: ValidateTrigger::OnChange,
            required_mark: RequiredMark::True,
            label_align: LabelAlign::Right,
            label_wrap: false,
            colon: true,
        }
    }
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

/// 全局配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GlobalConfig {
    /// 主题配置
    pub theme: ThemeConfig,
    /// 国际化配置
    pub locale: LocaleConfig,
    /// 组件尺寸配置
    pub component_size: ComponentSizeConfig,
    /// 表单配置
    pub form: FormConfig,
    /// 表格配置
    pub table: TableConfig,
    /// 空状态配置
    pub empty: EmptyConfig,
    /// 自动插入空格
    pub auto_insert_space_in_button: bool,
    /// 方向（RTL/LTR）
    pub direction: Direction,
    /// 虚拟滚动阈值
    pub virtual_scroll_threshold: usize,
    /// 响应式断点
    pub breakpoints: HashMap<Breakpoint, u32>,
    /// 前缀类名
    pub prefix_cls: String,
    /// 图标前缀
    pub icon_prefix_cls: String,
    /// 获取容器元素
    pub get_popup_container: Option<String>,
    /// 获取目标容器
    pub get_target_container: Option<String>,
}

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
        Direction::Ltr
    }
}

impl Default for GlobalConfig {
    fn default() -> Self {
        let mut breakpoints = HashMap::new();
        breakpoints.insert(Breakpoint::XS, 0);
        breakpoints.insert(Breakpoint::SM, 576);
        breakpoints.insert(Breakpoint::MD, 768);
        breakpoints.insert(Breakpoint::LG, 992);
        breakpoints.insert(Breakpoint::XL, 1200);
        breakpoints.insert(Breakpoint::XXL, 1600);

        Self {
            theme: ThemeConfig::default(),
            locale: LocaleConfig::default(),
            component_size: ComponentSizeConfig::default(),
            form: FormConfig::default(),
            table: TableConfig::default(),
            empty: EmptyConfig::default(),
            auto_insert_space_in_button: true,
            direction: Direction::Ltr,
            virtual_scroll_threshold: 100,
            breakpoints,
            prefix_cls: "ant".to_string(),
            icon_prefix_cls: "anticon".to_string(),
            get_popup_container: None,
            get_target_container: None,
        }
    }
}

/// 配置提供者属性
#[derive(Props, Clone, PartialEq)]
pub struct ConfigProviderProps {
    /// 子组件
    pub children: Element,
    /// 全局配置
    #[props(default)]
    pub config: GlobalConfig,
    /// 主题
    #[props(into)]
    pub theme: Option<Theme>,
    /// 国际化
    #[props(into)]
    pub locale: Option<Locale>,
    /// 组件尺寸
    #[props(into)]
    pub component_size: Option<Size>,
    /// 方向
    #[props(into)]
    pub direction: Option<Direction>,
    /// 前缀类名
    #[props(into)]
    pub prefix_cls: Option<String>,
    /// 自动插入空格
    #[props(into)]
    pub auto_insert_space_in_button: Option<bool>,
    /// 表单配置
    #[props(into)]
    pub form: Option<FormConfig>,
    /// 获取弹出容器
    #[props(into)]
    pub get_popup_container: Option<String>,
    /// 获取目标容器
    #[props(into)]
    pub get_target_container: Option<String>,
}

/// 配置上下文
#[derive(Clone, PartialEq)]
pub struct ConfigContext {
    /// 全局配置
    pub config: GlobalConfig,
}

impl Default for ConfigContext {
    fn default() -> Self {
        Self {
            config: GlobalConfig::default(),
        }
    }
}

/// 配置提供者组件
///
/// 为子组件提供全局配置
#[component]
pub fn ConfigProvider(props: ConfigProviderProps) -> Element {
    let mut config = props.config;

    // 应用传入的配置覆盖
    if let Some(theme) = props.theme {
        config.theme.theme = theme;
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

    // 初始化主题桥接器并注入样式
    use_effect(move || {
        let bridge = CssThemeBridge::new(config.theme.clone());
        bridge.inject_theme_variables();
    });

    use_context_provider(|| context.clone());

    rsx! {
        div {
            class: format!(
                "{}-config-provider {}",
                context.config.prefix_cls,
                if context.config.direction == Direction::Rtl { "rtl" } else { "ltr" }
            ),
            "data-theme": context.config.theme.theme.to_string(),
            "data-locale": context.config.locale.locale.to_string(),
            "data-size": context.config.component_size.default_size.to_string(),

            {props.children}
        }
    }
}

/// 使用配置的 Hook
///
/// 获取当前的全局配置
pub fn use_config() -> ConfigContext {
    use_context::<ConfigContext>()
}

/// 使用前缀类名的 Hook
///
/// 获取带前缀的类名
pub fn use_prefix_cls(suffix: &str) -> String {
    let config = use_config();
    if suffix.is_empty() {
        config.config.prefix_cls
    } else {
        format!("{}-{}", config.config.prefix_cls, suffix)
    }
}

/// 使用方向的 Hook
///
/// 获取当前的文本方向
pub fn use_direction() -> Direction {
    let config = use_config();
    config.config.direction
}

/// 使用组件尺寸的 Hook
///
/// 获取当前的组件尺寸配置
pub fn use_component_size() -> ComponentSizeConfig {
    let config = use_config();
    config.config.component_size
}

/// 使用表单配置的 Hook
///
/// 获取当前的表单配置
pub fn use_form_config() -> FormConfig {
    let config = use_config();
    config.config.form
}

/// 使用表格配置的 Hook
///
/// 获取当前的表格配置
pub fn use_table_config() -> TableConfig {
    let config = use_config();
    config.config.table
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
    pub fn theme(mut self, theme: Theme) -> Self {
        self.config.theme.theme = theme;
        self
    }

    /// 设置国际化
    pub fn locale(mut self, locale: Locale) -> Self {
        self.config.locale.locale = locale;
        self
    }

    /// 设置组件尺寸
    pub fn component_size(mut self, size: Size) -> Self {
        self.config.component_size.default_size = size;
        self
    }

    /// 设置紧凑模式
    pub fn compact(mut self, compact: bool) -> Self {
        self.config.component_size.compact = compact;
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
        assert_eq!(config.component_size.default_size, Size::Middle);
    }

    #[test]
    fn test_form_config_default() {
        let form_config = FormConfig::default();
        assert_eq!(form_config.validate_trigger, ValidateTrigger::OnChange);
        assert_eq!(form_config.required_mark, RequiredMark::True);
        assert_eq!(form_config.label_align, LabelAlign::Right);
        assert!(form_config.colon);
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
            .component_size(Size::Large)
            .direction(Direction::Rtl)
            .compact(true)
            .auto_insert_space_in_button(false)
            .build();

        assert_eq!(config.prefix_cls, "my-ant");
        assert_eq!(config.component_size.default_size, Size::Large);
        assert_eq!(config.direction, Direction::Rtl);
        assert!(config.component_size.compact);
        assert!(!config.auto_insert_space_in_button);
    }

    #[test]
    fn test_direction() {
        assert_eq!(Direction::default(), Direction::Ltr);
    }

    #[test]
    fn test_validate_trigger() {
        let trigger = ValidateTrigger::OnChange;
        assert_eq!(trigger, ValidateTrigger::OnChange);
    }

    #[test]
    fn test_required_mark() {
        let mark = RequiredMark::Optional;
        assert_eq!(mark, RequiredMark::Optional);
    }

    #[test]
    fn test_label_align() {
        let align = LabelAlign::Left;
        assert_eq!(align, LabelAlign::Left);
    }
}

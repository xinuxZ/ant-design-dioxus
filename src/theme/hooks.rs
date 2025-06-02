//! Dioxus 主题 Hooks
//!
//! 本模块提供了在 Dioxus 组件中使用 Ant Design 主题的 hooks 和上下文管理。
//! 这些 hooks 简化了主题的使用，并提供了响应式的主题切换功能。

use dioxus::prelude::*;
use std::collections::HashMap;

use super::tokens::{
    animation_presets::{AntDesignAnimationConfig, AntDesignEasing},
    theme_presets::{AntDesignTheme, AntDesignThemePresets},
};
use css_in_rust::theme::{DesignTokens, ThemeVariant};

/// 主题上下文状态
#[derive(Debug, Clone, PartialEq)]
pub struct ThemeContext {
    /// 当前主题
    pub current_theme: AntDesignTheme,
    /// 可用的主题列表
    pub available_themes: HashMap<String, AntDesignTheme>,
    /// 是否启用自动主题切换
    pub auto_theme: bool,
    /// 主题切换动画配置
    pub transition_config: AntDesignAnimationConfig,
}

impl Default for ThemeContext {
    fn default() -> Self {
        Self {
            current_theme: AntDesignTheme::light(),
            available_themes: AntDesignThemePresets::all_presets(),
            auto_theme: false,
            transition_config: AntDesignAnimationConfig::new(AntDesignEasing::Standard)
                .with_duration(300)
                .with_delay(0),
        }
    }
}

/// 主题提供者组件的属性
#[derive(Props, PartialEq, Clone)]
pub struct ThemeProviderProps {
    /// 子组件
    children: Element,
    /// 初始主题（可选）
    #[props(default)]
    initial_theme: Option<AntDesignTheme>,
    /// 是否启用自动主题切换（可选）
    #[props(default = false)]
    auto_theme: bool,
    /// 主题切换动画持续时间（毫秒）
    #[props(default = 300)]
    transition_duration: u32,
}

/// 主题提供者组件
///
/// 为整个应用或组件树的一部分提供主题上下文。
/// 所有子组件都可以通过 `use_theme` hook 访问主题。
#[component]
pub fn ThemeProvider(props: ThemeProviderProps) -> Element {
    let initial_theme = props.initial_theme.unwrap_or_else(|| {
        if props.auto_theme {
            super::tokens::theme_presets::theme_utils::detect_system_theme()
        } else {
            AntDesignTheme::light()
        }
    });

    let theme_context = use_signal(|| ThemeContext {
        current_theme: initial_theme,
        available_themes: AntDesignThemePresets::all_presets(),
        auto_theme: props.auto_theme,
        transition_config: AntDesignAnimationConfig::new(AntDesignEasing::Standard)
            .with_duration(props.transition_duration as u64)
            .with_delay(0),
    });

    // 监听系统主题变化（如果启用了自动主题）
    use_effect(move || {
        if theme_context.read().auto_theme {
            // 在实际应用中，这里应该监听系统主题变化事件
            // 目前只是一个示例
        }
    });

    // 提供主题上下文
    use_context_provider(|| theme_context);

    // 生成 CSS 变量
    let css_variables = theme_context.read().current_theme.to_css_variables();

    rsx! {
        style {
            ":root {{\n{css_variables}}}"
        }
        {props.children}
    }
}

/// 使用主题的 Hook
///
/// 返回当前主题和主题操作函数。
/// 必须在 `ThemeProvider` 的子组件中使用。
pub fn use_theme() -> UseTheme {
    let context = use_context::<Signal<ThemeContext>>();
    UseTheme { context }
}

/// 主题 Hook 的返回类型
pub struct UseTheme {
    context: Signal<ThemeContext>,
}

impl UseTheme {
    /// 获取当前主题
    pub fn current_theme(&self) -> AntDesignTheme {
        self.context.read().current_theme.clone()
    }

    /// 切换到指定主题
    pub fn set_theme(&mut self, theme_name: &str) {
        let mut context = self.context.write();
        if let Some(theme) = context.available_themes.get(theme_name) {
            context.current_theme = theme.clone();
        }
    }

    /// 切换到自定义主题
    pub fn set_custom_theme(&mut self, theme: AntDesignTheme) {
        let mut context = self.context.write();
        context.current_theme = theme;
    }

    /// 在浅色和深色主题之间切换
    pub fn toggle_theme(&mut self) {
        let current_variant = self.context.read().current_theme.variant;
        match current_variant {
            ThemeVariant::Light => self.set_theme("dark"),
            ThemeVariant::Dark => self.set_theme("light"),
            ThemeVariant::Auto => self.set_theme("dark"), // 自动模式切换到深色
        }
    }

    /// 获取当前主题变体
    pub fn theme_variant(&self) -> ThemeVariant {
        self.context.read().current_theme.variant
    }

    /// 检查是否为深色主题
    pub fn is_dark(&self) -> bool {
        matches!(self.theme_variant(), ThemeVariant::Dark)
    }

    /// 检查是否为浅色主题
    pub fn is_light(&self) -> bool {
        matches!(self.theme_variant(), ThemeVariant::Light)
    }

    /// 获取主色调
    pub fn primary_color(&self) -> String {
        self.context.read().current_theme.primary_color.clone()
    }

    /// 获取成功色
    pub fn success_color(&self) -> String {
        self.context.read().current_theme.success_color.clone()
    }

    /// 获取警告色
    pub fn warning_color(&self) -> String {
        self.context.read().current_theme.warning_color.clone()
    }

    /// 获取错误色
    pub fn error_color(&self) -> String {
        self.context.read().current_theme.error_color.clone()
    }

    /// 获取信息色
    pub fn info_color(&self) -> String {
        self.context.read().current_theme.info_color.clone()
    }

    /// 启用/禁用自动主题切换
    pub fn set_auto_theme(&mut self, enabled: bool) {
        let mut context = self.context.write();
        context.auto_theme = enabled;
        if enabled {
            // 立即应用系统主题
            let system_theme = super::tokens::theme_presets::theme_utils::detect_system_theme();
            context.current_theme = system_theme;
        }
    }

    /// 获取可用主题列表
    pub fn available_themes(&self) -> Vec<String> {
        self.context
            .read()
            .available_themes
            .keys()
            .cloned()
            .collect()
    }

    /// 添加自定义主题到可用主题列表
    pub fn add_theme(&mut self, name: String, theme: AntDesignTheme) {
        let mut context = self.context.write();
        context.available_themes.insert(name, theme);
    }

    /// 移除主题
    pub fn remove_theme(&mut self, name: &str) {
        let mut context = self.context.write();
        context.available_themes.remove(name);
    }

    /// 获取当前主题的设计令牌
    pub fn design_tokens(&self) -> DesignTokens {
        self.context.read().current_theme.to_design_tokens()
    }

    /// 获取主题切换动画配置
    pub fn transition_config(&self) -> AntDesignAnimationConfig {
        self.context.read().transition_config.clone()
    }

    /// 设置主题切换动画配置
    pub fn set_transition_config(&mut self, config: AntDesignAnimationConfig) {
        let mut context = self.context.write();
        context.transition_config = config;
    }
}

/// 使用设计令牌的 Hook
///
/// 直接返回当前主题的设计令牌，方便在组件中使用。
pub fn use_design_tokens() -> DesignTokens {
    let theme = use_theme();
    theme.design_tokens()
}

/// 使用主题颜色的 Hook
///
/// 返回当前主题的主要颜色。
pub fn use_theme_colors() -> ThemeColors {
    let theme = use_theme();
    ThemeColors {
        primary: theme.primary_color(),
        success: theme.success_color(),
        warning: theme.warning_color(),
        error: theme.error_color(),
        info: theme.info_color(),
    }
}

/// 主题颜色结构
#[derive(Debug, Clone, PartialEq)]
pub struct ThemeColors {
    pub primary: String,
    pub success: String,
    pub warning: String,
    pub error: String,
    pub info: String,
}

/// 使用响应式主题的 Hook
///
/// 提供响应式的主题功能，包括媒体查询支持。
pub fn use_responsive_theme() -> UseResponsiveTheme {
    let theme = use_theme();
    UseResponsiveTheme { theme }
}

/// 响应式主题 Hook 的返回类型
pub struct UseResponsiveTheme {
    theme: UseTheme,
}

impl UseResponsiveTheme {
    /// 根据屏幕尺寸获取适配的主题
    pub fn theme_for_breakpoint(&self, breakpoint: &str) -> AntDesignTheme {
        let mut theme = self.theme.current_theme();

        // 根据断点调整主题（示例逻辑）
        match breakpoint {
            "xs" | "sm" => {
                // 小屏幕使用紧凑主题
                theme = AntDesignTheme::compact();
            }
            "md" | "lg" | "xl" => {
                // 大屏幕使用标准主题
                // 保持当前主题
            }
            _ => {}
        }

        theme
    }

    /// 检查是否为移动设备
    pub fn is_mobile(&self) -> bool {
        // 在实际应用中，这里应该检测屏幕尺寸
        // 目前返回 false
        false
    }

    /// 获取当前断点
    pub fn current_breakpoint(&self) -> String {
        // 在实际应用中，这里应该检测当前屏幕尺寸对应的断点
        // 目前返回默认值
        "lg".to_string()
    }
}

/// 主题持久化 Hook
///
/// 提供主题的本地存储功能。
pub fn use_theme_persistence() -> UseThemePersistence {
    let theme = use_theme();
    UseThemePersistence { theme }
}

/// 主题持久化 Hook 的返回类型
pub struct UseThemePersistence {
    theme: UseTheme,
}

impl UseThemePersistence {
    /// 保存当前主题到本地存储
    pub fn save_theme(&self) {
        let theme_name = match self.theme.theme_variant() {
            ThemeVariant::Light => "light",
            ThemeVariant::Dark => "dark",
            ThemeVariant::Auto => "auto",
        };

        // 在实际应用中，这里应该使用 localStorage 或其他持久化方案
        // 目前只是一个示例
        log::info!("Saving theme: {}", theme_name);
    }

    /// 从本地存储加载主题
    pub fn load_theme(&self) {
        // 在实际应用中，这里应该从 localStorage 读取主题设置
        // 目前只是一个示例
        log::info!("Loading theme from storage");
    }

    /// 清除本地存储的主题设置
    pub fn clear_theme(&self) {
        // 在实际应用中，这里应该清除 localStorage 中的主题设置
        log::info!("Clearing theme from storage");
    }
}

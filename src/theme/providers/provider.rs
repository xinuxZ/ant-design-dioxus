//!
//! 主题提供者组件
//!
//! 提供 ThemeProvider 组件，为整个应用或组件树的一部分提供主题上下文。

use dioxus::prelude::*;

use super::context::{
    ThemeColors, ThemeContext, UseResponsiveTheme, UseTheme, UseThemePersistence,
};
use super::use_theme;
use crate::theme::tokens::{
    animation_presets::{AntDesignAnimationConfig, AntDesignEasing},
    theme_presets::{AntDesignTheme, AntDesignThemePresets},
};
use css_in_rust::theme::DesignTokens;

#[cfg(target_arch = "wasm32")]
use web_sys;

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
///
/// # 示例
///
/// ```rust
/// use dioxus::prelude::*;
/// use ant_design_dioxus::theme::{ThemeProvider, AntDesignTheme};
///
/// #[component]
/// fn App() -> Element {
///     rsx! {
///         ThemeProvider {
///             initial_theme: AntDesignTheme::dark(),
///             auto_theme: true,
///             transition_duration: 200,
///
///             div {
///                 "Your app content here"
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn ThemeProvider(props: ThemeProviderProps) -> Element {
    let initial_theme = props.initial_theme.unwrap_or_else(|| {
        if props.auto_theme {
            crate::theme::tokens::theme_presets::theme_utils::detect_system_theme()
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

    // 生成 CSS 变量和主题样式
    let current_theme = &theme_context.read().current_theme;
    let css_variables = current_theme.to_css_variables();
    let theme_name = &current_theme.name;

    // 注入主题样式到 DOM
    use_effect(move || {
        let theme_context = theme_context.read();
        let theme_name = &theme_context.current_theme.name;

        // 设置根元素的 data-theme 属性（仅在 WASM 环境下）
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    if let Some(html_element) = document.document_element() {
                        let _ = html_element.set_attribute("data-theme", theme_name);
                    }
                }
            }
        }
    });

    rsx! {
        style {
            ":root {{\n{css_variables}}}"
        }
        {props.children}
    }
}

/// 使用设计令牌的 Hook
///
/// 直接返回当前主题的设计令牌，方便在组件中使用。
///
/// # 示例
///
/// ```rust
/// use dioxus::prelude::*;
/// use ant_design_dioxus::theme::use_design_tokens;
///
/// #[component]
/// fn TokenizedComponent() -> Element {
///     let tokens = use_design_tokens();
///
///     rsx! {
///         div {
///             style: "padding: {tokens.spacing.md}px;",
///             "Tokenized content"
///         }
///     }
/// }
/// ```
pub fn use_design_tokens() -> DesignTokens {
    let theme = use_theme();
    theme.design_tokens()
}

/// 使用主题颜色的 Hook
///
/// 返回当前主题的主要颜色。
///
/// # 示例
///
/// ```rust
/// use dioxus::prelude::*;
/// use ant_design_dioxus::theme::use_theme_colors;
///
/// #[component]
/// fn ColoredComponent() -> Element {
///     let colors = use_theme_colors();
///
///     rsx! {
///         div {
///             style: "color: {colors.primary}; background: {colors.success};",
///             "Colored content"
///         }
///     }
/// }
/// ```
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

/// 使用响应式主题的 Hook
///
/// 提供响应式的主题功能，包括媒体查询支持。
///
/// # 示例
///
/// ```rust
/// use dioxus::prelude::*;
/// use ant_design_dioxus::theme::use_responsive_theme;
///
/// #[component]
/// fn ResponsiveComponent() -> Element {
///     let responsive = use_responsive_theme();
///     let breakpoint = responsive.current_breakpoint();
///
///     rsx! {
///         div {
///             class: "responsive-{breakpoint}",
///             "Responsive content for {breakpoint}"
///         }
///     }
/// }
/// ```
pub fn use_responsive_theme() -> UseResponsiveTheme {
    let theme = use_theme();
    UseResponsiveTheme::new(theme)
}

/// 主题持久化 Hook
///
/// 提供主题的本地存储功能。
///
/// # 示例
///
/// ```rust
/// use dioxus::prelude::*;
/// use ant_design_dioxus::theme::use_theme_persistence;
///
/// #[component]
/// fn PersistentThemeComponent() -> Element {
///     let persistence = use_theme_persistence();
///
///     use_effect(move || {
///         persistence.load_theme();
///     });
///
///     rsx! {
///         button {
///             onclick: move |_| {
///                 persistence.save_theme();
///             },
///             "Save Theme"
///         }
///     }
/// }
/// ```
pub fn use_theme_persistence() -> UseThemePersistence {
    let theme = use_theme();
    UseThemePersistence::new(theme)
}

//!
//! 主题相关 Hooks
//!
//! 提供在 Dioxus 组件中使用 Ant Design 主题的便捷 hooks，
//! 简化主题的使用，并提供响应式的主题切换功能。

use dioxus::prelude::*;

use super::context::{ThemeContext, UseTheme};
use crate::theme::tokens::theme_presets::ThemePreset;
use css_in_rust::theme::ThemeVariant;

/// 使用主题的 Hook
///
/// 返回主题管理器，提供完整的主题操作功能
///
/// # 返回值
///
/// 返回 `UseTheme` 实例，提供以下功能：
/// - 获取当前主题
/// - 切换主题
/// - 检查主题状态
///
/// # 示例
///
/// ```rust
/// use dioxus::prelude::*;
/// use ant_design_dioxus::theme::use_theme;
///
/// #[component]
/// fn MyComponent() -> Element {
///     let mut theme = use_theme();
///
///     rsx! {
///         div {
///             style: "color: {theme.primary_color()};",
///             "Hello, themed world!"
///         }
///         button {
///             onclick: move |_| {
///                 theme.toggle_theme();
///             },
///             "Toggle Theme"
///         }
///     }
/// }
/// ```
pub fn use_theme() -> UseTheme {
    let context = use_context::<Signal<ThemeContext>>();
    UseTheme::new(context)
}

/// 使用主题的简化 Hook
///
/// 返回当前主题配置和主题切换函数的元组形式
///
/// # 返回值
///
/// 返回一个元组 `(theme, set_theme)`，其中：
/// - `theme`: 当前的主题配置
/// - `set_theme`: 用于切换主题的函数
///
/// # 示例
///
/// ```rust
/// use dioxus::prelude::*;
/// use ant_design_dioxus::theme::use_theme_simple;
///
/// #[component]
/// fn MyComponent() -> Element {
///     let (theme, mut set_theme) = use_theme_simple();
///
///     rsx! {
///         div {
///             style: "color: {theme.colors.primary};",
///             "Hello, themed world!"
///         }
///         button {
///             onclick: move |_| {
///                 set_theme(ThemePreset::dark());
///             },
///             "Switch to Dark Theme"
///         }
///     }
/// }
/// ```
pub fn use_theme_simple() -> (ThemePreset, impl FnMut(ThemePreset) + Clone) {
    let mut context = use_context::<Signal<ThemeContext>>();

    let theme = context.read().current_theme.clone();

    let set_theme = {
        move |new_theme: ThemePreset| {
            context.with_mut(|ctx| {
                ctx.current_theme = new_theme;
            });
        }
    };

    (theme, set_theme)
}

/// 使用主题上下文的 Hook
///
/// 返回完整的主题上下文，包括当前主题、可用主题列表等
///
/// # 返回值
///
/// 返回 `ThemeContext` 的 Signal
///
/// # 示例
///
/// ```rust
/// use dioxus::prelude::*;
/// use ant_design_dioxus::theme::use_theme_context;
///
/// #[component]
/// fn ThemeSelector() -> Element {
///     let context = use_theme_context();
///
///     rsx! {
///         select {
///             onchange: move |evt| {
///                 if let Some(theme) = context.read().available_themes.get(&evt.value()) {
///                     context.write().current_theme = theme.clone();
///                 }
///             },
///             for (name, _) in context.read().available_themes.iter() {
///                 option { value: "{name}", "{name}" }
///             }
///         }
///     }
/// }
/// ```
pub fn use_theme_context() -> Signal<ThemeContext> {
    use_context::<Signal<ThemeContext>>()
}

/// 使用主题颜色的 Hook
///
/// 提供便捷的颜色访问方法
///
/// # 返回值
///
/// 返回当前主题的颜色配置
///
/// # 示例
///
/// ```rust
/// use dioxus::prelude::*;
/// use ant_design_dioxus::theme::use_theme_colors;
///
/// #[component]
/// fn ColoredButton() -> Element {
///     let colors = use_theme_colors();
///
///     rsx! {
///         button {
///             style: "background-color: {colors.primary}; color: {colors.text_primary};",
///             "Themed Button"
///         }
///     }
/// }
/// ```
pub fn use_theme_colors() -> crate::theme::tokens::color_presets::AntDesignColors {
    // 返回AntDesignColors实例，调用者可以使用其静态方法获取颜色
    crate::theme::tokens::color_presets::AntDesignColors
}

/// 使用主题动画配置的 Hook
///
/// 提供便捷的动画配置访问方法
///
/// # 返回值
///
/// 返回当前主题的动画配置
///
/// # 示例
///
/// ```rust
/// use dioxus::prelude::*;
/// use ant_design_dioxus::theme::use_theme_animations;
///
/// #[component]
/// fn AnimatedComponent() -> Element {
///     let animations = use_theme_animations();
///
///     rsx! {
///         div {
///             style: "transition: all {animations.duration_base}ms {animations.ease_out};",
///             "Animated content"
///         }
///     }
/// }
/// ```
pub fn use_theme_animations() -> crate::theme::ant_design::animations::AntDesignAnimations {
    // 从设计令牌中获取动画配置
    crate::theme::ant_design::animations::AntDesignAnimations::default()
}

/// 使用主题尺寸配置的 Hook
///
/// 提供便捷的尺寸配置访问方法
///
/// # 返回值
///
/// 返回当前主题的尺寸配置
///
/// # 示例
///
/// ```rust
/// use dioxus::prelude::*;
/// use ant_design_dioxus::theme::use_theme_sizes;
///
/// #[component]
/// fn SizedComponent() -> Element {
///     let sizes = use_theme_sizes();
///
///     rsx! {
///         button {
///             style: "height: {sizes.control_height}px; padding: 0 {sizes.padding_md}px;",
///             "Sized Button"
///         }
///     }
/// }
/// ```
pub fn use_theme_sizes() -> crate::theme::ant_design::sizing::AntDesignSizing {
    // 从设计令牌中获取尺寸配置
    crate::theme::ant_design::sizing::AntDesignSizing::default()
}

/// 使用主题间距配置的 Hook
///
/// 提供便捷的间距配置访问方法
///
/// # 返回值
///
/// 返回当前主题的间距配置
///
/// # 示例
///
/// ```rust
/// use dioxus::prelude::*;
/// use ant_design_dioxus::theme::use_theme_spacing;
///
/// #[component]
/// fn SpacedComponent() -> Element {
///     let spacing = use_theme_spacing();
///
///     rsx! {
///         div {
///             style: "margin: {spacing.md}px; padding: {spacing.sm}px;",
///             "Spaced content"
///         }
///     }
/// }
/// ```
pub fn use_theme_spacing() -> crate::theme::ant_design::spacing::AntDesignSpacing {
    // 从设计令牌中获取间距配置
    crate::theme::ant_design::spacing::AntDesignSpacing::default()
}

/// 使用主题字体配置的 Hook
///
/// 提供便捷的字体配置访问方法
///
/// # 返回值
///
/// 返回当前主题的字体配置
///
/// # 示例
///
/// ```rust
/// use dioxus::prelude::*;
/// use ant_design_dioxus::theme::use_theme_typography;
///
/// #[component]
/// fn TypographyComponent() -> Element {
///     let typography = use_theme_typography();
///
///     rsx! {
///         h1 {
///             style: "font-size: {typography.h1.size}; font-weight: {typography.h1.weight};",
///             "Themed Heading"
///         }
///     }
/// }
/// ```
pub fn use_theme_typography() -> crate::theme::ant_design::typography::AntDesignTypography {
    // 从设计令牌中获取字体配置
    crate::theme::ant_design::typography::AntDesignTypography::default()
}

/// 使用暗色模式检测的 Hook
///
/// 检测当前是否为暗色模式
///
/// # 返回值
///
/// 返回布尔值，表示当前是否为暗色模式
///
/// # 示例
///
/// ```rust
/// use dioxus::prelude::*;
/// use ant_design_dioxus::theme::use_dark_mode;
///
/// #[component]
/// fn DarkModeIndicator() -> Element {
///     let is_dark = use_dark_mode();
///
///     rsx! {
///         div {
///             "Current mode: {if is_dark { \"Dark\" } else { \"Light\" }}"
///         }
///     }
/// }
/// ```
pub fn use_dark_mode() -> bool {
    let (theme, _) = use_theme_simple();
    matches!(theme.variant, ThemeVariant::Dark)
}

/// 使用主题切换的 Hook
///
/// 提供便捷的主题切换功能
///
/// # 返回值
///
/// 返回一个函数，用于在明暗主题之间切换
///
/// # 示例
///
/// ```rust
/// use dioxus::prelude::*;
/// use ant_design_dioxus::theme::use_theme_toggle;
///
/// #[component]
/// fn ThemeToggleButton() -> Element {
///     let toggle_theme = use_theme_toggle();
///
///     rsx! {
///         button {
///             onclick: move |_| toggle_theme(),
///             "Toggle Theme"
///         }
///     }
/// }
/// ```
pub fn use_theme_toggle() -> impl FnMut() + Clone {
    let (theme, mut set_theme) = use_theme_simple();

    move || {
        let new_theme = match theme.variant {
            ThemeVariant::Light => ThemePreset::dark(),
            ThemeVariant::Dark => ThemePreset::light(),
            _ => ThemePreset::light(),
        };
        set_theme(new_theme);
    }
}

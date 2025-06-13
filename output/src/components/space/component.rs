//! Space 间距组件
//!
//! Space 组件用于设置组件之间的间距，避免组件紧贴在一起，拉开统一的空间。
//! 适用于需要在某个方向上保持统一间距的场景，支持水平、垂直方向的间距设置。
//!
//! ## 何时使用
//!
//! - 避免组件紧贴在一起，拉开统一的空间
//! - 在某组件的某个方向上，保持统一的间距
//! - 支持水平、垂直方向
//! - 支持自动换行和对齐方式
//!
//! ## 示例
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::{Space, SpaceSize, SpaceDirection};
//!
//! fn app() -> Element {
//!     rsx! {
//!         Space {
//!             size: SpaceSize::Large,
//!             direction: SpaceDirection::Vertical,
//!             Button { "按钮1" }
//!             Button { "按钮2" }
//!             Button { "按钮3" }
//!         }
//!     }
//! }
//! ```

use css_in_rust::css;
use css_in_rust::theme::theme_types::ThemeMode;
use dioxus::prelude::*;

use crate::config_provider::use_config;
use crate::config_provider::ConfigContext;
use crate::theme::{use_theme, Theme};

use super::styles::{SpaceStyleGenerator, SpaceStyles};
use super::types::*;

/// Space 间距组件
///
/// 设置组件之间的间距，避免组件紧贴在一起。
///
/// # Props
/// - `direction`: 间距方向，默认为水平
/// - `size`: 间距大小，默认为中等
/// - `align`: 对齐方式，默认为起始对齐
/// - `wrap`: 是否自动换行，仅在水平方向时有效
/// - `split`: 设置拆分元素
/// - `class`: 自定义CSS类名
/// - `style`: 自定义样式
/// - `prefix_cls`: 自定义前缀类名
/// - `children`: 子元素
///
/// # 示例
///
/// ```rust
/// use dioxus::prelude::*;
/// use ant_design_dioxus::{Space, SpaceSize, SpaceDirection};
///
/// fn app() -> Element {
///     rsx! {
///         Space {
///             size: SpaceSize::Large,
///             direction: SpaceDirection::Vertical,
///             Button { "按钮1" }
///             Button { "按钮2" }
///             Button { "按钮3" }
///         }
///     }
/// }
/// ```
#[component]
pub fn Space(props: SpaceProps) -> Element {
    // 获取配置和主题
    let config = use_config();
    let theme_context = use_theme();

    // 构建样式生成器
    let mut style_generator = SpaceStyleGenerator::new()
        .with_direction(props.direction.clone())
        .with_size(props.size.clone())
        .with_align(props.align.clone())
        .with_wrap(props.wrap);

    // 设置前缀类名
    if let Some(prefix) = &props.prefix_cls {
        style_generator = style_generator.with_prefix_cls(prefix);
    } else {
        style_generator = style_generator.with_prefix_cls(&config.config.prefix_cls);
    }

    // 设置主题令牌 - 转换css_in_rust::Theme到本地Theme
    let local_theme = match theme_context.config.theme.mode {
        ThemeMode::Light => Theme::Light,
        ThemeMode::Dark => Theme::Dark,
        _ => Theme::Light,
    };
    style_generator = style_generator.with_theme_token(local_theme);

    // 设置分割元素
    if props.split.is_some() {
        style_generator = style_generator.with_split(true);
    }

    // 注册样式
    let styles = SpaceStyles::new();
    use_effect(move || {
        let _ = css!(styles.base_styles());
        let _ = css!(styles.variant_styles());
    });

    // 生成CSS类名
    let class_name = style_generator.generate_class_name();
    let final_class = if let Some(custom_class) = &props.class {
        format!("{} {}", class_name, custom_class)
    } else {
        class_name
    };

    // 生成内联样式
    let inline_style = style_generator.generate_inline_styles();
    let final_style = if let Some(custom_style) = &props.style {
        if inline_style.is_empty() {
            custom_style.clone()
        } else {
            format!("{};{}", inline_style, custom_style)
        }
    } else {
        inline_style
    };

    // 渲染组件
    rsx! {
        div {
            class: final_class,
            style: if final_style.is_empty() { None } else { Some(final_style) },
            {props.children}
        }
    }
}

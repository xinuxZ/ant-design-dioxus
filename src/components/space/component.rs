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
    let prefix_cls = if let Some(prefix) = &props.prefix_cls {
        style_generator = style_generator.with_prefix_cls(prefix);
        prefix.clone()
    } else {
        style_generator = style_generator.with_prefix_cls(&config.config.prefix_cls);
        config.config.prefix_cls.clone()
    };

    // 设置主题令牌 - 转换css_in_rust::Theme到本地Theme
    let local_theme = match theme_context.config.theme.mode {
        ThemeMode::Light => Theme::Light,
        ThemeMode::Dark => Theme::Dark,
        _ => Theme::Light,
    };
    style_generator = style_generator.with_theme_token(local_theme);

    // 设置分割元素（支持简单版本和增强版本）
    let has_split = props.split.is_some() || props.split_config.is_some();
    if has_split {
        style_generator = style_generator.with_split(true);
    }

    // 设置新功能配置
    style_generator = style_generator.with_debug_config(props.debug_config.clone());
    style_generator = style_generator.with_animation_config(props.animation_config.clone());
    style_generator = style_generator.with_performance_config(props.performance_config.clone());
    style_generator = style_generator.with_i18n_config(props.i18n_config.clone());

    // 注册样式
    let styles = SpaceStyles::new();
    use_effect(move || {
        let _ = css!(styles.base_styles());
        let _ = css!(styles.variant_styles());
    });

    // 生成CSS类名（支持语义化类名）
    let class_name = {
        let mut classes = vec![style_generator.generate_class_name()];

        // 添加自定义类名
        if let Some(custom_class) = &props.class {
            classes.push(custom_class.clone());
        }

        // 添加语义化类名
        if let Some(class_names) = &props.class_names {
            for (_, class) in class_names {
                classes.push(class.clone());
            }
        }

        classes.join(" ")
    };

    // 生成内联样式（支持语义化样式）
    let inline_style = {
        let mut styles = vec![style_generator.generate_inline_styles()];

        // 添加自定义样式
        if let Some(custom_style) = &props.style {
            styles.push(custom_style.clone());
        }

        // 添加语义化样式
        if let Some(semantic_styles) = &props.styles {
            for (_, style) in semantic_styles {
                styles.push(style.clone());
            }
        }

        styles.join("; ")
    };

    let final_class = class_name;
    let final_style = inline_style;

    // 渲染组件
    rsx! {
        div {
            class: final_class,
            style: if final_style.is_empty() { None } else { Some(final_style) },

            // 调试模式数据属性
            "data-space-size": if props.debug_config.show_size_info {
                Some(format!("{:?}", props.size))
            } else {
                None
            },

            // 处理子元素和分割元素
            {
                let children_vec: Vec<_> = props.children.into_iter().collect();
                let mut elements = Vec::new();

                for (index, child) in children_vec.iter().enumerate() {
                    // 添加子元素包装器
                    elements.push(rsx! {
                        div {
                            class: format!("{}-item", prefix_cls),
                            key: format!("item-{}", index),
                            {child.clone()}
                        }
                    });

                    // 添加分割元素（如果不是最后一个子元素）
                    if index < children_vec.len() - 1 {
                        // 优先使用增强的分割元素配置
                        if let Some(split_config) = &props.split_config {
                            // 只有在visible为true时才显示分割元素
                            if split_config.visible {
                                elements.push(rsx! {
                                    div {
                                        class: if let Some(class) = &split_config.class {
                                            format!("{}-split {}", prefix_cls, class)
                                        } else {
                                            format!("{}-split", prefix_cls)
                                        },
                                        style: split_config.style.clone(),
                                        key: format!("split-{}", index),
                                        {split_config.element.clone()}
                                    }
                                });
                            }
                        }
                        // 向后兼容：使用简单分割元素
                        else if let Some(split) = &props.split {
                            elements.push(rsx! {
                                div {
                                    class: format!("{}-split", prefix_cls),
                                    key: format!("split-{}", index),
                                    {split.clone()}
                                }
                            });
                        }
                    }
                }

                elements.into_iter()
            }
        }
    }
}

/// Space.Compact 紧凑间距组件
///
/// 当子表单组件紧密连接且边框折叠时使用。
/// 适用于需要紧凑布局的场景，如表单控件组合、按钮组等。
///
/// # Props
/// - `block`: 适应父元素宽度，默认为false
/// - `direction`: 间距方向，默认为水平
/// - `size`: 紧凑间距大小，默认为中等
/// - `class`: 自定义CSS类名
/// - `style`: 自定义样式
/// - `prefix_cls`: 自定义前缀类名
/// - `children`: 子元素
///
/// # 示例
///
/// ```rust
/// use dioxus::prelude::*;
/// use ant_design_dioxus::{SpaceCompact, SpaceCompactSize, SpaceDirection};
///
/// fn app() -> Element {
///     rsx! {
///         SpaceCompact {
///             size: SpaceCompactSize::Small,
///             direction: SpaceDirection::Horizontal,
///             Button { "按钮1" }
///             Button { "按钮2" }
///             Button { "按钮3" }
///         }
///     }
/// }
/// ```
#[component]
pub fn SpaceCompact(props: SpaceCompactProps) -> Element {
    // 获取配置和主题
    let config = use_config();
    let theme_context = use_theme();

    // 构建样式生成器
    let mut style_generator = SpaceStyleGenerator::new()
        .with_direction(props.direction.clone())
        .with_align(SpaceAlign::Start) // 紧凑模式默认起始对齐
        .with_wrap(false); // 紧凑模式不支持换行

    // 设置紧凑模式特定的尺寸
    let compact_size = match props.size {
        SpaceCompactSize::Small => SpaceSize::Custom(4),
        SpaceCompactSize::Middle => SpaceSize::Custom(8),
        SpaceCompactSize::Large => SpaceSize::Custom(12),
    };
    style_generator = style_generator.with_size(compact_size);

    // 设置前缀类名
    let prefix_cls = props
        .prefix_cls
        .as_ref()
        .unwrap_or(&config.config.prefix_cls);
    style_generator = style_generator.with_prefix_cls(prefix_cls);

    // 设置主题令牌
    let local_theme = match theme_context.config.theme.mode {
        ThemeMode::Light => Theme::Light,
        ThemeMode::Dark => Theme::Dark,
        _ => Theme::Light,
    };
    style_generator = style_generator.with_theme_token(local_theme);

    // 生成样式
    let styles = style_generator.generate();

    // 构建类名
    let mut class_parts = vec![format!("{}-space-compact", prefix_cls), styles.base];

    // 添加方向类名
    match props.direction {
        SpaceDirection::Horizontal => {
            class_parts.push(format!("{}-space-compact-horizontal", prefix_cls))
        }
        SpaceDirection::Vertical => {
            class_parts.push(format!("{}-space-compact-vertical", prefix_cls))
        }
    }

    // 添加尺寸类名
    let size_class = match props.size {
        SpaceCompactSize::Small => format!("{}-space-compact-sm", prefix_cls),
        SpaceCompactSize::Middle => format!("{}-space-compact-md", prefix_cls),
        SpaceCompactSize::Large => format!("{}-space-compact-lg", prefix_cls),
    };
    class_parts.push(size_class);

    // 添加block类名
    if props.block {
        class_parts.push(format!("{}-space-compact-block", prefix_cls));
    }

    // 添加自定义类名
    if let Some(custom_class) = &props.class {
        class_parts.push(custom_class.clone());
    }

    let class = class_parts.join(" ");

    // 构建自定义样式
    let mut style_parts = Vec::new();

    // 处理不同类型的尺寸
    match &props.size {
        SpaceCompactSize::Small => {
            style_parts.push("--ant-space-gap: 4px".to_string());
        }
        SpaceCompactSize::Middle => {
            style_parts.push("--ant-space-gap: 8px".to_string());
        }
        SpaceCompactSize::Large => {
            style_parts.push("--ant-space-gap: 12px".to_string());
        }
    }

    // 添加紧凑模式特定样式
    let gap_value = match props.size {
        SpaceCompactSize::Small => "4px",
        SpaceCompactSize::Middle => "8px",
        SpaceCompactSize::Large => "12px",
    };
    style_parts.push(format!("--ant-space-compact-gap: {}", gap_value));

    // 添加自定义样式
    if let Some(custom_style) = &props.style {
        style_parts.push(custom_style.clone());
    }

    let style = if style_parts.is_empty() {
        String::new()
    } else {
        style_parts.join("; ")
    };

    rsx! {
        div {
            class: class,
            style: style,
            {props.children}
        }
    }
}

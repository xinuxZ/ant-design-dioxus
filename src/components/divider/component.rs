//! Divider 组件实现

use crate::components::divider::styles::*;
use crate::components::divider::types::*;
use crate::components::divider::utils::*;
use dioxus::prelude::*;
use wasm_bindgen::JsCast;

/// Divider 分割线组件
///
/// 用于分隔内容的分割线组件，支持水平和垂直方向，可以添加文本内容。
///
/// # 特性
/// - 支持水平和垂直分割线
/// - 支持添加文本内容
/// - 支持多种样式变体（实线、虚线、点线）
/// - 支持不同尺寸
/// - 支持文本位置控制
/// - 支持主题定制
/// - 支持响应式设计
/// - 支持可访问性
///
/// # 示例
///
/// ```rust
/// use dioxus::prelude::*;
/// use ant_design_dioxus::Divider;
///
/// fn app() -> Element {
///     rsx! {
///         div {
///             p { "上方内容" }
///             Divider {}
///             p { "下方内容" }
///
///             Divider {
///                 "带文本的分割线"
///             }
///
///             Divider {
///                 r#type: DividerType::Vertical,
///             }
///
///             Divider {
///                 variant: DividerVariant::Dashed,
///                 orientation: DividerOrientation::Left,
///                 "左对齐文本"
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn Divider(props: DividerProps) -> Element {
    // 验证 Props
    if let Err(error) = validate_divider_props(&props) {
        log::warn!("Divider props validation failed: {}", error);
    }

    // 创建配置
    let has_text = props.children.is_some();
    let config = DividerConfig::from_props(&props, has_text);

    // 获取主题
    let theme = use_divider_theme();

    // 生成样式
    let container_style = generate_divider_container_style(&config, &theme);
    let class_name = generate_divider_class_name(&config);

    // 合并自定义样式
    let final_style = if let Some(ref custom_style) = config.style {
        format!("{};{}", container_style, custom_style)
    } else {
        container_style
    };

    if config.is_vertical() {
        // 垂直分割线
        render_vertical_divider(&config, &theme, &class_name, &final_style)
    } else if config.has_text {
        // 带文本的水平分割线
        render_horizontal_divider_with_text(
            &config,
            &theme,
            &class_name,
            &final_style,
            props.children,
        )
    } else {
        // 纯水平分割线
        render_horizontal_divider(&config, &theme, &class_name, &final_style)
    }
}

/// 渲染垂直分割线
fn render_vertical_divider(
    config: &DividerConfig,
    theme: &DividerTheme,
    class_name: &str,
    style: &str,
) -> Element {
    let line_style = generate_divider_line_style(config, theme, None);

    rsx! {
        span {
            class: "{class_name}",
            style: "{style}",
            role: "separator",
            "aria-orientation": "vertical",
            span {
                class: "ant-divider-line",
                style: "{line_style}",
            }
        }
    }
}

/// 渲染水平分割线
fn render_horizontal_divider(
    config: &DividerConfig,
    theme: &DividerTheme,
    class_name: &str,
    style: &str,
) -> Element {
    let line_style = generate_divider_line_style(config, theme, None);

    rsx! {
        div {
            class: "{class_name}",
            style: "{style}",
            role: "separator",
            "aria-orientation": "horizontal",
            hr {
                class: "ant-divider-line",
                style: "{line_style}",
            }
        }
    }
}

/// 渲染带文本的水平分割线
fn render_horizontal_divider_with_text(
    config: &DividerConfig,
    theme: &DividerTheme,
    class_name: &str,
    style: &str,
    children: Option<Element>,
) -> Element {
    let before_line_style = generate_divider_line_style(config, theme, Some("before"));
    let after_line_style = generate_divider_line_style(config, theme, Some("after"));
    let text_style = generate_divider_text_style(config, theme);

    let actual_orientation = get_actual_orientation(&config.orientation);

    rsx! {
        div {
            class: "{class_name}",
            style: "{style}",
            role: "separator",
            "aria-orientation": "horizontal",
            "aria-label": "分割线",

            // 前置分割线
            if !matches!(actual_orientation, DividerOrientation::Left) {
                hr {
                    class: "ant-divider-line ant-divider-line-before",
                    style: "{before_line_style}",
                }
            }

            // 文本内容
            span {
                class: "ant-divider-inner-text",
                style: "{text_style}",
                {children}
            }

            // 后置分割线
            if !matches!(actual_orientation, DividerOrientation::Right) {
                hr {
                    class: "ant-divider-line ant-divider-line-after",
                    style: "{after_line_style}",
                }
            }
        }
    }
}

/// Divider 主题提供者
#[component]
pub fn DividerThemeProvider(
    theme: Option<DividerTheme>,
    dark_theme: Option<DividerDarkTheme>,
    compact_theme: Option<DividerCompactTheme>,
    children: Element,
) -> Element {
    let theme = theme.unwrap_or_default();
    let context = DividerThemeContext {
        theme: theme.clone(),
        dark_theme,
        compact_theme,
    };

    use_context_provider(|| context);

    // 注入全局样式
    let stylesheet = generate_divider_stylesheet(&theme);

    rsx! {
        style { "{stylesheet}" }
        {children}
    }
}

/// Divider 主题上下文
#[derive(Clone, PartialEq)]
struct DividerThemeContext {
    theme: DividerTheme,
    dark_theme: Option<DividerDarkTheme>,
    compact_theme: Option<DividerCompactTheme>,
}

/// 使用 Divider 主题的 Hook
pub fn use_divider_theme() -> DividerTheme {
    use_context::<DividerThemeContext>().theme

    // Some(context) => context.theme.clone(),
    // None => get_default_divider_theme(),
}

/// 使用暗色主题的 Hook
pub fn use_divider_dark_theme() -> Option<DividerDarkTheme> {
    use_context::<DividerThemeContext>()
        .dark_theme
        .and_then(|t| Some(t))
}

/// 使用紧凑主题的 Hook
pub fn use_divider_compact_theme() -> Option<DividerCompactTheme> {
    use_context::<DividerThemeContext>()
        .compact_theme
        .and_then(|t| Some(t))
}

/// 响应式 Divider Hook
pub fn use_responsive_divider(base_config: DividerConfig) -> DividerConfig {
    let window_size = use_window_size();

    let mut config = base_config;

    // 根据屏幕尺寸调整配置
    if window_size.width < 576.0 {
        // 小屏幕
        config.size = DividerSize::Small;
    } else if window_size.width > 1200.0 {
        // 大屏幕
        config.size = DividerSize::Large;
    }

    config
}

/// 窗口尺寸结构体
#[derive(Clone, PartialEq, Debug)]
struct WindowSize {
    width: f64,
    height: f64,
}

/// 获取窗口尺寸的 Hook
fn use_window_size() -> WindowSize {
    let mut size = use_signal(|| WindowSize {
        width: 1024.0,
        height: 768.0,
    });

    use_effect(move || {
        // 获取 window 对象
        let window = web_sys::window().expect("获取window对象失败");

        // 创建 resize 事件处理函数
        let mut size_clone = size.clone();
        let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
            let window = web_sys::window().expect("获取window对象失败");
            size_clone.set(WindowSize {
                width: window.inner_width().unwrap().as_f64().unwrap(),
                height: window.inner_height().unwrap().as_f64().unwrap(),
            });
        }) as Box<dyn FnMut()>);

        // 添加 resize 事件监听
        window
            .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
            .expect("添加resize事件监听失败");

        // 初始化窗口尺寸
        size.set(WindowSize {
            width: window.inner_width().unwrap().as_f64().unwrap(),
            height: window.inner_height().unwrap().as_f64().unwrap(),
        });

        // 清理函数
        (move || {
            window
                .remove_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
                .expect("移除resize事件监听失败");
        });
    });

    let x = size.read().clone();
    x
}

/// 动态主题 Hook
pub fn use_dynamic_divider_theme(
    base_theme: DividerTheme,
    is_dark: bool,
    is_compact: bool,
) -> DividerTheme {
    let mut theme = base_theme;

    if is_dark {
        let dark_theme = DividerDarkTheme::default();
        theme = apply_dark_theme(&theme, &dark_theme);
    }

    if is_compact {
        let compact_theme = DividerCompactTheme::default();
        theme = apply_compact_theme(&theme, &compact_theme);
    }

    theme
}

/// 性能优化的 Divider Hook
pub fn use_optimized_divider(config: DividerConfig) -> (DividerConfig, String) {
    let theme = use_divider_theme();

    // 使用 memo 缓存样式计算
    let config_clone = config.clone();
    let style = use_memo(move || generate_divider_container_style(&config_clone, &theme));

    let res = style.read().clone();
    (config, res)
}

/// 可访问性增强的 Divider Hook
pub fn use_accessible_divider(config: DividerConfig) -> DividerConfig {
    let mut enhanced_config = config;

    // 根据内容自动设置 ARIA 属性
    if enhanced_config.has_text {
        // 有文本的分割线作为标题分隔符
        enhanced_config.class = Some(
            format!(
                "{} ant-divider-accessible",
                enhanced_config.class.unwrap_or_default()
            )
            .trim()
            .to_string(),
        );
    }

    enhanced_config
}

// 便捷构造函数

/// 创建水平分割线
pub fn horizontal_divider() -> Element {
    rsx! {
        Divider {
            r#type: DividerType::Horizontal,
        }
    }
}

/// 创建垂直分割线
pub fn vertical_divider() -> Element {
    rsx! {
        Divider {
            r#type: DividerType::Vertical,
        }
    }
}

/// 创建虚线分割线
pub fn dashed_divider() -> Element {
    rsx! {
        Divider {
            variant: DividerVariant::Dashed,
        }
    }
}

/// 创建点线分割线
pub fn dotted_divider() -> Element {
    rsx! {
        Divider {
            variant: DividerVariant::Dotted,
        }
    }
}

/// 创建带文本的分割线
pub fn text_divider(text: &str, orientation: Option<DividerOrientation>) -> Element {
    rsx! {
        Divider {
            orientation: orientation.unwrap_or(DividerOrientation::Center),
            "{text}"
        }
    }
}

/// 创建左对齐文本分割线
pub fn left_text_divider(text: &str) -> Element {
    text_divider(text, Some(DividerOrientation::Left))
}

/// 创建右对齐文本分割线
pub fn right_text_divider(text: &str) -> Element {
    text_divider(text, Some(DividerOrientation::Right))
}

/// 创建居中文本分割线
pub fn center_text_divider(text: &str) -> Element {
    text_divider(text, Some(DividerOrientation::Center))
}

/// 创建朴素文本分割线
pub fn plain_text_divider(text: &str) -> Element {
    rsx! {
        Divider {
            plain: true,
            "{text}"
        }
    }
}

/// 创建小尺寸分割线
pub fn small_divider() -> Element {
    rsx! {
        Divider {
            size: DividerSize::Small,
        }
    }
}

/// 创建大尺寸分割线
pub fn large_divider() -> Element {
    rsx! {
        Divider {
            size: DividerSize::Large,
        }
    }
}

/// 创建自定义样式分割线
pub fn custom_divider(
    r#type: Option<DividerType>,
    variant: Option<DividerVariant>,
    orientation: Option<DividerOrientation>,
    size: Option<DividerSize>,
    plain: Option<bool>,
    class: Option<String>,
    style: Option<String>,
) -> Element {
    rsx! {
        Divider {
            r#type: r#type.unwrap_or_default(),
            variant: variant.unwrap_or_default(),
            orientation: orientation.unwrap_or_default(),
            size: size,
            plain: plain.unwrap_or(false),
            class: class,
            style: style,
        }
    }
}

// 高阶组件

/// 带间距的分割线容器
#[component]
pub fn DividerContainer(spacing: Option<String>, children: Element) -> Element {
    let container_style = format!(
        "display: flex; flex-direction: column; gap: {}",
        spacing.unwrap_or_else(|| "16px".to_string())
    );

    rsx! {
        div {
            class: "ant-divider-container",
            style: "{container_style}",
            {children}
        }
    }
}

/// 分割线组
#[component]
pub fn DividerGroup(
    direction: Option<DividerType>,
    spacing: Option<String>,
    children: Element,
) -> Element {
    let direction = direction.unwrap_or(DividerType::Horizontal);
    let spacing = spacing.unwrap_or_else(|| "8px".to_string());

    let container_style = match direction {
        DividerType::Horizontal => {
            format!("display: flex; flex-direction: column; gap: {}", spacing)
        }
        DividerType::Vertical => format!(
            "display: flex; flex-direction: row; align-items: center; gap: {}",
            spacing
        ),
    };

    rsx! {
        div {
            class: "ant-divider-group ant-divider-group-{direction}",
            style: "{container_style}",
            {children}
        }
    }
}

/// 响应式分割线
#[component]
pub fn ResponsiveDivider(
    xs: Option<DividerProps>,
    sm: Option<DividerProps>,
    md: Option<DividerProps>,
    lg: Option<DividerProps>,
    xl: Option<DividerProps>,
) -> Element {
    let window_size = use_window_size();

    let props = if window_size.width < 576.0 {
        xs.unwrap_or_default()
    } else if window_size.width < 768.0 {
        sm.unwrap_or_default()
    } else if window_size.width < 992.0 {
        md.unwrap_or_default()
    } else if window_size.width < 1200.0 {
        lg.unwrap_or_default()
    } else {
        xl.unwrap_or_default()
    };

    rsx! {
        Divider { ..props }
    }
}

/// 条件分割线
#[component]
pub fn ConditionalDivider(
    condition: bool,
    divider_props: Option<DividerProps>,
    fallback: Option<Element>,
) -> Element {
    if condition {
        let props = divider_props.unwrap_or_default();
        rsx! {
            Divider { ..props }
        }
    } else {
        fallback.unwrap_or_else(|| rsx! { span {} })
    }
}

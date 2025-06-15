//! Spin 组件实现

use super::types::{SpinProps, SpinSize, SpinTheme};
use super::utils::*;
use dioxus::prelude::*;
use gloo_timers::callback::Timeout;
use std::rc::Rc;

use super::utils::{
    create_spin_state, generate_cache_key, should_show_with_delay, validate_spin_props,
};
use super::{
    calculate_spin_size, generate_css_variables, generate_spin_container_styles,
    generate_spin_dot_styles, generate_spin_indicator_style, generate_spin_mask_styles,
    generate_spin_text_styles, generate_spin_theme_styles, get_spin_container_class_name,
    get_spin_content_class_name, get_spin_indicator_class, get_spin_mask_class,
    get_spin_text_class,
};

/// Spin 加载指示器组件
///
/// 用于页面和区块的加载中状态。
///
/// # 特性
/// - 流畅的旋转动画
/// - 多种尺寸选择
/// - 延迟显示支持
/// - 自定义指示器
/// - 内容包装功能
/// - 主题支持
/// - 可访问性支持
///
/// # 示例
///
/// ```rust
/// use dioxus::prelude::*;
/// use ant_design_dioxus::{SpinSize, Spin};
///
/// fn app() -> Element {
///     rsx! {
///         // 基础用法
///         Spin { spinning: true }
///
///         // 包装内容
///         Spin {
///             spinning: true,
///             tip: "加载中...",
///             div {
///                 class: "content",
///                 "这里是内容"
///             }
///         }
///
///         // 不同尺寸
///         Spin { spinning: true, size: SpinSize::Small }
///         Spin { spinning: true, size: SpinSize::Large }
///
///         // 延迟显示
///         Spin {
///             spinning: true,
///             delay: 500, // 500ms 后显示
///         }
///     }
/// }
/// ```
#[component]
pub fn Spin(props: SpinProps) -> Element {
    // 验证 Props
    if let Err(error) = validate_spin_props(&props) {
        return rsx! {
            div {
                class: "ant-spin-error",
                "Spin 组件错误: {error}"
            }
        };
    }

    // 状态管理
    let mut spin_state = use_signal(|| create_spin_state(props.spinning, props.delay));
    let mut delay_timer = use_signal(|| None::<Timeout>);

    // 主题
    let theme = use_context::<Signal<SpinTheme>>(); //.unwrap_or(Signal::new(SpinTheme::default()));

    // 延迟显示逻辑
    use_effect(move || {
        if props.spinning {
            if let Some(delay_ms) = props.delay {
                // 设置延迟显示
                spin_state.write().visible = false;
                spin_state.write().delayed = true;

                let timer = Timeout::new(delay_ms, move || {
                    spin_state.write().visible = true;
                    spin_state.write().delayed = false;
                });

                delay_timer.set(Some(timer));
            } else {
                // 立即显示
                spin_state.write().visible = true;
                spin_state.write().delayed = false;
            }
        } else {
            // 停止旋转
            spin_state.write().visible = false;
            spin_state.write().delayed = false;

            // 清除计时器
            if let Some(_timer) = delay_timer.take() {
                // gloo_timers::callback::Timeout 会在 drop 时自动取消
            }
        }
    });

    // 生成样式
    let container_class =
        get_spin_container_class_name(spin_state.read().visible, props.class.as_deref());
    let indicator_class = get_spin_indicator_class(&props.size);
    let text_class = get_spin_text_class();
    let mask_class = get_spin_mask_class();

    let container_style = generate_spin_container_styles(&props.size, props.children.is_ok());
    let indicator_style = generate_spin_indicator_style(&theme.read(), &props.size);
    let text_style = generate_spin_text_styles();
    let mask_style = generate_spin_mask_styles();

    // CSS 变量
    let css_vars = generate_css_variables(&theme.read(), &props.size);
    let css_vars_style = css_vars
        .iter()
        .map(|(k, v)| format!("{}: {}", k, v))
        .collect::<Vec<_>>()
        .join("; ");

    // 组合最终样式
    let final_container_style = if props.style.is_some() {
        format!(
            "{};{};{}",
            container_style,
            css_vars_style,
            props.style.as_ref().unwrap()
        )
    } else {
        format!("{};{}", container_style, css_vars_style)
    };

    // 组合类名
    let final_container_class = if let Some(ref class) = props.class {
        format!("{} {}", container_class, class)
    } else {
        container_class
    };

    // 渲染指示器
    let render_indicator = || {
        if let Some(custom_indicator) = props.indicator {
            // 自定义指示器
            rsx! {
                div {
                    class: "{indicator_class} ant-spin-custom",
                    style: "{indicator_style}",
                    {custom_indicator}
                }
            }
        } else {
            // 默认指示器
            rsx! {
                div {
                    class: "{indicator_class}",
                    style: "{indicator_style}",
                    "aria-label": "loading",

                    // 旋转圆点
                    div {
                        class: "ant-spin-dot",
                        div { class: "ant-spin-dot-item" }
                        div { class: "ant-spin-dot-item" }
                        div { class: "ant-spin-dot-item" }
                        div { class: "ant-spin-dot-item" }
                    }
                }
            }
        }
    };

    // 检查是否有子元素
    let has_children = props.children.is_ok();

    // 渲染提示文本
    let render_tip = || {
        if should_show_tip(&props.tip, has_children) {
            if let Some(formatted_tip) = format_tip_text(&props.tip) {
                rsx! {
                    div {
                        class: "{text_class}",
                        style: "{text_style}",
                        "{formatted_tip}"
                    }
                }
            } else {
                rsx! { div {} }
            }
        } else {
            rsx! { div {} }
        }
    };

    // 如果有子元素，渲染包装模式
    if let Ok(children) = props.children {
        let wrapper_class = if let Some(ref wrapper_class_name) = props.wrapper_class_name {
            format!("ant-spin-container {}", wrapper_class_name)
        } else {
            "ant-spin-container".to_string()
        };

        let aria_label = if props.tip.is_some() {
            props.tip.as_ref().unwrap().clone()
        } else {
            "loading".to_string()
        };

        rsx! {
            div {
                class: "{wrapper_class}",
                "data-spinning": "{spin_state.read().visible}",

                // 内容区域
                div {
                    class: if spin_state.read().visible { "ant-spin-blur" } else { "" },
                    {children}
                }

                // 加载遮罩
                if spin_state.read().visible {
                    div {
                        class: "{mask_class}",
                        style: "{mask_style}",
                        "aria-hidden": "true",

                        div {
                            class: "{final_container_class}",
                            style: "{final_container_style}",
                            role: "status",
                            "aria-live": "polite",
                            "aria-label": "{aria_label}",

                            {render_indicator()}
                            {render_tip()}
                        }
                    }
                }
            }
        }
    } else {
        let arial_label2 = if props.tip.is_some() {
            props.tip.as_ref().unwrap().clone()
        } else {
            "loading".to_string()
        };

        // 独立模式
        if spin_state.read().visible {
            rsx! {
                div {
                    class: "{final_container_class}",
                    style: "{final_container_style}",
                    role: "status",
                    "aria-live": "polite",
                    "aria-label": "{arial_label2}",

                    {render_indicator()}
                    {render_tip()}
                }
            }
        } else {
            rsx! { div {} }
        }
    }
}

// ============================================================================
// 便捷构造函数
// ============================================================================

/// 创建小尺寸 Spin
pub fn spin_small() -> SpinProps {
    SpinProps {
        size: SpinSize::Small,
        spinning: true,
        ..Default::default()
    }
}

/// 创建大尺寸 Spin
pub fn spin_large() -> SpinProps {
    SpinProps {
        size: SpinSize::Large,
        spinning: true,
        ..Default::default()
    }
}

/// 创建带延迟的 Spin
pub fn spin_with_delay(delay_ms: u32) -> SpinProps {
    SpinProps {
        spinning: true,
        delay: Some(delay_ms),
        ..Default::default()
    }
}

/// 创建带提示的 Spin
pub fn spin_with_tip(tip: impl Into<String>) -> SpinProps {
    SpinProps {
        spinning: true,
        tip: Some(tip.into()),
        ..Default::default()
    }
}

/// 创建自定义指示器的 Spin
pub fn spin_with_indicator(indicator: Element) -> SpinProps {
    SpinProps {
        spinning: true,
        indicator: Some(Ok(indicator.into())),
        ..Default::default()
    }
}

/// 创建包装内容的 Spin
pub fn spin_wrapper(spinning: bool) -> SpinProps {
    SpinProps {
        spinning,
        ..Default::default()
    }
}

// ============================================================================
// 高阶组件
// ============================================================================

/// Spin 包装器组件
///
/// 用于包装其他组件并提供加载状态
#[component]
pub fn SpinWrapper(
    spinning: bool,
    #[props(optional)] tip: Option<String>,
    #[props(optional)] delay: Option<u32>,
    #[props(optional)] size: Option<SpinSize>,
    #[props(optional)] class: Option<String>,
    #[props(optional)] style: Option<String>,
    children: Element,
) -> Element {
    rsx! {
        Spin {
            spinning,
            tip,
            delay,
            size: size.unwrap_or(SpinSize::Default),
            class,
            style,
            children
        }
    }
}

/// 全屏加载组件
#[component]
pub fn SpinFullscreen(
    spinning: bool,
    #[props(optional)] tip: Option<String>,
    #[props(optional)] size: Option<SpinSize>,
) -> Element {
    if spinning {
        rsx! {
            div {
                class: "ant-spin-fullscreen",
                style: "
                    position: fixed;
                    top: 0;
                    left: 0;
                    width: 100%;
                    height: 100%;
                    background: rgba(255, 255, 255, 0.8);
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    z-index: 9999;
                ",

                Spin {
                    spinning: true,
                    tip,
                    size: size.unwrap_or(SpinSize::Large),
                }
            }
        }
    } else {
        rsx! { div {} }
    }
}

/// 页面加载组件
#[component]
pub fn SpinPage(
    spinning: bool,
    #[props(optional)] tip: Option<String>,
    #[props(optional)] min_height: Option<String>,
    children: Element,
) -> Element {
    let min_height = min_height.unwrap_or_else(|| "200px".to_string());

    rsx! {
        div {
            class: "ant-spin-page",
            style: "min-height: {min_height}; position: relative;",

            SpinWrapper {
                spinning,
                tip,
                size: SpinSize::Large,
                children
            }
        }
    }
}

// ============================================================================
// 辅助函数
// ============================================================================

/// 设置全局默认指示器
pub fn set_default_indicator(indicator: fn() -> Element) {
    super::types::set_default_indicator(indicator);
}

/// 获取全局默认指示器
pub fn get_default_indicator() -> Option<fn() -> Element> {
    super::types::get_default_indicator()
}

/// 创建主题提供者
#[component]
pub fn SpinThemeProvider(theme: SpinTheme, children: Element) -> Element {
    use_context_provider(|| Signal::new(theme));

    rsx! {
        {children}
    }
}

/// 使用 Spin 主题
pub fn use_spin_theme() -> Signal<SpinTheme> {
    use_context::<Signal<SpinTheme>>()
}

/// 创建加载状态 Hook
pub fn use_loading(initial: bool) -> (Signal<bool>, Box<dyn FnMut(bool)>) {
    let mut loading = use_signal(|| initial);
    let set_loading = Box::new(move |value: bool| {
        loading.set(value);
    });

    (loading, set_loading)
}

/// 创建异步加载 Hook
pub fn use_async_loading<T, F, Fut>(
    async_fn: F,
) -> (
    Signal<bool>,
    Signal<Option<T>>,
    Signal<Option<String>>,
    Box<dyn Fn() + 'static>,
)
where
    F: Fn() -> Fut + 'static,
    Fut: std::future::Future<Output = Result<T, String>> + 'static,
    T: 'static,
{
    let loading = use_signal(|| false);
    let data = use_signal(|| None);
    let error = use_signal(|| None);

    let loading_clone = loading.clone();
    let data_clone = data.clone();
    let error_clone = error.clone();
    let async_fn = Rc::new(async_fn);

    let execute = Box::new(move || {
        let mut loading = loading_clone.clone();
        let mut data = data_clone.clone();
        let mut error = error_clone.clone();
        let async_fn = async_fn.clone();

        spawn(async move {
            loading.set(true);
            error.set(None);

            match (*async_fn)().await {
                Ok(result) => {
                    data.set(Some(result));
                }
                Err(err) => {
                    error.set(Some(err));
                }
            }

            loading.set(false);
        });
    });

    (loading, data, error, execute)
}

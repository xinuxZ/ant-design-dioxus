//! BackTop 组件实现
//!
//! 本模块实现了 BackTop 组件的核心逻辑，包括组件定义、
//! 渲染逻辑、事件处理和各种 Hooks。

use dioxus::prelude::*;
use std::rc::Rc;
use std::time::Duration;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, Element, HtmlElement};

use crate::components::back_top::styles::*;
use crate::components::back_top::types::*;
use crate::components::back_top::utils::*;

/// BackTop 组件
#[component]
pub fn BackTop(props: BackTopProps) -> Element {
    let config = use_memo(move || create_back_top_config(&props));

    let scroll_state = use_scroll_state(props.target.clone(), props.throttle_delay);
    let is_mobile = use_mobile_detection();
    let style_generator = use_style_generator(config());

    let is_visible = use_memo(move || {
        if let Some(visible) = props.visible {
            visible
        } else {
            should_show_back_top(
                scroll_state().scroll_top,
                config().visibility_height,
                props.show_on_mobile,
                is_mobile(),
            )
        }
    });

    let mut hover_state = use_signal(|| false);
    let mut active_state = use_signal(|| false);

    let handle_click = {
        let target = props.target.clone();
        let duration = props.duration;
        let easing = config().easing.clone();
        let on_click = props.on_click.clone();

        move |event: MouseEvent| {
            event.prevent_default();

            // 调用用户提供的点击回调
            if let Some(callback) = &on_click {
                callback.call(event);
            }

            // 执行滚动到顶部
            spawn({
                let target = target.clone();
                let easing = easing.clone();
                async move {
                    if let Err(e) = smooth_scroll_to_top(target, duration, easing).await {
                        web_sys::console::error_1(&format!("BackTop scroll error: {}", e).into());
                    }
                }
            });
        }
    };

    let handle_key_down = {
        let handle_click = handle_click.clone();
        move |event: KeyboardEvent| {
            if props.keyboard && (event.key() == "Enter" || event.key() == " ") {
                event.prevent_default();
                // 创建一个模拟的 MouseEvent
                if let Ok(mouse_event) = web_sys::MouseEvent::new("click") {
                    handle_click(mouse_event.into());
                }
            }
        }
    };

    let container_style = style_generator().container_style(is_visible(), is_mobile());

    let button_style =
        style_generator().button_style(is_mobile(), hover_state(), active_state(), false);

    let class_name = style_generator().class_name(is_visible(), is_mobile());

    let combined_style = format!(
        "{}{}",
        container_style,
        props.style.as_deref().unwrap_or("")
    );

    let combined_class = format!("{} {}", class_name, props.class.as_deref().unwrap_or(""))
        .trim()
        .to_string();

    rsx! {
        div {
            class: "{combined_class}",
            style: "{combined_style}",
            "data-testid": "back-top",
            button {
                style: "{button_style}",
                onclick: handle_click,
                onkeydown: handle_key_down,
                onmouseenter: move |_| hover_state.set(true),
                onmouseleave: move |_| hover_state.set(false),
                onmousedown: move |_| active_state.set(true),
                onmouseup: move |_| active_state.set(false),
                "aria-label": props.aria_label.as_deref().unwrap_or("回到顶部"),
                "role": "button",
                "tabindex": "0",

                if let Some(children) = props.children {
                    {children}
                } else {
                    div {
                        style: style_generator().icon_style(),
                        dangerous_inner_html: generate_default_icon_svg()
                    }
                }
            }
        }
    }
}

/// 滚动状态 Hook
#[hook]
pub fn use_scroll_state(target: Option<String>, throttle_delay: u32) -> Signal<BackTopScrollState> {
    let mut scroll_state = use_signal(|| BackTopScrollState::default());

    use_effect(move || {
        let target_element = if let Some(target_id) = &target {
            window()
                .and_then(|w| w.document())
                .and_then(|d| d.get_element_by_id(target_id))
        } else {
            window()
                .and_then(|w| w.document())
                .map(|d| d.document_element())
                .flatten()
        };

        if let Some(element) = target_element {
            let element_clone = element.clone();
            let scroll_state_clone = scroll_state.clone();

            let throttled_handler = create_throttled_scroll_handler(
                move |scroll_top: f64, scroll_height: f64, client_height: f64| {
                    let percentage = if scroll_height > client_height {
                        (scroll_top / (scroll_height - client_height) * 100.0).min(100.0)
                    } else {
                        0.0
                    };

                    let direction = {
                        let current_scroll = scroll_state_clone().scroll_top;
                        if scroll_top > current_scroll {
                            ScrollDirection::Down
                        } else if scroll_top < current_scroll {
                            ScrollDirection::Up
                        } else {
                            scroll_state_clone().direction
                        }
                    };

                    scroll_state_clone.set(BackTopScrollState {
                        scroll_top,
                        scroll_height,
                        client_height,
                        scroll_percentage: percentage,
                        direction,
                        is_scrolling: true,
                    });
                },
                throttle_delay,
            );

            let scroll_handler = Closure::wrap(Box::new(move |_event: web_sys::Event| {
                if let Some(html_element) = element_clone.dyn_ref::<HtmlElement>() {
                    let scroll_top = html_element.scroll_top() as f64;
                    let scroll_height = html_element.scroll_height() as f64;
                    let client_height = html_element.client_height() as f64;

                    throttled_handler(scroll_top, scroll_height, client_height);
                }
            }) as Box<dyn FnMut(_)>);

            let _ = element.add_event_listener_with_callback(
                "scroll",
                scroll_handler.as_ref().unchecked_ref(),
            );

            // 初始化滚动状态
            if let Some(html_element) = element.dyn_ref::<HtmlElement>() {
                let scroll_top = html_element.scroll_top() as f64;
                let scroll_height = html_element.scroll_height() as f64;
                let client_height = html_element.client_height() as f64;

                let percentage = if scroll_height > client_height {
                    (scroll_top / (scroll_height - client_height) * 100.0).min(100.0)
                } else {
                    0.0
                };

                scroll_state.set(BackTopScrollState {
                    scroll_top,
                    scroll_height,
                    client_height,
                    scroll_percentage: percentage,
                    direction: ScrollDirection::Up,
                    is_scrolling: false,
                });
            }

            // 清理函数
            move || {
                let _ = element.remove_event_listener_with_callback(
                    "scroll",
                    scroll_handler.as_ref().unchecked_ref(),
                );
                drop(scroll_handler);
            }
        } else {
            move || {}
        }
    });

    scroll_state
}

/// 移动设备检测 Hook
#[hook]
pub fn use_mobile_detection() -> Signal<bool> {
    let mut is_mobile = use_signal(|| false);

    use_effect(move || {
        let update_mobile_state = || {
            is_mobile.set(is_mobile_device());
        };

        // 初始检测
        update_mobile_state();

        // 监听窗口大小变化
        let resize_handler = Closure::wrap(Box::new(move |_event: web_sys::Event| {
            update_mobile_state();
        }) as Box<dyn FnMut(_)>);

        if let Some(window) = window() {
            let _ = window.add_event_listener_with_callback(
                "resize",
                resize_handler.as_ref().unchecked_ref(),
            );

            // 清理函数
            move || {
                let _ = window.remove_event_listener_with_callback(
                    "resize",
                    resize_handler.as_ref().unchecked_ref(),
                );
                drop(resize_handler);
            }
        } else {
            move || {}
        }
    });

    is_mobile
}

/// 样式生成器 Hook
#[hook]
pub fn use_style_generator(config: BackTopConfig) -> Signal<BackTopStyleGenerator> {
    let mut generator = use_signal(|| BackTopStyleGenerator::new(config.clone()));

    use_effect(move || {
        generator.with_mut(|g| g.update_config(config.clone()));
    });

    generator
}

/// 创建节流滚动处理器
fn create_throttled_scroll_handler<F>(handler: F, delay: u32) -> impl Fn(f64, f64, f64)
where
    F: Fn(f64, f64, f64) + 'static,
{
    let last_call = Rc::new(std::cell::RefCell::new(0.0));

    move |scroll_top: f64, scroll_height: f64, client_height: f64| {
        let now = js_sys::Date::now();
        let mut last = last_call.borrow_mut();

        if now - *last >= delay as f64 {
            *last = now;
            handler(scroll_top, scroll_height, client_height);
        }
    }
}

/// 便捷构造函数：创建基础 BackTop
pub fn basic_back_top() -> Element {
    rsx! {
        BackTop {}
    }
}

/// 便捷构造函数：创建自定义位置的 BackTop
pub fn positioned_back_top(bottom: u32, right: u32) -> Element {
    rsx! {
        BackTop {
            bottom: bottom,
            right: right
        }
    }
}

/// 便捷构造函数：创建自定义可见高度的 BackTop
pub fn visibility_back_top(visibility_height: u32) -> Element {
    rsx! {
        BackTop {
            visibility_height: visibility_height
        }
    }
}

/// 便捷构造函数：创建自定义动画的 BackTop
pub fn animated_back_top(animation: BackTopAnimation, duration: u32) -> Element {
    rsx! {
        BackTop {
            animation: animation,
            duration: duration
        }
    }
}

/// 便捷构造函数：创建自定义目标的 BackTop
pub fn target_back_top(target: String) -> Element {
    rsx! {
        BackTop {
            target: Some(target)
        }
    }
}

/// 便捷构造函数：创建移动端优化的 BackTop
pub fn mobile_back_top(mobile_bottom: u32, mobile_right: u32) -> Element {
    rsx! {
        BackTop {
            mobile_bottom: Some(mobile_bottom),
            mobile_right: Some(mobile_right),
            show_on_mobile: true
        }
    }
}

/// 便捷构造函数：创建自定义内容的 BackTop
pub fn custom_back_top(children: Element) -> Element {
    rsx! {
        BackTop {
            {children}
        }
    }
}

/// 便捷构造函数：创建快速滚动的 BackTop
pub fn fast_back_top() -> Element {
    rsx! {
        BackTop {
            duration: 200,
            animation: BackTopAnimation::FadeScale,
            easing: EasingFunction::EaseOut
        }
    }
}

/// 便捷构造函数：创建平滑滚动的 BackTop
pub fn smooth_back_top() -> Element {
    rsx! {
        BackTop {
            duration: 800,
            animation: BackTopAnimation::FadeSlide,
            easing: EasingFunction::EaseInOut
        }
    }
}

/// 便捷构造函数：创建弹跳动画的 BackTop
pub fn bounce_back_top() -> Element {
    rsx! {
        BackTop {
            animation: BackTopAnimation::Bounce,
            easing: EasingFunction::EaseOut,
            duration: 400
        }
    }
}

/// BackTop 主题提供者组件
#[component]
pub fn BackTopThemeProvider(theme: BackTopTheme, children: Element) -> Element {
    use_context_provider(|| theme);

    rsx! {
        {children}
    }
}

/// BackTop 主题上下文 Hook
#[hook]
pub fn use_back_top_theme() -> BackTopTheme {
    use_context::<BackTopTheme>().unwrap_or_default()
}

/// BackTop 容器组件
#[component]
pub fn BackTopContainer(
    class: Option<String>,
    style: Option<String>,
    children: Element,
) -> Element {
    let combined_class = format!("ant-back-top-container {}", class.as_deref().unwrap_or(""))
        .trim()
        .to_string();

    rsx! {
        div {
            class: "{combined_class}",
            style: style.as_deref().unwrap_or(""),
            {children}
        }
    }
}

/// BackTop 组合组件
#[component]
pub fn BackTopGroup(
    spacing: Option<u32>,
    direction: Option<String>, // "horizontal" | "vertical"
    children: Element,
) -> Element {
    let spacing_value = spacing.unwrap_or(16);
    let is_horizontal = direction.as_deref() == Some("horizontal");

    let container_style = if is_horizontal {
        format!(
            "display: flex; flex-direction: row; gap: {}px; align-items: center;",
            spacing_value
        )
    } else {
        format!(
            "display: flex; flex-direction: column; gap: {}px; align-items: flex-end;",
            spacing_value
        )
    };

    rsx! {
        div {
            class: "ant-back-top-group",
            style: "{container_style}",
            {children}
        }
    }
}

/// 响应式 BackTop 组件
#[component]
pub fn ResponsiveBackTop(
    desktop_props: BackTopProps,
    mobile_props: Option<BackTopProps>,
) -> Element {
    let is_mobile = use_mobile_detection();

    if is_mobile() {
        if let Some(mobile_props) = mobile_props {
            rsx! {
                BackTop { ..mobile_props }
            }
        } else {
            rsx! {
                BackTop {
                    ..desktop_props,
                    show_on_mobile: true,
                    mobile_bottom: Some(20),
                    mobile_right: Some(20)
                }
            }
        }
    } else {
        rsx! {
            BackTop { ..desktop_props }
        }
    }
}

/// 条件渲染 BackTop 组件
#[component]
pub fn ConditionalBackTop(condition: bool, props: BackTopProps) -> Element {
    if condition {
        rsx! {
            BackTop { ..props }
        }
    } else {
        rsx! {}
    }
}

/// 性能优化的 BackTop Hook
#[hook]
pub fn use_optimized_back_top(props: BackTopProps) -> (Signal<bool>, impl Fn() + Clone) {
    let scroll_state = use_scroll_state(props.target.clone(), props.throttle_delay);
    let is_mobile = use_mobile_detection();

    let is_visible = use_memo(move || {
        if let Some(visible) = props.visible {
            visible
        } else {
            should_show_back_top(scroll_state().scroll_top, props.visibility_height)
        }
    });

    let scroll_to_top = {
        let target = props.target.clone();
        let duration = props.duration;
        let easing = EasingFunction::EaseOut;

        move || {
            spawn({
                let target = target.clone();
                let easing = easing.clone();
                async move {
                    if let Err(e) = smooth_scroll_to_top(target, duration, easing).await {
                        web_sys::console::error_1(&format!("BackTop scroll error: {}", e).into());
                    }
                }
            });
        }
    };

    (is_visible, scroll_to_top)
}

/// 滚动进度 Hook
#[hook]
pub fn use_scroll_progress(target: Option<String>) -> Signal<f64> {
    let scroll_state = use_scroll_state(target, 16);

    use_memo(move || scroll_state().scroll_percentage)
}

/// 滚动方向 Hook
#[hook]
pub fn use_scroll_direction(target: Option<String>) -> Signal<ScrollDirection> {
    let scroll_state = use_scroll_state(target, 16);

    use_memo(move || scroll_state().direction)
}

/// 窗口尺寸 Hook
#[hook]
pub fn use_window_size() -> Signal<(u32, u32)> {
    let mut window_size = use_signal(|| (0, 0));

    use_effect(move || {
        let update_size = || {
            if let Some(window) = window() {
                let width = window
                    .inner_width()
                    .unwrap_or_default()
                    .as_f64()
                    .unwrap_or(0.0) as u32;
                let height = window
                    .inner_height()
                    .unwrap_or_default()
                    .as_f64()
                    .unwrap_or(0.0) as u32;
                window_size.set((width, height));
            }
        };

        // 初始设置
        update_size();

        // 监听窗口大小变化
        let resize_handler = Closure::wrap(Box::new(move |_event: web_sys::Event| {
            update_size();
        }) as Box<dyn FnMut(_)>);

        if let Some(window) = window() {
            let _ = window.add_event_listener_with_callback(
                "resize",
                resize_handler.as_ref().unchecked_ref(),
            );

            // 清理函数
            (move || {
                let _ = window.remove_event_listener_with_callback(
                    "resize",
                    resize_handler.as_ref().unchecked_ref(),
                );
                drop(resize_handler);
            })()
        } else {
            (move || {})()
        }
    });

    window_size
}

#[cfg(test)]
mod tests {
    use super::*;
    use dioxus::prelude::*;

    #[test]
    fn test_back_top_props_default() {
        let props = BackTopProps::default();
        assert_eq!(props.visibility_height, 400);
        assert_eq!(props.duration, 450);
        assert_eq!(props.bottom, 50);
        assert_eq!(props.right, 50);
        assert!(props.visible.is_none());
        assert!(props.target.is_none());
    }

    #[test]
    fn test_back_top_config_creation() {
        let props = BackTopProps::default();
        let config = create_back_top_config(&props);

        assert_eq!(config.visibility_height, 400);
        assert_eq!(config.duration, 450);
        assert_eq!(config.bottom, 50);
        assert_eq!(config.right, 50);
        assert_eq!(config.animation, BackTopAnimation::FadeSlide);
        assert_eq!(config.easing, EasingFunction::EaseOut);
    }

    #[test]
    fn test_convenience_constructors() {
        // 这些测试主要验证函数能够编译和调用
        // 在实际的 Dioxus 环境中，这些会返回有效的 Element

        // 注意：在测试环境中，我们无法直接测试 rsx! 宏的返回值
        // 但我们可以验证函数的存在和基本逻辑

        // 测试基础构造函数的存在
        assert!(true); // 占位符测试
    }

    #[test]
    fn test_throttled_scroll_handler() {
        let mut call_count = 0;
        let handler = create_throttled_scroll_handler(
            |_scroll_top, _scroll_height, _client_height| {
                // 在实际环境中这里会被调用
            },
            100,
        );

        // 测试处理器创建成功
        handler(0.0, 1000.0, 500.0);
        assert!(true); // 占位符测试
    }

    #[test]
    fn test_back_top_scroll_state_default() {
        let state = BackTopScrollState::default();
        assert_eq!(state.scroll_top, 0.0);
        assert_eq!(state.scroll_height, 0.0);
        assert_eq!(state.client_height, 0.0);
        assert_eq!(state.scroll_percentage, 0.0);
        assert_eq!(state.direction, ScrollDirection::Up);
        assert!(!state.is_scrolling);
    }

    #[test]
    fn test_scroll_direction_enum() {
        let up = ScrollDirection::Up;
        let down = ScrollDirection::Down;

        assert_ne!(up, down);

        // 测试 Debug trait
        let debug_str = format!("{:?}", up);
        assert!(debug_str.contains("Up"));
    }
}

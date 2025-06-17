//! Tooltip 组件核心实现

use dioxus::prelude::*;
use std::rc::Rc;
use web_sys::{window, Element, HtmlElement};
use wasm_bindgen::JsCast;

use crate::components::tooltip::types::*;
use crate::components::tooltip::styles::*;
use crate::components::tooltip::utils::*;
use crate::config_provider::hooks::use_component_config;
use crate::theme::use_theme;

/// Tooltip 组件
#[component]
pub fn Tooltip(props: TooltipProps) -> Element {
    // 获取全局配置
    let component_config = use_component_config();
    let read_config = component_config.read();
    let tooltip_config = read_config.as_ref().and_then(|config| config.get_tooltip_config());

    // 应用全局配置
    let props = TooltipProps {
        title: props.title.or_else(|| {
            tooltip_config.and_then(|c| c.title.clone())
        }),
        placement: props.placement.or_else(|| {
            tooltip_config.and_then(|c| c.placement.clone())
        }),
        trigger: props.trigger.or_else(|| {
            tooltip_config.and_then(|c| c.trigger.clone())
        }),
        color: props.color.or_else(|| {
            tooltip_config.and_then(|c| c.color.clone())
        }),
        mouse_enter_delay: props.mouse_enter_delay.or_else(|| {
            tooltip_config.and_then(|c| c.mouse_enter_delay)
        }),
        mouse_leave_delay: props.mouse_leave_delay.or_else(|| {
            tooltip_config.and_then(|c| c.mouse_leave_delay)
        }),
        overlay_class_name: props.overlay_class_name.or_else(|| {
            tooltip_config.and_then(|c| c.overlay_class_name.clone())
        }),
        overlay_style: props.overlay_style.or_else(|| {
            tooltip_config.and_then(|c| {
                c.overlay_style.clone().map(|s| {
                    s.iter()
                        .map(|(k, v)| format!("{}: {}", k, v))
                        .collect::<Vec<_>>()
                        .join("; ")
                })
            })
        }),
        ..props
    };

    let theme = use_theme();
    
    // 状态管理
    let mut is_open = use_signal(|| props.default_open);
    let mut tooltip_position = use_signal(|| None::<TooltipPosition>);
    let mut tooltip_state = use_signal(|| TooltipState::Hidden);
    let mut delay_executor = use_signal(|| DelayExecutor::new());
    
    // 元素引用
    let trigger_ref = use_signal(|| None::<Rc<MountedData>>);
    let tooltip_ref = use_signal(|| None::<Rc<MountedData>>);
    
    // 受控模式处理
    let current_open = props.open.unwrap_or_else(|| is_open.read().clone());
    
    // 样式生成
    let base_style = tooltip_base_style(&theme);
    let placement_style = tooltip_placement_style(&props.placement, &theme);
    let animation_style = tooltip_animation_style(&theme);
    let responsive_style = tooltip_responsive_style(&theme);
    
    // 自定义颜色样式
    let color_style = props.color.as_ref().map(|color| tooltip_color_style(color));
    
    // 显示/隐藏逻辑
    let show_tooltip = move || {
        if props.title.is_none() || props.title.as_ref().unwrap().is_empty() {
            return;
        }
        
        tooltip_state.set(TooltipState::Showing);
        
        // 计算位置
        if let Some(trigger_element) = trigger_ref.read().as_ref() {
            if let Ok(element) = trigger_element.downcast::<Element>() {
                if let Ok(target_bounds) = get_element_bounds(&element) {
                    let tooltip_size = estimate_tooltip_size(
                        props.title.as_ref().unwrap(),
                        Some(300.0),
                    );
                    
                    if let Ok(position) = calculate_tooltip_position(
                        &target_bounds,
                        tooltip_size,
                        &props.placement,
                        props.auto_adjust_overflow,
                    ) {
                        tooltip_position.set(Some(position));
                    }
                }
            }
        }
        
        if props.open.is_none() {
            is_open.set(true);
        }
        
        tooltip_state.set(TooltipState::Visible);
        
        // 触发回调
        if let Some(on_open_change) = &props.on_open_change {
            on_open_change.call(true);
        }
    };
    
    let hide_tooltip = move || {
        tooltip_state.set(TooltipState::Hiding);
        
        if props.open.is_none() {
            is_open.set(false);
        }
        
        // 延迟隐藏以允许动画完成
        let mut executor = delay_executor.write();
        let _ = executor.execute_after_delay(150.0, move || {
            tooltip_state.set(TooltipState::Hidden);
        });
        
        // 触发回调
        if let Some(on_open_change) = &props.on_open_change {
            on_open_change.call(false);
        }
    };
    
    // 延迟显示
    let show_with_delay = move || {
        let mut executor = delay_executor.write();
        let delay_ms = props.mouse_enter_delay * 1000.0;
        let _ = executor.execute_after_delay(delay_ms, move || {
            show_tooltip();
        });
    };
    
    // 延迟隐藏
    let hide_with_delay = move || {
        let mut executor = delay_executor.write();
        let delay_ms = props.mouse_leave_delay * 1000.0;
        let _ = executor.execute_after_delay(delay_ms, move || {
            hide_tooltip();
        });
    };
    
    // 取消延迟
    let cancel_delay = move || {
        delay_executor.write().clear();
    };
    
    // 事件处理器
    let handle_mouse_enter = move |_| {
        if matches!(props.trigger, TooltipTrigger::Hover) {
            cancel_delay();
            show_with_delay();
        }
    };
    
    let handle_mouse_leave = move |_| {
        if matches!(props.trigger, TooltipTrigger::Hover) {
            cancel_delay();
            hide_with_delay();
        }
    };
    
    let handle_focus = move |_| {
        if matches!(props.trigger, TooltipTrigger::Focus) {
            show_tooltip();
        }
    };
    
    let handle_blur = move |_| {
        if matches!(props.trigger, TooltipTrigger::Focus) {
            hide_tooltip();
        }
    };
    
    let handle_click = move |_| {
        if matches!(props.trigger, TooltipTrigger::Click) {
            if current_open {
                hide_tooltip();
            } else {
                show_tooltip();
            }
        }
    };
    
    let handle_context_menu = move |evt: Event<MouseData>| {
        if matches!(props.trigger, TooltipTrigger::ContextMenu) {
            evt.prevent_default();
            show_tooltip();
        }
    };
    
    // 键盘事件处理
    let handle_key_down = move |evt: Event<KeyboardData>| {
        match evt.key().as_str() {
            "Escape" => {
                if current_open {
                    hide_tooltip();
                }
            }
            "Enter" | " " => {
                if matches!(props.trigger, TooltipTrigger::Focus | TooltipTrigger::Click) {
                    if current_open {
                        hide_tooltip();
                    } else {
                        show_tooltip();
                    }
                }
            }
            _ => {}
        }
    };
    
    // Tooltip 鼠标事件（防止意外隐藏）
    let handle_tooltip_mouse_enter = move |_| {
        if matches!(props.trigger, TooltipTrigger::Hover) {
            cancel_delay();
        }
    };
    
    let handle_tooltip_mouse_leave = move |_| {
        if matches!(props.trigger, TooltipTrigger::Hover) {
            hide_with_delay();
        }
    };
    
    // 监听受控状态变化
    use_effect(move || {
        if let Some(open) = props.open {
            if open != is_open.read().clone() {
                if open {
                    show_tooltip();
                } else {
                    hide_tooltip();
                }
            }
        }
    });
    
    // 监听滚动事件以更新位置
    use_effect(move || {
        if current_open {
            // 这里可以添加滚动监听逻辑
            // 由于 Dioxus 的限制，暂时简化处理
        }
    });
    
    // 构建 CSS 类名
    let mut tooltip_classes = vec!["ant-tooltip"];
    if current_open {
        tooltip_classes.push("ant-tooltip-open");
    }
    tooltip_classes.push(props.placement.to_class_name());
    
    if props.color.is_some() {
        tooltip_classes.push("ant-tooltip-custom-color");
    }
    
    match tooltip_state.read().clone() {
        TooltipState::Showing => tooltip_classes.push("ant-tooltip-zoom-enter"),
        TooltipState::Hiding => tooltip_classes.push("ant-tooltip-zoom-leave"),
        _ => {}
    }
    
    if let Some(class) = &props.class {
        tooltip_classes.push(class);
    }
    
    // 构建 Tooltip 样式
    let tooltip_style = if let Some(position) = tooltip_position.read().as_ref() {
        let mut style_parts = vec![
            format!("left: {}px", position.x),
            format!("top: {}px", position.y),
        ];
        
        if let Some(z_index) = props.z_index {
            style_parts.push(format!("z-index: {}", z_index));
        }
        
        if let Some(style) = &props.style {
            style_parts.push(style.clone());
        }
        
        Some(style_parts.join("; "))
    } else {
        props.style.clone()
    };
    
    rsx! {
        // 注入样式
        style { {base_style.to_string()} }
        style { {placement_style.to_string()} }
        style { {animation_style.to_string()} }
        style { {responsive_style.to_string()} }
        if let Some(color_style) = color_style {
            style { {color_style.to_string()} }
        }
        
        // 触发元素包装器
        span {
            class: "ant-tooltip-trigger",
            onmounted: move |evt| {
                trigger_ref.set(Some(evt.data()));
            },
            onmouseenter: handle_mouse_enter,
            onmouseleave: handle_mouse_leave,
            onfocus: handle_focus,
            onblur: handle_blur,
            onclick: handle_click,
            oncontextmenu: handle_context_menu,
            onkeydown: handle_key_down,
            tabindex: if matches!(props.trigger, TooltipTrigger::Focus | TooltipTrigger::Click) {
                "0"
            } else {
                "-1"
            },
            
            {props.children}
        }
        
        // Tooltip 内容
        if current_open && !matches!(tooltip_state.read().clone(), TooltipState::Hidden) {
            if let Some(title) = &props.title {
                if !title.is_empty() {
                    div {
                        class: tooltip_classes.join(" "),
                        style: tooltip_style.unwrap_or_default(),
                        onmounted: move |evt| {
                            tooltip_ref.set(Some(evt.data()));
                        },
                        onmouseenter: handle_tooltip_mouse_enter,
                        onmouseleave: handle_tooltip_mouse_leave,
                        role: "tooltip",
                        "aria-hidden": if current_open { "false" } else { "true" },
                        
                        div {
                            class: "ant-tooltip-content",
                            
                            if props.arrow {
                                div {
                                    class: "ant-tooltip-arrow",
                                }
                            }
                            
                            div {
                                class: "ant-tooltip-inner",
                                {title}
                            }
                        }
                    }
                }
            }
        }
    }
}

// 导出便捷的创建函数

/// 创建简单的 Tooltip
pub fn simple_tooltip(title: &str, children: Element) -> Element {
    rsx! {
        Tooltip {
            title: title.to_string(),
            {children}
        }
    }
}

/// 创建带位置的 Tooltip
pub fn positioned_tooltip(
    title: &str,
    placement: TooltipPlacement,
    children: Element,
) -> Element {
    rsx! {
        Tooltip {
            title: title.to_string(),
            placement,
            {children}
        }
    }
}

/// 创建自定义颜色的 Tooltip
pub fn colored_tooltip(
    title: &str,
    color: &str,
    children: Element,
) -> Element {
    rsx! {
        Tooltip {
            title: title.to_string(),
            color: color.to_string(),
            {children}
        }
    }
}

/// 创建受控的 Tooltip
pub fn controlled_tooltip(
    title: &str,
    open: bool,
    on_open_change: EventHandler<bool>,
    children: Element,
) -> Element {
    rsx! {
        Tooltip {
            title: title.to_string(),
            open,
            on_open_change,
            {children}
        }
    }
}
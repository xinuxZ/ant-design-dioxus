//! Spin 组件实现

use dioxus::prelude::*;
use std::time::Duration;
use gloo_timers::future::TimeoutFuture;
use super::types::*;
use super::styles::*;

/// 默认的加载指示器组件
///
/// 渲染一个旋转的圆形指示器
#[component]
fn DefaultIndicator(size: SpinSize) -> Element {
    let indicator_size = size.indicator_size();

    rsx! {
        span {
            class: "ant-spin-dot ant-spin-dot-spin",
            style: format!("width: {}px; height: {}px;", indicator_size, indicator_size),
            i { class: "ant-spin-dot-item" }
            i { class: "ant-spin-dot-item" }
            i { class: "ant-spin-dot-item" }
            i { class: "ant-spin-dot-item" }
        }
    }
}

/// 获取无障碍属性
fn get_accessibility_attrs(props: &SpinProps) -> AccessibilityAttrs {
    AccessibilityAttrs {
        role: Some("status".to_string()),
        aria_label: props.aria_label.clone().or_else(|| Some("加载中".to_string())),
        aria_describedby: props.aria_describedby.clone(),
        aria_live: Some("polite".to_string()),
        aria_busy: Some(props.spinning.to_string()),
    }
}

/// 获取进度文本
fn get_progress_text(props: &SpinProps) -> Option<String> {
    if let Some(percent) = props.percent {
        Some(format!("{}%", percent.round() as i32))
    } else {
        None
    }
}

/// 无障碍属性结构体
#[derive(Debug, Clone, PartialEq)]
struct AccessibilityAttrs {
    role: Option<String>,
    aria_label: Option<String>,
    aria_describedby: Option<String>,
    aria_live: Option<String>,
    aria_busy: Option<String>,
}

/// Spin 加载中组件
///
/// 用于页面和区块的加载中状态，提供了一个简单的加载动画
#[component]
pub fn Spin(props: SpinProps) -> Element {
    // 暂时移除主题依赖，使用默认值
    // let theme = use_theme();
    // let config = use_config_provider();
    let mut is_visible = use_signal(|| props.spinning);

    // 处理延迟显示逻辑
    use_effect(move || {
        if props.delay > 0 && props.spinning {
            is_visible.set(false);
            let delay = props.delay;
            spawn(async move {
                gloo_timers::future::TimeoutFuture::new(delay).await;
                is_visible.set(true);
            });
        } else {
            is_visible.set(props.spinning);
        }
    });

    // 生成样式
    let spin_class = use_memo(move || {
        generate_spin_styles()
    });

    let spin_class = use_memo(move || {
        let mut classes = vec!["ant-spin"];

        if is_visible() {
            classes.push("ant-spin-spinning");
        }

        if props.fullscreen {
            classes.push("ant-spin-fullscreen");
        }

        let size_class = props.size.to_class();
        if !size_class.is_empty() {
            classes.push(size_class);
        }

        if props.tip.is_some() || props.percent.is_some() {
            classes.push("ant-spin-show-text");
        }

        // 暂时移除RTL支持，后续可以重新添加
        // if config.direction == crate::config_provider::Direction::Rtl {
        //     classes.push("ant-spin-rtl");
        // }

        if let Some(ref class) = props.class {
            classes.push(class);
        }

        classes.join(" ")
    });

    // 获取无障碍属性
    let accessibility_attrs = get_accessibility_attrs(&props);

    // 渲染逻辑
    if props.fullscreen {
        // 全屏模式
        rsx! {
            div {
                class: "{spin_class()}",
                style: props.style.clone().unwrap_or_default(),
                role: accessibility_attrs.role.unwrap_or_default(),
                "aria-label": accessibility_attrs.aria_label.unwrap_or_default(),
                "aria-describedby": accessibility_attrs.aria_describedby.unwrap_or_default(),
                "aria-live": accessibility_attrs.aria_live.unwrap_or_default(),
                "aria-busy": accessibility_attrs.aria_busy.unwrap_or_default(),
                
                div {
                    class: "ant-spin-container",
                    
                    if let Some(indicator) = &props.indicator {
                        {indicator.clone()}
                    } else {
                        DefaultIndicator { size: props.size.unwrap_or(SpinSize::Default) }
                    }
                    
                    if let Some(tip) = &props.tip {
                        div {
                            class: "ant-spin-text",
                            {tip.clone()}
                        }
                    }
                    
                    if let Some(progress_text) = get_progress_text(&props) {
                        div {
                            class: "ant-spin-progress",
                            {progress_text}
                        }
                    }
                }
            }
        }
    } else if props.children.is_some() {
        // 包裹内容模式
        rsx! {
            div {
                class: "{spin_class()} ant-spin-container",
                style: props.style.clone().unwrap_or_default(),
                
                div {
                     class: if is_visible() { "ant-spin-blur" } else { "" },
                     {props.children.clone()}
                }
                
                if is_visible() {
                    div {
                        class: "ant-spin-spinning",
                        role: accessibility_attrs.role.unwrap_or_default(),
                        "aria-label": accessibility_attrs.aria_label.unwrap_or_default(),
                        "aria-describedby": accessibility_attrs.aria_describedby.unwrap_or_default(),
                        "aria-live": accessibility_attrs.aria_live.unwrap_or_default(),
                        "aria-busy": accessibility_attrs.aria_busy.unwrap_or_default(),
                        
                        if let Some(indicator) = &props.indicator {
                            {indicator.clone()}
                        } else {
                            DefaultIndicator { size: props.size.unwrap_or(SpinSize::Default) }
                        }
                        
                        if let Some(tip) = &props.tip {
                            div {
                                class: "ant-spin-text",
                                {tip.clone()}
                            }
                        }
                        
                        if let Some(progress_text) = get_progress_text(&props) {
                            div {
                                class: "ant-spin-progress",
                                {progress_text}
                            }
                        }
                    }
                }
            }
        }
    } else {
        // 独立加载指示器模式
         if is_visible() {
            rsx! {
                div {
                    class: "{spin_class()}",
                    style: props.style.clone().unwrap_or_default(),
                    role: accessibility_attrs.role.unwrap_or_default(),
                    "aria-label": accessibility_attrs.aria_label.unwrap_or_default(),
                    "aria-describedby": accessibility_attrs.aria_describedby.unwrap_or_default(),
                    "aria-live": accessibility_attrs.aria_live.unwrap_or_default(),
                    "aria-busy": accessibility_attrs.aria_busy.unwrap_or_default(),
                    
                    if let Some(indicator) = &props.indicator {
                        {indicator.clone()}
                    } else {
                        DefaultIndicator { size: props.size.unwrap_or(SpinSize::Default) }
                    }
                    
                    if let Some(tip) = &props.tip {
                        div {
                            class: "ant-spin-text",
                            {tip.clone()}
                        }
                    }
                    
                    if let Some(progress_text) = get_progress_text(&props) {
                        div {
                            class: "ant-spin-progress",
                            {progress_text}
                        }
                    }
                }
            }
        } else {
            rsx! { div {} }
        }
    }
}



/// 全局设置Spin配置的Hook
pub fn use_spin_config() -> SpinConfig {
    SpinConfig::default()
}

/// 设置全局Spin配置
pub fn set_global_spin_config(config: SpinConfig) {
    // 这里可以实现全局配置的设置逻辑
    // 在实际应用中，可能需要通过Context或其他状态管理方式来实现
}
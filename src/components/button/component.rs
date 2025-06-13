//! Button 组件实现
//!
//! 提供 Button 和 ButtonGroup 组件的完整实现

use dioxus::prelude::*;
use super::types::*;
use super::styles::*;
use crate::components::icon::Icon;

/// Button 组件
/// 
/// # 示例
/// 
/// ```rust
/// use dioxus::prelude::*;
/// use ant_design_dioxus::Button;
/// 
/// fn app() -> Element {
///     rsx! {
///         Button {
///             button_type: ButtonType::Primary,
///             "点击我"
///         }
///     }
/// }
/// ```
#[component]
pub fn Button(props: ButtonProps) -> Element {
    let _styles = use_memo(move || generate_button_styles());
    
    // 构建CSS类名
    let class_name = get_button_css_class(&props);
    let full_class = format!("{} {}", class_name, props.class.as_deref().unwrap_or_default());
    
    // 获取HTML类型
    let html_type = get_html_type(&props.html_type);
    
    rsx! {
        button {
            class: full_class,
            style: props.style,
            r#type: html_type,
            disabled: props.disabled || props.loading,
            onclick: move |evt| {
                if !props.disabled && !props.loading {
                    if let Some(handler) = &props.onclick {
                        handler.call(evt);
                    }
                }
            },
            
            // 加载图标
            if props.loading {
                span {
                    class: "ant-btn-loading-icon",
                    Icon {
                        icon_type: "loading".to_string(),
                        spin: true,
                        size: Some("14px".to_string()),
                    }
                }
            }
            
            // 按钮内容
            span {
                class: "ant-btn-content",
                {props.children}
            }
        }
    }
}

/// ButtonGroup 组件
/// 
/// # 示例
/// 
/// ```rust
/// use dioxus::prelude::*;
/// use ant_design_dioxus::{Button, ButtonGroup};
/// 
/// fn app() -> Element {
///     rsx! {
///         ButtonGroup {
///             size: ButtonGroupSize::Large,
///             Button { "按钮1" }
///             Button { "按钮2" }
///             Button { "按钮3" }
///         }
///     }
/// }
/// ```
#[component]
pub fn ButtonGroup(props: ButtonGroupProps) -> Element {
    let class_name = get_button_group_class_name(&props);
    let full_class = format!("{} {}", class_name, props.class.as_deref().unwrap_or_default());
    
    let group_style = get_button_group_style(&props);
    let full_style = match (&group_style, &props.style) {
        (Some(gs), Some(ps)) => Some(format!("{};{}", gs, ps)),
        (Some(gs), None) => Some(gs.clone()),
        (None, Some(ps)) => Some(ps.clone()),
        (None, None) => None,
    };
    
    rsx! {
        div {
            class: full_class,
            style: full_style,
            {props.children}
        }
    }
}

/// 获取按钮CSS类名
fn get_button_css_class(props: &ButtonProps) -> String {
    let mut classes = vec!["ant-btn".to_string()];
    
    // 按钮类型
    match props.button_type {
        ButtonType::Primary => classes.push("ant-btn-primary".to_string()),
        ButtonType::Dashed => classes.push("ant-btn-dashed".to_string()),
        ButtonType::Text => classes.push("ant-btn-text".to_string()),
        ButtonType::Link => classes.push("ant-btn-link".to_string()),
        ButtonType::Default => {}
    }
    
    // 按钮尺寸
    match props.size {
        ButtonSize::Large => classes.push("ant-btn-lg".to_string()),
        ButtonSize::Small => classes.push("ant-btn-sm".to_string()),
        ButtonSize::Middle => {}
    }
    
    // 按钮形状
    match props.shape {
        ButtonShape::Circle => classes.push("ant-btn-circle".to_string()),
        ButtonShape::Round => classes.push("ant-btn-round".to_string()),
        ButtonShape::Default => {}
    }
    
    // 危险状态
    if props.danger {
        classes.push("ant-btn-dangerous".to_string());
    }
    
    // 幽灵状态
    if props.ghost {
        classes.push("ant-btn-background-ghost".to_string());
    }
    
    // 加载状态
    if props.loading {
        classes.push("ant-btn-loading".to_string());
    }
    
    // 块级按钮
    if props.block {
        classes.push("ant-btn-block".to_string());
    }
    
    classes.join(" ")
}

/// 获取HTML按钮类型
fn get_html_type(html_type: &ButtonHtmlType) -> &'static str {
    match html_type {
        ButtonHtmlType::Submit => "submit",
        ButtonHtmlType::Reset => "reset",
        ButtonHtmlType::Button => "button",
    }
}

/// 获取按钮组CSS类名
fn get_button_group_class_name(props: &ButtonGroupProps) -> String {
    let mut classes = vec!["ant-btn-group".to_string()];
    
    // 按钮组尺寸
    match props.size {
        ButtonGroupSize::Large => classes.push("ant-btn-group-lg".to_string()),
        ButtonGroupSize::Small => classes.push("ant-btn-group-sm".to_string()),
        ButtonGroupSize::Middle => {}
    }
    
    classes.join(" ")
}

/// 获取按钮组样式
fn get_button_group_style(props: &ButtonGroupProps) -> Option<String> {
    // 这里可以根据需要添加动态样式逻辑
    None
}

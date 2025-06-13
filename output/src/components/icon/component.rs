//! Icon 组件实现

use dioxus::prelude::*;
use super::types::*;
use super::styles::*;

/// Icon 图标组件
/// 
/// # 示例
/// 
/// ```rust
/// use dioxus::prelude::*;
/// use ant_design_dioxus::Icon;
/// 
/// fn app() -> Element {
///     rsx! {
///         Icon {
///             icon_type: "loading".to_string(),
///             spin: true,
///             size: Some("16px".to_string()),
///         }
///     }
/// }
/// ```
#[component]
pub fn Icon(props: IconProps) -> Element {
    let _styles = use_memo(move || generate_icon_styles());
    
    // 构建CSS类名
    let mut class_names = vec!["ant-icon".to_string()];
    
    if props.spin {
        class_names.push("ant-icon-spin".to_string());
    }
    
    if let Some(custom_class) = &props.class {
        class_names.push(custom_class.clone());
    }
    
    let class_name = class_names.join(" ");
    
    // 构建样式
    let mut inline_styles = Vec::new();
    
    if let Some(size) = &props.size {
        inline_styles.push(format!("font-size: {}; width: {}; height: {}", size, size, size));
    }
    
    if let Some(color) = &props.color {
        inline_styles.push(format!("color: {}", color));
    }
    
    if let Some(custom_style) = &props.style {
        inline_styles.push(custom_style.clone());
    }
    
    let style = if inline_styles.is_empty() {
        None
    } else {
        Some(inline_styles.join("; "))
    };
    
    // 获取SVG内容
    let svg_content = get_icon_svg(&props.icon_type);
    
    rsx! {
        span {
            class: class_name,
            style: style,
            onclick: move |evt| {
                if let Some(handler) = &props.onclick {
                    handler.call(evt);
                }
            },
            dangerous_inner_html: svg_content,
        }
    }
}

/// 获取图标的CSS类名
fn get_icon_css_class(props: &IconProps) -> String {
    let mut classes = vec!["ant-icon".to_string()];
    
    if props.spin {
        classes.push("ant-icon-spin".to_string());
    }
    
    classes.join(" ")
}

/// 获取图标的内联样式
fn get_icon_inline_style(props: &IconProps) -> Option<String> {
    let mut styles = Vec::new();
    
    if let Some(size) = &props.size {
        styles.push(format!("font-size: {}; width: {}; height: {}", size, size, size));
    }
    
    if let Some(color) = &props.color {
        styles.push(format!("color: {}", color));
    }
    
    if let Some(custom_style) = &props.style {
        styles.push(custom_style.clone());
    }
    
    if styles.is_empty() {
        None
    } else {
        Some(styles.join("; "))
    }
}
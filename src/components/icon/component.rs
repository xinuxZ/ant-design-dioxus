//! Icon 组件的核心实现

use super::styles::*;
use super::types::*;
use super::utils::*;
use dioxus::prelude::*;
use wasm_bindgen::JsCast;

/// Icon 组件
///
/// 用于显示图标的组件，支持多种主题、尺寸和交互状态
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
///             icon_type: CommonIconType::Home,
///             theme: IconTheme::Outlined,
///             size: IconSize::Large,
///             spin: false,
///         }
///     }
/// }
/// ```
#[component]
pub fn Icon(props: IconProps) -> Element {
    let IconProps {
        icon_type,
        theme,
        rotate,
        spin,
        two_tone_color,
        component,
        class_name,
        style,
        on_click,
        size,
        disabled,
        // children,
    } = props;

    // 设置默认值
    let theme = theme.unwrap_or(IconTheme::Outlined);
    let spin = spin;
    let disabled = disabled;
    let clickable = on_click.is_some();

    // 生成样式
    let icon_style = generate_icon_style(
        &theme,
        size.as_ref(),
        rotate,
        spin,
        disabled,
        clickable,
        two_tone_color.as_deref(),
    );

    // 生成CSS类名
    let css_classes =
        generate_class_names(&theme, spin, disabled, clickable, class_name.as_deref());

    // 合并自定义样式
    let final_style = if let Some(custom_style) = style {
        format!("{} {}", icon_style, custom_style)
    } else {
        icon_style
    };

    // 处理点击事件
    let handle_click = move |event: MouseEvent| {
        if !disabled {
            if let Some(callback) = &on_click {
                callback.call(event);
            }
        }
    };

    // 渲染图标内容
    let icon_content = match (&icon_type, &component) {
        // 优先使用自定义组件
        (_, Some(custom_component)) => {
            rsx! {
                {custom_component.clone()}
            }
        }
        // 使用预定义的常用图标
        (Some(common_icon), _) => {
            if let Some(svg_icon) = get_common_icon_svg(common_icon) {
                let svg_html = generate_svg_html(&svg_icon, &theme, two_tone_color.as_deref());
                rsx! {
                    div {
                        dangerous_inner_html: "{svg_html}"
                    }
                }
            } else {
                rsx! {
                    span { "Icon not found" }
                }
            }
        }
        // 没有指定图标类型，显示子元素
        (None, None) => {
            rsx! {
                {}
            }
        }
    };

    rsx! {
        span {
            class: "{css_classes}",
            style: "{final_style}",
            onclick: handle_click,
            role: if clickable { "button" } else { "img" },
            tabindex: if clickable && !disabled { "0" } else { "-1" },
            "aria-disabled": if disabled { "true" } else { "false" },

            {icon_content}
        }
    }
}

/// 创建自定义图标组件的辅助函数
///
/// # 参数
///
/// * `svg_content` - SVG内容字符串
/// * `props` - 图标属性
///
/// # 示例
///
/// ```rust
/// use dioxus::prelude::*;
/// use ant_design_dioxus::create_icon;
///
/// fn custom_icon() -> Element {
///     let svg_content = r#"<svg viewBox="0 0 24 24"><path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z"/></svg>"#;
///
///     create_icon(svg_content, IconProps {
///         theme: Some(IconTheme::Filled),
///         size: Some(IconSize::Large),
///         ..Default::default()
///     })
/// }
/// ```
pub fn create_icon(svg_content: &str, props: IconProps) -> Element {
    match parse_svg_string(svg_content) {
        Ok(svg_icon) => {
            let theme = props.theme.clone().unwrap_or(IconTheme::Outlined);
            let svg_html = generate_svg_html(&svg_icon, &theme, props.two_tone_color.as_deref());

            let custom_component = rsx! {
                div {
                    dangerous_inner_html: "{svg_html}"
                }
            };

            rsx! {
                Icon {
                    component: Some(custom_component),
                    theme: props.theme.unwrap(),
                    rotate: props.rotate,
                    spin: props.spin,
                    two_tone_color: props.two_tone_color,
                    class_name: props.class_name,
                    style: props.style,
                    on_click: props.on_click,
                    size: props.size,
                    disabled: props.disabled,
                }
            }
        }
        Err(_) => {
            rsx! {
                span {
                    class: "ant-icon ant-icon-error",
                    "Invalid SVG"
                }
            }
        }
    }
}

/// 图标字体配置组件
///
/// 用于配置图标字体的全局设置
///
/// # 示例
///
/// ```rust
/// use dioxus::prelude::*;
/// use ant_design_dioxus::{IconFontProvider, IconFontConfig};
///
/// fn app() -> Element {
///     rsx! {
///         IconFontProvider {
///             config: IconFontConfig {
///                 script_url: "//at.alicdn.com/t/font_8d5l8fzk5b87iudi.js".to_string(),
///                 extra_common_props: None,
///             },
///
///             // 应用内容
///             div {
///                 "Your app content here"
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn IconFontProvider(config: IconFontConfig, children: Element) -> Element {
    // 在组件挂载时加载图标字体脚本
    use_effect(move || {
        let script_url = config.script_url.clone();

        // 创建script标签并添加到head
        let script_id = format!("iconfont-{}", script_url.replace(['/', ':', '.'], "-"));

        // 检查是否已经加载过该脚本
        if web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id(&script_id))
            .is_none()
        {
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    if let Ok(script) = document.create_element("script") {
                        let script = script.dyn_into::<web_sys::HtmlScriptElement>().unwrap();
                        script.set_src(&script_url);
                        script.set_id(&script_id);

                        if let Some(head) = document.head() {
                            let _ = head.append_child(&script);
                        }
                    }
                }
            }
        }
    });

    rsx! {
        {children}
    }
}

/// 创建图标字体图标的辅助函数
///
/// # 参数
///
/// * `icon_type` - 图标类型（通常是图标字体中的类名）
/// * `props` - 图标属性
///
/// # 示例
///
/// ```rust
/// use dioxus::prelude::*;
/// use ant_design_dioxus::create_iconfont_icon;
///
/// fn iconfont_icon() -> Element {
///     create_iconfont_icon("icon-home", IconProps {
///         size: Some(IconSize::Large),
///         ..Default::default()
///     })
/// }
/// ```
pub fn create_iconfont_icon(icon_type: &str, props: IconProps) -> Element {
    let custom_component = rsx! {
        svg {
            class: "ant-icon",
            "aria-hidden": "true",

            use {
                "xlink:href": "#{icon_type}"
            }
        }
    };

    rsx! {
        Icon {
            component: Some(custom_component),
            theme: props.theme,
            rotate: props.rotate,
            spin: props.spin,
            two_tone_color: props.two_tone_color,
            class_name: props.class_name,
            style: props.style,
            on_click: props.on_click,
            size: props.size,
            disabled: props.disabled,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dioxus::prelude::*;

    #[test]
    fn test_icon_props_default() {
        let props = IconProps::default();
        assert_eq!(props.theme, Some(IconTheme::Outlined));
        assert_eq!(props.spin, false);
        assert_eq!(props.disabled, false);
        assert!(props.icon_type.is_none());
        assert!(props.component.is_none());
    }

    #[test]
    fn test_icon_size_conversion() {
        assert_eq!(IconSize::Small.to_css(), "12px");
        assert_eq!(IconSize::Default.to_css(), "14px");
        assert_eq!(IconSize::Large.to_css(), "16px");
        assert_eq!(IconSize::Custom(20).to_css(), "20px");
    }
}

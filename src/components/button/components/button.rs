use dioxus::prelude::*;

use crate::components::button::components::wave::Wave;
use crate::components::button::styles::button_styles;
use crate::components::button::types::*;
use crate::components::icon::{Icon, IconType};

/// Button 组件
#[component]
pub fn Button(props: ButtonProps) -> Element {
    // 注入样式
    provide_context(button_styles());

    // 状态管理
    let is_loading = use_signal(|| match props.loading {
        LoadingConfig::Loading => true,
        LoadingConfig::NotLoading => false,
        LoadingConfig::DelayedLoading(_) => false,
    });

    // 处理延迟加载
    if let LoadingConfig::DelayedLoading(delay) = props.loading {
        use_effect(move || {
            let timeout = gloo_timers::callback::Timeout::new(delay as u32, move || {
                is_loading.set(true);
            });

            (move || {
                drop(timeout);
            })()
        });
    }

    // 处理两个中文字符之间的空格
    let has_two_cn_chars = use_signal(|| false);
    let inner_content = use_signal(|| String::new());

    // 提取文本内容
    if props.auto_insert_space {
        use_effect(move || {
            if let Some(text) = extract_text_content(&props.children) {
                inner_content.set(text.clone());
                has_two_cn_chars.set(is_two_chinese_chars(&text));
            }
        });
    }

    // 使用 memo 优化类名生成，避免不必要的重新计算
    let class_name = use_memo(move || {
        generate_button_class_name(&props, has_two_cn_chars.get(), is_loading.get())
    });

    // 处理点击事件
    let handle_click = move |e: MouseEvent| {
        if props.disabled || is_loading.get() {
            e.stop_propagation();
            return;
        }

        if let Some(handler) = &props.on_click {
            handler.call(e);
        }
    };

    // 处理键盘事件
    let handle_key_down = move |e: KeyboardEvent| {
        if props.disabled || is_loading.get() {
            return;
        }

        if e.key() == Key::Enter {
            e.stop_propagation();
            e.prevent_default();

            if let Some(handler) = &props.on_click {
                let event = MouseEvent::new(web_sys::MouseEvent::new("click").unwrap(), e.target());
                handler.call(event);
            }
        }
    };

    // 使用 memo 优化按钮内容渲染，避免不必要的重新渲染
    let button_children = use_memo(move || render_button_content(&props, is_loading.get()));

    // 获取 ARIA 标签
    let aria_label = props.aria_label.clone().unwrap_or_default();
    let aria_disabled = props.disabled.to_string();
    let aria_busy = is_loading.get().to_string();

    // 按钮内容
    let button_content = if let Some(href) = &props.href {
        rsx! {
            a {
                class: "{class_name}",
                href: "{href}",
                target: props.target.as_deref().unwrap_or(""),
                style: props.style.as_deref().unwrap_or(""),
                disabled: props.disabled,
                "aria-label": aria_label,
                "aria-disabled": aria_disabled,
                "aria-busy": aria_busy,
                role: "button",
                tabindex: if props.disabled { "-1" } else { "0" },
                onclick: handle_click,
                onkeydown: handle_key_down,
                {button_children}
            }
        }
    } else {
        rsx! {
            button {
                class: "{class_name}",
                r#type: match props.html_type {
                    HtmlType::Button => "button",
                    HtmlType::Submit => "submit",
                    HtmlType::Reset => "reset",
                },
                style: props.style.as_deref().unwrap_or(""),
                disabled: props.disabled,
                "aria-label": aria_label,
                "aria-disabled": aria_disabled,
                "aria-busy": aria_busy,
                tabindex: if props.disabled { "-1" } else { "0" },
                onclick: handle_click,
                onkeydown: handle_key_down,
                {button_children}
            }
        }
    };

    // 使用 Wave 组件包装按钮内容，添加波纹效果
    rsx! {
        Wave {
            disabled: props.disabled || is_loading.get(),
            color: get_ripple_color(&props),
            {button_content}
        }
    }
}

/// 获取波纹颜色
fn get_ripple_color(props: &ButtonProps) -> String {
    if let Some(button_type) = &props.button_type {
        match button_type {
            ButtonType::Primary => "rgba(255, 255, 255, 0.3)".to_string(),
            ButtonType::Default => "rgba(0, 0, 0, 0.1)".to_string(),
            ButtonType::Dashed => "rgba(0, 0, 0, 0.1)".to_string(),
            ButtonType::Text => "rgba(0, 0, 0, 0.1)".to_string(),
            ButtonType::Link => "rgba(0, 0, 0, 0.1)".to_string(),
        }
    } else if let Some(color) = &props.color {
        match color {
            ButtonColor::Primary => "rgba(255, 255, 255, 0.3)".to_string(),
            ButtonColor::Default => "rgba(0, 0, 0, 0.1)".to_string(),
            ButtonColor::Danger => "rgba(255, 255, 255, 0.3)".to_string(),
            ButtonColor::Custom(_) => "rgba(0, 0, 0, 0.1)".to_string(),
        }
    } else {
        "rgba(0, 0, 0, 0.1)".to_string()
    }
}

/// 生成按钮类名
fn generate_button_class_name(
    props: &ButtonProps,
    has_two_cn_chars: bool,
    is_loading: bool,
) -> String {
    let mut classes = vec!["ant-btn".to_string()];

    // 添加用户自定义类名
    if let Some(class) = &props.class {
        classes.push(class.clone());
    }

    // 处理按钮类型
    if let Some(button_type) = &props.button_type {
        match button_type {
            ButtonType::Primary => classes.push("ant-btn-primary".to_string()),
            ButtonType::Default => classes.push("ant-btn-default".to_string()),
            ButtonType::Dashed => classes.push("ant-btn-dashed".to_string()),
            ButtonType::Text => classes.push("ant-btn-text".to_string()),
            ButtonType::Link => classes.push("ant-btn-link".to_string()),
        }
    } else {
        // 默认类型
        classes.push("ant-btn-default".to_string());
    }

    // 处理按钮颜色
    if let Some(color) = &props.color {
        match color {
            ButtonColor::Primary => classes.push("ant-btn-primary".to_string()),
            ButtonColor::Default => classes.push("ant-btn-default".to_string()),
            ButtonColor::Danger => classes.push("ant-btn-dangerous".to_string()),
            ButtonColor::Custom(custom_color) => {
                classes.push(format!("ant-btn-custom-{}", custom_color.replace("#", "")));
            }
        }
    }

    // 处理按钮变体
    if let Some(variant) = &props.variant {
        match variant {
            ButtonVariant::Outlined => classes.push("ant-btn-outlined".to_string()),
            ButtonVariant::Solid => classes.push("ant-btn-solid".to_string()),
            ButtonVariant::Dashed => classes.push("ant-btn-dashed".to_string()),
            ButtonVariant::Text => classes.push("ant-btn-text".to_string()),
            ButtonVariant::Link => classes.push("ant-btn-link".to_string()),
        }
    }

    // 处理按钮大小
    match props.size {
        ButtonSize::Large => classes.push("ant-btn-large".to_string()),
        ButtonSize::Middle => {} // 默认大小，不添加类名
        ButtonSize::Small => classes.push("ant-btn-small".to_string()),
    }

    // 处理按钮形状
    match props.shape {
        ButtonShape::Default => {} // 默认形状，不添加类名
        ButtonShape::Circle => classes.push("ant-btn-circle".to_string()),
        ButtonShape::Round => classes.push("ant-btn-round".to_string()),
    }

    // 处理危险按钮
    if props.danger {
        classes.push("ant-btn-dangerous".to_string());
    }

    // 处理块级按钮
    if props.block {
        classes.push("ant-btn-block".to_string());
    }

    // 处理幽灵按钮
    if props.ghost {
        classes.push("ant-btn-background-ghost".to_string());
    }

    // 处理加载状态
    if is_loading {
        classes.push("ant-btn-loading".to_string());
    }

    // 处理图标位置
    if props.icon_position == IconPosition::End {
        classes.push("ant-btn-icon-end".to_string());
    }

    // 处理纯图标按钮
    if props.icon.is_some() && props.children.is_err() {
        classes.push("ant-btn-icon-only".to_string());
    }

    // 处理两个中文字符
    if has_two_cn_chars {
        classes.push("ant-btn-two-chinese-chars".to_string());
    }

    // 添加焦点可见类
    classes.push("focus-visible:outline-offset-2".to_string());
    classes.push("focus-visible:outline-2".to_string());
    classes.push("focus-visible:outline-blue-400".to_string());

    classes.join(" ")
}

/// 渲染按钮内容
fn render_button_content(props: &ButtonProps, is_loading: bool) -> Element {
    let has_icon = props.icon.is_some();
    let has_children = props.children.is_ok();

    // 加载图标
    let loading_icon = if is_loading {
        Some(rsx! {
            span {
                class: "ant-btn-loading-icon",
                "aria-hidden": "true",
                Icon {
                    icon_type: IconType::Loading,
                    ..Default::default()
                }
            }
        })
    } else {
        None
    };

    // 图标
    let icon = if !is_loading && has_icon {
        let mut icon_element = props.icon.clone();

        // 为图标添加 aria-hidden 属性
        if let Some(element) = &icon_element {
            // 这里简化处理，实际项目中可能需要更复杂的方式来添加属性
            icon_element = Some(rsx! {
                span {
                    "aria-hidden": "true",
                    {element}
                }
            });
        }

        icon_element
    } else {
        None
    };

    // 根据图标位置渲染内容
    if props.icon_position == IconPosition::Start {
        rsx! {
            {loading_icon}
            {icon}
            {if has_children {
                rsx! {
                    span {
                        class: if has_icon || is_loading { "ant-btn-text-with-icon" } else { "" },
                        {props.children.clone()}
                    }
                }
            } else {
                rsx! {}
            }}
        }
    } else {
        rsx! {
            {if has_children {
                rsx! {
                    span {
                        class: if has_icon || is_loading { "ant-btn-text-with-icon" } else { "" },
                        {props.children.clone()}
                    }
                }
            } else {
                rsx! {}
            }}
            {loading_icon}
            {icon}
        }
    }
}

/// 提取文本内容
fn extract_text_content(element: &Element) -> Option<String> {
    match element {
        Ok(vnode) => match vnode {
            VNode::Text(text) => Some(text.to_string()),
            VNode::Fragment(children) => {
                let mut result = String::new();
                for child in children {
                    if let VNode::Text(text) = child {
                        result.push_str(text);
                    }
                }
                if result.is_empty() {
                    None
                } else {
                    Some(result)
                }
            }
            _ => None,
        },
        Err(_) => None,
    }
}

/// 判断是否是两个中文字符
fn is_two_chinese_chars(text: &str) -> bool {
    let chars: Vec<char> = text.chars().collect();

    if chars.len() != 2 {
        return false;
    }

    chars.iter().all(|&c| is_chinese_char(c))
}

/// 判断是否是中文字符
fn is_chinese_char(c: char) -> bool {
    let code = c as u32;

    (code >= 0x4E00 && code <= 0x9FFF)
        || (code >= 0x3400 && code <= 0x4DBF)
        || (code >= 0x20000 && code <= 0x2A6DF)
        || (code >= 0x2A700 && code <= 0x2B73F)
        || (code >= 0x2B740 && code <= 0x2B81F)
        || (code >= 0x2B820 && code <= 0x2CEAF)
}

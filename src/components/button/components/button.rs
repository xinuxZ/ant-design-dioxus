use dioxus::prelude::*;

use crate::components::button::styles::button_styles;
use crate::components::button::types::*;
use crate::components::icon::Icon;

/// Button 组件
#[component]
pub fn Button(props: ButtonProps) -> Element {
    // 注入样式
    use_shared_state_provider(|| button_styles());

    // 状态管理
    let is_loading = use_signal(|| match props.loading {
        LoadingConfig::Loading => true,
        LoadingConfig::NotLoading => false,
        LoadingConfig::DelayedLoading(_) => false,
    });

    // 处理延迟加载
    if let LoadingConfig::DelayedLoading(delay) = props.loading {
        use_effect(move || {
            let timer = set_timeout(
                move || {
                    is_loading.set(true);
                },
                std::time::Duration::from_millis(delay as u64),
            );

            move || {
                clear_timeout(timer);
            }
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
    let class_name =
        use_memo(move || generate_button_class_name(&props, *has_two_cn_chars, *is_loading));

    // 处理点击事件
    let handle_click = move |e: MouseEvent| {
        if props.disabled || *is_loading {
            e.stop_propagation();
            return;
        }

        if let Some(handler) = &props.on_click {
            handler.call(e);
        }
    };

    // 处理键盘事件
    let handle_key_down = move |e: KeyboardEvent| {
        if props.disabled || *is_loading {
            return;
        }

        // 当按下 Enter 或 Space 键时触发点击事件
        if e.key() == "Enter" || e.key() == " " {
            e.stop_propagation();
            e.prevent_default();

            if let Some(handler) = &props.on_click {
                handler.call(MouseEvent::default());
            }
        }
    };

    // 使用 memo 优化按钮内容渲染，避免不必要的重新渲染
    let button_children = use_memo(move || render_button_content(&props, *is_loading));

    // 获取 ARIA 标签
    let aria_label = props.aria_label.clone().unwrap_or_default();
    let aria_disabled = props.disabled.to_string();
    let aria_busy = is_loading.to_string();

    // 根据是否有 href 属性决定渲染 button 还是 a 标签
    if let Some(href) = &props.href {
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
    if props.icon.is_some() && props.children.is_empty() {
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
    let has_children = !props.children.is_empty();

    // 加载图标
    let loading_icon = if is_loading {
        Some(rsx! {
            span {
                class: "ant-btn-loading-icon",
                "aria-hidden": "true",
                Icon {
                    r#type: "loading",
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

    // 根据图标位置排列内容
    if props.icon_position == IconPosition::Start {
        rsx! {
            {loading_icon}
            {icon}
            {if has_children {
                rsx! { span { {props.children.clone()} } }
            } else {
                None
            }}
        }
    } else {
        rsx! {
            {if has_children {
                rsx! { span { {props.children.clone()} } }
            } else {
                None
            }}
            {loading_icon}
            {icon}
        }
    }
}

/// 提取文本内容
fn extract_text_content(element: &Element) -> Option<String> {
    // 简单实现，实际项目中可能需要更复杂的递归遍历
    if let Some(text) = element.as_text() {
        Some(text.to_string())
    } else {
        None
    }
}

/// 判断是否为两个中文字符
fn is_two_chinese_chars(text: &str) -> bool {
    let chinese_chars: Vec<char> = text
        .chars()
        .filter(|c| '\u{4e00}' <= *c && *c <= '\u{9fff}')
        .collect();

    chinese_chars.len() == 2
}

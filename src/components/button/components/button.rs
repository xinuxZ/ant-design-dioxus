use dioxus::html::input_data::keyboard_types::Key;
use dioxus::prelude::*;

use crate::components::button::components::wave::Wave;
use crate::components::button::styles::button_styles;
use crate::components::button::types::*;
use crate::components::icon::{CommonIconType, Icon};

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
            // Convert Result<VNode, RenderError> to Option<Element>
            // Just check if children is Ok and then extract text directly
            if props.children.is_ok() {
                // In a real implementation, we would extract text from the children
                // For now, use a placeholder
                let text = "Button".to_string();
                inner_content.set(text.clone());
                has_two_cn_chars.set(is_two_chinese_chars(&text));
            }
            // We already handled text extraction above
        });
    }

    // 使用 memo 优化类名生成，避免不必要的重新计算
    let class_name =
        use_memo(move || generate_button_class_name(&props, has_two_cn_chars(), is_loading()));

    // 处理点击事件
    let handle_click = move |e: MouseEvent| {
        if props.disabled || is_loading() {
            e.stop_propagation();
            return;
        }

        if let Some(handler) = &props.on_click {
            handler.call(e);
        }
    };

    // 处理键盘事件
    let handle_key_down = move |e: KeyboardEvent| {
        if props.disabled || is_loading() {
            return;
        }

        // 使用正确的Key枚举值
        if e.key() == Key::Enter || e.key().to_string() == " " {
            e.stop_propagation();
            e.prevent_default();

            if let Some(handler) = &props.on_click {
                // 创建一个模拟的点击事件
                // 使用默认构造函数
                let event = MouseEvent::default();
                handler.call(event);
            }
        }
    };

    // 使用 memo 优化按钮内容渲染，避免不必要的重新渲染
    let button_children = use_memo(move || render_button_content(&props, is_loading()));

    // 获取 ARIA 标签
    let aria_label = props.aria_label.clone().unwrap_or_default();
    let aria_disabled = props.disabled.to_string();
    let aria_busy = is_loading().to_string();

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
                disabled: props.disabled || is_loading(),
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
            disabled: props.disabled || is_loading(),
            color: Some(get_ripple_color(&props)),
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
        classes.push("ant-btn-icon-right".to_string());
    }

    // 处理两个中文字符之间的空格
    if has_two_cn_chars {
        classes.push("ant-btn-two-chinese-chars".to_string());
    }

    classes.join(" ")
}

/// 渲染按钮内容
fn render_button_content(props: &ButtonProps, is_loading: bool) -> Element {
    let has_icon = props.icon.is_some();
    let has_children = props.children.is_ok();
    let icon_position = props.icon_position;

    // 创建加载图标
    let loading_icon = if is_loading {
        rsx! {
            span {
                class: "ant-btn-loading-icon",
                "aria-hidden": "true",
                Icon {
                    icon_type: Some(CommonIconType::Loading),
                    spin: true,
                }
            }
        }
    } else {
        rsx! {}
    };

    // 创建按钮图标
    let button_icon = if let Some(icon) = &props.icon {
        if !is_loading {
            rsx! {
                span {
                    class: "ant-btn-icon",
                    "aria-hidden": "true",
                    {icon.clone()}
                }
            }
        } else {
            rsx! {}
        }
    } else {
        rsx! {}
    };

    // 根据图标位置渲染内容
    match icon_position {
        IconPosition::Start => {
            rsx! {
                {loading_icon}
                {button_icon}
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
        }
        IconPosition::End => {
            rsx! {
                {loading_icon}
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
                {button_icon}
            }
        }
    }
}

/// 提取文本内容
fn extract_text_content(element: &Option<Element>) -> Option<String> {
    if let Some(elem) = element {
        // Extract text content from element
        // This is a simplified implementation
        Some("Text content".to_string())
    } else {
        None
    }
}

/// 检查是否是两个中文字符
fn is_two_chinese_chars(text: &str) -> bool {
    let chars: Vec<char> = text.chars().collect();
    chars.len() == 2 && chars.iter().all(|c| is_chinese_char(*c))
}

/// 检查字符是否是中文字符
fn is_chinese_char(c: char) -> bool {
    // 中文字符的 Unicode 范围
    (c >= '\u{4e00}' && c <= '\u{9fff}') || // CJK 统一表意文字
    (c >= '\u{3400}' && c <= '\u{4dbf}') || // CJK 统一表意文字扩展 A
    (c >= '\u{20000}' && c <= '\u{2a6df}') // CJK 统一表意文字扩展 B
}

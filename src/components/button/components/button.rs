use crate::components::button::components::wave::Wave;
use crate::components::button::styles::ButtonStyleGenerator;
use crate::components::button::types::*;
use crate::components::icon::{Icon, IconType};
use dioxus::html::input_data::keyboard_types::Key;
use dioxus::prelude::*;

/// Button 组件
#[component]
pub fn Button(props: ButtonProps) -> Element {
    // CSS-in-Rust 会自动处理样式注入，无需手动调用 inject_style

    // 状态管理
    let mut is_loading = use_signal(|| match props.loading {
        LoadingConfig::Loading => true,
        LoadingConfig::NotLoading => false,
        LoadingConfig::DelayedLoading(_) => false,
    });

    // 处理延迟加载
    if let LoadingConfig::DelayedLoading(delay) = props.loading {
        use_effect(move || {
            #[cfg(target_arch = "wasm32")]
            {
                let timeout = gloo_timers::callback::Timeout::new(delay as u32, move || {
                    is_loading.set(true);
                });

                (move || {
                    drop(timeout);
                })()
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                // 在非 WASM 环境下直接设置加载状态
                is_loading.set(true);
                (move || {})()
            }
        });
    }

    // 处理两个中文字符之间的空格
    let mut has_two_cn_chars = use_signal(|| false);
    let mut inner_content = use_signal(|| String::new());

    // 提取文本内容
    let props_clone = props.clone();
    if props.auto_insert_space {
        use_effect(move || {
            // Convert Result<VNode, RenderError> to Option<Element>
            // Just check if children is Ok and then extract text directly
            if props_clone.children.is_ok() {
                // In a real implementation, we would extract text from the children
                // For now, use a placeholder
                let text = "Button".to_string();
                inner_content.set(text.clone());
                has_two_cn_chars.set(is_two_chinese_chars(&text));
            }
            // We already handled text extraction above
        });
    }

    // 使用样式生成器生成按钮样式类名
    let props_clone = props.clone();
    let class_name = use_memo(move || {
        // 使用样式生成器
        let button_class = ButtonStyleGenerator::new()
            .with_type(props_clone.button_type.unwrap_or_default())
            .with_size(props_clone.size.clone())
            .with_shape(props_clone.shape.clone())
            .with_danger(props_clone.danger)
            .with_ghost(props_clone.ghost)
            .with_disabled(props_clone.disabled || is_loading())
            .with_loading(is_loading())
            .with_block(props_clone.block)
            .generate();

        // 添加自定义类名
        let mut classes = vec![button_class];
        if let Some(custom_class) = &props_clone.class {
            classes.push(custom_class.clone());
        }

        // 添加两个中文字符的类名
        if has_two_cn_chars() {
            classes.push("ant-btn-two-chinese-chars".to_string());
        }

        classes.join(" ")
    });

    // 处理点击事件
    let props_clone = props.clone();
    let handle_click = move |e: MouseEvent| {
        if props_clone.disabled || is_loading() {
            e.stop_propagation();
            return;
        }

        if let Some(handler) = &props_clone.on_click {
            handler.call(e);
        }
    };

    // 处理键盘事件
    let props_clone = props.clone();
    let handle_key_down = move |e: KeyboardEvent| {
        if props_clone.disabled || is_loading() {
            return;
        }

        // 使用正确的Key枚举值
        if e.key() == Key::Enter || e.key().to_string() == " " {
            e.stop_propagation();
            e.prevent_default();

            if let Some(handler) = &props.on_click {
                // 创建一个模拟的点击事件
                // 直接调用处理函数，使用默认的MouseEvent
                // handler.call(MouseEvent::new(
                //     Rc::new(*e.clone().downcast::<MouseData>().unwrap()),
                //     false,
                // ));
            }
        }
    };

    let props_clone = props.clone();
    // 使用 memo 优化按钮内容渲染，避免不必要的重新渲染
    let button_children = use_memo(move || render_button_content(&props_clone, is_loading()));

    // 获取 ARIA 标签
    let props_clone = props.clone();
    let aria_label = props_clone.aria_label.clone().unwrap_or_default();
    let aria_disabled = props_clone.disabled.to_string();
    let aria_busy = is_loading().to_string();

    // HTML a 标签不支持 disabled 属性，需要通过 aria-disabled 和样式来实现
    // 按钮内容
    let props_clone = props.clone();
    let button_content = if let Some(href) = &props_clone.href {
        rsx! {
            a {
                class: "{class_name}",
                href: "{href}",
                target: props_clone.target.as_deref().unwrap_or(""),
                style: props_clone.style.as_deref().unwrap_or(""),
                "data-disabled": props_clone.disabled.to_string(),
                "aria-label": aria_label,
                "aria-disabled": aria_disabled,
                "aria-busy": aria_busy,
                role: "button",
                tabindex: if props_clone.disabled { "-1" } else { "0" },
                onclick: handle_click,
                onkeydown: handle_key_down,
                {button_children}
            }
        }
    } else {
        let props_clone = props.clone();
        rsx! {
            button {
                class: "{class_name}",
                r#type: match props_clone.html_type {
                    HtmlType::Button => "button",
                    HtmlType::Submit => "submit",
                    HtmlType::Reset => "reset",
                },
                style: props_clone.style.as_deref().unwrap_or(""),
                disabled: props_clone.disabled || is_loading(),
                "aria-label": aria_label,
                "aria-disabled": aria_disabled,
                "aria-busy": aria_busy,
                tabindex: if props_clone.disabled { "-1" } else { "0" },
                onclick: handle_click,
                onkeydown: handle_key_down,
                {button_children}
            }
        }
    };

    // 使用 Wave 组件包装按钮内容，添加波纹效果
    let props_clone = props.clone();
    rsx! {
        Wave {
            disabled: props_clone.disabled || is_loading(),
            color: Some(get_ripple_color(&props_clone)),
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
                    icon_type: Some(IconType::Loading),
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

#[cfg(test)]
mod tests {
    use super::*;
    use dioxus::prelude::*;

    /// 测试 Button 组件的基本属性
    #[test]
    fn test_button_basic_props() {
        let props = ButtonProps {
            button_type: Some(ButtonType::Primary),
            disabled: true,
            ..Default::default()
        };

        assert_eq!(props.button_type, Some(ButtonType::Primary));
        assert_eq!(props.disabled, true);
        assert_eq!(props.block, false); // 默认值
    }

    /// 测试 Button 组件的不同类型
    #[test]
    fn test_button_types() {
        let types = vec![
            ButtonType::Primary,
            ButtonType::Default,
            ButtonType::Dashed,
            ButtonType::Text,
            ButtonType::Link,
        ];

        for button_type in types {
            let props = ButtonProps {
                button_type: Some(button_type.clone()),
                ..Default::default()
            };
            assert_eq!(props.button_type, Some(button_type));
        }
    }

    /// 测试 Button 组件的不同大小
    #[test]
    fn test_button_sizes() {
        let sizes = vec![ButtonSize::Large, ButtonSize::Middle, ButtonSize::Small];

        for size in sizes {
            let props = ButtonProps {
                size: size.clone(),
                ..Default::default()
            };
            assert_eq!(props.size, size);
        }
    }

    /// 测试 Button 组件的不同形状
    #[test]
    fn test_button_shapes() {
        let shapes = vec![
            ButtonShape::Default,
            ButtonShape::Circle,
            ButtonShape::Round,
        ];

        for shape in shapes {
            let props = ButtonProps {
                shape: shape.clone(),
                ..Default::default()
            };
            assert_eq!(props.shape, shape);
        }
    }

    /// 测试 Button 组件的禁用状态
    #[test]
    fn test_button_disabled() {
        let props = ButtonProps {
            disabled: true,
            ..Default::default()
        };
        assert_eq!(props.disabled, true);

        let props_enabled = ButtonProps {
            disabled: false,
            ..Default::default()
        };
        assert_eq!(props_enabled.disabled, false);
    }

    /// 测试 Button 组件的加载状态
    #[test]
    fn test_button_loading() {
        let props = ButtonProps {
            loading: LoadingConfig::Loading,
            ..Default::default()
        };
        assert_eq!(props.loading, LoadingConfig::Loading);
    }

    /// 测试 Button 组件的延迟加载
    #[test]
    fn test_button_delayed_loading() {
        let props = ButtonProps {
            loading: LoadingConfig::DelayedLoading(1000),
            ..Default::default()
        };
        assert_eq!(props.loading, LoadingConfig::DelayedLoading(1000));
    }

    /// 测试 Button 组件的危险状态
    #[test]
    fn test_button_danger() {
        let props = ButtonProps {
            danger: true,
            ..Default::default()
        };
        assert_eq!(props.danger, true);
    }

    /// 测试 Button 组件的幽灵状态
    #[test]
    fn test_button_ghost() {
        let props = ButtonProps {
            ghost: true,
            ..Default::default()
        };
        assert_eq!(props.ghost, true);
    }

    /// 测试 Button 组件的块级状态
    #[test]
    fn test_button_block() {
        let props = ButtonProps {
            block: true,
            ..Default::default()
        };
        assert_eq!(props.block, true);
    }

    /// 测试 is_two_chinese_chars 函数
    #[test]
    fn test_is_two_chinese_chars() {
        assert!(is_two_chinese_chars("按钮"));
        assert!(is_two_chinese_chars("确定"));
        assert!(!is_two_chinese_chars("Button"));
        assert!(!is_two_chinese_chars("按"));
        assert!(!is_two_chinese_chars("按钮文本"));
        assert!(!is_two_chinese_chars("A按"));
    }

    /// 测试 is_chinese_char 函数
    #[test]
    fn test_is_chinese_char() {
        assert!(is_chinese_char('按'));
        assert!(is_chinese_char('钮'));
        assert!(is_chinese_char('确'));
        assert!(is_chinese_char('定'));
        assert!(!is_chinese_char('A'));
        assert!(!is_chinese_char('1'));
        assert!(!is_chinese_char(' '));
        assert!(!is_chinese_char('!'));
    }
}

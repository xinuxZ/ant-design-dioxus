//! Rate 评分组件
//!
//! 评分组件，用于对事物进行评级操作。
//!
//! ## 何时使用
//!
//! - 对评价进行展示
//! - 对事物进行快速的评级操作
//!
//! ## 示例
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::Rate;
//!
//! fn app() -> Element {
//!     rsx! {
//!         Rate {
//!             default_value: 2.5,
//!             on_change: |value| {
//!                 println!("评分: {}", value);
//!             }
//!         }
//!     }
//! }
//! ```

use dioxus::prelude::*;
use std::fmt;

const RATE_STYLES: &str = include_str!("./style.css");

/// 评分组件尺寸
#[derive(Debug, Clone, PartialEq)]
pub enum RateSize {
    Small,
    Middle,
    Large,
}

impl Default for RateSize {
    fn default() -> Self {
        Self::Middle
    }
}

impl fmt::Display for RateSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Small => write!(f, "small"),
            Self::Middle => write!(f, "middle"),
            Self::Large => write!(f, "large"),
        }
    }
}

/// 评分组件属性
#[derive(Props, Clone, PartialEq)]
pub struct RateProps {
    /// 自定义类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,

    /// 组件尺寸
    #[props(default)]
    pub size: RateSize,

    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,

    /// 是否只读
    #[props(default = false)]
    pub readonly: bool,

    /// 是否允许半选
    #[props(default = false)]
    pub allow_half: bool,

    /// 是否允许清除
    #[props(default = true)]
    pub allow_clear: bool,

    /// 是否自动获取焦点
    #[props(default = false)]
    pub auto_focus: bool,

    /// 星星总数
    #[props(default = 5)]
    pub count: usize,

    /// 默认值
    #[props(default = 0.0)]
    pub default_value: f64,

    /// 当前值
    #[props(default)]
    pub value: Option<f64>,

    /// 自定义字符
    #[props(default)]
    pub character: Option<Element>,

    /// 自定义字符函数
    #[props(default)]
    pub character_render: Option<fn(usize) -> Element>,

    /// 自定义每项的提示信息
    #[props(default)]
    pub tooltips: Option<Vec<String>>,

    /// 值改变时的回调
    #[props(default)]
    pub on_change: Option<EventHandler<f64>>,

    /// 鼠标经过时的回调
    #[props(default)]
    pub on_hover_change: Option<EventHandler<f64>>,

    /// 获得焦点时的回调
    #[props(default)]
    pub on_focus: Option<EventHandler<FocusEvent>>,

    /// 失去焦点时的回调
    #[props(default)]
    pub on_blur: Option<EventHandler<FocusEvent>>,

    /// 按键事件回调
    #[props(default)]
    pub on_key_down: Option<EventHandler<KeyboardEvent>>,
}

/// Rate 组件
#[component]
pub fn Rate(props: RateProps) -> Element {
    let mut current_value = use_signal(|| props.value.unwrap_or(props.default_value));
    let mut hover_value = use_signal(|| None::<f64>);
    let mut focused = use_signal(|| false);

    let display_value = hover_value().unwrap_or(current_value());

    let class_name = format!(
        "ant-rate {} {} {} {}",
        if props.disabled {
            "ant-rate-disabled"
        } else {
            ""
        },
        if props.readonly {
            "ant-rate-readonly"
        } else {
            ""
        },
        if focused() { "ant-rate-focused" } else { "" },
        format!("ant-rate-{}", props.size),
    )
    .trim()
    .to_string();

    let class_name = if let Some(custom_class) = &props.class {
        format!("{} {}", class_name, custom_class)
    } else {
        class_name
    };

    let mut handle_star_click = move |index: usize, is_half: bool| {
        if props.disabled || props.readonly {
            return;
        }

        let new_value = if is_half {
            index as f64 + 0.5
        } else {
            index as f64 + 1.0
        };

        // 如果允许清除且点击的是当前值，则清除
        let final_value = if props.allow_clear && (new_value - current_value()).abs() < 0.1 {
            0.0
        } else {
            new_value
        };

        current_value.set(final_value);

        if let Some(handler) = &props.on_change {
            handler.call(final_value);
        }
    };

    let mut handle_star_hover = move |index: usize, is_half: bool| {
        if props.disabled {
            return;
        }

        let new_value = if is_half {
            index as f64 + 0.5
        } else {
            index as f64 + 1.0
        };

        hover_value.set(Some(new_value));

        if let Some(handler) = &props.on_hover_change {
            handler.call(new_value);
        }
    };

    let handle_mouse_leave = move |_| {
        hover_value.set(None);

        if let Some(handler) = &props.on_hover_change {
            handler.call(current_value());
        }
    };

    let handle_focus = move |e: FocusEvent| {
        focused.set(true);
        if let Some(handler) = &props.on_focus {
            handler.call(e);
        }
    };

    let handle_blur = move |e: FocusEvent| {
        focused.set(false);
        if let Some(handler) = &props.on_blur {
            handler.call(e);
        }
    };

    let handle_key_down = move |e: KeyboardEvent| {
        if props.disabled || props.readonly {
            return;
        }

        match e.key() {
            Key::ArrowRight | Key::ArrowUp => {
                e.prevent_default();
                let new_value = (current_value() + if props.allow_half { 0.5 } else { 1.0 })
                    .min(props.count as f64);
                current_value.set(new_value);

                if let Some(handler) = &props.on_change {
                    handler.call(new_value);
                }
            }
            Key::ArrowLeft | Key::ArrowDown => {
                e.prevent_default();
                let new_value =
                    (current_value() - if props.allow_half { 0.5 } else { 1.0 }).max(0.0);
                current_value.set(new_value);

                if let Some(handler) = &props.on_change {
                    handler.call(new_value);
                }
            }
            Key::Home => {
                e.prevent_default();
                current_value.set(0.0);

                if let Some(handler) = &props.on_change {
                    handler.call(0.0);
                }
            }
            Key::End => {
                e.prevent_default();
                let new_value = props.count as f64;
                current_value.set(new_value);

                if let Some(handler) = &props.on_change {
                    handler.call(new_value);
                }
            }
            _ => {}
        }

        if let Some(handler) = &props.on_key_down {
            handler.call(e);
        }
    };

    let get_star_state = |index: usize| -> (bool, bool, bool) {
        let star_value = index as f64 + 1.0;
        let half_star_value = index as f64 + 0.5;

        let is_full = display_value >= star_value;
        let is_half =
            props.allow_half && display_value >= half_star_value && display_value < star_value;
        let is_zero = display_value < half_star_value;

        (is_full, is_half, is_zero)
    };

    rsx! {
        style { {RATE_STYLES} }

        div {
            class: class_name,
            style: props.style.as_deref().unwrap_or(""),
            tabindex: if props.disabled { "-1" } else { "0" },
            autofocus: props.auto_focus,
            onmouseleave: handle_mouse_leave,
            onfocus: handle_focus,
            onblur: handle_blur,
            onkeydown: handle_key_down,

            for index in 0..props.count {
                {
                    let (is_full, is_half, _is_zero) = get_star_state(index);
                    let _tooltip = props.tooltips.as_ref().and_then(|t| t.get(index)).cloned().unwrap_or_default();

                    rsx! {
                        div {
                            key: index,
                            class: format!(
                                "ant-rate-star {} {} {}",
                                if is_full { "ant-rate-star-full" } else { "" },
                                if is_half { "ant-rate-star-half" } else { "" },
                                if is_full || is_half { "ant-rate-star-active" } else { "ant-rate-star-zero" }
                            ),
                            title: {
                                if let Some(tooltips) = &props.tooltips {
                                    if index < tooltips.len() {
                                        tooltips[index].clone()
                                    } else {
                                        format!("{}", (index + 1) as f64 * if props.allow_half { 0.5 } else { 1.0 })
                                    }
                                } else {
                                    format!("{}", (index + 1) as f64 * if props.allow_half { 0.5 } else { 1.0 })
                                }
                            },

                            div {
                                class: "ant-rate-star-first",
                                onmouseenter: {
                                    let index = index;
                                    move |_| handle_star_hover(index, true)
                                },
                                onclick: {
                                    let index = index;
                                    move |_| handle_star_click(index, true)
                                },

                                if let Some(character) = &props.character {
                                    {character.clone()}
                                } else if let Some(render_fn) = props.character_render {
                                    {render_fn(index)}
                                } else {
                                    span {
                                        class: "ant-rate-star-icon",
                                        "★"
                                    }
                                }
                            }

                            if props.allow_half {
                                div {
                                    class: "ant-rate-star-second",
                                    onmouseenter: {
                                        let index = index;
                                        move |_| handle_star_hover(index, false)
                                    },
                                    onclick: {
                                        let index = index;
                                        move |_| handle_star_click(index, false)
                                    },

                                    if let Some(character) = &props.character {
                                        {character.clone()}
                                    } else if let Some(render_fn) = props.character_render {
                                        {render_fn(index)}
                                    } else {
                                        span {
                                            class: "ant-rate-star-icon",
                                            "★"
                                        }
                                    }
                                }
                            } else {
                                div {
                                    class: "ant-rate-star-second",
                                    onmouseenter: {
                                        let index = index;
                                        move |_| handle_star_hover(index, false)
                                    },
                                    onclick: {
                                        let index = index;
                                        move |_| handle_star_click(index, false)
                                    },
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// 创建评分组件的宏
#[macro_export]
macro_rules! rate {
    () => {
        Rate {
            count: 5,
            default_value: 0.0
        }
    };
    ($($prop:ident: $value:expr),* $(,)?) => {
        Rate {
            $($prop: $value,)*
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_size() {
        assert_eq!(RateSize::Small.to_string(), "small");
        assert_eq!(RateSize::Middle.to_string(), "middle");
        assert_eq!(RateSize::Large.to_string(), "large");
    }

    #[test]
    fn test_rate_defaults() {
        assert_eq!(RateSize::default(), RateSize::Middle);
    }

    #[test]
    fn test_rate_value_calculation() {
        // 测试星级值计算逻辑
        let get_star_state =
            |display_value: f64, index: usize, allow_half: bool| -> (bool, bool, bool) {
                let star_value = index as f64 + 1.0;
                let half_star_value = index as f64 + 0.5;

                let is_full = display_value >= star_value;
                let is_half =
                    allow_half && display_value >= half_star_value && display_value < star_value;
                let is_zero = display_value < half_star_value;

                (is_full, is_half, is_zero)
            };

        // 测试全星
        let (is_full, is_half, is_zero) = get_star_state(3.0, 2, true);
        assert!(is_full);
        assert!(!is_half);
        assert!(!is_zero);

        // 测试半星
        let (is_full, is_half, is_zero) = get_star_state(2.5, 2, true);
        assert!(!is_full);
        assert!(is_half);
        assert!(!is_zero);

        // 测试空星
        let (is_full, is_half, is_zero) = get_star_state(1.0, 2, true);
        assert!(!is_full);
        assert!(!is_half);
        assert!(is_zero);
    }
}

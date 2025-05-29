//! Radio 单选框组件
//!
//! 单选框。
//!
//! ## 何时使用
//!
//! - 用于在多个备选项中选中单个状态。
//! - 和 Select 的区别是，Radio 所有选项默认可见，方便用户在比较中选择，因此选项不宜过多。

use dioxus::prelude::*;

const RADIO_STYLE: &str = include_str!("./style.css");

#[derive(Clone, PartialEq)]
pub enum RadioSize {
    Small,
    Middle,
    Large,
}

impl Default for RadioSize {
    fn default() -> Self {
        RadioSize::Middle
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct RadioProps {
    /// 指定当前是否选中
    #[props(default = false)]
    pub checked: bool,
    /// 初始是否选中
    #[props(default = false)]
    pub default_checked: bool,
    /// 禁用 Radio
    #[props(default = false)]
    pub disabled: bool,
    /// 根据 value 进行比较，判断是否选中
    pub value: Option<String>,
    /// 变化时回调函数
    pub on_change: Option<EventHandler<bool>>,
    /// 自定义 CSS 类名
    #[props(default = "".to_string())]
    pub class: String,
    /// 自定义样式
    pub style: Option<String>,
    /// 子元素
    pub children: Element,
}

#[component]
pub fn Radio(props: RadioProps) -> Element {
    let mut checked = use_signal(|| props.default_checked);

    // 如果传入了 checked 属性，使用受控模式
    let is_checked = if props.checked != props.default_checked {
        props.checked
    } else {
        checked.read().clone()
    };

    let class_name = {
        let mut classes = vec!["ant-radio-wrapper"];

        if props.disabled {
            classes.push("ant-radio-wrapper-disabled");
        }

        if is_checked {
            classes.push("ant-radio-wrapper-checked");
        }

        if !props.class.is_empty() {
            classes.push(&props.class);
        }

        classes.join(" ")
    };

    let handle_change = move |_| {
        if props.disabled {
            return;
        }

        let new_checked = !is_checked;
        checked.set(new_checked);

        if let Some(on_change) = &props.on_change {
            on_change.call(new_checked);
        }
    };

    rsx! {
        style { {RADIO_STYLE} }
        label {
            class: "{class_name}",
            style: props.style.as_deref().unwrap_or(""),
            onclick: handle_change,
            span {
                class: if is_checked { "ant-radio ant-radio-checked" } else { "ant-radio" },
                input {
                    r#type: "radio",
                    class: "ant-radio-input",
                    checked: is_checked,
                    disabled: props.disabled,
                    value: props.value.as_deref().unwrap_or(""),
                }
                span {
                    class: "ant-radio-inner"
                }
            }
            span {
                class: "ant-radio-label",
                {props.children}
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct RadioGroupProps {
    /// 默认选中的值
    pub default_value: Option<String>,
    /// 指定选中的值
    pub value: Option<String>,
    /// 选项变化时的回调函数
    pub on_change: Option<EventHandler<String>>,
    /// 禁用所有选项
    #[props(default = false)]
    pub disabled: bool,
    /// 大小，只对按钮样式生效
    #[props(default = RadioSize::Middle)]
    pub size: RadioSize,
    /// RadioGroup 下所有 input[type="radio"] 的 name 属性
    pub name: Option<String>,
    /// 以配置形式设置子元素
    pub options: Option<Vec<RadioOption>>,
    /// 自定义 CSS 类名
    #[props(default = "".to_string())]
    pub class: String,
    /// 自定义样式
    pub style: Option<String>,
    /// 子元素
    pub children: Element,
}

#[derive(Clone, PartialEq)]
pub struct RadioOption {
    pub label: String,
    pub value: String,
    pub disabled: Option<bool>,
}

#[component]
pub fn RadioGroup(props: RadioGroupProps) -> Element {
    let mut selected_value = use_signal(|| props.default_value.clone().unwrap_or_default());

    // 如果传入了 value 属性，使用受控模式
    let current_value = if let Some(ref value) = props.value {
        value.clone()
    } else {
        selected_value.read().clone()
    };

    let class_name = {
        let mut classes = vec!["ant-radio-group"];

        match props.size {
            RadioSize::Small => classes.push("ant-radio-group-small"),
            RadioSize::Large => classes.push("ant-radio-group-large"),
            _ => {}
        }

        if !props.class.is_empty() {
            classes.push(&props.class);
        }

        classes.join(" ")
    };

    let handle_change = move |value: String| {
        selected_value.set(value.clone());

        if let Some(on_change) = &props.on_change {
            on_change.call(value);
        }
    };

    rsx! {
        style { {RADIO_STYLE} }
        div {
            class: class_name,
            style: props.style.as_deref().unwrap_or(""),

            // 如果提供了 options，渲染选项
            if let Some(ref options) = props.options {
                for option in options {
                    RadioGroupItem {
                        key: "{option.value}",
                        label: option.label.clone(),
                        value: option.value.clone(),
                        checked: current_value == option.value,
                        disabled: props.disabled || option.disabled.unwrap_or(false),
                        name: props.name.clone(),
                        on_change: handle_change.clone(),
                    }
                }
            } else {
                // 否则渲染子元素
                {props.children}
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct RadioGroupItemProps {
    label: String,
    value: String,
    checked: bool,
    disabled: bool,
    name: Option<String>,
    on_change: EventHandler<String>,
}

#[component]
fn RadioGroupItem(props: RadioGroupItemProps) -> Element {
    let handle_click = {
        let value = props.value.clone();
        let on_change = props.on_change.clone();
        move |_| {
            if !props.disabled {
                on_change.call(value.clone());
            }
        }
    };

    let class_name = {
        let mut classes = vec!["ant-radio-wrapper"];

        if props.disabled {
            classes.push("ant-radio-wrapper-disabled");
        }

        if props.checked {
            classes.push("ant-radio-wrapper-checked");
        }

        classes.join(" ")
    };

    rsx! {
        label {
            class: "{class_name}",
            onclick: handle_click,
            span {
                class: if props.checked { "ant-radio ant-radio-checked" } else { "ant-radio" },
                input {
                    r#type: "radio",
                    class: "ant-radio-input",
                    checked: props.checked,
                    disabled: props.disabled,
                    value: "{props.value}",
                    name: props.name.as_deref().unwrap_or(""),
                }
                span {
                    class: "ant-radio-inner"
                }
            }
            span {
                class: "ant-radio-label",
                "{props.label}"
            }
        }
    }
}

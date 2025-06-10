//! Radio 单选框组件
//!
//! 单选框。
//!
//! ## 何时使用
//!
//! - 用于在多个备选项中选中单个状态。
//! - 和 Select 的区别是，Radio 所有选项默认可见，方便用户在比较中选择，因此选项不宜过多。

use dioxus::prelude::*;

mod styles;
use styles::{use_radio_style, RadioGroupStyleGenerator, RadioStyleGenerator};
pub use styles::{RadioButtonStyle, RadioSize};

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

    // 确保样式已注入
    use_radio_style();

    // 如果传入了 checked 属性，使用受控模式
    let is_checked = if props.checked != props.default_checked {
        props.checked
    } else {
        checked.read().clone()
    };

    // 使用RadioStyleGenerator生成样式
    let style_gen = RadioStyleGenerator::new()
        .with_checked(is_checked)
        .with_disabled(props.disabled);

    // 获取生成的样式字符串
    let mut radio_style = style_gen.generate();

    // 添加自定义样式
    if let Some(ref style) = props.style {
        radio_style.push_str(" ");
        radio_style.push_str(style);
    }

    // 从生成的字符串中提取radio内部使用的类名
    let parts: Vec<&str> = radio_style.split("data-radio-inner-class").collect();
    let wrapper_style = parts[0].trim();
    let inner_class = if parts.len() > 1 {
        parts[1].trim()
    } else {
        "ant-radio"
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
        label {
            style: wrapper_style,
            onclick: handle_change,
            span {
                class: inner_class,
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
    /// 是否使用按钮样式
    #[props(default = false)]
    pub button_style: bool,
    /// 按钮样式，只对按钮样式生效
    #[props(default = RadioButtonStyle::Outline)]
    pub button_type: RadioButtonStyle,
    /// 是否垂直排列
    #[props(default = false)]
    pub vertical: bool,
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

    // 确保样式已注入
    use_radio_style();

    // 如果传入了 value 属性，使用受控模式
    let current_value = if let Some(ref value) = props.value {
        value.clone()
    } else {
        selected_value.read().clone()
    };

    // 使用RadioGroupStyleGenerator生成样式
    let style_gen = RadioGroupStyleGenerator::new()
        .with_size(props.size.clone())
        .with_button_style(props.button_style)
        .with_vertical(props.vertical);

    // 获取生成的样式字符串
    let mut group_style = style_gen.generate();

    // 添加自定义样式
    if let Some(ref style) = props.style {
        group_style.push_str(" ");
        group_style.push_str(style);
    }

    let handle_change = move |value: String| {
        selected_value.set(value.clone());

        if let Some(on_change) = &props.on_change {
            on_change.call(value);
        }
    };

    rsx! {
        div {
            style: group_style,

            // 如果提供了 options，渲染选项
            if let Some(ref options) = props.options {
                for option in options {
                    RadioGroupItem {
                        key: "{option.value}",
                        label: option.label.clone(),
                        value: option.value.clone(),
                        checked: current_value == option.value,
                        disabled: props.disabled || option.disabled.unwrap_or(false),
                        is_button: props.button_style,
                        button_type: props.button_type,
                        size: props.size.clone(),
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
    is_button: bool,
    button_type: RadioButtonStyle,
    size: RadioSize,
    name: Option<String>,
    on_change: EventHandler<String>,
}

#[component]
fn RadioGroupItem(props: RadioGroupItemProps) -> Element {
    // 使用RadioStyleGenerator生成样式
    let style_gen = RadioStyleGenerator::new()
        .with_checked(props.checked)
        .with_disabled(props.disabled)
        .with_size(props.size.clone())
        .with_button(props.is_button)
        .with_button_style(props.button_type);

    // 获取生成的样式字符串
    let radio_style = style_gen.generate();

    // 从生成的字符串中提取radio内部使用的类名
    let parts: Vec<&str> = radio_style.split("data-radio-inner-class").collect();
    let wrapper_style = parts[0].trim();
    let inner_class = if parts.len() > 1 {
        parts[1].trim()
    } else {
        if props.is_button {
            "ant-radio-button"
        } else {
            "ant-radio"
        }
    };

    let handle_click = {
        let value = props.value.clone();
        let on_change = props.on_change.clone();
        move |_| {
            if !props.disabled {
                on_change.call(value.clone());
            }
        }
    };

    rsx! {
        label {
            style: wrapper_style,
            onclick: handle_click,
            span {
                class: inner_class,
                input {
                    r#type: "radio",
                    class: if props.is_button { "ant-radio-button-input" } else { "ant-radio-input" },
                    name: props.name.clone().unwrap_or_default(),
                    checked: props.checked,
                    disabled: props.disabled,
                    value: props.value.clone(),
                }
                span {
                    class: if props.is_button { "ant-radio-button-inner" } else { "ant-radio-inner" }
                }
            }
            span {
                class: if props.is_button { "ant-radio-button-label" } else { "ant-radio-label" },
                {props.label.clone()}
            }
        }
    }
}

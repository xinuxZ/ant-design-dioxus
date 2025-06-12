//! Input 输入框组件
//!
//! 通过鼠标或键盘输入内容，是最基础的表单域的包装。
//!
//! ## 何时使用
//!
//! - 需要用户输入表单域内容时。
//! - 提供组合型输入框，带搜索的输入框，还可以进行大小选择。
//!
//! ## 代码演示
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::{Input, InputSize};
//!
//! #[component]
//! fn App() -> Element {
//!     let mut value = use_signal(String::new);
//!
//!     rsx! {
//!         Input {
//!             value: value(),
//!             placeholder: "请输入内容",
//!             on_change: move |v| value.set(v)
//!         }
//!     }
//! }
//! ```

mod styles;

use self::styles::{
    generate_input_style, InputSize as StyleInputSize, InputStatus as StyleInputStatus,
};
use css_in_rust::css;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

/// Input 组件尺寸
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputSize {
    /// 大尺寸
    Large,
    /// 中等尺寸（默认）
    Middle,
    /// 小尺寸
    Small,
}

impl Default for InputSize {
    fn default() -> Self {
        Self::Middle
    }
}

impl From<InputSize> for StyleInputSize {
    fn from(size: InputSize) -> Self {
        match size {
            InputSize::Large => StyleInputSize::Large,
            InputSize::Middle => StyleInputSize::Middle,
            InputSize::Small => StyleInputSize::Small,
        }
    }
}

/// Input 组件状态
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputStatus {
    /// 正常状态
    Normal,
    /// 错误状态
    Error,
    /// 警告状态
    Warning,
}

impl Default for InputStatus {
    fn default() -> Self {
        Self::Normal
    }
}

impl From<InputStatus> for StyleInputStatus {
    fn from(status: InputStatus) -> Self {
        match status {
            InputStatus::Normal => StyleInputStatus::Default,
            InputStatus::Error => StyleInputStatus::Error,
            InputStatus::Warning => StyleInputStatus::Warning,
        }
    }
}

/// Input 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct InputProps {
    /// 输入框内容
    #[props(default)]
    pub value: String,

    /// 输入框占位符
    #[props(default)]
    pub placeholder: Option<String>,

    /// 输入框尺寸
    #[props(default)]
    pub size: InputSize,

    /// 输入框状态
    #[props(default)]
    pub status: InputStatus,

    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,

    /// 是否只读
    #[props(default = false)]
    pub readonly: bool,

    /// 是否显示清除图标
    #[props(default = false)]
    pub allow_clear: bool,

    /// 是否有边框
    #[props(default = true)]
    pub bordered: bool,

    /// 最大长度
    #[props(default)]
    pub max_length: Option<usize>,

    /// 是否显示字数统计
    #[props(default = false)]
    pub show_count: bool,

    /// 输入框前缀
    #[props(default)]
    pub prefix: Option<Element>,

    /// 输入框后缀
    #[props(default)]
    pub suffix: Option<Element>,

    /// 带标签的 input，设置前置标签
    #[props(default)]
    pub addon_before: Option<Element>,

    /// 带标签的 input，设置后置标签
    #[props(default)]
    pub addon_after: Option<Element>,

    /// 输入框类型
    #[props(default = "text".to_string())]
    pub input_type: String,

    /// 自定义样式类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,

    /// 输入时的回调
    #[props(default)]
    pub on_change: Option<EventHandler<String>>,

    /// 按下回车的回调
    #[props(default)]
    pub on_press_enter: Option<EventHandler<KeyboardEvent>>,

    /// 获得焦点时的回调
    #[props(default)]
    pub on_focus: Option<EventHandler<FocusEvent>>,

    /// 失去焦点时的回调
    #[props(default)]
    pub on_blur: Option<EventHandler<FocusEvent>>,

    /// 清除按钮的回调
    #[props(default)]
    pub on_clear: Option<EventHandler<()>>,
}

/// Input 输入框组件
///
/// 通过鼠标或键盘输入内容，是最基础的表单域的包装
#[component]
pub fn Input(props: InputProps) -> Element {
    let mut internal_value = use_signal(|| props.value.clone());
    let mut is_focused = use_signal(|| false);

    // 同步外部 value 到内部状态
    let props_value = props.value.clone();
    use_effect(move || {
        internal_value.set(props_value.clone());
    });

    let handle_input = move |evt: FormEvent| {
        let value = evt.value();
        internal_value.set(value.clone());
        if let Some(on_change) = &props.on_change {
            on_change.call(value);
        }
    };

    let handle_keydown = move |evt: KeyboardEvent| {
        if evt.key() == Key::Enter {
            if let Some(on_press_enter) = &props.on_press_enter {
                on_press_enter.call(evt);
            }
        }
    };

    let handle_focus = move |evt: FocusEvent| {
        is_focused.set(true);
        if let Some(on_focus) = &props.on_focus {
            on_focus.call(evt);
        }
    };

    let handle_blur = move |evt: FocusEvent| {
        is_focused.set(false);
        if let Some(on_blur) = &props.on_blur {
            on_blur.call(evt);
        }
    };

    let handle_clear = move |_| {
        internal_value.set(String::new());
        if let Some(on_change) = &props.on_change {
            on_change.call(String::new());
        }
        if let Some(on_clear) = &props.on_clear {
            on_clear.call(());
        }
    };

    // 生成 CSS-in-Rust 样式
    let input_style = generate_input_style(
        props.size.into(),
        props.status.into(),
        props.disabled,
        props.bordered,
        props.allow_clear,
        props.prefix.is_some(),
        props.suffix.is_some(),
        props.show_count,
        props.addon_before.is_some(),
        props.addon_after.is_some(),
        is_focused(),
    );

    // 获取适当的 CSS 类名
    let input_class = get_input_css_class(&props, is_focused());
    let wrapper_class = get_wrapper_css_class(&props, is_focused());
    let group_class = get_group_css_class(&props);

    // 计算字符数
    let char_count = internal_value().chars().count();
    let max_length = props.max_length.unwrap_or(usize::MAX);

    // 如果有 addon，使用 group 包装
    if props.addon_before.is_some() || props.addon_after.is_some() {
        rsx! {
            style { {input_style} }

            div {
                class: group_class,
                style: props.style.clone().unwrap_or_default(),

                div {
                    class: "ant-input-group",

                    if let Some(addon_before) = props.addon_before.clone() {
                        div {
                            class: "ant-input-group-addon",
                            {addon_before}
                        }
                    }

                    if !wrapper_class.is_empty() {
                        div {
                            class: wrapper_class,

                            if let Some(prefix) = props.prefix.clone() {
                                span {
                                    class: "ant-input-prefix",
                                    {prefix}
                                }
                            }

                            input {
                                class: input_class,
                                r#type: props.input_type.clone(),
                                value: internal_value(),
                                placeholder: props.placeholder.clone(),
                                disabled: props.disabled,
                                readonly: props.readonly,
                                maxlength: if max_length != usize::MAX { max_length.to_string() } else { String::new() },
                                oninput: handle_input,
                                onkeydown: handle_keydown,
                                onfocus: handle_focus,
                                onblur: handle_blur
                            }

                            if props.allow_clear && !internal_value().is_empty() && !props.disabled {
                                span {
                                    class: "ant-input-clear-icon",
                                    onclick: handle_clear,
                                    "×"
                                }
                            }

                            if let Some(suffix) = props.suffix.clone() {
                                span {
                                    class: "ant-input-suffix",
                                    {suffix}
                                }
                            }
                        }
                    } else {
                        input {
                            class: input_class,
                            r#type: props.input_type.clone(),
                            value: internal_value(),
                            placeholder: props.placeholder.clone(),
                            disabled: props.disabled,
                            readonly: props.readonly,
                            maxlength: if max_length != usize::MAX { max_length.to_string() } else { String::new() },
                            oninput: handle_input,
                            onkeydown: handle_keydown,
                            onfocus: handle_focus,
                            onblur: handle_blur
                        }
                    }

                    if let Some(addon_after) = props.addon_after.clone() {
                        div {
                            class: "ant-input-group-addon",
                            {addon_after}
                        }
                    }
                }

                if props.show_count {
                    div {
                        class: "ant-input-show-count-suffix",

                        {
                            if max_length != usize::MAX {
                                format!("{} / {}", char_count, max_length)
                            } else {
                                char_count.to_string()
                            }
                        }
                    }
                }
            }
        }
    } else if !wrapper_class.is_empty() {
        // 有前缀或后缀的情况
        rsx! {
            style { {input_style} }

            div {
                class: wrapper_class,
                style: props.style.clone().unwrap_or_default(),

                if let Some(prefix) = props.prefix.clone() {
                    span {
                        class: "ant-input-prefix",
                        {prefix}
                    }
                }

                input {
                    class: input_class,
                    r#type: props.input_type.clone(),
                    value: internal_value(),
                    placeholder: props.placeholder.clone(),
                    disabled: props.disabled,
                    readonly: props.readonly,
                    maxlength: if max_length != usize::MAX { max_length.to_string() } else { String::new() },
                    oninput: handle_input,
                    onkeydown: handle_keydown,
                    onfocus: handle_focus,
                    onblur: handle_blur
                }

                if props.allow_clear && !internal_value().is_empty() && !props.disabled {
                    span {
                        class: "ant-input-clear-icon",
                        onclick: handle_clear,
                        "×"
                    }
                }

                if let Some(suffix) = props.suffix.clone() {
                    span {
                        class: "ant-input-suffix",
                        {suffix}
                    }
                }

                if props.show_count {
                    span {
                        class: "ant-input-show-count-suffix",

                        {
                            if max_length != usize::MAX {
                                format!("{} / {}", char_count, max_length)
                            } else {
                                char_count.to_string()
                            }
                        }
                    }
                }
            }
        }
    } else {
        // 基本输入框
        rsx! {
            style { {input_style} }

            input {
                class: input_class,
                style: props.style.clone().unwrap_or_default(),
                r#type: props.input_type.clone(),
                value: internal_value(),
                placeholder: props.placeholder.clone(),
                disabled: props.disabled,
                readonly: props.readonly,
                maxlength: if max_length != usize::MAX { max_length.to_string() } else { String::new() },
                oninput: handle_input,
                onkeydown: handle_keydown,
                onfocus: handle_focus,
                onblur: handle_blur
            }

            if props.show_count {
                div {
                    class: "ant-input-show-count-suffix",
                    style: "margin-top: 4px;",

                    {
                        if max_length != usize::MAX {
                            format!("{} / {}", char_count, max_length)
                        } else {
                            char_count.to_string()
                        }
                    }
                }
            }
        }
    }
}

/// 获取输入框的 CSS 类名
fn get_input_css_class(props: &InputProps, is_focused: bool) -> String {
    let mut classes = vec!["ant-input"];

    // 添加尺寸相关的类名
    match props.size {
        InputSize::Large => classes.push("ant-input-lg"),
        InputSize::Small => classes.push("ant-input-sm"),
        _ => {}
    }

    // 添加状态相关的类名
    match props.status {
        InputStatus::Error => classes.push("ant-input-status-error"),
        InputStatus::Warning => classes.push("ant-input-status-warning"),
        _ => {}
    }

    // 添加禁用状态类名
    if props.disabled {
        classes.push("ant-input-disabled");
    }

    // 添加无边框类名
    if !props.bordered {
        classes.push("ant-input-borderless");
    }

    // 添加焦点类名
    if is_focused {
        classes.push("ant-input-focused");
    }

    // 添加自定义类名
    if let Some(class) = &props.class {
        classes.push(class);
    }

    classes.join(" ")
}

/// 获取输入框包装器的 CSS 类名
fn get_wrapper_css_class(props: &InputProps, is_focused: bool) -> String {
    // 如果没有前缀、后缀或清除按钮，则不需要包装器
    if props.prefix.is_none() && props.suffix.is_none() && !props.allow_clear && !props.show_count {
        return String::new();
    }

    let mut classes = vec!["ant-input-affix-wrapper"];

    // 添加尺寸相关的类名
    match props.size {
        InputSize::Large => classes.push("ant-input-lg"),
        InputSize::Small => classes.push("ant-input-sm"),
        _ => {}
    }

    // 添加状态相关的类名
    match props.status {
        InputStatus::Error => classes.push("ant-input-status-error"),
        InputStatus::Warning => classes.push("ant-input-status-warning"),
        _ => {}
    }

    // 添加禁用状态类名
    if props.disabled {
        classes.push("ant-input-affix-wrapper-disabled");
    }

    // 添加无边框类名
    if !props.bordered {
        classes.push("ant-input-affix-wrapper-borderless");
    }

    // 添加焦点类名
    if is_focused {
        classes.push("ant-input-affix-wrapper-focused");
    }

    classes.join(" ")
}

/// 获取输入框组的 CSS 类名
fn get_group_css_class(props: &InputProps) -> String {
    let mut classes = vec!["ant-input-group-wrapper"];

    // 添加尺寸相关的类名
    match props.size {
        InputSize::Large => classes.push("ant-input-group-wrapper-lg"),
        InputSize::Small => classes.push("ant-input-group-wrapper-sm"),
        _ => {}
    }

    // 添加自定义类名
    if let Some(class) = &props.class {
        classes.push(class);
    }

    classes.join(" ")
}

// 组件已通过#[component]宏自动导出
// 无需重新导出

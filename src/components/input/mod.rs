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

use css_in_rust_macros::css;
use dioxus::prelude::*;

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

impl InputSize {
    /// 获取尺寸对应的CSS样式
    pub fn to_css(&self) -> String {
        match self {
            InputSize::Large => {
                css!("height: 40px; padding: 6px 11px; font-size: var(--ant-font-size-lg);")
            }
            InputSize::Middle => String::new(),
            InputSize::Small => {
                css!("height: 24px; padding: 0px 7px; font-size: var(--ant-font-size-sm);")
            }
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

impl InputStatus {
    /// 获取状态对应的CSS样式
    pub fn to_css(&self) -> String {
        match self {
            InputStatus::Normal => String::new(),
            InputStatus::Error => css!(
                r#"
                border-color: var(--ant-error-color);

                &:hover {
                    border-color: #ff7875;
                }

                &:focus {
                    border-color: #ff7875;
                    box-shadow: 0 0 0 2px rgba(255, 77, 79, 0.2);
                }
            "#
            ),
            InputStatus::Warning => css!(
                r#"
                border-color: var(--ant-warning-color);

                &:hover {
                    border-color: #ffc53d;
                }

                &:focus {
                    border-color: #ffc53d;
                    box-shadow: 0 0 0 2px rgba(255, 197, 61, 0.2);
                }
            "#
            ),
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
    use_effect(move || {
        internal_value.set(props.value.clone());
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

    let input_class = get_input_css_class(&props, is_focused());

    let wrapper_class = get_wrapper_css_class(&props, is_focused());

    let group_class = get_group_css_class(&props);

    // 计算字符数
    let char_count = internal_value().chars().count();
    let max_length = props.max_length.unwrap_or(usize::MAX);

    // 如果有 addon，使用 group 包装
    if props.addon_before.is_some() || props.addon_after.is_some() {
        rsx! {
            div {
                class: group_class,
                style: props.style,

                div {
                        class: "{}",
                        css!(r#"
                            position: relative;
                            display: table;
                            width: 100%;
                            border-collapse: separate;
                            border-spacing: 0;
                        "#),

                    if let Some(addon_before) = props.addon_before {
                        div {
                            class: "{}",
                            css!(r#"
                                position: relative;
                                padding: 0 11px;
                                color: var(--ant-text-color);
                                font-weight: normal;
                                font-size: var(--ant-font-size-base);
                                text-align: center;
                                background-color: var(--ant-bg-color-container);
                                border: 1px solid var(--ant-border-color);
                                border-radius: var(--ant-border-radius);
                                transition: all 0.2s;
                                display: table-cell;
                                width: 1px;
                                white-space: nowrap;
                                vertical-align: middle;
                            "#),
                            {addon_before}
                        }
                    }

                    if !wrapper_class.is_empty() {
                        div {
                            class: wrapper_class,

                            if let Some(prefix) = props.prefix {
                                span {
                                    class: "{}",
                                    css!(r#"
                                        display: flex;
                                        flex: none;
                                        align-items: center;
                                        margin-inline-end: 4px;
                                        color: var(--ant-text-color-tertiary);
                                    "#),
                                    {prefix}
                                }
                            }

                            input {
                                class: input_class,
                                r#type: props.input_type,
                                value: internal_value(),
                                placeholder: props.placeholder,
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
                                    class: "{}",
                                    css!(r#"
                                        position: absolute;
                                        top: 50%;
                                        inset-inline-end: 8px;
                                        z-index: 1;
                                        display: inline-block;
                                        width: 12px;
                                        height: 12px;
                                        margin-top: -6px;
                                        color: var(--ant-text-color-quaternary);
                                        font-size: 12px;
                                        font-style: normal;
                                        line-height: 1;
                                        text-align: center;
                                        text-transform: none;
                                        background: var(--ant-bg-color-container);
                                        cursor: pointer;
                                        opacity: 0;
                                        transition: color 0.2s ease, opacity 0.15s ease;
                                        text-rendering: auto;

                                        &:hover {
                                            color: var(--ant-text-color-tertiary);
                                        }
                                    "#),
                                    onclick: handle_clear,
                                    "×"
                                }
                            }

                            if let Some(suffix) = props.suffix {
                                span {
                                    class: "{}",
                                    css!(r#"
                                        display: flex;
                                        flex: none;
                                        align-items: center;
                                        margin-inline-start: 4px;
                                        color: var(--ant-text-color-tertiary);
                                    "#),
                                    {suffix}
                                }
                            }
                        }
                    } else {
                        input {
                            class: input_class,
                            r#type: props.input_type,
                            value: internal_value(),
                            placeholder: props.placeholder,
                            disabled: props.disabled,
                            readonly: props.readonly,
                            maxlength: if max_length != usize::MAX { max_length.to_string() } else { String::new() },
                            oninput: handle_input,
                            onkeydown: handle_keydown,
                            onfocus: handle_focus,
                            onblur: handle_blur
                        }
                    }

                    if let Some(addon_after) = props.addon_after {
                        div {
                            class: "{}",
                            css!(r#"
                                position: relative;
                                padding: 0 11px;
                                color: var(--ant-text-color);
                                font-weight: normal;
                                font-size: var(--ant-font-size-base);
                                text-align: center;
                                background-color: var(--ant-bg-color-container);
                                border: 1px solid var(--ant-border-color);
                                border-radius: var(--ant-border-radius);
                                transition: all 0.2s;
                                display: table-cell;
                                width: 1px;
                                white-space: nowrap;
                                vertical-align: middle;
                            "#),
                            {addon_after}
                        }
                    }
                }

                if props.show_count {
                    div {
                        class: "ant-input-show-count-suffix",
                        style: "margin-top: 4px; color: rgba(0, 0, 0, 0.45); font-size: 12px;",

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
            div {
                div {
                    class: wrapper_class,
                    style: props.style,

                    if let Some(prefix) = props.prefix {
                        span {
                            class: "ant-input-prefix",
                            {prefix}
                        }
                    }

                    input {
                        class: input_class,
                        r#type: props.input_type,
                        value: internal_value(),
                        placeholder: props.placeholder,
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

                    if let Some(suffix) = props.suffix {
                        span {
                            class: "ant-input-suffix",
                            {suffix}
                        }
                    }
                }

                if props.show_count {
                    div {
                        class: "ant-input-show-count-suffix",
                        style: "margin-top: 4px; color: rgba(0, 0, 0, 0.45); font-size: 12px;",

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
        // 基础输入框
        rsx! {
            div {
                input {
                    class: format!("{} {}", input_class, props.class.as_deref().unwrap_or("")),
                    style: props.style,
                    r#type: props.input_type,
                    value: internal_value(),
                    placeholder: props.placeholder,
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
                        style: "margin-top: 4px; color: rgba(0, 0, 0, 0.45); font-size: 12px;",

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
}

/// 获取输入框的CSS样式类名
fn get_input_css_class(props: &InputProps, is_focused: bool) -> String {
    // 基础输入框样式
    let base_class = css!(
        r#"
        box-sizing: border-box;
        margin: 0;
        padding: 4px 11px;
        color: var(--ant-text-color);
        font-size: var(--ant-font-size-base);
        line-height: var(--ant-line-height-base);
        list-style: none;
        font-family: inherit;
        position: relative;
        display: inline-block;
        width: 100%;
        min-width: 0;
        background-color: var(--ant-bg-color-container);
        background-image: none;
        border: 1px solid var(--ant-border-color);
        border-radius: var(--ant-border-radius);
        transition: all 0.2s;

        &:hover {
            border-color: var(--ant-primary-color-hover);
            border-inline-end-width: 1px;
        }

        &:focus {
            border-color: var(--ant-primary-color-hover);
            box-shadow: 0 0 0 2px rgba(5, 145, 255, 0.1);
            border-inline-end-width: 1px;
            outline: 0;
        }

        &::placeholder {
            color: var(--ant-text-color-quaternary);
            user-select: none;
        }
    "#
    );

    // 尺寸样式
    let size_class = props.size.to_css();

    // 状态样式
    let status_class = props.status.to_css();

    // 禁用状态
    let disabled_class = if props.disabled {
        css!(
            r#"
            color: var(--ant-text-color-disabled);
            background-color: var(--ant-bg-color-disabled);
            border-color: var(--ant-border-color);
            box-shadow: none;
            cursor: not-allowed;
            opacity: 1;

            &:hover {
                border-color: var(--ant-border-color);
            }
        "#
        )
    } else {
        String::new()
    };

    // 无边框样式
    let borderless_class = if !props.bordered {
        css!(
            r#"
            background-color: transparent;
            border: none;
            box-shadow: none;
        "#
        )
    } else {
        String::new()
    };

    // 聚焦状态
    let focused_class = if is_focused {
        css!(
            r#"
            border-color: var(--ant-primary-color-hover);
            box-shadow: 0 0 0 2px rgba(5, 145, 255, 0.1);
            border-inline-end-width: 1px;
            outline: 0;
        "#
        )
    } else {
        String::new()
    };

    // 组合所有样式
    let mut combined_classes = vec![
        base_class,
        size_class,
        status_class,
        disabled_class,
        borderless_class,
        focused_class,
    ];

    combined_classes
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

/// 获取包装器的CSS样式类名
fn get_wrapper_css_class(props: &InputProps, is_focused: bool) -> String {
    if props.prefix.is_none() && props.suffix.is_none() && !props.allow_clear {
        return String::new();
    }

    // 基础包装器样式
    let base_class = css!(
        r#"
        box-sizing: border-box;
        margin: 0;
        padding: 0;
        color: var(--ant-text-color);
        font-size: var(--ant-font-size-base);
        line-height: var(--ant-line-height-base);
        list-style: none;
        font-family: inherit;
        position: relative;
        display: inline-block;
        width: 100%;
        min-width: 0;
        padding: 4px 11px;
        background-color: var(--ant-bg-color-container);
        background-image: none;
        border: 1px solid var(--ant-border-color);
        border-radius: var(--ant-border-radius);
        transition: all 0.2s;

        &:hover {
            border-color: var(--ant-primary-color-hover);
            border-inline-end-width: 1px;
        }

        &:focus-within {
            border-color: var(--ant-primary-color-hover);
            box-shadow: 0 0 0 2px rgba(5, 145, 255, 0.1);
            border-inline-end-width: 1px;
            outline: 0;
        }
    "#
    );

    // 尺寸样式
    let size_class = props.size.to_css();

    // 状态样式
    let status_class = props.status.to_css();

    // 禁用状态
    let disabled_class = if props.disabled {
        css!(
            r#"
            color: var(--ant-text-color-disabled);
            background-color: var(--ant-bg-color-disabled);
            border-color: var(--ant-border-color);
            box-shadow: none;
            cursor: not-allowed;
            opacity: 1;

            &:hover {
                border-color: var(--ant-border-color);
            }
        "#
        )
    } else {
        String::new()
    };

    // 无边框样式
    let borderless_class = if !props.bordered {
        css!(
            r#"
            background-color: transparent;
            border: none;
            box-shadow: none;
        "#
        )
    } else {
        String::new()
    };

    // 聚焦状态
    let focused_class = if is_focused {
        css!(
            r#"
            border-color: var(--ant-primary-color-hover);
            box-shadow: 0 0 0 2px rgba(5, 145, 255, 0.1);
            border-inline-end-width: 1px;
            outline: 0;
        "#
        )
    } else {
        String::new()
    };

    // 组合所有样式
    let mut combined_classes = vec![
        base_class,
        size_class,
        status_class,
        disabled_class,
        borderless_class,
        focused_class,
    ];

    // 添加自定义类名
    if let Some(ref class) = props.class {
        combined_classes.push(class.clone());
    }

    combined_classes
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

/// 获取分组的CSS样式类名
fn get_group_css_class(props: &InputProps) -> String {
    if props.addon_before.is_none() && props.addon_after.is_none() {
        return String::new();
    }

    // 基础分组样式
    let base_class = css!(
        r#"
        box-sizing: border-box;
        margin: 0;
        padding: 0;
        color: var(--ant-text-color);
        font-size: var(--ant-font-size-base);
        line-height: var(--ant-line-height-base);
        list-style: none;
        font-family: inherit;
        position: relative;
        display: table;
        width: 100%;
        border-collapse: separate;
        border-spacing: 0;
    "#
    );

    // 尺寸样式
    let size_class = props.size.to_css();

    // 组合样式
    let mut combined_classes = vec![base_class, size_class];

    combined_classes
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

// 组件已通过#[component]宏自动导出
// 无需重新导出

//! Checkbox 复选框组件
//!
//! 复选框。
//!
//! ## 何时使用
//!
//! - 在一组可选项中进行多项选择时；
//! - 单独使用可以表示两种状态之间的切换，和 switch 类似。区别在于切换 switch 会直接触发状态改变，而 checkbox 一般用于状态标记，需要和提交操作配合。
//!
//! ## 代码演示
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::{Checkbox, CheckboxGroup};
//!
//! #[component]
//! fn App() -> Element {
//!     let mut checked = use_signal(|| false);
//!
//!     rsx! {
//!         Checkbox {
//!             checked: checked(),
//!             on_change: move |c| checked.set(c),
//!             "复选框"
//!         }
//!     }
//! }
//! ```

use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CheckboxSize {
    /// 大尺寸
    Large,
    /// 中等尺寸（默认）
    Middle,
    /// 小尺寸
    Small,
}

impl Default for CheckboxSize {
    fn default() -> Self {
        Self::Middle
    }
}

impl CheckboxSize {
    /// 获取尺寸对应的CSS类名
    pub fn to_class(&self) -> &'static str {
        match self {
            CheckboxSize::Large => "ant-checkbox-lg",
            CheckboxSize::Middle => "",
            CheckboxSize::Small => "ant-checkbox-sm",
        }
    }
}

/// Checkbox 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct CheckboxProps {
    /// 是否选中
    #[props(default = false)]
    pub checked: bool,

    /// 默认是否选中
    #[props(default = false)]
    pub default_checked: bool,

    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,

    /// 是否为不确定状态
    #[props(default = false)]
    pub indeterminate: bool,

    /// 复选框尺寸
    #[props(default)]
    pub size: CheckboxSize,

    /// 复选框的值
    #[props(default)]
    pub value: Option<String>,

    /// 自定义样式类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,

    /// 变化时的回调
    #[props(default)]
    pub on_change: Option<EventHandler<bool>>,

    /// 获得焦点时的回调
    #[props(default)]
    pub on_focus: Option<EventHandler<FocusEvent>>,

    /// 失去焦点时的回调
    #[props(default)]
    pub on_blur: Option<EventHandler<FocusEvent>>,

    /// 子元素（标签文本）
    children: Element,
}

/// Checkbox 复选框组件
///
/// 复选框
#[component]
pub fn Checkbox(props: CheckboxProps) -> Element {
    let mut internal_checked = use_signal(|| props.default_checked);

    // 使用外部传入的 checked 值，如果没有则使用内部状态
    let is_checked = props.checked || internal_checked();

    let handle_change = move |_| {
        if !props.disabled {
            let new_checked = !is_checked;
            internal_checked.set(new_checked);
            if let Some(on_change) = &props.on_change {
                on_change.call(new_checked);
            }
        }
    };

    let handle_focus = move |evt: FocusEvent| {
        if let Some(on_focus) = &props.on_focus {
            on_focus.call(evt);
        }
    };

    let handle_blur = move |evt: FocusEvent| {
        if let Some(on_blur) = &props.on_blur {
            on_blur.call(evt);
        }
    };

    let checkbox_class = {
        let mut classes = vec!["ant-checkbox-wrapper"];

        if props.disabled {
            classes.push("ant-checkbox-wrapper-disabled");
        }

        let size_class = props.size.to_class();
        if !size_class.is_empty() {
            classes.push(size_class);
        }

        if let Some(ref class) = props.class {
            classes.push(class);
        }

        classes.join(" ")
    };

    let inner_class = {
        let mut classes = vec!["ant-checkbox"];

        if is_checked {
            classes.push("ant-checkbox-checked");
        }

        if props.indeterminate {
            classes.push("ant-checkbox-indeterminate");
        }

        if props.disabled {
            classes.push("ant-checkbox-disabled");
        }

        classes.join(" ")
    };

    rsx! {
        label {
            class: checkbox_class,
            style: props.style,
            onclick: handle_change,

            span {
                class: inner_class,

                input {
                    r#type: "checkbox",
                    class: "ant-checkbox-input",
                    checked: is_checked,
                    disabled: props.disabled,
                    value: props.value.unwrap_or_default(),
                    onfocus: handle_focus,
                    onblur: handle_blur
                }

                span {
                    class: "ant-checkbox-inner"
                }
            }

            span {
                class: "ant-checkbox-label",
                {props.children}
            }
        }
    }
}

/// CheckboxGroup 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct CheckboxGroupProps {
    /// 当前选中的值
    #[props(default)]
    pub value: Vec<String>,

    /// 默认选中的值
    #[props(default)]
    pub default_value: Vec<String>,

    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,

    /// 复选框组的名称
    #[props(default)]
    pub name: Option<String>,

    /// 可选项数据源
    #[props(default)]
    pub options: Vec<CheckboxOption>,

    /// 复选框尺寸
    #[props(default)]
    pub size: CheckboxSize,

    /// 自定义样式类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,

    /// 变化时的回调
    #[props(default)]
    pub on_change: Option<EventHandler<Vec<String>>>,

    /// 子元素
    children: Element,
}

/// CheckboxGroup 复选框选项
#[derive(Debug, Clone, PartialEq)]
pub struct CheckboxOption {
    /// 选项标签
    pub label: String,
    /// 选项值
    pub value: String,
    /// 是否禁用
    pub disabled: bool,
}

impl CheckboxOption {
    /// 创建新的复选框选项
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            disabled: false,
        }
    }

    /// 设置禁用状态
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// CheckboxGroup 复选框组组件
///
/// 复选框组
#[component]
pub fn CheckboxGroup(props: CheckboxGroupProps) -> Element {
    let mut internal_value = use_signal(|| props.default_value.clone());

    // 使用外部传入的 value，如果没有则使用内部状态
    let current_value = if !props.value.is_empty() {
        props.value.clone()
    } else {
        internal_value()
    };

    let current_value01 = current_value.clone();
    let handle_checkbox_change = move |option_value: String, checked: bool| {
        if !props.disabled {
            let mut new_value = current_value01.clone();

            if checked {
                if !new_value.contains(&option_value) {
                    new_value.push(option_value);
                }
            } else {
                new_value.retain(|v| v != &option_value);
            }

            internal_value.set(new_value.clone());
            if let Some(on_change) = &props.on_change {
                on_change.call(new_value);
            }
        }
    };

    let group_class = {
        let mut classes = vec!["ant-checkbox-group"];

        if props.disabled {
            classes.push("ant-checkbox-group-disabled");
        }

        let size_class = props.size.to_class();
        if !size_class.is_empty() {
            classes.push(size_class);
        }

        if let Some(ref class) = props.class {
            classes.push(class);
        }

        classes.join(" ")
    };

    let value = handle_checkbox_change;
    rsx! {
        div {
            class: group_class,
            style: props.style,

            // 渲染选项
            if !props.options.is_empty() {
                for option in props.options.iter() {
                    Checkbox {
                        key: option.value.clone(),
                        checked: current_value.contains(&option.value),
                        disabled: props.disabled || option.disabled,
                        size: props.size,
                        value: option.value.clone(),
                        on_change: {
                            let option_value = option.value.clone();
                            let mut value_fn = value.clone();
                            move |checked| value_fn(option_value.clone(), checked)
                        },
                        label {
                            class: "ant-checkbox-wrapper",
                            key: option.value.clone(),

                            span {
                                class: "ant-checkbox-inner"
                            }
                        }
                    }
                }
            } else {
                // 渲染子元素
                {props.children}
            }
        }
    }
}

/// 全选复选框组件属性
#[derive(Props, Clone, PartialEq)]
pub struct CheckAllProps {
    /// 是否全选
    #[props(default = false)]
    pub checked: bool,

    /// 是否为不确定状态
    #[props(default = false)]
    pub indeterminate: bool,

    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,

    /// 复选框尺寸
    #[props(default)]
    pub size: CheckboxSize,

    /// 自定义样式类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,

    /// 变化时的回调
    #[props(default)]
    pub on_change: Option<EventHandler<bool>>,

    /// 子元素（标签文本）
    children: Element,
}

/// CheckAll 全选复选框组件
///
/// 全选复选框，通常用于表格或列表的全选功能
#[component]
pub fn CheckAll(props: CheckAllProps) -> Element {
    rsx! {
        Checkbox {
            checked: props.checked,
            indeterminate: props.indeterminate,
            disabled: props.disabled,
            size: props.size,
            class: props.class,
            style: props.style,
            on_change: props.on_change,

            {props.children}
        }
    }
}

// 组件已通过#[component]宏自动导出
// 无需重新导出

//! AutoComplete 自动完成组件
//!
//! 输入框自动完成功能。
//!
//! ## 何时使用
//!
//! - 需要一个输入框而不是选择器。
//! - 需要输入建议/辅助提示。

use crate::utils::class_names::conditional_class_names_array;
use dioxus::prelude::*;
use std::collections::HashMap;

/// 自动完成选项
#[derive(Debug, Clone, PartialEq)]
pub struct AutoCompleteOption {
    /// 选项值
    pub value: String,
    /// 显示文本
    pub label: String,
    /// 是否禁用
    pub disabled: bool,
    /// 自定义数据
    pub data: HashMap<String, String>,
}

impl AutoCompleteOption {
    pub fn new(value: &str, label: &str) -> Self {
        Self {
            value: value.to_string(),
            label: label.to_string(),
            disabled: false,
            data: HashMap::new(),
        }
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    pub fn with_data(mut self, key: &str, value: &str) -> Self {
        self.data.insert(key.to_string(), value.to_string());
        self
    }
}

/// 自动完成尺寸
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AutoCompleteSize {
    /// 小尺寸
    Small,
    /// 中等尺寸
    Middle,
    /// 大尺寸
    Large,
}

impl Default for AutoCompleteSize {
    fn default() -> Self {
        AutoCompleteSize::Middle
    }
}

/// 自动完成状态
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AutoCompleteStatus {
    /// 默认状态
    Default,
    /// 错误状态
    Error,
    /// 警告状态
    Warning,
}

impl Default for AutoCompleteStatus {
    fn default() -> Self {
        AutoCompleteStatus::Default
    }
}

/// AutoComplete 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct AutoCompleteProps {
    /// 当前值
    #[props(default = String::new())]
    pub value: String,
    /// 默认值
    #[props(default = String::new())]
    pub default_value: String,
    /// 自动完成的数据源
    #[props(default = Vec::new())]
    pub options: Vec<AutoCompleteOption>,
    /// 占位符
    #[props(default = String::new())]
    pub placeholder: String,
    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,
    /// 是否允许清除
    #[props(default = false)]
    pub allow_clear: bool,
    /// 是否自动获取焦点
    #[props(default = false)]
    pub auto_focus: bool,
    /// 是否默认高亮第一个选项
    #[props(default = true)]
    pub default_active_first_option: bool,
    /// 是否根据输入项进行筛选
    #[props(default = true)]
    pub filter_option: bool,
    /// 组件尺寸
    #[props(default = AutoCompleteSize::Middle)]
    pub size: AutoCompleteSize,
    /// 组件状态
    #[props(default = AutoCompleteStatus::Default)]
    pub status: AutoCompleteStatus,
    /// 下拉菜单的最大高度
    #[props(default = 256)]
    pub dropdown_max_height: u32,
    /// 值变化时的回调
    pub on_change: Option<EventHandler<String>>,
    /// 选择选项时的回调
    pub on_select: Option<EventHandler<AutoCompleteOption>>,
    /// 搜索时的回调
    pub on_search: Option<EventHandler<String>>,
    /// 获得焦点时的回调
    pub on_focus: Option<EventHandler<FocusEvent>>,
    /// 失去焦点时的回调
    pub on_blur: Option<EventHandler<FocusEvent>>,
    /// 按下回车键时的回调
    pub on_press_enter: Option<EventHandler<KeyboardEvent>>,
    /// 自定义样式类名
    #[props(default = String::new())]
    pub class: String,
    /// 自定义样式
    #[props(default = String::new())]
    pub style: String,
}

/// AutoComplete 自动完成组件
#[component]
pub fn AutoComplete(props: AutoCompleteProps) -> Element {
    // 克隆所有在闭包中使用的props以确保'static生命周期
    let options_static = props.options.clone();
    let default_value_static = props.default_value.clone();
    let filter_option_static = props.filter_option;
    let default_active_first_option_static = props.default_active_first_option;
    let on_change_static = props.on_change.clone();
    let on_search_static = props.on_search.clone();
    let on_select_static = props.on_select.clone();
    let on_focus_static = props.on_focus.clone();
    let on_blur_static = props.on_blur.clone();
    let on_press_enter_static = props.on_press_enter.clone();

    let mut input_value = use_signal(|| props.value.clone());
    let mut is_open = use_signal(|| false);
    let mut active_index = use_signal(|| {
        if default_active_first_option_static {
            0
        } else {
            -1
        }
    });
    let mut filtered_options = use_signal(|| options_static.clone());

    // 使用 use_memo 来创建一个持久的选项列表用于渲染
    let display_options = use_memo(move || filtered_options.read().clone());

    // 初始化值
    use_effect(move || {
        if input_value.read().is_empty() && !default_value_static.is_empty() {
            input_value.set(default_value_static.clone());
        }
    });

    // 过滤选项
    use_effect(move || {
        let value = input_value.read();
        if filter_option_static && !value.is_empty() {
            let filtered: Vec<AutoCompleteOption> = options_static
                .iter()
                .filter(|option| {
                    option.label.to_lowercase().contains(&value.to_lowercase())
                        || option.value.to_lowercase().contains(&value.to_lowercase())
                })
                .cloned()
                .collect();
            filtered_options.set(filtered);
        } else {
            filtered_options.set(options_static.clone());
        }

        // 重置活跃索引
        if default_active_first_option_static && !filtered_options.read().is_empty() {
            active_index.set(0);
        } else {
            active_index.set(-1);
        }
    });

    // 处理输入变化
    let handle_input_change = move |evt: FormEvent| {
        let value = evt.value();
        input_value.set(value.clone());
        is_open.set(!value.is_empty());

        if let Some(callback) = &on_change_static {
            callback.call(value.clone());
        }

        if let Some(callback) = &on_search_static {
            callback.call(value);
        }
    };

    // 处理选项选择
    let mut handle_option_select = move |option: AutoCompleteOption| {
        input_value.set(option.value.clone());
        is_open.set(false);

        if let Some(callback) = &on_select_static {
            callback.call(option.clone());
        }

        if let Some(callback) = &on_change_static {
            callback.call(option.value);
        }
    };

    // 处理键盘事件
    let handle_key_down = move |evt: KeyboardEvent| {
        let options = filtered_options.read().clone();
        let current_index = *active_index.read();

        match evt.key() {
            Key::ArrowDown => {
                evt.prevent_default();
                if !options.is_empty() {
                    let new_index = if current_index < options.len() as i32 - 1 {
                        current_index + 1
                    } else {
                        0
                    };
                    active_index.set(new_index);
                    is_open.set(true);
                }
            }
            Key::ArrowUp => {
                evt.prevent_default();
                if !options.is_empty() {
                    let new_index = if current_index > 0 {
                        current_index - 1
                    } else {
                        options.len() as i32 - 1
                    };
                    active_index.set(new_index);
                    is_open.set(true);
                }
            }
            Key::Enter => {
                evt.prevent_default();
                if *is_open.read() && current_index >= 0 && (current_index as usize) < options.len()
                {
                    let option = options[current_index as usize].clone();
                    handle_option_select(option);
                }

                if let Some(callback) = &on_press_enter_static {
                    callback.call(evt);
                }
            }
            Key::Escape => {
                evt.prevent_default();
                is_open.set(false);
                active_index.set(-1);
            }
            _ => {}
        }
    };

    // 处理焦点事件
    let handle_focus = move |evt: FocusEvent| {
        if !input_value.read().is_empty() {
            is_open.set(true);
        }

        if let Some(callback) = &on_focus_static {
            callback.call(evt);
        }
    };

    let handle_blur = move |evt: FocusEvent| {
        // 延迟关闭，允许点击选项
        spawn(async move {
            gloo_timers::future::TimeoutFuture::new(150).await;
            is_open.set(false);
        });

        if let Some(callback) = &on_blur_static {
            callback.call(evt);
        }
    };

    // 处理清除
    let handle_clear = move |_| {
        input_value.set(String::new());
        is_open.set(false);

        if let Some(callback) = &on_change_static {
            callback.call(String::new());
        }
    };

    let container_class = conditional_class_names_array(&[
        ("ant-auto-complete", true),
        ("ant-auto-complete-open", *is_open.read()),
        ("ant-auto-complete-disabled", props.disabled),
        (
            "ant-auto-complete-small",
            props.size == AutoCompleteSize::Small,
        ),
        (
            "ant-auto-complete-large",
            props.size == AutoCompleteSize::Large,
        ),
        (
            "ant-auto-complete-status-error",
            props.status == AutoCompleteStatus::Error,
        ),
        (
            "ant-auto-complete-status-warning",
            props.status == AutoCompleteStatus::Warning,
        ),
        (&props.class, !props.class.is_empty()),
    ]);

    let input_class = conditional_class_names_array(&[
        ("ant-input", true),
        ("ant-input-disabled", props.disabled),
        ("ant-input-sm", props.size == AutoCompleteSize::Small),
        ("ant-input-lg", props.size == AutoCompleteSize::Large),
        (
            "ant-input-status-error",
            props.status == AutoCompleteStatus::Error,
        ),
        (
            "ant-input-status-warning",
            props.status == AutoCompleteStatus::Warning,
        ),
    ]);

    rsx! {
        div {
            class: container_class,
            style: props.style,

            div { class: "ant-auto-complete-input",
                input {
                    class: "{input_class}",
                    r#type: "text",
                    value: "{input_value.read()}",
                    placeholder: "{props.placeholder}",
                    disabled: props.disabled,
                    autofocus: props.auto_focus,
                    oninput: handle_input_change,
                    onkeydown: handle_key_down,
                    onfocus: handle_focus,
                    onblur: handle_blur,
                }

                // 清除按钮
                if props.allow_clear && !input_value.read().is_empty() && !props.disabled {
                    span {
                        class: "ant-auto-complete-clear",
                        onclick: handle_clear,
                        "×"
                    }
                }
            }

            // 下拉选项
            if *is_open.read() && !display_options.read().is_empty() {
                {
                    let options_list = display_options.read().clone();
                    let current_active_index = *active_index.read();

                    rsx! {
                        div {
                            class: "ant-auto-complete-dropdown",
                            style: "max-height: {props.dropdown_max_height}px;",

                            div { class: "ant-auto-complete-dropdown-menu",
                                for (index, option) in options_list.iter().enumerate() {
                                    {
                                        let option_clone = option.clone();
                                        let option_clone2 = option.clone();
                                        rsx! {
                                            div {
                                                key: "{option.value}-{index}",
                                                class: conditional_class_names_array(&[
                                                    ("ant-auto-complete-dropdown-menu-item", true),
                                                    ("ant-auto-complete-dropdown-menu-item-active", index == current_active_index as usize),
                                                    ("ant-auto-complete-dropdown-menu-item-disabled", option.disabled),
                                                ]),
                                                onclick: move |_| {
                                                    if !option_clone.disabled {
                                                        handle_option_select(option_clone.clone());
                                                    }
                                                },
                                                onmouseenter: move |_| {
                                                    if !option_clone2.disabled {
                                                        active_index.set(index as i32);
                                                    }
                                                },

                                                "{option.label}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// 自动完成选项构建器
pub struct AutoCompleteOptionBuilder {
    option: AutoCompleteOption,
}

impl AutoCompleteOptionBuilder {
    pub fn new(value: &str, label: &str) -> Self {
        Self {
            option: AutoCompleteOption::new(value, label),
        }
    }

    pub fn disabled(mut self) -> Self {
        self.option.disabled = true;
        self
    }

    pub fn with_data(mut self, key: &str, value: &str) -> Self {
        self.option.data.insert(key.to_string(), value.to_string());
        self
    }

    pub fn build(self) -> AutoCompleteOption {
        self.option
    }
}

/// 便捷的自动完成选项创建宏
#[macro_export]
macro_rules! auto_complete_option {
    ($value:expr, $label:expr) => {
        AutoCompleteOption::new($value, $label)
    };
    ($value:expr, $label:expr, disabled) => {
        AutoCompleteOption::new($value, $label).disabled()
    };
    ($value:expr, $label:expr, data: {$($key:expr => $val:expr),*}) => {
        {
            let mut option = AutoCompleteOption::new($value, $label);
            $(
                option = option.with_data($key, $val);
            )*
            option
        }
    };
}

/// 便捷的自动完成选项列表创建宏
#[macro_export]
macro_rules! auto_complete_options {
    [$($value:expr => $label:expr),*] => {
        vec![
            $(
                AutoCompleteOption::new($value, $label),
            )*
        ]
    };
    [$($value:expr => $label:expr $(, $modifier:ident)?),*] => {
        vec![
            $(
                {
                    let mut option = AutoCompleteOption::new($value, $label);
                    $(
                        option = option.$modifier();
                    )?
                    option
                },
            )*
        ]
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auto_complete_option_new() {
        let option = AutoCompleteOption::new("value1", "Label 1");
        assert_eq!(option.value, "value1");
        assert_eq!(option.label, "Label 1");
        assert!(!option.disabled);
        assert!(option.data.is_empty());
    }

    #[test]
    fn test_auto_complete_option_disabled() {
        let option = AutoCompleteOption::new("value1", "Label 1").disabled();
        assert!(option.disabled);
    }

    #[test]
    fn test_auto_complete_option_with_data() {
        let option = AutoCompleteOption::new("value1", "Label 1")
            .with_data("key1", "value1")
            .with_data("key2", "value2");
        assert_eq!(option.data.get("key1"), Some(&"value1".to_string()));
        assert_eq!(option.data.get("key2"), Some(&"value2".to_string()));
    }

    #[test]
    fn test_auto_complete_size_default() {
        assert_eq!(AutoCompleteSize::default(), AutoCompleteSize::Middle);
    }

    #[test]
    fn test_auto_complete_status_default() {
        assert_eq!(AutoCompleteStatus::default(), AutoCompleteStatus::Default);
    }

    #[test]
    fn test_auto_complete_option_builder() {
        let option = AutoCompleteOptionBuilder::new("value1", "Label 1")
            .disabled()
            .with_data("key1", "value1")
            .build();

        assert_eq!(option.value, "value1");
        assert_eq!(option.label, "Label 1");
        assert!(option.disabled);
        assert_eq!(option.data.get("key1"), Some(&"value1".to_string()));
    }
}

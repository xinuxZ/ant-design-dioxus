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

mod styles;
use styles::{
    generate_auto_complete_style, AutoCompleteSize as StyleSize, AutoCompleteStatus as StyleStatus,
};

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
    /// 是否使用暗色主题
    #[props(default = false)]
    pub dark_theme: bool,
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
    let handle_input_change = move |evt: Event<FormData>| {
        let new_value = evt.value.clone();
        input_value.set(new_value.clone());

        // 触发搜索回调
        if let Some(on_search) = &on_search_static {
            on_search.call(new_value.clone());
        }

        // 触发变化回调
        if let Some(on_change) = &on_change_static {
            on_change.call(new_value);
        }

        is_open.set(true);
    };

    // 处理选项选择
    let handle_select = move |option: AutoCompleteOption| {
        if option.disabled {
            return;
        }

        input_value.set(option.label.clone());
        is_open.set(false);

        // 触发选择回调
        if let Some(on_select) = &on_select_static {
            on_select.call(option.clone());
        }

        // 触发变化回调
        if let Some(on_change) = &on_change_static {
            on_change.call(option.label);
        }
    };

    // 处理焦点获取
    let handle_focus = move |evt: Event<FocusData>| {
        is_open.set(true);

        if let Some(on_focus) = &on_focus_static {
            on_focus.call(evt);
        }
    };

    // 处理焦点丢失
    let handle_blur = move |evt: Event<FocusData>| {
        // 延迟关闭下拉菜单，以便用户可以点击选项
        use_future(move || async move {
            // 模拟延迟，让点击事件先触发
            #[cfg(target_arch = "wasm32")]
            {
                use wasm_bindgen::prelude::*;
                let promise = js_sys::Promise::new(&mut |resolve, _| {
                    let closure = Closure::once_into_js(move || {
                        resolve.call0(&JsValue::NULL).unwrap();
                    });
                    web_sys::window()
                        .unwrap()
                        .set_timeout_with_callback_and_timeout_and_arguments_0(
                            closure.as_ref().unchecked_ref(),
                            150,
                        )
                        .unwrap();
                });
                wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();
            }

            is_open.set(false);
        });

        if let Some(on_blur) = &on_blur_static {
            on_blur.call(evt);
        }
    };

    // 处理键盘事件
    let handle_key_down = move |evt: Event<KeyboardData>| {
        match evt.key.as_str() {
            "ArrowDown" => {
                evt.stop_propagation();
                evt.prevent_default();

                if !is_open.get() {
                    is_open.set(true);
                    return;
                }

                let options_len = filtered_options.read().len() as i32;
                if options_len == 0 {
                    return;
                }

                let mut new_index = active_index.get() + 1;
                if new_index >= options_len {
                    new_index = 0;
                }

                // 跳过禁用的选项
                let mut attempts = 0;
                while attempts < options_len
                    && filtered_options
                        .read()
                        .get(new_index as usize)
                        .map_or(false, |o| o.disabled)
                {
                    new_index = (new_index + 1) % options_len;
                    attempts += 1;
                }

                active_index.set(new_index);
            }
            "ArrowUp" => {
                evt.stop_propagation();
                evt.prevent_default();

                if !is_open.get() {
                    is_open.set(true);
                    return;
                }

                let options_len = filtered_options.read().len() as i32;
                if options_len == 0 {
                    return;
                }

                let mut new_index = active_index.get() - 1;
                if new_index < 0 {
                    new_index = options_len - 1;
                }

                // 跳过禁用的选项
                let mut attempts = 0;
                while attempts < options_len
                    && filtered_options
                        .read()
                        .get(new_index as usize)
                        .map_or(false, |o| o.disabled)
                {
                    new_index = (new_index - 1 + options_len) % options_len;
                    attempts += 1;
                }

                active_index.set(new_index);
            }
            "Enter" => {
                if is_open.get() && active_index.get() >= 0 {
                    evt.stop_propagation();
                    evt.prevent_default();

                    if let Some(option) = filtered_options
                        .read()
                        .get(active_index.get() as usize)
                        .cloned()
                    {
                        handle_select(option);
                    }
                } else if let Some(on_press_enter) = &on_press_enter_static {
                    on_press_enter.call(evt);
                }
            }
            "Escape" => {
                evt.stop_propagation();
                evt.prevent_default();
                is_open.set(false);
            }
            _ => {}
        }
    };

    // 处理清除
    let handle_clear = move |_| {
        input_value.set(String::new());
        is_open.set(false);

        // 触发变化回调
        if let Some(on_change) = &on_change_static {
            on_change.call(String::new());
        }
    };

    // 生成CSS样式
    let auto_complete_style = generate_auto_complete_style(
        props.size.into(),
        props.status.into(),
        props.disabled,
        props.allow_clear,
        props.dropdown_max_height,
        props.dark_theme,
    );

    // 构建类名
    let container_class = conditional_class_names_array(&[
        ("ant-auto-complete", true),
        (props.class.as_str(), !props.class.is_empty()),
        (
            match props.size {
                AutoCompleteSize::Small => "ant-auto-complete-small",
                AutoCompleteSize::Middle => "",
                AutoCompleteSize::Large => "ant-auto-complete-large",
            },
            props.size != AutoCompleteSize::Middle,
        ),
        (
            match props.status {
                AutoCompleteStatus::Default => "",
                AutoCompleteStatus::Error => "ant-auto-complete-status-error",
                AutoCompleteStatus::Warning => "ant-auto-complete-status-warning",
            },
            props.status != AutoCompleteStatus::Default,
        ),
        ("ant-auto-complete-disabled", props.disabled),
        ("ant-auto-complete-open", is_open.get()),
    ]);

    // 构建输入框类名
    let input_class = conditional_class_names_array(&[
        ("ant-input", true),
        (
            match props.size {
                AutoCompleteSize::Small => "ant-input-sm",
                AutoCompleteSize::Middle => "",
                AutoCompleteSize::Large => "ant-input-lg",
            },
            props.size != AutoCompleteSize::Middle,
        ),
        (
            match props.status {
                AutoCompleteStatus::Default => "",
                AutoCompleteStatus::Error => "ant-input-status-error",
                AutoCompleteStatus::Warning => "ant-input-status-warning",
            },
            props.status != AutoCompleteStatus::Default,
        ),
        ("ant-input-disabled", props.disabled),
    ]);

    rsx! {
        style { {auto_complete_style} }
        div {
            class: container_class,
            style: props.style.clone(),
            div {
                class: "ant-auto-complete-input",
                input {
                    class: input_class,
                    type: "text",
                    value: input_value.read().clone(),
                    placeholder: props.placeholder.clone(),
                    disabled: props.disabled,
                    autofocus: props.auto_focus,
                    oninput: handle_input_change,
                    onfocus: handle_focus,
                    onblur: handle_blur,
                    onkeydown: handle_key_down,
                }
                if props.allow_clear && !input_value.read().is_empty() && !props.disabled {
                    span {
                        class: "ant-auto-complete-clear",
                        aria_label: "Clear",
                        role: "button",
                        onclick: handle_clear,
                        "×"
                    }
                }
            }
            if is_open.get() && !props.disabled {
                div {
                    class: "ant-auto-complete-dropdown",
                    ul {
                        class: "ant-auto-complete-dropdown-menu",
                        if display_options.is_empty() {
                            li {
                                class: "ant-auto-complete-dropdown-menu-empty",
                                "No options"
                            }
                        } else {
                            for (index, option) in display_options.iter().enumerate() {
                                let is_active = active_index.get() == index as i32;
                                let is_disabled = option.disabled;
                                li {
                                    key: "{option.value}",
                                    class: conditional_class_names_array(&[
                                        ("ant-auto-complete-dropdown-menu-item", true),
                                        ("ant-auto-complete-dropdown-menu-item-active", is_active),
                                        ("ant-auto-complete-dropdown-menu-item-disabled", is_disabled),
                                    ]),
                                    onclick: move |_| {
                                        handle_select(option.clone());
                                    },
                                    {option.label.clone()}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// AutoComplete 选项构建器
pub struct AutoCompleteOptionBuilder {
    option: AutoCompleteOption,
}

impl AutoCompleteOptionBuilder {
    /// 创建新的选项构建器
    pub fn new(value: &str, label: &str) -> Self {
        Self {
            option: AutoCompleteOption::new(value, label),
        }
    }

    /// 设置为禁用
    pub fn disabled(mut self) -> Self {
        self.option.disabled = true;
        self
    }

    /// 添加自定义数据
    pub fn with_data(mut self, key: &str, value: &str) -> Self {
        self.option.data.insert(key.to_string(), value.to_string());
        self
    }

    /// 构建选项
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
        let option = AutoCompleteOption::new("value", "label");
        assert_eq!(option.value, "value");
        assert_eq!(option.label, "label");
        assert!(!option.disabled);
        assert!(option.data.is_empty());
    }

    #[test]
    fn test_auto_complete_option_disabled() {
        let option = AutoCompleteOption::new("value", "label").disabled();
        assert!(option.disabled);
    }

    #[test]
    fn test_auto_complete_option_with_data() {
        let option = AutoCompleteOption::new("value", "label").with_data("key", "value");
        assert_eq!(option.data.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_auto_complete_size_default() {
        let size = AutoCompleteSize::default();
        assert_eq!(size, AutoCompleteSize::Middle);
    }

    #[test]
    fn test_auto_complete_status_default() {
        let status = AutoCompleteStatus::default();
        assert_eq!(status, AutoCompleteStatus::Default);
    }

    #[test]
    fn test_auto_complete_option_builder() {
        let option = AutoCompleteOptionBuilder::new("value", "label")
            .disabled()
            .with_data("key", "value")
            .build();

        assert_eq!(option.value, "value");
        assert_eq!(option.label, "label");
        assert!(option.disabled);
        assert_eq!(option.data.get("key"), Some(&"value".to_string()));
    }
}

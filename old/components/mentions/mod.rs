//! Mentions 提及组件
//!
//! 提及组件，用于在输入中提及某人或某事，常用于发布、聊天或评论功能。
//!
//! ## 何时使用
//!
//! 用于在输入中提及某人或某事，常用于发布、聊天或评论功能。
//!
//! ## 示例
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::{Mentions, MentionOption};
//!
//! fn app() -> Element {
//!     let options = vec![
//!         MentionOption::new("afc163", "afc163"),
//!         MentionOption::new("zombieJ", "zombieJ"),
//!         MentionOption::new("yesmeck", "yesmeck"),
//!     ];
//!
//!     rsx! {
//!         Mentions {
//!             placeholder: "输入 @ 来提及某人",
//!             options: options,
//!             on_change: |value| {
//!                 println!("输入内容: {}", value);
//!             }
//!         }
//!     }
//! }
//! ```

use dioxus::events::Key;
use dioxus::prelude::*;
use std::fmt;

const MENTIONS_STYLE: &str = include_str!("./style.css");

/// 提及选项
#[derive(Debug, Clone, PartialEq)]
pub struct MentionOption {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

impl MentionOption {
    /// 创建新的提及选项
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            disabled: false,
        }
    }

    /// 设置为禁用状态
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

/// 提及组件尺寸
#[derive(Debug, Clone, PartialEq)]
pub enum MentionsSize {
    Small,
    Middle,
    Large,
}

impl Default for MentionsSize {
    fn default() -> Self {
        Self::Middle
    }
}

impl fmt::Display for MentionsSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Small => write!(f, "small"),
            Self::Middle => write!(f, "middle"),
            Self::Large => write!(f, "large"),
        }
    }
}

/// 提及组件状态
#[derive(Debug, Clone, PartialEq)]
pub enum MentionsStatus {
    Default,
    Error,
    Warning,
}

impl Default for MentionsStatus {
    fn default() -> Self {
        Self::Default
    }
}

impl fmt::Display for MentionsStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Default => write!(f, "default"),
            Self::Error => write!(f, "error"),
            Self::Warning => write!(f, "warning"),
        }
    }
}

/// 提及组件属性
#[derive(Props, Clone, PartialEq)]
pub struct MentionsProps {
    /// 自定义类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,

    /// 输入框提示文字
    #[props(default)]
    pub placeholder: Option<String>,

    /// 组件尺寸
    #[props(default)]
    pub size: MentionsSize,

    /// 组件状态
    #[props(default)]
    pub status: MentionsStatus,

    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,

    /// 是否只读
    #[props(default = false)]
    pub readonly: bool,

    /// 是否自动获取焦点
    #[props(default = false)]
    pub auto_focus: bool,

    /// 是否显示边框
    #[props(default = true)]
    pub bordered: bool,

    /// 是否允许清除
    #[props(default = false)]
    pub allow_clear: bool,

    /// 触发字符
    #[props(default = "@".to_string())]
    pub prefix: String,

    /// 分隔符
    #[props(default = " ".to_string())]
    pub split: String,

    /// 行数（多行模式）
    #[props(default)]
    pub rows: Option<usize>,

    /// 自动调整高度
    #[props(default = false)]
    pub auto_size: bool,

    /// 最大行数
    #[props(default)]
    pub max_rows: Option<usize>,

    /// 最小行数
    #[props(default)]
    pub min_rows: Option<usize>,

    /// 默认值
    #[props(default)]
    pub default_value: Option<String>,

    /// 当前值
    #[props(default)]
    pub value: Option<String>,

    /// 提及选项
    #[props(default)]
    pub options: Vec<MentionOption>,

    /// 过滤函数
    #[props(default)]
    pub filter_option: Option<EventHandler<(String, MentionOption)>>,

    /// 值改变时的回调
    #[props(default)]
    pub on_change: Option<EventHandler<String>>,

    /// 选择提及项时的回调
    #[props(default)]
    pub on_select: Option<EventHandler<MentionOption>>,

    /// 搜索时的回调
    #[props(default)]
    pub on_search: Option<EventHandler<String>>,

    /// 获得焦点时的回调
    #[props(default)]
    pub on_focus: Option<EventHandler<FocusEvent>>,

    /// 失去焦点时的回调
    #[props(default)]
    pub on_blur: Option<EventHandler<FocusEvent>>,

    /// 按下回车键时的回调
    #[props(default)]
    pub on_press_enter: Option<EventHandler<KeyboardEvent>>,
}

/// Mentions 组件
#[component]
pub fn Mentions(props: MentionsProps) -> Element {
    let mut current_value = use_signal(|| {
        props
            .value
            .clone()
            .or_else(|| props.default_value.clone())
            .unwrap_or_default()
    });
    let mut search_text = use_signal(|| String::new());
    let mut show_dropdown = use_signal(|| false);
    let mut _cursor_position = use_signal(|| 0);
    let mut mention_start = use_signal(|| None::<usize>);
    let mut filtered_options = use_signal(|| props.options.clone());
    let mut selected_index = use_signal(|| 0);
    let mut focused = use_signal(|| false);

    let class_name = format!(
        "ant-mentions {} {} {} {} {}",
        if props.disabled {
            "ant-mentions-disabled"
        } else {
            ""
        },
        if !props.bordered {
            "ant-mentions-borderless"
        } else {
            ""
        },
        if focused() {
            "ant-mentions-focused"
        } else {
            ""
        },
        format!("ant-mentions-{}", props.size),
        if props.status != MentionsStatus::Default {
            format!("ant-mentions-status-{}", props.status)
        } else {
            String::new()
        },
    )
    .trim()
    .to_string();

    let class_name = if let Some(custom_class) = &props.class {
        format!("{} {}", class_name, custom_class)
    } else {
        class_name
    };

    let prefix_for_input = props.prefix.clone();
    let split_for_input = props.split.clone();
    let handle_input_change = move |e: FormEvent| {
        let value = e.value();
        current_value.set(value.clone());

        // 检查是否触发提及
        if let Some(pos) = value.rfind(&prefix_for_input) {
            let after_prefix = &value[pos + prefix_for_input.len()..];
            if !after_prefix.contains(&split_for_input) {
                mention_start.set(Some(pos));
                search_text.set(after_prefix.to_string());
                show_dropdown.set(true);

                // 过滤选项
                let filtered = if let Some(_filter) = &props.filter_option {
                    // 注意：EventHandler不能直接返回bool，这里需要重新设计
                    // 暂时使用默认过滤逻辑
                    props
                        .options
                        .iter()
                        .filter(|option| {
                            option
                                .label
                                .to_lowercase()
                                .contains(&after_prefix.to_lowercase())
                        })
                        .cloned()
                        .collect()
                } else {
                    props
                        .options
                        .iter()
                        .filter(|option| {
                            option
                                .label
                                .to_lowercase()
                                .contains(&after_prefix.to_lowercase())
                        })
                        .cloned()
                        .collect()
                };

                filtered_options.set(filtered);
                selected_index.set(0);

                if let Some(handler) = &props.on_search {
                    handler.call(after_prefix.to_string());
                }
            } else {
                show_dropdown.set(false);
                mention_start.set(None);
            }
        } else {
            show_dropdown.set(false);
            mention_start.set(None);
        }

        if let Some(handler) = &props.on_change {
            handler.call(value);
        }
    };

    // 为选项选择准备共享的数据
    let prefix_for_select = props.prefix.clone();
    let split_for_select = props.split.clone();
    let on_select_handler = props.on_select.clone();

    let prefix_for_key = prefix_for_select.clone();
    let split_for_key = split_for_select.clone();
    let on_select_for_key = on_select_handler.clone();
    let handle_key_down = move |e: KeyboardEvent| {
        if show_dropdown() && !filtered_options().is_empty() {
            match e.key() {
                Key::ArrowDown => {
                    e.prevent_default();
                    let new_index = (selected_index() + 1) % filtered_options().len();
                    selected_index.set(new_index);
                }
                Key::ArrowUp => {
                    e.prevent_default();
                    let new_index = if selected_index() == 0 {
                        filtered_options().len() - 1
                    } else {
                        selected_index() - 1
                    };
                    selected_index.set(new_index);
                }
                Key::Enter => {
                    e.prevent_default();
                    if let Some(option) = filtered_options().get(selected_index()) {
                        if let Some(start_pos) = mention_start() {
                            let mut new_value = current_value();
                            let end_pos = start_pos + prefix_for_key.len() + search_text().len();

                            new_value.replace_range(
                                start_pos..(end_pos.min(new_value.len())),
                                &format!("{}{}{}", prefix_for_key, option.value, split_for_key),
                            );

                            current_value.set(new_value.clone());
                            show_dropdown.set(false);
                            mention_start.set(None);
                            search_text.set(String::new());

                            if let Some(handler) = &on_select_for_key {
                                handler.call(option.clone());
                            }
                        }
                    }
                }
                Key::Escape => {
                    e.prevent_default();
                    show_dropdown.set(false);
                    mention_start.set(None);
                }
                _ => {
                    if e.code() == Code::Enter {
                        if let Some(handler) = &props.on_press_enter {
                            handler.call(e);
                        }
                    }
                }
            }
        } else if e.code() == Code::Enter {
            if let Some(handler) = &props.on_press_enter {
                handler.call(e);
            }
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
        // 延迟隐藏下拉菜单，以便点击选项
        spawn(async move {
            gloo_timers::future::TimeoutFuture::new(150).await;
            show_dropdown.set(false);
        });

        if let Some(handler) = &props.on_blur {
            handler.call(e);
        }
    };

    let handle_clear = move |_| {
        current_value.set(String::new());
        show_dropdown.set(false);
        mention_start.set(None);
        search_text.set(String::new());

        if let Some(handler) = &props.on_change {
            handler.call(String::new());
        }
    };

    rsx! {
        style { {MENTIONS_STYLE} }

        div {
            class: class_name,
            style: props.style.as_deref().unwrap_or(""),

            div {
                class: "ant-mentions-wrapper",

                if props.rows.is_some() || props.auto_size {
                    textarea {
                        class: "ant-mentions-input",
                        placeholder: props.placeholder.as_deref().unwrap_or(""),
                        value: current_value(),
                        rows: props.rows.unwrap_or(4),
                        disabled: props.disabled,
                        readonly: props.readonly,
                        autofocus: props.auto_focus,
                        oninput: handle_input_change,
                        onkeydown: handle_key_down,
                        onfocus: handle_focus,
                        onblur: handle_blur,
                    }
                } else {
                    input {
                        class: "ant-mentions-input",
                        r#type: "text",
                        placeholder: props.placeholder.as_deref().unwrap_or(""),
                        value: current_value(),
                        disabled: props.disabled,
                        readonly: props.readonly,
                        autofocus: props.auto_focus,
                        oninput: handle_input_change,
                        onkeydown: handle_key_down,
                        onfocus: handle_focus,
                        onblur: handle_blur,
                    }
                }

                if props.allow_clear && !current_value().is_empty() && !props.disabled {
                    span {
                        class: "ant-mentions-clear",
                        onclick: handle_clear,
                        "✕"
                    }
                }
            }

            if show_dropdown() && !filtered_options().is_empty() {
                div {
                    class: "ant-mentions-dropdown",

                    div {
                        class: "ant-mentions-dropdown-menu",

                        for (index, option) in filtered_options().iter().enumerate() {
                            div {
                                key: option.value.clone(),
                                class: format!(
                                    "ant-mentions-dropdown-menu-item {} {} {}",
                                    if index == selected_index() { "ant-mentions-dropdown-menu-item-active" } else { "" },
                                    if option.disabled { "ant-mentions-dropdown-menu-item-disabled" } else { "" },
                                    "ant-mentions-dropdown-menu-item"
                                ),
                                onclick: {
                                    let option = option.clone();
                                    let prefix_clone = prefix_for_select.clone();
                                    let split_clone = split_for_select.clone();
                                    let on_select_clone = on_select_handler.clone();
                                    let mut current_value_clone = current_value.clone();
                                    let mut show_dropdown_clone = show_dropdown.clone();
                                    let mut mention_start_clone = mention_start.clone();
                                    let mut search_text_clone = search_text.clone();
                                    move |_| {
                                        if !option.disabled {
                                            if let Some(start_pos) = mention_start_clone() {
                                                let mut new_value = current_value_clone();
                                                let end_pos = start_pos + prefix_clone.len() + search_text_clone().len();

                                                new_value.replace_range(
                                                    start_pos..(end_pos.min(new_value.len())),
                                                    &format!("{}{}{}", prefix_clone, option.value, split_clone),
                                                );

                                                current_value_clone.set(new_value.clone());
                                                show_dropdown_clone.set(false);
                                                mention_start_clone.set(None);
                                                search_text_clone.set(String::new());

                                                if let Some(handler) = &on_select_clone {
                                                    handler.call(option.clone());
                                                }
                                            }
                                        }
                                    }
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

/// 提及选项构建器
pub struct MentionOptionBuilder {
    value: String,
    label: String,
    disabled: bool,
}

impl MentionOptionBuilder {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            disabled: false,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn build(self) -> MentionOption {
        MentionOption {
            value: self.value,
            label: self.label,
            disabled: self.disabled,
        }
    }
}

/// 创建提及选项的宏
#[macro_export]
macro_rules! mention_option {
    ($value:expr, $label:expr) => {
        MentionOption::new($value, $label)
    };
    ($value:expr, $label:expr, disabled: $disabled:expr) => {
        MentionOption::new($value, $label).disabled()
    };
}

/// 创建提及选项列表的宏
#[macro_export]
macro_rules! mention_options {
    [$($value:expr => $label:expr),* $(,)?] => {
        vec![
            $(MentionOption::new($value, $label),)*
        ]
    };
}

/// 创建提及组件的宏
#[macro_export]
macro_rules! mentions {
    () => {
        Mentions {
            options: vec![]
        }
    };
    ($($prop:ident: $value:expr),* $(,)?) => {
        Mentions {
            $($prop: $value,)*
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mention_option() {
        let option = MentionOption::new("test", "Test User");
        assert_eq!(option.value, "test");
        assert_eq!(option.label, "Test User");
        assert!(!option.disabled);

        let disabled_option = MentionOption::new("test", "Test User").disabled();
        assert!(disabled_option.disabled);
    }

    #[test]
    fn test_mentions_size() {
        assert_eq!(MentionsSize::Small.to_string(), "small");
        assert_eq!(MentionsSize::Middle.to_string(), "middle");
        assert_eq!(MentionsSize::Large.to_string(), "large");
    }

    #[test]
    fn test_mentions_status() {
        assert_eq!(MentionsStatus::Default.to_string(), "default");
        assert_eq!(MentionsStatus::Error.to_string(), "error");
        assert_eq!(MentionsStatus::Warning.to_string(), "warning");
    }

    #[test]
    fn test_mention_option_builder() {
        let option = MentionOptionBuilder::new("test", "Test User")
            .disabled(true)
            .build();

        assert_eq!(option.value, "test");
        assert_eq!(option.label, "Test User");
        assert!(option.disabled);
    }

    #[test]
    fn test_mentions_defaults() {
        assert_eq!(MentionsSize::default(), MentionsSize::Middle);
        assert_eq!(MentionsStatus::default(), MentionsStatus::Default);
    }
}

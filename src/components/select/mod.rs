//! Select 选择器组件
//!
//! 下拉选择器。
//!
//! ## 何时使用
//!
//! - 弹出一个下拉菜单给用户选择操作，用于代替原生的选择器，或者需要一个更优雅的多选器时。
//! - 当选项少时（少于 5 项），建议直接将选项平铺，使用 Radio 是更好的选择。
//!
//! ## 代码演示
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::{Select, SelectOption, SelectSize};
//!
//! #[component]
//! fn App() -> Element {
//!     let mut value = use_signal(String::new);
//!
//!     rsx! {
//!         Select {
//!             value: value(),
//!             placeholder: "请选择",
//!             on_change: move |v| value.set(v),
//!
//!             SelectOption {
//!                 value: "option1",
//!                 "选项1"
//!             }
//!             SelectOption {
//!                 value: "option2",
//!                 "选项2"
//!             }
//!         }
//!     }
//! }
//! ```

use dioxus::prelude::*;

/// Select 组件尺寸
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SelectSize {
    /// 大尺寸
    Large,
    /// 中等尺寸（默认）
    Middle,
    /// 小尺寸
    Small,
}

impl Default for SelectSize {
    fn default() -> Self {
        Self::Middle
    }
}

impl SelectSize {
    /// 获取尺寸对应的CSS类名
    pub fn to_class(&self) -> &'static str {
        match self {
            SelectSize::Large => "ant-select-lg",
            SelectSize::Middle => "",
            SelectSize::Small => "ant-select-sm",
        }
    }
}

/// Select 组件状态
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SelectStatus {
    /// 正常状态
    Normal,
    /// 错误状态
    Error,
    /// 警告状态
    Warning,
}

impl Default for SelectStatus {
    fn default() -> Self {
        Self::Normal
    }
}

impl SelectStatus {
    /// 获取状态对应的CSS类名
    pub fn to_class(&self) -> &'static str {
        match self {
            SelectStatus::Normal => "",
            SelectStatus::Error => "ant-select-status-error",
            SelectStatus::Warning => "ant-select-status-warning",
        }
    }
}

/// SelectOption 选项数据
#[derive(Debug, Clone, PartialEq)]
pub struct OptionData {
    /// 选项值
    pub value: String,
    /// 选项标签
    pub label: String,
    /// 是否禁用
    pub disabled: bool,
}

/// Select 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct SelectProps {
    /// 当前选中的值
    #[props(default)]
    pub value: String,

    /// 多选时的值
    #[props(default)]
    pub values: Vec<String>,

    /// 选择框默认文字
    #[props(default)]
    pub placeholder: Option<String>,

    /// 选择器尺寸
    #[props(default)]
    pub size: SelectSize,

    /// 选择器状态
    #[props(default)]
    pub status: SelectStatus,

    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,

    /// 是否支持多选
    #[props(default = false)]
    pub multiple: bool,

    /// 是否支持清除
    #[props(default = false)]
    pub allow_clear: bool,

    /// 是否显示搜索框
    #[props(default = false)]
    pub show_search: bool,

    /// 是否根据输入项进行筛选
    #[props(default = true)]
    pub filter_option: bool,

    /// 搜索时对筛选结果项的处理
    #[props(default)]
    pub search_value: Option<String>,

    /// 是否有边框
    #[props(default = true)]
    pub bordered: bool,

    /// 是否在选中项后清空搜索框，只在 mode 为 multiple 或 tags 时有效
    #[props(default = true)]
    pub auto_clear_search_value: bool,

    /// 下拉菜单的最大高度
    #[props(default = 256)]
    pub list_height: u32,

    /// 自定义样式类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,

    /// 选中选项时调用
    #[props(default)]
    pub on_change: Option<EventHandler<String>>,

    /// 多选时的回调
    #[props(default)]
    pub on_change_multiple: Option<EventHandler<Vec<String>>>,

    /// 搜索时的回调
    #[props(default)]
    pub on_search: Option<EventHandler<String>>,

    /// 获得焦点时的回调
    #[props(default)]
    pub on_focus: Option<EventHandler<FocusEvent>>,

    /// 失去焦点时的回调
    #[props(default)]
    pub on_blur: Option<EventHandler<FocusEvent>>,

    /// 下拉框出现/隐藏时的回调
    #[props(default)]
    pub on_dropdown_visible_change: Option<EventHandler<bool>>,

    /// 清除时的回调
    #[props(default)]
    pub on_clear: Option<EventHandler<()>>,

    /// 子元素（SelectOption）
    children: Element,
}

/// Select 选择器组件
///
/// 下拉选择器
#[component]
pub fn Select(props: SelectProps) -> Element {
    let mut is_open = use_signal(|| false);
    let mut search_value = use_signal(|| String::new());
    let mut is_focused = use_signal(|| false);
    let mut selected_values = use_signal(|| {
        if props.multiple {
            props.values.clone()
        } else if !props.value.is_empty() {
            vec![props.value.clone()]
        } else {
            vec![]
        }
    });

    // 同步外部值到内部状态
    use_effect(move || {
        if props.multiple {
            selected_values.set(props.values.clone());
        } else if !props.value.is_empty() {
            selected_values.set(vec![props.value.clone()]);
        } else {
            selected_values.set(vec![]);
        }
    });

    let handle_click = move |_| {
        if !props.disabled {
            is_open.set(!is_open());
            if let Some(on_dropdown_visible_change) = &props.on_dropdown_visible_change {
                on_dropdown_visible_change.call(!is_open());
            }
        }
    };

    let _handle_option_click = move |value: String| {
        if props.multiple {
            let mut current_values = selected_values();
            if current_values.contains(&value) {
                current_values.retain(|v| v != &value);
            } else {
                current_values.push(value);
            }
            selected_values.set(current_values.clone());
            if let Some(on_change_multiple) = &props.on_change_multiple {
                on_change_multiple.call(current_values);
            }
            if props.auto_clear_search_value {
                search_value.set(String::new());
            }
        } else {
            selected_values.set(vec![value.clone()]);
            if let Some(on_change) = &props.on_change {
                on_change.call(value);
            }
            is_open.set(false);
            if let Some(on_dropdown_visible_change) = &props.on_dropdown_visible_change {
                on_dropdown_visible_change.call(false);
            }
        }
    };

    let handle_search = move |evt: FormEvent| {
        let value = evt.value();
        search_value.set(value.clone());
        if let Some(on_search) = &props.on_search {
            on_search.call(value);
        }
    };

    let handle_clear = move |_| {
        selected_values.set(vec![]);
        search_value.set(String::new());
        if props.multiple {
            if let Some(on_change_multiple) = &props.on_change_multiple {
                on_change_multiple.call(vec![]);
            }
        } else {
            if let Some(on_change) = &props.on_change {
                on_change.call(String::new());
            }
        }
        if let Some(on_clear) = &props.on_clear {
            on_clear.call(());
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

    let select_class = {
        let mut classes = vec!["ant-select"];

        let size_class = props.size.to_class();
        if !size_class.is_empty() {
            classes.push(size_class);
        }

        let status_class = props.status.to_class();
        if !status_class.is_empty() {
            classes.push(status_class);
        }

        if props.disabled {
            classes.push("ant-select-disabled");
        }

        if !props.bordered {
            classes.push("ant-select-borderless");
        }

        if is_open() {
            classes.push("ant-select-open");
        }

        if is_focused() {
            classes.push("ant-select-focused");
        }

        if props.multiple {
            classes.push("ant-select-multiple");
        }

        if props.show_search {
            classes.push("ant-select-show-search");
        }

        if let Some(ref class) = props.class {
            classes.push(class);
        }

        classes.join(" ")
    };

    // 获取显示文本
    let display_text = if props.multiple {
        if selected_values().is_empty() {
            props.placeholder.unwrap_or_default()
        } else {
            format!("{} 项已选择", selected_values().len())
        }
    } else {
        if selected_values().is_empty() {
            props.placeholder.unwrap_or_default()
        } else {
            selected_values().first().cloned().unwrap_or_default()
        }
    };

    let has_value = !selected_values().is_empty();

    rsx! {
        div {
            class: select_class,
            style: props.style,

            div {
                class: "ant-select-selector",
                onclick: handle_click,
                onfocus: handle_focus,
                onblur: handle_blur,
                tabindex: "0",

                // 多选标签
                if props.multiple && !selected_values().is_empty() {
                    div {
                        class: "ant-select-selection-overflow",

                        for value in selected_values() {
                            div {
                                class: "ant-select-selection-overflow-item",

                                span {
                                    class: "ant-select-selection-item",

                                    span {
                                        class: "ant-select-selection-item-content",
                                        {value.clone()}
                                    }

                                    span {
                                        class: "ant-select-selection-item-remove",
                                        onclick: move |_| {
                                            let mut current_values = selected_values();
                                            current_values.retain(|v| v != &value);
                                            selected_values.set(current_values.clone());
                                            if let Some(on_change_multiple) = &props.on_change_multiple {
                                                on_change_multiple.call(current_values);
                                            }
                                        },
                                        "×"
                                    }
                                }
                            }
                        }
                    }
                }

                // 搜索输入框
                if props.show_search {
                    div {
                        class: "ant-select-selection-search",

                        input {
                            class: "ant-select-selection-search-input",
                            value: search_value(),
                            placeholder: if has_value { "" } else { display_text.as_str() },
                            oninput: handle_search
                        }
                    }
                } else {
                    // 选择项显示
                    div {
                        class: "ant-select-selection-item",
                        title: display_text.clone(),
                        {display_text.clone()}
                    }
                }

                // 占位符
                if !has_value && !props.show_search {
                    div {
                        class: "ant-select-selection-placeholder",
                        {display_text.clone()}
                    }
                }
            }

            // 清除按钮
            if props.allow_clear && has_value && !props.disabled {
                span {
                    class: "ant-select-clear",
                    onclick: handle_clear,
                    "×"
                }
            }

            // 下拉箭头
            span {
                class: "ant-select-arrow",
                span {
                    class: "ant-select-suffix",
                    if is_open() { "▲" } else { "▼" }
                }
            }

            // 下拉菜单
            if is_open() {
                div {
                    class: "ant-select-dropdown",
                    style: format!("max-height: {}px; overflow-y: auto;", props.list_height),

                    div {
                        class: "ant-select-dropdown-menu",

                        // 渲染选项
                        {props.children}
                    }
                }
            }
        }
    }
}

/// SelectOption 选项组件属性
#[derive(Props, Clone, PartialEq)]
pub struct SelectOptionProps {
    /// 选项值
    pub value: String,

    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,

    /// 自定义样式类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,

    /// 子元素
    children: Element,
}

/// SelectOption 选项组件
///
/// Select 的选项
#[component]
pub fn SelectOption(props: SelectOptionProps) -> Element {
    let option_class = {
        let mut classes = vec!["ant-select-item", "ant-select-item-option"];

        if props.disabled {
            classes.push("ant-select-item-option-disabled");
        }

        if let Some(ref class) = props.class {
            classes.push(class);
        }

        classes.join(" ")
    };

    rsx! {
        div {
            class: option_class,
            style: props.style,
            "data-value": props.value.clone(),

            div {
                class: "ant-select-item-option-content",
                {props.children}
            }
        }
    }
}

/// SelectOptGroup 选项组组件属性
#[derive(Props, Clone, PartialEq)]
pub struct SelectOptGroupProps {
    /// 组标签
    pub label: String,

    /// 自定义样式类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,

    /// 子元素
    children: Element,
}

/// SelectOptGroup 选项组组件
///
/// Select 的选项组
#[component]
pub fn SelectOptGroup(props: SelectOptGroupProps) -> Element {
    let group_class = {
        let mut classes = vec!["ant-select-item", "ant-select-item-group"];

        if let Some(ref class) = props.class {
            classes.push(class);
        }

        classes.join(" ")
    };

    rsx! {
        div {
            class: group_class,
            style: props.style,

            div {
                class: "ant-select-item-group-label",
                {props.label.clone()}
            }

            {props.children}
        }
    }
}

// 组件已通过#[component]宏自动导出
// 无需重新导出

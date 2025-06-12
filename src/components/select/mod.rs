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

mod styles;
use styles::{SelectSize as StyleSize, SelectStatus as StyleStatus, SelectStyleBuilder};

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

    /// 清除内容时的回调
    #[props(default)]
    pub on_clear: Option<EventHandler<()>>,

    /// 子元素（SelectOption）
    children: Element,
}

/// Select 选择器组件
#[component]
pub fn Select(props: SelectProps) -> Element {
    // 状态管理
    let search_value = create_signal(props.search_value.clone().unwrap_or_default());
    let dropdown_visible = create_signal(false);
    let focused = create_signal(false);

    // 提取所有选项
    let mut options = Vec::new();
    if let Ok(children) = props.children.as_ref() {
        extract_options(children, &mut options);
    }

    // 处理选项点击事件
    let handle_option_click = move |value: String| {
        // 多选模式
        if props.multiple {
            let mut new_values = props.values.clone();
            if new_values.contains(&value) {
                new_values.retain(|v| v != &value);
            } else {
                new_values.push(value.clone());
            }

            if let Some(handler) = &props.on_change_multiple {
                handler.call(new_values);
            }

            if props.auto_clear_search_value {
                search_value.set(String::new());
            }
        }
        // 单选模式
        else {
            if let Some(handler) = &props.on_change {
                handler.call(value);
            }

            dropdown_visible.set(false);

            if props.auto_clear_search_value && props.show_search {
                search_value.set(String::new());
            }
        }

        if let Some(handler) = &props.on_dropdown_visible_change {
            handler.call(dropdown_visible());
        }
    };

    // 处理搜索框值变化
    let handle_search_input = move |e: Event<FormData>| {
        let value = e.value().clone();
        search_value.set(value.clone());

        if let Some(handler) = &props.on_search {
            handler.call(value);
        }
    };

    // 处理焦点事件
    let handle_focus = move |e: Event<FocusEvent>| {
        focused.set(true);
        if !props.disabled {
            if let Some(handler) = &props.on_focus {
                handler.call(e);
            }
        }
    };

    // 处理失焦事件
    let handle_blur = move |e: Event<FocusEvent>| {
        focused.set(false);
        if !props.disabled {
            if let Some(handler) = &props.on_blur {
                handler.call(e);
            }
        }
    };

    // 切换下拉框显示状态
    let toggle_dropdown = move |_| {
        if !props.disabled {
            let new_visible = !dropdown_visible();
            dropdown_visible.set(new_visible);

            if let Some(handler) = &props.on_dropdown_visible_change {
                handler.call(new_visible);
            }
        }
    };

    // 清空选择
    let handle_clear = move |_| {
        if props.multiple {
            if let Some(handler) = &props.on_change_multiple {
                handler.call(vec![]);
            }
        } else {
            if let Some(handler) = &props.on_change {
                handler.call(String::new());
            }
        }

        if let Some(handler) = &props.on_clear {
            handler.call(());
        }

        search_value.set(String::new());
    };

    // 过滤选项
    let filtered_options = if props.filter_option && search_value().len() > 0 {
        options
            .iter()
            .filter(|option| {
                option
                    .label
                    .to_lowercase()
                    .contains(&search_value().to_lowercase())
                    || option
                        .value
                        .to_lowercase()
                        .contains(&search_value().to_lowercase())
            })
            .cloned()
            .collect::<Vec<_>>()
    } else {
        options.clone()
    };

    // 计算选中项标签
    let selected_label = if props.multiple {
        props
            .values
            .iter()
            .filter_map(|value| {
                options
                    .iter()
                    .find(|opt| &opt.value == value)
                    .map(|opt| opt.label.clone())
            })
            .collect::<Vec<_>>()
            .join(", ")
    } else {
        options
            .iter()
            .find(|opt| opt.value == props.value)
            .map(|opt| opt.label.clone())
            .unwrap_or_default()
    };

    // 构建类名
    let mut class_names = vec![
        "ant-select".to_string(),
        format!("ant-select-{}", props.size.to_class()),
        props.status.to_class().to_string(),
    ];

    if focused() {
        class_names.push("ant-select-focused".to_string());
    }

    if props.disabled {
        class_names.push("ant-select-disabled".to_string());
    }

    if dropdown_visible() {
        class_names.push("ant-select-open".to_string());
    }

    if props.multiple {
        class_names.push("ant-select-multiple".to_string());
    }

    if !props.bordered {
        class_names.push("ant-select-borderless".to_string());
    }

    if props.show_search {
        class_names.push("ant-select-show-search".to_string());
    }

    if let Some(class) = &props.class {
        class_names.push(class.clone());
    }

    let style = props.style.clone().unwrap_or_default();

    // 渲染组件
    rsx! {
        div {
            class: "{class_names.join(" ")}",
            style: "{style}",
            onclick: toggle_dropdown,
            tabindex: "0",
            onfocus: handle_focus,
            onblur: handle_blur,

            // 选择框
            div {
                class: "ant-select-selector",

                // 多选模式
                if props.multiple {
                    div {
                        class: "ant-select-selection-overflow",
                        for value in &props.values {
                            if let Some(option) = options.iter().find(|opt| &opt.value == value) {
                                div {
                                    class: "ant-select-selection-item",
                                    "{option.label}"
                                    if !props.disabled {
                                        span {
                                            class: "ant-select-selection-item-remove",
                                            onclick: move |e| {
                                                e.stop_propagation();
                                                let mut new_values = props.values.clone();
                                                new_values.retain(|v| v != value);
                                                if let Some(handler) = &props.on_change_multiple {
                                                    handler.call(new_values);
                                                }
                                            },
                                            "×"
                                        }
                                    }
                                }
                            }
                        }

                        // 搜索框
                        if props.show_search {
                            input {
                                class: "ant-select-selection-search-input",
                                placeholder: if props.values.is_empty() {
                                    props.placeholder.clone().unwrap_or_default()
                                } else {
                                    String::new()
                                },
                                value: search_value(),
                                oninput: handle_search_input,
                                disabled: props.disabled,
                                onclick: |e| e.stop_propagation(),
                            }
                        }
                    }
                } else {
                    // 单选模式
                    input {
                        class: "ant-select-selection-search-input",
                        value: if dropdown_visible() { search_value() } else { selected_label.clone() },
                        readonly: !props.show_search || dropdown_visible(),
                        placeholder: if props.value.is_empty() {
                            props.placeholder.clone().unwrap_or_default()
                        } else {
                            String::new()
                        },
                        oninput: handle_search_input,
                        disabled: props.disabled,
                        onclick: |e| e.stop_propagation(),
                    }
                }
            }

            // 清除按钮
            if props.allow_clear && !props.disabled && ((props.multiple && !props.values.is_empty()) || (!props.multiple && !props.value.is_empty())) {
                span {
                    class: "ant-select-clear",
                    onclick: move |e| {
                        e.stop_propagation();
                        handle_clear(());
                    },
                    "×"
                }
            }

            // 下拉箭头
            span {
                class: "ant-select-arrow",
                "▼"
            }

            // 下拉菜单
            if dropdown_visible() {
                div {
                    class: "ant-select-dropdown",
                    style: format!("width: 100%; max-height: {}px;", props.list_height),
                    onclick: move |e| e.stop_propagation(),
                    div {
                        class: "ant-select-dropdown-menu",
                        for option in &filtered_options {
                            div {
                                class: "ant-select-item ant-select-item-option",
                                class: {
                                    if option.disabled {
                                        "ant-select-item-option-disabled"
                                    } else if (props.multiple && props.values.contains(&option.value)) || (!props.multiple && props.value == option.value) {
                                        "ant-select-item-option-selected"
                                    } else {
                                        ""
                                    }
                                },
                                onclick: move |e| {
                                    e.stop_propagation();
                                    if !option.disabled {
                                        handle_option_click(option.value.clone());
                                    }
                                },
                                div {
                                    class: "ant-select-item-option-content",
                                    "{option.label}"
                                }
                            }
                        }

                        if filtered_options.is_empty() {
                            div {
                                class: "ant-select-item",
                                style: "cursor: default;",
                                "无匹配结果"
                            }
                        }
                    }
                }
            }
        }
    }
}

// 递归提取所有选项
fn extract_options(element: &VNode, options: &mut Vec<OptionData>) {
    match element {
        VNode::DynamicComponentNode(comp) => {
            // 检查是否是 SelectOption 组件
            if comp.name.contains("SelectOption") {
                let mut value = String::new();
                let mut disabled = false;
                let mut label = String::new();

                // 提取属性
                for (key, val) in &comp.props {
                    match key.as_str() {
                        "value" => {
                            if let Some(v) = val.as_str() {
                                value = v.to_string();
                            }
                        }
                        "disabled" => {
                            if let Some(d) = val.as_bool() {
                                disabled = *d;
                            }
                        }
                        _ => {}
                    }
                }

                // 提取子元素作为标签
                if let Some(children) = &comp.children {
                    for child in children.iter() {
                        if let VNode::Text(text) = child {
                            label = text.to_string();
                            break;
                        }
                    }
                }

                options.push(OptionData {
                    value,
                    label,
                    disabled,
                });
            }
            // SelectOptGroup 组件中提取子选项
            else if comp.name.contains("SelectOptGroup") {
                if let Some(children) = &comp.children {
                    for child in children.iter() {
                        extract_options(child, options);
                    }
                }
            }
            // 处理可能包含选项的其他组件
            else if let Some(children) = &comp.children {
                for child in children.iter() {
                    extract_options(child, options);
                }
            }
        }
        VNode::Fragment(children) => {
            for child in children {
                extract_options(child, options);
            }
        }
        _ => {}
    }
}

/// SelectOption 选项属性
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
#[component]
pub fn SelectOption(props: SelectOptionProps) -> Element {
    // SelectOption 不会直接渲染，而是被 Select 组件提取并使用
    // 但为了在组件树中保留它们，我们需要返回一个空的元素
    rsx! {
        div {
            style: "display: none;",
            value: "{props.value}",
            disabled: props.disabled,
            {props.children}
        }
    }
}

/// SelectOptGroup 选项组属性
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
#[component]
pub fn SelectOptGroup(props: SelectOptGroupProps) -> Element {
    // SelectOptGroup 不会直接渲染，而是被 Select 组件提取并使用
    // 但为了在组件树中保留它们，我们需要返回一个空的元素
    rsx! {
        div {
            style: "display: none;",
            label: "{props.label}",
            {props.children}
        }
    }
}

// 组件已通过#[component]宏自动导出
// 无需重新导出

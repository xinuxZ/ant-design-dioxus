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
    let mut search_value = use_signal(|| props.search_value.clone().unwrap_or_default());
    let mut dropdown_visible = use_signal(|| false);
    let mut focused = use_signal(|| false);

    // 提取所有选项
    let mut option_datas: Vec<OptionData> = vec![];
    if let Some(children) = props.children.as_ref() {
        extract_options(children, &mut option_datas);
    }

    // 当前选中项的标签
    let selected_label = if props.multiple {
        String::new() // 多选时不显示单个标签
    } else if props.value.is_empty() {
        String::new() // 未选中任何项
    } else {
        // 根据值查找选项标签
        option_datas
            .iter()
            .find(|option| option.value == props.value)
            .map(|option| option.label.clone())
            .unwrap_or_default()
    };

    // 多选下选中的选项
    let selected_options = if props.multiple {
        option_datas
            .iter()
            .filter(|option| props.values.contains(&option.value))
            .cloned()
            .collect::<Vec<_>>()
    } else {
        vec![]
    };

    // 处理选项点击
    let handle_option_click = move |value: String| {
        if props.disabled {
            return;
        }

        if props.multiple {
            let mut new_values = props.values.clone();
            if new_values.contains(&value) {
                new_values.retain(|v| v != &value);
            } else {
                new_values.push(value.clone());
            }

            if let Some(on_change_multiple) = &props.on_change_multiple {
                on_change_multiple.call(new_values);
            }

            if props.auto_clear_search_value {
                search_value.set(String::new());
            }
        } else {
            if let Some(on_change) = &props.on_change {
                on_change.call(value);
            }
            dropdown_visible.set(false);
            search_value.set(String::new());
        }
    };

    // 处理搜索输入
    let handle_search = move |e: Event<FormData>| {
        let value = e.value.clone();
        search_value.set(value.clone());

        if let Some(on_search) = &props.on_search {
            on_search.call(value);
        }
    };

    // 处理清除
    let handle_clear = move |_| {
        if props.disabled {
            return;
        }

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

        search_value.set(String::new());
    };

    // 处理下拉框显示/隐藏
    let toggle_dropdown = move |_| {
        if props.disabled {
            return;
        }

        let new_visible = !dropdown_visible.get();
        dropdown_visible.set(new_visible);

        if let Some(on_dropdown_visible_change) = &props.on_dropdown_visible_change {
            on_dropdown_visible_change.call(new_visible);
        }
    };

    // 处理焦点
    let handle_focus = move |e: FocusEvent| {
        if props.disabled {
            return;
        }

        focused.set(true);

        if let Some(on_focus) = &props.on_focus {
            on_focus.call(e);
        }
    };

    // 处理失焦
    let handle_blur = move |e: FocusEvent| {
        focused.set(false);

        if let Some(on_blur) = &props.on_blur {
            on_blur.call(e);
        }
    };

    // 筛选选项
    let filtered_options = if props.filter_option && search_value.get().len() > 0 {
        option_datas
            .iter()
            .filter(|option| {
                option
                    .label
                    .to_lowercase()
                    .contains(&search_value.get().to_lowercase())
            })
            .cloned()
            .collect::<Vec<_>>()
    } else {
        option_datas.clone()
    };

    // CSS 类名
    let select_class = {
        let mut classes = vec!["ant-select"];

        // 尺寸
        let size_class = props.size.to_class();
        if !size_class.is_empty() {
            classes.push(size_class);
        }

        // 状态
        let status_class = props.status.to_class();
        if !status_class.is_empty() {
            classes.push(status_class);
        }

        // 禁用
        if props.disabled {
            classes.push("ant-select-disabled");
        }

        // 聚焦
        if focused.get() {
            classes.push("ant-select-focused");
        }

        // 下拉框显示
        if dropdown_visible.get() {
            classes.push("ant-select-open");
        }

        // 多选
        if props.multiple {
            classes.push("ant-select-multiple");
        }

        // 搜索
        if props.show_search {
            classes.push("ant-select-show-search");
        }

        // 无边框
        if !props.bordered {
            classes.push("ant-select-borderless");
        }

        // 自定义类名
        if let Some(class) = &props.class {
            classes.push(class);
        }

        classes.join(" ")
    };

    // 生成CSS样式
    let style_builder = SelectStyleBuilder::new()
        .size(props.size.into())
        .status(props.status.into())
        .disabled(props.disabled)
        .multiple(props.multiple)
        .show_search(props.show_search)
        .bordered(props.bordered);

    let select_styles = style_builder.build();

    rsx! {
        style { {select_styles} }
        div {
            class: "{select_class}",
            style: props.style.clone().unwrap_or_default(),
            onclick: toggle_dropdown,
            div {
                class: "ant-select-selector",
                if props.multiple {
                    // 多选模式
                    div {
                        class: "ant-select-selection-overflow",
                        for option in &selected_options {
                            div {
                                class: "ant-select-selection-overflow-item",
                                div {
                                    class: "ant-select-selection-item",
                                    span {
                                        class: "ant-select-selection-item-content",
                                        "{option.label}"
                                    }
                                    span {
                                        class: "ant-select-selection-item-remove",
                                        onclick: move |e| {
                                            e.stop_propagation();
                                            handle_option_click(option.value.clone());
                                        },
                                        "×"
                                    }
                                }
                            }
                        }
                        if props.show_search {
                            div {
                                class: "ant-select-selection-overflow-item",
                                div {
                                    class: "ant-select-selection-search",
                                    input {
                                        class: "ant-select-selection-search-input",
                                        value: search_value.get(),
                                        placeholder: if selected_options.is_empty() {
                                            props.placeholder.clone().unwrap_or_default()
                                        } else {
                                            String::new()
                                        },
                                        oninput: handle_search,
                                        onfocus: handle_focus,
                                        onblur: handle_blur,
                                    }
                                }
                            }
                        } else if selected_options.is_empty() {
                            span {
                                class: "ant-select-selection-placeholder",
                                "{props.placeholder.clone().unwrap_or_default()}"
                            }
                        }
                    }
                } else {
                    // 单选模式
                    if props.show_search {
                        div {
                            class: "ant-select-selection-search",
                            input {
                                class: "ant-select-selection-search-input",
                                value: if dropdown_visible.get() { search_value.get() } else { selected_label.clone() },
                                placeholder: if selected_label.is_empty() { props.placeholder.clone().unwrap_or_default() } else { String::new() },
                                oninput: handle_search,
                                onfocus: handle_focus,
                                onblur: handle_blur,
                            }
                        }
                    } else {
                        span {
                            class: "ant-select-selection-item",
                            "{selected_label}"
                        }
                        if selected_label.is_empty() {
                            span {
                                class: "ant-select-selection-placeholder",
                                "{props.placeholder.clone().unwrap_or_default()}"
                            }
                        }
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
            if dropdown_visible.get() {
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
        VNode::Component(comp) => {
            // 检查是否是 SelectOption 组件
            if comp.name.contains("SelectOption") {
                let mut value = String::new();
                let mut disabled = false;
                let mut label = String::new();

                // 提取属性
                for (key, val) in comp.props.iter() {
                    match key.as_str() {
                        "value" => {
                            if let Some(v) = val.as_string() {
                                value = v.to_string();
                            }
                        }
                        "disabled" => {
                            if let Some(d) = val.as_bool() {
                                disabled = d;
                            }
                        }
                        _ => {}
                    }
                }

                // 提取子元素作为标签
                if let Some(children) = &comp.children {
                    for child in children.iter() {
                        if let VNode::Text(text) = child {
                            label = text.clone();
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
        VNode::Fragment(frag) => {
            for child in frag.children.iter() {
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

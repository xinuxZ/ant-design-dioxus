//! Cascader 级联选择组件
//!
//! 级联选择框。
//!
//! ## 何时使用
//!
//! - 需要从一组相关联的数据集合进行选择，例如省市区，公司层级，事物分类等。
//! - 从一个较大的数据集合中进行选择时，用多级分类进行分隔，方便选择。
//! - 比起 Select 组件，可以在同一个浮层中完成选择，有较好的体验。

use crate::utils::class_names::conditional_class_names_array;
use dioxus::prelude::*;
use std::collections::HashMap;

/// 级联选择器选项
#[derive(Debug, Clone, PartialEq)]
pub struct CascaderOption {
    pub value: String,
    pub label: String,
    pub children: Option<Vec<CascaderOption>>,
    pub disabled: bool,
    pub is_leaf: bool,
    /// 是否正在加载子节点
    pub loading: bool,
    /// 自定义数据
    pub data: HashMap<String, String>,
}

impl CascaderOption {
    pub fn new(value: &str, label: &str) -> Self {
        Self {
            value: value.to_string(),
            label: label.to_string(),
            children: None,
            disabled: false,
            is_leaf: false,
            loading: false,
            data: HashMap::new(),
        }
    }

    pub fn with_children(mut self, children: Vec<CascaderOption>) -> Self {
        self.children = Some(children);
        self
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    pub fn leaf(mut self) -> Self {
        self.is_leaf = true;
        self
    }

    pub fn loading(mut self) -> Self {
        self.loading = true;
        self
    }

    pub fn with_data(mut self, key: &str, value: &str) -> Self {
        self.data.insert(key.to_string(), value.to_string());
        self
    }

    /// 获取所有叶子节点的路径
    pub fn get_leaf_paths(&self, current_path: &[String]) -> Vec<Vec<String>> {
        let mut paths = Vec::new();
        let mut path = current_path.to_vec();
        path.push(self.value.clone());

        if let Some(children) = &self.children {
            if children.is_empty() || self.is_leaf {
                paths.push(path);
            } else {
                for child in children {
                    paths.extend(child.get_leaf_paths(&path));
                }
            }
        } else {
            paths.push(path);
        }

        paths
    }
}

/// 级联选择尺寸
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CascaderSize {
    /// 小尺寸
    Small,
    /// 中等尺寸
    Middle,
    /// 大尺寸
    Large,
}

impl Default for CascaderSize {
    fn default() -> Self {
        CascaderSize::Middle
    }
}

/// 级联选择状态
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CascaderStatus {
    /// 默认状态
    Default,
    /// 错误状态
    Error,
    /// 警告状态
    Warning,
}

impl Default for CascaderStatus {
    fn default() -> Self {
        CascaderStatus::Default
    }
}

/// 展开触发方式
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CascaderExpandTrigger {
    /// 点击触发
    Click,
    /// 悬停触发
    Hover,
}

impl Default for CascaderExpandTrigger {
    fn default() -> Self {
        CascaderExpandTrigger::Click
    }
}

/// 级联选择器组件
#[component]
pub fn Cascader(
    /// 当前选中的值
    value: Option<Vec<String>>,
    /// 默认选中的值
    default_value: Option<Vec<String>>,
    /// 可选项数据源
    options: Vec<CascaderOption>,
    /// 输入框占位文本
    #[props(default = "请选择".to_string())]
    placeholder: String,
    /// 是否禁用
    #[props(default = false)]
    disabled: bool,
    /// 是否支持清除
    #[props(default = true)]
    allow_clear: bool,
    /// 自动获取焦点
    #[props(default = false)]
    auto_focus: bool,
    /// 次级菜单的展开方式
    #[props(default = "click".to_string())]
    expand_trigger: String,
    /// 是否支持多选
    #[props(default = false)]
    multiple: bool,
    /// 是否显示搜索框
    #[props(default = false)]
    show_search: bool,
    /// 输入框大小
    #[props(default = "middle".to_string())]
    size: String,
    /// 设置校验状态
    status: Option<String>,
    /// 自定义显示渲染函数
    display_render: Option<fn(&Vec<String>) -> String>,
    /// 选择完成后的回调
    on_change: Option<EventHandler<Vec<String>>>,
    /// 显示/隐藏浮层的回调
    on_popup_visible_change: Option<EventHandler<bool>>,
    /// 监听搜索，返回输入的值
    on_search: Option<EventHandler<String>>,
    /// 用于动态加载选项
    load_data: Option<EventHandler<Vec<String>>>,
    /// 自定义类名
    class: Option<String>,
    /// 自定义样式
    style: Option<String>,
) -> Element {
    // 克隆所有需要在闭包中使用的props，确保'static生命周期
    let options_static = options.clone();
    let placeholder_static = placeholder.clone();
    let expand_trigger_static = expand_trigger.clone();
    let on_change_static = on_change.clone();
    let on_search_static = on_search.clone();
    let on_popup_visible_change_static = on_popup_visible_change.clone();
    let load_data_static = load_data.clone();

    // 状态管理
    let mut selected_value =
        use_signal(|| value.clone().or(default_value.clone()).unwrap_or_default());
    let mut active_path = use_signal(|| Vec::<usize>::new());
    let mut is_open = use_signal(|| false);
    let mut search_value = use_signal(|| String::new());
    let mut filtered_options = use_signal(|| options_static.clone());

    // 搜索过滤效果
    {
        let options_clone = options_static.clone();
        use_effect(move || {
            let search = search_value.read();
            if show_search && !search.is_empty() {
                let filtered = filter_options(&options_clone, &search);
                filtered_options.set(filtered);
            } else {
                filtered_options.set(options_clone.clone());
            }
        });
    }

    // 获取显示文本
    let get_display_text = {
        let options_clone = options_static.clone();
        let placeholder_clone = placeholder_static.clone();
        let display_render_clone = display_render;
        move || {
            let value = selected_value.read();
            if value.is_empty() {
                return placeholder_clone.clone();
            }

            if let Some(render) = display_render_clone {
                render(&value)
            } else {
                get_option_labels(&options_clone, &value).join(" / ")
            }
        }
    };

    // 处理选项点击
    let handle_option_click = {
        let options_clone = options_static.clone();
        let on_change_clone = on_change_static.clone();
        let load_data_clone = load_data_static.clone();
        move |(path, option): (Vec<usize>, CascaderOption)| {
            let mut new_path = Vec::new();
            let mut current_options = &options_clone;

            for &index in &path {
                if let Some(opt) = current_options.get(index) {
                    new_path.push(opt.value.clone());
                    if let Some(children) = &opt.children {
                        current_options = children;
                    }
                }
            }

            new_path.push(option.value.clone());

            // 如果是叶子节点或没有子节点，完成选择
            if option.is_leaf
                || option.children.is_none()
                || option.children.as_ref().unwrap().is_empty()
            {
                selected_value.set(new_path.clone());
                is_open.set(false);
                if let Some(handler) = on_change_clone {
                    handler.call(new_path);
                }
            } else {
                // 更新活动路径
                let mut new_active_path = path;
                new_active_path.push(0); // 选择第一个子选项
                active_path.set(new_active_path);

                // 如果有动态加载函数，调用它
                if let Some(handler) = load_data_clone {
                    handler.call(new_path);
                }
            }
        }
    };

    // 处理搜索
    let handle_search = {
        let on_search_clone = on_search_static.clone();
        move |evt: FormEvent| {
            let value = evt.value();
            search_value.set(value.clone());

            if let Some(callback) = on_search_clone {
                callback.call(value);
            }
        }
    };

    // 处理清除
    let handle_clear = {
        let on_change_clone = on_change_static.clone();
        move |_| {
            selected_value.set(Vec::new());
            search_value.set(String::new());
            active_path.set(Vec::new());

            if let Some(callback) = on_change_clone {
                callback.call(Vec::new());
            }
        }
    };

    // 切换下拉菜单
    let toggle_dropdown = {
        let on_popup_visible_change_clone = on_popup_visible_change_static.clone();
        move |_| {
            if disabled {
                return;
            }

            let new_open = !*is_open.read();
            is_open.set(new_open);

            if let Some(callback) = on_popup_visible_change_clone {
                callback.call(new_open);
            }
        }
    };

    let container_class = conditional_class_names_array(&[
        ("ant-cascader", true),
        ("ant-cascader-open", *is_open.read()),
        ("ant-cascader-disabled", disabled),
        ("ant-cascader-multiple", multiple),
        ("ant-cascader-small", size == "small"),
        ("ant-cascader-large", size == "large"),
        (
            "ant-cascader-status-error",
            status.as_ref().map_or(false, |s| s == "error"),
        ),
        (
            "ant-cascader-status-warning",
            status.as_ref().map_or(false, |s| s == "warning"),
        ),
        (
            class.as_ref().map_or("", |c| c),
            class.as_ref().map_or(false, |c| !c.is_empty()),
        ),
    ]);

    let selector_class = conditional_class_names_array(&[
        ("ant-cascader-selector", true),
        ("ant-cascader-selector-disabled", disabled),
        ("ant-cascader-selector-small", size == "small"),
        ("ant-cascader-selector-large", size == "large"),
        (
            "ant-cascader-selector-status-error",
            status.as_ref().map_or(false, |s| s == "error"),
        ),
        (
            "ant-cascader-selector-status-warning",
            status.as_ref().map_or(false, |s| s == "warning"),
        ),
    ]);

    rsx! {
        div {
            class: "{container_class}",
            style: "{style.as_ref().map_or(\"\", |s| s)}",

            // 选择器
            div {
                class: "{selector_class}",
                onclick: toggle_dropdown,

                // 搜索输入框
                if show_search {
                    input {
                        class: "ant-cascader-input",
                        r#type: "text",
                        value: "{search_value.read()}",
                        placeholder: "{placeholder_static}",
                        disabled: disabled,
                        autofocus: auto_focus,
                        oninput: handle_search,
                    }
                } else {
                    // 显示选中值
                    span { class: "ant-cascader-selection-item",
                        "{get_display_text()}"
                    }
                }

                // 清除按钮
                if allow_clear && !selected_value.read().is_empty() && !disabled {
                    span {
                        class: "ant-cascader-clear",
                        onclick: handle_clear,
                        "×"
                    }
                }

                // 下拉箭头
                span { class: "ant-cascader-arrow", "▼" }
            }

            // 下拉菜单
            if *is_open.read() {
                div { class: "ant-cascader-dropdown",
                    CascaderMenu {
                        options: filtered_options.read().clone(),
                        active_path: active_path.read().clone(),
                        selected_value: selected_value.read().clone(),
                        expand_trigger: expand_trigger_static.clone(),
                        on_option_click: handle_option_click,
                    }
                }
            }
        }
    }
}

/// 级联菜单组件属性
#[derive(Props, Clone, PartialEq)]
struct CascaderMenuProps {
    options: Vec<CascaderOption>,
    active_path: Vec<usize>,
    selected_value: Vec<String>,
    expand_trigger: String,
    on_option_click: EventHandler<(Vec<usize>, CascaderOption)>,
}

/// 级联菜单组件
#[component]
fn CascaderMenu(props: CascaderMenuProps) -> Element {
    // 克隆所有需要的props字段以避免借用冲突
    let options = props.options.clone();
    let active_path = props.active_path.clone();
    let expand_trigger = props.expand_trigger;
    let on_option_click = props.on_option_click.clone();

    let mut hover_path = use_signal(|| Vec::<usize>::new());

    // 渲染菜单列
    let render_menu_column = {
        let active_path = active_path.clone();
        let expand_trigger = expand_trigger;
        let on_option_click = on_option_click.clone();
        move |options: &[CascaderOption], level: usize| {
            rsx! {
                    ul { class: "ant-cascader-menu",
                        for (index, option) in options.iter().enumerate() {
                            li {
                                key: format!("{}-{}-{}", option.value, level, index),
                                class: conditional_class_names_array(&[
                                    ("ant-cascader-menu-item", true),
                                    ("ant-cascader-menu-item-disabled", option.disabled),
                                    ("ant-cascader-menu-item-active", active_path.get(level) == Some(&index)),
                                    ("ant-cascader-menu-item-expand", option.children.is_some()),
                                    ("ant-cascader-menu-item-loading", option.loading),
                                ]),
                                onclick: {
                                    let option = option.clone();
                                    let active_path = active_path.clone();
                                    let on_option_click = on_option_click.clone();
                                    move |_| {
                                        if !option.disabled {
                                            let mut click_path = active_path[..level].to_vec();
                                            click_path.push(index);
                                            on_option_click.call((click_path, option.clone()));
                                        }
                                    }
                                },
                                onmouseenter: {
                                    let option_disabled = option.disabled;
                                    let active_path = active_path.clone();
                                    let expand_trigger = expand_trigger.clone();
                                move |_| {
                                    if expand_trigger == "hover" && !option_disabled {
                                        let mut new_path = active_path[..level].to_vec();
                                        new_path.push(index);
                                        hover_path.set(new_path);
                                    }
                                }
                            },

                            span { class: "ant-cascader-menu-item-content",
                                "{option.label}"
                            }

                            if option.children.is_some() {
                                span { class: "ant-cascader-menu-item-expand-icon", "▶" }
                            }

                            if option.loading {
                                span { class: "ant-cascader-menu-item-loading-icon", "⟳" }
                            }
                        }
                    }
                }
            }
        }
    };

    // 获取当前显示的菜单列
    let get_menu_columns = {
        let active_path = active_path.clone();
        let options_clone = options.clone();
        move || {
            let mut columns = vec![options_clone.clone()];
            let mut current_options = &options_clone;

            for &index in &active_path {
                if let Some(option) = current_options.get(index) {
                    if let Some(children) = &option.children {
                        columns.push(children.clone());
                        current_options = children;
                    }
                }
            }

            columns
        }
    };

    let menu_columns = get_menu_columns();
    rsx! {
        div { class: "ant-cascader-menus",
            for (level, options) in menu_columns.iter().enumerate() {
                {render_menu_column(options, level)}
            }
        }
    }
}

// 辅助函数

/// 过滤选项
fn filter_options(options: &[CascaderOption], search: &str) -> Vec<CascaderOption> {
    let mut filtered = Vec::new();

    for option in options {
        if option.label.to_lowercase().contains(&search.to_lowercase())
            || option.value.to_lowercase().contains(&search.to_lowercase())
        {
            filtered.push(option.clone());
        } else if let Some(children) = &option.children {
            let filtered_children = filter_options(children, search);
            if !filtered_children.is_empty() {
                let mut new_option = option.clone();
                new_option.children = Some(filtered_children);
                filtered.push(new_option);
            }
        }
    }

    filtered
}

/// 根据值路径获取标签
fn get_option_labels(options: &[CascaderOption], values: &[String]) -> Vec<String> {
    let mut labels = Vec::new();
    let mut current_options = options;

    for value in values {
        if let Some(option) = current_options.iter().find(|opt| opt.value == *value) {
            labels.push(option.label.clone());
            if let Some(children) = &option.children {
                current_options = children;
            }
        }
    }

    labels
}

/// 级联选择选项构建器
pub struct CascaderOptionBuilder {
    option: CascaderOption,
}

impl CascaderOptionBuilder {
    pub fn new(value: &str, label: &str) -> Self {
        Self {
            option: CascaderOption::new(value, label),
        }
    }

    pub fn with_children(mut self, children: Vec<CascaderOption>) -> Self {
        self.option.children = Some(children);
        self
    }

    pub fn disabled(mut self) -> Self {
        self.option.disabled = true;
        self
    }

    pub fn leaf(mut self) -> Self {
        self.option.is_leaf = true;
        self
    }

    pub fn loading(mut self) -> Self {
        self.option.loading = true;
        self
    }

    pub fn with_data(mut self, key: &str, value: &str) -> Self {
        self.option.data.insert(key.to_string(), value.to_string());
        self
    }

    pub fn build(self) -> CascaderOption {
        self.option
    }
}

/// 便捷的级联选择选项创建宏
#[macro_export]
macro_rules! cascader_option {
    ($value:expr, $label:expr) => {
        CascaderOption::new($value, $label)
    };
    ($value:expr, $label:expr, children: [$($child:expr),*]) => {
        CascaderOption::new($value, $label).with_children(vec![$($child),*])
    };
    ($value:expr, $label:expr, disabled) => {
        CascaderOption::new($value, $label).disabled()
    };
    ($value:expr, $label:expr, leaf) => {
        CascaderOption::new($value, $label).leaf()
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cascader_option_new() {
        let option = CascaderOption::new("value1", "Label 1");
        assert_eq!(option.value, "value1");
        assert_eq!(option.label, "Label 1");
        assert!(option.children.is_none());
        assert!(!option.disabled);
        assert!(!option.is_leaf);
        assert!(!option.loading);
    }

    #[test]
    fn test_cascader_option_with_children() {
        let child = CascaderOption::new("child1", "Child 1");
        let option = CascaderOption::new("parent", "Parent").with_children(vec![child]);
        assert!(option.children.is_some());
        assert_eq!(option.children.unwrap().len(), 1);
    }

    #[test]
    fn test_cascader_option_modifiers() {
        let option = CascaderOption::new("value1", "Label 1")
            .disabled()
            .leaf()
            .loading()
            .with_data("key1", "value1");

        assert!(option.disabled);
        assert!(option.is_leaf);
        assert!(option.loading);
        assert_eq!(option.data.get("key1"), Some(&"value1".to_string()));
    }

    #[test]
    fn test_cascader_size_default() {
        assert_eq!(CascaderSize::default(), CascaderSize::Middle);
    }

    #[test]
    fn test_cascader_status_default() {
        assert_eq!(CascaderStatus::default(), CascaderStatus::Default);
    }

    #[test]
    fn test_cascader_expand_trigger_default() {
        assert_eq!(
            CascaderExpandTrigger::default(),
            CascaderExpandTrigger::Click
        );
    }

    #[test]
    fn test_get_leaf_paths() {
        let child1 = CascaderOption::new("child1", "Child 1").leaf();
        let child2 = CascaderOption::new("child2", "Child 2").leaf();
        let parent = CascaderOption::new("parent", "Parent").with_children(vec![child1, child2]);

        let paths = parent.get_leaf_paths(&[]);
        assert_eq!(paths.len(), 2);
        assert_eq!(paths[0], vec!["parent".to_string(), "child1".to_string()]);
        assert_eq!(paths[1], vec!["parent".to_string(), "child2".to_string()]);
    }

    #[test]
    fn test_filter_options() {
        let options = vec![
            CascaderOption::new("option1", "Option 1"),
            CascaderOption::new("option2", "Test Option"),
            CascaderOption::new("option3", "Another"),
        ];

        let filtered = filter_options(&options, "test");
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].value, "option2");
    }

    #[test]
    fn test_get_option_labels() {
        let child = CascaderOption::new("child1", "Child 1");
        let parent = CascaderOption::new("parent", "Parent").with_children(vec![child]);
        let options = vec![parent];

        let labels = get_option_labels(&options, &["parent".to_string(), "child1".to_string()]);
        assert_eq!(labels, vec!["Parent".to_string(), "Child 1".to_string()]);
    }
}

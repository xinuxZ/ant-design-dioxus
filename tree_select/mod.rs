//! TreeSelect 树选择组件
//!
//! 树形选择控件，类似于 Select 组件，但是值以树形结构提供。
//! 适用于任何具有层次结构的数据。

use dioxus::prelude::*;
use std::collections::HashMap;

const TREE_SELECT_STYLE: &str = include_str!("./style.css");

/// 树节点数据结构
#[derive(Debug, Clone, PartialEq)]
pub struct TreeNode {
    /// 节点的唯一标识
    pub value: String,
    /// 节点显示的标题
    pub title: String,
    /// 子节点列表
    pub children: Option<Vec<TreeNode>>,
    /// 是否禁用该节点
    pub disabled: bool,
    /// 是否可选择（在 checkable 模式下）
    pub selectable: bool,
    /// 是否可勾选
    pub checkable: bool,
    /// 是否禁用勾选框
    pub disable_checkbox: bool,
}

impl TreeNode {
    /// 创建新的树节点
    pub fn new(value: String, title: String) -> Self {
        Self {
            value,
            title,
            children: None,
            disabled: false,
            selectable: true,
            checkable: true,
            disable_checkbox: false,
        }
    }

    /// 添加子节点
    pub fn with_children(mut self, children: Vec<TreeNode>) -> Self {
        self.children = Some(children);
        self
    }

    /// 设置禁用状态
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// 设置可选择状态
    pub fn with_selectable(mut self, selectable: bool) -> Self {
        self.selectable = selectable;
        self
    }
}

/// 选中策略枚举
#[derive(Debug, Clone, PartialEq)]
pub enum ShowCheckedStrategy {
    /// 显示所有选中的节点（包括父节点）
    ShowAll,
    /// 只显示父节点
    ShowParent,
    /// 只显示子节点（默认）
    ShowChild,
}

/// 组件尺寸
#[derive(Debug, Clone, PartialEq)]
pub enum TreeSelectSize {
    Large,
    Middle,
    Small,
}

/// 状态类型
#[derive(Debug, Clone, PartialEq)]
pub enum TreeSelectStatus {
    Error,
    Warning,
}

/// 展开动作类型
#[derive(Debug, Clone, PartialEq)]
pub enum TreeExpandAction {
    False,
    Click,
    DoubleClick,
}

/// TreeSelect 组件属性
#[derive(Props)]
pub struct TreeSelectProps {
    /// 是否显示清除图标
    #[props(default = false)]
    pub allow_clear: bool,

    /// 多选时是否自动清除搜索值
    #[props(default = true)]
    pub auto_clear_search_value: bool,

    /// 自定义类名
    #[props(default = String::new())]
    pub class_name: String,

    /// 默认展开状态
    #[props(default = false)]
    pub default_open: bool,

    /// 默认选中的值
    pub default_value: Option<Vec<String>>,

    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,

    /// 下拉菜单类名
    #[props(default = String::new())]
    pub popup_class_name: String,

    /// 下拉菜单是否与选择器同宽
    #[props(default = true)]
    pub popup_match_select_width: bool,

    /// 自定义字段名
    pub field_names: Option<HashMap<String, String>>,

    /// 是否根据输入项进行筛选
    pub filter_tree_node: Option<std::rc::Rc<dyn Fn(&str, &TreeNode) -> bool>>,

    /// 是否在值中嵌入标签
    #[props(default = false)]
    pub label_in_value: bool,

    /// 下拉菜单的高度
    #[props(default = 256)]
    pub list_height: i32,

    /// 异步加载数据
    pub load_data: Option<Box<dyn Fn(&TreeNode)>>,

    /// 最多显示多少个 tag
    pub max_tag_count: Option<i32>,

    /// 最多选择的项目数量
    pub max_count: Option<i32>,

    /// 隐藏 tag 时显示的内容
    pub max_tag_placeholder: Option<String>,

    /// 最大 tag 文本长度
    pub max_tag_text_length: Option<i32>,

    /// 是否支持多选
    #[props(default = false)]
    pub multiple: bool,

    /// 当下拉列表为空时显示的内容
    #[props(default = "Not Found".to_string())]
    pub not_found_content: String,

    /// 是否展开下拉菜单
    pub open: Option<bool>,

    /// 输入框占位文本
    #[props(default = String::new())]
    pub placeholder: String,

    /// 选择框弹出的位置
    #[props(default = "bottomLeft".to_string())]
    pub placement: String,

    /// 自定义的选择框前缀图标
    pub prefix: Option<Element>,

    /// 搜索框的值
    pub search_value: Option<String>,

    /// 定义选中项回填的方式
    #[props(default = ShowCheckedStrategy::ShowChild)]
    pub show_checked_strategy: ShowCheckedStrategy,

    /// 是否显示搜索框
    pub show_search: Option<bool>,

    /// 选择框大小
    pub size: Option<TreeSelectSize>,

    /// 设置校验状态
    pub status: Option<TreeSelectStatus>,

    /// 自定义的选择框后缀图标
    pub suffix_icon: Option<Element>,

    /// 自定义树节点的展开/折叠图标
    pub switcher_icon: Option<Element>,

    /// 自定义 tag 内容的渲染
    pub tag_render: Option<Box<dyn Fn(&str) -> Element>>,

    /// 是否显示 checkbox
    #[props(default = false)]
    pub tree_checkable: bool,

    /// checkable 状态下节点选择完全受控
    #[props(default = false)]
    pub tree_check_strictly: bool,

    /// treeNodes 数据
    #[props(default = vec![])]
    pub tree_data: Vec<TreeNode>,

    /// 使用简单格式的 treeData
    #[props(default = false)]
    pub tree_data_simple_mode: bool,

    /// 自定义树节点标题渲染
    pub tree_title_render: Option<Box<dyn Fn(&TreeNode) -> Element>>,

    /// 默认展开所有树节点
    #[props(default = false)]
    pub tree_default_expand_all: bool,

    /// 默认展开的树节点
    pub tree_default_expanded_keys: Option<Vec<String>>,

    /// 点击树节点标题时的展开逻辑
    #[props(default = TreeExpandAction::False)]
    pub tree_expand_action: TreeExpandAction,

    /// 设置展开的树节点
    pub tree_expanded_keys: Option<Vec<String>>,

    /// 是否展示 TreeNode title 前的图标
    #[props(default = false)]
    pub tree_icon: bool,

    /// 已经加载的节点
    pub tree_loaded_keys: Option<Vec<String>>,

    /// 是否展示连接线
    #[props(default = false)]
    pub tree_line: bool,

    /// 树节点过滤属性
    pub tree_node_filter_prop: Option<String>,

    /// 树节点标签属性
    pub tree_node_label_prop: Option<String>,

    /// 当前选中的值
    pub value: Option<Vec<String>>,

    /// 是否开启虚拟滚动
    #[props(default = true)]
    pub r#virtual: bool,

    /// 选中值发生变化时的回调
    pub on_change: Option<Box<dyn Fn(Vec<String>, Vec<String>, HashMap<String, String>)>>,

    /// 下拉框出现/隐藏时的回调
    pub on_dropdown_visible_change: Option<Box<dyn Fn(bool)>>,

    /// 搜索时的回调
    pub on_search: Option<Box<dyn Fn(String)>>,

    /// 选择/取消选择时的回调
    pub on_select: Option<Box<dyn Fn(String, bool, HashMap<String, String>)>>,

    /// 展开/收起节点时的回调
    pub on_tree_expand: Option<Box<dyn Fn(Vec<String>)>>,
}

// 手动实现 Clone trait
impl Clone for TreeSelectProps {
    fn clone(&self) -> Self {
        Self {
            allow_clear: self.allow_clear,
            auto_clear_search_value: self.auto_clear_search_value,
            class_name: self.class_name.clone(),
            default_open: self.default_open,
            default_value: self.default_value.clone(),
            disabled: self.disabled,
            popup_class_name: self.popup_class_name.clone(),
            popup_match_select_width: self.popup_match_select_width,
            field_names: self.field_names.clone(),
            filter_tree_node: self.filter_tree_node.clone(),
            label_in_value: self.label_in_value,
            list_height: self.list_height,
            load_data: None, // 函数指针无法克隆，设为 None
            max_tag_count: self.max_tag_count,
            max_count: self.max_count,
            max_tag_placeholder: self.max_tag_placeholder.clone(),
            max_tag_text_length: self.max_tag_text_length,
            multiple: self.multiple,
            not_found_content: self.not_found_content.clone(),
            open: self.open,
            placeholder: self.placeholder.clone(),
            placement: self.placement.clone(),
            prefix: None, // Element 无法克隆
            search_value: self.search_value.clone(),
            show_checked_strategy: self.show_checked_strategy.clone(),
            show_search: self.show_search,
            size: self.size.clone(),
            status: self.status.clone(),
            suffix_icon: None,   // Element 无法克隆
            switcher_icon: None, // Element 无法克隆
            tag_render: None,    // 函数指针无法克隆
            tree_checkable: self.tree_checkable,
            tree_check_strictly: self.tree_check_strictly,
            tree_data: self.tree_data.clone(),
            tree_data_simple_mode: self.tree_data_simple_mode,
            tree_default_expand_all: self.tree_default_expand_all,
            tree_default_expanded_keys: self.tree_default_expanded_keys.clone(),
            tree_expanded_keys: self.tree_expanded_keys.clone(),
            tree_title_render: None, // 函数指针无法克隆
            tree_expand_action: self.tree_expand_action.clone(),
            tree_loaded_keys: self.tree_loaded_keys.clone(),
            tree_icon: self.tree_icon,
            tree_line: self.tree_line,
            tree_node_filter_prop: self.tree_node_filter_prop.clone(),
            tree_node_label_prop: self.tree_node_label_prop.clone(),
            value: self.value.clone(),
            r#virtual: self.r#virtual,
            on_change: None,                  // 函数指针无法克隆
            on_dropdown_visible_change: None, // 函数指针无法克隆
            on_search: None,                  // 函数指针无法克隆
            on_select: None,                  // 函数指针无法克隆
            on_tree_expand: None,             // 函数指针无法克隆
        }
    }
}

// 手动实现 PartialEq trait
impl PartialEq for TreeSelectProps {
    fn eq(&self, other: &Self) -> bool {
        self.allow_clear == other.allow_clear
            && self.auto_clear_search_value == other.auto_clear_search_value
            && self.class_name == other.class_name
            && self.default_open == other.default_open
            && self.default_value == other.default_value
            && self.disabled == other.disabled
            && self.popup_class_name == other.popup_class_name
            && self.popup_match_select_width == other.popup_match_select_width
            && self.field_names == other.field_names
            && self.label_in_value == other.label_in_value
            && self.list_height == other.list_height
            && self.max_tag_count == other.max_tag_count
            && self.max_count == other.max_count
            && self.max_tag_placeholder == other.max_tag_placeholder
            && self.max_tag_text_length == other.max_tag_text_length
            && self.multiple == other.multiple
            && self.not_found_content == other.not_found_content
            && self.open == other.open
            && self.placeholder == other.placeholder
            && self.placement == other.placement
            && self.search_value == other.search_value
            && self.show_checked_strategy == other.show_checked_strategy
            && self.show_search == other.show_search
            && self.size == other.size
            && self.status == other.status
            && self.tree_checkable == other.tree_checkable
            && self.tree_check_strictly == other.tree_check_strictly
            && self.tree_data == other.tree_data
            && self.tree_data_simple_mode == other.tree_data_simple_mode
            && self.tree_default_expand_all == other.tree_default_expand_all
            && self.tree_default_expanded_keys == other.tree_default_expanded_keys
            && self.tree_expanded_keys == other.tree_expanded_keys
            && self.tree_line == other.tree_line
            && self.tree_expand_action == other.tree_expand_action
            && self.tree_loaded_keys == other.tree_loaded_keys
            && self.tree_icon == other.tree_icon
            && self.tree_node_filter_prop == other.tree_node_filter_prop
            && self.tree_node_label_prop == other.tree_node_label_prop
            && self.value == other.value
            && self.r#virtual == other.r#virtual
        // 函数指针和 Element 无法比较，忽略
    }
}

/// TreeSelect 组件
#[component]
pub fn TreeSelect(props: TreeSelectProps) -> Element {
    // 内部状态管理
    let mut open = use_signal(|| props.open.unwrap_or(props.default_open));
    let mut selected_values = use_signal(|| {
        props
            .value
            .clone()
            .unwrap_or_else(|| props.default_value.clone().unwrap_or_default())
    });
    let mut search_value = use_signal(|| props.search_value.clone().unwrap_or_default());
    let mut expanded_keys = use_signal(|| {
        props.tree_expanded_keys.clone().unwrap_or_else(|| {
            if props.tree_default_expand_all {
                collect_all_keys(&props.tree_data)
            } else {
                props.tree_default_expanded_keys.clone().unwrap_or_default()
            }
        })
    });

    // 计算样式类名
    let class_name = format!(
        "ant-select ant-tree-select {} {} {} {} {}",
        if props.disabled {
            "ant-select-disabled"
        } else {
            ""
        },
        if open() { "ant-select-open" } else { "" },
        if props.multiple {
            "ant-select-multiple"
        } else {
            "ant-select-single"
        },
        match props.size {
            Some(TreeSelectSize::Large) => "ant-select-lg",
            Some(TreeSelectSize::Small) => "ant-select-sm",
            _ => "",
        },
        props.class_name
    )
    .trim()
    .to_string();

    // 处理点击选择器
    let handle_selector_click = {
        let mut open = open.clone();
        move |_| {
            if !props.disabled {
                open.set(!open());
            }
        }
    };

    // 处理搜索
    let handle_search = {
        let mut search_value = search_value.clone();
        move |evt: FormEvent| {
            let value = evt.value();
            search_value.set(value.clone());
            if let Some(on_search) = &props.on_search {
                on_search(value);
            }
        }
    };
    // 处理清除逻辑将直接在按钮点击事件中实现

    // 渲染选中值显示
    let render_selection = || {
        let values = selected_values();
        if values.is_empty() {
            rsx! {
                span {
                    class: "ant-select-selection-placeholder",
                    "{props.placeholder}"
                }
            }
        } else if props.multiple {
            rsx! {
                div {
                    class: "ant-select-selection-overflow",
                    for value in values {
                        span {
                            key: "{value}",
                            class: "ant-select-selection-item",
                            span {
                                class: "ant-select-selection-item-content",
                                "{value}"
                            }
                            if !props.disabled {
                                span {
                                    class: "ant-select-selection-item-remove",
                                    onclick: |_| {
                                        let mut new_values = selected_values();
                                        new_values.retain(|v| v != &value);
                                        selected_values.set(new_values.clone());
                                        if let Some(on_change) = props.on_change.as_ref() {
                                            let labels = HashMap::new();
                                            on_change(new_values, vec![], labels);
                                        }
                                    },
                                    "×"
                                }
                            }
                        }
                    }
                }
            }
        } else {
            rsx! {
                span {
                    class: "ant-select-selection-item",
                    "{values[0]}"
                }
            }
        }
    };

    // 渲染树节点
    let render_tree_nodes = |nodes: &[TreeNode], level: usize| -> Element {
        rsx! {
            for node in nodes {
                div {
                    key: "{node.value}",
                    class: format!(
                        "ant-tree-treenode ant-tree-treenode-leaf-last {}",
                        if selected_values().contains(&node.value) { "ant-tree-treenode-selected" } else { "" }
                    ),
                    style: format!("padding-left: {}px", level * 24),

                    div {
                        class: "ant-tree-node-content-wrapper",
                        onclick: |_| {
                            if !node.disabled {
                                let mut new_values = selected_values();

                                if props.multiple {
                                    if new_values.contains(&node.value) {
                                        new_values.retain(|v| v != &node.value);
                                    } else {
                                        new_values.push(node.value.clone());
                                    }
                                } else {
                                    new_values = vec![node.value.clone()];
                                    open.set(false);
                                }

                                selected_values.set(new_values.clone());

                                if let Some(on_change) = props.on_change.as_ref() {
                                    let labels = HashMap::new(); // 简化实现
                                    on_change(new_values, vec![node.value.clone()], labels);
                                }

                                if let Some(on_select) = props.on_select.as_ref() {
                                    let extra = HashMap::new(); // 简化实现
                                    on_select(node.value.clone(), true, extra);
                                }
                            }
                        },

                        if props.tree_checkable {
                            span {
                                class: "ant-tree-checkbox",
                                input {
                                    r#type: "checkbox",
                                    checked: selected_values().contains(&node.value),
                                    disabled: node.disabled || node.disable_checkbox,
                                }
                                span {
                                    class: "ant-tree-checkbox-inner"
                                }
                            }
                        }

                        span {
                            class: "ant-tree-title",
                            "{node.title}"
                        }
                    }

                    if let Some(children) = &node.children {
                        if expanded_keys().contains(&node.value) {
                            for child in children {
                                div {
                                    key: "{child.value}",
                                    class: format!(
                                        "ant-tree-treenode ant-tree-treenode-leaf-last {}",
                                        if selected_values().contains(&child.value) { "ant-tree-treenode-selected" } else { "" }
                                    ),
                                    style: format!("padding-left: {}px", (level + 1) * 24),

                                    div {
                                        class: "ant-tree-node-content-wrapper",
                                        onclick: |_| {
                            if !child.disabled {
                                let mut new_values = selected_values();

                                if props.multiple {
                                    if new_values.contains(&child.value) {
                                        new_values.retain(|v| v != &child.value);
                                    } else {
                                        new_values.push(child.value.clone());
                                    }
                                } else {
                                    new_values = vec![child.value.clone()];
                                    open.set(false);
                                }

                                selected_values.set(new_values.clone());

                                if let Some(on_change) = props.on_change.as_ref() {
                                    let labels = HashMap::new(); // 简化实现
                                    on_change(new_values, vec![child.value.clone()], labels);
                                }

                                if let Some(on_select) = props.on_select.as_ref() {
                                    let extra = HashMap::new(); // 简化实现
                                    on_select(child.value.clone(), true, extra);
                                }
                            }
                        },

                                        if props.tree_checkable {
                                            span {
                                                class: "ant-tree-checkbox",
                                                input {
                                                    r#type: "checkbox",
                                                    checked: selected_values().contains(&child.value),
                                                    disabled: child.disabled || child.disable_checkbox,
                                                }
                                                span {
                                                    class: "ant-tree-checkbox-inner"
                                                }
                                            }
                                        }

                                        span {
                                            class: "ant-tree-title",
                                            "{child.title}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    };

    rsx! {
        style { {TREE_SELECT_STYLE} }

        div {
            class: class_name,

            div {
                class: "ant-select-selector",
                onclick: handle_selector_click,

                div {
                    class: "ant-select-selection-search",
                    if props.show_search.unwrap_or(!props.multiple) && open() {
                        input {
                            class: "ant-select-selection-search-input",
                            r#type: "text",
                            value: "{search_value()}",
                            placeholder: if selected_values().is_empty() { props.placeholder.clone() } else { "" },
                            oninput: handle_search,
                            readonly: !props.show_search.unwrap_or(true),
                        }
                    }
                }

                {render_selection()}
            }

            if props.allow_clear && !selected_values().is_empty() && !props.disabled {
                span {
                    class: "ant-select-clear",
                    onclick: |_| {
                        selected_values.set(vec![]);
                        if let Some(on_change) = props.on_change.as_ref() {
                            let labels = HashMap::new();
                            on_change(vec![], vec![], labels);
                        }
                    },
                    "×"
                }
            }

            span {
                class: "ant-select-arrow",
                if let Some(suffix_icon) = &props.suffix_icon {
                    {suffix_icon.clone()}
                } else {
                    span {
                        class: "ant-select-suffix",
                        "▼"
                    }
                }
            }
        }

        if open() {
            div {
                class: format!(
                    "ant-select-dropdown ant-tree-select-dropdown {}",
                    props.popup_class_name
                ),
                style: format!("min-width: {}px; height: {}px;",
                    if props.popup_match_select_width { "100%" } else { "auto" },
                    props.list_height
                ),

                div {
                    class: "ant-select-dropdown-content",

                    div {
                        class: "ant-tree ant-tree-directory",

                        if props.tree_data.is_empty() {
                            div {
                                class: "ant-select-item-empty",
                                {props.not_found_content}
                            }
                        } else {
                            {render_tree_nodes(&props.tree_data, 0)}
                        }
                    }
                }
            }
        }
    }
}

// 辅助函数：收集所有节点的键值
fn collect_all_keys(nodes: &[TreeNode]) -> Vec<String> {
    let mut keys = Vec::new();
    for node in nodes {
        keys.push(node.value.clone());
        if let Some(children) = &node.children {
            keys.extend(collect_all_keys(children));
        }
    }
    keys
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_node_creation() {
        let node = TreeNode::new("test".to_string(), "Test Node".to_string());
        assert_eq!(node.value, "test");
        assert_eq!(node.title, "Test Node");
        assert!(!node.disabled);
        assert!(node.selectable);
    }

    #[test]
    fn test_tree_node_with_children() {
        let child = TreeNode::new("child".to_string(), "Child Node".to_string());
        let parent = TreeNode::new("parent".to_string(), "Parent Node".to_string())
            .with_children(vec![child]);

        assert!(parent.children.is_some());
        assert_eq!(parent.children.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn test_collect_all_keys() {
        let tree_data = vec![
            TreeNode::new("1".to_string(), "Node 1".to_string()).with_children(vec![
                TreeNode::new("1-1".to_string(), "Node 1-1".to_string()),
                TreeNode::new("1-2".to_string(), "Node 1-2".to_string()),
            ]),
            TreeNode::new("2".to_string(), "Node 2".to_string()),
        ];

        let keys = collect_all_keys(&tree_data);
        assert_eq!(keys.len(), 4);
        assert!(keys.contains(&"1".to_string()));
        assert!(keys.contains(&"1-1".to_string()));
        assert!(keys.contains(&"1-2".to_string()));
        assert!(keys.contains(&"2".to_string()));
    }
}

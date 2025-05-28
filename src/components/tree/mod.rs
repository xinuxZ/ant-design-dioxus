//! Tree 树形控件组件
//!
//! 多层次的结构列表。
//!
//! ## 何时使用
//!
//! 文件夹、组织架构、生物分类、国家地区等等，世间万物的大多数结构都是树形结构。使用 `树控件` 可以完整展现其中的层级关系，并具有展开收起选择等交互功能。
//!
//! ## 代码演示
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::{Tree, TreeNode};
//!
//! #[component]
//! fn App() -> Element {
//!     let tree_data = vec![
//!         TreeNode {
//!             key: "0-0".to_string(),
//!             title: "parent 1".to_string(),
//!             children: Some(vec![
//!                 TreeNode {
//!                     key: "0-0-0".to_string(),
//!                     title: "parent 1-0".to_string(),
//!                     children: Some(vec![
//!                         TreeNode {
//!                             key: "0-0-0-0".to_string(),
//!                             title: "leaf".to_string(),
//!                             children: None,
//!                             ..Default::default()
//!                         },
//!                     ]),
//!                     ..Default::default()
//!                 },
//!             ]),
//!             ..Default::default()
//!         },
//!     ];
//!
//!     rsx! {
//!         Tree {
//!             tree_data: tree_data,
//!         }
//!     }
//! }
//! ```

use dioxus::prelude::*;
use std::collections::HashSet;

/// TreeNodeComponent 组件属性
#[derive(Props, PartialEq, Clone)]
struct TreeNodeComponentProps {
    node: TreeNode,
    level: usize,
    expanded_keys: HashSet<String>,
    selected_keys: HashSet<String>,
    checked_keys: HashSet<String>,
    on_expand: EventHandler<String>,
    on_select: EventHandler<String>,
    on_check: EventHandler<String>,
    checkable: bool,
    selectable: bool,
    show_line: bool,
    show_icon: bool,
    draggable: bool,
}

/// TreeNodeComponent 递归渲染组件
#[component]
fn TreeNodeComponent(props: TreeNodeComponentProps) -> Element {
    let is_expanded = props.expanded_keys.contains(&props.node.key);
    let is_selected = props.selected_keys.contains(&props.node.key);
    let is_checked = props.checked_keys.contains(&props.node.key);
    let has_children = props
        .node
        .children
        .as_ref()
        .map_or(false, |children| !children.is_empty());
    let is_leaf = props.node.is_leaf || !has_children;

    rsx! {
        div {
            class: format!(
                "ant-tree-treenode ant-tree-treenode-switcher-{} {}",
                if is_expanded { "open" } else { "close" },
                if is_selected { "ant-tree-treenode-selected" } else { "" }
            ),
            style: format!("padding-left: {}px;", props.level * 24),

            // 展开/收起按钮
            if !is_leaf {
                span {
                    class: format!(
                        "ant-tree-switcher {}",
                        if is_expanded {
                            "ant-tree-switcher-open"
                        } else {
                            "ant-tree-switcher-close"
                        }
                    ),
                    onclick: {
                        let key = props.node.key.clone();
                        let on_expand = props.on_expand.clone();
                        move |_| {
                            on_expand.call(key.clone());
                        }
                    },
                    if is_expanded { "−" } else { "+" }
                }
            } else {
                span {
                    class: "ant-tree-switcher ant-tree-switcher-noop",
                }
            }

            // 复选框
            if props.checkable {
                span {
                    class: format!(
                        "ant-tree-checkbox {}",
                        if is_checked {
                            "ant-tree-checkbox-checked"
                        } else {
                            "ant-tree-checkbox-unchecked"
                        }
                    ),
                    onclick: {
                        let key = props.node.key.clone();
                        let on_check = props.on_check.clone();
                        move |_| {
                            on_check.call(key.clone());
                        }
                    },
                    span {
                        class: "ant-tree-checkbox-inner",
                    }
                }
            }

            // 图标
            if props.show_icon {
                span {
                    class: "ant-tree-iconEle ant-tree-icon__customize",
                    if let Some(icon) = &props.node.icon {
                        {icon.clone()}
                    }
                }
            }

            // 节点内容
            span {
                class: format!(
                    "ant-tree-node-content-wrapper {}",
                    if is_selected {
                        "ant-tree-node-content-wrapper-selected"
                    } else {
                        "ant-tree-node-content-wrapper-normal"
                    }
                ),
                onclick: {
                    let key = props.node.key.clone();
                    let on_select = props.on_select.clone();
                    move |_| {
                        if props.selectable {
                            on_select.call(key.clone());
                        }
                    }
                },
                span {
                    class: "ant-tree-title",
                    {props.node.title.clone()}
                }
            }

            // 子节点
            if has_children && is_expanded {
                div {
                    class: "ant-tree-child-tree",
                    for child in props.node.children.as_ref().unwrap() {
                        TreeNodeComponent {
                            node: child.clone(),
                            level: props.level + 1,
                            expanded_keys: props.expanded_keys.clone(),
                            selected_keys: props.selected_keys.clone(),
                            checked_keys: props.checked_keys.clone(),
                            on_expand: props.on_expand.clone(),
                            on_select: props.on_select.clone(),
                            on_check: props.on_check.clone(),
                            checkable: props.checkable,
                            selectable: props.selectable,
                            show_line: props.show_line,
                            show_icon: props.show_icon,
                            draggable: props.draggable,
                        }
                    }
                }
            }
        }
    }
}

/// 树节点数据结构
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TreeNode {
    /// 节点的唯一标识
    pub key: String,
    /// 节点标题
    pub title: String,
    /// 子节点
    pub children: Option<Vec<TreeNode>>,
    /// 是否禁用节点
    pub disabled: bool,
    /// 是否禁用复选框
    pub disabled_checkbox: bool,
    /// 节点前添加的图标
    pub icon: Option<String>,
    /// 是否为叶子节点
    pub is_leaf: bool,
    /// 是否可选择
    pub selectable: bool,
    /// 自定义数据
    pub data: Option<String>,
}

/// TreeNode 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct TreeNodeProps {
    /// 节点的唯一标识
    pub node_key: String,
}

/// Tree 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct TreeProps {
    /// 树形数据
    pub tree_data: Vec<TreeNode>,
    /// 是否支持多选
    #[props(default = false)]
    pub multiple: bool,
    /// 是否支持选中
    #[props(default = true)]
    pub selectable: bool,
    /// 是否显示连接线
    #[props(default = false)]
    pub show_line: bool,
    /// 是否显示图标
    #[props(default = true)]
    pub show_icon: bool,
    /// 是否支持点击展开节点
    #[props(default = true)]
    pub expand_on_click: bool,
    /// 默认展开的节点
    #[props(default = vec![])]
    pub default_expanded_keys: Vec<String>,
    /// 默认选中的节点
    #[props(default = vec![])]
    pub default_selected_keys: Vec<String>,
    /// 默认选中的复选框节点
    #[props(default = vec![])]
    pub default_checked_keys: Vec<String>,
    /// 是否支持复选框
    #[props(default = false)]
    pub checkable: bool,
    /// 复选框的选中状态完全受控（父子节点选中状态不再关联）
    #[props(default = false)]
    pub check_strictly: bool,
    /// 是否禁用树
    #[props(default = false)]
    pub disabled: bool,
    /// 是否支持拖拽
    #[props(default = false)]
    pub draggable: bool,
    /// 节点点击回调
    pub on_select: Option<EventHandler<(Vec<String>, TreeNode)>>,
    /// 复选框选中回调
    pub on_check: Option<EventHandler<(Vec<String>, TreeNode)>>,
    /// 展开/收起节点时触发
    pub on_expand: Option<EventHandler<(Vec<String>, TreeNode)>>,
    /// 右键点击节点时触发
    pub on_right_click: Option<EventHandler<TreeNode>>,
    /// 拖拽时触发
    pub on_drag_start: Option<EventHandler<TreeNode>>,
    /// 拖拽进入时触发
    pub on_drag_enter: Option<EventHandler<TreeNode>>,
    /// 拖拽离开时触发
    pub on_drag_leave: Option<EventHandler<TreeNode>>,
    /// 拖拽结束时触发
    pub on_drop: Option<EventHandler<(TreeNode, TreeNode)>>,
    /// 自定义样式类名
    #[props(default = String::new())]
    pub class: String,
    /// 自定义内联样式
    #[props(default = String::new())]
    pub style: String,
}

/// Tree 树形控件组件
///
/// 多层次的结构列表。
#[component]
pub fn Tree(props: TreeProps) -> Element {
    let mut expanded_keys = use_signal(|| {
        props
            .default_expanded_keys
            .iter()
            .cloned()
            .collect::<HashSet<String>>()
    });
    let mut selected_keys = use_signal(|| {
        props
            .default_selected_keys
            .iter()
            .cloned()
            .collect::<HashSet<String>>()
    });
    let mut checked_keys = use_signal(|| {
        props
            .default_checked_keys
            .iter()
            .cloned()
            .collect::<HashSet<String>>()
    });

    // 事件处理函数
    let handle_expand = move |key: String| {
        let mut current_expanded = expanded_keys();
        if current_expanded.contains(&key) {
            current_expanded.remove(&key);
        } else {
            current_expanded.insert(key.clone());
        }
        expanded_keys.set(current_expanded);
    };

    let handle_select = move |key: String| {
        let mut current_selected = selected_keys();
        if props.multiple {
            if current_selected.contains(&key) {
                current_selected.remove(&key);
            } else {
                current_selected.insert(key.clone());
            }
        } else {
            current_selected.clear();
            current_selected.insert(key.clone());
        }
        selected_keys.set(current_selected);
    };

    let handle_check = move |key: String| {
        let mut current_checked = checked_keys();
        if current_checked.contains(&key) {
            current_checked.remove(&key);
        } else {
            current_checked.insert(key.clone());
        }
        checked_keys.set(current_checked);
    };

    rsx! {
        div {
            class: format!(
                "ant-tree {} {} {} {}",
                if props.show_line { "ant-tree-show-line" } else { "" },
                if props.disabled { "ant-tree-disabled" } else { "" },
                if props.checkable { "ant-tree-checkable" } else { "" },
                props.class
            ),
            style: props.style.clone(),
            role: "tree",

            div {
                class: "ant-tree-list",
                for node in &props.tree_data {
                    TreeNodeComponent {
                        node: node.clone(),
                        level: 0,
                        expanded_keys: expanded_keys(),
                        selected_keys: selected_keys(),
                        checked_keys: checked_keys(),
                        on_expand: handle_expand.clone(),
                        on_select: handle_select.clone(),
                        on_check: handle_check.clone(),
                        checkable: props.checkable,
                        selectable: props.selectable,
                        show_line: props.show_line,
                        show_icon: props.show_icon,
                        draggable: props.draggable,
                    }
                }
            }
        }
    }
}

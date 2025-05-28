//!
//!
//! 展示 Tree 组件的各种用法和样式

use crate::common::*;
use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

/// Tree 组件演示
#[component]
pub fn TreeDemo() -> Element {
    let tree_data = use_signal(|| {
        vec![
            TreeNode {
                key: "0-0".to_string(),
                title: "parent 1".to_string(),
                icon: None,
                disabled: false,
                selectable: true,
                is_leaf: false,
                disabled_checkbox: false,
                data: None,
                children: Some(vec![
                    TreeNode {
                        key: "0-0-0".to_string(),
                        title: "parent 1-0".to_string(),
                        icon: None,
                        disabled: false,
                        selectable: true,
                        is_leaf: false,
                        disabled_checkbox: false,
                        data: None,
                        children: Some(vec![
                            TreeNode {
                                key: "0-0-0-0".to_string(),
                                title: "leaf".to_string(),
                                icon: None,
                                disabled: false,
                                selectable: true,
                                is_leaf: true,
                                children: None,
                                disabled_checkbox: false,
                                data: None,
                            },
                            TreeNode {
                                key: "0-0-0-1".to_string(),
                                title: "leaf".to_string(),
                                icon: None,
                                disabled: false,
                                selectable: true,
                                is_leaf: true,
                                children: None,
                                disabled_checkbox: false,
                                data: None,
                            },
                        ]),
                    },
                    TreeNode {
                        key: "0-0-1".to_string(),
                        title: "parent 1-1".to_string(),
                        icon: None,
                        disabled: false,
                        selectable: true,
                        is_leaf: false,
                        disabled_checkbox: false,
                        data: None,
                        children: Some(vec![TreeNode {
                            key: "0-0-1-0".to_string(),
                            title: "leaf".to_string(),
                            icon: None,
                            disabled: true,
                            selectable: true,
                            is_leaf: true,
                            children: None,
                            disabled_checkbox: false,
                            data: None,
                        }]),
                    },
                ]),
            },
            TreeNode {
                key: "0-1".to_string(),
                title: "parent 2".to_string(),
                icon: None,
                disabled: false,
                selectable: true,
                is_leaf: false,
                disabled_checkbox: false,
                data: None,
                children: Some(vec![TreeNode {
                    key: "0-1-0".to_string(),
                    title: "parent 2-0".to_string(),
                    icon: None,
                    disabled: false,
                    selectable: true,
                    is_leaf: false,
                    disabled_checkbox: false,
                    data: None,
                    children: Some(vec![
                        TreeNode {
                            key: "0-1-0-0".to_string(),
                            title: "leaf".to_string(),
                            icon: None,
                            disabled: false,
                            selectable: true,
                            is_leaf: true,
                            children: None,
                            disabled_checkbox: false,
                            data: None,
                        },
                        TreeNode {
                            key: "0-1-0-1".to_string(),
                            title: "leaf".to_string(),
                            icon: None,
                            disabled: false,
                            selectable: true,
                            is_leaf: true,
                            children: None,
                            disabled_checkbox: false,
                            data: None,
                        },
                    ]),
                }]),
            },
        ]
    });

    let expanded_keys = use_signal(|| {
        vec![
            "0-0".to_string(),
            "0-0-0".to_string(),
            "0-0-0-0".to_string(),
        ]
    });
    let selected_keys = use_signal(|| vec!["0-0-0-0".to_string()]);
    let checked_keys = use_signal(|| vec!["0-0-0-0".to_string()]);

    rsx! {
        div { class: "tree-demo",
            h1 { "Tree 树形控件" }
            p { "多层次的结构列表。" }

            DemoSection {
                title: "基本用法",
                description: "最简单的用法，展示可勾选，可选中，禁用，默认展开等功能。",

                div {
                    Tree {
                        tree_data: tree_data.read().clone(),
                        checkable: true,
                        default_expanded_keys: vec!["0-0".to_string()],
                        default_selected_keys: vec!["0-0-0-0".to_string()],
                        default_checked_keys: vec!["0-0-0-0".to_string()],
                        show_icon: false,
                        show_line: false,
                        multiple: false,
                        disabled: false,
                        draggable: false,
                        // on_expand: move |keys: Vec<String>| {
                        //     expanded_keys.set(keys);
                        // },
                        // on_select: move |keys: Vec<String>, info| {
                        //     selected_keys.set(keys);
                        //     println!("Selected: {:?}", info);
                        // },
                        // on_check: move |keys: Vec<String>, info| {
                        //     checked_keys.set(keys);
                        //     println!("Checked: {:?}", info);
                        // },
                        on_right_click: move |info| {
                            println!("Right clicked: {:?}", info);
                        },
                        on_drag_start: move |info| {
                            println!("Drag start: {:?}", info);
                        },
                        on_drag_enter: move |info| {
                            println!("Drag enter: {:?}", info);
                        },
                        on_drag_leave: move |info| {
                            println!("Drag leave: {:?}", info);
                        },
                        on_drop: move |info| {
                            println!("Drop: {:?}", info);
                        },
                    }
                }
            }

            DemoSection {
                title: "受控操作示例",
                description: "受控操作示例",

                div {
                    Space {
                        direction: SpaceDirection::Vertical,
                        Space {
                            // Button {
                            //     onclick: move |_| {
                            //         expanded_keys.set(vec!["0-0".to_string(), "0-0-0".to_string(), "0-1".to_string(), "0-1-0".to_string()]);
                            //     },
                            //     "展开所有"
                            // }
                            // Button {
                            //     onclick: move |_| {
                            //         expanded_keys.set(vec![]);
                            //     },
                            //     "收起所有"
                            // }
                            // Button {
                            //     onclick: move |_| {
                            //         checked_keys.set(vec!["0-0-0-0".to_string(), "0-0-0-1".to_string(), "0-1-0-0".to_string()]);
                            //     },
                            //     "选中所有叶子节点"
                            // }
                            // Button {
                            //     onclick: move |_| {
                            //         checked_keys.set(vec![]);
                            //     },
                            //     "取消选中"
                            // }
                        }
                        Tree {
                            tree_data: tree_data.read().clone(),
                            checkable: true,
                            // on_expand: move |keys: Vec<String>| {
                            //     expanded_keys.set(keys);
                            // },
                            // on_select: move |keys: Vec<String>, info| {
                            //     selected_keys.set(keys);
                            // },
                            // on_check: move |keys: Vec<String>, info| {
                            //     checked_keys.set(keys);
                            // },
                        }
                    }
                }
            }

            DemoSection {
                title: "连接线",
                description: "节点之间带连接线的树，常用于文件目录类型的树。",

                div {
                    Tree {
                        tree_data: tree_data.read().clone(),
                        show_line: true,
                        show_icon: false,
                        default_expanded_keys: vec!["0-0".to_string(), "0-0-0".to_string()],
                        // on_select: move |keys: Vec<String>, info| {
                        //     println!("Selected: {:?}", info);
                        // },
                    }
                }
            }

            DemoSection {
                title: "可拖拽",
                description: "将节点拖拽到其他节点内部或前后。",

                div  {
                    Tree {
                        tree_data: tree_data.read().clone(),
                        draggable: true,
                        default_expanded_keys: vec!["0-0".to_string(), "0-0-0".to_string()],
                        on_drop: move |info| {
                            println!("Drop: {:?}", info);
                            // 这里可以处理拖拽后的数据更新
                        },
                    }
                }
            }

            DemoSection {
                title: "目录树",
                description: "内置的目录树，multiple 模式支持 ctrl(Windows) / cmd(Mac) 复选。",

                div {
                    Tree {
                        tree_data: tree_data.read().clone(),
                        multiple: true,
                        show_icon: true,
                        show_line: true,
                        default_expanded_keys: vec!["0-0".to_string()],
                        // on_select: move |keys: Vec<String>, info| {
                        //     println!("Selected: {:?}", info);
                        // },
                    }
                }
            }
        }
    }
}

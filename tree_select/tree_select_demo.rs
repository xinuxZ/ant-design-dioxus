use crate::components::tree_select::{TreeNode, TreeSelect, TreeSelectProps};
use dioxus::prelude::*;

#[component]
pub fn TreeSelectDemo() -> Element {
    // 基础树选择状态
    let mut basic_value = use_signal(|| None::<String>);

    // 多选树选择状态
    let mut multiple_value = use_signal(|| Vec::<String>::new());

    // 可搜索树选择状态
    let mut searchable_value = use_signal(|| None::<String>);

    // 复选框树选择状态
    let mut checkable_value = use_signal(|| Vec::<String>::new());

    // 禁用状态
    let mut disabled_value = use_signal(|| None::<String>);

    // 基础树数据
    let basic_tree_data = vec![
        TreeNode {
            value: "parent1".to_string(),
            title: "父节点1".to_string(),
            children: Some(vec![
                TreeNode {
                    value: "parent1-0".to_string(),
                    title: "子节点1-0".to_string(),
                    children: Some(vec![
                        TreeNode {
                            value: "leaf1".to_string(),
                            title: "叶子节点1".to_string(),
                            children: None,
                            disabled: false,
                            checkable: true,
                            selectable: true,
                            disableCheckbox: false,
                        },
                        TreeNode {
                            value: "leaf2".to_string(),
                            title: "叶子节点2".to_string(),
                            children: None,
                            disabled: false,
                            checkable: true,
                            selectable: true,
                            disableCheckbox: false,
                        },
                    ]),
                    disabled: false,
                    checkable: true,
                    selectable: true,
                    disableCheckbox: false,
                },
                TreeNode {
                    value: "parent1-1".to_string(),
                    title: "子节点1-1".to_string(),
                    children: Some(vec![TreeNode {
                        value: "leaf3".to_string(),
                        title: "叶子节点3".to_string(),
                        children: None,
                        disabled: false,
                        checkable: true,
                        selectable: true,
                        disableCheckbox: false,
                    }]),
                    disabled: false,
                    checkable: true,
                    selectable: true,
                    disableCheckbox: false,
                },
            ]),
            disabled: false,
            checkable: true,
            selectable: true,
            disableCheckbox: false,
        },
        TreeNode {
            value: "parent2".to_string(),
            title: "父节点2".to_string(),
            children: Some(vec![TreeNode {
                value: "parent2-0".to_string(),
                title: "子节点2-0".to_string(),
                children: Some(vec![
                    TreeNode {
                        value: "leaf4".to_string(),
                        title: "叶子节点4".to_string(),
                        children: None,
                        disabled: false,
                        checkable: true,
                        selectable: true,
                        disableCheckbox: false,
                    },
                    TreeNode {
                        value: "leaf5".to_string(),
                        title: "叶子节点5".to_string(),
                        children: None,
                        disabled: true, // 禁用节点
                        checkable: true,
                        selectable: false,
                        disableCheckbox: true,
                    },
                ]),
                disabled: false,
                checkable: true,
                selectable: true,
                disableCheckbox: false,
            }]),
            disabled: false,
            checkable: true,
            selectable: true,
            disableCheckbox: false,
        },
    ];

    // 部门组织架构数据
    let org_tree_data = vec![TreeNode {
        value: "company".to_string(),
        title: "公司总部".to_string(),
        children: Some(vec![
            TreeNode {
                value: "tech".to_string(),
                title: "技术部".to_string(),
                children: Some(vec![
                    TreeNode {
                        value: "frontend".to_string(),
                        title: "前端组".to_string(),
                        children: None,
                        disabled: false,
                        checkable: true,
                        selectable: true,
                        disableCheckbox: false,
                    },
                    TreeNode {
                        value: "backend".to_string(),
                        title: "后端组".to_string(),
                        children: None,
                        disabled: false,
                        checkable: true,
                        selectable: true,
                        disableCheckbox: false,
                    },
                    TreeNode {
                        value: "mobile".to_string(),
                        title: "移动端组".to_string(),
                        children: None,
                        disabled: false,
                        checkable: true,
                        selectable: true,
                        disableCheckbox: false,
                    },
                ]),
                disabled: false,
                checkable: true,
                selectable: true,
                disableCheckbox: false,
            },
            TreeNode {
                value: "product".to_string(),
                title: "产品部".to_string(),
                children: Some(vec![
                    TreeNode {
                        value: "pm".to_string(),
                        title: "产品经理".to_string(),
                        children: None,
                        disabled: false,
                        checkable: true,
                        selectable: true,
                        disableCheckbox: false,
                    },
                    TreeNode {
                        value: "ui".to_string(),
                        title: "UI设计师".to_string(),
                        children: None,
                        disabled: false,
                        checkable: true,
                        selectable: true,
                        disableCheckbox: false,
                    },
                ]),
                disabled: false,
                checkable: true,
                selectable: true,
                disableCheckbox: false,
            },
            TreeNode {
                value: "marketing".to_string(),
                title: "市场部".to_string(),
                children: None,
                disabled: false,
                checkable: true,
                selectable: true,
                disableCheckbox: false,
            },
        ]),
        disabled: false,
        checkable: true,
        selectable: true,
        disableCheckbox: false,
    }];

    rsx! {
        div { class: "tree-select-demo",
            style {
                ".tree-select-demo {{ padding: 24px; background: #f5f5f5; }}
                .demo-section {{ margin-bottom: 32px; padding: 24px; background: white; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.1); }}
                .demo-title {{ font-size: 18px; font-weight: 600; margin-bottom: 16px; color: #1890ff; }}
                .demo-description {{ margin-bottom: 16px; color: #666; line-height: 1.6; }}
                .demo-item {{ margin-bottom: 16px; }}
                .demo-label {{ display: block; margin-bottom: 8px; font-weight: 500; color: #333; }}
                .demo-value {{ margin-top: 8px; padding: 8px; background: #f0f0f0; border-radius: 4px; font-family: monospace; color: #666; }}
                .demo-grid {{ display: grid; grid-template-columns: 1fr 1fr; gap: 16px; }}
                @media (max-width: 768px) {{
                    .demo-grid {{ grid-template-columns: 1fr; }}
                }}"
            }

            h1 { "TreeSelect 树选择演示" }

            // 基础用法
            div { class: "demo-section",
                h2 { class: "demo-title", "基础用法" }
                p { class: "demo-description",
                    "最简单的用法，展示树形结构的数据选择。支持单选模式，点击节点即可选择。"
                }

                div { class: "demo-item",
                    label { class: "demo-label", "基础树选择:" }
                    TreeSelect {
                        value: basic_value.read().clone(),
                        tree_data: basic_tree_data.clone(),
                        placeholder: "请选择节点".to_string(),
                        style: "width: 300px;".to_string(),
                        on_change: move |value: Option<String>| {
                            basic_value.set(value);
                        },
                    }
                    div { class: "demo-value",
                        "选中值: {basic_value.read():?}"
                    }
                }
            }

            // 多选模式
            div { class: "demo-section",
                h2 { class: "demo-title", "多选模式" }
                p { class: "demo-description",
                    "支持多选模式，可以同时选择多个节点。选中的节点会以标签形式显示在选择框中。"
                }

                div { class: "demo-item",
                    label { class: "demo-label", "多选树选择:" }
                    TreeSelect {
                        multiple: true,
                        multiple_value: multiple_value.read().clone(),
                        tree_data: basic_tree_data.clone(),
                        placeholder: "请选择多个节点".to_string(),
                        style: "width: 400px;".to_string(),
                        on_multiple_change: move |values: Vec<String>| {
                            multiple_value.set(values);
                        },
                    }
                    div { class: "demo-value",
                        "选中值: {multiple_value.read():?}"
                    }
                }
            }

            // 可搜索
            div { class: "demo-section",
                h2 { class: "demo-title", "可搜索" }
                p { class: "demo-description",
                    "支持搜索功能，可以通过输入关键词快速定位到目标节点。搜索会匹配节点标题。"
                }

                div { class: "demo-item",
                    label { class: "demo-label", "可搜索树选择:" }
                    TreeSelect {
                        value: searchable_value.read().clone(),
                        tree_data: org_tree_data.clone(),
                        placeholder: "搜索并选择部门".to_string(),
                        show_search: true,
                        style: "width: 350px;".to_string(),
                        on_change: move |value: Option<String>| {
                            searchable_value.set(value);
                        },
                        filter_tree_node: Some(Box::new(|input: &str, node: &TreeNode| {
                            node.title.to_lowercase().contains(&input.to_lowercase())
                        })),
                    }
                    div { class: "demo-value",
                        "选中值: {searchable_value.read():?}"
                    }
                }
            }

            // 复选框模式
            div { class: "demo-section",
                h2 { class: "demo-title", "复选框模式" }
                p { class: "demo-description",
                    "启用复选框模式，支持父子节点关联选择。选择父节点会自动选择所有子节点。"
                }

                div { class: "demo-item",
                    label { class: "demo-label", "复选框树选择:" }
                    TreeSelect {
                        multiple: true,
                        tree_checkable: true,
                        multiple_value: checkable_value.read().clone(),
                        tree_data: org_tree_data.clone(),
                        placeholder: "选择部门（复选框模式）".to_string(),
                        style: "width: 400px;".to_string(),
                        on_multiple_change: move |values: Vec<String>| {
                            checkable_value.set(values);
                        },
                    }
                    div { class: "demo-value",
                        "选中值: {checkable_value.read():?}"
                    }
                }
            }

            // 不同尺寸和状态
            div { class: "demo-section",
                h2 { class: "demo-title", "不同尺寸和状态" }
                p { class: "demo-description",
                    "支持大、中、小三种尺寸，以及禁用状态。可以根据不同场景选择合适的尺寸。"
                }

                div { class: "demo-grid",
                    div { class: "demo-item",
                        label { class: "demo-label", "大尺寸:" }
                        TreeSelect {
                            value: None,
                            tree_data: basic_tree_data.clone(),
                            placeholder: "大尺寸选择器".to_string(),
                            size: "large".to_string(),
                            style: "width: 100%;".to_string(),
                        }
                    }

                    div { class: "demo-item",
                        label { class: "demo-label", "中尺寸（默认）:" }
                        TreeSelect {
                            value: None,
                            tree_data: basic_tree_data.clone(),
                            placeholder: "中尺寸选择器".to_string(),
                            style: "width: 100%;".to_string(),
                        }
                    }

                    div { class: "demo-item",
                        label { class: "demo-label", "小尺寸:" }
                        TreeSelect {
                            value: None,
                            tree_data: basic_tree_data.clone(),
                            placeholder: "小尺寸选择器".to_string(),
                            size: "small".to_string(),
                            style: "width: 100%;".to_string(),
                        }
                    }

                    div { class: "demo-item",
                        label { class: "demo-label", "禁用状态:" }
                        TreeSelect {
                            value: disabled_value.read().clone(),
                            tree_data: basic_tree_data.clone(),
                            placeholder: "禁用的选择器".to_string(),
                            disabled: true,
                            style: "width: 100%;".to_string(),
                        }
                    }
                }
            }

            // 自定义功能
            div { class: "demo-section",
                h2 { class: "demo-title", "自定义功能" }
                p { class: "demo-description",
                    "展示更多自定义功能，包括清除按钮、下拉高度限制、自定义渲染等。"
                }

                div { class: "demo-grid",
                    div { class: "demo-item",
                        label { class: "demo-label", "允许清除:" }
                        TreeSelect {
                            value: None,
                            tree_data: basic_tree_data.clone(),
                            placeholder: "可清除的选择器".to_string(),
                            allow_clear: true,
                            style: "width: 100%;".to_string(),
                        }
                    }

                    div { class: "demo-item",
                        label { class: "demo-label", "限制下拉高度:" }
                        TreeSelect {
                            value: None,
                            tree_data: org_tree_data.clone(),
                            placeholder: "限制高度的选择器".to_string(),
                            dropdown_style: "max-height: 200px; overflow-y: auto;".to_string(),
                            style: "width: 100%;".to_string(),
                        }
                    }
                }
            }

            // API 说明
            div { class: "demo-section",
                h2 { class: "demo-title", "API 说明" }
                div { class: "demo-description",
                    h3 { "TreeSelect Props" }
                    ul {
                        li { "value: Option<String> - 当前选中的值（单选模式）" }
                        li { "multiple_value: Vec<String> - 当前选中的值（多选模式）" }
                        li { "tree_data: Vec<TreeNode> - 树形数据源" }
                        li { "placeholder: String - 占位符文本" }
                        li { "disabled: bool - 是否禁用" }
                        li { "multiple: bool - 是否支持多选" }
                        li { "tree_checkable: bool - 是否显示复选框" }
                        li { "show_search: bool - 是否支持搜索" }
                        li { "allow_clear: bool - 是否显示清除按钮" }
                        li { "size: String - 尺寸大小（large/middle/small）" }
                        li { "style: String - 自定义样式" }
                        li { "dropdown_style: String - 下拉菜单样式" }
                    }

                    h3 { "TreeNode 结构" }
                    ul {
                        li { "value: String - 节点的唯一标识" }
                        li { "title: String - 节点显示的标题" }
                        li { "children: Option<Vec<TreeNode>> - 子节点列表" }
                        li { "disabled: bool - 是否禁用该节点" }
                        li { "checkable: bool - 是否可勾选" }
                        li { "selectable: bool - 是否可选择" }
                        li { "disableCheckbox: bool - 是否禁用复选框" }
                    }

                    h3 { "回调函数" }
                    ul {
                        li { "on_change: 单选模式下的选择回调" }
                        li { "on_multiple_change: 多选模式下的选择回调" }
                        li { "on_search: 搜索输入回调" }
                        li { "on_dropdown_visible_change: 下拉菜单显示状态变化回调" }
                        li { "filter_tree_node: 自定义搜索过滤函数" }
                    }
                }
            }
        }
    }
}

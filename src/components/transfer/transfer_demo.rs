use crate::components::transfer::{Transfer, TransferItem, TransferProps};
use dioxus::prelude::*;

#[component]
pub fn TransferDemo() -> Element {
    // 基础穿梭框状态
    let mut basic_target_keys = use_signal(|| vec!["2".to_string(), "4".to_string()]);
    let mut basic_selected_keys = use_signal(|| Vec::<String>::new());

    // 带搜索功能的穿梭框状态
    let mut search_target_keys = use_signal(|| vec!["b".to_string()]);
    let mut search_selected_keys = use_signal(|| Vec::<String>::new());

    // 禁用状态的穿梭框
    let mut disabled_target_keys = use_signal(|| vec!["key2".to_string()]);
    let mut disabled_selected_keys = use_signal(|| Vec::<String>::new());

    // 单向模式的穿梭框
    let mut one_way_target_keys = use_signal(|| Vec::<String>::new());
    let mut one_way_selected_keys = use_signal(|| Vec::<String>::new());

    // 自定义操作按钮的穿梭框
    let mut custom_target_keys = use_signal(|| vec!["item2".to_string()]);
    let mut custom_selected_keys = use_signal(|| Vec::<String>::new());

    // 基础数据源
    let basic_data_source = vec![
        TransferItem {
            key: "1".to_string(),
            title: "内容一".to_string(),
            description: Some("描述一".to_string()),
            disabled: false,
        },
        TransferItem {
            key: "2".to_string(),
            title: "内容二".to_string(),
            description: Some("描述二".to_string()),
            disabled: false,
        },
        TransferItem {
            key: "3".to_string(),
            title: "内容三".to_string(),
            description: Some("描述三".to_string()),
            disabled: true,
        },
        TransferItem {
            key: "4".to_string(),
            title: "内容四".to_string(),
            description: Some("描述四".to_string()),
            disabled: false,
        },
        TransferItem {
            key: "5".to_string(),
            title: "内容五".to_string(),
            description: Some("描述五".to_string()),
            disabled: false,
        },
    ];

    // 搜索数据源
    let search_data_source = vec![
        TransferItem {
            key: "a".to_string(),
            title: "苹果".to_string(),
            description: Some("红色的苹果".to_string()),
            disabled: false,
        },
        TransferItem {
            key: "b".to_string(),
            title: "香蕉".to_string(),
            description: Some("黄色的香蕉".to_string()),
            disabled: false,
        },
        TransferItem {
            key: "c".to_string(),
            title: "橙子".to_string(),
            description: Some("橙色的橙子".to_string()),
            disabled: false,
        },
        TransferItem {
            key: "d".to_string(),
            title: "葡萄".to_string(),
            description: Some("紫色的葡萄".to_string()),
            disabled: false,
        },
    ];

    // 禁用数据源
    let disabled_data_source = vec![
        TransferItem {
            key: "key1".to_string(),
            title: "选项一".to_string(),
            description: Some("这是选项一的描述".to_string()),
            disabled: false,
        },
        TransferItem {
            key: "key2".to_string(),
            title: "选项二".to_string(),
            description: Some("这是选项二的描述".to_string()),
            disabled: false,
        },
        TransferItem {
            key: "key3".to_string(),
            title: "选项三".to_string(),
            description: Some("这是选项三的描述".to_string()),
            disabled: false,
        },
    ];

    // 单向数据源
    let one_way_data_source = vec![
        TransferItem {
            key: "item1".to_string(),
            title: "任务一".to_string(),
            description: Some("待处理的任务一".to_string()),
            disabled: false,
        },
        TransferItem {
            key: "item2".to_string(),
            title: "任务二".to_string(),
            description: Some("待处理的任务二".to_string()),
            disabled: false,
        },
        TransferItem {
            key: "item3".to_string(),
            title: "任务三".to_string(),
            description: Some("待处理的任务三".to_string()),
            disabled: false,
        },
    ];

    // 自定义数据源
    let custom_data_source = vec![
        TransferItem {
            key: "item1".to_string(),
            title: "文档一".to_string(),
            description: Some("重要文档".to_string()),
            disabled: false,
        },
        TransferItem {
            key: "item2".to_string(),
            title: "文档二".to_string(),
            description: Some("普通文档".to_string()),
            disabled: false,
        },
        TransferItem {
            key: "item3".to_string(),
            title: "文档三".to_string(),
            description: Some("临时文档".to_string()),
            disabled: false,
        },
    ];

    rsx! {
        div {
            style: "padding: 24px; background-color: #f5f5f5; min-height: 100vh;",

            h1 {
                style: "color: #1890ff; margin-bottom: 24px;",
                "Transfer 穿梭框组件演示"
            }

            // 基础穿梭框
            div {
                style: "margin-bottom: 32px; padding: 24px; background: white; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.1);",

                h2 {
                    style: "color: #262626; margin-bottom: 16px;",
                    "基础穿梭框"
                }

                p {
                    style: "color: #666; margin-bottom: 16px;",
                    "最基本的穿梭框用法，支持双向传输。"
                }

                Transfer {
                    data_source: basic_data_source.clone(),
                    target_keys: basic_target_keys.read().clone(),
                    selected_keys: basic_selected_keys.read().clone(),
                    titles: Some(vec!["源列表".to_string(), "目标列表".to_string()]),
                    operations: Some(vec![">".to_string(), "<".to_string()]),
                    on_change: move |target_keys, direction, move_keys| {
                        basic_target_keys.set(target_keys);
                        println!("Transfer changed: direction={}, move_keys={:?}", direction, move_keys);
                    },
                    on_select_change: move |source_selected_keys, target_selected_keys| {
                        let mut all_selected = source_selected_keys;
                        all_selected.extend(target_selected_keys);
                        basic_selected_keys.set(all_selected);
                    },
                }

                div {
                    style: "margin-top: 16px; padding: 12px; background: #f9f9f9; border-radius: 4px;",
                    p {
                        style: "margin: 0; color: #666; font-size: 12px;",
                        "目标键: {basic_target_keys.read():?}"
                    }
                    p {
                        style: "margin: 4px 0 0 0; color: #666; font-size: 12px;",
                        "选中键: {basic_selected_keys.read():?}"
                    }
                }
            }

            // 带搜索功能的穿梭框
            div {
                style: "margin-bottom: 32px; padding: 24px; background: white; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.1);",

                h2 {
                    style: "color: #262626; margin-bottom: 16px;",
                    "带搜索功能的穿梭框"
                }

                p {
                    style: "color: #666; margin-bottom: 16px;",
                    "支持搜索功能的穿梭框，可以快速筛选选项。"
                }

                Transfer {
                    data_source: search_data_source.clone(),
                    target_keys: search_target_keys.read().clone(),
                    selected_keys: search_selected_keys.read().clone(),
                    titles: Some(vec!["水果列表".to_string(), "已选水果".to_string()]),
                    show_search: true,
                    filter_option: Some(Box::new(|input_value, item| {
                        item.title.to_lowercase().contains(&input_value.to_lowercase()) ||
                        item.description.as_ref().map_or(false, |desc|
                            desc.to_lowercase().contains(&input_value.to_lowercase())
                        )
                    })),
                    on_search: Some(Box::new(|direction, value| {
                        println!("Search in {}: {}", direction, value);
                    })),
                    on_change: move |target_keys, direction, move_keys| {
                        search_target_keys.set(target_keys);
                        println!("Search transfer changed: direction={}, move_keys={:?}", direction, move_keys);
                    },
                    on_select_change: move |source_selected_keys, target_selected_keys| {
                        let mut all_selected = source_selected_keys;
                        all_selected.extend(target_selected_keys);
                        search_selected_keys.set(all_selected);
                    },
                }
            }

            // 禁用状态的穿梭框
            div {
                style: "margin-bottom: 32px; padding: 24px; background: white; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.1);",

                h2 {
                    style: "color: #262626; margin-bottom: 16px;",
                    "禁用状态的穿梭框"
                }

                p {
                    style: "color: #666; margin-bottom: 16px;",
                    "整个穿梭框处于禁用状态，无法进行任何操作。"
                }

                Transfer {
                    data_source: disabled_data_source.clone(),
                    target_keys: disabled_target_keys.read().clone(),
                    selected_keys: disabled_selected_keys.read().clone(),
                    titles: Some(vec!["源选项".to_string(), "目标选项".to_string()]),
                    disabled: true,
                    on_change: move |target_keys, direction, move_keys| {
                        disabled_target_keys.set(target_keys);
                    },
                    on_select_change: move |source_selected_keys, target_selected_keys| {
                        let mut all_selected = source_selected_keys;
                        all_selected.extend(target_selected_keys);
                        disabled_selected_keys.set(all_selected);
                    },
                }
            }

            // 单向模式的穿梭框
            div {
                style: "margin-bottom: 32px; padding: 24px; background: white; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.1);",

                h2 {
                    style: "color: #262626; margin-bottom: 16px;",
                    "单向模式的穿梭框"
                }

                p {
                    style: "color: #666; margin-bottom: 16px;",
                    "只能从左侧向右侧传输，不支持反向传输。"
                }

                Transfer {
                    data_source: one_way_data_source.clone(),
                    target_keys: one_way_target_keys.read().clone(),
                    selected_keys: one_way_selected_keys.read().clone(),
                    titles: Some(vec!["待处理任务".to_string(), "已完成任务".to_string()]),
                    one_way: true,
                    operations: Some(vec!["完成".to_string()]),
                    on_change: move |target_keys, direction, move_keys| {
                        one_way_target_keys.set(target_keys);
                        println!("One-way transfer: direction={}, move_keys={:?}", direction, move_keys);
                    },
                    on_select_change: move |source_selected_keys, target_selected_keys| {
                        let mut all_selected = source_selected_keys;
                        all_selected.extend(target_selected_keys);
                        one_way_selected_keys.set(all_selected);
                    },
                }
            }

            // 自定义操作按钮的穿梭框
            div {
                style: "margin-bottom: 32px; padding: 24px; background: white; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.1);",

                h2 {
                    style: "color: #262626; margin-bottom: 16px;",
                    "自定义操作按钮的穿梭框"
                }

                p {
                    style: "color: #666; margin-bottom: 16px;",
                    "自定义操作按钮文本和样式。"
                }

                Transfer {
                    data_source: custom_data_source.clone(),
                    target_keys: custom_target_keys.read().clone(),
                    selected_keys: custom_selected_keys.read().clone(),
                    titles: Some(vec!["文档库".to_string(), "我的文档".to_string()]),
                    operations: Some(vec!["添加 →".to_string(), "← 移除".to_string()]),
                    show_search: true,
                    on_change: move |target_keys, direction, move_keys| {
                        custom_target_keys.set(target_keys);
                        println!("Custom transfer: direction={}, move_keys={:?}", direction, move_keys);
                    },
                    on_select_change: move |source_selected_keys, target_selected_keys| {
                        let mut all_selected = source_selected_keys;
                        all_selected.extend(target_selected_keys);
                        custom_selected_keys.set(all_selected);
                    },
                }
            }

            // 使用说明
            div {
                style: "padding: 24px; background: white; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.1);",

                h2 {
                    style: "color: #262626; margin-bottom: 16px;",
                    "使用说明"
                }

                div {
                    style: "color: #666; line-height: 1.6;",

                    h3 {
                        style: "color: #1890ff; margin: 16px 0 8px 0;",
                        "主要特性："
                    }
                    ul {
                        style: "margin: 8px 0; padding-left: 20px;",
                        li { "支持双向传输和单向传输模式" }
                        li { "内置搜索功能，支持自定义过滤逻辑" }
                        li { "支持禁用状态和单个选项禁用" }
                        li { "可自定义操作按钮文本和列表标题" }
                        li { "完整的事件回调系统" }
                        li { "响应式设计，支持移动端" }
                    }

                    h3 {
                        style: "color: #1890ff; margin: 16px 0 8px 0;",
                        "使用场景："
                    }
                    ul {
                        style: "margin: 8px 0; padding-left: 20px;",
                        li { "权限分配和角色管理" }
                        li { "数据筛选和分类" }
                        li { "任务分配和状态管理" }
                        li { "文件和资源管理" }
                        li { "用户组和成员管理" }
                    }

                    h3 {
                        style: "color: #1890ff; margin: 16px 0 8px 0;",
                        "API 说明："
                    }
                    ul {
                        style: "margin: 8px 0; padding-left: 20px;",
                        li { "data_source: 数据源，包含所有可选项" }
                        li { "target_keys: 右侧列表中的项目键值" }
                        li { "selected_keys: 当前选中的项目键值" }
                        li { "on_change: 传输操作的回调函数" }
                        li { "on_select_change: 选择状态变化的回调函数" }
                        li { "show_search: 是否显示搜索框" }
                        li { "disabled: 是否禁用整个组件" }
                        li { "one_way: 是否为单向传输模式" }
                    }
                }
            }
        }
    }
}

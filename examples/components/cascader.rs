//!
//! 展示 Cascader 组件的各种用法和样式

use crate::common::*;
use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;
use std::collections::HashMap;

/// 显示标签的函数
fn display_labels(labels: &Vec<String>) -> String {
    labels.join(" > ")
}

/// Cascader 组件演示
#[component]
pub fn CascaderDemo() -> Element {
    let mut selected_value = use_signal(|| Vec::<String>::new());
    let mut multiple_value = use_signal(|| Vec::<Vec<String>>::new());
    let mut search_value = use_signal(|| String::new());
    let mut popup_visible = use_signal(|| false);

    // 基础选项数据
    let basic_options = vec![
        CascaderOption::new("zhejiang", "浙江").with_children(vec![
            CascaderOption::new("hangzhou", "杭州").with_children(vec![
                CascaderOption::new("xihu", "西湖"),
                CascaderOption::new("yuhang", "余杭"),
            ]),
            CascaderOption::new("ningbo", "宁波").with_children(vec![
                CascaderOption::new("haishu", "海曙"),
                CascaderOption::new("jiangbei", "江北"),
            ]),
        ]),
        CascaderOption::new("jiangsu", "江苏").with_children(vec![
            CascaderOption::new("nanjing", "南京").with_children(vec![
                CascaderOption::new("xuanwu", "玄武"),
                CascaderOption::new("qinhuai", "秦淮"),
            ]),
            CascaderOption::new("suzhou", "苏州").with_children(vec![
                CascaderOption::new("gusu", "姑苏"),
                CascaderOption::new("wuzhong", "吴中"),
            ]),
        ]),
    ];

    // 禁用选项数据
    let disabled_options = vec![
        CascaderOption::new("beijing", "北京").with_children(vec![
            CascaderOption::new("chaoyang", "朝阳"),
            CascaderOption::new("haidian", "海淀").disabled(),
            CascaderOption::new("dongcheng", "东城"),
        ]),
        CascaderOption::new("shanghai", "上海")
            .disabled()
            .with_children(vec![
                CascaderOption::new("huangpu", "黄浦"),
                CascaderOption::new("xuhui", "徐汇"),
            ]),
    ];

    // 动态加载选项
    let mut dynamic_options = use_signal(|| {
        vec![
            CascaderOption::new("dynamic1", "动态选项1"),
            CascaderOption::new("dynamic2", "动态选项2"),
        ]
    });

    rsx! {
        div {
            class: "component-showcase",

            h1 {
                style: "margin: 0 0 16px 0; color: #262626; font-size: 28px;",
                "Cascader 级联选择"
            }

            p {
                style: "margin: 0 0 24px 0; color: #666; font-size: 14px;",
                "级联选择框。需要从一组相关联的数据集合进行选择，例如省市区，公司层级，事物分类等。"
            }

            // 基础用法
            DemoSection {
                title: "基础用法",
                description: "省市区级联。",

                div {
                    style: "width: 300px;",
                    Cascader {
                        options: basic_options.clone(),
                        placeholder: "请选择地区",
                        on_change: move |value| {
                            selected_value.set(value);
                        }
                    }
                }

                div {
                    style: "margin-top: 16px; color: #666; font-size: 14px;",
                    "选中值: {selected_value:?}"
                }
            }

            // 默认值
            DemoSection {
                title: "默认值",
                description: "默认选中项。",

                div {
                    style: "width: 300px;",
                    Cascader {
                        options: basic_options.clone(),
                        default_value: vec!["zhejiang".to_string(), "hangzhou".to_string(), "xihu".to_string()],
                        placeholder: "请选择地区"
                    }
                }
            }

            // 禁用选项
            DemoSection {
                title: "禁用选项",
                description: "通过设置 disabled 字段禁用某些选项。",

                div {
                    style: "width: 300px;",
                    Cascader {
                        options: disabled_options.clone(),
                        placeholder: "请选择地区"
                    }
                }
            }

            // 尺寸
            DemoSection {
                title: "三种大小",
                description: "三种大小的级联选择器。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    div {
                        style: "width: 300px;",
                        "大尺寸:"
                        Cascader {
                            options: basic_options.clone(),
                            size: "large",
                            placeholder: "请选择地区"
                        }
                    }

                    div {
                        style: "width: 300px;",
                        "中等尺寸:"
                        Cascader {
                            options: basic_options.clone(),
                            size: "middle",
                            placeholder: "请选择地区"
                        }
                    }

                    div {
                        style: "width: 300px;",
                        "小尺寸:"
                        Cascader {
                            options: basic_options.clone(),
                            size: "small",
                            placeholder: "请选择地区"
                        }
                    }
                }
            }

            // 禁用状态
            DemoSection {
                title: "禁用状态",
                description: "禁用整个级联选择器。",

                div {
                    style: "width: 300px;",
                    Cascader {
                        options: basic_options.clone(),
                        disabled: true,
                        default_value: vec!["zhejiang".to_string(), "hangzhou".to_string()],
                        placeholder: "禁用状态"
                    }
                }
            }

            // 自定义触发器
            DemoSection {
                title: "hover 触发",
                description: "通过移入展开下级菜单。",

                div {
                    style: "width: 300px;",
                    Cascader {
                        options: basic_options.clone(),
                        expand_trigger: "hover",
                        placeholder: "请选择地区"
                    }
                }
            }

            // 搜索
            DemoSection {
                title: "搜索",
                description: "可以直接搜索选项并选择。",

                div {
                    style: "width: 300px;",
                    Cascader {
                        options: basic_options.clone(),
                        show_search: true,
                        placeholder: "请选择或搜索地区",
                        on_search: move |value| {
                            search_value.set(value);
                        }
                    }
                }

                div {
                    style: "margin-top: 16px; color: #666; font-size: 14px;",
                    "搜索值: {search_value}"
                }
            }

            // 多选
            DemoSection {
                title: "多选",
                description: "一次性选择多个选项。",

                div {
                    style: "width: 300px;",
                    Cascader {
                        options: basic_options.clone(),
                        multiple: true,
                        placeholder: "请选择地区",
                        on_change: move |value| {
                            // 注意：多选模式下需要处理多个值
                            web_sys::console::log_1(&format!("多选值: {:?}", value).into());
                        }
                    }
                }
            }

            // 自定义显示
            DemoSection {
                title: "自定义显示",
                description: "切换按钮和结果分离。",

                div {
                    style: "width: 300px;",
                    Cascader {
                        options: basic_options.clone(),
                        placeholder: "请选择地区",
                        display_render: Some(display_labels as fn(&Vec<String>) -> String)
                    }
                }
            }

            // 状态
            DemoSection {
                title: "状态",
                description: "不同的校验状态。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    div {
                        style: "width: 300px;",
                        "错误状态:"
                        Cascader {
                            options: basic_options.clone(),
                            status: "error",
                            placeholder: "请选择地区"
                        }
                    }

                    div {
                        style: "width: 300px;",
                        "警告状态:"
                        Cascader {
                            options: basic_options.clone(),
                            status: "warning",
                            placeholder: "请选择地区"
                        }
                    }
                }
            }

            // 动态加载
            DemoSection {
                title: "动态加载选项",
                description: "使用 load_data 实现动态加载选项。",

                div {
                    style: "width: 300px;",
                    Cascader {
                        options: dynamic_options(),
                        placeholder: "请选择",
                        load_data: move |selected_options| {
                            // 模拟异步加载
                            web_sys::console::log_1(&format!("加载数据: {:?}", selected_options).into());

                            // 这里可以根据选中的选项动态加载子选项
                            let new_options = vec![
                                CascaderOption::new("loaded1", "动态加载1")
                                    .with_children(vec![
                                        CascaderOption::new("sub1", "子选项1"),
                                        CascaderOption::new("sub2", "子选项2"),
                                    ]),
                                CascaderOption::new("loaded2", "动态加载2"),
                            ];
                            dynamic_options.set(new_options);
                        }
                    }
                }
            }

            // 自定义样式
            DemoSection {
                title: "自定义样式",
                description: "自定义级联选择器的样式。",

                div {
                    style: "width: 300px;",
                    Cascader {
                        options: basic_options.clone(),
                        placeholder: "自定义样式",
                        class: "custom-cascader",
                        style: "border: 2px solid #1890ff; border-radius: 8px;"
                    }
                }
            }

            // 回调事件
            DemoSection {
                title: "回调事件",
                description: "监听级联选择器的各种事件。",

                div {
                    style: "width: 300px;",
                    Cascader {
                        options: basic_options.clone(),
                        placeholder: "请选择地区",
                        on_change: move |value| {
                            web_sys::console::log_1(&format!("选择变化: {:?}", value).into());
                        },
                        on_popup_visible_change: move |visible| {
                            popup_visible.set(visible);
                            web_sys::console::log_1(&format!("弹窗显示状态: {}", visible).into());
                        }
                    }
                }

                div {
                    style: "margin-top: 16px; color: #666; font-size: 14px;",
                    "弹窗显示状态: {popup_visible}"
                }
            }
        }
    }
}

//! Pagination 组件示例
//!
//! 展示 Pagination 分页组件的各种用法

use crate::common::{ApiDocumentation, DemoSection, PropDoc};
use dioxus::prelude::*;

/// Pagination 组件演示页面
#[component]
pub fn PaginationDemo() -> Element {
    let mut current = use_signal(|| 1);
    let mut page_size = use_signal(|| 10);
    let mut simple_current = use_signal(|| 1);
    let mut mini_current = use_signal(|| 1);
    let mut show_size_changer_current = use_signal(|| 1);
    let mut show_size_changer_page_size = use_signal(|| 10);

    rsx! {
        div {
            class: "pagination-demo",
            style: "padding: 24px;",

            h1 {
                style: "margin: 0 0 24px 0; color: #262626; font-size: 28px;",
                "Pagination 分页"
            }

            p {
                style: "margin: 0 0 32px 0; color: #666; font-size: 14px;",
                "采用分页的形式分隔长列表，每次只加载一个页面。"
            }

            // 基础分页
            DemoSection {
                title: "基础分页".to_string(),
                description: "基础分页。".to_string(),
                div {
                    style: "display: flex; align-items: center; gap: 8px;",

                    button {
                        style: if current() == 1 {
                            "padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 6px; background: #f5f5f5; color: #bfbfbf; cursor: not-allowed;"
                        } else {
                            "padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer;"
                        },
                        disabled: current() == 1,
                        onclick: move |_| if current() > 1 { current.set(current() - 1) },
                        "‹"
                    }

                    for page in 1..=10 {
                        button {
                            style: if current() == page {
                                "padding: 4px 8px; border: 1px solid #1890ff; border-radius: 6px; background: #1890ff; color: white; cursor: pointer; min-width: 32px;"
                            } else {
                                "padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer; min-width: 32px;"
                            },
                            onclick: move |_| current.set(page),
                            "{page}"
                        }
                    }

                    button {
                        style: if current() == 10 {
                            "padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 6px; background: #f5f5f5; color: #bfbfbf; cursor: not-allowed;"
                        } else {
                            "padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer;"
                        },
                        disabled: current() == 10,
                        onclick: move |_| if current() < 10 { current.set(current() + 1) },
                        "›"
                    }
                }
            }

            // 更多
            DemoSection {
                title: "更多".to_string(),
                description: "更多分页。".to_string(),
                div {
                    style: "display: flex; align-items: center; gap: 8px;",

                    button {
                        style: "padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer;",
                        "‹"
                    }

                    button {
                        style: "padding: 4px 8px; border: 1px solid #1890ff; border-radius: 6px; background: #1890ff; color: white; cursor: pointer; min-width: 32px;",
                        "1"
                    }
                    button {
                        style: "padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer; min-width: 32px;",
                        "2"
                    }
                    button {
                        style: "padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer; min-width: 32px;",
                        "3"
                    }
                    button {
                        style: "padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer; min-width: 32px;",
                        "4"
                    }
                    button {
                        style: "padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer; min-width: 32px;",
                        "5"
                    }
                    span {
                        style: "padding: 4px 8px; color: #666;",
                        "•••"
                    }
                    button {
                        style: "padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer; min-width: 32px;",
                        "50"
                    }

                    button {
                        style: "padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer;",
                        "›"
                    }
                }
            }

            // 改变每页显示条目数
            DemoSection {
                title: "改变每页显示条目数".to_string(),
                description: "改变每页显示条目数。".to_string(),
                div {
                    style: "display: flex; align-items: center; gap: 16px;",

                    div {
                        style: "display: flex; align-items: center; gap: 8px;",

                        button {
                            style: if show_size_changer_current() == 1 {
                                "padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 6px; background: #f5f5f5; color: #bfbfbf; cursor: not-allowed;"
                            } else {
                                "padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer;"
                            },
                            disabled: show_size_changer_current() == 1,
                            onclick: move |_| if show_size_changer_current() > 1 { show_size_changer_current.set(show_size_changer_current() - 1) },
                            "‹"
                        }

                        for page in 1..=5 {
                            button {
                                style: if show_size_changer_current() == page {
                                    "padding: 4px 8px; border: 1px solid #1890ff; border-radius: 6px; background: #1890ff; color: white; cursor: pointer; min-width: 32px;"
                                } else {
                                    "padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer; min-width: 32px;"
                                },
                                onclick: move |_| show_size_changer_current.set(page),
                                "{page}"
                            }
                        }

                        button {
                            style: if show_size_changer_current() == 5 {
                                "padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 6px; background: #f5f5f5; color: #bfbfbf; cursor: not-allowed;"
                            } else {
                                "padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer;"
                            },
                            disabled: show_size_changer_current() == 5,
                            onclick: move |_| if show_size_changer_current() < 5 { show_size_changer_current.set(show_size_changer_current() + 1) },
                            "›"
                        }
                    }

                    select {
                        style: "padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 6px; background: white;",
                        value: "{show_size_changer_page_size()}",
                        onchange: move |evt| {
                            if let Ok(size) = evt.value().parse::<i32>() {
                                show_size_changer_page_size.set(size);
                            }
                        },

                        option { value: "10", "10 / page" }
                        option { value: "20", "20 / page" }
                        option { value: "50", "50 / page" }
                        option { value: "100", "100 / page" }
                    }

                    span {
                        style: "color: #666; font-size: 14px;",
                        "共 85 条"
                    }
                }
            }

            // 快速跳转
            DemoSection {
                title: "快速跳转".to_string(),
                description: "快速跳转到某一页。".to_string(),
                div {
                    style: "display: flex; align-items: center; gap: 16px;",

                    div {
                        style: "display: flex; align-items: center; gap: 8px;",

                        button {
                            style: "padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer;",
                            "‹"
                        }

                        for page in 1..=5 {
                            button {
                                style: if page == 1 {
                                    "padding: 4px 8px; border: 1px solid #1890ff; border-radius: 6px; background: #1890ff; color: white; cursor: pointer; min-width: 32px;"
                                } else {
                                    "padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer; min-width: 32px;"
                                },
                                "{page}"
                            }
                        }

                        button {
                            style: "padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer;",
                            "›"
                        }
                    }

                    div {
                        style: "display: flex; align-items: center; gap: 8px;",

                        span {
                            style: "color: #666; font-size: 14px;",
                            "跳至"
                        }
                        input {
                            r#type: "number",
                            style: "width: 50px; padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 6px;",
                            min: 1,
                            max: 50
                        }
                        span {
                            style: "color: #666; font-size: 14px;",
                            "页"
                        }
                    }
                }
            }

            // 迷你版本
            DemoSection {
                title: "迷你版本".to_string(),
                description: "迷你版本。".to_string(),
                div {
                    style: "display: flex; align-items: center; gap: 4px;",

                    button {
                        style: if mini_current() == 1 {
                            "padding: 2px 6px; border: 1px solid #d9d9d9; border-radius: 4px; background: #f5f5f5; color: #bfbfbf; cursor: not-allowed; font-size: 12px;"
                        } else {
                            "padding: 2px 6px; border: 1px solid #d9d9d9; border-radius: 4px; background: white; cursor: pointer; font-size: 12px;"
                        },
                        disabled: mini_current() == 1,
                        onclick: move |_| if mini_current() > 1 { mini_current.set(mini_current() - 1) },
                        "‹"
                    }

                    for page in 1..=5 {
                        button {
                            style: if mini_current() == page {
                                "padding: 2px 6px; border: 1px solid #1890ff; border-radius: 4px; background: #1890ff; color: white; cursor: pointer; min-width: 24px; font-size: 12px;"
                            } else {
                                "padding: 2px 6px; border: 1px solid #d9d9d9; border-radius: 4px; background: white; cursor: pointer; min-width: 24px; font-size: 12px;"
                            },
                            onclick: move |_| mini_current.set(page),
                            "{page}"
                        }
                    }

                    button {
                        style: if mini_current() == 5 {
                            "padding: 2px 6px; border: 1px solid #d9d9d9; border-radius: 4px; background: #f5f5f5; color: #bfbfbf; cursor: not-allowed; font-size: 12px;"
                        } else {
                            "padding: 2px 6px; border: 1px solid #d9d9d9; border-radius: 4px; background: white; cursor: pointer; font-size: 12px;"
                        },
                        disabled: mini_current() == 5,
                        onclick: move |_| if mini_current() < 5 { mini_current.set(mini_current() + 1) },
                        "›"
                    }
                }
            }

            // 简洁版本
            DemoSection {
                title: "简洁版本".to_string(),
                description: "简单的翻页。".to_string(),
                div {
                    style: "display: flex; align-items: center; gap: 8px;",

                    button {
                        style: if simple_current() == 1 {
                            "padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 6px; background: #f5f5f5; color: #bfbfbf; cursor: not-allowed;"
                        } else {
                            "padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer;"
                        },
                        disabled: simple_current() == 1,
                        onclick: move |_| if simple_current() > 1 { simple_current.set(simple_current() - 1) },
                        "‹"
                    }

                    div {
                        style: "display: flex; align-items: center; gap: 4px;",

                        input {
                            r#type: "number",
                            style: "width: 50px; padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 6px; text-align: center;",
                            value: "{simple_current()}",
                            min: 1,
                            max: 50,
                            onchange: move |evt| {
                                if let Ok(page) = evt.value().parse::<i32>() {
                                    if page >= 1 && page <= 50 {
                                        simple_current.set(page);
                                    }
                                }
                            }
                        }
                        span {
                            style: "color: #666; font-size: 14px;",
                            "/ 50"
                        }
                    }

                    button {
                        style: if simple_current() == 50 {
                            "padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 6px; background: #f5f5f5; color: #bfbfbf; cursor: not-allowed;"
                        } else {
                            "padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer;"
                        },
                        disabled: simple_current() == 50,
                        onclick: move |_| if simple_current() < 50 { simple_current.set(simple_current() + 1) },
                        "›"
                    }
                }
            }

            // 上一步和下一步
            DemoSection {
                title: "上一步和下一步".to_string(),
                description: "修改上一步和下一步为文字链接。".to_string(),
                div {
                    style: "display: flex; align-items: center; gap: 8px;",

                    button {
                        style: "padding: 4px 12px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer;",
                        "Previous"
                    }

                    for page in 1..=5 {
                        button {
                            style: if page == 1 {
                                "padding: 4px 8px; border: 1px solid #1890ff; border-radius: 6px; background: #1890ff; color: white; cursor: pointer; min-width: 32px;"
                            } else {
                                "padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer; min-width: 32px;"
                            },
                            "{page}"
                        }
                    }

                    button {
                        style: "padding: 4px 12px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer;",
                        "Next"
                    }
                }
            }

            // 全部展示
            DemoSection {
                title: "全部展示".to_string(),
                description: "展示所有配置选项。".to_string(),
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    div {
                        style: "display: flex; align-items: center; gap: 16px; flex-wrap: wrap;",

                        div {
                            style: "display: flex; align-items: center; gap: 8px;",

                            button {
                                style: "padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer;",
                                "‹"
                            }

                            for page in 1..=5 {
                                button {
                                    style: if page == 1 {
                                        "padding: 4px 8px; border: 1px solid #1890ff; border-radius: 6px; background: #1890ff; color: white; cursor: pointer; min-width: 32px;"
                                    } else {
                                        "padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer; min-width: 32px;"
                                    },
                                    "{page}"
                                }
                            }

                            button {
                                style: "padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 6px; background: white; cursor: pointer;",
                                "›"
                            }
                        }

                        select {
                            style: "padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 6px; background: white;",

                            option { value: "10", "10 / page" }
                            option { value: "20", "20 / page" }
                            option { value: "50", "50 / page" }
                            option { value: "100", "100 / page" }
                        }

                        div {
                            style: "display: flex; align-items: center; gap: 8px;",

                            span {
                                style: "color: #666; font-size: 14px;",
                                "跳至"
                            }
                            input {
                                r#type: "number",
                                style: "width: 50px; padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 6px;",
                                min: 1,
                                max: 50
                            }
                            span {
                                style: "color: #666; font-size: 14px;",
                                "页"
                            }
                        }
                    }

                    div {
                        style: "color: #666; font-size: 14px;",
                        "共 85 条记录，每页 10 条，共 9 页，当前第 1 页"
                    }
                }
            }

            // API 文档
            ApiDocumentation {
                component_name: "Pagination".to_string(),
                props: vec![
                    PropDoc {
                        name: "current".to_string(),
                        prop_type: "i32".to_string(),
                        default: "-".to_string(),
                        description: "当前页数".to_string(),
                    },
                    PropDoc {
                        name: "default_current".to_string(),
                        prop_type: "i32".to_string(),
                        default: "1".to_string(),
                        description: "默认的当前页数".to_string(),
                    },
                    PropDoc {
                        name: "default_page_size".to_string(),
                        prop_type: "i32".to_string(),
                        default: "10".to_string(),
                        description: "默认的每页条数".to_string(),
                    },
                    PropDoc {
                        name: "disabled".to_string(),
                        prop_type: "bool".to_string(),
                        default: "-".to_string(),
                        description: "禁用分页".to_string(),
                    },
                    PropDoc {
                        name: "hide_on_single_page".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "只有一页时是否隐藏分页器".to_string(),
                    },
                    PropDoc {
                        name: "item_render".to_string(),
                        prop_type: "Function".to_string(),
                        default: "-".to_string(),
                        description: "用于自定义页码的结构".to_string(),
                    },
                    PropDoc {
                        name: "page_size".to_string(),
                        prop_type: "i32".to_string(),
                        default: "-".to_string(),
                        description: "每页条数".to_string(),
                    },
                    PropDoc {
                        name: "page_size_options".to_string(),
                        prop_type: "Vec<String>".to_string(),
                        default: "['10', '20', '50', '100']".to_string(),
                        description: "指定每页可以显示多少条".to_string(),
                    },
                    PropDoc {
                        name: "responsive".to_string(),
                        prop_type: "bool".to_string(),
                        default: "-".to_string(),
                        description: "当 size 未指定时，根据屏幕宽度自动调整尺寸".to_string(),
                    },
                    PropDoc {
                        name: "show_less_items".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "是否显示较少页面内容".to_string(),
                    },
                    PropDoc {
                        name: "show_quick_jumper".to_string(),
                        prop_type: "bool | Object".to_string(),
                        default: "false".to_string(),
                        description: "是否可以快速跳转至某页".to_string(),
                    },
                    PropDoc {
                        name: "show_size_changer".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "是否展示 pageSize 切换器".to_string(),
                    },
                    PropDoc {
                        name: "show_title".to_string(),
                        prop_type: "bool".to_string(),
                        default: "true".to_string(),
                        description: "是否显示原生 tooltip 页码提示".to_string(),
                    },
                    PropDoc {
                        name: "show_total".to_string(),
                        prop_type: "Function".to_string(),
                        default: "-".to_string(),
                        description: "用于显示数据总量和当前数据顺序".to_string(),
                    },
                    PropDoc {
                        name: "simple".to_string(),
                        prop_type: "bool".to_string(),
                        default: "-".to_string(),
                        description: "当添加该属性时，显示为简单分页".to_string(),
                    },
                    PropDoc {
                        name: "size".to_string(),
                        prop_type: "String".to_string(),
                        default: "default".to_string(),
                        description: "当为 small 时，是小尺寸分页".to_string(),
                    },
                    PropDoc {
                        name: "total".to_string(),
                        prop_type: "i32".to_string(),
                        default: "0".to_string(),
                        description: "数据总数".to_string(),
                    },
                    PropDoc {
                        name: "onchange".to_string(),
                        prop_type: "Function".to_string(),
                        default: "-".to_string(),
                        description: "页码或 pageSize 改变的回调".to_string(),
                    },
                    PropDoc {
                        name: "on_show_size_change".to_string(),
                        prop_type: "Function".to_string(),
                        default: "-".to_string(),
                        description: "pageSize 变化的回调".to_string(),
                    },
                ]
            }
        }
    }
}

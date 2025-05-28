//!
//! 展示 Pagination 组件的各种用法和样式

use crate::common::*;
use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

/// 显示总数的函数
fn show_total_fn(total: usize, range: (usize, usize)) -> String {
    format!("第 {}-{} 条，共 {} 条", range.0, range.1, total)
}

/// Pagination 组件演示
#[component]
pub fn PaginationDemo() -> Element {
    let mut basic_current = use_signal(|| 1);
    let mut basic_page_size = use_signal(|| 10);
    let mut more_current = use_signal(|| 6);
    let mut changer_current = use_signal(|| 3);
    let mut changer_page_size = use_signal(|| 10);
    let mut jumper_current = use_signal(|| 2);
    let mut mini_current = use_signal(|| 1);
    let mut simple_current = use_signal(|| 1);
    let mut controlled_current = use_signal(|| 1);
    let mut controlled_page_size = use_signal(|| 20);
    let mut total_current = use_signal(|| 1);
    let mut event_log = use_signal(|| Vec::<String>::new());

    // 模拟数据
    let total_items = 500;
    let small_total = 50;
    let large_total = 1000;

    // 页面大小选项
    let page_size_options = vec![
        PageSizeOption::new(10),
        PageSizeOption::new(20),
        PageSizeOption::new(50),
        PageSizeOption::new(100),
    ];

    // 添加事件日志
    let mut add_log = move |message: String| {
        let mut logs = event_log();
        logs.insert(0, message);
        if logs.len() > 5 {
            logs.truncate(5);
        }
        event_log.set(logs);
    };

    // 自定义总数显示
    let show_total_render = Some(show_total_fn as fn(usize, (usize, usize)) -> String);

    rsx! {
        div {
            class: "component-showcase",

            h1 {
                style: "margin: 0 0 16px 0; color: #262626; font-size: 28px;",
                "Pagination 分页"
            }

            p {
                style: "margin: 0 0 24px 0; color: #666; font-size: 14px;",
                "采用分页的形式分隔长列表，每次只加载一个页面。"
            }

            // 基础用法
            DemoSection {
                title: "基础用法",
                description: "基础分页。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    Pagination {
                        current: basic_current(),
                        total: small_total,
                        page_size: basic_page_size(),
                        on_change: move |page| {
                            basic_current.set(page);
                            add_log(format!("基础分页: 切换到第 {} 页", page));
                        }
                    }

                    div {
                        style: "color: #666; font-size: 12px;",
                        "当前第 {basic_current()} 页，每页 {basic_page_size()} 条，共 {small_total} 条数据"
                    }
                }
            }

            // 更多分页
            DemoSection {
                title: "更多分页",
                description: "更多分页。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    Pagination {
                        current: more_current(),
                        total: total_items,
                        on_change: move |page| {
                            more_current.set(page);
                            add_log(format!("更多分页: 切换到第 {} 页", page));
                        }
                    }

                    div {
                        style: "color: #666; font-size: 12px;",
                        "当前第 {more_current()} 页，共 {total_items / 10} 页"
                    }
                }
            }

            // 改变每页显示条目数
            DemoSection {
                title: "改变每页显示条目数",
                description: "改变每页显示条目数。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    Pagination {
                        current: changer_current(),
                        total: total_items,
                        page_size: changer_page_size(),
                        show_size_changer: true,
                        page_size_options: Some(page_size_options.clone()),
                        on_change: move |page| {
                            changer_current.set(page);
                            add_log(format!("页面大小变更分页: 切换到第 {} 页", page));
                        },
                        on_show_size_change: move |(page, size)| {
                            changer_current.set(page);
                            changer_page_size.set(size);
                            add_log(format!("页面大小变更: 第 {} 页，每页 {} 条", page, size));
                        }
                    }

                    div {
                        style: "color: #666; font-size: 12px;",
                        "当前第 {changer_current()} 页，每页 {changer_page_size()} 条"
                    }
                }
            }

            // 快速跳转
            DemoSection {
                title: "快速跳转",
                description: "快速跳转到某一页。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    Pagination {
                        current: jumper_current(),
                        total: total_items,
                        show_quick_jumper: true,
                        show_size_changer: true,
                        page_size_options: Some(page_size_options.clone()),
                        on_change: move |page| {
                            jumper_current.set(page);
                            add_log(format!("快速跳转: 跳转到第 {} 页", page));
                        }
                    }

                    div {
                        style: "color: #666; font-size: 12px;",
                        "支持快速跳转和页面大小选择"
                    }
                }
            }

            // 迷你版
            DemoSection {
                title: "迷你版",
                description: "迷你版本。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    Pagination {
                        current: mini_current(),
                        total: total_items,
                        size: PaginationSize::Small,
                        show_size_changer: true,
                        show_quick_jumper: true,
                        on_change: move |page| {
                            mini_current.set(page);
                            add_log(format!("迷你分页: 切换到第 {} 页", page));
                        }
                    }

                    div {
                        style: "color: #666; font-size: 12px;",
                        "小尺寸分页组件"
                    }
                }
            }

            // 简洁版
            DemoSection {
                title: "简洁版",
                description: "简单的翻页。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    Pagination {
                        current: simple_current(),
                        total: total_items,
                        simple: true,
                        on_change: move |page| {
                            simple_current.set(page);
                            add_log(format!("简洁分页: 切换到第 {} 页", page));
                        }
                    }

                    div {
                        style: "color: #666; font-size: 12px;",
                        "简洁模式，适合移动端使用"
                    }
                }
            }

            // 受控
            DemoSection {
                title: "受控",
                description: "受控的分页器。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    div {
                        style: "display: flex; gap: 8px; margin-bottom: 16px;",

                        Button {
                            onclick: move |_| {
                                if controlled_current() > 1 {
                                    controlled_current.set(controlled_current() - 1);
                                }
                            },
                            disabled: controlled_current() <= 1,
                            "上一页"
                        }

                        Button {
                            onclick: move |_| {
                                let total_pages = (large_total + controlled_page_size() - 1) / controlled_page_size();
                                if controlled_current() < total_pages {
                                    controlled_current.set(controlled_current() + 1);
                                }
                            },
                            disabled: {
                                let total_pages = (large_total + controlled_page_size() - 1) / controlled_page_size();
                                controlled_current() >= total_pages
                            },
                            "下一页"
                        }

                        Button {
                            onclick: move |_| controlled_current.set(1),
                            "回到首页"
                        }

                        Button {
                            onclick: move |_| {
                                let total_pages = (large_total + controlled_page_size() - 1) / controlled_page_size();
                                controlled_current.set(total_pages);
                            },
                            "跳到末页"
                        }
                    }

                    Pagination {
                        current: controlled_current(),
                        total: large_total,
                        page_size: controlled_page_size(),
                        show_size_changer: true,
                        show_quick_jumper: true,
                        page_size_options: Some(page_size_options.clone()),
                        on_change: move |page| {
                            controlled_current.set(page);
                            add_log(format!("受控分页: 切换到第 {} 页", page));
                        },
                        on_show_size_change: move |(page, size)| {
                            controlled_current.set(page);
                            controlled_page_size.set(size);
                            add_log(format!("受控分页大小变更: 第 {} 页，每页 {} 条", page, size));
                        }
                    }

                    div {
                        style: "color: #666; font-size: 12px;",
                        "通过外部按钮控制分页状态"
                    }
                }
            }

            // 总数
            DemoSection {
                title: "总数",
                description: "通过设置 show_total 展示总共有多少数据。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    Pagination {
                        current: total_current(),
                        total: total_items,
                        show_total: true,
                        show_size_changer: true,
                        show_quick_jumper: true,
                        page_size_options: Some(page_size_options.clone()),
                        show_total_render: show_total_render,
                        on_change: move |page| {
                            total_current.set(page);
                            add_log(format!("总数分页: 切换到第 {} 页", page));
                        }
                    }

                    div {
                        style: "color: #666; font-size: 12px;",
                        "显示数据总数和当前范围"
                    }
                }
            }

            // 禁用状态
            DemoSection {
                title: "禁用状态",
                description: "禁用分页。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    Pagination {
                        current: 2,
                        total: total_items,
                        disabled: true,
                        show_size_changer: true,
                        show_quick_jumper: true
                    }

                    div {
                        style: "color: #666; font-size: 12px;",
                        "禁用状态下无法操作"
                    }
                }
            }

            // 隐藏单页
            DemoSection {
                title: "只有一页时隐藏分页器",
                description: "当只有一页时隐藏分页器。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    div {
                        "数据少于一页时:"
                        Pagination {
                            current: 1,
                            total: 5,
                            page_size: 10,
                            hide_on_single_page: true
                        }
                        div {
                            style: "color: #999; font-style: italic; margin-top: 8px;",
                            "(分页器已隐藏，因为只有一页数据)"
                        }
                    }

                    div {
                        style: "margin-top: 16px;",
                        "数据超过一页时:"
                        Pagination {
                            current: 1,
                            total: 25,
                            page_size: 10,
                            hide_on_single_page: true
                        }
                    }
                }
            }

            // 事件回调
            DemoSection {
                title: "事件回调",
                description: "演示分页事件回调。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    Pagination {
                        current: 1,
                        total: total_items,
                        show_size_changer: true,
                        show_quick_jumper: true,
                        page_size_options: Some(page_size_options.clone()),
                        on_change: move |page| {
                            add_log(format!("页码变化: 第 {} 页", page));
                        },
                        on_show_size_change: move |(page, size)| {
                            add_log(format!("页面大小变化: 第 {} 页，每页 {} 条", page, size));
                        }
                    }

                    div {
                        style: "background: #f5f5f5; padding: 12px; border-radius: 6px; max-height: 150px; overflow-y: auto;",

                        h4 {
                            style: "margin: 0 0 8px 0; color: #333;",
                            "事件日志:"
                        }

                        if event_log().is_empty() {
                            div {
                                style: "color: #999; font-style: italic;",
                                "暂无事件"
                            }
                        } else {
                            for (index, log) in event_log().iter().enumerate() {
                                div {
                                    key: "{index}",
                                    style: "font-family: monospace; font-size: 12px; color: #666; margin-bottom: 4px;",
                                    "{log}"
                                }
                            }
                        }
                    }
                }
            }

            // 自定义样式
            DemoSection {
                title: "自定义样式",
                description: "通过 class 和 style 属性自定义样式。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    div {
                        style: "background: #001529; padding: 16px; border-radius: 6px;",

                        Pagination {
                            current: 3,
                            total: total_items,
                            style: "color: white;",
                            class: "dark-pagination"
                        }
                    }

                    div {
                        style: "background: linear-gradient(45deg, #ff6b6b, #4ecdc4); padding: 16px; border-radius: 6px;",

                        Pagination {
                            current: 2,
                            total: total_items,
                            style: "color: white; font-weight: bold;"
                        }
                    }

                    Pagination {
                        current: 4,
                        total: total_items,
                        style: "border: 2px solid #1890ff; border-radius: 8px; padding: 8px; background: #f0f8ff;"
                    }
                }
            }
        }
    }
}

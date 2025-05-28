//!
//! 展示 Anchor 组件的各种用法和样式

use crate::common::*;
use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

/// Anchor 组件演示
#[component]
pub fn AnchorDemo() -> Element {
    let mut current_anchor = use_signal(|| String::new());
    let mut offset_top = use_signal(|| 50);

    // 创建锚点链接数据
    let basic_links = vec![
        AnchorLink::new("#basic-usage", "基础用法"),
        AnchorLink::new("#static-position", "静态位置"),
        AnchorLink::new("#custom-offset", "自定义偏移量"),
        AnchorLink::new("#nested-anchors", "嵌套锚点"),
    ];

    let nested_links = vec![
        AnchorLink::new("#section1", "第一部分").with_children(vec![
            AnchorLink::new("#section1-1", "1.1 子章节"),
            AnchorLink::new("#section1-2", "1.2 子章节"),
        ]),
        AnchorLink::new("#section2", "第二部分").with_children(vec![
            AnchorLink::new("#section2-1", "2.1 子章节"),
            AnchorLink::new("#section2-2", "2.2 子章节"),
        ]),
        AnchorLink::new("#section3", "第三部分"),
    ];

    rsx! {
        div {
            class: "component-showcase",

            h1 {
                style: "margin: 0 0 16px 0; color: #262626; font-size: 28px;",
                "Anchor 锚点"
            }

            p {
                style: "margin: 0 0 24px 0; color: #666; font-size: 14px;",
                "用于跳转到页面指定位置。需要展现当前页面上可供跳转的锚点链接，以及快速在锚点之间跳转。"
            }

            // 基础用法
            DemoSection {
                title: "基础用法",
                description: "最简单的用法。",

                div {
                    style: "display: flex; gap: 24px;",

                    div {
                        style: "width: 200px;",
                        Anchor {
                            links: basic_links.clone(),
                            offset_top: 80,
                            on_click: move |href: String| {
                                web_sys::console::log_1(&format!("点击锚点: {}", href).into());
                            },
                            on_change: move |href: String| {
                                current_anchor.set(href.clone());
                                web_sys::console::log_1(&format!("当前锚点: {}", href).into());
                            }
                        }
                    }

                    div {
                        style: "flex: 1; height: 400px; overflow-y: auto; border: 1px solid #d9d9d9; padding: 16px;",

                        div {
                            id: "basic-usage",
                            style: "height: 200px; background: #f0f2ff; padding: 20px; margin-bottom: 20px; border-radius: 6px;",
                            h3 { "基础用法" }
                            p { "这里是基础用法的内容区域。锚点可以快速定位到这个位置。" }
                            p { "内容区域..." }
                        }

                        div {
                            id: "static-position",
                            style: "height: 200px; background: #f6ffed; padding: 20px; margin-bottom: 20px; border-radius: 6px;",
                            h3 { "静态位置" }
                            p { "这里是静态位置的内容区域。" }
                            p { "内容区域..." }
                        }

                        div {
                            id: "custom-offset",
                            style: "height: 200px; background: #fff2e8; padding: 20px; margin-bottom: 20px; border-radius: 6px;",
                            h3 { "自定义偏移量" }
                            p { "这里是自定义偏移量的内容区域。" }
                            p { "内容区域..." }
                        }

                        div {
                            id: "nested-anchors",
                            style: "height: 200px; background: #f9f0ff; padding: 20px; margin-bottom: 20px; border-radius: 6px;",
                            h3 { "嵌套锚点" }
                            p { "这里是嵌套锚点的内容区域。" }
                            p { "内容区域..." }
                        }
                    }
                }
            }

            // 自定义偏移量
            DemoSection {
                title: "自定义偏移量",
                description: "设置锚点滚动偏移量。",

                div {
                    style: "margin-bottom: 16px;",
                    "偏移量: "
                    input {
                        r#type: "number",
                        value: "{offset_top}",
                        min: "0",
                        max: "200",
                        style: "margin-left: 8px; padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 4px;",
                        oninput: move |e| {
                            if let Ok(val) = e.value().parse::<i32>() {
                                offset_top.set(val);
                            }
                        }
                    }
                    " px"
                }

                div {
                    style: "display: flex; gap: 24px;",

                    div {
                        style: "width: 200px;",
                        Anchor {
                            links: basic_links.clone(),
                            offset_top: offset_top(),
                            on_click: move |href: String| {
                                web_sys::console::log_1(&format!("点击锚点: {}", href).into());
                            }
                        }
                    }

                    div {
                        style: "flex: 1; height: 300px; overflow-y: auto; border: 1px solid #d9d9d9; padding: 16px;",
                        p { "偏移量: {offset_top}px" }
                        div {
                            style: "height: 800px; background: linear-gradient(to bottom, #e6f7ff, #bae7ff);",
                            p { style: "padding: 20px;", "滚动内容区域..." }
                            p { style: "padding: 20px;", "内容区域..." }
                        }
                    }
                }
            }

            // 嵌套锚点
            DemoSection {
                title: "嵌套锚点",
                description: "锚点可以嵌套。",

                div {
                    style: "display: flex; gap: 24px;",

                    div {
                        style: "width: 200px;",
                        Anchor {
                            links: nested_links.clone(),
                            offset_top: 60,
                            current_anchor: current_anchor(),
                            on_change: move |href: String| {
                                current_anchor.set(href);
                            }
                        }
                    }

                    div {
                        style: "flex: 1; height: 400px; overflow-y: auto; border: 1px solid #d9d9d9; padding: 16px;",

                        div {
                            id: "section1",
                            style: "margin-bottom: 20px;",
                            h3 { style: "color: #1890ff; margin-bottom: 16px;", "第一部分" }

                            div {
                                id: "section1-1",
                                style: "height: 150px; background: #f0f2ff; padding: 16px; margin-bottom: 16px; border-radius: 6px;",
                                h4 { "1.1 子章节" }
                                p { "这是第一部分的第一个子章节内容。" }
                                p { "内容区域..." }
                            }

                            div {
                                id: "section1-2",
                                style: "height: 150px; background: #f0f2ff; padding: 16px; margin-bottom: 16px; border-radius: 6px;",
                                h4 { "1.2 子章节" }
                                p { "这是第一部分的第二个子章节内容。" }
                                p { "内容区域..." }
                            }
                        }

                        div {
                            id: "section2",
                            style: "margin-bottom: 20px;",
                            h3 { style: "color: #52c41a; margin-bottom: 16px;", "第二部分" }

                            div {
                                id: "section2-1",
                                style: "height: 150px; background: #f6ffed; padding: 16px; margin-bottom: 16px; border-radius: 6px;",
                                h4 { "2.1 子章节" }
                                p { "这是第二部分的第一个子章节内容。" }
                                p { "内容区域..." }
                            }

                            div {
                                id: "section2-2",
                                style: "height: 150px; background: #f6ffed; padding: 16px; margin-bottom: 16px; border-radius: 6px;",
                                h4 { "2.2 子章节" }
                                p { "这是第二部分的第二个子章节内容。" }
                                p { "内容区域..." }
                            }
                        }

                        div {
                            id: "section3",
                            style: "height: 200px; background: #fff2e8; padding: 16px; border-radius: 6px;",
                            h3 { style: "color: #fa8c16; margin-bottom: 16px;", "第三部分" }
                            p { "这是第三部分的内容。" }
                            p { "内容区域..." }
                        }
                    }
                }
            }

            // 当前锚点状态
            DemoSection {
                title: "当前锚点",
                description: "显示当前激活的锚点。",

                div {
                    style: "padding: 16px; background: #f5f5f5; border-radius: 6px;",
                    "当前激活的锚点: "
                    span {
                        style: "color: #1890ff; font-weight: 500;",
                        {
                            let anchor = current_anchor();
                            if anchor.is_empty() { "无".to_string() } else { anchor }
                        }
                    }
                }
            }
        }
    }
}

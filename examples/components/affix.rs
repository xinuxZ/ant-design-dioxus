//!
//! 展示 Affix 组件的各种用法和样式

use crate::common::*;
use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

/// Affix 组件演示
#[component]
pub fn AffixDemo() -> Element {
    let mut count = use_signal(|| 0);
    let mut top_offset = use_signal(|| 10);
    let mut bottom_offset = use_signal(|| 10);

    rsx! {
        div {
            class: "component-showcase",

            h1 {
                style: "margin: 0 0 16px 0; color: #262626; font-size: 28px;",
                "Affix 固钉"
            }

            p {
                style: "margin: 0 0 24px 0; color: #666; font-size: 14px;",
                "将页面元素钉在可视范围。当内容区域比较长，需要滚动页面时，这部分内容对应的操作或者导航需要在滚动过程中始终展现。"
            }

            // 基础用法
            DemoSection {
                title: "基础用法",
                description: "最简单的用法。",

                div {
                    style: "height: 100px; overflow-y: auto; border: 1px solid #d9d9d9; padding: 16px;",

                    Affix {
                        offset_top: 10,
                        on_change: move |fixed: bool| {
                            web_sys::console::log_1(&format!("Affix state changed: {}", fixed).into());
                        },

                        Button {
                            button_type: ButtonType::Primary,
                            "固定在顶部"
                        }
                    }

                    div {
                        style: "height: 1000px; background: linear-gradient(to bottom, #f0f0f0, #ffffff);",
                        p { style: "padding: 20px;", "滚动页面查看固钉效果" }
                        p { style: "padding: 20px;", "内容区域..." }
                        p { style: "padding: 20px;", "内容区域..." }
                        p { style: "padding: 20px;", "内容区域..." }
                        p { style: "padding: 20px;", "内容区域..." }
                    }
                }
            }

            // 固定位置
            DemoSection {
                title: "固定位置",
                description: "可以设置偏移量。",

                div {
                    style: "margin-bottom: 16px;",
                    "偏移量: "
                    input {
                        r#type: "number",
                        value: "{top_offset}",
                        min: "0",
                        max: "100",
                        style: "margin-left: 8px; padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 4px;",
                        oninput: move |e| {
                            if let Ok(val) = e.value().parse::<i32>() {
                                top_offset.set(val);
                            }
                        }
                    }
                    " px"
                }

                div {
                    style: "height: 200px; overflow-y: auto; border: 1px solid #d9d9d9; padding: 16px;",

                    Affix {
                        offset_top: top_offset(),

                        Button {
                            button_type: ButtonType::Primary,
                            "固定按钮 (偏移 {top_offset}px)"
                        }
                    }

                    div {
                        style: "height: 800px; background: linear-gradient(45deg, #f0f0f0, #e0e0e0);",
                        p { style: "padding: 20px;", "滚动查看不同偏移量的效果" }
                        p { style: "padding: 20px;", "内容区域..." }
                        p { style: "padding: 20px;", "内容区域..." }
                    }
                }
            }

            // 固定在底部
            DemoSection {
                title: "固定在底部",
                description: "也可以固定在底部。",

                div {
                    style: "margin-bottom: 16px;",
                    "底部偏移量: "
                    input {
                        r#type: "number",
                        value: "{bottom_offset}",
                        min: "0",
                        max: "100",
                        style: "margin-left: 8px; padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 4px;",
                        oninput: move |e| {
                            if let Ok(val) = e.value().parse::<i32>() {
                                bottom_offset.set(val);
                            }
                        }
                    }
                    " px"
                }

                div {
                    style: "height: 200px; overflow-y: auto; border: 1px solid #d9d9d9; padding: 16px;",

                    div {
                        style: "height: 800px; background: linear-gradient(to top, #f0f0f0, #e0e0e0);",
                        p { style: "padding: 20px;", "滚动查看底部固定效果" }
                        p { style: "padding: 20px;", "内容区域..." }
                        p { style: "padding: 20px;", "内容区域..." }
                    }

                    Affix {
                        offset_bottom: bottom_offset(),

                        Button {
                            button_type: ButtonType::Primary,
                            "固定在底部 (偏移 {bottom_offset}px)"
                        }
                    }
                }
            }

            // 回调函数
            DemoSection {
                title: "回调函数",
                description: "可以获得是否固定的状态。",

                div {
                    style: "margin-bottom: 16px;",
                    "状态变化次数: {count}"
                }

                div {
                    style: "height: 150px; overflow-y: auto; border: 1px solid #d9d9d9; padding: 16px;",

                    Affix {
                        offset_top: 20,
                        on_change: move |fixed: bool| {
                            count.set(count() + 1);
                            web_sys::console::log_1(&format!("Affix 状态改变: {} (第{}次)", if fixed { "固定" } else { "取消固定" }, count()).into());
                        },

                        div {
                            style: "padding: 8px 16px; background: #1890ff; color: white; border-radius: 4px;",
                            "监听状态变化"
                        }
                    }

                    div {
                        style: "height: 600px; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);",
                        p { style: "padding: 20px; color: white;", "滚动页面触发状态变化" }
                        p { style: "padding: 20px; color: white;", "内容区域..." }
                        p { style: "padding: 20px; color: white;", "内容区域..." }
                    }
                }
            }
        }
    }
}

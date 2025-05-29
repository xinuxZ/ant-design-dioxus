//!
//! 展示 BackTop 组件的各种用法和样式

use crate::common::*;
use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

/// BackTop 组件演示
#[component]
pub fn BackTopDemo() -> Element {
    let mut visibility_height = use_signal(|| 400);
    let mut click_count = use_signal(|| 0);
    let scroll_position = use_signal(|| 0);

    rsx! {
        div {
            class: "component-showcase",

            h1 {
                style: "margin: 0 0 16px 0; color: #262626; font-size: 28px;",
                "BackTop 回到顶部"
            }

            p {
                style: "margin: 0 0 24px 0; color: #666; font-size: 14px;",
                "返回页面顶部的操作按钮。当页面内容区域比较长时，当用户需要频繁返回顶部查看相关内容时使用。"
            }

            // 基础用法
            DemoSection {
                title: "基础用法",
                description: "最简单的用法。",

                div {
                    style: "position: relative; height: 300px; overflow-y: auto; border: 1px solid #d9d9d9; padding: 16px;",

                    div {
                        style: "height: 1000px; background: linear-gradient(to bottom, #e6f7ff, #bae7ff, #91d5ff, #69c0ff);",

                        div {
                            style: "padding: 20px;",
                            h3 { "滚动内容区域" }
                            p { "这是一个很长的内容区域，需要滚动查看。" }
                            p { "向下滚动，当滚动距离超过 400px 时，右下角会出现回到顶部按钮。" }

                            for i in 1..=20 {
                                p {
                                    style: "margin: 16px 0; padding: 16px; background: rgba(255, 255, 255, 0.8); border-radius: 6px;",
                                    "第 {i} 段内容 - 这里是一些示例文本内容，用于演示回到顶部功能。当页面内容很长时，用户可以通过点击回到顶部按钮快速返回页面顶部。"
                                }
                            }
                        }
                    }

                    BackTop {
                        visibility_height: 100,
                        on_click: move |_| {
                            click_count.set(click_count() + 1);
                            web_sys::console::log_1(&"点击了回到顶部按钮".into());
                        }
                    }
                }

                div {
                    style: "margin-top: 16px; color: #666; font-size: 14px;",
                    "回到顶部按钮点击次数: {click_count}"
                }
            }

            // 自定义样式
            DemoSection {
                title: "自定义样式",
                description: "可以自定义回到顶部按钮的样式和内容。",

                div {
                    style: "position: relative; height: 250px; overflow-y: auto; border: 1px solid #d9d9d9; padding: 16px;",

                    div {
                        style: "height: 800px; background: linear-gradient(45deg, #f6ffed, #d9f7be, #b7eb8f, #95de64);",

                        div {
                            style: "padding: 20px;",
                            h3 { "自定义样式演示" }
                            p { "这个示例展示了如何自定义回到顶部按钮的样式。" }

                            for i in 1..=15 {
                                div {
                                    style: "margin: 16px 0; padding: 16px; background: rgba(255, 255, 255, 0.9); border-radius: 8px; box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);",
                                    h4 { "内容块 {i}" }
                                    p { "这是第 {i} 个内容块，包含一些示例文本。自定义的回到顶部按钮具有不同的样式和动画效果。" }
                                }
                            }
                        }
                    }

                    BackTop {
                        visibility_height: 50,
                        style: "background: #52c41a; border-radius: 50%; width: 50px; height: 50px; box-shadow: 0 4px 12px rgba(82, 196, 26, 0.4);",

                        div {
                            style: "display: flex; align-items: center; justify-content: center; width: 100%; height: 100%; color: white; font-size: 18px; font-weight: bold;",
                            "↑"
                        }
                    }
                }
            }

            // 自定义触发高度
            DemoSection {
                title: "自定义触发高度",
                description: "可以自定义触发显示的滚动高度。",

                div {
                    style: "margin-bottom: 16px;",
                    "触发高度: "
                    input {
                        r#type: "number",
                        value: "{visibility_height}",
                        min: "0",
                        max: "500",
                        step: "50",
                        style: "margin-left: 8px; padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 4px;",
                        oninput: move |e| {
                            if let Ok(val) = e.value().parse::<i32>() {
                                visibility_height.set(val);
                            }
                        }
                    }
                    " px"
                }

                div {
                    style: "position: relative; height: 200px; overflow-y: auto; border: 1px solid #d9d9d9; padding: 16px;",

                    div {
                        style: "height: 600px; background: linear-gradient(to right, #fff2e8, #ffd8bf, #ffbb96, #ff9c6e);",

                        div {
                            style: "padding: 20px;",
                            h3 { "自定义触发高度演示" }
                            p { "当前设置的触发高度: {visibility_height}px" }
                            p { "滚动超过设定高度后，回到顶部按钮会出现。" }

                            for i in 1..=10 {
                                div {
                                    style: "margin: 20px 0; padding: 16px; background: rgba(255, 255, 255, 0.8); border-radius: 6px;",
                                    "内容区域 {i} - 滚动距离: {scroll_position}px"
                                }
                            }
                        }
                    }

                    BackTop {
                        visibility_height: visibility_height(),
                        on_click: move |_| {
                            web_sys::console::log_1(&format!("触发高度 {}px 的回到顶部按钮被点击", visibility_height()).into());
                        }
                    }
                }
            }

            // 自定义内容
            DemoSection {
                title: "自定义内容",
                description: "可以自定义按钮的内容。",

                div {
                    style: "position: relative; height: 200px; overflow-y: auto; border: 1px solid #d9d9d9; padding: 16px;",

                    div {
                        style: "height: 600px; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);",

                        div {
                            style: "padding: 20px; color: white;",
                            h3 { "自定义内容演示" }
                            p { "这个示例展示了如何自定义回到顶部按钮的内容。" }

                            for i in 1..=8 {
                                div {
                                    style: "margin: 20px 0; padding: 16px; background: rgba(255, 255, 255, 0.1); border-radius: 6px;",
                                    "内容块 {i} - 自定义按钮内容可以是文字、图标或其他元素。"
                                }
                            }
                        }
                    }

                    BackTop {
                        visibility_height: 80,

                        Button {
                            button_type: ButtonType::Primary,
                            size: ButtonSize::Large,
                            style: "border-radius: 50%; width: 60px; height: 60px; display: flex; align-items: center; justify-content: center;",
                            "TOP"
                        }
                    }
                }
            }

            // 多个回到顶部按钮
            DemoSection {
                title: "多个按钮",
                description: "可以在同一个容器中使用多个回到顶部按钮。",

                div {
                    style: "position: relative; height: 180px; overflow-y: auto; border: 1px solid #d9d9d9; padding: 16px;",

                    div {
                        style: "height: 500px; background: linear-gradient(to bottom, #ffeaa7, #fab1a0, #fd79a8, #e84393);",

                        div {
                            style: "padding: 20px;",
                            h3 { "多按钮演示" }
                            p { "这个示例展示了多个不同样式的回到顶部按钮。" }

                            for i in 1..=6 {
                                div {
                                    style: "margin: 20px 0; padding: 16px; background: rgba(255, 255, 255, 0.8); border-radius: 6px;",
                                    "内容区域 {i}"
                                }
                            }
                        }
                    }

                    // 第一个按钮 - 默认样式
                    BackTop {
                        visibility_height: 50,
                        style: "right: 80px;"
                    }

                    // 第二个按钮 - 自定义样式
                    BackTop {
                        visibility_height: 50,
                        style: "right: 20px; background: #fa541c; border-radius: 4px;",

                        div {
                            style: "padding: 8px 12px; color: white; font-size: 12px;",
                            "回顶部"
                        }
                    }
                }
            }
        }
    }
}

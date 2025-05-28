//! Spin 组件示例
//!
//! 展示 Spin 组件的各种用法，包括基础用法、尺寸变体、延迟显示、自定义指示器等。

use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn SpinDemo() -> Element {
    let mut loading = use_signal(|| false);
    let mut delay_loading = use_signal(|| false);

    rsx! {
        // style { include_str!("../src/components/spin/style.css") }

        div {
            style: "padding: 20px; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;",

            h1 { "Spin 组件示例" }

            // 基础用法
            div {
                style: "margin-bottom: 40px;",
                h2 { "基础用法" }
                p { "一个简单的 loading 状态。" }

                div {
                    style: "text-align: center; background: rgba(0,0,0,0.05); padding: 30px;",
                    Spin {}
                }
            }

            // 各种尺寸
            div {
                style: "margin-bottom: 40px;",
                h2 { "各种尺寸" }
                p { "小的用于文本加载，默认用于卡片容器级加载，大的用于页面级加载。" }

                div {
                    style: "display: flex; gap: 40px; align-items: center; justify-content: center; background: rgba(0,0,0,0.05); padding: 30px;",

                    div {
                        style: "text-align: center;",
                        Spin { size: SpinSize::Small }
                        div { style: "margin-top: 10px;", "Small" }
                    }

                    div {
                        style: "text-align: center;",
                        Spin { size: SpinSize::Default }
                        div { style: "margin-top: 10px;", "Default" }
                    }

                    div {
                        style: "text-align: center;",
                        Spin { size: SpinSize::Large }
                        div { style: "margin-top: 10px;", "Large" }
                    }
                }
            }

            // 包含描述
            div {
                style: "margin-bottom: 40px;",
                h2 { "包含描述" }
                p { "可以自定义描述文案。" }

                div {
                    style: "text-align: center; background: rgba(0,0,0,0.05); padding: 30px;",
                    Spin {
                        tip: "加载中...",
                        size: SpinSize::Large
                    }
                }
            }

            // 卡片加载中
            div {
                style: "margin-bottom: 40px;",
                h2 { "卡片加载中" }
                p { "可以直接把内容内嵌到 Spin 中，将现有容器变为加载状态。" }

                div {
                    style: "display: flex; gap: 20px;",

                    // 加载中的卡片
                    Spin {
                        spinning: loading(),
                        tip: "加载中...",
                        div {
                            style: "border: 1px solid #d9d9d9; border-radius: 6px; padding: 20px; width: 300px;",
                            h3 { "卡片标题" }
                            p { "卡片内容区域，这里可以放置任何内容。当 spinning 为 true 时，内容会被模糊处理并显示加载指示器。" }
                            p { "可以包含多个段落和其他元素。" }
                        }
                    }

                    // 正常卡片
                    div {
                        style: "border: 1px solid #d9d9d9; border-radius: 6px; padding: 20px; width: 300px;",
                        h3 { "正常卡片" }
                        p { "这是一个正常状态的卡片，没有加载效果。" }
                        p { "用于对比展示效果。" }
                    }
                }

                div {
                    style: "margin-top: 20px;",
                    button {
                        style: "padding: 8px 16px; background: #1677ff; color: white; border: none; border-radius: 4px; cursor: pointer;",
                        onclick: move |_| loading.set(!loading()),
                        if loading() { "停止加载" } else { "开始加载" }
                    }
                }
            }

            // 延迟显示
            div {
                style: "margin-bottom: 40px;",
                h2 { "延迟显示" }
                p { "延迟 300ms 显示 loading，避免闪烁。" }

                Spin {
                    spinning: delay_loading(),
                    delay: 300,
                    tip: "延迟加载中...",
                    div {
                        style: "border: 1px solid #d9d9d9; border-radius: 6px; padding: 40px; text-align: center;",
                        "这个区域有 300ms 的延迟显示，避免加载状态闪烁"
                    }
                }

                div {
                    style: "margin-top: 20px;",
                    button {
                        style: "padding: 8px 16px; background: #1677ff; color: white; border: none; border-radius: 4px; cursor: pointer;",
                        onclick: move |_| delay_loading.set(!delay_loading()),
                        if delay_loading() { "停止延迟加载" } else { "开始延迟加载" }
                    }
                }
            }

            // 自定义指示器
            div {
                style: "margin-bottom: 40px;",
                h2 { "自定义指示器" }
                p { "使用自定义指示器。" }

                div {
                    style: "text-align: center; background: rgba(0,0,0,0.05); padding: 30px;",
                    Spin {
                        tip: "自定义加载中...",
                        indicator: rsx! {
                            div {
                                style: "display: inline-block; width: 24px; height: 24px; border: 2px solid #f3f3f3; border-top: 2px solid #1677ff; border-radius: 50%; animation: spin 1s linear infinite;",
                            }
                        }
                    }
                }
            }

            // 各种场景
            div {
                h2 { "各种场景" }
                p { "在不同场景下的应用示例。" }

                div {
                    style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 20px; margin-top: 20px;",

                    // 表格加载
                    Spin {
                        spinning: true,
                        tip: "表格加载中...",
                        div {
                            style: "border: 1px solid #d9d9d9; border-radius: 6px;",
                            div {
                                style: "background: #fafafa; padding: 12px; border-bottom: 1px solid #d9d9d9; font-weight: 500;",
                                "数据表格"
                            }
                            div {
                                style: "padding: 20px;",
                                div { style: "height: 20px; background: #f0f0f0; margin-bottom: 8px; border-radius: 4px;" }
                                div { style: "height: 20px; background: #f0f0f0; margin-bottom: 8px; border-radius: 4px;" }
                                div { style: "height: 20px; background: #f0f0f0; border-radius: 4px;" }
                            }
                        }
                    }

                    // 图表加载
                    Spin {
                        spinning: true,
                        tip: "图表渲染中...",
                        size: SpinSize::Large,
                        div {
                            style: "border: 1px solid #d9d9d9; border-radius: 6px; height: 200px; display: flex; align-items: center; justify-content: center; background: #fafafa;",
                            "图表区域"
                        }
                    }

                    // 表单提交
                    Spin {
                        spinning: true,
                        tip: "提交中...",
                        div {
                            style: "border: 1px solid #d9d9d9; border-radius: 6px; padding: 20px;",
                            h4 { "表单提交" }
                            div { style: "margin-bottom: 12px;", "用户名: admin" }
                            div { style: "margin-bottom: 12px;", "邮箱: admin@example.com" }
                            button {
                                style: "padding: 8px 16px; background: #1677ff; color: white; border: none; border-radius: 4px;",
                                "提交"
                            }
                        }
                    }
                }
            }
        }

        // 添加自定义指示器的动画样式
        style {
            r#"
            @keyframes spin {
                0% { transform: rotate(0deg); }
                100% { transform: rotate(360deg); }
            }
            "#
        }
    }
}

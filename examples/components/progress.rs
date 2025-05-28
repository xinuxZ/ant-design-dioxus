//! Progress 组件示例
//!
//! 展示 Progress 组件的各种用法，包括线形、圆形、仪表盘进度条，以及不同状态和尺寸。

use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn ProgressDemo() -> Element {
    let mut percent = use_signal(|| 30);
    let mut circle_percent = use_signal(|| 75);
    let mut dynamic_percent = use_signal(|| 0);

    // 动态进度条效果
    use_effect(move || {
        spawn(async move {
            loop {
                gloo_timers::future::TimeoutFuture::new(100).await;
                dynamic_percent.set((dynamic_percent() + 1) % 101);
            }
        });
    });

    rsx! {
        // style { include_str!("../src/components/progress/style.css") }

        div {
            style: "padding: 20px; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;",

            h1 { "Progress 组件示例" }

            // 基本用法
            section {
                style: "margin-bottom: 40px;",
                h2 { "基本用法" }
                p { "标准的进度条。" }

                div {
                    style: "width: 400px;",
                    Progress { percent: 30 }
                    Progress { percent: 50, status: ProgressStatus::Active }
                    Progress { percent: 70, status: ProgressStatus::Exception }
                    Progress { percent: 100, status: ProgressStatus::Success }
                }
            }

            // 进度圈
            section {
                style: "margin-bottom: 40px;",
                h2 { "进度圈" }
                p { "圆形的进度。" }

                div {
                    style: "display: flex; gap: 20px; align-items: center;",

                    Progress {
                        progress_type: ProgressType::Circle,
                        percent: 75
                    }

                    Progress {
                        progress_type: ProgressType::Circle,
                        percent: 70,
                        status: ProgressStatus::Exception
                    }

                    Progress {
                        progress_type: ProgressType::Circle,
                        percent: 100,
                        status: ProgressStatus::Success
                    }
                }
            }

            // 小型进度圈
            section {
                style: "margin-bottom: 40px;",
                h2 { "小型进度圈" }
                p { "小一号的圆形进度。" }

                div {
                    style: "display: flex; gap: 20px; align-items: center;",

                    Progress {
                        progress_type: ProgressType::Circle,
                        percent: 30,
                        width: 80
                    }

                    Progress {
                        progress_type: ProgressType::Circle,
                        percent: 70,
                        width: 80,
                        status: ProgressStatus::Exception
                    }

                    Progress {
                        progress_type: ProgressType::Circle,
                        percent: 100,
                        width: 80,
                        status: ProgressStatus::Success
                    }
                }
            }

            // 仪表盘
            section {
                style: "margin-bottom: 40px;",
                h2 { "仪表盘" }
                p { "仪表盘样式的进度条。" }

                div {
                    style: "display: flex; gap: 20px; align-items: center;",

                    Progress {
                        progress_type: ProgressType::Dashboard,
                        percent: 75
                    }

                    Progress {
                        progress_type: ProgressType::Dashboard,
                        percent: 75,
                        stroke_color: "#52c41a".to_string()
                    }
                }
            }

            // 动态展示
            section {
                style: "margin-bottom: 40px;",
                h2 { "动态展示" }
                p { "会动的进度条才是好进度条。" }

                div {
                    style: "width: 400px; margin-bottom: 20px;",
                    Progress {
                        percent: percent(),
                        status: if percent() < 100 { ProgressStatus::Active } else { ProgressStatus::Success }
                    }
                }

                div {
                    style: "display: flex; gap: 20px; align-items: center; margin-bottom: 20px;",

                    Progress {
                        progress_type: ProgressType::Circle,
                        percent: circle_percent()
                    }

                    Progress {
                        progress_type: ProgressType::Dashboard,
                        percent: circle_percent()
                    }
                }

                div {
                    style: "display: flex; gap: 10px;",

                    button {
                        style: "padding: 8px 16px; background: #1677ff; color: white; border: none; border-radius: 4px; cursor: pointer;",
                        onclick: move |_| {
                            let new_percent = (percent() + 10).min(100);
                            percent.set(new_percent);
                            circle_percent.set(new_percent);
                        },
                        "+"
                    }

                    button {
                        style: "padding: 8px 16px; background: #ff4d4f; color: white; border: none; border-radius: 4px; cursor: pointer;",
                        onclick: move |_| {
                            let new_percent = (percent() - 10).max(0);
                            percent.set(new_percent);
                            circle_percent.set(new_percent);
                        },
                        "-"
                    }

                    button {
                        style: "padding: 8px 16px; background: #52c41a; color: white; border: none; border-radius: 4px; cursor: pointer;",
                        onclick: move |_| {
                            percent.set(0);
                            circle_percent.set(0);
                        },
                        "重置"
                    }
                }
            }

            // 自定义文字格式
            section {
                style: "margin-bottom: 40px;",
                h2 { "自定义文字格式" }
                p { "通过 format 属性指定格式。" }

                div {
                    style: "width: 400px;",

                    Progress {
                        percent: 75,
                        // format: Some(|percent| format!("{}天", percent))
                    }

                    Progress {
                        progress_type: ProgressType::Circle,
                        percent: 75,
                        // format: |percent| format!("{}°C", percent)
                    }
                }
            }

            // 进度条线宽
            section {
                style: "margin-bottom: 40px;",
                h2 { "进度条线宽" }
                p { "通过 strokeWidth 更改进度条线宽。" }

                div {
                    style: "width: 400px;",

                    Progress {
                        percent: 50,
                        stroke_width: 20
                    }

                    Progress {
                        percent: 50,
                        stroke_width: 2
                    }
                }

                div {
                    style: "display: flex; gap: 20px; align-items: center; margin-top: 20px;",

                    Progress {
                        progress_type: ProgressType::Circle,
                        percent: 50,
                        stroke_width: 20
                    }

                    Progress {
                        progress_type: ProgressType::Circle,
                        percent: 50,
                        stroke_width: 2
                    }
                }
            }

            // 自定义颜色
            section {
                style: "margin-bottom: 40px;",
                h2 { "自定义颜色" }
                p { "通过 strokeColor 设置单色进度条，trailColor 设置背景色。" }

                div {
                    style: "width: 400px;",

                    Progress {
                        percent: 50,
                        stroke_color: "#52c41a".to_string()
                    }

                    Progress {
                        percent: 50,
                        stroke_color: "#faad14".to_string(),
                        trail_color: "#f0f0f0".to_string()
                    }

                    Progress {
                        percent: 50,
                        stroke_color: "#ff4d4f".to_string()
                    }
                }

                div {
                    style: "display: flex; gap: 20px; align-items: center; margin-top: 20px;",

                    Progress {
                        progress_type: ProgressType::Circle,
                        percent: 90,
                        stroke_color: "#52c41a".to_string()
                    }

                    Progress {
                        progress_type: ProgressType::Dashboard,
                        percent: 90,
                        stroke_color: "#faad14".to_string()
                    }
                }
            }

            // 步骤进度条
            section {
                style: "margin-bottom: 40px;",
                h2 { "步骤进度条" }
                p { "带步骤的进度条。" }

                div {
                    style: "width: 400px;",

                    Progress {
                        percent: 50,
                        steps: 3,
                        show_steps: true
                    }

                    Progress {
                        percent: 30,
                        steps: 5,
                        show_steps: true,
                        stroke_color: "#52c41a".to_string()
                    }

                    Progress {
                        percent: 100,
                        steps: 5,
                        show_steps: true,
                        stroke_color: "#1677ff".to_string(),
                        size: ProgressSize::Small
                    }
                }
            }

            // 小型进度条
            section {
                style: "margin-bottom: 40px;",
                h2 { "小型进度条" }
                p { "适合放在较狭窄的区域内。" }

                div {
                    style: "width: 400px;",

                    Progress {
                        percent: 30,
                        size: ProgressSize::Small
                    }

                    Progress {
                        percent: 50,
                        size: ProgressSize::Small,
                        status: ProgressStatus::Active
                    }

                    Progress {
                        percent: 70,
                        size: ProgressSize::Small,
                        status: ProgressStatus::Exception
                    }

                    Progress {
                        percent: 100,
                        size: ProgressSize::Small,
                        status: ProgressStatus::Success
                    }
                }
            }

            // 动态进度条
            section {
                h2 { "动态进度条" }
                p { "自动循环的进度条效果。" }

                div {
                    style: "width: 400px;",

                    Progress {
                        percent: dynamic_percent(),
                        status: ProgressStatus::Active
                    }
                }

                div {
                    style: "display: flex; gap: 20px; align-items: center; margin-top: 20px;",

                    Progress {
                        progress_type: ProgressType::Circle,
                        percent: dynamic_percent(),
                        width: 100
                    }

                    Progress {
                        progress_type: ProgressType::Dashboard,
                        percent: dynamic_percent(),
                        width: 100
                    }
                }
            }
        }
    }
}

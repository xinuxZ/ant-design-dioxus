//!
//! 展示 Rate 组件的各种用法和样式

use crate::common::*;
use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

/// 心形字符渲染函数
fn heart_character_fn(_index: usize) -> Element {
    rsx! {
        span {
            style: "color: #eb2f96;",
            "♥"
        }
    }
}

/// 表情字符渲染函数
fn face_character_fn(index: usize) -> Element {
    let faces = ["😞", "😐", "🙂", "😊", "😍"];
    let face = faces.get(index).unwrap_or(&"😐");
    rsx! {
        span {
            style: "font-size: 18px;",
            "{face}"
        }
    }
}

/// Rate 组件演示
#[component]
pub fn RateDemo() -> Element {
    let mut basic_value = use_signal(|| 0.0);
    let mut half_value = use_signal(|| 2.5);
    let mut readonly_value = use_signal(|| 3.5);
    let mut clear_value = use_signal(|| 3.0);
    let mut controlled_value = use_signal(|| 2.0);
    let mut hover_value = use_signal(|| None);
    let mut event_log = use_signal(|| Vec::<String>::new());

    // 添加事件日志
    let mut add_log = move |message: String| {
        let mut logs = event_log();
        logs.insert(0, message);
        if logs.len() > 5 {
            logs.truncate(5);
        }
        event_log.set(logs);
    };

    // 自定义字符渲染函数
    let heart_character = Some(heart_character_fn as fn(usize) -> Element);
    let face_character = Some(face_character_fn as fn(usize) -> Element);

    // 提示信息
    let tooltips = vec![
        "terrible".to_string(),
        "bad".to_string(),
        "normal".to_string(),
        "good".to_string(),
        "wonderful".to_string(),
    ];

    let chinese_tooltips = vec![
        "极差".to_string(),
        "失望".to_string(),
        "一般".to_string(),
        "满意".to_string(),
        "惊喜".to_string(),
    ];

    rsx! {
        div {
            class: "component-showcase",

            h1 {
                style: "margin: 0 0 16px 0; color: #262626; font-size: 28px;",
                "Rate 评分"
            }

            p {
                style: "margin: 0 0 24px 0; color: #666; font-size: 14px;",
                "评分组件，用于对事物进行评级操作。"
            }

            // 基础用法
            DemoSection {
                title: "基础用法",
                description: "最简单的用法。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    Rate {
                        value: Some(basic_value()),
                        on_change: move |value| {
                            basic_value.set(value);
                            add_log(format!("基础评分: {:.1} 星", value));
                        }
                    }

                    div {
                        style: "color: #666; font-size: 12px;",
                        "当前评分: {basic_value():.1} 星"
                    }
                }
            }

            // 半星
            DemoSection {
                title: "半星",
                description: "支持选择半星。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    Rate {
                        value: Some(half_value()),
                        allow_half: true,
                        on_change: move |value| {
                            half_value.set(value);
                            add_log(format!("半星评分: {:.1} 星", value));
                        }
                    }

                    div {
                        style: "color: #666; font-size: 12px;",
                        "当前评分: {half_value():.1} 星（支持半星）"
                    }
                }
            }

            // 只读
            DemoSection {
                title: "只读",
                description: "只读，无法进行鼠标交互。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    Rate {
                        value: Some(readonly_value()),
                        readonly: true,
                        allow_half: true
                    }

                    div {
                        style: "color: #666; font-size: 12px;",
                        "只读模式: {readonly_value():.1} 星"
                    }
                }
            }

            // 清除
            DemoSection {
                title: "清除",
                description: "支持允许或者禁用清除。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    div {
                        "允许清除（点击当前星级可清除）:"
                        Rate {
                            value: Some(clear_value()),
                            allow_clear: true,
                            on_change: move |value| {
                                clear_value.set(value);
                                add_log(format!("可清除评分: {:.1} 星", value));
                            }
                        }
                    }

                    div {
                        "不允许清除:"
                        Rate {
                            default_value: 3.0,
                            allow_clear: false
                        }
                    }

                    div {
                        style: "color: #666; font-size: 12px;",
                        "当前评分: {clear_value():.1} 星"
                    }
                }
            }

            // 不同尺寸
            DemoSection {
                title: "不同尺寸",
                description: "三种尺寸的评分组件。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    div {
                        "小尺寸:"
                        Rate {
                            default_value: 2.0,
                            size: RateSize::Small
                        }
                    }

                    div {
                        "中等尺寸（默认）:"
                        Rate {
                            default_value: 3.0,
                            size: RateSize::Middle
                        }
                    }

                    div {
                        "大尺寸:"
                        Rate {
                            default_value: 4.0,
                            size: RateSize::Large
                        }
                    }
                }
            }

            // 禁用状态
            DemoSection {
                title: "禁用状态",
                description: "禁用状态下无法交互。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    Rate {
                        default_value: 2.0,
                        disabled: true
                    }

                    div {
                        style: "color: #666; font-size: 12px;",
                        "禁用状态，无法进行交互"
                    }
                }
            }

            // 其他字符
            DemoSection {
                title: "其他字符",
                description: "可以将星星替换为其他字符，比如字母，数字，汉字等等。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    div {
                        "字母:"
                        Rate {
                            default_value: 3.0,
                            character: Some(rsx! {
                                span { "A" }
                            })
                        }
                    }

                    div {
                        "数字:"
                        Rate {
                            default_value: 2.0,
                            character: Some(rsx! {
                                span {
                                    style: "color: #1890ff; font-weight: bold;",
                                    "1"
                                }
                            })
                        }
                    }

                    div {
                        "汉字:"
                        Rate {
                            default_value: 4.0,
                            character: Some(rsx! {
                                span {
                                    style: "color: #52c41a;",
                                    "好"
                                }
                            })
                        }
                    }

                    div {
                        "爱心:"
                        Rate {
                            default_value: 3.0,
                            character_render: heart_character
                        }
                    }

                    div {
                        "表情:"
                        Rate {
                            default_value: 3.0,
                            character_render: face_character
                        }
                    }
                }
            }

            // 半星自定义字符
            DemoSection {
                title: "半星自定义字符",
                description: "自定义字符的半星用法。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    Rate {
                        default_value: 2.5,
                        allow_half: true,
                        character: Some(rsx! {
                            span {
                                style: "color: #f5222d; font-size: 20px;",
                                "♥"
                            }
                        })
                    }

                    div {
                        style: "color: #666; font-size: 12px;",
                        "自定义爱心字符，支持半星选择"
                    }
                }
            }

            // 显示提示
            DemoSection {
                title: "显示提示",
                description: "给评分组件加上文案展示。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    div {
                        "英文提示:"
                        Rate {
                            default_value: 3.0,
                            tooltips: Some(tooltips.clone())
                        }
                    }

                    div {
                        "中文提示:"
                        Rate {
                            default_value: 4.0,
                            tooltips: Some(chinese_tooltips.clone())
                        }
                    }
                }
            }

            // 受控组件
            DemoSection {
                title: "受控组件",
                description: "受控组件。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    div {
                        style: "display: flex; gap: 8px; margin-bottom: 16px;",

                        Button {
                            onclick: move |_| controlled_value.set(0.0),
                            "清除"
                        }

                        Button {
                            onclick: move |_| controlled_value.set(1.0),
                            "1星"
                        }

                        Button {
                            onclick: move |_| controlled_value.set(2.5),
                            "2.5星"
                        }

                        Button {
                            onclick: move |_| controlled_value.set(5.0),
                            "5星"
                        }
                    }

                    Rate {
                        value: Some(controlled_value()),
                        allow_half: true,
                        on_change: move |value| {
                            controlled_value.set(value);
                            add_log(format!("受控评分: {:.1} 星", value));
                        }
                    }

                    div {
                        style: "color: #666; font-size: 12px;",
                        "当前评分: {controlled_value():.1} 星"
                    }
                }
            }

            // 事件回调
            DemoSection {
                title: "事件回调",
                description: "演示评分事件回调。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    Rate {
                        default_value: 0.0,
                        allow_half: true,
                        on_change: move |value| {
                            add_log(format!("评分变化: {:.1} 星", value));
                        },
                        on_hover_change: move |value| {
                            hover_value.set(Some(value));
                            add_log(format!("鼠标悬停: {:.1} 星", value));
                        }
                    }

                    if let Some(hover) = hover_value() {
                        div {
                            style: "color: #1890ff; font-size: 12px;",
                            "鼠标悬停: {hover:.1} 星"
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

                        Rate {
                            default_value: 3.0,
                            style: "color: #fadb14;",
                            class: "dark-rate"
                        }
                    }

                    div {
                        style: "background: linear-gradient(45deg, #ff6b6b, #4ecdc4); padding: 16px; border-radius: 6px;",

                        Rate {
                            default_value: 4.0,
                            style: "color: white; font-size: 24px;"
                        }
                    }

                    Rate {
                        default_value: 2.0,
                        style: "border: 2px solid #1890ff; border-radius: 8px; padding: 8px; background: #f0f8ff;"
                    }
                }
            }
        }
    }
}

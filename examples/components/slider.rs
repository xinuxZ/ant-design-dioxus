//!
//! 展示 Slider 组件的各种用法和样式

use crate::common::*;
use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;
use std::collections::HashMap;

/// Slider 组件演示
#[component]
pub fn SliderDemo() -> Element {
    let mut basic_value = use_signal(|| 30.0);
    let mut range_value = use_signal(|| (20.0, 50.0));
    let mut disabled_value = use_signal(|| 30.0);
    let mut step_value = use_signal(|| 0.0);
    let mut vertical_value = use_signal(|| 30.0);
    let mut vertical_range_value = use_signal(|| (20.0, 50.0));
    let mut controlled_value = use_signal(|| 50.0);
    let mut controlled_range_value = use_signal(|| (20.0, 80.0));
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

    // 创建标记
    let mut marks = HashMap::new();
    marks.insert("0".to_string(), SliderMark::new(0.0, "0°C"));
    marks.insert("26".to_string(), SliderMark::new(26.0, "26°C"));
    marks.insert("37".to_string(), SliderMark::new(37.0, "37°C"));
    marks.insert("100".to_string(), SliderMark::new(100.0, "100°C"));

    let mut icon_marks = HashMap::new();
    icon_marks.insert("0".to_string(), SliderMark::new(0.0, "😰"));
    icon_marks.insert("26".to_string(), SliderMark::new(26.0, "😐"));
    icon_marks.insert("37".to_string(), SliderMark::new(37.0, "🙂"));
    icon_marks.insert("100".to_string(), SliderMark::new(100.0, "😍"));

    // 自定义工具提示格式化函数
    let tooltip_formatter = |value: f64| -> String { format!("{:.1}%", value) };

    let temperature_formatter = |value: f64| -> String { format!("{:.0}°C", value) };

    rsx! {
        div {
            class: "component-showcase",

            h1 {
                style: "margin: 0 0 16px 0; color: #262626; font-size: 28px;",
                "Slider 滑动输入条"
            }

            p {
                style: "margin: 0 0 24px 0; color: #666; font-size: 14px;",
                "滑动型输入器，展示当前值和可选范围。"
            }

            // 基础用法
            DemoSection {
                title: "基础用法",
                description: "基本滑动条。当 range 为 true 时，渲染为双滑块。当 disabled 为 true 时，滑块处于不可用状态。",

                div {
                    style: "display: flex; flex-direction: column; gap: 24px; padding: 16px 0;",

                    div {
                        "基础滑块:"
                        div {
                            style: "margin: 16px 0;",
                            Slider {
                                value: Some(basic_value()),
                                on_change: move |value| {
                                    basic_value.set(value);
                                    add_log(format!("基础滑块: {:.1}", value));
                                }
                            }
                        }
                        div {
                            style: "color: #666; font-size: 12px;",
                            "当前值: {basic_value():.1}"
                        }
                    }

                    div {
                        "范围滑块:"
                        div {
                            style: "margin: 16px 0;",
                            Slider {
                                range: true,
                                range_value: Some(range_value()),
                                on_range_change: move |value| {
                                    range_value.set(value);
                                    add_log(format!("范围滑块: {:.1} - {:.1}", value.0, value.1));
                                }
                            }
                        }
                        div {
                            style: "color: #666; font-size: 12px;",
                            "当前范围: {range_value().0:.1} - {range_value().1:.1}"
                        }
                    }
                }
            }

            // 带输入框的滑块
            DemoSection {
                title: "带输入框的滑块",
                description: "和数字输入框组件保持同步。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    div {
                        style: "display: flex; align-items: center; gap: 16px;",

                        div {
                            style: "flex: 1;",
                            Slider {
                                min: 1.0,
                                max: 20.0,
                                value: Some(controlled_value()),
                                on_change: move |value| {
                                    controlled_value.set(value);
                                }
                            }
                        }

                        InputNumber {
                            min: Some(1.0),
                            max: Some(20.0),
                            style: "width: 100px;",
                            value: Some(controlled_value()),
                            on_change: move |value| {
                                if let Some(v) = value {
                                    controlled_value.set(v);
                                }
                            }
                        }
                    }

                    div {
                        style: "color: #666; font-size: 12px;",
                        "滑块和输入框保持同步"
                    }
                }
            }

            // 带刻度的滑块
            DemoSection {
                title: "带刻度的滑块",
                description: "使用 marks 属性标注分段式滑块，使用 value / defaultValue 指定滑块位置。当 included=false 时，表明不同标记间为并列关系。当 step=null 时，Slider 的可选值仅有 marks 标出来的部分。",

                div {
                    style: "display: flex; flex-direction: column; gap: 32px; padding: 16px 0;",

                    div {
                        "带标记的滑块:"
                        div {
                            style: "margin: 24px 0;",
                            Slider {
                                marks: marks.clone(),
                                default_value: Some(37.0),
                                tooltip_formatter: Some(temperature_formatter as fn(f64) -> String)
                            }
                        }
                    }

                    div {
                        "带图标标记的滑块:"
                        div {
                            style: "margin: 24px 0;",
                            Slider {
                                marks: icon_marks.clone(),
                                default_value: Some(26.0),
                                step: 26.0
                            }
                        }
                    }

                    div {
                        "包含关系:"
                        div {
                            style: "margin: 24px 0;",
                            Slider {
                                marks: marks.clone(),
                                included: false,
                                default_value: Some(37.0)
                            }
                        }
                    }
                }
            }

            // 带点的滑块
            DemoSection {
                title: "带点的滑块",
                description: "当 step 不为 0 时，可以设置 dots=true 显示步长刻度。",

                div {
                    style: "display: flex; flex-direction: column; gap: 24px; padding: 16px 0;",

                    div {
                        "显示步长点:"
                        div {
                            style: "margin: 16px 0;",
                            Slider {
                                dots: true,
                                step: 10.0,
                                default_value: Some(30.0)
                            }
                        }
                    }

                    div {
                        "范围滑块带点:"
                        div {
                            style: "margin: 16px 0;",
                            Slider {
                                range: true,
                                dots: true,
                                step: 10.0,
                                default_range_value: Some((20.0, 50.0))
                            }
                        }
                    }
                }
            }

            // 自定义步长
            DemoSection {
                title: "自定义步长",
                description: "用 step 属性指定步长，禁用状态下滑块不可操作。",

                div {
                    style: "display: flex; flex-direction: column; gap: 24px; padding: 16px 0;",

                    div {
                        "步长为 20:"
                        div {
                            style: "margin: 16px 0;",
                            Slider {
                                step: 20.0,
                                default_value: Some(20.0)
                            }
                        }
                    }

                    div {
                        "步长为 0.1:"
                        div {
                            style: "margin: 16px 0;",
                            Slider {
                                min: 0.0,
                                max: 1.0,
                                step: 0.1,
                                value: Some(step_value()),
                                on_change: move |value| {
                                    step_value.set(value);
                                }
                            }
                        }
                        div {
                            style: "color: #666; font-size: 12px;",
                            "当前值: {step_value():.1}"
                        }
                    }
                }
            }

            // 垂直
            DemoSection {
                title: "垂直",
                description: "垂直方向的 Slider。",

                div {
                    style: "display: flex; gap: 32px; align-items: flex-start; padding: 16px 0;",

                    div {
                        style: "display: flex; flex-direction: column; align-items: center; gap: 16px;",

                        div {
                            style: "height: 300px;",
                            Slider {
                                vertical: true,
                                value: Some(vertical_value()),
                                on_change: move |value| {
                                    vertical_value.set(value);
                                }
                            }
                        }

                        div {
                            style: "color: #666; font-size: 12px;",
                            "基础垂直滑块: {vertical_value():.0}"
                        }
                    }

                    div {
                        style: "display: flex; flex-direction: column; align-items: center; gap: 16px;",

                        div {
                            style: "height: 300px;",
                            Slider {
                                vertical: true,
                                range: true,
                                range_value: Some(vertical_range_value()),
                                on_range_change: move |value| {
                                    vertical_range_value.set(value);
                                }
                            }
                        }

                        div {
                            style: "color: #666; font-size: 12px;",
                            "范围垂直滑块: {vertical_range_value().0:.0}-{vertical_range_value().1:.0}"
                        }
                    }

                    div {
                        style: "display: flex; flex-direction: column; align-items: center; gap: 16px;",

                        div {
                            style: "height: 300px;",
                            Slider {
                                vertical: true,
                                marks: marks.clone(),
                                default_value: Some(26.0)
                            }
                        }

                        div {
                            style: "color: #666; font-size: 12px;",
                            "带标记垂直滑块"
                        }
                    }
                }
            }

            // 不同尺寸
            DemoSection {
                title: "不同尺寸",
                description: "三种尺寸的滑块。",

                div {
                    style: "display: flex; flex-direction: column; gap: 24px; padding: 16px 0;",

                    div {
                        "小尺寸:"
                        div {
                            style: "margin: 16px 0;",
                            Slider {
                                size: SliderSize::Small,
                                default_value: Some(30.0)
                            }
                        }
                    }

                    div {
                        "中等尺寸（默认）:"
                        div {
                            style: "margin: 16px 0;",
                            Slider {
                                size: SliderSize::Middle,
                                default_value: Some(50.0)
                            }
                        }
                    }

                    div {
                        "大尺寸:"
                        div {
                            style: "margin: 16px 0;",
                            Slider {
                                size: SliderSize::Large,
                                default_value: Some(70.0)
                            }
                        }
                    }
                }
            }

            // 禁用状态
            DemoSection {
                title: "禁用状态",
                description: "禁用状态下滑块不可操作。",

                div {
                    style: "display: flex; flex-direction: column; gap: 24px; padding: 16px 0;",

                    div {
                        "禁用基础滑块:"
                        div {
                            style: "margin: 16px 0;",
                            Slider {
                                disabled: true,
                                value: Some(disabled_value()),
                                on_change: move |value| {
                                    disabled_value.set(value);
                                }
                            }
                        }
                    }

                    div {
                        "禁用范围滑块:"
                        div {
                            style: "margin: 16px 0;",
                            Slider {
                                range: true,
                                disabled: true,
                                default_range_value: Some((20.0, 50.0))
                            }
                        }
                    }
                }
            }

            // 不同状态
            DemoSection {
                title: "不同状态",
                description: "不同状态的滑块。",

                div {
                    style: "display: flex; flex-direction: column; gap: 24px; padding: 16px 0;",

                    div {
                        "默认状态:"
                        div {
                            style: "margin: 16px 0;",
                            Slider {
                                status: SliderStatus::Default,
                                default_value: Some(30.0)
                            }
                        }
                    }

                    div {
                        "错误状态:"
                        div {
                            style: "margin: 16px 0;",
                            Slider {
                                status: SliderStatus::Error,
                                default_value: Some(50.0)
                            }
                        }
                    }

                    div {
                        "警告状态:"
                        div {
                            style: "margin: 16px 0;",
                            Slider {
                                status: SliderStatus::Warning,
                                default_value: Some(70.0)
                            }
                        }
                    }
                }
            }

            // 反向滑块
            DemoSection {
                title: "反向滑块",
                description: "设置 reverse 可以将滑块反向。",

                div {
                    style: "display: flex; flex-direction: column; gap: 24px; padding: 16px 0;",

                    div {
                        "反向基础滑块:"
                        div {
                            style: "margin: 16px 0;",
                            Slider {
                                reverse: true,
                                default_value: Some(30.0)
                            }
                        }
                    }

                    div {
                        "反向范围滑块:"
                        div {
                            style: "margin: 16px 0;",
                            Slider {
                                range: true,
                                reverse: true,
                                default_range_value: Some((20.0, 50.0))
                            }
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
                            onclick: move |_| controlled_range_value.set((0.0, 100.0)),
                            "全范围"
                        }

                        Button {
                            onclick: move |_| controlled_range_value.set((25.0, 75.0)),
                            "中间范围"
                        }

                        Button {
                            onclick: move |_| controlled_range_value.set((40.0, 60.0)),
                            "小范围"
                        }

                        Button {
                            onclick: move |_| controlled_range_value.set((50.0, 50.0)),
                            "重置"
                        }
                    }

                    Slider {
                        range: true,
                        range_value: Some(controlled_range_value()),
                        on_range_change: move |value| {
                            controlled_range_value.set(value);
                            add_log(format!("受控范围滑块: {:.1} - {:.1}", value.0, value.1));
                        }
                    }

                    div {
                        style: "color: #666; font-size: 12px;",
                        "当前范围: {controlled_range_value().0:.1} - {controlled_range_value().1:.1}"
                    }
                }
            }

            // 事件回调
            DemoSection {
                title: "事件回调",
                description: "演示滑块事件回调。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    Slider {
                        default_value: Some(30.0),
                        tooltip_formatter: Some(tooltip_formatter as fn(f64) -> String),
                        on_change: move |value| {
                            add_log(format!("值变化: {:.1}", value));
                        },
                        on_after_change: move |value| {
                            add_log(format!("拖拽结束: {:.1}", value));
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
                    style: "display: flex; flex-direction: column; gap: 24px; padding: 16px 0;",

                    div {
                        style: "background: #001529; padding: 16px; border-radius: 6px;",

                        Slider {
                            default_value: Some(30.0),
                            style: "color: #1890ff;",
                            class: "dark-slider"
                        }
                    }

                    div {
                        style: "background: linear-gradient(45deg, #ff6b6b, #4ecdc4); padding: 16px; border-radius: 6px;",

                        Slider {
                            default_value: Some(50.0),
                            style: "color: white;"
                        }
                    }

                    Slider {
                        default_value: Some(70.0),
                        style: "border: 2px solid #1890ff; border-radius: 8px; padding: 16px; background: #f0f8ff;"
                    }
                }
            }
        }
    }
}

//!
//! 展示 InputNumber 组件的各种用法和样式

use crate::common::*;
use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

/// InputNumber 组件演示
#[component]
pub fn InputNumberDemo() -> Element {
    let mut basic_value = use_signal(|| Some(3.0));
    let mut size_value = use_signal(|| Some(10.0));
    let mut precision_value = use_signal(|| Some(3.1415));
    let mut range_value = use_signal(|| Some(50.0));
    let mut step_value = use_signal(|| Some(0.0));
    let mut formatter_value = use_signal(|| Some(1000.0));
    let mut disabled_value = use_signal(|| Some(100.0));
    let mut status_value = use_signal(|| Some(0.0));
    let mut borderless_value = use_signal(|| Some(20.0));
    let mut controls_value = use_signal(|| Some(5.0));
    let mut prefix_suffix_value = use_signal(|| Some(25.0));
    let mut keyboard_value = use_signal(|| Some(1.0));
    let mut readonly_value = use_signal(|| Some(99.0));
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

    rsx! {
        div {
            class: "component-showcase",

            h1 {
                style: "margin: 0 0 16px 0; color: #262626; font-size: 28px;",
                "InputNumber 数字输入框"
            }

            p {
                style: "margin: 0 0 24px 0; color: #666; font-size: 14px;",
                "通过鼠标或键盘，输入范围内的数值。"
            }

            // 基础用法
            DemoSection {
                title: "基础用法",
                description: "数字输入框。",

                div {
                    style: "display: flex; gap: 16px; align-items: center;",

                    InputNumber {
                        value: basic_value(),
                        placeholder: "请输入数字",
                        on_change: move |value| {
                            basic_value.set(value);
                            add_log(format!("基础输入框值变化: {:?}", value));
                        }
                    }

                    span {
                        style: "color: #666;",
                        "当前值: {basic_value():?}"
                    }
                }
            }

            // 三种大小
            DemoSection {
                title: "三种大小",
                description: "三种大小的数字输入框，当 size 分别为 large、default、small 时。",

                div {
                    style: "display: flex; gap: 16px; align-items: center;",

                    InputNumber {
                        size: InputNumberSize::Large,
                        value: size_value(),
                        placeholder: "大号",
                        on_change: move |value| size_value.set(value)
                    }

                    InputNumber {
                        size: InputNumberSize::Middle,
                        value: size_value(),
                        placeholder: "默认",
                        on_change: move |value| size_value.set(value)
                    }

                    InputNumber {
                        size: InputNumberSize::Small,
                        value: size_value(),
                        placeholder: "小号",
                        on_change: move |value| size_value.set(value)
                    }
                }
            }

            // 小数
            DemoSection {
                title: "小数",
                description: "和原生的数字输入框一样，value 的精度由 step 的小数位数决定。",

                div {
                    style: "display: flex; gap: 16px; align-items: center; flex-wrap: wrap;",

                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",

                        label { "step=1" }
                        InputNumber {
                            value: precision_value(),
                            step: 1.0,
                            on_change: move |value| precision_value.set(value)
                        }
                    }

                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",

                        label { "step=0.1" }
                        InputNumber {
                            value: precision_value(),
                            step: 0.1,
                            on_change: move |value| precision_value.set(value)
                        }
                    }

                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",

                        label { "step=0.01" }
                        InputNumber {
                            value: precision_value(),
                            step: 0.01,
                            on_change: move |value| precision_value.set(value)
                        }
                    }

                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",

                        label { "precision=2" }
                        InputNumber {
                            value: precision_value(),
                            precision: Some(2),
                            on_change: move |value| precision_value.set(value)
                        }
                    }
                }
            }

            // 受控
            DemoSection {
                title: "受控",
                description: "通过 value 和 onChange 结合使用，可以实现一个受控的数字输入框。",

                div {
                    style: "display: flex; gap: 16px; align-items: center;",

                    InputNumber {
                        value: range_value(),
                        min: Some(1.0),
                        max: Some(100.0),
                        on_change: move |value| {
                            range_value.set(value);
                            add_log(format!("受控输入框值变化: {:?}", value));
                        }
                    }

                    Button {
                        style: "margin-left: 8px;",
                        onclick: move |_| range_value.set(Some(50.0)),
                        "重置为50"
                    }

                    span {
                        style: "color: #666; margin-left: 8px;",
                        "范围: 1-100, 当前值: {range_value():?}"
                    }
                }
            }

            // 步数
            DemoSection {
                title: "步数",
                description: "通过 step 属性指定每次改变的步数。",

                div {
                    style: "display: flex; gap: 16px; align-items: center; flex-wrap: wrap;",

                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",

                        label { "step=1" }
                        InputNumber {
                            value: step_value(),
                            step: 1.0,
                            on_change: move |value| step_value.set(value)
                        }
                    }

                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",

                        label { "step=5" }
                        InputNumber {
                            value: step_value(),
                            step: 5.0,
                            on_change: move |value| step_value.set(value)
                        }
                    }

                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",

                        label { "step=10" }
                        InputNumber {
                            value: step_value(),
                            step: 10.0,
                            on_change: move |value| step_value.set(value)
                        }
                    }

                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",

                        label { "step=0.5" }
                        InputNumber {
                            value: step_value(),
                            step: 0.5,
                            on_change: move |value| step_value.set(value)
                        }
                    }
                }
            }

            // 格式化展示
            DemoSection {
                title: "格式化展示",
                description: "通过 formatter 格式化数字，以展示具有具体含义的数据，往往需要配合 parser 一起使用。",

                div {
                    style: "display: flex; gap: 16px; align-items: center; flex-wrap: wrap;",

                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",

                        label { "货币格式" }
                        InputNumber {
                            value: formatter_value(),
                            prefix: Some("$".to_string()),
                            step: 100.0,
                            on_change: move |value| formatter_value.set(value)
                        }
                    }

                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",

                        label { "百分比格式" }
                        InputNumber {
                            value: Some(formatter_value().unwrap_or(0.0) / 100.0),
                            suffix: Some("%".to_string()),
                            step: 0.01,
                            min: Some(0.0),
                            max: Some(1.0),
                            on_change: move |value| {
                                if let Some(v) = value {
                                    formatter_value.set(Some(v * 100.0));
                                }
                            }
                        }
                    }

                    span {
                        style: "color: #666; margin-left: 8px;",
                        "原始值: {formatter_value():?}"
                    }
                }
            }

            // 不可用
            DemoSection {
                title: "不可用",
                description: "点击按钮切换可用状态。",

                div {
                    style: "display: flex; gap: 16px; align-items: center;",

                    InputNumber {
                        value: disabled_value(),
                        disabled: true,
                        on_change: move |value| disabled_value.set(value)
                    }

                    InputNumber {
                        value: disabled_value(),
                        disabled: false,
                        on_change: move |value| disabled_value.set(value)
                    }
                }
            }

            // 状态
            DemoSection {
                title: "状态",
                description: "使用 status 为 InputNumber 添加状态，可选 error 或者 warning。",

                div {
                    style: "display: flex; gap: 16px; align-items: center; flex-wrap: wrap;",

                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",

                        label { "默认状态" }
                        InputNumber {
                            value: status_value(),
                            status: InputNumberStatus::Default,
                            placeholder: "默认状态",
                            on_change: move |value| status_value.set(value)
                        }
                    }

                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",

                        label { "错误状态" }
                        InputNumber {
                            value: status_value(),
                            status: InputNumberStatus::Error,
                            placeholder: "错误状态",
                            on_change: move |value| status_value.set(value)
                        }
                    }

                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",

                        label { "警告状态" }
                        InputNumber {
                            value: status_value(),
                            status: InputNumberStatus::Warning,
                            placeholder: "警告状态",
                            on_change: move |value| status_value.set(value)
                        }
                    }
                }
            }

            // 无边框
            DemoSection {
                title: "无边框",
                description: "没有边框。",

                div {
                    style: "display: flex; gap: 16px; align-items: center;",

                    InputNumber {
                        value: borderless_value(),
                        bordered: false,
                        placeholder: "无边框",
                        on_change: move |value| borderless_value.set(value)
                    }

                    InputNumber {
                        value: borderless_value(),
                        bordered: true,
                        placeholder: "有边框",
                        on_change: move |value| borderless_value.set(value)
                    }
                }
            }

            // 按钮模式
            DemoSection {
                title: "按钮模式",
                description: "通过 controls 属性控制是否显示增减按钮。",

                div {
                    style: "display: flex; gap: 16px; align-items: center;",

                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",

                        label { "显示按钮" }
                        InputNumber {
                            value: controls_value(),
                            controls: true,
                            on_change: move |value| controls_value.set(value)
                        }
                    }

                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",

                        label { "隐藏按钮" }
                        InputNumber {
                            value: controls_value(),
                            controls: false,
                            on_change: move |value| controls_value.set(value)
                        }
                    }
                }
            }

            // 前缀和后缀
            DemoSection {
                title: "前缀和后缀",
                description: "在数字输入框前后添加前缀和后缀。",

                div {
                    style: "display: flex; gap: 16px; align-items: center; flex-wrap: wrap;",

                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",

                        label { "前缀" }
                        InputNumber {
                            value: prefix_suffix_value(),
                            prefix: Some("¥".to_string()),
                            on_change: move |value| prefix_suffix_value.set(value)
                        }
                    }

                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",

                        label { "后缀" }
                        InputNumber {
                            value: prefix_suffix_value(),
                            suffix: Some("元".to_string()),
                            on_change: move |value| prefix_suffix_value.set(value)
                        }
                    }

                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",

                        label { "前缀+后缀" }
                        InputNumber {
                            value: prefix_suffix_value(),
                            prefix: Some("$".to_string()),
                            suffix: Some("USD".to_string()),
                            on_change: move |value| prefix_suffix_value.set(value)
                        }
                    }
                }
            }

            // 键盘行为
            DemoSection {
                title: "键盘行为",
                description: "通过 keyboard 属性控制是否启用键盘快速操作。",

                div {
                    style: "display: flex; gap: 16px; align-items: center;",

                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",

                        label { "启用键盘操作" }
                        InputNumber {
                            value: keyboard_value(),
                            keyboard: true,
                            placeholder: "支持上下键",
                            on_change: move |value| keyboard_value.set(value)
                        }
                    }

                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",

                        label { "禁用键盘操作" }
                        InputNumber {
                            value: keyboard_value(),
                            keyboard: false,
                            placeholder: "不支持上下键",
                            on_change: move |value| keyboard_value.set(value)
                        }
                    }
                }
            }

            // 只读
            DemoSection {
                title: "只读",
                description: "只读状态。",

                div {
                    style: "display: flex; gap: 16px; align-items: center;",

                    InputNumber {
                        value: readonly_value(),
                        readonly: true,
                        on_change: move |value| readonly_value.set(value)
                    }

                    span {
                        style: "color: #666;",
                        "只读模式，值: {readonly_value():?}"
                    }
                }
            }

            // 事件回调
            DemoSection {
                title: "事件回调",
                description: "演示各种事件回调。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    InputNumber {
                        placeholder: "输入数字测试事件",
                        on_change: move |value| {
                            add_log(format!("onChange: {:?}", value));
                        },
                        on_focus: move |_| {
                            add_log("onFocus: 获得焦点".to_string());
                        },
                        on_blur: move |_| {
                            add_log("onBlur: 失去焦点".to_string());
                        },
                        on_press_enter: move |_| {
                            add_log("onPressEnter: 按下回车".to_string());
                        },
                        on_step: move |(value, info): (f64, StepInfo)| {
                            add_log(format!("onStep: 值={}, 偏移={}, 类型={:?}", value, info.offset, info.r#type));
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
                    style: "display: flex; gap: 16px; align-items: center; flex-wrap: wrap;",

                    InputNumber {
                        value: Some(42.0),
                        style: "width: 200px; border: 2px solid #1890ff; border-radius: 8px;",
                        placeholder: "自定义边框"
                    }

                    InputNumber {
                        value: Some(88.0),
                        style: "width: 150px; background: linear-gradient(45deg, #ff6b6b, #4ecdc4); color: white; border: none;",
                        placeholder: "渐变背景"
                    }

                    InputNumber {
                        value: Some(100.0),
                        style: "width: 120px; border-radius: 20px; text-align: center; font-weight: bold;",
                        placeholder: "圆角样式"
                    }
                }
            }
        }
    }
}

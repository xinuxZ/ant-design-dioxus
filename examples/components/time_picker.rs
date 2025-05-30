//! TimePicker 时间选择器演示
//!
//! 展示 TimePicker 组件的各种用法和功能

use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

/// TimePicker 演示应用
#[component]
pub fn TimePickerDemo() -> Element {
    // 基础时间选择器状态
    let mut basic_time = use_signal(|| None::<TimeValue>);

    // 不同尺寸的时间选择器状态
    let mut large_time = use_signal(|| None::<TimeValue>);
    let mut small_time = use_signal(|| None::<TimeValue>);

    // 禁用状态控制
    let mut is_disabled = use_signal(|| false);
    let mut disabled_time = use_signal(|| Some(TimeValue::new(14, 30, 0)));

    // 12小时制时间选择器
    let mut time_12h = use_signal(|| None::<TimeValue>);

    // 自定义格式时间选择器
    let mut custom_format_time = use_signal(|| None::<TimeValue>);

    // 自定义步长时间选择器
    let mut step_time = use_signal(|| None::<TimeValue>);

    rsx! {
        div {
            // style: "padding: 24px; max-width: 1200px; margin: 0 auto;",

            h1 { "TimePicker 时间选择器演示" }

            // 基础用法
            section {
                style: "margin-bottom: 32px;",
                h2 { "基础用法" }
                p { "最简单的用法，点击选择时间。" }

                div {
                    style: "margin-bottom: 16px;",
                    TimePicker {
                        value: basic_time(),
                        placeholder: "选择时间",
                        onchange: move |value| {
                            basic_time.set(value);
                        },
                    }
                }

                if let Some(time) = basic_time() {
                    p {
                        style: "color: #666; font-size: 14px;",
                        "选择的时间: {time.to_string(&TimeFormat::default())}"
                    }
                }
            }

            // 三种尺寸
            section {
                style: "margin-bottom: 32px;",
                h2 { "三种尺寸" }
                p { "三种尺寸的时间选择器，当 size 分别为 large、default 和 small 时。" }

                div {
                    style: "display: flex; gap: 16px; align-items: center; margin-bottom: 16px;",

                    div {
                        TimePicker {
                            size: TimePickerSize::Large,
                            placeholder: "大尺寸",
                            value: large_time(),
                            onchange: move |value| {
                                large_time.set(value);
                            },
                        }
                    }

                    div {
                        TimePicker {
                            size: TimePickerSize::Middle,
                            placeholder: "默认尺寸",
                            value: basic_time(),
                            onchange: move |value| {
                                basic_time.set(value);
                            },
                        }
                    }

                    div {
                        TimePicker {
                            size: TimePickerSize::Small,
                            placeholder: "小尺寸",
                            value: small_time(),
                            onchange: move |value| {
                                small_time.set(value);
                            },
                        }
                    }
                }
            }

            // 禁用状态
            section {
                style: "margin-bottom: 32px;",
                h2 { "禁用状态" }
                p { "选择器不可用状态。" }

                div {
                    style: "margin-bottom: 16px;",
                    button {
                        style: "margin-bottom: 16px; padding: 4px 15px; border: 1px solid #d9d9d9; border-radius: 6px; background: #fff; cursor: pointer;",
                        onclick: move |_| is_disabled.set(!is_disabled()),
                        if is_disabled() { "启用" } else { "禁用" }
                    }
                }

                div {
                    style: "margin-bottom: 16px;",
                    TimePicker {
                        disabled: is_disabled(),
                        value: disabled_time(),
                        placeholder: "禁用状态",
                        onchange: move |value| {
                            disabled_time.set(value);
                        },
                    }
                }
            }

            // 12小时制
            section {
                style: "margin-bottom: 32px;",
                h2 { "12小时制" }
                p { "12小时制的时间选择器，默认的 format 为 h:mm:ss a。" }

                div {
                    style: "margin-bottom: 16px;",
                    TimePicker {
                        format: TimeFormat {
                            format: "h:mm:ss a".to_string(),
                            show_hour: true,
                            show_minute: true,
                            show_second: true,
                            use_12_hours: true,
                        },
                        value: time_12h(),
                        placeholder: "选择时间",
                        onchange: move |value| {
                            time_12h.set(value);
                        },
                    }
                }

                if let Some(time) = time_12h() {
                    p {
                        style: "color: #666; font-size: 14px;",
                        "选择的时间: {time.to_string(&TimeFormat { format: \"h:mm:ss a\".to_string(), show_hour: true, show_minute: true, show_second: true, use_12_hours: true })}"
                    }
                }
            }

            // 自定义时间格式
            section {
                style: "margin-bottom: 32px;",
                h2 { "自定义时间格式" }
                p { "展示的时间格式。" }

                div {
                    style: "display: flex; gap: 16px; margin-bottom: 16px;",

                    div {
                        TimePicker {
                            format: TimeFormat {
                                format: "HH:mm".to_string(),
                                show_hour: true,
                                show_minute: true,
                                show_second: false,
                                use_12_hours: false,
                            },
                            placeholder: "HH:mm",
                            value: custom_format_time(),
                            onchange: move |value| {
                                custom_format_time.set(value);
                            },
                        }
                    }

                    div {
                        TimePicker {
                            format: TimeFormat {
                                format: "HH".to_string(),
                                show_hour: true,
                                show_minute: false,
                                show_second: false,
                                use_12_hours: false,
                            },
                            placeholder: "HH",
                            value: custom_format_time(),
                            onchange: move |value| {
                                custom_format_time.set(value);
                            },
                        }
                    }

                    div {
                        TimePicker {
                            format: TimeFormat {
                                format: "mm:ss".to_string(),
                                show_hour: false,
                                show_minute: true,
                                show_second: true,
                                use_12_hours: false,
                            },
                            placeholder: "mm:ss",
                            value: custom_format_time(),
                            onchange: move |value| {
                                custom_format_time.set(value);
                            },
                        }
                    }
                }
            }

            // 步长选项
            section {
                style: "margin-bottom: 32px;",
                h2 { "步长选项" }
                p { "可以使用 hourStep minuteStep secondStep 按步长展示可选的时分秒。" }

                div {
                    style: "margin-bottom: 16px;",
                    TimePicker {
                        hour_step: 2,
                        minute_step: 15,
                        second_step: 10,
                        value: step_time(),
                        placeholder: "选择时间",
                        onchange: move |value| {
                            step_time.set(value);
                        },
                    }
                }

                p {
                    style: "color: #666; font-size: 14px;",
                    "小时步长: 2, 分钟步长: 15, 秒步长: 10"
                }

                if let Some(time) = step_time() {
                    p {
                        style: "color: #666; font-size: 14px;",
                        "选择的时间: {time.to_string(&TimeFormat::default())}"
                    }
                }
            }

            // 不允许清除
            section {
                style: "margin-bottom: 32px;",
                h2 { "不允许清除" }
                p { "设置 allowClear 为 false 后，不展示清除按钮。" }

                div {
                    style: "margin-bottom: 16px;",
                    TimePicker {
                        allow_clear: false,
                        default_value: Some(TimeValue::new(12, 0, 0)),
                        placeholder: "选择时间",
                    }
                }
            }

            // 自动获取焦点
            section {
                style: "margin-bottom: 32px;",
                h2 { "自动获取焦点" }
                p { "自动获取焦点。" }

                div {
                    style: "margin-bottom: 16px;",
                    TimePicker {
                        auto_focus: true,
                        placeholder: "自动获取焦点",
                    }
                }
            }
        }
    }
}

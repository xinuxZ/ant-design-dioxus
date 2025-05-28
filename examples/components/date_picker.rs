//!
//! 展示 DatePicker 组件的各种用法和样式

use crate::common::*;
use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

/// DatePicker 组件演示
#[component]
pub fn DatePickerDemo() -> Element {
    let mut selected_date = use_signal(|| None::<String>);
    let mut range_start = use_signal(|| None::<String>);
    let mut range_end = use_signal(|| None::<String>);
    let mut open_state = use_signal(|| false);
    let mut focus_state = use_signal(|| false);

    rsx! {
        div {
            class: "component-showcase",

            h1 {
                style: "margin: 0 0 16px 0; color: #262626; font-size: 28px;",
                "DatePicker 日期选择框"
            }

            p {
                style: "margin: 0 0 24px 0; color: #666; font-size: 14px;",
                "输入或选择日期的控件。当用户需要输入一个日期，可以点击标准输入框，弹出日期面板进行选择。"
            }

            // 基础用法
            DemoSection {
                title: "基础用法",
                description: "最简单的用法，在浮层中可以选择或者输入日期。",

                div {
                    style: "display: flex; gap: 16px; align-items: center;",

                    DatePicker {
                        placeholder: "请选择日期",
                        on_change: move |date| {
                            selected_date.set(date);
                        }
                    }

                    // div {
                    //     style: "color: #666; font-size: 14px;",
                    //     "选中日期: {selected_date().unwrap_or_else(|| "未选择".to_string())}"
                    // }
                }
            }

            // 三种大小
            DemoSection {
                title: "三种大小",
                description: "三种大小的输入框，大的用在表单中，中的为默认。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    div {
                        style: "display: flex; gap: 16px; align-items: center;",
                        "大尺寸:"
                        DatePicker {
                            size: DatePickerSize::Large,
                            placeholder: "请选择日期"
                        }
                    }

                    div {
                        style: "display: flex; gap: 16px; align-items: center;",
                        "默认尺寸:"
                        DatePicker {
                            size: DatePickerSize::Middle,
                            placeholder: "请选择日期"
                        }
                    }

                    div {
                        style: "display: flex; gap: 16px; align-items: center;",
                        "小尺寸:"
                        DatePicker {
                            size: DatePickerSize::Small,
                            placeholder: "请选择日期"
                        }
                    }
                }
            }

            // 日期格式
            DemoSection {
                title: "日期格式",
                description: "使用 format 属性，可以自定义日期显示格式。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    div {
                        style: "display: flex; gap: 16px; align-items: center;",
                        "YYYY-MM-DD:"
                        DatePicker {
                            format: "YYYY-MM-DD",
                            placeholder: "请选择日期"
                        }
                    }

                    div {
                        style: "display: flex; gap: 16px; align-items: center;",
                        "YYYY/MM/DD:"
                        DatePicker {
                            format: "YYYY/MM/DD",
                            placeholder: "请选择日期"
                        }
                    }

                    div {
                        style: "display: flex; gap: 16px; align-items: center;",
                        "DD-MM-YYYY:"
                        DatePicker {
                            format: "DD-MM-YYYY",
                            placeholder: "请选择日期"
                        }
                    }
                }
            }

            // 禁用
            DemoSection {
                title: "禁用",
                description: "选择框的不可用状态。",

                div {
                    style: "display: flex; gap: 16px;",

                    DatePicker {
                        disabled: true,
                        placeholder: "禁用状态"
                    }

                    DatePicker {
                        disabled: true,
                        default_value: "2023-12-25",
                        placeholder: "禁用状态"
                    }
                }
            }

            // 不同状态
            DemoSection {
                title: "状态",
                description: "使用 status 为 DatePicker 添加状态。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    div {
                        style: "display: flex; gap: 16px; align-items: center;",
                        "错误状态:"
                        DatePicker {
                            status: DatePickerStatus::Error,
                            placeholder: "请选择日期"
                        }
                    }

                    div {
                        style: "display: flex; gap: 16px; align-items: center;",
                        "警告状态:"
                        DatePicker {
                            status: DatePickerStatus::Warning,
                            placeholder: "请选择日期"
                        }
                    }
                }
            }

            // 无边框
            DemoSection {
                title: "无边框",
                description: "无边框样式。",

                div {
                    style: "display: flex; gap: 16px;",

                    DatePicker {
                        bordered: false,
                        placeholder: "无边框"
                    }

                    DatePicker {
                        bordered: false,
                        size: DatePickerSize::Small,
                        placeholder: "无边框小尺寸"
                    }
                }
            }

            // 不同选择器
            DemoSection {
                title: "选择器类型",
                description: "通过设置 mode 可以实现不同类型的选择器。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    div {
                        style: "display: flex; gap: 16px; align-items: center;",
                        "日期选择:"
                        DatePicker {
                            mode: DatePickerMode::Date,
                            placeholder: "请选择日期"
                        }
                    }

                    div {
                        style: "display: flex; gap: 16px; align-items: center;",
                        "周选择:"
                        DatePicker {
                            mode: DatePickerMode::Week,
                            placeholder: "请选择周"
                        }
                    }

                    div {
                        style: "display: flex; gap: 16px; align-items: center;",
                        "月份选择:"
                        DatePicker {
                            mode: DatePickerMode::Month,
                            placeholder: "请选择月份"
                        }
                    }

                    div {
                        style: "display: flex; gap: 16px; align-items: center;",
                        "季度选择:"
                        DatePicker {
                            mode: DatePickerMode::Quarter,
                            placeholder: "请选择季度"
                        }
                    }

                    div {
                        style: "display: flex; gap: 16px; align-items: center;",
                        "年份选择:"
                        DatePicker {
                            mode: DatePickerMode::Year,
                            placeholder: "请选择年份"
                        }
                    }
                }
            }

            // 时间选择
            DemoSection {
                title: "时间选择",
                description: "增加选择时间功能。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    div {
                        style: "display: flex; gap: 16px; align-items: center;",
                        "时间选择:"
                        DatePicker {
                            mode: DatePickerMode::Time,
                            placeholder: "请选择时间"
                        }
                    }

                    div {
                        style: "display: flex; gap: 16px; align-items: center;",
                        "日期时间:"
                        DatePicker {
                            mode: DatePickerMode::DateTime,
                            placeholder: "请选择日期时间"
                        }
                    }
                }
            }

            // 默认值
            DemoSection {
                title: "默认值",
                description: "设置默认值。",

                div {
                    style: "display: flex; gap: 16px;",

                    DatePicker {
                        default_value: "2023-12-25",
                        placeholder: "请选择日期"
                    }

                    DatePicker {
                        mode: DatePickerMode::Month,
                        default_value: "2023-12",
                        placeholder: "请选择月份"
                    }
                }
            }

            // 受控模式
            DemoSection {
                title: "受控模式",
                description: "受控组件，配合 value 和 on_change 使用。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    div {
                        style: "display: flex; gap: 16px; align-items: center;",
                        DatePicker {
                            value: selected_date().clone(),
                            placeholder: "受控模式",
                            on_change: move |date| {
                                selected_date.set(date);
                            }
                        }

                        Button {
                            button_type: ButtonType::Primary,
                            size: ButtonSize::Small,
                            onclick: move |_| {
                                selected_date.set(Some("2023-12-25".to_string()));
                            },
                            "设置为 2023-12-25"
                        }

                        Button {
                            size: ButtonSize::Small,
                            onclick: move |_| {
                                selected_date.set(None);
                            },
                            "清空"
                        }
                    }

                    div {
                        style: "color: #666; font-size: 14px;",
                        "当前值: ", {selected_date().unwrap_or_else(|| "未选择".to_string())}
                    }
                }
            }

            // 事件回调
            DemoSection {
                title: "事件回调",
                description: "监听日期选择器的各种事件。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    DatePicker {
                        placeholder: "请选择日期",
                        on_change: move |date| {
                            web_sys::console::log_1(&format!("日期变化: {:?}", date).into());
                        },
                        on_open_change: move |open| {
                            open_state.set(open);
                            web_sys::console::log_1(&format!("面板状态: {}", open).into());
                        },
                        on_focus: move |_| {
                            focus_state.set(true);
                            web_sys::console::log_1(&"获得焦点".into());
                        },
                        on_blur: move |_| {
                            focus_state.set(false);
                            web_sys::console::log_1(&"失去焦点".into());
                        },
                        on_ok: move |date| {
                            web_sys::console::log_1(&format!("确认选择: {}", date).into());
                        }
                    }

                    // div {
                    //     style: "color: #666; font-size: 14px;",
                    //     "面板打开状态: {open_state} | 焦点状态: {focus_state}"
                    // }
                }
            }

            // 自定义样式
            DemoSection {
                title: "自定义样式",
                description: "自定义日期选择器的样式。",

                div {
                    style: "display: flex; gap: 16px;",

                    DatePicker {
                        placeholder: "自定义样式",
                        class: "custom-date-picker",
                        style: "border: 2px solid #52c41a; border-radius: 8px; background: #f6ffed;"
                    }

                    DatePicker {
                        placeholder: "圆角样式",
                        style: "border-radius: 20px; border-color: #1890ff;"
                    }
                }
            }

            // 不允许清除
            DemoSection {
                title: "不允许清除",
                description: "设置 allow_clear 为 false，不显示清除按钮。",

                div {
                    style: "display: flex; gap: 16px;",

                    DatePicker {
                        allow_clear: false,
                        default_value: "2023-12-25",
                        placeholder: "不允许清除"
                    }
                }
            }
        }
    }
}

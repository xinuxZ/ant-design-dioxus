#![allow(non_snake_case)]
//!
//! 展示 Statistic 组件的各种用法和样式

use crate::common::*;
use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

/// Statistic 组件演示
#[component]
pub fn StatisticDemo() -> Element {
    let mut like = use_signal(|| 156);
    let mut dislikes = use_signal(|| 156);

    rsx! {
        div {
            class: "component-showcase",

            h1 {
                style: "margin: 0 0 16px 0; color: #262626; font-size: 28px;",
                "Statistic 统计数值"
            }

            p {
                style: "margin: 0 0 24px 0; color: #666; font-size: 14px;",
                "展示统计数值。"
            }

            // 基础用法
            DemoSection {
                title: "基础用法",
                description: "简单的展示。",

                div {
                    style: "display: flex; gap: 32px; flex-wrap: wrap;",

                    Statistic {
                        title: "Active Users",
                        value: 112893.0
                    }

                    Statistic {
                        title: "Account Balance (CNY)",
                        value: 112893.0,
                        precision: StatisticPrecision { decimal_places: 2 }
                    }

                    Statistic {
                        title: "Active Users",
                        value: 112893.0,
                        suffix: "/",
                        value_style: Some("color: #3f8600")
                    }

                    Statistic {
                        title: "Idle Users",
                        value: 93.0,
                        suffix: "%",
                        value_style: Some("color: #cf1322")
                    }
                }
            }

            // 单位
            DemoSection {
                title: "单位",
                description: "通过前缀和后缀来标识不同的单位。",

                div {
                    style: "display: flex; gap: 32px; flex-wrap: wrap;",

                    Statistic {
                        title: "Feedback",
                        value: 1128.0,
                        prefix: rsx! {
                            Icon {
                                icon_type: "like".to_string(),
                                theme: IconTheme::Outlined,
                                aria_label: "Like".to_string(),
                                style: "color: #1890ff",
                            }
                        }
                    }

                    Statistic {
                        title: "Unmerged",
                        value: 93.0,
                        suffix: Some("/100")
                    }
                }
            }

            // 在卡片中使用
            DemoSection {
                title: "在卡片中使用",
                description: "在卡片中展示统计数值。",

                div {
                    style: "display: flex; gap: 16px; flex-wrap: wrap;",

                    Card {
                        style: "width: 300px;",
                        Statistic {
                            title: "Active Users",
                            value: 112893.0,
                            value_style: Some("color: #3f8600"),
                            prefix: rsx! {
                                Icon {
                                    icon_type: "arrow-up-outlined".to_string()
                                }
                            },
                            suffix: "%"
                        }
                    }

                    Card {
                        style: "width: 300px;",
                        Statistic {
                            title: "Idle Users",
                            value: 9.3,
                            precision: StatisticPrecision { decimal_places: 2 },
                            value_style: Some("color: #cf1322"),
                            prefix: rsx! {
                                Icon {
                                    icon_type: "arrow-down-outlined".to_string()
                                }
                            },
                            suffix: "%"
                        }
                    }
                }
            }

            // 倒计时
            DemoSection {
                title: "倒计时",
                description: "倒计时组件。",

                div {
                    style: "display: flex; gap: 32px; flex-wrap: wrap;",

                    Statistic::Countdown {
                        title: "Countdown",
                        value: js_sys::Date::now() + 1000.0 * 60.0 * 60.0 * 24.0 * 2.0 + 1000.0 * 30.0,
                        on_finish: move || {
                            web_sys::console::log_1(&"finished!".into());
                        }
                    }

                    Statistic::Countdown {
                        title: "Million Seconds",
                        value: js_sys::Date::now() + 1000.0 * 60.0 * 60.0 * 24.0 * 2.0 + 1000.0 * 30.0,
                        format: "HH:mm:ss:SSS"
                    }

                    Statistic::Countdown {
                        title: "Day Level",
                        value: js_sys::Date::now() + 1000.0 * 60.0 * 60.0 * 24.0 * 2.0 + 1000.0 * 30.0,
                        format: "D 天 H 时 m 分 s 秒"
                    }
                }
            }

            // 控制倒计时
            DemoSection {
                title: "控制倒计时",
                description: "通过 value 手动控制倒计时展示。",

                div {
                    style: "display: flex; gap: 32px; flex-wrap: wrap; align-items: center;",

                    Statistic::Countdown {
                        title: "Countdown",
                        value: js_sys::Date::now() + 10.0 * 1000.0,
                        on_finish: move || {
                            web_sys::console::log_1(&"finished!".into());
                        }
                    }

                    Button {
                        button_type: ButtonType::Primary,
                        onclick: move |_| {
                            // 重新设置倒计时
                        },
                        "Start"
                    }
                }
            }

            // 数值动画
            DemoSection {
                title: "数值动画",
                description: "数值动画。",

                div {
                    style: "display: flex; gap: 32px; flex-wrap: wrap; align-items: center;",

                    div {
                        style: "display: flex; gap: 16px; align-items: center;",
                        Button {
                            button_type: ButtonType::Primary,
                            onclick: move |_| {
                                like.set(like() + 1);
                            },
                            Icon {
                                icon_type: "like".to_string(),
                                theme: IconTheme::Outlined,
                                style: "color: #1890ff",
                                aria_label: "Like".to_string(),
                            }
                        }
                        span {
                            style: "margin: 0 16px;",
                            {like()}
                        }
                        Button {
                            onclick: move |_| {
                                dislikes.set(dislikes() + 1);
                            },
                            Icon {
                                icon_type: "dislike".to_string(),
                                theme: IconTheme::Outlined,
                                aria_label: "Dislike".to_string(),
                                style: "color: #eb2f96",
                            }
                        }
                        span {
                            style: "margin: 0 16px;",
                            {dislikes()}
                        }
                    }
                }
            }

            // API 文档
            ApiDocumentation {
                component_name: "Statistic",
                props: vec![
                    PropDoc {
                        name: "decimal_separator".to_string(),
                        prop_type: "String".to_string(),
                        default: ".".to_string(),
                        description: "设置小数点".to_string(),
                    },
                    PropDoc {
                        name: "formatter".to_string(),
                        prop_type: "Function".to_string(),
                        default: "-".to_string(),
                        description: "自定义数值展示".to_string(),
                    },
                    PropDoc {
                        name: "group_separator".to_string(),
                        prop_type: "String".to_string(),
                        default: ",".to_string(),
                        description: "设置千分位标识符".to_string(),
                    },
                    PropDoc {
                        name: "loading".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "数值是否加载中".to_string(),
                    },
                    PropDoc {
                        name: "precision".to_string(),
                        prop_type: "i32".to_string(),
                        default: "-".to_string(),
                        description: "数值精度".to_string(),
                    },
                    PropDoc {
                        name: "prefix".to_string(),
                        prop_type: "String | Element".to_string(),
                        default: "-".to_string(),
                        description: "设置数值的前缀".to_string(),
                    },
                    PropDoc {
                        name: "suffix".to_string(),
                        prop_type: "String | Element".to_string(),
                        default: "-".to_string(),
                        description: "设置数值的后缀".to_string(),
                    },
                    PropDoc {
                        name: "title".to_string(),
                        prop_type: "String | Element".to_string(),
                        default: "-".to_string(),
                        description: "数值的标题".to_string(),
                    },
                    PropDoc {
                        name: "value".to_string(),
                        prop_type: "String | f64".to_string(),
                        default: "-".to_string(),
                        description: "数值内容".to_string(),
                    },
                    PropDoc {
                        name: "value_style".to_string(),
                        prop_type: "String".to_string(),
                        default: "-".to_string(),
                        description: "设置数值的样式".to_string(),
                    },
                ]
            }

            // Statistic.Countdown API
            ApiDocumentation {
                component_name: "Statistic.Countdown",
                props: vec![
                    PropDoc {
                        name: "format".to_string(),
                        prop_type: "String".to_string(),
                        default: "HH:mm:ss".to_string(),
                        description: "格式化倒计时展示，参考 moment".to_string(),
                    },
                    PropDoc {
                        name: "prefix".to_string(),
                        prop_type: "String | Element".to_string(),
                        default: "-".to_string(),
                        description: "设置数值的前缀".to_string(),
                    },
                    PropDoc {
                        name: "suffix".to_string(),
                        prop_type: "String | Element".to_string(),
                        default: "-".to_string(),
                        description: "设置数值的后缀".to_string(),
                    },
                    PropDoc {
                        name: "title".to_string(),
                        prop_type: "String | Element".to_string(),
                        default: "-".to_string(),
                        description: "数值的标题".to_string(),
                    },
                    PropDoc {
                        name: "value".to_string(),
                        prop_type: "f64".to_string(),
                        default: "-".to_string(),
                        description: "数值内容".to_string(),
                    },
                    PropDoc {
                        name: "value_style".to_string(),
                        prop_type: "String".to_string(),
                        default: "-".to_string(),
                        description: "设置数值的样式".to_string(),
                    },
                    PropDoc {
                        name: "on_finish".to_string(),
                        prop_type: "Function".to_string(),
                        default: "-".to_string(),
                        description: "倒计时完成时触发".to_string(),
                    },
                    PropDoc {
                        name: "on_change".to_string(),
                        prop_type: "Function".to_string(),
                        default: "-".to_string(),
                        description: "倒计时时间变化时触发".to_string(),
                    },
                ]
            }
        }
    }
}

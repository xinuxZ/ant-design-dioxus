//! ThemeSwitch 组件演示
//!
//! 展示 ThemeSwitch 组件的各种用法和样式

use crate::common::*;
use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

/// ThemeSwitch 组件演示
#[component]
pub fn ThemeSwitchDemo() -> Element {
    // 当前主题状态
    let theme_context = use_context::<Signal<ant_design_dioxus::theme::ThemeConfig>>();

    rsx! {
        div {
            class: "component-showcase",

            h1 {
                style: "margin: 0 0 16px 0; color: var(--ant-heading-color, #262626); font-size: 28px;",
                "ThemeSwitch 主题切换"
            }

            p {
                style: "margin: 0 0 24px 0; color: var(--ant-text-color-secondary, #666); font-size: 14px;",
                "主题切换组件，用于切换应用的主题模式。"
            }

            // 基础用法
            DemoSection {
                title: "基础用法",
                description: "ThemeSwitch 组件有三种模式：按钮模式、开关模式和下拉选择模式。",

                div {
                    style: "display: flex; gap: 24px; flex-wrap: wrap;",

                    // 按钮模式
                    div {
                        style: "display: flex; flex-direction: column; align-items: center; gap: 8px;",

                        span { "按钮模式" }

                        ThemeSwitch {
                            mode: ThemeSwitchMode::Button,
                            on_change: move |theme| {
                                web_sys::console::log_1(&format!("Theme changed to: {:?}", theme).into());
                            }
                        }
                    }

                    // 开关模式
                    div {
                        style: "display: flex; flex-direction: column; align-items: center; gap: 8px;",

                        span { "开关模式" }

                        ThemeSwitch {
                            mode: ThemeSwitchMode::Switch,
                            on_change: move |theme| {
                                web_sys::console::log_1(&format!("Theme changed to: {:?}", theme).into());
                            }
                        }
                    }

                    // 下拉选择模式
                    div {
                        style: "display: flex; flex-direction: column; align-items: center; gap: 8px;",

                        span { "下拉选择模式" }

                        ThemeSwitch {
                            mode: ThemeSwitchMode::Select,
                            on_change: move |theme| {
                                web_sys::console::log_1(&format!("Theme changed to: {:?}", theme).into());
                            }
                        }
                    }
                }
            }

            // 尺寸变体
            DemoSection {
                title: "尺寸变体",
                description: "ThemeSwitch 组件支持三种尺寸：小、中、大。",

                div {
                    style: "display: flex; gap: 24px; flex-wrap: wrap;",

                    // 小尺寸
                    div {
                        style: "display: flex; flex-direction: column; align-items: center; gap: 8px;",

                        span { "小尺寸" }

                        ThemeSwitch {
                            mode: ThemeSwitchMode::Switch,
                            size: "small".to_string(),
                        }
                    }

                    // 中尺寸（默认）
                    div {
                        style: "display: flex; flex-direction: column; align-items: center; gap: 8px;",

                        span { "中尺寸（默认）" }

                        ThemeSwitch {
                            mode: ThemeSwitchMode::Switch,
                            size: "default".to_string(),
                        }
                    }

                    // 大尺寸
                    div {
                        style: "display: flex; flex-direction: column; align-items: center; gap: 8px;",

                        span { "大尺寸" }

                        ThemeSwitch {
                            mode: ThemeSwitchMode::Switch,
                            size: "large".to_string(),
                        }
                    }
                }
            }

            // 标签显示
            DemoSection {
                title: "标签显示",
                description: "可以控制是否显示标签文本。",

                div {
                    style: "display: flex; gap: 24px; flex-wrap: wrap;",

                    // 显示标签
                    div {
                        style: "display: flex; flex-direction: column; align-items: center; gap: 8px;",

                        span { "显示标签" }

                        ThemeSwitch {
                            mode: ThemeSwitchMode::Switch,
                            show_label: true,
                        }
                    }

                    // 不显示标签
                    div {
                        style: "display: flex; flex-direction: column; align-items: center; gap: 8px;",

                        span { "不显示标签" }

                        ThemeSwitch {
                            mode: ThemeSwitchMode::Switch,
                            show_label: false,
                        }
                    }
                }
            }

            // 当前主题信息
            DemoSection {
                title: "当前主题信息",
                description: "显示当前应用的主题信息。",

                div {
                    style: "padding: 16px; border: 1px solid var(--ant-border-color, #d9d9d9); border-radius: 8px;",

                    p {
                        "当前主题: "
                        strong {
                            match theme_context.read().theme {
                                ant_design_dioxus::theme::Theme::Dark => "暗色主题",
                                _ => "亮色主题",
                            }
                        }
                    }

                    // 显示一些主题变量
                    div {
                        style: "margin-top: 16px;",

                        h4 { "主题变量示例:" }

                        ul {
                            li { "主色: " span { style: "display: inline-block; width: 16px; height: 16px; background-color: var(--ant-primary-color); border-radius: 4px; margin-right: 8px;" } "var(--ant-primary-color)" }
                            li { "背景色: " span { style: "display: inline-block; width: 16px; height: 16px; background-color: var(--ant-bg-color); border-radius: 4px; margin-right: 8px;" } "var(--ant-bg-color)" }
                            li { "文本色: " span { style: "display: inline-block; width: 16px; height: 16px; background-color: var(--ant-text-color); border-radius: 4px; margin-right: 8px;" } "var(--ant-text-color)" }
                            li { "边框色: " span { style: "display: inline-block; width: 16px; height: 16px; background-color: var(--ant-border-color); border-radius: 4px; margin-right: 8px;" } "var(--ant-border-color)" }
                        }
                    }
                }
            }
        }
    }
}

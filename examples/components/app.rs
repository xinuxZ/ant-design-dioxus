use crate::common::demo_section::DemoSection;
use ant_design_dioxus::components::app::App;
use dioxus::prelude::*;

/// App组件示例
#[component]
pub fn AppDemo() -> Element {
    rsx! {
        div { class: "demo-container",
            h1 { "App 应用包裹器" }
            p { "App 组件只在 ConfigProvider 之下才生效，用于为应用提供统一的全局化配置。" }

            DemoSection {
                title: "基本使用",
                description: "最简单的用法。",
                code: r#"rsx! {
    App {
        div {
            h2 { "Hello World" }
            p { "这是一个基本的 App 组件示例" }
        }
    }
}"#,
                demo: rsx! {
                    App {
                        class: "demo-app",
                        style: "padding: 24px; border: 1px solid #d9d9d9; border-radius: 8px;",
                        div {
                            h2 { "Hello World" }
                            p { "这是一个基本的 App 组件示例" }
                        }
                    }
                }
            }

            DemoSection {
                title: "带样式的应用",
                description: "可以通过 class 和 style 属性自定义样式。",
                code: r#"rsx! {
    App {
        class: "custom-app",
        style: "background: #f5f5f5; padding: 32px;",
        div {
            h3 { "自定义样式的应用" }
            p { "通过 class 和 style 属性可以自定义应用的外观" }
        }
    }
}"#,
                demo: rsx! {
                    App {
                        class: "custom-app",
                        style: "background: #f5f5f5; padding: 32px; border-radius: 8px;",
                        div {
                            h3 { "自定义样式的应用" }
                            p { "通过 class 和 style 属性可以自定义应用的外观" }
                        }
                    }
                }
            }

            DemoSection {
                title: "应用布局",
                description: "App 组件可以作为应用的根容器，提供基础的布局结构。",
                code: r#"rsx! {
    App {
        style: "min-height: 200px; display: flex; flex-direction: column;",
        header {
            style: "background: #1677ff; color: white; padding: 16px; text-align: center;",
            "应用头部"
        }
        main {
            style: "flex: 1; padding: 24px; background: #fff;",
            "主要内容区域"
        }
        footer {
            style: "background: #f0f0f0; padding: 16px; text-align: center;",
            "应用底部"
        }
    }
}"#,
                demo: rsx! {
                    App {
                        style: "min-height: 200px; display: flex; flex-direction: column; border: 1px solid #d9d9d9; border-radius: 8px; overflow: hidden;",
                        header {
                            style: "background: #1677ff; color: white; padding: 16px; text-align: center;",
                            "应用头部"
                        }
                        main {
                            style: "flex: 1; padding: 24px; background: #fff;",
                            "主要内容区域"
                        }
                        footer {
                            style: "background: #f0f0f0; padding: 16px; text-align: center;",
                            "应用底部"
                        }
                    }
                }
            }
        }
    }
}

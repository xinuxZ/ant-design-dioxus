use crate::common::demo_section::DemoSection;
use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

/// FloatButton组件示例
#[component]
pub fn FloatButtonDemo() -> Element {
    rsx! {
        div { class: "demo-container",
            h1 { "FloatButton 悬浮按钮" }
            p { "悬浮按钮，用于页面中最主要的功能。" }

            DemoSection {
                title: "基本使用",
                description: "最简单的用法。",
                code: r#"rsx! {
    FloatButton {
        icon: "➕"
    }
}"#,
                demo: rsx! {
                    div { style: "position: relative; height: 200px; background: #fafafa; border: 1px solid #d9d9d9; border-radius: 8px;",
                        FloatButton {
                            class: "demo-float-button",
                            style: "position: absolute; bottom: 24px; right: 24px; width: 40px; height: 40px; border-radius: 50%; background: #1677ff; color: white; border: none; cursor: pointer; display: flex; align-items: center; justify-content: center; box-shadow: 0 2px 8px rgba(0,0,0,0.15);",
                            icon: "➕"
                        }
                    }
                }
            }

            DemoSection {
                title: "不同类型",
                description: "悬浮按钮有两种类型：primary（默认）和 default。",
                code: r#"rsx! {
    div { style: "display: flex; gap: 16px;",
        FloatButton { r#type: "primary", icon: "🏠" }
        FloatButton { r#type: "default", icon: "⚙️" }
    }
}"#,
                demo: rsx! {
                    div { style: "position: relative; height: 200px; background: #fafafa; border: 1px solid #d9d9d9; border-radius: 8px;",
                        FloatButton {
                            class: "float-button-primary",
                            style: "position: absolute; bottom: 24px; right: 80px; width: 40px; height: 40px; border-radius: 50%; background: #1677ff; color: white; border: none; cursor: pointer; display: flex; align-items: center; justify-content: center; box-shadow: 0 2px 8px rgba(0,0,0,0.15);",
                            icon: "🏠"
                        }
                        FloatButton {
                            class: "float-button-default",
                            style: "position: absolute; bottom: 24px; right: 24px; width: 40px; height: 40px; border-radius: 50%; background: white; color: #666; border: 1px solid #d9d9d9; cursor: pointer; display: flex; align-items: center; justify-content: center; box-shadow: 0 2px 8px rgba(0,0,0,0.15);",
                            icon: "⚙️"
                        }
                    }
                }
            }

            DemoSection {
                title: "不同形状",
                description: "支持圆形和方形两种形状。",
                code: r#"rsx! {
    div { style: "display: flex; gap: 16px;",
        FloatButton { shape: "circle", icon: "🔍" }
        FloatButton { shape: "square", icon: "📝" }
    }
}"#,
                demo: rsx! {
                    div { style: "position: relative; height: 200px; background: #fafafa; border: 1px solid #d9d9d9; border-radius: 8px;",
                        FloatButton {
                            class: "float-button-circle",
                            style: "position: absolute; bottom: 24px; right: 80px; width: 40px; height: 40px; border-radius: 50%; background: #1677ff; color: white; border: none; cursor: pointer; display: flex; align-items: center; justify-content: center; box-shadow: 0 2px 8px rgba(0,0,0,0.15);",
                            icon: "🔍"
                        }
                        FloatButton {
                            class: "float-button-square",
                            style: "position: absolute; bottom: 24px; right: 24px; width: 40px; height: 40px; border-radius: 6px; background: #1677ff; color: white; border: none; cursor: pointer; display: flex; align-items: center; justify-content: center; box-shadow: 0 2px 8px rgba(0,0,0,0.15);",
                            icon: "📝"
                        }
                    }
                }
            }

            DemoSection {
                title: "不同尺寸",
                description: "悬浮按钮有大、中、小三种尺寸。",
                code: r#"rsx! {
    div { style: "display: flex; gap: 16px; align-items: center;",
        FloatButton { size: "small", icon: "📧" }
        FloatButton { size: "default", icon: "📞" }
        FloatButton { size: "large", icon: "💬" }
    }
}"#,
                demo: rsx! {
                    div { style: "position: relative; height: 200px; background: #fafafa; border: 1px solid #d9d9d9; border-radius: 8px;",
                        FloatButton {
                            class: "float-button-small",
                            style: "position: absolute; bottom: 24px; right: 120px; width: 32px; height: 32px; border-radius: 50%; background: #1677ff; color: white; border: none; cursor: pointer; display: flex; align-items: center; justify-content: center; box-shadow: 0 2px 8px rgba(0,0,0,0.15); font-size: 12px;",
                            icon: "📧"
                        }
                        FloatButton {
                            class: "float-button-default",
                            style: "position: absolute; bottom: 24px; right: 72px; width: 40px; height: 40px; border-radius: 50%; background: #1677ff; color: white; border: none; cursor: pointer; display: flex; align-items: center; justify-content: center; box-shadow: 0 2px 8px rgba(0,0,0,0.15);",
                            icon: "📞"
                        }
                        FloatButton {
                            class: "float-button-large",
                            style: "position: absolute; bottom: 24px; right: 24px; width: 48px; height: 48px; border-radius: 50%; background: #1677ff; color: white; border: none; cursor: pointer; display: flex; align-items: center; justify-content: center; box-shadow: 0 2px 8px rgba(0,0,0,0.15); font-size: 18px;",
                            icon: "💬"
                        }
                    }
                }
            }

            DemoSection {
                title: "带描述的悬浮按钮",
                description: "可以添加描述文字的悬浮按钮。",
                code: r#"rsx! {
    FloatButton {
        icon: "❓",
        description: "帮助"
    }
}"#,
                demo: rsx! {
                    div { style: "position: relative; height: 200px; background: #fafafa; border: 1px solid #d9d9d9; border-radius: 8px;",
                        FloatButton {
                            class: "float-button-with-description",
                            style: "position: absolute; bottom: 24px; right: 24px; height: 40px; padding: 0 16px; border-radius: 20px; background: #1677ff; color: white; border: none; cursor: pointer; display: flex; align-items: center; gap: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.15);",
                            span { "❓" }
                            span { "帮助" }
                        }
                    }
                }
            }

            DemoSection {
                title: "回到顶部",
                description: "返回页面顶部的悬浮按钮。",
                code: r#"rsx! {
    FloatButton {
        class: "back-top",
        icon: "⬆️"
    }
}"#,
                demo: rsx! {
                    div { style: "position: relative; height: 200px; background: #fafafa; border: 1px solid #d9d9d9; border-radius: 8px;",
                        FloatButton {
                            class: "back-top",
                            style: "position: absolute; bottom: 24px; right: 24px; width: 40px; height: 40px; border-radius: 6px; background: rgba(0,0,0,0.65); color: white; border: none; cursor: pointer; display: flex; align-items: center; justify-content: center; box-shadow: 0 2px 8px rgba(0,0,0,0.15);",
                            icon: "⬆️"
                        }
                    }
                }
            }
        }
    }
}

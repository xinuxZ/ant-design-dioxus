//! Button 组件示例
//!
//! 展示 Button 组件的各种用法和样式

use ant_design_dioxus::{Button, ButtonShape, ButtonSize, ButtonType};
use dioxus::prelude::*;

// 引用依赖以避免未使用警告
#[allow(unused_imports)]
use once_cell as _;
#[allow(unused_imports)]
use serde as _;
#[allow(unused_imports)]
use serde_json as _;
#[allow(unused_imports)]
use wasm_bindgen_test as _;
#[allow(unused_imports)]
use web_sys as _;

/// 主应用组件
fn App() -> Element {
    rsx! {
        div {
            style: "padding: 20px; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;",

            // 页面标题
            h1 { "Ant Design Dioxus - Button 组件示例" }

            // 基础按钮
            section {
                style: "margin-bottom: 32px;",
                h2 { "基础按钮" }
                div {
                    style: "display: flex; gap: 8px; flex-wrap: wrap;",

                    Button {
                        button_type: ButtonType::Primary,
                        onclick: move |_| {
                            web_sys::console::log_1(&"Primary button clicked!".into());
                        },
                        "Primary"
                    }

                    Button {
                        button_type: ButtonType::Default,
                        onclick: move |_| {
                            web_sys::console::log_1(&"Default button clicked!".into());
                        },
                        "Default"
                    }

                    Button {
                        button_type: ButtonType::Dashed,
                        onclick: move |_| {
                            web_sys::console::log_1(&"Dashed button clicked!".into());
                        },
                        "Dashed"
                    }

                    Button {
                        button_type: ButtonType::Text,
                        onclick: move |_| {
                            web_sys::console::log_1(&"Text button clicked!".into());
                        },
                        "Text"
                    }

                    Button {
                        button_type: ButtonType::Link,
                        onclick: move |_| {
                            web_sys::console::log_1(&"Link button clicked!".into());
                        },
                        "Link"
                    }
                }
            }

            // 按钮尺寸
            section {
                style: "margin-bottom: 32px;",
                h2 { "按钮尺寸" }
                div {
                    style: "display: flex; gap: 8px; align-items: center; flex-wrap: wrap;",

                    Button {
                        button_type: ButtonType::Primary,
                        size: ButtonSize::Large,
                        "Large"
                    }

                    Button {
                        button_type: ButtonType::Primary,
                        size: ButtonSize::Middle,
                        "Middle"
                    }

                    Button {
                        button_type: ButtonType::Primary,
                        size: ButtonSize::Small,
                        "Small"
                    }
                }
            }

            // 按钮形状
            section {
                style: "margin-bottom: 32px;",
                h2 { "按钮形状" }
                div {
                    style: "display: flex; gap: 8px; align-items: center; flex-wrap: wrap;",

                    Button {
                        button_type: ButtonType::Primary,
                        shape: ButtonShape::Default,
                        "Default"
                    }

                    Button {
                        button_type: ButtonType::Primary,
                        shape: ButtonShape::Round,
                        "Round"
                    }

                    Button {
                        button_type: ButtonType::Primary,
                        shape: ButtonShape::Circle,
                        "A"
                    }
                }
            }

            // 危险按钮
            section {
                style: "margin-bottom: 32px;",
                h2 { "危险按钮" }
                div {
                    style: "display: flex; gap: 8px; flex-wrap: wrap;",

                    Button {
                        button_type: ButtonType::Primary,
                        danger: true,
                        "Primary Danger"
                    }

                    Button {
                        button_type: ButtonType::Default,
                        danger: true,
                        "Default Danger"
                    }

                    Button {
                        button_type: ButtonType::Dashed,
                        danger: true,
                        "Dashed Danger"
                    }

                    Button {
                        button_type: ButtonType::Text,
                        danger: true,
                        "Text Danger"
                    }

                    Button {
                        button_type: ButtonType::Link,
                        danger: true,
                        "Link Danger"
                    }
                }
            }

            // 禁用状态
            section {
                style: "margin-bottom: 32px;",
                h2 { "禁用状态" }
                div {
                    style: "display: flex; gap: 8px; flex-wrap: wrap;",

                    Button {
                        button_type: ButtonType::Primary,
                        disabled: true,
                        "Primary Disabled"
                    }

                    Button {
                        button_type: ButtonType::Default,
                        disabled: true,
                        "Default Disabled"
                    }

                    Button {
                        button_type: ButtonType::Dashed,
                        disabled: true,
                        "Dashed Disabled"
                    }

                    Button {
                        button_type: ButtonType::Text,
                        disabled: true,
                        "Text Disabled"
                    }

                    Button {
                        button_type: ButtonType::Link,
                        disabled: true,
                        "Link Disabled"
                    }
                }
            }

            // 加载状态
            section {
                style: "margin-bottom: 32px;",
                h2 { "加载状态" }
                div {
                    style: "display: flex; gap: 8px; flex-wrap: wrap;",

                    Button {
                        button_type: ButtonType::Primary,
                        loading: true,
                        "Loading"
                    }

                    Button {
                        button_type: ButtonType::Default,
                        loading: true,
                        "Loading"
                    }
                }
            }

            // 块级按钮
            section {
                style: "margin-bottom: 32px;",
                h2 { "块级按钮" }
                div {
                    style: "width: 300px;",

                    Button {
                        button_type: ButtonType::Primary,
                        block: true,
                        "Block Button"
                    }
                }
            }

            // 幽灵按钮
            section {
                style: "margin-bottom: 32px; padding: 20px; background: #1677ff; border-radius: 8px;",
                h2 {
                    style: "color: white;",
                    "幽灵按钮"
                }
                div {
                    style: "display: flex; gap: 8px; flex-wrap: wrap;",

                    Button {
                        button_type: ButtonType::Primary,
                        ghost: true,
                        "Primary Ghost"
                    }

                    Button {
                        button_type: ButtonType::Default,
                        ghost: true,
                        "Default Ghost"
                    }

                    Button {
                        button_type: ButtonType::Dashed,
                        ghost: true,
                        "Dashed Ghost"
                    }
                }
            }
        }
    }
}

fn main() {
    // 启动应用
    dioxus::launch(App);
}

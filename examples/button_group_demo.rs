//! 按钮组演示示例
//!
//! 展示 Ant Design Dioxus 按钮组组件的各种用法

// 声明未使用的外部crate
use once_cell as _;
use serde as _;
use serde_json as _;
use wasm_bindgen_test as _;
use web_sys as _;

use ant_design_dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

/// 主应用组件
fn app() -> Element {
    rsx! {
        div {
            style: "padding: 24px; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;",

            h1 { "按钮组演示" }

            // 基础按钮组
            section {
                style: "margin-bottom: 32px;",
                h2 { "基础按钮组" }
                p { "最简单的按钮组。" }

                ButtonGroup {
                    Button { "Left" }
                    Button { "Middle" }
                    Button { "Right" }
                }
            }

            // 不同尺寸的按钮组
            section {
                style: "margin-bottom: 32px;",
                h2 { "不同尺寸" }
                p { "按钮组有大、中、小三种尺寸。" }

                div { style: "margin-bottom: 16px;",
                    span { style: "margin-right: 16px;", "Large:" }
                    ButtonGroup {
                        size: ButtonGroupSize::Large,
                        Button { "Left" }
                        Button { "Middle" }
                        Button { "Right" }
                    }
                }

                div { style: "margin-bottom: 16px;",
                    span { style: "margin-right: 16px;", "Middle:" }
                    ButtonGroup {
                        size: ButtonGroupSize::Middle,
                        Button { "Left" }
                        Button { "Middle" }
                        Button { "Right" }
                    }
                }

                div {
                    span { style: "margin-right: 16px;", "Small:" }
                    ButtonGroup {
                        size: ButtonGroupSize::Small,
                        Button { "Left" }
                        Button { "Middle" }
                        Button { "Right" }
                    }
                }
            }

            // 主按钮组
            section {
                style: "margin-bottom: 32px;",
                h2 { "主按钮组" }
                p { "可以将按钮组设置为主按钮样式。" }

                ButtonGroup {
                    Button { button_type: ButtonType::Primary, "Left" }
                    Button { button_type: ButtonType::Primary, "Middle" }
                    Button { button_type: ButtonType::Primary, "Right" }
                }
            }

            // 带图标的按钮组
            section {
                style: "margin-bottom: 32px;",
                h2 { "带图标的按钮组" }
                p { "按钮组中的按钮可以包含图标。" }

                ButtonGroup {
                    Button {
                        Icon { icon_type: "left".to_string() }
                        "Go back"
                    }
                    Button {
                        "Go forward"
                        Icon { icon_type: "right".to_string() }
                    }
                }
            }

            // 禁用状态
            section {
                style: "margin-bottom: 32px;",
                h2 { "禁用状态" }
                p { "按钮组可以设置禁用状态。" }

                ButtonGroup {
                    disabled: true,
                    Button { "Left" }
                    Button { "Middle" }
                    Button { "Right" }
                }
            }

            // 混合按钮组
            section {
                style: "margin-bottom: 32px;",
                h2 { "混合按钮组" }
                p { "按钮组中可以包含不同类型的按钮。" }

                ButtonGroup {
                    Button { button_type: ButtonType::Primary, "Primary" }
                    Button { "Default" }
                    Button { button_type: ButtonType::Dashed, "Dashed" }
                    Button { button_type: ButtonType::Text, "Text" }
                }
            }

            // 垂直按钮组（通过自定义样式实现）
            section {
                style: "margin-bottom: 32px;",
                h2 { "垂直按钮组" }
                p { "通过自定义样式可以实现垂直排列的按钮组。" }

                ButtonGroup {
                    style: "flex-direction: column; display: inline-flex;",
                    Button { "Top" }
                    Button { "Middle" }
                    Button { "Bottom" }
                }
            }
        }
    }
}

//! Icon 图标组件示例
//!
//! 展示 Icon 组件的各种用法，包括不同类型、主题、尺寸的图标。

use ant_design_dioxus::prelude::*;
use dioxus as _;
use once_cell as _;
use serde as _;
use serde_json as _;
use wasm_bindgen_test as _;
use web_sys as _;

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        div {
            style: "padding: 24px; max-width: 800px; margin: 0 auto;",

            h1 { "Icon 图标组件示例" }

            // 基础图标示例
            div {
                style: "margin-bottom: 32px;",
                h2 { "基础图标" }

                div {
                    style: "display: flex; gap: 16px; align-items: center; margin-bottom: 16px;",

                    Icon {
                        icon_type: "home",
                    }

                    Icon {
                        icon_type: "user",
                    }

                    Icon {
                        icon_type: "setting",
                    }

                    Icon {
                        icon_type: "search",
                    }

                    Icon {
                        icon_type: "heart",
                    }

                    Icon {
                        icon_type: "star",
                    }
                }
            }

            // 不同主题示例
            div {
                style: "margin-bottom: 32px;",
                h2 { "不同主题" }

                div {
                    style: "margin-bottom: 16px;",
                    h3 { "Outlined (线框风格)" }
                    div {
                        style: "display: flex; gap: 16px; align-items: center;",

                        Icon {
                            icon_type: "home",
                            theme: IconTheme::Outlined,
                        }

                        Icon {
                            icon_type: "user",
                            theme: IconTheme::Outlined,
                        }

                        Icon {
                            icon_type: "setting",
                            theme: IconTheme::Outlined,
                        }
                    }
                }

                div {
                    style: "margin-bottom: 16px;",
                    h3 { "Filled (实心风格)" }
                    div {
                        style: "display: flex; gap: 16px; align-items: center;",

                        Icon {
                            icon_type: "home",
                            theme: IconTheme::Filled,
                        }

                        Icon {
                            icon_type: "user",
                            theme: IconTheme::Filled,
                        }

                        Icon {
                            icon_type: "setting",
                            theme: IconTheme::Filled,
                        }
                    }
                }

                div {
                    style: "margin-bottom: 16px;",
                    h3 { "TwoTone (双色风格)" }
                    div {
                        style: "display: flex; gap: 16px; align-items: center;",

                        Icon {
                            icon_type: "home",
                            theme: IconTheme::TwoTone,
                        }

                        Icon {
                            icon_type: "user",
                            theme: IconTheme::TwoTone,
                        }

                        Icon {
                            icon_type: "setting",
                            theme: IconTheme::TwoTone,
                        }
                    }
                }
            }

            // 不同尺寸示例
            div {
                style: "margin-bottom: 32px;",
                h2 { "不同尺寸" }

                div {
                    style: "display: flex; gap: 16px; align-items: center; margin-bottom: 16px;",

                    div {
                        style: "text-align: center;",
                        div { "小号 (12px)" }
                        Icon {
                            icon_type: "star",
                            size: Some("12px".to_string()),
                        }
                    }

                    div {
                        style: "text-align: center;",
                        div { "默认 (14px)" }
                        Icon {
                            icon_type: "star",
                        }
                    }

                    div {
                        style: "text-align: center;",
                        div { "中号 (18px)" }
                        Icon {
                            icon_type: "star",
                            size: Some("18px".to_string()),
                        }
                    }

                    div {
                        style: "text-align: center;",
                        div { "大号 (24px)" }
                        Icon {
                            icon_type: "star",
                            size: Some("24px".to_string()),
                        }
                    }

                    div {
                        style: "text-align: center;",
                        div { "超大 (32px)" }
                        Icon {
                            icon_type: "star",
                            size: Some("32px".to_string()),
                        }
                    }
                }
            }

            // 不同颜色示例
            div {
                style: "margin-bottom: 32px;",
                h2 { "不同颜色" }

                div {
                    style: "display: flex; gap: 16px; align-items: center; margin-bottom: 16px;",

                    div {
                        style: "text-align: center;",
                        div { "默认" }
                        Icon {
                            icon_type: "heart",
                            size: Some("20px".to_string()),
                        }
                    }

                    div {
                        style: "text-align: center;",
                        div { "红色" }
                        Icon {
                            icon_type: "heart",
                            size: Some("20px".to_string()),
                            color: Some("#ff4d4f".to_string()),
                        }
                    }

                    div {
                        style: "text-align: center;",
                        div { "绿色" }
                        Icon {
                            icon_type: "heart",
                            size: Some("20px".to_string()),
                            color: Some("#52c41a".to_string()),
                        }
                    }

                    div {
                        style: "text-align: center;",
                        div { "蓝色" }
                        Icon {
                            icon_type: "heart",
                            size: Some("20px".to_string()),
                            color: Some("#1677ff".to_string()),
                        }
                    }

                    div {
                        style: "text-align: center;",
                        div { "橙色" }
                        Icon {
                            icon_type: "heart",
                            size: Some("20px".to_string()),
                            color: Some("#faad14".to_string()),
                        }
                    }
                }
            }

            // 旋转示例
            div {
                style: "margin-bottom: 32px;",
                h2 { "旋转效果" }

                div {
                    style: "display: flex; gap: 16px; align-items: center; margin-bottom: 16px;",

                    div {
                        style: "text-align: center;",
                        div { "无旋转" }
                        Icon {
                            icon_type: "setting",
                            size: Some("20px".to_string()),
                        }
                    }

                    div {
                        style: "text-align: center;",
                        div { "90度" }
                        Icon {
                            icon_type: "setting",
                            size: Some("20px".to_string()),
                            rotate: IconRotate::Rotate90,
                        }
                    }

                    div {
                        style: "text-align: center;",
                        div { "180度" }
                        Icon {
                            icon_type: "setting",
                            size: Some("20px".to_string()),
                            rotate: IconRotate::Rotate180,
                        }
                    }

                    div {
                        style: "text-align: center;",
                        div { "270度" }
                        Icon {
                            icon_type: "setting",
                            size: Some("20px".to_string()),
                            rotate: IconRotate::Rotate270,
                        }
                    }
                }
            }

            // 旋转动画示例
            div {
                style: "margin-bottom: 32px;",
                h2 { "旋转动画" }

                div {
                    style: "display: flex; gap: 16px; align-items: center; margin-bottom: 16px;",

                    div {
                        style: "text-align: center;",
                        div { "静态" }
                        Icon {
                            icon_type: "loading",
                            size: Some("20px".to_string()),
                        }
                    }

                    div {
                        style: "text-align: center;",
                        div { "旋转动画" }
                        Icon {
                            icon_type: "loading",
                            size: Some("20px".to_string()),
                            spin: true,
                        }
                    }
                }
            }

            // 常用图标集合
            div {
                h2 { "常用图标" }

                div {
                    style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(120px, 1fr)); gap: 16px;",

                    div {
                        style: "text-align: center; padding: 16px; border: 1px solid #d9d9d9; border-radius: 6px;",
                        Icon {
                            icon_type: "home",
                            size: Some("24px".to_string()),
                        }
                        div { style: "margin-top: 8px; font-size: 12px;", "home" }
                    }

                    div {
                        style: "text-align: center; padding: 16px; border: 1px solid #d9d9d9; border-radius: 6px;",
                        Icon {
                            icon_type: "user",
                            size: Some("24px".to_string()),
                        }
                        div { style: "margin-top: 8px; font-size: 12px;", "user" }
                    }

                    div {
                        style: "text-align: center; padding: 16px; border: 1px solid #d9d9d9; border-radius: 6px;",
                        Icon {
                            icon_type: "setting",
                            size: Some("24px".to_string()),
                        }
                        div { style: "margin-top: 8px; font-size: 12px;", "setting" }
                    }

                    div {
                        style: "text-align: center; padding: 16px; border: 1px solid #d9d9d9; border-radius: 6px;",
                        Icon {
                            icon_type: "search",
                            size: Some("24px".to_string()),
                        }
                        div { style: "margin-top: 8px; font-size: 12px;", "search" }
                    }

                    div {
                        style: "text-align: center; padding: 16px; border: 1px solid #d9d9d9; border-radius: 6px;",
                        Icon {
                            icon_type: "heart",
                            size: Some("24px".to_string()),
                        }
                        div { style: "margin-top: 8px; font-size: 12px;", "heart" }
                    }

                    div {
                        style: "text-align: center; padding: 16px; border: 1px solid #d9d9d9; border-radius: 6px;",
                        Icon {
                            icon_type: "star",
                            size: Some("24px".to_string()),
                        }
                        div { style: "margin-top: 8px; font-size: 12px;", "star" }
                    }

                    div {
                        style: "text-align: center; padding: 16px; border: 1px solid #d9d9d9; border-radius: 6px;",
                        Icon {
                            icon_type: "loading",
                            size: Some("24px".to_string()),
                        }
                        div { style: "margin-top: 8px; font-size: 12px;", "loading" }
                    }

                    div {
                        style: "text-align: center; padding: 16px; border: 1px solid #d9d9d9; border-radius: 6px;",
                        Icon {
                            icon_type: "check",
                            size: Some("24px".to_string()),
                        }
                        div { style: "margin-top: 8px; font-size: 12px;", "check" }
                    }
                }
            }
        }
    }
}

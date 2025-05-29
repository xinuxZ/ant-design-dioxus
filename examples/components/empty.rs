//! Empty 空状态组件示例

use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

/// Empty 组件演示
#[component]
pub fn EmptyDemo() -> Element {
    rsx! {
        div {
            style: "padding: 24px;",

            h2 { "Empty 空状态" }
            p { "空状态时的展示占位图。" }

            // 基础用法
            div {
                style: "margin-bottom: 32px;",
                h3 { "基础用法" }
                div {
                    style: "border: 1px solid #d9d9d9; padding: 24px; border-radius: 6px;",
                    Empty {}
                }
            }

            // 自定义描述
            div {
                style: "margin-bottom: 32px;",
                h3 { "自定义描述" }
                div {
                    style: "border: 1px solid #d9d9d9; padding: 24px; border-radius: 6px;",
                    Empty {
                        description: "自定义描述内容"
                    }
                }
            }

            // 带操作按钮
            div {
                style: "margin-bottom: 32px;",
                h3 { "带操作按钮" }
                div {
                    style: "border: 1px solid #d9d9d9; padding: 24px; border-radius: 6px;",
                    Empty {
                        description: "暂无数据",
                        Button {
                            button_type: ButtonType::Primary,
                            "立即创建"
                        }
                    }
                }
            }

            // 自定义图片
            div {
                style: "margin-bottom: 32px;",
                h3 { "自定义图片" }
                div {
                    style: "border: 1px solid #d9d9d9; padding: 24px; border-radius: 6px;",
                    Empty {
                        image: "https://gw.alipayobjects.com/zos/antfincdn/ZHrcdLPrvN/empty.svg",
                        image_style: "height: 60px;",
                        description: "自定义图片的空状态",
                        Button {
                            button_type: ButtonType::Primary,
                            "重新加载"
                        }
                    }
                }
            }

            // 无描述
            div {
                style: "margin-bottom: 32px;",
                h3 { "无描述" }
                div {
                    style: "border: 1px solid #d9d9d9; padding: 24px; border-radius: 6px;",
                    Empty {
                        description: ""
                    }
                }
            }

            // 自定义样式
            div {
                style: "margin-bottom: 32px;",
                h3 { "自定义样式" }
                div {
                    style: "border: 1px solid #d9d9d9; padding: 24px; border-radius: 6px; background: #fafafa;",
                    Empty {
                        class: "custom-empty",
                        style: "color: #1890ff;",
                        description: "自定义样式的空状态",
                        Button {
                            button_type: ButtonType::Dashed,
                            "添加内容"
                        }
                    }
                }
            }
        }
    }
}

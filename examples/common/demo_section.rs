//! 演示区块组件
//!
//! 提供统一的示例展示格式

use dioxus::prelude::*;

/// 演示区块属性
#[derive(Props, Clone, PartialEq)]
pub struct DemoSectionProps {
    /// 示例标题
    pub title: String,
    /// 示例描述
    pub description: String,
    /// 子组件
    pub children: Element,
}

/// 演示区块组件
///
/// 用于包装每个功能演示，提供统一的标题、描述和内容区域
#[component]
pub fn DemoSection(props: DemoSectionProps) -> Element {
    rsx! {
        div {
            class: "demo-section",
            style: "margin-bottom: 32px; border: 1px solid #f0f0f0; border-radius: 6px;",

            div {
                class: "demo-content",
                style: "padding: 24px;",
                {props.children}
            }

            div {
                class: "demo-meta",
                style: "border-top: 1px solid #f0f0f0; padding: 16px 24px; background: #fafafa;",

                h4 {
                    style: "margin: 0 0 8px 0; color: #262626; font-size: 14px; font-weight: 500;",
                    "{props.title}"
                }

                p {
                    style: "margin: 0; color: #666; font-size: 12px; line-height: 1.5;",
                    "{props.description}"
                }
            }
        }
    }
}

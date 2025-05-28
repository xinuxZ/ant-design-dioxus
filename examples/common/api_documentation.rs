//! API文档组件
//!
//! 用于展示组件的属性和使用说明

use dioxus::prelude::*;

/// 属性文档结构
#[derive(Clone, PartialEq)]
pub struct PropDoc {
    pub name: String,
    pub prop_type: String,
    pub default: String,
    pub description: String,
}

/// API文档组件属性
#[derive(Props, Clone, PartialEq)]
pub struct ApiDocumentationProps {
    /// 组件名称
    pub component_name: String,
    /// 属性列表
    pub props: Vec<PropDoc>,
}

/// API文档组件
///
/// 以表格形式展示组件的所有属性、类型、默认值和说明
#[component]
pub fn ApiDocumentation(props: ApiDocumentationProps) -> Element {
    rsx! {
        div {
            class: "api-documentation",
            style: "margin-top: 48px;",

            h2 {
                style: "margin: 0 0 16px 0; color: #262626; font-size: 20px;",
                "API"
            }

            h3 {
                style: "margin: 0 0 16px 0; color: #262626; font-size: 16px;",
                "{props.component_name}"
            }

            div {
                class: "api-table",
                style: "border: 1px solid #f0f0f0; border-radius: 6px; overflow: hidden;",

                div {
                    class: "table-header",
                    style: "display: grid; grid-template-columns: 1fr 1fr 1fr 2fr; background: #fafafa; border-bottom: 1px solid #f0f0f0;",

                    div { style: "padding: 12px 16px; font-weight: 500; color: #262626;", "参数" }
                    div { style: "padding: 12px 16px; font-weight: 500; color: #262626;", "类型" }
                    div { style: "padding: 12px 16px; font-weight: 500; color: #262626;", "默认值" }
                    div { style: "padding: 12px 16px; font-weight: 500; color: #262626;", "说明" }
                }

                for prop in props.props {
                    div {
                        class: "table-row",
                        style: "display: grid; grid-template-columns: 1fr 1fr 1fr 2fr; border-bottom: 1px solid #f0f0f0;",

                        div {
                            style: "padding: 12px 16px; color: #262626; font-family: 'SFMono-Regular', Consolas, 'Liberation Mono', Menlo, Courier, monospace;",
                            "{prop.name}"
                        }
                        div {
                            style: "padding: 12px 16px; color: #1890ff; font-family: 'SFMono-Regular', Consolas, 'Liberation Mono', Menlo, Courier, monospace;",
                            "{prop.prop_type}"
                        }
                        div {
                            style: "padding: 12px 16px; color: #666; font-family: 'SFMono-Regular', Consolas, 'Liberation Mono', Menlo, Courier, monospace;",
                            "{prop.default}"
                        }
                        div {
                            style: "padding: 12px 16px; color: #666;",
                            "{prop.description}"
                        }
                    }
                }
            }
        }
    }
}

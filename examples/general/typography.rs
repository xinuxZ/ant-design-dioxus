//! Typography 组件示例
//!
//! 展示 Typography 组件的各种用法和样式

use crate::common::{ApiDocumentation, DemoSection, PropDoc};
use dioxus::prelude::*;

/// Typography 组件演示页面
#[component]
pub fn TypographyDemo() -> Element {
    rsx! {
        div {
            class: "typography-demo",
            style: "padding: 24px;",

            h1 {
                style: "margin: 0 0 24px 0; color: #262626; font-size: 28px;",
                "Typography 排版"
            }

            p {
                style: "margin: 0 0 32px 0; color: #666; font-size: 14px;",
                "文本的基本格式。"
            }

            // 标题
            DemoSection {
                title: "标题".to_string(),
                description: "展示不同级别的标题。".to_string(),
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    h1 { style: "margin: 0; color: #262626;", "h1. Ant Design" }
                    h2 { style: "margin: 0; color: #262626;", "h2. Ant Design" }
                    h3 { style: "margin: 0; color: #262626;", "h3. Ant Design" }
                    h4 { style: "margin: 0; color: #262626;", "h4. Ant Design" }
                    h5 { style: "margin: 0; color: #262626;", "h5. Ant Design" }
                    h6 { style: "margin: 0; color: #262626;", "h6. Ant Design" }
                }
            }

            // 文本
            DemoSection {
                title: "文本".to_string(),
                description: "内置不同样式的文本。".to_string(),
                div {
                    style: "display: flex; flex-direction: column; gap: 8px;",

                    p { style: "margin: 0; color: #262626;", "Ant Design (default)" }
                    p { style: "margin: 0; color: #666;", "Ant Design (secondary)" }
                    p { style: "margin: 0; color: #52c41a;", "Ant Design (success)" }
                    p { style: "margin: 0; color: #faad14;", "Ant Design (warning)" }
                    p { style: "margin: 0; color: #ff4d4f;", "Ant Design (danger)" }
                    p { style: "margin: 0; color: #bfbfbf;", "Ant Design (disabled)" }
                }
            }

            // 文本样式
            DemoSection {
                title: "文本样式".to_string(),
                description: "不同的文本样式：加粗、斜体、下划线、删除线、代码等。".to_string(),
                div {
                    style: "display: flex; flex-direction: column; gap: 8px;",

                    p {
                        style: "margin: 0; color: #262626;",
                        "We supply a series of design principles, practical patterns and high quality design resources ("
                        strong { "Bold" }
                        "), to help people create their product prototypes beautifully and efficiently."
                    }

                    p {
                        style: "margin: 0; color: #262626;",
                        "We supply a series of design principles, practical patterns and high quality design resources ("
                        em { "Italic" }
                        "), to help people create their product prototypes beautifully and efficiently."
                    }

                    p {
                        style: "margin: 0; color: #262626;",
                        "We supply a series of design principles, practical patterns and high quality design resources ("
                        u { "Underline" }
                        "), to help people create their product prototypes beautifully and efficiently."
                    }

                    p {
                        style: "margin: 0; color: #262626;",
                        "We supply a series of design principles, practical patterns and high quality design resources ("
                        s { "Delete" }
                        "), to help people create their product prototypes beautifully and efficiently."
                    }

                    p {
                        style: "margin: 0; color: #262626;",
                        "We supply a series of design principles, practical patterns and high quality design resources ("
                        code {
                            style: "background: #f5f5f5; padding: 2px 4px; border-radius: 3px; font-family: 'SFMono-Regular', Consolas, 'Liberation Mono', Menlo, Courier, monospace;",
                            "Code"
                        }
                        "), to help people create their product prototypes beautifully and efficiently."
                    }

                    p {
                        style: "margin: 0; color: #262626;",
                        "We supply a series of design principles, practical patterns and high quality design resources ("
                        mark {
                            style: "background: #ffe58f; padding: 0 2px;",
                            "Mark"
                        }
                        "), to help people create their product prototypes beautifully and efficiently."
                    }
                }
            }

            // 段落
            DemoSection {
                title: "段落".to_string(),
                description: "段落组件。".to_string(),
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    p {
                        style: "margin: 0; color: #262626; line-height: 1.5;",
                        "In the process of internal desktop applications development, many different design specs and implementations would be involved, which might cause designers and developers difficulties and duplication and reduce the efficiency of development."
                    }

                    p {
                        style: "margin: 0; color: #262626; line-height: 1.5;",
                        "After massive project practice and summaries, Ant Design, a design language for background applications, is refined by Ant UED Team, which aims to "
                        strong { "uniform the user interface specs for internal background projects, lower the unnecessary cost of design differences and implementation and liberate the resources of design and front-end development" }
                        "."
                    }
                }
            }

            // 列表
            DemoSection {
                title: "列表".to_string(),
                description: "无序和有序列表。".to_string(),
                div {
                    style: "display: grid; grid-template-columns: 1fr 1fr; gap: 24px;",

                    div {
                        h4 { style: "margin: 0 0 12px 0; color: #262626;", "无序列表" }
                        ul {
                            style: "margin: 0; padding-left: 20px; color: #262626;",
                            li { "Racing car sprays burning fuel into crowd." }
                            li { "Japanese princess to wed commoner." }
                            li { "Australian walks 100km after outback crash." }
                            li { "Man charged over missing wedding girl." }
                        }
                    }

                    div {
                        h4 { style: "margin: 0 0 12px 0; color: #262626;", "有序列表" }
                        ol {
                            style: "margin: 0; padding-left: 20px; color: #262626;",
                            li { "Racing car sprays burning fuel into crowd." }
                            li { "Japanese princess to wed commoner." }
                            li { "Australian walks 100km after outback crash." }
                            li { "Man charged over missing wedding girl." }
                        }
                    }
                }
            }

            // 引用
            DemoSection {
                title: "引用".to_string(),
                description: "引用样式的文本。".to_string(),
                blockquote {
                    style: "margin: 0; padding: 16px 24px; border-left: 4px solid #1890ff; background: #f6ffed; color: #262626; font-style: italic;",
                    "A well-known quote, contained in a blockquote element."
                }
            }

            // 代码块
            DemoSection {
                title: "代码块".to_string(),
                description: "代码块展示。".to_string(),
                pre {
                    style: "margin: 0; padding: 16px; background: #f5f5f5; border-radius: 6px; overflow-x: auto; font-family: 'SFMono-Regular', Consolas, 'Liberation Mono', Menlo, Courier, monospace;",
                    code {
                        style: "color: #262626;",
                        "fn main() {\n    println!(\"Hello, world!\");\n}"
                    }
                }
            }

            // API 文档
            ApiDocumentation {
                component_name: "Typography".to_string(),
                props: vec![
                    PropDoc {
                        name: "type".to_string(),
                        prop_type: "String".to_string(),
                        default: "default".to_string(),
                        description: "文本类型：default | secondary | success | warning | danger | disabled".to_string(),
                    },
                    PropDoc {
                        name: "strong".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "是否加粗".to_string(),
                    },
                    PropDoc {
                        name: "italic".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "是否斜体".to_string(),
                    },
                    PropDoc {
                        name: "underline".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "是否下划线".to_string(),
                    },
                    PropDoc {
                        name: "delete".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "是否删除线".to_string(),
                    },
                    PropDoc {
                        name: "code".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "是否代码样式".to_string(),
                    },
                    PropDoc {
                        name: "mark".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "是否标记样式".to_string(),
                    },
                ]
            }
        }
    }
}

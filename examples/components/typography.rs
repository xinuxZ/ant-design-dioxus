//! Typography 组件演示
//!
//! 展示 Typography 组件的各种用法，包括标题、文本、段落等

use crate::common::*;
use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

// 重命名 Text 组件以避免与 dioxus 的 Text 冲突
use ant_design_dioxus::prelude::Text as AntText;

/// Typography 组件演示
#[component]
pub fn TypographyDemo() -> Element {
    rsx! {
        div {
            class: "component-showcase",

            h1 {
                style: "margin: 0 0 16px 0; color: #262626; font-size: 28px;",
                "Typography 排版"
            }

            p {
                style: "margin: 0 0 24px 0; color: #666; font-size: 14px;",
                "文本的基本格式。"
            }

            // 标题
            DemoSection {
                title: "标题",
                description: "展示不同级别的标题。",

                div {
                    style: "display: flex; flex-direction: column; gap: 8px;",

                    Title {
                        level: HeadingLevel::H1,
                        "h1. Ant Design"
                    }

                    Title {
                        level: HeadingLevel::H2,
                        "h2. Ant Design"
                    }

                    Title {
                        level: HeadingLevel::H3,
                        "h3. Ant Design"
                    }

                    Title {
                        level: HeadingLevel::H4,
                        "h4. Ant Design"
                    }

                    Title {
                        level: HeadingLevel::H5,
                        "h5. Ant Design"
                    }
                }
            }

            // 文本类型
            DemoSection {
                title: "文本类型",
                description: "内置不同样式的文本。",

                div {
                    style: "display: flex; flex-direction: column; gap: 8px;",

                    div {
                        AntText { "默认文本" }
                    }

                    div {
                        AntText {
                            text_type: TextType::Secondary,
                            "次要文本"
                        }
                    }

                    div {
                        AntText {
                            text_type: TextType::Success,
                            "成功文本"
                        }
                    }

                    div {
                        AntText {
                            text_type: TextType::Warning,
                            "警告文本"
                        }
                    }

                    div {
                        AntText {
                            text_type: TextType::Danger,
                            "危险文本"
                        }
                    }

                    div {
                        AntText {
                            disabled: true,
                            "禁用文本"
                        }
                    }
                }
            }

            // 文本样式
            DemoSection {
                title: "文本样式",
                description: "不同的文本样式。",

                div {
                    style: "display: flex; flex-direction: column; gap: 8px;",

                    div {
                        AntText {
                            strong: true,
                            "加粗文本"
                        }
                    }

                    div {
                        AntText {
                            italic: true,
                            "斜体文本"
                        }
                    }

                    div {
                        AntText {
                            underline: true,
                            "下划线文本"
                        }
                    }

                    div {
                        AntText {
                            delete: true,
                            "删除线文本"
                        }
                    }

                    div {
                        AntText {
                            code: true,
                            "代码文本"
                        }
                    }

                    div {
                        AntText {
                            mark: true,
                            "标记文本"
                        }
                    }
                }
            }

            // 段落
            DemoSection {
                title: "段落",
                description: "段落组件。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    Paragraph {
                        "这是一个段落。Ant Design 是一套企业级 UI 设计语言和 React 组件库，
                        致力于提升『用户』和『设计者』的体验。"
                    }

                    Paragraph {
                        text_type: TextType::Secondary,
                        "这是一个次要段落。我们基于『确定』和『自然』的设计价值观，
                        通过模块化的解决方案，降低冗余的生产成本。"
                    }
                }
            }

            // 可复制文本
            DemoSection {
                title: "可复制文本",
                description: "提供额外的复制功能。",

                div {
                    AntText {
                        copyable: true,
                        "这是一段可复制的文本。"
                    }
                }
            }

            // 省略文本
            DemoSection {
                title: "省略文本",
                description: "多行文本省略。",

                div {
                    style: "width: 300px;",
                    Paragraph {
                        ellipsis: true,
                        "Ant Design 是一套企业级 UI 设计语言和 React 组件库，
                        致力于提升『用户』和『设计者』的体验。我们基于『确定』和『自然』的设计价值观，
                        通过模块化的解决方案，降低冗余的生产成本，让设计者专注于更好的用户体验。"
                    }
                }
            }

            // API 文档
            ApiDocumentation {
                component_name: "Typography",
                props: vec![
                    PropDoc {
                        name: "level".to_string(),
                        prop_type: "HeadingLevel".to_string(),
                        default: "HeadingLevel::H1".to_string(),
                        description: "标题级别 (仅 Title 组件)".to_string(),
                    },
                    PropDoc {
                        name: "text_type".to_string(),
                        prop_type: "TextType".to_string(),
                        default: "TextType::Default".to_string(),
                        description: "文本类型".to_string(),
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
                    PropDoc {
                        name: "disabled".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "是否禁用".to_string(),
                    },
                    PropDoc {
                        name: "copyable".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "是否可复制".to_string(),
                    },
                    PropDoc {
                        name: "ellipsis".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "是否省略 (仅 Paragraph 组件)".to_string(),
                    },
                ]
            }
        }
    }
}

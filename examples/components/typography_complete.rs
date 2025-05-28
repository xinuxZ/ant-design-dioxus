//! Typography 组件完整演示
//!
//! 展示 Typography 组件族的各种用法，包括 Title、Text、Paragraph、Link 等

use crate::common::*;
use ant_design_dioxus::prelude::{Link as AntLink, *};
use dioxus::prelude::*;

/// Typography 组件完整演示
#[component]
pub fn TypographyCompleteDemo() -> Element {
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

            // 标题演示
            DemoSection {
                title: "标题",
                description: "展示不同级别的标题。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

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
                description: "不同类型的文本样式。",

                div {
                    style: "display: flex; flex-direction: column; gap: 8px;",

                    Text {
                        "默认文本"
                    }
                    Text {
                        text_type: TextType::Secondary,
                        "次要文本"
                    }
                    Text {
                        text_type: TextType::Success,
                        "成功文本"
                    }
                    Text {
                        text_type: TextType::Warning,
                        "警告文本"
                    }
                    Text {
                        text_type: TextType::Danger,
                        "危险文本"
                    }
                    Text {
                        text_type: TextType::Disabled,
                        "禁用文本"
                    }
                }
            }

            // 文本样式
            DemoSection {
                title: "文本样式",
                description: "各种文本装饰样式。",

                div {
                    style: "display: flex; flex-direction: column; gap: 8px;",

                    Text {
                        strong: true,
                        "加粗文本"
                    }
                    Text {
                        italic: true,
                        "斜体文本"
                    }
                    Text {
                        underline: true,
                        "下划线文本"
                    }
                    Text {
                        delete: true,
                        "删除线文本"
                    }
                    Text {
                        mark: true,
                        "标记文本"
                    }
                    Text {
                        code: true,
                        "代码文本"
                    }
                }
            }

            // 段落
            DemoSection {
                title: "段落",
                description: "段落文本的展示。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    Paragraph {
                        "这是一个普通的段落。Ant Design 是一套企业级 UI 设计语言和 React 组件库，
                        致力于提升『用户』和『设计者』的体验。"
                    }

                    Paragraph {
                        text_type: TextType::Secondary,
                        "这是一个次要的段落。我们正在构建一个更好的用户体验，
                        通过统一的设计规范和组件库来提高开发效率。"
                    }
                }
            }

            // 省略文本
            DemoSection {
                title: "省略文本",
                description: "超长文本的省略展示。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px; width: 400px;",

                    Paragraph {
                        ellipsis: true,
                        "这是一段很长的文本，当文本超出容器宽度时会自动省略显示。
                        Ant Design 是一套企业级 UI 设计语言和 React 组件库。"
                    }

                    Paragraph {
                        ellipsis: true,
                        ellipsis_rows: Some(2),
                        "这是一段很长的文本，支持多行省略。当文本超出指定行数时会自动省略显示。
                        Ant Design 是一套企业级 UI 设计语言和 React 组件库，
                        致力于提升『用户』和『设计者』的体验。我们正在构建一个更好的用户体验，
                        通过统一的设计规范和组件库来提高开发效率。"
                    }
                }
            }

            // 链接
            DemoSection {
                title: "链接",
                description: "各种链接样式。",

                div {
                    style: "display: flex; flex-direction: column; gap: 8px;",

                    AntLink {
                        href: "https://ant.design",
                        "Ant Design (默认)"
                    }

                    AntLink {
                        href: "https://ant.design",
                        text_type: TextType::Success,
                        "Ant Design (成功)"
                    }

                    AntLink {
                        href: "https://ant.design",
                        text_type: TextType::Warning,
                        "Ant Design (警告)"
                    }

                    AntLink {
                        href: "https://ant.design",
                        text_type: TextType::Danger,
                        "Ant Design (危险)"
                    }

                    AntLink {
                        disabled: true,
                        "Ant Design (禁用)"
                    }
                }
            }

            // 组合使用
            DemoSection {
                title: "组合使用",
                description: "多种排版元素的组合使用。",

                div {
                    style: "line-height: 1.8;",

                    Title {
                        level: HeadingLevel::H3,
                        "介绍"
                    }

                    Paragraph {
                        "Ant Design 是一套企业级 UI 设计语言和 React 组件库，
                        致力于提升『用户』和『设计者』的体验。"
                    }

                    Title {
                        level: HeadingLevel::H4,
                        "设计原则"
                    }

                    Paragraph {
                        "我们基于 "
                        Text {
                            strong: true,
                            "确定性"
                        }
                        "、"
                        Text {
                            strong: true,
                            "意义感"
                        }
                        "、"
                        Text {
                            strong: true,
                            "生长性"
                        }
                        "、"
                        Text {
                            strong: true,
                            "自然"
                        }
                        " 的价值观，通过模块化的解决方案，降低冗余的生产成本，
                        让设计者专注于 "
                        Text {
                            mark: true,
                            "更好的用户体验"
                        }
                        "。"
                    }

                    Paragraph {
                        "更多信息请访问 "
                        AntLink {
                            href: "https://ant.design",
                            target: "_blank",
                            "官方网站"
                        }
                        "。"
                    }
                }
            }

            // 可交互功能
            DemoSection {
                title: "可交互功能",
                description: "可复制、可编辑等交互功能（功能演示，实际交互需要额外实现）。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    Paragraph {
                        copyable: true,
                        "这是可复制的文本。点击复制图标可以复制文本内容。"
                    }

                    Paragraph {
                        editable: true,
                        "这是可编辑的文本。点击编辑图标可以编辑文本内容。"
                    }

                    Paragraph {
                        copyable: true,
                        editable: true,
                        "这是既可复制又可编辑的文本。"
                    }
                }
            }
        }
    }
}

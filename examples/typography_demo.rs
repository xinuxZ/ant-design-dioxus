//! Typography 排版组件示例
//!
//! 展示 Typography 组件的各种用法，包括标题、文本、段落等。

use ant_design_dioxus::prelude::*;
use dioxus as _;
use once_cell as _;
use serde as _;
use serde_json as _;
use wasm_bindgen_test as _;
use web_sys as _;

// 重命名 Text 组件以避免与 dioxus 的 Text 冲突
use ant_design_dioxus::prelude::Text as AntText;

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        div {
            style: "padding: 24px; max-width: 800px; margin: 0 auto;",

            // 标题示例
            div {
                style: "margin-bottom: 32px;",
                h2 { "标题示例" }

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

            // 文本类型示例
            div {
                style: "margin-bottom: 32px;",
                h2 { "文本类型" }

                div {
                    style: "margin-bottom: 8px;",
                    AntText { "默认文本" }
                }

                div {
                    style: "margin-bottom: 8px;",
                    AntText {
                        text_type: TextType::Secondary,
                        "次要文本"
                    }
                }

                div {
                    style: "margin-bottom: 8px;",
                    AntText {
                        text_type: TextType::Success,
                        "成功文本"
                    }
                }

                div {
                    style: "margin-bottom: 8px;",
                    AntText {
                        text_type: TextType::Warning,
                        "警告文本"
                    }
                }

                div {
                    style: "margin-bottom: 8px;",
                    AntText {
                        text_type: TextType::Danger,
                        "危险文本"
                    }
                }

                div {
                    style: "margin-bottom: 8px;",
                    AntText {
                        text_type: TextType::Disabled,
                        "禁用文本"
                    }
                }
            }

            // 文本修饰示例
            div {
                style: "margin-bottom: 32px;",
                h2 { "文本修饰" }

                div {
                    style: "margin-bottom: 8px;",
                    AntText {
                        strong: true,
                        "加粗文本"
                    }
                }

                div {
                    style: "margin-bottom: 8px;",
                    AntText {
                        italic: true,
                        "斜体文本"
                    }
                }

                div {
                    style: "margin-bottom: 8px;",
                    AntText {
                        underline: true,
                        "下划线文本"
                    }
                }

                div {
                    style: "margin-bottom: 8px;",
                    AntText {
                        delete: true,
                        "删除线文本"
                    }
                }

                div {
                    style: "margin-bottom: 8px;",
                    AntText {
                        mark: true,
                        "标记文本"
                    }
                }

                div {
                    style: "margin-bottom: 8px;",
                    AntText {
                        code: true,
                        "代码文本"
                    }
                }
            }

            // 段落示例
            div {
                style: "margin-bottom: 32px;",
                h2 { "段落" }

                Paragraph {
                    "这是一个普通的段落。Ant Design 是一套企业级 UI 设计语言和 React 组件库，
                    致力于提升『用户』和『设计者』的体验。"
                }

                Paragraph {
                    text_type: TextType::Secondary,
                    "这是一个次要段落。我们基于『确定』和『自然』的设计价值观，
                    通过模块化的解决方案，降低冗余的生产成本，让设计者专注于更好的用户体验。"
                }
            }

            // 省略文本示例
            div {
                style: "margin-bottom: 32px;",
                h2 { "省略文本" }

                div {
                    style: "width: 300px; margin-bottom: 16px;",
                    AntText {
                        ellipsis: true,
                        "这是一段很长的文本，当超出容器宽度时会自动省略显示..."
                    }
                }

                div {
                    style: "width: 300px; margin-bottom: 16px;",
                    Paragraph {
                        ellipsis: true,
                        ellipsis_rows: Some(3),
                        "这是一个多行省略的段落示例。当文本内容超过指定的行数时，
                        会自动在末尾显示省略号。这对于需要限制文本显示区域的场景非常有用，
                        比如卡片组件、列表项等。用户可以通过设置 ellipsis_rows 属性来控制显示的行数。
                        这样可以保持界面的整洁和一致性，同时提供良好的用户体验。"
                    }
                }
            }

            // 交互功能示例
            div {
                style: "margin-bottom: 32px;",
                h2 { "交互功能" }

                div {
                    style: "margin-bottom: 16px;",
                    AntText {
                        copyable: true,
                        "可复制的文本（悬停查看复制图标）"
                    }
                }

                div {
                    style: "margin-bottom: 16px;",
                    AntText {
                        editable: true,
                        "可编辑的文本（悬停查看编辑图标）"
                    }
                }
            }

            // 组合使用示例
            div {
                h2 { "组合使用" }

                Paragraph {
                    "在 Ant Design 中，我们提供了丰富的排版组件。你可以使用 "
                    AntText {
                        strong: true,
                        text_type: TextType::Success,
                        "加粗的成功文本"
                    }
                    "，也可以使用 "
                    AntText {
                        italic: true,
                        text_type: TextType::Warning,
                        "斜体的警告文本"
                    }
                    "，甚至是 "
                    AntText {
                        code: true,
                        "代码样式的文本"
                    }
                    " 来满足不同的设计需求。"
                }
            }
        }
    }
}

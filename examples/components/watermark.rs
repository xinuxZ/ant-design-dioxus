use crate::common::demo_section::DemoSection;
use ant_design_dioxus::components::watermark::Watermark;
use dioxus::prelude::*;

/// Watermark组件示例
#[component]
pub fn WatermarkDemo() -> Element {
    rsx! {
        div { class: "demo-container",
            h1 { "Watermark 水印" }
            p { "给页面的某个区域加上水印。" }

            DemoSection {
                title: "基本使用",
                description: "最简单的用法。",
                code: r#"rsx! {
    Watermark {
        content: "Ant Design",
        div {
            style: "height: 200px; padding: 24px;",
            "这里是内容区域"
        }
    }
}"#,
                demo: rsx! {
                    Watermark {
                        class: "demo-watermark",
                        style: "position: relative; border: 1px solid #d9d9d9; border-radius: 8px; overflow: hidden;",
                        content: "Ant Design",
                        div {
                            style: "height: 200px; padding: 24px; background: #fafafa; display: flex; align-items: center; justify-content: center;",
                            "这里是内容区域"
                        }
                    }
                }
            }

            DemoSection {
                title: "多行水印",
                description: "通过设置多行文本作为水印内容。",
                code: r#"rsx! {
    Watermark {
        content: "Ant Design\nDioxus UI",
        div {
            style: "height: 200px; padding: 24px;",
            "多行水印内容"
        }
    }
}"#,
                demo: rsx! {
                    Watermark {
                        class: "watermark-multiline",
                        style: "position: relative; border: 1px solid #d9d9d9; border-radius: 8px; overflow: hidden;",
                        content: "Ant Design\nDioxus UI",
                        div {
                            style: "height: 200px; padding: 24px; background: #f0f2f5; display: flex; align-items: center; justify-content: center;",
                            "多行水印内容"
                        }
                    }
                }
            }

            DemoSection {
                title: "图片水印",
                description: "使用图片作为水印。",
                code: r#"rsx! {
    Watermark {
        class: "watermark-image",
        div {
            style: "height: 200px; padding: 24px;",
            "图片水印示例"
        }
    }
}"#,
                demo: rsx! {
                    Watermark {
                        class: "watermark-image",
                        style: "position: relative; border: 1px solid #d9d9d9; border-radius: 8px; overflow: hidden;",
                        div {
                            style: "height: 200px; padding: 24px; background: #e6f4ff; display: flex; align-items: center; justify-content: center;",
                            "图片水印示例"
                        }
                    }
                }
            }

            DemoSection {
                title: "自定义配置",
                description: "自定义水印的样式配置。",
                code: r#"rsx! {
    Watermark {
        class: "watermark-custom",
        content: "Custom Style",
        div {
            style: "height: 200px; padding: 24px;",
            "自定义样式水印"
        }
    }
}"#,
                demo: rsx! {
                    Watermark {
                        class: "watermark-custom",
                        style: "position: relative; border: 1px solid #d9d9d9; border-radius: 8px; overflow: hidden;",
                        content: "Custom Style",
                        div {
                            style: "height: 200px; padding: 24px; background: #fff1f0; display: flex; align-items: center; justify-content: center;",
                            "自定义样式水印"
                        }
                    }
                }
            }

            DemoSection {
                title: "全屏水印",
                description: "覆盖整个页面的水印。",
                code: r#"rsx! {
    Watermark {
        class: "watermark-fullscreen",
        content: "Full Screen",
        div {
            style: "height: 300px; padding: 24px;",
            "全屏水印内容区域"
        }
    }
}"#,
                demo: rsx! {
                    Watermark {
                        class: "watermark-fullscreen",
                        style: "position: relative; border: 1px solid #d9d9d9; border-radius: 8px; overflow: hidden;",
                        content: "Full Screen",
                        div {
                            style: "height: 300px; padding: 24px; background: #f6ffed; display: flex; align-items: center; justify-content: center; flex-direction: column; gap: 16px;",
                            h3 { style: "margin: 0;", "全屏水印" }
                            p { style: "margin: 0; text-align: center; color: #666;", "这是一个全屏水印的示例，水印会覆盖整个内容区域" }
                        }
                    }
                }
            }

            DemoSection {
                title: "互相嵌套",
                description: "水印组件可以互相嵌套使用。",
                code: r#"rsx! {
    Watermark {
        content: "Outer Watermark",
        div {
            style: "padding: 24px;",
            Watermark {
                content: "Inner Watermark",
                div {
                    style: "height: 150px; padding: 16px;",
                    "嵌套水印内容"
                }
            }
        }
    }
}"#,
                demo: rsx! {
                    Watermark {
                        class: "watermark-outer",
                        style: "position: relative; border: 1px solid #d9d9d9; border-radius: 8px; overflow: hidden;",
                        content: "Outer Watermark",
                        div {
                            style: "padding: 24px; background: #f9f9f9;",
                            Watermark {
                                class: "watermark-inner",
                                style: "position: relative; border: 1px dashed #1677ff; border-radius: 6px; overflow: hidden;",
                                content: "Inner Watermark",
                                div {
                                    style: "height: 150px; padding: 16px; background: white; display: flex; align-items: center; justify-content: center;",
                                    "嵌套水印内容"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

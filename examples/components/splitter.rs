use crate::common::demo_section::DemoSection;
use ant_design_dioxus::components::splitter::Splitter;
use dioxus::prelude::*;

/// Splitter组件示例
#[component]
pub fn SplitterDemo() -> Element {
    rsx! {
        div { class: "demo-container",
            h1 { "Splitter 分割面板" }
            p { "可分割的面板容器。" }

            DemoSection {
                title: "基本使用",
                description: "最简单的分割面板。",

                div {
                    Splitter {
                        style: "height: 200px; border: 1px solid #d9d9d9; border-radius: 8px; overflow: hidden;",
                        div {
                            style: "padding: 16px; background: #f5f5f5; height: 100%; display: flex; align-items: center; justify-content: center;",
                            "左侧面板"
                        }
                        div {
                            style: "padding: 16px; background: #fafafa; height: 100%; display: flex; align-items: center; justify-content: center;",
                            "右侧面板"
                        }
                    }
                }
            }

            DemoSection {
                title: "垂直分割",
                description: "垂直方向的分割面板。",

                div {
                    Splitter {
                        class: "splitter-vertical",
                        style: "height: 300px; border: 1px solid #d9d9d9; border-radius: 8px; overflow: hidden;",
                        div {
                            style: "padding: 16px; background: #e6f4ff; height: 50%; display: flex; align-items: center; justify-content: center;",
                            "上方面板"
                        }
                        div {
                            style: "padding: 16px; background: #f6ffed; height: 50%; display: flex; align-items: center; justify-content: center;",
                            "下方面板"
                        }
                    }
                }
            }

            DemoSection {
                title: "多面板分割",
                description: "支持多个面板的分割。",

                div {
                    Splitter {
                        style: "height: 200px; border: 1px solid #d9d9d9; border-radius: 8px; overflow: hidden;",
                        div {
                            style: "padding: 16px; background: #fff1f0; height: 100%; display: flex; align-items: center; justify-content: center; flex: 1;",
                            "面板 1"
                        }
                        div {
                            style: "padding: 16px; background: #f6ffed; height: 100%; display: flex; align-items: center; justify-content: center; flex: 1;",
                            "面板 2"
                        }
                        div {
                            style: "padding: 16px; background: #e6f4ff; height: 100%; display: flex; align-items: center; justify-content: center; flex: 1;",
                            "面板 3"
                        }
                    }
                }
            }

            DemoSection {
                title: "可折叠面板",
                description: "支持面板的折叠和展开。",

                div {
                    Splitter {
                        class: "splitter-collapsible",
                        style: "height: 200px; border: 1px solid #d9d9d9; border-radius: 8px; overflow: hidden;",
                        div {
                            style: "padding: 16px; background: #f0f2f5; height: 100%; display: flex; align-items: center; justify-content: center;",
                            "可折叠的左侧面板"
                        }
                        div {
                            style: "padding: 16px; background: #ffffff; height: 100%; display: flex; align-items: center; justify-content: center; flex: 1;",
                            "主要内容区域"
                        }
                    }
                }
            }
        }
    }
}

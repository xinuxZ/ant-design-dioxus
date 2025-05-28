//! Layout 布局组件演示
//!
//! 展示 Layout、Header、Sider、Content、Footer 组件的各种用法

use ant_design_dioxus::prelude::*;

// 避免未使用依赖的编译错误
use once_cell as _;
use serde as _;
use serde_json as _;
use wasm_bindgen_test as _;
use web_sys as _;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        div {
            style: "padding: 24px; background: #f0f2f5; min-height: 100vh;",

            h1 { "Layout 布局组件演示" }

            // 基础布局
            section {
                style: "margin-bottom: 32px;",
                h2 { "基础布局" }
                div {
                    style: "height: 300px; border: 1px solid #d9d9d9;",
                    Layout {
                        Header {
                            style: "background: #7dbcea; color: white; text-align: center;",
                            "Header"
                        }
                        Content {
                            style: "background: #108ee9; color: white; text-align: center; line-height: 120px;",
                            "Content"
                        }
                        Footer {
                            style: "background: #7dbcea; color: white; text-align: center;",
                            "Footer"
                        }
                    }
                }
            }

            // 上中下布局
            section {
                style: "margin-bottom: 32px;",
                h2 { "上中下布局" }
                div {
                    style: "height: 300px; border: 1px solid #d9d9d9;",
                    Layout {
                        Header {
                            style: "background: #108ee9; color: white; text-align: center;",
                            "Header"
                        }
                        Layout {
                            style: "padding: 24px 0;",
                            Content {
                                style: "background: #fff; margin: 0 16px; padding: 24px; text-align: center;",
                                "Content"
                            }
                        }
                        Footer {
                            style: "background: #108ee9; color: white; text-align: center;",
                            "Footer"
                        }
                    }
                }
            }

            // 侧边栏布局
            SiderLayoutDemo {}

            // 自定义触发器
            CustomTriggerDemo {}

            // 响应式侧边栏
            ResponsiveSiderDemo {}
        }
    }
}

/// 侧边栏布局演示
#[component]
fn SiderLayoutDemo() -> Element {
    rsx! {
        section {
            style: "margin-bottom: 32px;",
            h2 { "侧边栏布局" }
            div {
                style: "height: 400px; border: 1px solid #d9d9d9;",
                Layout {
                    has_sider: true,
                    Sider {
                        width: 200,
                        style: "background: #3ba0e9; color: white; text-align: center; line-height: 120px;",
                        "Sider"
                    }
                    Layout {
                        Header {
                            style: "background: #108ee9; color: white; text-align: center;",
                            "Header"
                        }
                        Content {
                            style: "background: #fff; margin: 24px 16px; padding: 24px; text-align: center;",
                            "Content"
                        }
                        Footer {
                            style: "background: #108ee9; color: white; text-align: center;",
                            "Footer"
                        }
                    }
                }
            }
        }
    }
}

/// 自定义触发器演示
#[component]
fn CustomTriggerDemo() -> Element {
    let mut collapsed = use_signal(|| false);

    rsx! {
        section {
            style: "margin-bottom: 32px;",
            h2 { "自定义触发器" }
            div {
                style: "margin-bottom: 16px;",
                button {
                    onclick: move |_| collapsed.set(!collapsed()),
                    style: "padding: 8px 16px; background: #1890ff; color: white; border: none; border-radius: 4px; cursor: pointer;",
                    if collapsed() { "展开" } else { "收起" }
                }
            }
            div {
                style: "height: 400px; border: 1px solid #d9d9d9;",
                Layout {
                    has_sider: true,
                    Sider {
                        collapsed: collapsed(),
                        width: 200,
                        collapsed_width: 80,
                        style: "background: #3ba0e9; color: white;",
                        div {
                            style: "padding: 24px; text-align: center;",
                            if collapsed() {
                                "S"
                            } else {
                                "Sider"
                            }
                        }
                    }
                    Layout {
                        Header {
                            style: "background: #108ee9; color: white; padding: 0 24px; display: flex; align-items: center;",
                            "Header"
                        }
                        Content {
                            style: "background: #fff; margin: 24px 16px; padding: 24px; text-align: center;",
                            "Content"
                        }
                    }
                }
            }
        }
    }
}

/// 响应式侧边栏演示
#[component]
fn ResponsiveSiderDemo() -> Element {
    let mut collapsed = use_signal(|| false);

    rsx! {
        section {
            style: "margin-bottom: 32px;",
            h2 { "响应式侧边栏" }
            div {
                style: "height: 400px; border: 1px solid #d9d9d9;",
                Layout {
                    has_sider: true,
                    Sider {
                        collapsed: collapsed(),
                        collapsible: true,
                        on_collapse: move |value| collapsed.set(value),
                        width: 200,
                        collapsed_width: 80,
                        theme: SiderTheme::Dark,
                        breakpoint: SiderBreakpoint::Lg,
                        on_breakpoint: move |broken| {
                            if broken {
                                collapsed.set(true);
                            }
                        },

                        // 侧边栏内容
                        div {
                            style: "padding: 24px;",

                            // 品牌区域
                            div {
                                style: "color: white; font-size: 18px; font-weight: bold; margin-bottom: 24px; text-align: center;",
                                if collapsed() {
                                    "A"
                                } else {
                                    "Ant Design"
                                }
                            }

                            // 菜单项
                            div {
                                style: "color: rgba(255, 255, 255, 0.65);",
                                div {
                                    style: "padding: 12px 0; border-bottom: 1px solid rgba(255, 255, 255, 0.1);",
                                    if collapsed() {
                                        "📊"
                                    } else {
                                        "📊 仪表盘"
                                    }
                                }
                                div {
                                    style: "padding: 12px 0; border-bottom: 1px solid rgba(255, 255, 255, 0.1);",
                                    if collapsed() {
                                        "👥"
                                    } else {
                                        "👥 用户管理"
                                    }
                                }
                                div {
                                    style: "padding: 12px 0;",
                                    if collapsed() {
                                        "⚙️"
                                    } else {
                                        "⚙️ 系统设置"
                                    }
                                }
                            }
                        }
                    }
                    Layout {
                        Header {
                            style: "background: #fff; padding: 0 24px; box-shadow: 0 1px 4px rgba(0,21,41,.08); display: flex; align-items: center;",
                            "管理后台"
                        }
                        Content {
                            style: "margin: 24px 16px; padding: 24px; background: #fff; border-radius: 6px;",
                            h3 { "主要内容区域" }
                            p { "这里是页面的主要内容，可以根据需要调整布局和样式。" }
                            p { "侧边栏支持响应式设计，在小屏幕设备上会自动收起。" }
                        }
                        Footer {
                            style: "text-align: center; color: rgba(0, 0, 0, 0.45);",
                            "Ant Design Dioxus ©2024 Created by Ant UED"
                        }
                    }
                }
            }
        }
    }
}

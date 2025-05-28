//! Ant Design Dioxus 组件演示
//!
//! 统一的组件演示入口，展示所有已实现的组件
//! 参照 Ant Design 官方文档格式，左侧展示组件列表，右侧展示组件使用示例和参数说明

use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

mod common;
mod components;

use common::*;
use components::*;

// 引用依赖以避免未使用警告
#[allow(unused_imports)]
use once_cell as _;
#[allow(unused_imports)]
use serde as _;
#[allow(unused_imports)]
use serde_json as _;
#[allow(unused_imports)]
use wasm_bindgen_test as _;

/// 主应用组件
#[component]
fn App() -> Element {
    let mut selected_component = use_signal(|| "Button".to_string());

    rsx! {
        div {
            class: "showcase-container",
            style: "display: flex; min-height: 100vh; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;",

            // 左侧导航栏
            div {
                class: "sidebar",
                style: "width: 240px; background: #f5f5f5; border-right: 1px solid #d9d9d9; padding: 16px; overflow-y: auto;",

                h2 {
                    style: "margin: 0 0 16px 0; color: #1890ff; font-size: 18px;",
                    "Ant Design Dioxus"
                }

                ComponentMenu {
                    selected: selected_component.read().clone(),
                    on_select: move |component: String| {
                        selected_component.set(component);
                    }
                }
            }

            // 右侧内容区域
            div {
                class: "content",
                style: "flex: 1; padding: 24px; overflow-y: auto;",

                match selected_component.read().as_str() {
                    "Avatar" => rsx! { AvatarDemo {} },
                    "Button" => rsx! { ButtonDemo {} },
                    "Icon" => rsx! { IconDemo {} },
                    "Typography" => rsx! { TypographyDemo {} },
                    "TypographyComplete" => rsx! { TypographyCompleteDemo {} },
                    "Grid" => rsx! { GridDemo {} },
                    "Layout" => rsx! { LayoutDemo {} },
                    "Flex" => rsx! { FlexDemo {} },
                    "Space" => rsx! { SpaceDemo {} },
                    "Divider" => rsx! { DividerDemo {} },
                    "Badge" => rsx! { BadgeDemo {} },
                    "Tag" => rsx! { TagDemo {} },
                    "Empty" => rsx! { components::empty::EmptyExample {} },
                    _ => rsx! {
                        div {
                            style: "text-align: center; padding: 48px; color: #999;",
                            h2 { "Ant Design Dioxus 组件演示" }
                            p { "从左侧菜单选择要查看的组件" }
                            div {
                                style: "margin-top: 24px; padding: 16px; background: #f5f5f5; border-radius: 8px; text-align: left; max-width: 600px; margin-left: auto; margin-right: auto;",
                                h3 { "可用组件:" }
                                ul {
                                    li { "Button - 按钮组件" }
                                    li { "Grid - 栅格布局" }
                                    li { "Icon - 图标组件" }
                                    li { "Typography - 排版组件" }
                                    li { "Layout - 布局组件" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    dioxus::launch(App);
}

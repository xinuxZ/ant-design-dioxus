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

use dioxus::logger::tracing::{debug, error, info, Level};

// 引用依赖以避免未使用警告
#[allow(unused_imports)]
use gloo_timers as _;
#[allow(unused_imports)]
use js_sys as _;
#[allow(unused_imports)]
use once_cell as _;
#[allow(unused_imports)]
use serde as _;
#[allow(unused_imports)]
use serde_json as _;

#[allow(unused_imports)]
use wasm_bindgen as _;
// #[allow(unused_imports)]
// use wasm_bindgen_test as _;
use css_in_rust as _;
use css_in_rust_macros as _;
use log as _;

/// 主应用组件
#[component]
fn App() -> Element {
    let mut selected_component = use_signal(|| "Button".to_string());

    rsx! {
        // 主题提供者包装整个应用
        ThemeProvider {
             initial_theme: ant_design_dioxus::theme::ThemePreset::dark(),

            div {
                class: "showcase-container",
                style: "display: flex; min-height: 100vh; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif; background-color: var(--ant-bg-color, #fff); color: var(--ant-text-color, rgba(0,0,0,0.85));",

                // 左侧导航栏
                div {
                    class: "sidebar",
                    style: "width: 240px; background: var(--ant-bg-color-container, #f5f5f5); border-right: 1px solid var(--ant-border-color, #d9d9d9); padding: 16px; overflow-y: auto;",

                    // 标题和主题切换器
                    div {
                        style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 16px;",

                        h2 {
                            style: "margin: 0; color: var(--ant-primary-color, #1890ff); font-size: 18px;",
                            "Ant Design Dioxus"
                        }

                        // 主题切换器
                        ThemeSwitch {
                            mode: ThemeSwitchMode::Switch,
                            size: "small".to_string(),
                            show_label: false,
                        }
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
                        "Grid" => rsx! { GridDemo{} },
                        "Layout" => rsx! { LayoutDemo{} },
                        "Space" => rsx! { SpaceDemo{} },
                        "Divider" => rsx! { DividerDemo{} },
                        "Flex" => rsx! { FlexDemo{} },
                        "Button" => rsx! { ButtonDemo{} },
                        "Icon" => rsx! { IconDemo{} },
                        "Typography" => rsx! { TypographyDemo{} },
                        "ThemeSwitch" => rsx! { ThemeSwitchDemo{} },
                        "Avatar" => rsx! { AvatarDemo{} },
                        "Badge" => rsx! { BadgeDemo{} },
                        "Card" => rsx! { CardDemo{} },
                        "Carousel" => rsx! { CarouselDemo{} },
                        "Collapse" => rsx! { CollapseDemo{} },
                        "Descriptions" => rsx! { DescriptionsDemo{} },
                        "Empty" => rsx! { EmptyDemo{} },
                        "Image" => rsx! { ImageDemo{} },
                        "List" => rsx! { ListDemo{} },
                        "Popover" => rsx! { PopoverDemo{} },
                        "Statistic" => rsx! { StatisticDemo{} },
                        "Table" => rsx! { TableDemo{} },
                        // "Tabs" => rsx! { TabsDemo{} },
                        "Tag" => rsx! { TagDemo{} },
                        "Timeline" => rsx! { TimelineDemo{} },
                        "Tooltip" => rsx! { TooltipDemo{} },
                        "Tree" => rsx! { TreeDemo{} },
                        "Affix" => rsx! { AffixDemo{} },
                        "Anchor" => rsx! { AnchorDemo{} },
                        "BackTop" => rsx! { BackTopDemo{} },
                        "Breadcrumb" => rsx! { BreadcrumbDemo{} },
                        "Dropdown" => rsx! { DropdownDemo{} },
                        "Menu" => rsx! { MenuDemo{} },
                        "Pagination" => rsx! { PaginationDemo{} },
                        "Steps" => rsx! { StepsDemo{} },
                        "Form" => rsx! { FormDemo{} },
                        "AutoComplete" => rsx! { AutoCompleteDemo{} },
                        "Cascader" => rsx! { CascaderDemo{} },
                        "Checkbox" => rsx! { CheckboxDemo{} },
                        "ColorPicker" => rsx! { ColorPickerDemo{} },
                        "DatePicker" => rsx! { DatePickerDemo{} },
                        "Input" => rsx! { InputDemo{} },
                        "InputNumber" => rsx! { InputNumberDemo{} },
                        "Mentions" => rsx! { MentionsDemo{} },
                        "Radio" => rsx! { RadioDemo{} },
                        "Rate" => rsx! { RateDemo{} },
                        "Select" => rsx! { SelectDemo{} },
                        "Slider" => rsx! { SliderDemo{} },
                        "Switch" => rsx! { SwitchDemo{} },
                        "Tabs" => rsx! { TabsDemo{} },
                        "TimePicker" => rsx! { TimePickerDemo{} },
                        "Transfer" => rsx! { TransferDemo{} },
                        "TreeSelect" => rsx! { TreeSelectDemo{} },
                        "Upload" => rsx! { UploadDemo{} },
                        "Alert" => rsx! { AlertDemo{} },
                        "Drawer" => rsx! { DrawerDemo{} },
                        "Message" => rsx! { MessageDemo{} },
                        "Modal" => rsx! { ModalDemo{} },
                        "Notification" => rsx! { NotificationDemo{} },
                        "Popconfirm" => rsx! { PopconfirmDemo{} },
                        "Progress" => rsx! { ProgressDemo{} },
                        "Result" => rsx! { ResultDemo{} },
                        "Skeleton" => rsx! { SkeletonDemo{} },
                        "Spin" => rsx! { SpinDemo{} },
                        "Tour" => rsx! { TourDemo{} },
                        "App" => rsx! { AppDemo{} },
                        "Calendar" => rsx! { CalendarDemo{} },
                        "FloatButton" => rsx! { FloatButtonDemo{} },
                        "QRCode" => rsx! { QRCodeDemo{} },
                        "Segmented" => rsx! { SegmentedDemo{} },
                        "Splitter" => rsx! { SplitterDemo{} },
                        "Watermark" => rsx! { WatermarkDemo{} },
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
                                        li { "Table - 表格组件" }
                                        li { "Tooltip - 文字提示" }
                                        li { "Tree - 树形控件" }
                                        li { "Breadcrumb - 面包屑" }
                                        li { "Menu - 导航菜单" }
                                    }
                                }
                            }
                        }
                }
                }
            }
        }
    }
}

/// 初始化 Ant Design CSS 变量系统
fn init_ant_design_theme() {
    use std::collections::HashMap;

    // 创建 Ant Design 主题变量
    let mut css_variables = HashMap::new();

    // 主要颜色
    css_variables.insert("--ant-primary-color".to_string(), "#1890ff".to_string());
    css_variables.insert("--ant-success-color".to_string(), "#52c41a".to_string());
    css_variables.insert("--ant-warning-color".to_string(), "#faad14".to_string());
    css_variables.insert("--ant-error-color".to_string(), "#ff4d4f".to_string());

    // 文本颜色
    css_variables.insert("--ant-text-color".to_string(), "#000000d9".to_string());
    css_variables.insert(
        "--ant-text-color-secondary".to_string(),
        "#00000073".to_string(),
    );
    css_variables.insert(
        "--ant-text-color-disabled".to_string(),
        "#00000040".to_string(),
    );

    // 边框和背景
    css_variables.insert("--ant-border-color".to_string(), "#d9d9d9".to_string());
    css_variables.insert("--ant-background-color".to_string(), "#ffffff".to_string());
    css_variables.insert(
        "--ant-background-color-light".to_string(),
        "#fafafa".to_string(),
    );

    // 尺寸
    css_variables.insert("--ant-border-radius".to_string(), "6px".to_string());
    css_variables.insert("--ant-padding-xs".to_string(), "4px".to_string());
    css_variables.insert("--ant-padding-sm".to_string(), "8px".to_string());
    css_variables.insert("--ant-padding-md".to_string(), "16px".to_string());
    css_variables.insert("--ant-padding-lg".to_string(), "24px".to_string());

    // 阴影
    css_variables.insert(
        "--ant-box-shadow".to_string(),
        "0 2px 8px rgba(0, 0, 0, 0.15)".to_string(),
    );
    css_variables.insert(
        "--ant-box-shadow-sm".to_string(),
        "0 2px 0 rgba(0, 0, 0, 0.045)".to_string(),
    );

    // 在 WASM 环境中注入 CSS 变量到 DOM
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsCast;
        use web_sys::{window, Document, Element, HtmlStyleElement};

        if let Some(window) = window() {
            if let Some(document) = window.document() {
                // 检查是否已经存在 CSS 变量样式
                if document
                    .query_selector("[data-ant-css-variables]")
                    .unwrap_or(None)
                    .is_none()
                {
                    // 创建 style 元素
                    if let Ok(style_element) = document.create_element("style") {
                        let style_element: HtmlStyleElement = style_element.unchecked_into();

                        // 构建 CSS 变量字符串
                        let mut css_content = String::from(":root {\n");
                        for (name, value) in &css_variables {
                            css_content.push_str(&format!("  {}: {};\n", name, value));
                        }
                        css_content.push_str("}\n");

                        // 设置样式内容
                        style_element.set_text_content(Some(&css_content));
                        style_element
                            .set_attribute("data-ant-css-variables", "true")
                            .ok();

                        // 添加到 head
                        if let Some(head) = document.head() {
                            head.append_child(&style_element).ok();
                            web_sys::console::log_1(&"✅ Ant Design CSS 变量已注入到 DOM".into());
                        }
                    }
                }
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        println!("📝 Ant Design CSS 变量 (非 WASM 环境):");
        for (name, value) in &css_variables {
            println!("  {}: {}", name, value);
        }
    }
}

fn main() {
    // 初始化基础 CSS 系统
    css_in_rust::init();

    // 初始化 Ant Design 主题系统
    init_ant_design_theme();

    dioxus::logger::init(Level::DEBUG).expect("logger failed to init");
    info!("App rendered");

    error!("This is an error");
    info!("This is an info");
    debug!("This is a debug");
    dioxus::launch(App);
}

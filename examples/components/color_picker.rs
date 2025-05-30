use crate::common::demo_section::DemoSection;
use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

/// ColorPicker组件示例
#[component]
pub fn ColorPickerDemo() -> Element {
    rsx! {
        div { class: "demo-container",
            h1 { "ColorPicker 颜色选择器" }
            p { "提供颜色选取的组件。" }

            DemoSection {
                title: "基本使用",
                description: "最简单的用法。",

                div {
                    ColorPicker {
                        class: "demo-color-picker"
                    }
                }
            }

            DemoSection {
                title: "不同尺寸",
                description: "三种尺寸的颜色选择器。",

                div {
                    div { style: "display: flex; gap: 16px; align-items: center;",
                        ColorPicker { class: "color-picker-small", style: "width: 24px; height: 24px;" }
                        ColorPicker { class: "color-picker-default", style: "width: 32px; height: 32px;" }
                        ColorPicker { class: "color-picker-large", style: "width: 40px; height: 40px;" }
                    }
                }
            }

            DemoSection {
                title: "预设颜色",
                description: "可以预设一些常用的颜色。",

                div {
                    ColorPicker {
                        class: "color-picker-presets",
                        style: "width: 40px; height: 40px;"
                    }
                }
            }

            DemoSection {
                title: "禁用状态",
                description: "禁用状态的颜色选择器。",

                div {
                    ColorPicker {
                        class: "color-picker-disabled",
                        style: "width: 32px; height: 32px; opacity: 0.5; cursor: not-allowed;"
                    }
                }
            }

            DemoSection {
                title: "自定义触发器",
                description: "自定义颜色选择器的触发器样式。",

                div {
                    ColorPicker {
                        class: "color-picker-custom",
                        style: "width: 100px; height: 32px; border-radius: 16px; border: 2px solid #1677ff;"
                    }
                }
            }
        }
    }
}

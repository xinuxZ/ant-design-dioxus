use crate::common::demo_section::DemoSection;
use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

/// Calendar组件示例
#[component]
pub fn CalendarDemo() -> Element {
    rsx! {
        div { class: "demo-container",
            h1 { "Calendar 日历" }
            p { "按照日历形式展示数据的容器。" }

            DemoSection {
                title: "基本使用",
                description: "一个通用的日历面板，支持年/月切换。",

                div {
                    Calendar {
                        class: "demo-calendar",
                        style: "border: 1px solid #d9d9d9; border-radius: 8px;"
                    }
                }
            }

            DemoSection {
                title: "卡片模式",
                description: "用于嵌套在空间有限的容器中。",

                div {
                    Calendar {
                        class: "calendar-card",
                        style: "width: 300px; border: 1px solid #d9d9d9; border-radius: 8px;"
                    }
                }
            }

            DemoSection {
                title: "自定义样式",
                description: "可以通过 class 和 style 属性自定义日历的外观。",

                div {
                    Calendar {
                        class: "custom-calendar",
                        style: "background: #f5f5f5; padding: 16px; border-radius: 8px;"
                    }
                }
            }

            DemoSection {
                title: "紧凑模式",
                description: "紧凑型的日历，适用于空间较小的场景。",

                div {
                    Calendar {
                        class: "calendar-compact",
                        style: "width: 280px; border: 1px solid #d9d9d9; border-radius: 8px;"
                    }
                }
            }
        }
    }
}

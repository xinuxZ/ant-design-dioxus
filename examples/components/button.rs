//! Button 组件演示
//!
//! 展示 Button 组件的各种用法和样式

use crate::common::*;
use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

/// Button 组件演示
#[component]
pub fn ButtonDemo() -> Element {
    rsx! {
        div {
            class: "component-showcase",

            h1 {
                style: "margin: 0 0 16px 0; color: #262626; font-size: 28px;",
                "Button 按钮"
            }

            p {
                style: "margin: 0 0 24px 0; color: #666; font-size: 14px;",
                "按钮用于开始一个即时操作。"
            }

            // 基础用法
            DemoSection {
                title: "基础用法",
                description: "按钮有五种类型：主按钮、次按钮、虚线按钮、文本按钮和链接按钮。",

                div {
                    style: "display: flex; gap: 8px; flex-wrap: wrap;",

                    Button {
                        button_type: ButtonType::Primary,
                        onclick: move |_| {
                            web_sys::console::log_1(&"Primary button clicked!".into());
                        },
                        "Primary Button"
                    }

                    Button {
                        button_type: ButtonType::Default,
                        onclick: move |_| {
                            web_sys::console::log_1(&"Default button clicked!".into());
                        },
                        "Default Button"
                    }

                    Button {
                        button_type: ButtonType::Dashed,
                        onclick: move |_| {
                            web_sys::console::log_1(&"Dashed button clicked!".into());
                        },
                        "Dashed Button"
                    }

                    Button {
                        button_type: ButtonType::Text,
                        onclick: move |_| {
                            web_sys::console::log_1(&"Text button clicked!".into());
                        },
                        "Text Button"
                    }

                    Button {
                        button_type: ButtonType::Link,
                        onclick: move |_| {
                            web_sys::console::log_1(&"Link button clicked!".into());
                        },
                        "Link Button"
                    }
                }
            }

            // 不同尺寸
            DemoSection {
                title: "三种尺寸",
                description: "按钮有大、中、小三种尺寸。",

                div {
                    style: "display: flex; gap: 8px; align-items: center;",

                    Button {
                        button_type: ButtonType::Primary,
                        size: ButtonSize::Large,
                        "Large"
                    }

                    Button {
                        button_type: ButtonType::Primary,
                        size: ButtonSize::Middle,
                        "Middle"
                    }

                    Button {
                        button_type: ButtonType::Primary,
                        size: ButtonSize::Small,
                        "Small"
                    }
                }
            }

            // 不可用状态
            DemoSection {
                title: "不可用状态",
                description: "添加 disabled 属性即可让按钮处于不可用状态。",

                div {
                    style: "display: flex; gap: 8px;",

                    Button {
                        button_type: ButtonType::Primary,
                        disabled: true,
                        "Primary(disabled)"
                    }

                    Button {
                        button_type: ButtonType::Default,
                        disabled: true,
                        "Default(disabled)"
                    }
                }
            }

            // 加载状态
            DemoSection {
                title: "加载状态",
                description: "添加 loading 属性即可让按钮处于加载状态。",

                div {
                    style: "display: flex; gap: 8px;",

                    Button {
                        button_type: ButtonType::Primary,
                        loading: true,
                        "Loading"
                    }

                    Button {
                        button_type: ButtonType::Default,
                        loading: true,
                        "Loading"
                    }
                }
            }

            // 危险按钮
            DemoSection {
                title: "危险按钮",
                description: "删除/移动/修改权限等危险操作，一般需要二次确认。",

                div {
                    style: "display: flex; gap: 8px;",

                    Button {
                        button_type: ButtonType::Primary,
                        danger: true,
                        "Primary Danger"
                    }

                    Button {
                        button_type: ButtonType::Default,
                        danger: true,
                        "Default Danger"
                    }
                }
            }

            // 按钮形状
            DemoSection {
                title: "按钮形状",
                description: "按钮有默认、圆形和圆角矩形三种形状。",

                div {
                    style: "display: flex; gap: 8px; align-items: center;",

                    Button {
                        button_type: ButtonType::Primary,
                        "Default"
                    }

                    Button {
                        button_type: ButtonType::Primary,
                        shape: ButtonShape::Circle,
                        "A"
                    }

                    Button {
                        button_type: ButtonType::Primary,
                        shape: ButtonShape::Round,
                        "Round"
                    }
                }
            }

            // 块级按钮
            DemoSection {
                title: "块级按钮",
                description: "block 属性将使按钮适合其父宽度。",

                div {
                    style: "display: flex; flex-direction: column; gap: 8px;",

                    Button {
                        button_type: ButtonType::Primary,
                        block: true,
                        "Primary Block Button"
                    }

                    Button {
                        button_type: ButtonType::Default,
                        block: true,
                        "Default Block Button"
                    }
                }
            }

            // API 文档
            ApiDocumentation {
                component_name: "Button",
                props: vec![
                    PropDoc {
                        name: "button_type".to_string(),
                        prop_type: "ButtonType".to_string(),
                        default: "ButtonType::Default".to_string(),
                        description: "设置按钮类型".to_string(),
                    },
                    PropDoc {
                        name: "size".to_string(),
                        prop_type: "ButtonSize".to_string(),
                        default: "ButtonSize::Middle".to_string(),
                        description: "设置按钮大小".to_string(),
                    },
                    PropDoc {
                        name: "shape".to_string(),
                        prop_type: "ButtonShape".to_string(),
                        default: "ButtonShape::Default".to_string(),
                        description: "设置按钮形状".to_string(),
                    },
                    PropDoc {
                        name: "disabled".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "按钮失效状态".to_string(),
                    },
                    PropDoc {
                        name: "danger".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "设置危险按钮".to_string(),
                    },
                    PropDoc {
                        name: "loading".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "设置按钮载入状态".to_string(),
                    },
                    PropDoc {
                        name: "block".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "将按钮宽度调整为其父宽度的选项".to_string(),
                    },
                ]
            }
        }
    }
}

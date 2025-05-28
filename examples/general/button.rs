//! Button 组件示例
//!
//! 展示 Button 组件的各种用法和样式

use crate::common::{ApiDocumentation, DemoSection, PropDoc};
use ant_design_dioxus::{Button, ButtonShape, ButtonSize, ButtonType};
use dioxus::prelude::*;

/// Button 组件演示页面
#[component]
pub fn ButtonDemo() -> Element {
    rsx! {
        div {
            class: "button-demo",
            style: "padding: 24px;",

            h1 {
                style: "margin: 0 0 24px 0; color: #262626; font-size: 28px;",
                "Button 按钮"
            }

            p {
                style: "margin: 0 0 32px 0; color: #666; font-size: 14px;",
                "按钮用于开始一个即时操作。"
            }

            // 基础按钮
            DemoSection {
                title: "基础按钮".to_string(),
                description: "按钮有五种类型：主按钮、次按钮、虚线按钮、文本按钮和链接按钮。主按钮在同一个操作区域最多出现一次。".to_string(),
                div {
                    style: "display: flex; gap: 8px; flex-wrap: wrap;",

                    Button {
                        button_type: ButtonType::Primary,
                        onclick: move |_| {
                            web_sys::console::log_1(&"Primary button clicked!".into());
                        },
                        "Primary"
                    }

                    Button {
                        button_type: ButtonType::Default,
                        onclick: move |_| {
                            web_sys::console::log_1(&"Default button clicked!".into());
                        },
                        "Default"
                    }

                    Button {
                        button_type: ButtonType::Dashed,
                        onclick: move |_| {
                            web_sys::console::log_1(&"Dashed button clicked!".into());
                        },
                        "Dashed"
                    }

                    Button {
                        button_type: ButtonType::Text,
                        onclick: move |_| {
                            web_sys::console::log_1(&"Text button clicked!".into());
                        },
                        "Text"
                    }

                    Button {
                        button_type: ButtonType::Link,
                        onclick: move |_| {
                            web_sys::console::log_1(&"Link button clicked!".into());
                        },
                        "Link"
                    }
                }
            }

            // 按钮尺寸
            DemoSection {
                title: "按钮尺寸".to_string(),
                description: "按钮有大、中、小三种尺寸。通过设置 size 为 large small 分别把按钮设为大、小尺寸。若不设置 size，则尺寸为中。".to_string(),
                div {
                    style: "display: flex; gap: 8px; align-items: center; flex-wrap: wrap;",

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

            // 按钮形状
            DemoSection {
                title: "按钮形状".to_string(),
                description: "按钮有默认、圆角和圆形三种形状。".to_string(),
                div {
                    style: "display: flex; gap: 8px; align-items: center; flex-wrap: wrap;",

                    Button {
                        button_type: ButtonType::Primary,
                        shape: ButtonShape::Default,
                        "Default"
                    }

                    Button {
                        button_type: ButtonType::Primary,
                        shape: ButtonShape::Round,
                        "Round"
                    }

                    Button {
                        button_type: ButtonType::Primary,
                        shape: ButtonShape::Circle,
                        "A"
                    }
                }
            }

            // 危险按钮
            DemoSection {
                title: "危险按钮".to_string(),
                description: "删除/移动/修改权限等危险操作，一般需要二次确认。".to_string(),
                div {
                    style: "display: flex; gap: 8px; flex-wrap: wrap;",

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

                    Button {
                        button_type: ButtonType::Dashed,
                        danger: true,
                        "Dashed Danger"
                    }

                    Button {
                        button_type: ButtonType::Text,
                        danger: true,
                        "Text Danger"
                    }

                    Button {
                        button_type: ButtonType::Link,
                        danger: true,
                        "Link Danger"
                    }
                }
            }

            // 禁用状态
            DemoSection {
                title: "禁用状态".to_string(),
                description: "添加 disabled 属性即可让按钮处于不可用状态，同时按钮样式也会改变。".to_string(),
                div {
                    style: "display: flex; gap: 8px; flex-wrap: wrap;",

                    Button {
                        button_type: ButtonType::Primary,
                        disabled: true,
                        "Primary Disabled"
                    }

                    Button {
                        button_type: ButtonType::Default,
                        disabled: true,
                        "Default Disabled"
                    }

                    Button {
                        button_type: ButtonType::Dashed,
                        disabled: true,
                        "Dashed Disabled"
                    }

                    Button {
                        button_type: ButtonType::Text,
                        disabled: true,
                        "Text Disabled"
                    }

                    Button {
                        button_type: ButtonType::Link,
                        disabled: true,
                        "Link Disabled"
                    }
                }
            }

            // 加载状态
            DemoSection {
                title: "加载状态".to_string(),
                description: "添加 loading 属性即可让按钮处于加载状态，最后两个按钮演示点击后进入加载状态。".to_string(),
                div {
                    style: "display: flex; gap: 8px; flex-wrap: wrap;",

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

            // 块级按钮
            DemoSection {
                title: "块级按钮".to_string(),
                description: "block 属性将使按钮适合其父宽度。".to_string(),
                div {
                    style: "width: 300px;",

                    Button {
                        button_type: ButtonType::Primary,
                        block: true,
                        "Block Button"
                    }
                }
            }

            // 幽灵按钮
            DemoSection {
                title: "幽灵按钮".to_string(),
                description: "幽灵按钮将按钮的内容反色，背景变为透明，常用在有色背景上。".to_string(),
                div {
                    style: "padding: 20px; background: #1677ff; border-radius: 8px;",
                    div {
                        style: "display: flex; gap: 8px; flex-wrap: wrap;",

                        Button {
                            button_type: ButtonType::Primary,
                            ghost: true,
                            "Primary Ghost"
                        }

                        Button {
                            button_type: ButtonType::Default,
                            ghost: true,
                            "Default Ghost"
                        }

                        Button {
                            button_type: ButtonType::Dashed,
                            ghost: true,
                            "Dashed Ghost"
                        }
                    }
                }
            }

            // API 文档
            ApiDocumentation {
                component_name: "Button".to_string(),
                props: vec![
                    PropDoc {
                        name: "button_type".to_string(),
                        prop_type: "ButtonType".to_string(),
                        default: "Default".to_string(),
                        description: "设置按钮类型".to_string(),
                    },
                    PropDoc {
                        name: "size".to_string(),
                        prop_type: "ButtonSize".to_string(),
                        default: "Middle".to_string(),
                        description: "设置按钮大小".to_string(),
                    },
                    PropDoc {
                        name: "shape".to_string(),
                        prop_type: "ButtonShape".to_string(),
                        default: "Default".to_string(),
                        description: "设置按钮形状".to_string(),
                    },
                    PropDoc {
                        name: "disabled".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "按钮失效状态".to_string(),
                    },
                    PropDoc {
                        name: "loading".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "设置按钮载入状态".to_string(),
                    },
                    PropDoc {
                        name: "danger".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "设置危险按钮".to_string(),
                    },
                    PropDoc {
                        name: "ghost".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "幽灵属性，使按钮背景透明".to_string(),
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

//! Icon 组件示例
//!
//! 展示 Icon 组件的各种用法和样式

use crate::common::{ApiDocumentation, DemoSection, PropDoc};
use ant_design_dioxus::{Icon, IconRotate, IconTheme};
use dioxus::prelude::*;

/// Icon 组件演示页面
#[component]
pub fn IconDemo() -> Element {
    rsx! {
        div {
            class: "icon-demo",
            style: "padding: 24px;",

            h1 {
                style: "margin: 0 0 24px 0; color: #262626; font-size: 28px;",
                "Icon 图标"
            }

            p {
                style: "margin: 0 0 32px 0; color: #666; font-size: 14px;",
                "语义化的矢量图形。使用图标组件，你需要安装 @ant-design/icons 图标组件包。"
            }

            // 基础图标
            DemoSection {
                title: "基础图标".to_string(),
                description: "最简单的用法，直接使用图标名称。".to_string(),
                div {
                    style: "display: flex; gap: 16px; flex-wrap: wrap; align-items: center;",

                    Icon {
                        icon_type: "home".to_string(),
                        style: "font-size: 24px;".to_string(),
                    }

                    Icon {
                        icon_type: "user".to_string(),
                        style: "font-size: 24px;".to_string(),
                    }

                    Icon {
                        icon_type: "setting".to_string(),
                        style: "font-size: 24px;".to_string(),
                    }

                    Icon {
                        icon_type: "heart".to_string(),
                        style: "font-size: 24px;".to_string(),
                    }

                    Icon {
                        icon_type: "star".to_string(),
                        style: "font-size: 24px;".to_string(),
                    }
                }
            }

            // 图标主题
            DemoSection {
                title: "图标主题".to_string(),
                description: "图标有三种主题：线框风格、实底风格、双色风格。".to_string(),
                div {
                    style: "display: flex; gap: 24px; flex-wrap: wrap;",

                    div {
                        style: "display: flex; flex-direction: column; gap: 8px; align-items: center;",
                        Icon {
                            icon_type: "home".to_string(),
                            theme: IconTheme::Outlined,
                            style: "font-size: 24px;".to_string(),
                        }
                        span { style: "font-size: 12px; color: #666;", "Outlined" }
                    }

                    div {
                        style: "display: flex; flex-direction: column; gap: 8px; align-items: center;",
                        Icon {
                            icon_type: "home".to_string(),
                            theme: IconTheme::Filled,
                            style: "font-size: 24px;".to_string(),
                        }
                        span { style: "font-size: 12px; color: #666;", "Filled" }
                    }

                    div {
                        style: "display: flex; flex-direction: column; gap: 8px; align-items: center;",
                        Icon {
                            icon_type: "home".to_string(),
                            theme: IconTheme::TwoTone,
                            style: "font-size: 24px;".to_string(),
                        }
                        span { style: "font-size: 12px; color: #666;", "TwoTone" }
                    }
                }
            }

            // 图标尺寸
            DemoSection {
                title: "图标尺寸".to_string(),
                description: "通过 style 属性设置图标的尺寸。".to_string(),
                div {
                    style: "display: flex; gap: 16px; align-items: center; flex-wrap: wrap;",

                    Icon {
                        icon_type: "star".to_string(),
                        style: "font-size: 16px;".to_string(),
                    }

                    Icon {
                        icon_type: "star".to_string(),
                        style: "font-size: 24px;".to_string(),
                    }

                    Icon {
                        icon_type: "star".to_string(),
                        style: "font-size: 32px;".to_string(),
                    }

                    Icon {
                        icon_type: "star".to_string(),
                        style: "font-size: 48px;".to_string(),
                    }
                }
            }

            // 图标颜色
            DemoSection {
                title: "图标颜色".to_string(),
                description: "通过 style 属性设置图标的颜色。".to_string(),
                div {
                    style: "display: flex; gap: 16px; align-items: center; flex-wrap: wrap;",

                    Icon {
                        icon_type: "heart".to_string(),
                        style: "font-size: 24px; color: #ff4d4f;".to_string(),
                    }

                    Icon {
                        icon_type: "star".to_string(),
                        style: "font-size: 24px; color: #faad14;".to_string(),
                    }

                    Icon {
                        icon_type: "like".to_string(),
                        style: "font-size: 24px; color: #52c41a;".to_string(),
                    }

                    Icon {
                        icon_type: "message".to_string(),
                        style: "font-size: 24px; color: #1890ff;".to_string(),
                    }

                    Icon {
                        icon_type: "crown".to_string(),
                        style: "font-size: 24px; color: #722ed1;".to_string(),
                    }
                }
            }

            // 图标旋转
            DemoSection {
                title: "图标旋转".to_string(),
                description: "通过 rotate 属性设置图标的旋转角度。".to_string(),
                div {
                    style: "display: flex; gap: 16px; align-items: center; flex-wrap: wrap;",

                    Icon {
                        icon_type: "reload".to_string(),
                        rotate: IconRotate::Rotate90,
                        style: "font-size: 24px;".to_string(),
                    }

                    Icon {
                        icon_type: "reload".to_string(),
                        rotate: IconRotate::Rotate180,
                        style: "font-size: 24px;".to_string(),
                    }

                    Icon {
                        icon_type: "reload".to_string(),
                        rotate: IconRotate::Rotate270,
                        style: "font-size: 24px;".to_string(),
                    }
                }
            }

            // 常用图标
            DemoSection {
                title: "常用图标".to_string(),
                description: "展示一些常用的图标。".to_string(),
                div {
                    style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(120px, 1fr)); gap: 16px;",

                    for icon_name in [
                        "home", "user", "setting", "search", "plus", "minus",
                        "edit", "delete", "copy", "save", "download", "upload",
                        "left", "right", "up", "down", "close", "check",
                        "info", "warning", "error", "success", "question", "exclamation"
                    ] {
                        div {
                            style: "display: flex; flex-direction: column; align-items: center; gap: 8px; padding: 12px; border: 1px solid #f0f0f0; border-radius: 4px; cursor: pointer; transition: all 0.2s;",
                            onmouseenter: move |_| {},
                            onmouseleave: move |_| {},

                            Icon {
                                icon_type: icon_name.to_string(),
                                style: "font-size: 20px;".to_string(),
                            }

                            span {
                                style: "font-size: 12px; color: #666; text-align: center;",
                                "{icon_name}"
                            }
                        }
                    }
                }
            }

            // API 文档
            ApiDocumentation {
                component_name: "Icon".to_string(),
                props: vec![
                    PropDoc {
                        name: "icon_type".to_string(),
                        prop_type: "String".to_string(),
                        default: "-".to_string(),
                        description: "图标类型".to_string(),
                    },
                    PropDoc {
                        name: "theme".to_string(),
                        prop_type: "IconTheme".to_string(),
                        default: "Outlined".to_string(),
                        description: "图标主题风格".to_string(),
                    },
                    PropDoc {
                        name: "rotate".to_string(),
                        prop_type: "IconRotate".to_string(),
                        default: "None".to_string(),
                        description: "图标旋转角度".to_string(),
                    },
                    PropDoc {
                        name: "spin".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "是否有旋转动画".to_string(),
                    },
                    PropDoc {
                        name: "style".to_string(),
                        prop_type: "String".to_string(),
                        default: "".to_string(),
                        description: "设置图标的样式，例如 fontSize 和 color".to_string(),
                    },
                ]
            }
        }
    }
}

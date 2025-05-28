//! Icon 组件演示
//!
//! 展示 Icon 组件的各种用法，包括不同类型、主题、尺寸的图标

use crate::common::*;
use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

/// Icon 组件演示
#[component]
pub fn IconDemo() -> Element {
    rsx! {
        div {
            class: "component-showcase",

            h1 {
                style: "margin: 0 0 16px 0; color: #262626; font-size: 28px;",
                "Icon 图标"
            }

            p {
                style: "margin: 0 0 24px 0; color: #666; font-size: 14px;",
                "语义化的矢量图形。使用图标组件，你需要安装 @ant-design/icons 图标组件包。"
            }

            // 基础用法
            DemoSection {
                title: "基础用法",
                description: "最简单的用法。",

                div {
                    style: "display: flex; gap: 16px; align-items: center;",

                    Icon {
                        icon_type: "home",
                        theme: IconTheme::Outlined,
                    }

                    Icon {
                        icon_type: "user",
                        theme: IconTheme::Outlined,
                    }

                    Icon {
                        icon_type: "setting",
                        theme: IconTheme::Outlined,
                    }

                    Icon {
                        icon_type: "search",
                        theme: IconTheme::Outlined,
                    }

                    Icon {
                        icon_type: "heart",
                        theme: IconTheme::Outlined,
                    }

                    Icon {
                        icon_type: "star",
                        theme: IconTheme::Outlined,
                    }
                }
            }

            // 不同主题
            DemoSection {
                title: "图标主题",
                description: "图标有三种主题：线框风格、实心风格和双色风格。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    div {
                        h4 { "Outlined (线框风格)" }
                        div {
                            style: "display: flex; gap: 16px; align-items: center;",

                            Icon {
                                icon_type: "home",
                                theme: IconTheme::Outlined,
                            }

                            Icon {
                                icon_type: "user",
                                theme: IconTheme::Outlined,
                            }

                            Icon {
                                icon_type: "setting",
                                theme: IconTheme::Outlined,
                            }
                        }
                    }

                    div {
                        h4 { "Filled (实心风格)" }
                        div {
                            style: "display: flex; gap: 16px; align-items: center;",

                            Icon {
                                icon_type: "home",
                                theme: IconTheme::Filled,
                            }

                            Icon {
                                icon_type: "user",
                                theme: IconTheme::Filled,
                            }

                            Icon {
                                icon_type: "setting",
                                theme: IconTheme::Filled,
                            }
                        }
                    }

                    div {
                        h4 { "TwoTone (双色风格)" }
                        div {
                            style: "display: flex; gap: 16px; align-items: center;",

                            Icon {
                                icon_type: "home",
                                theme: IconTheme::TwoTone,
                            }

                            Icon {
                                icon_type: "user",
                                theme: IconTheme::TwoTone,
                            }

                            Icon {
                                icon_type: "setting",
                                theme: IconTheme::TwoTone,
                            }
                        }
                    }
                }
            }

            // 不同尺寸
            DemoSection {
                title: "图标尺寸",
                description: "图标支持不同的尺寸。",

                div {
                    style: "display: flex; gap: 16px; align-items: center;",

                    Icon {
                        icon_type: "home",
                        theme: IconTheme::Outlined,
                        // size: IconSize::Small,
                    }

                    Icon {
                        icon_type: "home",
                        theme: IconTheme::Outlined,
                        // size: IconSize::Medium,
                    }

                    Icon {
                        icon_type: "home",
                        theme: IconTheme::Outlined,
                        // size: IconSize::Large,
                    }
                }
            }

            // 自定义颜色
            DemoSection {
                title: "自定义颜色",
                description: "可以通过 color 属性自定义图标颜色。",

                div {
                    style: "display: flex; gap: 16px; align-items: center;",

                    Icon {
                        icon_type: "heart",
                        theme: IconTheme::Filled,
                        color: "#ff4d4f",
                    }

                    Icon {
                        icon_type: "star",
                        theme: IconTheme::Filled,
                        color: "#faad14",
                    }

                    Icon {
                        icon_type: "like",
                        theme: IconTheme::Filled,
                        color: "#52c41a",
                    }

                    Icon {
                        icon_type: "message",
                        theme: IconTheme::Filled,
                        color: "#1890ff",
                    }
                }
            }

            // 旋转动画
            DemoSection {
                title: "旋转动画",
                description: "设置 spin 属性可以让图标旋转。",

                div {
                    style: "display: flex; gap: 16px; align-items: center;",

                    Icon {
                        icon_type: "loading",
                        theme: IconTheme::Outlined,
                        spin: true,
                    }

                    Icon {
                        icon_type: "sync",
                        theme: IconTheme::Outlined,
                        spin: true,
                    }
                }
            }

            // API 文档
            ApiDocumentation {
                component_name: "Icon",
                props: vec![
                    PropDoc {
                        name: "icon_type".to_string(),
                        prop_type: "&str".to_string(),
                        default: "-".to_string(),
                        description: "图标类型".to_string(),
                    },
                    PropDoc {
                        name: "theme".to_string(),
                        prop_type: "IconTheme".to_string(),
                        default: "IconTheme::Outlined".to_string(),
                        description: "图标主题风格".to_string(),
                    },
                    PropDoc {
                        name: "size".to_string(),
                        prop_type: "IconSize".to_string(),
                        default: "IconSize::Medium".to_string(),
                        description: "图标大小".to_string(),
                    },
                    PropDoc {
                        name: "color".to_string(),
                        prop_type: "Option<&str>".to_string(),
                        default: "None".to_string(),
                        description: "图标颜色".to_string(),
                    },
                    PropDoc {
                        name: "spin".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "是否有旋转动画".to_string(),
                    },
                ]
            }
        }
    }
}

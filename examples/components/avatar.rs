//! Avatar 头像组件演示

use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

use crate::common::*;

/// Avatar 组件演示
#[component]
pub fn AvatarDemo() -> Element {
    rsx! {
        div {
            class: "component-showcase",

            h1 {
                style: "margin: 0 0 16px 0; color: #262626; font-size: 28px;",
                "Avatar 头像"
            }

            p {
                style: "margin: 0 0 24px 0; color: #666; font-size: 14px;",
                "用来代表用户或事物，支持图片、图标以及字符展示。"
            }

            // 基础用法
            DemoSection {
                title: "基础用法",
                description: "头像有三种尺寸，两种形状可选。",

                div {
                    style: "display: flex; gap: 16px; align-items: center;",

                    Avatar {
                        size: AvatarSize::Large,
                        "U"
                    }
                    Avatar {
                        "U"
                    }
                    Avatar {
                        size: AvatarSize::Small,
                        "U"
                    }
                    Avatar {
                        shape: AvatarShape::Square,
                        size: AvatarSize::Large,
                        "U"
                    }
                    Avatar {
                        shape: AvatarShape::Square,
                        "U"
                    }
                    Avatar {
                        shape: AvatarShape::Square,
                        size: AvatarSize::Small,
                        "U"
                    }
                }
            }

            // 类型
            DemoSection {
                title: "类型",
                description: "支持三种类型：图片、Icon 以及字符，其中 Icon 和字符型可以自定义图标颜色及背景色。",

                div {
                    style: "display: flex; gap: 16px; align-items: center;",

                    Avatar {
                        icon: "👤",
                    }
                    Avatar {
                        "U"
                    }
                    Avatar {
                        "USER"
                    }
                    Avatar {
                        src: "https://api.dicebear.com/7.x/miniavs/svg?seed=1",
                        alt: "头像",
                    }
                    Avatar {
                        style: "color: #f56a00; background-color: #fde3cf;",
                        "U"
                    }
                    Avatar {
                        style: "background-color: #87d068;",
                        icon: "👤",
                    }
                }
            }

            // 带徽标的头像
            DemoSection {
                title: "带徽标的头像",
                description: "通常用于消息提示。",

                div {
                    style: "display: flex; gap: 16px; align-items: center;",

                    div {
                        style: "position: relative; display: inline-block;",
                        Avatar {
                            shape: AvatarShape::Square,
                            size: AvatarSize::Large,
                            icon: "👤",
                        }
                        span {
                            style: "position: absolute; top: 0; right: 0; width: 10px; height: 10px; background: #ff4d4f; border: 2px solid #fff; border-radius: 50%;",
                        }
                    }
                    div {
                        style: "position: relative; display: inline-block;",
                        Avatar {
                            shape: AvatarShape::Square,
                            icon: "👤",
                        }
                        span {
                            style: "position: absolute; top: 0; right: 0; width: 8px; height: 8px; background: #ff4d4f; border: 2px solid #fff; border-radius: 50%;",
                        }
                    }
                }
            }

            // 响应式尺寸
            DemoSection {
                title: "响应式尺寸",
                description: "头像大小可以根据屏幕大小自动调整。",

                div {
                    style: "display: flex; gap: 16px; align-items: center;",

                    Avatar {
                        size: AvatarSize::Custom(64),
                        style: "background-color: #1890ff;",
                        "64"
                    }
                    Avatar {
                        size: AvatarSize::Custom(48),
                        style: "background-color: #52c41a;",
                        "48"
                    }
                    Avatar {
                        size: AvatarSize::Custom(40),
                        style: "background-color: #faad14;",
                        "40"
                    }
                    Avatar {
                        size: AvatarSize::Custom(32),
                        style: "background-color: #f5222d;",
                        "32"
                    }
                }
            }

            // 头像组
            DemoSection {
                title: "头像组",
                description: "头像组合展示。",

                div {
                    style: "display: flex; gap: 24px; align-items: center; flex-direction: column;",

                    div {
                        class: "ant-avatar-group",
                        style: "display: inline-flex;",
                        Avatar {
                            "A"
                        }
                        Avatar {
                            "B"
                        }
                        Avatar {
                            "C"
                        }
                        Avatar {
                            "D"
                        }
                    }

                    div {
                        class: "ant-avatar-group",
                        style: "display: inline-flex;",
                        Avatar {
                            style: "background-color: #f56a00;",
                            "A"
                        }
                        Avatar {
                            style: "background-color: #7265e6;",
                            "B"
                        }
                        Avatar {
                            style: "background-color: #ffbf00;",
                            "C"
                        }
                        Avatar {
                            style: "background-color: #00a2ae;",
                            "+2"
                        }
                    }
                }
            }

            // 交互式头像
            DemoSection {
                title: "交互式头像",
                description: "点击头像触发事件。",

                div {
                    style: "display: flex; gap: 16px; align-items: center;",

                    Avatar {
                        onclick: move |_| {
                            web_sys::console::log_1(&"点击了头像".into());
                        },
                        style: "cursor: pointer; background-color: #1890ff;",
                        "点击"
                    }
                    Avatar {
                        src: "https://api.dicebear.com/7.x/miniavs/svg?seed=2",
                        alt: "可点击头像",
                        onclick: move |_| {
                            web_sys::console::log_1(&"点击了图片头像".into());
                        },
                        style: "cursor: pointer;",
                    }
                }
            }

            // 图片加载失败
            DemoSection {
                title: "图片加载失败",
                description: "图片加载失败时显示备用内容。",

                div {
                    style: "display: flex; gap: 16px; align-items: center;",

                    Avatar {
                        src: "https://invalid-url.com/avatar.jpg",
                        alt: "加载失败",
                        on_error: move |_| {
                            web_sys::console::log_1(&"图片加载失败".into());
                        },
                        "Fallback"
                    }
                    Avatar {
                        src: "https://invalid-url.com/avatar2.jpg",
                        icon: "👤",
                        on_error: move |_| {
                            web_sys::console::log_1(&"图片加载失败，显示图标".into());
                        },
                    }
                }
            }

            // API 文档
            ApiDocumentation {
                component_name: "Avatar",
                props: vec![
                    PropDoc {
                        name: "src".to_string(),
                        prop_type: "Option<String>".to_string(),
                        default: "None".to_string(),
                        description: "头像的图片地址".to_string(),
                    },
                    PropDoc {
                        name: "alt".to_string(),
                        prop_type: "Option<String>".to_string(),
                        default: "None".to_string(),
                        description: "图片无法显示时的替代文本".to_string(),
                    },
                    PropDoc {
                        name: "shape".to_string(),
                        prop_type: "AvatarShape".to_string(),
                        default: "AvatarShape::Circle".to_string(),
                        description: "头像的形状，可选 Circle、Square".to_string(),
                    },
                    PropDoc {
                        name: "size".to_string(),
                        prop_type: "AvatarSize".to_string(),
                        default: "AvatarSize::Default".to_string(),
                        description: "头像的尺寸，可选 Large、Default、Small 或自定义数值".to_string(),
                    },
                    PropDoc {
                        name: "icon".to_string(),
                        prop_type: "Option<String>".to_string(),
                        default: "None".to_string(),
                        description: "设置头像的图标类型".to_string(),
                    },
                    PropDoc {
                        name: "class".to_string(),
                        prop_type: "Option<String>".to_string(),
                        default: "None".to_string(),
                        description: "自定义类名".to_string(),
                    },
                    PropDoc {
                        name: "style".to_string(),
                        prop_type: "Option<String>".to_string(),
                        default: "None".to_string(),
                        description: "自定义样式".to_string(),
                    },
                    PropDoc {
                        name: "onclick".to_string(),
                        prop_type: "Option<EventHandler<MouseEvent>>".to_string(),
                        default: "None".to_string(),
                        description: "点击事件".to_string(),
                    },
                    PropDoc {
                        name: "on_error".to_string(),
                        prop_type: "Option<EventHandler<Event>>".to_string(),
                        default: "None".to_string(),
                        description: "图片加载失败时的回调".to_string(),
                    },
                ]
            }
        }
    }
}

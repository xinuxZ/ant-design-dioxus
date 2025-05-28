//! Badge 徽标数组件示例

use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn BadgeDemo() -> Element {
    rsx! {
        div {
            style: "padding: 20px;",

            h2 { "Badge 徽标数" }

            // 基础用法
            div {
                style: "margin-bottom: 20px;",
                h3 { "基础用法" }
                div {
                    style: "display: flex; gap: 20px; align-items: center;",

                    Badge {
                        count: 5,
                        Avatar {
                            shape: AvatarShape::Square,
                            size: AvatarSize::Large,
                            "U"
                        }
                    }

                    Badge {
                        count: 0,
                        show_zero: true,
                        Avatar {
                            shape: AvatarShape::Square,
                            size: AvatarSize::Large,
                            "U"
                        }
                    }

                    Badge {
                        count: 25,
                        Avatar {
                            shape: AvatarShape::Square,
                            size: AvatarSize::Large,
                            "U"
                        }
                    }
                }
            }

            // 封顶数字
            div {
                style: "margin-bottom: 20px;",
                h3 { "封顶数字" }
                div {
                    style: "display: flex; gap: 20px; align-items: center;",

                    Badge {
                        count: 99,
                        Avatar {
                            shape: AvatarShape::Square,
                            size: AvatarSize::Large,
                            "U"
                        }
                    }

                    Badge {
                        count: 100,
                        Avatar {
                            shape: AvatarShape::Square,
                            size: AvatarSize::Large,
                            "U"
                        }
                    }

                    Badge {
                        count: 99,
                        overflow_count: 10,
                        Avatar {
                            shape: AvatarShape::Square,
                            size: AvatarSize::Large,
                            "U"
                        }
                    }

                    Badge {
                        count: 1000,
                        overflow_count: 999,
                        Avatar {
                            shape: AvatarShape::Square,
                            size: AvatarSize::Large,
                            "U"
                        }
                    }
                }
            }

            // 小红点
            div {
                style: "margin-bottom: 20px;",
                h3 { "小红点" }
                div {
                    style: "display: flex; gap: 20px; align-items: center;",

                    Badge {
                        dot: true,
                        Avatar {
                            shape: AvatarShape::Square,
                            size: AvatarSize::Large,
                            "U"
                        }
                    }

                    Badge {
                        dot: true,
                        a {
                            href: "#",
                            "Link something"
                        }
                    }
                }
            }

            // 状态点
            div {
                style: "margin-bottom: 20px;",
                h3 { "状态点" }
                div {
                    style: "display: flex; flex-direction: column; gap: 10px;",

                    Badge {
                        status: BadgeStatus::Success,
                        text: "Success",
                    }

                    Badge {
                        status: BadgeStatus::Error,
                        text: "Error",
                    }

                    Badge {
                        status: BadgeStatus::Default,
                        text: "Default",
                    }

                    Badge {
                        status: BadgeStatus::Processing,
                        text: "Processing",
                    }

                    Badge {
                        status: BadgeStatus::Warning,
                        text: "Warning",
                    }
                }
            }

            // 自定义颜色
            div {
                style: "margin-bottom: 20px;",
                h3 { "自定义颜色" }
                div {
                    style: "display: flex; gap: 20px; align-items: center;",

                    Badge {
                        count: 5,
                        color: "#52c41a",
                        Avatar {
                            shape: AvatarShape::Square,
                            size: AvatarSize::Large,
                            "U"
                        }
                    }

                    Badge {
                        count: 5,
                        color: "#1890ff",
                        Avatar {
                            shape: AvatarShape::Square,
                            size: AvatarSize::Large,
                            "U"
                        }
                    }

                    Badge {
                        count: 5,
                        color: "#faad14",
                        Avatar {
                            shape: AvatarShape::Square,
                            size: AvatarSize::Large,
                            "U"
                        }
                    }

                    Badge {
                        dot: true,
                        color: "#52c41a",
                        Avatar {
                            shape: AvatarShape::Square,
                            size: AvatarSize::Large,
                            "U"
                        }
                    }
                }
            }

            // 自定义位置偏移
            div {
                style: "margin-bottom: 20px;",
                h3 { "自定义位置偏移" }
                div {
                    style: "display: flex; gap: 20px; align-items: center;",

                    Badge {
                        count: 5,
                        offset: (10, -10),
                        Avatar {
                            shape: AvatarShape::Square,
                            size: AvatarSize::Large,
                            "U"
                        }
                    }

                    Badge {
                        dot: true,
                        offset: (-5, 5),
                        Avatar {
                            shape: AvatarShape::Square,
                            size: AvatarSize::Large,
                            "U"
                        }
                    }
                }
            }

            // 尺寸
            div {
                style: "margin-bottom: 20px;",
                h3 { "尺寸" }
                div {
                    style: "display: flex; gap: 20px; align-items: center;",

                    Badge {
                        count: 5,
                        size: BadgeSize::Default,
                        Avatar {
                            shape: AvatarShape::Square,
                            size: AvatarSize::Large,
                            "Default"
                        }
                    }

                    Badge {
                        count: 5,
                        size: BadgeSize::Small,
                        Avatar {
                            shape: AvatarShape::Square,
                            size: AvatarSize::Large,
                            "Small"
                        }
                    }
                }
            }
        }
    }
}

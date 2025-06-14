//! Skeleton 骨架屏组件示例

use crate::components::button::{Button, ButtonType};
use crate::components::divider::Divider;
use crate::components::skeleton::{
    Skeleton, SkeletonAvatarProps, SkeletonAvatarShape, SkeletonAvatarSize, SkeletonButton,
    SkeletonButtonShape, SkeletonButtonSize, SkeletonImage, SkeletonInput, SkeletonInputSize,
    SkeletonParagraphProps, SkeletonTitleProps, SkeletonWidth,
};
use crate::components::switch::Switch;
use dioxus::prelude::*;

/// Skeleton 组件示例
#[component]
pub fn SkeletonExample() -> Element {
    let mut loading = use_signal(|| true);
    let mut active = use_signal(|| true);
    let mut avatar = use_signal(|| true);
    let mut title = use_signal(|| true);
    let mut paragraph = use_signal(|| true);
    let mut round = use_signal(|| false);

    rsx! {
        div {
            style: "padding: 20px;",

            h2 { "Skeleton 骨架屏" }

            // 控制面板
            div {
                style: "margin-bottom: 20px; padding: 16px; border: 1px solid #d9d9d9; border-radius: 6px;",

                div {
                    style: "display: flex; gap: 16px; flex-wrap: wrap; align-items: center;",

                    label {
                        style: "display: flex; align-items: center; gap: 8px;",
                        "Loading:"
                        Switch {
                            checked: loading(),
                            on_change: move |checked| loading.set(checked),
                        }
                    }

                    label {
                        style: "display: flex; align-items: center; gap: 8px;",
                        "Active:"
                        Switch {
                            checked: active(),
                            on_change: move |checked| active.set(checked),
                        }
                    }

                    label {
                        style: "display: flex; align-items: center; gap: 8px;",
                        "Avatar:"
                        Switch {
                            checked: avatar(),
                            on_change: move |checked| avatar.set(checked),
                        }
                    }

                    label {
                        style: "display: flex; align-items: center; gap: 8px;",
                        "Title:"
                        Switch {
                            checked: title(),
                            on_change: move |checked| title.set(checked),
                        }
                    }

                    label {
                        style: "display: flex; align-items: center; gap: 8px;",
                        "Paragraph:"
                        Switch {
                            checked: paragraph(),
                            on_change: move |checked| paragraph.set(checked),
                        }
                    }

                    label {
                        style: "display: flex; align-items: center; gap: 8px;",
                        "Round:"
                        Switch {
                            checked: round(),
                            on_change: move |checked| round.set(checked),
                        }
                    }
                }
            }

            // 基本使用
            h3 { "基本使用" }
            div {
                style: "margin-bottom: 32px; padding: 16px; border: 1px solid #f0f0f0; border-radius: 6px;",

                Skeleton {
                    loading: loading(),
                    active: active(),
                    avatar: avatar(),
                    title: title(),
                    paragraph: paragraph(),
                    round: round(),

                    if !loading() {
                        div {
                            style: "display: flex; align-items: center; gap: 12px;",

                            div {
                                style: "width: 40px; height: 40px; background: #1890ff; border-radius: 50%; display: flex; align-items: center; justify-content: center; color: white; font-weight: bold;",
                                "A"
                            }

                            div {
                                h4 {
                                    style: "margin: 0 0 8px 0;",
                                    "Ant Design"
                                }
                                p {
                                    style: "margin: 0; color: #666;",
                                    "We supply a series of design principles, practical patterns and high quality design resources (Sketch and Axure), to help people create their product prototypes beautifully and efficiently."
                                }
                            }
                        }
                    }
                }
            }

            Divider {}

            // 复杂的组合
            h3 { "复杂的组合" }
            div {
                style: "margin-bottom: 32px; padding: 16px; border: 1px solid #f0f0f0; border-radius: 6px;",

                Skeleton {
                    loading: true,
                    active: true,
                    avatar: true,
                    avatar_props: Some(SkeletonAvatarProps {
                        active: Some(true),
                        size: Some(SkeletonAvatarSize::Large),
                        shape: Some(SkeletonAvatarShape::Square),
                    }),
                    paragraph_props: Some(SkeletonParagraphProps {
                        rows: Some(4),
                        width: Some(SkeletonWidth::Array(vec![
                            SkeletonWidth::Percentage(100),
                            SkeletonWidth::Percentage(80),
                            SkeletonWidth::Percentage(90),
                            SkeletonWidth::Percentage(60),
                        ])),
                    }),
                    title_props: Some(SkeletonTitleProps {
                        width: Some(SkeletonWidth::Percentage(50)),
                    }),
                }
            }

            Divider {}

            // 按钮骨架屏
            h3 { "按钮骨架屏" }
            div {
                style: "margin-bottom: 32px; padding: 16px; border: 1px solid #f0f0f0; border-radius: 6px;",

                div {
                    style: "display: flex; gap: 16px; align-items: center; margin-bottom: 16px;",

                    SkeletonButton {
                        active: true,
                    }

                    SkeletonButton {
                        active: true,
                        size: Some(SkeletonButtonSize::Small),
                    }

                    SkeletonButton {
                        active: true,
                        size: Some(SkeletonButtonSize::Large),
                    }
                }

                div {
                    style: "display: flex; gap: 16px; align-items: center; margin-bottom: 16px;",

                    SkeletonButton {
                        active: true,
                        shape: Some(SkeletonButtonShape::Circle),
                    }

                    SkeletonButton {
                        active: true,
                        shape: Some(SkeletonButtonShape::Round),
                    }
                }

                SkeletonButton {
                    active: true,
                    block: true,
                }
            }

            Divider {}

            // 输入框骨架屏
            h3 { "输入框骨架屏" }
            div {
                style: "margin-bottom: 32px; padding: 16px; border: 1px solid #f0f0f0; border-radius: 6px;",

                div {
                    style: "margin-bottom: 16px;",

                    SkeletonInput {
                        active: true,
                        size: Some(SkeletonInputSize::Large),
                    }
                }

                div {
                    style: "margin-bottom: 16px;",

                    SkeletonInput {
                        active: true,
                        size: Some(SkeletonInputSize::Default),
                    }
                }

                SkeletonInput {
                    active: true,
                    size: Some(SkeletonInputSize::Small),
                }
            }

            Divider {}

            // 图片骨架屏
            h3 { "图片骨架屏" }
            div {
                style: "margin-bottom: 32px; padding: 16px; border: 1px solid #f0f0f0; border-radius: 6px;",

                div {
                    style: "display: flex; gap: 16px;",

                    SkeletonImage {
                        active: true,
                    }

                    SkeletonImage {
                        active: true,
                        style: Some("width: 120px; height: 120px;".to_string()),
                    }

                    SkeletonImage {
                        active: true,
                        style: Some("width: 200px; height: 100px;".to_string()),
                    }
                }
            }

            Divider {}

            // 列表骨架屏
            h3 { "列表骨架屏" }
            div {
                style: "margin-bottom: 32px; padding: 16px; border: 1px solid #f0f0f0; border-radius: 6px;",

                for i in 0..3 {
                    div {
                        key: "{i}",
                        style: "margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #f0f0f0;",

                        Skeleton {
                            loading: true,
                            active: true,
                            avatar: true,
                            title: false,
                            paragraph_props: Some(SkeletonParagraphProps {
                                rows: Some(2),
                                width: None,
                            }),
                        }
                    }
                }
            }

            Divider {}

            // 卡片骨架屏
            h3 { "卡片骨架屏" }
            div {
                style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 16px;",

                for i in 0..3 {
                    div {
                        key: "{i}",
                        style: "padding: 16px; border: 1px solid #f0f0f0; border-radius: 6px;",

                        div {
                            style: "margin-bottom: 16px;",
                            SkeletonImage {
                                active: true,
                                style: Some("width: 100%; height: 150px;".to_string()),
                            }
                        }

                        Skeleton {
                            loading: true,
                            active: true,
                            avatar: false,
                            title: true,
                            paragraph_props: Some(SkeletonParagraphProps {
                                rows: Some(3),
                                width: None,
                            }),
                        }

                        div {
                            style: "margin-top: 16px; display: flex; gap: 8px;",

                            SkeletonButton {
                                active: true,
                                size: Some(SkeletonButtonSize::Small),
                            }

                            SkeletonButton {
                                active: true,
                                size: Some(SkeletonButtonSize::Small),
                            }
                        }
                    }
                }
            }
        }
    }
}

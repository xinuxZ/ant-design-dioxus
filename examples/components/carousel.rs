#![allow(non_snake_case)]
//!
//! 展示 Carousel 组件的各种用法和样式

use crate::common::*;
use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

/// Carousel 组件演示
#[component]
pub fn CarouselDemo() -> Element {
    rsx! {
        div {
            class: "component-showcase",

            h1 {
                style: "margin: 0 0 16px 0; color: #262626; font-size: 28px;",
                "Carousel 走马灯"
            }

            p {
                style: "margin: 0 0 24px 0; color: #666; font-size: 14px;",
                "旋转木马，一组轮播的区域。"
            }

            // 基础用法
            DemoSection {
                title: "基础用法",
                description: "最简单的用法。",

                div {
                    style: "width: 100%; max-width: 600px;",

                    Carousel {
                        div {
                            style: "height: 160px; color: #fff; line-height: 160px; text-align: center; background: #364d79;",
                            h3 { style: "margin: 0; color: #fff;", "1" }
                        }
                        div {
                            style: "height: 160px; color: #fff; line-height: 160px; text-align: center; background: #364d79;",
                            h3 { style: "margin: 0; color: #fff;", "2" }
                        }
                        div {
                            style: "height: 160px; color: #fff; line-height: 160px; text-align: center; background: #364d79;",
                            h3 { style: "margin: 0; color: #fff;", "3" }
                        }
                        div {
                            style: "height: 160px; color: #fff; line-height: 160px; text-align: center; background: #364d79;",
                            h3 { style: "margin: 0; color: #fff;", "4" }
                        }
                    }
                }
            }

            // 位置
            DemoSection {
                title: "位置",
                description: "位置有 4 个方向。",

                div {
                    style: "width: 100%; max-width: 600px;",

                    Carousel {
                        dot_position: DotPosition::Top,
                        div {
                            style: "height: 160px; color: #fff; line-height: 160px; text-align: center; background: #364d79;",
                            h3 { style: "margin: 0; color: #fff;", "1" }
                        }
                        div {
                            style: "height: 160px; color: #fff; line-height: 160px; text-align: center; background: #364d79;",
                            h3 { style: "margin: 0; color: #fff;", "2" }
                        }
                        div {
                            style: "height: 160px; color: #fff; line-height: 160px; text-align: center; background: #364d79;",
                            h3 { style: "margin: 0; color: #fff;", "3" }
                        }
                        div {
                            style: "height: 160px; color: #fff; line-height: 160px; text-align: center; background: #364d79;",
                            h3 { style: "margin: 0; color: #fff;", "4" }
                        }
                    }
                }
            }

            // 自动切换
            DemoSection {
                title: "自动切换",
                description: "定时切换下一张。",

                div {
                    style: "width: 100%; max-width: 600px;",

                    Carousel {
                        autoplay: true,
                        div {
                            style: "height: 160px; color: #fff; line-height: 160px; text-align: center; background: #364d79;",
                            h3 { style: "margin: 0; color: #fff;", "1" }
                        }
                        div {
                            style: "height: 160px; color: #fff; line-height: 160px; text-align: center; background: #364d79;",
                            h3 { style: "margin: 0; color: #fff;", "2" }
                        }
                        div {
                            style: "height: 160px; color: #fff; line-height: 160px; text-align: center; background: #364d79;",
                            h3 { style: "margin: 0; color: #fff;", "3" }
                        }
                        div {
                            style: "height: 160px; color: #fff; line-height: 160px; text-align: center; background: #364d79;",
                            h3 { style: "margin: 0; color: #fff;", "4" }
                        }
                    }
                }
            }

            // 渐显
            DemoSection {
                title: "渐显",
                description: "切换效果为渐显。",

                div {
                    style: "width: 100%; max-width: 600px;",

                    Carousel {
                        effect: Effect::Fade,
                        div {
                            style: "height: 160px; color: #fff; line-height: 160px; text-align: center; background: #364d79;",
                            h3 { style: "margin: 0; color: #fff;", "1" }
                        }
                        div {
                            style: "height: 160px; color: #fff; line-height: 160px; text-align: center; background: #364d79;",
                            h3 { style: "margin: 0; color: #fff;", "2" }
                        }
                        div {
                            style: "height: 160px; color: #fff; line-height: 160px; text-align: center; background: #364d79;",
                            h3 { style: "margin: 0; color: #fff;", "3" }
                        }
                        div {
                            style: "height: 160px; color: #fff; line-height: 160px; text-align: center; background: #364d79;",
                            h3 { style: "margin: 0; color: #fff;", "4" }
                        }
                    }
                }
            }

            // API 文档
            ApiDocumentation {
                component_name: "Carousel",
                props: vec![
                    PropDoc {
                        name: "autoplay".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "是否自动切换".to_string(),
                    },
                    PropDoc {
                        name: "dots_position".to_string(),
                        prop_type: "String".to_string(),
                        default: "bottom".to_string(),
                        description: "面板指示点位置，可选 top left right bottom".to_string(),
                    },
                    PropDoc {
                        name: "effect".to_string(),
                        prop_type: "String".to_string(),
                        default: "scrollx".to_string(),
                        description: "动画效果函数，可选 scrollx, fade".to_string(),
                    },
                    PropDoc {
                        name: "autoplay_speed".to_string(),
                        prop_type: "u32".to_string(),
                        default: "3000".to_string(),
                        description: "自动切换的时间间隔，单位为毫秒".to_string(),
                    },
                ]
            }
        }
    }
}

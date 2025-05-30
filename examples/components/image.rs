#![allow(non_snake_case)]
//!
//! 展示 Image 组件的各种用法和样式

use crate::common::*;
use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

/// Image 组件演示
#[component]
pub fn ImageDemo() -> Element {
    rsx! {
        div {
            class: "component-showcase",

            h1 {
                style: "margin: 0 0 16px 0; color: #262626; font-size: 28px;",
                "Image 图片"
            }

            p {
                style: "margin: 0 0 24px 0; color: #666; font-size: 14px;",
                "可预览的图片。"
            }

            // 基础用法
            DemoSection {
                title: "基础用法",
                description: "单击图片可以放大预览。",

                div {
                    style: "display: flex; gap: 16px; flex-wrap: wrap;",

                    Image {
                        width: Some("200px".to_string()),
                        src: "https://zos.alipayobjects.com/rmsportal/jkjgkEfvpUPVyRjUImniVslZfWPnJuuZ.png".to_string(),
                        alt: Some("example".to_string()),
                    }
                }
            }

            // 容错处理
            DemoSection {
                title: "容错处理",
                description: "加载失败显示图像占位符。",

                div {
                    style: "display: flex; gap: 16px; flex-wrap: wrap;",

                    Image {
                        width: Some("200px".to_string()),
                        height: Some("200px".to_string()),
                        src: "error".to_string(),
                        fallback: Some("data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAMIAAADDCAYAAADQvc6UAAABRWlDQ1BJQ0MgUHJvZmlsZQAAKJFjYGASSSwoyGFhYGDIzSspCnJ3UoiIjFJgf8LAwSDCIMogwMCcmFxc4BgQ4ANUwgCjUcG3awyMIPqyLsis7PPOq3QdDFcvjV3jOD1boQVTPQrgSkktTgbSf4A4LbmgqISBgTEFyFYuLykAsTuAbJEioKOA7DkgdjqEvQHEToKwj4DVhAQ5A9k3gGyB5IxEoBmML4BsnSQk8XQkNtReEOBxcfXxUQg1Mjc0dyHgXNJBSWpFCYh2zi+oLMpMzyhRcASGUqqCZ16yno6CkYGRAQMDKMwhqj/fAIcloxgHQqxAjIHBEugw5sUIsSQpBobtQPdLciLEVJYzMPBHMDBsayhILEqEO4DxG0txmrERhM29nYGBddr//5/DGRjYNRkY/l7////39v///y4Dmn+LgeHANwDrkl1AuO+pmgAAADhlWElmTU0AKgAAAAgAAYdpAAQAAAABAAAAGgAAAAAAAqACAAQAAAABAAAAwqADAAQAAAABAAAAwwAAAAD9b/HnAAAHlklEQVR4Ae3dP3Ik1RnG4W+FgYxN".to_string()),
                        alt: Some("fallback".to_string()),
                    }
                }
            }

            // 渐进加载
            DemoSection {
                title: "渐进加载",
                description: "大图使用 placeholder 渐进加载。",

                div {
                    style: "display: flex; gap: 16px; flex-wrap: wrap;",

                    Image {
                        width: Some("200px".to_string()),
                        src: "https://zos.alipayobjects.com/rmsportal/jkjgkEfvpUPVyRjUImniVslZfWPnJuuZ.png".to_string(),
                        placeholder: Some(rsx! {
                            div {
                                style: "width: 200px; height: 200px; background: #f5f5f5; display: flex; align-items: center; justify-content: center; color: #999;",
                                "Loading..."
                            }
                        }),
                        alt: Some("example".to_string()),
                    }
                }
            }

            // 多张图片预览
            DemoSection {
                title: "多张图片预览",
                description: "点击左右切换按钮可以预览多张图片。",

                div {
                    style: "display: flex; gap: 16px; flex-wrap: wrap;",

                    Image {
                        width: Some("200px".to_string()),
                        src: "https://gw.alipayobjects.com/zos/antfincdn/LlvErxo8H9/photo-1503185912284-5271ff81b9a8.webp".to_string(),
                        alt: Some("image-1".to_string()),
                    }

                    Image {
                        width: Some("200px".to_string()),
                        src: "https://gw.alipayobjects.com/zos/antfincdn/cV16ZqzMjW/photo-1473091540282-9b846e7965e3.webp".to_string(),
                        alt: Some("image-2".to_string()),
                    }

                    Image {
                        width: Some("200px".to_string()),
                        src: "https://gw.alipayobjects.com/zos/antfincdn/x43I27A55%26/photo-1438109491414-7198515b166b.webp".to_string(),
                        alt: Some("image-3".to_string()),
                    }
                }
            }

            // 相册模式
            DemoSection {
                title: "相册模式",
                description: "统一的预览管理。",

                div {
                    style: "display: flex; gap: 16px; flex-wrap: wrap;",

                    ImagePreviewGroup {
                        Image {
                            width: Some("200px".to_string()),
                            src: "https://gw.alipayobjects.com/zos/rmsportal/KDpgvguMpGfqaHPjicRK.svg".to_string(),
                            alt: Some("image-1".to_string()),
                        }

                        Image {
                            width: Some("200px".to_string()),
                            src: "https://gw.alipayobjects.com/zos/antfincdn/aPkFc8Sj7n/method-draw-image.svg".to_string(),
                            alt: Some("image-2".to_string()),
                        }
                    }
                }
            }

            // 自定义预览图片
            DemoSection {
                title: "自定义预览图片",
                description: "可以设置不同的预览图片。",

                div {
                    style: "display: flex; gap: 16px; flex-wrap: wrap;",

                    Image {
                        width: Some("200px".to_string()),
                        src: "https://zos.alipayobjects.com/rmsportal/jkjgkEfvpUPVyRjUImniVslZfWPnJuuZ.png".to_string(),
                        // preview_src: "https://gw.alipayobjects.com/zos/antfincdn/LlvErxo8H9/photo-1503185912284-5271ff81b9a8.webp",
                        alt: Some("example".to_string()),
                    }
                }
            }

            // API 文档
            ApiDocumentation {
                component_name: "Image",
                props: vec![
                    PropDoc {
                        name: "alt".to_string(),
                        prop_type: "String".to_string(),
                        default: "-".to_string(),
                        description: "图像描述".to_string(),
                    },
                    PropDoc {
                        name: "fallback".to_string(),
                        prop_type: "String".to_string(),
                        default: "-".to_string(),
                        description: "加载失败容错地址".to_string(),
                    },
                    PropDoc {
                        name: "height".to_string(),
                        prop_type: "u32".to_string(),
                        default: "-".to_string(),
                        description: "图像高度".to_string(),
                    },
                    PropDoc {
                        name: "placeholder".to_string(),
                        prop_type: "Element".to_string(),
                        default: "-".to_string(),
                        description: "加载占位, 为 true 时使用默认占位".to_string(),
                    },
                    PropDoc {
                        name: "preview".to_string(),
                        prop_type: "bool".to_string(),
                        default: "true".to_string(),
                        description: "预览参数，为 false 时禁用".to_string(),
                    },
                    PropDoc {
                        name: "src".to_string(),
                        prop_type: "String".to_string(),
                        default: "-".to_string(),
                        description: "图片地址".to_string(),
                    },
                    PropDoc {
                        name: "width".to_string(),
                        prop_type: "u32".to_string(),
                        default: "-".to_string(),
                        description: "图像宽度".to_string(),
                    },
                ]
            }
        }
    }
}

use crate::common::demo_section::DemoSection;
use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

/// QRCode组件示例
#[component]
pub fn QRCodeDemo() -> Element {
    rsx! {
        div { class: "demo-container",
            h1 { "QRCode 二维码" }
            p { "能够将链接转换成二维码的组件。" }

            DemoSection {
                title: "基本使用",
                description: "最简单的用法。",

                div {
                    QRCode {
                        class: "demo-qr-code",
                        style: "border: 1px solid #d9d9d9; border-radius: 8px; padding: 16px; background: white;",
                        value: "https://ant.design"
                    }
                }
            }

            DemoSection {
                title: "不同尺寸",
                description: "三种尺寸的二维码。",

                div {
                    div { style: "display: flex; gap: 24px; align-items: center;",
                        QRCode {
                            class: "qr-code-small",
                            style: "width: 80px; height: 80px; border: 1px solid #d9d9d9; border-radius: 6px; padding: 8px; background: white;",
                            value: "Small QR"
                        }
                        QRCode {
                            class: "qr-code-default",
                            style: "width: 120px; height: 120px; border: 1px solid #d9d9d9; border-radius: 6px; padding: 12px; background: white;",
                            value: "Default QR"
                        }
                        QRCode {
                            class: "qr-code-large",
                            style: "width: 160px; height: 160px; border: 1px solid #d9d9d9; border-radius: 6px; padding: 16px; background: white;",
                            value: "Large QR"
                        }
                    }
                }
            }

            DemoSection {
                title: "自定义颜色",
                description: "可以自定义二维码的颜色。",

                div {
                    div { style: "display: flex; gap: 24px;",
                        QRCode {
                            class: "qr-code-blue",
                            style: "width: 120px; height: 120px; border: 1px solid #d9d9d9; border-radius: 6px; padding: 12px; background: white; color: #1677ff;",
                            value: "Blue QR Code"
                        }
                        QRCode {
                            class: "qr-code-green",
                            style: "width: 120px; height: 120px; border: 1px solid #d9d9d9; border-radius: 6px; padding: 12px; background: white; color: #52c41a;",
                            value: "Green QR Code"
                        }
                        QRCode {
                            class: "qr-code-red",
                            style: "width: 120px; height: 120px; border: 1px solid #d9d9d9; border-radius: 6px; padding: 12px; background: white; color: #ff4d4f;",
                            value: "Red QR Code"
                        }
                    }
                }
            }

            DemoSection {
                title: "带Logo的二维码",
                description: "可以在二维码中心添加Logo。",

                div {
                    QRCode {
                        class: "qr-code-with-logo",
                        style: "width: 160px; height: 160px; border: 1px solid #d9d9d9; border-radius: 8px; padding: 16px; background: white; position: relative;",
                        value: "https://ant.design"
                    }
                }
            }

            DemoSection {
                title: "错误级别",
                description: "不同错误纠正级别的二维码。",

                div {
                    div { style: "display: flex; gap: 16px; flex-wrap: wrap;",
                        div { style: "text-align: center;",
                            QRCode {
                                class: "qr-code-level-l",
                                style: "width: 100px; height: 100px; border: 1px solid #d9d9d9; border-radius: 6px; padding: 8px; background: white; margin-bottom: 8px;",
                                value: "Error Level L"
                            }
                            div { style: "font-size: 12px; color: #666;", "Level L" }
                        }
                        div { style: "text-align: center;",
                            QRCode {
                                class: "qr-code-level-m",
                                style: "width: 100px; height: 100px; border: 1px solid #d9d9d9; border-radius: 6px; padding: 8px; background: white; margin-bottom: 8px;",
                                value: "Error Level M"
                            }
                            div { style: "font-size: 12px; color: #666;", "Level M" }
                        }
                        div { style: "text-align: center;",
                            QRCode {
                                class: "qr-code-level-q",
                                style: "width: 100px; height: 100px; border: 1px solid #d9d9d9; border-radius: 6px; padding: 8px; background: white; margin-bottom: 8px;",
                                value: "Error Level Q"
                            }
                            div { style: "font-size: 12px; color: #666;", "Level Q" }
                        }
                        div { style: "text-align: center;",
                            QRCode {
                                class: "qr-code-level-h",
                                style: "width: 100px; height: 100px; border: 1px solid #d9d9d9; border-radius: 6px; padding: 8px; background: white; margin-bottom: 8px;",
                                value: "Error Level H"
                            }
                            div { style: "font-size: 12px; color: #666;", "Level H" }
                        }
                    }
                }
            }
        }
    }
}

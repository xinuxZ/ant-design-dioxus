use dioxus::prelude::*;

const QR_CODE_STYLE: &str = include_str!("./style.css");

/// QRCode 状态类型
#[derive(Clone, PartialEq, Debug)]
pub enum QRStatus {
    Active,
    Expired,
    Loading,
    Scanned,
}

/// QRCode 渲染类型
#[derive(Clone, PartialEq, Debug)]
pub enum QRType {
    Canvas,
    Svg,
}

/// QRCode 错误纠正级别
#[derive(Clone, PartialEq, Debug)]
pub enum ErrorLevel {
    L, // 约7%错误纠正
    M, // 约15%错误纠正
    Q, // 约25%错误纠正
    H, // 约30%错误纠正
}

/// QRCode 图标尺寸
#[derive(Clone, PartialEq, Debug)]
pub struct IconSize {
    pub width: u32,
    pub height: u32,
}

/// QRCode 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct QRCodeProps {
    /// 扫描后的文本
    #[props(default = "".to_string())]
    pub value: String,

    /// 渲染类型
    #[props(default = QRType::Canvas)]
    pub qr_type: QRType,

    /// 二维码中图片的地址（目前只支持图片链接）
    #[props(default = None)]
    pub icon: Option<String>,

    /// 二维码尺寸
    #[props(default = 160)]
    pub size: u32,

    /// 二维码中图片的大小
    #[props(default = None)]
    pub icon_size: Option<IconSize>,

    /// 二维码颜色
    #[props(default = "#000".to_string())]
    pub color: String,

    /// 二维码背景颜色
    #[props(default = "transparent".to_string())]
    pub bg_color: String,

    /// 是否有边框
    #[props(default = true)]
    pub bordered: bool,

    /// 错误纠正级别
    #[props(default = ErrorLevel::M)]
    pub error_level: ErrorLevel,

    /// 二维码状态
    #[props(default = QRStatus::Active)]
    pub status: QRStatus,

    /// 点击"点击刷新"的回调
    #[props(default = None)]
    pub on_refresh: Option<EventHandler<()>>,

    /// 自定义渲染状态
    #[props(default = None)]
    pub status_render: Option<Element>,

    /// CSS 类名
    #[props(default)]
    pub class: Option<String>,

    /// 内联样式
    #[props(default)]
    pub style: Option<String>,

    /// 元素 ID
    #[props(default)]
    pub id: Option<String>,
}

/// QRCode 组件
#[component]
pub fn QRCode(props: QRCodeProps) -> Element {
    let QRCodeProps {
        value,
        qr_type,
        icon,
        size,
        icon_size,
        color,
        bg_color,
        bordered,
        error_level,
        status,
        on_refresh,
        status_render,
        class,
        style,
        id,
    } = props;

    // 计算图标尺寸
    let calculated_icon_size = icon_size.unwrap_or(IconSize {
        width: size / 4,
        height: size / 4,
    });

    // 根据状态渲染不同内容
    let content = match status {
        QRStatus::Active => {
            rsx! {
                div {
                    class: "ant-qrcode",
                    class: if bordered { "ant-qrcode-bordered" } else { "" },
                    style: "width: {size}px; height: {size}px; position: relative;",

                    // 根据类型渲染 Canvas 或 SVG
                    {match qr_type {
                        QRType::Canvas => rsx! {
                            canvas {
                                width: "{size}",
                                height: "{size}",
                                style: "background-color: {bg_color};",
                                // 这里需要使用 JavaScript 生成二维码
                                "data-value": "{value}",
                                "data-color": "{color}",
                                "data-error-level": match error_level {
                                    ErrorLevel::L => "L",
                                    ErrorLevel::M => "M",
                                    ErrorLevel::Q => "Q",
                                    ErrorLevel::H => "H",
                                },
                            }
                        },
                        QRType::Svg => rsx! {
                            svg {
                                width: "{size}",
                                height: "{size}",
                                style: "background-color: {bg_color};",
                                // SVG 二维码内容
                                rect {
                                    width: "100%",
                                    height: "100%",
                                    fill: "{bg_color}",
                                }
                                // 这里需要生成实际的二维码 SVG 路径
                            }
                        }
                    }}

                    // 如果有图标，渲染图标
                    {if let Some(icon_url) = icon {
                        rsx! {
                            div {
                                class: "ant-qrcode-icon",
                                style: "position: absolute; top: 50%; left: 50%; transform: translate(-50%, -50%); width: {calculated_icon_size.width}px; height: {calculated_icon_size.height}px;",
                                img {
                                    src: "{icon_url}",
                                    alt: "QR Code Icon",
                                    style: "width: 100%; height: 100%; object-fit: cover; border-radius: 4px;",
                                }
                            }
                        }
                    } else {
                        rsx! { }
                    }}
                }
            }
        }
        QRStatus::Expired => {
            if let Some(custom_render) = status_render {
                custom_render
            } else {
                rsx! {
                    div {
                        class: "ant-qrcode ant-qrcode-expired",
                        class: if bordered { "ant-qrcode-bordered" } else { "" },
                        style: "width: {size}px; height: {size}px; display: flex; flex-direction: column; align-items: center; justify-content: center; background-color: #f5f5f5;",

                        div {
                            class: "ant-qrcode-expired-icon",
                            style: "font-size: 24px; color: #bfbfbf; margin-bottom: 8px;",
                            "⚠️"
                        }

                        div {
                            class: "ant-qrcode-expired-text",
                            style: "color: #8c8c8c; font-size: 12px; text-align: center;",
                            "二维码已过期"
                        }

                        if let Some(refresh_handler) = on_refresh {
                            button {
                                class: "ant-btn ant-btn-link ant-qrcode-refresh",
                                style: "padding: 4px 0; font-size: 12px; height: auto;",
                                onclick: move |_| refresh_handler.call(()),
                                "点击刷新"
                            }
                        }
                    }
                }
            }
        }
        QRStatus::Loading => {
            if let Some(custom_render) = status_render {
                custom_render
            } else {
                rsx! {
                    div {
                        class: "ant-qrcode ant-qrcode-loading",
                        class: if bordered { "ant-qrcode-bordered" } else { "" },
                        style: "width: {size}px; height: {size}px; display: flex; align-items: center; justify-content: center; background-color: #f5f5f5;",

                        div {
                            class: "ant-spin-dot ant-spin-dot-spin",
                            style: "font-size: 20px;",
                            div { class: "ant-spin-dot-item" }
                            div { class: "ant-spin-dot-item" }
                            div { class: "ant-spin-dot-item" }
                            div { class: "ant-spin-dot-item" }
                        }
                    }
                }
            }
        }
        QRStatus::Scanned => {
            if let Some(custom_render) = status_render {
                custom_render
            } else {
                rsx! {
                    div {
                        class: "ant-qrcode ant-qrcode-scanned",
                        class: if bordered { "ant-qrcode-bordered" } else { "" },
                        style: "width: {size}px; height: {size}px; display: flex; flex-direction: column; align-items: center; justify-content: center; background-color: #f6ffed;",

                        div {
                            class: "ant-qrcode-scanned-icon",
                            style: "font-size: 24px; color: #52c41a; margin-bottom: 8px;",
                            "✓"
                        }

                        div {
                            class: "ant-qrcode-scanned-text",
                            style: "color: #52c41a; font-size: 12px; text-align: center;",
                            "扫描成功"
                        }
                    }
                }
            }
        }
    };

    rsx! {
        style { {QR_CODE_STYLE} }

        div {
            class: format!("ant-qrcode-wrapper {}", class.as_deref().unwrap_or("")),
            id: id.as_deref(),
            style: style.as_deref(),
            {content}
        }
    }
}

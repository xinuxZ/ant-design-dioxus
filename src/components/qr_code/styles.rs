use css_in_rust::css;

use super::types::QRCodeStatus;

/// 生成QRCode容器的基础样式
pub fn generate_container_style() -> String {
    css!(
        r#"
        position: relative;
        display: inline-block;
        font-size: 0;
    "#
    )
}

/// 生成QRCode边框样式
pub fn generate_bordered_style() -> String {
    css!(
        r#"
        padding: 12px;
        border: 1px solid rgba(0, 0, 0, 0.06);
        border-radius: 8px;
    "#
    )
}

/// 生成QRCode图标样式
pub fn generate_icon_style() -> String {
    css!(
        r#"
        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        z-index: 10;
        pointer-events: none;
    "#
    )
}

/// 生成QRCode状态遮罩样式
pub fn generate_status_mask_style() -> String {
    css!(
        r#"
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background-color: rgba(255, 255, 255, 0.8);
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        z-index: 20;
    "#
    )
}

/// 生成QRCode刷新按钮样式
pub fn generate_refresh_button_style() -> String {
    css!(
        r#"
        margin-top: 8px;
        padding: 4px 8px;
        font-size: 14px;
        line-height: 1.5;
        border-radius: 2px;
        color: #fff;
        background-color: #1677ff;
        border: 1px solid #1677ff;
        cursor: pointer;
        transition: all 0.3s;
    "#
    )
}

/// 生成QRCode刷新按钮悬停样式
pub fn generate_refresh_button_hover_style() -> String {
    css!(
        r#"
        background-color: #4096ff;
        border-color: #4096ff;
    "#
    )
}

/// 生成QRCode状态文本样式
pub fn generate_status_text_style() -> String {
    css!(
        r#"
        margin: 8px 0;
        font-size: 16px;
        color: rgba(0, 0, 0, 0.88);
    "#
    )
}

/// 生成QRCode加载中样式
pub fn generate_loading_style() -> String {
    css!(
        r#"
        display: inline-block;
        width: 20px;
        height: 20px;
        border: 2px solid rgba(0, 0, 0, 0.1);
        border-top-color: #1677ff;
        border-radius: 50%;
        animation: qrcode-spin 1s infinite linear;

        @keyframes qrcode-spin {
            to {
                transform: rotate(360deg);
            }
        }
    "#
    )
}

/// 生成QRCode已扫描图标样式
pub fn generate_scanned_icon_style() -> String {
    css!(
        r#"
        font-size: 48px;
        color: #52c41a;
    "#
    )
}

/// 生成QRCode过期图标样式
pub fn generate_expired_icon_style() -> String {
    css!(
        r#"
        font-size: 48px;
        color: #ff4d4f;
    "#
    )
}

/// 根据状态生成对应的图标样式
pub fn get_status_icon_style(status: QRCodeStatus) -> String {
    match status {
        QRCodeStatus::Loading => generate_loading_style(),
        QRCodeStatus::Scanned => generate_scanned_icon_style(),
        QRCodeStatus::Expired => generate_expired_icon_style(),
        _ => String::new(),
    }
}

/// 根据状态生成对应的文本内容
pub fn get_status_text(status: QRCodeStatus) -> &'static str {
    match status {
        QRCodeStatus::Loading => "加载中...",
        QRCodeStatus::Scanned => "已扫描",
        QRCodeStatus::Expired => "二维码已过期",
        _ => "",
    }
}

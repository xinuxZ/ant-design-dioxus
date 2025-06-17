use qrcode::render::svg;
use qrcode::{EcLevel, QrCode, Version};
use std::fmt::Write;

use super::types::{QRCodeErrorLevel, QRCodeIconSettings};

/// 生成QRCode的SVG字符串
pub fn generate_qrcode_svg(
    content: &str,
    size: u32,
    color: &str,
    bg_color: &str,
    error_level: QRCodeErrorLevel,
    icon: Option<&QRCodeIconSettings>,
) -> Result<String, String> {
    // 创建QR码实例
    let qr = QrCode::with_error_correction_level(content.as_bytes(), error_level.into())
        .map_err(|e| format!("生成二维码失败: {}", e))?;

    // 设置SVG渲染器
    let mut svg_render = qr
        .render()
        .min_dimensions(size, size)
        .max_dimensions(size, size)
        .dark_color(svg::Color(color))
        .light_color(svg::Color(bg_color))
        .build();

    // 如果有图标，在SVG中添加图标
    if let Some(icon_settings) = icon {
        if !icon_settings.src.is_empty() {
            let x = icon_settings.x.unwrap_or((size - icon_settings.width) / 2);
            let y = icon_settings.y.unwrap_or((size - icon_settings.height) / 2);

            // 在SVG末尾插入图标
            let icon_svg = format!(
                r#"<image href="{}" x="{}" y="{}" width="{}" height="{}" opacity="{}"/>"#,
                icon_settings.src,
                x,
                y,
                icon_settings.width,
                icon_settings.height,
                icon_settings.opacity
            );

            // 在SVG关闭标签前插入图标
            let svg_len = svg_render.len();
            let insert_pos = svg_render.rfind("</svg>").unwrap_or(svg_len);
            svg_render.insert_str(insert_pos, &icon_svg);
        }
    }

    Ok(svg_render)
}

/// 生成QRCode的Data URL
pub fn generate_qrcode_data_url(
    content: &str,
    size: u32,
    color: &str,
    bg_color: &str,
    error_level: QRCodeErrorLevel,
) -> Result<String, String> {
    // 创建QR码实例
    let qr = QrCode::with_error_correction_level(content.as_bytes(), error_level.into())
        .map_err(|e| format!("生成二维码失败: {}", e))?;

    // 获取QR码矩阵
    // let modules = qr.modules();
    let width = qr.width() as u32;

    // 计算每个模块的大小
    let module_size = size / width;

    // 创建SVG字符串
    let mut svg = String::new();
    use std::fmt::Write;
    write!(
        svg,
        "<svg width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\" xmlns=\"http://www.w3.org/2000/svg\">\n<rect width=\"{}\" height=\"{}\" fill=\"{}\"/>\n",
        size, size, size, size, size, size, bg_color
    ).map_err(|e| format!("Failed to write SVG header: {}", e))?;

    // 绘制二维码模块
    for y in 0..width {
        for x in 0..width {
            if qr.is_functional(x as usize, y as usize) {
                let x_pos = x * module_size;
                let y_pos = y * module_size;
                write!(
                    svg,
                    "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\"/>\n",
                    x_pos, y_pos, module_size, module_size, color
                ).map_err(|e| format!("Failed to write SVG rect: {}", e))?;
            }
        }
    }

    write!(svg, "</svg>").map_err(|e| format!("Failed to write SVG footer: {}", e))?;

    // 转换为Data URL
    let svg_base64 = base64::encode(&svg);
    Ok(format!("data:image/svg+xml;base64,{}", svg_base64))
}

/// 根据内容计算最佳的QR码版本
pub fn calculate_optimal_version(content: &str) -> Version {
    let len = content.len();

    // 根据内容长度粗略估计版本
    if len <= 25 {
        Version::Normal(1)
    } else if len <= 47 {
        Version::Normal(2)
    } else if len <= 77 {
        Version::Normal(3)
    } else if len <= 114 {
        Version::Normal(4)
    } else if len <= 154 {
        Version::Normal(5)
    } else if len <= 195 {
        Version::Normal(6)
    } else if len <= 224 {
        Version::Normal(7)
    } else if len <= 279 {
        Version::Normal(8)
    } else if len <= 335 {
        Version::Normal(9)
    } else if len <= 395 {
        Version::Normal(10)
    } else {
        Version::Normal(12) // 默认使用版本12，支持较长的内容
    }
}

/// 将十六进制颜色转换为RGB元组
pub fn hex_to_rgb(hex: &str) -> (u8, u8, u8) {
    let hex = hex.trim_start_matches('#');

    if hex.len() == 6 {
        // 标准6位十六进制
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
        (r, g, b)
    } else if hex.len() == 3 {
        // 简写3位十六进制
        let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).unwrap_or(0);
        let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).unwrap_or(0);
        let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).unwrap_or(0);
        (r, g, b)
    } else {
        // 默认黑色
        (0, 0, 0)
    }
}

use dioxus::prelude::*;

/// 水印字体配置
#[derive(Clone, PartialEq, Debug)]
pub struct WatermarkFont {
    /// 字体颜色
    pub color: String,
    /// 字体大小
    pub font_size: u32,
    /// 字体粗细
    pub font_weight: String,
    /// 字体样式
    pub font_style: String,
    /// 字体族
    pub font_family: String,
}

impl Default for WatermarkFont {
    fn default() -> Self {
        Self {
            color: "rgba(0,0,0,.15)".to_string(),
            font_size: 16,
            font_weight: "normal".to_string(),
            font_style: "normal".to_string(),
            font_family: "sans-serif".to_string(),
        }
    }
}

/// 水印间距配置
#[derive(Clone, PartialEq, Debug)]
pub struct WatermarkGap {
    /// 水印之间的水平间距
    pub x: i32,
    /// 水印之间的垂直间距
    pub y: i32,
}

impl Default for WatermarkGap {
    fn default() -> Self {
        Self { x: 212, y: 222 }
    }
}

/// 水印偏移配置
#[derive(Clone, PartialEq, Debug)]
pub struct WatermarkOffset {
    /// 水印在 canvas 画布上绘制的水平偏移量
    pub x: i32,
    /// 水印在 canvas 画布上绘制的垂直偏移量
    pub y: i32,
}

impl Default for WatermarkOffset {
    fn default() -> Self {
        Self {
            x: (212 / 2) as i32,
            y: (222 / 2) as i32,
        }
    }
}

/// Watermark 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct WatermarkProps {
    id: Option<String>,
    class: Option<String>,
    style: Option<String>,

    /// 水印文字内容
    #[props(default = None)]
    pub content: Option<String>,

    /// 水印文字内容（多行）
    #[props(default = None)]
    pub content_list: Option<Vec<String>>,

    /// 水印图片源，建议导出 2 倍或 3 倍图，优先级高于文字
    #[props(default = None)]
    pub image: Option<String>,

    /// 水印的宽度
    #[props(default = 120)]
    pub width: u32,

    /// 水印的高度
    #[props(default = 64)]
    pub height: u32,

    /// 水印绘制时，旋转的角度，单位 °
    #[props(default = -22)]
    pub rotate: i32,

    /// 追加的水印元素的 z-index
    #[props(default = 9)]
    pub z_index: u32,

    /// 水印字体配置
    #[props(default = WatermarkFont::default())]
    pub font: WatermarkFont,

    /// 水印之间的间距
    #[props(default = WatermarkGap::default())]
    pub gap: WatermarkGap,

    /// 水印偏移量
    #[props(default = WatermarkOffset::default())]
    pub offset: WatermarkOffset,

    /// 子元素
    children: Element,
}

/// Watermark 组件
#[component]
pub fn Watermark(props: WatermarkProps) -> Element {
    let WatermarkProps {
        content,
        content_list,
        image,
        width,
        height,
        rotate,
        z_index,
        font,
        gap,
        offset,
        children,
        class,
        style,
        id,
    } = props;

    // 生成水印背景图片的 data URL
    let offset_clone = offset.clone();
    let watermark_bg = use_memo(move || {
        generate_watermark_bg(
            content.clone(),
            content_list.clone(),
            image.clone(),
            width,
            height,
            rotate,
            &font,
            &gap,
            &offset_clone,
        )
    });

    rsx! {
            div {
                class: format!("ant-watermark {}", class.as_deref().unwrap_or("")),
                id: id,
                style: style,
                position: "relative",

            // 子元素内容
            {children}

            // 水印层
            div {
                class: "ant-watermark-wrapper",
                style: format!("
                    position: absolute;
                    top: 0;
                    left: 0;
                    width: 100%;
                    height: 100%;
                    pointer-events: none;
                    background-repeat: repeat;
                    background-position: {}px {}px;
                    background-image: url('{}');
                    z-index: {};
                ", offset.x, offset.y, watermark_bg(), props.z_index)
            }
        }
    }
}

/// 生成水印背景图片
fn generate_watermark_bg(
    content: Option<String>,
    content_list: Option<Vec<String>>,
    image: Option<String>,
    width: u32,
    height: u32,
    rotate: i32,
    font: &WatermarkFont,
    gap: &WatermarkGap,
    offset: &WatermarkOffset,
) -> String {
    // 计算画布尺寸
    let canvas_width = (width + gap.x as u32) * 2;
    let canvas_height = (height + gap.y as u32) * 2;

    // 如果有图片，优先使用图片
    if let Some(img_url) = image {
        return generate_image_watermark(
            img_url,
            canvas_width,
            canvas_height,
            width,
            height,
            rotate,
            offset,
        );
    }

    // 使用文字生成水印
    let text_content = if let Some(list) = content_list {
        list.join("\n")
    } else {
        content.unwrap_or_default()
    };

    generate_text_watermark(
        text_content,
        canvas_width,
        canvas_height,
        width,
        height,
        rotate,
        font,
        offset,
    )
}

/// 生成文字水印
fn generate_text_watermark(
    text: String,
    canvas_width: u32,
    canvas_height: u32,
    width: u32,
    height: u32,
    rotate: i32,
    font: &WatermarkFont,
    offset: &WatermarkOffset,
) -> String {
    // 创建 SVG 水印
    let svg_content = format!(
        r#"<svg width="{canvas_width}" height="{canvas_height}" xmlns="http://www.w3.org/2000/svg">
            <defs>
                <pattern id="watermark" x="0" y="0" width="{pattern_width}" height="{pattern_height}" patternUnits="userSpaceOnUse">
                    <g transform="translate({center_x},{center_y}) rotate({rotate})">
                        <text x="0" y="0"
                              text-anchor="middle"
                              dominant-baseline="middle"
                              font-family="{font_family}"
                              font-size="{font_size}px"
                              font-weight="{font_weight}"
                              font-style="{font_style}"
                              fill="{color}"
                              opacity="0.15">
                            {text}
                        </text>
                    </g>
                </pattern>
            </defs>
            <rect width="100%" height="100%" fill="url(#watermark)" />
        </svg>"#,
        canvas_width = canvas_width,
        canvas_height = canvas_height,
        pattern_width = width + offset.x.abs() as u32,
        pattern_height = height + offset.y.abs() as u32,
        center_x = width / 2,
        center_y = height / 2,
        rotate = rotate,
        font_family = font.font_family,
        font_size = font.font_size,
        font_weight = font.font_weight,
        font_style = font.font_style,
        color = font.color,
        text = escape_xml(&text),
    );

    // 转换为 data URL
    format!("data:image/svg+xml;base64,{}", base64_encode(&svg_content))
}

/// 生成图片水印
fn generate_image_watermark(
    img_url: String,
    canvas_width: u32,
    canvas_height: u32,
    width: u32,
    height: u32,
    rotate: i32,
    offset: &WatermarkOffset,
) -> String {
    // 创建包含图片的 SVG 水印
    let svg_content = format!(
        r#"<svg width="{canvas_width}" height="{canvas_height}" xmlns="http://www.w3.org/2000/svg">
            <defs>
                <pattern id="watermark" x="0" y="0" width="{pattern_width}" height="{pattern_height}" patternUnits="userSpaceOnUse">
                    <g transform="translate({center_x},{center_y}) rotate({rotate})">
                        <image href="{img_url}" x="{img_x}" y="{img_y}" width="{width}" height="{height}" opacity="0.15" />
                    </g>
                </pattern>
            </defs>
            <rect width="100%" height="100%" fill="url(#watermark)" />
        </svg>"#,
        canvas_width = canvas_width,
        canvas_height = canvas_height,
        pattern_width = width + offset.x.abs() as u32,
        pattern_height = height + offset.y.abs() as u32,
        center_x = width / 2,
        center_y = height / 2,
        rotate = rotate,
        img_url = img_url,
        img_x = -(width as i32 / 2),
        img_y = -(height as i32 / 2),
        width = width,
        height = height,
    );

    // 转换为 data URL
    format!("data:image/svg+xml;base64,{}", base64_encode(&svg_content))
}

/// XML 转义
fn escape_xml(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

/// Base64 编码（简单实现）
fn base64_encode(input: &str) -> String {
    // 这里应该使用实际的 base64 编码库
    // 为了简化，这里返回一个占位符
    // 在实际项目中，应该使用 base64 crate
    // use std::str;

    // 简单的 base64 编码实现
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();
    let bytes = input.as_bytes();

    for chunk in bytes.chunks(3) {
        let mut buf = [0u8; 3];
        for (i, &byte) in chunk.iter().enumerate() {
            buf[i] = byte;
        }

        let b = ((buf[0] as u32) << 16) | ((buf[1] as u32) << 8) | (buf[2] as u32);

        result.push(CHARS[((b >> 18) & 63) as usize] as char);
        result.push(CHARS[((b >> 12) & 63) as usize] as char);
        result.push(if chunk.len() > 1 {
            CHARS[((b >> 6) & 63) as usize] as char
        } else {
            '='
        });
        result.push(if chunk.len() > 2 {
            CHARS[(b & 63) as usize] as char
        } else {
            '='
        });
    }

    result
}

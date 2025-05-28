//! Icon 图标组件
//!
//! 语义化的矢量图形。使用图标组件，你可以使用这些内置的图标，
//! 也可以使用自定义图标。
//!
//! ## 何时使用
//!
//! - 提供快速识别和理解功能
//! - 节省界面空间
//! - 增强视觉效果
//!
//! ## 设计师专属
//!
//! 安装 [Kitchen Sketch 插件 💎](https://kitchen.alipay.com)，
//! 就可以一键拖拽使用 Ant Design 和 Iconfont 的海量图标，还可以关联自有项目。

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

// 引入图标样式
const ICON_STYLE: &str = include_str!("style.css");

/// 图标旋转方向
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IconRotate {
    /// 不旋转
    None,
    /// 顺时针旋转 90 度
    Rotate90,
    /// 顺时针旋转 180 度
    Rotate180,
    /// 顺时针旋转 270 度
    Rotate270,
}

impl Default for IconRotate {
    fn default() -> Self {
        Self::None
    }
}

/// 图标主题
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IconTheme {
    /// 线框风格
    Outlined,
    /// 实底风格
    Filled,
    /// 双色风格
    TwoTone,
}

impl Default for IconTheme {
    fn default() -> Self {
        Self::Outlined
    }
}

/// 图标属性
#[derive(Props, Clone, PartialEq)]
pub struct IconProps {
    /// 图标类型（图标名称）
    pub icon_type: String,
    /// 图标主题
    #[props(default)]
    pub theme: IconTheme,
    /// 是否旋转
    #[props(default)]
    pub spin: bool,
    /// 旋转角度
    #[props(default)]
    pub rotate: IconRotate,
    /// 图标尺寸
    #[props(default)]
    pub size: Option<String>,
    /// 图标颜色
    #[props(default)]
    pub color: Option<String>,
    /// 自定义类名
    #[props(default)]
    pub class: Option<String>,
    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,
    /// 点击事件处理器
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    /// 双色图标的主色
    #[props(default)]
    pub two_tone_color: Option<String>,
    /// 双色图标的次色
    #[props(default)]
    pub two_tone_secondary_color: Option<String>,
}

/// 图标组件
///
/// # 参数
///
/// * `props` - 图标属性
///
/// # 示例
///
/// ```rust
/// use dioxus::prelude::*;
/// use ant_design_dioxus::{Icon, IconTheme};
///
/// fn app() -> Element {
///     rsx! {
///         Icon {
///             icon_type: "home".to_string(),
///             theme: IconTheme::Outlined,
///             size: Some("24px".to_string()),
///             color: Some("#1890ff".to_string()),
///         }
///     }
/// }
/// ```
#[component]
pub fn Icon(props: IconProps) -> Element {
    let class_name = get_icon_class_name(&props);
    let icon_style = get_icon_style(&props);

    rsx! {
        // 注入图标样式
        style { {ICON_STYLE} }

        i {
            class: class_name.clone(),
            style: icon_style.clone(),
            onclick: move |evt| {
                if let Some(handler) = &props.onclick {
                    handler.call(evt);
                }
            },

            // 使用 SVG 图标或字体图标
            if props.theme == IconTheme::TwoTone {
                // 双色图标使用 SVG
                svg {
                    width: props.size.as_deref().unwrap_or("1em"),
                    height: props.size.as_deref().unwrap_or("1em"),
                    fill: "currentColor",
                    "aria-hidden": "true",
                    "focusable": "false",

                    // 这里应该根据 icon_type 渲染对应的 SVG 路径
                    // 暂时使用占位符
                    circle {
                        cx: "12",
                        cy: "12",
                        r: "10",
                        fill: props.two_tone_color.as_deref().unwrap_or("#1890ff"),
                    }
                    circle {
                        cx: "12",
                        cy: "12",
                        r: "6",
                        fill: props.two_tone_secondary_color.as_deref().unwrap_or("#91d5ff"),
                    }
                }
            } else {
                // 线框和实底图标使用字体图标或简单 SVG
                svg {
                    width: props.size.as_deref().unwrap_or("1em"),
                    height: props.size.as_deref().unwrap_or("1em"),
                    fill: "currentColor",
                    "aria-hidden": "true",
                    "focusable": "false",

                    // 根据图标类型渲染不同的 SVG 内容
                    // 这里使用一些常见图标的简单实现
                    {render_icon_svg(&props.icon_type, &props.theme)}
                }
            }
        }
    }
}

/// 获取图标的类名
///
/// # 参数
///
/// * `props` - 图标属性
///
/// # 返回值
///
/// 返回图标的完整类名字符串
fn get_icon_class_name(props: &IconProps) -> String {
    let mut classes = vec!["anticon"];

    // 添加图标类型类名
    let icon_type_class = format!("anticon-{}", props.icon_type);
    classes.push(&icon_type_class);

    // 添加主题类名
    match props.theme {
        IconTheme::Outlined => {} // 默认不需要额外类名
        IconTheme::Filled => classes.push("anticon-filled"),
        IconTheme::TwoTone => classes.push("anticon-two-tone"),
    }

    // 添加旋转类名
    if props.spin {
        classes.push("anticon-spin");
    }

    match props.rotate {
        IconRotate::None => {}
        IconRotate::Rotate90 => classes.push("anticon-rotate-90"),
        IconRotate::Rotate180 => classes.push("anticon-rotate-180"),
        IconRotate::Rotate270 => classes.push("anticon-rotate-270"),
    }

    // 添加自定义类名
    let mut class_string = classes.join(" ");
    if let Some(custom_class) = &props.class {
        class_string.push(' ');
        class_string.push_str(custom_class);
    }

    class_string
}

/// 获取图标的内联样式
///
/// # 参数
///
/// * `props` - 图标属性
///
/// # 返回值
///
/// 返回图标的内联样式字符串
fn get_icon_style(props: &IconProps) -> String {
    let mut styles = Vec::new();

    // 设置尺寸
    if let Some(size) = &props.size {
        styles.push(format!("font-size: {}", size));
    }

    // 设置颜色
    if let Some(color) = &props.color {
        styles.push(format!("color: {}", color));
    }

    // 添加自定义样式
    let mut style_string = styles.join("; ");
    if let Some(custom_style) = &props.style {
        if !style_string.is_empty() {
            style_string.push_str("; ");
        }
        style_string.push_str(custom_style);
    }

    style_string
}

/// 渲染图标 SVG 内容
///
/// # 参数
///
/// * `icon_type` - 图标类型
/// * `theme` - 图标主题
///
/// # 返回值
///
/// 返回对应的 SVG 元素
fn render_icon_svg(icon_type: &str, theme: &IconTheme) -> Element {
    match icon_type {
        "home" => rsx! {
            path {
                d: "M10 20v-6h4v6h5v-8h3L12 3 2 12h3v8z",
                fill: if matches!(theme, IconTheme::Filled) { "currentColor" } else { "none" },
                stroke: if matches!(theme, IconTheme::Outlined) { "currentColor" } else { "none" },
                "stroke-width": if matches!(theme, IconTheme::Outlined) { "2" } else { "0" },
            }
        },
        "user" => rsx! {
            path {
                d: "M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z",
                fill: if matches!(theme, IconTheme::Filled) { "currentColor" } else { "none" },
                stroke: if matches!(theme, IconTheme::Outlined) { "currentColor" } else { "none" },
                "stroke-width": if matches!(theme, IconTheme::Outlined) { "2" } else { "0" },
            }
        },
        "setting" => rsx! {
            path {
                d: "M19.14,12.94c0.04-0.3,0.06-0.61,0.06-0.94c0-0.32-0.02-0.64-0.07-0.94l2.03-1.58c0.18-0.14,0.23-0.41,0.12-0.61 l-1.92-3.32c-0.12-0.22-0.37-0.29-0.59-0.22l-2.39,0.96c-0.5-0.38-1.03-0.7-1.62-0.94L14.4,2.81c-0.04-0.24-0.24-0.41-0.48-0.41 h-3.84c-0.24,0-0.43,0.17-0.47,0.41L9.25,5.35C8.66,5.59,8.12,5.92,7.63,6.29L5.24,5.33c-0.22-0.08-0.47,0-0.59,0.22L2.74,8.87 C2.62,9.08,2.66,9.34,2.86,9.48l2.03,1.58C4.84,11.36,4.8,11.69,4.8,12s0.02,0.64,0.07,0.94l-2.03,1.58 c-0.18,0.14-0.23,0.41-0.12,0.61l1.92,3.32c0.12,0.22,0.37,0.29,0.59,0.22l2.39-0.96c0.5,0.38,1.03,0.7,1.62,0.94l0.36,2.54 c0.05,0.24,0.24,0.41,0.48,0.41h3.84c0.24,0,0.44-0.17,0.47-0.41l0.36-2.54c0.59-0.24,1.13-0.56,1.62-0.94l2.39,0.96 c0.22,0.08,0.47,0,0.59-0.22l1.92-3.32c0.12-0.22,0.07-0.47-0.12-0.61L19.14,12.94z M12,15.6c-1.98,0-3.6-1.62-3.6-3.6 s1.62-3.6,3.6-3.6s3.6,1.62,3.6,3.6S13.98,15.6,12,15.6z",
                fill: if matches!(theme, IconTheme::Filled) { "currentColor" } else { "none" },
                stroke: if matches!(theme, IconTheme::Outlined) { "currentColor" } else { "none" },
                "stroke-width": if matches!(theme, IconTheme::Outlined) { "1" } else { "0" },
            }
        },
        "close" => rsx! {
            path {
                d: "M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z",
                fill: if matches!(theme, IconTheme::Filled) { "currentColor" } else { "none" },
                stroke: if matches!(theme, IconTheme::Outlined) { "currentColor" } else { "none" },
                "stroke-width": if matches!(theme, IconTheme::Outlined) { "2" } else { "0" },
            }
        },
        "check" => rsx! {
            path {
                d: "M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41z",
                fill: if matches!(theme, IconTheme::Filled) { "currentColor" } else { "none" },
                stroke: if matches!(theme, IconTheme::Outlined) { "currentColor" } else { "none" },
                "stroke-width": if matches!(theme, IconTheme::Outlined) { "2" } else { "0" },
            }
        },
        _ => rsx! {
            // 默认图标（圆形）
            circle {
                cx: "12",
                cy: "12",
                r: "10",
                fill: if matches!(theme, IconTheme::Filled) { "currentColor" } else { "none" },
                stroke: if matches!(theme, IconTheme::Outlined) { "currentColor" } else { "none" },
                "stroke-width": if matches!(theme, IconTheme::Outlined) { "2" } else { "0" },
            }
        },
    }
}

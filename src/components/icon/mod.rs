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

mod styles;

use self::styles::{
    IconRotate as StyleIconRotate, IconSize, IconStyleGenerator, IconTheme as StyleIconTheme,
};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

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
    // 使用 CSS-in-Rust 样式生成器
    let style_generator = IconStyleGenerator::new()
        .with_theme(convert_theme(&props.theme))
        .with_rotate(convert_rotate(&props.rotate))
        .with_size(convert_size(&props.size))
        .with_color(props.color.clone())
        .with_spin(props.spin)
        .with_disabled(false); // 图标组件暂不支持禁用状态

    // 生成样式类名
    let css_class = style_generator.generate();

    // 合并自定义类名
    let class_name = if let Some(custom_class) = &props.class {
        format!("anticon anticon-{} {}", props.icon_type, custom_class)
    } else {
        format!("anticon anticon-{}", props.icon_type)
    };

    // 合并所有类名
    let combined_class = format!("{} {}", class_name, css_class);

    // 获取自定义样式
    let custom_style = props.style.clone().unwrap_or_default();

    rsx! {
        i {
            class: combined_class,
            style: custom_style,
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

/// 将组件的主题属性转换为样式生成器的主题枚举
fn convert_theme(theme: &IconTheme) -> StyleIconTheme {
    match theme {
        IconTheme::Outlined => StyleIconTheme::Outlined,
        IconTheme::Filled => StyleIconTheme::Filled,
        IconTheme::TwoTone => StyleIconTheme::TwoTone,
    }
}

/// 将组件的旋转属性转换为样式生成器的旋转枚举
fn convert_rotate(rotate: &IconRotate) -> StyleIconRotate {
    match rotate {
        IconRotate::None => StyleIconRotate::None,
        IconRotate::Rotate90 => StyleIconRotate::Rotate90,
        IconRotate::Rotate180 => StyleIconRotate::Rotate180,
        IconRotate::Rotate270 => StyleIconRotate::Rotate270,
    }
}

/// 将组件的尺寸属性转换为样式生成器的尺寸枚举
fn convert_size(size: &Option<String>) -> IconSize {
    match size {
        Some(size_value) => {
            if size_value == "12px" {
                IconSize::Small
            } else if size_value == "16px" {
                IconSize::Medium
            } else if size_value == "18px" {
                IconSize::Large
            } else if size_value == "24px" {
                IconSize::ExtraLarge
            } else {
                IconSize::Custom(size_value.clone())
            }
        }
        None => IconSize::Medium,
    }
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

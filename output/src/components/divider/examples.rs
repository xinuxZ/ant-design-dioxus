//!
//! Divider 组件使用示例
//!
//! 展示 Divider 组件的各种使用方式和配置选项。

use dioxus::prelude::*;
use super::*;

/// 基础分割线示例
pub fn basic_divider_example() -> Element {
    rsx! {
        div {
            p { "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed nonne merninisti licere mihi ista probare, quae sunt a te dicta?" }
            Divider {}
            p { "Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat." }
            Divider { dashed: true }
            p { "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur." }
        }
    }
}

/// 带文字的分割线示例
pub fn divider_with_text_example() -> Element {
    rsx! {
        div {
            p { "Lorem ipsum dolor sit amet, consectetur adipiscing elit." }
            Divider {
                "Text"
            }
            p { "Lorem ipsum dolor sit amet, consectetur adipiscing elit." }
            Divider {
                orientation: DividerOrientation::Left,
                "Left Text"
            }
            p { "Lorem ipsum dolor sit amet, consectetur adipiscing elit." }
            Divider {
                orientation: DividerOrientation::Right,
                "Right Text"
            }
            p { "Lorem ipsum dolor sit amet, consectetur adipiscing elit." }
        }
    }
}

/// 垂直分割线示例
pub fn vertical_divider_example() -> Element {
    rsx! {
        div {
            style: "display: flex; align-items: center;",
            "Text"
            Divider {
                r#type: DividerType::Vertical
            }
            a { href: "#", "Link" }
            Divider {
                r#type: DividerType::Vertical
            }
            a { href: "#", "Link" }
        }
    }
}

/// 不同尺寸的分割线示例
pub fn divider_sizes_example() -> Element {
    rsx! {
        div {
            Divider {
                size: DividerSize::Small,
                "Small Size"
            }
            Divider {
                "Default Size"
            }
            Divider {
                size: DividerSize::Large,
                "Large Size"
            }
        }
    }
}

/// 简洁模式分割线示例
pub fn plain_divider_example() -> Element {
    rsx! {
        div {
            p { "Lorem ipsum dolor sit amet, consectetur adipiscing elit." }
            Divider {
                plain: true,
                "Text"
            }
            p { "Lorem ipsum dolor sit amet, consectetur adipiscing elit." }
        }
    }
}

/// 自定义样式分割线示例
pub fn custom_divider_example() -> Element {
    rsx! {
        div {
            p { "Lorem ipsum dolor sit amet, consectetur adipiscing elit." }
            Divider {
                class: "custom-divider",
                style: "border-color: #ff4d4f; margin: 24px 0;",
                "Custom Style"
            }
            p { "Lorem ipsum dolor sit amet, consectetur adipiscing elit." }
        }
    }
}

/// 分割线变体示例
pub fn divider_variants_example() -> Element {
    rsx! {
        div {
            p { "实线分割线（默认）" }
            Divider {
                variant: DividerVariant::Solid,
                "Solid"
            }
            p { "虚线分割线" }
            Divider {
                variant: DividerVariant::Dashed,
                "Dashed"
            }
            p { "点线分割线" }
            Divider {
                variant: DividerVariant::Dotted,
                "Dotted"
            }
        }
    }
}

/// 方向边距示例
pub fn orientation_margin_example() -> Element {
    rsx! {
        div {
            p { "默认边距" }
            Divider {
                orientation: DividerOrientation::Left,
                "Default Margin"
            }
            p { "自定义像素边距" }
            Divider {
                orientation: DividerOrientation::Left,
                orientation_margin: Some(OrientationMargin::Pixels(50)),
                "50px Margin"
            }
            p { "百分比边距" }
            Divider {
                orientation: DividerOrientation::Right,
                orientation_margin: Some(OrientationMargin::Percentage(20.0)),
                "20% Margin"
            }
        }
    }
}

/// 自定义前缀类名示例
pub fn prefix_cls_example() -> Element {
    rsx! {
        div {
            p { "默认前缀类名" }
            Divider {
                "Default Prefix"
            }
            p { "自定义前缀类名" }
            Divider {
                prefix_cls: Some("my-custom".to_string()),
                "Custom Prefix"
            }
        }
    }
}

/// 复杂组合示例
pub fn complex_combination_example() -> Element {
    rsx! {
        div {
            p { "复杂属性组合示例" }
            Divider {
                r#type: DividerType::Horizontal,
                size: DividerSize::Large,
                variant: DividerVariant::Dashed,
                orientation: DividerOrientation::Left,
                orientation_margin: Some(OrientationMargin::Pixels(30)),
                plain: false,
                class: "complex-divider",
                style: "color: #1890ff; font-weight: bold;",
                "Complex Example"
            }
            p { "这是一个包含多种属性的复杂分割线示例" }
        }
    }
}

/// 完整示例应用
pub fn divider_showcase() -> Element {
    rsx! {
        div {
            style: "padding: 20px; max-width: 800px; margin: 0 auto;",
            
            h2 { "Divider 分割线组件示例" }
            
            h3 { "基础用法" }
            {basic_divider_example()}
            
            h3 { "带文字的分割线" }
            {divider_with_text_example()}
            
            h3 { "垂直分割线" }
            {vertical_divider_example()}
            
            h3 { "不同尺寸" }
            {divider_sizes_example()}
            
            h3 { "简洁模式" }
            {plain_divider_example()}
            
            h3 { "分割线变体" }
            {divider_variants_example()}
            
            h3 { "方向边距" }
            {orientation_margin_example()}
            
            h3 { "自定义前缀类名" }
            {prefix_cls_example()}
            
            h3 { "复杂组合" }
            {complex_combination_example()}
            
            h3 { "自定义样式" }
            {custom_divider_example()}
        }
    }
}
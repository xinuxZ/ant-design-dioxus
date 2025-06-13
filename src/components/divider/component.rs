//!
//! Divider 分割线组件
//!
//! 区隔内容的分割线。
//!
//! ## 何时使用
//!
//! - 对不同章节的文本段落进行分割。
//! - 对行内文字/链接进行分割，例如表格的操作列。
//!
//! ## 示例
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::Divider;
//!
//! fn app() -> Element {
//!     rsx! {
//!         div {
//!             p { "Lorem ipsum dolor sit amet, consectetur adipiscing elit." }
//!             Divider {}
//!             p { "Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua." }
//!         }
//!     }
//! }
//! ```

use dioxus::prelude::*;
use css_in_rust::css;
use super::types::*;
use super::styles::{generate_divider_styles, DividerStyleGenerator};
use crate::config_provider::use_config;
use crate::theme::use_theme;

/// Divider 分割线组件
///
/// 区隔内容的分割线。
#[component]
pub fn Divider(props: DividerProps) -> Element {
    // 获取配置和主题
    let config = use_config();
    let theme = use_theme();
    
    // 确定前缀类名
    let prefix_cls = props.prefix_cls
        .clone()
        .unwrap_or_else(|| config.config.prefix_cls.clone());
    
    // 确定实际的变体（variant优先级高于dashed）
    let actual_variant = if props.dashed && matches!(props.variant, DividerVariant::Solid) {
        DividerVariant::Dashed
    } else {
        props.variant.clone()
    };
    
    // 创建样式生成器
    let style_generator = DividerStyleGenerator::new()
        .with_type(props.r#type.clone())
        .with_size(props.size.clone())
        .with_variant(actual_variant.clone())
        .with_plain(props.plain)
        .with_has_text(props.children.is_some())
        .with_orientation(props.orientation.clone())
        .with_orientation_margin(props.orientation_margin.clone())
        .with_prefix_cls(prefix_cls.clone())
        .with_token(crate::theme::AliasToken::default());
    
    // 获取样式
    let styles = generate_divider_styles();
    
    // 注册基础样式
    {
        let style_generator_clone = style_generator.clone();
        let has_children = props.children.is_some();
        let size = props.size.clone();
        
        use_effect(move || {
            let _ = css!(styles.base);
            
            // 根据属性注册变体样式
            if has_children {
                let _ = css!(styles.variants.type_text);
            }
            
            // 注册变体样式
            match actual_variant {
                DividerVariant::Dashed => { let _ = css!(styles.variants.type_dashed); },
                DividerVariant::Dotted => { let _ = css!(styles.variants.type_dotted); },
                _ => {}
            }
            
            // 注册尺寸样式
            match size {
                DividerSize::Small => { let _ = css!(styles.variants.size_small); },
                DividerSize::Large => { let _ = css!(styles.variants.size_large); },
                _ => {}
            }
            
            // 注册方向边距样式
            if let Some(margin_style) = style_generator_clone.generate_orientation_margin_style() {
                let _ = css!(margin_style);
            }
        });
    }

    // 生成CSS类名
    let mut class_names = style_generator.generate_class_names();
    
    // 添加自定义类名
    if let Some(custom_class) = &props.class {
        class_names.push(custom_class.clone());
    }
    
    // 生成内联样式
    let inline_styles = style_generator.generate_inline_styles();
    
    // 合并自定义样式
    let final_styles = if let Some(custom_style) = &props.style {
        format!("{};{}", inline_styles, custom_style)
    } else {
        inline_styles
    };
    
    let class_name = class_names.join(" ");
    
    // 渲染组件
    match props.r#type {
        DividerType::Horizontal => {
            if let Some(children) = props.children {
                // 水平分割线带文字
                rsx! {
                    div {
                        class: "{class_name}",
                        style: "{final_styles}",
                        role: "separator",
                        span {
                            class: "{prefix_cls}-divider-inner-text",
                            {children}
                        }
                    }
                }
            } else {
                // 水平分割线无文字
                rsx! {
                    div {
                        class: "{class_name}",
                        style: "{final_styles}",
                        role: "separator"
                    }
                }
            }
        }
        DividerType::Vertical => {
            // 垂直分割线
            rsx! {
                div {
                    class: "{class_name}",
                    style: "{final_styles}",
                    role: "separator"
                }
            }
        }
    }
}
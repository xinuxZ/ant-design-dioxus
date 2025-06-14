//! # Divider 分割线
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
//!
//! ### 带文字的分割线
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::{Divider, DividerOrientation};
//!
//! fn app() -> Element {
//!     rsx! {
//!         div {
//!             p { "Lorem ipsum dolor sit amet." }
//!             Divider {
//!                 orientation: DividerOrientation::Left,
//!                 "Left Text"
//!             }
//!             p { "Lorem ipsum dolor sit amet." }
//!             Divider {
//!                 orientation: DividerOrientation::Right,
//!                 "Right Text"
//!             }
//!             p { "Lorem ipsum dolor sit amet." }
//!         }
//!     }
//! }
//! ```
//!
//! ### 垂直分割线
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::{Divider, DividerType};
//!
//! fn app() -> Element {
//!     rsx! {
//!         div {
//!             "Text"
//!             Divider { r#type: DividerType::Vertical }
//!             a { href: "#", "Link" }
//!             Divider { r#type: DividerType::Vertical }
//!             a { href: "#", "Link" }
//!         }
//!     }
//! }
//! ```

mod styles;

use css_in_rust::css;
use dioxus::prelude::*;
use styles::{DividerOrientation, DividerSize, DividerType};

/// Divider 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct DividerProps {
    /// 子元素
    #[props(default)]
    pub children: Option<Element>,
    /// 分割线类型
    #[props(default)]
    pub r#type: DividerType,
    /// 分割线尺寸
    #[props(default)]
    pub size: DividerSize,
    /// 是否为虚线
    #[props(default = false)]
    pub dashed: bool,
    /// 是否为简洁模式
    #[props(default = false)]
    pub plain: bool,
    /// 分割线文字的位置
    #[props(default)]
    pub orientation: DividerOrientation,
}

/// 注册全局样式
fn register_styles() {
    use_effect(|| {
        let _ = css!(styles::DividerStyleGenerator::base_style());
    });
}

/// Divider 分割线组件
///
/// 区隔内容的分割线。
#[component]
pub fn Divider(props: DividerProps) -> Element {
    register_styles();

    // 创建样式生成器
    let has_text = props.children.is_some();

    // 创建样式生成器
    let style_generator = styles::DividerStyleGenerator::new(
        props.r#type.clone(),
        props.size.clone(),
        props.dashed,
        props.plain,
        has_text,
        props.orientation.clone(),
    );

    // 生成CSS样式
    let css_style = style_generator.generate();

    // 根据类型渲染不同的分割线
    match props.r#type {
        DividerType::Horizontal => {
            if let Some(children) = &props.children {
                // 带文字的水平分割线
                rsx! {
                    div {
                        class: "ant-divider ant-divider-horizontal ant-divider-with-text",
                        style: "{css_style}",
                        span {
                            class: "ant-divider-inner-text",
                            {children}
                        }
                    }
                }
            } else {
                // 普通水平分割线
                rsx! {
                    div {
                        class: "ant-divider ant-divider-horizontal",
                        style: "{css_style}",
                    }
                }
            }
        }
        DividerType::Vertical => {
            // 垂直分割线不支持文字
            rsx! {
                div {
                    class: "ant-divider ant-divider-vertical",
                    style: "{css_style}",
                }
            }
        }
    }
}

// 重新导出公共类型
// 注意：不使用通配符导出以避免命名冲突

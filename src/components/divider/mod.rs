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

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

/// 分割线类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DividerType {
    /// 水平分割线
    Horizontal,
    /// 垂直分割线
    Vertical,
}

impl Default for DividerType {
    fn default() -> Self {
        Self::Horizontal
    }
}

impl DividerType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

/// 分割线文字位置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DividerOrientation {
    /// 左侧
    Left,
    /// 居中
    Center,
    /// 右侧
    Right,
}

impl Default for DividerOrientation {
    fn default() -> Self {
        Self::Center
    }
}

impl DividerOrientation {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Center => "center",
            Self::Right => "right",
        }
    }
}

/// 分割线尺寸
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DividerSize {
    /// 小号
    Small,
    /// 默认
    Default,
    /// 大号
    Large,
}

impl Default for DividerSize {
    fn default() -> Self {
        Self::Default
    }
}

impl DividerSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Default => "default",
            Self::Large => "large",
        }
    }
}

/// Divider 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct DividerProps {
    /// 分割线类型
    #[props(default)]
    pub r#type: DividerType,

    /// 分割线文字的位置
    #[props(default)]
    pub orientation: DividerOrientation,

    /// 分割线尺寸
    #[props(default)]
    pub size: DividerSize,

    /// 是否为虚线
    #[props(default = false)]
    pub dashed: bool,

    /// 是否为简洁模式（无边距）
    #[props(default = false)]
    pub plain: bool,

    /// 自定义 CSS 类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,

    /// 子元素（分割线文字）
    #[props(default)]
    pub children: Option<Element>,
}

/// Divider 分割线组件
///
/// 区隔内容的分割线。
#[component]
pub fn Divider(props: DividerProps) -> Element {
    let mut class_list = vec!["ant-divider"];

    // 添加类型样式
    class_list.push(match props.r#type {
        DividerType::Horizontal => "ant-divider-horizontal",
        DividerType::Vertical => "ant-divider-vertical",
    });

    // 添加尺寸样式
    class_list.push(match props.size {
        DividerSize::Small => "ant-divider-small",
        DividerSize::Default => "ant-divider-default",
        DividerSize::Large => "ant-divider-large",
    });

    // 添加虚线样式
    if props.dashed {
        class_list.push("ant-divider-dashed");
    }

    // 添加简洁模式样式
    if props.plain {
        class_list.push("ant-divider-plain");
    }

    // 如果有文字内容，添加文字位置样式
    if props.children.is_some() {
        class_list.push("ant-divider-with-text");
        class_list.push(match props.orientation {
            DividerOrientation::Left => "ant-divider-with-text-left",
            DividerOrientation::Center => "ant-divider-with-text-center",
            DividerOrientation::Right => "ant-divider-with-text-right",
        });
    }

    // 添加自定义类名
    if let Some(custom_class) = &props.class {
        class_list.push(custom_class);
    }

    let class_name = class_list.join(" ");

    match props.r#type {
        DividerType::Horizontal => {
            if let Some(children) = props.children {
                rsx! {
                    div {
                        class: "{class_name}",
                        style: props.style,
                        role: "separator",
                        span {
                            class: "ant-divider-inner-text",
                            {children}
                        }
                    }
                }
            } else {
                rsx! {
                    hr {
                        class: "{class_name}",
                        style: props.style,
                        role: "separator"
                    }
                }
            }
        }
        DividerType::Vertical => {
            rsx! {
                span {
                    class: "{class_name}",
                    style: props.style,
                    role: "separator",
                    "aria-orientation": "vertical"
                }
            }
        }
    }
}

// 重新导出公共类型
// 注意：不使用通配符导出以避免命名冲突

//! Space 间距组件
//!
//! Space 组件用于设置组件之间的间距，避免组件紧贴在一起，拉开统一的空间。
//!
//! ## 何时使用
//!
//! - 避免组件紧贴在一起，拉开统一的空间。
//! - 在某组件的某个方向上，保持统一的间距。
//! - 支持水平、垂直方向。
//!
//! ## 示例
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::Space;
//!
//! fn app() -> Element {
//!     rsx! {
//!         Space {
//!             Button { "按钮1" }
//!             Button { "按钮2" }
//!             Button { "按钮3" }
//!         }
//!     }
//! }
//! ```

use crate::theme::core::types::SpaceSize;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

const SPACE_STYLES: &str = include_str!("./style.css");

/// Space 组件的方向
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SpaceDirection {
    /// 水平方向 - 默认
    Horizontal,
    /// 垂直方向
    Vertical,
}

impl Default for SpaceDirection {
    fn default() -> Self {
        Self::Horizontal
    }
}

/// Space 组件的对齐方式
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SpaceAlign {
    /// 起始对齐
    Start,
    /// 结束对齐
    End,
    /// 居中对齐
    Center,
    /// 基线对齐
    Baseline,
}

impl Default for SpaceAlign {
    fn default() -> Self {
        Self::Start
    }
}

impl SpaceAlign {
    /// 获取CSS类名
    pub fn to_class(&self) -> String {
        match self {
            SpaceAlign::Start => "ant-space-align-start".to_string(),
            SpaceAlign::End => "ant-space-align-end".to_string(),
            SpaceAlign::Center => "ant-space-align-center".to_string(),
            SpaceAlign::Baseline => "ant-space-align-baseline".to_string(),
        }
    }
}

/// Space 组件的属性
#[derive(Props, Clone, PartialEq)]
pub struct SpaceProps {
    /// 间距方向
    #[props(default)]
    pub direction: SpaceDirection,

    /// 间距大小
    #[props(default)]
    pub size: SpaceSize,

    /// 对齐方式
    #[props(default)]
    pub align: SpaceAlign,

    /// 是否自动换行，仅在 horizontal 时有效
    #[props(default = false)]
    pub wrap: bool,

    /// 设置拆分
    #[props(default)]
    pub split: Option<Element>,

    /// 自定义CSS类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,

    /// 子元素
    pub children: Element,
}

/// Space 间距组件
///
/// 设置组件之间的间距，避免组件紧贴在一起。
///
/// # Props
/// - `direction`: 间距方向，默认为水平
/// - `size`: 间距大小，默认为中等
/// - `align`: 对齐方式，默认为起始对齐
/// - `wrap`: 是否自动换行，仅在水平方向时有效
/// - `split`: 设置拆分元素
/// - `class`: 自定义CSS类名
/// - `style`: 自定义样式
/// - `children`: 子元素
///
/// # 示例
///
/// ```rust
/// use dioxus::prelude::*;
/// use ant_design_dioxus::{Space, SpaceSize, SpaceDirection};
///
/// fn app() -> Element {
///     rsx! {
///         Space {
///             size: SpaceSize::Large,
///             direction: SpaceDirection::Vertical,
///             Button { "按钮1" }
///             Button { "按钮2" }
///             Button { "按钮3" }
///         }
///     }
/// }
/// ```
#[component]
pub fn Space(props: SpaceProps) -> Element {
    let mut classes = vec!["ant-space".to_string()];

    // 添加方向类
    match props.direction {
        SpaceDirection::Horizontal => classes.push("ant-space-horizontal".to_string()),
        SpaceDirection::Vertical => classes.push("ant-space-vertical".to_string()),
    }

    // 添加尺寸类
    let size_class = match props.size {
        SpaceSize::Small => "ant-space-small".to_string(),
        SpaceSize::Middle => "ant-space-middle".to_string(),
        SpaceSize::Large => "ant-space-large".to_string(),
        SpaceSize::Custom(_) => "ant-space-custom".to_string(),
        _ => "ant-space-small".to_string(),
    };
    classes.push(size_class);

    // 添加对齐类
    classes.push(props.align.to_class());

    // 添加换行类
    if props.wrap {
        classes.push("ant-space-wrap".to_string());
    }

    // 添加自定义类
    if let Some(class) = &props.class {
        classes.push(class.clone());
    }

    let class_str = classes.join(" ");

    // 构建自定义样式
    let mut style_parts = Vec::new();

    // 如果是自定义尺寸，添加CSS变量
    if let SpaceSize::Custom(size) = props.size {
        let gap_value = format!("{}px", size);
        style_parts.push(format!("--ant-space-gap: {}", gap_value));
    }

    if let Some(style) = &props.style {
        style_parts.push(style.clone());
    }

    let style_str = if style_parts.is_empty() {
        None
    } else {
        Some(style_parts.join("; "))
    };

    rsx! {
        style { {SPACE_STYLES} }

        div {
            class: class_str.clone(),
            style: style_str,
            {props.children}
        }
    }
}

// 重新导出公共类型
// 注意：不使用通配符导出以避免命名冲突

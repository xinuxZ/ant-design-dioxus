//! # Flex 弹性布局
//!
//! 弹性布局组件，基于 CSS Flexbox 实现。
//!
//! ## 何时使用
//!
//! - 需要快速实现弹性布局
//! - 需要对齐、分布子元素
//! - 需要响应式的布局方案
//!
//! ## 示例
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::{Flex, FlexJustify, FlexAlign};
//!
//! fn app() -> Element {
//!     rsx! {
//!         Flex {
//!             justify: FlexJustify::SpaceBetween,
//!             align: FlexAlign::Center,
//!             gap: "middle",
//!             div { "Item 1" }
//!             div { "Item 2" }
//!             div { "Item 3" }
//!         }
//!     }
//! }
//! ```
//!
//! ### 垂直布局
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::{Flex, FlexDirection};
//!
//! fn app() -> Element {
//!     rsx! {
//!         Flex {
//!             vertical: true,
//!             gap: "large",
//!             div { "Item 1" }
//!             div { "Item 2" }
//!             div { "Item 3" }
//!         }
//!     }
//! }
//! ```
//!
//! ### 响应式布局
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::Flex;
//!
//! fn app() -> Element {
//!     rsx! {
//!         Flex {
//!             wrap: "wrap",
//!             gap: "small",
//!             div { flex: "1 1 300px", "Responsive Item 1" }
//!             div { flex: "1 1 300px", "Responsive Item 2" }
//!             div { flex: "1 1 300px", "Responsive Item 3" }
//!         }
//!     }
//! }
//! ```

use dioxus::prelude::*;

mod styles;
use styles::{
    register_styles, FlexAlign, FlexDirection, FlexGap, FlexJustify, FlexStyleGenerator, FlexWrap,
};

/// Flex 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct FlexProps {
    /// 是否垂直布局
    #[props(default = false)]
    pub vertical: bool,

    /// 主轴对齐方式
    #[props(default)]
    pub justify: FlexJustify,

    /// 交叉轴对齐方式
    #[props(default)]
    pub align: FlexAlign,

    /// 是否换行
    #[props(default)]
    pub wrap: FlexWrap,

    /// 间距大小
    #[props(default)]
    pub gap: Option<FlexGap>,

    /// 自定义 flex 属性
    #[props(default)]
    pub flex: Option<String>,

    /// 自定义 CSS 类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,

    pub direction: FlexDirection, // 新增属性：FlexDirection 枚举，用于指定主轴方向，默认值为 FlexDirection::Row，可选值为 FlexDirection::Row 和 FlexDirection::Column，分别表示水平和垂直方向

    // ... 其他属性，如 flex、class、style 等，根据需要添加
    /// 子元素
    children: Element,
}

/// 注册 Flex 组件样式
pub fn register_flex_styles() -> String {
    register_styles()
}

/// Flex 弹性布局组件
///
/// 基于 CSS Flexbox 的弹性布局组件。
#[component]
pub fn Flex(props: FlexProps) -> Element {
    // 使用样式生成器生成类名
    let style_generator = FlexStyleGenerator::new()
        .with_vertical(props.vertical)
        .with_justify(props.justify.clone())
        .with_align(props.align.clone())
        .with_wrap(props.wrap.clone())
        .with_gap(props.gap.clone());

    let class_name = style_generator.generate();

    // 添加自定义类名
    let class_name = if let Some(custom_class) = &props.class {
        format!("{} {}", class_name, custom_class)
    } else {
        class_name
    };

    // 构建样式
    let mut style_parts = Vec::new();

    // 添加 flex 属性
    if let Some(flex_value) = &props.flex {
        style_parts.push(format!("flex: {}", flex_value));
    }

    // 添加间距样式（自定义间距）
    if let Some(FlexGap::Custom(px)) = &props.gap {
        style_parts.push(format!("gap: {}px", px));
    }

    // 添加自定义样式
    if let Some(custom_style) = &props.style {
        style_parts.push(custom_style.clone());
    }

    let style_attr = if style_parts.is_empty() {
        None
    } else {
        Some(style_parts.join("; "))
    };

    rsx! {
        // 注册样式
        style { {register_flex_styles()} }

        div {
            class: class_name,
            style: style_attr,
            {props.children}
        }
    }
}

// 重新导出公共类型
// 注意：不使用通配符导出以避免命名冲突

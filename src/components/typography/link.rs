//! # Link 链接
//!
//! 文字超链接组件。
//!
//! ## 何时使用
//!
//! - 需要跳转到其他页面或锚点
//! - 需要触发某些操作的文字链接
//! - 需要在文本中嵌入可点击的链接
//!
//! ## 示例
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::{Link, LinkType};
//!
//! fn app() -> Element {
//!     rsx! {
//!         div {
//!             Link {
//!                 href: "https://ant.design",
//!                 "Ant Design"
//!             }
//!             br {}
//!             Link {
//!                 r#type: LinkType::Secondary,
//!                 href: "#",
//!                 "Secondary Link"
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! ### 不同状态的链接
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::{Link, LinkType};
//!
//! fn app() -> Element {
//!     rsx! {
//!         div {
//!             Link { href: "#", "Default Link" }
//!             br {}
//!             Link {
//!                 r#type: LinkType::Success,
//!                 href: "#",
//!                 "Success Link"
//!             }
//!             br {}
//!             Link {
//!                 r#type: LinkType::Warning,
//!                 href: "#",
//!                 "Warning Link"
//!             }
//!             br {}
//!             Link {
//!                 r#type: LinkType::Danger,
//!                 href: "#",
//!                 "Danger Link"
//!             }
//!             br {}
//!             Link {
//!                 disabled: true,
//!                 href: "#",
//!                 "Disabled Link"
//!             }
//!         }
//!     }
//! }
//! ```

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

/// 链接类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LinkType {
    /// 默认链接
    Default,
    /// 次要链接
    Secondary,
    /// 成功链接
    Success,
    /// 警告链接
    Warning,
    /// 危险链接
    Danger,
}

impl Default for LinkType {
    fn default() -> Self {
        Self::Default
    }
}

impl LinkType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Secondary => "secondary",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Danger => "danger",
        }
    }
}

/// 链接目标
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LinkTarget {
    /// 当前窗口
    Self_,
    /// 新窗口
    Blank,
    /// 父窗口
    Parent,
    /// 顶层窗口
    Top,
}

impl LinkTarget {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Self_ => "_self",
            Self::Blank => "_blank",
            Self::Parent => "_parent",
            Self::Top => "_top",
        }
    }
}

/// Link 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct LinkProps {
    /// 链接类型
    #[props(default)]
    pub r#type: LinkType,

    /// 链接地址
    #[props(default)]
    pub href: Option<String>,

    /// 链接目标
    #[props(default)]
    pub target: Option<LinkTarget>,

    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,

    /// 是否显示下划线
    #[props(default = true)]
    pub underline: bool,

    /// 是否强制显示下划线（即使在 hover 状态）
    #[props(default = false)]
    pub strong: bool,

    /// 是否为块级元素
    #[props(default = false)]
    pub block: bool,

    /// 点击事件
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,

    /// 自定义 CSS 类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,

    /// 子元素
    children: Element,
}

/// Link 链接组件
///
/// 文字超链接组件。
#[component]
pub fn Link(props: LinkProps) -> Element {
    let mut class_list = vec!["ant-typography", "ant-typography-link"];

    // 添加类型样式
    class_list.push(match props.r#type {
        LinkType::Default => "ant-typography-link-default",
        LinkType::Secondary => "ant-typography-link-secondary",
        LinkType::Success => "ant-typography-link-success",
        LinkType::Warning => "ant-typography-link-warning",
        LinkType::Danger => "ant-typography-link-danger",
    });

    // 添加状态样式
    if props.disabled {
        class_list.push("ant-typography-link-disabled");
    }

    if !props.underline {
        class_list.push("ant-typography-link-no-underline");
    }

    if props.strong {
        class_list.push("ant-typography-link-strong");
    }

    if props.block {
        class_list.push("ant-typography-link-block");
    }

    // 添加自定义类名
    if let Some(custom_class) = &props.class {
        class_list.push(custom_class);
    }

    let class_name = class_list.join(" ");

    // 处理点击事件
    let handle_click = move |evt: MouseEvent| {
        if props.disabled {
            evt.prevent_default();
            return;
        }

        if let Some(onclick) = &props.onclick {
            onclick.call(evt);
        }
    };

    // 构建链接属性
    let href = if props.disabled {
        None
    } else {
        props.href.as_deref()
    };

    let target = props.target.as_ref().map(|t| t.as_str());

    // 如果是外部链接且在新窗口打开，添加安全属性
    let rel = if let (Some(href_val), Some(LinkTarget::Blank)) = (&props.href, &props.target) {
        if href_val.starts_with("http") {
            Some("noopener noreferrer")
        } else {
            None
        }
    } else {
        None
    };

    rsx! {
        a {
            class: class_name.clone(),
            style: props.style,
            href: href,
            target: target,
            rel: rel,
            onclick: handle_click,
            {props.children}
        }
    }
}

// 重新导出公共类型
// 注意：不使用通配符导出以避免命名冲突

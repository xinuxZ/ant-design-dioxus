//! Breadcrumb 面包屑组件
//!
//! 显示当前页面在系统层级结构中的位置，并能向上返回。
//!
//! ## 何时使用
//!
//! - 当系统拥有超过两级以上的层级结构时；
//! - 当需要告知用户『你在哪里』时；
//! - 当需要向上导航的功能时。
//!
//! ## 代码演示
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::{Breadcrumb, BreadcrumbItem};
//!
//! #[component]
//! fn App() -> Element {
//!     rsx! {
//!         Breadcrumb {
//!             BreadcrumbItem {
//!                 href: "/",
//!                 "首页"
//!             }
//!             BreadcrumbItem {
//!                 href: "/list",
//!                 "列表页"
//!             }
//!             BreadcrumbItem {
//!                 "详情页"
//!             }
//!         }
//!     }
//! }
//! ```

use dioxus::prelude::*;

/// 面包屑项数据结构
#[derive(Debug, Clone, PartialEq)]
pub struct BreadcrumbItemData {
    /// 链接地址
    pub href: Option<String>,
    /// 标题
    pub title: String,
    /// 图标
    pub icon: Option<String>,
    /// 是否禁用
    pub disabled: bool,
    /// 点击事件
    pub onclick: Option<String>,
}

/// BreadcrumbItem 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct BreadcrumbItemProps {
    /// 链接地址
    pub href: Option<String>,
    /// 图标
    pub icon: Option<String>,
    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,
    /// 点击事件回调
    pub onclick: Option<EventHandler<()>>,
    /// 自定义样式类名
    #[props(default = String::new())]
    pub class: String,
    /// 自定义内联样式
    #[props(default = String::new())]
    pub style: String,
    /// 子元素
    pub children: Element,
}

/// Breadcrumb 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct BreadcrumbProps {
    /// 面包屑项数据
    #[props(default = vec![])]
    pub items: Vec<BreadcrumbItemData>,
    /// 分隔符
    #[props(default = "/".to_string())]
    pub separator: String,
    /// 自定义分隔符元素
    pub separator_element: Option<Element>,
    /// 自定义样式类名
    #[props(default = String::new())]
    pub class: String,
    /// 自定义内联样式
    #[props(default = String::new())]
    pub style: String,
    /// 子元素（BreadcrumbItem）
    pub children: Element,
}

/// BreadcrumbItem 面包屑项组件
#[component]
pub fn BreadcrumbItem(props: BreadcrumbItemProps) -> Element {
    let handle_click = move |_| {
        if !props.disabled {
            if let Some(handler) = &props.onclick {
                handler.call(());
            }
        }
    };

    rsx! {
        li {
            class: format!(
                "ant-breadcrumb-item {}",
                props.class
            ),
            style: "{props.style}",

            if let Some(href) = &props.href {
                if !props.disabled {
                    a {
                        href: "{href}",
                        class: "ant-breadcrumb-link",
                        onclick: handle_click,

                        if let Some(icon) = &props.icon {
                            span {
                                class: "ant-breadcrumb-icon",
                                "{icon}"
                            }
                        }

                        span {
                            class: "ant-breadcrumb-text",
                            {props.children}
                        }
                    }
                } else {
                    span {
                        class: "ant-breadcrumb-link ant-breadcrumb-link-disabled",

                        if let Some(icon) = &props.icon {
                            span {
                                class: "ant-breadcrumb-icon",
                                "{icon}"
                            }
                        }

                        span {
                            class: "ant-breadcrumb-text",
                            {props.children}
                        }
                    }
                }
            } else {
                span {
                    class: format!(
                        "ant-breadcrumb-link {}",
                        if props.disabled { "ant-breadcrumb-link-disabled" } else { "" }
                    ),
                    onclick: handle_click,

                    if let Some(icon) = &props.icon {
                        span {
                            class: "ant-breadcrumb-icon",
                            "{icon}"
                        }
                    }

                    span {
                        class: "ant-breadcrumb-text",
                        {props.children}
                    }
                }
            }
        }
    }
}

/// Breadcrumb 面包屑组件
///
/// 显示当前页面在系统层级结构中的位置，并能向上返回。
#[component]
pub fn Breadcrumb(props: BreadcrumbProps) -> Element {
    // 处理子元素，在每个项之间插入分隔符
    let render_children_with_separators = || {
        rsx! {
            {props.children.clone()}
        }
    };

    // 如果提供了 items 数据，使用数据渲染
    let render_items = || {
        rsx! {
            for (index, item) in props.items.iter().enumerate() {
                BreadcrumbItem {
                    key: "{index}",
                    href: item.href.clone(),
                    icon: item.icon.clone(),
                    disabled: item.disabled,
                    "{item.title}"
                }

                // 如果不是最后一个元素，添加分隔符
                if index < props.items.len() - 1 {
                    li {
                        key: "separator-{index}",
                        class: "ant-breadcrumb-separator",
                        "aria-hidden": "true",

                        if let Some(separator_elem) = &props.separator_element {
                            {separator_elem.clone()}
                        } else {
                            "{props.separator}"
                        }
                    }
                }
            }
        }
    };

    rsx! {
        nav {
            class: format!("ant-breadcrumb {}", props.class),
            style: "{props.style}",
            "aria-label": "Breadcrumb",

            ol {
                class: "ant-breadcrumb-list",

                // 如果有 items 数据，使用数据渲染，否则使用子元素
                if !props.items.is_empty() {
                    {render_items()}
                } else {
                    {render_children_with_separators()}
                }
            }
        }
    }
}

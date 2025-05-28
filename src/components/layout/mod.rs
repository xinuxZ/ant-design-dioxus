//! Layout 布局组件
//!
//! 协助进行页面级整体布局。
//!
//! ## 设计规则
//!
//! ### 尺寸
//!
//! 一级导航项偏左靠近 logo 放置，辅助菜单偏右放置。
//!
//! - 顶部导航（大部分系统）：一级导航高度 `64px`，二级导航 `48px`。
//! - 顶部导航（展示类页面）：一级导航高度 `80px`，二级导航 `56px`。
//! - 顶部导航高度的范围计算公式为：`48+8n`。
//! - 侧边导航宽度的范围计算公式：`200+8n`。
//!
//! ### 交互
//!
//! - 一级导航和末级的导航需要在可视化的层面被强调出来；
//! - 当前项应该在呈现上优先级最高；
//! - 当导航收起的时候，当前项的样式自动赋予给它的上一个层级；
//! - 左侧导航栏的收放交互同时支持手风琴和全展开的样式，手风琴是默认推荐样式；
//! - 二级导航的操作方式：点击和悬停。
//!
//! ## 组件概览
//!
//! - `Layout`：布局容器，其下可嵌套 `Header` `Sider` `Content` `Footer` 或 `Layout` 本身，可以放在任何父容器中。
//! - `Header`：顶部布局，自带默认样式，其下可嵌套任何元素，只能放在 `Layout` 中。
//! - `Sider`：侧边栏，自带默认样式及基本功能，其下可嵌套任何元素，只能放在 `Layout` 中。
//! - `Content`：内容部分，自带默认样式，其下可嵌套任何元素，只能放在 `Layout` 中。
//! - `Footer`：底部布局，自带默认样式，其下可嵌套任何元素，只能放在 `Layout` 中。
//!
//! > 注意：采用 flex 布局实现，请注意[浏览器兼容性](http://caniuse.com/#search=flex)问题。

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

// 引入布局样式
const LAYOUT_STYLE: &str = include_str!("layout.css");
const HEADER_STYLE: &str = include_str!("header.css");
const SIDER_STYLE: &str = include_str!("sider.css");
const CONTENT_STYLE: &str = include_str!("content.css");
const FOOTER_STYLE: &str = include_str!("footer.css");

/// 侧边栏主题
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SiderTheme {
    /// 亮色主题
    Light,
    /// 暗色主题
    Dark,
}

impl Default for SiderTheme {
    fn default() -> Self {
        Self::Dark
    }
}

/// 侧边栏断点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SiderBreakpoint {
    /// 屏幕 < 576px
    Xs,
    /// 屏幕 ≥ 576px
    Sm,
    /// 屏幕 ≥ 768px
    Md,
    /// 屏幕 ≥ 992px
    Lg,
    /// 屏幕 ≥ 1200px
    Xl,
    /// 屏幕 ≥ 1600px
    Xxl,
}

/// Layout 布局容器属性
#[derive(Props, Clone, PartialEq)]
pub struct LayoutProps {
    /// 表示子元素里有 Sider，一般不用指定。可用于服务端渲染时避免样式闪动
    #[props(default = false)]
    pub has_sider: bool,

    /// 自定义类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,

    /// 子元素
    children: Element,
}

/// Header 顶部布局属性
#[derive(Props, Clone, PartialEq)]
pub struct HeaderProps {
    /// 自定义类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,

    /// 子元素
    children: Element,
}

/// Sider 侧边栏属性
#[derive(Props, Clone, PartialEq)]
pub struct SiderProps {
    /// 当前收起状态
    #[props(default = false)]
    pub collapsed: bool,

    /// 是否可收起
    #[props(default = false)]
    pub collapsible: bool,

    /// 收起状态时宽度
    #[props(default = 80)]
    pub collapsed_width: u32,

    /// 宽度
    #[props(default = 200)]
    pub width: u32,

    /// 主题颜色
    #[props(default)]
    pub theme: SiderTheme,

    /// 触发响应式布局的断点
    #[props(default)]
    pub breakpoint: Option<SiderBreakpoint>,

    /// 收起展开时的回调函数
    #[props(default)]
    pub on_collapse: Option<EventHandler<bool>>,

    /// 断点触发时的回调
    #[props(default)]
    pub on_breakpoint: Option<EventHandler<bool>>,

    /// 自定义类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,

    /// 子元素
    children: Element,
}

/// Content 内容部分属性
#[derive(Props, Clone, PartialEq)]
pub struct ContentProps {
    /// 自定义类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,

    /// 子元素
    children: Element,
}

/// Footer 底部布局属性
#[derive(Props, Clone, PartialEq)]
pub struct FooterProps {
    /// 自定义类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,

    /// 子元素
    children: Element,
}

/// 获取布局容器的类名
///
/// # 参数
///
/// * `has_sider` - 是否包含侧边栏
/// * `class` - 自定义类名
///
/// # 返回值
///
/// 返回完整的CSS类名字符串
fn get_layout_class_name(has_sider: bool, class: Option<&str>) -> String {
    let mut class_names = vec!["ant-layout"];

    if has_sider {
        class_names.push("ant-layout-has-sider");
    }

    if let Some(custom_class) = class {
        class_names.push(custom_class);
    }

    class_names.join(" ")
}

/// 获取侧边栏的类名
///
/// # 参数
///
/// * `collapsed` - 是否收起
/// * `theme` - 主题
/// * `class` - 自定义类名
///
/// # 返回值
///
/// 返回完整的CSS类名字符串
fn get_sider_class_name(collapsed: bool, theme: &SiderTheme, class: Option<&str>) -> String {
    let mut class_names = vec!["ant-layout-sider"];

    if collapsed {
        class_names.push("ant-layout-sider-collapsed");
    }

    match theme {
        SiderTheme::Light => class_names.push("ant-layout-sider-light"),
        SiderTheme::Dark => class_names.push("ant-layout-sider-dark"),
    }

    if let Some(custom_class) = class {
        class_names.push(custom_class);
    }

    class_names.join(" ")
}

/// 获取侧边栏的样式
///
/// # 参数
///
/// * `collapsed` - 是否收起
/// * `width` - 宽度
/// * `collapsed_width` - 收起时宽度
/// * `style` - 自定义样式
///
/// # 返回值
///
/// 返回完整的CSS样式字符串
fn get_sider_style(
    collapsed: bool,
    width: u32,
    collapsed_width: u32,
    style: Option<&str>,
) -> String {
    let mut styles = Vec::new();

    let current_width = if collapsed { collapsed_width } else { width };
    styles.push(format!("width: {}px", current_width));
    styles.push(format!("min-width: {}px", current_width));
    styles.push(format!("max-width: {}px", current_width));
    styles.push(format!("flex: 0 0 {}px", current_width));

    if let Some(custom_style) = style {
        styles.push(custom_style.to_string());
    }

    styles.join("; ")
}

/// Layout 布局容器组件
///
/// # Props
///
/// * `has_sider` - 表示子元素里有 Sider，一般不用指定
/// * `class` - 自定义类名
/// * `style` - 自定义样式
/// * `children` - 子元素
///
/// # 示例
///
/// ```rust
/// use ant_design_dioxus::prelude::*;
/// use dioxus::prelude::*;
///
/// #[component]
/// fn App() -> Element {
///     rsx! {
///         Layout {
///             Header {
///                 "Header"
///             }
///             Content {
///                 "Content"
///             }
///             Footer {
///                 "Footer"
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn Layout(props: LayoutProps) -> Element {
    rsx! {
        style { {LAYOUT_STYLE} }
        section {
            class: get_layout_class_name(props.has_sider, props.class.as_deref()),
            style: props.style.unwrap_or_default(),
            {props.children}
        }
    }
}

/// Header 顶部布局组件
///
/// # Props
///
/// * `class` - 自定义类名
/// * `style` - 自定义样式
/// * `children` - 子元素
///
/// # 示例
///
/// ```rust
/// use ant_design_dioxus::prelude::*;
/// use dioxus::prelude::*;
///
/// #[component]
/// fn App() -> Element {
///     rsx! {
///         Layout {
///             Header {
///                 style: "background: #fff; padding: 0 24px;",
///                 "My Header"
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn Header(props: HeaderProps) -> Element {
    let class_name = if let Some(class) = &props.class {
        format!("ant-layout-header {}", class)
    } else {
        "ant-layout-header".to_string()
    };

    rsx! {
        style { {HEADER_STYLE} }
        header {
            class: class_name,
            style: props.style.unwrap_or_default(),
            {props.children}
        }
    }
}

/// Sider 侧边栏组件
///
/// # Props
///
/// * `collapsed` - 当前收起状态
/// * `collapsible` - 是否可收起
/// * `collapsed_width` - 收起状态时宽度
/// * `width` - 宽度
/// * `theme` - 主题颜色
/// * `breakpoint` - 触发响应式布局的断点
/// * `on_collapse` - 收起展开时的回调函数
/// * `on_breakpoint` - 断点触发时的回调
/// * `class` - 自定义类名
/// * `style` - 自定义样式
/// * `children` - 子元素
///
/// # 示例
///
/// ```rust
/// use ant_design_dioxus::prelude::*;
/// use dioxus::prelude::*;
///
/// #[component]
/// fn App() -> Element {
///     let mut collapsed = use_signal(|| false);
///
///     rsx! {
///         Layout {
///             Sider {
///                 collapsed: collapsed(),
///                 collapsible: true,
///                 on_collapse: move |value| collapsed.set(value),
///                 "Sider Content"
///             }
///             Content {
///                 "Main Content"
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn Sider(props: SiderProps) -> Element {
    rsx! {
        style { {SIDER_STYLE} }
        aside {
            class: get_sider_class_name(props.collapsed, &props.theme, props.class.as_deref()),
            style: get_sider_style(props.collapsed, props.width, props.collapsed_width, props.style.as_deref()),

            // 侧边栏内容
            div {
                class: "ant-layout-sider-children",
                {props.children}
            }

            // 收起按钮（如果可收起）
            if props.collapsible {
                div {
                    class: "ant-layout-sider-trigger",
                    onclick: move |_| {
                        if let Some(on_collapse) = &props.on_collapse {
                            on_collapse.call(!props.collapsed);
                        }
                    },
                    if props.collapsed {
                        "▶"
                    } else {
                        "◀"
                    }
                }
            }
        }
    }
}

/// Content 内容部分组件
///
/// # Props
///
/// * `class` - 自定义类名
/// * `style` - 自定义样式
/// * `children` - 子元素
///
/// # 示例
///
/// ```rust
/// use ant_design_dioxus::prelude::*;
/// use dioxus::prelude::*;
///
/// #[component]
/// fn App() -> Element {
///     rsx! {
///         Layout {
///             Content {
///                 style: "padding: 24px; background: #fff;",
///                 "Main content area"
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn Content(props: ContentProps) -> Element {
    let class_name = if let Some(class) = &props.class {
        format!("ant-layout-content {}", class)
    } else {
        "ant-layout-content".to_string()
    };

    rsx! {
        style { {CONTENT_STYLE} }
        main {
            class: class_name,
            style: props.style.unwrap_or_default(),
            {props.children}
        }
    }
}

/// Footer 底部布局组件
///
/// # Props
///
/// * `class` - 自定义类名
/// * `style` - 自定义样式
/// * `children` - 子元素
///
/// # 示例
///
/// ```rust
/// use ant_design_dioxus::prelude::*;
/// use dioxus::prelude::*;
///
/// #[component]
/// fn App() -> Element {
///     rsx! {
///         Layout {
///             Footer {
///                 style: "text-align: center;",
///                 "Ant Design Dioxus ©2024 Created by Ant UED"
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn Footer(props: FooterProps) -> Element {
    let class_name = if let Some(class) = &props.class {
        format!("ant-layout-footer {}", class)
    } else {
        "ant-layout-footer".to_string()
    };

    rsx! {
        style { {FOOTER_STYLE} }
        footer {
            class: class_name,
            style: props.style.unwrap_or_default(),
            {props.children}
        }
    }
}

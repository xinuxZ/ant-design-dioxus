//! Layout 布局组件
//!
//! 协助进行页面级整体布局。
//!
//! ## 设计规则
//!
//! ### 尺寸
//!
//! 一级导航项偏左靠近 logo 放置，辅助菜单偏右放置。
//! 顶部导航（大部分系统）：一级导航高度 64px，二级导航 48px。
//! 顶部导航（展示类页面）：一级导航高度 80px，二级导航 56px。
//! 顶部导航高度的范围计算公式为：48+8n。
//! 侧边导航宽度的范围计算公式：200+8n。
//!
//! ### 交互
//!
//! 一级导航和末级的导航需要在可视化的层面被强调出来；
//! 当前项应该在呈现上优先级最高；
//! 当导航收起的时候，当前项的样式自动赋予给它的上一个层级；
//! 左侧导航栏的收放交互同时支持手风琴和全展开的样式，根据业务的要求进行适当的选择。
//!
//! ## 组件概览
//!
//! - Layout：布局容器，其下可嵌套 Header Sider Content Footer 或 Layout 本身，可以放在任何父容器中。
//! - Header：顶部布局，自带默认样式，其下可嵌套任何元素，只能放在 Layout 中。
//! - Sider：侧边栏，自带默认样式及基本功能，其下可嵌套任何元素，只能放在 Layout 中。
//! - Content：内容部分，自带默认样式，其下可嵌套任何元素，只能放在 Layout 中。
//! - Footer：底部布局，自带默认样式，其下可嵌套任何元素，只能放在 Layout 中。

mod styles;

use css_in_rust::css;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use styles::{
    ContentStyleGenerator, FooterStyleGenerator, HeaderStyleGenerator, LayoutStyleGenerator,
    SiderStyleGenerator,
};

/// Sider主题
#[derive(Debug, Clone, PartialEq)]
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

/// Sider响应式断点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SiderBreakpoint {
    /// 超小屏幕 < 576px
    Xs,
    /// 小屏幕 ≥ 576px
    Sm,
    /// 中等屏幕 ≥ 768px
    Md,
    /// 大屏幕 ≥ 992px
    Lg,
    /// 超大屏幕 ≥ 1200px
    Xl,
    /// 超超大屏幕 ≥ 1600px
    Xxl,
}

/// Layout属性
#[derive(Props, Clone, PartialEq)]
pub struct LayoutProps {
    /// 是否包含Sider
    #[props(default = false)]
    pub has_sider: bool,

    /// 自定义类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,

    /// 子元素
    pub children: Element,
}

/// Header属性
#[derive(Props, Clone, PartialEq)]
pub struct HeaderProps {
    /// 是否为亮色主题
    #[props(default = false)]
    pub light: bool,

    /// 是否固定头部
    #[props(default = false)]
    pub fixed: bool,

    /// 自定义类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,

    /// 子元素
    pub children: Element,
}

/// Sider属性
#[derive(Props, Clone, PartialEq)]
pub struct SiderProps {
    /// 主题
    #[props(default)]
    pub theme: SiderTheme,

    /// 宽度
    #[props(default = 200)]
    pub width: u32,

    /// 收缩宽度，默认为80px
    #[props(default = 80)]
    pub collapsed_width: u32,

    /// 是否可收起
    #[props(default = false)]
    pub collapsible: bool,

    /// 是否收起
    #[props(default = false)]
    pub collapsed: bool,

    /// 触发响应式布局的断点
    #[props(default)]
    pub breakpoint: Option<SiderBreakpoint>,

    /// 收起按钮的位置
    #[props(default = true)]
    pub left: bool,

    /// 自定义类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,

    /// 收起时的回调函数
    #[props(default)]
    pub on_collapse: Option<EventHandler<bool>>,

    /// 子元素
    pub children: Element,
}

/// Content属性
#[derive(Props, Clone, PartialEq)]
pub struct ContentProps {
    /// 内边距大小
    #[props(default)]
    pub padding: Option<String>,

    /// 背景色
    #[props(default)]
    pub background: Option<String>,

    /// 是否有边框
    #[props(default = false)]
    pub bordered: bool,

    /// 是否有阴影
    #[props(default = false)]
    pub shadow: bool,

    /// 自定义类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,

    /// 子元素
    pub children: Element,
}

/// Footer属性
#[derive(Props, Clone, PartialEq)]
pub struct FooterProps {
    /// 主题
    #[props(default)]
    pub theme: Option<String>,

    /// 自定义类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,

    /// 子元素
    pub children: Element,
}

/// 注册全局样式
fn register_styles() {
    use_effect(|| {
        let _ = css!(LayoutStyleGenerator::base_style());
        let _ = css!(HeaderStyleGenerator::base_style());
        let _ = css!(SiderStyleGenerator::base_style());
        let _ = css!(ContentStyleGenerator::base_style());
        let _ = css!(FooterStyleGenerator::base_style());
    });
}

/// Layout组件
///
/// # 参数
///
/// * `props` - Layout属性
///
/// # 示例
///
/// ```rust
/// use dioxus::prelude::*;
/// use ant_design_dioxus::{Layout, Header, Content, Footer};
///
/// fn app() -> Element {
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
    register_styles();

    let layout_class = LayoutStyleGenerator::new()
        .with_has_sider(props.has_sider)
        .generate();

    let class = format!(
        "{} {}",
        layout_class,
        props.class.clone().unwrap_or_default()
    );

    let style = props.style.clone().unwrap_or_default();

    rsx! {
        section {
            class: class,
            style: style,
            {props.children}
        }
    }
}

/// Header组件
///
/// # 参数
///
/// * `props` - Header属性
///
/// # 示例
///
/// ```rust
/// use dioxus::prelude::*;
/// use ant_design_dioxus::{Layout, Header};
///
/// fn app() -> Element {
///     rsx! {
///         Layout {
///             Header {
///                 light: true,
///                 "Header"
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn Header(props: HeaderProps) -> Element {
    register_styles();

    let header_class = HeaderStyleGenerator::new()
        .with_light(props.light)
        .with_fixed(props.fixed)
        .generate();

    let class = format!(
        "{} {}",
        header_class,
        props.class.clone().unwrap_or_default()
    );

    let style = props.style.clone().unwrap_or_default();

    rsx! {
        header {
            class: class,
            style: style,
            {props.children}
        }
    }
}

/// Sider组件
///
/// # 参数
///
/// * `props` - Sider属性
///
/// # 示例
///
/// ```rust
/// use dioxus::prelude::*;
/// use ant_design_dioxus::{Layout, Sider, Content};
///
/// fn app() -> Element {
///     rsx! {
///         Layout {
///             has_sider: true,
///             Sider {
///                 collapsible: true,
///                 "Sider"
///             }
///             Content {
///                 "Content"
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn Sider(props: SiderProps) -> Element {
    register_styles();

    // 将SiderTheme转换为styles::SiderTheme
    let theme = match props.theme {
        SiderTheme::Light => styles::SiderTheme::Light,
        SiderTheme::Dark => styles::SiderTheme::Dark,
    };

    let sider_class = SiderStyleGenerator::new()
        .with_theme(theme)
        .with_collapsible(props.collapsible)
        .with_collapsed(props.collapsed)
        .with_has_trigger(props.collapsible)
        .with_left(props.left)
        .generate();

    let class = format!(
        "{} {}",
        sider_class,
        props.class.clone().unwrap_or_default()
    );

    let width = if props.collapsed {
        props.collapsed_width
    } else {
        props.width
    };

    let style = format!(
        "width: {}px; min-width: {}px; max-width: {}px; {}",
        width,
        width,
        width,
        props.style.clone().unwrap_or_default()
    );

    rsx! {
        aside {
            class: class,
            style: style,
            {props.children}

            // 折叠触发器
            if props.collapsible {
                div {
                    class: "ant-layout-sider-trigger",
                    style: format!("width: {}px", width),
                    onclick: move |_| {
                        if let Some(handler) = &props.on_collapse {
                            handler.call(!props.collapsed);
                        }
                    },
                    // TODO: 添加折叠图标
                    if props.collapsed { "→" } else { "←" }
                }
            }
        }
    }
}

/// Content组件
///
/// # 参数
///
/// * `props` - Content属性
///
/// # 示例
///
/// ```rust
/// use dioxus::prelude::*;
/// use ant_design_dioxus::{Layout, Content};
///
/// fn app() -> Element {
///     rsx! {
///         Layout {
///             Content {
///                 padding: Some("large".to_string()),
///                 background: Some("white".to_string()),
///                 "Content"
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn Content(props: ContentProps) -> Element {
    register_styles();

    let content_class = ContentStyleGenerator::new()
        .with_padding(props.padding.clone())
        .with_background(props.background.clone())
        .with_bordered(props.bordered)
        .with_shadow(props.shadow)
        .generate();

    let class = format!(
        "{} {}",
        content_class,
        props.class.clone().unwrap_or_default()
    );

    let style = props.style.clone().unwrap_or_default();

    rsx! {
        main {
            class: class,
            style: style,
            {props.children}
        }
    }
}

/// Footer组件
///
/// # 参数
///
/// * `props` - Footer属性
///
/// # 示例
///
/// ```rust
/// use dioxus::prelude::*;
/// use ant_design_dioxus::{Layout, Footer};
///
/// fn app() -> Element {
///     rsx! {
///         Layout {
///             Footer {
///                 theme: Some("light".to_string()),
///                 "Footer"
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn Footer(props: FooterProps) -> Element {
    register_styles();

    let footer_class = FooterStyleGenerator::new()
        .with_theme(props.theme.clone())
        .generate();

    let class = format!(
        "{} {}",
        footer_class,
        props.class.clone().unwrap_or_default()
    );

    let style = props.style.clone().unwrap_or_default();

    rsx! {
        footer {
            class: class,
            style: style,
            {props.children}
        }
    }
}

//! Button 按钮组件
//!
//! 按钮用于开始一个即时操作。
//!
//! ## 何时使用
//!
//! 标记了一个（或封装一组）操作命令，响应用户点击行为，触发相应的业务逻辑。
//!
//! 在 Ant Design 中我们提供了五种按钮。
//!
//! - 主按钮：用于主行动点，一个操作区域只能有一个主按钮。
//! - 默认按钮：用于没有主次之分的一组行动点。
//! - 虚线按钮：常用于添加操作。
//! - 文本按钮：用于最次级的行动点。
//! - 链接按钮：一般用于链接，即导航至某位置。
//!
//! 以及四种状态属性与上面配合使用。
//!
//! - 危险：删除/移动/修改权限等危险操作，一般需要二次确认。
//! - 幽灵：用于背景色比较复杂的地方，常用在首页/产品页等展示场景。
//! - 禁用：行动点不可用的时候，一般需要文案解释。
//! - 加载中：用于异步操作等待反馈的时候，也可以避免多次提交。

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

// 引入按钮样式
const BUTTON_STYLE: &str = include_str!("style.css");

// 导出按钮组模块
mod button_group;
pub use button_group::*;

/// 按钮类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ButtonType {
    /// 主按钮
    Primary,
    /// 默认按钮
    Default,
    /// 虚线按钮
    Dashed,
    /// 文本按钮
    Text,
    /// 链接按钮
    Link,
}

impl Default for ButtonType {
    fn default() -> Self {
        Self::Default
    }
}

/// 按钮尺寸
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ButtonSize {
    /// 大尺寸
    Large,
    /// 中等尺寸（默认）
    Middle,
    /// 小尺寸
    Small,
}

impl Default for ButtonSize {
    fn default() -> Self {
        Self::Middle
    }
}

/// 按钮形状
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ButtonShape {
    /// 默认形状
    Default,
    /// 圆形按钮
    Circle,
    /// 圆角按钮
    Round,
}

impl Default for ButtonShape {
    fn default() -> Self {
        Self::Default
    }
}

/// 按钮 HTML 类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ButtonHtmlType {
    /// 提交按钮
    Submit,
    /// 重置按钮
    Reset,
    /// 普通按钮
    Button,
}

impl Default for ButtonHtmlType {
    fn default() -> Self {
        Self::Button
    }
}

/// 按钮属性
#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    /// 按钮类型
    #[props(default)]
    pub button_type: ButtonType,

    /// 按钮尺寸
    #[props(default)]
    pub size: ButtonSize,

    /// 按钮形状
    #[props(default)]
    pub shape: ButtonShape,

    /// HTML 按钮类型
    #[props(default)]
    pub html_type: ButtonHtmlType,

    /// 是否为危险按钮
    #[props(default = false)]
    pub danger: bool,

    /// 是否为幽灵按钮
    #[props(default = false)]
    pub ghost: bool,

    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,

    /// 是否加载中
    #[props(default = false)]
    pub loading: bool,

    /// 是否为块级按钮
    #[props(default = false)]
    pub block: bool,

    /// 自定义类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,

    /// 点击事件处理器
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,

    /// 子元素
    children: Element,
}

/// 按钮组件
///
/// # 参数
///
/// * `props` - 按钮属性
///
/// # 示例
///
/// ```rust
/// use dioxus::prelude::*;
/// use ant_design_dioxus::Button;
///
/// fn app() -> Element {
///     rsx! {
///         Button {
///             button_type: ButtonType::Primary,
///             onclick: move |_| {
///                 println!("Button clicked!");
///             },
///             "Primary Button"
///         }
///     }
/// }
/// ```
#[component]
pub fn Button(props: ButtonProps) -> Element {
    let class_name = get_button_class_name(&props);
    let button_style = get_button_style(&props);

    rsx! {
        // 注入按钮样式
        style { {BUTTON_STYLE} }

        button {
            class: class_name.clone(),
            style: button_style.clone(),
            r#type: get_html_type(&props.html_type),
            disabled: props.disabled || props.loading,
            onclick: move |evt| {
                if !props.disabled && !props.loading {
                    if let Some(handler) = &props.onclick {
                        handler.call(evt);
                    }
                }
            },

            // 加载图标
            if props.loading {
                span {
                    class: "ant-btn-loading-icon",
                    // TODO: 添加加载图标
                }
            }

            // 按钮内容
            span {
                class: "ant-btn-content",
                {props.children}
            }
        }
    }
}

/// 获取按钮的 CSS 类名
///
/// # 参数
///
/// * `props` - 按钮属性
///
/// # 返回值
///
/// 返回按钮的完整 CSS 类名字符串
fn get_button_class_name(props: &ButtonProps) -> String {
    let mut classes = vec!["ant-btn"];

    // 按钮类型
    match props.button_type {
        ButtonType::Primary => classes.push("ant-btn-primary"),
        ButtonType::Default => classes.push("ant-btn-default"),
        ButtonType::Dashed => classes.push("ant-btn-dashed"),
        ButtonType::Text => classes.push("ant-btn-text"),
        ButtonType::Link => classes.push("ant-btn-link"),
    }

    // 按钮尺寸
    match props.size {
        ButtonSize::Large => classes.push("ant-btn-lg"),
        ButtonSize::Middle => {} // 默认尺寸不需要额外类名
        ButtonSize::Small => classes.push("ant-btn-sm"),
    }

    // 按钮形状
    match props.shape {
        ButtonShape::Circle => classes.push("ant-btn-circle"),
        ButtonShape::Round => classes.push("ant-btn-round"),
        ButtonShape::Default => {}
    }

    // 状态类名
    if props.danger {
        classes.push("ant-btn-dangerous");
    }

    if props.ghost {
        classes.push("ant-btn-background-ghost");
    }

    if props.disabled {
        classes.push("ant-btn-disabled");
    }

    if props.loading {
        classes.push("ant-btn-loading");
    }

    if props.block {
        classes.push("ant-btn-block");
    }

    // 添加自定义类名
    let mut class_string = classes.join(" ");
    if let Some(custom_class) = &props.class {
        class_string.push(' ');
        class_string.push_str(custom_class);
    }

    class_string
}

/// 获取按钮的内联样式
///
/// # 参数
///
/// * `props` - 按钮属性
///
/// # 返回值
///
/// 返回按钮的内联样式字符串
fn get_button_style(props: &ButtonProps) -> String {
    props.style.clone().unwrap_or_default()
}

/// 获取 HTML 按钮类型字符串
///
/// # 参数
///
/// * `html_type` - HTML 按钮类型枚举
///
/// # 返回值
///
/// 返回对应的 HTML type 属性值
fn get_html_type(html_type: &ButtonHtmlType) -> &'static str {
    match html_type {
        ButtonHtmlType::Submit => "submit",
        ButtonHtmlType::Reset => "reset",
        ButtonHtmlType::Button => "button",
    }
}

// 重新导出主要类型（避免名称冲突，不使用通配符导出）

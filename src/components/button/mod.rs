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

use css_in_rust_macros::css;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

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
    // 使用 css! 宏生成样式类名
    let button_class = get_button_css_class(&props);
    let custom_style = props.style.clone().unwrap_or_default();

    rsx! {
        button {
            class: "{button_class} {}",
            props.class.clone().unwrap_or_default(),
            style: custom_style,
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

/// 获取按钮的 CSS-in-Rust 样式类名
///
/// # 参数
///
/// * `props` - 按钮属性
///
/// # 返回值
///
/// 返回使用 css! 宏生成的样式类名
fn get_button_css_class(props: &ButtonProps) -> String {
    // 基础按钮样式
    let base_class = css!(
        r#"
        position: relative;
        display: inline-block;
        font-weight: 400;
        white-space: nowrap;
        text-align: center;
        background-image: none;
        border: 1px solid transparent;
        box-shadow: 0 2px 0 rgba(0, 0, 0, 0.02);
        cursor: pointer;
        transition: all 0.2s cubic-bezier(0.645, 0.045, 0.355, 1);
        user-select: none;
        touch-action: manipulation;
        height: 32px;
        padding: 4px 15px;
        font-size: var(--ant-font-size-base);
        border-radius: var(--ant-border-radius);
        color: var(--ant-text-color);
        background: var(--ant-bg-color);
        border-color: var(--ant-border-color);
        outline: none;
        text-decoration: none;
        line-height: var(--ant-line-height-base);

        &:hover {
            color: var(--ant-primary-color);
            border-color: var(--ant-primary-color);
        }

        &:focus {
            color: var(--ant-primary-color);
            border-color: var(--ant-primary-color);
            outline: 0;
            box-shadow: 0 0 0 2px rgba(5, 145, 255, 0.1);
        }

        &:active {
            color: var(--ant-primary-color-active);
            border-color: var(--ant-primary-color-active);
        }
    "#
    );

    // 按钮类型样式
    let type_class = match props.button_type {
        ButtonType::Primary => css!(
            r#"
            color: #fff;
            background: var(--ant-primary-color);
            border-color: var(--ant-primary-color);

            &:hover {
                background: var(--ant-primary-color-hover);
                border-color: var(--ant-primary-color-hover);
            }

            &:active {
                background: var(--ant-primary-color-active);
                border-color: var(--ant-primary-color-active);
            }
        "#
        ),
        ButtonType::Dashed => css!(
            r#"
            border-style: dashed;
        "#
        ),
        ButtonType::Text => css!(
            r#"
            color: var(--ant-text-color);
            background: transparent;
            border-color: transparent;
            box-shadow: none;

            &:hover {
                color: var(--ant-primary-color);
                background: rgba(0, 0, 0, 0.06);
            }
        "#
        ),
        ButtonType::Link => css!(
            r#"
            color: var(--ant-primary-color);
            background: transparent;
            border-color: transparent;
            box-shadow: none;

            &:hover {
                color: var(--ant-primary-color-hover);
            }
        "#
        ),
        _ => String::new(),
    };

    // 尺寸样式
    let size_class = match props.size {
        ButtonSize::Large => {
            css!("height: 40px; padding: 6px 15px; font-size: var(--ant-font-size-lg);")
        }
        ButtonSize::Small => {
            css!("height: 24px; padding: 0px 7px; font-size: var(--ant-font-size-sm);")
        }
        _ => String::new(),
    };

    // 形状样式
    let shape_class = match props.shape {
        ButtonShape::Circle => css!("border-radius: 50%; width: 32px; padding: 0;"),
        ButtonShape::Round => css!("border-radius: 32px;"),
        _ => String::new(),
    };

    // 状态样式
    let mut state_classes = Vec::new();

    if props.danger {
        state_classes.push(css!(
            r#"
            color: var(--ant-error-color);
            border-color: var(--ant-error-color);

            &:hover {
                color: #ff7875;
                border-color: #ff7875;
            }
        "#
        ));
    }

    if props.ghost {
        state_classes.push(css!(
            r#"
            background: transparent;

            &:hover {
                background: transparent;
            }
        "#
        ));
    }

    if props.disabled || props.loading {
        state_classes.push(css!(
            r#"
            opacity: 0.5;
            cursor: not-allowed;
            pointer-events: none;
        "#
        ));
    }

    if props.block {
        state_classes.push(css!("width: 100%; display: block;"));
    }

    // 组合所有样式类名
    let mut combined_classes = vec![base_class, type_class, size_class, shape_class];
    combined_classes.extend(state_classes);

    combined_classes
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
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

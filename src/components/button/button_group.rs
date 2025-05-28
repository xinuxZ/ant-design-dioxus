//! 按钮组组件
//!
//! 按钮组用于将多个按钮组合在一起，形成一个整体的操作区域。
//!
//! ## 何时使用
//!
//! - 当需要将多个相关的按钮组合在一起时
//! - 当需要统一设置一组按钮的样式时
//! - 当需要实现工具栏或操作栏时

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use super::{ButtonSize, ButtonType};

/// 按钮组尺寸
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ButtonGroupSize {
    /// 大尺寸
    Large,
    /// 中等尺寸（默认）
    Middle,
    /// 小尺寸
    Small,
}

impl Default for ButtonGroupSize {
    fn default() -> Self {
        Self::Middle
    }
}

impl From<ButtonGroupSize> for ButtonSize {
    fn from(size: ButtonGroupSize) -> Self {
        match size {
            ButtonGroupSize::Large => ButtonSize::Large,
            ButtonGroupSize::Middle => ButtonSize::Middle,
            ButtonGroupSize::Small => ButtonSize::Small,
        }
    }
}

/// 按钮组属性
#[derive(Props, Clone, PartialEq)]
pub struct ButtonGroupProps {
    /// 按钮组尺寸
    #[props(default)]
    pub size: ButtonGroupSize,

    /// 按钮类型（统一设置组内所有按钮的类型）
    #[props(default)]
    pub button_type: Option<ButtonType>,

    /// 是否禁用（统一设置组内所有按钮的禁用状态）
    #[props(default = false)]
    pub disabled: bool,

    /// 自定义类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,

    /// 子元素（按钮）
    children: Element,
}

/// 按钮组组件
///
/// # 参数
///
/// * `props` - 按钮组属性
///
/// # 示例
///
/// ```rust
/// use dioxus::prelude::*;
/// use ant_design_dioxus::{Button, ButtonGroup, ButtonType};
///
/// fn app() -> Element {
///     rsx! {
///         ButtonGroup {
///             size: ButtonGroupSize::Large,
///             button_type: ButtonType::Primary,
///             Button { "Left" }
///             Button { "Middle" }
///             Button { "Right" }
///         }
///     }
/// }
/// ```
#[component]
pub fn ButtonGroup(props: ButtonGroupProps) -> Element {
    let class_name = get_button_group_class_name(&props);
    let group_style = get_button_group_style(&props);

    rsx! {
        div {
            class: "{class_name}",
            style: "{group_style}",
            {props.children}
        }
    }
}

/// 获取按钮组类名
///
/// # 参数
///
/// * `props` - 按钮组属性
///
/// # 返回值
///
/// 返回按钮组的CSS类名字符串
fn get_button_group_class_name(props: &ButtonGroupProps) -> String {
    let mut classes = vec!["ant-btn-group".to_string()];

    // 添加尺寸类名
    match props.size {
        ButtonGroupSize::Large => classes.push("ant-btn-group-lg".to_string()),
        ButtonGroupSize::Small => classes.push("ant-btn-group-sm".to_string()),
        ButtonGroupSize::Middle => {}
    }

    // 添加自定义类名
    if let Some(custom_class) = &props.class {
        classes.push(custom_class.clone());
    }

    classes.join(" ")
}

/// 获取按钮组样式
///
/// # 参数
///
/// * `props` - 按钮组属性
///
/// # 返回值
///
/// 返回按钮组的内联样式字符串
fn get_button_group_style(props: &ButtonGroupProps) -> String {
    let mut styles = Vec::new();

    // 添加自定义样式
    if let Some(custom_style) = &props.style {
        styles.push(custom_style.clone());
    }

    styles.join("; ")
}

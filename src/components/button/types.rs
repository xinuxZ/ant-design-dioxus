//! Button 组件类型定义
//!
//! 包含 Button 组件的所有类型定义，包括属性、枚举等

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

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
    pub children: Element,
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
    pub children: Element,
}


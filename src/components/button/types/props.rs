use dioxus::prelude::*;
use std::borrow::Cow;

/// 按钮类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonType {
    Primary,
    Default,
    Dashed,
    Text,
    Link,
}

impl Default for ButtonType {
    fn default() -> Self {
        Self::Default
    }
}

/// 按钮颜色
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ButtonColor {
    Primary,
    Default,
    Danger,
    Custom(String),
}

impl Default for ButtonColor {
    fn default() -> Self {
        Self::Default
    }
}

/// 按钮变体
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Outlined,
    Solid,
    Dashed,
    Text,
    Link,
}

impl Default for ButtonVariant {
    fn default() -> Self {
        Self::Outlined
    }
}

/// 按钮大小
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonSize {
    Large,
    Middle,
    Small,
}

impl Default for ButtonSize {
    fn default() -> Self {
        Self::Middle
    }
}

/// 按钮形状
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonShape {
    Default,
    Circle,
    Round,
}

impl Default for ButtonShape {
    fn default() -> Self {
        Self::Default
    }
}

/// 图标位置
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconPosition {
    Start,
    End,
}

impl Default for IconPosition {
    fn default() -> Self {
        Self::Start
    }
}

/// 加载配置
#[derive(Debug, Clone, PartialEq)]
pub enum LoadingConfig {
    NotLoading,
    Loading,
    DelayedLoading(u32), // 延迟时间（毫秒）
}

impl Default for LoadingConfig {
    fn default() -> Self {
        Self::NotLoading
    }
}

/// 按钮 HTML 类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HtmlType {
    Button,
    Submit,
    Reset,
}

impl Default for HtmlType {
    fn default() -> Self {
        Self::Button
    }
}

/// 按钮属性
#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    /// 按钮类型
    #[props(into, default)]
    pub button_type: Option<ButtonType>,

    /// 按钮颜色
    #[props(into, default)]
    pub color: Option<ButtonColor>,

    /// 按钮变体
    #[props(into, default)]
    pub variant: Option<ButtonVariant>,

    /// 将按钮宽度调整为其父宽度
    #[props(default)]
    pub block: bool,

    /// 设置危险按钮
    #[props(default)]
    pub danger: bool,

    /// 按钮失效状态
    #[props(default)]
    pub disabled: bool,

    /// 幽灵属性，使按钮背景透明
    #[props(default)]
    pub ghost: bool,

    /// 点击跳转的地址
    #[props(into, default)]
    pub href: Option<String>,

    /// 设置 button 原生的 type 值
    #[props(default)]
    pub html_type: HtmlType,

    /// 设置按钮的图标组件
    #[props(into, default)]
    pub icon: Option<Element>,

    /// 设置图标位置
    #[props(default)]
    pub icon_position: IconPosition,

    /// 设置按钮载入状态
    #[props(default)]
    pub loading: LoadingConfig,

    /// 设置按钮形状
    #[props(default)]
    pub shape: ButtonShape,

    /// 设置按钮大小
    #[props(default)]
    pub size: ButtonSize,

    /// 相当于 a 链接的 target 属性
    #[props(into, default)]
    pub target: Option<String>,

    /// CSS 类名
    #[props(into, default)]
    pub class: Option<String>,

    /// 内联样式
    #[props(into, default)]
    pub style: Option<String>,

    /// 点击按钮时的回调
    #[props(into, default)]
    pub on_click: Option<EventHandler<MouseEvent>>,

    /// 自动在两个中文字符之间插入空格
    #[props(default = true)]
    pub auto_insert_space: bool,

    /// 无障碍访问描述
    #[props(into, default)]
    pub aria_label: Option<String>,

    /// 子元素
    #[props(default)]
    pub children: Element,
}

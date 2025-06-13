//!
//! Divider 组件类型定义
//!
//! 包含 Divider 组件相关的所有类型定义。

use dioxus::prelude::*;

/// 分割线类型
#[derive(Debug, Clone, PartialEq)]
pub enum DividerType {
    /// 水平分割线
    Horizontal,
    /// 垂直分割线
    Vertical,
}

impl Default for DividerType {
    fn default() -> Self {
        Self::Horizontal
    }
}

/// 分割线文字位置
#[derive(Debug, Clone, PartialEq)]
pub enum DividerOrientation {
    /// 左侧
    Left,
    /// 居中
    Center,
    /// 右侧
    Right,
}

impl Default for DividerOrientation {
    fn default() -> Self {
        Self::Center
    }
}

/// 分割线尺寸
#[derive(Debug, Clone, PartialEq)]
pub enum DividerSize {
    /// 小号
    Small,
    /// 默认
    Default,
    /// 大号
    Large,
}

impl Default for DividerSize {
    fn default() -> Self {
        Self::Default
    }
}

/// 分割线变体
#[derive(Debug, Clone, PartialEq)]
pub enum DividerVariant {
    /// 实线
    Solid,
    /// 虚线
    Dashed,
    /// 点线
    Dotted,
}

impl Default for DividerVariant {
    fn default() -> Self {
        Self::Solid
    }
}

/// 方向边距
#[derive(Debug, Clone, PartialEq)]
pub enum OrientationMargin {
    /// 像素值
    Pixels(u32),
    /// 百分比
    Percentage(f32),
}

impl Default for OrientationMargin {
    fn default() -> Self {
        Self::Pixels(16)
    }
}

/// Divider 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct DividerProps {
    /// 子元素
    #[props(default)]
    pub children: Option<Element>,
    /// 分割线类型
    #[props(default)]
    pub r#type: DividerType,
    /// 分割线尺寸
    #[props(default)]
    pub size: DividerSize,
    /// 分割线变体（优先级高于dashed属性）
    #[props(default)]
    pub variant: DividerVariant,
    /// 是否为虚线（向后兼容，建议使用variant）
    #[props(default = false)]
    pub dashed: bool,
    /// 是否为简洁模式
    #[props(default = false)]
    pub plain: bool,
    /// 分割线文字的位置
    #[props(default)]
    pub orientation: DividerOrientation,
    /// 文字与分割线边缘的距离
    #[props(default)]
    pub orientation_margin: Option<OrientationMargin>,
    /// 自定义样式类名
    #[props(default)]
    pub class: Option<String>,
    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,
    /// 自定义前缀类名
    #[props(default)]
    pub prefix_cls: Option<String>,
}
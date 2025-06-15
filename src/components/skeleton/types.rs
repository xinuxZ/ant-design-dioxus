use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;

/// Skeleton 主组件属性
#[derive(Props, Clone, PartialEq)]
pub struct SkeletonProps {
    /// 是否显示骨架屏
    pub loading: Option<bool>,
    /// 是否显示动画效果
    pub active: Option<bool>,
    /// 是否显示圆角效果
    pub round: Option<bool>,
    /// 头像占位符配置
    pub avatar: Option<SkeletonAvatarConfig>,
    /// 标题占位符配置
    pub title: Option<SkeletonTitleConfig>,
    /// 段落占位符配置
    pub paragraph: Option<SkeletonParagraphConfig>,
    /// 自定义类名
    pub class_name: Option<String>,
    /// 自定义样式
    pub style: Option<String>,
    /// 主题配置
    pub theme: Option<SkeletonTheme>,
    /// 子元素
    pub children: Option<Element>,
}

/// Skeleton Button 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct SkeletonButtonProps {
    /// 是否显示动画效果
    pub active: Option<bool>,
    /// 是否适应父容器宽度
    pub block: Option<bool>,
    /// 按钮形状
    pub shape: Option<ButtonShape>,
    /// 按钮大小
    pub size: Option<ButtonSize>,
    /// 自定义类名
    pub class_name: Option<String>,
    /// 自定义样式
    pub style: Option<String>,
}

/// Skeleton Input 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct SkeletonInputProps {
    /// 是否显示动画效果
    pub active: Option<bool>,
    /// 输入框大小
    pub size: Option<InputSize>,
    /// 自定义类名
    pub class_name: Option<String>,
    /// 自定义样式
    pub style: Option<String>,
}

/// Skeleton Image 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct SkeletonImageProps {
    /// 是否显示动画效果
    pub active: Option<bool>,
    /// 图片宽度
    pub width: Option<SkeletonWidth>,
    /// 图片高度
    pub height: Option<SkeletonWidth>,
    /// 自定义类名
    pub class_name: Option<String>,
    /// 自定义样式
    pub style: Option<String>,
}

/// Skeleton Avatar 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct SkeletonAvatarProps {
    /// 是否显示动画效果（仅在独立使用时有效）
    pub active: Option<bool>,
    /// 头像形状
    pub shape: Option<AvatarShape>,
    /// 头像大小
    pub size: Option<AvatarSize>,
}

/// Skeleton Title 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct SkeletonTitleProps {
    /// 标题宽度
    pub width: Option<SkeletonWidth>,
}

/// Skeleton Paragraph 组件属性
#[derive(Props, Clone, Debug, PartialEq)]
pub struct SkeletonParagraphProps {
    /// 段落行数
    pub rows: Option<u32>,
    /// 段落宽度配置
    pub width: Option<SkeletonWidthConfig>,
}

/// 头像配置枚举
#[derive(Clone, PartialEq)]
pub enum SkeletonAvatarConfig {
    /// 布尔值配置
    Boolean(bool),
    /// 详细配置
    Config(SkeletonAvatarProps),
}

impl Default for SkeletonAvatarConfig {
    fn default() -> Self {
        SkeletonAvatarConfig::Boolean(false)
    }
}

/// 标题配置枚举
#[derive(Clone, PartialEq)]
pub enum SkeletonTitleConfig {
    /// 布尔值配置
    Boolean(bool),
    /// 详细配置
    Config(SkeletonTitleProps),
}

impl Default for SkeletonTitleConfig {
    fn default() -> Self {
        SkeletonTitleConfig::Boolean(false)
    }
}

/// 段落配置枚举
#[derive(Clone, Debug, PartialEq)]
pub enum SkeletonParagraphConfig {
    /// 布尔值配置
    Boolean(bool),
    /// 详细配置
    Config(SkeletonParagraphProps),
}
impl Default for SkeletonParagraphConfig {
    fn default() -> Self {
        SkeletonParagraphConfig::Boolean(false)
    }
}

/// 宽度配置类型
#[derive(Clone, PartialEq, Debug)]
pub enum SkeletonWidth {
    /// 像素值
    Pixels(u32),
    /// 百分比
    Percentage(u32),
    /// CSS 字符串
    String(String),
}

/// 段落宽度配置
#[derive(Clone, Debug, PartialEq)]
pub enum SkeletonWidthConfig {
    /// 单一宽度（应用于最后一行）
    Single(SkeletonWidth),
    /// 多行宽度（每行独立配置）
    Multiple(Vec<SkeletonWidth>),
}

/// 头像形状
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum AvatarShape {
    /// 圆形
    Circle,
    /// 方形
    Square,
}

/// 头像大小
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum AvatarSize {
    /// 大号
    Large,
    /// 默认
    Default,
    /// 小号
    Small,
    /// 自定义像素值
    Custom(u32),
}

/// impl Display for AvatarSize
impl Display for AvatarSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AvatarSize::Large => write!(f, "large"),
            AvatarSize::Default => write!(f, "default"),
            AvatarSize::Small => write!(f, "small"),
            AvatarSize::Custom(p) => write!(f, "{}px", p),
        }
    }
}

/// 按钮形状
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum ButtonShape {
    /// 圆形
    Circle,
    /// 圆角
    Round,
    /// 方形
    Square,
    /// 默认
    Default,
}

/// 按钮大小
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum ButtonSize {
    /// 大号
    Large,
    /// 默认
    Default,
    /// 小号
    Small,
}
/// Default impl for ButtonSize
impl Default for ButtonSize {
    fn default() -> Self {
        ButtonSize::Default
    }
}

/// impl Display for ButtonSize
impl Display for ButtonSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ButtonSize::Large => write!(f, "large"),
            ButtonSize::Default => write!(f, "default"),
            ButtonSize::Small => write!(f, "small"),
        }
    }
}

/// 输入框大小
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum InputSize {
    /// 大号
    Large,
    /// 默认
    Default,
    /// 小号
    Small,
}

impl Default for InputSize {
    fn default() -> Self {
        InputSize::Default
    }
}

/// impl Display for InputSize
impl Display for InputSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputSize::Large => write!(f, "large"),
            InputSize::Default => write!(f, "default"),
            InputSize::Small => write!(f, "small"),
        }
    }
}

/// Skeleton 主题配置
#[derive(Clone, PartialEq, Debug)]
pub struct SkeletonTheme {
    /// 骨架屏圆角
    pub block_radius: Option<u32>,
    /// 渐变起始颜色
    pub gradient_from_color: Option<String>,
    /// 渐变结束颜色
    pub gradient_to_color: Option<String>,
    /// 段落行高
    pub paragraph_line_height: Option<u32>,
    /// 段落上边距
    pub paragraph_margin_top: Option<u32>,
    /// 标题高度
    pub title_height: Option<u32>,
}

/// 骨架屏状态
#[derive(Clone, PartialEq, Debug)]
pub struct SkeletonState {
    /// 是否正在加载
    pub loading: bool,
    /// 是否显示动画
    pub active: bool,
    /// 当前主题
    pub theme: SkeletonTheme,
    /// 缓存的样式
    pub cached_styles: HashMap<String, String>,
}

// 实现默认值
impl Default for SkeletonProps {
    fn default() -> Self {
        Self {
            loading: Some(true),
            active: Some(false),
            round: Some(false),
            avatar: None,
            title: Some(SkeletonTitleConfig::Boolean(true)),
            paragraph: Some(SkeletonParagraphConfig::Boolean(true)),
            class_name: None,
            style: None,
            theme: None,
            children: None,
        }
    }
}

impl Default for SkeletonButtonProps {
    fn default() -> Self {
        Self {
            active: Some(false),
            block: Some(false),
            shape: Some(ButtonShape::Default),
            size: Some(ButtonSize::Default),
            class_name: None,
            style: None,
        }
    }
}

impl Default for SkeletonInputProps {
    fn default() -> Self {
        Self {
            active: Some(false),
            size: Some(InputSize::Default),
            class_name: None,
            style: None,
        }
    }
}

impl Default for SkeletonImageProps {
    fn default() -> Self {
        Self {
            active: Some(false),
            width: None,
            height: None,
            class_name: None,
            style: None,
        }
    }
}

impl Default for SkeletonTheme {
    fn default() -> Self {
        Self {
            block_radius: Some(4),
            gradient_from_color: Some("rgba(0,0,0,0.06)".to_string()),
            gradient_to_color: Some("rgba(0,0,0,0.15)".to_string()),
            paragraph_line_height: Some(16),
            paragraph_margin_top: Some(28),
            title_height: Some(16),
        }
    }
}

// 实现类型转换方法
impl SkeletonWidth {
    /// 转换为 CSS 值
    pub fn to_css(&self) -> String {
        match self {
            SkeletonWidth::Pixels(px) => format!("{}px", px),
            SkeletonWidth::Percentage(pct) => format!("{}%", pct),
            SkeletonWidth::String(s) => s.clone(),
        }
    }

    /// 获取数值（如果是像素或百分比）
    pub fn to_number(&self) -> Option<u32> {
        match self {
            SkeletonWidth::Pixels(px) => Some(*px),
            SkeletonWidth::Percentage(pct) => Some(*pct),
            SkeletonWidth::String(_) => None,
        }
    }
}

impl AvatarSize {
    /// 转换为 CSS 像素值
    pub fn to_css(&self) -> String {
        match self {
            AvatarSize::Large => "40px".to_string(),
            AvatarSize::Default => "32px".to_string(),
            AvatarSize::Small => "24px".to_string(),
            AvatarSize::Custom(px) => format!("{}px", px),
        }
    }

    /// 获取像素数值
    pub fn to_pixels(&self) -> u32 {
        match self {
            AvatarSize::Large => 40,
            AvatarSize::Default => 32,
            AvatarSize::Small => 24,
            AvatarSize::Custom(px) => *px,
        }
    }
}

impl ButtonSize {
    /// 转换为 CSS 高度值
    pub fn to_height_css(&self) -> String {
        match self {
            ButtonSize::Large => "40px".to_string(),
            ButtonSize::Default => "32px".to_string(),
            ButtonSize::Small => "24px".to_string(),
        }
    }

    /// 转换为 CSS 宽度值
    pub fn to_width_css(&self) -> String {
        match self {
            ButtonSize::Large => "80px".to_string(),
            ButtonSize::Default => "64px".to_string(),
            ButtonSize::Small => "48px".to_string(),
        }
    }
}

impl InputSize {
    /// 转换为 CSS 高度值
    pub fn to_height_css(&self) -> String {
        match self {
            InputSize::Large => "40px".to_string(),
            InputSize::Default => "32px".to_string(),
            InputSize::Small => "24px".to_string(),
        }
    }
}

// 实现显示特性
impl std::fmt::Display for AvatarShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AvatarShape::Circle => write!(f, "circle"),
            AvatarShape::Square => write!(f, "square"),
        }
    }
}

impl Default for AvatarShape {
    fn default() -> Self {
        AvatarShape::Circle
    }
}

impl From<&str> for AvatarShape {
    fn from(value: &str) -> Self {
        match value {
            "circle" => AvatarShape::Circle,
            "square" => AvatarShape::Square,
            _ => AvatarShape::Circle,
        }
    }
}

impl std::fmt::Display for ButtonShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ButtonShape::Circle => write!(f, "circle"),
            ButtonShape::Round => write!(f, "round"),
            ButtonShape::Square => write!(f, "square"),
            ButtonShape::Default => write!(f, "default"),
        }
    }
}

/// impl from str for ButtonShape
impl From<&str> for ButtonShape {
    fn from(value: &str) -> Self {
        match value {
            "circle" => ButtonShape::Circle,
            "round" => ButtonShape::Round,
            "square" => ButtonShape::Square,
            _ => ButtonShape::Default,
        }
    }
}

// 便捷构造函数
impl SkeletonProps {
    /// 创建基础骨架屏
    pub fn basic() -> Self {
        Self::default()
    }

    /// 创建带头像的骨架屏
    pub fn with_avatar() -> Self {
        Self {
            avatar: Some(SkeletonAvatarConfig::Boolean(true)),
            ..Self::default()
        }
    }

    /// 创建活跃动画的骨架屏
    pub fn active() -> Self {
        Self {
            active: Some(true),
            ..Self::default()
        }
    }

    /// 创建圆角骨架屏
    pub fn round() -> Self {
        Self {
            round: Some(true),
            ..Self::default()
        }
    }

    /// 设置加载状态
    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = Some(loading);
        self
    }

    /// 设置动画状态
    pub fn set_active(mut self, active: bool) -> Self {
        self.active = Some(active);
        self
    }

    /// 设置圆角
    pub fn set_round(mut self, round: bool) -> Self {
        self.round = Some(round);
        self
    }

    /// 设置头像配置
    pub fn set_avatar(mut self, avatar: SkeletonAvatarConfig) -> Self {
        self.avatar = Some(avatar);
        self
    }

    /// 设置标题配置
    pub fn set_title(mut self, title: SkeletonTitleConfig) -> Self {
        self.title = Some(title);
        self
    }

    /// 设置段落配置
    pub fn set_paragraph(mut self, paragraph: SkeletonParagraphConfig) -> Self {
        self.paragraph = Some(paragraph);
        self
    }

    /// 设置主题
    pub fn set_theme(mut self, theme: SkeletonTheme) -> Self {
        self.theme = Some(theme);
        self
    }

    /// 设置类名
    pub fn set_class_name(mut self, class_name: String) -> Self {
        self.class_name = Some(class_name);
        self
    }

    /// 设置样式
    pub fn set_style(mut self, style: String) -> Self {
        self.style = Some(style);
        self
    }
}

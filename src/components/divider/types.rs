//! Divider 组件的类型定义

use dioxus::prelude::*;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::sync::RwLock;

/// Divider 组件的 Props
#[derive(Props, Clone, PartialEq)]
pub struct DividerProps {
    /// 分割线类型（水平/垂直）
    #[props(default = DividerType::Horizontal)]
    pub r#type: DividerType,
    
    /// 文本位置
    #[props(default = DividerOrientation::Center)]
    pub orientation: DividerOrientation,
    
    /// 文本边距
    #[props(default)]
    pub orientation_margin: Option<String>,
    
    /// 样式变体
    #[props(default = DividerVariant::Solid)]
    pub variant: DividerVariant,
    
    /// 是否为虚线（向后兼容）
    #[props(default = false)]
    pub dashed: bool,
    
    /// 是否为朴素文本样式
    #[props(default = false)]
    pub plain: bool,
    
    /// 尺寸
    #[props(default)]
    pub size: Option<DividerSize>,
    
    /// 自定义类名
    #[props(default)]
    pub class: Option<String>,
    
    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,
    
    /// 子元素（文本内容）
    #[props(default)]
    pub children: Option<Element>,
}

/// 分割线类型
#[derive(Clone, PartialEq, Debug, Default)]
pub enum DividerType {
    /// 水平分割线
    #[default]
    Horizontal,
    /// 垂直分割线
    Vertical,
}

impl Display for DividerType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            DividerType::Horizontal => write!(f, "horizontal"),
            DividerType::Vertical => write!(f, "vertical"),
        }
    }
}

impl From<&str> for DividerType {
    fn from(s: &str) -> Self {
        match s {
            "vertical" => DividerType::Vertical,
            _ => DividerType::Horizontal,
        }
    }
}

/// 文本位置
#[derive(Clone, PartialEq, Debug, Default)]
pub enum DividerOrientation {
    /// 左对齐
    Left,
    /// 右对齐
    Right,
    /// 居中对齐
    #[default]
    Center,
    /// 开始位置（支持RTL）
    Start,
    /// 结束位置（支持RTL）
    End,
}

impl Display for DividerOrientation {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            DividerOrientation::Left => write!(f, "left"),
            DividerOrientation::Right => write!(f, "right"),
            DividerOrientation::Center => write!(f, "center"),
            DividerOrientation::Start => write!(f, "start"),
            DividerOrientation::End => write!(f, "end"),
        }
    }
}

impl From<&str> for DividerOrientation {
    fn from(s: &str) -> Self {
        match s {
            "left" => DividerOrientation::Left,
            "right" => DividerOrientation::Right,
            "start" => DividerOrientation::Start,
            "end" => DividerOrientation::End,
            _ => DividerOrientation::Center,
        }
    }
}

/// 样式变体
#[derive(Clone, PartialEq, Debug, Default)]
pub enum DividerVariant {
    /// 实线
    #[default]
    Solid,
    /// 虚线
    Dashed,
    /// 点线
    Dotted,
}

impl Display for DividerVariant {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            DividerVariant::Solid => write!(f, "solid"),
            DividerVariant::Dashed => write!(f, "dashed"),
            DividerVariant::Dotted => write!(f, "dotted"),
        }
    }
}

impl From<&str> for DividerVariant {
    fn from(s: &str) -> Self {
        match s {
            "dashed" => DividerVariant::Dashed,
            "dotted" => DividerVariant::Dotted,
            _ => DividerVariant::Solid,
        }
    }
}

/// 分割线尺寸
#[derive(Clone, PartialEq, Debug, Default)]
pub enum DividerSize {
    /// 小尺寸
    Small,
    /// 中等尺寸
    #[default]
    Middle,
    /// 大尺寸
    Large,
}

impl Display for DividerSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            DividerSize::Small => write!(f, "small"),
            DividerSize::Middle => write!(f, "middle"),
            DividerSize::Large => write!(f, "large"),
        }
    }
}

impl From<&str> for DividerSize {
    fn from(s: &str) -> Self {
        match s {
            "small" => DividerSize::Small,
            "large" => DividerSize::Large,
            _ => DividerSize::Middle,
        }
    }
}

/// Divider 主题配置
#[derive(Clone, PartialEq, Debug)]
pub struct DividerTheme {
    /// 基础颜色
    pub color_split: String,
    /// 文本颜色
    pub color_text: String,
    /// 文本颜色（朴素）
    pub color_text_secondary: String,
    /// 字体大小
    pub font_size: String,
    /// 字体大小（小）
    pub font_size_sm: String,
    /// 字体大小（大）
    pub font_size_lg: String,
    /// 行高
    pub line_height: String,
    /// 边距
    pub margin: String,
    /// 边距（小）
    pub margin_sm: String,
    /// 边距（大）
    pub margin_lg: String,
    /// 文本内边距
    pub text_padding: String,
    /// 垂直分割线高度
    pub vertical_height: String,
    /// 垂直分割线边距
    pub vertical_margin: String,
    /// 边框宽度
    pub border_width: String,
    /// 过渡动画
    pub transition: String,
}

impl Default for DividerTheme {
    fn default() -> Self {
        Self {
            color_split: "rgba(5, 5, 5, 0.06)".to_string(),
            color_text: "rgba(0, 0, 0, 0.88)".to_string(),
            color_text_secondary: "rgba(0, 0, 0, 0.45)".to_string(),
            font_size: "14px".to_string(),
            font_size_sm: "12px".to_string(),
            font_size_lg: "16px".to_string(),
            line_height: "1.5714285714285714".to_string(),
            margin: "16px 0".to_string(),
            margin_sm: "8px 0".to_string(),
            margin_lg: "24px 0".to_string(),
            text_padding: "0 16px".to_string(),
            vertical_height: "0.9em".to_string(),
            vertical_margin: "0 8px".to_string(),
            border_width: "1px".to_string(),
            transition: "all 0.2s".to_string(),
        }
    }
}

/// Divider 配置
#[derive(Clone, PartialEq, Debug)]
pub struct DividerConfig {
    /// 分割线类型
    pub r#type: DividerType,
    /// 文本位置
    pub orientation: DividerOrientation,
    /// 文本边距
    pub orientation_margin: Option<String>,
    /// 样式变体
    pub variant: DividerVariant,
    /// 是否为朴素文本
    pub plain: bool,
    /// 尺寸
    pub size: DividerSize,
    /// 是否有文本内容
    pub has_text: bool,
    /// 自定义类名
    pub class: Option<String>,
    /// 自定义样式
    pub style: Option<String>,
}

impl Default for DividerConfig {
    fn default() -> Self {
        Self {
            r#type: DividerType::Horizontal,
            orientation: DividerOrientation::Center,
            orientation_margin: None,
            variant: DividerVariant::Solid,
            plain: false,
            size: DividerSize::Middle,
            has_text: false,
            class: None,
            style: None,
        }
    }
}

/// 暗色主题配置
#[derive(Clone, PartialEq, Debug)]
pub struct DividerDarkTheme {
    pub color_split: String,
    pub color_text: String,
    pub color_text_secondary: String,
}

impl Default for DividerDarkTheme {
    fn default() -> Self {
        Self {
            color_split: "rgba(253, 253, 253, 0.12)".to_string(),
            color_text: "rgba(255, 255, 255, 0.85)".to_string(),
            color_text_secondary: "rgba(255, 255, 255, 0.45)".to_string(),
        }
    }
}

/// 紧凑主题配置
#[derive(Clone, PartialEq, Debug)]
pub struct DividerCompactTheme {
    pub margin: String,
    pub margin_sm: String,
    pub margin_lg: String,
    pub text_padding: String,
    pub vertical_margin: String,
}

impl Default for DividerCompactTheme {
    fn default() -> Self {
        Self {
            margin: "12px 0".to_string(),
            margin_sm: "6px 0".to_string(),
            margin_lg: "18px 0".to_string(),
            text_padding: "0 12px".to_string(),
            vertical_margin: "0 6px".to_string(),
        }
    }
}

// 全局默认主题
static DEFAULT_THEME: RwLock<DividerTheme> = RwLock::new(DividerTheme {
    color_split: String::new(),
    color_text: String::new(),
    color_text_secondary: String::new(),
    font_size: String::new(),
    font_size_sm: String::new(),
    font_size_lg: String::new(),
    line_height: String::new(),
    margin: String::new(),
    margin_sm: String::new(),
    margin_lg: String::new(),
    text_padding: String::new(),
    vertical_height: String::new(),
    vertical_margin: String::new(),
    border_width: String::new(),
    transition: String::new(),
});

/// 获取默认主题
pub fn get_default_divider_theme() -> DividerTheme {
    let theme = DEFAULT_THEME.read().unwrap();
    if theme.color_split.is_empty() {
        DividerTheme::default()
    } else {
        theme.clone()
    }
}

/// 设置默认主题
pub fn set_default_divider_theme(theme: DividerTheme) {
    let mut default_theme = DEFAULT_THEME.write().unwrap();
    *default_theme = theme;
}

// 便捷构造函数
impl DividerProps {
    /// 创建新的 DividerProps
    pub fn new() -> Self {
        Self::default()
    }
    
    /// 设置类型
    pub fn r#type(mut self, r#type: DividerType) -> Self {
        self.r#type = r#type;
        self
    }
    
    /// 设置文本位置
    pub fn orientation(mut self, orientation: DividerOrientation) -> Self {
        self.orientation = orientation;
        self
    }
    
    /// 设置文本边距
    pub fn orientation_margin<S: Into<String>>(mut self, margin: S) -> Self {
        self.orientation_margin = Some(margin.into());
        self
    }
    
    /// 设置样式变体
    pub fn variant(mut self, variant: DividerVariant) -> Self {
        self.variant = variant;
        self
    }
    
    /// 设置为虚线
    pub fn dashed(mut self, dashed: bool) -> Self {
        self.dashed = dashed;
        if dashed {
            self.variant = DividerVariant::Dashed;
        }
        self
    }
    
    /// 设置为朴素文本
    pub fn plain(mut self, plain: bool) -> Self {
        self.plain = plain;
        self
    }
    
    /// 设置尺寸
    pub fn size(mut self, size: DividerSize) -> Self {
        self.size = Some(size);
        self
    }
    
    /// 设置自定义类名
    pub fn class<S: Into<String>>(mut self, class: S) -> Self {
        self.class = Some(class.into());
        self
    }
    
    /// 设置自定义样式
    pub fn style<S: Into<String>>(mut self, style: S) -> Self {
        self.style = Some(style.into());
        self
    }
}

impl Default for DividerProps {
    fn default() -> Self {
        Self {
            r#type: DividerType::Horizontal,
            orientation: DividerOrientation::Center,
            orientation_margin: None,
            variant: DividerVariant::Solid,
            dashed: false,
            plain: false,
            size: None,
            class: None,
            style: None,
            children: None,
        }
    }
}

impl DividerConfig {
    /// 从 Props 创建配置
    pub fn from_props(props: &DividerProps, has_text: bool) -> Self {
        let mut variant = props.variant.clone();
        
        // 向后兼容：如果设置了 dashed，则使用虚线样式
        if props.dashed {
            variant = DividerVariant::Dashed;
        }
        
        Self {
            r#type: props.r#type.clone(),
            orientation: props.orientation.clone(),
            orientation_margin: props.orientation_margin.clone(),
            variant,
            plain: props.plain,
            size: props.size.clone().unwrap_or_default(),
            has_text,
            class: props.class.clone(),
            style: props.style.clone(),
        }
    }
    
    /// 是否为垂直分割线
    pub fn is_vertical(&self) -> bool {
        matches!(self.r#type, DividerType::Vertical)
    }
    
    /// 是否为水平分割线
    pub fn is_horizontal(&self) -> bool {
        matches!(self.r#type, DividerType::Horizontal)
    }
    
    /// 是否有文本内容
    pub fn has_text(&self) -> bool {
        self.has_text
    }
    
    /// 获取边框样式
    pub fn get_border_style(&self) -> &str {
        match self.variant {
            DividerVariant::Solid => "solid",
            DividerVariant::Dashed => "dashed",
            DividerVariant::Dotted => "dotted",
        }
    }
}
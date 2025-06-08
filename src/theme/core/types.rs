//! 主题系统核心类型
//!
//! 定义主题系统使用的核心类型和枚举

use serde::{Deserialize, Serialize};

/// 组件尺寸
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Size {
    /// 小尺寸
    Small,
    /// 中等尺寸
    Middle,
    /// 大尺寸
    Large,
}

impl Default for Size {
    fn default() -> Self {
        Size::Middle
    }
}

impl std::fmt::Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Size::Small => write!(f, "small"),
            Size::Middle => write!(f, "middle"),
            Size::Large => write!(f, "large"),
        }
    }
}

/// 颜色类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ColorType {
    /// 主色
    Primary,
    /// 成功色
    Success,
    /// 警告色
    Warning,
    /// 错误色
    Error,
    /// 信息色
    Info,
}

impl std::fmt::Display for ColorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ColorType::Primary => write!(f, "primary"),
            ColorType::Success => write!(f, "success"),
            ColorType::Warning => write!(f, "warning"),
            ColorType::Error => write!(f, "error"),
            ColorType::Info => write!(f, "info"),
        }
    }
}

/// 种子令牌
///
/// Ant Design 设计系统中的基础设计变量
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SeedToken {
    /// 主色
    pub color_primary: String,
    /// 成功色
    pub color_success: String,
    /// 警告色
    pub color_warning: String,
    /// 错误色
    pub color_error: String,
    /// 信息色
    pub color_info: String,
    /// 基础字体大小
    pub font_size: f32,
    /// 基础边框圆角
    pub border_radius: f32,
    /// 线框模式
    pub wireframe: bool,
}

impl Default for SeedToken {
    fn default() -> Self {
        Self {
            color_primary: "#1890ff".to_string(),
            color_success: "#52c41a".to_string(),
            color_warning: "#faad14".to_string(),
            color_error: "#f5222d".to_string(),
            color_info: "#1890ff".to_string(),
            font_size: 14.0,
            border_radius: 2.0,
            wireframe: false,
        }
    }
}

/// 映射令牌
///
/// 从种子令牌派生的设计变量
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapToken {
    // 颜色
    /// 主色调色板
    pub color_primary_palette: Vec<String>,
    /// 成功色调色板
    pub color_success_palette: Vec<String>,
    /// 警告色调色板
    pub color_warning_palette: Vec<String>,
    /// 错误色调色板
    pub color_error_palette: Vec<String>,
    /// 信息色调色板
    pub color_info_palette: Vec<String>,

    // 中性色
    /// 背景色
    pub color_bg_base: String,
    /// 文本色
    pub color_text_base: String,

    // 派生颜色
    /// 主要文本色
    pub color_text: String,
    /// 次要文本色
    pub color_text_secondary: String,
    /// 禁用文本色
    pub color_text_disabled: String,
    /// 边框色
    pub color_border: String,
    /// 分割线色
    pub color_split: String,

    // 尺寸
    /// 基础单位
    pub size_unit: f32,
    /// 尺寸步长
    pub size_step: f32,
}

impl Default for MapToken {
    fn default() -> Self {
        Self {
            color_primary_palette: vec![
                "#e6f7ff".to_string(),
                "#bae7ff".to_string(),
                "#91d5ff".to_string(),
                "#69c0ff".to_string(),
                "#40a9ff".to_string(),
                "#1890ff".to_string(),
                "#096dd9".to_string(),
                "#0050b3".to_string(),
                "#003a8c".to_string(),
                "#002766".to_string(),
            ],
            color_success_palette: vec![],
            color_warning_palette: vec![],
            color_error_palette: vec![],
            color_info_palette: vec![],
            color_bg_base: "#fff".to_string(),
            color_text_base: "rgba(0, 0, 0, 0.85)".to_string(),
            color_text: "rgba(0, 0, 0, 0.85)".to_string(),
            color_text_secondary: "rgba(0, 0, 0, 0.45)".to_string(),
            color_text_disabled: "rgba(0, 0, 0, 0.25)".to_string(),
            color_border: "#d9d9d9".to_string(),
            color_split: "rgba(0, 0, 0, 0.06)".to_string(),
            size_unit: 4.0,
            size_step: 4.0,
        }
    }
}

/// 别名令牌
///
/// 组件使用的特定设计变量，引用映射令牌
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AliasToken {
    // 组件通用
    /// 组件背景色
    pub component_background: String,
    /// 组件文本色
    pub component_text_color: String,
    /// 组件边框色
    pub component_border_color: String,

    // 按钮
    /// 按钮高度（小）
    pub button_height_sm: f32,
    /// 按钮高度（中）
    pub button_height: f32,
    /// 按钮高度（大）
    pub button_height_lg: f32,

    // 输入框
    /// 输入框高度（小）
    pub input_height_sm: f32,
    /// 输入框高度（中）
    pub input_height: f32,
    /// 输入框高度（大）
    pub input_height_lg: f32,
    // 其他组件特定变量
    // ...
}

impl Default for AliasToken {
    fn default() -> Self {
        Self {
            component_background: "#fff".to_string(),
            component_text_color: "rgba(0, 0, 0, 0.85)".to_string(),
            component_border_color: "#d9d9d9".to_string(),
            button_height_sm: 24.0,
            button_height: 32.0,
            button_height_lg: 40.0,
            input_height_sm: 24.0,
            input_height: 32.0,
            input_height_lg: 40.0,
        }
    }
}

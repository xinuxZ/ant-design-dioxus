use dioxus::prelude::*;
use qrcode::EcLevel;
use serde::{Deserialize, Serialize};
use std::fmt;

/// QRCode 渲染类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QRCodeType {
    /// 使用 Canvas 渲染
    Canvas,
    /// 使用 SVG 渲染
    Svg,
}

impl Default for QRCodeType {
    fn default() -> Self {
        QRCodeType::Canvas
    }
}

impl fmt::Display for QRCodeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            QRCodeType::Canvas => write!(f, "canvas"),
            QRCodeType::Svg => write!(f, "svg"),
        }
    }
}

impl QRCodeType {
    /// 从字符串创建 QRCodeType
    pub fn from_str(s: &str) -> Self {
        match s {
            "svg" => QRCodeType::Svg,
            _ => QRCodeType::Canvas, // 默认为 Canvas
        }
    }

    /// 检查是否为 Canvas 类型
    pub fn is_canvas(&self) -> bool {
        matches!(self, QRCodeType::Canvas)
    }

    /// 检查是否为 SVG 类型
    pub fn is_svg(&self) -> bool {
        matches!(self, QRCodeType::Svg)
    }
}

/// QRCode 状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QRCodeStatus {
    /// 活跃状态（默认）
    Active,
    /// 已过期
    Expired,
    /// 加载中
    Loading,
    /// 已扫描
    Scanned,
}

impl Default for QRCodeStatus {
    fn default() -> Self {
        QRCodeStatus::Active
    }
}

impl fmt::Display for QRCodeStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            QRCodeStatus::Active => write!(f, "active"),
            QRCodeStatus::Expired => write!(f, "expired"),
            QRCodeStatus::Loading => write!(f, "loading"),
            QRCodeStatus::Scanned => write!(f, "scanned"),
        }
    }
}

impl QRCodeStatus {
    /// 从字符串创建 QRCodeStatus
    pub fn from_str(s: &str) -> Self {
        match s {
            "active" => QRCodeStatus::Active,
            "expired" => QRCodeStatus::Expired,
            "loading" => QRCodeStatus::Loading,
            "scanned" => QRCodeStatus::Scanned,
            _ => QRCodeStatus::Active, // 默认为 Active
        }
    }

    /// 检查是否为活跃状态
    pub fn is_active(&self) -> bool {
        matches!(self, QRCodeStatus::Active)
    }

    /// 检查是否为过期状态
    pub fn is_expired(&self) -> bool {
        matches!(self, QRCodeStatus::Expired)
    }

    /// 检查是否为加载状态
    pub fn is_loading(&self) -> bool {
        matches!(self, QRCodeStatus::Loading)
    }

    /// 检查是否为已扫描状态
    pub fn is_scanned(&self) -> bool {
        matches!(self, QRCodeStatus::Scanned)
    }

    /// 检查是否可以刷新
    pub fn can_refresh(&self) -> bool {
        matches!(self, QRCodeStatus::Expired)
    }

    /// 检查是否需要加载状态
    pub fn needs_loading(&self) -> bool {
        matches!(self, QRCodeStatus::Loading)
    }
}

/// QRCode 图标设置
#[derive(Debug, Clone, PartialEq)]
pub struct QRCodeIconSettings {
    /// 图标URL
    pub src: String,
    /// 图标宽度
    pub width: u32,
    /// 图标高度
    pub height: u32,
    /// 图标横向位置（如果不指定则居中）
    pub x: Option<u32>,
    /// 图标纵向位置（如果不指定则居中）
    pub y: Option<u32>,
    /// 是否挖空图标下方的二维码
    pub excavate: bool,
    /// 图标透明度
    pub opacity: f32,
}

impl Default for QRCodeIconSettings {
    fn default() -> Self {
        Self {
            src: String::new(),
            width: 40,
            height: 40,
            x: None,
            y: None,
            excavate: true,
            opacity: 1.0,
        }
    }
}

/// QRCode 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct QRCodeProps {
    /// 二维码内容
    pub value: String,

    /// 渲染类型，可选 'canvas' 或 'svg'
    #[props(default = QRCodeType::Canvas)]
    pub qr_type: QRCodeType,

    /// 渲染类型（别名，兼容性）
    #[props(default = QRCodeType::Canvas)]
    pub r#type: QRCodeType,

    /// 二维码大小
    #[props(default = QRCodeSize::Medium)]
    pub size: QRCodeSize,

    /// 二维码状态
    #[props(default = QRCodeStatus::Active)]
    pub status: QRCodeStatus,

    /// 纠错级别
    #[props(default = QRCodeErrorLevel::M)]
    pub error_level: QRCodeErrorLevel,

    /// 二维码颜色
    #[props(default)]
    pub color: Option<String>,

    /// 二维码背景色
    #[props(default)]
    pub bg_color: Option<String>,

    /// 嵌入的图标地址
    #[props(default)]
    pub icon: Option<String>,

    /// 图标大小
    #[props(default)]
    pub icon_size: Option<u32>,

    /// 是否有边框
    #[props(default)]
    pub bordered: Option<bool>,

    /// 边框颜色
    #[props(default)]
    pub border_color: Option<String>,

    /// 自定义状态渲染
    #[props(default)]
    pub status_render: Option<Element>,

    /// 刷新回调
    #[props(default)]
    pub on_refresh: Option<EventHandler<()>>,

    /// 自定义样式类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,

    /// 子元素
    #[props(default)]
    pub children: Option<Element>,
}

/// QRCode 尺寸
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QRCodeSize {
    /// 小尺寸 (120px)
    Small,
    /// 中等尺寸 (160px)
    Medium,
    /// 大尺寸 (200px)
    Large,
    /// 自定义尺寸
    Custom(u32),
}

impl Default for QRCodeSize {
    fn default() -> Self {
        QRCodeSize::Medium
    }
}

impl fmt::Display for QRCodeSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QRCodeSize::Small => write!(f, "small"),
            QRCodeSize::Medium => write!(f, "medium"),
            QRCodeSize::Large => write!(f, "large"),
            QRCodeSize::Custom(size) => write!(f, "{}", size),
        }
    }
}

impl QRCodeSize {
    /// 转换为像素值
    pub fn to_pixels(&self) -> u32 {
        match self {
            QRCodeSize::Small => 120,
            QRCodeSize::Medium => 160,
            QRCodeSize::Large => 200,
            QRCodeSize::Custom(size) => *size,
        }
    }

    /// 从像素值创建尺寸
    pub fn from_pixels(pixels: u32) -> Self {
        match pixels {
            120 => QRCodeSize::Small,
            160 => QRCodeSize::Medium,
            200 => QRCodeSize::Large,
            size => QRCodeSize::Custom(size),
        }
    }

    /// 转换为CSS值
    pub fn to_css_value(&self) -> String {
        format!("{}px", self.to_pixels())
    }
}

/// QRCode 错误纠正级别
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QRCodeErrorLevel {
    /// 低级别纠错，约7%的码字可被恢复
    L,
    /// 中级别纠错，约15%的码字可被恢复
    M,
    /// 四分之一级别纠错，约25%的码字可被恢复
    Q,
    /// 高级别纠错，约30%的码字可被恢复
    H,
}

impl Default for QRCodeErrorLevel {
    fn default() -> Self {
        QRCodeErrorLevel::M
    }
}

impl From<QRCodeErrorLevel> for EcLevel {
    fn from(level: QRCodeErrorLevel) -> Self {
        match level {
            QRCodeErrorLevel::L => EcLevel::L,
            QRCodeErrorLevel::M => EcLevel::M,
            QRCodeErrorLevel::Q => EcLevel::Q,
            QRCodeErrorLevel::H => EcLevel::H,
        }
    }
}

impl fmt::Display for QRCodeErrorLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QRCodeErrorLevel::L => write!(f, "L"),
            QRCodeErrorLevel::M => write!(f, "M"),
            QRCodeErrorLevel::Q => write!(f, "Q"),
            QRCodeErrorLevel::H => write!(f, "H"),
        }
    }
}

impl QRCodeErrorLevel {
    /// 从字符串创建错误级别
    pub fn from_str(s: &str) -> Self {
        match s {
            "L" => QRCodeErrorLevel::L,
            "M" => QRCodeErrorLevel::M,
            "Q" => QRCodeErrorLevel::Q,
            "H" => QRCodeErrorLevel::H,
            _ => QRCodeErrorLevel::M, // 默认值
        }
    }

    /// 获取错误纠正百分比
    pub fn to_percentage(&self) -> &'static str {
        match self {
            QRCodeErrorLevel::L => "7%",
            QRCodeErrorLevel::M => "15%",
            QRCodeErrorLevel::Q => "25%",
            QRCodeErrorLevel::H => "30%",
        }
    }

    /// 获取数值表示
    pub fn to_numeric_value(&self) -> u8 {
        match self {
            QRCodeErrorLevel::L => 1,
            QRCodeErrorLevel::M => 0,
            QRCodeErrorLevel::Q => 3,
            QRCodeErrorLevel::H => 2,
        }
    }
}

/// 状态渲染信息
#[derive(Debug, Clone)]
pub struct StatusRenderInfo {
    /// 当前状态
    pub status: QRCodeStatus,
    /// 刷新回调
    pub on_refresh: Option<EventHandler<()>>,
}

/// QRCode 组件的便捷构造函数
impl QRCodeProps {
    /// 创建一个默认的 QRCode 组件
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }

    /// 设置渲染类型
    pub fn r#type(mut self, qr_type: QRCodeType) -> Self {
        self.r#type = qr_type;
        self
    }

    /// 设置图标
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// 设置二维码大小
    pub fn size(mut self, size: QRCodeSize) -> Self {
        self.size = size;
        self
    }

    /// 设置图标大小
    pub fn icon_size(mut self, icon_size: Option<u32>) -> Self {
        self.icon_size = icon_size;
        self
    }

    /// 设置二维码颜色
    pub fn color(mut self, color: impl Into<Option<String>>) -> Self {
        self.color = color.into();
        self
    }

    /// 设置二维码背景色
    pub fn bg_color(mut self, bg_color: impl Into<Option<String>>) -> Self {
        self.bg_color = bg_color.into();
        self
    }

    /// 设置是否有边框
    pub fn bordered(mut self, bordered: Option<bool>) -> Self {
        self.bordered = bordered;
        self
    }

    /// 设置纠错级别
    pub fn error_level(mut self, error_level: QRCodeErrorLevel) -> Self {
        self.error_level = error_level;
        self
    }

    /// 设置二维码状态
    pub fn status(mut self, status: QRCodeStatus) -> Self {
        self.status = status;
        self
    }

    /// 设置自定义样式类名
    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.class = Some(class.into());
        self
    }

    /// 设置自定义样式
    pub fn style(mut self, style: impl Into<String>) -> Self {
        self.style = Some(style.into());
        self
    }
}

impl Default for QRCodeProps {
    fn default() -> Self {
        Self {
            value: String::new(),
            qr_type: QRCodeType::default(),
            r#type: QRCodeType::default(),
            icon: None,
            size: QRCodeSize::default(),
            icon_size: Some(40),
            color: Some("#000000".to_string()),
            bg_color: Some("transparent".to_string()),
            bordered: Some(true),
            border_color: Some("#000000".to_string()),
            error_level: QRCodeErrorLevel::default(),
            status: QRCodeStatus::default(),
            status_render: None,
            on_refresh: None,
            class: None,
            style: None,
            children: None,
        }
    }
}

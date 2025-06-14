use dioxus::prelude::*;
use qrcode::EcLevel;
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
    pub r#type: QRCodeType,

    /// 嵌入的图标地址
    #[props(default)]
    pub icon: Option<String>,

    /// 二维码大小
    #[props(default = 160)]
    pub size: u32,

    /// 图标大小
    #[props(default = 40)]
    pub icon_size: u32,

    /// 二维码颜色
    #[props(default = "#000000".to_string())]
    pub color: String,

    /// 二维码背景色
    #[props(default = "transparent".to_string())]
    pub bg_color: String,

    /// 是否有边框
    #[props(default = true)]
    pub bordered: bool,

    /// 纠错级别
    #[props(default = QRCodeErrorLevel::M)]
    pub error_level: QRCodeErrorLevel,

    /// 二维码状态
    #[props(default = QRCodeStatus::Active)]
    pub status: QRCodeStatus,

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

/// 状态渲染信息
#[derive(Debug, Clone)]
pub struct StatusRenderInfo {
    /// 当前状态
    pub status: QRCodeStatus,
    /// 刷新回调
    pub on_refresh: Option<EventHandler<()>>,
}

//! Icon 组件的类型定义

use dioxus::prelude::*;
use std::collections::HashMap;

/// Icon 组件的属性
#[derive(Props, Clone, PartialEq)]
pub struct IconProps {
    /// 图标类型/名称
    #[props(default)]
    pub icon_type: Option<CommonIconType>,

    /// 主题类型
    #[props(default)]
    pub theme: Option<IconTheme>,

    /// 旋转角度（度数）
    #[props(default)]
    pub rotate: Option<i32>,

    /// 是否显示旋转动画
    #[props(default = false)]
    pub spin: bool,

    /// 双色图标的主色调（十六进制颜色值）
    #[props(default)]
    pub two_tone_color: Option<String>,

    /// 自定义SVG组件
    #[props(default)]
    pub component: Option<Element>,

    /// 样式类名
    #[props(default)]
    pub class_name: Option<String>,

    /// 内联样式
    #[props(default)]
    pub style: Option<String>,

    /// 点击事件处理器
    #[props(default)]
    pub on_click: Option<EventHandler<MouseEvent>>,

    /// 图标尺寸
    #[props(default)]
    pub size: Option<IconSize>,

    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,
    // pub children: Element,
}

/// 图标主题类型
#[derive(Clone, PartialEq, Default, Debug)]
pub enum IconTheme {
    /// 线性图标（默认）
    #[default]
    Outlined,
    /// 填充图标
    Filled,
    /// 双色图标
    TwoTone,
}

impl IconTheme {
    /// 获取主题的字符串表示
    pub fn as_str(&self) -> &'static str {
        match self {
            IconTheme::Outlined => "outlined",
            IconTheme::Filled => "filled",
            IconTheme::TwoTone => "twoTone",
        }
    }

    /// 获取主题的CSS类名后缀
    pub fn css_suffix(&self) -> &'static str {
        match self {
            IconTheme::Outlined => "outlined",
            IconTheme::Filled => "filled",
            IconTheme::TwoTone => "two-tone",
        }
    }
}

/// Icon尺寸枚举
#[derive(Debug, Clone, PartialEq)]
pub enum IconSize {
    /// 小尺寸 (12px)
    Small,
    /// 默认尺寸 (14px)
    Default,
    /// 大尺寸 (16px)
    Large,
    /// 自定义尺寸 (像素值)
    Custom(u32),
}

impl IconSize {
    /// 转换为CSS像素值
    pub fn to_css(&self) -> String {
        match self {
            IconSize::Small => "12px".to_string(),
            IconSize::Default => "14px".to_string(),
            IconSize::Large => "16px".to_string(),
            IconSize::Custom(size) => format!("{}px", size),
        }
    }

    /// 转换为数值
    pub fn to_pixels(&self) -> u32 {
        match self {
            IconSize::Small => 12,
            IconSize::Default => 14,
            IconSize::Large => 16,
            IconSize::Custom(size) => *size,
        }
    }
}

/// IconFont 配置
#[derive(Clone, PartialEq, Debug)]
pub struct IconFontConfig {
    /// 脚本URL列表
    pub script_url: String,
    /// 额外的通用属性
    pub extra_common_props: HashMap<String, String>,
}

impl Default for IconFontConfig {
    fn default() -> Self {
        Self {
            script_url: String::new(),
            extra_common_props: HashMap::new(),
        }
    }
}

/// 自定义图标组件的属性
#[derive(Props, Clone, PartialEq)]
pub struct CustomIconProps {
    /// CSS类名
    #[props(default)]
    pub class_name: Option<String>,

    /// 填充颜色
    #[props(default)]
    pub fill: Option<String>,

    /// 高度
    #[props(default)]
    pub height: Option<String>,

    /// 宽度
    #[props(default)]
    pub width: Option<String>,

    /// viewBox属性
    #[props(default)]
    pub view_box: Option<String>,

    /// SVG内容
    pub children: Element,
}

/// 图标状态
#[derive(Clone, PartialEq, Debug)]
pub struct IconState {
    /// 当前是否正在加载
    pub loading: bool,
    /// 当前主题
    pub theme: IconTheme,
    /// 当前旋转角度
    pub rotation: i32,
    /// 是否启用动画
    pub animated: bool,
}

impl Default for IconState {
    fn default() -> Self {
        Self {
            loading: false,
            theme: IconTheme::default(),
            rotation: 0,
            animated: false,
        }
    }
}

/// 预定义的常用图标类型
#[derive(Clone, PartialEq, Debug)]
pub enum CommonIconType {
    // 方向性图标
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    CaretUp,
    CaretDown,
    CaretLeft,
    CaretRight,

    // 建议性图标
    Question,
    Plus,
    Minus,
    Info,
    Exclamation,
    Close,
    Check,

    // 应用图标
    Home,
    Setting,
    User,
    Search,
    Menu,
    More,
    Edit,
    Delete,
    Copy,
    Save,

    // 状态图标
    Loading,
    Success,
    Warning,
    Error,

    // 文件图标
    Download,
    Upload,

    // 自定义图标
    Custom(String),
}

impl CommonIconType {
    /// 获取图标的字符串标识
    pub fn as_str(&self) -> &str {
        match self {
            // 方向性图标
            CommonIconType::ArrowUp => "arrow-up",
            CommonIconType::ArrowDown => "arrow-down",
            CommonIconType::ArrowLeft => "arrow-left",
            CommonIconType::ArrowRight => "arrow-right",
            CommonIconType::CaretUp => "caret-up",
            CommonIconType::CaretDown => "caret-down",
            CommonIconType::CaretLeft => "caret-left",
            CommonIconType::CaretRight => "caret-right",

            // 建议性图标
            CommonIconType::Question => "question",
            CommonIconType::Plus => "plus",
            CommonIconType::Minus => "minus",
            CommonIconType::Info => "info",
            CommonIconType::Exclamation => "exclamation",
            CommonIconType::Close => "close",
            CommonIconType::Check => "check",

            // 应用图标
            CommonIconType::Home => "home",
            CommonIconType::Setting => "setting",
            CommonIconType::User => "user",
            CommonIconType::Search => "search",
            CommonIconType::Menu => "menu",
            CommonIconType::More => "more",
            CommonIconType::Edit => "edit",
            CommonIconType::Delete => "delete",
            CommonIconType::Copy => "copy",
            CommonIconType::Save => "save",

            // 状态图标
            CommonIconType::Loading => "loading",
            CommonIconType::Success => "check-circle",
            CommonIconType::Warning => "warning",
            CommonIconType::Error => "close-circle",

            // 文件图标
            CommonIconType::Download => "download",
            CommonIconType::Upload => "upload",

            // 自定义图标
            CommonIconType::Custom(name) => name,
        }
    }
}

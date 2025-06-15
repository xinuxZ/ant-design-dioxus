//! Icon 组件的类型定义

use dioxus::prelude::*;
use std::collections::HashMap;
use std::fmt;

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

/// 图标组件的默认实现
impl Default for IconProps {
    fn default() -> Self {
        Self {
            icon_type: None,
            theme: Some(IconTheme::Outlined),
            rotate: None,
            spin: false,
            two_tone_color: None,
            component: None,
            class_name: None,
            style: None,
            on_click: None,
            size: None,
            disabled: false,
        }
    }
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

impl fmt::Display for IconTheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IconTheme::Outlined => write!(f, "outlined"),
            IconTheme::Filled => write!(f, "filled"),
            IconTheme::TwoTone => write!(f, "twotone"),
        }
    }
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

    /// 获取主题的CSS类名
    pub fn to_class_name(&self) -> String {
        format!("anticon-{}", self.css_suffix())
    }

    /// 从字符串创建主题
    pub fn from_str(s: &str) -> Self {
        match s {
            "outlined" => IconTheme::Outlined,
            "filled" => IconTheme::Filled,
            "twotone" => IconTheme::TwoTone,
            _ => IconTheme::Outlined, // 默认值
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
    /// 中等尺寸 (16px)
    Medium,
    /// 大尺寸 (20px)
    Large,
    /// 自定义尺寸 (像素值)
    Custom(u32),
}

impl Default for IconSize {
    fn default() -> Self {
        IconSize::Medium
    }
}

impl fmt::Display for IconSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IconSize::Small => write!(f, "small"),
            IconSize::Default => write!(f, "default"),
            IconSize::Medium => write!(f, "medium"),
            IconSize::Large => write!(f, "large"),
            IconSize::Custom(size) => write!(f, "{}px", size),
        }
    }
}

impl IconSize {
    /// 转换为CSS像素值
    pub fn to_css(&self) -> String {
        match self {
            IconSize::Small => "14px".to_string(),
            IconSize::Default => "14px".to_string(),
            IconSize::Medium => "16px".to_string(),
            IconSize::Large => "20px".to_string(),
            IconSize::Custom(size) => format!("{}px", size),
        }
    }

    /// 转换为CSS像素值（别名方法）
    pub fn to_css_value(&self) -> String {
        self.to_css()
    }

    /// 转换为数值
    pub fn to_pixels(&self) -> u32 {
        match self {
            IconSize::Small => 14,
            IconSize::Default => 14,
            IconSize::Medium => 16,
            IconSize::Large => 20,
            IconSize::Custom(size) => *size,
        }
    }

    /// 从字符串创建尺寸
    pub fn from_str(s: &str) -> Self {
        match s {
            "small" => IconSize::Small,
            "default" => IconSize::Default,
            "medium" => IconSize::Medium,
            "large" => IconSize::Large,
            _ => IconSize::Medium, // 默认值
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

/// 图标类型枚举（用于测试兼容性）
#[derive(Clone, PartialEq, Debug)]
pub enum IconType {
    // 方向性图标
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    CaretUp,
    CaretDown,
    CaretLeft,
    CaretRight,
    Arrow,

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
    Paste,
    Cut,
    Undo,
    Redo,
    Refresh,

    // 状态图标
    Loading,
    Success,
    Warning,
    Error,

    // 文件图标
    Download,
    Upload,
    File,
    Folder,

    // 媒体图标
    Star,
    Heart,
    Eye,
    Camera,
    Image,
    Video,
    Music,

    // 通信图标
    Mail,
    Phone,
    Link,
    Share,

    // 时间和位置
    Calendar,
    Clock,
    Location,

    // 安全
    Lock,

    // 自定义图标
    Custom(String),
}

impl fmt::Display for IconType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IconType::ArrowUp => write!(f, "arrow-up"),
            IconType::ArrowDown => write!(f, "arrow-down"),
            IconType::ArrowLeft => write!(f, "arrow-left"),
            IconType::ArrowRight => write!(f, "arrow-right"),
            IconType::CaretUp => write!(f, "caret-up"),
            IconType::CaretDown => write!(f, "caret-down"),
            IconType::CaretLeft => write!(f, "caret-left"),
            IconType::CaretRight => write!(f, "caret-right"),
            IconType::Arrow => write!(f, "arrow"),
            IconType::Question => write!(f, "question"),
            IconType::Plus => write!(f, "plus"),
            IconType::Minus => write!(f, "minus"),
            IconType::Info => write!(f, "info"),
            IconType::Exclamation => write!(f, "exclamation"),
            IconType::Close => write!(f, "close"),
            IconType::Check => write!(f, "check"),
            IconType::Home => write!(f, "home"),
            IconType::Setting => write!(f, "setting"),
            IconType::User => write!(f, "user"),
            IconType::Search => write!(f, "search"),
            IconType::Menu => write!(f, "menu"),
            IconType::More => write!(f, "more"),
            IconType::Edit => write!(f, "edit"),
            IconType::Delete => write!(f, "delete"),
            IconType::Copy => write!(f, "copy"),
            IconType::Save => write!(f, "save"),
            IconType::Paste => write!(f, "paste"),
            IconType::Cut => write!(f, "cut"),
            IconType::Undo => write!(f, "undo"),
            IconType::Redo => write!(f, "redo"),
            IconType::Refresh => write!(f, "refresh"),
            IconType::Loading => write!(f, "loading"),
            IconType::Success => write!(f, "success"),
            IconType::Warning => write!(f, "warning"),
            IconType::Error => write!(f, "error"),
            IconType::Download => write!(f, "download"),
            IconType::Upload => write!(f, "upload"),
            IconType::File => write!(f, "file"),
            IconType::Folder => write!(f, "folder"),
            IconType::Star => write!(f, "star"),
            IconType::Heart => write!(f, "heart"),
            IconType::Eye => write!(f, "eye"),
            IconType::Camera => write!(f, "camera"),
            IconType::Image => write!(f, "image"),
            IconType::Video => write!(f, "video"),
            IconType::Music => write!(f, "music"),
            IconType::Mail => write!(f, "mail"),
            IconType::Phone => write!(f, "phone"),
            IconType::Link => write!(f, "link"),
            IconType::Share => write!(f, "share"),
            IconType::Calendar => write!(f, "calendar"),
            IconType::Clock => write!(f, "clock"),
            IconType::Location => write!(f, "location"),
            IconType::Lock => write!(f, "lock"),
            IconType::Custom(name) => write!(f, "{}", name),
        }
    }
}

impl IconType {
    /// 从字符串创建图标类型
    pub fn from_str(s: &str) -> Self {
        match s {
            "arrow-up" => IconType::ArrowUp,
            "arrow-down" => IconType::ArrowDown,
            "arrow-left" => IconType::ArrowLeft,
            "arrow-right" => IconType::ArrowRight,
            "caret-up" => IconType::CaretUp,
            "caret-down" => IconType::CaretDown,
            "caret-left" => IconType::CaretLeft,
            "caret-right" => IconType::CaretRight,
            "arrow" => IconType::Arrow,
            "question" => IconType::Question,
            "plus" => IconType::Plus,
            "minus" => IconType::Minus,
            "info" => IconType::Info,
            "exclamation" => IconType::Exclamation,
            "close" => IconType::Close,
            "check" => IconType::Check,
            "home" => IconType::Home,
            "setting" => IconType::Setting,
            "user" => IconType::User,
            "search" => IconType::Search,
            "menu" => IconType::Menu,
            "more" => IconType::More,
            "edit" => IconType::Edit,
            "delete" => IconType::Delete,
            "copy" => IconType::Copy,
            "save" => IconType::Save,
            "paste" => IconType::Paste,
            "cut" => IconType::Cut,
            "undo" => IconType::Undo,
            "redo" => IconType::Redo,
            "refresh" => IconType::Refresh,
            "loading" => IconType::Loading,
            "success" => IconType::Success,
            "warning" => IconType::Warning,
            "error" => IconType::Error,
            "download" => IconType::Download,
            "upload" => IconType::Upload,
            "file" => IconType::File,
            "folder" => IconType::Folder,
            "star" => IconType::Star,
            "heart" => IconType::Heart,
            "eye" => IconType::Eye,
            "camera" => IconType::Camera,
            "image" => IconType::Image,
            "video" => IconType::Video,
            "music" => IconType::Music,
            "mail" => IconType::Mail,
            "phone" => IconType::Phone,
            "link" => IconType::Link,
            "share" => IconType::Share,
            "calendar" => IconType::Calendar,
            "clock" => IconType::Clock,
            "location" => IconType::Location,
            "lock" => IconType::Lock,
            _ => IconType::Custom(s.to_string()),
        }
    }

    /// 获取图标的CSS类名
    pub fn to_class_name(&self) -> String {
        format!("anticon-{}", self)
    }

    /// 获取SVG内容
    pub fn get_svg_content(&self) -> Option<String> {
        // 这里应该返回对应的SVG内容
        // 为了测试，返回一个简单的占位符
        Some(format!("<svg>{}</svg>", self))
    }
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

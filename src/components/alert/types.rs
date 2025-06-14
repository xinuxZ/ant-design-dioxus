//! Alert 组件的类型定义

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;

/// Alert 组件的属性
#[derive(Props, Clone, PartialEq)]
pub struct AlertProps {
    /// 警告提示内容
    pub message: String,

    /// 警告类型
    #[props(default = AlertType::Info)]
    pub alert_type: AlertType,

    /// 是否可关闭
    #[props(default = false)]
    pub closable: bool,

    /// 辅助性文字介绍
    #[props(default)]
    pub description: Option<String>,

    /// 是否显示辅助图标
    #[props(default = false)]
    pub show_icon: bool,

    /// 自定义图标
    #[props(default)]
    pub icon: Option<Element>,

    /// 自定义操作项
    #[props(default)]
    pub action: Option<Element>,

    /// 是否用作顶部公告
    #[props(default = false)]
    pub banner: bool,

    /// 样式类名
    #[props(default)]
    pub class_name: Option<String>,

    /// 内联样式
    #[props(default)]
    pub style: Option<String>,

    /// 关闭时的回调
    #[props(default)]
    pub on_close: Option<EventHandler<MouseEvent>>,

    /// 关闭动画结束后的回调
    #[props(default)]
    pub after_close: Option<EventHandler<()>>,

    /// 是否显示
    #[props(default = true)]
    pub visible: bool,

    /// 主题配置
    #[props(default)]
    pub theme: Option<AlertTheme>,

    /// 动画配置
    #[props(default)]
    pub animation: Option<AlertAnimation>,

    /// 尺寸
    #[props(default = AlertSize::Default)]
    pub size: AlertSize,

    /// 是否启用动画
    #[props(default = true)]
    pub enable_animation: bool,

    /// 动画持续时间（毫秒）
    #[props(default = 300)]
    pub animation_duration: u32,

    /// 自定义关闭图标
    #[props(default)]
    pub close_icon: Option<Element>,

    /// 是否显示边框
    #[props(default = true)]
    pub show_border: bool,

    /// 圆角大小
    #[props(default)]
    pub border_radius: Option<String>,

    /// 自定义颜色
    #[props(default)]
    pub color: Option<String>,

    /// 自定义背景色
    #[props(default)]
    pub background_color: Option<String>,

    /// 自定义边框颜色
    #[props(default)]
    pub border_color: Option<String>,

    /// 点击事件
    #[props(default)]
    pub on_click: Option<EventHandler<MouseEvent>>,

    /// 鼠标进入事件
    #[props(default)]
    pub on_mouse_enter: Option<EventHandler<MouseEvent>>,

    /// 鼠标离开事件
    #[props(default)]
    pub on_mouse_leave: Option<EventHandler<MouseEvent>>,

    /// 键盘事件
    #[props(default)]
    pub on_key_down: Option<EventHandler<KeyboardEvent>>,

    /// ARIA 标签
    #[props(default)]
    pub aria_label: Option<String>,

    /// 角色
    #[props(default = "alert".to_string())]
    pub role: String,

    /// 是否自动聚焦
    #[props(default = false)]
    pub auto_focus: bool,

    /// Tab 索引
    #[props(default)]
    pub tab_index: Option<i32>,

    /// 数据测试ID
    #[props(default)]
    pub data_testid: Option<String>,
}

impl Default for AlertProps {
    fn default() -> Self {
        Self {
            message: String::new(),
            alert_type: AlertType::Info,
            closable: false,
            description: None,
            show_icon: false,
            icon: None,
            action: None,
            banner: false,
            class_name: None,
            style: None,
            on_close: None,
            after_close: None,
            visible: true,
            theme: None,
            animation: None,
            size: AlertSize::Default,
            enable_animation: true,
            animation_duration: 300,
            close_icon: None,
            show_border: true,
            border_radius: None,
            color: None,
            background_color: None,
            border_color: None,
            on_click: None,
            on_mouse_enter: None,
            on_mouse_leave: None,
            on_key_down: None,
            aria_label: None,
            role: "alert".to_string(),
            auto_focus: false,
            tab_index: None,
            data_testid: None,
        }
    }
}

/// Alert 类型枚举
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum AlertType {
    /// 成功提示
    Success,
    /// 信息提示
    Info,
    /// 警告提示
    Warning,
    /// 错误提示
    Error,
}
/// impl Display for AlertType
impl Display for AlertType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlertType::Success => write!(f, "success"),
            AlertType::Info => write!(f, "info"),
            AlertType::Warning => write!(f, "warning"),
            AlertType::Error => write!(f, "error"),
        }
    }
}

impl Default for AlertType {
    fn default() -> Self {
        Self::Info
    }
}

impl AlertType {
    /// 获取类型对应的颜色
    pub fn get_color(&self) -> &'static str {
        match self {
            AlertType::Success => "#52c41a",
            AlertType::Info => "#1890ff",
            AlertType::Warning => "#faad14",
            AlertType::Error => "#ff4d4f",
        }
    }

    /// 获取类型对应的背景色
    pub fn get_background_color(&self) -> &'static str {
        match self {
            AlertType::Success => "#f6ffed",
            AlertType::Info => "#e6f7ff",
            AlertType::Warning => "#fffbe6",
            AlertType::Error => "#fff2f0",
        }
    }

    /// 获取类型对应的边框颜色
    pub fn get_border_color(&self) -> &'static str {
        match self {
            AlertType::Success => "#b7eb8f",
            AlertType::Info => "#91d5ff",
            AlertType::Warning => "#ffe58f",
            AlertType::Error => "#ffccc7",
        }
    }

    /// 获取类型对应的图标名称
    pub fn get_icon_name(&self) -> &'static str {
        match self {
            AlertType::Success => "check-circle",
            AlertType::Info => "info-circle",
            AlertType::Warning => "exclamation-circle",
            AlertType::Error => "close-circle",
        }
    }

    /// 获取类型对应的CSS类名
    pub fn get_css_class(&self) -> &'static str {
        match self {
            AlertType::Success => "ant-alert-success",
            AlertType::Info => "ant-alert-info",
            AlertType::Warning => "ant-alert-warning",
            AlertType::Error => "ant-alert-error",
        }
    }
}

/// Alert 尺寸枚举
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum AlertSize {
    /// 小尺寸
    Small,
    /// 默认尺寸
    Default,
    /// 大尺寸
    Large,
}

/// impl Display for AlertSize
impl Display for AlertSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlertSize::Small => write!(f, "small"),
            AlertSize::Default => write!(f, "default"),
            AlertSize::Large => write!(f, "large"),
        }
    }
}

impl Default for AlertSize {
    fn default() -> Self {
        Self::Default
    }
}

impl AlertSize {
    /// 获取尺寸对应的字体大小
    pub fn get_font_size(&self) -> &'static str {
        match self {
            AlertSize::Small => "12px",
            AlertSize::Default => "14px",
            AlertSize::Large => "16px",
        }
    }

    /// 获取尺寸对应的内边距
    pub fn get_padding(&self) -> &'static str {
        match self {
            AlertSize::Small => "4px 8px",
            AlertSize::Default => "8px 15px",
            AlertSize::Large => "12px 20px",
        }
    }

    /// 获取尺寸对应的图标大小
    pub fn get_icon_size(&self) -> &'static str {
        match self {
            AlertSize::Small => "14px",
            AlertSize::Default => "16px",
            AlertSize::Large => "18px",
        }
    }
}

/// Alert 动画类型
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum AlertAnimation {
    /// 无动画
    None,
    /// 淡入淡出
    Fade,
    /// 滑动
    Slide,
    /// 缩放
    Scale,
    /// 自定义动画
    Custom(String),
}

impl Default for AlertAnimation {
    fn default() -> Self {
        Self::Fade
    }
}

/// Alert 主题配置
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct AlertTheme {
    /// 主色调
    pub primary_color: String,
    /// 成功色
    pub success_color: String,
    /// 信息色
    pub info_color: String,
    /// 警告色
    pub warning_color: String,
    /// 错误色
    pub error_color: String,
    /// 边框圆角
    pub border_radius: String,
    /// 字体大小
    pub font_size: String,
    /// 字体族
    pub font_family: String,
    /// 行高
    pub line_height: String,
    /// 阴影
    pub box_shadow: String,
    /// 动画持续时间
    pub animation_duration: String,
    /// 动画缓动函数
    pub animation_timing_function: String,
}

impl Default for AlertTheme {
    fn default() -> Self {
        Self {
            primary_color: "#1890ff".to_string(),
            success_color: "#52c41a".to_string(),
            info_color: "#1890ff".to_string(),
            warning_color: "#faad14".to_string(),
            error_color: "#ff4d4f".to_string(),
            border_radius: "6px".to_string(),
            font_size: "14px".to_string(),
            font_family: "-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Noto Sans', sans-serif".to_string(),
            line_height: "1.5715".to_string(),
            box_shadow: "0 2px 8px rgba(0, 0, 0, 0.15)".to_string(),
            animation_duration: "0.3s".to_string(),
            animation_timing_function: "cubic-bezier(0.645, 0.045, 0.355, 1)".to_string(),
        }
    }
}

/// Alert 配置
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct AlertConfig {
    /// 是否启用动画
    pub enable_animation: bool,
    /// 动画持续时间
    pub animation_duration: u32,
    /// 默认是否显示图标
    pub default_show_icon: bool,
    /// 默认是否可关闭
    pub default_closable: bool,
    /// 默认主题
    pub default_theme: AlertTheme,
    /// 默认尺寸
    pub default_size: AlertSize,
    /// 是否启用键盘导航
    pub enable_keyboard_navigation: bool,
    /// 是否启用无障碍功能
    pub enable_accessibility: bool,
    /// 自定义CSS类前缀
    pub css_prefix: String,
    /// 是否启用RTL支持
    pub enable_rtl: bool,
}

impl Default for AlertConfig {
    fn default() -> Self {
        Self {
            enable_animation: true,
            animation_duration: 300,
            default_show_icon: false,
            default_closable: false,
            default_theme: AlertTheme::default(),
            default_size: AlertSize::Default,
            enable_keyboard_navigation: true,
            enable_accessibility: true,
            css_prefix: "ant-alert".to_string(),
            enable_rtl: false,
        }
    }
}

/// Alert 状态
#[derive(Clone, PartialEq, Debug)]
pub struct AlertState {
    /// 是否可见
    pub visible: bool,
    /// 是否正在关闭
    pub closing: bool,
    /// 是否已挂载
    pub mounted: bool,
    /// 动画状态
    pub animation_state: AnimationState,
}

impl Default for AlertState {
    fn default() -> Self {
        Self {
            visible: true,
            closing: false,
            mounted: false,
            animation_state: AnimationState::Idle,
        }
    }
}

/// 动画状态
#[derive(Clone, PartialEq, Debug)]
pub enum AnimationState {
    /// 空闲
    Idle,
    /// 进入中
    Entering,
    /// 已进入
    Entered,
    /// 退出中
    Exiting,
    /// 已退出
    Exited,
}

/// impl Display for AnimationState
impl Display for AnimationState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnimationState::Idle => write!(f, "idle"),
            AnimationState::Entering => write!(f, "entering"),
            AnimationState::Entered => write!(f, "entered"),
            AnimationState::Exiting => write!(f, "exiting"),
            AnimationState::Exited => write!(f, "exited"),
        }
    }
}

impl Default for AnimationState {
    fn default() -> Self {
        Self::Idle
    }
}

/// Alert 构建器
#[derive(Default)]
pub struct AlertBuilder {
    props: AlertProps,
}

impl AlertBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置消息
    pub fn message<S: Into<String>>(mut self, message: S) -> Self {
        self.props.message = message.into();
        self
    }

    /// 设置类型
    pub fn alert_type(mut self, alert_type: AlertType) -> Self {
        self.props.alert_type = alert_type;
        self
    }

    /// 设置是否可关闭
    pub fn closable(mut self, closable: bool) -> Self {
        self.props.closable = closable;
        self
    }

    /// 设置描述
    pub fn description<S: Into<String>>(mut self, description: S) -> Self {
        self.props.description = Some(description.into());
        self
    }

    /// 设置是否显示图标
    pub fn show_icon(mut self, show_icon: bool) -> Self {
        self.props.show_icon = show_icon;
        self
    }

    /// 设置是否为横幅
    pub fn banner(mut self, banner: bool) -> Self {
        self.props.banner = banner;
        self
    }

    /// 设置尺寸
    pub fn size(mut self, size: AlertSize) -> Self {
        self.props.size = size;
        self
    }

    /// 设置主题
    pub fn theme(mut self, theme: AlertTheme) -> Self {
        self.props.theme = Some(theme);
        self
    }

    /// 设置样式类名
    pub fn class_name<S: Into<String>>(mut self, class_name: S) -> Self {
        self.props.class_name = Some(class_name.into());
        self
    }

    /// 设置内联样式
    pub fn style<S: Into<String>>(mut self, style: S) -> Self {
        self.props.style = Some(style.into());
        self
    }

    /// 构建属性
    pub fn build(self) -> AlertProps {
        self.props
    }
}

// 全局默认配置
static mut GLOBAL_ALERT_CONFIG: Option<AlertConfig> = None;

/// 设置全局默认Alert配置
pub fn set_global_alert_config(config: AlertConfig) {
    unsafe {
        GLOBAL_ALERT_CONFIG = Some(config);
    }
}

/// 获取全局默认Alert配置
pub fn get_global_alert_config() -> AlertConfig {
    unsafe { GLOBAL_ALERT_CONFIG.clone().unwrap_or_default() }
}

/// 创建Alert配置
pub fn create_alert_config() -> AlertConfig {
    AlertConfig::default()
}

/// 创建Alert主题
pub fn create_alert_theme() -> AlertTheme {
    AlertTheme::default()
}

/// 创建Alert属性
pub fn create_alert_props() -> AlertProps {
    AlertProps::default()
}

/// 验证Alert属性
pub fn validate_alert_props(props: &AlertProps) -> Result<(), String> {
    if props.message.is_empty() {
        return Err("Alert message cannot be empty".to_string());
    }

    if props.animation_duration == 0 {
        return Err("Animation duration must be greater than 0".to_string());
    }

    Ok(())
}

/// 合并主题配置
pub fn merge_alert_theme(base: &AlertTheme, override_theme: &AlertTheme) -> AlertTheme {
    AlertTheme {
        primary_color: if override_theme.primary_color.is_empty() {
            base.primary_color.clone()
        } else {
            override_theme.primary_color.clone()
        },
        success_color: if override_theme.success_color.is_empty() {
            base.success_color.clone()
        } else {
            override_theme.success_color.clone()
        },
        info_color: if override_theme.info_color.is_empty() {
            base.info_color.clone()
        } else {
            override_theme.info_color.clone()
        },
        warning_color: if override_theme.warning_color.is_empty() {
            base.warning_color.clone()
        } else {
            override_theme.warning_color.clone()
        },
        error_color: if override_theme.error_color.is_empty() {
            base.error_color.clone()
        } else {
            override_theme.error_color.clone()
        },
        border_radius: if override_theme.border_radius.is_empty() {
            base.border_radius.clone()
        } else {
            override_theme.border_radius.clone()
        },
        font_size: if override_theme.font_size.is_empty() {
            base.font_size.clone()
        } else {
            override_theme.font_size.clone()
        },
        font_family: if override_theme.font_family.is_empty() {
            base.font_family.clone()
        } else {
            override_theme.font_family.clone()
        },
        line_height: if override_theme.line_height.is_empty() {
            base.line_height.clone()
        } else {
            override_theme.line_height.clone()
        },
        box_shadow: if override_theme.box_shadow.is_empty() {
            base.box_shadow.clone()
        } else {
            override_theme.box_shadow.clone()
        },
        animation_duration: if override_theme.animation_duration.is_empty() {
            base.animation_duration.clone()
        } else {
            override_theme.animation_duration.clone()
        },
        animation_timing_function: if override_theme.animation_timing_function.is_empty() {
            base.animation_timing_function.clone()
        } else {
            override_theme.animation_timing_function.clone()
        },
    }
}

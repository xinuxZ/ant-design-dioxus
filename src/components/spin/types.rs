//! Spin 组件的类型定义

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

/// Spin 组件的 Props
#[derive(Props, Clone, PartialEq)]
pub struct SpinProps {
    /// 是否为加载中状态
    #[props(default = true)]
    pub spinning: bool,

    /// 组件大小
    #[props(default)]
    pub size: SpinSize,

    /// 延迟显示加载效果的时间（毫秒）
    /// 当 spinning 状态在延迟时间内结束，则不显示 loading 状态
    #[props(default)]
    pub delay: Option<u32>,

    /// 自定义加载指示器
    pub indicator: Option<Element>,

    /// 当作为包裹元素时，可以自定义描述文案
    pub tip: Option<String>,

    /// 自定义 CSS 类名
    pub class: Option<String>,

    /// 自定义样式
    pub style: Option<String>,

    /// 包装器的 CSS 类名（当有子元素时）
    pub wrapper_class_name: Option<String>,

    /// 子元素
    pub children: Element,
}

impl Default for SpinProps {
    fn default() -> Self {
        SpinProps::new()
    }
}

/// Spin 组件的尺寸
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum SpinSize {
    /// 小尺寸 - 用于文本加载
    Small,
    /// 默认尺寸 - 用于卡片级别的区块加载
    Default,
    /// 大尺寸 - 用于页面级别的加载
    Large,
}

impl Default for SpinSize {
    fn default() -> Self {
        Self::Default
    }
}

impl Display for SpinSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SpinSize::Small => write!(f, "small"),
            SpinSize::Default => write!(f, "default"),
            SpinSize::Large => write!(f, "large"),
        }
    }
}

impl From<&str> for SpinSize {
    fn from(value: &str) -> Self {
        match value {
            "small" => SpinSize::Small,
            "large" => SpinSize::Large,
            _ => SpinSize::Default,
        }
    }
}

/// Spin 组件的主题配置
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct SpinTheme {
    /// 主色调
    pub color_primary: String,
    /// 文本颜色
    pub color_text: String,
    /// 背景颜色
    pub color_bg_container: String,
    /// 遮罩背景颜色
    pub color_bg_mask: String,
    /// 动画持续时间
    pub motion_duration_slow: String,
    /// 字体大小映射
    pub font_size_sm: String,
    pub font_size: String,
    pub font_size_lg: String,
}

impl Default for SpinTheme {
    fn default() -> Self {
        Self {
            color_primary: "#1677ff".to_string(),
            color_text: "rgba(0, 0, 0, 0.88)".to_string(),
            color_bg_container: "#ffffff".to_string(),
            color_bg_mask: "rgba(255, 255, 255, 0.8)".to_string(),
            motion_duration_slow: "0.3s".to_string(),
            font_size_sm: "12px".to_string(),
            font_size: "14px".to_string(),
            font_size_lg: "16px".to_string(),
        }
    }
}

/// Spin 组件的状态
#[derive(Clone, PartialEq, Debug)]
pub struct SpinState {
    /// 当前是否显示加载状态
    pub visible: bool,
    /// 是否有延迟
    pub delayed: bool,
    /// 延迟计时器 ID
    pub delay_timer: Option<i32>,
}

impl Default for SpinState {
    fn default() -> Self {
        Self {
            visible: false,
            delayed: false,
            delay_timer: None,
        }
    }
}

/// Spin 组件的配置
#[derive(Clone, PartialEq, Debug)]
pub struct SpinConfig {
    /// 尺寸
    pub size: SpinSize,
    /// 是否旋转
    pub spinning: bool,
    /// 延迟时间
    pub delay: Option<u32>,
    /// 提示文本
    pub tip: Option<String>,
    /// 是否有子元素
    pub has_children: bool,
}

impl Default for SpinConfig {
    fn default() -> Self {
        Self {
            size: SpinSize::Default,
            spinning: true,
            delay: None,
            tip: None,
            has_children: false,
        }
    }
}

// 便捷构造函数的实现
impl SpinProps {
    /// 创建一个新的 SpinProps
    pub fn new() -> Self {
        Self {
            spinning: true,
            size: SpinSize::Default,
            delay: None,
            indicator: None,
            tip: None,
            class: None,
            style: None,
            wrapper_class_name: None,
            children: rsx! { "" },
        }
    }

    /// 设置加载状态
    pub fn spinning(mut self, spinning: bool) -> Self {
        self.spinning = spinning;
        self
    }

    /// 设置尺寸
    pub fn size(mut self, size: SpinSize) -> Self {
        self.size = size;
        self
    }

    /// 设置延迟时间
    pub fn delay(mut self, delay: u32) -> Self {
        self.delay = Some(delay);
        self
    }

    /// 设置提示文本
    pub fn tip<S: Into<String>>(mut self, tip: S) -> Self {
        self.tip = Some(tip.into());
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

    /// 设置包装器类名
    pub fn wrapper_class_name<S: Into<String>>(mut self, wrapper_class_name: S) -> Self {
        self.wrapper_class_name = Some(wrapper_class_name.into());
        self
    }
}

// SpinSize 的便捷方法
impl SpinSize {
    /// 获取对应的像素大小
    pub fn to_pixel_size(&self) -> u32 {
        match self {
            SpinSize::Small => 14,
            SpinSize::Default => 20,
            SpinSize::Large => 32,
        }
    }

    /// 获取对应的字体大小
    pub fn to_font_size(&self) -> &'static str {
        match self {
            SpinSize::Small => "12px",
            SpinSize::Default => "14px",
            SpinSize::Large => "16px",
        }
    }

    /// 是否为小尺寸
    pub fn is_small(&self) -> bool {
        matches!(self, SpinSize::Small)
    }

    /// 是否为大尺寸
    pub fn is_large(&self) -> bool {
        matches!(self, SpinSize::Large)
    }
}

/// 全局默认指示器配置
static mut DEFAULT_INDICATOR: Option<fn() -> Element> = None;

/// 设置全局默认指示器
pub fn set_default_indicator(indicator: fn() -> Element) {
    unsafe {
        DEFAULT_INDICATOR = Some(indicator);
    }
}

/// 获取全局默认指示器
pub fn get_default_indicator() -> Option<fn() -> Element> {
    unsafe { DEFAULT_INDICATOR }
}

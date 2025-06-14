//! BackTop 组件类型定义
//!
//! 本模块定义了 BackTop 组件的所有类型，包括 Props、主题配置、
//! 动画类型等。

use dioxus::prelude::*;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::collections::HashMap;

/// BackTop 组件的 Props
#[derive(Props, Clone, PartialEq)]
pub struct BackTopProps {
    /// 自定义按钮内容
    pub children: Option<Element>,
    
    /// 滚动高度达到此参数值才出现 BackTop
    #[props(default = 400)]
    pub visibility_height: i32,
    
    /// 设置需要监听其滚动事件的元素
    pub target: Option<Callback<(), Option<web_sys::Element>>>,
    
    /// 点击按钮的回调函数
    pub on_click: Option<Callback<web_sys::MouseEvent>>,
    
    /// 自定义样式
    pub style: Option<String>,
    
    /// 自定义类名
    pub class: Option<String>,
    
    /// 滚动动画持续时间（毫秒）
    #[props(default = 450)]
    pub duration: u32,
    
    /// 距离底部的距离（像素）
    #[props(default = 50)]
    pub bottom: i32,
    
    /// 距离右侧的距离（像素）
    #[props(default = 50)]
    pub right: i32,
    
    /// 是否显示（受控模式）
    pub visible: Option<bool>,
    
    /// 主题配置
    pub theme: Option<BackTopTheme>,
    
    /// 动画类型
    #[props(default = BackTopAnimation::FadeSlide)]
    pub animation: BackTopAnimation,
    
    /// 缓动函数类型
    #[props(default = EasingFunction::EaseOutQuart)]
    pub easing: EasingFunction,
    
    /// 是否启用节流优化
    #[props(default = true)]
    pub throttle: bool,
    
    /// 节流间隔（毫秒）
    #[props(default = 16)]
    pub throttle_delay: u32,
    
    /// 是否启用键盘导航
    #[props(default = true)]
    pub keyboard: bool,
    
    /// ARIA 标签
    pub aria_label: Option<String>,
    
    /// 是否在移动端显示
    #[props(default = true)]
    pub show_on_mobile: bool,
    
    /// 移动端底部距离
    pub mobile_bottom: Option<i32>,
    
    /// 移动端右侧距离
    pub mobile_right: Option<i32>,
}

impl Default for BackTopProps {
    fn default() -> Self {
        Self {
            children: None,
            visibility_height: 400,
            target: None,
            on_click: None,
            style: None,
            class: None,
            duration: 450,
            bottom: 50,
            right: 50,
            visible: None,
            theme: None,
            animation: BackTopAnimation::FadeSlide,
            easing: EasingFunction::EaseOutQuart,
            throttle: true,
            throttle_delay: 16,
            keyboard: true,
            aria_label: None,
            show_on_mobile: true,
            mobile_bottom: None,
            mobile_right: None,
        }
    }
}

/// BackTop 动画类型
#[derive(Clone, Debug, PartialEq)]
pub enum BackTopAnimation {
    /// 淡入淡出
    Fade,
    /// 滑动
    Slide,
    /// 淡入淡出 + 滑动
    FadeSlide,
    /// 缩放
    Scale,
    /// 淡入淡出 + 缩放
    FadeScale,
    /// 弹跳
    Bounce,
    /// 无动画
    None,
}

impl Default for BackTopAnimation {
    fn default() -> Self {
        Self::FadeSlide
    }
}

impl Display for BackTopAnimation {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Fade => write!(f, "fade"),
            Self::Slide => write!(f, "slide"),
            Self::FadeSlide => write!(f, "fade-slide"),
            Self::Scale => write!(f, "scale"),
            Self::FadeScale => write!(f, "fade-scale"),
            Self::Bounce => write!(f, "bounce"),
            Self::None => write!(f, "none"),
        }
    }
}

impl From<&str> for BackTopAnimation {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "fade" => Self::Fade,
            "slide" => Self::Slide,
            "fade-slide" | "fadeslide" => Self::FadeSlide,
            "scale" => Self::Scale,
            "fade-scale" | "fadescale" => Self::FadeScale,
            "bounce" => Self::Bounce,
            "none" => Self::None,
            _ => Self::default(),
        }
    }
}

/// 缓动函数类型
#[derive(Clone, Debug, PartialEq)]
pub enum EasingFunction {
    /// 线性
    Linear,
    /// 缓入
    EaseIn,
    /// 缓出
    EaseOut,
    /// 缓入缓出
    EaseInOut,
    /// 四次方缓入
    EaseInQuart,
    /// 四次方缓出
    EaseOutQuart,
    /// 四次方缓入缓出
    EaseInOutQuart,
    /// 立方缓入
    EaseInCubic,
    /// 立方缓出
    EaseOutCubic,
    /// 立方缓入缓出
    EaseInOutCubic,
    /// 自定义贝塞尔曲线
    CubicBezier(f64, f64, f64, f64),
}

impl Default for EasingFunction {
    fn default() -> Self {
        Self::EaseOutQuart
    }
}

impl Display for EasingFunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Linear => write!(f, "linear"),
            Self::EaseIn => write!(f, "ease-in"),
            Self::EaseOut => write!(f, "ease-out"),
            Self::EaseInOut => write!(f, "ease-in-out"),
            Self::EaseInQuart => write!(f, "cubic-bezier(0.5, 0, 0.75, 0)"),
            Self::EaseOutQuart => write!(f, "cubic-bezier(0.25, 1, 0.5, 1)"),
            Self::EaseInOutQuart => write!(f, "cubic-bezier(0.76, 0, 0.24, 1)"),
            Self::EaseInCubic => write!(f, "cubic-bezier(0.32, 0, 0.67, 0)"),
            Self::EaseOutCubic => write!(f, "cubic-bezier(0.33, 1, 0.68, 1)"),
            Self::EaseInOutCubic => write!(f, "cubic-bezier(0.65, 0, 0.35, 1)"),
            Self::CubicBezier(x1, y1, x2, y2) => {
                write!(f, "cubic-bezier({}, {}, {}, {})", x1, y1, x2, y2)
            }
        }
    }
}

/// BackTop 主题配置
#[derive(Clone, Debug, PartialEq)]
pub struct BackTopTheme {
    /// 背景颜色
    pub background_color: String,
    /// 悬停背景颜色
    pub hover_background_color: String,
    /// 图标颜色
    pub icon_color: String,
    /// 悬停图标颜色
    pub hover_icon_color: String,
    /// 边框圆角
    pub border_radius: String,
    /// 阴影
    pub box_shadow: String,
    /// 悬停阴影
    pub hover_box_shadow: String,
    /// 过渡动画
    pub transition: String,
    /// 层级
    pub z_index: i32,
    /// 按钮尺寸
    pub size: String,
    /// 字体大小
    pub font_size: String,
    /// 边框
    pub border: String,
    /// 悬停边框
    pub hover_border: String,
    /// 激活状态背景色
    pub active_background_color: String,
    /// 禁用状态背景色
    pub disabled_background_color: String,
    /// 禁用状态图标颜色
    pub disabled_icon_color: String,
}

impl Default for BackTopTheme {
    fn default() -> Self {
        Self {
            background_color: "rgba(0, 0, 0, 0.65)".to_string(),
            hover_background_color: "rgba(0, 0, 0, 0.8)".to_string(),
            icon_color: "#ffffff".to_string(),
            hover_icon_color: "#ffffff".to_string(),
            border_radius: "20px".to_string(),
            box_shadow: "0 2px 8px rgba(0, 0, 0, 0.15)".to_string(),
            hover_box_shadow: "0 4px 16px rgba(0, 0, 0, 0.25)".to_string(),
            transition: "all 0.3s cubic-bezier(0.4, 0, 0.2, 1)".to_string(),
            z_index: 1000,
            size: "40px".to_string(),
            font_size: "18px".to_string(),
            border: "none".to_string(),
            hover_border: "none".to_string(),
            active_background_color: "rgba(0, 0, 0, 0.9)".to_string(),
            disabled_background_color: "rgba(0, 0, 0, 0.25)".to_string(),
            disabled_icon_color: "rgba(255, 255, 255, 0.3)".to_string(),
        }
    }
}

/// BackTop 暗色主题
#[derive(Clone, Debug, PartialEq)]
pub struct BackTopDarkTheme;

impl BackTopDarkTheme {
    pub fn theme() -> BackTopTheme {
        BackTopTheme {
            background_color: "rgba(255, 255, 255, 0.85)".to_string(),
            hover_background_color: "rgba(255, 255, 255, 0.95)".to_string(),
            icon_color: "#000000".to_string(),
            hover_icon_color: "#000000".to_string(),
            border_radius: "20px".to_string(),
            box_shadow: "0 2px 8px rgba(255, 255, 255, 0.15)".to_string(),
            hover_box_shadow: "0 4px 16px rgba(255, 255, 255, 0.25)".to_string(),
            transition: "all 0.3s cubic-bezier(0.4, 0, 0.2, 1)".to_string(),
            z_index: 1000,
            size: "40px".to_string(),
            font_size: "18px".to_string(),
            border: "none".to_string(),
            hover_border: "none".to_string(),
            active_background_color: "rgba(255, 255, 255, 1)".to_string(),
            disabled_background_color: "rgba(255, 255, 255, 0.25)".to_string(),
            disabled_icon_color: "rgba(0, 0, 0, 0.3)".to_string(),
        }
    }
}

/// BackTop 紧凑主题
#[derive(Clone, Debug, PartialEq)]
pub struct BackTopCompactTheme;

impl BackTopCompactTheme {
    pub fn theme() -> BackTopTheme {
        BackTopTheme {
            background_color: "rgba(0, 0, 0, 0.65)".to_string(),
            hover_background_color: "rgba(0, 0, 0, 0.8)".to_string(),
            icon_color: "#ffffff".to_string(),
            hover_icon_color: "#ffffff".to_string(),
            border_radius: "16px".to_string(),
            box_shadow: "0 1px 4px rgba(0, 0, 0, 0.15)".to_string(),
            hover_box_shadow: "0 2px 8px rgba(0, 0, 0, 0.25)".to_string(),
            transition: "all 0.2s cubic-bezier(0.4, 0, 0.2, 1)".to_string(),
            z_index: 1000,
            size: "32px".to_string(),
            font_size: "14px".to_string(),
            border: "none".to_string(),
            hover_border: "none".to_string(),
            active_background_color: "rgba(0, 0, 0, 0.9)".to_string(),
            disabled_background_color: "rgba(0, 0, 0, 0.25)".to_string(),
            disabled_icon_color: "rgba(255, 255, 255, 0.3)".to_string(),
        }
    }
}

/// BackTop 配置
#[derive(Clone, Debug, PartialEq)]
pub struct BackTopConfig {
    /// 滚动高度阈值
    pub visibility_height: i32,
    /// 动画持续时间
    pub duration: u32,
    /// 底部距离
    pub bottom: i32,
    /// 右侧距离
    pub right: i32,
    /// 是否有自定义内容
    pub has_custom_content: bool,
    /// 主题配置
    pub theme: BackTopTheme,
    /// 动画类型
    pub animation: BackTopAnimation,
    /// 缓动函数
    pub easing: EasingFunction,
    /// 是否启用节流
    pub throttle: bool,
    /// 节流延迟
    pub throttle_delay: u32,
    /// 是否启用键盘导航
    pub keyboard: bool,
    /// 是否在移动端显示
    pub show_on_mobile: bool,
    /// 移动端配置
    pub mobile_config: Option<BackTopMobileConfig>,
}

impl Default for BackTopConfig {
    fn default() -> Self {
        Self {
            visibility_height: 400,
            duration: 450,
            bottom: 50,
            right: 50,
            has_custom_content: false,
            theme: BackTopTheme::default(),
            animation: BackTopAnimation::default(),
            easing: EasingFunction::default(),
            throttle: true,
            throttle_delay: 16,
            keyboard: true,
            show_on_mobile: true,
            mobile_config: None,
        }
    }
}

/// BackTop 移动端配置
#[derive(Clone, Debug, PartialEq)]
pub struct BackTopMobileConfig {
    /// 移动端底部距离
    pub bottom: i32,
    /// 移动端右侧距离
    pub right: i32,
    /// 移动端主题
    pub theme: Option<BackTopTheme>,
}

impl Default for BackTopMobileConfig {
    fn default() -> Self {
        Self {
            bottom: 20,
            right: 20,
            theme: None,
        }
    }
}

/// BackTop 滚动状态
#[derive(Clone, Debug, PartialEq)]
pub struct BackTopScrollState {
    /// 当前滚动位置
    pub scroll_top: f64,
    /// 是否应该显示
    pub should_show: bool,
    /// 是否正在滚动
    pub is_scrolling: bool,
    /// 滚动方向
    pub scroll_direction: ScrollDirection,
    /// 上次滚动位置
    pub last_scroll_top: f64,
}

impl Default for BackTopScrollState {
    fn default() -> Self {
        Self {
            scroll_top: 0.0,
            should_show: false,
            is_scrolling: false,
            scroll_direction: ScrollDirection::None,
            last_scroll_top: 0.0,
        }
    }
}

/// 滚动方向
#[derive(Clone, Debug, PartialEq)]
pub enum ScrollDirection {
    /// 向上滚动
    Up,
    /// 向下滚动
    Down,
    /// 无滚动
    None,
}

impl Default for ScrollDirection {
    fn default() -> Self {
        Self::None
    }
}

/// 全局默认主题
static mut DEFAULT_THEME: Option<BackTopTheme> = None;

/// 设置全局默认主题
pub fn set_default_back_top_theme(theme: BackTopTheme) {
    unsafe {
        DEFAULT_THEME = Some(theme);
    }
}

/// 获取全局默认主题
pub fn get_default_back_top_theme() -> BackTopTheme {
    unsafe {
        DEFAULT_THEME.clone().unwrap_or_default()
    }
}

/// BackTopProps 便捷构造函数
impl BackTopProps {
    /// 创建新的 BackTopProps
    pub fn new() -> Self {
        Self::default()
    }
    
    /// 设置滚动高度阈值
    pub fn visibility_height(mut self, height: i32) -> Self {
        self.visibility_height = height;
        self
    }
    
    /// 设置动画持续时间
    pub fn duration(mut self, duration: u32) -> Self {
        self.duration = duration;
        self
    }
    
    /// 设置位置
    pub fn position(mut self, bottom: i32, right: i32) -> Self {
        self.bottom = bottom;
        self.right = right;
        self
    }
    
    /// 设置主题
    pub fn theme(mut self, theme: BackTopTheme) -> Self {
        self.theme = Some(theme);
        self
    }
    
    /// 设置动画类型
    pub fn animation(mut self, animation: BackTopAnimation) -> Self {
        self.animation = animation;
        self
    }
    
    /// 设置缓动函数
    pub fn easing(mut self, easing: EasingFunction) -> Self {
        self.easing = easing;
        self
    }
    
    /// 设置点击回调
    pub fn on_click(mut self, callback: Callback<web_sys::MouseEvent>) -> Self {
        self.on_click = Some(callback);
        self
    }
    
    /// 设置自定义样式
    pub fn style(mut self, style: impl Into<String>) -> Self {
        self.style = Some(style.into());
        self
    }
    
    /// 设置自定义类名
    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.class = Some(class.into());
        self
    }
    
    /// 设置 ARIA 标签
    pub fn aria_label(mut self, label: impl Into<String>) -> Self {
        self.aria_label = Some(label.into());
        self
    }
    
    /// 设置移动端配置
    pub fn mobile_config(mut self, bottom: i32, right: i32) -> Self {
        self.mobile_bottom = Some(bottom);
        self.mobile_right = Some(right);
        self
    }
    
    /// 禁用键盘导航
    pub fn disable_keyboard(mut self) -> Self {
        self.keyboard = false;
        self
    }
    
    /// 禁用节流优化
    pub fn disable_throttle(mut self) -> Self {
        self.throttle = false;
        self
    }
    
    /// 隐藏在移动端
    pub fn hide_on_mobile(mut self) -> Self {
        self.show_on_mobile = false;
        self
    }
}

/// BackTopConfig 辅助方法
impl BackTopConfig {
    /// 创建新的配置
    pub fn new() -> Self {
        Self::default()
    }
    
    /// 从 Props 创建配置
    pub fn from_props(props: &BackTopProps) -> Self {
        Self {
            visibility_height: props.visibility_height,
            duration: props.duration,
            bottom: props.bottom,
            right: props.right,
            has_custom_content: props.children.is_some(),
            theme: props.theme.clone().unwrap_or_default(),
            animation: props.animation.clone(),
            easing: props.easing.clone(),
            throttle: props.throttle,
            throttle_delay: props.throttle_delay,
            keyboard: props.keyboard,
            show_on_mobile: props.show_on_mobile,
            mobile_config: if props.mobile_bottom.is_some() || props.mobile_right.is_some() {
                Some(BackTopMobileConfig {
                    bottom: props.mobile_bottom.unwrap_or(20),
                    right: props.mobile_right.unwrap_or(20),
                    theme: None,
                })
            } else {
                None
            },
        }
    }
    
    /// 应用暗色主题
    pub fn with_dark_theme(mut self) -> Self {
        self.theme = BackTopDarkTheme::theme();
        self
    }
    
    /// 应用紧凑主题
    pub fn with_compact_theme(mut self) -> Self {
        self.theme = BackTopCompactTheme::theme();
        self
    }
    
    /// 获取有效的底部距离
    pub fn effective_bottom(&self, is_mobile: bool) -> i32 {
        if is_mobile {
            self.mobile_config.as_ref().map(|c| c.bottom).unwrap_or(self.bottom)
        } else {
            self.bottom
        }
    }
    
    /// 获取有效的右侧距离
    pub fn effective_right(&self, is_mobile: bool) -> i32 {
        if is_mobile {
            self.mobile_config.as_ref().map(|c| c.right).unwrap_or(self.right)
        } else {
            self.right
        }
    }
    
    /// 获取有效的主题
    pub fn effective_theme(&self, is_mobile: bool) -> &BackTopTheme {
        if is_mobile {
            self.mobile_config.as_ref()
                .and_then(|c| c.theme.as_ref())
                .unwrap_or(&self.theme)
        } else {
            &self.theme
        }
    }
}
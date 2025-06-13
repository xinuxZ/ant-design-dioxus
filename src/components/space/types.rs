//! Space 组件的类型定义
//!
//! 定义了 Space 组件使用的所有类型，包括方向、对齐方式、尺寸等。

use dioxus::prelude::*;

/// 间距方向
#[derive(Debug, Clone, PartialEq, Default)]
pub enum SpaceDirection {
    /// 水平方向
    #[default]
    Horizontal,
    /// 垂直方向
    Vertical,
}

impl SpaceDirection {
    /// 转换为CSS类名
    pub fn to_class(&self) -> &'static str {
        match self {
            SpaceDirection::Horizontal => "horizontal",
            SpaceDirection::Vertical => "vertical",
        }
    }
}

/// 对齐方式
#[derive(Debug, Clone, PartialEq, Default)]
pub enum SpaceAlign {
    /// 起始对齐
    #[default]
    Start,
    /// 结束对齐
    End,
    /// 居中对齐
    Center,
    /// 基线对齐
    Baseline,
}

impl SpaceAlign {
    /// 转换为CSS类名
    pub fn to_class(&self) -> &'static str {
        match self {
            SpaceAlign::Start => "start",
            SpaceAlign::End => "end",
            SpaceAlign::Center => "center",
            SpaceAlign::Baseline => "baseline",
        }
    }
}

/// 间距大小
#[derive(Debug, Clone, PartialEq, Default)]
pub enum SpaceSize {
    /// 小间距 (8px)
    Small,
    /// 中等间距 (16px)
    #[default]
    Middle,
    /// 大间距 (24px)
    Large,
    /// 自定义间距
    Custom(u32),
    /// 数组尺寸支持（水平间距，垂直间距）
    Array(Vec<SpaceSize>),
}

/// Space.Compact 组件的尺寸
#[derive(Debug, Clone, PartialEq)]
pub enum SpaceCompactSize {
    /// 大号紧凑间距
    Large,
    /// 中号紧凑间距（默认）
    Middle,
    /// 小号紧凑间距
    Small,
}

impl Default for SpaceCompactSize {
    fn default() -> Self {
        Self::Middle
    }
}

impl SpaceSize {
    /// 转换为CSS类名
    pub fn to_class(&self) -> &'static str {
        match self {
            SpaceSize::Small => "small",
            SpaceSize::Middle => "middle",
            SpaceSize::Large => "large",
            SpaceSize::Custom(_) => "custom",
            SpaceSize::Array(_) => "array",
        }
    }

    /// 获取间距值（像素）
    pub fn to_pixels(&self) -> u32 {
        match self {
            SpaceSize::Small => 8,
            SpaceSize::Middle => 16,
            SpaceSize::Large => 24,
            SpaceSize::Custom(size) => *size,
            SpaceSize::Array(sizes) => {
                // 对于数组尺寸，返回第一个元素的像素值，如果为空则返回默认值
                sizes.first().map(|s| s.to_pixels()).unwrap_or(16)
            }
        }
    }

    /// 获取水平间距值（像素）
    pub fn to_horizontal_pixels(&self) -> u32 {
        match self {
            SpaceSize::Array(sizes) => {
                // 数组的第一个元素是水平间距
                sizes.first().map(|s| s.to_pixels()).unwrap_or(16)
            }
            _ => self.to_pixels(),
        }
    }

    /// 获取垂直间距值（像素）
    pub fn to_vertical_pixels(&self) -> u32 {
        match self {
            SpaceSize::Array(sizes) => {
                // 数组的第二个元素是垂直间距，如果没有则使用第一个
                sizes.get(1).or_else(|| sizes.first()).map(|s| s.to_pixels()).unwrap_or(16)
            }
            _ => self.to_pixels(),
        }
    }
}

/// 分割元素配置
#[derive(Clone, PartialEq)]
pub struct SpaceSplit {
    /// 分割元素内容
    pub element: Element,
    /// 分割元素样式
    pub style: Option<String>,
    /// 分割元素类名
    pub class: Option<String>,
    /// 是否显示分割元素
    pub visible: bool,
}

/// 调试模式配置
#[derive(Debug, Clone, PartialEq)]
pub struct SpaceDebugConfig {
    /// 是否启用调试模式
    pub enabled: bool,
    /// 是否显示间距边界
    pub show_boundaries: bool,
    /// 是否显示尺寸信息
    pub show_size_info: bool,
    /// 调试信息颜色
    pub debug_color: Option<String>,
}

impl Default for SpaceDebugConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            show_boundaries: false,
            show_size_info: false,
            debug_color: None,
        }
    }
}

/// 动画配置
#[derive(Debug, Clone, PartialEq)]
pub struct SpaceAnimationConfig {
    /// 是否启用动画
    pub enabled: bool,
    /// 动画持续时间（毫秒）
    pub duration: u32,
    /// 动画缓动函数
    pub easing: String,
    /// 是否遵循用户的减少动画偏好
    pub respect_reduced_motion: bool,
}

impl Default for SpaceAnimationConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            duration: 300,
            easing: "ease-in-out".to_string(),
            respect_reduced_motion: true,
        }
    }
}

/// 性能优化配置
#[derive(Debug, Clone, PartialEq)]
pub struct SpacePerformanceConfig {
    /// 是否启用虚拟滚动（大量子元素时）
    pub virtual_scroll: bool,
    /// 虚拟滚动阈值
    pub virtual_scroll_threshold: usize,
    /// 是否启用懒加载
    pub lazy_loading: bool,
    /// 是否启用渲染缓存
    pub render_cache: bool,
}

impl Default for SpacePerformanceConfig {
    fn default() -> Self {
        Self {
            virtual_scroll: false,
            virtual_scroll_threshold: 100,
            lazy_loading: false,
            render_cache: true,
        }
    }
}

/// 国际化配置
#[derive(Debug, Clone, PartialEq)]
pub struct SpaceI18nConfig {
    /// 是否启用 RTL 布局
    pub rtl: bool,
    /// 语言代码
    pub locale: Option<String>,
    /// 是否自动检测方向
    pub auto_direction: bool,
}

impl Default for SpaceI18nConfig {
    fn default() -> Self {
        Self {
            rtl: false,
            locale: None,
            auto_direction: false,
        }
    }
}

/// Space 组件的属性
#[derive(Props, Clone, PartialEq)]
pub struct SpaceProps {
    /// 间距方向
    #[props(default = SpaceDirection::Horizontal)]
    pub direction: SpaceDirection,

    /// 间距大小
    #[props(default = SpaceSize::Middle)]
    pub size: SpaceSize,

    /// 对齐方式
    #[props(default = SpaceAlign::Start)]
    pub align: SpaceAlign,

    /// 是否自动换行，仅在 horizontal 时有效
    #[props(default = false)]
    pub wrap: bool,

    /// 分割元素（简单版本，向后兼容）
    #[props(default = None)]
    pub split: Option<Element>,

    /// 增强的分割元素配置
    #[props(default = None)]
    pub split_config: Option<SpaceSplit>,

    /// 自定义类名
    #[props(default = None)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default = None)]
    pub style: Option<String>,

    /// 自定义前缀类名
    #[props(default)]
    pub prefix_cls: Option<String>,

    /// 语义化类名
    #[props(default = None)]
    pub class_names: Option<std::collections::HashMap<String, String>>,

    /// 语义化样式
    #[props(default = None)]
    pub styles: Option<std::collections::HashMap<String, String>>,

    /// 调试模式配置
    #[props(default)]
    pub debug_config: SpaceDebugConfig,

    /// 动画配置
    #[props(default)]
    pub animation_config: SpaceAnimationConfig,

    /// 性能优化配置
    #[props(default)]
    pub performance_config: SpacePerformanceConfig,

    /// 国际化配置
    #[props(default)]
    pub i18n_config: SpaceI18nConfig,

    /// 子元素
    pub children: Element,
}

/// Space.Compact 组件的属性
#[derive(Props, Clone, PartialEq)]
pub struct SpaceCompactProps {
    /// 适应父元素宽度
    #[props(default = false)]
    pub block: bool,

    /// 间距方向
    #[props(default)]
    pub direction: SpaceDirection,

    /// 紧凑间距大小
    #[props(default)]
    pub size: SpaceCompactSize,

    /// 自定义CSS类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,

    /// 自定义前缀类名
    #[props(default)]
    pub prefix_cls: Option<String>,

    /// 子元素
    pub children: Element,
}

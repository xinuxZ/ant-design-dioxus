//! Spin 组件类型定义

use dioxus::prelude::*;

/// Spin 组件尺寸
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SpinSize {
    /// 小尺寸
    Small,
    /// 默认尺寸
    Default,
    /// 大尺寸
    Large,
}

impl Default for SpinSize {
    fn default() -> Self {
        Self::Default
    }
}

impl SpinSize {
    /// 获取尺寸对应的CSS类名
    pub fn to_class(&self) -> &'static str {
        match self {
            SpinSize::Small => "ant-spin-sm",
            SpinSize::Default => "",
            SpinSize::Large => "ant-spin-lg",
        }
    }

    /// 获取指示器尺寸
    pub fn indicator_size(&self) -> u32 {
        match self {
            SpinSize::Small => 14,
            SpinSize::Default => 20,
            SpinSize::Large => 32,
        }
    }
}

/// Spin 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct SpinProps {
    /// 延迟显示加载效果的时间（防止闪烁），单位：毫秒
    #[props(default = 0)]
    pub delay: u32,

    /// 自定义指示符
    #[props(default)]
    pub indicator: Option<Element>,

    /// 组件大小
    #[props(default)]
    pub size: Option<SpinSize>,

    /// 是否为加载中状态
    #[props(default = true)]
    pub spinning: bool,

    /// 当作为包裹元素时，可以自定义描述文案
    #[props(default)]
    pub tip: Option<String>,

    /// 包裹的内容
    #[props(default)]
    pub children: Option<Element>,

    /// 自定义样式类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,

    /// 全屏模式
    #[props(default = false)]
    pub fullscreen: bool,

    /// 进度百分比（0-100）
    #[props(default)]
    pub percent: Option<f32>,

    /// 无障碍标签
    #[props(default)]
    pub aria_label: Option<String>,

    /// 无障碍描述
    #[props(default)]
    pub aria_describedby: Option<String>,
}

/// 全局Spin配置
#[derive(Debug, Clone, PartialEq)]
pub struct SpinConfig {
    /// 全局默认指示器
    pub indicator: Option<Element>,
    /// 全局默认延迟时间
    pub delay: u32,
    /// 全局默认尺寸
    pub size: SpinSize,
}

impl Default for SpinConfig {
    fn default() -> Self {
        Self {
            indicator: None,
            delay: 0,
            size: SpinSize::Default,
        }
    }
}

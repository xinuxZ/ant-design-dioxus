//!
//! Tag 组件类型定义
//!
//! 包含 Tag 组件相关的所有类型定义。

use dioxus::prelude::*;

/// 标签颜色类型
#[derive(Debug, Clone, PartialEq)]
pub enum TagColor {
    /// 默认颜色
    Default,
    /// 主要颜色
    Primary,
    /// 成功颜色
    Success,
    /// 警告颜色
    Warning,
    /// 错误颜色
    Error,
    /// 信息颜色
    Info,
    /// 处理中颜色
    Processing,
    /// 自定义颜色（十六进制）
    Custom(String),
    /// 预设颜色
    Preset(TagPresetColor),
}

impl Default for TagColor {
    fn default() -> Self {
        Self::Default
    }
}

/// 预设颜色枚举
#[derive(Debug, Clone, PartialEq)]
pub enum TagPresetColor {
    /// 品红色
    Magenta,
    /// 红色
    Red,
    /// 火山色
    Volcano,
    /// 橙色
    Orange,
    /// 金色
    Gold,
    /// 青柠色
    Lime,
    /// 绿色
    Green,
    /// 青色
    Cyan,
    /// 蓝色
    Blue,
    /// 极光蓝
    GeekBlue,
    /// 紫色
    Purple,
}

/// 标签尺寸
#[derive(Debug, Clone, PartialEq)]
pub enum TagSize {
    /// 小号
    Small,
    /// 中号（默认）
    Middle,
    /// 大号
    Large,
}

impl Default for TagSize {
    fn default() -> Self {
        Self::Middle
    }
}

/// 标签变体
#[derive(Debug, Clone, PartialEq)]
pub enum TagVariant {
    /// 填充样式
    Filled,
    /// 轮廓样式
    Outlined,
    /// 无边框样式
    Borderless,
}

impl Default for TagVariant {
    fn default() -> Self {
        Self::Filled
    }
}

/// 标签状态
#[derive(Debug, Clone, PartialEq)]
pub enum TagStatus {
    /// 默认状态
    Default,
    /// 成功状态
    Success,
    /// 处理中状态
    Processing,
    /// 错误状态
    Error,
    /// 警告状态
    Warning,
}

impl Default for TagStatus {
    fn default() -> Self {
        Self::Default
    }
}

/// 关闭事件回调类型
pub type OnCloseCallback = EventHandler<()>;

/// 点击事件回调类型
pub type OnClickCallback = EventHandler<MouseEvent>;

/// Tag 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct TagProps {
    /// 子元素（标签内容）
    #[props(default)]
    pub children: Option<Element>,
    
    /// 标签颜色
    #[props(default)]
    pub color: TagColor,
    
    /// 标签尺寸
    #[props(default)]
    pub size: TagSize,
    
    /// 标签变体
    #[props(default)]
    pub variant: TagVariant,
    
    /// 是否可关闭
    #[props(default = false)]
    pub closable: bool,
    
    /// 是否显示边框
    #[props(default = true)]
    pub bordered: bool,
    
    /// 自定义关闭图标
    #[props(default)]
    pub close_icon: Option<Element>,
    
    /// 自定义图标
    #[props(default)]
    pub icon: Option<Element>,
    
    /// 关闭事件回调
    #[props(default)]
    pub on_close: Option<OnCloseCallback>,
    
    /// 点击事件回调
    #[props(default)]
    pub on_click: Option<OnClickCallback>,
    
    /// 自定义样式类名
    #[props(default)]
    pub class: Option<String>,
    
    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,
    
    /// 自定义前缀类名
    #[props(default)]
    pub prefix_cls: Option<String>,
    
    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,
    
    /// 标签状态（用于状态指示）
    #[props(default)]
    pub status: Option<TagStatus>,
}

/// CheckableTag 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct CheckableTagProps {
    /// 子元素（标签内容）
    #[props(default)]
    pub children: Option<Element>,
    
    /// 是否选中
    #[props(default = false)]
    pub checked: bool,
    
    /// 选中状态变化回调
    #[props(default)]
    pub on_change: Option<EventHandler<bool>>,
    
    /// 自定义样式类名
    #[props(default)]
    pub class: Option<String>,
    
    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,
    
    /// 自定义前缀类名
    #[props(default)]
    pub prefix_cls: Option<String>,
    
    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,
}
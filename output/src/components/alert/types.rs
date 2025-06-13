//!
//! Alert 组件类型定义
//!
//! 定义 Alert 组件的所有类型、属性和回调函数。

use dioxus::prelude::*;

/// Alert 类型枚举
#[derive(Debug, Clone, PartialEq)]
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

impl Default for AlertType {
    fn default() -> Self {
        AlertType::Info
    }
}

impl From<&str> for AlertType {
    fn from(value: &str) -> Self {
        match value {
            "success" => AlertType::Success,
            "warning" => AlertType::Warning,
            "error" => AlertType::Error,
            _ => AlertType::Info,
        }
    }
}

/// Alert 尺寸枚举
#[derive(Debug, Clone, PartialEq)]
pub enum AlertSize {
    /// 小尺寸
    Small,
    /// 默认尺寸
    Default,
    /// 大尺寸
    Large,
}

impl Default for AlertSize {
    fn default() -> Self {
        AlertSize::Default
    }
}

/// Alert 变体枚举
#[derive(Debug, Clone, PartialEq)]
pub enum AlertVariant {
    /// 填充样式
    Filled,
    /// 轮廓样式
    Outlined,
}

impl Default for AlertVariant {
    fn default() -> Self {
        AlertVariant::Filled
    }
}

/// 关闭回调函数类型
pub type OnCloseCallback = EventHandler<MouseEvent>;

/// 关闭后回调函数类型
pub type AfterCloseCallback = EventHandler<()>;

/// Alert 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct AlertProps {
    /// 子元素内容
    #[props(default)]
    pub children: Element,
    
    /// Alert 类型
    #[props(default)]
    pub r#type: AlertType,
    
    /// 警告提示内容
    #[props(default)]
    pub message: Option<String>,
    
    /// 警告提示的辅助性文字介绍
    #[props(default)]
    pub description: Option<String>,
    
    /// 是否显示图标
    #[props(default)]
    pub show_icon: Option<bool>,
    
    /// 自定义图标，show_icon 为 true 时有效
    #[props(default)]
    pub icon: Option<Element>,
    
    /// 默认不显示关闭按钮
    #[props(default)]
    pub closable: Option<bool>,
    
    /// 自定义关闭按钮文字
    #[props(default)]
    pub close_text: Option<String>,
    
    /// 自定义关闭图标
    #[props(default)]
    pub close_icon: Option<Element>,
    
    /// 关闭时触发的回调函数
    #[props(default)]
    pub on_close: Option<OnCloseCallback>,
    
    /// 关闭动画结束后触发的回调函数
    #[props(default)]
    pub after_close: Option<AfterCloseCallback>,
    
    /// 是否用作顶部公告
    #[props(default)]
    pub banner: Option<bool>,
    
    /// 自定义操作项
    #[props(default)]
    pub action: Option<Element>,
    
    /// 组件尺寸
    #[props(default)]
    pub size: AlertSize,
    
    /// 组件变体
    #[props(default)]
    pub variant: AlertVariant,
    
    /// 是否显示边框
    #[props(default = true)]
    pub bordered: bool,
    
    /// 是否可见
    #[props(default = true)]
    pub visible: bool,
    
    /// 是否禁用
    #[props(default)]
    pub disabled: Option<bool>,
    
    /// 自定义 CSS 类名
    #[props(default)]
    pub class: Option<String>,
    
    /// 自定义内联样式
    #[props(default)]
    pub style: Option<String>,
    
    /// 自定义前缀类名
    #[props(default)]
    pub prefix_cls: Option<String>,
    
    /// 自定义根元素的 HTML 属性
    #[props(default)]
    pub id: Option<String>,
    
    /// 自定义 role 属性
    #[props(default)]
    pub role: Option<String>,
    
    /// 自定义 aria-label 属性
    #[props(default)]
    pub aria_label: Option<String>,
    
    /// 自定义 aria-describedby 属性
    #[props(default)]
    pub aria_describedby: Option<String>,
    
    /// 自定义 data 属性
    #[props(default)]
    pub data_testid: Option<String>,
}

/// ErrorBoundary Alert 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct ErrorBoundaryAlertProps {
    /// 子元素内容
    pub children: Element,
    
    /// 错误时显示的消息
    #[props(default)]
    pub message: Option<String>,
    
    /// 错误时显示的描述
    #[props(default)]
    pub description: Option<String>,
    
    /// 是否显示错误堆栈
    #[props(default)]
    pub show_stack: Option<bool>,
    
    /// 错误回调函数
    #[props(default)]
    pub on_error: Option<EventHandler<String>>,
    
    /// 重置回调函数
    #[props(default)]
    pub on_reset: Option<EventHandler<()>>,
    
    /// 自定义 CSS 类名
    #[props(default)]
    pub class: Option<String>,
    
    /// 自定义内联样式
    #[props(default)]
    pub style: Option<String>,
}

/// Alert 样式生成器配置
#[derive(Debug, Clone)]
pub struct AlertStyleConfig {
    /// Alert 类型
    pub alert_type: AlertType,
    /// 组件尺寸
    pub size: AlertSize,
    /// 组件变体
    pub variant: AlertVariant,
    /// 是否显示图标
    pub show_icon: bool,
    /// 是否有描述
    pub has_description: bool,
    /// 是否可关闭
    pub closable: bool,
    /// 是否为横幅模式
    pub banner: bool,
    /// 是否显示边框
    pub bordered: bool,
    /// 是否禁用
    pub disabled: bool,
    /// 是否可见
    pub visible: bool,
}

impl Default for AlertStyleConfig {
    fn default() -> Self {
        Self {
            alert_type: AlertType::default(),
            size: AlertSize::default(),
            variant: AlertVariant::default(),
            show_icon: false,
            has_description: false,
            closable: false,
            banner: false,
            bordered: true,
            disabled: false,
            visible: true,
        }
    }
}

/// Alert 动画状态
#[derive(Debug, Clone, PartialEq)]
pub enum AlertAnimationState {
    /// 进入中
    Entering,
    /// 已进入
    Entered,
    /// 退出中
    Exiting,
    /// 已退出
    Exited,
}

impl Default for AlertAnimationState {
    fn default() -> Self {
        AlertAnimationState::Entered
    }
}

/// Alert 内部状态
#[derive(Debug, Clone)]
pub struct AlertState {
    /// 是否可见
    pub visible: bool,
    /// 动画状态
    pub animation_state: AlertAnimationState,
    /// 是否正在关闭
    pub closing: bool,
}

impl Default for AlertState {
    fn default() -> Self {
        Self {
            visible: true,
            animation_state: AlertAnimationState::default(),
            closing: false,
        }
    }
}
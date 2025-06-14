//! Input 组件的类型定义
//!
//! 本模块包含 Input 组件相关的所有类型定义，包括 Props、枚举、事件处理器等。
//! 从组件逻辑中分离出来，提高代码的可维护性和复用性。

use dioxus::prelude::*;
use crate::components::input::styles::{InputSize as StyleInputSize, InputStatus as StyleInputStatus};

/// 输入框尺寸枚举
#[derive(Debug, Clone, PartialEq)]
pub enum InputSize {
    /// 大尺寸输入框
    Large,
    /// 中等尺寸输入框（默认）
    Middle,
    /// 小尺寸输入框
    Small,
}

impl Default for InputSize {
    fn default() -> Self {
        Self::Middle
    }
}

impl From<InputSize> for StyleInputSize {
    fn from(size: InputSize) -> Self {
        match size {
            InputSize::Large => StyleInputSize::Large,
            InputSize::Middle => StyleInputSize::Middle,
            InputSize::Small => StyleInputSize::Small,
        }
    }
}

/// 输入框状态枚举
#[derive(Debug, Clone, PartialEq)]
pub enum InputStatus {
    /// 默认状态
    Default,
    /// 错误状态
    Error,
    /// 警告状态
    Warning,
}

impl Default for InputStatus {
    fn default() -> Self {
        Self::Default
    }
}

impl From<InputStatus> for StyleInputStatus {
    fn from(status: InputStatus) -> Self {
        match status {
            InputStatus::Default => StyleInputStatus::Default,
            InputStatus::Error => StyleInputStatus::Error,
            InputStatus::Warning => StyleInputStatus::Warning,
        }
    }
}

/// Input 组件的 Props
#[derive(Props, Clone, PartialEq)]
pub struct InputProps {
    /// 输入框的值
    #[props(default)]
    pub value: Option<String>,
    
    /// 输入框占位符文本
    #[props(default)]
    pub placeholder: Option<String>,
    
    /// 输入框尺寸
    #[props(default)]
    pub size: InputSize,
    
    /// 输入框状态
    #[props(default)]
    pub status: InputStatus,
    
    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,
    
    /// 是否只读
    #[props(default = false)]
    pub readonly: bool,
    
    /// 是否显示清除按钮
    #[props(default = false)]
    pub allow_clear: bool,
    
    /// 是否有边框
    #[props(default = true)]
    pub bordered: bool,
    
    /// 最大输入长度
    #[props(default)]
    pub max_length: Option<usize>,
    
    /// 是否显示字符计数
    #[props(default = false)]
    pub show_count: bool,
    
    /// 前缀图标或文本
    #[props(default)]
    pub prefix: Option<Element>,
    
    /// 后缀图标或文本
    #[props(default)]
    pub suffix: Option<Element>,
    
    /// 前置标签
    #[props(default)]
    pub addon_before: Option<Element>,
    
    /// 后置标签
    #[props(default)]
    pub addon_after: Option<Element>,
    
    /// 输入框类型
    #[props(default = "text".to_string())]
    pub input_type: String,
    
    /// 自定义 CSS 类名
    #[props(default)]
    pub class: Option<String>,
    
    /// 自定义内联样式
    #[props(default)]
    pub style: Option<String>,
    
    /// 输入值变化时的回调
    #[props(default)]
    pub on_change: Option<EventHandler<String>>,
    
    /// 按下回车键时的回调
    #[props(default)]
    pub on_press_enter: Option<EventHandler<KeyboardEvent>>,
    
    /// 获得焦点时的回调
    #[props(default)]
    pub on_focus: Option<EventHandler<FocusEvent>>,
    
    /// 失去焦点时的回调
    #[props(default)]
    pub on_blur: Option<EventHandler<FocusEvent>>,
    
    /// 点击清除按钮时的回调
    #[props(default)]
    pub on_clear: Option<EventHandler<MouseEvent>>,
    
    /// 子元素（用于带文本的输入框）
    #[props(default)]
    pub children: Option<Element>,
}

/// Input.TextArea 组件的 Props
#[derive(Props, Clone, PartialEq)]
pub struct TextAreaProps {
    /// 输入框的值
    #[props(default)]
    pub value: Option<String>,
    
    /// 输入框占位符文本
    #[props(default)]
    pub placeholder: Option<String>,
    
    /// 输入框尺寸
    #[props(default)]
    pub size: InputSize,
    
    /// 输入框状态
    #[props(default)]
    pub status: InputStatus,
    
    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,
    
    /// 是否只读
    #[props(default = false)]
    pub readonly: bool,
    
    /// 是否显示清除按钮
    #[props(default = false)]
    pub allow_clear: bool,
    
    /// 是否有边框
    #[props(default = true)]
    pub bordered: bool,
    
    /// 最大输入长度
    #[props(default)]
    pub max_length: Option<usize>,
    
    /// 是否显示字符计数
    #[props(default = false)]
    pub show_count: bool,
    
    /// 文本域行数
    #[props(default = 4)]
    pub rows: usize,
    
    /// 是否自动调整高度
    #[props(default = false)]
    pub auto_size: bool,
    
    /// 最小行数（当 auto_size 为 true 时）
    #[props(default)]
    pub min_rows: Option<usize>,
    
    /// 最大行数（当 auto_size 为 true 时）
    #[props(default)]
    pub max_rows: Option<usize>,
    
    /// 是否允许调整大小
    #[props(default = true)]
    pub resize: bool,
    
    /// 自定义 CSS 类名
    #[props(default)]
    pub class: Option<String>,
    
    /// 自定义内联样式
    #[props(default)]
    pub style: Option<String>,
    
    /// 输入值变化时的回调
    #[props(default)]
    pub on_change: Option<EventHandler<String>>,
    
    /// 按下回车键时的回调
    #[props(default)]
    pub on_press_enter: Option<EventHandler<KeyboardEvent>>,
    
    /// 获得焦点时的回调
    #[props(default)]
    pub on_focus: Option<EventHandler<FocusEvent>>,
    
    /// 失去焦点时的回调
    #[props(default)]
    pub on_blur: Option<EventHandler<FocusEvent>>,
    
    /// 点击清除按钮时的回调
    #[props(default)]
    pub on_clear: Option<EventHandler<MouseEvent>>,
}

/// Input.Search 组件的 Props
#[derive(Props, Clone, PartialEq)]
pub struct SearchProps {
    /// 输入框的值
    #[props(default)]
    pub value: Option<String>,
    
    /// 输入框占位符文本
    #[props(default)]
    pub placeholder: Option<String>,
    
    /// 输入框尺寸
    #[props(default)]
    pub size: InputSize,
    
    /// 输入框状态
    #[props(default)]
    pub status: InputStatus,
    
    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,
    
    /// 是否只读
    #[props(default = false)]
    pub readonly: bool,
    
    /// 是否显示清除按钮
    #[props(default = false)]
    pub allow_clear: bool,
    
    /// 是否有边框
    #[props(default = true)]
    pub bordered: bool,
    
    /// 最大输入长度
    #[props(default)]
    pub max_length: Option<usize>,
    
    /// 是否显示字符计数
    #[props(default = false)]
    pub show_count: bool,
    
    /// 前缀图标或文本
    #[props(default)]
    pub prefix: Option<Element>,
    
    /// 搜索按钮文本
    #[props(default)]
    pub enter_button: Option<String>,
    
    /// 是否显示搜索图标
    #[props(default = true)]
    pub search_icon: bool,
    
    /// 是否正在加载
    #[props(default = false)]
    pub loading: bool,
    
    /// 自定义 CSS 类名
    #[props(default)]
    pub class: Option<String>,
    
    /// 自定义内联样式
    #[props(default)]
    pub style: Option<String>,
    
    /// 输入值变化时的回调
    #[props(default)]
    pub on_change: Option<EventHandler<String>>,
    
    /// 点击搜索按钮或按下回车键时的回调
    #[props(default)]
    pub on_search: Option<EventHandler<String>>,
    
    /// 获得焦点时的回调
    #[props(default)]
    pub on_focus: Option<EventHandler<FocusEvent>>,
    
    /// 失去焦点时的回调
    #[props(default)]
    pub on_blur: Option<EventHandler<FocusEvent>>,
    
    /// 点击清除按钮时的回调
    #[props(default)]
    pub on_clear: Option<EventHandler<MouseEvent>>,
}

/// Input.Password 组件的 Props
#[derive(Props, Clone, PartialEq)]
pub struct PasswordProps {
    /// 输入框的值
    #[props(default)]
    pub value: Option<String>,
    
    /// 输入框占位符文本
    #[props(default)]
    pub placeholder: Option<String>,
    
    /// 输入框尺寸
    #[props(default)]
    pub size: InputSize,
    
    /// 输入框状态
    #[props(default)]
    pub status: InputStatus,
    
    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,
    
    /// 是否只读
    #[props(default = false)]
    pub readonly: bool,
    
    /// 是否有边框
    #[props(default = true)]
    pub bordered: bool,
    
    /// 最大输入长度
    #[props(default)]
    pub max_length: Option<usize>,
    
    /// 是否显示字符计数
    #[props(default = false)]
    pub show_count: bool,
    
    /// 前缀图标或文本
    #[props(default)]
    pub prefix: Option<Element>,
    
    /// 是否显示切换按钮
    #[props(default = true)]
    pub visibility_toggle: bool,
    
    /// 自定义可见性图标
    #[props(default)]
    pub icon_render: Option<fn(bool) -> Element>,
    
    /// 自定义 CSS 类名
    #[props(default)]
    pub class: Option<String>,
    
    /// 自定义内联样式
    #[props(default)]
    pub style: Option<String>,
    
    /// 输入值变化时的回调
    #[props(default)]
    pub on_change: Option<EventHandler<String>>,
    
    /// 按下回车键时的回调
    #[props(default)]
    pub on_press_enter: Option<EventHandler<KeyboardEvent>>,
    
    /// 获得焦点时的回调
    #[props(default)]
    pub on_focus: Option<EventHandler<FocusEvent>>,
    
    /// 失去焦点时的回调
    #[props(default)]
    pub on_blur: Option<EventHandler<FocusEvent>>,
}

/// Input.OTP 组件的 Props
#[derive(Props, Clone, PartialEq)]
pub struct OTPProps {
    /// OTP 的值
    #[props(default)]
    pub value: Option<String>,
    
    /// OTP 长度
    #[props(default = 6)]
    pub length: usize,
    
    /// 输入框尺寸
    #[props(default)]
    pub size: InputSize,
    
    /// 输入框状态
    #[props(default)]
    pub status: InputStatus,
    
    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,
    
    /// 是否有边框
    #[props(default = true)]
    pub bordered: bool,
    
    /// 是否使用掩码显示
    #[props(default = false)]
    pub mask: bool,
    
    /// 自定义 CSS 类名
    #[props(default)]
    pub class: Option<String>,
    
    /// 自定义内联样式
    #[props(default)]
    pub style: Option<String>,
    
    /// 输入值变化时的回调
    #[props(default)]
    pub on_change: Option<EventHandler<String>>,
    
    /// 输入完成时的回调
    #[props(default)]
    pub on_finish: Option<EventHandler<String>>,
}
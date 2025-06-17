//! Tooltip 组件类型定义

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

/// Tooltip 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct TooltipProps {
    /// 提示文本内容
    pub title: Option<String>,
    
    /// 位置
    #[props(default = TooltipPlacement::Top)]
    pub placement: TooltipPlacement,
    
    /// 触发方式
    #[props(default = TooltipTrigger::Hover)]
    pub trigger: TooltipTrigger,
    
    /// 受控显示状态
    pub open: Option<bool>,
    
    /// 默认显示状态
    #[props(default = false)]
    pub default_open: bool,
    
    /// 鼠标进入延迟（秒）
    #[props(default = 0.1)]
    pub mouse_enter_delay: f64,
    
    /// 鼠标离开延迟（秒）
    #[props(default = 0.1)]
    pub mouse_leave_delay: f64,
    
    /// 是否显示箭头
    #[props(default = true)]
    pub arrow: bool,
    
    /// 背景颜色
    pub color: Option<String>,
    
    /// 自动调整溢出
    #[props(default = true)]
    pub auto_adjust_overflow: bool,
    
    /// z-index
    pub z_index: Option<i32>,
    
    /// 显示状态变化回调
    pub on_open_change: Option<EventHandler<bool>>,
    
    /// 子元素
    pub children: Element,
    
    /// CSS 类名
    pub class: Option<String>,
    
    /// 内联样式
    pub style: Option<String>,
}

/// Tooltip 位置枚举
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum TooltipPlacement {
    /// 上方
    Top,
    /// 左侧
    Left,
    /// 右侧
    Right,
    /// 下方
    Bottom,
    /// 上方左对齐
    TopLeft,
    /// 上方右对齐
    TopRight,
    /// 下方左对齐
    BottomLeft,
    /// 下方右对齐
    BottomRight,
    /// 左侧上对齐
    LeftTop,
    /// 左侧下对齐
    LeftBottom,
    /// 右侧上对齐
    RightTop,
    /// 右侧下对齐
    RightBottom,
}

impl Default for TooltipPlacement {
    fn default() -> Self {
        Self::Top
    }
}

impl TooltipPlacement {
    /// 获取位置的 CSS 类名
    pub fn to_class_name(&self) -> &'static str {
        match self {
            Self::Top => "ant-tooltip-placement-top",
            Self::Left => "ant-tooltip-placement-left",
            Self::Right => "ant-tooltip-placement-right",
            Self::Bottom => "ant-tooltip-placement-bottom",
            Self::TopLeft => "ant-tooltip-placement-topLeft",
            Self::TopRight => "ant-tooltip-placement-topRight",
            Self::BottomLeft => "ant-tooltip-placement-bottomLeft",
            Self::BottomRight => "ant-tooltip-placement-bottomRight",
            Self::LeftTop => "ant-tooltip-placement-leftTop",
            Self::LeftBottom => "ant-tooltip-placement-leftBottom",
            Self::RightTop => "ant-tooltip-placement-rightTop",
            Self::RightBottom => "ant-tooltip-placement-rightBottom",
        }
    }
    
    /// 获取箭头的位置偏移
    pub fn get_arrow_offset(&self) -> (f64, f64) {
        match self {
            Self::Top | Self::TopLeft | Self::TopRight => (0.0, 100.0),
            Self::Bottom | Self::BottomLeft | Self::BottomRight => (0.0, -100.0),
            Self::Left | Self::LeftTop | Self::LeftBottom => (100.0, 0.0),
            Self::Right | Self::RightTop | Self::RightBottom => (-100.0, 0.0),
        }
    }
    
    /// 获取相对于目标元素的位置偏移
    pub fn get_position_offset(&self) -> (f64, f64) {
        match self {
            Self::Top => (0.0, -8.0),
            Self::TopLeft => (-8.0, -8.0),
            Self::TopRight => (8.0, -8.0),
            Self::Bottom => (0.0, 8.0),
            Self::BottomLeft => (-8.0, 8.0),
            Self::BottomRight => (8.0, 8.0),
            Self::Left => (-8.0, 0.0),
            Self::LeftTop => (-8.0, -8.0),
            Self::LeftBottom => (-8.0, 8.0),
            Self::Right => (8.0, 0.0),
            Self::RightTop => (8.0, -8.0),
            Self::RightBottom => (8.0, 8.0),
        }
    }
}

/// Tooltip 触发方式枚举
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum TooltipTrigger {
    /// 鼠标悬停
    Hover,
    /// 焦点
    Focus,
    /// 点击
    Click,
    /// 右键菜单
    ContextMenu,
}

impl Default for TooltipTrigger {
    fn default() -> Self {
        Self::Hover
    }
}

/// Tooltip 位置信息
#[derive(Clone, PartialEq, Debug)]
pub struct TooltipPosition {
    /// X 坐标
    pub x: f64,
    /// Y 坐标
    pub y: f64,
    /// 实际使用的位置（可能因边界调整而改变）
    pub actual_placement: TooltipPlacement,
}

/// Tooltip 状态
#[derive(Clone, PartialEq, Debug)]
pub enum TooltipState {
    /// 隐藏
    Hidden,
    /// 显示中
    Showing,
    /// 已显示
    Visible,
    /// 隐藏中
    Hiding,
}

impl Default for TooltipState {
    fn default() -> Self {
        Self::Hidden
    }
}

/// 元素边界信息
#[derive(Clone, PartialEq, Debug, Default)]
pub struct ElementBounds {
    /// 左边距
    pub left: f64,
    /// 上边距
    pub top: f64,
    /// 宽度
    pub width: f64,
    /// 高度
    pub height: f64,
}

impl ElementBounds {
    /// 创建新的边界信息
    pub fn new(left: f64, top: f64, width: f64, height: f64) -> Self {
        Self {
            left,
            top,
            width,
            height,
        }
    }
    
    /// 获取右边距
    pub fn right(&self) -> f64 {
        self.left + self.width
    }
    
    /// 获取下边距
    pub fn bottom(&self) -> f64 {
        self.top + self.height
    }
    
    /// 获取中心点 X 坐标
    pub fn center_x(&self) -> f64 {
        self.left + self.width / 2.0
    }
    
    /// 获取中心点 Y 坐标
    pub fn center_y(&self) -> f64 {
        self.top + self.height / 2.0
    }
}
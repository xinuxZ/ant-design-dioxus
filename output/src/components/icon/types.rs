//! Icon 组件类型定义

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

/// 图标属性
#[derive(Props, Clone, PartialEq)]
pub struct IconProps {
    /// 图标类型/名称
    pub icon_type: String,
    
    /// 是否旋转
    #[props(default = false)]
    pub spin: bool,
    
    /// 图标大小
    #[props(default)]
    pub size: Option<String>,
    
    /// 图标颜色
    #[props(default)]
    pub color: Option<String>,
    
    /// 自定义类名
    #[props(default)]
    pub class: Option<String>,
    
    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,
    
    /// 点击事件处理器
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
}

/// 图标尺寸枚举
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IconSize {
    Small,
    Medium,
    Large,
    Custom(String),
}

impl Default for IconSize {
    fn default() -> Self {
        Self::Medium
    }
}

impl IconSize {
    pub fn to_css_value(&self) -> String {
        match self {
            IconSize::Small => "12px".to_string(),
            IconSize::Medium => "14px".to_string(),
            IconSize::Large => "16px".to_string(),
            IconSize::Custom(size) => size.clone(),
        }
    }
}
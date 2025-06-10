//! Divider组件的样式生成器
//!
//! 提供Divider组件的样式生成功能，包括水平/垂直分割线、带文字的分割线、虚线等样式的生成。

use serde::{Deserialize, Serialize};

use css_in_rust::css;

/// 分割线类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DividerType {
    /// 水平分割线
    Horizontal,
    /// 垂直分割线
    Vertical,
}

impl Default for DividerType {
    fn default() -> Self {
        Self::Horizontal
    }
}

impl DividerType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

/// 分割线文字位置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DividerOrientation {
    /// 左侧
    Left,
    /// 居中
    Center,
    /// 右侧
    Right,
}

impl Default for DividerOrientation {
    fn default() -> Self {
        Self::Center
    }
}

impl DividerOrientation {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Center => "center",
            Self::Right => "right",
        }
    }
}

/// 分割线尺寸
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DividerSize {
    /// 小号
    Small,
    /// 默认
    Default,
    /// 大号
    Large,
}

impl Default for DividerSize {
    fn default() -> Self {
        Self::Default
    }
}

impl DividerSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Default => "default",
            Self::Large => "large",
        }
    }
}

/// Divider组件样式生成器
#[derive(Debug, Clone)]
pub struct DividerStyleGenerator {
    /// 分割线类型
    pub divider_type: DividerType,
    /// 分割线尺寸
    pub size: DividerSize,
    /// 是否虚线
    pub dashed: bool,
    /// 是否简洁模式
    pub plain: bool,
    /// 是否有文字
    pub has_text: bool,
    /// 文字位置
    pub orientation: DividerOrientation,
}

impl DividerStyleGenerator {
    /// 创建新的Divider样式生成器
    pub fn new(
        divider_type: DividerType,
        size: DividerSize,
        dashed: bool,
        plain: bool,
        has_text: bool,
        orientation: DividerOrientation,
    ) -> Self {
        Self {
            divider_type,
            size,
            dashed,
            plain,
            has_text,
            orientation,
        }
    }

    /// 生成Divider组件的CSS样式
    pub fn generate(&self) -> String {
        let mut css_content = String::new();

        // 基本样式
        css_content.push_str(".ant-divider { box-sizing: border-box; margin: 16px 0; padding: 0; color: rgba(0, 0, 0, 0.88); }");

        // 类型样式
        match &self.divider_type {
            DividerType::Horizontal => {
                css_content.push_str(".ant-divider-horizontal { display: flex; clear: both; width: 100%; min-width: 100%; margin: 24px 0; }");
            }
            DividerType::Vertical => {
                css_content.push_str(".ant-divider-vertical { position: relative; top: -0.06em; display: inline-block; height: 0.9em; margin: 0 8px; vertical-align: middle; border-top: 0; border-left: 1px solid rgba(5, 5, 5, 0.06); }");
            }
        }

        // 尺寸样式
        match &self.size {
            DividerSize::Small => {
                css_content.push_str(".ant-divider-small { margin: 8px 0; }");
            }
            DividerSize::Default => {
                // 默认尺寸已包含在基本样式中
            }
            DividerSize::Large => {
                css_content.push_str(".ant-divider-large { margin: 32px 0; }");
            }
        }

        // 虚线样式
        if self.dashed {
            css_content.push_str(".ant-divider-dashed { background: none; border-style: dashed; border-width: 1px 0 0; }");
        }

        // 简洁模式样式
        if self.plain {
            css_content.push_str(".ant-divider-plain { font-weight: normal; }");
        }

        // 文字样式
        if self.has_text {
            css_content.push_str(".ant-divider-with-text { display: flex; margin: 16px 0; color: rgba(0, 0, 0, 0.88); font-weight: 500; font-size: 16px; white-space: nowrap; text-align: center; border-top: 0; border-top-color: rgba(5, 5, 5, 0.06); }");

            match &self.orientation {
                DividerOrientation::Left => {
                    css_content.push_str(".ant-divider-with-text-left::before { width: 5%; } .ant-divider-with-text-left::after { width: 95%; }");
                }
                DividerOrientation::Center => {
                    css_content.push_str(".ant-divider-with-text-center::before, .ant-divider-with-text-center::after { width: 50%; }");
                }
                DividerOrientation::Right => {
                    css_content.push_str(".ant-divider-with-text-right::before { width: 95%; } .ant-divider-with-text-right::after { width: 5%; }");
                }
            }
        }

        css_content
    }

    /// 生成Divider组件的基础样式
    pub fn base_style() -> String {
        let css_content = r#"
            .ant-divider {
                box-sizing: border-box;
                margin: 16px 0;
                padding: 0;
                color: rgba(0, 0, 0, 0.88);
                font-size: 14px;
                font-variant: tabular-nums;
                line-height: 1.5714285714285714;
                list-style: none;
                border-top: 1px solid rgba(5, 5, 5, 0.06);
                transition: all 0.2s;
            }

            .ant-divider-horizontal {
                display: flex;
                clear: both;
                width: 100%;
                min-width: 100%;
                margin: 24px 0;
            }

            .ant-divider-with-text {
                display: flex;
                margin: 16px 0;
                color: rgba(0, 0, 0, 0.88);
                font-weight: 500;
                font-size: 16px;
                white-space: nowrap;
                text-align: center;
                border-top: 0;
                border-top-color: rgba(5, 5, 5, 0.06);
            }

            .ant-divider-with-text::before,
            .ant-divider-with-text::after {
                position: relative;
                top: 50%;
                width: 50%;
                border-top: 1px solid transparent;
                border-top-color: inherit;
                border-bottom: 0;
                transform: translateY(50%);
                content: "";
            }

            .ant-divider-with-text-left::before {
                width: 5%;
            }

            .ant-divider-with-text-left::after {
                width: 95%;
            }

            .ant-divider-with-text-right::before {
                width: 95%;
            }

            .ant-divider-with-text-right::after {
                width: 5%;
            }

            .ant-divider-inner-text {
                display: inline-block;
                padding: 0 1em;
            }

            .ant-divider-dashed {
                background: none;
                border-style: dashed;
                border-width: 1px 0 0;
            }

            .ant-divider-vertical {
                position: relative;
                top: -0.06em;
                display: inline-block;
                height: 0.9em;
                margin: 0 8px;
                vertical-align: middle;
                border-top: 0;
                border-left: 1px solid rgba(5, 5, 5, 0.06);
            }

            .ant-divider-plain {
                font-weight: normal;
            }

            .ant-divider-small {
                margin: 8px 0;
            }

            .ant-divider-large {
                margin: 32px 0;
            }
        "#
        .to_string();

        css_content
    }
}

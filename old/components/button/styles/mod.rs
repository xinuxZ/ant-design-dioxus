//! Button组件样式模块
//!
//! 本模块包含Button组件的所有样式定义，从组件逻辑中分离出来，
//! 提高代码的可维护性和复用性。

use serde::{Deserialize, Serialize};

use css_in_rust::css;

use crate::shared::styles::mixins::*;
// use crate::shared::styles::tokens::DesignToken;

/// 按钮类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ButtonType {
    /// 主按钮
    Primary,
    /// 默认按钮
    Default,
    /// 虚线按钮
    Dashed,
    /// 文本按钮
    Text,
    /// 链接按钮
    Link,
}
impl Default for ButtonType {
    fn default() -> Self {
        Self::Default
    }
}

/// 按钮尺寸
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ButtonSize {
    /// 大尺寸
    Large,
    /// 中等尺寸（默认）
    Middle,
    /// 小尺寸
    Small,
}

impl Default for ButtonSize {
    fn default() -> Self {
        Self::Middle
    }
}

/// 按钮形状
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ButtonShape {
    /// 默认形状
    Default,
    /// 圆形按钮
    Circle,
    /// 圆角按钮
    Round,
}

impl Default for ButtonShape {
    fn default() -> Self {
        Self::Default
    }
}

/// 按钮 HTML 类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ButtonHtmlType {
    /// 提交按钮
    Submit,
    /// 重置按钮
    Reset,
    /// 普通按钮
    Button,
}

impl Default for ButtonHtmlType {
    fn default() -> Self {
        Self::Button
    }
}

/// 按钮组尺寸
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ButtonGroupSize {
    /// 大尺寸
    Large,
    /// 中等尺寸（默认）
    Middle,
    /// 小尺寸
    Small,
}

impl Default for ButtonGroupSize {
    fn default() -> Self {
        Self::Middle
    }
}

impl From<ButtonGroupSize> for ButtonSize {
    fn from(size: ButtonGroupSize) -> Self {
        match size {
            ButtonGroupSize::Large => ButtonSize::Large,
            ButtonGroupSize::Middle => ButtonSize::Middle,
            ButtonGroupSize::Small => ButtonSize::Small,
        }
    }
}

/// 按钮样式生成器
pub struct ButtonStyleGenerator {
    pub button_type: ButtonType,
    pub size: ButtonSize,
    pub shape: ButtonShape,
    pub danger: bool,
    pub ghost: bool,
    pub disabled: bool,
    pub loading: bool,
    pub block: bool,
}

impl ButtonStyleGenerator {
    /// 创建新的按钮样式生成器
    pub fn new() -> Self {
        Self {
            button_type: ButtonType::Default,
            size: ButtonSize::Middle,
            shape: ButtonShape::Default,
            danger: false,
            ghost: false,
            disabled: false,
            loading: false,
            block: false,
        }
    }

    /// 设置按钮类型
    pub fn with_type(mut self, button_type: ButtonType) -> Self {
        self.button_type = button_type;
        self
    }

    /// 设置按钮尺寸
    pub fn with_size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    /// 设置按钮形状
    pub fn with_shape(mut self, shape: ButtonShape) -> Self {
        self.shape = shape;
        self
    }

    /// 设置危险状态
    pub fn with_danger(mut self, danger: bool) -> Self {
        self.danger = danger;
        self
    }

    /// 设置幽灵状态
    pub fn with_ghost(mut self, ghost: bool) -> Self {
        self.ghost = ghost;
        self
    }

    /// 设置禁用状态
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// 设置加载状态
    pub fn with_loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    /// 设置块级状态
    pub fn with_block(mut self, block: bool) -> Self {
        self.block = block;
        self
    }

    /// 生成完整的按钮样式类名
    pub fn generate(&self) -> String {
        let mut classes = vec![
            self.base_style(),
            self.type_style(),
            self.size_style(),
            self.shape_style(),
        ];

        if self.danger {
            classes.push(self.danger_style());
        }

        if self.ghost {
            classes.push(self.ghost_style());
        }

        if self.disabled {
            classes.push(self.disabled_style());
        }

        if self.loading {
            classes.push(self.loading_style());
        }

        if self.block {
            classes.push(self.block_style());
        }

        classes.join(" ")
    }

    /// 基础按钮样式
    fn base_style(&self) -> String {
        format!(
            "{} {}",
            button_base(),
            css!(
                r#"
            &:hover {
                color: var(--ant-primary-color);
                border-color: var(--ant-primary-color);
            }

            &:focus {
                color: var(--ant-primary-color);
                border-color: var(--ant-primary-color);
                outline: 0;
                box-shadow: 0 0 0 2px rgba(24, 144, 255, 0.2);
            }

            &:active {
                color: var(--ant-primary-color-active);
                border-color: var(--ant-primary-color-active);
            }
            "#
            )
        )
    }

    /// 按钮类型样式
    fn type_style(&self) -> String {
        let style = match self.button_type {
            ButtonType::Primary => css!(
                r#"
                color: #fff;
                background: var(--ant-primary-color);
                border-color: var(--ant-primary-color);

                &:hover {
                    color: #fff;
                    background: var(--ant-primary-color-hover);
                    border-color: var(--ant-primary-color-hover);
                }

                &:active {
                    color: #fff;
                    background: var(--ant-primary-color-active);
                    border-color: var(--ant-primary-color-active);
                }
                "#
            ),
            ButtonType::Dashed => css!(
                r#"
                border-style: dashed;
                "#
            ),
            ButtonType::Text => css!(
                "
                color: var(--ant-text-color);
                background: transparent;
                border-color: transparent;
                box-shadow: none;

                &:hover {
                    color: var(--ant-primary-color);
                    background: rgba(0, 0, 0, 0.04);
                }

                &:active {
                    background: rgba(0, 0, 0, 0.08);
                }
                "
            ),
            ButtonType::Link => css!(
                r#"
                color: var(--ant-primary-color);
                background: transparent;
                border-color: transparent;
                box-shadow: none;
                text-decoration: none;

                &:hover {
                    color: var(--ant-primary-color-hover);
                    text-decoration: underline;
                }

                &:active {
                    color: var(--ant-primary-color-active);
                }
                "#
            ),
            ButtonType::Default => css!(
                r#"
                color: var(--ant-text-color);
                background: var(--ant-bg-color);
                border-color: var(--ant-border-color);
                "#
            ),
        };

        style.to_string()
    }

    /// 按钮尺寸样式
    fn size_style(&self) -> String {
        let style = match self.size {
            ButtonSize::Large => css!(
                r#"
                height: 40px;
                padding: 6px 15px;
                font-size: 16px;
                border-radius: var(--ant-border-radius);
                "#
            ),
            ButtonSize::Small => css!(
                r#"
                height: 24px;
                padding: 0px 7px;
                font-size: var(--ant-font-size-sm);
                border-radius: var(--ant-border-radius-sm);
                "#
            ),
            ButtonSize::Middle => css!(
                r#"
                height: 32px;
                padding: 4px 15px;
                font-size: var(--ant-font-size-base);
                border-radius: var(--ant-border-radius);
                "#
            ),
        };

        style.to_string()
    }

    /// 按钮形状样式
    fn shape_style(&self) -> String {
        match self.shape {
            ButtonShape::Circle => css!(
                r#"
                border-radius: 50%;
                width: 32px;
                padding: 0;
                "#
            )
            .to_string(),
            ButtonShape::Round => css!(
                r#"
                border-radius: 32px;
                "#
            )
            .to_string(),
            ButtonShape::Default => String::new(),
        }
    }

    /// 危险状态样式
    fn danger_style(&self) -> String {
        let style = match self.button_type {
            ButtonType::Primary => css!(
                r#"
                background: var(--ant-error-color);
                border-color: var(--ant-error-color);

                &:hover {
                    background: #ff7875;
                    border-color: #ff7875;
                }

                &:active {
                    background: #d9363e;
                    border-color: #d9363e;
                }
                "#
            ),
            _ => css!(
                r#"
                color: var(--ant-error-color);
                border-color: var(--ant-error-color);

                &:hover {
                    color: #ff7875;
                    border-color: #ff7875;
                }

                &:active {
                    color: #d9363e;
                    border-color: #d9363e;
                }
                "#
            ),
        };

        style.to_string()
    }

    /// 幽灵状态样式
    fn ghost_style(&self) -> String {
        css!(
            r#"
            background: transparent;

            &:hover {
                background: transparent;
            }
            "#
        )
        .to_string()
    }

    /// 禁用状态样式
    fn disabled_style(&self) -> String {
        format!(
            "{} {}",
            disabled_state(),
            css!(
                r#"
            color: var(--ant-text-color-disabled);
            background: var(--ant-bg-color-container);
            border-color: var(--ant-border-color);
            box-shadow: none;
            "#
            )
        )
    }

    /// 加载状态样式
    fn loading_style(&self) -> String {
        format!(
            "{} {}",
            loading_state(),
            css!(
                r#"
            pointer-events: none;
            "#
            )
        )
    }

    /// 块级状态样式
    fn block_style(&self) -> String {
        css!(
            r#"
            width: 100%;
            display: block;
            "#
        )
        .to_string()
    }
}

/// 按钮组样式
pub fn button_group_style() -> String {
    css!(
        r#"
        position: relative;
        display: inline-block;

        & > .ant-btn {
            position: relative;
            z-index: 1;
        }

        & > .ant-btn:hover,
        & > .ant-btn:focus,
        & > .ant-btn:active {
            z-index: 2;
        }

        & > .ant-btn:not(:first-child):not(:last-child) {
            border-radius: 0;
        }

        & > .ant-btn:first-child:not(:last-child) {
            border-top-right-radius: 0;
            border-bottom-right-radius: 0;
        }

        & > .ant-btn:last-child:not(:first-child) {
            border-top-left-radius: 0;
            border-bottom-left-radius: 0;
        }

        & > .ant-btn:not(:first-child) {
            margin-left: -1px;
        }
        "#
    )
    .to_string()
}

/// 按钮加载图标样式
pub fn button_loading_icon_style() -> String {
    css!(
        r#"
        display: inline-block;
        margin-right: 8px;
        "#
    )
    .to_string()
}

/// 按钮内容样式
pub fn button_content_style() -> String {
    css!(
        r#"
        display: inline-block;
        "#
    )
    .to_string()
}

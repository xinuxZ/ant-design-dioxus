//! 按钮样式生成器
//!
//! 提供按钮样式的生成逻辑，支持链式调用和组合不同的样式状态

use crate::components::button::types::*;
use crate::shared::styles::mixins::*;
use css_in_rust::css;

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
                r#"
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
                "#
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

impl Default for ButtonStyleGenerator {
    fn default() -> Self {
        Self::new()
    }
}
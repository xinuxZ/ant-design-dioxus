//! Button 组件基础样式
//!
//! 包含 Button 组件的所有样式定义和生成逻辑

use std::collections::HashMap;

use super::super::types::*;
use css_in_rust::css;

/// Button 样式结构
#[derive(PartialEq)]
pub struct ButtonStyles {
    pub base: String,
    pub variants: VariantStyles,
    pub states: StateStyles,
}

/// 变体样式
#[derive(PartialEq)]
pub struct VariantStyles {
    pub primary: String,
    pub dashed: String,
    pub text: String,
    pub link: String,
    pub large: String,
    pub small: String,
    pub circle: String,
    pub round: String,
}

/// 状态样式
#[derive(PartialEq)]
pub struct StateStyles {
    pub dangerous: String,
    pub ghost: String,
    pub loading: String,
    pub block: String,
    pub disabled: String,
}

/// Button 样式生成器
pub struct ButtonStyleGenerator;

impl ButtonStyleGenerator {
    /// 生成完整的按钮样式
    pub fn generate() -> ButtonStyles {
        ButtonStyles {
            base: Self::base_styles(),
            variants: Self::variant_styles(),
            states: Self::state_styles(),
        }
    }

    /// 基础样式
    fn base_styles() -> String {
        css!(
            r#"
            .ant-btn {
                position: relative;
                display: inline-block;
                font-weight: 400;
                white-space: nowrap;
                text-align: center;
                background-image: none;
                border: 1px solid transparent;
                box-shadow: 0 2px 0 rgba(0, 0, 0, 0.015);
                cursor: pointer;
                transition: all 0.3s cubic-bezier(0.645, 0.045, 0.355, 1);
                user-select: none;
                touch-action: manipulation;
                height: 32px;
                padding: 4px 15px;
                font-size: 14px;
                border-radius: 6px;
                color: rgba(0, 0, 0, 0.88);
                background: #ffffff;
                border-color: #d9d9d9;
                outline: none;
                line-height: 1.5714285714285714;
            }

            .ant-btn:hover {
                color: #4096ff;
                background: #ffffff;
                border-color: #4096ff;
            }

            .ant-btn:focus {
                color: #4096ff;
                background: #ffffff;
                border-color: #4096ff;
                outline: 0;
                box-shadow: 0 0 0 2px rgba(5, 145, 255, 0.1);
            }

            .ant-btn:active {
                color: #0958d9;
                background: #ffffff;
                border-color: #0958d9;
            }

            .ant-btn:disabled {
                color: rgba(0, 0, 0, 0.25);
                background: rgba(0, 0, 0, 0.04);
                border-color: #d9d9d9;
                cursor: not-allowed;
                box-shadow: none;
            }
        "#
        )
        .to_string()
    }

    /// 变体样式
    fn variant_styles() -> VariantStyles {
        VariantStyles {
            primary: css!(
                r#"
                .ant-btn-primary {
                    color: #fff;
                    background: #1677ff;
                    border-color: #1677ff;
                    box-shadow: 0 2px 0 rgba(5, 145, 255, 0.1);
                }

                .ant-btn-primary:hover {
                    color: #fff;
                    background: #4096ff;
                    border-color: #4096ff;
                }

                .ant-btn-primary:focus {
                    color: #fff;
                    background: #4096ff;
                    border-color: #4096ff;
                    outline: 0;
                    box-shadow: 0 0 0 2px rgba(5, 145, 255, 0.1);
                }

                .ant-btn-primary:active {
                    color: #fff;
                    background: #0958d9;
                    border-color: #0958d9;
                }

                .ant-btn-primary:disabled {
                    color: rgba(0, 0, 0, 0.25);
                    background: rgba(0, 0, 0, 0.04);
                    border-color: #d9d9d9;
                    box-shadow: none;
                }
            "#
            )
            .to_string(),
            dashed: css!(
                r#"
                .ant-btn-dashed {
                    border-style: dashed;
                }
            "#
            )
            .to_string(),
            text: css!(
                r#"
                .ant-btn-text {
                    color: rgba(0, 0, 0, 0.88);
                    background: transparent;
                    border-color: transparent;
                    box-shadow: none;
                }

                .ant-btn-text:hover {
                    color: rgba(0, 0, 0, 0.88);
                    background: rgba(0, 0, 0, 0.06);
                    border-color: transparent;
                }

                .ant-btn-text:focus {
                    color: rgba(0, 0, 0, 0.88);
                    background: rgba(0, 0, 0, 0.06);
                    border-color: transparent;
                    outline: 0;
                    box-shadow: 0 0 0 2px rgba(5, 145, 255, 0.1);
                }

                .ant-btn-text:active {
                    color: rgba(0, 0, 0, 0.88);
                    background: rgba(0, 0, 0, 0.15);
                    border-color: transparent;
                }
            "#
            )
            .to_string(),
            link: css!(
                r#"
                .ant-btn-link {
                    color: #1677ff;
                    background: transparent;
                    border-color: transparent;
                    box-shadow: none;
                }

                .ant-btn-link:hover {
                    color: #4096ff;
                    background: transparent;
                    border-color: transparent;
                }

                .ant-btn-link:focus {
                    color: #4096ff;
                    background: transparent;
                    border-color: transparent;
                    outline: 0;
                    box-shadow: 0 0 0 2px rgba(5, 145, 255, 0.1);
                }

                .ant-btn-link:active {
                    color: #0958d9;
                    background: transparent;
                    border-color: transparent;
                }
            "#
            )
            .to_string(),
            large: css!(
                r#"
                .ant-btn-lg {
                    height: 40px;
                    padding: 6.4px 15px;
                    font-size: 16px;
                    border-radius: 6px;
                }
            "#
            )
            .to_string(),
            small: css!(
                r#"
                .ant-btn-sm {
                    height: 24px;
                    padding: 0px 7px;
                    font-size: 14px;
                    border-radius: 4px;
                }
            "#
            )
            .to_string(),
            circle: css!(
                r#"
                .ant-btn-circle {
                    min-width: 32px;
                    padding-left: 0;
                    padding-right: 0;
                    border-radius: 50%;
                }

                .ant-btn-circle.ant-btn-lg {
                    min-width: 40px;
                }

                .ant-btn-circle.ant-btn-sm {
                    min-width: 24px;
                }
            "#
            )
            .to_string(),
            round: css!(
                r#"
                .ant-btn-round {
                    border-radius: 32px;
                }

                .ant-btn-round.ant-btn-lg {
                    border-radius: 40px;
                }

                .ant-btn-round.ant-btn-sm {
                    border-radius: 24px;
                }
            "#
            )
            .to_string(),
        }
    }

    /// 状态样式
    fn state_styles() -> StateStyles {
        StateStyles {
            dangerous: css!(
                r#"
                .ant-btn-dangerous {
                    color: #ff4d4f;
                    border-color: #ff4d4f;
                }

                .ant-btn-dangerous:hover {
                    color: #ff7875;
                    border-color: #ff7875;
                }

                .ant-btn-dangerous:focus {
                    color: #ff7875;
                    border-color: #ff7875;
                    outline: 0;
                    box-shadow: 0 0 0 2px rgba(255, 77, 79, 0.1);
                }

                .ant-btn-dangerous:active {
                    color: #d9363e;
                    border-color: #d9363e;
                }

                .ant-btn-dangerous.ant-btn-primary {
                    color: #fff;
                    background: #ff4d4f;
                    border-color: #ff4d4f;
                }

                .ant-btn-dangerous.ant-btn-primary:hover {
                    color: #fff;
                    background: #ff7875;
                    border-color: #ff7875;
                }

                .ant-btn-dangerous.ant-btn-primary:focus {
                    color: #fff;
                    background: #ff7875;
                    border-color: #ff7875;
                    outline: 0;
                    box-shadow: 0 0 0 2px rgba(255, 77, 79, 0.1);
                }

                .ant-btn-dangerous.ant-btn-primary:active {
                    color: #fff;
                    background: #d9363e;
                    border-color: #d9363e;
                }
            "#
            )
            .to_string(),
            ghost: css!(
                r#"
                .ant-btn-background-ghost {
                    color: #fff;
                    background: transparent;
                    border-color: #fff;
                }

                .ant-btn-background-ghost:hover {
                    color: rgba(255, 255, 255, 0.8);
                    background: transparent;
                    border-color: rgba(255, 255, 255, 0.8);
                }

                .ant-btn-background-ghost:focus {
                    color: rgba(255, 255, 255, 0.8);
                    background: transparent;
                    border-color: rgba(255, 255, 255, 0.8);
                    outline: 0;
                    box-shadow: 0 0 0 2px rgba(255, 255, 255, 0.1);
                }

                .ant-btn-background-ghost:active {
                    color: rgba(255, 255, 255, 0.65);
                    background: transparent;
                    border-color: rgba(255, 255, 255, 0.65);
                }
            "#
            )
            .to_string(),
            loading: css!(
                r#"
                .ant-btn-loading {
                    position: relative;
                    pointer-events: none;
                }

                .ant-btn-loading-icon {
                    display: inline-block;
                    margin-right: 8px;
                    animation: loadingCircle 1s infinite linear;
                }

                @keyframes loadingCircle {
                    100% {
                        transform: rotate(360deg);
                    }
                }
            "#
            )
            .to_string(),
            block: css!(
                r#"
                .ant-btn-block {
                    width: 100%;
                }
            "#
            )
            .to_string(),
            disabled: css!(
                r#"
                .ant-btn:disabled {
                    color: rgba(0, 0, 0, 0.25);
                    background: rgba(0, 0, 0, 0.04);
                    border-color: #d9d9d9;
                    cursor: not-allowed;
                    box-shadow: none;
                }
            "#
            )
            .to_string(),
        }
    }
}

/// 生成按钮样式的便捷函数
pub fn generate_button_styles() -> ButtonStyles {
    ButtonStyleGenerator::generate()
}

/// 按钮组样式
pub fn generate_button_group_styles() -> String {
    css!(
        r#"
        .ant-btn-group {
            position: relative;
            display: inline-flex;
        }

        .ant-btn-group > .ant-btn {
            position: relative;
            border-radius: 0;
        }

        .ant-btn-group > .ant-btn:first-child {
            border-top-left-radius: 6px;
            border-bottom-left-radius: 6px;
        }

        .ant-btn-group > .ant-btn:last-child {
            border-top-right-radius: 6px;
            border-bottom-right-radius: 6px;
        }

        .ant-btn-group > .ant-btn:not(:first-child) {
            margin-left: -1px;
        }

        .ant-btn-group > .ant-btn:hover,
        .ant-btn-group > .ant-btn:focus,
        .ant-btn-group > .ant-btn:active {
            z-index: 2;
        }

        .ant-btn-group-lg > .ant-btn {
            height: 40px;
            padding: 6.4px 15px;
            font-size: 16px;
        }

        .ant-btn-group-lg > .ant-btn:first-child {
            border-top-left-radius: 6px;
            border-bottom-left-radius: 6px;
        }

        .ant-btn-group-lg > .ant-btn:last-child {
            border-top-right-radius: 6px;
            border-bottom-right-radius: 6px;
        }

        .ant-btn-group-sm > .ant-btn {
            height: 24px;
            padding: 0px 7px;
            font-size: 14px;
        }

        .ant-btn-group-sm > .ant-btn:first-child {
            border-top-left-radius: 4px;
            border-bottom-left-radius: 4px;
        }

        .ant-btn-group-sm > .ant-btn:last-child {
            border-top-right-radius: 4px;
            border-bottom-right-radius: 4px;
        }
    "#
    )
    .to_string()
}

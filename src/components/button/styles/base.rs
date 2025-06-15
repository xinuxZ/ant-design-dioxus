use crate::components::button::types::{ButtonColor, ButtonShape, ButtonSize, ButtonVariant};
use crate::theme::provider::use_css_var_name;
use css_in_rust::css;

/// 生成按钮基础样式
pub fn button_base_styles() -> String {
    // 使用CSS变量名
    let font_size_base = use_css_var_name("fontSize");
    let height_base = use_css_var_name("controlHeight");
    let padding_horizontal = use_css_var_name("paddingContentHorizontal");
    let border_radius_base = use_css_var_name("borderRadius");
    let line_width = use_css_var_name("lineWidth");
    let line_type = use_css_var_name("lineType");
    let transition_duration = use_css_var_name("motionDurationMid");
    let transition_function = use_css_var_name("motionEaseInOut");
    let disabled_color = use_css_var_name("colorTextDisabled");
    let disabled_bg = use_css_var_name("colorBgContainerDisabled");
    let disabled_border = use_css_var_name("colorBorder");

    css!(
        r#"
        /* 按钮基础样式 */
        .ant-btn {
            position: relative;
            display: inline-flex;
            align-items: center;
            justify-content: center;
            font-weight: 400;
            white-space: nowrap;
            text-align: center;
            background-image: none;
            background-color: transparent;
            border: var(--${line_width}) var(--${line_type}) transparent;
            border-radius: var(--${border_radius_base});
            cursor: pointer;
            transition: all var(--${transition_duration}) var(--${transition_function});
            user-select: none;
            touch-action: manipulation;
            height: var(--${height_base});
            padding: 0 var(--${padding_horizontal});
            font-size: var(--${font_size_base});
            box-shadow: 0 2px 0 rgba(0, 0, 0, 0.015);

            /* 确保内部元素垂直居中 */
            > * {
                line-height: 1;
                display: inline-flex;
                align-items: center;
            }

            /* 禁用状态 */
            &[disabled], &[disabled]:hover, &[disabled]:focus, &[disabled]:active {
                color: var(--${disabled_color});
                background: var(--${disabled_bg});
                border-color: var(--${disabled_border});
                text-shadow: none;
                box-shadow: none;
                cursor: not-allowed;
            }

            /* 加载状态 */
            &.ant-btn-loading {
                position: relative;
                pointer-events: none;
                opacity: 0.65;

                .ant-btn-loading-icon {
                    margin-right: 8px;
                }
            }

            /* 图标样式 */
            .anticon {
                display: inline-flex;
                align-items: center;
                justify-content: center;
                line-height: 1;

                /* 图标间距 */
                &:not(:last-child) {
                    margin-right: 8px;
                }
            }

            /* 两个中文字符之间自动插入空格 */
            &.ant-btn-two-chinese-chars > *:not(.anticon) {
                letter-spacing: 0.3em;
                margin-right: -0.3em;
            }
        }

        /* 按钮大小 */
        .ant-btn-lg {
            height: calc(var(--${height_base}) + 8px);
            padding: 0 calc(var(--${padding_horizontal}) + 8px);
            font-size: calc(var(--${font_size_base}) + 2px);
            border-radius: calc(var(--${border_radius_base}) + 2px);
        }

        .ant-btn-sm {
            height: calc(var(--${height_base}) - 8px);
            padding: 0 calc(var(--${padding_horizontal}) - 4px);
            font-size: calc(var(--${font_size_base}) - 2px);
            border-radius: calc(var(--${border_radius_base}) - 1px);
        }

        /* 块级按钮 */
        .ant-btn-block {
            width: 100%;
            display: flex;
        }

        /* 按钮形状 */
        .ant-btn-circle {
            min-width: var(--${height_base});
            padding: 0;
            border-radius: 50%;

            &.ant-btn-lg {
                min-width: calc(var(--${height_base}) + 8px);
            }

            &.ant-btn-sm {
                min-width: calc(var(--${height_base}) - 8px);
            }
        }

        .ant-btn-round {
            border-radius: calc(var(--${height_base}) / 2);
            padding: 0 calc(var(--${padding_horizontal}) + 6px);

            &.ant-btn-lg {
                border-radius: calc((var(--${height_base}) + 8px) / 2);
                padding: 0 calc(var(--${padding_horizontal}) + 12px);
            }

            &.ant-btn-sm {
                border-radius: calc((var(--${height_base}) - 8px) / 2);
                padding: 0 calc(var(--${padding_horizontal}) + 2px);
            }
        }

        /* 按钮组样式 */
        .ant-btn-group {
            display: inline-flex;

            .ant-btn {
                position: relative;
                border-radius: 0;

                &:not(:first-child) {
                    margin-left: -1px;
                }

                &:first-child {
                    border-top-left-radius: var(--${border_radius_base});
                    border-bottom-left-radius: var(--${border_radius_base});
                }

                &:last-child {
                    border-top-right-radius: var(--${border_radius_base});
                    border-bottom-right-radius: var(--${border_radius_base});
                }

                &:hover, &:focus, &:active {
                    z-index: 2;
                }
            }

            &.ant-btn-group-lg .ant-btn {
                &:first-child {
                    border-top-left-radius: calc(var(--${border_radius_base}) + 2px);
                    border-bottom-left-radius: calc(var(--${border_radius_base}) + 2px);
                }

                &:last-child {
                    border-top-right-radius: calc(var(--${border_radius_base}) + 2px);
                    border-bottom-right-radius: calc(var(--${border_radius_base}) + 2px);
                }
            }

            &.ant-btn-group-sm .ant-btn {
                &:first-child {
                    border-top-left-radius: calc(var(--${border_radius_base}) - 1px);
                    border-bottom-left-radius: calc(var(--${border_radius_base}) - 1px);
                }

                &:last-child {
                    border-top-right-radius: calc(var(--${border_radius_base}) - 1px);
                    border-bottom-right-radius: calc(var(--${border_radius_base}) - 1px);
                }
            }
        }

        /* 波纹动画样式 */
        .ant-wave-wrapper {
            position: relative;
            display: inline-block;
            width: 100%;
            height: 100%;
            overflow: hidden;
        }

        .ant-wave-ripple {
            position: absolute;
            background-color: currentColor;
            border-radius: 50%;
            transform: scale(0);
            opacity: 0.6;
            pointer-events: none;
            transition: opacity 0.5s cubic-bezier(0.08, 0.82, 0.17, 1),
                        transform 0.5s cubic-bezier(0.08, 0.82, 0.17, 1);
        }
        "#
    )
    .to_string()
}

/// 生成按钮大小样式
pub fn generate_button_size_style(size: &ButtonSize) -> String {
    match size {
        ButtonSize::Large => "ant-btn-lg".to_string(),
        ButtonSize::Middle => "".to_string(), // 默认大小，不添加类名
        ButtonSize::Small => "ant-btn-sm".to_string(),
    }
}

/// 生成按钮形状样式
pub fn generate_button_shape_style(shape: &ButtonShape) -> String {
    match shape {
        ButtonShape::Default => "".to_string(), // 默认形状，不添加类名
        ButtonShape::Circle => "ant-btn-circle".to_string(),
        ButtonShape::Round => "ant-btn-round".to_string(),
    }
}

/// 生成自定义颜色样式
pub fn generate_custom_color_class(color: &ButtonColor) -> String {
    match color {
        ButtonColor::Custom(hex_color) => {
            let color_name = hex_color.trim_start_matches('#');
            let styles = generate_button_color_styles(color);

            format!(
                r#"
            .ant-btn-custom-{} {{
                {}
            }}
            "#,
                color_name, styles
            )
        }
        _ => String::new(),
    }
}

/// 生成变体样式
pub fn generate_variant_class(variant: &ButtonVariant) -> String {
    let styles = generate_button_variant_styles(variant);

    match variant {
        ButtonVariant::Outlined => format!(
            r#"
            .ant-btn-outlined {{
                {}
            }}
            "#,
            styles
        ),
        ButtonVariant::Solid => format!(
            r#"
            .ant-btn-solid {{
                {}
            }}
            "#,
            styles
        ),
        ButtonVariant::Dashed => format!(
            r#"
            .ant-btn-dashed {{
                {}
            }}
            "#,
            styles
        ),
        ButtonVariant::Text => format!(
            r#"
            .ant-btn-text {{
                {}
            }}
            "#,
            styles
        ),
        ButtonVariant::Link => format!(
            r#"
            .ant-btn-link {{
                {}
            }}
            "#,
            styles
        ),
    }
}

/// 生成按钮样式
pub fn button_styles() -> String {
    button_base_styles()
}

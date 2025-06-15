use crate::components::button::styles::color_variant::{
    generate_button_color_styles, generate_button_variant_styles,
};
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::button::types::*;

    /// 测试 generate_button_size_style 函数
    #[test]
    fn test_generate_button_size_style() {
        let large_style = generate_button_size_style(&ButtonSize::Large);
        assert!(large_style.contains("height: 40px"));
        assert!(large_style.contains("padding: 6.4px 15px"));
        assert!(large_style.contains("font-size: 16px"));

        let middle_style = generate_button_size_style(&ButtonSize::Middle);
        assert!(middle_style.contains("height: 32px"));
        assert!(middle_style.contains("padding: 4px 15px"));
        assert!(middle_style.contains("font-size: 14px"));

        let small_style = generate_button_size_style(&ButtonSize::Small);
        assert!(small_style.contains("height: 24px"));
        assert!(small_style.contains("padding: 0px 7px"));
        assert!(small_style.contains("font-size: 14px"));
    }

    /// 测试 generate_button_shape_style 函数
    #[test]
    fn test_generate_button_shape_style() {
        let default_style = generate_button_shape_style(&ButtonShape::Default);
        assert!(default_style.contains("border-radius: 6px"));

        let circle_style = generate_button_shape_style(&ButtonShape::Circle);
        assert!(circle_style.contains("border-radius: 50%"));
        assert!(circle_style.contains("width: 32px"));
        assert!(circle_style.contains("padding: 4px"));

        let round_style = generate_button_shape_style(&ButtonShape::Round);
        assert!(round_style.contains("border-radius: 32px"));
    }

    /// 测试 generate_variant_class 函数
    #[test]
    fn test_generate_variant_class() {
        let solid_class = generate_variant_class(&ButtonVariant::Solid);
        assert!(solid_class.contains("ant-btn-variant-solid"));

        let outlined_class = generate_variant_class(&ButtonVariant::Outlined);
        assert!(outlined_class.contains("ant-btn-variant-outlined"));

        let dashed_class = generate_variant_class(&ButtonVariant::Dashed);
        assert!(dashed_class.contains("ant-btn-variant-dashed"));

        // let filled_class = generate_variant_class(&ButtonVariant::Filled);
        // assert!(filled_class.contains("ant-btn-variant-filled"));

        let text_class = generate_variant_class(&ButtonVariant::Text);
        assert!(text_class.contains("ant-btn-variant-text"));

        let link_class = generate_variant_class(&ButtonVariant::Link);
        assert!(link_class.contains("ant-btn-variant-link"));
    }

    /// 测试 button_base_styles 函数
    #[test]
    fn test_button_base_styles() {
        let styles = button_base_styles();
        assert!(styles.contains(".ant-btn"));
        assert!(styles.contains("cursor: pointer"));
        assert!(styles.contains("transition: all 0.2s cubic-bezier(0.645, 0.045, 0.355, 1)"));
        assert!(styles.contains("user-select: none"));
        assert!(styles.contains("touch-action: manipulation"));
    }

    /// 测试 button_styles 函数
    #[test]
    fn test_button_styles() {
        let styles = button_styles();
        // button_styles 应该返回与 button_base_styles 相同的内容
        let base_styles = button_base_styles();
        assert_eq!(styles, base_styles);
    }

    /// 测试样式的完整性
    #[test]
    fn test_styles_completeness() {
        let styles = button_base_styles();

        // 检查基本按钮样式
        assert!(styles.contains(".ant-btn"));

        // 检查悬停状态
        assert!(styles.contains(".ant-btn:hover"));

        // 检查激活状态
        assert!(styles.contains(".ant-btn:active"));

        // 检查禁用状态
        assert!(styles.contains(".ant-btn:disabled"));

        // 检查加载状态
        assert!(styles.contains(".ant-btn-loading"));

        // 检查危险状态
        assert!(styles.contains(".ant-btn-dangerous"));

        // 检查幽灵状态
        assert!(styles.contains(".ant-btn-background-ghost"));

        // 检查块级状态
        assert!(styles.contains(".ant-btn-block"));
    }

    /// 测试不同尺寸的样式生成
    #[test]
    fn test_all_button_sizes() {
        let sizes = [ButtonSize::Large, ButtonSize::Middle, ButtonSize::Small];

        for size in sizes.iter() {
            let style = generate_button_size_style(&size);
            assert!(!style.is_empty());
            assert!(style.contains("height:"));
            assert!(style.contains("padding:"));
            assert!(style.contains("font-size:"));
        }
    }

    /// 测试不同形状的样式生成
    #[test]
    fn test_all_button_shapes() {
        let shapes = [
            ButtonShape::Default,
            ButtonShape::Circle,
            ButtonShape::Round,
        ];

        for shape in shapes.iter() {
            let style = generate_button_shape_style(&shape);
            assert!(!style.is_empty());
            assert!(style.contains("border-radius:"));
        }
    }

    /// 测试所有变体的类名生成
    #[test]
    fn test_all_button_variants() {
        let variants = [
            ButtonVariant::Solid,
            ButtonVariant::Outlined,
            ButtonVariant::Dashed,
            // ButtonVariant::Filled,
            ButtonVariant::Text,
            ButtonVariant::Link,
        ];

        for variant in variants.iter() {
            let class = generate_variant_class(&variant);
            assert!(!class.is_empty());
            assert!(class.contains("ant-btn-variant-"));
        }
    }
}

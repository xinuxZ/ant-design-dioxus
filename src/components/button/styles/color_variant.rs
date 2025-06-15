use crate::components::button::types::{ButtonColor, ButtonType, ButtonVariant};
use crate::theme::provider::use_component_token;
use crate::theme::provider::use_css_var_name;
use crate::theme::provider::use_theme;

/// 生成按钮颜色变体的样式
pub fn generate_button_color_style(
    button_type: &ButtonType,
    danger: bool,
    ghost: bool,
    disabled: bool,
    color: Option<&str>,
) -> String {
    // 获取主题上下文
    let theme_context = use_theme();
    let is_dark = matches!(
        theme_context.config.theme.mode,
        crate::theme::ThemeMode::Dark
    );

    // 如果有自定义颜色，优先使用自定义颜色
    if let Some(custom_color) = color {
        return generate_custom_color_style(custom_color, button_type, disabled, ghost);
    }

    // 否则根据按钮类型和状态生成样式
    match button_type {
        ButtonType::Primary => {
            if danger {
                generate_danger_primary_style(disabled, ghost, is_dark)
            } else {
                generate_primary_style(disabled, ghost, is_dark)
            }
        }
        ButtonType::Default => {
            if danger {
                generate_danger_default_style(disabled, ghost, is_dark)
            } else {
                generate_default_style(disabled, ghost, is_dark)
            }
        }
        ButtonType::Dashed => {
            if danger {
                generate_danger_dashed_style(disabled, ghost, is_dark)
            } else {
                generate_dashed_style(disabled, ghost, is_dark)
            }
        }
        ButtonType::Link => {
            if danger {
                generate_danger_link_style(disabled, is_dark)
            } else {
                generate_link_style(disabled, is_dark)
            }
        }
        ButtonType::Text => {
            if danger {
                generate_danger_text_style(disabled, is_dark)
            } else {
                generate_text_style(disabled, is_dark)
            }
        }
    }
}

/// 生成按钮颜色样式
pub fn generate_button_color_styles(color: &ButtonColor) -> String {
    match color {
        ButtonColor::Primary => generate_primary_color_style(),
        ButtonColor::Default => generate_default_color_style(),
        ButtonColor::Danger => generate_danger_color_style(),
        ButtonColor::Custom(custom_color) => {
            generate_custom_color_style(custom_color, &ButtonType::Default, false, false)
        }
    }
}

/// 生成按钮变体样式
pub fn generate_button_variant_styles(variant: &ButtonVariant) -> String {
    match variant {
        ButtonVariant::Outlined => generate_outlined_variant_style(),
        ButtonVariant::Solid => generate_solid_variant_style(),
        ButtonVariant::Dashed => generate_dashed_variant_style(),
        ButtonVariant::Text => generate_text_variant_style(),
        ButtonVariant::Link => generate_link_variant_style(),
    }
}

/// 生成自定义颜色按钮样式
fn generate_custom_color_style(
    color: &str,
    button_type: &ButtonType,
    disabled: bool,
    ghost: bool,
) -> String {
    if disabled {
        return "".to_string(); // 禁用状态使用默认禁用样式
    }

    match button_type {
        ButtonType::Primary => {
            if ghost {
                format!(
                    "
                    color: {color};
                    border-color: {color};
                    background: transparent;

                    &:hover, &:focus {{
                        color: {color}e6;
                        border-color: {color}e6;
                    }}

                    &:active {{
                        color: {color}cc;
                        border-color: {color}cc;
                    }}
                    "
                )
            } else {
                format!(
                    "
                    color: #fff;
                    background: {color};
                    border-color: {color};
                    text-shadow: 0 -1px 0 rgba(0, 0, 0, 0.12);
                    box-shadow: 0 2px 0 rgba(0, 0, 0, 0.045);

                    &:hover, &:focus {{
                        background: {color}e6;
                        border-color: {color}e6;
                    }}

                    &:active {{
                        background: {color}cc;
                        border-color: {color}cc;
                    }}
                    "
                )
            }
        }
        ButtonType::Default | ButtonType::Dashed => {
            let border_style = if *button_type == ButtonType::Dashed {
                "border-style: dashed;"
            } else {
                ""
            };

            format!(
                "
                color: {color};
                border-color: {color};
                background: transparent;
                {border_style}

                &:hover, &:focus {{
                    color: {color}e6;
                    border-color: {color}e6;
                }}

                &:active {{
                    color: {color}cc;
                    border-color: {color}cc;
                }}
                "
            )
        }
        ButtonType::Link | ButtonType::Text => {
            format!(
                "
                color: {color};
                border-color: transparent;
                background: transparent;

                &:hover, &:focus {{
                    color: {color}e6;
                }}

                &:active {{
                    color: {color}cc;
                }}
                "
            )
        }
    }
}

/// 生成主要按钮样式
fn generate_primary_style(disabled: bool, ghost: bool, is_dark: bool) -> String {
    // 使用组件令牌获取按钮样式变量
    let primary_bg = use_component_token("button", "buttonPrimaryBg");
    let primary_color = use_component_token("button", "buttonPrimaryColor");
    let primary_hover_bg = use_component_token("button", "buttonPrimaryHoverBg");
    let primary_active_bg = use_component_token("button", "buttonPrimaryActiveBg");

    // 使用CSS变量名
    let primary_bg_var = use_css_var_name("colorPrimary");
    let primary_hover_bg_var = use_css_var_name("colorPrimaryHover");
    let primary_active_bg_var = use_css_var_name("colorPrimaryActive");

    if disabled {
        format!(
            "
            color: rgba(0, 0, 0, 0.25);
            background: #f5f5f5;
            border-color: #d9d9d9;
            text-shadow: none;
            box-shadow: none;
            "
        )
    } else if ghost {
        format!(
            "
            color: var(--{primary_bg_var});
            background: transparent;
            border-color: var(--{primary_bg_var});
            text-shadow: none;

            &:hover, &:focus {{
                color: var(--{primary_hover_bg_var});
                border-color: var(--{primary_hover_bg_var});
            }}

            &:active {{
                color: var(--{primary_active_bg_var});
                border-color: var(--{primary_active_bg_var});
            }}
            "
        )
    } else {
        format!(
            "
            color: {primary_color};
            background: var(--{primary_bg_var});
            border-color: var(--{primary_bg_var});
            text-shadow: 0 -1px 0 rgba(0, 0, 0, 0.12);
            box-shadow: 0 2px 0 rgba(0, 0, 0, 0.045);

            &:hover, &:focus {{
                background: var(--{primary_hover_bg_var});
                border-color: var(--{primary_hover_bg_var});
            }}

            &:active {{
                background: var(--{primary_active_bg_var});
                border-color: var(--{primary_active_bg_var});
            }}
            "
        )
    }
}

/// 生成默认按钮样式
fn generate_default_style(disabled: bool, ghost: bool, is_dark: bool) -> String {
    // 使用组件令牌获取按钮样式变量
    let default_bg = use_component_token("button", "buttonDefaultBg");
    let default_color = use_component_token("button", "buttonDefaultColor");
    let default_border = use_component_token("button", "buttonDefaultBorderColor");
    let hover_color = use_component_token("button", "buttonPrimaryHoverBg");
    let active_color = use_component_token("button", "buttonPrimaryActiveBg");

    // 使用CSS变量名
    let primary_color_var = use_css_var_name("colorPrimary");
    let primary_hover_color_var = use_css_var_name("colorPrimaryHover");
    let primary_active_color_var = use_css_var_name("colorPrimaryActive");
    let border_color_var = use_css_var_name("colorBorder");

    if disabled {
        format!(
            "
            color: rgba(0, 0, 0, 0.25);
            background: #f5f5f5;
            border-color: #d9d9d9;
            text-shadow: none;
            box-shadow: none;
            "
        )
    } else if ghost {
        let text_color = if is_dark { "#fff" } else { "#000" };
        let border_color = if is_dark { "#434343" } else { "#d9d9d9" };

        format!(
            "
            color: {text_color};
            background: transparent;
            border-color: {border_color};

            &:hover, &:focus {{
                color: var(--{primary_hover_color_var});
                border-color: var(--{primary_hover_color_var});
            }}

            &:active {{
                color: var(--{primary_active_color_var});
                border-color: var(--{primary_active_color_var});
            }}
            "
        )
    } else {
        format!(
            "
            color: {default_color};
            background: {default_bg};
            border-color: var(--{border_color_var});

            &:hover, &:focus {{
                color: var(--{primary_hover_color_var});
                border-color: var(--{primary_hover_color_var});
            }}

            &:active {{
                color: var(--{primary_active_color_var});
                border-color: var(--{primary_active_color_var});
            }}
            "
        )
    }
}

/// 生成虚线按钮样式
fn generate_dashed_style(disabled: bool, ghost: bool, is_dark: bool) -> String {
    let base_style = generate_default_style(disabled, ghost, is_dark);
    format!("{}\nborder-style: dashed;", base_style)
}

/// 生成链接按钮样式
fn generate_link_style(disabled: bool, is_dark: bool) -> String {
    // 使用组件令牌和CSS变量
    let primary_color_var = use_css_var_name("colorPrimary");
    let primary_hover_color_var = use_css_var_name("colorPrimaryHover");
    let primary_active_color_var = use_css_var_name("colorPrimaryActive");

    if disabled {
        format!(
            "
            color: rgba(0, 0, 0, 0.25);
            background: transparent;
            border-color: transparent;
            text-shadow: none;
            box-shadow: none;
            "
        )
    } else {
        format!(
            "
            color: var(--{primary_color_var});
            background: transparent;
            border-color: transparent;

            &:hover, &:focus {{
                color: var(--{primary_hover_color_var});
            }}

            &:active {{
                color: var(--{primary_active_color_var});
            }}
            "
        )
    }
}

/// 生成文本按钮样式
fn generate_text_style(disabled: bool, is_dark: bool) -> String {
    let text_color = if is_dark {
        "rgba(255, 255, 255, 0.85)"
    } else {
        "rgba(0, 0, 0, 0.88)"
    };
    let hover_bg = if is_dark {
        "rgba(255, 255, 255, 0.03)"
    } else {
        "rgba(0, 0, 0, 0.06)"
    };
    let active_bg = if is_dark {
        "rgba(255, 255, 255, 0.06)"
    } else {
        "rgba(0, 0, 0, 0.15)"
    };

    if disabled {
        format!(
            "
            color: rgba(0, 0, 0, 0.25);
            background: transparent;
            border-color: transparent;
            text-shadow: none;
            box-shadow: none;
            "
        )
    } else {
        format!(
            "
            color: {text_color};
            background: transparent;
            border-color: transparent;

            &:hover, &:focus {{
                background: {hover_bg};
                color: {text_color};
            }}

            &:active {{
                background: {active_bg};
                color: {text_color};
            }}
            "
        )
    }
}

/// 生成危险主要按钮样式
fn generate_danger_primary_style(disabled: bool, ghost: bool, is_dark: bool) -> String {
    // 使用组件令牌和CSS变量
    let error_color_var = use_css_var_name("colorError");
    let error_hover_color_var = use_css_var_name("colorErrorHover");
    let error_active_color_var = use_css_var_name("colorErrorActive");

    if disabled {
        format!(
            "
            color: rgba(0, 0, 0, 0.25);
            background: #f5f5f5;
            border-color: #d9d9d9;
            text-shadow: none;
            box-shadow: none;
            "
        )
    } else if ghost {
        format!(
            "
            color: var(--{error_color_var});
            background: transparent;
            border-color: var(--{error_color_var});
            text-shadow: none;

            &:hover, &:focus {{
                color: var(--{error_hover_color_var});
                border-color: var(--{error_hover_color_var});
            }}

            &:active {{
                color: var(--{error_active_color_var});
                border-color: var(--{error_active_color_var});
            }}
            "
        )
    } else {
        format!(
            "
            color: #fff;
            background: var(--{error_color_var});
            border-color: var(--{error_color_var});
            text-shadow: 0 -1px 0 rgba(0, 0, 0, 0.12);
            box-shadow: 0 2px 0 rgba(0, 0, 0, 0.045);

            &:hover, &:focus {{
                background: var(--{error_hover_color_var});
                border-color: var(--{error_hover_color_var});
            }}

            &:active {{
                background: var(--{error_active_color_var});
                border-color: var(--{error_active_color_var});
            }}
            "
        )
    }
}

/// 生成危险默认按钮样式
fn generate_danger_default_style(disabled: bool, ghost: bool, is_dark: bool) -> String {
    // 使用组件令牌和CSS变量
    let error_color_var = use_css_var_name("colorError");
    let error_hover_color_var = use_css_var_name("colorErrorHover");
    let error_active_color_var = use_css_var_name("colorErrorActive");

    if disabled {
        format!(
            "
            color: rgba(0, 0, 0, 0.25);
            background: #f5f5f5;
            border-color: #d9d9d9;
            text-shadow: none;
            box-shadow: none;
            "
        )
    } else if ghost {
        let text_color = if is_dark { "#ff4d4f" } else { "#ff4d4f" };

        format!(
            "
            color: var(--{error_color_var});
            background: transparent;
            border-color: var(--{error_color_var});

            &:hover, &:focus {{
                color: var(--{error_hover_color_var});
                border-color: var(--{error_hover_color_var});
            }}

            &:active {{
                color: var(--{error_active_color_var});
                border-color: var(--{error_active_color_var});
            }}
            "
        )
    } else {
        format!(
            "
            color: var(--{error_color_var});
            background: transparent;
            border-color: var(--{error_color_var});

            &:hover, &:focus {{
                color: var(--{error_hover_color_var});
                border-color: var(--{error_hover_color_var});
            }}

            &:active {{
                color: var(--{error_active_color_var});
                border-color: var(--{error_active_color_var});
            }}
            "
        )
    }
}

/// 生成危险虚线按钮样式
fn generate_danger_dashed_style(disabled: bool, ghost: bool, is_dark: bool) -> String {
    let base_style = generate_danger_default_style(disabled, ghost, is_dark);
    format!("{}\nborder-style: dashed;", base_style)
}

/// 生成危险链接按钮样式
fn generate_danger_link_style(disabled: bool, is_dark: bool) -> String {
    // 使用组件令牌和CSS变量
    let error_color_var = use_css_var_name("colorError");
    let error_hover_color_var = use_css_var_name("colorErrorHover");
    let error_active_color_var = use_css_var_name("colorErrorActive");

    if disabled {
        format!(
            "
            color: rgba(0, 0, 0, 0.25);
            background: transparent;
            border-color: transparent;
            text-shadow: none;
            box-shadow: none;
            "
        )
    } else {
        format!(
            "
            color: var(--{error_color_var});
            background: transparent;
            border-color: transparent;

            &:hover, &:focus {{
                color: var(--{error_hover_color_var});
            }}

            &:active {{
                color: var(--{error_active_color_var});
            }}
            "
        )
    }
}

/// 生成危险文本按钮样式
fn generate_danger_text_style(disabled: bool, is_dark: bool) -> String {
    // 使用组件令牌和CSS变量
    let error_color_var = use_css_var_name("colorError");
    let hover_bg = if is_dark {
        "rgba(255, 255, 255, 0.03)"
    } else {
        "rgba(0, 0, 0, 0.06)"
    };
    let active_bg = if is_dark {
        "rgba(255, 255, 255, 0.06)"
    } else {
        "rgba(0, 0, 0, 0.15)"
    };

    if disabled {
        format!(
            "
            color: rgba(0, 0, 0, 0.25);
            background: transparent;
            border-color: transparent;
            text-shadow: none;
            box-shadow: none;
            "
        )
    } else {
        format!(
            "
            color: var(--{error_color_var});
            background: transparent;
            border-color: transparent;

            &:hover, &:focus {{
                background: {hover_bg};
            }}

            &:active {{
                background: {active_bg};
            }}
            "
        )
    }
}

/// 生成主色调样式
fn generate_primary_color_style() -> String {
    let primary_color = use_component_token("button", "colorPrimary");
    let hover_color = use_component_token("button", "colorPrimaryHover");
    let active_color = use_component_token("button", "colorPrimaryActive");
    let text_color = use_component_token("button", "colorTextLightSolid");

    format!(
        "background-color: {}; border-color: {}; color: {};
        &:hover {{ background-color: {}; border-color: {}; }}
        &:active {{ background-color: {}; border-color: {}; }}",
        primary_color,
        primary_color,
        text_color,
        hover_color,
        hover_color,
        active_color,
        active_color
    )
}

/// 生成默认颜色样式
fn generate_default_color_style() -> String {
    let border_color = use_component_token("button", "colorBorder");
    let text_color = use_component_token("button", "colorText");
    let bg_color = use_component_token("button", "colorBgContainer");
    let hover_color = use_component_token("button", "colorPrimaryHover");
    let active_color = use_component_token("button", "colorPrimaryActive");

    format!(
        "background-color: {}; border-color: {}; color: {};
        &:hover {{ border-color: {}; color: {}; }}
        &:active {{ border-color: {}; color: {}; }}",
        bg_color, border_color, text_color, hover_color, hover_color, active_color, active_color
    )
}

/// 生成危险颜色样式
fn generate_danger_color_style() -> String {
    let danger_color = use_component_token("button", "colorError");
    let hover_color = use_component_token("button", "colorErrorHover");
    let active_color = use_component_token("button", "colorErrorActive");
    let text_color = use_component_token("button", "colorTextLightSolid");

    format!(
        "background-color: {}; border-color: {}; color: {};
        &:hover {{ background-color: {}; border-color: {}; }}
        &:active {{ background-color: {}; border-color: {}; }}",
        danger_color,
        danger_color,
        text_color,
        hover_color,
        hover_color,
        active_color,
        active_color
    )
}

/// 生成轮廓变体样式
fn generate_outlined_variant_style() -> String {
    let primary_color = use_component_token("button", "colorPrimary");
    let hover_color = use_component_token("button", "colorPrimaryHover");
    let active_color = use_component_token("button", "colorPrimaryActive");
    let bg_color = use_component_token("button", "colorBgContainer");

    format!(
        "background-color: {}; border-color: {}; color: {};
        &:hover {{ border-color: {}; color: {}; }}
        &:active {{ border-color: {}; color: {}; }}",
        bg_color,
        primary_color,
        primary_color,
        hover_color,
        hover_color,
        active_color,
        active_color
    )
}

/// 生成实心变体样式
fn generate_solid_variant_style() -> String {
    let primary_color = use_component_token("button", "colorPrimary");
    let hover_color = use_component_token("button", "colorPrimaryHover");
    let active_color = use_component_token("button", "colorPrimaryActive");
    let text_color = use_component_token("button", "colorTextLightSolid");

    format!(
        "background-color: {}; border-color: {}; color: {};
        &:hover {{ background-color: {}; border-color: {}; }}
        &:active {{ background-color: {}; border-color: {}; }}",
        primary_color,
        primary_color,
        text_color,
        hover_color,
        hover_color,
        active_color,
        active_color
    )
}

/// 生成虚线变体样式
fn generate_dashed_variant_style() -> String {
    let primary_color = use_component_token("button", "colorPrimary");
    let hover_color = use_component_token("button", "colorPrimaryHover");
    let active_color = use_component_token("button", "colorPrimaryActive");
    let bg_color = use_component_token("button", "colorBgContainer");

    format!(
        "background-color: {}; border-color: {}; border-style: dashed; color: {};
        &:hover {{ border-color: {}; color: {}; }}
        &:active {{ border-color: {}; color: {}; }}",
        bg_color,
        primary_color,
        primary_color,
        hover_color,
        hover_color,
        active_color,
        active_color
    )
}

/// 生成文本变体样式
fn generate_text_variant_style() -> String {
    let text_color = use_component_token("button", "colorText");
    let hover_bg = use_component_token("button", "colorBgTextHover");
    let active_bg = use_component_token("button", "colorBgTextActive");

    format!(
        "background-color: transparent; border-color: transparent; color: {};
        &:hover {{ background-color: {}; color: {}; }}
        &:active {{ background-color: {}; color: {}; }}",
        text_color, hover_bg, text_color, active_bg, text_color
    )
}

/// 生成链接变体样式
fn generate_link_variant_style() -> String {
    let primary_color = use_component_token("button", "colorPrimary");
    let hover_color = use_component_token("button", "colorPrimaryHover");
    let active_color = use_component_token("button", "colorPrimaryActive");

    format!(
        "background-color: transparent; border-color: transparent; color: {};
        &:hover {{ color: {}; }}
        &:active {{ color: {}; }}",
        primary_color, hover_color, active_color
    )
}

// 辅助函数：使颜色变亮
fn lighten_color(color: &str, percent: i32) -> String {
    // 简化实现，实际应该解析颜色并调整亮度
    color.to_string()
}

// 辅助函数：使颜色变暗
fn darken_color(color: &str, percent: i32) -> String {
    // 简化实现，实际应该解析颜色并调整亮度
    color.to_string()
}

// 辅助函数：获取对比文本颜色
fn get_contrast_text_color(background: &str) -> String {
    // 简化实现，实际应该计算颜色亮度并返回黑色或白色
    "#ffffff".to_string()
}

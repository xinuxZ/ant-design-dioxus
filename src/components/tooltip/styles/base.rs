//! Tooltip 组件样式实现

use crate::theme::{use_theme, Theme};
use crate::components::tooltip::types::TooltipPlacement;
use css_in_rust::Style;

/// 生成 Tooltip 基础样式
pub fn tooltip_base_style(theme: &Theme) -> Style {
    Style::create(
        "ant-tooltip",
        format!(
            r#"
            .ant-tooltip {{
                position: absolute;
                z-index: 1070;
                display: block;
                visibility: visible;
                font-size: {}px;
                font-style: normal;
                font-weight: normal;
                line-height: {};
                text-decoration: none;
                text-shadow: none;
                text-transform: none;
                letter-spacing: normal;
                word-break: normal;
                word-spacing: normal;
                word-wrap: normal;
                white-space: normal;
                line-break: auto;
                font-size-adjust: none;
                font-stretch: normal;
                opacity: 0;
                pointer-events: none;
                transition: opacity {}ms ease-in-out;
            }}
            
            .ant-tooltip.ant-tooltip-open {{
                opacity: 1;
                pointer-events: auto;
            }}
            
            .ant-tooltip-content {{
                position: relative;
            }}
            
            .ant-tooltip-inner {{
                min-width: 30px;
                min-height: 32px;
                padding: {}px {}px;
                color: {};
                background-color: {};
                border-radius: {}px;
                box-shadow: {};
                word-wrap: break-word;
                text-align: start;
            }}
            
            .ant-tooltip-arrow {{
                position: absolute;
                width: 16px;
                height: 16px;
                overflow: hidden;
            }}
            
            .ant-tooltip-arrow::before {{
                position: absolute;
                width: 8px;
                height: 8px;
                background: {};
                content: '';
                transform: rotate(45deg);
            }}
            "#,
            theme.font_size_sm(),
            theme.line_height_base(),
            theme.motion_duration_mid(),
            theme.padding_xs(),
            theme.padding_sm(),
            theme.color_text_light_solid(),
            theme.color_bg_spotlight(),
            theme.border_radius_sm(),
            theme.box_shadow_secondary(),
            theme.color_bg_spotlight(),
        ),
    )
}

/// 生成 Tooltip 位置样式
pub fn tooltip_placement_style(placement: &TooltipPlacement, theme: &Theme) -> Style {
    let class_name = placement.to_class_name();
    
    match placement {
        TooltipPlacement::Top => Style::create(
            class_name,
            format!(
                r#"
                .{} .ant-tooltip-arrow {{
                    bottom: -4px;
                    left: 50%;
                    transform: translateX(-50%);
                }}
                
                .{} .ant-tooltip-arrow::before {{
                    bottom: 0;
                    left: 0;
                }}
                "#,
                class_name, class_name
            ),
        ),
        TooltipPlacement::TopLeft => Style::create(
            class_name,
            format!(
                r#"
                .{} .ant-tooltip-arrow {{
                    bottom: -4px;
                    left: 16px;
                }}
                
                .{} .ant-tooltip-arrow::before {{
                    bottom: 0;
                    left: 0;
                }}
                "#,
                class_name, class_name
            ),
        ),
        TooltipPlacement::TopRight => Style::create(
            class_name,
            format!(
                r#"
                .{} .ant-tooltip-arrow {{
                    bottom: -4px;
                    right: 16px;
                }}
                
                .{} .ant-tooltip-arrow::before {{
                    bottom: 0;
                    left: 0;
                }}
                "#,
                class_name, class_name
            ),
        ),
        TooltipPlacement::Bottom => Style::create(
            class_name,
            format!(
                r#"
                .{} .ant-tooltip-arrow {{
                    top: -4px;
                    left: 50%;
                    transform: translateX(-50%);
                }}
                
                .{} .ant-tooltip-arrow::before {{
                    top: 0;
                    left: 0;
                }}
                "#,
                class_name, class_name
            ),
        ),
        TooltipPlacement::BottomLeft => Style::create(
            class_name,
            format!(
                r#"
                .{} .ant-tooltip-arrow {{
                    top: -4px;
                    left: 16px;
                }}
                
                .{} .ant-tooltip-arrow::before {{
                    top: 0;
                    left: 0;
                }}
                "#,
                class_name, class_name
            ),
        ),
        TooltipPlacement::BottomRight => Style::create(
            class_name,
            format!(
                r#"
                .{} .ant-tooltip-arrow {{
                    top: -4px;
                    right: 16px;
                }}
                
                .{} .ant-tooltip-arrow::before {{
                    top: 0;
                    left: 0;
                }}
                "#,
                class_name, class_name
            ),
        ),
        TooltipPlacement::Left => Style::create(
            class_name,
            format!(
                r#"
                .{} .ant-tooltip-arrow {{
                    right: -4px;
                    top: 50%;
                    transform: translateY(-50%);
                }}
                
                .{} .ant-tooltip-arrow::before {{
                    top: 0;
                    right: 0;
                }}
                "#,
                class_name, class_name
            ),
        ),
        TooltipPlacement::LeftTop => Style::create(
            class_name,
            format!(
                r#"
                .{} .ant-tooltip-arrow {{
                    right: -4px;
                    top: 8px;
                }}
                
                .{} .ant-tooltip-arrow::before {{
                    top: 0;
                    right: 0;
                }}
                "#,
                class_name, class_name
            ),
        ),
        TooltipPlacement::LeftBottom => Style::create(
            class_name,
            format!(
                r#"
                .{} .ant-tooltip-arrow {{
                    right: -4px;
                    bottom: 8px;
                }}
                
                .{} .ant-tooltip-arrow::before {{
                    top: 0;
                    right: 0;
                }}
                "#,
                class_name, class_name
            ),
        ),
        TooltipPlacement::Right => Style::create(
            class_name,
            format!(
                r#"
                .{} .ant-tooltip-arrow {{
                    left: -4px;
                    top: 50%;
                    transform: translateY(-50%);
                }}
                
                .{} .ant-tooltip-arrow::before {{
                    top: 0;
                    left: 0;
                }}
                "#,
                class_name, class_name
            ),
        ),
        TooltipPlacement::RightTop => Style::create(
            class_name,
            format!(
                r#"
                .{} .ant-tooltip-arrow {{
                    left: -4px;
                    top: 8px;
                }}
                
                .{} .ant-tooltip-arrow::before {{
                    top: 0;
                    left: 0;
                }}
                "#,
                class_name, class_name
            ),
        ),
        TooltipPlacement::RightBottom => Style::create(
            class_name,
            format!(
                r#"
                .{} .ant-tooltip-arrow {{
                    left: -4px;
                    bottom: 8px;
                }}
                
                .{} .ant-tooltip-arrow::before {{
                    top: 0;
                    left: 0;
                }}
                "#,
                class_name, class_name
            ),
        ),
    }
}

/// 生成自定义颜色样式
pub fn tooltip_color_style(color: &str) -> Style {
    Style::create(
        "ant-tooltip-custom-color",
        format!(
            r#"
            .ant-tooltip-custom-color .ant-tooltip-inner {{
                background-color: {};
            }}
            
            .ant-tooltip-custom-color .ant-tooltip-arrow::before {{
                background: {};
            }}
            "#,
            color, color
        ),
    )
}

/// 生成暗色主题样式
pub fn tooltip_dark_style(theme: &Theme) -> Style {
    Style::create(
        "ant-tooltip-dark",
        format!(
            r#"
            .ant-tooltip.ant-tooltip-dark .ant-tooltip-inner {{
                color: {};
                background-color: {};
            }}
            
            .ant-tooltip.ant-tooltip-dark .ant-tooltip-arrow::before {{
                background: {};
            }}
            "#,
            theme.color_text_dark_solid(),
            theme.color_bg_elevated(),
            theme.color_bg_elevated(),
        ),
    )
}

/// 生成动画样式
pub fn tooltip_animation_style(theme: &Theme) -> Style {
    Style::create(
        "ant-tooltip-animation",
        format!(
            r#"
            @keyframes ant-tooltip-zoom-in {{
                0% {{
                    opacity: 0;
                    transform: scale(0.8);
                }}
                100% {{
                    opacity: 1;
                    transform: scale(1);
                }}
            }}
            
            @keyframes ant-tooltip-zoom-out {{
                0% {{
                    opacity: 1;
                    transform: scale(1);
                }}
                100% {{
                    opacity: 0;
                    transform: scale(0.8);
                }}
            }}
            
            .ant-tooltip.ant-tooltip-zoom-enter {{
                animation: ant-tooltip-zoom-in {}ms ease-out;
            }}
            
            .ant-tooltip.ant-tooltip-zoom-leave {{
                animation: ant-tooltip-zoom-out {}ms ease-in;
            }}
            "#,
            theme.motion_duration_mid(),
            theme.motion_duration_mid(),
        ),
    )
}

/// 生成响应式样式
pub fn tooltip_responsive_style(theme: &Theme) -> Style {
    Style::create(
        "ant-tooltip-responsive",
        format!(
            r#"
            @media (max-width: {}px) {{
                .ant-tooltip {{
                    font-size: {}px;
                }}
                
                .ant-tooltip-inner {{
                    padding: {}px {}px;
                    max-width: calc(100vw - 32px);
                }}
            }}
            
            @media (prefers-reduced-motion: reduce) {{
                .ant-tooltip {{
                    transition: none;
                }}
                
                .ant-tooltip.ant-tooltip-zoom-enter,
                .ant-tooltip.ant-tooltip-zoom-leave {{
                    animation: none;
                }}
            }}
            "#,
            theme.screen_sm(),
            theme.font_size_sm() - 1,
            theme.padding_xs() - 1,
            theme.padding_sm() - 1,
        ),
    )
}
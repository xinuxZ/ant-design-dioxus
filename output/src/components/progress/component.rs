//! Progress 组件实现
//!
//! 包含线形、圆形和仪表盘三种类型的进度条组件

use dioxus::prelude::*;
use crate::components::progress::{types::*, styles::*};

/// 无障碍属性结构
#[derive(Debug, Clone)]
struct AccessibilityAttrs {
    role: String,
    aria_label: Option<String>,
    aria_labelledby: Option<String>,
}

/// 获取无障碍属性
fn get_accessibility_attrs(props: &ProgressProps) -> AccessibilityAttrs {
    AccessibilityAttrs {
        role: props.role.clone().unwrap_or_else(|| "progressbar".to_string()),
        aria_label: props.aria_label.clone(),
        aria_labelledby: props.aria_labelledby.clone(),
    }
}

/// 获取进度文本
fn get_progress_text(props: &ProgressProps) -> String {
    if let Some(format_fn) = props.format {
        format_fn(props.percent)
    } else {
        match props.status {
            ProgressStatus::Exception => {
                if let Some(ref locale) = props.locale {
                    locale.exception_text.clone()
                } else {
                    "✕".to_string()
                }
            },
            ProgressStatus::Success => {
                if let Some(ref locale) = props.locale {
                    locale.success_text.clone()
                } else {
                    "✓".to_string()
                }
            },
            _ => {
                if let Some(ref locale) = props.locale {
                    locale.format_percent(props.percent)
                } else {
                    format!("{}%", props.percent)
                }
            },
        }
    }
}

/// 线形进度条组件
#[component]
fn LineProgress(props: ProgressProps) -> Element {
    let stroke_width = props.stroke_width.unwrap_or(match props.size {
        ProgressSize::Small => 6,
        ProgressSize::Default => 8,
    });

    let progress_class = {
        let mut classes = vec!["ant-progress", "ant-progress-line"];

        match props.size {
            ProgressSize::Small => classes.push("ant-progress-small"),
            ProgressSize::Default => {}
        }

        let status_class = props.status.to_class();
        if !status_class.is_empty() {
            classes.push(status_class);
        }

        // 添加响应式类名
        let responsive_class = get_responsive_class(props.responsive);
        if !responsive_class.is_empty() {
            classes.push(responsive_class);
        }

        // 添加动画类名
        let animation_class = get_animation_class(props.no_animation);
        if !animation_class.is_empty() {
            classes.push(animation_class);
        }

        // 添加主题色类名
        let theme_class = get_theme_class(props.theme_color.as_deref());
        if !theme_class.is_empty() {
            classes.push(theme_class);
        }

        // 添加渐变色类名
        if props.gradient.is_some() {
            classes.push("ant-progress-gradient");
        }

        // 添加进度条位置类名
        match props.percent_position {
            PercentPosition::Top => classes.push("ant-progress-percent-top"),
            PercentPosition::Bottom => classes.push("ant-progress-percent-bottom"),
            PercentPosition::Inside => classes.push("ant-progress-percent-inside"),
            PercentPosition::Outside => classes.push("ant-progress-percent-outside"),
        }

        if let Some(ref class) = props.class {
            classes.push(class);
        }

        classes.join(" ")
    };

    let stroke_color = if let Some(ref gradient) = props.gradient {
        gradient.to_css()
    } else {
        props
            .stroke_color
            .as_deref()
            .unwrap_or(props.status.to_color())
            .to_string()
    };
    let trail_color = props.trail_color.as_deref().unwrap_or("#f5f5f5");

    // 获取无障碍属性
    let aria_attrs = get_accessibility_attrs(&props);
    
    rsx! {
        style { {get_progress_styles()} }

        div {
            class: progress_class,
            style: format!(
                "{}{}",
                get_theme_css_vars(props.theme_color.as_deref()),
                props.style.as_deref().unwrap_or("")
            ),
            // 添加无障碍属性
            role: aria_attrs.role,
            "aria-label": aria_attrs.aria_label,
            "aria-labelledby": aria_attrs.aria_labelledby,
            "aria-valuenow": props.percent.to_string(),
            "aria-valuemin": "0",
            "aria-valuemax": "100",

            div {
                class: "ant-progress-outer",

                div {
                    class: "ant-progress-inner",
                    style: format!("height: {}px; background-color: {};", stroke_width, trail_color),

                    if let Some(ref step_colors) = props.step_colors {
                        // 多色彩分段进度条
                        {
                            let mut current_percent = 0;
                            rsx! {
                                for (index, step) in step_colors.iter().enumerate() {
                                    div {
                                        key: index.to_string(),
                                        style: get_color_segment_style(
                                            current_percent,
                                            (current_percent + step.percent).min(props.percent),
                                            &step.color,
                                            stroke_width
                                        ),
                                        {current_percent += step.percent; ""}
                                    }
                                }
                            }
                        }
                    } else if props.show_steps && props.steps.is_some() {
                        // 步骤进度条
                        {
                            let steps = props.steps.unwrap_or(0);
                            let completed_steps = (props.percent as f32 / 100.0 * steps as f32).round() as i32;

                            rsx! {
                                for i in 0..steps {
                                    div {
                                        key: i.to_string(),
                                        class: if i < completed_steps { "ant-progress-step ant-progress-step-completed" } else { "ant-progress-step" },
                                        style: format!(
                                            "width: {}%; height: {}px; background-color: {};",
                                            100.0 / steps as f32,
                                            stroke_width,
                                            if i < completed_steps { &stroke_color } else { trail_color }
                                        )
                                    }
                                }
                            }
                        }
                    } else {
                        // 普通进度条
                        div {
                            class: "ant-progress-bg",
                            style: format!(
                                "width: {}%; height: {}px; background: {}; border-radius: {}px;",
                                props.percent,
                                stroke_width,
                                stroke_color,
                                stroke_width / 2
                            )
                        }
                    }

                    // 成功进度段
                    if let Some(ref success) = props.success {
                        div {
                            class: "ant-progress-success-bg",
                            style: get_success_segment_style(
                                success.percent,
                                success.stroke_color.as_deref().unwrap_or("#52c41a"),
                                stroke_width
                            )
                        }
                    }
                }

                if props.show_info {
                    div {
                        class: "ant-progress-text",
                        style: "margin-left: 8px; display: inline-block; color: rgba(0, 0, 0, 0.45); font-size: 14px; line-height: 1; white-space: nowrap; text-align: left; vertical-align: middle; word-wrap: normal;",

                        {
                            get_progress_text(&props)
                        }
                    }
                }
            }
        }
    }
}

/// 圆形进度条组件
#[component]
fn CircleProgress(props: ProgressProps) -> Element {
    let stroke_width = props.stroke_width.unwrap_or(6);
    let radius = (props.width as f64 - stroke_width as f64) / 2.0;
    let circumference = 2.0 * std::f64::consts::PI * radius;
    let stroke_dashoffset = circumference * (1.0 - props.percent as f64 / 100.0);

    let progress_class = {
        let mut classes = vec!["ant-progress", "ant-progress-circle"];

        let status_class = props.status.to_class();
        if !status_class.is_empty() {
            classes.push(status_class);
        }

        // 添加响应式类名
        let responsive_class = get_responsive_class(props.responsive);
        if !responsive_class.is_empty() {
            classes.push(responsive_class);
        }

        // 添加动画类名
        let animation_class = get_animation_class(props.no_animation);
        if !animation_class.is_empty() {
            classes.push(animation_class);
        }

        // 添加主题色类名
        let theme_class = get_theme_class(props.theme_color.as_deref());
        if !theme_class.is_empty() {
            classes.push(theme_class);
        }

        // 添加渐变色类名
        if props.gradient.is_some() {
            classes.push("ant-progress-gradient");
        }

        if let Some(ref class) = props.class {
            classes.push(class);
        }

        classes.join(" ")
    };

    let stroke_color = if let Some(ref gradient) = props.gradient {
        gradient.to_css()
    } else {
        props
            .stroke_color
            .as_deref()
            .unwrap_or(props.status.to_color())
            .to_string()
    };
    let trail_color = props.trail_color.as_deref().unwrap_or("#f5f5f5");

    // 获取无障碍属性
    let aria_attrs = get_accessibility_attrs(&props);
    
    rsx! {
        style { {get_progress_styles()} }

        div {
            class: progress_class,
            style: format!(
                "width: {}px; height: {}px; {}{}",
                props.width,
                props.width,
                get_theme_css_vars(props.theme_color.as_deref()),
                props.style.as_deref().unwrap_or("")
            ),
            // 添加无障碍属性
            role: aria_attrs.role,
            "aria-label": aria_attrs.aria_label,
            "aria-labelledby": aria_attrs.aria_labelledby,
            "aria-valuenow": props.percent.to_string(),
            "aria-valuemin": "0",
            "aria-valuemax": "100",

            div {
                class: "ant-progress-inner",
                style: "position: relative; width: 100%; height: 100%;",

                svg {
                    width: props.width.to_string(),
                    height: props.width.to_string(),
                    view_box: format!("0 0 {} {}", props.width, props.width),

                    // 渐变定义
                    if let Some(ref gradient) = props.gradient {
                        defs {
                            "linearGradient" {
                                id: "circle-gradient",
                                x1: "0%",
                                y1: "0%",
                                x2: "100%",
                                y2: "0%",
                                stop {
                                    offset: "0%",
                                    "stop-color": gradient.from.clone()
                                }
                                stop {
                                    offset: "100%",
                                    "stop-color": gradient.to.clone()
                                }
                            }
                        }
                    }

                    // 背景圆环
                    circle {
                        cx: (props.width / 2).to_string(),
                        cy: (props.width / 2).to_string(),
                        r: radius.to_string(),
                        fill: "none",
                        stroke: trail_color,
                        "stroke-width": stroke_width.to_string()
                    }

                    // 进度圆环
                    circle {
                        cx: (props.width / 2).to_string(),
                        cy: (props.width / 2).to_string(),
                        r: radius.to_string(),
                        fill: "none",
                        stroke: if props.gradient.is_some() { "url(#circle-gradient)" } else { &stroke_color },
                        "stroke-width": stroke_width.to_string(),
                        "stroke-linecap": props.stroke_linecap.to_string(),
                        "stroke-dasharray": circumference.to_string(),
                        "stroke-dashoffset": stroke_dashoffset.to_string(),
                        transform: format!("rotate(-90 {} {})", props.width / 2, props.width / 2),
                        style: "transition: stroke-dashoffset 0.3s ease;"
                    }
                }

                if props.show_info {
                    div {
                        class: "ant-progress-text",
                        style: "position: absolute; top: 50%; left: 50%; transform: translate(-50%, -50%); text-align: center;",

                        {
                            get_progress_text(&props)
                        }
                    }
                }
            }
        }
    }
}

/// 仪表盘进度条组件
#[component]
fn DashboardProgress(props: ProgressProps) -> Element {
    let stroke_width = props.stroke_width.unwrap_or(6);
    let radius = (props.width as f64 - stroke_width as f64) / 2.0;
    let gap_degree = props.gap_degree;
    let gap_position = props.gap_position;
    
    // 计算弧长
    let arc_angle = 360.0 - gap_degree;
    let arc_length = (arc_angle / 360.0) * 2.0 * std::f64::consts::PI * radius;
    let stroke_dasharray = format!("{} {}", arc_length, 2.0 * std::f64::consts::PI * radius);
    let stroke_dashoffset = arc_length * (1.0 - props.percent as f64 / 100.0);

    let progress_class = {
        let mut classes = vec![
            "ant-progress",
            "ant-progress-circle",
            "ant-progress-dashboard",
        ];

        let status_class = props.status.to_class();
        if !status_class.is_empty() {
            classes.push(status_class);
        }

        // 添加响应式类名
        let responsive_class = get_responsive_class(props.responsive);
        if !responsive_class.is_empty() {
            classes.push(responsive_class);
        }

        // 添加动画类名
        let animation_class = get_animation_class(props.no_animation);
        if !animation_class.is_empty() {
            classes.push(animation_class);
        }

        // 添加主题色类名
        let theme_class = get_theme_class(props.theme_color.as_deref());
        if !theme_class.is_empty() {
            classes.push(theme_class);
        }

        // 添加渐变色类名
        if props.gradient.is_some() {
            classes.push("ant-progress-gradient");
        }

        if let Some(ref class) = props.class {
            classes.push(class);
        }

        classes.join(" ")
    };

    let stroke_color = if let Some(ref gradient) = props.gradient {
        gradient.to_css()
    } else {
        props
            .stroke_color
            .as_deref()
            .unwrap_or(props.status.to_color())
            .to_string()
    };
    let trail_color = props.trail_color.as_deref().unwrap_or("#f5f5f5");

    rsx! {
        style { {get_progress_styles()} }

        div {
            class: progress_class,
            style: format!(
                "width: {}px; height: {}px; {}{}",
                props.width,
                props.width,
                get_theme_css_vars(props.theme_color.as_deref()),
                props.style.as_deref().unwrap_or("")
            ),

            div {
                class: "ant-progress-inner",
                style: "position: relative; width: 100%; height: 100%;",

                svg {
                    width: props.width.to_string(),
                    height: props.width.to_string(),
                    view_box: format!("0 0 {} {}", props.width, props.width),

                    // 渐变定义
                    if let Some(ref gradient) = props.gradient {
                        defs {
                            "linearGradient" {
                                id: "dashboard-gradient",
                                x1: "0%",
                                y1: "0%",
                                x2: "100%",
                                y2: "0%",
                                stop {
                                    offset: "0%",
                                    "stop-color": gradient.from.clone()
                                }
                                stop {
                                    offset: "100%",
                                    "stop-color": gradient.to.clone()
                                }
                            }
                        }
                    }

                    // 背景弧线
                    circle {
                        cx: (props.width / 2).to_string(),
                        cy: (props.width / 2).to_string(),
                        r: radius.to_string(),
                        fill: "none",
                        stroke: trail_color,
                        "stroke-width": stroke_width.to_string(),
                        "stroke-linecap": "round",
                        "stroke-dasharray": stroke_dasharray.clone(),
                        transform: format!("rotate({} {} {})", gap_position - 90.0, props.width / 2, props.width / 2)
                    }

                    // 进度弧线
                    circle {
                        cx: (props.width / 2).to_string(),
                        cy: (props.width / 2).to_string(),
                        r: radius.to_string(),
                        fill: "none",
                        stroke: if props.gradient.is_some() { "url(#dashboard-gradient)" } else { &stroke_color },
                        "stroke-width": stroke_width.to_string(),
                        "stroke-linecap": "round",
                        "stroke-dasharray": stroke_dasharray,
                        "stroke-dashoffset": stroke_dashoffset.to_string(),
                        transform: format!("rotate({} {} {})", gap_position - 90.0, props.width / 2, props.width / 2),
                        style: "transition: stroke-dashoffset 0.3s ease;"
                    }
                }

                if props.show_info {
                    div {
                        class: "ant-progress-text",
                        style: "position: absolute; top: 50%; left: 50%; transform: translate(-50%, -50%); text-align: center;",

                        {
                            get_progress_text(&props)
                        }
                    }
                }
            }
        }
    }
}

/// Progress 进度条组件
///
/// 展示操作的当前进度
#[component]
pub fn Progress(props: ProgressProps) -> Element {
    match props.progress_type {
        ProgressType::Line => rsx! { LineProgress { ..props } },
        ProgressType::Circle => rsx! { CircleProgress { ..props } },
        ProgressType::Dashboard => rsx! { DashboardProgress { ..props } },
    }
}
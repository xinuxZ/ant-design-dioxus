//! Progress 进度条组件
//!
//! 展示操作的当前进度。
//!
//! ## 何时使用
//!
//! 在操作需要较长时间才能完成时，为用户显示该操作的当前进度和状态。
//!
//! - 当一个操作会打断当前界面，或者需要在后台运行，且耗时可能超过2秒时；
//! - 当需要显示一个操作完成的百分比时。
//!
//! ## 代码演示
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::{Progress, ProgressType};
//!
//! #[component]
//! fn App() -> Element {
//!     rsx! {
//!         Progress {
//!             percent: 30,
//!             progress_type: ProgressType::Line
//!         }
//!     }
//! }
//! ```

use dioxus::prelude::*;

/// Progress 组件类型
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProgressType {
    /// 线形进度条
    Line,
    /// 环形进度条
    Circle,
    /// 仪表盘
    Dashboard,
}

impl Default for ProgressType {
    fn default() -> Self {
        Self::Line
    }
}

/// Progress 组件尺寸
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProgressSize {
    /// 默认尺寸
    Default,
    /// 小尺寸
    Small,
}

impl Default for ProgressSize {
    fn default() -> Self {
        Self::Default
    }
}

/// Progress 组件状态
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProgressStatus {
    /// 正常状态
    Normal,
    /// 异常状态
    Exception,
    /// 成功状态
    Success,
    /// 活跃状态
    Active,
}

impl Default for ProgressStatus {
    fn default() -> Self {
        Self::Normal
    }
}

impl ProgressStatus {
    /// 获取状态对应的CSS类名
    pub fn to_class(&self) -> &'static str {
        match self {
            ProgressStatus::Normal => "",
            ProgressStatus::Exception => "ant-progress-status-exception",
            ProgressStatus::Success => "ant-progress-status-success",
            ProgressStatus::Active => "ant-progress-status-active",
        }
    }

    /// 获取状态对应的颜色
    pub fn to_color(&self) -> &'static str {
        match self {
            ProgressStatus::Normal => "#1677ff",
            ProgressStatus::Exception => "#ff4d4f",
            ProgressStatus::Success => "#52c41a",
            ProgressStatus::Active => "#1677ff",
        }
    }
}

/// Progress 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct ProgressProps {
    /// 百分比
    #[props(default = 0)]
    pub percent: i32,

    /// 进度条类型
    #[props(default)]
    pub progress_type: ProgressType,

    /// 进度条的状态
    #[props(default)]
    pub status: ProgressStatus,

    /// 进度条的尺寸
    #[props(default)]
    pub size: ProgressSize,

    /// 是否显示进度数值或状态图标
    #[props(default = true)]
    pub show_info: bool,

    /// 进度条线的宽度，单位 px
    #[props(default)]
    pub stroke_width: Option<i32>,

    /// 圆形进度条画布宽度，单位 px
    #[props(default = 120)]
    pub width: i32,

    /// 进度条的色彩
    #[props(default)]
    pub stroke_color: Option<String>,

    /// 未完成的分段的颜色
    #[props(default)]
    pub trail_color: Option<String>,

    /// 内容的模板函数
    #[props(default)]
    pub format: Option<fn(i32) -> String>,

    /// 自定义样式类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,

    /// 步骤数
    #[props(default)]
    pub steps: Option<i32>,

    /// 是否显示为步骤进度条
    #[props(default = false)]
    pub show_steps: bool,
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

        if let Some(ref class) = props.class {
            classes.push(class);
        }

        classes.join(" ")
    };

    let stroke_color = props
        .stroke_color
        .as_deref()
        .unwrap_or(props.status.to_color());
    let trail_color = props.trail_color.as_deref().unwrap_or("#f5f5f5");

    rsx! {
        div {
            class: progress_class,
            style: props.style,

            div {
                class: "ant-progress-outer",

                div {
                    class: "ant-progress-inner",
                    style: format!("height: {}px; background-color: {};", stroke_width, trail_color),

                    if props.show_steps && props.steps.is_some() {
                        // 步骤进度条
                        {
                            let steps = props.steps.unwrap_or(0);
                            let completed_steps = (props.percent as f32 / 100.0 * steps as f32).round() as i32;

                            rsx! {
                                div {
                                    class: "ant-progress-steps-outer",
                                    style: "display: flex; height: 100%;",

                                    for i in 0..steps {
                                        div {
                                            key: i.to_string(),
                                            class: if i < completed_steps { "ant-progress-steps-item-active" } else { "ant-progress-steps-item" },
                                            style: format!(
                                                "flex: 1; margin-right: {}px; background-color: {}; height: 100%;",
                                                if i < steps - 1 { 2 } else { 0 },
                                                if i < completed_steps { stroke_color } else { "transparent" }
                                            )
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        // 普通进度条
                        div {
                            class: "ant-progress-bg",
                            style: format!(
                                "width: {}%; height: 100%; background-color: {}; transition: width 0.3s ease;",
                                props.percent.min(100).max(0),
                                stroke_color
                            )
                        }
                    }
                }
            }

            if props.show_info {
                div {
                    class: "ant-progress-text",

                    {
                        if let Some(format_fn) = props.format {
                            format_fn(props.percent)
                        } else {
                            match props.status {
                                ProgressStatus::Exception => "✕".to_string(),
                                ProgressStatus::Success => "✓".to_string(),
                                _ => format!("{}%", props.percent),
                            }
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
    let radius = (props.width - stroke_width) / 2;
    let circumference = 2.0 * std::f64::consts::PI * radius as f64;
    let stroke_dasharray = circumference;
    let stroke_dashoffset = circumference * (1.0 - props.percent as f64 / 100.0);

    let progress_class = {
        let mut classes = vec!["ant-progress", "ant-progress-circle"];

        let status_class = props.status.to_class();
        if !status_class.is_empty() {
            classes.push(status_class);
        }

        if let Some(ref class) = props.class {
            classes.push(class);
        }

        classes.join(" ")
    };

    let stroke_color = props
        .stroke_color
        .as_deref()
        .unwrap_or(props.status.to_color());
    let trail_color = props.trail_color.as_deref().unwrap_or("#f5f5f5");

    rsx! {
        div {
            class: progress_class,
            style: format!("width: {}px; height: {}px; {}", props.width, props.width, props.style.as_deref().unwrap_or("")),

            div {
                class: "ant-progress-inner",
                style: "position: relative; width: 100%; height: 100%;",

                svg {
                    width: props.width.to_string(),
                    height: props.width.to_string(),
                    view_box: format!("0 0 {} {}", props.width, props.width),

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
                        stroke: stroke_color,
                        "stroke-width": stroke_width.to_string(),
                        "stroke-linecap": "round",
                        "stroke-dasharray": stroke_dasharray.to_string(),
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
                            if let Some(format_fn) = props.format {
                                format_fn(props.percent)
                            } else {
                                 match props.status {
                                     ProgressStatus::Exception => "✕".to_string(),
                                     ProgressStatus::Success => "✓".to_string(),
                                     _ => format!("{}%", props.percent),
                                 }
                             }
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
    let radius = (props.width - stroke_width) / 2;
    let gap_degree = 75.0; // 仪表盘缺口角度
    let gap_position = 270.0; // 缺口位置（底部）

    let circumference = 2.0 * std::f64::consts::PI * radius as f64;
    let arc_length = circumference * (360.0 - gap_degree) / 360.0;
    let stroke_dasharray = format!("{} {}", arc_length, circumference);
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

        if let Some(ref class) = props.class {
            classes.push(class);
        }

        classes.join(" ")
    };

    let stroke_color = props
        .stroke_color
        .as_deref()
        .unwrap_or(props.status.to_color());
    let trail_color = props.trail_color.as_deref().unwrap_or("#f5f5f5");

    rsx! {
        div {
            class: progress_class,
            style: format!("width: {}px; height: {}px; {}", props.width, props.width, props.style.as_deref().unwrap_or("")),

            div {
                class: "ant-progress-inner",
                style: "position: relative; width: 100%; height: 100%;",

                svg {
                    width: props.width.to_string(),
                    height: props.width.to_string(),
                    view_box: format!("0 0 {} {}", props.width, props.width),

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
                        stroke: stroke_color,
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
                            if let Some(format_fn) = props.format {
                                format_fn(props.percent)
                            } else {
                                 match props.status {
                                     ProgressStatus::Exception => "✕".to_string(),
                                     ProgressStatus::Success => "✓".to_string(),
                                     _ => format!("{}%", props.percent),
                                 }
                             }
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

// 组件已通过#[component]宏自动导出
// 无需重新导出

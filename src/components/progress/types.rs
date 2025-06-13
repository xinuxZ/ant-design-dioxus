//! Progress 组件类型定义

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

/// 线条端点样式
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StrokeLinecap {
    /// 圆形端点
    Round,
    /// 方形端点
    Square,
    /// 平直端点
    Butt,
}

impl Default for StrokeLinecap {
    fn default() -> Self {
        Self::Round
    }
}

impl StrokeLinecap {
    pub fn to_str(&self) -> &'static str {
        match self {
            StrokeLinecap::Round => "round",
            StrokeLinecap::Square => "square",
            StrokeLinecap::Butt => "butt",
        }
    }
}

/// 进度条位置
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PercentPosition {
    /// 顶部
    Top,
    /// 底部
    Bottom,
    /// 内部
    Inside,
    /// 外部
    Outside,
}

impl Default for PercentPosition {
    fn default() -> Self {
        Self::Top
    }
}

impl PercentPosition {
    pub fn to_class(&self) -> &'static str {
        match self {
            PercentPosition::Top => "ant-progress-percent-top",
            PercentPosition::Bottom => "ant-progress-percent-bottom",
            PercentPosition::Inside => "ant-progress-percent-inside",
            PercentPosition::Outside => "ant-progress-percent-outside",
        }
    }
}

/// 渐变色配置
#[derive(Debug, Clone, PartialEq)]
pub struct GradientConfig {
    /// 起始颜色
    pub from: String,
    /// 结束颜色
    pub to: String,
    /// 渐变方向
    pub direction: Option<String>,
}

impl GradientConfig {
    pub fn new(from: impl Into<String>, to: impl Into<String>) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
            direction: None,
        }
    }
    
    pub fn with_direction(mut self, direction: impl Into<String>) -> Self {
        self.direction = Some(direction.into());
        self
    }
    
    pub fn to_css(&self) -> String {
        format!(
            "linear-gradient({}, {}, {})",
            self.direction.as_deref().unwrap_or("to right"),
            self.from,
            self.to
        )
    }
}

/// 成功进度段配置
#[derive(Debug, Clone, PartialEq)]
pub struct SuccessConfig {
    /// 成功进度百分比
    pub percent: i32,
    /// 成功段颜色
    pub stroke_color: Option<String>,
}

impl SuccessConfig {
    pub fn new(percent: i32) -> Self {
        Self {
            percent: percent.max(0).min(100),
            stroke_color: None,
        }
    }
    
    pub fn with_color(mut self, color: impl Into<String>) -> Self {
        self.stroke_color = Some(color.into());
        self
    }
}

/// 步骤配置
#[derive(Debug, Clone, PartialEq)]
pub struct StepConfig {
    pub percent: i32,
    pub color: String,
}

impl StepConfig {
    pub fn new(percent: i32, color: impl Into<String>) -> Self {
        Self {
            percent: percent.max(0).min(100),
            color: color.into(),
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

    /// 线条端点样式
    #[props(default)]
    pub stroke_linecap: StrokeLinecap,

    /// 进度条位置（仅线形进度条）
    #[props(default)]
    pub percent_position: PercentPosition,

    /// 渐变色配置
    #[props(default)]
    pub gradient: Option<GradientConfig>,

    /// 成功进度段配置
    #[props(default)]
    pub success: Option<SuccessConfig>,

    /// 多色彩分段配置
    #[props(default)]
    pub step_colors: Option<Vec<StepConfig>>,
    
    /// 仪表盘缺口角度（度）
    #[props(default = 75.0)]
    pub gap_degree: f64,
    
    /// 仪表盘缺口位置（度）
    #[props(default = 270.0)]
    pub gap_position: f64,
    
    /// 响应式支持
    #[props(default = false)]
    pub responsive: bool,
    
    /// 禁用动画
    #[props(default = false)]
    pub no_animation: bool,
    
    /// 主题色
    #[props(default)]
    pub theme_color: Option<String>,
    
    // 低优先级功能
    /// 国际化配置
    #[props(default)]
    pub locale: Option<ProgressLocale>,
    
    /// 无障碍标签
    #[props(default)]
    pub aria_label: Option<String>,
    
    /// 无障碍标签引用
    #[props(default)]
    pub aria_labelledby: Option<String>,
    
    /// 角色属性
    #[props(default)]
    pub role: Option<String>,
}

/// 国际化配置
#[derive(Debug, Clone, PartialEq)]
pub struct ProgressLocale {
    /// 进度条文本格式
    pub format: Option<String>,
    /// 成功状态文本
    pub success_text: Option<String>,
    /// 异常状态文本
    pub exception_text: Option<String>,
    /// 活跃状态文本
    pub active_text: Option<String>,
}

impl Default for ProgressLocale {
    fn default() -> Self {
        Self {
            format: Some("{percent}%".to_string()),
            success_text: Some("Done".to_string()),
            exception_text: Some("Error".to_string()),
            active_text: Some("Active".to_string()),
        }
    }
}

impl ProgressLocale {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn with_format(mut self, format: impl Into<String>) -> Self {
        self.format = Some(format.into());
        self
    }
    
    pub fn with_success_text(mut self, text: impl Into<String>) -> Self {
        self.success_text = Some(text.into());
        self
    }
    
    pub fn with_exception_text(mut self, text: impl Into<String>) -> Self {
        self.exception_text = Some(text.into());
        self
    }
    
    pub fn with_active_text(mut self, text: impl Into<String>) -> Self {
        self.active_text = Some(text.into());
        self
    }
    
    pub fn get_status_text(&self, status: ProgressStatus) -> Option<&String> {
        match status {
            ProgressStatus::Success => self.success_text.as_ref(),
            ProgressStatus::Exception => self.exception_text.as_ref(),
            ProgressStatus::Active => self.active_text.as_ref(),
            ProgressStatus::Normal => None,
        }
    }
}
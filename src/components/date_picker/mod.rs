//! DatePicker 日期选择器组件
//!
//! 输入或选择日期的控件。
//!
//! ## 何时使用
//!
//! 当用户需要输入一个日期，可以点击标准输入框，弹出日期面板进行选择。
//!
//! ## 示例
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::DatePicker;
//!
//! fn app() -> Element {
//!     rsx! {
//!         DatePicker {
//!             placeholder: "请选择日期",
//!             on_change: |date| {
//!                 println!("选择的日期: {:?}", date);
//!             }
//!         }
//!     }
//! }
//! ```

use dioxus::prelude::*;
use std::fmt;

/// 日期选择器尺寸
#[derive(Debug, Clone, PartialEq)]
pub enum DatePickerSize {
    Small,
    Middle,
    Large,
}

impl Default for DatePickerSize {
    fn default() -> Self {
        Self::Middle
    }
}

impl fmt::Display for DatePickerSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Small => write!(f, "small"),
            Self::Middle => write!(f, "middle"),
            Self::Large => write!(f, "large"),
        }
    }
}

/// 日期选择器状态
#[derive(Debug, Clone, PartialEq)]
pub enum DatePickerStatus {
    Default,
    Error,
    Warning,
}

impl Default for DatePickerStatus {
    fn default() -> Self {
        Self::Default
    }
}

impl fmt::Display for DatePickerStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Default => write!(f, "default"),
            Self::Error => write!(f, "error"),
            Self::Warning => write!(f, "warning"),
        }
    }
}

/// 日期选择器模式
#[derive(Debug, Clone, PartialEq)]
pub enum DatePickerMode {
    Date,
    Week,
    Month,
    Quarter,
    Year,
    Time,
    DateTime,
}

impl Default for DatePickerMode {
    fn default() -> Self {
        Self::Date
    }
}

impl fmt::Display for DatePickerMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Date => write!(f, "date"),
            Self::Week => write!(f, "week"),
            Self::Month => write!(f, "month"),
            Self::Quarter => write!(f, "quarter"),
            Self::Year => write!(f, "year"),
            Self::Time => write!(f, "time"),
            Self::DateTime => write!(f, "datetime"),
        }
    }
}

/// 日期选择器属性
#[derive(Props, Clone, PartialEq)]
pub struct DatePickerProps {
    /// 自定义类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,

    /// 输入框提示文字
    #[props(default)]
    pub placeholder: Option<String>,

    /// 组件尺寸
    #[props(default)]
    pub size: DatePickerSize,

    /// 组件状态
    #[props(default)]
    pub status: DatePickerStatus,

    /// 选择器模式
    #[props(default)]
    pub mode: DatePickerMode,

    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,

    /// 是否允许清除
    #[props(default = true)]
    pub allow_clear: bool,

    /// 是否自动获取焦点
    #[props(default = false)]
    pub auto_focus: bool,

    /// 是否显示边框
    #[props(default = true)]
    pub bordered: bool,

    /// 日期格式
    #[props(default)]
    pub format: Option<String>,

    /// 默认值
    #[props(default)]
    pub default_value: Option<String>,

    /// 当前值
    #[props(default)]
    pub value: Option<String>,

    /// 不可选择的日期
    #[props(default)]
    pub disabled_date: Option<EventHandler<String>>,

    /// 值改变时的回调
    #[props(default)]
    pub on_change: Option<EventHandler<Option<String>>>,

    /// 确定时的回调
    #[props(default)]
    pub on_ok: Option<EventHandler<String>>,

    /// 面板打开/关闭时的回调
    #[props(default)]
    pub on_open_change: Option<EventHandler<bool>>,

    /// 输入框获得焦点时的回调
    #[props(default)]
    pub on_focus: Option<EventHandler<FocusEvent>>,

    /// 输入框失去焦点时的回调
    #[props(default)]
    pub on_blur: Option<EventHandler<FocusEvent>>,
}

/// DatePicker 组件
#[component]
pub fn DatePicker(props: DatePickerProps) -> Element {
    let mut open = use_signal(|| false);
    let mut current_value =
        use_signal(|| props.value.clone().or_else(|| props.default_value.clone()));
    let mut input_value = use_signal(|| {
        current_value
            .read()
            .clone()
            .unwrap_or_else(|| String::new())
    });

    let class_name = format!(
        "ant-picker {} {} {} {} {} {}",
        if props.disabled {
            "ant-picker-disabled"
        } else {
            ""
        },
        if !props.bordered {
            "ant-picker-borderless"
        } else {
            ""
        },
        if open() { "ant-picker-focused" } else { "" },
        format!("ant-picker-{}", props.size),
        if props.status != DatePickerStatus::Default {
            format!("ant-picker-status-{}", props.status)
        } else {
            String::new()
        },
        props.class.as_deref().unwrap_or("")
    )
    .trim()
    .to_string();

    let placeholder_text = props.placeholder.as_deref().unwrap_or(match props.mode {
        DatePickerMode::Date => "请选择日期",
        DatePickerMode::Week => "请选择周",
        DatePickerMode::Month => "请选择月份",
        DatePickerMode::Quarter => "请选择季度",
        DatePickerMode::Year => "请选择年份",
        DatePickerMode::Time => "请选择时间",
        DatePickerMode::DateTime => "请选择日期时间",
    });

    let handle_input_click = move |_| {
        if !props.disabled {
            open.set(!open());
            if let Some(handler) = &props.on_open_change {
                handler.call(open());
            }
        }
    };

    let handle_clear = move |e: MouseEvent| {
        e.stop_propagation();
        if !props.disabled && props.allow_clear {
            current_value.set(None);
            input_value.set(String::new());
            if let Some(handler) = &props.on_change {
                handler.call(None);
            }
        }
    };

    let handle_date_select = move |date: String| {
        current_value.set(Some(date.clone()));
        input_value.set(date.clone());
        open.set(false);

        if let Some(handler) = &props.on_change {
            handler.call(Some(date.clone()));
        }
        if let Some(handler) = &props.on_ok {
            handler.call(date);
        }
        if let Some(handler) = &props.on_open_change {
            handler.call(false);
        }
    };

    let handle_focus = move |e: FocusEvent| {
        if let Some(handler) = &props.on_focus {
            handler.call(e);
        }
    };

    let handle_blur = move |e: FocusEvent| {
        if let Some(handler) = &props.on_blur {
            handler.call(e);
        }
    };

    rsx! {
        div {
            class: class_name,
            style: props.style.as_deref().unwrap_or(""),

            div {
                class: "ant-picker-input",
                onclick: handle_input_click,

                input {
                    class: "ant-picker-input-inner",
                    r#type: "text",
                    placeholder: placeholder_text,
                    value: input_value(),
                    disabled: props.disabled,
                    readonly: true,
                    autofocus: props.auto_focus,
                    onfocus: handle_focus,
                    onblur: handle_blur,
                }

                if props.allow_clear && !input_value().is_empty() && !props.disabled {
                    span {
                        class: "ant-picker-clear",
                        onclick: handle_clear,
                        "✕"
                    }
                }

                span {
                    class: "ant-picker-suffix",
                    match props.mode {
                        DatePickerMode::Time => "🕐",
                        _ => "📅",
                    }
                }
            }

            if open() {
                DatePickerPanel {
                    mode: props.mode.clone(),
                    value: current_value().clone(),
                    disabled_date: props.disabled_date.clone(),
                    on_select: handle_date_select,
                    on_close: move |_| {
                        open.set(false);
                        if let Some(handler) = &props.on_open_change {
                            handler.call(false);
                        }
                    }
                }
            }
        }
    }
}

/// 日期选择面板属性
#[derive(Props, Clone, PartialEq)]
struct DatePickerPanelProps {
    mode: DatePickerMode,
    value: Option<String>,
    disabled_date: Option<EventHandler<String>>,
    on_select: EventHandler<String>,
    on_close: EventHandler<()>,
}

/// 日期选择面板组件
#[component]
fn DatePickerPanel(props: DatePickerPanelProps) -> Element {
    let mut current_year = use_signal(|| 2024);
    let mut current_month = use_signal(|| 1);
    let view_mode = use_signal(|| props.mode.clone());

    let handle_prev = move |_| match view_mode() {
        DatePickerMode::Date => {
            if current_month() == 1 {
                current_month.set(12);
                current_year.set(current_year() - 1);
            } else {
                current_month.set(current_month() - 1);
            }
        }
        DatePickerMode::Month | DatePickerMode::Quarter => {
            current_year.set(current_year() - 1);
        }
        DatePickerMode::Year => {
            current_year.set(current_year() - 10);
        }
        _ => {}
    };

    let handle_next = move |_| match view_mode() {
        DatePickerMode::Date => {
            if current_month() == 12 {
                current_month.set(1);
                current_year.set(current_year() + 1);
            } else {
                current_month.set(current_month() + 1);
            }
        }
        DatePickerMode::Month | DatePickerMode::Quarter => {
            current_year.set(current_year() + 1);
        }
        DatePickerMode::Year => {
            current_year.set(current_year() + 10);
        }
        _ => {}
    };

    let handle_date_click = move |date: String| {
        props.on_select.call(date);
    };

    rsx! {
        div {
            class: "ant-picker-dropdown",

            div {
                class: "ant-picker-panel",

                div {
                    class: "ant-picker-header",

                    button {
                        class: "ant-picker-header-prev-btn",
                        onclick: handle_prev,
                        "‹"
                    }

                    div {
                        class: "ant-picker-header-view",
                        match view_mode() {
                            DatePickerMode::Date => format!("{} 年 {} 月", current_year(), current_month()),
                            DatePickerMode::Month | DatePickerMode::Quarter => format!("{} 年", current_year()),
                            DatePickerMode::Year => format!("{}-{}", current_year(), current_year() + 9),
                            _ => format!("{} 年 {} 月", current_year(), current_month()),
                        }
                    }

                    button {
                        class: "ant-picker-header-next-btn",
                        onclick: handle_next,
                        "›"
                    }
                }

                div {
                    class: "ant-picker-body",

                    match view_mode() {
                        DatePickerMode::Date => rsx! {
                            DateGrid {
                                year: current_year(),
                                month: current_month(),
                                value: props.value.clone(),
                                disabled_date: props.disabled_date.clone(),
                                on_select: handle_date_click
                            }
                        },
                        DatePickerMode::Month => rsx! {
                            MonthGrid {
                                year: current_year(),
                                value: props.value.clone(),
                                on_select: handle_date_click
                            }
                        },
                        DatePickerMode::Year => rsx! {
                            YearGrid {
                                start_year: current_year(),
                                value: props.value.clone(),
                                on_select: handle_date_click
                            }
                        },
                        _ => rsx! {
                            DateGrid {
                                year: current_year(),
                                month: current_month(),
                                value: props.value.clone(),
                                disabled_date: props.disabled_date.clone(),
                                on_select: handle_date_click
                            }
                        }
                    }
                }
            }
        }
    }
}

/// 日期网格属性
#[derive(Props, Clone, PartialEq)]
struct DateGridProps {
    year: i32,
    month: i32,
    value: Option<String>,
    disabled_date: Option<EventHandler<String>>,
    on_select: EventHandler<String>,
}

/// 日期网格组件
#[component]
fn DateGrid(props: DateGridProps) -> Element {
    let days_in_month = match props.month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if props.year % 4 == 0 && (props.year % 100 != 0 || props.year % 400 == 0) {
                29
            } else {
                28
            }
        }
        _ => 30,
    };

    let handle_day_click = move |day: i32| {
        let date = format!("{}-{:02}-{:02}", props.year, props.month, day);
        props.on_select.call(date);
    };

    rsx! {
        table {
            class: "ant-picker-content",

            thead {
                tr {
                    th { "日" }
                    th { "一" }
                    th { "二" }
                    th { "三" }
                    th { "四" }
                    th { "五" }
                    th { "六" }
                }
            }

            tbody {
                for week in 0..6 {
                    tr {
                        for day_of_week in 0..7 {
                            {
                                let day = week * 7 + day_of_week - 6; // 简化计算
                                if day >= 1 && day <= days_in_month {
                                    let date = format!("{}-{:02}-{:02}", props.year, props.month, day);
                                    let is_selected = props.value.as_ref().map_or(false, |v| v == &date);
                                    let is_disabled = false; // 暂时禁用此功能，需要重新设计
                                    // TODO: 重新实现 disabled_date 功能

                                    rsx! {
                                        td {
                                            class: format!(
                                                "ant-picker-cell {} {}",
                                                if is_selected { "ant-picker-cell-selected" } else { "" },
                                                if is_disabled { "ant-picker-cell-disabled" } else { "" }
                                            ),
                                            onclick: move |_| {
                                                if !is_disabled {
                                                    handle_day_click(day);
                                                }
                                            },

                                            div {
                                                class: "ant-picker-cell-inner",
                                                "{day}"
                                            }
                                        }
                                    }
                                } else {
                                    rsx! {
                                        td {
                                            class: "ant-picker-cell ant-picker-cell-disabled",
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// 月份网格属性
#[derive(Props, Clone, PartialEq)]
struct MonthGridProps {
    year: i32,
    value: Option<String>,
    on_select: EventHandler<String>,
}

/// 月份网格组件
#[component]
fn MonthGrid(props: MonthGridProps) -> Element {
    let months = [
        "1月", "2月", "3月", "4月", "5月", "6月", "7月", "8月", "9月", "10月", "11月", "12月",
    ];

    let handle_month_click = move |month: i32| {
        let date = format!("{}-{:02}", props.year, month);
        props.on_select.call(date);
    };

    rsx! {
        div {
            class: "ant-picker-month-panel",

            for (index, month_name) in months.iter().enumerate() {
                {
                    let month = index + 1;
                    let date = format!("{}-{:02}", props.year, month);
                    let is_selected = props.value.as_ref().map_or(false, |v| v.starts_with(&date));

                    rsx! {
                        div {
                            key: "{month}",
                            class: format!(
                                "ant-picker-cell {}",
                                if is_selected { "ant-picker-cell-selected" } else { "" }
                            ),
                            onclick: move |_| handle_month_click(month as i32),

                            div {
                                class: "ant-picker-cell-inner",
                                "{month_name}"
                            }
                        }
                    }
                }
            }
        }
    }
}

/// 年份网格属性
#[derive(Props, Clone, PartialEq)]
struct YearGridProps {
    start_year: i32,
    value: Option<String>,
    on_select: EventHandler<String>,
}

/// 年份网格组件
#[component]
fn YearGrid(props: YearGridProps) -> Element {
    let handle_year_click = move |year: i32| {
        let date = format!("{}", year);
        props.on_select.call(date);
    };

    rsx! {
        div {
            class: "ant-picker-year-panel",

            for year in props.start_year..(props.start_year + 10) {
                {
                    let date = format!("{}", year);
                    let is_selected = props.value.as_ref().map_or(false, |v| v.starts_with(&date));

                    rsx! {
                        div {
                            key: "{year}",
                            class: format!(
                                "ant-picker-cell {}",
                                if is_selected { "ant-picker-cell-selected" } else { "" }
                            ),
                            onclick: move |_| handle_year_click(year),

                            div {
                                class: "ant-picker-cell-inner",
                                "{year}"
                            }
                        }
                    }
                }
            }
        }
    }
}

/// 创建日期选择器的宏
#[macro_export]
macro_rules! date_picker {
    () => {
        DatePicker {}
    };
    ($($prop:ident: $value:expr),* $(,)?) => {
        DatePicker {
            $($prop: $value,)*
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_picker_size() {
        assert_eq!(DatePickerSize::Small.to_string(), "small");
        assert_eq!(DatePickerSize::Middle.to_string(), "middle");
        assert_eq!(DatePickerSize::Large.to_string(), "large");
    }

    #[test]
    fn test_date_picker_status() {
        assert_eq!(DatePickerStatus::Default.to_string(), "default");
        assert_eq!(DatePickerStatus::Error.to_string(), "error");
        assert_eq!(DatePickerStatus::Warning.to_string(), "warning");
    }

    #[test]
    fn test_date_picker_mode() {
        assert_eq!(DatePickerMode::Date.to_string(), "date");
        assert_eq!(DatePickerMode::Week.to_string(), "week");
        assert_eq!(DatePickerMode::Month.to_string(), "month");
        assert_eq!(DatePickerMode::Quarter.to_string(), "quarter");
        assert_eq!(DatePickerMode::Year.to_string(), "year");
        assert_eq!(DatePickerMode::Time.to_string(), "time");
        assert_eq!(DatePickerMode::DateTime.to_string(), "datetime");
    }

    #[test]
    fn test_date_picker_defaults() {
        assert_eq!(DatePickerSize::default(), DatePickerSize::Middle);
        assert_eq!(DatePickerStatus::default(), DatePickerStatus::Default);
        assert_eq!(DatePickerMode::default(), DatePickerMode::Date);
    }
}

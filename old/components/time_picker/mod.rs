//! TimePicker 时间选择框组件
//!
//! 输入或选择时间的控件。
//!
//! ## 何时使用
//!
//! 当用户需要输入一个时间，可以点击标准输入框，弹出时间面板进行选择。
//!
//! ## 示例
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::TimePicker;
//!
//! #[component]
//! fn App() -> Element {
//!     let mut time = use_signal(|| None);
//!
//!     rsx! {
//!         TimePicker {
//!             value: time(),
//!             onchange: move |value| {
//!                 time.set(value);
//!             },
//!         }
//!     }
//! }
//! ```

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

// 引入样式
const STYLE: &str = include_str!("./style.css");

/// TimePicker 组件尺寸
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TimePickerSize {
    /// 大尺寸
    Large,
    /// 默认尺寸
    Middle,
    /// 小尺寸
    Small,
}

impl Default for TimePickerSize {
    fn default() -> Self {
        Self::Middle
    }
}

impl TimePickerSize {
    pub fn to_class(&self) -> &'static str {
        match self {
            TimePickerSize::Large => "ant-picker-large",
            TimePickerSize::Middle => "",
            TimePickerSize::Small => "ant-picker-small",
        }
    }
}

/// 时间格式
#[derive(Debug, Clone, PartialEq)]
pub struct TimeFormat {
    pub format: String,
    pub show_hour: bool,
    pub show_minute: bool,
    pub show_second: bool,
    pub use_12_hours: bool,
}

impl Default for TimeFormat {
    fn default() -> Self {
        Self {
            format: "HH:mm:ss".to_string(),
            show_hour: true,
            show_minute: true,
            show_second: true,
            use_12_hours: false,
        }
    }
}

/// 时间值结构
#[derive(Debug, Clone, PartialEq)]
pub struct TimeValue {
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub is_pm: bool, // 用于12小时制
}

impl TimeValue {
    pub fn new(hour: u8, minute: u8, second: u8) -> Self {
        Self {
            hour,
            minute,
            second,
            is_pm: false,
        }
    }

    pub fn to_string(&self, format: &TimeFormat) -> String {
        if format.use_12_hours {
            let display_hour = if self.hour == 0 {
                12
            } else if self.hour > 12 {
                self.hour - 12
            } else {
                self.hour
            };
            let period = if self.hour >= 12 { "PM" } else { "AM" };
            format!(
                "{:02}:{:02}:{:02} {}",
                display_hour, self.minute, self.second, period
            )
        } else {
            format!("{:02}:{:02}:{:02}", self.hour, self.minute, self.second)
        }
    }

    pub fn from_string(time_str: &str, format: &TimeFormat) -> Option<Self> {
        if format.use_12_hours {
            // 解析12小时制时间
            let parts: Vec<&str> = time_str.split_whitespace().collect();
            if parts.len() != 2 {
                return None;
            }

            let time_part = parts[0];
            let period = parts[1].to_uppercase();
            let is_pm = period == "PM";

            let time_components: Vec<&str> = time_part.split(':').collect();
            if time_components.len() != 3 {
                return None;
            }

            let hour: u8 = time_components[0].parse().ok()?;
            let minute: u8 = time_components[1].parse().ok()?;
            let second: u8 = time_components[2].parse().ok()?;

            let actual_hour = if is_pm && hour != 12 {
                hour + 12
            } else if !is_pm && hour == 12 {
                0
            } else {
                hour
            };

            Some(Self {
                hour: actual_hour,
                minute,
                second,
                is_pm,
            })
        } else {
            // 解析24小时制时间
            let time_components: Vec<&str> = time_str.split(':').collect();
            if time_components.len() != 3 {
                return None;
            }

            let hour: u8 = time_components[0].parse().ok()?;
            let minute: u8 = time_components[1].parse().ok()?;
            let second: u8 = time_components[2].parse().ok()?;

            if hour > 23 || minute > 59 || second > 59 {
                return None;
            }

            Some(Self {
                hour,
                minute,
                second,
                is_pm: hour >= 12,
            })
        }
    }
}

/// TimePicker 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct TimePickerProps {
    /// 当前时间值
    #[props(default = None)]
    pub value: Option<TimeValue>,

    /// 默认时间值
    #[props(default = None)]
    pub default_value: Option<TimeValue>,

    /// 时间格式
    #[props(default = TimeFormat::default())]
    pub format: TimeFormat,

    /// 输入框大小
    #[props(default = TimePickerSize::Middle)]
    pub size: TimePickerSize,

    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,

    /// 占位符
    #[props(default = None)]
    pub placeholder: Option<String>,

    /// 是否允许清除
    #[props(default = true)]
    pub allow_clear: bool,

    /// 自动获取焦点
    #[props(default = false)]
    pub auto_focus: bool,

    /// 小时选项间隔
    #[props(default = 1)]
    pub hour_step: u8,

    /// 分钟选项间隔
    #[props(default = 1)]
    pub minute_step: u8,

    /// 秒选项间隔
    #[props(default = 1)]
    pub second_step: u8,

    /// 时间改变时的回调
    #[props(default = None)]
    pub onchange: Option<EventHandler<Option<TimeValue>>>,

    /// 面板打开/关闭时的回调
    #[props(default = None)]
    pub on_open_change: Option<EventHandler<bool>>,

    /// 自定义类名
    #[props(default = None)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default = None)]
    pub style: Option<String>,

    /// 自定义 id
    #[props(default = None)]
    pub id: Option<String>,
}

/// TimePicker 时间选择框组件
///
/// ## 属性
///
/// - `value`: 当前时间值
/// - `default_value`: 默认时间值
/// - `format`: 时间格式
/// - `size`: 输入框大小
/// - `disabled`: 是否禁用
/// - `placeholder`: 占位符
/// - `allow_clear`: 是否允许清除
/// - `auto_focus`: 自动获取焦点
/// - `hour_step`: 小时选项间隔
/// - `minute_step`: 分钟选项间隔
/// - `second_step`: 秒选项间隔
/// - `onchange`: 时间改变时的回调
/// - `on_open_change`: 面板打开/关闭时的回调
/// - `class`: 自定义类名
/// - `style`: 自定义样式
/// - `id`: 自定义 id
#[component]
pub fn TimePicker(props: TimePickerProps) -> Element {
    // 内部状态管理
    let mut internal_value = use_signal(|| props.default_value.clone());
    let mut is_open = use_signal(|| false);
    let mut input_value = use_signal(|| String::new());

    // 使用受控模式还是非受控模式
    let is_controlled = props.onchange.is_some();
    let current_value = if is_controlled {
        props.value.clone()
    } else {
        internal_value()
    };

    // 初始化输入框显示值
    let format_clone = props.format.clone();
    let current_value_clone = current_value.clone().unwrap();
    use_effect(move || {
        let value = current_value_clone.to_string(&format_clone);
        input_value.set(value);
    });

    // 处理输入框点击事件
    let handle_input_click = move |_: MouseEvent| {
        if !props.disabled {
            is_open.set(!is_open());
            if let Some(on_open_change) = &props.on_open_change {
                on_open_change.call(!is_open());
            }
        }
    };

    // 处理时间选择
    let mut handle_time_select = move |new_value: Option<TimeValue>| {
        if !is_controlled {
            internal_value.set(new_value.clone());
        }

        if let Some(onchange) = &props.onchange {
            onchange.call(new_value);
        }

        is_open.set(false);
        if let Some(on_open_change) = &props.on_open_change {
            on_open_change.call(false);
        }
    };

    // 处理清除按钮
    let handle_clear = move |evt: MouseEvent| {
        evt.stop_propagation();
        handle_time_select(None);
    };

    // 构建类名
    let mut class_names = vec!["ant-picker", "ant-picker-time"];

    if props.disabled {
        class_names.push("ant-picker-disabled");
    }

    if is_open() {
        class_names.push("ant-picker-focused");
    }

    let size_class = props.size.to_class();
    if !size_class.is_empty() {
        class_names.push(size_class);
    }

    if let Some(custom_class) = &props.class {
        class_names.push(custom_class);
    }

    let class_str = class_names.join(" ");

    // 获取占位符
    let placeholder = props.placeholder.as_deref().unwrap_or("请选择时间");

    rsx! {
        style { {STYLE} }
        div {
            class: "ant-picker-wrapper",
            style: props.style.as_deref().unwrap_or(""),

            div {
                class: "{class_str}",
                id: props.id.as_deref().unwrap_or(""),
                onclick: handle_input_click,

                div {
                    class: "ant-picker-input",
                    input {
                        r#type: "text",
                        placeholder: "{placeholder}",
                        value: "{input_value()}",
                        disabled: props.disabled,
                        autofocus: props.auto_focus,
                        readonly: true,
                    }
                }

                div {
                    class: "ant-picker-suffix",
                    if props.allow_clear && current_value.is_some() && !props.disabled {
                        span {
                            class: "ant-picker-clear",
                            onclick: handle_clear,
                            "✕"
                        }
                    }
                    span {
                        class: "ant-picker-clock-icon",
                        "🕐"
                    }
                }
            }

            if is_open() && !props.disabled {
                TimePickerPanel {
                    value: current_value.clone(),
                    format: props.format.clone(),
                    hour_step: props.hour_step,
                    minute_step: props.minute_step,
                    second_step: props.second_step,
                    onselect: handle_time_select,
                }
            }
        }
    }
}

/// 时间选择面板属性
#[derive(Props, Clone, PartialEq)]
struct TimePickerPanelProps {
    value: Option<TimeValue>,
    format: TimeFormat,
    hour_step: u8,
    minute_step: u8,
    second_step: u8,
    onselect: EventHandler<Option<TimeValue>>,
}

/// 时间选择面板组件
#[component]
fn TimePickerPanel(props: TimePickerPanelProps) -> Element {
    let mut selected_hour = use_signal(|| props.value.as_ref().map(|v| v.hour).unwrap_or(0));
    let mut selected_minute = use_signal(|| props.value.as_ref().map(|v| v.minute).unwrap_or(0));
    let mut selected_second = use_signal(|| props.value.as_ref().map(|v| v.second).unwrap_or(0));

    // 生成小时选项
    let hours: Vec<u8> = (0..24).step_by(props.hour_step as usize).collect();
    // 生成分钟选项
    let minutes: Vec<u8> = (0..60).step_by(props.minute_step as usize).collect();
    // 生成秒选项
    let seconds: Vec<u8> = (0..60).step_by(props.second_step as usize).collect();

    // 处理确认选择
    let handle_ok = move |_: MouseEvent| {
        let time_value = TimeValue {
            hour: selected_hour(),
            minute: selected_minute(),
            second: selected_second(),
            is_pm: selected_hour() >= 12,
        };
        props.onselect.call(Some(time_value));
    };

    rsx! {
        div {
            class: "ant-picker-dropdown ant-picker-time-panel",

            div {
                class: "ant-picker-time-panel-content",

                if props.format.show_hour {
                    div {
                        class: "ant-picker-time-panel-column",
                        ul {
                            class: "ant-picker-time-panel-column-list",
                            for hour in hours {
                                li {
                                    class: if hour == selected_hour() { "ant-picker-time-panel-cell ant-picker-time-panel-cell-selected" } else { "ant-picker-time-panel-cell" },
                                    onclick: move |_| selected_hour.set(hour),
                                    div {
                                        class: "ant-picker-time-panel-cell-inner",
                                        "{hour:02}"
                                    }
                                }
                            }
                        }
                    }
                }

                if props.format.show_minute {
                    div {
                        class: "ant-picker-time-panel-column",
                        ul {
                            class: "ant-picker-time-panel-column-list",
                            for minute in minutes {
                                li {
                                    class: if minute == selected_minute() { "ant-picker-time-panel-cell ant-picker-time-panel-cell-selected" } else { "ant-picker-time-panel-cell" },
                                    onclick: move |_| selected_minute.set(minute),
                                    div {
                                        class: "ant-picker-time-panel-cell-inner",
                                        "{minute:02}"
                                    }
                                }
                            }
                        }
                    }
                }

                if props.format.show_second {
                    div {
                        class: "ant-picker-time-panel-column",
                        ul {
                            class: "ant-picker-time-panel-column-list",
                            for second in seconds {
                                li {
                                    class: if second == selected_second() { "ant-picker-time-panel-cell ant-picker-time-panel-cell-selected" } else { "ant-picker-time-panel-cell" },
                                    onclick: move |_| selected_second.set(second),
                                    div {
                                        class: "ant-picker-time-panel-cell-inner",
                                        "{second:02}"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            div {
                class: "ant-picker-footer",
                button {
                    class: "ant-btn ant-btn-primary ant-btn-sm",
                    onclick: handle_ok,
                    "确定"
                }
            }
        }
    }
}

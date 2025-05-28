//! Statistic 统计数值组件
//!
//! 展示统计数值。
//!
//! ## 何时使用
//!
//! - 当需要突出某个或某组数字时。
//! - 当需要展示带描述的统计类数据时使用。

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

// 引入统计数值样式
const STATISTIC_STYLE: &str = include_str!("style.css");

/// 数值精度
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatisticPrecision {
    /// 小数点后位数
    pub decimal_places: usize,
}

impl Default for StatisticPrecision {
    fn default() -> Self {
        Self { decimal_places: 0 }
    }
}

/// Statistic 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct StatisticProps {
    /// 数值内容
    pub value: f64,

    /// 数值的标题
    #[props(default)]
    pub title: Option<String>,

    /// 设置数值的前缀
    #[props(default)]
    pub prefix: Option<Element>,

    /// 设置数值的后缀
    #[props(default)]
    pub suffix: Option<Element>,

    /// 数值精度
    #[props(default)]
    pub precision: StatisticPrecision,

    /// 设置千分位标识符
    #[props(default = ",".to_string())]
    pub group_separator: String,

    /// 设置小数点
    #[props(default = ".".to_string())]
    pub decimal_separator: String,

    /// 自定义数值展示
    #[props(default)]
    pub formatter: Option<fn(f64) -> String>,

    /// 数值样式
    #[props(default)]
    pub value_style: Option<String>,

    /// 自定义 CSS 类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,
}

/// Statistic 组件
///
/// 展示统计数值
///
/// # Props
/// - `value`: 数值内容
/// - `title`: 数值的标题
/// - `prefix`: 设置数值的前缀
/// - `suffix`: 设置数值的后缀
/// - `precision`: 数值精度，默认为 0
/// - `group_separator`: 设置千分位标识符，默认为 ","
/// - `decimal_separator`: 设置小数点，默认为 "."
/// - `formatter`: 自定义数值展示
/// - `value_style`: 数值样式
/// - `class`: 自定义 CSS 类名
/// - `style`: 自定义样式
#[component]
pub fn Statistic(props: StatisticProps) -> Element {
    let mut class_list = vec!["ant-statistic"];

    // 添加自定义类
    if let Some(custom_class) = &props.class {
        class_list.push(custom_class);
    }

    let class_str = class_list.join(" ");

    // 格式化数值
    let formatted_value = if let Some(formatter) = props.formatter {
        formatter(props.value)
    } else {
        format_number(
            props.value,
            props.precision.decimal_places,
            &props.group_separator,
            &props.decimal_separator,
        )
    };

    rsx! {
        style { {STATISTIC_STYLE} }
        div {
            class: "{class_str}",
            style: props.style.unwrap_or_default(),

            // 标题
            if let Some(title) = &props.title {
                div { class: "ant-statistic-title", "{title}" }
            }

            // 数值内容
            div { class: "ant-statistic-content",
                style: props.value_style.unwrap_or_default(),

                // 前缀
                if let Some(prefix) = &props.prefix {
                    span { class: "ant-statistic-content-prefix", {prefix} }
                }

                // 数值
                span { class: "ant-statistic-content-value",
                    "{formatted_value}"
                }

                // 后缀
                if let Some(suffix) = &props.suffix {
                    span { class: "ant-statistic-content-suffix", {suffix} }
                }
            }
        }
    }
}

/// Countdown 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct CountdownProps {
    /// 目标时间（毫秒时间戳）
    pub value: u64,

    /// 数值的标题
    #[props(default)]
    pub title: Option<String>,

    /// 时间格式化显示，参考 moment
    #[props(default = "HH:mm:ss".to_string())]
    pub format: String,

    /// 设置数值的前缀
    #[props(default)]
    pub prefix: Option<Element>,

    /// 设置数值的后缀
    #[props(default)]
    pub suffix: Option<Element>,

    /// 倒计时完成时触发
    #[props(default)]
    pub on_finish: Option<EventHandler<()>>,

    /// 倒计时时间变化时触发
    #[props(default)]
    pub on_change: Option<EventHandler<u64>>,

    /// 数值样式
    #[props(default)]
    pub value_style: Option<String>,

    /// 自定义 CSS 类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,
}

/// Countdown 组件
///
/// 倒计时组件
///
/// # Props
/// - `value`: 目标时间（毫秒时间戳）
/// - `title`: 数值的标题
/// - `format`: 时间格式化显示，默认为 "HH:mm:ss"
/// - `prefix`: 设置数值的前缀
/// - `suffix`: 设置数值的后缀
/// - `on_finish`: 倒计时完成时触发
/// - `on_change`: 倒计时时间变化时触发
/// - `value_style`: 数值样式
/// - `class`: 自定义 CSS 类名
/// - `style`: 自定义样式
#[component]
pub fn Countdown(props: CountdownProps) -> Element {
    let mut remaining_time = use_signal(|| 0u64);
    let mut is_finished = use_signal(|| false);

    // 计算剩余时间
    use_effect(move || {
        let target_time = props.value;
        let now = js_sys::Date::now() as u64;

        if target_time > now {
            let remaining = target_time - now;
            remaining_time.set(remaining);

            if let Some(on_change) = &props.on_change {
                on_change.call(remaining);
            }
        } else {
            remaining_time.set(0);
            if !is_finished.read().clone() {
                is_finished.set(true);
                if let Some(on_finish) = &props.on_finish {
                    on_finish.call(());
                }
            }
        }
    });

    // 定时器更新
    use_effect(move || {
        let interval = gloo_timers::callback::Interval::new(1000, move || {
            let target_time = props.value;
            let now = js_sys::Date::now() as u64;

            if target_time > now {
                let remaining = target_time - now;
                remaining_time.set(remaining);

                if let Some(on_change) = &props.on_change {
                    on_change.call(remaining);
                }
            } else {
                remaining_time.set(0);
                if !is_finished.read().clone() {
                    is_finished.set(true);
                    if let Some(on_finish) = &props.on_finish {
                        on_finish.call(());
                    }
                }
            }
        });

        (move || drop(interval))();
    });

    let mut class_list = vec!["ant-statistic", "ant-statistic-countdown"];

    // 添加自定义类
    if let Some(custom_class) = &props.class {
        class_list.push(custom_class);
    }

    let class_str = class_list.join(" ");

    // 格式化时间显示
    let formatted_time = format_countdown_time(remaining_time.read().clone(), &props.format);

    rsx! {
        style { {STATISTIC_STYLE} }
        div {
            class: "{class_str}",
            style: props.style.unwrap_or_default(),

            // 标题
            if let Some(title) = &props.title {
                div { class: "ant-statistic-title", "{title}" }
            }

            // 数值内容
            div { class: "ant-statistic-content",
                style: props.value_style.unwrap_or_default(),

                // 前缀
                if let Some(prefix) = &props.prefix {
                    span { class: "ant-statistic-content-prefix", {prefix} }
                }

                // 倒计时数值
                span { class: "ant-statistic-content-value",
                    "{formatted_time}"
                }

                // 后缀
                if let Some(suffix) = &props.suffix {
                    span { class: "ant-statistic-content-suffix", {suffix} }
                }
            }
        }
    }
}

/// 格式化数字
///
/// # Arguments
/// - `value`: 要格式化的数值
/// - `precision`: 小数点后位数
/// - `group_separator`: 千分位分隔符
/// - `decimal_separator`: 小数点分隔符
fn format_number(
    value: f64,
    precision: usize,
    group_separator: &str,
    decimal_separator: &str,
) -> String {
    let formatted = format!("{:.precision$}", value, precision = precision);
    let parts: Vec<&str> = formatted.split('.').collect();

    let integer_part = parts[0];
    let decimal_part = if parts.len() > 1 { parts[1] } else { "" };

    // 添加千分位分隔符
    let mut integer_with_separator = String::new();
    let chars: Vec<char> = integer_part.chars().collect();

    for (i, ch) in chars.iter().enumerate() {
        if i > 0 && (chars.len() - i) % 3 == 0 {
            integer_with_separator.push_str(group_separator);
        }
        integer_with_separator.push(*ch);
    }

    if precision > 0 && !decimal_part.is_empty() {
        format!(
            "{}{}{}",
            integer_with_separator, decimal_separator, decimal_part
        )
    } else {
        integer_with_separator
    }
}

/// 格式化倒计时时间
///
/// # Arguments
/// - `milliseconds`: 剩余毫秒数
/// - `format`: 格式化字符串
fn format_countdown_time(milliseconds: u64, format: &str) -> String {
    let total_seconds = milliseconds / 1000;
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    format
        .replace("HH", &format!("{:02}", hours))
        .replace("mm", &format!("{:02}", minutes))
        .replace("ss", &format!("{:02}", seconds))
}

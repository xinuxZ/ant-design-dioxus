//! InputNumber 数字输入框组件
//!
//! 通过鼠标或键盘，输入范围内的数值。
//!
//! ## 何时使用
//!
//! 当需要获取标准数值时。
//!
//! ## 示例
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::InputNumber;
//!
//! fn app() -> Element {
//!     rsx! {
//!         InputNumber {
//!             placeholder: "请输入数字",
//!             min: 0.0,
//!             max: 100.0,
//!             step: 1.0,
//!             on_change: |value| {
//!                 println!("输入的数字: {:?}", value);
//!             }
//!         }
//!     }
//! }
//! ```

use dioxus::prelude::*;
use std::fmt;

/// 数字输入框尺寸
#[derive(Debug, Clone, PartialEq)]
pub enum InputNumberSize {
    Small,
    Middle,
    Large,
}

impl Default for InputNumberSize {
    fn default() -> Self {
        Self::Middle
    }
}

impl fmt::Display for InputNumberSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Small => write!(f, "small"),
            Self::Middle => write!(f, "middle"),
            Self::Large => write!(f, "large"),
        }
    }
}

/// 数字输入框状态
#[derive(Debug, Clone, PartialEq)]
pub enum InputNumberStatus {
    Default,
    Error,
    Warning,
}

impl Default for InputNumberStatus {
    fn default() -> Self {
        Self::Default
    }
}

impl fmt::Display for InputNumberStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Default => write!(f, "default"),
            Self::Error => write!(f, "error"),
            Self::Warning => write!(f, "warning"),
        }
    }
}

/// 数字输入框属性
#[derive(Props, Clone, PartialEq)]
pub struct InputNumberProps {
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
    pub size: InputNumberSize,

    /// 组件状态
    #[props(default)]
    pub status: InputNumberStatus,

    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,

    /// 是否只读
    #[props(default = false)]
    pub readonly: bool,

    /// 是否自动获取焦点
    #[props(default = false)]
    pub auto_focus: bool,

    /// 是否显示边框
    #[props(default = true)]
    pub bordered: bool,

    /// 是否显示增减按钮
    #[props(default = true)]
    pub controls: bool,

    /// 键盘快速操作
    #[props(default = true)]
    pub keyboard: bool,

    /// 最小值
    #[props(default)]
    pub min: Option<f64>,

    /// 最大值
    #[props(default)]
    pub max: Option<f64>,

    /// 每次改变步数
    #[props(default = 1.0)]
    pub step: f64,

    /// 数值精度
    #[props(default)]
    pub precision: Option<usize>,

    /// 默认值
    #[props(default)]
    pub default_value: Option<f64>,

    /// 当前值
    #[props(default)]
    pub value: Option<f64>,

    /// 前缀
    #[props(default)]
    pub prefix: Option<String>,

    /// 后缀
    #[props(default)]
    pub suffix: Option<String>,

    /// 格式化函数
    #[props(default)]
    pub formatter: Option<EventHandler<f64>>,

    /// 解析函数
    #[props(default)]
    pub parser: Option<EventHandler<String>>,

    /// 值改变时的回调
    #[props(default)]
    pub on_change: Option<EventHandler<Option<f64>>>,

    /// 按下回车键时的回调
    #[props(default)]
    pub on_press_enter: Option<EventHandler<KeyboardEvent>>,

    /// 获得焦点时的回调
    #[props(default)]
    pub on_focus: Option<EventHandler<FocusEvent>>,

    /// 失去焦点时的回调
    #[props(default)]
    pub on_blur: Option<EventHandler<FocusEvent>>,

    /// 步进器点击时的回调
    #[props(default)]
    pub on_step: Option<EventHandler<(f64, StepInfo)>>,
}

/// 步进信息
#[derive(Debug, Clone, PartialEq)]
pub struct StepInfo {
    pub offset: f64,
    pub r#type: StepType,
}

/// 步进类型
#[derive(Debug, Clone, PartialEq)]
pub enum StepType {
    Up,
    Down,
}

/// InputNumber 组件
#[component]
pub fn InputNumber(props: InputNumberProps) -> Element {
    let mut current_value = use_signal(|| props.value.or(props.default_value));
    let mut input_value = use_signal(|| {
        if let Some(value) = current_value() {
            format_number(value, props.precision, props.formatter.as_ref())
        } else {
            String::new()
        }
    });
    let mut focused = use_signal(|| false);

    let class_name = format!(
        "ant-input-number {} {} {} {} {} {}",
        if props.disabled {
            "ant-input-number-disabled"
        } else {
            ""
        },
        if !props.bordered {
            "ant-input-number-borderless"
        } else {
            ""
        },
        if focused() {
            "ant-input-number-focused"
        } else {
            ""
        },
        if !props.controls {
            "ant-input-number-handler-wrap-hidden"
        } else {
            ""
        },
        format!("ant-input-number-{}", props.size),
        if props.status != InputNumberStatus::Default {
            format!("ant-input-number-status-{}", props.status)
        } else {
            String::new()
        },
    )
    .trim()
    .to_string();

    let class_name = if let Some(custom_class) = &props.class {
        format!("{} {}", class_name, custom_class)
    } else {
        class_name
    };

    let handle_input_change = move |e: FormEvent| {
        let input_val = e.value();
        input_value.set(input_val.clone());

        if let Ok(parsed_value) = input_val.parse::<f64>() {
            let validated_value = validate_value(parsed_value, props.min, props.max);
            current_value.set(Some(validated_value));

            if let Some(handler) = &props.on_change {
                handler.call(Some(validated_value));
            }
        } else if input_val.is_empty() {
            current_value.set(None);
            if let Some(handler) = &props.on_change {
                handler.call(None);
            }
        }
    };

    let mut handle_step = move |step_type: StepType| {
        let current = current_value().unwrap_or(0.0);
        let offset = match step_type {
            StepType::Up => props.step,
            StepType::Down => -props.step,
        };

        let new_value = current + offset;
        let validated_value = validate_value(new_value, props.min, props.max);

        current_value.set(Some(validated_value));
        input_value.set(format_number(
            validated_value,
            props.precision,
            props.formatter.as_ref(),
        ));

        if let Some(handler) = &props.on_change {
            handler.call(Some(validated_value));
        }

        if let Some(handler) = &props.on_step {
            handler.call((
                validated_value,
                StepInfo {
                    offset,
                    r#type: step_type,
                },
            ));
        }
    };

    let handle_up_click = move |_| {
        if !props.disabled {
            handle_step(StepType::Up);
        }
    };

    let handle_down_click = move |_| {
        if !props.disabled {
            handle_step(StepType::Down);
        }
    };

    let handle_focus = move |e: FocusEvent| {
        focused.set(true);
        if let Some(handler) = &props.on_focus {
            handler.call(e);
        }
    };

    let handle_blur = move |e: FocusEvent| {
        focused.set(false);

        // 格式化输入值
        if let Some(value) = current_value() {
            input_value.set(format_number(
                value,
                props.precision,
                props.formatter.as_ref(),
            ));
        }

        if let Some(handler) = &props.on_blur {
            handler.call(e);
        }
    };

    let handle_key_down = move |e: KeyboardEvent| {
        if !props.keyboard {
            return;
        }

        match e.key() {
            Key::ArrowUp => {
                e.prevent_default();
                if !props.disabled {
                    handle_step(StepType::Up);
                }
            }
            Key::ArrowDown => {
                e.prevent_default();
                if !props.disabled {
                    handle_step(StepType::Down);
                }
            }
            Key::Enter => {
                if let Some(handler) = &props.on_press_enter {
                    handler.call(e);
                }
            }
            _ => {}
        }
    };

    let can_up = current_value().map_or(true, |v| props.max.map_or(true, |max| v < max));

    let can_down = current_value().map_or(true, |v| props.min.map_or(true, |min| v > min));

    rsx! {
        div {
            class: class_name,
            style: props.style.as_deref().unwrap_or(""),

            div {
                class: "ant-input-number-input-wrap",

                if let Some(prefix) = &props.prefix {
                    span {
                        class: "ant-input-number-prefix",
                        {prefix.clone()}
                    }
                }

                input {
                    class: "ant-input-number-input",
                    r#type: "text",
                    placeholder: props.placeholder.as_deref().unwrap_or(""),
                    value: input_value(),
                    disabled: props.disabled,
                    readonly: props.readonly,
                    autofocus: props.auto_focus,
                    oninput: handle_input_change,
                    onfocus: handle_focus,
                    onblur: handle_blur,
                    onkeydown: handle_key_down,
                }

                if let Some(suffix) = &props.suffix {
                    span {
                        class: "ant-input-number-suffix",
                        {suffix.clone()}
                    }
                }
            }

            if props.controls {
                div {
                    class: "ant-input-number-handler-wrap",

                    span {
                        class: format!(
                            "ant-input-number-handler ant-input-number-handler-up {}",
                            if !can_up || props.disabled { "ant-input-number-handler-up-disabled" } else { "" }
                        ),
                        onclick: handle_up_click,

                        span {
                            class: "ant-input-number-handler-up-inner",
                            "⌃"
                        }
                    }

                    span {
                        class: format!(
                            "ant-input-number-handler ant-input-number-handler-down {}",
                            if !can_down || props.disabled { "ant-input-number-handler-down-disabled" } else { "" }
                        ),
                        onclick: handle_down_click,

                        span {
                            class: "ant-input-number-handler-down-inner",
                            "⌄"
                        }
                    }
                }
            }
        }
    }
}

/// 验证数值范围
fn validate_value(value: f64, min: Option<f64>, max: Option<f64>) -> f64 {
    let mut result = value;

    if let Some(min_val) = min {
        if result < min_val {
            result = min_val;
        }
    }

    if let Some(max_val) = max {
        if result > max_val {
            result = max_val;
        }
    }

    result
}

/// 格式化数字
fn format_number(
    value: f64,
    precision: Option<usize>,
    formatter: Option<&EventHandler<f64>>,
) -> String {
    if let Some(_formatter) = formatter {
        // 注意：EventHandler<f64>不能直接返回String，这里需要重新设计
        // 暂时忽略formatter，使用默认格式化
        if let Some(precision) = precision {
            format!("{:.1$}", value, precision)
        } else {
            value.to_string()
        }
    } else if let Some(precision) = precision {
        format!("{:.1$}", value, precision)
    } else {
        value.to_string()
    }
}

/// 创建数字输入框的宏
#[macro_export]
macro_rules! input_number {
    () => {
        InputNumber {}
    };
    ($($prop:ident: $value:expr),* $(,)?) => {
        InputNumber {
            $($prop: $value,)*
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_number_size() {
        assert_eq!(InputNumberSize::Small.to_string(), "small");
        assert_eq!(InputNumberSize::Middle.to_string(), "middle");
        assert_eq!(InputNumberSize::Large.to_string(), "large");
    }

    #[test]
    fn test_input_number_status() {
        assert_eq!(InputNumberStatus::Default.to_string(), "default");
        assert_eq!(InputNumberStatus::Error.to_string(), "error");
        assert_eq!(InputNumberStatus::Warning.to_string(), "warning");
    }

    #[test]
    fn test_validate_value() {
        assert_eq!(validate_value(5.0, Some(0.0), Some(10.0)), 5.0);
        assert_eq!(validate_value(-1.0, Some(0.0), Some(10.0)), 0.0);
        assert_eq!(validate_value(15.0, Some(0.0), Some(10.0)), 10.0);
        assert_eq!(validate_value(5.0, None, None), 5.0);
    }

    #[test]
    fn test_format_number() {
        assert_eq!(format_number(3.14159, Some(2), None), "3.14");
        assert_eq!(format_number(3.0, Some(0), None), "3");
        assert_eq!(format_number(3.14159, None, None), "3.14159");
    }

    #[test]
    fn test_input_number_defaults() {
        assert_eq!(InputNumberSize::default(), InputNumberSize::Middle);
        assert_eq!(InputNumberStatus::default(), InputNumberStatus::Default);
    }
}

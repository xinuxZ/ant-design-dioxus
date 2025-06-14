//! Slider 滑动输入条组件
//!
//! 滑动型输入器，展示当前值和可选范围。
//!
//! ## 何时使用
//!
//! 当用户需要在数值区间/自定义区间内进行选择时，可为连续或离散值。
//!
//! ## 示例
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::Slider;
//!
//! fn app() -> Element {
//!     rsx! {
//!         Slider {
//!             min: 0.0,
//!             max: 100.0,
//!             default_value: 30.0,
//!             on_change: |value| {
//!                 println!("滑块值: {}", value);
//!             }
//!         }
//!     }
//! }
//! ```

use dioxus::prelude::*;
use std::collections::HashMap;
use std::fmt;

const SLIDER_STYLES: &str = include_str!("./style.css");

/// 滑块标记
#[derive(Debug, Clone, PartialEq)]
pub struct SliderMark {
    pub value: f64,
    pub label: String,
    pub style: Option<String>,
}

impl SliderMark {
    pub fn new(value: f64, label: impl Into<String>) -> Self {
        Self {
            value,
            label: label.into(),
            style: None,
        }
    }

    pub fn with_style(mut self, style: impl Into<String>) -> Self {
        self.style = Some(style.into());
        self
    }
}

/// 滑块尺寸
#[derive(Debug, Clone, PartialEq)]
pub enum SliderSize {
    Small,
    Middle,
    Large,
}

impl Default for SliderSize {
    fn default() -> Self {
        Self::Middle
    }
}

impl fmt::Display for SliderSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Small => write!(f, "small"),
            Self::Middle => write!(f, "middle"),
            Self::Large => write!(f, "large"),
        }
    }
}

/// 滑块状态
#[derive(Debug, Clone, PartialEq)]
pub enum SliderStatus {
    Default,
    Error,
    Warning,
}

impl Default for SliderStatus {
    fn default() -> Self {
        Self::Default
    }
}

impl fmt::Display for SliderStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Default => write!(f, "default"),
            Self::Error => write!(f, "error"),
            Self::Warning => write!(f, "warning"),
        }
    }
}

/// 滑块属性
#[derive(Props, Clone, PartialEq)]
pub struct SliderProps {
    /// 自定义类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,

    /// 组件尺寸
    #[props(default)]
    pub size: SliderSize,

    /// 组件状态
    #[props(default)]
    pub status: SliderStatus,

    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,

    /// 是否只读
    #[props(default = false)]
    pub readonly: bool,

    /// 是否自动获取焦点
    #[props(default = false)]
    pub auto_focus: bool,

    /// 是否垂直方向
    #[props(default = false)]
    pub vertical: bool,

    /// 是否反向
    #[props(default = false)]
    pub reverse: bool,

    /// 是否范围选择
    #[props(default = false)]
    pub range: bool,

    /// 最小值
    #[props(default = 0.0)]
    pub min: f64,

    /// 最大值
    #[props(default = 100.0)]
    pub max: f64,

    /// 步长
    #[props(default = 1.0)]
    pub step: f64,

    /// 默认值
    #[props(default)]
    pub default_value: Option<f64>,

    /// 当前值
    #[props(default)]
    pub value: Option<f64>,

    /// 范围默认值
    #[props(default)]
    pub default_range_value: Option<(f64, f64)>,

    /// 范围当前值
    #[props(default)]
    pub range_value: Option<(f64, f64)>,

    /// 是否包含关系
    #[props(default = true)]
    pub included: bool,

    /// 标记
    #[props(default)]
    pub marks: HashMap<String, SliderMark>,

    /// 是否显示点
    #[props(default = false)]
    pub dots: bool,

    /// 工具提示格式化函数
    #[props(default)]
    pub tooltip_formatter: Option<fn(f64) -> String>,

    /// 是否显示工具提示
    #[props(default = true)]
    pub tooltip_visible: bool,

    /// 工具提示放置位置
    #[props(default = "top".to_string())]
    pub tooltip_placement: String,

    /// 值改变时的回调
    #[props(default)]
    pub on_change: Option<EventHandler<f64>>,

    /// 范围值改变时的回调
    #[props(default)]
    pub on_range_change: Option<EventHandler<(f64, f64)>>,

    /// 拖拽结束时的回调
    #[props(default)]
    pub on_after_change: Option<EventHandler<f64>>,

    /// 范围拖拽结束时的回调
    #[props(default)]
    pub on_range_after_change: Option<EventHandler<(f64, f64)>>,

    /// 获得焦点时的回调
    #[props(default)]
    pub on_focus: Option<EventHandler<FocusEvent>>,

    /// 失去焦点时的回调
    #[props(default)]
    pub on_blur: Option<EventHandler<FocusEvent>>,
}

/// Slider 组件
#[component]
pub fn Slider(props: SliderProps) -> Element {
    let mut current_value = use_signal(|| {
        if props.range {
            props
                .range_value
                .or(props.default_range_value)
                .unwrap_or((props.min, props.max))
        } else {
            let single_value = props.value.or(props.default_value).unwrap_or(props.min);
            (single_value, single_value)
        }
    });

    let mut dragging = use_signal(|| false);
    let mut active_handle = use_signal(|| 0); // 0: left/single, 1: right
    let mut focused = use_signal(|| false);
    let mut show_tooltip = use_signal(|| false);

    let class_name = format!(
        "ant-slider {} {} {} {} {} {}",
        if props.disabled {
            "ant-slider-disabled"
        } else {
            ""
        },
        if props.vertical {
            "ant-slider-vertical"
        } else {
            "ant-slider-horizontal"
        },
        if props.range { "ant-slider-range" } else { "" },
        if focused() { "ant-slider-focused" } else { "" },
        format!("ant-slider-{}", props.size),
        if props.status != SliderStatus::Default {
            format!("ant-slider-status-{}", props.status)
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

    let normalize_value = move |value: f64| -> f64 {
        let stepped = ((value - props.min) / props.step).round() * props.step + props.min;
        stepped.max(props.min).min(props.max)
    };

    let get_percentage = |value: f64| -> f64 {
        if props.max == props.min {
            0.0
        } else {
            ((value - props.min) / (props.max - props.min)) * 100.0
        }
    };

    let mut handle_mouse_down = move |handle_index: usize| {
        if props.disabled || props.readonly {
            return;
        }

        dragging.set(true);
        active_handle.set(handle_index);
        show_tooltip.set(true);
    };

    let handle_mouse_up = move |_| {
        if !dragging() {
            return;
        }

        dragging.set(false);
        show_tooltip.set(false);

        let (left_value, right_value) = current_value();

        if props.range {
            if let Some(handler) = &props.on_range_after_change {
                handler.call((left_value, right_value));
            }
        } else {
            if let Some(handler) = &props.on_after_change {
                handler.call(left_value);
            }
        }
    };

    let handle_track_click = move |_e: MouseEvent| {
        if props.disabled || props.readonly {
            return;
        }

        // 这里需要计算点击位置对应的值
        // 实际实现中需要获取轨道的位置和尺寸
        let percentage = 50.0; // 简化实现
        let new_value = props.min + (props.max - props.min) * percentage / 100.0;
        let normalized_value = normalize_value(new_value);

        if props.range {
            let (left_value, right_value) = current_value();
            let new_range =
                if (normalized_value - left_value).abs() < (normalized_value - right_value).abs() {
                    (normalized_value, right_value)
                } else {
                    (left_value, normalized_value)
                };

            current_value.set(new_range);

            if let Some(handler) = &props.on_range_change {
                handler.call(new_range);
            }
        } else {
            current_value.set((normalized_value, normalized_value));

            if let Some(handler) = &props.on_change {
                handler.call(normalized_value);
            }
        }
    };

    let handle_key_down = move |e: KeyboardEvent| {
        if props.disabled || props.readonly {
            return;
        }

        let (left_value, right_value) = current_value();
        let current_handle_value = if active_handle() == 0 {
            left_value
        } else {
            right_value
        };

        let new_value = match e.key() {
            Key::ArrowRight | Key::ArrowUp => {
                e.prevent_default();
                normalize_value(current_handle_value + props.step)
            }
            Key::ArrowLeft | Key::ArrowDown => {
                e.prevent_default();
                normalize_value(current_handle_value - props.step)
            }
            Key::Home => {
                e.prevent_default();
                props.min
            }
            Key::End => {
                e.prevent_default();
                props.max
            }
            Key::PageUp => {
                e.prevent_default();
                normalize_value(current_handle_value + props.step * 10.0)
            }
            Key::PageDown => {
                e.prevent_default();
                normalize_value(current_handle_value - props.step * 10.0)
            }
            _ => return,
        };

        if props.range {
            let new_range = if active_handle() == 0 {
                (new_value.min(right_value), right_value)
            } else {
                (left_value, new_value.max(left_value))
            };

            current_value.set(new_range);

            if let Some(handler) = &props.on_range_change {
                handler.call(new_range);
            }
        } else {
            current_value.set((new_value, new_value));

            if let Some(handler) = &props.on_change {
                handler.call(new_value);
            }
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
        if let Some(handler) = &props.on_blur {
            handler.call(e);
        }
    };

    let (left_value, right_value) = current_value();
    let left_percentage = get_percentage(left_value);
    let right_percentage = get_percentage(right_value);

    let track_style = if props.range {
        if props.vertical {
            format!(
                "bottom: {}%; height: {}%",
                left_percentage,
                right_percentage - left_percentage
            )
        } else {
            format!(
                "left: {}%; width: {}%",
                left_percentage,
                right_percentage - left_percentage
            )
        }
    } else {
        if props.vertical {
            format!("bottom: 0%; height: {}%", left_percentage)
        } else {
            format!("left: 0%; width: {}%", left_percentage)
        }
    };

    let format_tooltip = |value: f64| -> String {
        if let Some(formatter) = props.tooltip_formatter {
            formatter(value)
        } else {
            format!("{:.1}", value)
        }
    };

    rsx! {
        style { {SLIDER_STYLES} }

        div {
            class: class_name,
            style: props.style.as_deref().unwrap_or(""),
            tabindex: if props.disabled { "-1" } else { "0" },
            autofocus: props.auto_focus,
            onkeydown: handle_key_down,
            onfocus: handle_focus,
            onblur: handle_blur,
            onmouseup: handle_mouse_up,

            div {
                class: "ant-slider-rail",
                onclick: handle_track_click,
            }

            div {
                class: "ant-slider-track",
                style: "{track_style}",
            }

            if props.dots {
                div {
                    class: "ant-slider-dots",

                    {
                        let mut dots = Vec::new();
                        let mut current = props.min;
                        while current <= props.max {
                            let percentage = get_percentage(current);
                            let is_active = if props.range {
                                current >= left_value && current <= right_value
                            } else {
                                current <= left_value
                            };

                            dots.push(rsx! {
                                span {
                                    key: "{current}",
                                    class: format!(
                                        "ant-slider-dot {}",
                                        if is_active { "ant-slider-dot-active" } else { "" }
                                    ),
                                    style: if props.vertical {
                                        format!("bottom: {}%", percentage)
                                    } else {
                                        format!("left: {}%", percentage)
                                    },
                                }
                            });

                            current += props.step;
                        }

                        rsx! { {dots.into_iter()} }
                    }
                }
            }

            if !props.marks.is_empty() {
                div {
                    class: "ant-slider-marks",

                    for (key, mark) in props.marks.iter() {
                        {
                            let percentage = get_percentage(mark.value);
                            let is_active = if props.range {
                                mark.value >= left_value && mark.value <= right_value
                            } else {
                                mark.value <= left_value
                            };

                            rsx! {
                                span {
                                    key: "{key}",
                                    class: format!(
                                        "ant-slider-mark {}",
                                        if is_active { "ant-slider-mark-active" } else { "" }
                                    ),
                                    style: format!(
                                        "{} {}",
                                        if props.vertical {
                                            format!("bottom: {}%", percentage)
                                        } else {
                                            format!("left: {}%", percentage)
                                        },
                                        mark.style.as_deref().unwrap_or("")
                                    ),

                                    "{mark.label}"
                                }
                            }
                        }
                    }
                }
            }

            // 左侧/单个滑块手柄
            div {
                class: format!(
                    "ant-slider-handle {} {}",
                    if active_handle() == 0 { "ant-slider-handle-active" } else { "" },
                    if dragging() && active_handle() == 0 { "ant-slider-handle-dragging" } else { "" }
                ),
                style: if props.vertical {
                    format!("bottom: {}%", left_percentage)
                } else {
                    format!("left: {}%", left_percentage)
                },
                tabindex: "0",
                onmousedown: move |_| handle_mouse_down(0),
                onmouseenter: move |_| show_tooltip.set(true),
                onmouseleave: move |_| if !dragging() { show_tooltip.set(false) },

                if props.tooltip_visible && (show_tooltip() || dragging()) {
                    div {
                        class: format!("ant-slider-tooltip ant-slider-tooltip-{}", props.tooltip_placement),
                        "{format_tooltip(left_value)}"
                    }
                }
            }

            // 右侧滑块手柄（仅范围模式）
            if props.range {
                div {
                    class: format!(
                        "ant-slider-handle {} {}",
                        if active_handle() == 1 { "ant-slider-handle-active" } else { "" },
                        if dragging() && active_handle() == 1 { "ant-slider-handle-dragging" } else { "" }
                    ),
                    style: if props.vertical {
                        format!("bottom: {}%", right_percentage)
                    } else {
                        format!("left: {}%", right_percentage)
                    },
                    tabindex: "0",
                    onmousedown: move |_| handle_mouse_down(1),
                    onmouseenter: move |_| show_tooltip.set(true),
                    onmouseleave: move |_| if !dragging() { show_tooltip.set(false) },

                    if props.tooltip_visible && (show_tooltip() || dragging()) {
                        div {
                            class: format!("ant-slider-tooltip ant-slider-tooltip-{}", props.tooltip_placement),
                            "{format_tooltip(right_value)}"
                        }
                    }
                }
            }
        }
    }
}

/// 滑块标记构建器
pub struct SliderMarkBuilder {
    value: f64,
    label: String,
    style: Option<String>,
}

impl SliderMarkBuilder {
    pub fn new(value: f64, label: impl Into<String>) -> Self {
        Self {
            value,
            label: label.into(),
            style: None,
        }
    }

    pub fn style(mut self, style: impl Into<String>) -> Self {
        self.style = Some(style.into());
        self
    }

    pub fn build(self) -> SliderMark {
        SliderMark {
            value: self.value,
            label: self.label,
            style: self.style,
        }
    }
}

/// 创建滑块标记的宏
#[macro_export]
macro_rules! slider_mark {
    ($value:expr, $label:expr) => {
        SliderMark::new($value, $label)
    };
    ($value:expr, $label:expr, style: $style:expr) => {
        SliderMark::new($value, $label).with_style($style)
    };
}

/// 创建滑块组件的宏
#[macro_export]
macro_rules! slider {
    () => {
        Slider {
            min: 0.0,
            max: 100.0,
            step: 1.0
        }
    };
    ($($prop:ident: $value:expr),* $(,)?) => {
        Slider {
            $($prop: $value,)*
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slider_mark() {
        let mark = SliderMark::new(50.0, "Middle");
        assert_eq!(mark.value, 50.0);
        assert_eq!(mark.label, "Middle");
        assert!(mark.style.is_none());

        let styled_mark = SliderMark::new(75.0, "High").with_style("color: red");
        assert_eq!(styled_mark.value, 75.0);
        assert_eq!(styled_mark.label, "High");
        assert_eq!(styled_mark.style, Some("color: red".to_string()));
    }

    #[test]
    fn test_slider_size() {
        assert_eq!(SliderSize::Small.to_string(), "small");
        assert_eq!(SliderSize::Middle.to_string(), "middle");
        assert_eq!(SliderSize::Large.to_string(), "large");
    }

    #[test]
    fn test_slider_status() {
        assert_eq!(SliderStatus::Default.to_string(), "default");
        assert_eq!(SliderStatus::Error.to_string(), "error");
        assert_eq!(SliderStatus::Warning.to_string(), "warning");
    }

    #[test]
    fn test_slider_mark_builder() {
        let mark = SliderMarkBuilder::new(25.0, "Quarter")
            .style("font-weight: bold")
            .build();

        assert_eq!(mark.value, 25.0);
        assert_eq!(mark.label, "Quarter");
        assert_eq!(mark.style, Some("font-weight: bold".to_string()));
    }

    #[test]
    fn test_slider_defaults() {
        assert_eq!(SliderSize::default(), SliderSize::Middle);
        assert_eq!(SliderStatus::default(), SliderStatus::Default);
    }
}

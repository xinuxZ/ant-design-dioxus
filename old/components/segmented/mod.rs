use dioxus::prelude::*;

/// Segmented item type
#[derive(Clone, Debug, PartialEq)]
pub struct SegmentedItem {
    /// Display text for Segmented item
    pub label: Option<Element>,
    /// Value for Segmented item
    pub value: String,
    /// Display icon for Segmented item
    pub icon: Option<Element>,
    /// Disabled state of segmented item
    pub disabled: bool,
    /// The additional css class
    pub class_name: Option<String>,
}

/// Segmented size
#[derive(Clone, Debug, PartialEq)]
pub enum SegmentedSize {
    Large,
    Middle,
    Small,
}

impl Default for SegmentedSize {
    fn default() -> Self {
        Self::Middle
    }
}

/// Segmented shape
#[derive(Clone, Debug, PartialEq)]
pub enum SegmentedShape {
    Default,
    Round,
}

impl Default for SegmentedShape {
    fn default() -> Self {
        Self::Default
    }
}

/// Segmented option type
#[derive(Clone, Debug, PartialEq)]
pub enum SegmentedOption {
    String(String),
    Number(i32),
    Item(SegmentedItem),
}

impl From<String> for SegmentedOption {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for SegmentedOption {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<i32> for SegmentedOption {
    fn from(value: i32) -> Self {
        Self::Number(value)
    }
}

impl From<SegmentedItem> for SegmentedOption {
    fn from(value: SegmentedItem) -> Self {
        Self::Item(value)
    }
}

/// Segmented component properties
#[derive(Props, Clone, PartialEq)]
pub struct SegmentedProps {
    /// Option to fit width to its parent's width
    #[props(default = false)]
    pub block: bool,

    /// Default selected value
    #[props(default)]
    pub default_value: Option<String>,

    /// Disable all segments
    #[props(default = false)]
    pub disabled: bool,

    /// The callback function that is triggered when the state changes
    #[props(default)]
    pub on_change: Option<EventHandler<String>>,

    /// Set children optional
    pub options: Vec<SegmentedOption>,

    /// The size of the Segmented
    #[props(default)]
    pub size: SegmentedSize,

    /// Orientation
    #[props(default = false)]
    pub vertical: bool,

    /// Currently selected value
    #[props(default)]
    pub value: Option<String>,

    /// Shape of Segmented
    #[props(default)]
    pub shape: SegmentedShape,

    /// The name property of all input[type="radio"] children
    #[props(default)]
    pub name: Option<String>,

    /// CSS 类名
    #[props(default)]
    pub class: Option<String>,

    /// 内联样式
    #[props(default)]
    pub style: Option<String>,

    /// 元素 ID
    #[props(default)]
    pub id: Option<String>,
}

/// Segmented component
#[component]
pub fn Segmented(props: SegmentedProps) -> Element {
    let mut selected_value = use_signal(|| {
        props
            .value
            .clone()
            .or_else(|| props.default_value.clone())
            .unwrap_or_else(|| {
                // Use first option as default
                match props.options.first() {
                    Some(SegmentedOption::String(s)) => s.clone(),
                    Some(SegmentedOption::Number(n)) => n.to_string(),
                    Some(SegmentedOption::Item(item)) => item.value.clone(),
                    None => String::new(),
                }
            })
    });

    // Update selected value when props.value changes
    use_effect({
        let mut selected_value = selected_value.clone();
        let value = props.value.clone();
        move || {
            if let Some(value) = &value {
                selected_value.set(value.clone());
            }
        }
    });

    let handle_click = move |value: String| {
        if !props.disabled {
            selected_value.set(value.clone());
            if let Some(on_change) = &props.on_change {
                on_change.call(value);
            }
        }
    };

    let size_class = match props.size {
        SegmentedSize::Large => "ant-segmented-lg",
        SegmentedSize::Middle => "",
        SegmentedSize::Small => "ant-segmented-sm",
    };

    let shape_class = match props.shape {
        SegmentedShape::Default => "",
        SegmentedShape::Round => "ant-segmented-round",
    };

    let class_name = format!(
        "ant-segmented {} {} {} {} {}",
        if props.block {
            "ant-segmented-block"
        } else {
            ""
        },
        if props.disabled {
            "ant-segmented-disabled"
        } else {
            ""
        },
        if props.vertical {
            "ant-segmented-vertical"
        } else {
            ""
        },
        size_class,
        shape_class
    )
    .trim()
    .to_string();

    let final_class = if let Some(custom_class) = &props.class {
        format!("{} {}", class_name, custom_class)
    } else {
        class_name
    };

    rsx! {
        div {
            class: "{final_class}",
            id: props.id,
            style: props.style,

            div {
                class: "ant-segmented-group",

                for (index, option) in props.options.iter().enumerate() {
                    {render_segmented_item(
                        option,
                        &selected_value.read(),
                        props.disabled,
                        props.name.as_deref(),
                        index,
                        handle_click
                    )}
                }
            }
        }
    }
}

fn render_segmented_item(
    option: &SegmentedOption,
    selected_value: &str,
    global_disabled: bool,
    name: Option<&str>,
    index: usize,
    mut on_click: impl FnMut(String) + 'static,
) -> Element {
    let (value, label, icon, disabled, class_name) = match option {
        SegmentedOption::String(s) => (s.clone(), Some(rsx! { "{s}" }), None, false, None),
        SegmentedOption::Number(n) => (n.to_string(), Some(rsx! { "{n}" }), None, false, None),
        SegmentedOption::Item(item) => {
            let label = item.label.as_ref().map(|l| rsx! { {l} });
            (
                item.value.clone(),
                label,
                item.icon.as_ref(),
                item.disabled,
                item.class_name.as_ref(),
            )
        }
    };

    let is_selected = selected_value == value;
    let is_disabled = global_disabled || disabled;

    let item_class = format!(
        "ant-segmented-item {} {} {}",
        if is_selected {
            "ant-segmented-item-selected"
        } else {
            ""
        },
        if is_disabled {
            "ant-segmented-item-disabled"
        } else {
            ""
        },
        class_name.unwrap_or(&("".to_string())),
    )
    .trim()
    .to_string();

    let input_name = name.unwrap_or("ant-segmented");
    let input_id = format!("{}-{}", input_name, index);

    rsx! {
        label {
            class: "{item_class}",

            input {
                r#type: "radio",
                name: "{input_name}",
                id: "{input_id}",
                value: "{value}",
                checked: is_selected,
                disabled: is_disabled,
                onchange: move |_| {
                    if !is_disabled {
                        on_click(value.clone());
                    }
                },
            }

            div {
                class: "ant-segmented-item-label",

                if let Some(icon_element) = icon {
                    span {
                        class: "ant-segmented-item-icon",
                        {icon_element}
                    }
                }

                if let Some(label_element) = label {
                    span {
                        class: "ant-segmented-item-text",
                        {label_element}
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segmented_option_from_string() {
        let option: SegmentedOption = "test".into();
        assert_eq!(option, SegmentedOption::String("test".to_string()));
    }

    #[test]
    fn test_segmented_option_from_number() {
        let option: SegmentedOption = 42.into();
        assert_eq!(option, SegmentedOption::Number(42));
    }

    #[test]
    fn test_segmented_item() {
        let item = SegmentedItem {
            label: None,
            value: "test".to_string(),
            icon: None,
            disabled: false,
            class_name: None,
        };
        let option: SegmentedOption = item.clone().into();
        assert_eq!(option, SegmentedOption::Item(item));
    }
}

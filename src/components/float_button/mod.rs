use dioxus::prelude::*;
use wasm_bindgen::JsCast;

const FLOAT_BUTTON_STYLE: &str = include_str!("./style.css");

/// FloatButton type
#[derive(Clone, Debug, PartialEq)]
pub enum FloatButtonType {
    Default,
    Primary,
}

impl Default for FloatButtonType {
    fn default() -> Self {
        Self::Default
    }
}

/// FloatButton shape
#[derive(Clone, Debug, PartialEq)]
pub enum FloatButtonShape {
    Circle,
    Square,
}

impl Default for FloatButtonShape {
    fn default() -> Self {
        Self::Circle
    }
}

/// FloatButton trigger type
#[derive(Clone, Debug, PartialEq)]
pub enum FloatButtonTrigger {
    Click,
    Hover,
}

/// FloatButton placement
#[derive(Clone, Debug, PartialEq)]
pub enum FloatButtonPlacement {
    Top,
    Left,
    Right,
    Bottom,
}

impl Default for FloatButtonPlacement {
    fn default() -> Self {
        Self::Top
    }
}

/// FloatButton HTML type
#[derive(Clone, Debug, PartialEq)]
pub enum FloatButtonHtmlType {
    Submit,
    Reset,
    Button,
}

impl Default for FloatButtonHtmlType {
    fn default() -> Self {
        Self::Button
    }
}

/// Badge properties for FloatButton
#[derive(Props, Clone, PartialEq)]
pub struct FloatButtonBadgeProps {
    /// Badge count
    #[props(default)]
    pub count: Option<i32>,

    /// Badge dot
    #[props(default = false)]
    pub dot: bool,

    /// Badge color
    #[props(default)]
    pub color: Option<String>,

    /// Badge size
    #[props(default)]
    pub size: Option<String>,
}

/// FloatButton component properties
#[derive(Props, Clone, PartialEq)]
pub struct FloatButtonProps {
    /// Set the icon component of button
    #[props(default)]
    pub icon: Option<Element>,

    /// Text and other
    #[props(default)]
    pub description: Option<Element>,

    /// The text shown in the tooltip
    #[props(default)]
    pub tooltip: Option<String>,

    /// Setting button type
    #[props(default)]
    pub r#type: FloatButtonType,

    /// Setting button shape
    #[props(default)]
    pub shape: FloatButtonShape,

    /// Set the handler to handle click event
    #[props(default)]
    pub on_click: Option<EventHandler<MouseEvent>>,

    /// The target of hyperlink
    #[props(default)]
    pub href: Option<String>,

    /// Specifies where to display the linked URL
    #[props(default)]
    pub target: Option<String>,

    /// Set the original html type of button
    #[props(default)]
    pub html_type: FloatButtonHtmlType,

    /// Attach Badge to FloatButton
    #[props(default)]
    pub badge: Option<FloatButtonBadgeProps>,

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

/// FloatButton component
#[component]
pub fn FloatButton(props: FloatButtonProps) -> Element {
    let type_class = match props.r#type {
        FloatButtonType::Default => "",
        FloatButtonType::Primary => "ant-float-btn-primary",
    };

    let shape_class = match props.shape {
        FloatButtonShape::Circle => "",
        FloatButtonShape::Square => "ant-float-btn-square",
    };

    let class_name = format!("ant-float-btn {} {}", type_class, shape_class)
        .trim()
        .to_string();

    let final_class = if let Some(custom_class) = &props.class {
        format!("{} {}", class_name, custom_class)
    } else {
        class_name
    };

    let button_content = rsx! {
        div {
            class: "ant-float-btn-body",

            if let Some(icon) = &props.icon {
                div {
                    class: "ant-float-btn-icon",
                    {icon}
                }
            }

            if let Some(description) = &props.description {
                div {
                    class: "ant-float-btn-description",
                    {description}
                }
            }
        }
    };

    let button_element = if let Some(href) = &props.href {
        rsx! {
            style { {FLOAT_BUTTON_STYLE} }

            a {
                class: "{final_class}",
                id: props.id,
                style: props.style,
                href: "{href}",
                target: props.target,
                title: props.tooltip,
                onclick: move |evt| {
                    if let Some(on_click) = &props.on_click {
                        on_click.call(evt);
                    }
                },
                {button_content}
            }
        }
    } else {
        let html_type = match props.html_type {
            FloatButtonHtmlType::Submit => "submit",
            FloatButtonHtmlType::Reset => "reset",
            FloatButtonHtmlType::Button => "button",
        };

        rsx! {
            style { {FLOAT_BUTTON_STYLE} }

            button {
                class: "{final_class}",
                id: props.id,
                style: props.style,
                r#type: "{html_type}",
                title: props.tooltip,
                onclick: move |evt| {
                    if let Some(on_click) = &props.on_click {
                        on_click.call(evt);
                    }
                },
                {button_content}
            }
        }
    };

    if let Some(badge) = &props.badge {
        rsx! {
            style { {FLOAT_BUTTON_STYLE} }

            div {
                class: "ant-float-btn-badge",

                {button_element}

                if badge.dot {
                    span {
                        class: "ant-badge ant-badge-dot",
                        style: if let Some(color) = &badge.color {
                            format!("background-color: {};", color)
                        } else {
                            String::new()
                        },
                    }
                } else if let Some(count) = badge.count {
                    if count > 0 {
                        span {
                            class: "ant-badge ant-badge-count",
                            style: if let Some(color) = &badge.color {
                                format!("background-color: {};", color)
                            } else {
                                String::new()
                            },
                            "{count}"
                        }
                    }
                }
            }
        }
    } else {
        button_element
    }
}

/// FloatButton Group properties
#[derive(Props, Clone, PartialEq)]
pub struct FloatButtonGroupProps {
    /// Setting button shape of children
    #[props(default)]
    pub shape: FloatButtonShape,

    /// Which action can trigger menu open/close
    #[props(default)]
    pub trigger: Option<FloatButtonTrigger>,

    /// Whether the menu is visible or not
    #[props(default)]
    pub open: Option<bool>,

    /// Customize close button icon
    #[props(default)]
    pub close_icon: Option<Element>,

    /// Customize menu animation placement
    #[props(default)]
    pub placement: FloatButtonPlacement,

    /// Callback executed when active menu is changed
    #[props(default)]
    pub on_open_change: Option<EventHandler<bool>>,

    /// Set the handler to handle click event (only work in Menu mode)
    #[props(default)]
    pub on_click: Option<EventHandler<MouseEvent>>,

    /// Main trigger button icon
    #[props(default)]
    pub icon: Option<Element>,

    /// Main trigger button description
    #[props(default)]
    pub description: Option<Element>,

    /// Main trigger button type
    #[props(default)]
    pub r#type: FloatButtonType,

    /// Children FloatButton components
    pub children: Element,

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

/// FloatButton Group component
#[component]
pub fn FloatButtonGroup(props: FloatButtonGroupProps) -> Element {
    let mut is_open = use_signal(|| props.open.unwrap_or(false));

    // Update open state when props.open changes
    use_effect(move || {
        if let Some(open) = props.open {
            is_open.set(open);
        }
    });

    let is_menu_mode = props.trigger.is_some();

    let handle_trigger_click = move |evt: MouseEvent| {
        if is_menu_mode {
            let new_open = !*is_open.read();
            is_open.set(new_open);
            if let Some(on_open_change) = &props.on_open_change {
                on_open_change.call(new_open);
            }
        }
        if let Some(on_click) = &props.on_click {
            on_click.call(evt);
        }
    };

    let shape_class = match props.shape {
        FloatButtonShape::Circle => "",
        FloatButtonShape::Square => "ant-float-btn-group-square",
    };

    let placement_class = match props.placement {
        FloatButtonPlacement::Top => "ant-float-btn-group-top",
        FloatButtonPlacement::Left => "ant-float-btn-group-left",
        FloatButtonPlacement::Right => "ant-float-btn-group-right",
        FloatButtonPlacement::Bottom => "ant-float-btn-group-bottom",
    };

    let class_name = format!(
        "ant-float-btn-group {} {}",
        shape_class,
        if is_menu_mode { placement_class } else { "" }
    )
    .trim()
    .to_string();

    let final_class = if let Some(custom_class) = &props.class {
        format!("{} {}", class_name, custom_class)
    } else {
        class_name
    };

    let defaultview = &rsx! {
        svg {
            width: "1em",
            height: "1em",
            fill: "currentColor",
            view_box: "0 0 1024 1024",
            path {
                d: "M563.8 512l262.5-312.9c4.4-5.2.7-13.1-6.1-13.1h-79.8c-4.7 0-9.2 2.1-12.3 5.7L511.6 449.8 295.1 191.7c-3-3.6-7.5-5.7-12.3-5.7H203c-6.8 0-10.5 7.9-6.1 13.1L459.4 512 196.9 824.9A7.95 7.95 0 0 0 203 838h79.8c4.7 0 9.2-2.1 12.3-5.7l216.5-258.1 216.5 258.1c3 3.6 7.5 5.7 12.3 5.7h79.8c6.8 0 10.5-7.9 6.1-13.1L563.8 512z"
            }
        }
    };
    let close_icon = props.close_icon.as_ref().unwrap_or(defaultview);

    rsx! {
        div {
            class: "{final_class}",
            id: props.id,
            style: props.style,

            if is_menu_mode {
                if *is_open.read() {
                    div {
                        class: "ant-float-btn-group-wrap",
                        {props.children}
                    }
                }

                FloatButton {
                    r#type: props.r#type,
                    shape: props.shape,
                    icon: if *is_open.read() { Some(close_icon.clone()) } else { props.icon },
                    description: props.description,
                    on_click: handle_trigger_click,
                    class: Some("ant-float-btn-group-trigger".to_string()),
                }
            } else {
                {props.children}
            }
        }
    }
}

/// FloatButton BackTop properties
#[derive(Props, Clone, PartialEq)]
pub struct FloatButtonBackTopProps {
    /// Time to return to top (ms)
    #[props(default = 450)]
    pub duration: u32,

    /// The BackTop button will not show until the scroll height reaches this value
    #[props(default = 400)]
    pub visibility_height: u32,

    /// A callback function, which can be executed when you click the button
    #[props(default)]
    pub on_click: Option<EventHandler<MouseEvent>>,

    /// Custom icon
    #[props(default)]
    pub icon: Option<Element>,

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

/// FloatButton BackTop component
#[component]
pub fn FloatButtonBackTop(props: FloatButtonBackTopProps) -> Element {
    let is_visible = use_signal(|| false);

    // Check scroll position to show/hide button
    use_effect(move || {
        let mut is_visible_clone = is_visible.clone();
        let visibility_height = props.visibility_height;

        let mut check_scroll = move || {
            if let Some(window) = web_sys::window() {
                let scroll_y = window.scroll_y().unwrap_or(0.0);
                is_visible_clone.set(scroll_y >= visibility_height as f64);
            }
        };

        // Initial check
        check_scroll();

        // Add scroll listener
        if let Some(window) = web_sys::window() {
            let mut is_visible_clone2 = is_visible_clone.clone();
            let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
                if let Some(window) = web_sys::window() {
                    let scroll_y = window.scroll_y().unwrap_or(0.0);
                    is_visible_clone2.set(scroll_y >= visibility_height as f64);
                }
            }) as Box<dyn FnMut()>);

            let _ =
                window.add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref());
            closure.forget(); // Keep the closure alive
        }
    });

    let handle_click = move |evt: MouseEvent| {
        // Scroll to top with smooth behavior
        if let Some(window) = web_sys::window() {
            let options = web_sys::ScrollToOptions::new();
            options.set_top(0.0);
            options.set_behavior(web_sys::ScrollBehavior::Smooth);
            window.scroll_to_with_scroll_to_options(&options);
        }

        if let Some(on_click) = &props.on_click {
            on_click.call(evt);
        }
    };

    let default_icon = rsx! {
        svg {
            width: "1em",
            height: "1em",
            fill: "currentColor",
            view_box: "0 0 1024 1024",
            path {
                d: "M868 545.5L536.1 163a31.96 31.96 0 0 0-48.3 0L156 545.5a7.97 7.97 0 0 0 6 13.2h81c4.6 0 9-2 12.1-5.5L474 300.9V864c0 4.4 3.6 8 8 8h60c4.4 0 8-3.6 8-8V300.9l218.9 252.3c3 3.5 7.4 5.5 12.1 5.5h81c6.8 0 10.5-8 6-13.2z"
            }
        }
    };

    if *is_visible.read() {
        rsx! {
            FloatButton {
                icon: props.icon.unwrap_or(default_icon),
                on_click: handle_click,
                class: Some(format!(
                    "ant-float-btn-back-top {}",
                    props.class.as_deref().unwrap_or("")
                )),
                id: props.id,
                style: props.style,
            }
        }
    } else {
        rsx! { div { style: "display: none;" } }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_float_button_type_default() {
        let button_type = FloatButtonType::default();
        assert_eq!(button_type, FloatButtonType::Default);
    }

    #[test]
    fn test_float_button_shape_default() {
        let shape = FloatButtonShape::default();
        assert_eq!(shape, FloatButtonShape::Circle);
    }

    #[test]
    fn test_float_button_placement_default() {
        let placement = FloatButtonPlacement::default();
        assert_eq!(placement, FloatButtonPlacement::Top);
    }

    #[test]
    fn test_float_button_html_type_default() {
        let html_type = FloatButtonHtmlType::default();
        assert_eq!(html_type, FloatButtonHtmlType::Button);
    }
}

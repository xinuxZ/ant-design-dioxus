use dioxus::prelude::*;
use wasm_bindgen::JsCast;

/// Layout direction for Splitter
#[derive(Clone, Debug, PartialEq)]
pub enum SplitterLayout {
    Horizontal,
    Vertical,
}

impl Default for SplitterLayout {
    fn default() -> Self {
        Self::Horizontal
    }
}

/// Collapsible configuration
#[derive(Clone, Debug, PartialEq)]
pub struct CollapsibleConfig {
    /// Allow collapse at start
    pub start: bool,
    /// Allow collapse at end
    pub end: bool,
}

impl Default for CollapsibleConfig {
    fn default() -> Self {
        Self {
            start: false,
            end: false,
        }
    }
}

/// Collapsible option
#[derive(Clone, Debug, PartialEq)]
pub enum Collapsible {
    Bool(bool),
    Config(CollapsibleConfig),
}

impl Default for Collapsible {
    fn default() -> Self {
        Self::Bool(false)
    }
}

/// Size type for panel dimensions
#[derive(Clone, Debug, PartialEq)]
pub enum SizeType {
    /// Pixel value
    Px(f64),
    /// Percentage value
    Percent(f64),
}

impl SizeType {
    /// Convert to CSS string
    pub fn to_css(&self) -> String {
        match self {
            SizeType::Px(value) => format!("{}px", value),
            SizeType::Percent(value) => format!("{}%", value),
        }
    }

    /// Parse from string
    pub fn from_str(s: &str) -> Option<Self> {
        if s.ends_with('%') {
            s.trim_end_matches('%')
                .parse::<f64>()
                .ok()
                .map(SizeType::Percent)
        } else if s.ends_with("px") {
            s.trim_end_matches("px")
                .parse::<f64>()
                .ok()
                .map(SizeType::Px)
        } else {
            s.parse::<f64>().ok().map(SizeType::Px)
        }
    }
}

/// Splitter component properties
#[derive(Props, Clone, PartialEq)]
pub struct SplitterProps {
    /// Layout direction
    #[props(default)]
    pub layout: SplitterLayout,

    /// Callback before dragging starts
    #[props(default)]
    pub on_resize_start: Option<EventHandler<Vec<f64>>>,

    /// Panel size change callback
    #[props(default)]
    pub on_resize: Option<EventHandler<Vec<f64>>>,

    /// Drag end callback
    #[props(default)]
    pub on_resize_end: Option<EventHandler<Vec<f64>>>,

    /// Lazy mode - updates size only when released
    #[props(default)]
    pub lazy: bool,

    /// Children panels
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

/// Panel component properties
#[derive(Props, Clone, PartialEq)]
pub struct SplitterPanelProps {
    /// Initial panel size
    #[props(default)]
    pub default_size: Option<SizeType>,

    /// Minimum threshold
    #[props(default)]
    pub min: Option<SizeType>,

    /// Maximum threshold
    #[props(default)]
    pub max: Option<SizeType>,

    /// Controlled panel size
    #[props(default)]
    pub size: Option<SizeType>,

    /// Quick folding configuration
    #[props(default)]
    pub collapsible: Collapsible,

    /// Whether to enable drag and drop
    #[props(default = true)]
    pub resizable: bool,

    /// Panel content
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

/// Splitter component
#[component]
pub fn Splitter(props: SplitterProps) -> Element {
    let mut is_dragging = use_signal(|| false);
    let mut drag_start_pos = use_signal(|| (0.0, 0.0));
    let panel_sizes = use_signal(|| Vec::<f64>::new());
    let mut active_splitter = use_signal(|| None::<usize>);

    let class_name = format!(
        "ant-splitter ant-splitter-{} {}",
        match props.layout {
            SplitterLayout::Horizontal => "horizontal",
            SplitterLayout::Vertical => "vertical",
        },
        props.class.as_deref().unwrap_or("")
    )
    .trim()
    .to_string();

    let _handle_mouse_down = move |evt: MouseEvent, splitter_index: usize| {
        evt.prevent_default();
        is_dragging.set(true);
        active_splitter.set(Some(splitter_index));
        // Simplified: use fixed coordinates for now
        drag_start_pos.set((0.0, 0.0));

        if let Some(on_resize_start) = &props.on_resize_start {
            on_resize_start.call(panel_sizes.read().clone());
        }
    };

    let _handle_mouse_move = move |evt: MouseEvent| {
        if !*is_dragging.read() {
            return;
        }

        evt.prevent_default();

        // Simplified: use fixed coordinates for now
        let current_pos = (100.0, 100.0);
        let start_pos = *drag_start_pos.read();

        let _delta = match props.layout {
            SplitterLayout::Horizontal => current_pos.0 - start_pos.0,
            SplitterLayout::Vertical => current_pos.1 - start_pos.1,
        };

        // Update panel sizes based on delta
        // This is a simplified implementation
        if let Some(on_resize) = &props.on_resize {
            if !props.lazy {
                on_resize.call(panel_sizes.read().clone());
            }
        }
    };

    let _handle_mouse_up = move |_evt: MouseEvent| {
        if *is_dragging.read() {
            is_dragging.set(false);
            active_splitter.set(None);

            if let Some(on_resize_end) = &props.on_resize_end {
                on_resize_end.call(panel_sizes.read().clone());
            }

            if let Some(on_resize) = &props.on_resize {
                if props.lazy {
                    on_resize.call(panel_sizes.read().clone());
                }
            }
        }
    };

    // Add global mouse event listeners
    use_effect(move || {
        let document = web_sys::window().unwrap().document().unwrap();

        let mousemove_closure =
            wasm_bindgen::closure::Closure::wrap(Box::new(move |_evt: web_sys::MouseEvent| {
                // TODO: Handle mouse move with proper coordinates
                // Simplified implementation for now
            }) as Box<dyn FnMut(_)>);

        let mouseup_closure =
            wasm_bindgen::closure::Closure::wrap(Box::new(move |_evt: web_sys::MouseEvent| {
                // TODO: Handle mouse up with proper coordinates
                // Simplified implementation for now
                // handle_mouse_up(mouse_evt);
            }) as Box<dyn FnMut(_)>);

        document
            .add_event_listener_with_callback(
                "mousemove",
                mousemove_closure.as_ref().unchecked_ref(),
            )
            .unwrap();
        document
            .add_event_listener_with_callback("mouseup", mouseup_closure.as_ref().unchecked_ref())
            .unwrap();

        // Cleanup function
        (move || {
            document
                .remove_event_listener_with_callback(
                    "mousemove",
                    mousemove_closure.as_ref().unchecked_ref(),
                )
                .unwrap();
            document
                .remove_event_listener_with_callback(
                    "mouseup",
                    mouseup_closure.as_ref().unchecked_ref(),
                )
                .unwrap();
            mousemove_closure.forget();
            mouseup_closure.forget();
        })()
    });

    rsx! {
            div {
                class: "{class_name}",
                id: props.id,
                style: props.style,

            // Render panels with splitters
            {props.children}
        }
    }
}

/// Panel component
#[component]
pub fn SplitterPanel(props: SplitterPanelProps) -> Element {
    let mut is_collapsed = use_signal(|| false);

    let class_name = format!(
        "ant-splitter-panel {}",
        props.class.as_deref().unwrap_or("")
    )
    .trim()
    .to_string();

    let mut panel_style = props.style.clone().unwrap_or_default();

    // Apply size constraints
    if let Some(size) = &props.size {
        let size_value = size.to_css();
        panel_style.push_str(&format!("flex: 0 0 {};", size_value));
    } else if let Some(default_size) = &props.default_size {
        let size_value = default_size.to_css();
        panel_style.push_str(&format!("flex: 0 0 {};", size_value));
    } else {
        panel_style.push_str("flex: 1;");
    }

    if let Some(min) = &props.min {
        let min_value = min.to_css();
        panel_style.push_str(&format!("min-width: {};", min_value));
    }

    if let Some(max) = &props.max {
        let max_value = max.to_css();
        panel_style.push_str(&format!("max-width: {};", max_value));
    }

    if *is_collapsed.read() {
        panel_style.push_str("flex: 0 0 0; overflow: hidden;");
    }

    let handle_collapse = move |_| {
        let current_value = *is_collapsed.read();
        is_collapsed.set(!current_value);
    };

    rsx! {
            div {
                class: "{class_name}",
                id: props.id,
                style: "{panel_style}",

            // Collapse button if collapsible
            if let Collapsible::Bool(true) = props.collapsible {
                button {
                    class: "ant-splitter-panel-collapse-btn",
                    onclick: handle_collapse,
                    "⟨"
                }
            }

            if let Collapsible::Config(config) = &props.collapsible {
                if config.start || config.end {
                    button {
                        class: "ant-splitter-panel-collapse-btn",
                        onclick: handle_collapse,
                        "⟨"
                    }
                }
            }

            div {
                class: "ant-splitter-panel-content",
                {props.children}
            }
        }
    }
}

/// Splitter bar component for dragging
#[component]
fn SplitterBar(
    layout: SplitterLayout,
    index: usize,
    on_mouse_down: EventHandler<(MouseEvent, usize)>,
    resizable: bool,
) -> Element {
    let class_name = format!(
        "ant-splitter-bar ant-splitter-bar-{}",
        match layout {
            SplitterLayout::Horizontal => "horizontal",
            SplitterLayout::Vertical => "vertical",
        }
    );

    let cursor_style = if resizable {
        match layout {
            SplitterLayout::Horizontal => "cursor: col-resize;",
            SplitterLayout::Vertical => "cursor: row-resize;",
        }
    } else {
        "cursor: not-allowed;"
    };

    rsx! {
        div {
            class: "{class_name}",
            style: "{cursor_style}",
            onmousedown: move |evt| {
                if resizable {
                    on_mouse_down.call((evt, index));
                }
            },

            div {
                class: "ant-splitter-bar-dragger",
            }
        }
    }
}

/// Helper function to create a horizontal splitter
pub fn horizontal_splitter(children: Element) -> Element {
    rsx! {
        Splitter {
            layout: SplitterLayout::Horizontal,
            children
        }
    }
}

/// Helper function to create a vertical splitter
pub fn vertical_splitter(children: Element) -> Element {
    rsx! {
        Splitter {
            layout: SplitterLayout::Vertical,
            children
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size_type_to_css() {
        assert_eq!(SizeType::Px(100.0).to_css(), "100px");
        assert_eq!(SizeType::Percent(50.0).to_css(), "50%");
    }

    #[test]
    fn test_size_type_from_str() {
        assert_eq!(SizeType::from_str("100px"), Some(SizeType::Px(100.0)));
        assert_eq!(SizeType::from_str("50%"), Some(SizeType::Percent(50.0)));
        assert_eq!(SizeType::from_str("200"), Some(SizeType::Px(200.0)));
        assert_eq!(SizeType::from_str("invalid"), None);
    }

    #[test]
    fn test_splitter_layout_default() {
        assert_eq!(SplitterLayout::default(), SplitterLayout::Horizontal);
    }

    #[test]
    fn test_collapsible_config_default() {
        let config = CollapsibleConfig::default();
        assert!(!config.start);
        assert!(!config.end);
    }
}

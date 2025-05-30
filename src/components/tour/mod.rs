use dioxus::prelude::*;

/// Tour placement options
#[derive(Clone, Debug, PartialEq)]
pub enum TourPlacement {
    Center,
    Left,
    LeftTop,
    LeftBottom,
    Right,
    RightTop,
    RightBottom,
    Top,
    TopLeft,
    TopRight,
    Bottom,
    BottomLeft,
    BottomRight,
}

impl Default for TourPlacement {
    fn default() -> Self {
        Self::Bottom
    }
}

impl TourPlacement {
    pub fn as_str(&self) -> &'static str {
        match self {
            TourPlacement::Center => "center",
            TourPlacement::Left => "left",
            TourPlacement::LeftTop => "leftTop",
            TourPlacement::LeftBottom => "leftBottom",
            TourPlacement::Right => "right",
            TourPlacement::RightTop => "rightTop",
            TourPlacement::RightBottom => "rightBottom",
            TourPlacement::Top => "top",
            TourPlacement::TopLeft => "topLeft",
            TourPlacement::TopRight => "topRight",
            TourPlacement::Bottom => "bottom",
            TourPlacement::BottomLeft => "bottomLeft",
            TourPlacement::BottomRight => "bottomRight",
        }
    }
}

/// Tour type
#[derive(Clone, Debug, PartialEq)]
pub enum TourType {
    Default,
    Primary,
}

impl Default for TourType {
    fn default() -> Self {
        Self::Default
    }
}

impl TourType {
    pub fn as_str(&self) -> &'static str {
        match self {
            TourType::Default => "default",
            TourType::Primary => "primary",
        }
    }
}

/// Arrow configuration
#[derive(Clone, Debug, PartialEq)]
pub struct ArrowConfig {
    /// Whether to show arrow
    pub show: bool,
    /// Point at center
    pub point_at_center: bool,
}

impl Default for ArrowConfig {
    fn default() -> Self {
        Self {
            show: true,
            point_at_center: false,
        }
    }
}

/// Gap configuration for highlight area
#[derive(Clone, Debug, PartialEq)]
pub struct GapConfig {
    /// Offset from the element
    pub offset: Option<(i32, i32)>,
    /// Radius of the highlight area
    pub radius: Option<i32>,
}

impl Default for GapConfig {
    fn default() -> Self {
        Self {
            offset: Some((6, 6)),
            radius: Some(2),
        }
    }
}

/// Mask configuration
#[derive(Clone, Debug, PartialEq)]
pub struct MaskConfig {
    /// Whether to show mask
    pub show: bool,
    /// Mask style
    pub style: Option<String>,
    /// Mask color
    pub color: Option<String>,
}

impl Default for MaskConfig {
    fn default() -> Self {
        Self {
            show: true,
            style: None,
            color: None,
        }
    }
}

/// Button properties for tour navigation
#[derive(Clone, Debug, PartialEq)]
pub struct TourButtonProps {
    /// Button text
    pub children: Option<String>,
    /// Click handler
    pub on_click: Option<EventHandler<MouseEvent>>,
}

/// Tour step definition
#[derive(Clone, Debug, PartialEq)]
pub struct TourStep {
    /// Target element selector or reference
    pub target: Option<String>,
    /// Arrow configuration
    pub arrow: Option<ArrowConfig>,
    /// Close icon
    pub close_icon: Option<Element>,
    /// Cover image or video
    pub cover: Option<Element>,
    /// Step title
    pub title: Option<String>,
    /// Step description
    pub description: Option<String>,
    /// Placement relative to target
    pub placement: Option<TourPlacement>,
    /// Close callback
    pub on_close: Option<EventHandler<()>>,
    /// Mask configuration
    pub mask: Option<MaskConfig>,
    /// Tour type
    pub tour_type: Option<TourType>,
    /// Next button properties
    pub next_button_props: Option<TourButtonProps>,
    /// Previous button properties
    pub prev_button_props: Option<TourButtonProps>,
    /// Scroll into view options
    pub scroll_into_view_options: Option<bool>,
}

impl Default for TourStep {
    fn default() -> Self {
        Self {
            target: None,
            arrow: None,
            close_icon: None,
            cover: None,
            title: None,
            description: None,
            placement: None,
            on_close: None,
            mask: None,
            tour_type: None,
            next_button_props: None,
            prev_button_props: None,
            scroll_into_view_options: None,
        }
    }
}

/// Tour component properties
#[derive(Props, Clone, PartialEq)]
pub struct TourProps {
    /// Tour steps
    pub steps: Vec<TourStep>,

    /// Arrow configuration
    #[props(default)]
    pub arrow: ArrowConfig,

    /// Close icon
    pub close_icon: Option<Element>,

    /// Disable interaction on highlighted area
    #[props(default)]
    pub disabled_interaction: bool,

    /// Gap configuration
    #[props(default)]
    pub gap: GapConfig,

    /// Placement of the guide card
    #[props(default)]
    pub placement: TourPlacement,

    /// Close callback
    #[props(default)]
    pub on_close: Option<EventHandler<()>>,

    /// Mask configuration
    #[props(default)]
    pub mask: MaskConfig,

    /// Tour type
    #[props(default)]
    pub tour_type: TourType,

    /// Whether tour is open
    #[props(default)]
    pub open: bool,

    /// Step change callback
    #[props(default)]
    pub on_change: Option<EventHandler<usize>>,

    /// Current step
    #[props(default)]
    pub current: usize,

    /// Scroll into view options
    #[props(default)]
    pub scroll_into_view_options: bool,

    /// Custom indicators renderer
    #[props(default)]
    pub indicators_render: Option<fn(usize, usize) -> Element>,

    /// Custom actions renderer
    #[props(default)]
    pub actions_render: Option<fn(Element, usize, usize) -> Element>,

    /// Tour z-index
    #[props(default = 1001)]
    pub z_index: i32,

    /// Get popup container
    #[props(default)]
    pub get_popup_container: Option<fn() -> String>,

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

/// Tour component
#[component]
pub fn Tour(props: TourProps) -> Element {
    let mut current_step = use_signal(|| props.current);
    let mut is_open = use_signal(|| props.open);
    let target_element = use_signal(|| None::<String>);

    // Update current step when props change
    use_effect(move || {
        current_step.set(props.current);
    });

    // Update open state when props change
    use_effect(move || {
        is_open.set(props.open);
    });

    let class_name = format!(
        "ant-tour ant-tour-{} {} {}",
        props.tour_type.as_str(),
        if props.disabled_interaction {
            "ant-tour-disabled"
        } else {
            ""
        },
        props.class.as_deref().unwrap_or("")
    )
    .trim()
    .to_string();

    let steps_len = props.steps.len();
    let handle_next = move |_evt: MouseEvent| {
        let current = *current_step.read();
        if current < steps_len - 1 {
            let new_step = current + 1;
            current_step.set(new_step);
            if let Some(on_change) = &props.on_change {
                on_change.call(new_step);
            }
        } else {
            // Tour finished
            is_open.set(false);
            if let Some(on_close) = &props.on_close {
                on_close.call(());
            }
        }
    };

    let handle_prev = move |_evt: MouseEvent| {
        let current = *current_step.read();
        if current > 0 {
            let new_step = current - 1;
            current_step.set(new_step);
            if let Some(on_change) = &props.on_change {
                on_change.call(new_step);
            }
        }
    };

    let handle_close = move |_evt: MouseEvent| {
        is_open.set(false);
        if let Some(on_close) = &props.on_close {
            on_close.call(());
        }
    };

    let handle_skip = move |_evt: MouseEvent| {
        is_open.set(false);
        if let Some(on_close) = &props.on_close {
            on_close.call(());
        }
    };

    if !*is_open.read() || props.steps.is_empty() {
        return rsx! { div {} };
    }

    let current_step_data = &props.steps[*current_step.read()];
    let total_steps = props.steps.len();
    let current_index = *current_step.read();

    // Determine effective placement
    let effective_placement = current_step_data
        .placement
        .as_ref()
        .unwrap_or(&props.placement);

    // Determine effective mask
    let effective_mask = current_step_data.mask.as_ref().unwrap_or(&props.mask);

    // Determine effective type
    let effective_type = current_step_data
        .tour_type
        .as_ref()
        .unwrap_or(&props.tour_type);

    rsx! {
        div {
            class: "{class_name}",
            id: props.id,
            style: format!("z-index: {}; {}", props.z_index, props.style.as_deref().unwrap_or("")),

            // Mask overlay
            if effective_mask.show {
                div {
                    class: "ant-tour-mask",
                    style: format!("background-color: {}; {}", effective_mask.color.as_deref().unwrap_or("rgba(0, 0, 0, 0.45)"), effective_mask.style.as_deref().unwrap_or("")),
                    onclick: move |evt| {
                        if !props.disabled_interaction {
                            evt.stop_propagation();
                        }
                    },

                    // Highlight area (simplified implementation)
                    if let Some(target) = &current_step_data.target {
                        div {
                            class: "ant-tour-target-placeholder",
                            "data-target": target.as_str(),
                        }
                    }
                }
            }

            // Tour popup
            TourPopup {
                step: current_step_data.clone(),
                current: current_index,
                total: total_steps,
                placement: effective_placement.clone(),
                tour_type: effective_type.clone(),
                arrow: current_step_data.arrow.as_ref().unwrap_or(&props.arrow).clone(),
                on_next: handle_next,
                on_prev: handle_prev,
                on_close: handle_close,
                on_skip: handle_skip,
                indicators_render: props.indicators_render,
                actions_render: props.actions_render,
            }
        }
    }
}

/// Tour popup component
#[component]
fn TourPopup(
    step: TourStep,
    current: usize,
    total: usize,
    placement: TourPlacement,
    tour_type: TourType,
    arrow: ArrowConfig,
    on_next: EventHandler<MouseEvent>,
    on_prev: EventHandler<MouseEvent>,
    on_close: EventHandler<MouseEvent>,
    on_skip: EventHandler<MouseEvent>,
    indicators_render: Option<fn(usize, usize) -> Element>,
    actions_render: Option<fn(Element, usize, usize) -> Element>,
) -> Element {
    let popup_class = format!(
        "ant-tour-content ant-tour-content-{} ant-tour-placement-{}",
        tour_type.as_str(),
        placement.as_str()
    );

    // Default actions
    let default_actions = rsx! {
        div {
            class: "ant-tour-buttons",

            if current > 0 {
                button {
                    class: "ant-btn ant-tour-prev-btn",
                    onclick: on_prev,
                    {<std::string::String as Clone>::clone(&step.prev_button_props.as_ref().and_then(|p| p.children.as_ref()).unwrap_or(&"Previous".to_string()))}
                }
            }

            if current < total - 1 {
                button {
                    class: "ant-btn ant-btn-primary ant-tour-next-btn",
                    onclick: on_next,
                    {<std::string::String as Clone>::clone(&step.next_button_props.as_ref().and_then(|p| p.children.as_ref()).unwrap_or(&"Next".to_string()))}
                }
            } else {
                button {
                    class: "ant-btn ant-btn-primary ant-tour-finish-btn",
                    onclick: on_next,
                    "Finish"
                }
            }

            button {
                class: "ant-btn ant-tour-skip-btn",
                onclick: on_skip,
                "Skip"
            }
        }
    };

    rsx! {
        div {
            class: "{popup_class}",

            // Arrow
            if arrow.show {
                div {
                    class: "ant-tour-arrow",
                    "data-point-at-center": "{arrow.point_at_center}",
                }
            }

            // Close button
            button {
                class: "ant-tour-close",
                onclick: on_close,
                if let Some(close_icon) = step.close_icon {
                    {close_icon}
                } else {
                    "×"
                }
            }

            // Cover
            if let Some(cover) = step.cover {
                div {
                    class: "ant-tour-cover",
                    {cover}
                }
            }

            // Header
            if step.title.is_some() {
                div {
                    class: "ant-tour-header",
                    div {
                        class: "ant-tour-title",
                        {step.title.as_deref().unwrap_or("")}
                    }
                }
            }

            // Body
            div {
                class: "ant-tour-inner",

                if let Some(description) = &step.description {
                    div {
                        class: "ant-tour-description",
                        "{description}"
                    }
                }

                // Indicators
                div {
                    class: "ant-tour-indicators",
                    if let Some(render_fn) = indicators_render {
                        {render_fn(current, total)}
                    } else {
                        DefaultIndicators { current, total }
                    }
                }

                // Actions
                div {
                    class: "ant-tour-footer",
                    if let Some(render_fn) = actions_render {
                        {render_fn(default_actions, current, total)}
                    } else {
                        {default_actions}
                    }
                }
            }
        }
    }
}

/// Default indicators component
#[component]
fn DefaultIndicators(current: usize, total: usize) -> Element {
    rsx! {
        div {
            class: "ant-tour-indicators",
            for i in 0..total {
                span {
                    key: "{i}",
                    class: if i == current { "ant-tour-indicator ant-tour-indicator-active" } else { "ant-tour-indicator" },
                }
            }
        }
    }
}

/// Tour hook for programmatic control
pub struct TourController {
    pub open: Signal<bool>,
    pub current: Signal<usize>,
}

impl TourController {
    pub fn new() -> Self {
        Self {
            open: use_signal(|| false),
            current: use_signal(|| 0),
        }
    }

    pub fn start(&mut self) {
        self.current.set(0);
        self.open.set(true);
    }

    pub fn close(&mut self) {
        self.open.set(false);
    }

    pub fn next(&mut self, total_steps: usize) {
        let current = *self.current.read();
        if current < total_steps - 1 {
            self.current.set(current + 1);
        } else {
            self.close();
        }
    }

    pub fn prev(&mut self) {
        let current = *self.current.read();
        if current > 0 {
            self.current.set(current - 1);
        }
    }

    pub fn go_to(&mut self, step: usize, total_steps: usize) {
        if step < total_steps {
            self.current.set(step);
        }
    }
}

/// Hook to create tour controller
pub fn use_tour() -> TourController {
    TourController::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tour_placement_as_str() {
        assert_eq!(TourPlacement::Center.as_str(), "center");
        assert_eq!(TourPlacement::Bottom.as_str(), "bottom");
        assert_eq!(TourPlacement::TopLeft.as_str(), "topLeft");
    }

    #[test]
    fn test_tour_type_as_str() {
        assert_eq!(TourType::Default.as_str(), "default");
        assert_eq!(TourType::Primary.as_str(), "primary");
    }

    #[test]
    fn test_arrow_config_default() {
        let arrow = ArrowConfig::default();
        assert_eq!(arrow.show, true);
        assert_eq!(arrow.point_at_center, false);
    }

    #[test]
    fn test_gap_config_default() {
        let gap = GapConfig::default();
        assert_eq!(gap.offset, Some((6, 6)));
        assert_eq!(gap.radius, Some(2));
    }

    // #[test]
    // fn test_tour_controller() {
    //     let controller = TourController::new();
    //     assert_eq!(*controller.open.read(), false);
    //     assert_eq!(*controller.current.read(), 0);

    //     controller.start();
    //     assert_eq!(*controller.open.read(), true);
    //     assert_eq!(*controller.current.read(), 0);

    //     controller.next(3);
    //     assert_eq!(*controller.current.read(), 1);

    //     controller.prev();
    //     assert_eq!(*controller.current.read(), 0);

    //     controller.close();
    //     assert_eq!(*controller.open.read(), false);
    // }
}

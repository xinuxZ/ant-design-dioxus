use dioxus::prelude::*;

/// Color format types
#[derive(Clone, Debug, PartialEq)]
pub enum ColorFormat {
    Hex,
    Rgb,
    Hsb,
}

impl Default for ColorFormat {
    fn default() -> Self {
        Self::Hex
    }
}

impl ColorFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            ColorFormat::Hex => "hex",
            ColorFormat::Rgb => "rgb",
            ColorFormat::Hsb => "hsb",
        }
    }
}

/// Color picker size
#[derive(Clone, Debug, PartialEq)]
pub enum ColorPickerSize {
    Small,
    Default,
    Large,
}

impl Default for ColorPickerSize {
    fn default() -> Self {
        Self::Default
    }
}

impl ColorPickerSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            ColorPickerSize::Small => "small",
            ColorPickerSize::Default => "default",
            ColorPickerSize::Large => "large",
        }
    }
}

/// Color picker trigger mode
#[derive(Clone, Debug, PartialEq)]
pub enum ColorPickerTrigger {
    Click,
    Hover,
}

impl Default for ColorPickerTrigger {
    fn default() -> Self {
        Self::Click
    }
}

/// Color picker placement
#[derive(Clone, Debug, PartialEq)]
pub enum ColorPickerPlacement {
    Top,
    TopLeft,
    TopRight,
    Bottom,
    BottomLeft,
    BottomRight,
}

impl Default for ColorPickerPlacement {
    fn default() -> Self {
        Self::BottomLeft
    }
}

/// Color picker mode
#[derive(Clone, Debug, PartialEq)]
pub enum ColorPickerMode {
    Single,
    Gradient,
}

impl Default for ColorPickerMode {
    fn default() -> Self {
        Self::Single
    }
}

/// Color value representation
#[derive(Clone, Debug, PartialEq)]
pub struct ColorValue {
    /// Hex value (e.g., "#1677ff")
    pub hex: String,
    /// RGB values
    pub rgb: (u8, u8, u8),
    /// HSB values (hue: 0-360, saturation: 0-100, brightness: 0-100)
    pub hsb: (u16, u8, u8),
    /// Alpha value (0.0-1.0)
    pub alpha: f32,
}

impl Default for ColorValue {
    fn default() -> Self {
        Self {
            hex: "#1677ff".to_string(),
            rgb: (22, 119, 255),
            hsb: (215, 91, 100),
            alpha: 1.0,
        }
    }
}

impl ColorValue {
    /// Create from hex string
    pub fn from_hex(hex: &str) -> Self {
        let hex = hex.trim_start_matches('#');
        if hex.len() == 6 {
            if let (Ok(r), Ok(g), Ok(b)) = (
                u8::from_str_radix(&hex[0..2], 16),
                u8::from_str_radix(&hex[2..4], 16),
                u8::from_str_radix(&hex[4..6], 16),
            ) {
                let hsb = rgb_to_hsb(r, g, b);
                return Self {
                    hex: format!("#{}", hex.to_uppercase()),
                    rgb: (r, g, b),
                    hsb,
                    alpha: 1.0,
                };
            }
        }
        Self::default()
    }

    /// Create from RGB values
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        let hex = format!("#{:02X}{:02X}{:02X}", r, g, b);
        let hsb = rgb_to_hsb(r, g, b);
        Self {
            hex,
            rgb: (r, g, b),
            hsb,
            alpha: 1.0,
        }
    }

    /// Get formatted string based on format
    pub fn to_string(&self, format: &ColorFormat) -> String {
        match format {
            ColorFormat::Hex => self.hex.clone(),
            ColorFormat::Rgb => format!("rgb({}, {}, {})", self.rgb.0, self.rgb.1, self.rgb.2),
            ColorFormat::Hsb => format!("hsb({}, {}%, {}%)", self.hsb.0, self.hsb.1, self.hsb.2),
        }
    }
}

/// Preset color group
#[derive(Clone, Debug, PartialEq)]
pub struct ColorPreset {
    /// Label for the preset group
    pub label: String,
    /// Colors in the preset
    pub colors: Vec<String>,
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
            show: false,
            point_at_center: false,
        }
    }
}

/// Color picker properties
#[derive(Props, Clone, PartialEq)]
pub struct ColorPickerProps {
    /// Current color value
    #[props(default)]
    pub value: Option<ColorValue>,

    /// Default color value
    #[props(default)]
    pub default_value: Option<ColorValue>,

    /// Color format
    #[props(default)]
    pub format: ColorFormat,

    /// Size of the trigger
    #[props(default)]
    pub size: ColorPickerSize,

    /// Trigger mode
    #[props(default)]
    pub trigger: ColorPickerTrigger,

    /// Placement of the popup
    #[props(default)]
    pub placement: ColorPickerPlacement,

    /// Color picker mode
    #[props(default)]
    pub mode: ColorPickerMode,

    /// Whether to allow clearing
    #[props(default)]
    pub allow_clear: bool,

    /// Whether to show text
    #[props(default)]
    pub show_text: bool,

    /// Whether disabled
    #[props(default)]
    pub disabled: bool,

    /// Whether to disable alpha
    #[props(default)]
    pub disable_alpha: bool,

    /// Whether popup is open
    #[props(default)]
    pub open: Option<bool>,

    /// Arrow configuration
    #[props(default)]
    pub arrow: ArrowConfig,

    /// Preset colors
    #[props(default)]
    pub presets: Vec<ColorPreset>,

    /// Custom trigger element
    pub children: Option<Element>,

    /// Callback when color changes
    #[props(default)]
    pub on_change: Option<EventHandler<ColorValue>>,

    /// Callback when color change is complete
    #[props(default)]
    pub on_change_complete: Option<EventHandler<ColorValue>>,

    /// Callback when format changes
    #[props(default)]
    pub on_format_change: Option<EventHandler<ColorFormat>>,

    /// Callback when open state changes
    #[props(default)]
    pub on_open_change: Option<EventHandler<bool>>,

    /// Callback when clear is clicked
    #[props(default)]
    pub on_clear: Option<EventHandler<()>>,

    /// CSS class name
    #[props(default)]
    pub class: Option<String>,

    /// Inline style
    #[props(default)]
    pub style: Option<String>,

    /// Element ID
    #[props(default)]
    pub id: Option<String>,
}

/// Color picker component
#[component]
pub fn ColorPicker(props: ColorPickerProps) -> Element {
    let mut is_open = use_signal(|| props.open.unwrap_or(false));
    let mut current_color = use_signal(|| {
        props
            .value
            .clone()
            .or_else(|| props.default_value.clone())
            .unwrap_or_default()
    });
    let mut current_format = use_signal(|| props.format.clone());
    let _is_dragging = use_signal(|| false);

    let class_name = format!(
        "ant-color-picker ant-color-picker-{} {} {}",
        props.size.as_str(),
        if props.disabled {
            "ant-color-picker-disabled"
        } else {
            ""
        },
        props.class.as_deref().unwrap_or("")
    )
    .trim()
    .to_string();

    let trigger_style = format!(
        "background-color: {}; {}",
        current_color.read().hex,
        props.style.as_deref().unwrap_or("")
    );

    let handle_trigger_click = move |_evt: MouseEvent| {
        if !props.disabled {
            let new_open = !*is_open.read();
            is_open.set(new_open);
            if let Some(on_open_change) = &props.on_open_change {
                on_open_change.call(new_open);
            }
        }
    };

    let mut handle_color_change = move |new_color: ColorValue| {
        current_color.set(new_color.clone());
        if let Some(on_change) = &props.on_change {
            on_change.call(new_color);
        }
    };

    let handle_color_change_complete = move |new_color: ColorValue| {
        if let Some(on_change_complete) = &props.on_change_complete {
            on_change_complete.call(new_color);
        }
    };

    let handle_format_change = move |new_format: ColorFormat| {
        current_format.set(new_format.clone());
        if let Some(on_format_change) = &props.on_format_change {
            on_format_change.call(new_format);
        }
    };

    let handle_clear = move |_evt: MouseEvent| {
        _evt.stop_propagation();
        if let Some(on_clear) = &props.on_clear {
            on_clear.call(());
        }
    };

    let mut handle_preset_click = move |color: String| {
        let new_color = ColorValue::from_hex(&color);
        handle_color_change(new_color.clone());
        handle_color_change_complete(new_color);
    };

    let presets = props.presets.clone();

    rsx! {
        div {
            class: "{class_name}",
            id: props.id,

            // Custom trigger or default trigger
            if let Some(children) = props.children {
                div {
                    class: "ant-color-picker-trigger",
                    onclick: handle_trigger_click,
                    {children}
                }
            } else {
                div {
                    class: "ant-color-picker-trigger",
                    style: "{trigger_style}",
                    onclick: handle_trigger_click,

                    div {
                        class: "ant-color-picker-color-block",
                        style: "background-color: {current_color.read().hex};"
                    }

                    if props.show_text {
                        span {
                            class: "ant-color-picker-text",
                            {current_color.read().to_string(&current_format.read())}
                        }
                    }

                    if props.allow_clear {
                        button {
                            class: "ant-color-picker-clear",
                            onclick: handle_clear,
                            "×"
                        }
                    }
                }
            }

            // Color picker popup
            if *is_open.read() {
                div {
                    class: "ant-color-picker-popup",

                    // Color panel
                    ColorPanel {
                        value: current_color.read().clone(),
                        format: current_format.read().clone(),
                        disable_alpha: props.disable_alpha,
                        mode: props.mode.clone(),
                        on_change: handle_color_change,
                        on_change_complete: handle_color_change_complete,
                        on_format_change: handle_format_change,
                    }

                    // Presets
                    if !presets.is_empty() {
                        div {
                            class: "ant-color-picker-presets",
                            for preset in &presets {
                                div {
                                    class: "ant-color-picker-preset-group",

                                    div {
                                        class: "ant-color-picker-preset-label",
                                        "{preset.label}"
                                    }

                                    div {
                                        class: "ant-color-picker-preset-colors",
                                        for color in preset.colors.iter() {
                                            div {
                                                class: "ant-color-picker-preset-color",
                                                style: "background-color: {color};",
                                                onclick: {
                                                    let color = color.clone();
                                                    move |_| handle_preset_click(color.clone())
                                                },
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
}

/// Color panel component for the popup
#[component]
fn ColorPanel(
    value: ColorValue,
    format: ColorFormat,
    disable_alpha: bool,
    mode: ColorPickerMode,
    on_change: EventHandler<ColorValue>,
    on_change_complete: EventHandler<ColorValue>,
    on_format_change: EventHandler<ColorFormat>,
) -> Element {
    let mut saturation_brightness =
        use_signal(|| (value.hsb.1 as f32 / 100.0, value.hsb.2 as f32 / 100.0));
    let mut hue = use_signal(|| value.hsb.0 as f32);
    let mut alpha = use_signal(|| value.alpha);

    let handle_saturation_brightness_change = move |_evt: MouseEvent| {
        // Calculate saturation and brightness based on mouse position
        // This is a simplified implementation
        // TODO: Use onmounted to get element reference for real coordinates
        let x: f32 = 0.5;
        let y: f32 = 0.5;

        let saturation: f32 = x.clamp(0.0, 1.0);
        let brightness: f32 = y.clamp(0.0, 1.0);

        saturation_brightness.set((saturation, brightness));

        let new_color = hsb_to_color_value(
            *hue.read() as u16,
            (saturation * 100.0) as u8,
            (brightness * 100.0) as u8,
            *alpha.read(),
        );

        on_change.call(new_color);
    };

    let handle_hue_change = move |new_hue: f32| {
        hue.set(new_hue);
        let (s, b) = *saturation_brightness.read();
        let new_color = hsb_to_color_value(
            new_hue as u16,
            (s * 100.0) as u8,
            (b * 100.0) as u8,
            *alpha.read(),
        );
        on_change.call(new_color);
    };

    let handle_alpha_change = move |new_alpha: f32| {
        alpha.set(new_alpha);
        let (s, b) = *saturation_brightness.read();
        let new_color = hsb_to_color_value(
            *hue.read() as u16,
            (s * 100.0) as u8,
            (b * 100.0) as u8,
            new_alpha,
        );
        on_change.call(new_color);
    };

    rsx! {
        div {
            class: "ant-color-picker-panel",

            // Saturation/Brightness picker
            div {
                class: "ant-color-picker-saturation",
                style: "background-color: hsl({hue.read()}, 100%, 50%);",
                onclick: handle_saturation_brightness_change,

                div {
                    class: "ant-color-picker-saturation-cursor",
                    style: "left: {saturation_brightness.read().0 * 100.0}%; top: {(1.0 - saturation_brightness.read().1) * 100.0}%;"
                }
            }

            // Hue slider
            HueSlider {
                value: *hue.read(),
                on_change: handle_hue_change,
            }

            // Alpha slider
            if !disable_alpha {
                AlphaSlider {
                    value: *alpha.read(),
                    color: value.clone(),
                    on_change: handle_alpha_change,
                }
            }

            // Format inputs
            ColorInputs {
                value: value.clone(),
                format: format.clone(),
                on_change: on_change,
                on_format_change: on_format_change,
            }
        }
    }
}

/// Hue slider component
#[component]
fn HueSlider(value: f32, on_change: EventHandler<f32>) -> Element {
    let handle_click = move |_evt: MouseEvent| {
        // TODO: Use onmounted to get element reference for real coordinates
        let x: f32 = 0.5;
        let hue: f32 = (x * 360.0).clamp(0.0, 360.0);

        on_change.call(hue);
    };

    rsx! {
        div {
            class: "ant-color-picker-hue",
            onclick: handle_click,

            div {
                class: "ant-color-picker-hue-cursor",
                style: "left: {value / 360.0 * 100.0}%;"
            }
        }
    }
}

/// Alpha slider component
#[component]
fn AlphaSlider(value: f32, color: ColorValue, on_change: EventHandler<f32>) -> Element {
    let handle_click = move |_evt: MouseEvent| {
        // TODO: Use onmounted to get element reference for proper coordinate calculation
        // Simplified: use fixed coordinates for now
        let x: f32 = 0.5;
        let alpha: f32 = x.clamp(0.0, 1.0);
        on_change.call(alpha);
    };

    let gradient_style = format!(
        "background: linear-gradient(to right, transparent, {});",
        color.hex
    );

    rsx! {
        div {
            class: "ant-color-picker-alpha",
            onclick: handle_click,

            div {
                class: "ant-color-picker-alpha-bg",
                style: "{gradient_style}"
            }

            div {
                class: "ant-color-picker-alpha-cursor",
                style: "left: {value * 100.0}%;"
            }
        }
    }
}

/// Color inputs component
#[component]
fn ColorInputs(
    value: ColorValue,
    format: ColorFormat,
    on_change: EventHandler<ColorValue>,
    on_format_change: EventHandler<ColorFormat>,
) -> Element {
    let handle_format_change = move |new_format: ColorFormat| {
        on_format_change.call(new_format);
    };

    rsx! {
        div {
            class: "ant-color-picker-inputs",

            // Format selector
            select {
                class: "ant-color-picker-format-select",
                value: format.as_str(),
                onchange: move |evt| {
                    let format_str = evt.value();
                    let new_format = match format_str.as_str() {
                        "hex" => ColorFormat::Hex,
                        "rgb" => ColorFormat::Rgb,
                        "hsb" => ColorFormat::Hsb,
                        _ => ColorFormat::Hex,
                    };
                    handle_format_change(new_format);
                },

                option { value: "hex", "HEX" }
                option { value: "rgb", "RGB" }
                option { value: "hsb", "HSB" }
            }

            // Color value display
            div {
                class: "ant-color-picker-input-container",
                input {
                    class: "ant-color-picker-input",
                    value: value.to_string(&format),
                    readonly: true,
                }
            }
        }
    }
}

/// Helper function to convert RGB to HSB
fn rgb_to_hsb(r: u8, g: u8, b: u8) -> (u16, u8, u8) {
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    let hue = if delta == 0.0 {
        0.0
    } else if max == r {
        60.0 * (((g - b) / delta) % 6.0)
    } else if max == g {
        60.0 * ((b - r) / delta + 2.0)
    } else {
        60.0 * ((r - g) / delta + 4.0)
    };

    let saturation = if max == 0.0 { 0.0 } else { delta / max };
    let brightness = max;

    (
        (hue.abs() % 360.0) as u16,
        (saturation * 100.0) as u8,
        (brightness * 100.0) as u8,
    )
}

/// Helper function to convert HSB to ColorValue
fn hsb_to_color_value(h: u16, s: u8, b: u8, a: f32) -> ColorValue {
    let h = h as f32 / 360.0;
    let s = s as f32 / 100.0;
    let b = b as f32 / 100.0;

    let c = b * s;
    let x = c * (1.0 - ((h * 6.0) % 2.0 - 1.0).abs());
    let m = b - c;

    let (r, g, b) = if h < 1.0 / 6.0 {
        (c, x, 0.0)
    } else if h < 2.0 / 6.0 {
        (x, c, 0.0)
    } else if h < 3.0 / 6.0 {
        (0.0, c, x)
    } else if h < 4.0 / 6.0 {
        (0.0, x, c)
    } else if h < 5.0 / 6.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    let r = ((r + m) * 255.0) as u8;
    let g = ((g + m) * 255.0) as u8;
    let b = ((b + m) * 255.0) as u8;

    ColorValue {
        hex: format!("#{:02X}{:02X}{:02X}", r, g, b),
        rgb: (r, g, b),
        hsb: (h as u16 * 360, s as u8 * 100, b as u8 * 100),
        alpha: a,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_value_from_hex() {
        let color = ColorValue::from_hex("#1677ff");
        assert_eq!(color.hex, "#1677FF");
        assert_eq!(color.rgb, (22, 119, 255));
    }

    #[test]
    fn test_color_value_from_rgb() {
        let color = ColorValue::from_rgb(22, 119, 255);
        assert_eq!(color.rgb, (22, 119, 255));
        assert_eq!(color.hex, "#1677FF");
    }

    #[test]
    fn test_color_format_as_str() {
        assert_eq!(ColorFormat::Hex.as_str(), "hex");
        assert_eq!(ColorFormat::Rgb.as_str(), "rgb");
        assert_eq!(ColorFormat::Hsb.as_str(), "hsb");
    }

    #[test]
    fn test_rgb_to_hsb() {
        let hsb = rgb_to_hsb(255, 0, 0); // Pure red
        assert_eq!(hsb.0, 0); // Hue should be 0 for red
        assert_eq!(hsb.1, 100); // Saturation should be 100%
        assert_eq!(hsb.2, 100); // Brightness should be 100%
    }
}

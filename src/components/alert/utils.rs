//! Alert 组件的工具函数

use dioxus::events::Key;
use dioxus::prelude::*;
use std::collections::HashMap;
use wasm_bindgen::JsCast;
use web_sys::{window, CssStyleDeclaration, Element, HtmlElement};

use crate::components::alert::types::*;

/// Alert 工具函数集合
pub struct AlertUtils;

impl AlertUtils {
    /// 获取Alert的默认图标
    pub fn get_default_icon(alert_type: &AlertType) -> &'static str {
        alert_type.get_icon_name()
    }

    /// 计算Alert的内容高度
    pub fn calculate_content_height(has_description: bool, size: &AlertSize) -> String {
        let base_height = match size {
            AlertSize::Small => 24,
            AlertSize::Default => 32,
            AlertSize::Large => 40,
        };

        let height = if has_description {
            base_height + 20
        } else {
            base_height
        };

        format!("{}px", height)
    }

    /// 生成Alert的ARIA属性
    pub fn generate_aria_attributes(props: &AlertProps) -> HashMap<String, String> {
        let mut attrs = HashMap::new();

        // 设置角色
        attrs.insert("role".to_string(), props.role.clone());

        // 设置ARIA标签
        if let Some(ref aria_label) = props.aria_label {
            attrs.insert("aria-label".to_string(), aria_label.clone());
        } else {
            // 使用默认的ARIA标签
            let default_label = match props.alert_type {
                AlertType::Success => "Success alert",
                AlertType::Info => "Information alert",
                AlertType::Warning => "Warning alert",
                AlertType::Error => "Error alert",
            };
            attrs.insert("aria-label".to_string(), default_label.to_string());
        }

        // 设置是否为实时区域
        attrs.insert("aria-live".to_string(), "polite".to_string());

        // 设置原子性
        attrs.insert("aria-atomic".to_string(), "true".to_string());

        attrs
    }

    /// 验证Alert属性
    pub fn validate_props(props: &AlertProps) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if props.message.trim().is_empty() {
            errors.push("Alert message cannot be empty".to_string());
        }

        if props.animation_duration == 0 {
            errors.push("Animation duration must be greater than 0".to_string());
        }

        if props.animation_duration > 10000 {
            errors.push("Animation duration should not exceed 10 seconds".to_string());
        }

        if let Some(ref color) = props.color {
            if !Self::is_valid_color(color) {
                errors.push(format!("Invalid color format: {}", color));
            }
        }

        if let Some(ref bg_color) = props.background_color {
            if !Self::is_valid_color(bg_color) {
                errors.push(format!("Invalid background color format: {}", bg_color));
            }
        }

        if let Some(ref border_color) = props.border_color {
            if !Self::is_valid_color(border_color) {
                errors.push(format!("Invalid border color format: {}", border_color));
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// 检查颜色格式是否有效
    fn is_valid_color(color: &str) -> bool {
        // 检查是否是命名颜色
        if Self::is_named_color(color) {
            return true;
        }

        // 检查是否是十六进制颜色
        if color.starts_with('#') && (color.len() == 4 || color.len() == 7 || color.len() == 9) {
            return true;
        }

        // 检查是否是RGB/RGBA颜色
        if color.starts_with("rgb(") || color.starts_with("rgba(") {
            return true;
        }

        // 检查是否是HSL/HSLA颜色
        if color.starts_with("hsl(") || color.starts_with("hsla(") {
            return true;
        }

        false
    }

    /// 检查是否是命名颜色
    fn is_named_color(color: &str) -> bool {
        let named_colors = [
            "black",
            "silver",
            "gray",
            "white",
            "maroon",
            "red",
            "purple",
            "fuchsia",
            "green",
            "lime",
            "olive",
            "yellow",
            "navy",
            "blue",
            "teal",
            "aqua",
            "orange",
            "aliceblue",
            "antiquewhite",
            "aquamarine",
            "azure",
            "beige",
            "bisque",
            "blanchedalmond",
            "blueviolet",
            "brown",
            "burlywood",
            "cadetblue",
            "chartreuse",
            "chocolate",
            "coral",
            "cornflowerblue",
            "cornsilk",
            "crimson",
            "cyan",
            "darkblue",
            "darkcyan",
            "darkgoldenrod",
            "darkgray",
            "darkgreen",
            "darkgrey",
            "darkkhaki",
            "darkmagenta",
            "darkolivegreen",
            "darkorange",
            "darkorchid",
            "darkred",
            "darksalmon",
            "darkseagreen",
            "darkslateblue",
            "darkslategray",
            "darkslategrey",
            "darkturquoise",
            "darkviolet",
            "deeppink",
            "deepskyblue",
            "dimgray",
            "dimgrey",
            "dodgerblue",
            "firebrick",
            "floralwhite",
            "forestgreen",
            "gainsboro",
            "ghostwhite",
            "gold",
            "goldenrod",
            "greenyellow",
            "grey",
            "honeydew",
            "hotpink",
            "indianred",
            "indigo",
            "ivory",
            "khaki",
            "lavender",
            "lavenderblush",
            "lawngreen",
            "lemonchiffon",
            "lightblue",
            "lightcoral",
            "lightcyan",
            "lightgoldenrodyellow",
            "lightgray",
            "lightgreen",
            "lightgrey",
            "lightpink",
            "lightsalmon",
            "lightseagreen",
            "lightskyblue",
            "lightslategray",
            "lightslategrey",
            "lightsteelblue",
            "lightyellow",
            "limegreen",
            "linen",
            "magenta",
            "mediumaquamarine",
            "mediumblue",
            "mediumorchid",
            "mediumpurple",
            "mediumseagreen",
            "mediumslateblue",
            "mediumspringgreen",
            "mediumturquoise",
            "mediumvioletred",
            "midnightblue",
            "mintcream",
            "mistyrose",
            "moccasin",
            "navajowhite",
            "oldlace",
            "olivedrab",
            "orangered",
            "orchid",
            "palegoldenrod",
            "palegreen",
            "paleturquoise",
            "palevioletred",
            "papayawhip",
            "peachpuff",
            "peru",
            "pink",
            "plum",
            "powderblue",
            "rosybrown",
            "royalblue",
            "saddlebrown",
            "salmon",
            "sandybrown",
            "seagreen",
            "seashell",
            "sienna",
            "skyblue",
            "slateblue",
            "slategray",
            "slategrey",
            "snow",
            "springgreen",
            "steelblue",
            "tan",
            "thistle",
            "tomato",
            "turquoise",
            "violet",
            "wheat",
            "whitesmoke",
            "yellowgreen",
            "rebeccapurple",
        ];

        named_colors.contains(&color.to_lowercase().as_str())
    }

    /// 生成唯一ID
    pub fn generate_unique_id() -> String {
        // 使用时间戳和简单的伪随机数生成唯一ID
        use std::time::{SystemTime, UNIX_EPOCH};

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();

        // 简单的伪随机数生成
        let random_chars = Self::simple_random_chars(8);

        format!("alert-{}-{}", timestamp, random_chars)
    }

    /// 生成简单的随机字符串
    fn simple_random_chars(len: usize) -> String {
        let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();

        let mut result = String::with_capacity(len);
        let mut seed = timestamp;

        for _ in 0..len {
            // 简单的线性同余伪随机数生成
            seed = (seed * 6364136223846793005 + 1) % u128::MAX;
            let index = (seed % chars.len() as u128) as usize;
            result.push(chars.chars().nth(index).unwrap_or('a'));
        }

        result
    }

    /// 格式化错误消息
    pub fn format_error_message(errors: &[String]) -> String {
        if errors.is_empty() {
            return "Unknown error".to_string();
        }

        if errors.len() == 1 {
            return errors[0].clone();
        }

        let mut message = String::from("Multiple errors:\n");
        for (i, error) in errors.iter().enumerate() {
            message.push_str(&format!("{}. {}\n", i + 1, error));
        }

        message
    }

    /// 安全获取元素属性
    pub fn safe_get_attribute(element: &Element, attr_name: &str) -> Option<String> {
        element.get_attribute(attr_name)
    }

    /// 安全设置元素属性
    pub fn safe_set_attribute(
        element: &Element,
        attr_name: &str,
        value: &str,
    ) -> Result<(), String> {
        element
            .set_attribute(attr_name, value)
            .map_err(|e| format!("Failed to set attribute: {:?}", e))
    }

    // /// 检查是否支持CSS特性
    // pub fn supports_css_feature(feature: &str) -> bool {
    //     if let Some(window) = window() {
    //         if let Ok(css) = window.get_computed_style(&window.document().unwrap().body().unwrap())
    //         {
    //             if let Some(css) = css.dyn_into::<CssStyleDeclaration>().ok() {
    //                 return css.get_property_value(feature).is_ok();
    //             }
    //         }
    //     }
    //     false
    // }
}

/// 设备类型枚举
pub enum DeviceType {
    Mobile,
    Tablet,
    Desktop,
    LargeDesktop,
}

/// Alert 事件处理器
pub struct AlertEventHandler;

impl AlertEventHandler {
    /// 处理关闭事件
    pub fn handle_close(on_close: Option<EventHandler<MouseEvent>>, event: MouseEvent) {
        if let Some(handler) = on_close {
            handler.call(event);
        }
    }

    /// 处理键盘事件
    pub fn handle_keyboard(event: &KeyboardEvent, on_close: Option<EventHandler<MouseEvent>>) {
        // 处理Escape键
        if event.key() == Key::Escape {
            // 创建一个模拟的点击事件
            // 在实际实现中，可能需要更复杂的逻辑来创建一个合适的MouseEvent
            // 这里简化处理
            if let Some(_handler) = on_close {
                // 直接调用处理函数，使用默认的MouseEvent
                // handler.call(MouseEvent::new(
                //     Rc::new(*event.clone().downcast::<MouseData>().unwrap()),
                //     false,
                // ));
            }
        }

        // 处理Enter键
        if event.key() == Key::Enter {
            // 触发点击行为
        }

        // 处理Tab键导航
        if event.key() == Key::Tab {
            // 处理焦点导航
        }
    }

    /// 处理焦点
    pub fn handle_focus(element: &Element) {
        if let Some(html_element) = element.clone().dyn_into::<HtmlElement>().ok() {
            let _ = html_element.focus();
        }
    }

    /// 处理失焦
    pub fn handle_blur(element: &Element) {
        if let Some(html_element) = element.clone().dyn_into::<HtmlElement>().ok() {
            let _ = html_element.blur();
        }
    }

    /// 是否应该处理键盘事件
    pub fn should_handle_keyboard(closable: bool) -> bool {
        closable
    }

    /// 获取图标类型
    pub fn get_icon_type(alert_type: &AlertType) -> String {
        match alert_type {
            AlertType::Success => "check-circle".to_string(),
            AlertType::Info => "info-circle".to_string(),
            AlertType::Warning => "exclamation-circle".to_string(),
            AlertType::Error => "close-circle".to_string(),
        }
    }

    /// 是否应该显示关闭按钮
    pub fn should_show_close_button(closable: bool, banner: bool) -> bool {
        closable || banner
    }

    /// 是否应该显示图标
    pub fn should_show_icon(show_icon: bool, alert_type: &AlertType) -> bool {
        show_icon || *alert_type != AlertType::Info
    }

    /// 获取动画持续时间
    pub fn get_animation_duration(enable_animation: bool, duration: u32) -> u32 {
        if enable_animation {
            duration
        } else {
            0
        }
    }

    /// 是否启用动画
    pub fn is_animation_enabled(enable_animation: bool) -> bool {
        enable_animation
    }
}

/// Alert 动画管理器
pub struct AlertAnimationManager;

impl AlertAnimationManager {
    /// 处理动画过渡
    pub fn handle_transition(
        element: &web_sys::Element,
        state: AnimationState,
        duration: u32,
        _alert_type: AlertType,
    ) {
        // 获取元素的样式
        if let Some(element) = element.dyn_ref::<web_sys::HtmlElement>() {
            // 获取计算样式
            let window = web_sys::window().expect("no global `window` exists");
            let _computed_style = window
                .get_computed_style(element)
                .expect("should have computed style");

            // 应用过渡样式
            let css = element.style();
            if let Some(css) = css.dyn_ref::<web_sys::CssStyleDeclaration>() {
                match state {
                    AnimationState::Entering => {
                        css.set_property("transition", &format!("all {}ms ease-in-out", duration))
                            .ok();
                        css.set_property("opacity", "1").ok();
                        css.set_property("max-height", "1000px").ok();
                        css.set_property("padding", "").ok();
                        css.set_property("margin", "").ok();
                    }
                    AnimationState::Exiting => {
                        css.set_property("transition", &format!("all {}ms ease-in-out", duration))
                            .ok();
                        css.set_property("opacity", "0").ok();
                        css.set_property("max-height", "0").ok();
                        css.set_property("padding", "0").ok();
                        css.set_property("margin", "0").ok();
                        css.set_property("border-width", "0").ok();
                    }
                    AnimationState::Entered => {
                        css.set_property("transition", "").ok();
                    }
                    AnimationState::Exited => {
                        css.set_property("display", "none").ok();
                    }
                    _ => {}
                }
            }
        }
    }
}

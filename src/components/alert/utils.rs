//! Alert 组件的工具函数

use dioxus::prelude::*;
use std::collections::HashMap;
use wasm_bindgen::JsCast;
use web_sys::{window, Element, HtmlElement};

use crate::components::alert::types::*;

/// Alert 工具函数集合
pub struct AlertUtils;

impl AlertUtils {
    /// 生成Alert的CSS类名
    pub fn generate_class_names(props: &AlertProps, state: &AlertState) -> String {
        let mut classes = vec!["ant-alert"];

        // 添加类型相关的类名
        classes.push(props.alert_type.get_css_class());

        // 添加尺寸相关的类名
        match props.size {
            AlertSize::Small => classes.push("ant-alert-small"),
            AlertSize::Default => {} // 默认尺寸不需要额外类名
            AlertSize::Large => classes.push("ant-alert-large"),
        }

        // 添加状态相关的类名
        if props.closable {
            classes.push("ant-alert-closable");
        }

        if props.show_icon {
            classes.push("ant-alert-with-icon");
        }

        if props.description.is_some() {
            classes.push("ant-alert-with-description");
        }

        if props.banner {
            classes.push("ant-alert-banner");
        }

        if !props.show_border {
            classes.push("ant-alert-no-border");
        }

        if !state.visible {
            classes.push("ant-alert-hidden");
        }

        if state.closing {
            classes.push("ant-alert-closing");
        }

        // 添加动画状态类名
        match state.animation_state {
            AnimationState::Entering => classes.push("ant-alert-entering"),
            AnimationState::Entered => classes.push("ant-alert-entered"),
            AnimationState::Exiting => classes.push("ant-alert-exiting"),
            AnimationState::Exited => classes.push("ant-alert-exited"),
            _ => {}
        }

        // 添加自定义类名
        if let Some(ref custom_class) = props.class_name {
            classes.push(custom_class);
        }

        classes.join(" ")
    }

    /// 生成Alert的内联样式
    pub fn generate_inline_styles(props: &AlertProps) -> String {
        let mut styles = Vec::new();

        // 添加自定义颜色
        if let Some(ref color) = props.color {
            styles.push(format!("color: {}", color));
        }

        // 添加自定义背景色
        if let Some(ref bg_color) = props.background_color {
            styles.push(format!("background-color: {}", bg_color));
        }

        // 添加自定义边框颜色
        if let Some(ref border_color) = props.border_color {
            styles.push(format!("border-color: {}", border_color));
        }

        // 添加自定义圆角
        if let Some(ref border_radius) = props.border_radius {
            styles.push(format!("border-radius: {}", border_radius));
        }

        // 添加动画持续时间
        if props.enable_animation {
            styles.push(format!(
                "transition-duration: {}ms",
                props.animation_duration
            ));
        }

        // 添加自定义样式
        if let Some(ref custom_style) = props.style {
            styles.push(custom_style.clone());
        }

        styles.join("; ")
    }

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

    /// 验证颜色格式
    fn is_valid_color(color: &str) -> bool {
        // 简单的颜色格式验证
        color.starts_with('#') && color.len() >= 4
            || color.starts_with("rgb")
            || color.starts_with("rgba")
            || color.starts_with("hsl")
            || color.starts_with("hsla")
            || Self::is_named_color(color)
    }

    /// 检查是否为命名颜色
    fn is_named_color(color: &str) -> bool {
        matches!(
            color.to_lowercase().as_str(),
            "red"
                | "green"
                | "blue"
                | "yellow"
                | "orange"
                | "purple"
                | "pink"
                | "brown"
                | "black"
                | "white"
                | "gray"
                | "grey"
                | "transparent"
                | "inherit"
                | "initial"
                | "unset"
                | "currentcolor"
        )
    }

    /// 创建关闭动画
    pub fn create_close_animation(duration: u32) -> String {
        format!(
            "@keyframes ant-alert-close {{
                0% {{
                    opacity: 1;
                    transform: scaleY(1);
                    max-height: 200px;
                }}
                100% {{
                    opacity: 0;
                    transform: scaleY(0);
                    max-height: 0;
                    padding: 0;
                    margin: 0;
                }}
            }}
            .ant-alert-closing {{
                animation: ant-alert-close {}ms ease-out forwards;
            }}",
            duration
        )
    }

    /// 创建进入动画
    pub fn create_enter_animation(duration: u32) -> String {
        format!(
            "@keyframes ant-alert-enter {{
                0% {{
                    opacity: 0;
                    transform: scaleY(0);
                    max-height: 0;
                }}
                100% {{
                    opacity: 1;
                    transform: scaleY(1);
                    max-height: 200px;
                }}
            }}
            .ant-alert-entering {{
                animation: ant-alert-enter {}ms ease-out forwards;
            }}",
            duration
        )
    }

    /// 获取响应式断点
    pub fn get_responsive_breakpoints() -> HashMap<&'static str, u32> {
        let mut breakpoints = HashMap::new();
        breakpoints.insert("xs", 480);
        breakpoints.insert("sm", 576);
        breakpoints.insert("md", 768);
        breakpoints.insert("lg", 992);
        breakpoints.insert("xl", 1200);
        breakpoints.insert("xxl", 1600);
        breakpoints
    }

    /// 检查是否为移动设备
    pub fn is_mobile_device() -> bool {
        if let Some(window) = window() {
            if let Ok(width) = window.inner_width() {
                if let Some(width_num) = width.as_f64() {
                    return width_num < 768.0;
                }
            }
        }
        false
    }

    /// 获取设备类型
    pub fn get_device_type() -> DeviceType {
        if let Some(window) = window() {
            if let Ok(width) = window.inner_width() {
                if let Some(width_num) = width.as_f64() {
                    return match width_num as u32 {
                        0..=575 => DeviceType::Mobile,
                        576..=767 => DeviceType::Tablet,
                        768..=991 => DeviceType::Desktop,
                        _ => DeviceType::LargeDesktop,
                    };
                }
            }
        }
        DeviceType::Desktop
    }

    /// 生成唯一ID
    pub fn generate_unique_id() -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        format!("ant-alert-{}", timestamp)
    }

    /// 防抖函数
    pub fn debounce<F>(func: F, delay: u32) -> impl Fn()
    where
        F: Fn() + 'static,
    {
        use std::sync::{Arc, Mutex};
        use std::time::{Duration, Instant};

        let last_call = Arc::new(Mutex::new(None::<Instant>));
        let func = Arc::new(func);

        move || {
            let now = Instant::now();
            let mut last = last_call.lock().unwrap();

            if let Some(last_time) = *last {
                if now.duration_since(last_time) < Duration::from_millis(delay as u64) {
                    return;
                }
            }

            *last = Some(now);
            func();
        }
    }

    /// 节流函数
    pub fn throttle<F>(func: F, delay: u32) -> impl Fn()
    where
        F: Fn() + 'static,
    {
        use std::sync::{Arc, Mutex};
        use std::time::{Duration, Instant};

        let last_call = Arc::new(Mutex::new(None::<Instant>));
        let func = Arc::new(func);

        move || {
            let now = Instant::now();
            let mut last = last_call.lock().unwrap();

            if let Some(last_time) = *last {
                if now.duration_since(last_time) < Duration::from_millis(delay as u64) {
                    return;
                }
            }

            *last = Some(now);
            func();
        }
    }

    /// 深度合并配置
    pub fn deep_merge_config(base: AlertConfig, override_config: AlertConfig) -> AlertConfig {
        AlertConfig {
            enable_animation: override_config.enable_animation,
            animation_duration: override_config.animation_duration,
            default_show_icon: override_config.default_show_icon,
            default_closable: override_config.default_closable,
            default_theme: merge_alert_theme(&base.default_theme, &override_config.default_theme),
            default_size: override_config.default_size,
            enable_keyboard_navigation: override_config.enable_keyboard_navigation,
            enable_accessibility: override_config.enable_accessibility,
            css_prefix: if override_config.css_prefix.is_empty() {
                base.css_prefix
            } else {
                override_config.css_prefix
            },
            enable_rtl: override_config.enable_rtl,
        }
    }

    /// 格式化错误消息
    pub fn format_error_message(errors: &[String]) -> String {
        if errors.is_empty() {
            return String::new();
        }

        if errors.len() == 1 {
            return errors[0].clone();
        }

        format!("Multiple errors: {}", errors.join(", "))
    }

    /// 安全地获取元素属性
    pub fn safe_get_attribute(element: &Element, attr_name: &str) -> Option<String> {
        element.get_attribute(attr_name)
    }

    /// 安全地设置元素属性
    pub fn safe_set_attribute(
        element: &Element,
        attr_name: &str,
        value: &str,
    ) -> Result<(), String> {
        element
            .set_attribute(attr_name, value)
            .map_err(|e| format!("Failed to set attribute {}: {:?}", attr_name, e))
    }

    /// 检查是否支持CSS特性
    pub fn supports_css_feature(feature: &str) -> bool {
        if let Some(window) = window() {
            if let Some(css) = window.get("CSS") {
                if let Ok(supports_fn) = js_sys::Reflect::get(&css, &"supports".into()) {
                    if let Ok(result) = js_sys::Reflect::apply(
                        &supports_fn.into(),
                        &css,
                        &js_sys::Array::of1(&feature.into()),
                    ) {
                        return result.as_bool().unwrap_or(false);
                    }
                }
            }
        }
        false
    }
}

/// 设备类型枚举
#[derive(Clone, PartialEq, Debug)]
pub enum DeviceType {
    Mobile,
    Tablet,
    Desktop,
    LargeDesktop,
}

/// Alert事件处理器
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
        match event.key().as_str() {
            "Escape" => {
                if let Some(handler) = on_close {
                    // 创建一个模拟的鼠标事件
                    // 注意：这里需要根据实际情况调整
                    event.prevent_default();
                }
            }
            "Enter" | " " => {
                // 处理回车和空格键
                event.prevent_default();
            }
            _ => {}
        }
    }

    /// 处理焦点事件
    pub fn handle_focus(element: &Element) {
        if let Ok(html_element) = element.clone().dyn_into::<HtmlElement>() {
            let _ = html_element.focus();
        }
    }

    /// 处理失焦事件
    pub fn handle_blur(element: &Element) {
        if let Ok(html_element) = element.clone().dyn_into::<HtmlElement>() {
            html_element.blur().ok();
        }
    }
}

/// Alert动画管理器
pub struct AlertAnimationManager;

impl AlertAnimationManager {
    /// 开始进入动画
    pub fn start_enter_animation(element: &Element, duration: u32) {
        if let Ok(html_element) = element.dyn_into::<HtmlElement>() {
            let style = html_element.style();
            let _ = style.set_property(
                "animation",
                &format!("ant-alert-enter {}ms ease-out", duration),
            );
        }
    }

    /// 开始退出动画
    pub fn start_exit_animation(element: &Element, duration: u32) {
        if let Ok(html_element) = element.dyn_into::<HtmlElement>() {
            let style = html_element.style();
            let _ = style.set_property(
                "animation",
                &format!("ant-alert-close {}ms ease-out", duration),
            );
        }
    }

    /// 清除动画
    pub fn clear_animation(element: &Element) {
        if let Ok(html_element) = element.dyn_into::<HtmlElement>() {
            let style = html_element.style();
            let _ = style.remove_property("animation");
        }
    }
}

/// 便捷函数

/// 创建成功Alert
pub fn success_alert(message: &str) -> AlertProps {
    AlertBuilder::new()
        .message(message)
        .alert_type(AlertType::Success)
        .show_icon(true)
        .build()
}

/// 创建信息Alert
pub fn info_alert(message: &str) -> AlertProps {
    AlertBuilder::new()
        .message(message)
        .alert_type(AlertType::Info)
        .show_icon(true)
        .build()
}

/// 创建警告Alert
pub fn warning_alert(message: &str) -> AlertProps {
    AlertBuilder::new()
        .message(message)
        .alert_type(AlertType::Warning)
        .show_icon(true)
        .build()
}

/// 创建错误Alert
pub fn error_alert(message: &str) -> AlertProps {
    AlertBuilder::new()
        .message(message)
        .alert_type(AlertType::Error)
        .show_icon(true)
        .build()
}

/// 创建可关闭Alert
pub fn closable_alert(message: &str, alert_type: AlertType) -> AlertProps {
    AlertBuilder::new()
        .message(message)
        .alert_type(alert_type)
        .closable(true)
        .show_icon(true)
        .build()
}

/// 创建横幅Alert
pub fn banner_alert(message: &str, alert_type: AlertType) -> AlertProps {
    AlertBuilder::new()
        .message(message)
        .alert_type(alert_type)
        .banner(true)
        .show_icon(true)
        .build()
}

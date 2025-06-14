//! BackTop 组件工具函数
//!
//! 本模块提供了 BackTop 组件的各种工具函数，包括配置创建、
//! 属性验证、滚动计算、动画处理等。

use crate::components::back_top::types::*;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::{window, Element, HtmlElement};

/// 创建 BackTop 配置
pub fn create_back_top_config(
    visibility_height: Option<i32>,
    duration: Option<u32>,
    bottom: Option<i32>,
    right: Option<i32>,
    theme: Option<BackTopTheme>,
) -> BackTopConfig {
    BackTopConfig {
        visibility_height: visibility_height.unwrap_or(400),
        duration: duration.unwrap_or(450),
        bottom: bottom.unwrap_or(50),
        right: right.unwrap_or(50),
        has_custom_content: false,
        theme: theme.unwrap_or_default(),
        animation: BackTopAnimation::default(),
        easing: EasingFunction::default(),
        throttle: true,
        throttle_delay: 16,
        keyboard: true,
        show_on_mobile: true,
        mobile_config: None,
    }
}

/// 验证 BackTop 属性
pub fn validate_back_top_props(props: &BackTopProps) -> Result<(), String> {
    // 验证滚动高度阈值
    if props.visibility_height < 0 {
        return Err("visibility_height must be non-negative".to_string());
    }
    
    // 验证动画持续时间
    if props.duration == 0 {
        return Err("duration must be greater than 0".to_string());
    }
    
    // 验证位置参数
    if props.bottom < 0 || props.right < 0 {
        return Err("bottom and right must be non-negative".to_string());
    }
    
    // 验证节流延迟
    if props.throttle && props.throttle_delay == 0 {
        return Err("throttle_delay must be greater than 0 when throttle is enabled".to_string());
    }
    
    // 验证移动端配置
    if let (Some(mobile_bottom), Some(mobile_right)) = (props.mobile_bottom, props.mobile_right) {
        if mobile_bottom < 0 || mobile_right < 0 {
            return Err("mobile_bottom and mobile_right must be non-negative".to_string());
        }
    }
    
    Ok(())
}

/// 计算滚动位置
pub fn calculate_scroll_position(target: Option<&Element>) -> Result<f64, String> {
    if let Some(element) = target {
        Ok(element.scroll_top() as f64)
    } else if let Some(window) = window() {
        Ok(window.page_y_offset().unwrap_or(0.0))
    } else {
        Err("Cannot access window object".to_string())
    }
}

/// 获取滚动容器的高度
pub fn get_scroll_height(target: Option<&Element>) -> Result<f64, String> {
    if let Some(element) = target {
        Ok(element.scroll_height() as f64)
    } else if let Some(window) = window() {
        if let Some(document) = window.document() {
            if let Some(body) = document.body() {
                Ok(body.scroll_height() as f64)
            } else {
                Err("Cannot access document body".to_string())
            }
        } else {
            Err("Cannot access document".to_string())
        }
    } else {
        Err("Cannot access window object".to_string())
    }
}

/// 获取容器的客户端高度
pub fn get_client_height(target: Option<&Element>) -> Result<f64, String> {
    if let Some(element) = target {
        Ok(element.client_height() as f64)
    } else if let Some(window) = window() {
        Ok(window.inner_height().unwrap_or(JsValue::from(0)).as_f64().unwrap_or(0.0))
    } else {
        Err("Cannot access window object".to_string())
    }
}

/// 平滑滚动到顶部
pub fn smooth_scroll_to_top(
    target: Option<&Element>,
    duration: u32,
    easing: &EasingFunction,
) -> Result<(), String> {
    let start_position = calculate_scroll_position(target)?;
    
    if start_position <= 0.0 {
        return Ok(());
    }
    
    let start_time = js_sys::Date::now();
    let duration_ms = duration as f64;
    
    let target_clone = target.map(|e| e.clone());
    
    let closure = Closure::wrap(Box::new(move |_: f64| {
        let current_time = js_sys::Date::now();
        let elapsed = current_time - start_time;
        let progress = (elapsed / duration_ms).min(1.0);
        
        let eased_progress = apply_easing_function(progress, easing);
        let current_position = start_position * (1.0 - eased_progress);
        
        if let Some(element) = &target_clone {
            element.set_scroll_top(current_position as i32);
        } else if let Some(window) = window() {
            let _ = window.scroll_to_with_x_and_y(0.0, current_position);
        }
        
        if progress < 1.0 {
            if let Some(window) = window() {
                let _ = window.request_animation_frame(
                    closure.as_ref().unchecked_ref()
                );
            }
        }
    }) as Box<dyn FnMut(f64)>);
    
    if let Some(window) = window() {
        let _ = window.request_animation_frame(closure.as_ref().unchecked_ref());
        closure.forget();
    }
    
    Ok(())
}

/// 应用缓动函数
pub fn apply_easing_function(progress: f64, easing: &EasingFunction) -> f64 {
    match easing {
        EasingFunction::Linear => progress,
        EasingFunction::EaseIn => progress * progress,
        EasingFunction::EaseOut => 1.0 - (1.0 - progress) * (1.0 - progress),
        EasingFunction::EaseInOut => {
            if progress < 0.5 {
                2.0 * progress * progress
            } else {
                1.0 - 2.0 * (1.0 - progress) * (1.0 - progress)
            }
        }
        EasingFunction::EaseInQuart => progress.powi(4),
        EasingFunction::EaseOutQuart => 1.0 - (1.0 - progress).powi(4),
        EasingFunction::EaseInOutQuart => {
            if progress < 0.5 {
                8.0 * progress.powi(4)
            } else {
                1.0 - 8.0 * (1.0 - progress).powi(4)
            }
        }
        EasingFunction::EaseInCubic => progress.powi(3),
        EasingFunction::EaseOutCubic => 1.0 - (1.0 - progress).powi(3),
        EasingFunction::EaseInOutCubic => {
            if progress < 0.5 {
                4.0 * progress.powi(3)
            } else {
                1.0 - 4.0 * (1.0 - progress).powi(3)
            }
        }
        EasingFunction::CubicBezier(x1, y1, x2, y2) => {
            // 简化的三次贝塞尔曲线实现
            cubic_bezier_ease(progress, *x1, *y1, *x2, *y2)
        }
    }
}

/// 三次贝塞尔曲线缓动函数
fn cubic_bezier_ease(t: f64, x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    // 使用牛顿法求解三次贝塞尔曲线
    let mut x = t;
    for _ in 0..8 {
        let fx = cubic_bezier_x(x, x1, x2) - t;
        if fx.abs() < 1e-6 {
            break;
        }
        let dfx = cubic_bezier_dx(x, x1, x2);
        if dfx.abs() < 1e-6 {
            break;
        }
        x -= fx / dfx;
    }
    cubic_bezier_y(x, y1, y2)
}

fn cubic_bezier_x(t: f64, x1: f64, x2: f64) -> f64 {
    3.0 * (1.0 - t) * (1.0 - t) * t * x1 + 3.0 * (1.0 - t) * t * t * x2 + t * t * t
}

fn cubic_bezier_y(t: f64, y1: f64, y2: f64) -> f64 {
    3.0 * (1.0 - t) * (1.0 - t) * t * y1 + 3.0 * (1.0 - t) * t * t * y2 + t * t * t
}

fn cubic_bezier_dx(t: f64, x1: f64, x2: f64) -> f64 {
    3.0 * (1.0 - t) * (1.0 - t) * x1 + 6.0 * (1.0 - t) * t * (x2 - x1) + 3.0 * t * t * (1.0 - x2)
}

/// 检查是否为移动设备
pub fn is_mobile_device() -> bool {
    if let Some(window) = window() {
        if let Some(navigator) = window.navigator() {
            let user_agent = navigator.user_agent().unwrap_or_default();
            return user_agent.contains("Mobile") || 
                   user_agent.contains("Android") ||
                   user_agent.contains("iPhone") ||
                   user_agent.contains("iPad");
        }
    }
    false
}

/// 获取视口宽度
pub fn get_viewport_width() -> f64 {
    if let Some(window) = window() {
        window.inner_width()
            .unwrap_or(JsValue::from(0))
            .as_f64()
            .unwrap_or(0.0)
    } else {
        0.0
    }
}

/// 获取视口高度
pub fn get_viewport_height() -> f64 {
    if let Some(window) = window() {
        window.inner_height()
            .unwrap_or(JsValue::from(0))
            .as_f64()
            .unwrap_or(0.0)
    } else {
        0.0
    }
}

/// 判断是否应该显示 BackTop
pub fn should_show_back_top(scroll_top: f64, visibility_height: i32) -> bool {
    scroll_top >= visibility_height as f64
}

/// 计算滚动百分比
pub fn calculate_scroll_percentage(
    scroll_top: f64,
    scroll_height: f64,
    client_height: f64,
) -> f64 {
    if scroll_height <= client_height {
        return 0.0;
    }
    
    let max_scroll = scroll_height - client_height;
    (scroll_top / max_scroll * 100.0).min(100.0).max(0.0)
}

/// 生成 CSS 变量映射
pub fn generate_css_variables(theme: &BackTopTheme) -> HashMap<String, String> {
    let mut variables = HashMap::new();
    
    variables.insert("--back-top-bg-color".to_string(), theme.background_color.clone());
    variables.insert("--back-top-hover-bg-color".to_string(), theme.hover_background_color.clone());
    variables.insert("--back-top-icon-color".to_string(), theme.icon_color.clone());
    variables.insert("--back-top-hover-icon-color".to_string(), theme.hover_icon_color.clone());
    variables.insert("--back-top-border-radius".to_string(), theme.border_radius.clone());
    variables.insert("--back-top-box-shadow".to_string(), theme.box_shadow.clone());
    variables.insert("--back-top-hover-box-shadow".to_string(), theme.hover_box_shadow.clone());
    variables.insert("--back-top-transition".to_string(), theme.transition.clone());
    variables.insert("--back-top-z-index".to_string(), theme.z_index.to_string());
    variables.insert("--back-top-size".to_string(), theme.size.clone());
    variables.insert("--back-top-font-size".to_string(), theme.font_size.clone());
    variables.insert("--back-top-border".to_string(), theme.border.clone());
    variables.insert("--back-top-hover-border".to_string(), theme.hover_border.clone());
    variables.insert("--back-top-active-bg-color".to_string(), theme.active_background_color.clone());
    variables.insert("--back-top-disabled-bg-color".to_string(), theme.disabled_background_color.clone());
    variables.insert("--back-top-disabled-icon-color".to_string(), theme.disabled_icon_color.clone());
    
    variables
}

/// 合并主题配置
pub fn merge_theme(base: &BackTopTheme, override_theme: &BackTopTheme) -> BackTopTheme {
    BackTopTheme {
        background_color: if override_theme.background_color.is_empty() {
            base.background_color.clone()
        } else {
            override_theme.background_color.clone()
        },
        hover_background_color: if override_theme.hover_background_color.is_empty() {
            base.hover_background_color.clone()
        } else {
            override_theme.hover_background_color.clone()
        },
        icon_color: if override_theme.icon_color.is_empty() {
            base.icon_color.clone()
        } else {
            override_theme.icon_color.clone()
        },
        hover_icon_color: if override_theme.hover_icon_color.is_empty() {
            base.hover_icon_color.clone()
        } else {
            override_theme.hover_icon_color.clone()
        },
        border_radius: if override_theme.border_radius.is_empty() {
            base.border_radius.clone()
        } else {
            override_theme.border_radius.clone()
        },
        box_shadow: if override_theme.box_shadow.is_empty() {
            base.box_shadow.clone()
        } else {
            override_theme.box_shadow.clone()
        },
        hover_box_shadow: if override_theme.hover_box_shadow.is_empty() {
            base.hover_box_shadow.clone()
        } else {
            override_theme.hover_box_shadow.clone()
        },
        transition: if override_theme.transition.is_empty() {
            base.transition.clone()
        } else {
            override_theme.transition.clone()
        },
        z_index: if override_theme.z_index == 0 {
            base.z_index
        } else {
            override_theme.z_index
        },
        size: if override_theme.size.is_empty() {
            base.size.clone()
        } else {
            override_theme.size.clone()
        },
        font_size: if override_theme.font_size.is_empty() {
            base.font_size.clone()
        } else {
            override_theme.font_size.clone()
        },
        border: if override_theme.border.is_empty() {
            base.border.clone()
        } else {
            override_theme.border.clone()
        },
        hover_border: if override_theme.hover_border.is_empty() {
            base.hover_border.clone()
        } else {
            override_theme.hover_border.clone()
        },
        active_background_color: if override_theme.active_background_color.is_empty() {
            base.active_background_color.clone()
        } else {
            override_theme.active_background_color.clone()
        },
        disabled_background_color: if override_theme.disabled_background_color.is_empty() {
            base.disabled_background_color.clone()
        } else {
            override_theme.disabled_background_color.clone()
        },
        disabled_icon_color: if override_theme.disabled_icon_color.is_empty() {
            base.disabled_icon_color.clone()
        } else {
            override_theme.disabled_icon_color.clone()
        },
    }
}

/// 应用暗色主题
pub fn apply_dark_theme(base: &BackTopTheme) -> BackTopTheme {
    merge_theme(base, &BackTopDarkTheme::theme())
}

/// 应用紧凑主题
pub fn apply_compact_theme(base: &BackTopTheme) -> BackTopTheme {
    merge_theme(base, &BackTopCompactTheme::theme())
}

/// 生成主题哈希
pub fn generate_theme_hash(theme: &BackTopTheme) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    theme.background_color.hash(&mut hasher);
    theme.hover_background_color.hash(&mut hasher);
    theme.icon_color.hash(&mut hasher);
    theme.hover_icon_color.hash(&mut hasher);
    theme.border_radius.hash(&mut hasher);
    theme.box_shadow.hash(&mut hasher);
    theme.hover_box_shadow.hash(&mut hasher);
    theme.transition.hash(&mut hasher);
    theme.z_index.hash(&mut hasher);
    theme.size.hash(&mut hasher);
    theme.font_size.hash(&mut hasher);
    
    format!("{:x}", hasher.finish())
}

/// 检查是否为有效的 CSS 颜色值
pub fn is_valid_css_color(color: &str) -> bool {
    if color.is_empty() {
        return false;
    }
    
    // 检查十六进制颜色
    if color.starts_with('#') {
        let hex = &color[1..];
        return hex.len() == 3 || hex.len() == 6 || hex.len() == 8;
    }
    
    // 检查 RGB/RGBA
    if color.starts_with("rgb(") || color.starts_with("rgba(") {
        return true;
    }
    
    // 检查 HSL/HSLA
    if color.starts_with("hsl(") || color.starts_with("hsla(") {
        return true;
    }
    
    // 检查命名颜色（简化版）
    matches!(color.to_lowercase().as_str(),
        "transparent" | "black" | "white" | "red" | "green" | "blue" |
        "yellow" | "orange" | "purple" | "pink" | "gray" | "grey"
    )
}

/// 解析动画类型字符串
pub fn parse_animation_string(s: &str) -> BackTopAnimation {
    BackTopAnimation::from(s)
}

/// 解析缓动函数字符串
pub fn parse_easing_string(s: &str) -> EasingFunction {
    match s.to_lowercase().as_str() {
        "linear" => EasingFunction::Linear,
        "ease-in" => EasingFunction::EaseIn,
        "ease-out" => EasingFunction::EaseOut,
        "ease-in-out" => EasingFunction::EaseInOut,
        "ease-in-quart" => EasingFunction::EaseInQuart,
        "ease-out-quart" => EasingFunction::EaseOutQuart,
        "ease-in-out-quart" => EasingFunction::EaseInOutQuart,
        "ease-in-cubic" => EasingFunction::EaseInCubic,
        "ease-out-cubic" => EasingFunction::EaseOutCubic,
        "ease-in-out-cubic" => EasingFunction::EaseInOutCubic,
        _ => {
            // 尝试解析 cubic-bezier
            if s.starts_with("cubic-bezier(") && s.ends_with(')') {
                let params = &s[13..s.len()-1];
                let parts: Vec<&str> = params.split(',').map(|s| s.trim()).collect();
                if parts.len() == 4 {
                    if let (Ok(x1), Ok(y1), Ok(x2), Ok(y2)) = (
                        parts[0].parse::<f64>(),
                        parts[1].parse::<f64>(),
                        parts[2].parse::<f64>(),
                        parts[3].parse::<f64>(),
                    ) {
                        return EasingFunction::CubicBezier(x1, y1, x2, y2);
                    }
                }
            }
            EasingFunction::default()
        }
    }
}

/// 便捷配置创建函数

/// 创建基础配置
pub fn basic_config() -> BackTopConfig {
    BackTopConfig::default()
}

/// 创建快速配置
pub fn quick_config(visibility_height: i32) -> BackTopConfig {
    BackTopConfig {
        visibility_height,
        ..Default::default()
    }
}

/// 创建自定义位置配置
pub fn positioned_config(bottom: i32, right: i32) -> BackTopConfig {
    BackTopConfig {
        bottom,
        right,
        ..Default::default()
    }
}

/// 创建暗色主题配置
pub fn dark_theme_config() -> BackTopConfig {
    BackTopConfig {
        theme: BackTopDarkTheme::theme(),
        ..Default::default()
    }
}

/// 创建紧凑主题配置
pub fn compact_theme_config() -> BackTopConfig {
    BackTopConfig {
        theme: BackTopCompactTheme::theme(),
        ..Default::default()
    }
}

/// 创建移动端优化配置
pub fn mobile_optimized_config() -> BackTopConfig {
    BackTopConfig {
        mobile_config: Some(BackTopMobileConfig {
            bottom: 20,
            right: 20,
            theme: Some(BackTopCompactTheme::theme()),
        }),
        ..Default::default()
    }
}

/// 创建高性能配置
pub fn performance_config() -> BackTopConfig {
    BackTopConfig {
        throttle: true,
        throttle_delay: 8, // 更快的节流
        animation: BackTopAnimation::Fade, // 简单动画
        duration: 300, // 更短的动画时间
        ..Default::default()
    }
}

/// 创建无障碍优化配置
pub fn accessible_config() -> BackTopConfig {
    BackTopConfig {
        keyboard: true,
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_back_top_config() {
        let config = create_back_top_config(
            Some(300),
            Some(500),
            Some(60),
            Some(60),
            None,
        );
        
        assert_eq!(config.visibility_height, 300);
        assert_eq!(config.duration, 500);
        assert_eq!(config.bottom, 60);
        assert_eq!(config.right, 60);
    }
    
    #[test]
    fn test_validate_back_top_props() {
        let valid_props = BackTopProps {
            visibility_height: 400,
            duration: 450,
            bottom: 50,
            right: 50,
            ..Default::default()
        };
        
        assert!(validate_back_top_props(&valid_props).is_ok());
        
        let invalid_props = BackTopProps {
            visibility_height: -100,
            ..Default::default()
        };
        
        assert!(validate_back_top_props(&invalid_props).is_err());
    }
    
    #[test]
    fn test_should_show_back_top() {
        assert!(should_show_back_top(500.0, 400));
        assert!(!should_show_back_top(300.0, 400));
        assert!(should_show_back_top(400.0, 400));
    }
    
    #[test]
    fn test_calculate_scroll_percentage() {
        let percentage = calculate_scroll_percentage(100.0, 1000.0, 500.0);
        assert_eq!(percentage, 20.0);
        
        let percentage = calculate_scroll_percentage(500.0, 1000.0, 500.0);
        assert_eq!(percentage, 100.0);
        
        let percentage = calculate_scroll_percentage(0.0, 1000.0, 500.0);
        assert_eq!(percentage, 0.0);
    }
    
    #[test]
    fn test_apply_easing_function() {
        let linear = apply_easing_function(0.5, &EasingFunction::Linear);
        assert_eq!(linear, 0.5);
        
        let ease_in = apply_easing_function(0.5, &EasingFunction::EaseIn);
        assert_eq!(ease_in, 0.25);
        
        let ease_out = apply_easing_function(0.5, &EasingFunction::EaseOut);
        assert_eq!(ease_out, 0.75);
    }
    
    #[test]
    fn test_is_valid_css_color() {
        assert!(is_valid_css_color("#ffffff"));
        assert!(is_valid_css_color("#fff"));
        assert!(is_valid_css_color("rgb(255, 255, 255)"));
        assert!(is_valid_css_color("rgba(255, 255, 255, 0.5)"));
        assert!(is_valid_css_color("red"));
        assert!(is_valid_css_color("transparent"));
        
        assert!(!is_valid_css_color(""));
        assert!(!is_valid_css_color("invalid"));
    }
    
    #[test]
    fn test_parse_animation_string() {
        assert_eq!(parse_animation_string("fade"), BackTopAnimation::Fade);
        assert_eq!(parse_animation_string("slide"), BackTopAnimation::Slide);
        assert_eq!(parse_animation_string("fade-slide"), BackTopAnimation::FadeSlide);
        assert_eq!(parse_animation_string("invalid"), BackTopAnimation::default());
    }
    
    #[test]
    fn test_parse_easing_string() {
        assert_eq!(parse_easing_string("linear"), EasingFunction::Linear);
        assert_eq!(parse_easing_string("ease-in"), EasingFunction::EaseIn);
        assert_eq!(parse_easing_string("ease-out"), EasingFunction::EaseOut);
        
        let cubic = parse_easing_string("cubic-bezier(0.4, 0, 0.2, 1)");
        if let EasingFunction::CubicBezier(x1, y1, x2, y2) = cubic {
            assert_eq!(x1, 0.4);
            assert_eq!(y1, 0.0);
            assert_eq!(x2, 0.2);
            assert_eq!(y2, 1.0);
        } else {
            panic!("Expected CubicBezier");
        }
    }
    
    #[test]
    fn test_generate_theme_hash() {
        let theme1 = BackTopTheme::default();
        let theme2 = BackTopTheme::default();
        let theme3 = BackTopDarkTheme::theme();
        
        assert_eq!(generate_theme_hash(&theme1), generate_theme_hash(&theme2));
        assert_ne!(generate_theme_hash(&theme1), generate_theme_hash(&theme3));
    }
    
    #[test]
    fn test_merge_theme() {
        let base = BackTopTheme::default();
        let override_theme = BackTopTheme {
            background_color: "#ff0000".to_string(),
            icon_color: "#00ff00".to_string(),
            ..Default::default()
        };
        
        let merged = merge_theme(&base, &override_theme);
        assert_eq!(merged.background_color, "#ff0000");
        assert_eq!(merged.icon_color, "#00ff00");
        assert_eq!(merged.hover_background_color, base.hover_background_color);
    }
    
    #[test]
    fn test_convenience_configs() {
        let basic = basic_config();
        assert_eq!(basic.visibility_height, 400);
        
        let quick = quick_config(300);
        assert_eq!(quick.visibility_height, 300);
        
        let positioned = positioned_config(100, 100);
        assert_eq!(positioned.bottom, 100);
        assert_eq!(positioned.right, 100);
        
        let dark = dark_theme_config();
        assert_eq!(dark.theme.background_color, BackTopDarkTheme::theme().background_color);
        
        let compact = compact_theme_config();
        assert_eq!(compact.theme.size, BackTopCompactTheme::theme().size);
        
        let mobile = mobile_optimized_config();
        assert!(mobile.mobile_config.is_some());
        
        let performance = performance_config();
        assert_eq!(performance.throttle_delay, 8);
        assert_eq!(performance.duration, 300);
        
        let accessible = accessible_config();
        assert!(accessible.keyboard);
    }
}
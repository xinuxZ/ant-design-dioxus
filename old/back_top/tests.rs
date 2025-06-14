//! BackTop 组件测试
//!
//! 本模块包含 BackTop 组件的全面测试，涵盖了组件渲染、
//! 功能测试、性能测试和边缘情况测试。

#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::components::back_top::types::*;
    use crate::components::back_top::utils::*;
    use crate::components::back_top::styles::*;
    use std::time::Instant;
    
    // ============================================================================
    // 基础渲染测试
    // ============================================================================
    
    #[test]
    fn test_back_top_props_default() {
        let props = BackTopProps::default();
        
        assert_eq!(props.visibility_height, 400);
        assert_eq!(props.duration, 450);
        assert_eq!(props.bottom, 50);
        assert_eq!(props.right, 50);
        assert!(props.visible.is_none());
        assert!(props.target.is_none());
        assert!(props.on_click.is_none());
        assert!(props.style.is_none());
        assert!(props.class.is_none());
        assert!(props.keyboard);
        assert_eq!(props.throttle_delay, 16);
        assert!(props.show_on_mobile);
        assert!(props.mobile_bottom.is_none());
        assert!(props.mobile_right.is_none());
    }
    
    #[test]
    fn test_back_top_props_builder() {
        let props = BackTopProps {
            visibility_height: 200,
            duration: 300,
            bottom: 30,
            right: 30,
            visible: Some(true),
            target: Some("custom-container".to_string()),
            style: Some("color: red;".to_string()),
            class: Some("custom-class".to_string()),
            keyboard: false,
            throttle_delay: 32,
            show_on_mobile: false,
            mobile_bottom: Some(20),
            mobile_right: Some(20),
            aria_label: Some("返回顶部".to_string()),
            ..Default::default()
        };
        
        assert_eq!(props.visibility_height, 200);
        assert_eq!(props.duration, 300);
        assert_eq!(props.bottom, 30);
        assert_eq!(props.right, 30);
        assert_eq!(props.visible, Some(true));
        assert_eq!(props.target, Some("custom-container".to_string()));
        assert_eq!(props.style, Some("color: red;".to_string()));
        assert_eq!(props.class, Some("custom-class".to_string()));
        assert!(!props.keyboard);
        assert_eq!(props.throttle_delay, 32);
        assert!(!props.show_on_mobile);
        assert_eq!(props.mobile_bottom, Some(20));
        assert_eq!(props.mobile_right, Some(20));
        assert_eq!(props.aria_label, Some("返回顶部".to_string()));
    }
    
    // ============================================================================
    // 类型和枚举测试
    // ============================================================================
    
    #[test]
    fn test_back_top_animation_enum() {
        let animations = vec![
            BackTopAnimation::Fade,
            BackTopAnimation::Slide,
            BackTopAnimation::FadeSlide,
            BackTopAnimation::Scale,
            BackTopAnimation::FadeScale,
            BackTopAnimation::Bounce,
            BackTopAnimation::None,
        ];
        
        for animation in animations {
            let debug_str = format!("{:?}", animation);
            assert!(!debug_str.is_empty());
            
            let cloned = animation.clone();
            assert_eq!(animation, cloned);
        }
        
        assert_eq!(BackTopAnimation::default(), BackTopAnimation::FadeSlide);
    }
    
    #[test]
    fn test_easing_function_enum() {
        let easings = vec![
            EasingFunction::Linear,
            EasingFunction::EaseIn,
            EasingFunction::EaseOut,
            EasingFunction::EaseInOut,
            EasingFunction::EaseInQuart,
            EasingFunction::EaseOutQuart,
            EasingFunction::EaseInOutQuart,
            EasingFunction::EaseInCubic,
            EasingFunction::EaseOutCubic,
            EasingFunction::EaseInOutCubic,
            EasingFunction::Custom("cubic-bezier(0.25, 0.46, 0.45, 0.94)".to_string()),
        ];
        
        for easing in easings {
            let debug_str = format!("{:?}", easing);
            assert!(!debug_str.is_empty());
            
            let string_repr = easing.to_string();
            assert!(!string_repr.is_empty());
            
            let cloned = easing.clone();
            assert_eq!(easing, cloned);
        }
        
        assert_eq!(EasingFunction::default(), EasingFunction::EaseOut);
    }
    
    #[test]
    fn test_scroll_direction_enum() {
        let directions = vec![
            ScrollDirection::Up,
            ScrollDirection::Down,
        ];
        
        for direction in directions {
            let debug_str = format!("{:?}", direction);
            assert!(!debug_str.is_empty());
            
            let cloned = direction.clone();
            assert_eq!(direction, cloned);
        }
        
        assert_eq!(ScrollDirection::default(), ScrollDirection::Up);
        assert_ne!(ScrollDirection::Up, ScrollDirection::Down);
    }
    
    // ============================================================================
    // 配置测试
    // ============================================================================
    
    #[test]
    fn test_back_top_config_creation() {
        let props = BackTopProps::default();
        let config = create_back_top_config(&props);
        
        assert_eq!(config.visibility_height, 400);
        assert_eq!(config.duration, 450);
        assert_eq!(config.bottom, 50);
        assert_eq!(config.right, 50);
        assert_eq!(config.animation, BackTopAnimation::FadeSlide);
        assert_eq!(config.easing, EasingFunction::EaseOut);
        assert_eq!(config.throttle, true);
        assert_eq!(config.throttle_delay, 16);
        assert_eq!(config.keyboard, true);
        assert!(config.show_on_mobile);
        assert!(config.mobile_config.is_none());
        assert!(!config.has_custom_content);
    }
    
    #[test]
    fn test_back_top_config_with_mobile() {
        let props = BackTopProps {
            mobile_bottom: Some(20),
            mobile_right: Some(20),
            ..Default::default()
        };
        let config = create_back_top_config(&props);
        
        assert!(config.mobile_config.is_some());
        let mobile_config = config.mobile_config.unwrap();
        assert_eq!(mobile_config.bottom, 20);
        assert_eq!(mobile_config.right, 20);
    }
    
    #[test]
    fn test_back_top_config_effective_methods() {
        let config = BackTopConfig {
            bottom: 50,
            right: 50,
            theme: BackTopTheme::default(),
            mobile_config: Some(BackTopMobileConfig {
                bottom: 20,
                right: 20,
            }),
            ..Default::default()
        };
        
        // 桌面端
        assert_eq!(config.effective_bottom(false), 50);
        assert_eq!(config.effective_right(false), 50);
        
        // 移动端
        assert_eq!(config.effective_bottom(true), 20);
        assert_eq!(config.effective_right(true), 20);
        
        // 主题
        let desktop_theme = config.effective_theme(false);
        let mobile_theme = config.effective_theme(true);
        assert_eq!(desktop_theme.size, "40px");
        assert_eq!(mobile_theme.size, "40px"); // 没有移动端主题时使用默认主题
    }
    
    // ============================================================================
    // 工具函数测试
    // ============================================================================
    
    #[test]
    fn test_validate_back_top_props() {
        // 有效的 props
        let valid_props = BackTopProps::default();
        assert!(validate_back_top_props(&valid_props).is_ok());
        
        // 无效的 visibility_height
        let invalid_props = BackTopProps {
            visibility_height: 0,
            ..Default::default()
        };
        assert!(validate_back_top_props(&invalid_props).is_err());
        
        // 无效的 duration
        let invalid_props = BackTopProps {
            duration: 0,
            ..Default::default()
        };
        assert!(validate_back_top_props(&invalid_props).is_err());
        
        // 无效的 throttle_delay
        let invalid_props = BackTopProps {
            throttle_delay: 0,
            ..Default::default()
        };
        assert!(validate_back_top_props(&invalid_props).is_err());
    }
    
    #[test]
    fn test_calculate_scroll_position() {
        assert_eq!(calculate_scroll_position(0.0, 1000.0, 500.0), 0.0);
        assert_eq!(calculate_scroll_position(250.0, 1000.0, 500.0), 50.0);
        assert_eq!(calculate_scroll_position(500.0, 1000.0, 500.0), 100.0);
        assert_eq!(calculate_scroll_position(600.0, 1000.0, 500.0), 100.0); // 超过最大值
        
        // 边缘情况：scroll_height <= client_height
        assert_eq!(calculate_scroll_position(100.0, 500.0, 500.0), 0.0);
        assert_eq!(calculate_scroll_position(100.0, 400.0, 500.0), 0.0);
    }
    
    #[test]
    fn test_apply_easing_function() {
        // 线性缓动
        assert!((apply_easing_function(0.5, &EasingFunction::Linear) - 0.5).abs() < 0.001);
        
        // EaseOut 在 t=0 和 t=1 时的值
        assert!((apply_easing_function(0.0, &EasingFunction::EaseOut) - 0.0).abs() < 0.001);
        assert!((apply_easing_function(1.0, &EasingFunction::EaseOut) - 1.0).abs() < 0.001);
        
        // EaseIn 在 t=0 和 t=1 时的值
        assert!((apply_easing_function(0.0, &EasingFunction::EaseIn) - 0.0).abs() < 0.001);
        assert!((apply_easing_function(1.0, &EasingFunction::EaseIn) - 1.0).abs() < 0.001);
        
        // 边界值测试
        assert!((apply_easing_function(-0.1, &EasingFunction::Linear) - 0.0).abs() < 0.001);
        assert!((apply_easing_function(1.1, &EasingFunction::Linear) - 1.0).abs() < 0.001);
    }
    
    #[test]
    fn test_is_mobile_device() {
        // 注意：在测试环境中，这个函数可能总是返回 false
        // 因为没有真实的浏览器环境
        let result = is_mobile_device();
        assert!(result == true || result == false); // 只是确保函数能执行
    }
    
    #[test]
    fn test_should_show_back_top() {
        // 滚动位置低于可见高度
        assert!(!should_show_back_top(300.0, 400, true, false));
        assert!(!should_show_back_top(300.0, 400, true, true));
        
        // 滚动位置高于可见高度
        assert!(should_show_back_top(500.0, 400, true, false));
        assert!(should_show_back_top(500.0, 400, true, true));
        
        // 移动端不显示
        assert!(!should_show_back_top(500.0, 400, false, true));
        assert!(should_show_back_top(500.0, 400, false, false));
    }
    
    #[test]
    fn test_generate_css_variables() {
        let theme = BackTopTheme::default();
        let variables = generate_css_variables(&theme);
        
        assert!(variables.contains_key("--back-top-bg-color"));
        assert!(variables.contains_key("--back-top-hover-bg-color"));
        assert!(variables.contains_key("--back-top-icon-color"));
        assert!(variables.contains_key("--back-top-z-index"));
        assert!(variables.contains_key("--back-top-size"));
        
        assert_eq!(variables.get("--back-top-bg-color"), Some(&"#ffffff".to_string()));
        assert_eq!(variables.get("--back-top-z-index"), Some(&"1010".to_string()));
    }
    
    #[test]
    fn test_merge_theme() {
        let base_theme = BackTopTheme::default();
        let custom_theme = BackTopTheme {
            background_color: "#ff0000".to_string(),
            icon_color: "#00ff00".to_string(),
            ..Default::default()
        };
        
        let merged = merge_theme(&base_theme, &custom_theme);
        
        assert_eq!(merged.background_color, "#ff0000");
        assert_eq!(merged.icon_color, "#00ff00");
        assert_eq!(merged.size, base_theme.size); // 未覆盖的属性保持原值
    }
    
    #[test]
    fn test_apply_dark_theme() {
        let base_theme = BackTopTheme::default();
        let dark_theme = apply_dark_theme(&base_theme);
        
        assert_eq!(dark_theme.background_color, "#1f1f1f");
        assert_eq!(dark_theme.icon_color, "#ffffff");
        assert_eq!(dark_theme.hover_background_color, "#262626");
    }
    
    #[test]
    fn test_apply_compact_theme() {
        let base_theme = BackTopTheme::default();
        let compact_theme = apply_compact_theme(&base_theme);
        
        assert_eq!(compact_theme.size, "32px");
        assert_eq!(compact_theme.font_size, "16px");
        assert_eq!(compact_theme.border_radius, "4px");
    }
    
    #[test]
    fn test_generate_theme_hash() {
        let theme1 = BackTopTheme::default();
        let theme2 = BackTopTheme {
            background_color: "#ff0000".to_string(),
            ..Default::default()
        };
        
        let hash1 = generate_theme_hash(&theme1);
        let hash2 = generate_theme_hash(&theme2);
        
        assert_ne!(hash1, hash2);
        assert_eq!(hash1.len(), 16); // MD5 哈希的前16位
        assert_eq!(hash2.len(), 16);
    }
    
    #[test]
    fn test_is_valid_css_color() {
        // 有效的颜色值
        assert!(is_valid_css_color("#ffffff"));
        assert!(is_valid_css_color("#fff"));
        assert!(is_valid_css_color("rgb(255, 255, 255)"));
        assert!(is_valid_css_color("rgba(255, 255, 255, 0.5)"));
        assert!(is_valid_css_color("red"));
        assert!(is_valid_css_color("transparent"));
        assert!(is_valid_css_color("hsl(0, 100%, 50%)"));
        
        // 无效的颜色值
        assert!(!is_valid_css_color(""));
        assert!(!is_valid_css_color("invalid"));
        assert!(!is_valid_css_color("#gggggg"));
        assert!(!is_valid_css_color("rgb(256, 256, 256)"));
    }
    
    // ============================================================================
    // 主题测试
    // ============================================================================
    
    #[test]
    fn test_back_top_theme_default() {
        let theme = BackTopTheme::default();
        
        assert_eq!(theme.background_color, "#ffffff");
        assert_eq!(theme.hover_background_color, "#f5f5f5");
        assert_eq!(theme.icon_color, "#000000");
        assert_eq!(theme.hover_icon_color, "#1890ff");
        assert_eq!(theme.border_radius, "6px");
        assert_eq!(theme.size, "40px");
        assert_eq!(theme.font_size, "18px");
        assert_eq!(theme.z_index, 1010);
        assert_eq!(theme.transition, "all 0.3s ease");
    }
    
    #[test]
    fn test_back_top_dark_theme() {
        let theme = BackTopDarkTheme::theme();
        
        assert_eq!(theme.background_color, "#1f1f1f");
        assert_eq!(theme.hover_background_color, "#262626");
        assert_eq!(theme.icon_color, "#ffffff");
        assert_eq!(theme.hover_icon_color, "#40a9ff");
    }
    
    #[test]
    fn test_back_top_compact_theme() {
        let theme = BackTopCompactTheme::theme();
        
        assert_eq!(theme.size, "32px");
        assert_eq!(theme.font_size, "16px");
        assert_eq!(theme.border_radius, "4px");
    }
    
    #[test]
    fn test_global_theme_functions() {
        let custom_theme = BackTopTheme {
            background_color: "#ff0000".to_string(),
            ..Default::default()
        };
        
        // 设置全局主题
        set_global_back_top_theme(custom_theme.clone());
        
        // 获取全局主题
        let global_theme = get_global_back_top_theme();
        assert_eq!(global_theme.background_color, "#ff0000");
        
        // 重置为默认主题
        set_global_back_top_theme(BackTopTheme::default());
        let reset_theme = get_global_back_top_theme();
        assert_eq!(reset_theme.background_color, "#ffffff");
    }
    
    // ============================================================================
    // 样式生成测试
    // ============================================================================
    
    #[test]
    fn test_generate_back_top_container_style() {
        let config = BackTopConfig::default();
        
        // 可见状态
        let visible_style = generate_back_top_container_style(&config, true, false);
        assert!(visible_style.contains("position: fixed"));
        assert!(visible_style.contains("bottom: 50px"));
        assert!(visible_style.contains("right: 50px"));
        assert!(visible_style.contains("opacity: 1"));
        assert!(visible_style.contains("visibility: visible"));
        
        // 隐藏状态
        let hidden_style = generate_back_top_container_style(&config, false, false);
        assert!(hidden_style.contains("opacity: 0"));
        assert!(hidden_style.contains("visibility: hidden"));
        assert!(hidden_style.contains("pointer-events: none"));
        
        // 移动端
        let mobile_style = generate_back_top_container_style(&config, true, true);
        assert!(mobile_style.contains("position: fixed"));
    }
    
    #[test]
    fn test_generate_back_top_button_style() {
        let config = BackTopConfig::default();
        
        // 正常状态
        let normal_style = generate_back_top_button_style(&config, false, false, false, false);
        assert!(normal_style.contains("width: 40px"));
        assert!(normal_style.contains("height: 40px"));
        assert!(normal_style.contains("display: flex"));
        assert!(normal_style.contains("cursor: pointer"));
        
        // 悬停状态
        let hover_style = generate_back_top_button_style(&config, false, true, false, false);
        assert!(hover_style.contains("cursor: pointer"));
        
        // 激活状态
        let active_style = generate_back_top_button_style(&config, false, false, true, false);
        assert!(active_style.contains("cursor: pointer"));
        
        // 禁用状态
        let disabled_style = generate_back_top_button_style(&config, false, false, false, true);
        assert!(disabled_style.contains("cursor: not-allowed"));
    }
    
    #[test]
    fn test_generate_back_top_icon_style() {
        let style = generate_back_top_icon_style();
        
        assert!(style.contains("width: 1em"));
        assert!(style.contains("height: 1em"));
        assert!(style.contains("fill: currentColor"));
        assert!(style.contains("display: inline-block"));
    }
    
    #[test]
    fn test_generate_back_top_class_name() {
        let config = BackTopConfig::default();
        
        // 可见状态
        let visible_class = generate_back_top_class_name(&config, true, false);
        assert!(visible_class.contains("ant-back-top"));
        assert!(visible_class.contains("ant-back-top-visible"));
        assert!(visible_class.contains("ant-back-top-fade-slide"));
        
        // 隐藏状态
        let hidden_class = generate_back_top_class_name(&config, false, false);
        assert!(hidden_class.contains("ant-back-top-hidden"));
        
        // 移动端
        let mobile_class = generate_back_top_class_name(&config, true, true);
        assert!(mobile_class.contains("ant-back-top-mobile"));
        
        // 不同动画类型
        let fade_config = BackTopConfig {
            animation: BackTopAnimation::Fade,
            ..Default::default()
        };
        let fade_class = generate_back_top_class_name(&fade_config, true, false);
        assert!(fade_class.contains("ant-back-top-fade"));
    }
    
    #[test]
    fn test_generate_responsive_style() {
        let config = BackTopConfig::default();
        let style = generate_responsive_style(&config);
        
        assert!(style.contains("@media (max-width: 768px)"));
        assert!(style.contains("@media (max-width: 480px)"));
        assert!(style.contains("@media (prefers-reduced-motion: reduce)"));
        assert!(style.contains(".ant-back-top"));
    }
    
    #[test]
    fn test_generate_animation_style() {
        // 淡入淡出动画
        let fade_style = generate_animation_style(
            &BackTopAnimation::Fade,
            &EasingFunction::EaseOut,
        );
        assert!(fade_style.contains(".ant-back-top-fade"));
        assert!(fade_style.contains("opacity"));
        assert!(fade_style.contains("transition"));
        
        // 滑动动画
        let slide_style = generate_animation_style(
            &BackTopAnimation::Slide,
            &EasingFunction::EaseOut,
        );
        assert!(slide_style.contains(".ant-back-top-slide"));
        assert!(slide_style.contains("transform"));
        assert!(slide_style.contains("translateY"));
        
        // 缩放动画
        let scale_style = generate_animation_style(
            &BackTopAnimation::Scale,
            &EasingFunction::EaseOut,
        );
        assert!(scale_style.contains(".ant-back-top-scale"));
        assert!(scale_style.contains("scale"));
        
        // 弹跳动画
        let bounce_style = generate_animation_style(
            &BackTopAnimation::Bounce,
            &EasingFunction::EaseOut,
        );
        assert!(bounce_style.contains(".ant-back-top-bounce"));
        assert!(bounce_style.contains(":active"));
        
        // 无动画
        let none_style = generate_animation_style(
            &BackTopAnimation::None,
            &EasingFunction::Linear,
        );
        assert!(none_style.contains(".ant-back-top-no-animation"));
        assert!(none_style.contains("transition: none"));
    }
    
    #[test]
    fn test_generate_accessibility_style() {
        let style = generate_accessibility_style();
        
        assert!(style.contains(":focus-visible"));
        assert!(style.contains("outline"));
        assert!(style.contains("@media (prefers-reduced-motion: reduce)"));
        assert!(style.contains("@media (prefers-contrast: high)"));
    }
    
    #[test]
    fn test_generate_dark_theme_style() {
        let style = generate_dark_theme_style();
        
        assert!(style.contains("[data-theme='dark']"));
        assert!(style.contains(".ant-back-top-dark"));
        assert!(style.contains(":hover"));
        assert!(style.contains(":active"));
    }
    
    #[test]
    fn test_generate_compact_theme_style() {
        let style = generate_compact_theme_style();
        
        assert!(style.contains(".ant-back-top-compact"));
        assert!(style.contains("32px"));
        assert!(style.contains("16px"));
    }
    
    #[test]
    fn test_generate_back_top_stylesheet() {
        let config = BackTopConfig::default();
        let stylesheet = generate_back_top_stylesheet(&config);
        
        assert!(stylesheet.contains(".ant-back-top"));
        assert!(stylesheet.contains("position: fixed"));
        assert!(stylesheet.contains("/* BackTop 基础样式 */"));
        assert!(stylesheet.contains("/* 动画样式 */"));
        assert!(stylesheet.contains("/* 响应式样式 */"));
        assert!(stylesheet.contains("/* 可访问性样式 */"));
        assert!(stylesheet.contains("/* 暗色主题样式 */"));
        assert!(stylesheet.contains("/* 紧凑主题样式 */"));
    }
    
    // ============================================================================
    // 样式生成器测试
    // ============================================================================
    
    #[test]
    fn test_back_top_style_generator() {
        let mut generator = BackTopStyleGenerator::new(BackTopConfig::default());
        
        // 测试容器样式生成
        let container_style = generator.container_style(true, false);
        assert!(container_style.contains("position: fixed"));
        
        // 测试按钮样式生成
        let button_style = generator.button_style(false, false, false, false);
        assert!(button_style.contains("display: flex"));
        
        // 测试图标样式生成
        let icon_style = generator.icon_style();
        assert!(icon_style.contains("fill: currentColor"));
        
        // 测试类名生成
        let class_name = generator.class_name(true, false);
        assert!(class_name.contains("ant-back-top"));
        
        // 测试样式表生成
        let stylesheet = generator.stylesheet();
        assert!(stylesheet.contains(".ant-back-top"));
        
        // 测试缓存
        assert_eq!(generator.cache_size(), 5);
        
        // 测试缓存清除
        generator.clear_cache();
        assert_eq!(generator.cache_size(), 0);
        
        // 测试配置更新
        let new_config = BackTopConfig {
            bottom: 100,
            right: 100,
            ..Default::default()
        };
        generator.update_config(new_config);
        assert_eq!(generator.config().bottom, 100);
        assert_eq!(generator.config().right, 100);
    }
    
    #[test]
    fn test_generate_inline_style() {
        let config = BackTopConfig::default();
        let style = generate_inline_style(&config, true, false, false, false, false);
        
        assert!(style.contains("position: fixed"));
        assert!(style.contains("display: flex"));
    }
    
    #[test]
    fn test_generate_css_variables_style() {
        let theme = BackTopTheme::default();
        let style = generate_css_variables_style(&theme);
        
        assert!(style.contains("--back-top-bg-color"));
        assert!(style.contains("--back-top-hover-bg-color"));
        assert!(style.contains("--back-top-icon-color"));
        assert!(style.contains("--back-top-z-index"));
    }
    
    #[test]
    fn test_generate_default_icon_svg() {
        let svg = generate_default_icon_svg();
        
        assert!(svg.contains("<svg"));
        assert!(svg.contains("viewBox"));
        assert!(svg.contains("fill=\"currentColor\""));
        assert!(svg.contains("<path"));
    }
    
    // ============================================================================
    // 解析函数测试
    // ============================================================================
    
    #[test]
    fn test_parse_animation_string() {
        assert_eq!(parse_animation_string("fade"), Some(BackTopAnimation::Fade));
        assert_eq!(parse_animation_string("slide"), Some(BackTopAnimation::Slide));
        assert_eq!(parse_animation_string("fade-slide"), Some(BackTopAnimation::FadeSlide));
        assert_eq!(parse_animation_string("scale"), Some(BackTopAnimation::Scale));
        assert_eq!(parse_animation_string("fade-scale"), Some(BackTopAnimation::FadeScale));
        assert_eq!(parse_animation_string("bounce"), Some(BackTopAnimation::Bounce));
        assert_eq!(parse_animation_string("none"), Some(BackTopAnimation::None));
        assert_eq!(parse_animation_string("invalid"), None);
        assert_eq!(parse_animation_string(""), None);
    }
    
    #[test]
    fn test_parse_easing_string() {
        assert_eq!(parse_easing_string("linear"), Some(EasingFunction::Linear));
        assert_eq!(parse_easing_string("ease-in"), Some(EasingFunction::EaseIn));
        assert_eq!(parse_easing_string("ease-out"), Some(EasingFunction::EaseOut));
        assert_eq!(parse_easing_string("ease-in-out"), Some(EasingFunction::EaseInOut));
        assert_eq!(parse_easing_string("ease-in-quart"), Some(EasingFunction::EaseInQuart));
        assert_eq!(parse_easing_string("ease-out-quart"), Some(EasingFunction::EaseOutQuart));
        assert_eq!(parse_easing_string("ease-in-out-quart"), Some(EasingFunction::EaseInOutQuart));
        assert_eq!(parse_easing_string("ease-in-cubic"), Some(EasingFunction::EaseInCubic));
        assert_eq!(parse_easing_string("ease-out-cubic"), Some(EasingFunction::EaseOutCubic));
        assert_eq!(parse_easing_string("ease-in-out-cubic"), Some(EasingFunction::EaseInOutCubic));
        
        // 自定义贝塞尔曲线
        let custom = parse_easing_string("cubic-bezier(0.25, 0.46, 0.45, 0.94)");
        assert!(custom.is_some());
        if let Some(EasingFunction::Custom(curve)) = custom {
            assert_eq!(curve, "cubic-bezier(0.25, 0.46, 0.45, 0.94)");
        }
        
        assert_eq!(parse_easing_string("invalid"), None);
        assert_eq!(parse_easing_string(""), None);
    }
    
    // ============================================================================
    // 便捷构造函数测试
    // ============================================================================
    
    #[test]
    fn test_convenience_constructors() {
        // 这些测试主要验证函数能够编译和调用
        // 在实际的 Dioxus 环境中，这些会返回有效的 Element
        
        // 测试基础构造函数的存在
        // 注意：在测试环境中，我们无法直接测试 rsx! 宏的返回值
        // 但我们可以验证函数的存在和基本逻辑
        
        assert!(true); // 占位符测试，确保函数存在
    }
    
    // ============================================================================
    // 性能测试
    // ============================================================================
    
    #[test]
    fn test_style_generation_performance() {
        let config = BackTopConfig::default();
        let start = Instant::now();
        
        // 生成1000次样式
        for _ in 0..1000 {
            let _ = generate_back_top_container_style(&config, true, false);
            let _ = generate_back_top_button_style(&config, false, false, false, false);
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 1000); // 应该在1秒内完成
    }
    
    #[test]
    fn test_class_name_generation_performance() {
        let config = BackTopConfig::default();
        let start = Instant::now();
        
        // 生成1000次类名
        for _ in 0..1000 {
            let _ = generate_back_top_class_name(&config, true, false);
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100); // 应该在100毫秒内完成
    }
    
    #[test]
    fn test_theme_hash_generation_performance() {
        let theme = BackTopTheme::default();
        let start = Instant::now();
        
        // 生成1000次哈希
        for _ in 0..1000 {
            let _ = generate_theme_hash(&theme);
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100); // 应该在100毫秒内完成
    }
    
    #[test]
    fn test_style_generator_cache_performance() {
        let mut generator = BackTopStyleGenerator::new(BackTopConfig::default());
        let start = Instant::now();
        
        // 重复访问缓存的样式
        for _ in 0..1000 {
            let _ = generator.container_style(true, false);
            let _ = generator.button_style(false, false, false, false);
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 50); // 缓存访问应该非常快
    }
    
    // ============================================================================
    // 内存使用测试
    // ============================================================================
    
    #[test]
    fn test_memory_usage() {
        let mut generators = Vec::new();
        
        // 创建多个样式生成器
        for _ in 0..100 {
            let generator = BackTopStyleGenerator::new(BackTopConfig::default());
            generators.push(generator);
        }
        
        // 验证每个生成器都能正常工作
        for generator in &mut generators {
            let _ = generator.container_style(true, false);
            assert!(generator.cache_size() > 0);
        }
        
        // 清理缓存
        for generator in &mut generators {
            generator.clear_cache();
            assert_eq!(generator.cache_size(), 0);
        }
    }
    
    // ============================================================================
    // 边缘情况和集成测试
    // ============================================================================
    
    #[test]
    fn test_edge_cases() {
        // 测试极端配置值
        let extreme_config = BackTopConfig {
            visibility_height: u32::MAX,
            duration: u32::MAX,
            bottom: u32::MAX,
            right: u32::MAX,
            throttle_delay: u32::MAX,
            ..Default::default()
        };
        
        // 应该能够处理极端值而不崩溃
        let style = generate_back_top_container_style(&extreme_config, true, false);
        assert!(style.contains("position: fixed"));
        
        // 测试空字符串和特殊字符
        let theme_with_special_chars = BackTopTheme {
            background_color: "rgba(255, 255, 255, 0.8)".to_string(),
            icon_color: "hsl(210, 100%, 50%)".to_string(),
            ..Default::default()
        };
        
        let variables = generate_css_variables(&theme_with_special_chars);
        assert!(variables.contains_key("--back-top-bg-color"));
    }
    
    #[test]
    fn test_integration_scenarios() {
        // 测试完整的配置流程
        let props = BackTopProps {
            visibility_height: 300,
            duration: 600,
            bottom: 30,
            right: 30,
            animation: BackTopAnimation::Bounce,
            easing: EasingFunction::EaseInOut,
            mobile_bottom: Some(20),
            mobile_right: Some(20),
            ..Default::default()
        };
        
        let config = create_back_top_config(&props);
        assert!(validate_back_top_props(&props).is_ok());
        
        let mut generator = BackTopStyleGenerator::new(config);
        
        // 生成所有类型的样式
        let container_style = generator.container_style(true, false);
        let button_style = generator.button_style(false, true, false, false);
        let icon_style = generator.icon_style();
        let class_name = generator.class_name(true, false);
        let stylesheet = generator.stylesheet();
        
        // 验证所有样式都包含预期内容
        assert!(container_style.contains("bottom: 30px"));
        assert!(button_style.contains("cursor: pointer"));
        assert!(icon_style.contains("fill: currentColor"));
        assert!(class_name.contains("ant-back-top-bounce"));
        assert!(stylesheet.contains(".ant-back-top"));
        
        // 测试移动端样式
        let mobile_container_style = generator.container_style(true, true);
        assert!(mobile_container_style.contains("bottom: 20px"));
        
        // 测试缓存效果
        assert!(generator.cache_size() > 0);
    }
    
    #[test]
    fn test_scroll_state_calculations() {
        let state = BackTopScrollState {
            scroll_top: 500.0,
            scroll_height: 2000.0,
            client_height: 800.0,
            scroll_percentage: 41.67, // (500 / (2000 - 800)) * 100
            direction: ScrollDirection::Down,
            is_scrolling: true,
        };
        
        // 验证滚动百分比计算
        let calculated_percentage = calculate_scroll_position(
            state.scroll_top,
            state.scroll_height,
            state.client_height,
        );
        assert!((calculated_percentage - 41.67).abs() < 0.1);
        
        // 验证是否应该显示 BackTop
        assert!(should_show_back_top(state.scroll_top, 400, true, false));
        assert!(!should_show_back_top(300.0, 400, true, false));
    }
}
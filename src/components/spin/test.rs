#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::spin::types::*;
    use crate::components::spin::utils::*;
    use crate::components::spin::styles::*;
    use dioxus::prelude::*;
    use dioxus_testing::*;
    use std::time::Duration;

    // 测试 SpinSize 枚举
    #[test]
    fn test_spin_size_default() {
        assert_eq!(SpinSize::default(), SpinSize::Default);
    }

    #[test]
    fn test_spin_size_display() {
        assert_eq!(format!("{}", SpinSize::Small), "small");
        assert_eq!(format!("{}", SpinSize::Default), "default");
        assert_eq!(format!("{}", SpinSize::Large), "large");
    }

    #[test]
    fn test_spin_size_from_str() {
        assert_eq!(SpinSize::from_str("small"), SpinSize::Small);
        assert_eq!(SpinSize::from_str("default"), SpinSize::Default);
        assert_eq!(SpinSize::from_str("large"), SpinSize::Large);
        assert_eq!(SpinSize::from_str("unknown"), SpinSize::Default);
    }

    #[test]
    fn test_spin_size_to_string() {
        assert_eq!(SpinSize::Small.to_string(), "small");
        assert_eq!(SpinSize::Default.to_string(), "default");
        assert_eq!(SpinSize::Large.to_string(), "large");
    }

    #[test]
    fn test_spin_size_to_pixels() {
        assert_eq!(SpinSize::Small.to_pixels(), 14);
        assert_eq!(SpinSize::Default.to_pixels(), 20);
        assert_eq!(SpinSize::Large.to_pixels(), 32);
    }

    #[test]
    fn test_spin_size_get_class_name() {
        assert_eq!(SpinSize::Small.get_class_name(), "ant-spin-sm");
        assert_eq!(SpinSize::Default.get_class_name(), "ant-spin");
        assert_eq!(SpinSize::Large.get_class_name(), "ant-spin-lg");
    }

    #[test]
    fn test_spin_size_get_dot_size() {
        assert_eq!(SpinSize::Small.get_dot_size(), 14);
        assert_eq!(SpinSize::Default.get_dot_size(), 20);
        assert_eq!(SpinSize::Large.get_dot_size(), 32);
    }

    #[test]
    fn test_spin_size_get_animation_duration() {
        assert_eq!(SpinSize::Small.get_animation_duration(), Duration::from_millis(1200));
        assert_eq!(SpinSize::Default.get_animation_duration(), Duration::from_millis(1200));
        assert_eq!(SpinSize::Large.get_animation_duration(), Duration::from_millis(1200));
    }

    #[test]
    fn test_spin_size_is_small() {
        assert!(SpinSize::Small.is_small());
        assert!(!SpinSize::Default.is_small());
        assert!(!SpinSize::Large.is_small());
    }

    #[test]
    fn test_spin_size_is_large() {
        assert!(!SpinSize::Small.is_large());
        assert!(!SpinSize::Default.is_large());
        assert!(SpinSize::Large.is_large());
    }

    // 测试 SpinState 结构体
    #[test]
    fn test_spin_state_default() {
        let state = SpinState::default();
        assert_eq!(state.spinning, false);
        assert_eq!(state.delayed, false);
        assert_eq!(state.show_indicator, false);
    }

    #[test]
    fn test_spin_state_new() {
        let state = SpinState::new(true, false, true);
        assert_eq!(state.spinning, true);
        assert_eq!(state.delayed, false);
        assert_eq!(state.show_indicator, true);
    }

    #[test]
    fn test_spin_state_with_spinning() {
        let mut state = SpinState::default();
        state = state.with_spinning(true);
        assert_eq!(state.spinning, true);
    }

    #[test]
    fn test_spin_state_with_delayed() {
        let mut state = SpinState::default();
        state = state.with_delayed(true);
        assert_eq!(state.delayed, true);
    }

    #[test]
    fn test_spin_state_with_show_indicator() {
        let mut state = SpinState::default();
        state = state.with_show_indicator(true);
        assert_eq!(state.show_indicator, true);
    }

    #[test]
    fn test_spin_state_should_show() {
        let state = SpinState::new(true, false, true);
        assert!(state.should_show());
        
        let state = SpinState::new(false, false, false);
        assert!(!state.should_show());
        
        let state = SpinState::new(true, true, false);
        assert!(!state.should_show()); // 延迟状态下不显示
    }

    #[test]
    fn test_spin_state_is_active() {
        let state = SpinState::new(true, false, true);
        assert!(state.is_active());
        
        let state = SpinState::new(false, false, false);
        assert!(!state.is_active());
    }

    #[test]
    fn test_spin_state_is_delayed() {
        let state = SpinState::new(true, true, false);
        assert!(state.is_delayed());
        
        let state = SpinState::new(true, false, true);
        assert!(!state.is_delayed());
    }

    #[test]
    fn test_spin_state_has_indicator() {
        let state = SpinState::new(true, false, true);
        assert!(state.has_indicator());
        
        let state = SpinState::new(true, false, false);
        assert!(!state.has_indicator());
    }

    #[test]
    fn test_spin_state_reset() {
        let mut state = SpinState::new(true, true, true);
        state.reset();
        
        assert_eq!(state.spinning, false);
        assert_eq!(state.delayed, false);
        assert_eq!(state.show_indicator, false);
    }

    #[test]
    fn test_spin_state_start_spinning() {
        let mut state = SpinState::default();
        state.start_spinning();
        
        assert_eq!(state.spinning, true);
        assert_eq!(state.show_indicator, true);
    }

    #[test]
    fn test_spin_state_stop_spinning() {
        let mut state = SpinState::new(true, false, true);
        state.stop_spinning();
        
        assert_eq!(state.spinning, false);
        assert_eq!(state.show_indicator, false);
    }

    #[test]
    fn test_spin_state_set_delayed() {
        let mut state = SpinState::default();
        state.set_delayed(true);
        
        assert_eq!(state.delayed, true);
    }

    // 测试 SpinConfig 结构体
    #[test]
    fn test_spin_config_default() {
        let config = SpinConfig::default();
        assert_eq!(config.spinning, false);
        assert_eq!(config.size, SpinSize::Default);
        assert_eq!(config.delay, None);
        assert!(config.indicator.is_none());
        assert!(config.tip.is_none());
        assert_eq!(config.wrapper_class_name, None);
    }

    #[test]
    fn test_spin_config_new() {
        let config = SpinConfig::new(
            true,
            SpinSize::Large,
            Some(500),
            Some("Custom indicator".to_string()),
            Some("Loading...".to_string()),
            Some("wrapper-class".to_string()),
        );
        
        assert_eq!(config.spinning, true);
        assert_eq!(config.size, SpinSize::Large);
        assert_eq!(config.delay, Some(500));
        assert_eq!(config.indicator, Some("Custom indicator".to_string()));
        assert_eq!(config.tip, Some("Loading...".to_string()));
        assert_eq!(config.wrapper_class_name, Some("wrapper-class".to_string()));
    }

    #[test]
    fn test_spin_config_with_spinning() {
        let mut config = SpinConfig::default();
        config = config.with_spinning(true);
        assert_eq!(config.spinning, true);
    }

    #[test]
    fn test_spin_config_with_size() {
        let mut config = SpinConfig::default();
        config = config.with_size(SpinSize::Small);
        assert_eq!(config.size, SpinSize::Small);
    }

    #[test]
    fn test_spin_config_with_delay() {
        let mut config = SpinConfig::default();
        config = config.with_delay(Some(1000));
        assert_eq!(config.delay, Some(1000));
    }

    #[test]
    fn test_spin_config_with_indicator() {
        let mut config = SpinConfig::default();
        config = config.with_indicator(Some("Custom".to_string()));
        assert_eq!(config.indicator, Some("Custom".to_string()));
    }

    #[test]
    fn test_spin_config_with_tip() {
        let mut config = SpinConfig::default();
        config = config.with_tip(Some("Loading".to_string()));
        assert_eq!(config.tip, Some("Loading".to_string()));
    }

    #[test]
    fn test_spin_config_with_wrapper_class_name() {
        let mut config = SpinConfig::default();
        config = config.with_wrapper_class_name(Some("wrapper".to_string()));
        assert_eq!(config.wrapper_class_name, Some("wrapper".to_string()));
    }

    #[test]
    fn test_spin_config_is_spinning() {
        let config = SpinConfig {
            spinning: true,
            ..Default::default()
        };
        assert!(config.is_spinning());
        
        let config = SpinConfig {
            spinning: false,
            ..Default::default()
        };
        assert!(!config.is_spinning());
    }

    #[test]
    fn test_spin_config_has_delay() {
        let config = SpinConfig {
            delay: Some(500),
            ..Default::default()
        };
        assert!(config.has_delay());
        
        let config = SpinConfig {
            delay: None,
            ..Default::default()
        };
        assert!(!config.has_delay());
    }

    #[test]
    fn test_spin_config_get_delay() {
        let config = SpinConfig {
            delay: Some(1000),
            ..Default::default()
        };
        assert_eq!(config.get_delay(), 1000);
        
        let config = SpinConfig {
            delay: None,
            ..Default::default()
        };
        assert_eq!(config.get_delay(), 0);
    }

    #[test]
    fn test_spin_config_has_custom_indicator() {
        let config = SpinConfig {
            indicator: Some("Custom".to_string()),
            ..Default::default()
        };
        assert!(config.has_custom_indicator());
        
        let config = SpinConfig {
            indicator: None,
            ..Default::default()
        };
        assert!(!config.has_custom_indicator());
    }

    #[test]
    fn test_spin_config_has_tip() {
        let config = SpinConfig {
            tip: Some("Loading".to_string()),
            ..Default::default()
        };
        assert!(config.has_tip());
        
        let config = SpinConfig {
            tip: None,
            ..Default::default()
        };
        assert!(!config.has_tip());
    }

    #[test]
    fn test_spin_config_has_wrapper_class() {
        let config = SpinConfig {
            wrapper_class_name: Some("wrapper".to_string()),
            ..Default::default()
        };
        assert!(config.has_wrapper_class());
        
        let config = SpinConfig {
            wrapper_class_name: None,
            ..Default::default()
        };
        assert!(!config.has_wrapper_class());
    }

    #[test]
    fn test_spin_config_get_delay_duration() {
        let config = SpinConfig {
            delay: Some(1500),
            ..Default::default()
        };
        assert_eq!(config.get_delay_duration(), Duration::from_millis(1500));
        
        let config = SpinConfig {
            delay: None,
            ..Default::default()
        };
        assert_eq!(config.get_delay_duration(), Duration::from_millis(0));
    }

    // 测试工具函数
    #[test]
    fn test_validate_spin_props() {
        let valid_props = SpinProps {
            spinning: Some(true),
            size: Some(SpinSize::Large),
            delay: Some(500),
            indicator: None,
            tip: Some("Loading...".to_string()),
            class: None,
            style: None,
            wrapper_class_name: None,
            children: None,
        };
        
        assert!(validate_spin_props(&valid_props).is_ok());
    }

    #[test]
    fn test_validate_spin_props_invalid_delay() {
        let invalid_props = SpinProps {
            spinning: Some(true),
            size: Some(SpinSize::Default),
            delay: Some(-100), // 负数延迟无效
            indicator: None,
            tip: None,
            class: None,
            style: None,
            wrapper_class_name: None,
            children: None,
        };
        
        assert!(validate_spin_props(&invalid_props).is_err());
    }

    #[test]
    fn test_create_spin_config() {
        let props = SpinProps {
            spinning: Some(true),
            size: Some(SpinSize::Small),
            delay: Some(1000),
            indicator: Some("Custom indicator".to_string()),
            tip: Some("Please wait...".to_string()),
            class: None,
            style: None,
            wrapper_class_name: Some("custom-wrapper".to_string()),
            children: None,
        };
        
        let config = create_spin_config(&props);
        assert_eq!(config.spinning, true);
        assert_eq!(config.size, SpinSize::Small);
        assert_eq!(config.delay, Some(1000));
        assert_eq!(config.indicator, Some("Custom indicator".to_string()));
        assert_eq!(config.tip, Some("Please wait...".to_string()));
        assert_eq!(config.wrapper_class_name, Some("custom-wrapper".to_string()));
    }

    #[test]
    fn test_get_spin_container_class() {
        let config = SpinConfig {
            spinning: true,
            size: SpinSize::Large,
            ..Default::default()
        };
        
        let class_name = get_spin_container_class(&config);
        assert!(class_name.contains("ant-spin"));
        assert!(class_name.contains("ant-spin-lg"));
        assert!(class_name.contains("ant-spin-spinning"));
    }

    #[test]
    fn test_get_spin_container_class_not_spinning() {
        let config = SpinConfig {
            spinning: false,
            size: SpinSize::Small,
            ..Default::default()
        };
        
        let class_name = get_spin_container_class(&config);
        assert!(class_name.contains("ant-spin"));
        assert!(class_name.contains("ant-spin-sm"));
        assert!(!class_name.contains("ant-spin-spinning"));
    }

    #[test]
    fn test_get_spin_dot_class() {
        let class_name = get_spin_dot_class();
        assert_eq!(class_name, "ant-spin-dot");
    }

    #[test]
    fn test_get_spin_dot_item_class() {
        let class_name = get_spin_dot_item_class();
        assert_eq!(class_name, "ant-spin-dot-item");
    }

    #[test]
    fn test_get_spin_text_class() {
        let class_name = get_spin_text_class();
        assert_eq!(class_name, "ant-spin-text");
    }

    #[test]
    fn test_get_spin_wrapper_class() {
        let config = SpinConfig {
            wrapper_class_name: Some("custom-wrapper".to_string()),
            ..Default::default()
        };
        
        let class_name = get_spin_wrapper_class(&config);
        assert!(class_name.contains("ant-spin-container"));
        assert!(class_name.contains("custom-wrapper"));
    }

    #[test]
    fn test_get_spin_wrapper_class_no_custom() {
        let config = SpinConfig {
            wrapper_class_name: None,
            ..Default::default()
        };
        
        let class_name = get_spin_wrapper_class(&config);
        assert_eq!(class_name, "ant-spin-container");
    }

    #[test]
    fn test_get_spin_container_styles() {
        let config = SpinConfig {
            size: SpinSize::Large,
            ..Default::default()
        };
        
        let styles = get_spin_container_styles(&config);
        assert!(styles.contains("display: inline-block"));
        assert!(styles.contains("text-align: center"));
    }

    #[test]
    fn test_get_spin_dot_styles() {
        let config = SpinConfig {
            size: SpinSize::Small,
            ..Default::default()
        };
        
        let styles = get_spin_dot_styles(&config);
        assert!(styles.contains("width:"));
        assert!(styles.contains("height:"));
        assert!(styles.contains("14px")); // Small size
    }

    #[test]
    fn test_merge_spin_styles() {
        let base_styles = "display: inline-block;";
        let custom_styles = Some("margin: 10px;".to_string());
        
        let merged = merge_spin_styles(base_styles, custom_styles.as_deref());
        assert!(merged.contains("display: inline-block;"));
        assert!(merged.contains("margin: 10px;"));
        
        let no_custom = merge_spin_styles(base_styles, None);
        assert_eq!(no_custom, base_styles);
    }

    #[test]
    fn test_merge_spin_classes() {
        let base_class = "ant-spin";
        let custom_class = Some("custom-spin".to_string());
        
        let merged = merge_spin_classes(base_class, custom_class.as_deref());
        assert!(merged.contains("ant-spin"));
        assert!(merged.contains("custom-spin"));
        
        let no_custom = merge_spin_classes(base_class, None);
        assert_eq!(no_custom, base_class);
    }

    #[test]
    fn test_calculate_delay_timeout() {
        assert_eq!(calculate_delay_timeout(Some(500)), Duration::from_millis(500));
        assert_eq!(calculate_delay_timeout(Some(0)), Duration::from_millis(0));
        assert_eq!(calculate_delay_timeout(None), Duration::from_millis(0));
    }

    #[test]
    fn test_should_show_spin() {
        let state = SpinState::new(true, false, true);
        assert!(should_show_spin(&state));
        
        let state = SpinState::new(false, false, false);
        assert!(!should_show_spin(&state));
        
        let state = SpinState::new(true, true, false);
        assert!(!should_show_spin(&state)); // 延迟状态
    }

    #[test]
    fn test_generate_spin_keyframes() {
        let keyframes = generate_spin_keyframes();
        assert!(keyframes.contains("@keyframes"));
        assert!(keyframes.contains("antSpinMove"));
        assert!(keyframes.contains("transform: rotate"));
        assert!(keyframes.contains("0%"));
        assert!(keyframes.contains("100%"));
    }

    #[test]
    fn test_generate_spin_dot_animation() {
        let animation = generate_spin_dot_animation(&SpinSize::Default);
        assert!(animation.contains("animation:"));
        assert!(animation.contains("antSpinMove"));
        assert!(animation.contains("1.2s"));
        assert!(animation.contains("infinite"));
        assert!(animation.contains("linear"));
    }

    // 测试样式生成
    #[test]
    fn test_spin_styles_base() {
        let base_styles = SpinStyles::base();
        assert!(base_styles.contains(".ant-spin"));
        assert!(base_styles.contains("display: inline-block"));
        assert!(base_styles.contains("color:"));
        assert!(base_styles.contains("text-align: center"));
        assert!(base_styles.contains("vertical-align: middle"));
    }

    #[test]
    fn test_spin_styles_size_styles() {
        let small_styles = SpinStyles::size_styles(&SpinSize::Small);
        assert!(small_styles.contains(".ant-spin-sm"));
        assert!(small_styles.contains("14px"));
        
        let default_styles = SpinStyles::size_styles(&SpinSize::Default);
        assert!(default_styles.contains(".ant-spin"));
        assert!(default_styles.contains("20px"));
        
        let large_styles = SpinStyles::size_styles(&SpinSize::Large);
        assert!(large_styles.contains(".ant-spin-lg"));
        assert!(large_styles.contains("32px"));
    }

    #[test]
    fn test_spin_styles_spinning_styles() {
        let spinning_styles = SpinStyles::spinning_styles();
        assert!(spinning_styles.contains(".ant-spin-spinning"));
        assert!(spinning_styles.contains("position: static"));
    }

    #[test]
    fn test_spin_styles_dot_styles() {
        let dot_styles = SpinStyles::dot_styles();
        assert!(dot_styles.contains(".ant-spin-dot"));
        assert!(dot_styles.contains("position: relative"));
        assert!(dot_styles.contains("display: inline-block"));
        assert!(dot_styles.contains("transform: rotate(45deg)"));
        assert!(dot_styles.contains("animation:"));
    }

    #[test]
    fn test_spin_styles_dot_item_styles() {
        let dot_item_styles = SpinStyles::dot_item_styles();
        assert!(dot_item_styles.contains(".ant-spin-dot-item"));
        assert!(dot_item_styles.contains("position: absolute"));
        assert!(dot_item_styles.contains("display: block"));
        assert!(dot_item_styles.contains("width: 9px"));
        assert!(dot_item_styles.contains("height: 9px"));
        assert!(dot_item_styles.contains("background-color:"));
        assert!(dot_item_styles.contains("border-radius: 100%"));
        assert!(dot_item_styles.contains("transform: scale(0.75)"));
        assert!(dot_item_styles.contains("transform-origin: 50% 50%"));
        assert!(dot_item_styles.contains("opacity: 0.3"));
        assert!(dot_item_styles.contains("animation:"));
    }

    #[test]
    fn test_spin_styles_text_styles() {
        let text_styles = SpinStyles::text_styles();
        assert!(text_styles.contains(".ant-spin-text"));
        assert!(text_styles.contains("padding-left: 8px"));
        assert!(text_styles.contains("padding-right: 8px"));
        assert!(text_styles.contains("font-size: 14px"));
    }

    #[test]
    fn test_spin_styles_container_styles() {
        let container_styles = SpinStyles::container_styles();
        assert!(container_styles.contains(".ant-spin-container"));
        assert!(container_styles.contains("position: relative"));
        assert!(container_styles.contains("transition: opacity"));
    }

    #[test]
    fn test_spin_styles_blur_styles() {
        let blur_styles = SpinStyles::blur_styles();
        assert!(blur_styles.contains(".ant-spin-blur"));
        assert!(blur_styles.contains("clear: both"));
        assert!(blur_styles.contains("overflow: hidden"));
        assert!(blur_styles.contains("opacity: 0.5"));
        assert!(blur_styles.contains("-webkit-user-select: none"));
        assert!(blur_styles.contains("user-select: none"));
        assert!(blur_styles.contains("pointer-events: none"));
    }

    #[test]
    fn test_spin_styles_nested_loading_styles() {
        let nested_styles = SpinStyles::nested_loading_styles();
        assert!(nested_styles.contains(".ant-spin-nested-loading"));
        assert!(nested_styles.contains("position: relative"));
    }

    #[test]
    fn test_spin_styles_keyframes() {
        let keyframes = SpinStyles::keyframes();
        assert!(keyframes.contains("@keyframes antSpinMove"));
        assert!(keyframes.contains("to {"));
        assert!(keyframes.contains("transform: rotate(405deg)"));
        
        assert!(keyframes.contains("@keyframes antRotate"));
        assert!(keyframes.contains("to {"));
        assert!(keyframes.contains("transform: rotate(360deg)"));
    }

    #[test]
    fn test_spin_styles_responsive_styles() {
        let responsive_styles = SpinStyles::responsive_styles();
        assert!(responsive_styles.contains("@media"));
        assert!(responsive_styles.contains("max-width"));
    }

    // 测试组件渲染
    #[tokio::test]
    async fn test_spin_component_basic_render() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Spin {
                    spinning: true
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 基本渲染测试通过
    }

    #[tokio::test]
    async fn test_spin_component_not_spinning() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Spin {
                    spinning: false
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 非旋转状态渲染测试通过
    }

    #[tokio::test]
    async fn test_spin_component_different_sizes() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                div {
                    Spin {
                        spinning: true,
                        size: SpinSize::Small
                    }
                    Spin {
                        spinning: true,
                        size: SpinSize::Default
                    }
                    Spin {
                        spinning: true,
                        size: SpinSize::Large
                    }
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 不同尺寸渲染测试通过
    }

    #[tokio::test]
    async fn test_spin_component_with_tip() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Spin {
                    spinning: true,
                    tip: "Loading..."
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 带提示文本渲染测试通过
    }

    #[tokio::test]
    async fn test_spin_component_with_delay() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Spin {
                    spinning: true,
                    delay: 500
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 带延迟渲染测试通过
    }

    #[tokio::test]
    async fn test_spin_component_with_custom_indicator() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Spin {
                    spinning: true,
                    indicator: "Custom loading..."
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 自定义指示器渲染测试通过
    }

    #[tokio::test]
    async fn test_spin_component_with_children() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Spin {
                    spinning: true,
                    children: rsx! {
                        div {
                            style: "height: 200px; background: #f0f0f0; padding: 20px;",
                            "Content being loaded..."
                        }
                    }
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 包装内容渲染测试通过
    }

    #[tokio::test]
    async fn test_spin_component_with_wrapper_class() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Spin {
                    spinning: true,
                    wrapper_class_name: "custom-wrapper",
                    children: rsx! {
                        div { "Wrapped content" }
                    }
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 自定义包装类名渲染测试通过
    }

    #[tokio::test]
    async fn test_spin_component_with_custom_styles() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Spin {
                    spinning: true,
                    class: "custom-spin",
                    style: "margin: 20px;"
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 自定义样式渲染测试通过
    }

    #[tokio::test]
    async fn test_spin_component_complex_scenario() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Spin {
                    spinning: true,
                    size: SpinSize::Large,
                    delay: 300,
                    tip: "Please wait...",
                    wrapper_class_name: "loading-wrapper",
                    children: rsx! {
                        div {
                            style: "height: 300px; background: linear-gradient(45deg, #f0f0f0, #e0e0e0); padding: 40px; text-align: center;",
                            h2 { "Data Loading" }
                            p { "This content will be available shortly." }
                            div {
                                style: "margin-top: 20px;",
                                "• Processing data..."
                                br {}
                                "• Validating information..."
                                br {}
                                "• Preparing display..."
                            }
                        }
                    }
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 复杂场景渲染测试通过
    }

    #[tokio::test]
    async fn test_spin_component_minimal_config() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Spin {
                    // 最小配置，只设置spinning
                    spinning: false
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 最小配置渲染测试通过
    }
}
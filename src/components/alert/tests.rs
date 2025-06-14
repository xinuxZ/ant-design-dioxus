//! Alert 组件测试

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::alert::{
        component::*, styles::AlertStyles, types::*, utils::AlertUtils,
    };
    use dioxus::prelude::*;

    /// 测试Alert基础渲染
    #[tokio::test]
    async fn test_alert_basic_render() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Alert {
                    message: "Test message".to_string(),
                    alert_type: AlertType::Info,
                }
            }
        });

        let mutations = dom.rebuild_to_vec();

        // 验证基础结构
        assert!(mutations.iter().any(|m| {
            if let dioxus_core::Mutation::CreateTextNode { text, .. } = m {
                text.contains("Test message")
            } else {
                false
            }
        }));
    }

    /// 测试Alert类型样式
    #[test]
    fn test_alert_types() {
        let types = vec![
            AlertType::Success,
            AlertType::Info,
            AlertType::Warning,
            AlertType::Error,
        ];

        for alert_type in types {
            let props = AlertProps {
                message: "Test".to_string(),
                alert_type: alert_type.clone(),
                ..Default::default()
            };

            let state = AlertState::default();
            let class_names = AlertUtils::generate_class_names(&props, &state);

            match alert_type {
                AlertType::Success => assert!(class_names.contains("ant-alert-success")),
                AlertType::Info => assert!(class_names.contains("ant-alert-info")),
                AlertType::Warning => assert!(class_names.contains("ant-alert-warning")),
                AlertType::Error => assert!(class_names.contains("ant-alert-error")),
            }
        }
    }

    /// 测试可关闭Alert
    #[tokio::test]
    async fn test_alert_closable() {
        let mut closed = false;

        let mut dom = VirtualDom::new(move || {
            rsx! {
                Alert {
                    message: "Closable alert".to_string(),
                    alert_type: AlertType::Info,
                    closable: true,
                    on_close: move |_| {
                        closed = true;
                    },
                }
            }
        });

        let mutations = dom.rebuild_to_vec();

        // 验证关闭按钮存在
        assert!(mutations.iter().any(|m| {
            if let dioxus_core::Mutation::SetAttribute { name, value, .. } = m {
                name == "class"
                    && value
                        .as_any()
                        .downcast_ref::<String>()
                        .map_or(false, |s| s.contains("ant-alert-close-icon"))
            } else {
                false
            }
        }));
    }

    /// 测试Alert图标显示
    #[test]
    fn test_alert_with_icon() {
        let props = AlertProps {
            message: "Test with icon".to_string(),
            alert_type: AlertType::Success,
            show_icon: true,
            ..Default::default()
        };

        let state = AlertState::default();
        let class_names = AlertUtils::generate_class_names(&props, &state);

        assert!(class_names.contains("ant-alert-with-icon"));

        // 测试默认图标获取
        let icon = AlertUtils::get_default_icon(&AlertType::Success);
        assert_eq!(icon, "check-circle");

        let icon = AlertUtils::get_default_icon(&AlertType::Error);
        assert_eq!(icon, "close-circle");
    }

    /// 测试Alert描述文本
    #[tokio::test]
    async fn test_alert_with_description() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Alert {
                    message: "Title".to_string(),
                    description: Some("Description text".to_string()),
                    alert_type: AlertType::Info,
                }
            }
        });

        let mutations = dom.rebuild_to_vec();

        // 验证描述文本存在
        assert!(mutations.iter().any(|m| {
            if let dioxus_core::Mutation::CreateTextNode { text, .. } = m {
                text.contains("Description text")
            } else {
                false
            }
        }));
    }

    /// 测试Banner模式
    #[test]
    fn test_alert_banner() {
        let props = AlertProps {
            message: "Banner alert".to_string(),
            alert_type: AlertType::Warning,
            banner: true,
            ..Default::default()
        };

        let state = AlertState::default();
        let class_names = AlertUtils::generate_class_names(&props, &state);

        assert!(class_names.contains("ant-alert-banner"));
    }

    /// 测试Alert尺寸
    #[test]
    fn test_alert_sizes() {
        let sizes = vec![AlertSize::Small, AlertSize::Default, AlertSize::Large];

        for size in sizes {
            let props = AlertProps {
                message: "Test".to_string(),
                size: size.clone(),
                ..Default::default()
            };

            let state = AlertState::default();
            let class_names = AlertUtils::generate_class_names(&props, &state);

            match size {
                AlertSize::Small => assert!(class_names.contains("ant-alert-small")),
                AlertSize::Default => assert!(
                    !class_names.contains("ant-alert-small")
                        && !class_names.contains("ant-alert-large")
                ),
                AlertSize::Large => assert!(class_names.contains("ant-alert-large")),
            }
        }
    }

    /// 测试Alert动画状态
    #[test]
    fn test_alert_animation_states() {
        let animation_states = vec![
            AnimationState::Idle,
            AnimationState::Entering,
            AnimationState::Entered,
            AnimationState::Exiting,
            AnimationState::Exited,
        ];

        for animation_state in animation_states {
            let props = AlertProps {
                message: "Test".to_string(),
                enable_animation: true,
                ..Default::default()
            };

            let state = AlertState {
                animation_state: animation_state.clone(),
                ..Default::default()
            };

            let class_names = AlertUtils::generate_class_names(&props, &state);

            match animation_state {
                AnimationState::Entering => assert!(class_names.contains("ant-alert-entering")),
                AnimationState::Exiting => assert!(class_names.contains("ant-alert-exiting")),
                _ => {}
            }
        }
    }

    /// 测试ARIA属性生成
    #[test]
    fn test_alert_aria_attributes() {
        let props = AlertProps {
            message: "Accessible alert".to_string(),
            alert_type: AlertType::Error,
            aria_label: Some("Error notification".to_string()),
            ..Default::default()
        };

        let aria_attrs = AlertUtils::generate_aria_attributes(&props);

        assert_eq!(
            aria_attrs.get("aria-label"),
            Some(&"Error notification".to_string())
        );
        assert_eq!(aria_attrs.get("aria-live"), Some(&"polite".to_string()));
        assert_eq!(aria_attrs.get("aria-atomic"), Some(&"true".to_string()));
    }

    /// 测试样式生成
    #[test]
    fn test_alert_styles() {
        let props = AlertProps {
            message: "Styled alert".to_string(),
            alert_type: AlertType::Success,
            color: Some("#custom-color".to_string()),
            background_color: Some("#custom-bg".to_string()),
            ..Default::default()
        };

        let inline_styles = AlertUtils::generate_inline_styles(&props);

        assert!(inline_styles.contains("color: #custom-color"));
        assert!(inline_styles.contains("background-color: #custom-bg"));
    }

    /// 测试颜色验证
    #[test]
    fn test_color_validation() {
        // 有效颜色
        assert!(AlertUtils::validate_color("#ff0000"));
        assert!(AlertUtils::validate_color("rgb(255, 0, 0)"));
        assert!(AlertUtils::validate_color("rgba(255, 0, 0, 0.5)"));
        assert!(AlertUtils::validate_color("red"));

        // 无效颜色
        assert!(!AlertUtils::validate_color("invalid-color"));
        assert!(!AlertUtils::validate_color("#gggggg"));
    }

    /// 测试响应式断点
    #[test]
    fn test_responsive_breakpoints() {
        let breakpoints = AlertUtils::get_responsive_breakpoints();

        assert!(breakpoints.contains_key("xs"));
        assert!(breakpoints.contains_key("sm"));
        assert!(breakpoints.contains_key("md"));
        assert!(breakpoints.contains_key("lg"));
        assert!(breakpoints.contains_key("xl"));
        assert!(breakpoints.contains_key("xxl"));
    }

    /// 测试设备类型检测
    #[test]
    fn test_device_detection() {
        // 这里需要模拟不同的用户代理字符串
        // 实际实现中会检查navigator.userAgent
        let device_type = AlertUtils::get_device_type();

        // 在测试环境中，通常返回Desktop
        assert!(matches!(
            device_type,
            crate::components::alert::utils::DeviceType::Desktop
        ));
    }

    /// 测试Alert配置
    #[test]
    fn test_alert_config() {
        let config = AlertConfig {
            enable_animation: false,
            animation_duration: 500,
            default_show_icon: true,
            default_closable: false,
            ..Default::default()
        };

        // 设置全局配置
        set_global_alert_config(config.clone());

        // 获取全局配置
        let global_config = get_global_alert_config();

        assert_eq!(global_config.enable_animation, false);
        assert_eq!(global_config.animation_duration, 500);
        assert_eq!(global_config.default_show_icon, true);
        assert_eq!(global_config.default_closable, false);
    }

    /// 测试Alert主题
    #[test]
    fn test_alert_theme() {
        let theme = AlertTheme {
            primary_color: "#1890ff".to_string(),
            success_color: "#52c41a".to_string(),
            warning_color: "#faad14".to_string(),
            error_color: "#ff4d4f".to_string(),
            border_radius: 6,
            font_size: 14,
            ..Default::default()
        };

        let merged_theme = merge_alert_theme(&theme, &AlertTheme::default());

        assert_eq!(merged_theme.primary_color, "#1890ff");
        assert_eq!(merged_theme.border_radius, 6);
    }

    /// 测试Alert构建器
    #[test]
    fn test_alert_builder() {
        let props = AlertBuilder::new("Test message")
            .alert_type(AlertType::Success)
            .closable(true)
            .show_icon(true)
            .description("Test description")
            .build();

        assert_eq!(props.message, "Test message");
        assert_eq!(props.alert_type, AlertType::Success);
        assert_eq!(props.closable, true);
        assert_eq!(props.show_icon, true);
        assert_eq!(props.description, Some("Test description".to_string()));
    }

    /// 测试便捷构造函数
    #[test]
    fn test_convenience_constructors() {
        let success_props = success_alert("Success message");
        assert_eq!(success_props.alert_type, AlertType::Success);
        assert_eq!(success_props.show_icon, true);

        let error_props = error_alert("Error message");
        assert_eq!(error_props.alert_type, AlertType::Error);
        assert_eq!(error_props.show_icon, true);

        let closable_props = closable_alert("Closable message", AlertType::Warning);
        assert_eq!(closable_props.closable, true);
        assert_eq!(closable_props.alert_type, AlertType::Warning);

        let banner_props = banner_alert("Banner message", AlertType::Info);
        assert_eq!(banner_props.banner, true);
        assert_eq!(banner_props.alert_type, AlertType::Info);
    }

    /// 测试Alert属性验证
    #[test]
    fn test_alert_props_validation() {
        let valid_props = AlertProps {
            message: "Valid message".to_string(),
            alert_type: AlertType::Info,
            color: Some("#ff0000".to_string()),
            animation_duration: 300,
            ..Default::default()
        };

        let validation_result = validate_alert_props(&valid_props);
        assert!(validation_result.is_ok());

        let invalid_props = AlertProps {
            message: "".to_string(),                  // 空消息
            color: Some("invalid-color".to_string()), // 无效颜色
            animation_duration: 0,                    // 无效动画时长
            ..Default::default()
        };

        let validation_result = validate_alert_props(&invalid_props);
        assert!(validation_result.is_err());
    }

    /// 测试Alert工具函数
    #[test]
    fn test_alert_utils() {
        // 测试唯一ID生成
        let id1 = AlertUtils::generate_unique_id();
        let id2 = AlertUtils::generate_unique_id();
        assert_ne!(id1, id2);

        // 测试防抖函数
        let mut counter = 0;
        let debounced = AlertUtils::debounce(
            move || {
                counter += 1;
            },
            100,
        );

        // 多次调用，但由于防抖，只会执行一次
        debounced();
        debounced();
        debounced();

        // 这里需要等待防抖时间，在实际测试中可能需要异步处理

        // 测试节流函数
        let mut throttle_counter = 0;
        let throttled = AlertUtils::throttle(
            move || {
                throttle_counter += 1;
            },
            100,
        );

        throttled();
        throttled();
        throttled();
    }

    /// 测试Alert样式系统
    #[test]
    fn test_alert_styles_system() {
        let styles = AlertStyles::new();

        // 测试基础样式
        let base_styles = styles.get_base_styles();
        assert!(base_styles.contains("ant-alert"));

        // 测试类型样式
        let success_styles = styles.get_type_styles(&AlertType::Success);
        assert!(success_styles.contains("ant-alert-success"));

        // 测试尺寸样式
        let small_styles = styles.get_size_styles(&AlertSize::Small);
        assert!(small_styles.contains("ant-alert-small"));

        // 测试状态样式
        let closable_styles = styles.get_state_styles(true, false, false, false, false);
        assert!(closable_styles.contains("ant-alert-closable"));
    }

    /// 测试Alert动画管理器
    #[test]
    fn test_alert_animation_manager() {
        let manager = crate::components::alert::utils::AlertAnimationManager::new();

        // 测试动画开始
        let animation_id = manager.start_animation("fade-in", 300);
        assert!(!animation_id.is_empty());

        // 测试动画清除
        manager.clear_animation(&animation_id);
    }

    /// 测试Alert事件处理器
    #[test]
    fn test_alert_event_handler() {
        let handler = crate::components::alert::utils::AlertEventHandler::new();

        // 这里需要模拟DOM事件，在实际测试中可能需要更复杂的设置
        // 测试关闭事件处理
        // 测试键盘事件处理
        // 测试焦点事件处理
    }

    /// 性能测试 - Alert渲染性能
    #[tokio::test]
    async fn test_alert_render_performance() {
        use std::time::Instant;

        let start = Instant::now();

        // 渲染大量Alert组件
        for i in 0..1000 {
            let mut dom = VirtualDom::new(move || {
                rsx! {
                    Alert {
                        message: format!("Alert {}", i),
                        alert_type: AlertType::Info,
                    }
                }
            });

            dom.rebuild_to_vec();
        }

        let duration = start.elapsed();

        // 确保渲染1000个Alert组件在合理时间内完成（比如1秒）
        assert!(
            duration.as_secs() < 1,
            "Alert rendering took too long: {:?}",
            duration
        );
    }

    /// 内存测试 - Alert内存使用
    #[test]
    fn test_alert_memory_usage() {
        // 创建大量Alert属性，确保没有内存泄漏
        let mut alerts = Vec::new();

        for i in 0..10000 {
            let props = AlertProps {
                message: format!("Alert {}", i),
                alert_type: AlertType::Info,
                ..Default::default()
            };
            alerts.push(props);
        }

        // 清理
        alerts.clear();

        // 在实际测试中，可能需要更精确的内存监控
    }

    /// 集成测试 - Alert与其他组件的交互
    #[tokio::test]
    async fn test_alert_integration() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                div {
                    Alert {
                        message: "First alert".to_string(),
                        alert_type: AlertType::Success,
                    }
                    Alert {
                        message: "Second alert".to_string(),
                        alert_type: AlertType::Warning,
                    }
                }
            }
        });

        let mutations = dom.rebuild_to_vec();

        // 验证多个Alert可以正常渲染
        let text_nodes: Vec<_> = mutations
            .iter()
            .filter_map(|m| {
                if let dioxus_core::Mutation::CreateTextNode { text, .. } = m {
                    Some(text.as_str())
                } else {
                    None
                }
            })
            .collect();

        assert!(text_nodes.iter().any(|&text| text.contains("First alert")));
        assert!(text_nodes.iter().any(|&text| text.contains("Second alert")));
    }
}

/// 基准测试模块
#[cfg(test)]
mod benchmarks {
    use super::*;
    use std::time::Instant;

    /// Alert组件创建基准测试
    #[test]
    fn bench_alert_creation() {
        let start = Instant::now();

        for _ in 0..10000 {
            let _props = AlertProps {
                message: "Benchmark alert".to_string(),
                alert_type: AlertType::Info,
                ..Default::default()
            };
        }

        let duration = start.elapsed();
        println!("Alert creation benchmark: {:?}", duration);
    }

    /// Alert样式生成基准测试
    #[test]
    fn bench_alert_style_generation() {
        let props = AlertProps {
            message: "Benchmark alert".to_string(),
            alert_type: AlertType::Success,
            show_icon: true,
            closable: true,
            ..Default::default()
        };

        let state = AlertState::default();

        let start = Instant::now();

        for _ in 0..10000 {
            let _class_names = AlertUtils::generate_class_names(&props, &state);
            let _inline_styles = AlertUtils::generate_inline_styles(&props);
        }

        let duration = start.elapsed();
        println!("Alert style generation benchmark: {:?}", duration);
    }

    /// Alert ARIA属性生成基准测试
    #[test]
    fn bench_alert_aria_generation() {
        let props = AlertProps {
            message: "Benchmark alert".to_string(),
            alert_type: AlertType::Error,
            aria_label: Some("Error notification".to_string()),
            ..Default::default()
        };

        let start = Instant::now();

        for _ in 0..10000 {
            let _aria_attrs = AlertUtils::generate_aria_attributes(&props);
        }

        let duration = start.elapsed();
        println!("Alert ARIA generation benchmark: {:?}", duration);
    }
}

/// 模糊测试模块
#[cfg(test)]
mod fuzz_tests {
    use super::*;
    use rand::Rng;

    /// Alert属性模糊测试
    #[test]
    fn fuzz_alert_props() {
        let mut rng = rand::thread_rng();

        for _ in 0..1000 {
            let message = generate_random_string(&mut rng, 1, 1000);
            let alert_type = generate_random_alert_type(&mut rng);
            let color = if rng.gen_bool(0.5) {
                Some(generate_random_color(&mut rng))
            } else {
                None
            };

            let props = AlertProps {
                message,
                alert_type,
                color,
                closable: rng.gen_bool(0.5),
                show_icon: rng.gen_bool(0.5),
                banner: rng.gen_bool(0.5),
                enable_animation: rng.gen_bool(0.5),
                animation_duration: rng.gen_range(0..=5000),
                ..Default::default()
            };

            // 验证属性不会导致panic
            let state = AlertState::default();
            let _class_names = AlertUtils::generate_class_names(&props, &state);
            let _inline_styles = AlertUtils::generate_inline_styles(&props);
            let _aria_attrs = AlertUtils::generate_aria_attributes(&props);
        }
    }

    fn generate_random_string(rng: &mut impl Rng, min_len: usize, max_len: usize) -> String {
        let len = rng.gen_range(min_len..=max_len);
        (0..len)
            .map(|_| rng.gen_range(b'a'..=b'z') as char)
            .collect()
    }

    fn generate_random_alert_type(rng: &mut impl Rng) -> AlertType {
        match rng.gen_range(0..4) {
            0 => AlertType::Success,
            1 => AlertType::Info,
            2 => AlertType::Warning,
            _ => AlertType::Error,
        }
    }

    fn generate_random_color(rng: &mut impl Rng) -> String {
        if rng.gen_bool(0.5) {
            // 生成十六进制颜色
            format!("#{:06x}", rng.gen_range(0..0xffffff))
        } else {
            // 生成RGB颜色
            format!(
                "rgb({}, {}, {})",
                rng.gen_range(0..256),
                rng.gen_range(0..256),
                rng.gen_range(0..256)
            )
        }
    }
}

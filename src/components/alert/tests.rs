//!
//! Alert 组件测试
//!
//! 提供 Alert 组件的单元测试和集成测试。

#[cfg(test)]
mod tests {
    use super::super::*;
    use dioxus::prelude::*;
    use dioxus_testing::*;

    /// 测试 Alert 组件的默认渲染
    #[test]
    fn test_alert_default_render() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Alert {
                    message: "Test message".to_string()
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();

        assert!(html.contains("ant-alert"));
        assert!(html.contains("ant-alert-info")); // 默认类型为 info
        assert!(html.contains("Test message"));
        assert!(html.contains("role=\"alert\""));
        assert!(html.contains("aria-live=\"polite\""));
    }

    /// 测试 Alert 组件的国际化功能
    #[test]
    fn test_alert_i18n() {
        use crate::locale::{Locale, LocaleProvider};

        // 使用中文语言环境
        let mut dom = VirtualDom::new(|| {
            rsx! {
                LocaleProvider {
                    locale: Signal::new(Locale::ZhCN),
                    Alert {
                        message: "测试消息".to_string(),
                        closable: true
                    }
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("测试消息"));
        assert!(html.contains("aria-label=\"关闭\""));

        // 使用英文语言环境
        let mut dom = VirtualDom::new(|| {
            rsx! {
                LocaleProvider {
                    locale: Signal::new(Locale::En),
                    Alert {
                        message: "Test message".to_string(),
                        closable: true
                    }
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("Test message"));
        assert!(html.contains("aria-label=\"Close\""));
    }

    /// 测试不同 Alert 类型的渲染
    #[test]
    fn test_alert_types() {
        // Success 类型
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Alert {
                    alert_type: AlertType::Success,
                    message: "Success message".to_string()
                }
            }
        });
        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("ant-alert-success"));

        // Warning 类型
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Alert {
                    alert_type: AlertType::Warning,
                    message: "Warning message".to_string()
                }
            }
        });
        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("ant-alert-warning"));

        // Error 类型
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Alert {
                    alert_type: AlertType::Error,
                    message: "Error message".to_string()
                }
            }
        });
        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("ant-alert-error"));
    }

    /// 测试 Alert 尺寸
    #[test]
    fn test_alert_sizes() {
        // Small 尺寸
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Alert {
                    size: AlertSize::Small,
                    message: "Small alert".to_string()
                }
            }
        });
        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("ant-alert-sm"));

        // Large 尺寸
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Alert {
                    size: AlertSize::Large,
                    message: "Large alert".to_string()
                }
            }
        });
        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("ant-alert-lg"));
    }

    /// 测试 Alert 变体
    #[test]
    fn test_alert_variants() {
        // Outlined 变体
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Alert {
                    variant: AlertVariant::Outlined,
                    message: "Outlined alert".to_string()
                }
            }
        });
        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("ant-alert-outlined"));
    }

    /// 测试带描述的 Alert
    #[test]
    fn test_alert_with_description() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Alert {
                    message: "Alert message".to_string(),
                    description: "Alert description".to_string()
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();

        assert!(html.contains("ant-alert-with-description"));
        assert!(html.contains("Alert message"));
        assert!(html.contains("Alert description"));
        assert!(html.contains("ant-alert-description"));
    }

    /// 测试带图标的 Alert
    #[test]
    fn test_alert_with_icon() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Alert {
                    show_icon: true,
                    message: "Alert with icon".to_string()
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();

        assert!(html.contains("ant-alert-with-icon"));
        assert!(html.contains("ant-alert-icon"));
    }

    /// 测试可关闭的 Alert
    #[test]
    fn test_alert_closable() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Alert {
                    closable: true,
                    message: "Closable alert".to_string()
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();

        assert!(html.contains("ant-alert-closable"));
        assert!(html.contains("ant-alert-close-icon"));
        assert!(html.contains("aria-label=\"Close\""));
    }

    /// 测试自定义关闭文本
    #[test]
    fn test_alert_custom_close_text() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Alert {
                    closable: true,
                    close_text: "关闭".to_string(),
                    message: "Alert with custom close text".to_string()
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();

        assert!(html.contains("关闭"));
        assert!(html.contains("ant-alert-close-text"));
    }

    /// 测试横幅模式
    #[test]
    fn test_alert_banner() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Alert {
                    banner: true,
                    message: "Banner alert".to_string()
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();

        assert!(html.contains("ant-alert-banner"));
    }

    /// 测试无边框模式
    #[test]
    fn test_alert_no_border() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Alert {
                    bordered: false,
                    message: "No border alert".to_string()
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();

        assert!(html.contains("ant-alert-no-border"));
    }

    /// 测试禁用状态
    #[test]
    fn test_alert_disabled() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Alert {
                    disabled: true,
                    closable: true,
                    message: "Disabled alert".to_string()
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();

        assert!(html.contains("ant-alert-disabled"));
        assert!(html.contains("disabled"));
    }

    /// 测试不可见状态
    #[test]
    fn test_alert_invisible() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Alert {
                    visible: false,
                    message: "Invisible alert".to_string()
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();

        assert!(html.contains("display: none"));
    }

    /// 测试自定义类名和样式
    #[test]
    fn test_alert_custom_class_and_style() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Alert {
                    class: "custom-alert".to_string(),
                    style: "margin-top: 20px;".to_string(),
                    message: "Custom styled alert".to_string()
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();

        assert!(html.contains("custom-alert"));
        assert!(html.contains("margin-top: 20px"));
    }

    /// 测试自定义前缀类名
    #[test]
    fn test_alert_custom_prefix() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Alert {
                    prefix_cls: "my-alert".to_string(),
                    message: "Custom prefix alert".to_string()
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();

        assert!(html.contains("my-alert"));
        assert!(html.contains("my-alert-info"));
    }

    /// 测试 ErrorBoundaryAlert 组件
    #[test]
    fn test_error_boundary_alert() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                ErrorBoundaryAlert {
                    error_message: "Something went wrong".to_string(),
                    error_details: "Detailed error information".to_string()
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();

        assert!(html.contains("ant-alert-error"));
        assert!(html.contains("Something went wrong"));
        assert!(html.contains("ant-alert-error-actions"));
        assert!(html.contains("重新加载"));
    }

    /// 测试 AlertStyleGenerator
    #[test]
    fn test_alert_style_generator() {
        let generator = AlertStyleGenerator::new()
            .with_type(AlertType::Success)
            .with_size(AlertSize::Large)
            .with_variant(AlertVariant::Outlined)
            .with_show_icon(true)
            .with_closable(true)
            .with_banner(true);

        let class_names = generator.generate_class_names();

        assert!(class_names.contains(&"ant-alert".to_string()));
        assert!(class_names.contains(&"ant-alert-success".to_string()));
        assert!(class_names.contains(&"ant-alert-lg".to_string()));
        assert!(class_names.contains(&"ant-alert-outlined".to_string()));
        assert!(class_names.contains(&"ant-alert-with-icon".to_string()));
        assert!(class_names.contains(&"ant-alert-closable".to_string()));
        assert!(class_names.contains(&"ant-alert-banner".to_string()));
    }

    /// 测试样式生成器的内联样式
    #[test]
    fn test_alert_style_generator_inline_styles() {
        let generator = AlertStyleGenerator::new()
            .with_disabled(true)
            .with_visible(false);

        let inline_styles = generator.generate_inline_styles();

        assert!(inline_styles.contains("display: none"));
        assert!(inline_styles.contains("opacity: 0.6"));
        assert!(inline_styles.contains("pointer-events: none"));
    }

    /// 测试 AlertType 枚举
    #[test]
    fn test_alert_type_enum() {
        assert_eq!(AlertType::from("success"), AlertType::Success);
        assert_eq!(AlertType::from("info"), AlertType::Info);
        assert_eq!(AlertType::from("warning"), AlertType::Warning);
        assert_eq!(AlertType::from("error"), AlertType::Error);
        assert_eq!(AlertType::from("unknown"), AlertType::Info); // 默认值
    }

    /// 测试 AlertSize 枚举
    #[test]
    fn test_alert_size_enum() {
        assert_eq!("small".into(), AlertSize::Small);
        assert_eq!("default".into(), AlertSize::Default);
        assert_eq!("large".into(), AlertSize::Large);
        assert_eq!("unknown".into(), AlertSize::Default); // 默认值
    }

    /// 测试 AlertVariant 枚举
    #[test]
    fn test_alert_variant_enum() {
        assert_eq!(AlertVariant::from("filled"), AlertVariant::Filled);
        assert_eq!(AlertVariant::from("outlined"), AlertVariant::Outlined);
        assert_eq!(AlertVariant::from("unknown"), AlertVariant::Filled); // 默认值
    }

    /// 测试 AlertAnimationState 枚举
    #[test]
    fn test_alert_animation_state() {
        let state = AlertAnimationState::Entering;
        assert!(matches!(state, AlertAnimationState::Entering));

        let state = AlertAnimationState::Entered;
        assert!(matches!(state, AlertAnimationState::Entered));

        let state = AlertAnimationState::Exiting;
        assert!(matches!(state, AlertAnimationState::Exiting));

        let state = AlertAnimationState::Exited;
        assert!(matches!(state, AlertAnimationState::Exited));
    }

    /// 测试 AlertState 结构体
    #[test]
    fn test_alert_state() {
        let state = AlertState {
            visible: true,
            animation_state: AlertAnimationState::Entered,
            closing: false,
        };

        assert!(state.visible);
        assert!(!state.closing);
        assert!(matches!(
            state.animation_state,
            AlertAnimationState::Entered
        ));
    }

    /// 测试 AlertStyleConfig 结构体
    #[test]
    fn test_alert_style_config() {
        let config = AlertStyleConfig {
            alert_type: AlertType::Warning,
            size: AlertSize::Large,
            variant: AlertVariant::Outlined,
            show_icon: true,
            has_description: true,
            closable: true,
            banner: false,
            bordered: true,
            disabled: false,
            visible: true,
        };

        assert_eq!(config.alert_type, AlertType::Warning);
        assert_eq!(config.size, AlertSize::Large);
        assert_eq!(config.variant, AlertVariant::Outlined);
        assert!(config.show_icon);
        assert!(config.has_description);
        assert!(config.closable);
        assert!(!config.banner);
        assert!(config.bordered);
        assert!(!config.disabled);
        assert!(config.visible);
    }

    /// 测试默认配置
    #[test]
    fn test_alert_style_config_default() {
        let config = AlertStyleConfig::default();

        assert_eq!(config.alert_type, AlertType::Info);
        assert_eq!(config.size, AlertSize::Default);
        assert_eq!(config.variant, AlertVariant::Filled);
        assert!(!config.show_icon);
        assert!(!config.has_description);
        assert!(!config.closable);
        assert!(!config.banner);
        assert!(config.bordered);
        assert!(!config.disabled);
        assert!(config.visible);
    }
}

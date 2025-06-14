#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::alert::styles::*;
    use crate::components::alert::types::*;
    use crate::components::alert::utils::*;
    use dioxus::prelude::*;
    use std::time::Duration;

    // 测试 AlertType 枚举
    #[test]
    fn test_alert_type_css_class() {
        assert_eq!(AlertType::Success.get_css_class(), "ant-alert-success");
        assert_eq!(AlertType::Info.get_css_class(), "ant-alert-info");
        assert_eq!(AlertType::Warning.get_css_class(), "ant-alert-warning");
        assert_eq!(AlertType::Error.get_css_class(), "ant-alert-error");
    }

    #[test]
    fn test_alert_type_default() {
        assert_eq!(AlertType::default(), AlertType::Info);
    }

    #[test]
    fn test_alert_type_display() {
        assert_eq!(format!("{}", AlertType::Success), "success");
        assert_eq!(format!("{}", AlertType::Info), "info");
        assert_eq!(format!("{}", AlertType::Warning), "warning");
        assert_eq!(format!("{}", AlertType::Error), "error");
    }

    // 测试 AlertSize 枚举
    #[test]
    fn test_alert_size_default() {
        assert_eq!(AlertSize::default(), AlertSize::Default);
    }

    #[test]
    fn test_alert_size_display() {
        assert_eq!(format!("{}", AlertSize::Small), "Small");
        assert_eq!(format!("{}", AlertSize::Default), "default");
        assert_eq!(format!("{}", AlertSize::Large), "large");
    }

    // 测试 AnimationState 枚举
    #[test]
    fn test_animation_state_default() {
        assert_eq!(AnimationState::default(), AnimationState::Idle);
    }

    #[test]
    fn test_animation_state_display() {
        assert_eq!(format!("{}", AnimationState::Idle), "idle");
        assert_eq!(format!("{}", AnimationState::Entering), "entering");
        assert_eq!(format!("{}", AnimationState::Entered), "entered");
        assert_eq!(format!("{}", AnimationState::Exiting), "exiting");
        assert_eq!(format!("{}", AnimationState::Exited), "exited");
    }

    // 测试 AlertState 结构体
    #[test]
    fn test_alert_state_default() {
        let state = AlertState::default();
        assert!(!state.visible);
        assert!(!state.closing);
        assert!(!state.mounted);
        assert_eq!(state.animation_state, AnimationState::Idle);
    }

    #[test]
    fn test_alert_state_new() {
        let state = AlertState {
            visible: true,
            closing: false,
            mounted: true,
            animation_state: AnimationState::Entered,
        };
        assert!(state.visible);
        assert!(!state.closing);
        assert!(state.mounted);
        assert_eq!(state.animation_state, AnimationState::Entered);
    }

    // 测试 AlertUtils
    #[test]
    fn test_alert_utils_generate_class_names() {
        let props = AlertProps {
            message: "Test message".to_string(),
            alert_type: AlertType::Success,
            closable: true,
            show_icon: true,
            description: Some("Test description".to_string()),
            banner: true,
            show_border: false,
            size: AlertSize::Large,
            ..Default::default()
        };

        let state = AlertState {
            visible: true,
            closing: false,
            mounted: true,
            animation_state: AnimationState::Entered,
        };

        let class_names = AlertUtils::generate_class_names(&props, &state);

        assert!(class_names.contains("ant-alert"));
        assert!(class_names.contains("ant-alert-success"));
        assert!(class_names.contains("ant-alert-closable"));
        assert!(class_names.contains("ant-alert-with-icon"));
        assert!(class_names.contains("ant-alert-with-description"));
        assert!(class_names.contains("ant-alert-banner"));
        assert!(class_names.contains("ant-alert-no-border"));
        assert!(class_names.contains("ant-alert-large"));
    }

    #[test]
    fn test_alert_utils_generate_class_names_minimal() {
        let props = AlertProps {
            message: "Test message".to_string(),
            ..Default::default()
        };

        let state = AlertState::default();
        let class_names = AlertUtils::generate_class_names(&props, &state);

        assert!(class_names.contains("ant-alert"));
        assert!(class_names.contains("ant-alert-info")); // default type
        assert!(!class_names.contains("ant-alert-closable"));
        assert!(!class_names.contains("ant-alert-with-icon"));
    }

    #[test]
    fn test_alert_utils_get_icon_type() {
        assert_eq!(
            AlertUtils::get_icon_type(&AlertType::Success),
            "check-circle"
        );
        assert_eq!(AlertUtils::get_icon_type(&AlertType::Info), "info-circle");
        assert_eq!(
            AlertUtils::get_icon_type(&AlertType::Warning),
            "exclamation-circle"
        );
        assert_eq!(AlertUtils::get_icon_type(&AlertType::Error), "close-circle");
    }

    #[test]
    fn test_alert_utils_should_show_close_button() {
        let props_closable = AlertProps {
            message: "Test".to_string(),
            closable: true,
            ..Default::default()
        };
        assert!(AlertUtils::should_show_close_button(&props_closable));

        let props_not_closable = AlertProps {
            message: "Test".to_string(),
            closable: false,
            ..Default::default()
        };
        assert!(!AlertUtils::should_show_close_button(&props_not_closable));
    }

    #[test]
    fn test_alert_utils_should_show_icon() {
        let props_with_icon = AlertProps {
            message: "Test".to_string(),
            show_icon: true,
            ..Default::default()
        };
        assert!(AlertUtils::should_show_icon(&props_with_icon));

        let props_without_icon = AlertProps {
            message: "Test".to_string(),
            show_icon: false,
            ..Default::default()
        };
        assert!(!AlertUtils::should_show_icon(&props_without_icon));
    }

    #[test]
    fn test_alert_utils_get_animation_duration() {
        let props = AlertProps {
            message: "Test".to_string(),
            animation_duration: 500,
            ..Default::default()
        };
        assert_eq!(AlertUtils::get_animation_duration(&props), 500);
    }

    #[test]
    fn test_alert_utils_is_animation_enabled() {
        let props_enabled = AlertProps {
            message: "Test".to_string(),
            enable_animation: true,
            ..Default::default()
        };
        assert!(AlertUtils::is_animation_enabled(&props_enabled));

        let props_disabled = AlertProps {
            message: "Test".to_string(),
            enable_animation: false,
            ..Default::default()
        };
        assert!(!AlertUtils::is_animation_enabled(&props_disabled));
    }

    // 测试 AlertAnimationManager
    #[test]
    fn test_alert_animation_manager_get_animation_class() {
        assert_eq!(
            AlertAnimationManager::get_animation_class(&AnimationState::Idle),
            ""
        );
        assert_eq!(
            AlertAnimationManager::get_animation_class(&AnimationState::Entering),
            "ant-alert-motion-enter"
        );
        assert_eq!(
            AlertAnimationManager::get_animation_class(&AnimationState::Entered),
            "ant-alert-motion-enter-active"
        );
        assert_eq!(
            AlertAnimationManager::get_animation_class(&AnimationState::Exiting),
            "ant-alert-motion-leave"
        );
        assert_eq!(
            AlertAnimationManager::get_animation_class(&AnimationState::Exited),
            "ant-alert-motion-leave-active"
        );
    }

    #[test]
    fn test_alert_animation_manager_should_apply_animation() {
        let props_enabled = AlertProps {
            message: "Test".to_string(),
            enable_animation: true,
            ..Default::default()
        };
        assert!(AlertAnimationManager::should_apply_animation(
            &props_enabled
        ));

        let props_disabled = AlertProps {
            message: "Test".to_string(),
            enable_animation: false,
            ..Default::default()
        };
        assert!(!AlertAnimationManager::should_apply_animation(
            &props_disabled
        ));
    }

    // 测试 AlertEventHandler
    #[test]
    fn test_alert_event_handler_should_handle_keyboard() {
        let props_closable = AlertProps {
            message: "Test".to_string(),
            closable: true,
            ..Default::default()
        };
        assert!(AlertEventHandler::should_handle_keyboard(&props_closable));

        let props_not_closable = AlertProps {
            message: "Test".to_string(),
            closable: false,
            ..Default::default()
        };
        assert!(!AlertEventHandler::should_handle_keyboard(
            &props_not_closable
        ));
    }

    // 测试 AlertStyles
    #[test]
    fn test_alert_styles_base() {
        let base_styles = AlertStyles::base();
        assert!(base_styles.contains(".ant-alert"));
        assert!(base_styles.contains("position: relative"));
        assert!(base_styles.contains("display: flex"));
    }

    #[test]
    fn test_alert_styles_type_styles() {
        let success_styles = AlertStyles::type_styles();
        assert!(success_styles.contains("ant-alert-success"));

        let info_styles = AlertStyles::type_styles();
        assert!(info_styles.contains("ant-alert-info"));

        let warning_styles = AlertStyles::type_styles();
        assert!(warning_styles.contains("ant-alert-warning"));

        let error_styles = AlertStyles::type_styles();
        assert!(error_styles.contains("ant-alert-error"));
    }

    #[test]
    fn test_alert_styles_size_styles() {
        let small_styles = AlertStyles::size_styles();
        assert!(small_styles.contains("ant-alert-small"));

        let large_styles = AlertStyles::size_styles();
        assert!(large_styles.contains("ant-alert-large"));
    }

    #[test]
    fn test_alert_styles_animation_styles() {
        let animation_styles = AlertStyles::animation_styles();
        assert!(animation_styles.contains("@keyframes"));
        assert!(animation_styles.contains("ant-alert-motion"));
    }

    #[test]
    fn test_alert_styles_responsive_styles() {
        let responsive_styles = AlertStyles::responsive_styles();
        assert!(responsive_styles.contains("@media"));
    }

    #[test]
    fn test_alert_styles_dark_theme_styles() {
        let dark_styles = AlertStyles::dark_theme_styles();
        assert!(dark_styles.contains("[data-theme='dark']"));
    }

    // 测试 AlertProps 默认值
    #[test]
    fn test_alert_props_default() {
        let props = AlertProps::default();
        assert_eq!(props.message, "");
        assert_eq!(props.alert_type, AlertType::Info);
        assert!(!props.closable);
        assert!(props.description.is_none());
        assert!(!props.show_icon);
        assert!(props.icon.is_none());
        assert!(props.action.is_none());
        assert!(!props.banner);
        assert!(props.class_name.is_none());
        assert!(props.style.is_none());
        assert!(props.on_close.is_none());
        assert!(props.on_key_down.is_none());
        assert!(props.after_close.is_none());
        assert!(props.visible);
        assert!(props.enable_animation);
        assert_eq!(props.animation_duration, 300);
        assert!(props.auto_focus);
        assert_eq!(props.size, AlertSize::Default);
        assert!(props.show_border);
        assert!(props.role.len() > 0);
        assert!(props.aria_label.is_none());
        // assert!(props.aria_describedby.is_none());
    }

    // 测试组件渲染
    #[tokio::test]
    async fn test_alert_component_basic_render() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Alert {
                    message: "Test Alert Message",
                    alert_type: AlertType::Success,
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 基本渲染测试通过
    }

    #[tokio::test]
    async fn test_alert_component_with_description() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Alert {
                    message: "Test Alert Message",
                    description: "Test description",
                    alert_type: AlertType::Info,
                    show_icon: true,
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 带描述的渲染测试通过
    }

    #[tokio::test]
    async fn test_alert_component_closable() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Alert {
                    message: "Closable Alert",
                    closable: true,
                    on_close: |_| {},
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 可关闭的渲染测试通过
    }

    #[tokio::test]
    async fn test_alert_component_banner() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Alert {
                    message: "Banner Alert",
                    banner: true,
                    alert_type: AlertType::Warning,
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 横幅模式渲染测试通过
    }

    #[tokio::test]
    async fn test_alert_component_with_action() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Alert {
                    message: "Alert with action",
                    action: rsx! {
                        button { "Action" }
                    },
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 带操作按钮的渲染测试通过
    }

    #[tokio::test]
    async fn test_alert_component_different_sizes() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                div {
                    Alert {
                        message: "Small Alert",
                        size: AlertSize::Small,
                    }
                    Alert {
                        message: "Default Alert",
                        size: AlertSize::Default,
                    }
                    Alert {
                        message: "Large Alert",
                        size: AlertSize::Large,
                    }
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 不同尺寸渲染测试通过
    }

    #[tokio::test]
    async fn test_alert_component_all_types() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                div {
                    Alert {
                        message: "Success Alert",
                        alert_type: AlertType::Success,
                    }
                    Alert {
                        message: "Info Alert",
                        alert_type: AlertType::Info,
                    }
                    Alert {
                        message: "Warning Alert",
                        alert_type: AlertType::Warning,
                    }
                    Alert {
                        message: "Error Alert",
                        alert_type: AlertType::Error,
                    }
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 所有类型渲染测试通过
    }

    #[tokio::test]
    async fn test_alert_component_invisible() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Alert {
                    message: "Invisible Alert",
                    visible: false,
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 不可见状态渲染测试通过
    }

    #[tokio::test]
    async fn test_alert_component_with_custom_styles() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Alert {
                    message: "Custom Styled Alert",
                    class_name: "custom-alert",
                    style: "margin: 10px; padding: 20px;",
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 自定义样式渲染测试通过
    }

    #[tokio::test]
    async fn test_alert_component_accessibility() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Alert {
                    message: "Accessible Alert",
                    role: "alert",
                    aria_label: "Important notification",
                    aria_describedby: "alert-description",
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 无障碍功能渲染测试通过
    }
}

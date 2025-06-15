#[cfg(test)]
mod button_tests {
    use crate::components::button::*;
    use dioxus::prelude::*;

    #[test]
    fn test_button_renders() {
        fn app() -> Element {
            rsx! {
                Button {
                    "Click me"
                }
            }
        }

        let mut vdom = VirtualDom::new_with_props(app, ());
        let _ = vdom.rebuild();

        // 验证按钮已渲染
        assert!(vdom.body().contains("Click me"));
    }

    #[test]
    fn test_button_types() {
        fn app() -> Element {
            rsx! {
                div {
                    Button {
                        button_type: ButtonType::Primary,
                        "Primary Button"
                    }
                    Button {
                        button_type: ButtonType::Default,
                        "Default Button"
                    }
                    Button {
                        button_type: ButtonType::Dashed,
                        "Dashed Button"
                    }
                    Button {
                        button_type: ButtonType::Text,
                        "Text Button"
                    }
                    Button {
                        button_type: ButtonType::Link,
                        "Link Button"
                    }
                }
            }
        }

        let mut vdom = VirtualDom::new_with_props(app, ());
        let _ = vdom.rebuild();

        // 验证不同类型的按钮已渲染
        let html = vdom.body();
        assert!(html.contains("Primary Button"));
        assert!(html.contains("Default Button"));
        assert!(html.contains("Dashed Button"));
        assert!(html.contains("Text Button"));
        assert!(html.contains("Link Button"));

        // 验证类名
        assert!(html.contains("ant-btn-primary"));
        assert!(html.contains("ant-btn-default"));
        assert!(html.contains("ant-btn-dashed"));
        assert!(html.contains("ant-btn-text"));
        assert!(html.contains("ant-btn-link"));
    }

    #[test]
    fn test_button_sizes() {
        fn app() -> Element {
            rsx! {
                div {
                    Button {
                        size: ButtonSize::Large,
                        "Large Button"
                    }
                    Button {
                        size: ButtonSize::Middle,
                        "Middle Button"
                    }
                    Button {
                        size: ButtonSize::Small,
                        "Small Button"
                    }
                }
            }
        }

        let mut vdom = VirtualDom::new_with_props(app, ());
        let _ = vdom.rebuild();

        // 验证不同大小的按钮已渲染
        let html = vdom.body();
        assert!(html.contains("Large Button"));
        assert!(html.contains("Middle Button"));
        assert!(html.contains("Small Button"));

        // 验证类名
        assert!(html.contains("ant-btn-large"));
        assert!(html.contains("ant-btn-small"));
    }

    #[test]
    fn test_button_shapes() {
        fn app() -> Element {
            rsx! {
                div {
                    Button {
                        shape: ButtonShape::Default,
                        "Default Shape"
                    }
                    Button {
                        shape: ButtonShape::Circle,
                        "C"
                    }
                    Button {
                        shape: ButtonShape::Round,
                        "Round Shape"
                    }
                }
            }
        }

        let mut vdom = VirtualDom::new_with_props(app, ());
        let _ = vdom.rebuild();

        // 验证不同形状的按钮已渲染
        let html = vdom.body();
        assert!(html.contains("Default Shape"));
        assert!(html.contains("C"));
        assert!(html.contains("Round Shape"));

        // 验证类名
        assert!(html.contains("ant-btn-circle"));
        assert!(html.contains("ant-btn-round"));
    }

    #[test]
    fn test_button_disabled() {
        fn app() -> Element {
            rsx! {
                Button {
                    disabled: true,
                    "Disabled Button"
                }
            }
        }

        let mut vdom = VirtualDom::new_with_props(app, ());
        let _ = vdom.rebuild();

        // 验证禁用按钮已渲染
        let html = vdom.body();
        assert!(html.contains("Disabled Button"));
        assert!(html.contains("disabled=\"true\""));
    }

    #[test]
    fn test_button_loading() {
        fn app() -> Element {
            rsx! {
                Button {
                    loading: LoadingConfig::Loading,
                    "Loading Button"
                }
            }
        }

        let mut vdom = VirtualDom::new_with_props(app, ());
        let _ = vdom.rebuild();

        // 验证加载按钮已渲染
        let html = vdom.body();
        assert!(html.contains("Loading Button"));
        assert!(html.contains("ant-btn-loading"));
        assert!(html.contains("ant-btn-loading-icon"));
    }

    #[test]
    fn test_button_group() {
        fn app() -> Element {
            rsx! {
                ButtonGroup {
                    Button {
                        "Button 1"
                    }
                    Button {
                        "Button 2"
                    }
                    Button {
                        "Button 3"
                    }
                }
            }
        }

        let mut vdom = VirtualDom::new_with_props(app, ());
        let _ = vdom.rebuild();

        // 验证按钮组已渲染
        let html = vdom.body();
        assert!(html.contains("Button 1"));
        assert!(html.contains("Button 2"));
        assert!(html.contains("Button 3"));
        assert!(html.contains("ant-btn-group"));
    }
}

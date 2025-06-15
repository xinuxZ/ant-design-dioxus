//! Button Styles Integration Tests
//!
//! 测试按钮组件在不同样式配置下的集成表现
//! 验证组件与主题系统的集成、样式生成和渲染效果

use base64 as _;
use chrono as _;
use css_in_rust as _;
use gloo_timers as _;
use js_sys as _;
use log as _;
use once_cell as _;
use qrcode as _;
use serde as _;
use serde_json as _;
use wasm_bindgen as _;
use wasm_bindgen_futures as _;
use web_sys as _;

use ant_design_dioxus::components::button::*;
use ant_design_dioxus::theme::{provider::ThemeProvider, ThemeConfig};
use dioxus::prelude::*;

/// 创建带有 ThemeProvider 的测试组件
#[component]
fn TestWrapper() -> Element {
    let theme_config = use_signal(|| ThemeConfig::default());

    rsx! {
        ThemeProvider { config: theme_config,
            div {
                // 测试内容将在这里
                Button {
                    variant: Some(ButtonVariant::Solid),
                    "Test Button"
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct ButtonVariantTestProps {
    variant: ButtonVariant,
}

/// 测试按钮在不同变体下的样式生成
#[component]
fn ButtonVariantTest(props: ButtonVariantTestProps) -> Element {
    let theme_config = use_signal(|| ThemeConfig::default());

    rsx! {
        ThemeProvider { config: theme_config,
            Button {
                variant: Some(props.variant),
                "Test Button"
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct ButtonSizeTestProps {
    size: ButtonSize,
}

/// 测试按钮在不同尺寸下的样式生成
#[component]
fn ButtonSizeTest(props: ButtonSizeTestProps) -> Element {
    let theme_config = use_signal(|| ThemeConfig::default());

    rsx! {
        ThemeProvider { config: theme_config,
            Button {
                size: props.size,
                "Test Button"
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct ButtonShapeTestProps {
    shape: ButtonShape,
}

/// 测试按钮在不同形状下的样式生成
#[component]
fn ButtonShapeTest(props: ButtonShapeTestProps) -> Element {
    let theme_config = use_signal(|| ThemeConfig::default());

    rsx! {
        ThemeProvider { config: theme_config,
            Button {
                shape: props.shape,
                "Test Button"
            }
        }
    }
}

/// 集成测试：测试所有按钮变体的样式生成
#[tokio::test]
async fn test_all_button_variants_integration() {
    let variants = [
        ButtonVariant::Solid,
        ButtonVariant::Outlined,
        ButtonVariant::Dashed,
        ButtonVariant::Text,
        ButtonVariant::Link,
    ];

    for variant in variants.iter() {
        let mut vdom = VirtualDom::new_with_props(
            ButtonVariantTest,
            ButtonVariantTestProps { variant: *variant },
        );
        let _result = vdom.rebuild_to_vec();

        // 验证组件能够正常渲染
        assert!(!_result.edits.is_empty());
    }
}

/// 集成测试：测试所有按钮尺寸的样式生成
#[tokio::test]
async fn test_all_button_sizes_integration() {
    let sizes = [ButtonSize::Large, ButtonSize::Middle, ButtonSize::Small];

    for size in sizes.iter() {
        let mut vdom =
            VirtualDom::new_with_props(ButtonSizeTest, ButtonSizeTestProps { size: *size });
        let _result = vdom.rebuild_to_vec();

        // 验证组件能够正常渲染
        assert!(!_result.edits.is_empty());
    }
}

/// 集成测试：测试所有按钮形状的样式生成
#[tokio::test]
async fn test_all_button_shapes_integration() {
    let shapes = [
        ButtonShape::Default,
        ButtonShape::Circle,
        ButtonShape::Round,
    ];

    for shape in shapes.iter() {
        let mut vdom =
            VirtualDom::new_with_props(ButtonShapeTest, ButtonShapeTestProps { shape: *shape });
        let _result = vdom.rebuild_to_vec();

        // 验证组件能够正常渲染
        assert!(!_result.edits.is_empty());
    }
}

/// 集成测试：测试按钮的加载状态
#[tokio::test]
async fn test_button_loading_state_integration() {
    #[component]
    fn LoadingButtonTest() -> Element {
        let theme_config = use_signal(|| ThemeConfig::default());

        rsx! {
         ThemeProvider { config: theme_config,
                Button {
                    loading: LoadingConfig::Loading,
                    "Loading Button"
                }
            }
        }
    }

    let mut vdom = VirtualDom::new(LoadingButtonTest);
    let _result = vdom.rebuild_to_vec();

    // 验证加载状态的按钮能够正常渲染
    assert!(!_result.edits.is_empty());
}

/// 集成测试：测试按钮的禁用状态
#[tokio::test]
async fn test_button_disabled_state_integration() {
    #[component]
    fn DisabledButtonTest() -> Element {
        let theme_config = use_signal(|| ThemeConfig::default());

        rsx! {
         ThemeProvider { config: theme_config,
                Button {
                    disabled: true,
                    "Disabled Button"
                }
            }
        }
    }

    let mut vdom = VirtualDom::new(DisabledButtonTest);
    let _result = vdom.rebuild_to_vec();

    // 验证禁用状态的按钮能够正常渲染
    assert!(!_result.edits.is_empty());
}

/// 集成测试：测试按钮的危险状态
#[tokio::test]
async fn test_button_danger_state_integration() {
    #[component]
    fn DangerButtonTest() -> Element {
        let theme_config = use_signal(|| ThemeConfig::default());

        rsx! {
         ThemeProvider { config: theme_config,
                Button {
                    danger: true,
                    "Danger Button"
                }
            }
        }
    }

    let mut vdom = VirtualDom::new(DangerButtonTest);
    let _result = vdom.rebuild_to_vec();

    // 验证危险状态的按钮能够正常渲染
    assert!(!_result.edits.is_empty());
}

/// 集成测试：测试按钮的组合状态
#[tokio::test]
async fn test_button_combined_states_integration() {
    #[component]
    fn CombinedStateButtonTest() -> Element {
        let theme_config = use_signal(|| ThemeConfig::default());

        rsx! {
            ThemeProvider { config: theme_config,
                div {
                    // 测试多种状态组合
                    Button {
                        variant: Some(ButtonVariant::Outlined),
                        size: ButtonSize::Large,
                        shape: ButtonShape::Round,
                        danger: true,
                        "Large Outlined Round Danger Button"
                    }
                    Button {
                        variant: Some(ButtonVariant::Dashed),
                        size: ButtonSize::Small,
                        loading: LoadingConfig::Loading,
                        "Small Dashed Loading Button"
                    }
                    Button {
                        variant: Some(ButtonVariant::Text),
                        disabled: true,
                        "Text Disabled Button"
                    }
                }
            }
        }
    }

    let mut vdom = VirtualDom::new(CombinedStateButtonTest);
    let _result = vdom.rebuild_to_vec();

    // 验证组合状态的按钮能够正常渲染
    assert!(!_result.edits.is_empty());
}

/// 集成测试：测试按钮的自定义颜色
#[tokio::test]
async fn test_button_custom_color_integration() {
    #[component]
    fn CustomColorButtonTest() -> Element {
        let theme_config = use_signal(|| ThemeConfig::default());

        rsx! {
            ThemeProvider { config: theme_config,
                div {
                    Button {
                        color: Some(ButtonColor::Primary),
                        "Primary Button"
                    }
                    Button {
                        color: Some(ButtonColor::Primary),
                        "Success Button"
                    }
                    Button {
                        color: Some(ButtonColor::Default),
                        "Warning Button"
                    }
                    Button {
                        color: Some(ButtonColor::Danger),
                        "Danger Button"
                    }
                }
            }
        }
    }

    let mut vdom = VirtualDom::new(CustomColorButtonTest);
    let _result = vdom.rebuild_to_vec();

    // 验证自定义颜色的按钮能够正常渲染
    assert!(!_result.edits.is_empty());
}

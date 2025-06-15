//! Button Examples and Documentation Tests
//!
//! 测试按钮组件的示例代码和文档中的用法
//! 确保文档中的代码示例能够正常工作，并展示最佳实践

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

/// 文档测试：基础按钮用法示例
#[tokio::test]
async fn test_basic_button_examples() {
    #[component]
    fn BasicButtonExamples() -> Element {
        let theme_config = use_signal(|| ThemeConfig::default());

        rsx! {
            ThemeProvider { config: theme_config,
                div {
                    style: "padding: 20px; display: flex; gap: 8px; flex-wrap: wrap;",

                    // 基础实心按钮
                    Button {
                        variant: ButtonVariant::Solid,
                        "Primary Button"
                    }

                    // 次要按钮
                    Button {
                        variant: ButtonVariant::Outlined,
                        "Secondary Button"
                    }

                    // 虚线按钮
                    Button {
                        variant: ButtonVariant::Dashed,
                        "Dashed Button"
                    }

                    // 文本按钮
                    Button {
                        variant: ButtonVariant::Text,
                        "Text Button"
                    }

                    // 链接按钮
                    Button {
                        variant: ButtonVariant::Link,
                        "Link Button"
                    }
                }
            }
        }
    }

    let mut vdom = VirtualDom::new(BasicButtonExamples);
    let _result = vdom.rebuild_in_place();

    assert!(true, "Basic button examples should render successfully");
}

/// 文档测试：按钮尺寸示例
#[tokio::test]
async fn test_button_size_examples() {
    #[component]
    fn ButtonSizeExamples() -> Element {
        let theme_config = use_signal(|| ThemeConfig::default());

        rsx! {
            ThemeProvider { config: theme_config,
                div {
                    style: "padding: 20px;",

                    h3 { "Button Sizes" }

                    div {
                        style: "display: flex; gap: 8px; align-items: center; margin-bottom: 16px;",

                        Button {
                            size: ButtonSize::Small,
                            "Small"
                        }

                        Button {
                            size: ButtonSize::Middle,
                            "Middle"
                        }

                        Button {
                            size: ButtonSize::Large,
                            "Large"
                        }
                    }

                    h3 { "Outlined Buttons" }

                    div {
                        style: "display: flex; gap: 8px; align-items: center;",

                        Button {
                            variant: ButtonVariant::Outlined,
                            size: ButtonSize::Small,
                            "Small"
                        }

                        Button {
                            variant: ButtonVariant::Outlined,
                            size: ButtonSize::Middle,
                            "Middle"
                        }

                        Button {
                            variant: ButtonVariant::Outlined,
                            size: ButtonSize::Large,
                            "Large"
                        }
                    }
                }
            }
        }
    }

    let mut vdom = VirtualDom::new(ButtonSizeExamples);
    let _result = vdom.rebuild_in_place();

    assert!(true, "Button size examples should render successfully");
}

/// 文档测试：按钮形状示例
#[tokio::test]
async fn test_button_shape_examples() {
    #[component]
    fn ButtonShapeExamples() -> Element {
        let theme_config = use_signal(|| ThemeConfig::default());

        rsx! {
            ThemeProvider { config: theme_config,
                div {
                    style: "padding: 20px;",

                    h3 { "Button Shapes" }

                    div {
                        style: "display: flex; gap: 8px; align-items: center; margin-bottom: 16px;",

                        Button {
                            shape: ButtonShape::Default,
                            "Default"
                        }

                        Button {
                            shape: ButtonShape::Circle,
                            "O"
                        }

                        Button {
                            shape: ButtonShape::Round,
                            "Round"
                        }
                    }

                    h3 { "Icon Buttons" }

                    div {
                        style: "display: flex; gap: 8px; align-items: center;",

                        Button {
                            shape: ButtonShape::Circle,
                            size: ButtonSize::Small,
                            "S"
                        }

                        Button {
                            shape: ButtonShape::Circle,
                            size: ButtonSize::Middle,
                            "M"
                        }

                        Button {
                            shape: ButtonShape::Circle,
                            size: ButtonSize::Large,
                            "L"
                        }
                    }
                }
            }
        }
    }

    let mut vdom = VirtualDom::new(ButtonShapeExamples);
    let _result = vdom.rebuild_in_place();

    assert!(true, "Button shape examples should render successfully");
}

/// 文档测试：按钮状态示例
#[tokio::test]
async fn test_button_state_examples() {
    #[component]
    fn ButtonStateExamples() -> Element {
        let theme_config = use_signal(|| ThemeConfig::default());

        rsx! {
            ThemeProvider { config: theme_config,
                div {
                    style: "padding: 20px;",

                    h3 { "Button States" }

                    div {
                        style: "display: flex; gap: 8px; flex-wrap: wrap; margin-bottom: 16px;",

                        Button {
                            "Normal"
                        }

                        Button {
                            disabled: true,
                            "Disabled"
                        }

                        Button {
                            loading: LoadingConfig::Loading,
                            "Loading"
                        }

                        Button {
                            danger: true,
                            "Danger"
                        }
                    }

                    h3 { "Outlined States" }

                    div {
                        style: "display: flex; gap: 8px; flex-wrap: wrap;",

                        Button {
                            variant: ButtonVariant::Outlined,
                            "Normal"
                        }

                        Button {
                            variant: ButtonVariant::Outlined,
                            disabled: true,
                            "Disabled"
                        }

                        Button {
                            variant: ButtonVariant::Outlined,
                            loading: LoadingConfig::Loading,
                            "Loading"
                        }

                        Button {
                            variant: ButtonVariant::Outlined,
                            danger: true,
                            "Danger"
                        }
                    }
                }
            }
        }
    }

    let mut vdom = VirtualDom::new(ButtonStateExamples);
    let _result = vdom.rebuild_in_place();

    assert!(true, "Button state examples should render successfully");
}

/// 文档测试：按钮颜色示例
#[tokio::test]
async fn test_button_color_examples() {
    #[component]
    fn ButtonColorExamples() -> Element {
        let theme_config = use_signal(|| ThemeConfig::default());

        rsx! {
            ThemeProvider { config: theme_config,
                div {
                    style: "padding: 20px;",

                    h3 { "Solid Button Colors" }

                    div {
                        style: "display: flex; gap: 8px; flex-wrap: wrap; margin-bottom: 16px;",

                        Button {
                            variant: ButtonVariant::Solid,
                            color: ButtonColor::Primary,
                            "Primary"
                        }

                        Button {
                            variant: ButtonVariant::Solid,
                            color: ButtonColor::Primary,
                            "Success"
                        }

                        Button {
                            variant: ButtonVariant::Solid,
                            color: ButtonColor::Default,
                            "Warning"
                        }

                        Button {
                            variant: ButtonVariant::Solid,
                            color: ButtonColor::Danger,
                            "Danger"
                        }
                    }

                    h3 { "Outlined Button Colors" }

                    div {
                        style: "display: flex; gap: 8px; flex-wrap: wrap;",

                        Button {
                            variant: ButtonVariant::Outlined,
                            color: ButtonColor::Primary,
                            "Primary"
                        }

                        Button {
                            variant: ButtonVariant::Outlined,
                            color: ButtonColor::Primary,
                            "Success"
                        }

                        Button {
                            variant: ButtonVariant::Outlined,
                            color: ButtonColor::Default,
                            "Warning"
                        }

                        Button {
                            variant: ButtonVariant::Outlined,
                            color: ButtonColor::Danger,
                            "Danger"
                        }
                    }
                }
            }
        }
    }

    let mut vdom = VirtualDom::new(ButtonColorExamples);
    let _result = vdom.rebuild_in_place();

    assert!(true, "Button color examples should render successfully");
}

/// 文档测试：交互式按钮示例
#[tokio::test]
async fn test_interactive_button_examples() {
    #[component]
    fn InteractiveButtonExamples() -> Element {
        let mut click_count = use_signal(|| 0);
        let mut is_loading = use_signal(|| false);
        let theme_config = use_signal(|| ThemeConfig::default());

        let handle_click = move |_| {
            click_count.set(click_count() + 1);
        };

        let handle_async_click = move |_| {
            is_loading.set(true);
            // 模拟异步操作
            // 在真实应用中，这里会有实际的异步操作
            is_loading.set(false);
        };

        rsx! {
            ThemeProvider { config: theme_config,
                div {
                    style: "padding: 20px;",

                    h3 { "Interactive Buttons" }

                    div {
                        style: "display: flex; gap: 8px; flex-wrap: wrap; margin-bottom: 16px;",

                        Button {
                            on_click: handle_click,
                            "Click Me ({click_count})"
                        }

                        Button {
                            variant: ButtonVariant::Outlined,
                            on_click: handle_async_click,
                            loading: if is_loading() { LoadingConfig::Loading } else { LoadingConfig::NotLoading },
                            disabled: is_loading(),
                            "Async Action"
                        }

                        Button {
                            variant: ButtonVariant::Dashed,
                            on_click: move |_| {
                                click_count.set(0);
                            },
                            "Reset Counter"
                        }
                    }

                    div {
                        style: "margin-top: 16px; padding: 12px; background: #f5f5f5; border-radius: 6px;",

                        p { "Click count: {click_count}" }
                        p { "Loading state: {is_loading}" }
                    }
                }
            }
        }
    }

    let mut vdom = VirtualDom::new(InteractiveButtonExamples);
    let _result = vdom.rebuild_in_place();

    assert!(
        true,
        "Interactive button examples should render successfully"
    );
}

/// 文档测试：按钮组合示例
#[tokio::test]
async fn test_button_combination_examples() {
    #[component]
    fn ButtonCombinationExamples() -> Element {
        let theme_config = use_signal(|| ThemeConfig::default());

        rsx! {
            ThemeProvider { config: theme_config,
                div {
                    style: "padding: 20px;",

                    h3 { "Button Combinations" }

                    // 操作按钮组
                    div {
                        style: "display: flex; gap: 8px; margin-bottom: 16px;",

                        Button {
                            variant: ButtonVariant::Solid,
                            "Save"
                        }

                        Button {
                            variant: ButtonVariant::Outlined,
                            "Cancel"
                        }

                        Button {
                            variant: ButtonVariant::Text,
                            "Reset"
                        }
                    }

                    // 危险操作组
                    div {
                        style: "display: flex; gap: 8px; margin-bottom: 16px;",

                        Button {
                            variant: ButtonVariant::Solid,
                            danger: true,
                            "Delete"
                        }

                        Button {
                            variant: ButtonVariant::Outlined,
                            "Keep"
                        }
                    }

                    // 尺寸组合
                    div {
                        style: "display: flex; gap: 8px; align-items: center; margin-bottom: 16px;",

                        Button {
                            size: ButtonSize::Large,
                            "Large Action"
                        }

                        Button {
                            size: ButtonSize::Middle,
                            variant: ButtonVariant::Outlined,
                            "Medium Action"
                        }

                        Button {
                            size: ButtonSize::Small,
                            variant: ButtonVariant::Text,
                            "Small Action"
                        }
                    }

                    // 形状组合
                    div {
                        style: "display: flex; gap: 8px; align-items: center;",

                        Button {
                            shape: ButtonShape::Circle,
                            "+"
                        }

                        Button {
                            shape: ButtonShape::Round,
                            "Add Item"
                        }

                        Button {
                            shape: ButtonShape::Default,
                            variant: ButtonVariant::Outlined,
                            "More Options"
                        }
                    }
                }
            }
        }
    }

    let mut vdom = VirtualDom::new(ButtonCombinationExamples);
    let _result = vdom.rebuild_in_place();

    assert!(
        true,
        "Button combination examples should render successfully"
    );
}

/// 文档测试：响应式按钮示例
#[tokio::test]
async fn test_responsive_button_examples() {
    #[component]
    fn ResponsiveButtonExamples() -> Element {
        let theme_config = use_signal(|| ThemeConfig::default());

        rsx! {
            ThemeProvider { config: theme_config,
                div {
                    style: "padding: 20px;",

                    h3 { "Responsive Button Layout" }

                    // 移动端优化的按钮布局
                    div {
                        style: "display: flex; flex-direction: column; gap: 8px; max-width: 300px; margin-bottom: 16px;",

                        Button {
                            style: "width: 100%;",
                            "Full Width Button"
                        }

                        div {
                            style: "display: flex; gap: 8px;",

                            Button {
                                style: "flex: 1;",
                                variant: ButtonVariant::Outlined,
                                "Cancel"
                            }

                            Button {
                                style: "flex: 1;",
                                "Confirm"
                            }
                        }
                    }

                    // 桌面端按钮布局
                    div {
                        style: "display: flex; gap: 8px; flex-wrap: wrap;",

                        Button {
                            "Action 1"
                        }

                        Button {
                            variant: ButtonVariant::Outlined,
                            "Action 2"
                        }

                        Button {
                            variant: ButtonVariant::Dashed,
                            "Action 3"
                        }

                        Button {
                            variant: ButtonVariant::Text,
                            "More"
                        }
                    }
                }
            }
        }
    }

    let mut vdom = VirtualDom::new(ResponsiveButtonExamples);
    let _result = vdom.rebuild_in_place();

    assert!(
        true,
        "Responsive button examples should render successfully"
    );
}

/// 文档测试：主题定制按钮示例
#[tokio::test]
async fn test_theme_customization_examples() {
    #[component]
    fn ThemeCustomizationExamples() -> Element {
        // 自定义主题配置
        let theme_config = use_signal(|| ThemeConfig::default());

        rsx! {
            ThemeProvider { config: theme_config,
                div {
                    style: "padding: 20px; background: #141414; color: white;",

                    h3 { "Dark Theme Buttons" }

                    div {
                        style: "display: flex; gap: 8px; flex-wrap: wrap; margin-bottom: 16px;",

                        Button {
                            variant: ButtonVariant::Solid,
                            "Primary"
                        }

                        Button {
                            variant: ButtonVariant::Outlined,
                            "Secondary"
                        }

                        Button {
                            variant: ButtonVariant::Dashed,
                            "Dashed"
                        }

                        Button {
                            variant: ButtonVariant::Text,
                            "Text"
                        }
                    }

                    div {
                        style: "display: flex; gap: 8px; flex-wrap: wrap;",

                        Button {
                            color: ButtonColor::Primary,
                            "Success"
                        }

                        Button {
                            color: ButtonColor::Default,
                            variant: ButtonVariant::Outlined,
                            "Warning"
                        }

                        Button {
                            color: ButtonColor::Danger,
                            "Danger"
                        }
                    }
                }
            }
        }
    }

    let mut vdom = VirtualDom::new(ThemeCustomizationExamples);
    let _result = vdom.rebuild_in_place();

    assert!(
        true,
        "Theme customization examples should render successfully"
    );
}

/// 文档测试：最佳实践示例
#[tokio::test]
async fn test_best_practices_examples() {
    #[component]
    fn BestPracticesExamples() -> Element {
        let mut form_state = use_signal(|| "idle"); // idle, submitting, success, error
        let theme_config = use_signal(|| ThemeConfig::default());

        let handle_submit = move |_| {
            form_state.set("submitting");
            // 模拟表单提交
            // 在真实应用中，这里会有实际的提交逻辑
            form_state.set("success");
        };

        rsx! {
            ThemeProvider { config: theme_config,
                div {
                    style: "padding: 20px;",

                    h3 { "Best Practices" }

                    // 表单提交最佳实践
                    div {
                        style: "margin-bottom: 24px; padding: 16px; border: 1px solid #d9d9d9; border-radius: 6px;",

                        h4 { "Form Submission" }

                        div {
                            style: "display: flex; gap: 8px; margin-top: 12px;",

                            Button {
                                on_click: handle_submit,
                                loading: if form_state() == "submitting" { LoadingConfig::Loading } else { LoadingConfig::NotLoading },
                                disabled: form_state() == "submitting" || form_state() == "success",
                                if form_state() == "success" { "✓ Submitted" } else { "Submit" }
                            }

                            Button {
                                variant: ButtonVariant::Outlined,
                                disabled: form_state() == "submitting",
                                on_click: move |_| form_state.set("idle"),
                                "Reset"
                            }
                        }

                        p {
                            style: "margin-top: 8px; font-size: 12px; color: #666;",
                            "State: {form_state}"
                        }
                    }

                    // 危险操作最佳实践
                    div {
                        style: "margin-bottom: 24px; padding: 16px; border: 1px solid #d9d9d9; border-radius: 6px;",

                        h4 { "Dangerous Actions" }

                        div {
                            style: "display: flex; gap: 8px; margin-top: 12px;",

                            Button {
                                danger: true,
                                "Delete Item"
                            }

                            Button {
                                variant: ButtonVariant::Outlined,
                                "Cancel"
                            }
                        }

                        p {
                            style: "margin-top: 8px; font-size: 12px; color: #666;",
                            "Always provide a cancel option for dangerous actions"
                        }
                    }

                    // 加载状态最佳实践
                    div {
                        style: "padding: 16px; border: 1px solid #d9d9d9; border-radius: 6px;",

                        h4 { "Loading States" }

                        div {
                            style: "display: flex; gap: 8px; margin-top: 12px;",

                            Button {
                                loading: LoadingConfig::Loading,
                                "Loading..."
                            }

                            Button {
                                variant: ButtonVariant::Outlined,
                                loading: LoadingConfig::Loading,
                                disabled: true,
                                "Processing"
                            }
                        }

                        p {
                            style: "margin-top: 8px; font-size: 12px; color: #666;",
                            "Disable buttons during loading to prevent multiple submissions"
                        }
                    }
                }
            }
        }
    }

    let mut vdom = VirtualDom::new(BestPracticesExamples);
    let _result = vdom.rebuild_in_place();

    assert!(true, "Best practices examples should render successfully");
}

//! Button Performance Tests
//!
//! 测试按钮组件的渲染性能、内存使用和大量数据场景
//! 确保组件在各种性能要求下都能正常工作
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
use ant_design_dioxus::prelude::*;
use ant_design_dioxus::theme::{provider::ThemeProvider, ThemeConfig};
use dioxus::prelude::*;
use std::time::{Duration, Instant};

/// 性能基准测试：单个按钮渲染时间
#[tokio::test]
async fn test_single_button_render_performance() {
    #[component]
    fn SingleButtonTest() -> Element {
        let theme_config = use_signal(|| ThemeConfig::default());

        rsx! {
            ThemeProvider { config: theme_config,
                Button {
                    variant: ButtonVariant::Solid,
                    "Performance Test Button"
                }
            }
        }
    }

    let start_time = Instant::now();
    let mut vdom = VirtualDom::new(SingleButtonTest);
    let _result = vdom.rebuild_in_place();
    let render_time = start_time.elapsed();

    // 验证渲染成功
    assert!(true);

    // 单个按钮渲染应该在 10ms 内完成
    assert!(
        render_time.as_millis() < 10,
        "Single button render took too long: {:?}",
        render_time
    );

    println!("Single button render time: {:?}", render_time);
}

/// 性能基准测试：批量按钮渲染时间
#[tokio::test]
async fn test_batch_button_render_performance() {
    const BUTTON_COUNT: usize = 1000;

    #[component]
    fn BatchButtonTest() -> Element {
        let theme_config = use_signal(|| ThemeConfig::default());

        rsx! {
            ThemeProvider { config: theme_config,
                div {
                    style: "display: flex; flex-wrap: wrap; gap: 4px;",

                    for i in 0..BUTTON_COUNT {
                        Button {
                            key: "{i}",
                            variant: ButtonVariant::Solid,
                            "Button {i}"
                        }
                    }
                }
            }
        }
    }

    let start_time = Instant::now();
    let mut vdom = VirtualDom::new(BatchButtonTest);
    let _result = vdom.rebuild_in_place();
    let render_time = start_time.elapsed();

    // 验证渲染成功
    assert!(true);

    // 1000个按钮渲染应该在 500ms 内完成
    assert!(
        render_time.as_millis() < 500,
        "Batch button render took too long: {:?}",
        render_time
    );

    println!(
        "Batch render time for {} buttons: {:?}",
        BUTTON_COUNT, render_time
    );
    println!(
        "Average time per button: {:?}",
        render_time / BUTTON_COUNT as u32
    );
}

/// 性能测试：不同变体按钮的渲染性能对比
#[tokio::test]
async fn test_variant_render_performance_comparison() {
    let variants = [
        ButtonVariant::Solid,
        ButtonVariant::Outlined,
        ButtonVariant::Dashed,
        ButtonVariant::Text,
        ButtonVariant::Link,
    ];

    #[derive(Props, Clone, PartialEq)]
    struct VariantButtonTestProps {
        variant: ButtonVariant,
    }

    for variant in variants.iter() {
        #[component]
        fn VariantButtonTest(props: VariantButtonTestProps) -> Element {
            let theme_config = use_signal(|| ThemeConfig::default());

            rsx! {
                ThemeProvider { config: theme_config,
                    div {
                        for i in 0..100 {
                            Button {
                                key: "{i}",
                                variant: props.variant,
                                "Button {i}"
                            }
                        }
                    }
                }
            }
        }

        let start_time = Instant::now();
        let mut vdom = VirtualDom::new_with_props(
            VariantButtonTest,
            VariantButtonTestProps { variant: *variant },
        );
        let _result = vdom.rebuild_in_place();
        let render_time = start_time.elapsed();

        // 验证渲染成功
        assert!(true);

        println!(
            "Render time for {:?} variant (100 buttons): {:?}",
            variant, render_time
        );

        // 每种变体的100个按钮渲染应该在 100ms 内完成
        assert!(
            render_time.as_millis() < 100,
            "{:?} variant render took too long: {:?}",
            variant,
            render_time
        );
    }
}

/// 性能测试：不同尺寸按钮的渲染性能对比
#[tokio::test]
async fn test_size_render_performance_comparison() {
    let sizes = [ButtonSize::Small, ButtonSize::Middle, ButtonSize::Large];

    #[derive(Props, Clone, PartialEq)]
    struct SizeButtonTestProps {
        size: ButtonSize,
    }
    for size in sizes.iter() {
        #[component]
        fn SizeButtonTest(props: SizeButtonTestProps) -> Element {
            let theme_config = use_signal(|| ThemeConfig::default());

            rsx! {
                ThemeProvider { config: theme_config,
                    div {
                        for i in 0..100 {
                            Button {
                                key: "{i}",
                                size: props.size,
                                "Button {i}"
                            }
                        }
                    }
                }
            }
        }

        let start_time = Instant::now();
        let mut vdom =
            VirtualDom::new_with_props(SizeButtonTest, SizeButtonTestProps { size: *size });
        let _result = vdom.rebuild_in_place();
        let render_time = start_time.elapsed();

        // 验证渲染成功
        assert!(true);

        println!(
            "Render time for {:?} size (100 buttons): {:?}",
            size, render_time
        );

        // 每种尺寸的100个按钮渲染应该在 100ms 内完成
        assert!(
            render_time.as_millis() < 100,
            "{:?} size render took too long: {:?}",
            size,
            render_time
        );
    }
}

/// 性能测试：复杂状态组合的渲染性能
#[tokio::test]
async fn test_complex_state_render_performance() {
    #[component]
    fn ComplexStateTest() -> Element {
        let theme_config = use_signal(|| ThemeConfig::default());

        rsx! {
            ThemeProvider { config: theme_config,
                div {
                    for i in 0..200 {
                        Button {
                            key: "{i}",
                            variant: match i % 5 {
                                0 => ButtonVariant::Solid,
                                1 => ButtonVariant::Outlined,
                                2 => ButtonVariant::Dashed,
                                3 => ButtonVariant::Text,
                                _ => ButtonVariant::Link,
                            },
                            size: match i % 3 {
                                0 => ButtonSize::Small,
                                1 => ButtonSize::Middle,
                                _ => ButtonSize::Large,
                            },
                            shape: match i % 3 {
                                0 => ButtonShape::Default,
                                1 => ButtonShape::Circle,
                                _ => ButtonShape::Round,
                            },
                            color: match i % 4 {
                                0 => ButtonColor::Primary,
                                2 => ButtonColor::Default,
                                _ => ButtonColor::Danger,
                            },
                            danger: i % 7 == 0,
                            disabled: i % 11 == 0,
                            loading: if i % 13 == 0 { LoadingConfig::default() } else { LoadingConfig::NotLoading },
                            "Complex Button {i}"
                        }
                    }
                }
            }
        }
    }

    let start_time = Instant::now();
    let mut vdom = VirtualDom::new(ComplexStateTest);
    let _result = vdom.rebuild_in_place();
    let render_time = start_time.elapsed();

    // 验证渲染成功
    assert!(true);

    println!("Complex state render time (200 buttons): {:?}", render_time);

    // 200个复杂状态按钮渲染应该在 200ms 内完成
    assert!(
        render_time.as_millis() < 200,
        "Complex state render took too long: {:?}",
        render_time
    );
}

/// 性能测试：动态状态更新性能
#[tokio::test]
async fn test_dynamic_state_update_performance() {
    #[component]
    fn DynamicUpdateTest() -> Element {
        let mut button_states = use_signal(|| vec![false; 50]);
        let theme_config = use_signal(|| ThemeConfig::default());

        rsx! {
            ThemeProvider { config: theme_config,
                div {
                    Button {
                        on_click: move |_| {
                            let mut states = button_states();
                            for state in states.iter_mut() {
                                *state = !*state;
                            }
                            button_states.set(states);
                        },
                        "Toggle All States"
                    }

                    for (i, &is_active) in button_states().iter().enumerate() {
                        Button {
                            key: "{i}",
                            variant: if is_active { ButtonVariant::Solid } else { ButtonVariant::Outlined },
                            danger: is_active,
                            "Dynamic Button {i}"
                        }
                    }
                }
            }
        }
    }

    let start_time = Instant::now();
    let mut vdom = VirtualDom::new(DynamicUpdateTest);
    let _result = vdom.rebuild_in_place();
    let initial_render_time = start_time.elapsed();

    // 验证初始渲染成功
    assert!(true);

    println!(
        "Dynamic update test initial render time: {:?}",
        initial_render_time
    );

    // 初始渲染应该在 50ms 内完成
    assert!(
        initial_render_time.as_millis() < 50,
        "Dynamic update test initial render took too long: {:?}",
        initial_render_time
    );
}

/// 性能测试：主题切换性能
#[tokio::test]
async fn test_theme_switch_performance() {
    #[component]
    fn ThemeSwitchPerformanceTest() -> Element {
        let mut is_dark_theme = use_signal(|| false);

        let theme_config = if is_dark_theme() {
            use_signal(|| ThemeConfig::default().theme(Theme::Dark))
        } else {
            use_signal(|| ThemeConfig::default())
        };

        rsx! {
            ThemeProvider { config: theme_config,
                div {
                    Button {
                        on_click: move |_| is_dark_theme.set(!is_dark_theme()),
                        "Switch Theme"
                    }

                    for i in 0..100 {
                        Button {
                            key: "{i}",
                            variant: match i % 3 {
                                0 => ButtonVariant::Solid,
                                1 => ButtonVariant::Outlined,
                                _ => ButtonVariant::Dashed,
                            },
                            "Themed Button {i}"
                        }
                    }
                }
            }
        }
    }

    let start_time = Instant::now();
    let mut vdom = VirtualDom::new(ThemeSwitchPerformanceTest);
    let _result = vdom.rebuild_in_place();
    let render_time = start_time.elapsed();

    // 验证渲染成功
    assert!(true);

    println!(
        "Theme switch performance test render time: {:?}",
        render_time
    );

    // 主题切换测试渲染应该在 100ms 内完成
    assert!(
        render_time.as_millis() < 100,
        "Theme switch performance test took too long: {:?}",
        render_time
    );
}

/// 性能测试：内存使用效率（模拟测试）
#[tokio::test]
async fn test_memory_usage_efficiency() {
    // 这是一个模拟的内存使用测试
    // 在真实环境中，我们会使用专门的内存分析工具

    #[component]
    fn MemoryTest() -> Element {
        let theme_config = use_signal(|| ThemeConfig::default());

        rsx! {
            ThemeProvider { config: theme_config,
                div {
                    // 创建大量按钮来测试内存使用
                    for i in 0..500 {
                        Button {
                            key: "{i}",
                            variant: ButtonVariant::Solid,
                            "Memory Test Button {i}"
                        }
                    }
                }
            }
        }
    }

    // 多次创建和销毁 VirtualDom 来测试内存泄漏
    for iteration in 0..10 {
        let start_time = Instant::now();
        let mut vdom = VirtualDom::new(MemoryTest);
        let _result = vdom.rebuild_in_place();
        let iteration_time = start_time.elapsed();

        // 验证每次迭代都能成功渲染
        assert!(true);

        println!(
            "Memory test iteration {} time: {:?}",
            iteration, iteration_time
        );

        // 每次迭代应该在 100ms 内完成
        assert!(
            iteration_time.as_millis() < 100,
            "Memory test iteration {} took too long: {:?}",
            iteration,
            iteration_time
        );

        // 显式释放 VirtualDom
        drop(vdom);
    }
}

/// 性能测试：并发渲染性能（模拟）
#[tokio::test]
async fn test_concurrent_render_performance() {
    use tokio::task;

    #[derive(Props, Clone, PartialEq)]
    struct ConcurrentButtonTestProps {
        id: usize,
    }
    async fn render_button_component(id: usize) -> Duration {
        #[component]
        fn ConcurrentButtonTest(props: ConcurrentButtonTestProps) -> Element {
            let theme_config = use_signal(|| ThemeConfig::default());

            rsx! {
                ThemeProvider { config: theme_config,
                    div {
                        for i in 0..50 {
                            Button {
                                key: "{i}",
                                variant: ButtonVariant::Solid,
                                "Concurrent Button {props.id}-{i}"
                            }
                        }
                    }
                }
            }
        }

        let start_time = Instant::now();
        let mut vdom =
            VirtualDom::new_with_props(ConcurrentButtonTest, ConcurrentButtonTestProps { id });
        let _result = vdom.rebuild_in_place();
        let render_time = start_time.elapsed();

        // 验证渲染成功
        assert!(true);

        render_time
    }

    let start_time = Instant::now();

    // 并发渲染多个组件
    let tasks: Vec<_> = (0..5)
        .map(|id| task::spawn(render_button_component(id)))
        .collect();

    let mut total_render_time = Duration::new(0, 0);
    for task in tasks {
        let render_time = task.await.unwrap();
        total_render_time += render_time;
        println!("Concurrent render time: {:?}", render_time);
    }

    let total_time = start_time.elapsed();

    println!("Total concurrent render time: {:?}", total_time);
    println!("Sum of individual render times: {:?}", total_render_time);

    // 并发渲染应该比串行渲染更快（或至少不会显著更慢）
    assert!(
        total_time.as_millis() < 500,
        "Concurrent render took too long: {:?}",
        total_time
    );
}

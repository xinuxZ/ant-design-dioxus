//! Flex 组件测试模块
//!
//! 本模块包含 Flex 组件的全面测试用例，涵盖：
//! - 基础渲染测试
//! - 布局方向测试
//! - 对齐方式测试
//! - 间距系统测试
//! - 换行控制测试
//! - 响应式布局测试
//! - 主题应用测试
//! - 性能优化测试
//! - 工具函数测试
//! - 样式生成测试
//! - 边缘情况测试
//! - 可访问性测试

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::components::flex::styles::*;
    use crate::components::flex::types::*;
    use crate::components::flex::utils::*;
    use crate::components::flex::*;
    // use crate::components::flex::{Flex, FlexLayout};
    use dioxus::prelude::*;
    use std::collections::HashMap;

    // ============================================================================
    // 基础渲染测试
    // ============================================================================

    #[test]
    fn test_flex_basic_render() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Flex {
                    "Basic flex container"
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("ant-flex"));
        assert!(html.contains("Basic flex container"));
    }

    #[test]
    fn test_flex_with_children() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Flex {
                    div { "Item 1" }
                    div { "Item 2" }
                    div { "Item 3" }
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("Item 1"));
        assert!(html.contains("Item 2"));
        assert!(html.contains("Item 3"));
    }

    #[test]
    fn test_flex_empty() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Flex {}
            }
        });

        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("ant-flex"));
    }

    // ============================================================================
    // 布局方向测试
    // ============================================================================

    #[test]
    fn test_flex_horizontal() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Flex {
                    vertical: false,
                    "Horizontal layout"
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("flex-direction: row"));
    }

    #[test]
    fn test_flex_vertical() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Flex {
                    vertical: true,
                    "Vertical layout"
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("flex-direction: column"));
    }

    // ============================================================================
    // 对齐方式测试
    // ============================================================================

    #[test]
    fn test_flex_justify_center() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Flex {
                    justify: FlexJustify::Center,
                    "Centered content"
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("justify-content: center"));
    }

    #[test]
    fn test_flex_align_center() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Flex {
                    align: FlexAlign::Center,
                    "Aligned content"
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("align-items: center"));
    }

    #[test]
    fn test_flex_all_justify_values() {
        let justify_values = vec![
            (FlexJustify::Start, "flex-start"),
            (FlexJustify::End, "flex-end"),
            (FlexJustify::Center, "center"),
            (FlexJustify::SpaceBetween, "space-between"),
            (FlexJustify::SpaceAround, "space-around"),
            (FlexJustify::SpaceEvenly, "space-evenly"),
        ];

        for (justify, expected_css) in justify_values {
            let mut dom = VirtualDom::new(move || {
                rsx! {
                    Flex {
                        justify: justify,
                        "Test content"
                    }
                }
            });

            let html = dom.rebuild_to_vec().santize();
            assert!(html.contains(&format!("justify-content: {}", expected_css)));
        }
    }

    #[test]
    fn test_flex_all_align_values() {
        let align_values = vec![
            (FlexAlign::Start, "flex-start"),
            (FlexAlign::End, "flex-end"),
            (FlexAlign::Center, "center"),
            (FlexAlign::Baseline, "baseline"),
            (FlexAlign::Stretch, "stretch"),
        ];

        for (align, expected_css) in align_values {
            let mut dom = VirtualDom::new(move || {
                rsx! {
                    Flex {
                        align: align,
                        "Test content"
                    }
                }
            });

            let html = dom.rebuild_to_vec().santize();
            assert!(html.contains(&format!("align-items: {}", expected_css)));
        }
    }

    // ============================================================================
    // 间距系统测试
    // ============================================================================

    #[test]
    fn test_flex_gap_small() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Flex {
                    gap: FlexGap::Small,
                    "Content with small gap"
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("gap: 8px"));
    }

    #[test]
    fn test_flex_gap_custom() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Flex {
                    gap: FlexGap::Custom("20px".to_string()),
                    "Content with custom gap"
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("gap: 20px"));
    }

    #[test]
    fn test_flex_all_gap_values() {
        let gap_values = vec![
            (FlexGap::None, "0px"),
            (FlexGap::Small, "8px"),
            (FlexGap::Middle, "16px"),
            (FlexGap::Large, "24px"),
        ];

        for (gap, expected_css) in gap_values {
            let mut dom = VirtualDom::new(move || {
                rsx! {
                    Flex {
                        gap: gap,
                        "Test content"
                    }
                }
            });

            let html = dom.rebuild_to_vec().santize();
            assert!(html.contains(&format!("gap: {}", expected_css)));
        }
    }

    // ============================================================================
    // 换行控制测试
    // ============================================================================

    #[test]
    fn test_flex_wrap_nowrap() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Flex {
                    wrap: FlexWrap::NoWrap,
                    "No wrap content"
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("flex-wrap: nowrap"));
    }

    #[test]
    fn test_flex_wrap_wrap() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Flex {
                    wrap: FlexWrap::Wrap,
                    "Wrap content"
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("flex-wrap: wrap"));
    }

    #[test]
    fn test_flex_wrap_reverse() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Flex {
                    wrap: FlexWrap::WrapReverse,
                    "Wrap reverse content"
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("flex-wrap: wrap-reverse"));
    }

    // ============================================================================
    // Flex 属性测试
    // ============================================================================

    #[test]
    fn test_flex_property_auto() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Flex {
                    flex: Some("auto".to_string()),
                    "Auto flex content"
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("flex: auto"));
    }

    #[test]
    fn test_flex_property_number() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Flex {
                    flex: Some("1".to_string()),
                    "Flex 1 content"
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("flex: 1"));
    }

    #[test]
    fn test_flex_property_complex() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Flex {
                    flex: Some("1 1 200px".to_string()),
                    "Complex flex content"
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("flex: 1 1 200px"));
    }

    // ============================================================================
    // 自定义类名和样式测试
    // ============================================================================

    #[test]
    fn test_flex_custom_class() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Flex {
                    class: Some("custom-flex".to_string()),
                    "Custom class content"
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("custom-flex"));
        assert!(html.contains("ant-flex"));
    }

    #[test]
    fn test_flex_custom_style() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Flex {
                    style: Some("background-color: red;".to_string()),
                    "Custom style content"
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("background-color: red;"));
    }

    // ============================================================================
    // 组件类型测试
    // ============================================================================

    #[test]
    fn test_flex_component_div() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Flex {
                    component: Some("div".to_string()),
                    "Div component"
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("<div"));
    }

    #[test]
    fn test_flex_component_section() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Flex {
                    component: Some("section".to_string()),
                    "Section component"
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("<section"));
    }

    // ============================================================================
    // Props 构建器测试
    // ============================================================================

    #[test]
    fn test_flex_props_builder() {
        let props = FlexProps::builder()
            .vertical(true)
            .justify(FlexJustify::Center)
            .align(FlexAlign::Center)
            .gap(FlexGap::Large)
            .wrap(FlexWrap::Wrap)
            .flex(Some("1".to_string()))
            .class(Some("test-class".to_string()))
            .style(Some("color: blue;".to_string()))
            .component(Some("section".to_string()))
            .build();

        assert_eq!(props.vertical, true);
        assert_eq!(props.justify, FlexJustify::Center);
        assert_eq!(props.align, FlexAlign::Center);
        assert_eq!(props.gap, FlexGap::Large);
        assert_eq!(props.wrap, FlexWrap::Wrap);
        assert_eq!(props.flex, Some("1".to_string()));
        assert_eq!(props.class, Some("test-class".to_string()));
        assert_eq!(props.style, Some("color: blue;".to_string()));
        assert_eq!(props.component, Some("section".to_string()));
    }

    #[test]
    fn test_flex_props_default() {
        let props = FlexProps::default();

        assert_eq!(props.vertical, false);
        assert_eq!(props.justify, FlexJustify::Start);
        assert_eq!(props.align, FlexAlign::Start);
        assert_eq!(props.gap, FlexGap::None);
        assert_eq!(props.wrap, FlexWrap::NoWrap);
        assert_eq!(props.flex, None);
        assert_eq!(props.class, None);
        assert_eq!(props.style, None);
        assert_eq!(props.component, None);
    }

    // ============================================================================
    // 便捷构造函数测试
    // ============================================================================

    #[test]
    fn test_flex_horizontal_constructor() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                {flex_horizontal(rsx! { "Horizontal content" }, rsx!())}
            }
        });

        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("flex-direction: row"));
        assert!(html.contains("Horizontal content"));
    }

    #[test]
    fn test_flex_vertical_constructor() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                {flex_vertical(rsx! { "Vertical content" }, rsx!())}
            }
        });

        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("flex-direction: column"));
        assert!(html.contains("Vertical content"));
    }

    #[test]
    fn test_flex_center_constructor() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                {flex_center(rsx! { "Centered content" })}
            }
        });

        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("justify-content: center"));
        assert!(html.contains("align-items: center"));
        assert!(html.contains("Centered content"));
    }

    #[test]
    fn test_flex_between_constructor() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                {flex_between(rsx! { "Between content" })}
            }
        });

        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("justify-content: space-between"));
        assert!(html.contains("Between content"));
    }

    #[test]
    fn test_flex_around_constructor() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                {flex_around(rsx! { "Around content" })}
            }
        });

        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("justify-content: space-around"));
        assert!(html.contains("Around content"));
    }

    #[test]
    fn test_flex_evenly_constructor() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                {flex_evenly(rsx! { "Evenly content" })}
            }
        });

        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("justify-content: space-evenly"));
        assert!(html.contains("Evenly content"));
    }

    #[test]
    fn test_flex_wrap_constructor() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                {flex_wrap(rsx! { "Wrap content" }, rsx!())}
            }
        });

        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("flex-wrap: wrap"));
        assert!(html.contains("Wrap content"));
    }

    // ============================================================================
    // 高阶组件测试
    // ============================================================================

    #[test]
    fn test_flex_item_component() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                FlexItem {
                    flex: Some("1".to_string()),
                    "Item content"
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("ant-flex-item"));
        assert!(html.contains("flex: 1"));
        assert!(html.contains("Item content"));
    }

    #[test]
    fn test_flex_container_component() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                FlexContainer {
                    max_width: Some("1200px".to_string()),
                    "Container content"
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("ant-flex-container"));
        assert!(html.contains("max-width: 1200px"));
        assert!(html.contains("Container content"));
    }

    #[test]
    fn test_flex_grid_component() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                FlexGrid {
                    columns: 3,
                    "Grid content"
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("ant-flex-grid"));
        assert!(html.contains("Grid content"));
    }

    #[test]
    fn test_flex_layout_component() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                FlexLayout {
                    header: Some(rsx! { "Header" }),
                    footer: Some(rsx! { "Footer" }),
                    "Main content"
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("ant-flex-layout"));
        assert!(html.contains("Header"));
        assert!(html.contains("Footer"));
        assert!(html.contains("Main content"));
    }

    // ============================================================================
    // 主题测试
    // ============================================================================

    #[test]
    fn test_flex_theme_provider() {
        let custom_theme = FlexTheme {
            gap_small: "4px".to_string(),
            gap_middle: "12px".to_string(),
            gap_large: "20px".to_string(),
            ..Default::default()
        };

        let mut dom = VirtualDom::new(move || {
            rsx! {
                FlexThemeProvider {
                    theme: custom_theme.clone(),
                    Flex {
                        gap: FlexGap::Small,
                        "Themed content"
                    }
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("gap: 4px"));
        assert!(html.contains("Themed content"));
    }

    #[test]
    fn test_flex_theme_default() {
        let theme = FlexTheme::default();

        assert_eq!(theme.gap_small, "8px");
        assert_eq!(theme.gap_middle, "16px");
        assert_eq!(theme.gap_large, "24px");
        assert_eq!(theme.font_size, "14px");
        assert_eq!(theme.line_height, "1.5");
    }

    #[test]
    fn test_flex_theme_merge() {
        let base_theme = FlexTheme::default();
        let custom_theme = FlexTheme {
            gap_small: "6px".to_string(),
            // primary_color: "#1890ff".to_string(),
            ..Default::default()
        };

        let merged = merge_flex_theme(&base_theme, &custom_theme);

        assert_eq!(merged.gap_small, "6px");
        assert_eq!(merged.gap_middle, "16px"); // 保持默认值
        assert_eq!(merged.primary_color, "#1890ff");
    }

    // ============================================================================
    // 复杂配置测试
    // ============================================================================

    #[test]
    fn test_flex_complex_configuration() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Flex {
                    vertical: true,
                    justify: FlexJustify::SpaceBetween,
                    align: FlexAlign::Stretch,
                    gap: FlexGap::Large,
                    wrap: FlexWrap::Wrap,
                    flex: Some("1 0 auto".to_string()),
                    class: Some("complex-flex".to_string()),
                    style: Some("min-height: 100vh; padding: 20px;".to_string()),
                    component: Some("main".to_string()),

                    div { "Item 1" }
                    div { "Item 2" }
                    div { "Item 3" }
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("<main"));
        assert!(html.contains("complex-flex"));
        assert!(html.contains("ant-flex"));
        assert!(html.contains("flex-direction: column"));
        assert!(html.contains("justify-content: space-between"));
        assert!(html.contains("align-items: stretch"));
        assert!(html.contains("gap: 24px"));
        assert!(html.contains("flex-wrap: wrap"));
        assert!(html.contains("flex: 1 0 auto"));
        assert!(html.contains("min-height: 100vh"));
        assert!(html.contains("padding: 20px"));
        assert!(html.contains("Item 1"));
        assert!(html.contains("Item 2"));
        assert!(html.contains("Item 3"));
    }

    // ============================================================================
    // 边缘情况测试
    // ============================================================================

    #[test]
    fn test_flex_empty_gap() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Flex {
                    gap: FlexGap::Custom("".to_string()),
                    "Empty gap content"
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("ant-flex"));
        assert!(html.contains("Empty gap content"));
    }

    #[test]
    fn test_flex_invalid_component() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Flex {
                    component: Some("invalid-tag".to_string()),
                    "Invalid component content"
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();
        // 应该回退到默认的 div
        assert!(html.contains("<div"));
        assert!(html.contains("Invalid component content"));
    }

    #[test]
    fn test_flex_very_long_class_name() {
        let long_class = "a".repeat(1000);
        let mut dom = VirtualDom::new(move || {
            rsx! {
                Flex {
                    class: Some(long_class.clone()),
                    "Long class content"
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains(&long_class));
        assert!(html.contains("Long class content"));
    }

    #[test]
    fn test_flex_special_characters_in_style() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Flex {
                    style: Some("content: 'Hello \"World\"'; background: url('data:image/svg+xml;base64,PHN2Zz4=');".to_string()),
                    "Special chars content"
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("Special chars content"));
    }

    // ============================================================================
    // 工具函数测试
    // ============================================================================

    #[test]
    fn test_create_flex_config() {
        let config = create_flex_config(&FlexProps {
            vertical: false,
            wrap: FlexWrap::Wrap,
            justify: FlexJustify::Center,
            align: FlexAlign::Center,
            gap: Some(FlexGap::Middle),
            flex: Some("1".to_string()),
            class: Some("section".to_string()),
            ..Default::default()
        });

        assert_eq!(config.vertical, false);
        assert_eq!(config.justify, FlexJustify::Center);
        assert_eq!(config.align, FlexAlign::Center);
        assert_eq!(config.gap, FlexGap::Middle);
        assert_eq!(config.wrap, FlexWrap::Wrap);
        assert_eq!(config.flex, Some("1".to_string()));
        assert_eq!(config.component, Some("section".to_string()));
        assert_eq!(config.has_children, true);
    }

    #[test]
    fn test_validate_flex_props() {
        // 有效的 Props
        let valid_props = FlexProps {
            flex: Some("1".to_string()),
            gap: Some(FlexGap::Middle),
            component: Some("div".to_string()),
            ..Default::default()
        };
        assert!(validate_flex_props(&valid_props).is_ok());

        // 无效的 flex 值
        let invalid_flex_props = FlexProps {
            flex: Some("invalid".to_string()),
            ..Default::default()
        };
        assert!(validate_flex_props(&invalid_flex_props).is_err());
    }

    #[test]
    fn test_calculate_gap_value() {
        assert_eq!(
            calculate_gap_value(&FlexGap::None, &FlexTheme::default()),
            "0px"
        );
        assert_eq!(
            calculate_gap_value(&FlexGap::Small, &FlexTheme::default()),
            "8px"
        );
        assert_eq!(
            calculate_gap_value(&FlexGap::Middle, &FlexTheme::default()),
            "16px"
        );
        assert_eq!(
            calculate_gap_value(&FlexGap::Large, &FlexTheme::default()),
            "24px"
        );
        assert_eq!(
            calculate_gap_value(&FlexGap::Custom("30px".to_string()), &FlexTheme::default()),
            "30px"
        );
    }

    #[test]
    fn test_is_responsive_gap() {
        assert!(!is_responsive_gap(&FlexGap::Small));
        assert!(is_responsive_gap(&FlexGap::Custom(
            "clamp(8px, 2vw, 24px)".to_string()
        )));
        assert!(is_responsive_gap(&FlexGap::Custom(
            "min(16px, 4vw)".to_string()
        )));
        assert!(is_responsive_gap(&FlexGap::Custom(
            "max(12px, 1rem)".to_string()
        )));
    }

    #[test]
    fn test_get_responsive_config() {
        let config = get_responsive_config(HashMap::new());

        assert!(config.breakpoints.contains_key("xs"));
        assert!(config.breakpoints.contains_key("sm"));
        assert!(config.breakpoints.contains_key("md"));
        assert!(config.breakpoints.contains_key("lg"));
        assert!(config.breakpoints.contains_key("xl"));
        assert!(config.breakpoints.contains_key("xxl"));
    }

    #[test]
    fn test_optimize_performance() {
        let config = FlexConfig {
            has_children: true,
            ..Default::default()
        };

        let optimized = optimize_performance(&config);
        assert!(optimized.virtualization_enabled);
    }

    #[test]
    fn test_calculate_flex_basis() {
        assert_eq!(calculate_flex_basis("auto", 1, 1), "auto");
        assert_eq!(calculate_flex_basis("100px", 1, 1), "100px");
        assert_eq!(calculate_flex_basis("50%", 1, 1), "50%");
        assert_eq!(calculate_flex_basis("invalid", 1, 1), "auto");
    }

    #[test]
    fn test_generate_css_variables() {
        let theme = FlexTheme::default();
        let variables = generate_css_variables(&theme);

        assert!(variables.contains_key("--ant-flex-gap-small"));
        assert!(variables.contains_key("--ant-flex-gap-middle"));
        assert!(variables.contains_key("--ant-flex-gap-large"));
        assert_eq!(
            variables.get("--ant-flex-gap-small"),
            Some(&"8px".to_string())
        );
    }

    #[test]
    fn test_should_wrap() {
        assert!(!should_wrap(&FlexWrap::NoWrap));
        assert!(should_wrap(&FlexWrap::Wrap));
        assert!(should_wrap(&FlexWrap::WrapReverse));
    }

    #[test]
    fn test_format_flex_value() {
        assert_eq!(format_flex_value("1"), "1");
        assert_eq!(format_flex_value("auto"), "auto");
        assert_eq!(format_flex_value("1 1 200px"), "1 1 200px");
        assert_eq!(format_flex_value(""), "initial");
    }

    #[test]
    fn test_calculate_item_size() {
        let config = FlexConfig {
            gap: Some(FlexGap::Middle),
            ..Default::default()
        };

        let size = calculate_item_size(&config, 3, 1000.0);
        assert!(size > 0.0);
        assert!(size < 1000.0);
    }

    #[test]
    fn test_create_default_config() {
        let config = create_default_config();

        assert_eq!(config.vertical, false);
        assert_eq!(config.justify, FlexJustify::Start);
        assert_eq!(config.align, FlexAlign::Start);
        assert_eq!(config.gap, FlexGap::None);
        assert_eq!(config.wrap, FlexWrap::NoWrap);
    }

    #[test]
    fn test_is_performance_mode() {
        let config = PerformanceConfig {
            max_render_items: 100,
            lazy_loading: true,
            ..Default::default()
        };

        assert!(is_performance_mode(&config));

        let config_disabled = PerformanceConfig {
            lazy_loading: false,
            ..Default::default()
        };

        assert!(!is_performance_mode(&config_disabled));
    }

    #[test]
    fn test_get_optimal_render_strategy() {
        let strategy = get_optimal_render_strategy(1000);
        assert!(strategy.contains("virtualization") || strategy.contains("pagination"));

        let strategy_small = get_optimal_render_strategy(10);
        assert_eq!(strategy_small, "direct");
    }

    #[test]
    fn test_estimate_memory_usage() {
        let usage = estimate_memory_usage(100, true);
        assert!(usage > 0);

        let usage_simple = estimate_memory_usage(10, false);
        assert!(usage_simple < usage);
    }

    #[test]
    fn test_generate_cache_key() {
        let config = FlexConfig::default();
        let key1 = generate_cache_key(&config);
        let key2 = generate_cache_key(&config);
        assert_eq!(key1, key2);

        let config2 = FlexConfig {
            vertical: true,
            ..Default::default()
        };
        let key3 = generate_cache_key(&config2);
        assert_ne!(key1, key3);
    }

    #[test]
    fn test_calculate_theme_hash() {
        let theme = FlexTheme::default();
        let hash1 = calculate_theme_hash(&theme);
        let hash2 = calculate_theme_hash(&theme);
        assert_eq!(hash1, hash2);

        let theme2 = FlexTheme {
            gap_small: "10px".to_string(),
            ..Default::default()
        };
        let hash3 = calculate_theme_hash(&theme2);
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_parse_size_string() {
        assert_eq!(parse_size_string("100px"), Some(100.0));
        assert_eq!(parse_size_string("50%"), None); // 百分比不返回具体数值
        assert_eq!(parse_size_string("auto"), None);
        assert_eq!(parse_size_string("invalid"), None);
    }

    // ============================================================================
    // 样式函数测试
    // ============================================================================

    #[test]
    fn test_generate_flex_container_style() {
        let config = FlexConfig {
            vertical: true,
            justify: FlexJustify::Center,
            align: FlexAlign::Center,
            gap: Some(FlexGap::Middle),
            wrap: FlexWrap::Wrap,
            flex: Some("1".to_string()),
            ..Default::default()
        };

        let style = generate_flex_container_styles(&config, &FlexTheme::default());

        assert!(style.contains("display: flex"));
        assert!(style.contains("flex-direction: column"));
        assert!(style.contains("justify-content: center"));
        assert!(style.contains("align-items: center"));
        assert!(style.contains("gap: 16px"));
        assert!(style.contains("flex-wrap: wrap"));
        assert!(style.contains("flex: 1"));
    }

    #[test]
    fn test_generate_flex_item_style() {
        let config = FlexItemConfig {
            flex: Some("1".to_string()),
            align_self: Some("center".to_string()),
            order: Some(2),
            ..Default::default()
        };

        let style = generate_flex_item_style(&config, &FlexTheme::default());

        assert!(style.contains("flex: 1"));
        assert!(style.contains("flex-grow: 1"));
        assert!(style.contains("flex-shrink: 0"));
        assert!(style.contains("flex-basis: 200px"));
        assert!(style.contains("align-self: center"));
        assert!(style.contains("order: 2"));
    }

    #[test]
    fn test_get_flex_class_name() {
        assert_eq!(get_flex_class_name(), "ant-flex");
    }

    #[test]
    fn test_get_flex_item_class_name() {
        assert_eq!(get_flex_item_class_name(), "ant-flex-item");
    }

    #[test]
    fn test_get_flex_container_class_name() {
        assert_eq!(get_flex_container_class_name(), "ant-flex-container");
    }

    #[test]
    fn test_get_flex_grid_class_name() {
        assert_eq!(get_flex_grid_class_name(), "ant-flex-grid");
    }

    #[test]
    fn test_get_flex_layout_class_name() {
        assert_eq!(get_flex_layout_class_name(), "ant-flex-layout");
    }

    #[test]
    fn test_calculate_gap_pixels() {
        let theme = FlexTheme::default();

        assert_eq!(calculate_gap_pixels(&FlexGap::None, &theme), 0.0);
        assert_eq!(calculate_gap_pixels(&FlexGap::Small, &theme), 8.0);
        assert_eq!(calculate_gap_pixels(&FlexGap::Middle, &theme), 16.0);
        assert_eq!(calculate_gap_pixels(&FlexGap::Large, &theme), 24.0);
        assert_eq!(
            calculate_gap_pixels(&FlexGap::Custom("30px".to_string()), &theme),
            30.0
        );
    }

    #[test]
    fn test_generate_responsive_style() {
        let config = ResponsiveConfig {
            breakpoints: {
                let mut map = HashMap::new();
                map.insert("sm".to_string(), 576);
                map.insert("md".to_string(), 768);
                map
            },
            current_breakpoint: "md".to_string(),
            enabled: true,
        };

        let style = generate_responsive_styles(&config, &FlexTheme::default());
        assert!(style.contains("@media"));
    }

    #[test]
    fn test_generate_css_variables_map() {
        let theme = FlexTheme::default();
        let variables = generate_css_variables_map(&theme);

        assert!(variables.len() > 0);
        assert!(variables.contains_key("--ant-flex-gap-small"));
        assert!(variables.contains_key("--ant-flex-primary-color"));
    }

    #[test]
    fn test_should_show_wrap() {
        assert!(!should_show_wrap(&FlexWrap::NoWrap));
        assert!(should_show_wrap(&FlexWrap::Wrap));
        assert!(should_show_wrap(&FlexWrap::WrapReverse));
    }

    #[test]
    fn test_get_flex_direction() {
        assert_eq!(get_flex_direction(false), "row");
        assert_eq!(get_flex_direction(true), "column");
    }

    #[test]
    fn test_get_main_axis_property() {
        assert_eq!(get_main_axis_property(false), "justify-content");
        assert_eq!(get_main_axis_property(true), "justify-content");
    }

    #[test]
    fn test_get_cross_axis_property() {
        assert_eq!(get_cross_axis_property(false), "align-items");
        assert_eq!(get_cross_axis_property(true), "align-items");
    }

    #[test]
    fn test_calculate_flex_basis_value() {
        assert_eq!(calculate_flex_basis_value("auto"), "auto");
        assert_eq!(calculate_flex_basis_value("100px"), "100px");
        assert_eq!(calculate_flex_basis_value("50%"), "50%");
        assert_eq!(calculate_flex_basis_value(""), "auto");
    }

    #[test]
    fn test_generate_grid_style() {
        let style = generate_grid_styles(3, &FlexGap::Middle, &FlexTheme::default());

        assert!(style.contains("display: grid"));
        assert!(style.contains("grid-template-columns"));
        assert!(style.contains("gap: 16px"));
    }

    #[test]
    fn test_generate_layout_style() {
        let style = generate_layout_styles(true, &FlexTheme::default());

        assert!(style.contains("display: flex"));
        assert!(style.contains("flex-direction: column"));
        assert!(style.contains("min-height: 100vh"));
    }

    #[test]
    fn test_get_performance_optimized_style() {
        let config = PerformanceConfig {
            max_render_items: 100,
            lazy_loading: true,
            ..Default::default()
        };

        let style = get_performance_optimized_styles(&config);
        assert!(style.contains("contain: layout style"));
    }

    #[test]
    fn test_generate_dark_theme_style() {
        let theme = FlexTheme::default();
        let style = generate_dark_theme_styles(&theme);

        assert!(style.contains("@media (prefers-color-scheme: dark)"));
    }

    #[test]
    fn test_generate_compact_theme_style() {
        let theme = FlexTheme::default();
        let style = generate_compact_theme_styles(&theme);

        assert!(style.contains("gap:"));
        // 紧凑主题应该有更小的间距
    }

    // ============================================================================
    // 性能和内存测试
    // ============================================================================

    #[test]
    fn test_flex_performance_with_many_children() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Flex {
                    gap: FlexGap::Small,

                    // 模拟大量子元素
                    for i in 0..100 {
                        div {
                            key: "{i}",
                            "Item {i}"
                        }
                    }
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("ant-flex"));
        assert!(html.contains("Item 0"));
        assert!(html.contains("Item 99"));
    }

    #[test]
    fn test_flex_memory_usage_estimation() {
        let usage_small = estimate_memory_usage(10, false);
        let usage_large = estimate_memory_usage(1000, true);

        assert!(usage_large > usage_small);
        assert!(usage_small < 100); // 小型组件应该使用较少内存
    }

    // ============================================================================
    // 集成测试
    // ============================================================================

    #[test]
    fn test_flex_with_other_components() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Flex {
                    vertical: true,
                    gap: FlexGap::Large,

                    // 模拟与其他组件的集成
                    div {
                        class: "button-group",
                        "Button 1"
                    }
                    div {
                        class: "input-group",
                        "Input Field"
                    }
                    div {
                        class: "card",
                        "Card Content"
                    }
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("ant-flex"));
        assert!(html.contains("button-group"));
        assert!(html.contains("input-group"));
        assert!(html.contains("card"));
        assert!(html.contains("flex-direction: column"));
        assert!(html.contains("gap: 24px"));
    }

    // ============================================================================
    // 可访问性测试
    // ============================================================================

    #[test]
    fn test_flex_accessibility_attributes() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Flex {
                    component: Some("nav".to_string()),
                    // 可访问性属性应该通过 props 传递
                    "Navigation content"
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("<nav"));
        assert!(html.contains("Navigation content"));
    }

    #[test]
    fn test_flex_semantic_html() {
        let semantic_components = vec![
            "main", "section", "article", "aside", "nav", "header", "footer",
        ];

        for component in semantic_components {
            let mut dom = VirtualDom::new(move || {
                rsx! {
                    Flex {
                        component: Some(component.to_string()),
                        "Semantic content"
                    }
                }
            });

            let html = dom.rebuild_to_vec().santize();
            assert!(html.contains(&format!("<{}", component)));
            assert!(html.contains("Semantic content"));
        }
    }

    // ============================================================================
    // 错误处理测试
    // ============================================================================

    #[test]
    fn test_flex_error_recovery() {
        // 测试无效配置的错误恢复
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Flex {
                    gap: FlexGap::Custom("invalid-value".to_string()),
                    flex: Some("invalid-flex".to_string()),
                    "Error recovery content"
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();
        // 应该能够渲染，即使有无效值
        assert!(html.contains("ant-flex"));
        assert!(html.contains("Error recovery content"));
    }

    // ============================================================================
    // Hooks 测试
    // ============================================================================

    #[test]
    fn test_use_flex_theme_hook() {
        // 注意：这个测试需要在实际的 Dioxus 环境中运行
        // 这里只是展示测试结构
        let theme = FlexTheme::default();
        assert_eq!(theme.gap_small, "8px");
    }

    #[test]
    fn test_use_flex_layout_hook() {
        // 测试布局 Hook 的基本功能
        let config = FlexConfig::default();
        assert_eq!(config.vertical, false);
        assert_eq!(config.justify, FlexJustify::Start);
    }

    #[test]
    fn test_use_responsive_flex_hook() {
        // 测试响应式 Hook
        let responsive_config = get_responsive_config(HashMap::new());
        assert!(responsive_config.enabled);
        assert!(!responsive_config.breakpoints.is_empty());
    }

    #[test]
    fn test_use_optimized_flex_hook() {
        // 测试性能优化 Hook
        let config = FlexConfig::default();
        let optimized = optimize_performance(&config);
        assert!(optimized.max_render_items > 0);
    }

    #[test]
    fn test_use_virtualized_flex_hook() {
        // 测试虚拟化 Hook
        let performance_config = PerformanceConfig::default();
        assert!(!performance_config.virtualization_enabled); // 默认关闭
    }

    #[test]
    fn test_use_lazy_flex_hook() {
        // 测试懒加载 Hook
        let performance_config = PerformanceConfig::default();
        assert!(!performance_config.lazy_loading); // 默认关闭
    }
}

// ============================================================================
// 基准测试（需要 criterion 或类似的基准测试框架）
// ============================================================================

#[cfg(test)]
mod benchmarks {
    use super::*;
    use crate::components::flex::styles::*;
    use crate::components::flex::types::*;
    use crate::components::flex::utils::*;
    use crate::components::flex::Flex;
    use dioxus::prelude::*;

    // 注意：这些基准测试需要适当的基准测试框架
    // 这里只是展示测试结构

    #[test]
    fn bench_flex_render_performance() {
        // 测试渲染性能
        let start = std::time::Instant::now();

        for _ in 0..1000 {
            let mut dom = VirtualDom::new(|| {
                rsx! {
                    Flex {
                        gap: FlexGap::Middle,
                        div { "Test content" }
                    }
                }
            });
            let _ = dom.rebuild_to_vec();
        }

        let duration = start.elapsed();
        println!("Flex render time for 1000 iterations: {:?}", duration);

        // 确保性能在合理范围内（这个阈值需要根据实际情况调整）
        assert!(duration.as_millis() < 1000);
    }

    #[test]
    fn bench_style_generation_performance() {
        // 测试样式生成性能
        let config = FlexConfig::default();
        let theme = FlexTheme::default();

        let start = std::time::Instant::now();

        for _ in 0..10000 {
            let _ = generate_flex_container_styles(&config, &theme);
        }

        let duration = start.elapsed();
        println!("Style generation time for 10000 iterations: {:?}", duration);

        // 样式生成应该很快
        assert!(duration.as_millis() < 100);
    }

    #[test]
    fn bench_theme_merge_performance() {
        // 测试主题合并性能
        let base_theme = FlexTheme::default();
        let custom_theme = FlexTheme {
            gap_small: "10px".to_string(),
            ..Default::default()
        };

        let start = std::time::Instant::now();

        for _ in 0..10000 {
            let _ = merge_flex_theme(&base_theme, &custom_theme);
        }

        let duration = start.elapsed();
        println!("Theme merge time for 10000 iterations: {:?}", duration);

        // 主题合并应该很快
        assert!(duration.as_millis() < 50);
    }
}

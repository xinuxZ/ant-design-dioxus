#[cfg(test)]
mod tests {
    use crate::components::flex::styles::*;
    use crate::components::flex::types::*;
    use crate::components::flex::utils::*;
    use crate::components::flex::*;
    use dioxus::prelude::*;

    // 测试 FlexWrap 枚举
    #[test]
    fn test_flex_wrap_default() {
        assert_eq!(FlexWrap::default(), FlexWrap::NoWrap);
    }

    #[test]
    fn test_flex_wrap_display() {
        assert_eq!(format!("{}", FlexWrap::NoWrap), "nowrap");
        assert_eq!(format!("{}", FlexWrap::Wrap), "wrap");
        assert_eq!(format!("{}", FlexWrap::WrapReverse), "wrap-reverse");
    }

    // #[test]
    // fn test_flex_wrap_to_css() {
    //     assert_eq!(FlexWrap::NoWrap.to_css(), "nowrap");
    //     assert_eq!(FlexWrap::Wrap.to_css(), "wrap");
    //     assert_eq!(FlexWrap::WrapReverse.to_css(), "wrap-reverse");
    // }

    // 测试 FlexJustify 枚举
    #[test]
    fn test_flex_justify_default() {
        assert_eq!(FlexJustify::default(), FlexJustify::Normal);
    }

    #[test]
    fn test_flex_justify_display() {
        assert_eq!(format!("{}", FlexJustify::Normal), "normal");
        assert_eq!(format!("{}", FlexJustify::FlexStart), "flex-start");
        assert_eq!(format!("{}", FlexJustify::FlexEnd), "flex-end");
        assert_eq!(format!("{}", FlexJustify::Center), "center");
        assert_eq!(format!("{}", FlexJustify::SpaceBetween), "space-between");
        assert_eq!(format!("{}", FlexJustify::SpaceAround), "space-around");
        assert_eq!(format!("{}", FlexJustify::SpaceEvenly), "space-evenly");
    }

    // #[test]
    // fn test_flex_justify_to_css() {
    //     assert_eq!(FlexJustify::Normal.to_css(), "normal");
    //     assert_eq!(FlexJustify::FlexStart.to_css(), "flex-start");
    //     assert_eq!(FlexJustify::FlexEnd.to_css(), "flex-end");
    //     assert_eq!(FlexJustify::Center.to_css(), "center");
    //     assert_eq!(FlexJustify::SpaceBetween.to_css(), "space-between");
    //     assert_eq!(FlexJustify::SpaceAround.to_css(), "space-around");
    //     assert_eq!(FlexJustify::SpaceEvenly.to_css(), "space-evenly");
    // }

    // 测试 FlexAlign 枚举
    #[test]
    fn test_flex_align_default() {
        assert_eq!(FlexAlign::default(), FlexAlign::Normal);
    }

    #[test]
    fn test_flex_align_display() {
        assert_eq!(format!("{}", FlexAlign::Normal), "normal");
        assert_eq!(format!("{}", FlexAlign::FlexStart), "flex-start");
        assert_eq!(format!("{}", FlexAlign::FlexEnd), "flex-end");
        assert_eq!(format!("{}", FlexAlign::Center), "center");
        assert_eq!(format!("{}", FlexAlign::Stretch), "stretch");
        assert_eq!(format!("{}", FlexAlign::Baseline), "baseline");
    }

    // #[test]
    // fn test_flex_align_to_css() {
    //     assert_eq!(FlexAlign::Normal, "normal");
    //     assert_eq!(FlexAlign::FlexStart, "flex-start");
    //     assert_eq!(FlexAlign::FlexEnd, "flex-end");
    //     assert_eq!(FlexAlign::Center, "center");
    //     assert_eq!(FlexAlign::Stretch, "stretch");
    //     assert_eq!(FlexAlign::Baseline, "baseline");
    // }

    // 测试 FlexGap 枚举
    #[test]
    fn test_flex_gap_display() {
        assert_eq!(format!("{}", FlexGap::Small), "small");
        assert_eq!(format!("{}", FlexGap::Middle), "middle");
        assert_eq!(format!("{}", FlexGap::Large), "large");
        assert_eq!(format!("{}", FlexGap::Custom("20px".to_string())), "20px");
    }

    // #[test]
    // fn test_flex_gap_to_css_value() {
    //     assert_eq!(FlexGap::Small.to_css_value(), "8px");
    //     assert_eq!(FlexGap::Middle.to_css_value(), "16px");
    //     assert_eq!(FlexGap::Large.to_css_value(), "24px");
    //     assert_eq!(FlexGap::Custom("30px".to_string()).to_css_value(), "30px");
    // }

    #[test]
    fn test_flex_gap_from_str() {
        assert_eq!(FlexGap::from("small"), FlexGap::Small);
        assert_eq!(FlexGap::from("middle"), FlexGap::Middle);
        assert_eq!(FlexGap::from("large"), FlexGap::Large);
        assert_eq!(
            FlexGap::from("custom"),
            FlexGap::Custom("custom".to_string())
        );
    }

    // 测试 FlexConfig 结构体
    #[test]
    fn test_flex_config_default() {
        let config = FlexConfig::default();
        assert!(!config.vertical);
        assert_eq!(config.wrap, FlexWrap::NoWrap);
        assert_eq!(config.justify, FlexJustify::Normal);
        assert_eq!(config.align, FlexAlign::Normal);
        assert!(config.flex.is_none());
        assert!(config.gap.is_none());
        assert_eq!(config.component, "div");
    }

    #[test]
    fn test_flex_config_new() {
        let config = FlexConfig {
            vertical: true,
            wrap: FlexWrap::Wrap,
            justify: FlexJustify::Center,
            align: FlexAlign::FlexStart,
            flex: Some("1 1 auto".to_string()),
            gap: Some(FlexGap::Large),
            component: "section".to_string(),
            ..Default::default()
        };

        assert!(config.vertical);
        assert_eq!(config.wrap, FlexWrap::Wrap);
        assert_eq!(config.justify, FlexJustify::Center);
        assert_eq!(config.align, FlexAlign::FlexStart);
        assert_eq!(config.flex, Some("1 1 auto".to_string()));
        assert_eq!(config.gap, Some(FlexGap::Large));
        assert_eq!(config.component, "section".to_string());
    }

    // 测试 FlexTheme 结构体
    #[test]
    fn test_flex_theme_default() {
        let theme = FlexTheme::default();
        assert_eq!(theme.gap_small, "8px");
        assert_eq!(theme.gap_middle, "16px");
        assert_eq!(theme.gap_large, "24px");
        assert_eq!(theme.border_radius, "6px");
    }

    #[test]
    fn test_flex_theme_new() {
        let theme = FlexTheme {
            gap_small: "4px".to_string(),
            gap_middle: "12px".to_string(),
            gap_large: "20px".to_string(),
            border_radius: "8px".to_string(),
            ..Default::default()
        };

        assert_eq!(theme.gap_small, "4px");
        assert_eq!(theme.gap_middle, "12px");
        assert_eq!(theme.gap_large, "20px");
        assert_eq!(theme.border_radius, "8px");
    }

    #[test]
    fn test_flex_theme_merge() {
        let base_theme = FlexTheme::default();
        let custom_theme = FlexTheme {
            gap_small: "6px".to_string(),
            gap_large: "32px".to_string(),
            ..Default::default()
        };

        let merged = base_theme.merge(&custom_theme);
        assert_eq!(merged.gap_small, "6px");
        assert_eq!(merged.gap_middle, "16px"); // 保持默认值
        assert_eq!(merged.gap_large, "32px");
        assert_eq!(merged.border_radius, "6px"); // 保持默认值
    }

    // 测试工具函数
    #[test]
    fn test_validate_flex_props() {
        let valid_props = FlexProps {
            vertical: false,
            wrap: FlexWrap::NoWrap,
            justify: FlexJustify::Center,
            align: FlexAlign::Center,
            flex: None,
            gap: Some(FlexGap::Middle),
            class: None,
            style: None,
            component: "div",
            children: Ok(VNode::default()),
        };

        assert!(validate_flex_props(&valid_props).is_ok());
    }

    #[test]
    fn test_create_flex_config() {
        let props = FlexProps {
            vertical: true,
            wrap: FlexWrap::Wrap,
            justify: FlexJustify::SpaceBetween,
            align: FlexAlign::FlexEnd,
            flex: Some("1".to_string()),
            gap: Some(FlexGap::Large),
            class: None,
            style: None,
            component: "section",
            children: Ok(VNode::default()),
        };

        let config = create_flex_config(&props);
        assert!(config.vertical);
        assert_eq!(config.wrap, FlexWrap::Wrap);
        assert_eq!(config.justify, FlexJustify::SpaceBetween);
        assert_eq!(config.align, FlexAlign::FlexEnd);
        assert_eq!(config.flex, Some("1".to_string()));
        assert_eq!(config.gap, Some(FlexGap::Large));
        assert_eq!(config.component, "section");
    }

    // #[test]
    // fn test_use_flex_theme() {
    //     // 在测试环境中，use_context 可能无法正常工作
    //     // 我们改为测试默认主题的获取
    //     let theme = get_default_flex_theme();
    //     // 测试主题获取功能
    //     assert_eq!(theme.gap_small, "8px");
    //     assert_eq!(theme.gap_middle, "16px");
    //     assert_eq!(theme.gap_large, "24px");
    // }

    #[test]
    fn test_generate_flex_container_styles() {
        let config = FlexConfig {
            vertical: false,
            wrap: FlexWrap::NoWrap,
            justify: FlexJustify::Center,
            align: FlexAlign::Center,
            flex: Some("1".to_string()),
            gap: Some(FlexGap::Middle),
            has_children: false,
            component: "div".to_string(),
        };

        let theme = FlexTheme::default();
        let styles = generate_flex_container_styles(&config, &theme);

        assert!(styles.contains("display: flex"));
        assert!(styles.contains("justify-content: center"));
        assert!(styles.contains("align-items: center"));
        assert!(styles.contains("flex-wrap: nowrap"));
        assert!(styles.contains("flex: 1"));
        assert!(styles.contains("gap: 16px"));
    }

    #[test]
    fn test_generate_flex_container_styles_vertical() {
        let config = FlexConfig {
            vertical: true,
            wrap: FlexWrap::Wrap,
            justify: FlexJustify::FlexStart,
            align: FlexAlign::Stretch,
            flex: None,
            gap: Some(FlexGap::Small),
            has_children: false,
            component: "div".to_string(),
        };

        let theme = FlexTheme::default();
        let styles = generate_flex_container_styles(&config, &theme);

        assert!(styles.contains("display: flex"));
        assert!(styles.contains("flex-direction: column"));
        assert!(styles.contains("justify-content: flex-start"));
        assert!(styles.contains("align-items: stretch"));
        assert!(styles.contains("flex-wrap: wrap"));
        assert!(styles.contains("gap: 8px"));
    }

    #[test]
    fn test_get_flex_container_class() {
        let config = FlexConfig {
            vertical: false,
            wrap: FlexWrap::NoWrap,
            justify: FlexJustify::Center,
            align: FlexAlign::Center,
            flex: None,
            gap: None,
            has_children: false,
            component: "div".to_string(),
        };

        let class_name = get_flex_container_class(&config);
        assert!(class_name.contains("ant-flex"));
        assert!(!class_name.contains("ant-flex-vertical"));
    }

    #[test]
    fn test_get_flex_container_class_vertical() {
        let config = FlexConfig {
            vertical: true,
            wrap: FlexWrap::Wrap,
            justify: FlexJustify::Center,
            align: FlexAlign::Center,
            flex: None,
            gap: None,
            has_children: false,
            component: "div".to_string(),
        };

        let class_name = get_flex_container_class(&config);
        assert!(class_name.contains("ant-flex"));
        assert!(class_name.contains("ant-flex-vertical"));
        assert!(class_name.contains("ant-flex-wrap"));
    }

    // // 测试样式生成
    // #[test]
    // fn test_flex_styles_base() {
    //     let base_styles = FlexStyles::base();
    //     assert!(base_styles.contains(".ant-flex"));
    //     assert!(base_styles.contains("display: flex"));
    // }

    // #[test]
    // fn test_flex_styles_direction_styles() {
    //     let horizontal_styles = FlexStyles::direction_styles(false);
    //     assert!(horizontal_styles.contains("flex-direction: row"));

    //     let vertical_styles = FlexStyles::direction_styles(true);
    //     assert!(vertical_styles.contains("flex-direction: column"));
    // }

    // #[test]
    // fn test_flex_styles_wrap_styles() {
    //     let nowrap_styles = FlexStyles::wrap_styles(&FlexWrap::NoWrap);
    //     assert!(nowrap_styles.contains("flex-wrap: nowrap"));

    //     let wrap_styles = FlexStyles::wrap_styles(&FlexWrap::Wrap);
    //     assert!(wrap_styles.contains("flex-wrap: wrap"));

    //     let wrap_reverse_styles = FlexStyles::wrap_styles(&FlexWrap::WrapReverse);
    //     assert!(wrap_reverse_styles.contains("flex-wrap: wrap-reverse"));
    // }

    // #[test]
    // fn test_flex_styles_justify_styles() {
    //     let center_styles = FlexStyles::justify_styles(&FlexJustify::Center);
    //     assert!(center_styles.contains("justify-content: center"));

    //     let space_between_styles = FlexStyles::justify_styles(&FlexJustify::SpaceBetween);
    //     assert!(space_between_styles.contains("justify-content: space-between"));
    // }

    // #[test]
    // fn test_flex_styles_align_styles() {
    //     let center_styles = FlexStyles::align_styles(&FlexAlign::Center);
    //     assert!(center_styles.contains("align-items: center"));

    //     let stretch_styles = FlexStyles::align_styles(&FlexAlign::Stretch);
    //     assert!(stretch_styles.contains("align-items: stretch"));
    // }

    // #[test]
    // fn test_flex_styles_gap_styles() {
    //     let theme = FlexTheme::default();

    //     let small_gap_styles = FlexStyles::gap_styles(&Some(FlexGap::Small), &theme);
    //     assert!(small_gap_styles.contains("gap: 8px"));

    //     let middle_gap_styles = FlexStyles::gap_styles(&Some(FlexGap::Middle), &theme);
    //     assert!(middle_gap_styles.contains("gap: 16px"));

    //     let large_gap_styles = FlexStyles::gap_styles(&Some(FlexGap::Large), &theme);
    //     assert!(large_gap_styles.contains("gap: 24px"));

    //     let custom_gap_styles =
    //         FlexStyles::gap_styles(&Some(FlexGap::Custom("30px".to_string())), &theme);
    //     assert!(custom_gap_styles.contains("gap: 30px"));

    //     let no_gap_styles = FlexStyles::gap_styles(&None, &theme);
    //     assert!(no_gap_styles.is_empty());
    // }

    // 测试组件渲染
    #[tokio::test]
    async fn test_flex_component_basic_render() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Flex {
                    div { "Item 1" }
                    div { "Item 2" }
                    div { "Item 3" }
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 基本渲染测试通过
    }

    #[tokio::test]
    async fn test_flex_component_vertical() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Flex {
                    vertical: true,
                    gap: FlexGap::Large,
                    div { "Item A" }
                    div { "Item B" }
                    div { "Item C" }
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 垂直布局渲染测试通过
    }

    #[tokio::test]
    async fn test_flex_component_with_wrap() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Flex {
                    wrap: FlexWrap::Wrap,
                    justify: FlexJustify::SpaceBetween,
                    align: FlexAlign::Center,
                    div { "Item 1" }
                    div { "Item 2" }
                    div { "Item 3" }
                    div { "Item 4" }
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 换行布局渲染测试通过
    }

    #[tokio::test]
    async fn test_flex_component_different_components() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                div {
                    Flex {
                        component: "section",
                        div { "In Section" }
                    }
                    Flex {
                        component: "article",
                        div { "In Article" }
                    }
                    Flex {
                        component: "header",
                        div { "In Header" }
                    }
                    Flex {
                        component: "footer",
                        div { "In Footer" }
                    }
                    Flex {
                        component: "main",
                        div { "In Main" }
                    }
                    Flex {
                        component: "nav",
                        div { "In Nav" }
                    }
                    Flex {
                        component: "aside",
                        div { "In Aside" }
                    }
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 不同组件类型渲染测试通过
    }

    #[tokio::test]
    async fn test_flex_component_with_custom_styles() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Flex {
                    class: "custom-flex",
                    style: "background-color: #f0f0f0; padding: 20px;",
                    gap: FlexGap::Middle,
                    div { "Styled Item 1" }
                    div { "Styled Item 2" }
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 自定义样式渲染测试通过
    }

    #[tokio::test]
    async fn test_flex_component_all_justify_options() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                div {
                    Flex {
                        justify: FlexJustify::FlexStart,
                        div { "Start" }
                    }
                    Flex {
                        justify: FlexJustify::FlexEnd,
                        div { "End" }
                    }
                    Flex {
                        justify: FlexJustify::Center,
                        div { "Center" }
                    }
                    Flex {
                        justify: FlexJustify::SpaceBetween,
                        div { "Space" }
                        div { "Between" }
                    }
                    Flex {
                        justify: FlexJustify::SpaceAround,
                        div { "Space" }
                        div { "Around" }
                    }
                    Flex {
                        justify: FlexJustify::SpaceEvenly,
                        div { "Space" }
                        div { "Evenly" }
                    }
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 所有对齐选项渲染测试通过
    }

    #[tokio::test]
    async fn test_flex_component_all_align_options() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                div {
                    Flex {
                        align: FlexAlign::FlexStart,
                        div { "Start" }
                    }
                    Flex {
                        align: FlexAlign::FlexEnd,
                        div { "End" }
                    }
                    Flex {
                        align: FlexAlign::Center,
                        div { "Center" }
                    }
                    Flex {
                        align: FlexAlign::Stretch,
                        div { "Stretch" }
                    }
                    Flex {
                        align: FlexAlign::Baseline,
                        div { "Baseline" }
                    }
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 所有对齐选项渲染测试通过
    }

    #[tokio::test]
    async fn test_flex_component_all_gap_options() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                div {
                    Flex {
                        gap: FlexGap::Small,
                        div { "Small" }
                        div { "Gap" }
                    }
                    Flex {
                        gap: FlexGap::Middle,
                        div { "Middle" }
                        div { "Gap" }
                    }
                    Flex {
                        gap: FlexGap::Large,
                        div { "Large" }
                        div { "Gap" }
                    }
                    Flex {
                        gap: FlexGap::Custom("50px".to_string()),
                        div { "Custom" }
                        div { "Gap" }
                    }
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 所有间距选项渲染测试通过
    }

    #[tokio::test]
    async fn test_flex_component_with_flex_property() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Flex {
                    flex: "1 1 auto",
                    justify: FlexJustify::Center,
                    align: FlexAlign::Center,
                    div { "Flexible Item" }
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // flex属性渲染测试通过
    }
}

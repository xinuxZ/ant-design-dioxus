#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::space::styles::*;
    use crate::components::space::types::*;
    use crate::components::space::utils::*;
    use crate::components::space::*;
    use dioxus::prelude::*;

    // 测试 SpaceDirection 枚举
    #[test]
    fn test_space_direction_default() {
        assert_eq!(SpaceDirection::default(), SpaceDirection::Horizontal);
    }

    #[test]
    fn test_space_direction_display() {
        assert_eq!(format!("{}", SpaceDirection::Horizontal), "horizontal");
        assert_eq!(format!("{}", SpaceDirection::Vertical), "vertical");
    }

    #[test]
    fn test_space_direction_from_str() {
        assert_eq!(
            SpaceDirection::from("horizontal"),
            SpaceDirection::Horizontal
        );
        assert_eq!(SpaceDirection::from("vertical"), SpaceDirection::Vertical);
        assert_eq!(SpaceDirection::from("unknown"), SpaceDirection::Horizontal);
    }

    #[test]
    fn test_space_direction_to_string() {
        assert_eq!(SpaceDirection::Horizontal.to_string(), "horizontal");
        assert_eq!(SpaceDirection::Vertical.to_string(), "vertical");
    }

    // #[test]
    // fn test_space_direction_is_horizontal() {
    //     assert!(SpaceDirection::Horizontal.is_horizontal());
    //     assert!(!SpaceDirection::Vertical.is_horizontal());
    // }

    // #[test]
    // fn test_space_direction_is_vertical() {
    //     assert!(!SpaceDirection::Horizontal.is_vertical());
    //     assert!(SpaceDirection::Vertical.is_vertical());
    // }

    // #[test]
    // fn test_space_direction_to_flex_direction() {
    //     assert_eq!(SpaceDirection::Horizontal.to_flex_direction(), "row");
    //     assert_eq!(SpaceDirection::Vertical.to_flex_direction(), "column");
    // }

    // 测试 SpaceAlign 枚举
    #[test]
    fn test_space_align_default() {
        assert_eq!(SpaceAlign::default(), SpaceAlign::Start);
    }

    #[test]
    fn test_space_align_display() {
        assert_eq!(format!("{}", SpaceAlign::Start), "start");
        assert_eq!(format!("{}", SpaceAlign::End), "end");
        assert_eq!(format!("{}", SpaceAlign::Center), "center");
        assert_eq!(format!("{}", SpaceAlign::Baseline), "baseline");
    }

    #[test]
    fn test_space_align_from_str() {
        assert_eq!(SpaceAlign::from("start"), SpaceAlign::Start);
        assert_eq!(SpaceAlign::from("end"), SpaceAlign::End);
        assert_eq!(SpaceAlign::from("center"), SpaceAlign::Center);
        assert_eq!(SpaceAlign::from("baseline"), SpaceAlign::Baseline);
        assert_eq!(SpaceAlign::from("unknown"), SpaceAlign::Start);
    }

    #[test]
    fn test_space_align_to_string() {
        assert_eq!(SpaceAlign::Start.to_string(), "start");
        assert_eq!(SpaceAlign::End.to_string(), "end");
        assert_eq!(SpaceAlign::Center.to_string(), "center");
        assert_eq!(SpaceAlign::Baseline.to_string(), "baseline");
    }

    // #[test]
    // fn test_space_align_to_css_value() {
    //     assert_eq!(SpaceAlign::Start.to_css_value(), "flex-start");
    //     assert_eq!(SpaceAlign::End.to_css_value(), "flex-end");
    //     assert_eq!(SpaceAlign::Center.to_css_value(), "center");
    //     assert_eq!(SpaceAlign::Baseline.to_css_value(), "baseline");
    // }

    // 测试 SpaceSize 枚举
    #[test]
    fn test_space_size_default() {
        assert_eq!(SpaceSize::default(), SpaceSize::Small);
    }

    #[test]
    fn test_space_size_display() {
        assert_eq!(format!("{}", SpaceSize::Small), "small");
        assert_eq!(format!("{}", SpaceSize::Middle), "middle");
        assert_eq!(format!("{}", SpaceSize::Large), "large");
        assert_eq!(format!("{}", SpaceSize::Custom(20)), "20");
        assert_eq!(
            format!("{}", SpaceSize::Responsive(vec![8, 16, 24])),
            "responsive"
        );
    }

    // #[test]
    // fn test_space_size_to_pixels() {
    //     assert_eq!(SpaceSize::Small.to_pixels(), 8);
    //     assert_eq!(SpaceSize::Middle.to_pixels(), 16);
    //     assert_eq!(SpaceSize::Large.to_pixels(), 24);
    //     assert_eq!(SpaceSize::Custom(30).to_pixels(), 30);
    //     assert_eq!(SpaceSize::Responsive(vec![8, 16, 24]).to_pixels(), 8);
    // }

    #[test]
    fn test_space_size_from_pixels() {
        assert_eq!(SpaceSize::from(8), SpaceSize::Small);
        assert_eq!(SpaceSize::from(16), SpaceSize::Middle);
        assert_eq!(SpaceSize::from(24), SpaceSize::Large);
        assert_eq!(SpaceSize::from(32), SpaceSize::Custom(32));
    }

    // #[test]
    // fn test_space_size_to_css_value() {
    //     assert_eq!(SpaceSize::Small.to_css_value(), "8px");
    //     assert_eq!(SpaceSize::Middle.to_css_value(), "16px");
    //     assert_eq!(SpaceSize::Large.to_css_value(), "24px");
    //     assert_eq!(SpaceSize::Custom(40).to_css_value(), "40px");
    //     assert_eq!(SpaceSize::Responsive(vec![8, 16, 24]).to_css_value(), "8px");
    // }

    // #[test]
    // fn test_space_size_is_responsive() {
    //     assert!(!SpaceSize::Small.is_responsive());
    //     assert!(!SpaceSize::Middle.is_responsive());
    //     assert!(!SpaceSize::Large.is_responsive());
    //     assert!(!SpaceSize::Custom(20).is_responsive());
    //     assert!(SpaceSize::Responsive(vec![8, 16]).is_responsive());
    // }

    // #[test]
    // fn test_space_size_get_responsive_values() {
    //     assert_eq!(SpaceSize::Small.get_responsive_values(), vec![8]);
    //     assert_eq!(SpaceSize::Middle.get_responsive_values(), vec![16]);
    //     assert_eq!(SpaceSize::Large.get_responsive_values(), vec![24]);
    //     assert_eq!(SpaceSize::Custom(30).get_responsive_values(), vec![30]);
    //     assert_eq!(
    //         SpaceSize::Responsive(vec![8, 16, 24]).get_responsive_values(),
    //         vec![8, 16, 24]
    //     );
    // }

    #[test]
    fn test_space_size_get_value_for_breakpoint() {
        let responsive_size = SpaceSize::Responsive(vec![8, 16, 24]);
        assert_eq!(responsive_size.get_value_for_breakpoint(0), 8);
        assert_eq!(responsive_size.get_value_for_breakpoint(1), 16);
        assert_eq!(responsive_size.get_value_for_breakpoint(2), 24);
        assert_eq!(responsive_size.get_value_for_breakpoint(3), 24); // 超出范围返回最后一个值

        let fixed_size = SpaceSize::Small;
        assert_eq!(fixed_size.get_value_for_breakpoint(0), 8);
        assert_eq!(fixed_size.get_value_for_breakpoint(5), 8);
    }

    // 测试 SpaceWrap 枚举
    #[test]
    fn test_space_wrap_default() {
        assert_eq!(SpaceWrap::default(), SpaceWrap::NoWrap);
    }

    #[test]
    fn test_space_wrap_display() {
        assert_eq!(format!("{}", SpaceWrap::NoWrap), "nowrap");
        assert_eq!(format!("{}", SpaceWrap::Wrap), "wrap");
        assert_eq!(format!("{}", SpaceWrap::WrapReverse), "wrap-reverse");
    }

    #[test]
    fn test_space_wrap_from_str() {
        assert_eq!(SpaceWrap::from("nowrap"), SpaceWrap::NoWrap);
        assert_eq!(SpaceWrap::from("wrap"), SpaceWrap::Wrap);
        assert_eq!(SpaceWrap::from("wrap-reverse"), SpaceWrap::WrapReverse);
        assert_eq!(SpaceWrap::from("unknown"), SpaceWrap::NoWrap);
    }

    #[test]
    fn test_space_wrap_to_string() {
        assert_eq!(SpaceWrap::NoWrap.to_string(), "nowrap");
        assert_eq!(SpaceWrap::Wrap.to_string(), "wrap");
        assert_eq!(SpaceWrap::WrapReverse.to_string(), "wrap-reverse");
    }

    #[test]
    fn test_space_wrap_to_css_value() {
        assert_eq!(SpaceWrap::NoWrap.to_css_value(), "nowrap");
        assert_eq!(SpaceWrap::Wrap.to_css_value(), "wrap");
        assert_eq!(SpaceWrap::WrapReverse.to_css_value(), "wrap-reverse");
    }

    #[test]
    fn test_space_wrap_allows_wrapping() {
        assert!(!SpaceWrap::NoWrap.allows_wrapping());
        assert!(SpaceWrap::Wrap.allows_wrapping());
        assert!(SpaceWrap::WrapReverse.allows_wrapping());
    }

    // 测试 SpaceConfig 结构体
    #[test]
    fn test_space_config_default() {
        let config = SpaceConfig::default();
        assert_eq!(config.direction, SpaceDirection::Horizontal);
        assert_eq!(config.align, SpaceAlign::Start);
        assert_eq!(config.size, SpaceSize::Small);
        assert_eq!(config.wrap, SpaceWrap::NoWrap);
        assert!(config.split.is_none());
        assert_eq!(config.compact, false);
    }

    #[test]
    fn test_space_config_new() {
        let config = SpaceConfig::new(
            SpaceDirection::Vertical,
            SpaceAlign::Center,
            SpaceSize::Large,
            SpaceWrap::Wrap,
            Some("|".to_string()),
            true,
        );

        assert_eq!(config.direction, SpaceDirection::Vertical);
        assert_eq!(config.align, SpaceAlign::Center);
        assert_eq!(config.size, SpaceSize::Large);
        assert_eq!(config.wrap, SpaceWrap::Wrap);
        assert_eq!(config.split, Some("|".to_string()));
        assert_eq!(config.compact, true);
    }

    #[test]
    fn test_space_config_with_direction() {
        let mut config = SpaceConfig::default();
        config = config.with_direction(SpaceDirection::Vertical);

        assert_eq!(config.direction, SpaceDirection::Vertical);
    }

    #[test]
    fn test_space_config_with_align() {
        let mut config = SpaceConfig::default();
        config = config.with_align(SpaceAlign::End);

        assert_eq!(config.align, SpaceAlign::End);
    }

    #[test]
    fn test_space_config_with_size() {
        let mut config = SpaceConfig::default();
        config = config.with_size(SpaceSize::Custom(32));

        assert_eq!(config.size, SpaceSize::Custom(32));
    }

    #[test]
    fn test_space_config_with_wrap() {
        let mut config = SpaceConfig::default();
        config = config.with_wrap(SpaceWrap::Wrap);

        assert_eq!(config.wrap, SpaceWrap::Wrap);
    }

    #[test]
    fn test_space_config_with_split() {
        let mut config = SpaceConfig::default();
        config = config.with_split(Some("-".to_string()));

        assert_eq!(config.split, Some("-".to_string()));
    }

    #[test]
    fn test_space_config_with_compact() {
        let mut config = SpaceConfig::default();
        config = config.with_compact(true);

        assert_eq!(config.compact, true);
    }

    #[test]
    fn test_space_config_is_horizontal() {
        let horizontal_config = SpaceConfig {
            direction: SpaceDirection::Horizontal,
            ..Default::default()
        };
        assert!(horizontal_config.is_horizontal());

        let vertical_config = SpaceConfig {
            direction: SpaceDirection::Vertical,
            ..Default::default()
        };
        assert!(!vertical_config.is_horizontal());
    }

    #[test]
    fn test_space_config_is_vertical() {
        let vertical_config = SpaceConfig {
            direction: SpaceDirection::Vertical,
            ..Default::default()
        };
        assert!(vertical_config.is_vertical());

        let horizontal_config = SpaceConfig {
            direction: SpaceDirection::Horizontal,
            ..Default::default()
        };
        assert!(!horizontal_config.is_vertical());
    }

    #[test]
    fn test_space_config_has_split() {
        let with_split = SpaceConfig {
            split: Some("|".to_string()),
            ..Default::default()
        };
        assert!(with_split.has_split());

        let without_split = SpaceConfig {
            split: None,
            ..Default::default()
        };
        assert!(!without_split.has_split());
    }

    #[test]
    fn test_space_config_allows_wrapping() {
        let wrapping_config = SpaceConfig {
            wrap: SpaceWrap::Wrap,
            ..Default::default()
        };
        assert!(wrapping_config.allows_wrapping());

        let no_wrap_config = SpaceConfig {
            wrap: SpaceWrap::NoWrap,
            ..Default::default()
        };
        assert!(!no_wrap_config.allows_wrapping());
    }

    #[test]
    fn test_space_config_get_gap_value() {
        let config = SpaceConfig {
            size: SpaceSize::Middle,
            compact: false,
            ..Default::default()
        };
        assert_eq!(config.get_gap_value(), 16);

        let compact_config = SpaceConfig {
            size: SpaceSize::Middle,
            compact: true,
            ..Default::default()
        };
        assert_eq!(compact_config.get_gap_value(), 8); // 紧凑模式减半
    }

    // 测试工具函数
    #[test]
    fn test_validate_space_props() {
        let valid_props = SpaceProps {
            children: Ok(VNode::default()),
            align: Some(SpaceAlign::Center),
            class: None,
            style: None,
            direction: Some(SpaceDirection::Horizontal),
            size: Some(SpaceSize::Middle),
            split: None,
            wrap: Some(SpaceWrap::NoWrap),
            theme: None,
            onclick: None,
        };

        assert!(validate_space_props(&valid_props).is_ok());
    }

    #[test]
    fn test_create_space_config() {
        let props = SpaceProps {
            children: Ok(VNode::default()),
            align: Some(SpaceAlign::End),
            class: None,
            style: None,
            direction: Some(SpaceDirection::Vertical),
            size: Some(SpaceSize::Large),
            // split: Some("|".to_string()),
            split: Ok(VNode::default()),
            wrap: Some(SpaceWrap::Wrap),
            theme: None,
            onclick: None,
        };

        let config = create_space_config(&props);
        assert_eq!(config.direction, SpaceDirection::Vertical);
        assert_eq!(config.align, SpaceAlign::End);
        assert_eq!(config.size, SpaceSize::Large);
        assert_eq!(config.wrap, SpaceWrap::Wrap);
        assert_eq!(config.split, Some("|".to_string()));
    }

    #[test]
    fn test_calculate_space_gap() {
        assert_eq!(calculate_space_gap(&SpaceSize::Small, false), 8);
        assert_eq!(calculate_space_gap(&SpaceSize::Middle, false), 16);
        assert_eq!(calculate_space_gap(&SpaceSize::Large, false), 24);
        assert_eq!(calculate_space_gap(&SpaceSize::Custom(30), false), 30);

        // 紧凑模式
        assert_eq!(calculate_space_gap(&SpaceSize::Small, true), 4);
        assert_eq!(calculate_space_gap(&SpaceSize::Middle, true), 8);
        assert_eq!(calculate_space_gap(&SpaceSize::Large, true), 12);
        assert_eq!(calculate_space_gap(&SpaceSize::Custom(30), true), 15);
    }

    #[test]
    fn test_get_space_container_class() {
        let config = SpaceConfig {
            direction: SpaceDirection::Vertical,
            align: SpaceAlign::Center,
            wrap: SpaceWrap::Wrap,
            compact: true,
            ..Default::default()
        };

        let class_name = get_space_container_class(&config);
        assert!(class_name.contains("ant-space"));
        assert!(class_name.contains("ant-space-vertical"));
        assert!(class_name.contains("ant-space-align-center"));
        assert!(class_name.contains("ant-space-wrap"));
        assert!(class_name.contains("ant-space-compact"));
    }

    #[test]
    fn test_get_space_container_class_horizontal() {
        let config = SpaceConfig {
            direction: SpaceDirection::Horizontal,
            align: SpaceAlign::Start,
            wrap: SpaceWrap::NoWrap,
            compact: false,
            ..Default::default()
        };

        let class_name = get_space_container_class(&config);
        assert!(class_name.contains("ant-space"));
        assert!(class_name.contains("ant-space-horizontal"));
        assert!(class_name.contains("ant-space-align-start"));
        assert!(!class_name.contains("ant-space-wrap"));
        assert!(!class_name.contains("ant-space-compact"));
    }

    #[test]
    fn test_get_space_container_styles() {
        let config = SpaceConfig {
            direction: SpaceDirection::Horizontal,
            align: SpaceAlign::Center,
            size: SpaceSize::Middle,
            wrap: SpaceWrap::Wrap,
            compact: false,
            ..Default::default()
        };

        let styles = get_space_container_styles(&config);
        assert!(styles.contains("display: flex"));
        assert!(styles.contains("flex-direction: row"));
        assert!(styles.contains("align-items: center"));
        assert!(styles.contains("gap: 16px"));
        assert!(styles.contains("flex-wrap: wrap"));
    }

    #[test]
    fn test_get_space_container_styles_vertical() {
        let config = SpaceConfig {
            direction: SpaceDirection::Vertical,
            align: SpaceAlign::End,
            size: SpaceSize::Large,
            wrap: SpaceWrap::NoWrap,
            compact: true,
            ..Default::default()
        };

        let styles = get_space_container_styles(&config);
        assert!(styles.contains("display: flex"));
        assert!(styles.contains("flex-direction: column"));
        assert!(styles.contains("align-items: flex-end"));
        assert!(styles.contains("gap: 12px")); // 紧凑模式下24px变为12px
        assert!(styles.contains("flex-wrap: nowrap"));
    }

    #[test]
    fn test_get_space_item_class() {
        let class_name = get_space_item_class();
        assert_eq!(class_name, "ant-space-item");
    }

    #[test]
    fn test_get_space_split_class() {
        let class_name = get_space_split_class();
        assert_eq!(class_name, "ant-space-split");
    }

    #[test]
    fn test_merge_space_styles() {
        let base_styles = "display: flex;";
        let custom_styles = Some("margin: 10px;".to_string());

        let merged = merge_space_styles(base_styles, custom_styles.as_deref());
        assert!(merged.contains("display: flex;"));
        assert!(merged.contains("margin: 10px;"));

        let no_custom = merge_space_styles(base_styles, None);
        assert_eq!(no_custom, base_styles);
    }

    #[test]
    fn test_merge_space_classes() {
        let base_class = "ant-space";
        let custom_class = Some("custom-space".to_string());

        let merged = merge_space_classes(base_class, custom_class.as_deref());
        assert!(merged.contains("ant-space"));
        assert!(merged.contains("custom-space"));

        let no_custom = merge_space_classes(base_class, None);
        assert_eq!(no_custom, base_class);
    }

    // #[test]
    // fn test_handle_space_click() {
    //     let mut clicked = false;
    //     let callback = move |_| {
    //         clicked = true;
    //     };

    //     // 模拟点击事件
    //     let event = MouseEvent::new("click".to_string());
    //     handle_space_click(Some(callback), event);
    //     // 由于闭包的限制，这里无法直接测试clicked的值变化
    //     // 但函数调用本身测试了事件处理逻辑
    // }

    #[test]
    fn test_generate_responsive_gap_styles() {
        let responsive_size = SpaceSize::Responsive(vec![8, 16, 24]);
        let styles = generate_responsive_gap_styles(&responsive_size, false);

        assert!(styles.contains("gap: 8px"));
        assert!(styles.contains("@media"));
        assert!(styles.contains("gap: 16px"));
        assert!(styles.contains("gap: 24px"));
    }

    #[test]
    fn test_generate_responsive_gap_styles_compact() {
        let responsive_size = SpaceSize::Responsive(vec![8, 16, 24]);
        let styles = generate_responsive_gap_styles(&responsive_size, true);

        assert!(styles.contains("gap: 4px")); // 紧凑模式减半
        assert!(styles.contains("@media"));
        assert!(styles.contains("gap: 8px"));
        assert!(styles.contains("gap: 12px"));
    }

    // 测试样式生成
    #[test]
    fn test_space_styles_base() {
        let base_styles = SpaceStyles::base();
        assert!(base_styles.contains(".ant-space"));
        assert!(base_styles.contains("display: flex"));
        assert!(base_styles.contains("align-items: flex-start"));
    }

    #[test]
    fn test_space_styles_direction_styles() {
        let horizontal_styles = SpaceStyles::direction_styles(&SpaceDirection::Horizontal);
        assert!(horizontal_styles.contains(".ant-space-horizontal"));
        assert!(horizontal_styles.contains("flex-direction: row"));

        let vertical_styles = SpaceStyles::direction_styles(&SpaceDirection::Vertical);
        assert!(vertical_styles.contains(".ant-space-vertical"));
        assert!(vertical_styles.contains("flex-direction: column"));
    }

    #[test]
    fn test_space_styles_align_styles() {
        let start_styles = SpaceStyles::align_styles(&SpaceAlign::Start);
        assert!(start_styles.contains(".ant-space-align-start"));
        assert!(start_styles.contains("align-items: flex-start"));

        let center_styles = SpaceStyles::align_styles(&SpaceAlign::Center);
        assert!(center_styles.contains(".ant-space-align-center"));
        assert!(center_styles.contains("align-items: center"));

        let end_styles = SpaceStyles::align_styles(&SpaceAlign::End);
        assert!(end_styles.contains(".ant-space-align-end"));
        assert!(end_styles.contains("align-items: flex-end"));

        let baseline_styles = SpaceStyles::align_styles(&SpaceAlign::Baseline);
        assert!(baseline_styles.contains(".ant-space-align-baseline"));
        assert!(baseline_styles.contains("align-items: baseline"));
    }

    #[test]
    fn test_space_styles_size_styles() {
        let small_styles = SpaceStyles::size_styles(&SpaceSize::Small);
        assert!(small_styles.contains(".ant-space-small"));
        assert!(small_styles.contains("gap: 8px"));

        let middle_styles = SpaceStyles::size_styles(&SpaceSize::Middle);
        assert!(middle_styles.contains(".ant-space-middle"));
        assert!(middle_styles.contains("gap: 16px"));

        let large_styles = SpaceStyles::size_styles(&SpaceSize::Large);
        assert!(large_styles.contains(".ant-space-large"));
        assert!(large_styles.contains("gap: 24px"));
    }

    #[test]
    fn test_space_styles_wrap_styles() {
        let wrap_styles = SpaceStyles::wrap_styles(&SpaceWrap::Wrap);
        assert!(wrap_styles.contains(".ant-space-wrap"));
        assert!(wrap_styles.contains("flex-wrap: wrap"));

        let nowrap_styles = SpaceStyles::wrap_styles(&SpaceWrap::NoWrap);
        assert!(nowrap_styles.contains(".ant-space-nowrap"));
        assert!(nowrap_styles.contains("flex-wrap: nowrap"));

        let wrap_reverse_styles = SpaceStyles::wrap_styles(&SpaceWrap::WrapReverse);
        assert!(wrap_reverse_styles.contains(".ant-space-wrap-reverse"));
        assert!(wrap_reverse_styles.contains("flex-wrap: wrap-reverse"));
    }

    #[test]
    fn test_space_styles_item_styles() {
        let item_styles = SpaceStyles::item_styles();
        assert!(item_styles.contains(".ant-space-item"));
        assert!(item_styles.contains("display: inline-block"));
    }

    #[test]
    fn test_space_styles_split_styles() {
        let split_styles = SpaceStyles::split_styles();
        assert!(split_styles.contains(".ant-space-split"));
        assert!(split_styles.contains("display: inline-block"));
    }

    #[test]
    fn test_space_styles_compact_styles() {
        let compact_styles = SpaceStyles::compact_styles();
        assert!(compact_styles.contains(".ant-space-compact"));
        assert!(compact_styles.contains("gap: calc"));
    }

    #[test]
    fn test_space_styles_responsive_styles() {
        let responsive_styles = SpaceStyles::responsive_styles();
        assert!(responsive_styles.contains("@media"));
        assert!(responsive_styles.contains("min-width"));
    }

    // 测试组件渲染
    #[tokio::test]
    async fn test_space_component_basic_render() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Space {
                    children: rsx! {
                        div { "Item 1" }
                        div { "Item 2" }
                        div { "Item 3" }
                    }
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 基本渲染测试通过
    }

    #[tokio::test]
    async fn test_space_component_vertical_direction() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Space {
                    direction: SpaceDirection::Vertical,
                    children: rsx! {
                        div { "Item 1" }
                        div { "Item 2" }
                    }
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 垂直方向渲染测试通过
    }

    #[tokio::test]
    async fn test_space_component_different_sizes() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                div {
                    Space {
                        size: SpaceSize::Small,
                        children: rsx! {
                            div { "Small" }
                            div { "Space" }
                        }
                    }
                    Space {
                        size: SpaceSize::Middle,
                        children: rsx! {
                            div { "Middle" }
                            div { "Space" }
                        }
                    }
                    Space {
                        size: SpaceSize::Large,
                        children: rsx! {
                            div { "Large" }
                            div { "Space" }
                        }
                    }
                    Space {
                        size: SpaceSize::Custom(32),
                        children: rsx! {
                            div { "Custom" }
                            div { "Space" }
                        }
                    }
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 不同尺寸渲染测试通过
    }

    #[tokio::test]
    async fn test_space_component_different_alignments() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                div {
                    Space {
                        align: SpaceAlign::Start,
                        children: rsx! {
                            div { "Start" }
                            div { "Aligned" }
                        }
                    }
                    Space {
                        align: SpaceAlign::Center,
                        children: rsx! {
                            div { "Center" }
                            div { "Aligned" }
                        }
                    }
                    Space {
                        align: SpaceAlign::End,
                        children: rsx! {
                            div { "End" }
                            div { "Aligned" }
                        }
                    }
                    Space {
                        align: SpaceAlign::Baseline,
                        children: rsx! {
                            div { "Baseline" }
                            div { "Aligned" }
                        }
                    }
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 不同对齐方式渲染测试通过
    }

    #[tokio::test]
    async fn test_space_component_with_wrap() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Space {
                    wrap: SpaceWrap::Wrap,
                    children: rsx! {
                        div { "Item 1" }
                        div { "Item 2" }
                        div { "Item 3" }
                        div { "Item 4" }
                        div { "Item 5" }
                    }
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 换行渲染测试通过
    }

    #[tokio::test]
    async fn test_space_component_with_split() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Space {
                    split: "|",
                    children: rsx! {
                        div { "Item 1" }
                        div { "Item 2" }
                        div { "Item 3" }
                    }
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 分割符渲染测试通过
    }

    #[tokio::test]
    async fn test_space_component_responsive_size() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Space {
                    size: SpaceSize::Responsive(vec![8, 16, 24]),
                    children: rsx! {
                        div { "Responsive" }
                        div { "Space" }
                    }
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 响应式尺寸渲染测试通过
    }

    #[tokio::test]
    async fn test_space_component_with_click_handler() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Space {
                    onclick: move |_| {
                        // 点击处理逻辑
                    },
                    children: rsx! {
                        div { "Clickable" }
                        div { "Space" }
                    }
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 点击事件渲染测试通过
    }

    #[tokio::test]
    async fn test_space_component_with_custom_styles() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Space {
                    class: "custom-space",
                    style: "background-color: #f0f0f0; padding: 16px;",
                    children: rsx! {
                        div { "Styled" }
                        div { "Space" }
                    }
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 自定义样式渲染测试通过
    }

    #[tokio::test]
    async fn test_space_component_empty_children() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Space {
                    // 无子元素
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 空子元素渲染测试通过
    }
}

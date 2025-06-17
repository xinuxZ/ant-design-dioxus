//! Divider 组件测试

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::divider::styles::*;
    use crate::components::divider::types::*;
    use crate::components::divider::utils::*;
    use crate::components::divider::Divider;
    use dioxus::prelude::*;
    use std::collections::HashMap;

    // 基础渲染测试

    #[test]
    fn test_divider_basic_render() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Divider {}
            }
        });

        let _ = dom.rebuild_to_vec();
        // 验证基础渲染不会崩溃
    }

    #[test]
    fn test_divider_with_text() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Divider {
                    "测试文本"
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 验证带文本的分割线渲染
    }

    #[test]
    fn test_vertical_divider() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Divider {
                    r#type: DividerType::Vertical,
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 验证垂直分割线渲染
    }

    // 类型和枚举测试

    #[test]
    fn test_divider_type_display() {
        assert_eq!(DividerType::Horizontal.to_string(), "horizontal");
        assert_eq!(DividerType::Vertical.to_string(), "vertical");
    }

    #[test]
    fn test_divider_type_from_str() {
        assert_eq!(DividerType::from("vertical"), DividerType::Vertical);
        assert_eq!(DividerType::from("horizontal"), DividerType::Horizontal);
        assert_eq!(DividerType::from("invalid"), DividerType::Horizontal);
    }

    #[test]
    fn test_divider_orientation_display() {
        assert_eq!(DividerOrientation::Left.to_string(), "left");
        assert_eq!(DividerOrientation::Right.to_string(), "right");
        assert_eq!(DividerOrientation::Center.to_string(), "center");
        assert_eq!(DividerOrientation::Start.to_string(), "start");
        assert_eq!(DividerOrientation::End.to_string(), "end");
    }

    #[test]
    fn test_divider_orientation_from_str() {
        assert_eq!(DividerOrientation::from("left"), DividerOrientation::Left);
        assert_eq!(DividerOrientation::from("right"), DividerOrientation::Right);
        assert_eq!(
            DividerOrientation::from("center"),
            DividerOrientation::Center
        );
        assert_eq!(DividerOrientation::from("start"), DividerOrientation::Start);
        assert_eq!(DividerOrientation::from("end"), DividerOrientation::End);
        assert_eq!(
            DividerOrientation::from("invalid"),
            DividerOrientation::Center
        );
    }

    #[test]
    fn test_divider_variant_display() {
        assert_eq!(DividerVariant::Solid.to_string(), "solid");
        assert_eq!(DividerVariant::Dashed.to_string(), "dashed");
        assert_eq!(DividerVariant::Dotted.to_string(), "dotted");
    }

    #[test]
    fn test_divider_variant_from_str() {
        assert_eq!(DividerVariant::from("solid"), DividerVariant::Solid);
        assert_eq!(DividerVariant::from("dashed"), DividerVariant::Dashed);
        assert_eq!(DividerVariant::from("dotted"), DividerVariant::Dotted);
        assert_eq!(DividerVariant::from("invalid"), DividerVariant::Solid);
    }

    #[test]
    fn test_divider_size_display() {
        assert_eq!(DividerSize::Small.to_string(), "small");
        assert_eq!(DividerSize::Middle.to_string(), "middle");
        assert_eq!(DividerSize::Large.to_string(), "large");
    }

    #[test]
    fn test_divider_size_from_str() {
        assert_eq!(DividerSize::from("small"), DividerSize::Small);
        assert_eq!(DividerSize::from("middle"), DividerSize::Middle);
        assert_eq!(DividerSize::from("large"), DividerSize::Large);
        assert_eq!(DividerSize::from("invalid"), DividerSize::Middle);
    }

    // Props 构建器测试

    #[test]
    fn test_divider_props_builder() {
        let props = DividerProps::new()
            .r#type(DividerType::Vertical)
            .orientation(DividerOrientation::Left)
            .variant(DividerVariant::Dashed)
            .plain(true)
            .size(DividerSize::Large)
            .class("custom-class")
            .style("color: red");

        assert_eq!(props.r#type, DividerType::Vertical);
        assert_eq!(props.orientation, DividerOrientation::Left);
        assert_eq!(props.variant, DividerVariant::Dashed);
        assert_eq!(props.plain, true);
        assert_eq!(props.size, Some(DividerSize::Large));
        assert_eq!(props.class, Some("custom-class".to_string()));
        assert_eq!(props.style, Some("color: red".to_string()));
    }

    #[test]
    fn test_divider_props_dashed_compatibility() {
        let props = DividerProps::new().dashed(true);
        assert_eq!(props.dashed, true);
        assert_eq!(props.variant, DividerVariant::Dashed);
    }

    #[test]
    fn test_divider_props_orientation_margin() {
        let props = DividerProps::new().orientation_margin("20px");
        assert_eq!(props.orientation_margin, Some("20px".to_string()));
    }

    // 配置测试

    #[test]
    fn test_divider_config_from_props() {
        let props = DividerProps {
            r#type: DividerType::Horizontal,
            orientation: DividerOrientation::Left,
            variant: DividerVariant::Dashed,
            plain: true,
            size: Some(DividerSize::Small),
            dashed: false,
            ..Default::default()
        };

        let config = DividerConfig::from_props(&props, true);

        assert_eq!(config.r#type, DividerType::Horizontal);
        assert_eq!(config.orientation, DividerOrientation::Left);
        assert_eq!(config.variant, DividerVariant::Dashed);
        assert_eq!(config.plain, true);
        assert_eq!(config.size, DividerSize::Small);
        assert_eq!(config.has_text, true);
    }

    #[test]
    fn test_divider_config_dashed_compatibility() {
        let props = DividerProps {
            dashed: true,
            variant: DividerVariant::Solid,
            ..Default::default()
        };

        let config = DividerConfig::from_props(&props, false);
        assert_eq!(config.variant, DividerVariant::Dashed);
    }

    #[test]
    fn test_divider_config_methods() {
        let config = DividerConfig {
            r#type: DividerType::Vertical,
            has_text: true,
            variant: DividerVariant::Dotted,
            ..Default::default()
        };

        assert!(config.is_vertical());
        assert!(!config.is_horizontal());
        assert!(config.has_text());
        assert_eq!(config.get_border_style(), "dotted");
    }

    // 工具函数测试

    #[test]
    fn test_create_divider_config() {
        let config = create_divider_config(
            DividerType::Vertical,
            DividerOrientation::Right,
            DividerVariant::Dashed,
            true,
            DividerSize::Large,
            false,
        );

        assert_eq!(config.r#type, DividerType::Vertical);
        assert_eq!(config.orientation, DividerOrientation::Right);
        assert_eq!(config.variant, DividerVariant::Dashed);
        assert_eq!(config.plain, true);
        assert_eq!(config.size, DividerSize::Large);
        assert_eq!(config.has_text, false);
    }

    #[test]
    fn test_validate_divider_props() {
        // 有效的 Props
        let valid_props = DividerProps {
            r#type: DividerType::Horizontal,
            orientation_margin: Some("16px".to_string()),
            ..Default::default()
        };
        assert!(validate_divider_props(&valid_props).is_ok());

        // 垂直分割线不应该有文本
        let invalid_props = DividerProps {
            r#type: DividerType::Vertical,
            children: Some(rsx! { "text" }),
            ..Default::default()
        };
        assert!(validate_divider_props(&invalid_props).is_err());

        // 无效的边距格式
        let invalid_margin_props = DividerProps {
            orientation_margin: Some("invalid".to_string()),
            ..Default::default()
        };
        assert!(validate_divider_props(&invalid_margin_props).is_err());
    }

    #[test]
    fn test_is_valid_margin() {
        assert!(is_valid_margin("10px"));
        assert!(is_valid_margin("1.5em"));
        assert!(is_valid_margin("100%"));
        assert!(is_valid_margin("0"));
        assert!(is_valid_margin("20"));
        assert!(is_valid_margin("2.5rem"));
        assert!(is_valid_margin("50vh"));

        assert!(!is_valid_margin(""));
        assert!(!is_valid_margin("invalid"));
        assert!(!is_valid_margin("px10"));
    }

    #[test]
    fn test_calculate_text_margin() {
        // 居中对齐
        let margin = calculate_text_margin(&DividerOrientation::Center, None, &DividerSize::Middle);
        assert_eq!(margin, "0 16px");

        // 左对齐
        let margin = calculate_text_margin(&DividerOrientation::Left, None, &DividerSize::Small);
        assert_eq!(margin, "0 12px 0 0");

        // 右对齐
        let margin = calculate_text_margin(&DividerOrientation::Right, None, &DividerSize::Large);
        assert_eq!(margin, "0 0 0 20px");

        // 自定义边距
        let custom_margin = "10px 20px".to_string();
        let margin = calculate_text_margin(
            &DividerOrientation::Center,
            Some(&custom_margin),
            &DividerSize::Middle,
        );
        assert_eq!(margin, "10px 20px");
    }

    #[test]
    fn test_get_actual_orientation() {
        // 非 RTL 环境
        assert_eq!(
            get_actual_orientation(&DividerOrientation::Start),
            DividerOrientation::Left
        );
        assert_eq!(
            get_actual_orientation(&DividerOrientation::End),
            DividerOrientation::Right
        );
        assert_eq!(
            get_actual_orientation(&DividerOrientation::Center),
            DividerOrientation::Center
        );
    }

    #[test]
    fn test_calculate_divider_thickness() {
        assert_eq!(
            calculate_divider_thickness(&DividerSize::Small, &DividerVariant::Solid),
            "0.5px"
        );
        assert_eq!(
            calculate_divider_thickness(&DividerSize::Middle, &DividerVariant::Solid),
            "1px"
        );
        assert_eq!(
            calculate_divider_thickness(&DividerSize::Large, &DividerVariant::Solid),
            "2px"
        );
        assert_eq!(
            calculate_divider_thickness(&DividerSize::Middle, &DividerVariant::Dotted),
            "1.5px"
        );
    }

    // 主题测试

    #[test]
    fn test_divider_theme_default() {
        let theme = DividerTheme::default();
        assert_eq!(theme.color_split, "rgba(5, 5, 5, 0.06)");
        assert_eq!(theme.color_text, "rgba(0, 0, 0, 0.88)");
        assert_eq!(theme.font_size, "14px");
        assert_eq!(theme.margin, "16px 0");
    }

    #[test]
    fn test_divider_dark_theme_default() {
        let dark_theme = DividerDarkTheme::default();
        assert_eq!(dark_theme.color_split, "rgba(253, 253, 253, 0.12)");
        assert_eq!(dark_theme.color_text, "rgba(255, 255, 255, 0.85)");
    }

    #[test]
    fn test_divider_compact_theme_default() {
        let compact_theme = DividerCompactTheme::default();
        assert_eq!(compact_theme.margin, "12px 0");
        assert_eq!(compact_theme.text_padding, "0 12px");
    }

    #[test]
    fn test_generate_css_variables() {
        let theme = DividerTheme::default();
        let variables = generate_css_variables(&theme);

        assert!(variables.contains_key("--divider-color-split"));
        assert!(variables.contains_key("--divider-color-text"));
        assert!(variables.contains_key("--divider-font-size"));
        assert_eq!(
            variables.get("--divider-color-split"),
            Some(&theme.color_split)
        );
    }

    #[test]
    fn test_merge_theme() {
        let base = DividerTheme::default();
        let override_theme = DividerTheme {
            color_split: "red".to_string(),
            font_size: "16px".to_string(),
            ..DividerTheme::default()
        };

        let merged = merge_theme(&base, &override_theme);
        assert_eq!(merged.color_split, "red");
        assert_eq!(merged.font_size, "16px");
        assert_eq!(merged.color_text, base.color_text);
    }

    #[test]
    fn test_apply_dark_theme() {
        let base = DividerTheme::default();
        let dark = DividerDarkTheme::default();

        let dark_theme = apply_dark_theme(&base, &dark);
        assert_eq!(dark_theme.color_split, dark.color_split);
        assert_eq!(dark_theme.color_text, dark.color_text);
        assert_eq!(dark_theme.font_size, base.font_size); // 保持基础主题的其他属性
    }

    #[test]
    fn test_apply_compact_theme() {
        let base = DividerTheme::default();
        let compact = DividerCompactTheme::default();

        let compact_theme = apply_compact_theme(&base, &compact);
        assert_eq!(compact_theme.margin, compact.margin);
        assert_eq!(compact_theme.text_padding, compact.text_padding);
        assert_eq!(compact_theme.color_text, base.color_text); // 保持基础主题的其他属性
    }

    // 解析函数测试

    #[test]
    fn test_parse_size_string() {
        assert_eq!(parse_size_string("small"), Some(DividerSize::Small));
        assert_eq!(parse_size_string("LARGE"), Some(DividerSize::Large));
        assert_eq!(parse_size_string("md"), Some(DividerSize::Middle));
        assert_eq!(parse_size_string("invalid"), None);
    }

    #[test]
    fn test_parse_type_string() {
        assert_eq!(
            parse_type_string("horizontal"),
            Some(DividerType::Horizontal)
        );
        assert_eq!(parse_type_string("V"), Some(DividerType::Vertical));
        assert_eq!(parse_type_string("invalid"), None);
    }

    #[test]
    fn test_parse_orientation_string() {
        assert_eq!(
            parse_orientation_string("left"),
            Some(DividerOrientation::Left)
        );
        assert_eq!(
            parse_orientation_string("C"),
            Some(DividerOrientation::Center)
        );
        assert_eq!(parse_orientation_string("invalid"), None);
    }

    #[test]
    fn test_parse_variant_string() {
        assert_eq!(parse_variant_string("solid"), Some(DividerVariant::Solid));
        assert_eq!(parse_variant_string("DASH"), Some(DividerVariant::Dashed));
        assert_eq!(parse_variant_string("invalid"), None);
    }

    // 工具函数测试

    #[test]
    fn test_generate_theme_hash() {
        let theme1 = DividerTheme::default();
        let theme2 = DividerTheme {
            color_split: "red".to_string(),
            ..DividerTheme::default()
        };

        let hash1 = generate_theme_hash(&theme1);
        let hash2 = generate_theme_hash(&theme2);
        assert_ne!(hash1, hash2);

        let hash1_again = generate_theme_hash(&theme1);
        assert_eq!(hash1, hash1_again);
    }

    #[test]
    fn test_is_valid_css_color() {
        // 十六进制颜色
        assert!(is_valid_css_color("#ff0000"));
        assert!(is_valid_css_color("#f00"));
        assert!(is_valid_css_color("#ff000080"));
        assert!(is_valid_css_color("#gg0000"));

        // RGB/RGBA
        assert!(is_valid_css_color("rgb(255, 0, 0)"));
        assert!(is_valid_css_color("rgba(255, 0, 0, 0.5)"));

        // HSL/HSLA
        assert!(is_valid_css_color("hsl(0, 100%, 50%)"));
        assert!(is_valid_css_color("hsla(0, 100%, 50%, 0.5)"));

        // CSS 颜色关键字
        assert!(is_valid_css_color("red"));
        assert!(is_valid_css_color("transparent"));
        assert!(is_valid_css_color("WHITE"));

        // 无效颜色
        assert!(!is_valid_css_color("invalid-color"));
    }

    // 便捷构造函数测试

    #[test]
    fn test_convenience_constructors() {
        let config = get_default_config();
        assert_eq!(config.r#type, DividerType::Horizontal);
        assert_eq!(config.variant, DividerVariant::Solid);

        let h_config = horizontal_divider();
        assert_eq!(h_config.r#type, DividerType::Horizontal);

        let v_config = vertical_divider();
        assert_eq!(v_config.r#type, DividerType::Vertical);

        let dashed_config = dashed_divider();
        assert_eq!(dashed_config.variant, DividerVariant::Dashed);

        let dotted_config = dotted_divider();
        assert_eq!(dotted_config.variant, DividerVariant::Dotted);

        let text_config = text_divider(DividerOrientation::Left);
        assert_eq!(text_config.orientation, DividerOrientation::Left);
        assert_eq!(text_config.has_text, true);
    }

    // 内存测试

    #[test]
    fn test_theme_memory_usage() {
        let theme = DividerTheme::default();
        let size = std::mem::size_of_val(&theme);

        // 确保主题结构体大小合理
        assert!(size < 1024); // 小于 1KB
    }

    #[test]
    fn test_config_memory_usage() {
        let config = DividerConfig::default();
        let size = std::mem::size_of_val(&config);

        // 确保配置结构体大小合理
        assert!(size < 256); // 小于 256 字节
    }

    // 边缘情况测试

    #[test]
    fn test_extreme_values() {
        // 极大的边距值
        assert!(is_valid_margin("9999px"));

        // 极小的边距值
        assert!(is_valid_margin("0.001em"));

        // 负值边距
        assert!(is_valid_margin("-10px"));
    }

    // 集成测试

    #[test]
    fn test_theme_switching() {
        let base_theme = DividerTheme::default();
        let dark_theme = DividerDarkTheme::default();
        let compact_theme = DividerCompactTheme::default();

        // 应用暗色主题
        let dark_applied = apply_dark_theme(&base_theme, &dark_theme);
        assert_eq!(dark_applied.color_split, dark_theme.color_split);

        // 应用紧凑主题
        let compact_applied = apply_compact_theme(&base_theme, &compact_theme);
        assert_eq!(compact_applied.margin, compact_theme.margin);

        // 同时应用两个主题
        let both_applied = apply_compact_theme(&dark_applied, &compact_theme);
        assert_eq!(both_applied.color_split, dark_theme.color_split);
        assert_eq!(both_applied.margin, compact_theme.margin);
    }
}

// 基准测试
#[cfg(test)]
mod benchmarks {
    use crate::components::divider::styles::*;
    use crate::components::divider::types::*;
    use crate::components::divider::utils::*;
    use std::time::Instant;

    #[test]
    fn bench_theme_hash_generation() {
        let theme = DividerTheme::default();

        let start = Instant::now();
        for _ in 0..10000 {
            let _ = generate_theme_hash(&theme);
        }
        let duration = start.elapsed();

        println!("Theme hash generation: {:?} for 10k iterations", duration);
        assert!(duration.as_millis() < 200);
    }
}

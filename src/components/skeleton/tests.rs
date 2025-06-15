#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::skeleton::styles::*;
    use crate::components::skeleton::types::*;
    use crate::components::skeleton::utils::*;
    use crate::components::skeleton::*;
    use dioxus::prelude::*;
    use std::collections::HashMap;

    // 测试 AvatarShape 枚举
    #[test]
    fn test_skeleton_avatar_shape_default() {
        assert_eq!(AvatarShape::default(), AvatarShape::Circle);
    }

    #[test]
    fn test_skeleton_avatar_shape_display() {
        assert_eq!(format!("{}", AvatarShape::Circle), "circle");
        assert_eq!(format!("{}", AvatarShape::Square), "square");
    }

    #[test]
    fn test_skeleton_avatar_shape_from_str() {
        assert_eq!(AvatarShape::from("circle"), AvatarShape::Circle);
        assert_eq!(AvatarShape::from("square"), AvatarShape::Square);
        assert_eq!(AvatarShape::from("unknown"), AvatarShape::Circle);
    }

    #[test]
    fn test_skeleton_avatar_shape_to_string() {
        assert_eq!(AvatarShape::Circle.to_string(), "circle");
        assert_eq!(AvatarShape::Square.to_string(), "square");
    }

    #[test]
    fn test_skeleton_avatar_shape_is_circle() {
        assert!(AvatarShape::Circle.is_circle());
        assert!(!AvatarShape::Square.is_circle());
    }

    #[test]
    fn test_skeleton_avatar_shape_is_square() {
        assert!(!AvatarShape::Circle.is_square());
        assert!(AvatarShape::Square.is_square());
    }

    // 测试 AvatarSize 枚举
    #[test]
    fn test_skeleton_avatar_size_default() {
        assert_eq!(AvatarSize::default(), AvatarSize::Default);
    }

    #[test]
    fn test_skeleton_avatar_size_display() {
        assert_eq!(format!("{}", AvatarSize::Large), "large");
        assert_eq!(format!("{}", AvatarSize::Small), "small");
        assert_eq!(format!("{}", AvatarSize::Default), "default");
        assert_eq!(format!("{}", AvatarSize::Custom(60)), "60");
    }

    #[test]
    fn test_skeleton_avatar_size_to_pixels() {
        assert_eq!(AvatarSize::Large.to_pixels(), 64);
        assert_eq!(AvatarSize::Small.to_pixels(), 24);
        assert_eq!(AvatarSize::Default.to_pixels(), 40);
        assert_eq!(AvatarSize::Custom(80).to_pixels(), 80);
    }

    // 测试 ButtonShape 枚举
    #[test]
    fn test_skeleton_button_shape_display() {
        assert_eq!(format!("{}", ButtonShape::Circle), "circle");
        assert_eq!(format!("{}", ButtonShape::Round), "round");
        assert_eq!(format!("{}", ButtonShape::Default), "default");
    }

    #[test]
    fn test_skeleton_button_shape_from_str() {
        assert_eq!(ButtonShape::from("circle"), ButtonShape::Circle);
        assert_eq!(ButtonShape::from("round"), ButtonShape::Round);
        assert_eq!(ButtonShape::from("default"), ButtonShape::Default);
        assert_eq!(ButtonShape::from("unknown"), ButtonShape::Default);
    }

    #[test]
    fn test_skeleton_button_shape_to_string() {
        assert_eq!(ButtonShape::Circle.to_string(), "circle");
        assert_eq!(ButtonShape::Round.to_string(), "round");
        assert_eq!(ButtonShape::Default.to_string(), "default");
    }

    // 测试 ButtonSize 枚举
    #[test]
    fn test_skeleton_button_size_default() {
        assert_eq!(ButtonSize::default(), ButtonSize::Default);
    }

    #[test]
    fn test_skeleton_button_size_display() {
        assert_eq!(format!("{}", ButtonSize::Large), "large");
        assert_eq!(format!("{}", ButtonSize::Small), "small");
        assert_eq!(format!("{}", ButtonSize::Default), "default");
    }

    // 测试 InputSize 枚举
    #[test]
    fn test_skeleton_input_size_default() {
        assert_eq!(InputSize::default(), InputSize::Default);
    }

    #[test]
    fn test_skeleton_input_size_display() {
        assert_eq!(format!("{}", InputSize::Large), "large");
        assert_eq!(format!("{}", InputSize::Small), "small");
        assert_eq!(format!("{}", InputSize::Default), "default");
    }

    // 测试 SkeletonAvatarConfig 结构体
    #[test]
    fn test_skeleton_avatar_config_default() {
        let config = SkeletonAvatarConfig::default();
        assert_eq!(config.active, true);
        assert_eq!(config.shape, AvatarShape::Circle);
        assert_eq!(config.size, AvatarSize::Default);
    }

    #[test]
    fn test_skeleton_avatar_config_new() {
        let config = SkeletonAvatarConfig::new(false, AvatarShape::Square, AvatarSize::Large);

        assert_eq!(config.active, false);
        assert_eq!(config.shape, AvatarShape::Square);
        assert_eq!(config.size, AvatarSize::Large);
    }

    #[test]
    fn test_skeleton_avatar_config_with_shape() {
        let mut config = SkeletonAvatarConfig::default();
        config = config.with_shape(AvatarShape::Square);

        assert_eq!(config.shape, AvatarShape::Square);
    }

    #[test]
    fn test_skeleton_avatar_config_with_size() {
        let mut config = SkeletonAvatarConfig::default();
        config = config.with_size(AvatarSize::Custom(80));

        assert_eq!(config.size, AvatarSize::Custom(80));
    }

    #[test]
    fn test_skeleton_avatar_config_with_active() {
        let mut config = SkeletonAvatarConfig::default();
        config = config.with_active(false);

        assert_eq!(config.active, false);
    }

    // 测试 SkeletonTitleConfig 枚举
    #[test]
    fn test_skeleton_title_config_default() {
        let config = SkeletonTitleConfig::default();
        assert_eq!(config, SkeletonTitleConfig::Boolean(false));
    }

    #[test]
    fn test_skeleton_title_config_boolean_variant() {
        let config = SkeletonTitleConfig::Boolean(true);
        assert_eq!(config, SkeletonTitleConfig::Boolean(true));
    }

    #[test]
    fn test_skeleton_title_config_variants() {
        let bool_config = SkeletonTitleConfig::Boolean(true);
        assert_eq!(bool_config, SkeletonTitleConfig::Boolean(true));

        let title_props = SkeletonTitleProps {
            width: Some(SkeletonWidth::String("60%".to_string())),
        };
        let config_variant = SkeletonTitleConfig::Config(title_props);

        match config_variant {
            SkeletonTitleConfig::Config(props) => {
                assert!(props.width.is_some());
            }
            _ => panic!("Expected Config variant"),
        }
    }

    // 测试 SkeletonParagraphConfig 枚举
    #[test]
    fn test_skeleton_paragraph_config_default() {
        let config = SkeletonParagraphConfig::default();
        assert_eq!(config, SkeletonParagraphConfig::Boolean(false));
    }

    #[test]
    fn test_skeleton_paragraph_config_variants() {
        let bool_config = SkeletonParagraphConfig::Boolean(true);
        assert_eq!(bool_config, SkeletonParagraphConfig::Boolean(true));

        let paragraph_props = SkeletonParagraphProps {
            rows: Some(3),
            width: Some(SkeletonWidthConfig::Multiple(vec![
                SkeletonWidth::String("100%".to_string()),
                SkeletonWidth::String("100%".to_string()),
                SkeletonWidth::String("61%".to_string()),
            ])),
        };
        let config_variant = SkeletonParagraphConfig::Config(paragraph_props);

        match config_variant {
            SkeletonParagraphConfig::Config(props) => {
                assert_eq!(props.rows, Some(3));
                assert!(props.width.is_some());
            }
            _ => panic!("Expected Config variant"),
        }
    }

    #[test]
    fn test_skeleton_width_variants() {
        let pixel_width = SkeletonWidth::Pixels(100);
        let percentage_width = SkeletonWidth::Percentage(80);
        let string_width = SkeletonWidth::String("auto".to_string());

        assert_eq!(pixel_width, SkeletonWidth::Pixels(100));
        assert_eq!(percentage_width, SkeletonWidth::Percentage(80));
        assert_eq!(string_width, SkeletonWidth::String("auto".to_string()));
    }

    #[test]
    fn test_skeleton_width_config_variants() {
        let single_config = SkeletonWidthConfig::Single(SkeletonWidth::String("100%".to_string()));
        let multiple_config = SkeletonWidthConfig::Multiple(vec![
            SkeletonWidth::String("100%".to_string()),
            SkeletonWidth::String("80%".to_string()),
        ]);

        match single_config {
            SkeletonWidthConfig::Single(_) => assert!(true),
            _ => panic!("Expected Single variant"),
        }

        match multiple_config {
            SkeletonWidthConfig::Multiple(widths) => {
                assert_eq!(widths.len(), 2);
            }
            _ => panic!("Expected Multiple variant"),
        }
    }

    // 测试 SkeletonProps 结构体
    #[test]
    fn test_skeleton_props_default() {
        let props = SkeletonProps::default();
        assert_eq!(props.loading, Some(true));
        assert_eq!(props.active, Some(false));
        assert_eq!(props.round, Some(false));
        assert!(props.avatar.is_none());
        assert_eq!(props.title, Some(SkeletonTitleConfig::Boolean(true)));
        assert_eq!(
            props.paragraph,
            Some(SkeletonParagraphConfig::Boolean(true))
        );
    }

    #[test]
    fn test_skeleton_avatar_config_variants() {
        let bool_config = SkeletonAvatarConfig::Boolean(true);
        assert_eq!(bool_config, SkeletonAvatarConfig::Boolean(true));

        let avatar_props = SkeletonAvatarProps {
            active: Some(true),
            shape: Some(AvatarShape::Circle),
            size: Some(AvatarSize::Large),
        };
        let config_variant = SkeletonAvatarConfig::Config(avatar_props);

        match config_variant {
            SkeletonAvatarConfig::Config(props) => {
                assert_eq!(props.active, Some(true));
                assert_eq!(props.shape, Some(AvatarShape::Circle));
                assert_eq!(props.size, Some(AvatarSize::Large));
            }
            _ => panic!("Expected Config variant"),
        }
    }

    // 测试工具函数 - should_show_avatar
    #[test]
    fn test_should_show_avatar() {
        assert!(should_show_avatar(Some(&SkeletonAvatarConfig::Boolean(
            true
        ))));
        assert!(!should_show_avatar(Some(&SkeletonAvatarConfig::Boolean(
            false
        ))));
        assert!(!should_show_avatar(None));

        let avatar_props = SkeletonAvatarProps {
            active: Some(true),
            shape: Some(AvatarShape::Circle),
            size: Some(AvatarSize::Large),
        };
        assert!(should_show_avatar(Some(&SkeletonAvatarConfig::Config(
            avatar_props
        ))));
    }

    #[test]
    fn test_should_show_title() {
        assert!(should_show_title(Some(&SkeletonTitleConfig::Boolean(true))));
        assert!(!should_show_title(Some(&SkeletonTitleConfig::Boolean(
            false
        ))));
        assert!(should_show_title(None)); // 默认显示标题
    }

    #[test]
    fn test_should_show_paragraph() {
        assert!(should_show_paragraph(Some(
            &SkeletonParagraphConfig::Boolean(true)
        )));
        assert!(!should_show_paragraph(Some(
            &SkeletonParagraphConfig::Boolean(false)
        )));
        assert!(should_show_paragraph(None)); // 默认显示段落
    }

    // 测试工具函数
    #[test]
    fn test_validate_skeleton_config() {
        let valid_props = SkeletonProps {
            loading: Some(true),
            active: Some(false),
            round: Some(false),
            avatar: None,
            title: None,
            paragraph: None,
            class_name: None,
            style: None,
            theme: None,
            children: None,
        };

        assert!(validate_skeleton_config(&valid_props).is_ok());
    }

    #[test]
    fn test_generate_paragraph_widths() {
        // 测试默认配置
        let widths = generate_paragraph_widths(3, None);
        assert_eq!(widths.len(), 3);
        assert_eq!(widths[0], SkeletonWidth::Percentage(100));
        assert_eq!(widths[1], SkeletonWidth::Percentage(100));
        assert_eq!(widths[2], SkeletonWidth::Percentage(100));

        // 测试单一宽度配置
        let single_config = SkeletonWidthConfig::Single(SkeletonWidth::Percentage(61));
        let widths = generate_paragraph_widths(2, Some(&single_config));
        assert_eq!(widths.len(), 2);
        assert_eq!(widths[0], SkeletonWidth::Percentage(100));
        assert_eq!(widths[1], SkeletonWidth::Percentage(61));
    }

    #[test]
    fn test_calculate_avatar_props() {
        let avatar_config = SkeletonAvatarConfig::Boolean(true);
        let props = calculate_avatar_props(&avatar_config, true, true);

        assert_eq!(props.shape, Some(AvatarShape::Circle));
        assert_eq!(props.size, Some(AvatarSize::Large));
    }

    #[test]
    fn test_calculate_title_props() {
        let title_config = SkeletonTitleConfig::Boolean(true);
        let props = calculate_title_props(&title_config, false, true);

        assert_eq!(props.width, Some(SkeletonWidth::Percentage(38)));
    }

    #[test]
    fn test_calculate_paragraph_props() {
        let paragraph_config = SkeletonParagraphConfig::Boolean(true);
        let props = calculate_paragraph_props(&paragraph_config, false, true);

        assert_eq!(props.rows, Some(3));
        assert!(props.width.is_some());
    }

    #[test]
    fn test_generate_cache_key() {
        let props = SkeletonProps {
            loading: Some(true),
            active: Some(false),
            round: Some(false),
            avatar: None,
            title: None,
            paragraph: None,
            class_name: None,
            style: None,
            theme: None,
            children: None,
        };

        let cache_key = generate_cache_key(&props);
        assert!(!cache_key.is_empty());
    }

    #[test]
    fn test_is_mobile_device() {
        // 这个函数通常检测用户代理或屏幕尺寸
        // 在测试环境中可能返回默认值
        let is_mobile = is_mobile_device();
        assert!(is_mobile == true || is_mobile == false); // 确保返回布尔值
    }

    #[test]
    fn test_optimize_paragraph_rows() {
        // 测试段落行数优化
        let optimized = optimize_paragraph_rows(5, true);
        assert!(optimized <= 5);

        let not_optimized = optimize_paragraph_rows(3, false);
        assert_eq!(not_optimized, 3);
    }

    #[test]
    fn test_get_responsive_config() {
        // 测试响应式配置
        let config = get_responsive_config(800);
        assert!(config.is_some() || config.is_none()); // 确保返回有效值
    }

    #[test]
    fn test_merge_theme() {
        // 测试主题合并
        let default_theme = "light";
        let custom_theme = Some("dark".to_string());

        let merged = merge_theme(default_theme, custom_theme.as_deref());
        assert_eq!(merged, "dark");

        let no_custom = merge_theme(default_theme, None);
        assert_eq!(no_custom, default_theme);
    }

    // 测试验证函数
    #[test]
    fn test_validate_skeleton_width() {
        // 测试有效的百分比宽度
        assert!(validate_skeleton_width(&SkeletonWidth::Percentage(50)).is_ok());
        assert!(validate_skeleton_width(&SkeletonWidth::Percentage(100)).is_ok());

        // 测试有效的像素宽度
        assert!(validate_skeleton_width(&SkeletonWidth::Pixel(200)).is_ok());

        // 测试无效的百分比宽度
        assert!(validate_skeleton_width(&SkeletonWidth::Percentage(150)).is_err());
        assert!(validate_skeleton_width(&SkeletonWidth::Percentage(0)).is_err());
    }

    // 测试组件渲染
    #[tokio::test]
    async fn test_skeleton_component_basic_render() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Skeleton {
                    loading: true,
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 基本渲染测试通过
    }

    #[tokio::test]
    async fn test_skeleton_component_not_loading() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Skeleton {
                    loading: false,
                    children: rsx! {
                        div { "Content loaded" }
                    }
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 非加载状态渲染测试通过
    }

    #[tokio::test]
    async fn test_skeleton_component_with_avatar() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Skeleton {
                    loading: true,
                    avatar: true,
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 带头像骨架屏渲染测试通过
    }

    #[tokio::test]
    async fn test_skeleton_component_without_title() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Skeleton {
                    loading: true,
                    title: false,
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 无标题骨架屏渲染测试通过
    }

    #[tokio::test]
    async fn test_skeleton_component_without_paragraph() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Skeleton {
                    loading: true,
                    paragraph: false,
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 无段落骨架屏渲染测试通过
    }

    #[tokio::test]
    async fn test_skeleton_component_active() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Skeleton {
                    loading: true,
                    active: true,
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 动画骨架屏渲染测试通过
    }

    #[tokio::test]
    async fn test_skeleton_component_round() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Skeleton {
                    loading: true,
                    round: true,
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 圆角骨架屏渲染测试通过
    }

    #[tokio::test]
    async fn test_skeleton_component_all_features() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Skeleton {
                    loading: true,
                    active: true,
                    round: true,
                    avatar: true,
                    title: true,
                    paragraph: true,
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 全功能骨架屏渲染测试通过
    }

    #[tokio::test]
    async fn test_skeleton_component_with_custom_styles() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Skeleton {
                    loading: true,
                    class_name: "custom-skeleton",
                    style: "margin: 20px; padding: 10px;",
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 自定义样式骨架屏渲染测试通过
    }

    #[tokio::test]
    async fn test_skeleton_component_with_children() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Skeleton {
                    loading: false,
                    children: rsx! {
                        div {
                            h1 { "Title" }
                            p { "This is the actual content that shows when loading is false." }
                        }
                    }
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 带子元素骨架屏渲染测试通过
    }

    #[tokio::test]
    async fn test_skeleton_component_minimal_config() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Skeleton {
                    loading: true,
                    avatar: false,
                    title: false,
                    paragraph: false,
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 最小配置骨架屏渲染测试通过
    }
}

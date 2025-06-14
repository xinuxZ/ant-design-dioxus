#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::skeleton::types::*;
    use crate::components::skeleton::utils::*;
    use crate::components::skeleton::styles::*;
    use dioxus::prelude::*;
    use dioxus_testing::*;
    use std::collections::HashMap;

    // 测试 SkeletonAvatarShape 枚举
    #[test]
    fn test_skeleton_avatar_shape_default() {
        assert_eq!(SkeletonAvatarShape::default(), SkeletonAvatarShape::Circle);
    }

    #[test]
    fn test_skeleton_avatar_shape_display() {
        assert_eq!(format!("{}", SkeletonAvatarShape::Circle), "circle");
        assert_eq!(format!("{}", SkeletonAvatarShape::Square), "square");
    }

    #[test]
    fn test_skeleton_avatar_shape_from_str() {
        assert_eq!(SkeletonAvatarShape::from_str("circle"), SkeletonAvatarShape::Circle);
        assert_eq!(SkeletonAvatarShape::from_str("square"), SkeletonAvatarShape::Square);
        assert_eq!(SkeletonAvatarShape::from_str("unknown"), SkeletonAvatarShape::Circle);
    }

    #[test]
    fn test_skeleton_avatar_shape_to_string() {
        assert_eq!(SkeletonAvatarShape::Circle.to_string(), "circle");
        assert_eq!(SkeletonAvatarShape::Square.to_string(), "square");
    }

    #[test]
    fn test_skeleton_avatar_shape_is_circle() {
        assert!(SkeletonAvatarShape::Circle.is_circle());
        assert!(!SkeletonAvatarShape::Square.is_circle());
    }

    #[test]
    fn test_skeleton_avatar_shape_is_square() {
        assert!(!SkeletonAvatarShape::Circle.is_square());
        assert!(SkeletonAvatarShape::Square.is_square());
    }

    // 测试 SkeletonAvatarSize 枚举
    #[test]
    fn test_skeleton_avatar_size_default() {
        assert_eq!(SkeletonAvatarSize::default(), SkeletonAvatarSize::Default);
    }

    #[test]
    fn test_skeleton_avatar_size_display() {
        assert_eq!(format!("{}", SkeletonAvatarSize::Large), "large");
        assert_eq!(format!("{}", SkeletonAvatarSize::Small), "small");
        assert_eq!(format!("{}", SkeletonAvatarSize::Default), "default");
        assert_eq!(format!("{}", SkeletonAvatarSize::Custom(60)), "60");
    }

    #[test]
    fn test_skeleton_avatar_size_to_pixels() {
        assert_eq!(SkeletonAvatarSize::Large.to_pixels(), 64);
        assert_eq!(SkeletonAvatarSize::Small.to_pixels(), 24);
        assert_eq!(SkeletonAvatarSize::Default.to_pixels(), 40);
        assert_eq!(SkeletonAvatarSize::Custom(80).to_pixels(), 80);
    }

    #[test]
    fn test_skeleton_avatar_size_from_pixels() {
        assert_eq!(SkeletonAvatarSize::from_pixels(64), SkeletonAvatarSize::Large);
        assert_eq!(SkeletonAvatarSize::from_pixels(24), SkeletonAvatarSize::Small);
        assert_eq!(SkeletonAvatarSize::from_pixels(40), SkeletonAvatarSize::Default);
        assert_eq!(SkeletonAvatarSize::from_pixels(100), SkeletonAvatarSize::Custom(100));
    }

    #[test]
    fn test_skeleton_avatar_size_to_css_value() {
        assert_eq!(SkeletonAvatarSize::Large.to_css_value(), "64px");
        assert_eq!(SkeletonAvatarSize::Small.to_css_value(), "24px");
        assert_eq!(SkeletonAvatarSize::Default.to_css_value(), "40px");
        assert_eq!(SkeletonAvatarSize::Custom(50).to_css_value(), "50px");
    }

    // 测试 SkeletonButtonShape 枚举
    #[test]
    fn test_skeleton_button_shape_default() {
        assert_eq!(SkeletonButtonShape::default(), SkeletonButtonShape::Default);
    }

    #[test]
    fn test_skeleton_button_shape_display() {
        assert_eq!(format!("{}", SkeletonButtonShape::Circle), "circle");
        assert_eq!(format!("{}", SkeletonButtonShape::Round), "round");
        assert_eq!(format!("{}", SkeletonButtonShape::Default), "default");
    }

    #[test]
    fn test_skeleton_button_shape_from_str() {
        assert_eq!(SkeletonButtonShape::from_str("circle"), SkeletonButtonShape::Circle);
        assert_eq!(SkeletonButtonShape::from_str("round"), SkeletonButtonShape::Round);
        assert_eq!(SkeletonButtonShape::from_str("default"), SkeletonButtonShape::Default);
        assert_eq!(SkeletonButtonShape::from_str("unknown"), SkeletonButtonShape::Default);
    }

    #[test]
    fn test_skeleton_button_shape_to_string() {
        assert_eq!(SkeletonButtonShape::Circle.to_string(), "circle");
        assert_eq!(SkeletonButtonShape::Round.to_string(), "round");
        assert_eq!(SkeletonButtonShape::Default.to_string(), "default");
    }

    #[test]
    fn test_skeleton_button_shape_is_circle() {
        assert!(SkeletonButtonShape::Circle.is_circle());
        assert!(!SkeletonButtonShape::Round.is_circle());
        assert!(!SkeletonButtonShape::Default.is_circle());
    }

    #[test]
    fn test_skeleton_button_shape_is_round() {
        assert!(!SkeletonButtonShape::Circle.is_round());
        assert!(SkeletonButtonShape::Round.is_round());
        assert!(!SkeletonButtonShape::Default.is_round());
    }

    // 测试 SkeletonButtonSize 枚举
    #[test]
    fn test_skeleton_button_size_default() {
        assert_eq!(SkeletonButtonSize::default(), SkeletonButtonSize::Default);
    }

    #[test]
    fn test_skeleton_button_size_display() {
        assert_eq!(format!("{}", SkeletonButtonSize::Large), "large");
        assert_eq!(format!("{}", SkeletonButtonSize::Small), "small");
        assert_eq!(format!("{}", SkeletonButtonSize::Default), "default");
    }

    #[test]
    fn test_skeleton_button_size_to_height() {
        assert_eq!(SkeletonButtonSize::Large.to_height(), 40);
        assert_eq!(SkeletonButtonSize::Small.to_height(), 24);
        assert_eq!(SkeletonButtonSize::Default.to_height(), 32);
    }

    #[test]
    fn test_skeleton_button_size_to_css_height() {
        assert_eq!(SkeletonButtonSize::Large.to_css_height(), "40px");
        assert_eq!(SkeletonButtonSize::Small.to_css_height(), "24px");
        assert_eq!(SkeletonButtonSize::Default.to_css_height(), "32px");
    }

    // 测试 SkeletonInputSize 枚举
    #[test]
    fn test_skeleton_input_size_default() {
        assert_eq!(SkeletonInputSize::default(), SkeletonInputSize::Default);
    }

    #[test]
    fn test_skeleton_input_size_display() {
        assert_eq!(format!("{}", SkeletonInputSize::Large), "large");
        assert_eq!(format!("{}", SkeletonInputSize::Small), "small");
        assert_eq!(format!("{}", SkeletonInputSize::Default), "default");
    }

    #[test]
    fn test_skeleton_input_size_to_height() {
        assert_eq!(SkeletonInputSize::Large.to_height(), 40);
        assert_eq!(SkeletonInputSize::Small.to_height(), 24);
        assert_eq!(SkeletonInputSize::Default.to_height(), 32);
    }

    #[test]
    fn test_skeleton_input_size_to_css_height() {
        assert_eq!(SkeletonInputSize::Large.to_css_height(), "40px");
        assert_eq!(SkeletonInputSize::Small.to_css_height(), "24px");
        assert_eq!(SkeletonInputSize::Default.to_css_height(), "32px");
    }

    // 测试 SkeletonAvatarConfig 结构体
    #[test]
    fn test_skeleton_avatar_config_default() {
        let config = SkeletonAvatarConfig::default();
        assert_eq!(config.active, true);
        assert_eq!(config.shape, SkeletonAvatarShape::Circle);
        assert_eq!(config.size, SkeletonAvatarSize::Default);
    }

    #[test]
    fn test_skeleton_avatar_config_new() {
        let config = SkeletonAvatarConfig::new(
            false,
            SkeletonAvatarShape::Square,
            SkeletonAvatarSize::Large,
        );
        
        assert_eq!(config.active, false);
        assert_eq!(config.shape, SkeletonAvatarShape::Square);
        assert_eq!(config.size, SkeletonAvatarSize::Large);
    }

    #[test]
    fn test_skeleton_avatar_config_with_shape() {
        let mut config = SkeletonAvatarConfig::default();
        config = config.with_shape(SkeletonAvatarShape::Square);
        
        assert_eq!(config.shape, SkeletonAvatarShape::Square);
    }

    #[test]
    fn test_skeleton_avatar_config_with_size() {
        let mut config = SkeletonAvatarConfig::default();
        config = config.with_size(SkeletonAvatarSize::Custom(80));
        
        assert_eq!(config.size, SkeletonAvatarSize::Custom(80));
    }

    #[test]
    fn test_skeleton_avatar_config_with_active() {
        let mut config = SkeletonAvatarConfig::default();
        config = config.with_active(false);
        
        assert_eq!(config.active, false);
    }

    // 测试 SkeletonTitleConfig 结构体
    #[test]
    fn test_skeleton_title_config_default() {
        let config = SkeletonTitleConfig::default();
        assert_eq!(config.width, "38%");
    }

    #[test]
    fn test_skeleton_title_config_new() {
        let config = SkeletonTitleConfig::new("50%".to_string());
        assert_eq!(config.width, "50%");
    }

    #[test]
    fn test_skeleton_title_config_with_width() {
        let mut config = SkeletonTitleConfig::default();
        config = config.with_width("60%".to_string());
        
        assert_eq!(config.width, "60%");
    }

    #[test]
    fn test_skeleton_title_config_is_valid_width() {
        assert!(SkeletonTitleConfig::is_valid_width("50%"));
        assert!(SkeletonTitleConfig::is_valid_width("100px"));
        assert!(SkeletonTitleConfig::is_valid_width("auto"));
        assert!(!SkeletonTitleConfig::is_valid_width("invalid"));
        assert!(!SkeletonTitleConfig::is_valid_width(""));
    }

    // 测试 SkeletonParagraphConfig 结构体
    #[test]
    fn test_skeleton_paragraph_config_default() {
        let config = SkeletonParagraphConfig::default();
        assert_eq!(config.rows, 3);
        assert_eq!(config.width, vec!["100%".to_string(), "100%".to_string(), "61%".to_string()]);
    }

    #[test]
    fn test_skeleton_paragraph_config_new() {
        let widths = vec!["80%".to_string(), "90%".to_string()];
        let config = SkeletonParagraphConfig::new(2, widths.clone());
        
        assert_eq!(config.rows, 2);
        assert_eq!(config.width, widths);
    }

    #[test]
    fn test_skeleton_paragraph_config_with_rows() {
        let mut config = SkeletonParagraphConfig::default();
        config = config.with_rows(5);
        
        assert_eq!(config.rows, 5);
        assert_eq!(config.width.len(), 5);
    }

    #[test]
    fn test_skeleton_paragraph_config_with_width() {
        let mut config = SkeletonParagraphConfig::default();
        let new_widths = vec!["70%".to_string(), "80%".to_string(), "50%".to_string()];
        config = config.with_width(new_widths.clone());
        
        assert_eq!(config.width, new_widths);
    }

    #[test]
    fn test_skeleton_paragraph_config_generate_default_widths() {
        let widths = SkeletonParagraphConfig::generate_default_widths(4);
        assert_eq!(widths.len(), 4);
        assert_eq!(widths[0], "100%");
        assert_eq!(widths[1], "100%");
        assert_eq!(widths[2], "100%");
        assert_eq!(widths[3], "61%");
    }

    #[test]
    fn test_skeleton_paragraph_config_is_valid_rows() {
        assert!(SkeletonParagraphConfig::is_valid_rows(1));
        assert!(SkeletonParagraphConfig::is_valid_rows(10));
        assert!(!SkeletonParagraphConfig::is_valid_rows(0));
        assert!(!SkeletonParagraphConfig::is_valid_rows(21));
    }

    // 测试 SkeletonConfig 结构体
    #[test]
    fn test_skeleton_config_default() {
        let config = SkeletonConfig::default();
        assert_eq!(config.loading, true);
        assert_eq!(config.active, false);
        assert_eq!(config.round, false);
        assert!(config.avatar.is_none());
        assert!(config.title.is_some());
        assert!(config.paragraph.is_some());
    }

    #[test]
    fn test_skeleton_config_new() {
        let avatar_config = SkeletonAvatarConfig::default();
        let title_config = SkeletonTitleConfig::default();
        let paragraph_config = SkeletonParagraphConfig::default();
        
        let config = SkeletonConfig::new(
            false,
            true,
            true,
            Some(avatar_config.clone()),
            Some(title_config.clone()),
            Some(paragraph_config.clone()),
        );
        
        assert_eq!(config.loading, false);
        assert_eq!(config.active, true);
        assert_eq!(config.round, true);
        assert_eq!(config.avatar, Some(avatar_config));
        assert_eq!(config.title, Some(title_config));
        assert_eq!(config.paragraph, Some(paragraph_config));
    }

    #[test]
    fn test_skeleton_config_with_loading() {
        let mut config = SkeletonConfig::default();
        config = config.with_loading(false);
        
        assert_eq!(config.loading, false);
    }

    #[test]
    fn test_skeleton_config_with_active() {
        let mut config = SkeletonConfig::default();
        config = config.with_active(true);
        
        assert_eq!(config.active, true);
    }

    #[test]
    fn test_skeleton_config_with_round() {
        let mut config = SkeletonConfig::default();
        config = config.with_round(true);
        
        assert_eq!(config.round, true);
    }

    #[test]
    fn test_skeleton_config_with_avatar() {
        let mut config = SkeletonConfig::default();
        let avatar_config = SkeletonAvatarConfig::default();
        config = config.with_avatar(Some(avatar_config.clone()));
        
        assert_eq!(config.avatar, Some(avatar_config));
    }

    #[test]
    fn test_skeleton_config_should_show_skeleton() {
        let loading_config = SkeletonConfig {
            loading: true,
            ..Default::default()
        };
        assert!(loading_config.should_show_skeleton());
        
        let not_loading_config = SkeletonConfig {
            loading: false,
            ..Default::default()
        };
        assert!(!not_loading_config.should_show_skeleton());
    }

    #[test]
    fn test_skeleton_config_has_avatar() {
        let with_avatar = SkeletonConfig {
            avatar: Some(SkeletonAvatarConfig::default()),
            ..Default::default()
        };
        assert!(with_avatar.has_avatar());
        
        let without_avatar = SkeletonConfig {
            avatar: None,
            ..Default::default()
        };
        assert!(!without_avatar.has_avatar());
    }

    #[test]
    fn test_skeleton_config_has_title() {
        let with_title = SkeletonConfig {
            title: Some(SkeletonTitleConfig::default()),
            ..Default::default()
        };
        assert!(with_title.has_title());
        
        let without_title = SkeletonConfig {
            title: None,
            ..Default::default()
        };
        assert!(!without_title.has_title());
    }

    #[test]
    fn test_skeleton_config_has_paragraph() {
        let with_paragraph = SkeletonConfig {
            paragraph: Some(SkeletonParagraphConfig::default()),
            ..Default::default()
        };
        assert!(with_paragraph.has_paragraph());
        
        let without_paragraph = SkeletonConfig {
            paragraph: None,
            ..Default::default()
        };
        assert!(!without_paragraph.has_paragraph());
    }

    // 测试工具函数
    #[test]
    fn test_validate_skeleton_props() {
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
        
        assert!(validate_skeleton_props(&valid_props).is_ok());
    }

    #[test]
    fn test_create_skeleton_config() {
        let props = SkeletonProps {
            loading: Some(false),
            active: Some(true),
            round: Some(true),
            avatar: Some(true),
            title: Some(false),
            paragraph: Some(true),
            class_name: None,
            style: None,
            theme: None,
            children: None,
        };
        
        let config = create_skeleton_config(&props);
        assert_eq!(config.loading, false);
        assert_eq!(config.active, true);
        assert_eq!(config.round, true);
        assert!(config.avatar.is_some());
        assert!(config.title.is_none());
        assert!(config.paragraph.is_some());
    }

    #[test]
    fn test_create_avatar_config_from_bool() {
        let config = create_avatar_config_from_bool(true);
        assert!(config.is_some());
        assert_eq!(config.unwrap(), SkeletonAvatarConfig::default());
        
        let no_config = create_avatar_config_from_bool(false);
        assert!(no_config.is_none());
    }

    #[test]
    fn test_create_title_config_from_bool() {
        let config = create_title_config_from_bool(true);
        assert!(config.is_some());
        assert_eq!(config.unwrap(), SkeletonTitleConfig::default());
        
        let no_config = create_title_config_from_bool(false);
        assert!(no_config.is_none());
    }

    #[test]
    fn test_create_paragraph_config_from_bool() {
        let config = create_paragraph_config_from_bool(true);
        assert!(config.is_some());
        assert_eq!(config.unwrap(), SkeletonParagraphConfig::default());
        
        let no_config = create_paragraph_config_from_bool(false);
        assert!(no_config.is_none());
    }

    #[test]
    fn test_get_skeleton_container_class() {
        let config = SkeletonConfig {
            active: true,
            round: true,
            ..Default::default()
        };
        
        let class_name = get_skeleton_container_class(&config);
        assert!(class_name.contains("ant-skeleton"));
        assert!(class_name.contains("ant-skeleton-active"));
        assert!(class_name.contains("ant-skeleton-round"));
    }

    #[test]
    fn test_get_skeleton_container_class_inactive() {
        let config = SkeletonConfig {
            active: false,
            round: false,
            ..Default::default()
        };
        
        let class_name = get_skeleton_container_class(&config);
        assert!(class_name.contains("ant-skeleton"));
        assert!(!class_name.contains("ant-skeleton-active"));
        assert!(!class_name.contains("ant-skeleton-round"));
    }

    #[test]
    fn test_get_skeleton_avatar_class() {
        let config = SkeletonAvatarConfig {
            shape: SkeletonAvatarShape::Square,
            size: SkeletonAvatarSize::Large,
            ..Default::default()
        };
        
        let class_name = get_skeleton_avatar_class(&config);
        assert!(class_name.contains("ant-skeleton-avatar"));
        assert!(class_name.contains("ant-skeleton-avatar-square"));
        assert!(class_name.contains("ant-skeleton-avatar-lg"));
    }

    #[test]
    fn test_get_skeleton_avatar_styles() {
        let config = SkeletonAvatarConfig {
            size: SkeletonAvatarSize::Custom(80),
            ..Default::default()
        };
        
        let styles = get_skeleton_avatar_styles(&config);
        assert!(styles.contains("width: 80px"));
        assert!(styles.contains("height: 80px"));
    }

    #[test]
    fn test_get_skeleton_title_styles() {
        let config = SkeletonTitleConfig {
            width: "60%".to_string(),
        };
        
        let styles = get_skeleton_title_styles(&config);
        assert!(styles.contains("width: 60%"));
    }

    #[test]
    fn test_get_skeleton_paragraph_row_styles() {
        let styles = get_skeleton_paragraph_row_styles("75%");
        assert!(styles.contains("width: 75%"));
    }

    #[test]
    fn test_merge_skeleton_styles() {
        let base_styles = "color: red;";
        let custom_styles = Some("margin: 10px;".to_string());
        
        let merged = merge_skeleton_styles(base_styles, custom_styles.as_deref());
        assert!(merged.contains("color: red;"));
        assert!(merged.contains("margin: 10px;"));
        
        let no_custom = merge_skeleton_styles(base_styles, None);
        assert_eq!(no_custom, base_styles);
    }

    #[test]
    fn test_merge_skeleton_classes() {
        let base_class = "ant-skeleton";
        let custom_class = Some("custom-skeleton".to_string());
        
        let merged = merge_skeleton_classes(base_class, custom_class.as_deref());
        assert!(merged.contains("ant-skeleton"));
        assert!(merged.contains("custom-skeleton"));
        
        let no_custom = merge_skeleton_classes(base_class, None);
        assert_eq!(no_custom, base_class);
    }

    // 测试样式生成
    #[test]
    fn test_skeleton_styles_base() {
        let base_styles = SkeletonStyles::base();
        assert!(base_styles.contains(".ant-skeleton"));
        assert!(base_styles.contains("position: relative"));
    }

    #[test]
    fn test_skeleton_styles_avatar_styles() {
        let avatar_styles = SkeletonStyles::avatar_styles();
        assert!(avatar_styles.contains(".ant-skeleton-avatar"));
        assert!(avatar_styles.contains("display: inline-block"));
        assert!(avatar_styles.contains("border-radius: 50%"));
    }

    #[test]
    fn test_skeleton_styles_title_styles() {
        let title_styles = SkeletonStyles::title_styles();
        assert!(title_styles.contains(".ant-skeleton-title"));
        assert!(title_styles.contains("height: 16px"));
        assert!(title_styles.contains("margin-bottom: 8px"));
    }

    #[test]
    fn test_skeleton_styles_paragraph_styles() {
        let paragraph_styles = SkeletonStyles::paragraph_styles();
        assert!(paragraph_styles.contains(".ant-skeleton-paragraph"));
        assert!(paragraph_styles.contains(".ant-skeleton-paragraph li"));
        assert!(paragraph_styles.contains("height: 16px"));
        assert!(paragraph_styles.contains("margin-bottom: 8px"));
    }

    #[test]
    fn test_skeleton_styles_active_styles() {
        let active_styles = SkeletonStyles::active_styles();
        assert!(active_styles.contains(".ant-skeleton-active"));
        assert!(active_styles.contains("@keyframes ant-skeleton-loading"));
        assert!(active_styles.contains("animation"));
    }

    #[test]
    fn test_skeleton_styles_round_styles() {
        let round_styles = SkeletonStyles::round_styles();
        assert!(round_styles.contains(".ant-skeleton-round"));
        assert!(round_styles.contains("border-radius: 6px"));
    }

    #[test]
    fn test_skeleton_styles_size_styles() {
        let size_styles = SkeletonStyles::size_styles();
        assert!(size_styles.contains(".ant-skeleton-avatar-lg"));
        assert!(size_styles.contains(".ant-skeleton-avatar-sm"));
        assert!(size_styles.contains("width: 64px"));
        assert!(size_styles.contains("width: 24px"));
    }

    #[test]
    fn test_skeleton_styles_shape_styles() {
        let shape_styles = SkeletonStyles::shape_styles();
        assert!(shape_styles.contains(".ant-skeleton-avatar-square"));
        assert!(shape_styles.contains("border-radius: 6px"));
    }

    #[test]
    fn test_skeleton_styles_element_styles() {
        let element_styles = SkeletonStyles::element_styles();
        assert!(element_styles.contains(".ant-skeleton-element"));
        assert!(element_styles.contains("background: linear-gradient"));
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
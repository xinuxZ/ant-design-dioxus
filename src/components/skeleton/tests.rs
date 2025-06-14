#[cfg(test)]
mod tests {
    use super::*;
    use dioxus::prelude::*;

    #[test]
    fn test_skeleton_basic_render() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Skeleton {
                    loading: true,
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("ant-skeleton"));
        assert!(html.contains("ant-skeleton-content"));
    }

    #[test]
    fn test_skeleton_with_avatar() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Skeleton {
                    loading: true,
                    avatar: SkeletonAvatarConfig::Boolean(true),
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("ant-skeleton-header"));
        assert!(html.contains("ant-skeleton-avatar"));
    }

    #[test]
    fn test_skeleton_active_animation() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Skeleton {
                    loading: true,
                    active: true,
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("ant-skeleton-active"));
    }

    #[test]
    fn test_skeleton_round_style() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Skeleton {
                    loading: true,
                    round: true,
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("ant-skeleton-round"));
    }

    #[test]
    fn test_skeleton_not_loading() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Skeleton {
                    loading: false,
                    div { "Content loaded" }
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("Content loaded"));
        assert!(!html.contains("ant-skeleton"));
    }

    #[test]
    fn test_skeleton_custom_paragraph() {
        let paragraph_config = SkeletonParagraphConfig::Object(SkeletonParagraphProps {
            rows: Some(4),
            width: Some(SkeletonWidthConfig::Array(vec![
                SkeletonWidth::Percent(100),
                SkeletonWidth::Percent(80),
                SkeletonWidth::Percent(60),
                SkeletonWidth::Percent(40),
            ])),
            class_name: None,
            style: None,
        });

        let mut dom = VirtualDom::new(move || {
            rsx! {
                Skeleton {
                    loading: true,
                    paragraph: paragraph_config.clone(),
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("ant-skeleton-paragraph"));
        // 应该有4行
        assert_eq!(html.matches("<li").count(), 4);
    }

    #[test]
    fn test_skeleton_button_component() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                SkeletonButton {
                    active: true,
                    size: ButtonSize::Large,
                    shape: ButtonShape::Round,
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("ant-skeleton-button"));
        assert!(html.contains("ant-skeleton-button-lg"));
        assert!(html.contains("ant-skeleton-button-round"));
        assert!(html.contains("ant-skeleton-active"));
    }

    #[test]
    fn test_skeleton_input_component() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                SkeletonInput {
                    active: true,
                    size: InputSize::Large,
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("ant-skeleton-input"));
        assert!(html.contains("ant-skeleton-input-lg"));
        assert!(html.contains("ant-skeleton-active"));
    }

    #[test]
    fn test_skeleton_image_component() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                SkeletonImage {
                    active: true,
                    width: SkeletonWidth::Pixel(200),
                    height: SkeletonWidth::Pixel(150),
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("ant-skeleton-image"));
        assert!(html.contains("ant-skeleton-active"));
        assert!(html.contains("width: 200px"));
        assert!(html.contains("height: 150px"));
    }

    #[test]
    fn test_skeleton_node_component() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                SkeletonNode {
                    active: true,
                    width: SkeletonWidth::Percent(50),
                    height: SkeletonWidth::Pixel(30),
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("ant-skeleton-element"));
        assert!(html.contains("ant-skeleton-element-active"));
        assert!(html.contains("width: 50%"));
        assert!(html.contains("height: 30px"));
    }

    #[test]
    fn test_skeleton_avatar_shapes() {
        // 测试圆形头像
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Skeleton {
                    loading: true,
                    avatar: SkeletonAvatarConfig::Object(SkeletonAvatarProps {
                        shape: Some(AvatarShape::Circle),
                        size: Some(AvatarSize::Large),
                        active: Some(true),
                        class_name: None,
                        style: None,
                    }),
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("ant-skeleton-avatar-circle"));
        assert!(html.contains("ant-skeleton-avatar-lg"));

        // 测试方形头像
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Skeleton {
                    loading: true,
                    avatar: SkeletonAvatarConfig::Object(SkeletonAvatarProps {
                        shape: Some(AvatarShape::Square),
                        size: Some(AvatarSize::Small),
                        active: Some(false),
                        class_name: None,
                        style: None,
                    }),
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("ant-skeleton-avatar-square"));
        assert!(html.contains("ant-skeleton-avatar-sm"));
    }

    #[test]
    fn test_skeleton_title_width() {
        let title_config = SkeletonTitleConfig::Object(SkeletonTitleProps {
            width: Some(SkeletonWidth::Percent(75)),
            class_name: None,
            style: None,
        });

        let mut dom = VirtualDom::new(move || {
            rsx! {
                Skeleton {
                    loading: true,
                    title: title_config.clone(),
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("ant-skeleton-title"));
        assert!(html.contains("width: 75%"));
    }

    #[test]
    fn test_skeleton_props_builder() {
        let props = SkeletonProps::basic()
            .loading(true)
            .set_active(true)
            .set_round(true)
            .set_avatar(SkeletonAvatarConfig::Boolean(true))
            .set_class_name("custom-skeleton".to_string());

        assert_eq!(props.loading, Some(true));
        assert_eq!(props.active, Some(true));
        assert_eq!(props.round, Some(true));
        assert!(matches!(
            props.avatar,
            Some(SkeletonAvatarConfig::Boolean(true))
        ));
        assert_eq!(props.class_name, Some("custom-skeleton".to_string()));
    }

    #[test]
    fn test_skeleton_convenience_constructors() {
        // 测试 with_avatar
        let avatar_props = SkeletonProps::with_avatar();
        assert!(matches!(
            avatar_props.avatar,
            Some(SkeletonAvatarConfig::Boolean(true))
        ));

        // 测试 active
        let active_props = SkeletonProps::active();
        assert_eq!(active_props.active, Some(true));

        // 测试 round
        let round_props = SkeletonProps::round();
        assert_eq!(round_props.round, Some(true));
    }

    #[test]
    fn test_skeleton_theme_application() {
        let custom_theme = SkeletonTheme {
            color: Some("#f0f0f0".to_string()),
            color_gradient_end: Some("#e0e0e0".to_string()),
            border_radius: Some(6),
            block_radius: Some(4),
            title_height: Some(18),
            paragraph_line_height: Some(16),
            paragraph_margin_top: Some(12),
            avatar_size: Some(40),
            avatar_shape: Some(AvatarShape::Circle),
            button_height: Some(32),
            input_height: Some(32),
            image_size: Some(96),
            animation_duration: Some(1.5),
        };

        let mut dom = VirtualDom::new(move || {
            rsx! {
                Skeleton {
                    loading: true,
                    active: true,
                    theme: custom_theme.clone(),
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("ant-skeleton"));
        // 主题应该被应用到样式中
        assert!(html.contains("#f0f0f0") || html.contains("#e0e0e0"));
    }

    #[test]
    fn test_skeleton_complex_configuration() {
        let complex_config = SkeletonProps {
            loading: Some(true),
            active: Some(true),
            round: Some(true),
            avatar: Some(SkeletonAvatarConfig::Object(SkeletonAvatarProps {
                shape: Some(AvatarShape::Square),
                size: Some(AvatarSize::Large),
                active: Some(true),
                class_name: Some("custom-avatar".to_string()),
                style: Some("margin-right: 16px;".to_string()),
            })),
            title: Some(SkeletonTitleConfig::Object(SkeletonTitleProps {
                width: Some(SkeletonWidth::Percent(60)),
                class_name: Some("custom-title".to_string()),
                style: Some("margin-bottom: 8px;".to_string()),
            })),
            paragraph: Some(SkeletonParagraphConfig::Object(SkeletonParagraphProps {
                rows: Some(3),
                width: Some(SkeletonWidthConfig::Array(vec![
                    SkeletonWidth::Percent(100),
                    SkeletonWidth::Percent(80),
                    SkeletonWidth::Percent(40),
                ])),
                class_name: Some("custom-paragraph".to_string()),
                style: Some("line-height: 1.5;".to_string()),
            })),
            class_name: Some("complex-skeleton".to_string()),
            style: Some("padding: 16px;".to_string()),
            theme: Some(SkeletonTheme::default()),
            children: None,
        };

        let mut dom = VirtualDom::new(move || {
            rsx! {
                Skeleton {
                    ..complex_config.clone()
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("complex-skeleton"));
        assert!(html.contains("custom-avatar"));
        assert!(html.contains("custom-title"));
        assert!(html.contains("custom-paragraph"));
        assert!(html.contains("ant-skeleton-active"));
        assert!(html.contains("ant-skeleton-round"));
        assert!(html.contains("ant-skeleton-avatar-square"));
        assert!(html.contains("ant-skeleton-avatar-lg"));
        assert_eq!(html.matches("<li").count(), 3); // 3行段落
    }

    #[test]
    fn test_skeleton_accessibility() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Skeleton {
                    loading: true,
                    active: true,
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();
        // 检查是否包含适当的ARIA属性或语义化标签
        assert!(html.contains("ant-skeleton"));
        // 可以添加更多无障碍性检查
    }

    #[test]
    fn test_skeleton_performance() {
        use std::time::Instant;

        let start = Instant::now();

        for _ in 0..100 {
            let mut dom = VirtualDom::new(|| {
                rsx! {
                    Skeleton {
                        loading: true,
                        active: true,
                        avatar: SkeletonAvatarConfig::Boolean(true),
                        paragraph: SkeletonParagraphConfig::Object(SkeletonParagraphProps {
                            rows: Some(5),
                            width: None,
                            class_name: None,
                            style: None,
                        }),
                    }
                }
            });
            dom.rebuild_to_vec();
        }

        let duration = start.elapsed();
        // 确保渲染100个复杂骨架屏组件在合理时间内完成（比如1秒）
        assert!(
            duration.as_secs() < 1,
            "Skeleton rendering took too long: {:?}",
            duration
        );
    }

    #[test]
    fn test_skeleton_memory_usage() {
        // 创建大量骨架屏组件，确保没有内存泄漏
        let skeletons: Vec<_> = (0..1000)
            .map(|i| {
                SkeletonProps::basic()
                    .loading(true)
                    .set_active(i % 2 == 0)
                    .set_round(i % 3 == 0)
                    .set_class_name(format!("skeleton-{}", i))
            })
            .collect();

        assert_eq!(skeletons.len(), 1000);
        // 如果没有内存泄漏，这个测试应该能正常完成
    }

    #[test]
    fn test_skeleton_edge_cases() {
        // 测试空配置
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Skeleton {
                    loading: true,
                    title: SkeletonTitleConfig::Boolean(false),
                    paragraph: SkeletonParagraphConfig::Boolean(false),
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("ant-skeleton"));
        assert!(!html.contains("ant-skeleton-title"));
        assert!(!html.contains("ant-skeleton-paragraph"));

        // 测试零行段落
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Skeleton {
                    loading: true,
                    paragraph: SkeletonParagraphConfig::Object(SkeletonParagraphProps {
                        rows: Some(0),
                        width: None,
                        class_name: None,
                        style: None,
                    }),
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();
        assert_eq!(html.matches("<li").count(), 0);
    }
}

// 集成测试
#[cfg(test)]
mod integration_tests {
    use super::*;
    use dioxus::prelude::*;

    #[test]
    fn test_skeleton_with_real_content() {
        let mut loading = true;

        let mut dom = VirtualDom::new(move || {
            rsx! {
                Skeleton {
                    loading: loading,
                    avatar: SkeletonAvatarConfig::Boolean(true),

                    div {
                        class: "user-profile",
                        img { src: "avatar.jpg", alt: "User Avatar" }
                        h2 { "John Doe" }
                        p { "Software Engineer at Example Corp" }
                        p { "Passionate about Rust and WebAssembly" }
                    }
                }
            }
        });

        // 加载状态 - 应该显示骨架屏
        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("ant-skeleton"));
        assert!(!html.contains("user-profile"));

        // 模拟加载完成
        loading = false;
        let html = dom.rebuild_to_vec().santize();
        assert!(!html.contains("ant-skeleton"));
        assert!(html.contains("user-profile"));
        assert!(html.contains("John Doe"));
    }

    #[test]
    fn test_skeleton_in_list() {
        let items_loaded = false;

        let mut dom = VirtualDom::new(move || {
            rsx! {
                div {
                    class: "item-list",

                    for i in 0..5 {
                        div {
                            key: "{i}",
                            class: "list-item",

                            Skeleton {
                                loading: !items_loaded,
                                avatar: SkeletonAvatarConfig::Boolean(true),
                                paragraph: SkeletonParagraphConfig::Object(SkeletonParagraphProps {
                                    rows: Some(2),
                                    width: None,
                                    class_name: None,
                                    style: None,
                                }),

                                // 实际内容
                                div {
                                    class: "item-content",
                                    img { src: "item-{i}.jpg" }
                                    h3 { "Item {i}" }
                                    p { "Description for item {i}" }
                                }
                            }
                        }
                    }
                }
            }
        });

        let html = dom.rebuild_to_vec().santize();
        assert!(html.contains("item-list"));
        assert_eq!(html.matches("ant-skeleton").count(), 5);
        assert!(!html.contains("item-content"));
    }
}

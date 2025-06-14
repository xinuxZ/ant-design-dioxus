//! Icon 组件的测试用例

#[cfg(test)]
mod tests {
    use super::super::*;
    use dioxus::prelude::*;
    // use dioxus_testing::*;

    /// 测试基础Icon组件渲染
    #[test]
    fn test_basic_icon_render() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Icon {
                    icon_type: CommonIconType::Home,
                    theme: IconTheme::Outlined,
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 验证组件能够正常渲染
    }

    /// 测试Icon组件的不同主题
    #[test]
    fn test_icon_themes() {
        // 测试Outlined主题
        let mut dom_outlined = VirtualDom::new(|| {
            rsx! {
                Icon {
                    icon_type: CommonIconType::User,
                    theme: IconTheme::Outlined,
                }
            }
        });
        let _ = dom_outlined.rebuild_to_vec();

        // 测试Filled主题
        let mut dom_filled = VirtualDom::new(|| {
            rsx! {
                Icon {
                    icon_type: CommonIconType::User,
                    theme: IconTheme::Filled,
                }
            }
        });
        let _ = dom_filled.rebuild_to_vec();

        // 测试TwoTone主题
        let mut dom_two_tone = VirtualDom::new(|| {
            rsx! {
                Icon {
                    icon_type: CommonIconType::User,
                    theme: IconTheme::TwoTone,
                    two_tone_color: "#ff0000",
                }
            }
        });
        let _ = dom_two_tone.rebuild_to_vec();
    }

    /// 测试Icon组件的不同尺寸
    #[test]
    fn test_icon_sizes() {
        let sizes = vec![
            IconSize::Small,
            IconSize::Default,
            IconSize::Large,
            IconSize::Custom(24),
        ];
        for size in sizes {
            let mut dom = VirtualDom::new(move || {
                rsx! {
                    Icon {
                        icon_type: CommonIconType::Setting,
                        size: size.clone(),
                    }
                }
            });
            let _ = dom.rebuild_to_vec();
        }
    }

    /// 测试Icon组件的旋转功能
    #[test]
    fn test_icon_rotation() {
        // 测试静态旋转
        let mut dom_rotate = VirtualDom::new(|| {
            rsx! {
                Icon {
                    icon_type: CommonIconType::Setting,
                    rotate: 90,
                }
            }
        });
        let _ = dom_rotate.rebuild_to_vec();

        // 测试旋转动画
        let mut dom_spin = VirtualDom::new(|| {
            rsx! {
                Icon {
                    icon_type: CommonIconType::Loading,
                    spin: true,
                }
            }
        });
        let _ = dom_spin.rebuild_to_vec();
    }

    /// 测试Icon组件的禁用状态
    #[test]
    fn test_icon_disabled_state() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Icon {
                    icon_type: CommonIconType::Delete,
                    disabled: true,
                    on_click: |_| {},
                }
            }
        });
        let _ = dom.rebuild_to_vec();
    }

    /// 测试Icon组件的点击事件
    #[test]
    fn test_icon_click_event() {
        let clicked = use_signal(|| false);

        let mut dom = VirtualDom::new(move || {
            rsx! {
                Icon {
                    icon_type: CommonIconType::Edit,
                    on_click: move |_| {
                        clicked.set(true);
                    },
                }
            }
        });
        let _ = dom.rebuild_to_vec();
    }

    /// 测试自定义Icon组件
    #[test]
    fn test_custom_icon() {
        let custom_svg = r#"<svg viewBox="0 0 24 24"><path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z"/></svg>"#;

        let mut dom = VirtualDom::new(move || {
            create_icon(
                custom_svg,
                IconProps {
                    theme: Some(IconTheme::Filled),
                    size: Some(IconSize::Large),
                    ..Default::default()
                },
            )
        });
        let _ = dom.rebuild_to_vec();
    }

    /// 测试IconFont组件
    #[test]
    fn test_iconfont_icon() {
        let mut dom = VirtualDom::new(|| {
            create_iconfont_icon(
                "icon-star",
                IconProps {
                    size: Some(IconSize::Large),
                    ..Default::default()
                },
            )
        });
        let _ = dom.rebuild_to_vec();
    }

    /// 测试IconFontProvider组件
    #[test]
    fn test_iconfont_provider() {
        let config = IconFontConfig {
            script_url: "//at.alicdn.com/t/font_test.js".to_string(),
            extra_common_props: None,
        };

        let mut dom = VirtualDom::new(move || {
            rsx! {
                IconFontProvider {
                    config: config.clone(),

                    div {
                        "Test content"
                    }
                }
            }
        });
        let _ = dom.rebuild_to_vec();
    }

    /// 测试Icon组件的样式生成
    #[test]
    fn test_icon_style_generation() {
        use crate::components::icon::styles::*;

        // 测试基础样式
        let base_style = generate_base_style();
        assert!(!base_style.is_empty());

        // 测试尺寸样式
        let size_style = generate_size_style(&IconSize::Large);
        assert!(size_style.contains("16px"));

        // 测试旋转样式
        let rotation_style = generate_rotation_style(90);
        assert!(rotation_style.contains("rotate(90deg)"));

        // 测试旋转动画样式
        let spin_style = generate_spin_style();
        assert!(spin_style.contains("animation"));

        // 测试主题样式
        let theme_style = generate_theme_style(&IconTheme::Outlined);
        assert!(!theme_style.is_empty());

        // 测试禁用样式
        let disabled_style = generate_disabled_style();
        assert!(disabled_style.contains("not-allowed"));

        // 测试可点击样式
        let clickable_style = generate_clickable_style();
        assert!(clickable_style.contains("pointer"));
    }

    /// 测试Icon组件的CSS类名生成
    #[test]
    fn test_icon_class_names() {
        use crate::components::icon::styles::*;

        let class_names = generate_class_names(
            &IconTheme::Outlined,
            false,
            false,
            true,
            Some("custom-class"),
        );

        assert!(class_names.contains("ant-icon"));
        assert!(class_names.contains("ant-icon-outlined"));
        assert!(class_names.contains("ant-icon-clickable"));
        assert!(class_names.contains("custom-class"));
    }

    /// 测试Icon工具函数
    #[test]
    fn test_icon_utils() {
        use crate::components::icon::utils::*;

        // 测试图标名称验证
        assert!(validate_icon_name("home"));
        assert!(validate_icon_name("user-circle"));
        assert!(validate_icon_name("icon_name"));
        assert!(!validate_icon_name(""));
        assert!(!validate_icon_name("icon with spaces"));

        // 测试缓存键生成
        let cache_key =
            get_icon_cache_key("home", &IconTheme::Outlined, Some("16px"), Some("#ff0000"));
        assert_eq!(cache_key, "home:outlined:16px:#ff0000");

        // 测试常用图标获取
        let home_icon = get_common_icon_svg(&CommonIconType::Home);
        assert!(home_icon.is_some());

        let user_icon = get_common_icon_svg(&CommonIconType::User);
        assert!(user_icon.is_some());
    }

    /// 测试SVG解析功能
    #[test]
    fn test_svg_parsing() {
        use crate::components::icon::utils::*;

        let svg_content = r#"<svg viewBox="0 0 24 24" width="24" height="24"><path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z"/></svg>"#;

        let parsed_icon = parse_svg_string(svg_content);
        assert!(parsed_icon.is_ok());

        let icon = parsed_icon.unwrap();
        assert_eq!(icon.view_box, "0 0 24 24");
        assert_eq!(icon.width, Some("24".to_string()));
        assert_eq!(icon.height, Some("24".to_string()));
        assert!(!icon.path.is_empty());
    }

    /// 测试SVG HTML生成
    #[test]
    fn test_svg_html_generation() {
        use crate::components::icon::utils::*;

        let icon = SvgIcon {
            view_box: "0 0 24 24".to_string(),
            path: "M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z".to_string(),
            width: Some("24".to_string()),
            height: Some("24".to_string()),
        };

        // 测试Outlined主题
        let outlined_html = generate_svg_html(&icon, &IconTheme::Outlined, None);
        assert!(outlined_html.contains("stroke="));
        assert!(outlined_html.contains("fill=\"none\""));

        // 测试Filled主题
        let filled_html = generate_svg_html(&icon, &IconTheme::Filled, None);
        assert!(filled_html.contains("fill=\"currentColor\""));

        // 测试TwoTone主题
        let two_tone_html = generate_svg_html(&icon, &IconTheme::TwoTone, Some("#ff0000"));
        assert!(two_tone_html.contains("#ff0000"));
        assert!(two_tone_html.contains("ant-icon-two-tone"));
    }

    /// 测试图标库功能
    #[test]
    fn test_icon_library() {
        use crate::components::icon::utils::*;

        let mut library = IconLibrary::new();

        let test_icon = SvgIcon {
            view_box: "0 0 24 24".to_string(),
            path: "M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z".to_string(),
            width: None,
            height: None,
        };

        // 注册图标
        library.register_icon("test-star".to_string(), test_icon);

        // 验证图标存在
        assert!(library.get_icon("test-star").is_some());
        assert!(library.get_icon("nonexistent").is_none());

        // 测试批量注册
        let mut icons = std::collections::HashMap::new();
        icons.insert("test-heart".to_string(), SvgIcon {
            view_box: "0 0 24 24".to_string(),
            path: "M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z".to_string(),
            width: None,
            height: None,
        });

        library.register_icons(icons);
        assert!(library.get_icon("test-heart").is_some());
    }

    /// 性能测试：大量图标渲染
    #[test]
    fn test_icon_performance() {
        use std::time::Instant;

        let start = Instant::now();

        for i in 0..100 {
            let mut dom = VirtualDom::new(move || {
                rsx! {
                    div {
                        Icon {
                            icon_type: CommonIconType::Home,
                            theme: IconTheme::Outlined,
                            key: "{i}",
                        }
                        Icon {
                            icon_type: CommonIconType::User,
                            theme: IconTheme::Filled,
                            key: "{i}_user",
                        }
                        Icon {
                            icon_type: CommonIconType::Setting,
                            theme: IconTheme::TwoTone,
                            key: "{i}_setting",
                        }
                    }
                }
            });
            let _ = dom.rebuild_to_vec();
        }

        let duration = start.elapsed();
        println!("渲染300个图标耗时: {:?}", duration);

        // 确保性能在合理范围内（这个阈值可能需要根据实际情况调整）
        assert!(duration.as_millis() < 1000, "图标渲染性能过慢");
    }
}

/// 集成测试模块
#[cfg(test)]
mod integration_tests {
    use super::super::*;
    use dioxus::prelude::*;

    /// 测试Icon组件与其他组件的集成
    #[test]
    fn test_icon_integration() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                div {
                    class: "icon-container",

                    // 在按钮中使用图标
                    button {
                        Icon {
                            icon_type: CommonIconType::Plus,
                            size: IconSize::Small,
                        }
                        " 添加"
                    }

                    // 在列表中使用图标
                    ul {
                        li {
                            Icon {
                                icon_type: CommonIconType::Home,
                                theme: IconTheme::Outlined,
                            }
                            " 首页"
                        }
                        li {
                            Icon {
                                icon_type: CommonIconType::User,
                                theme: IconTheme::Filled,
                            }
                            " 用户"
                        }
                    }

                    // 可点击的图标
                    div {
                        Icon {
                            icon_type: CommonIconType::Setting,
                            on_click: |_| {
                                // 处理设置点击
                            },
                        }
                    }
                }
            }
        });

        let _ = dom.rebuild_to_vec();
    }

    /// 测试主题切换对Icon的影响
    #[test]
    fn test_icon_theme_switching() {
        let current_theme = use_signal(|| IconTheme::Outlined);

        let mut dom = VirtualDom::new(move || {
            rsx! {
                div {
                    button {
                        onclick: move |_| {
                            current_theme.set(match current_theme() {
                                IconTheme::Outlined => IconTheme::Filled,
                                IconTheme::Filled => IconTheme::TwoTone,
                                IconTheme::TwoTone => IconTheme::Outlined,
                            });
                        },
                        "切换主题"
                    }

                    Icon {
                        icon_type: CommonIconType::Home,
                        theme: current_theme(),
                    }
                }
            }
        });

        let _ = dom.rebuild_to_vec();
    }
}

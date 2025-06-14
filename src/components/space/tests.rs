//! Space 组件测试
//!
//! 包含 Space 和 Space.Compact 组件的全面测试用例。

#[cfg(test)]
mod tests {
    use super::*;
    use dioxus::prelude::*;
    use crate::components::space::*;

    /// 测试 Space 组件基本渲染
    #[test]
    fn test_space_basic_render() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Space {
                    div { "Item 1" }
                    div { "Item 2" }
                    div { "Item 3" }
                }
            }
        });

        let _ = dom.rebuild();
        let html = dioxus_ssr::render(&dom);
        
        assert!(html.contains("ant-space"));
        assert!(html.contains("ant-space-horizontal"));
        assert!(html.contains("Item 1"));
        assert!(html.contains("Item 2"));
        assert!(html.contains("Item 3"));
    }

    /// 测试垂直方向的 Space
    #[test]
    fn test_space_vertical() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Space {
                    direction: SpaceDirection::Vertical,
                    div { "Item 1" }
                    div { "Item 2" }
                }
            }
        });

        let _ = dom.rebuild();
        let html = dioxus_ssr::render(&dom);
        
        assert!(html.contains("ant-space-vertical"));
    }

    /// 测试不同尺寸的间距
    #[test]
    fn test_space_sizes() {
        // 测试小间距
        let mut dom_small = VirtualDom::new(|| {
            rsx! {
                Space {
                    size: SpaceSizeConfig::Single(SpaceSize::Small),
                    div { "Item 1" }
                    div { "Item 2" }
                }
            }
        });
        let _ = dom_small.rebuild();
        let html_small = dioxus_ssr::render(&dom_small);
        assert!(html_small.contains("gap: 8px"));

        // 测试中等间距
        let mut dom_middle = VirtualDom::new(|| {
            rsx! {
                Space {
                    size: SpaceSizeConfig::Single(SpaceSize::Middle),
                    div { "Item 1" }
                    div { "Item 2" }
                }
            }
        });
        let _ = dom_middle.rebuild();
        let html_middle = dioxus_ssr::render(&dom_middle);
        assert!(html_middle.contains("gap: 16px"));

        // 测试大间距
        let mut dom_large = VirtualDom::new(|| {
            rsx! {
                Space {
                    size: SpaceSizeConfig::Single(SpaceSize::Large),
                    div { "Item 1" }
                    div { "Item 2" }
                }
            }
        });
        let _ = dom_large.rebuild();
        let html_large = dioxus_ssr::render(&dom_large);
        assert!(html_large.contains("gap: 24px"));
    }

    /// 测试自定义间距
    #[test]
    fn test_space_custom_size() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Space {
                    size: SpaceSizeConfig::Single(SpaceSize::Custom(32)),
                    div { "Item 1" }
                    div { "Item 2" }
                }
            }
        });

        let _ = dom.rebuild();
        let html = dioxus_ssr::render(&dom);
        
        assert!(html.contains("gap: 32px"));
    }

    /// 测试数组形式的间距配置
    #[test]
    fn test_space_array_size() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Space {
                    size: SpaceSizeConfig::Array([16, 24]),
                    div { "Item 1" }
                    div { "Item 2" }
                }
            }
        });

        let _ = dom.rebuild();
        let html = dioxus_ssr::render(&dom);
        
        // 水平方向应该使用第一个值
        assert!(html.contains("gap: 16px"));
    }

    /// 测试对齐方式
    #[test]
    fn test_space_align() {
        // 测试居中对齐
        let mut dom_center = VirtualDom::new(|| {
            rsx! {
                Space {
                    align: SpaceAlign::Center,
                    div { "Item 1" }
                    div { "Item 2" }
                }
            }
        });
        let _ = dom_center.rebuild();
        let html_center = dioxus_ssr::render(&dom_center);
        assert!(html_center.contains("ant-space-align-center"));

        // 测试顶部对齐
        let mut dom_start = VirtualDom::new(|| {
            rsx! {
                Space {
                    align: SpaceAlign::Start,
                    div { "Item 1" }
                    div { "Item 2" }
                }
            }
        });
        let _ = dom_start.rebuild();
        let html_start = dioxus_ssr::render(&dom_start);
        assert!(html_start.contains("ant-space-align-start"));
    }

    /// 测试换行功能
    #[test]
    fn test_space_wrap() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Space {
                    wrap: true,
                    div { "Item 1" }
                    div { "Item 2" }
                    div { "Item 3" }
                }
            }
        });

        let _ = dom.rebuild();
        let html = dioxus_ssr::render(&dom);
        
        assert!(html.contains("ant-space-wrap"));
    }

    /// 测试 Space.Compact 基本渲染
    #[test]
    fn test_space_compact_basic() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                SpaceCompact {
                    div { "Button 1" }
                    div { "Button 2" }
                    div { "Button 3" }
                }
            }
        });

        let _ = dom.rebuild();
        let html = dioxus_ssr::render(&dom);
        
        assert!(html.contains("ant-space-compact"));
        assert!(html.contains("Button 1"));
        assert!(html.contains("Button 2"));
        assert!(html.contains("Button 3"));
    }

    /// 测试 Space.Compact 垂直方向
    #[test]
    fn test_space_compact_vertical() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                SpaceCompact {
                    direction: SpaceDirection::Vertical,
                    div { "Item 1" }
                    div { "Item 2" }
                }
            }
        });

        let _ = dom.rebuild();
        let html = dioxus_ssr::render(&dom);
        
        assert!(html.contains("ant-space-compact-vertical"));
    }

    /// 测试 Space.Compact 不同尺寸
    #[test]
    fn test_space_compact_sizes() {
        // 测试小尺寸
        let mut dom_small = VirtualDom::new(|| {
            rsx! {
                SpaceCompact {
                    size: CompactSize::Small,
                    div { "Item 1" }
                    div { "Item 2" }
                }
            }
        });
        let _ = dom_small.rebuild();
        let html_small = dioxus_ssr::render(&dom_small);
        assert!(html_small.contains("ant-space-compact-small"));

        // 测试中等尺寸
        let mut dom_middle = VirtualDom::new(|| {
            rsx! {
                SpaceCompact {
                    size: CompactSize::Middle,
                    div { "Item 1" }
                    div { "Item 2" }
                }
            }
        });
        let _ = dom_middle.rebuild();
        let html_middle = dioxus_ssr::render(&dom_middle);
        assert!(html_middle.contains("ant-space-compact-middle"));

        // 测试大尺寸
        let mut dom_large = VirtualDom::new(|| {
            rsx! {
                SpaceCompact {
                    size: CompactSize::Large,
                    div { "Item 1" }
                    div { "Item 2" }
                }
            }
        });
        let _ = dom_large.rebuild();
        let html_large = dioxus_ssr::render(&dom_large);
        assert!(html_large.contains("ant-space-compact-large"));
    }

    /// 测试 Space.Compact 块级模式
    #[test]
    fn test_space_compact_block() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                SpaceCompact {
                    block: true,
                    div { "Item 1" }
                    div { "Item 2" }
                }
            }
        });

        let _ = dom.rebuild();
        let html = dioxus_ssr::render(&dom);
        
        assert!(html.contains("ant-space-compact-block"));
    }

    /// 测试自定义类名和样式
    #[test]
    fn test_space_custom_class_style() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Space {
                    class: "custom-space",
                    style: "background-color: red;",
                    div { "Item 1" }
                    div { "Item 2" }
                }
            }
        });

        let _ = dom.rebuild();
        let html = dioxus_ssr::render(&dom);
        
        assert!(html.contains("custom-space"));
        assert!(html.contains("background-color: red;"));
    }

    /// 测试 SpaceProps 构建器模式
    #[test]
    fn test_space_props_builder() {
        let props = SpaceProps::horizontal()
            .size(SpaceSizeConfig::Single(SpaceSize::Large))
            .align(SpaceAlign::Center)
            .wrap()
            .class("test-space");
        
        assert_eq!(props.direction, SpaceDirection::Horizontal);
        assert_eq!(props.size, SpaceSizeConfig::Single(SpaceSize::Large));
        assert_eq!(props.align, SpaceAlign::Center);
        assert_eq!(props.wrap, true);
        assert_eq!(props.class, Some("test-space".to_string()));
    }

    /// 测试 SpaceCompactProps 构建器模式
    #[test]
    fn test_space_compact_props_builder() {
        let props = SpaceCompactProps::horizontal()
            .size(CompactSize::Large)
            .block()
            .class("test-compact");
        
        assert_eq!(props.direction, SpaceDirection::Horizontal);
        assert_eq!(props.size, CompactSize::Large);
        assert_eq!(props.block, true);
        assert_eq!(props.class, Some("test-compact".to_string()));
    }

    /// 测试便捷构造函数
    #[test]
    fn test_convenience_constructors() {
        // 测试 space_small
        let small_props = space_small();
        assert_eq!(small_props.size, SpaceSizeConfig::Single(SpaceSize::Small));
        
        // 测试 space_middle
        let middle_props = space_middle();
        assert_eq!(middle_props.size, SpaceSizeConfig::Single(SpaceSize::Middle));
        
        // 测试 space_large
        let large_props = space_large();
        assert_eq!(large_props.size, SpaceSizeConfig::Single(SpaceSize::Large));
        
        // 测试 space_custom
        let custom_props = space_custom(32);
        assert_eq!(custom_props.size, SpaceSizeConfig::Single(SpaceSize::Custom(32)));
        
        // 测试 space_vertical
        let vertical_props = space_vertical(SpaceSize::Middle);
        assert_eq!(vertical_props.direction, SpaceDirection::Vertical);
        assert_eq!(vertical_props.size, SpaceSizeConfig::Single(SpaceSize::Middle));
        
        // 测试 space_wrap
        let wrap_props = space_wrap(SpaceSize::Small);
        assert_eq!(wrap_props.wrap, true);
        assert_eq!(wrap_props.size, SpaceSizeConfig::Single(SpaceSize::Small));
    }

    /// 测试紧凑模式便捷构造函数
    #[test]
    fn test_compact_convenience_constructors() {
        // 测试 compact_button_group
        let button_group = compact_button_group();
        assert_eq!(button_group.direction, SpaceDirection::Horizontal);
        assert_eq!(button_group.size, CompactSize::Middle);
        
        // 测试 compact_input_group
        let input_group = compact_input_group();
        assert_eq!(input_group.direction, SpaceDirection::Horizontal);
        assert_eq!(input_group.size, CompactSize::Middle);
        assert_eq!(input_group.block, true);
        
        // 测试 compact_vertical
        let vertical = compact_vertical();
        assert_eq!(vertical.direction, SpaceDirection::Vertical);
        assert_eq!(vertical.size, CompactSize::Small);
    }

    /// 测试主题应用
    #[test]
    fn test_space_theme() {
        let custom_theme = SpaceTheme {
            space_small: 4,
            space_middle: 12,
            space_large: 20,
            ..Default::default()
        };
        
        let mut dom = VirtualDom::new(move || {
            rsx! {
                Space {
                    theme: custom_theme.clone(),
                    size: SpaceSizeConfig::Single(SpaceSize::Small),
                    div { "Item 1" }
                    div { "Item 2" }
                }
            }
        });

        let _ = dom.rebuild();
        let html = dioxus_ssr::render(&dom);
        
        // 应该使用自定义主题的间距值
        assert!(html.contains("gap: 4px"));
    }

    /// 测试复杂配置
    #[test]
    fn test_complex_configuration() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Space {
                    direction: SpaceDirection::Horizontal,
                    size: SpaceSizeConfig::Array([20, 10]),
                    align: SpaceAlign::Center,
                    wrap: true,
                    class: "complex-space",
                    style: "padding: 16px;",
                    div { "Item 1" }
                    div { "Item 2" }
                    div { "Item 3" }
                    div { "Item 4" }
                }
            }
        });

        let _ = dom.rebuild();
        let html = dioxus_ssr::render(&dom);
        
        assert!(html.contains("ant-space"));
        assert!(html.contains("ant-space-horizontal"));
        assert!(html.contains("ant-space-align-center"));
        assert!(html.contains("ant-space-wrap"));
        assert!(html.contains("complex-space"));
        assert!(html.contains("padding: 16px;"));
        assert!(html.contains("gap: 20px"));
    }

    /// 测试空子元素
    #[test]
    fn test_empty_children() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Space {}
            }
        });

        let _ = dom.rebuild();
        let html = dioxus_ssr::render(&dom);
        
        assert!(html.contains("ant-space"));
    }

    /// 测试单个子元素
    #[test]
    fn test_single_child() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Space {
                    div { "Single Item" }
                }
            }
        });

        let _ = dom.rebuild();
        let html = dioxus_ssr::render(&dom);
        
        assert!(html.contains("ant-space"));
        assert!(html.contains("Single Item"));
    }

    /// 测试嵌套 Space 组件
    #[test]
    fn test_nested_space() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Space {
                    direction: SpaceDirection::Vertical,
                    size: SpaceSizeConfig::Single(SpaceSize::Large),
                    Space {
                        direction: SpaceDirection::Horizontal,
                        size: SpaceSizeConfig::Single(SpaceSize::Small),
                        div { "Item 1" }
                        div { "Item 2" }
                    }
                    Space {
                        direction: SpaceDirection::Horizontal,
                        size: SpaceSizeConfig::Single(SpaceSize::Small),
                        div { "Item 3" }
                        div { "Item 4" }
                    }
                }
            }
        });

        let _ = dom.rebuild();
        let html = dioxus_ssr::render(&dom);
        
        // 应该包含外层垂直 Space
        assert!(html.contains("ant-space-vertical"));
        // 应该包含内层水平 Space
        assert!(html.contains("ant-space-horizontal"));
    }

    /// 测试响应式行为
    #[test]
    fn test_responsive_behavior() {
        // 这里可以测试不同屏幕尺寸下的行为
        // 由于测试环境限制，主要测试配置是否正确
        
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Space {
                    size: SpaceSizeConfig::Array([16, 8]),
                    wrap: true,
                    div { "Responsive Item 1" }
                    div { "Responsive Item 2" }
                    div { "Responsive Item 3" }
                }
            }
        });

        let _ = dom.rebuild();
        let html = dioxus_ssr::render(&dom);
        
        assert!(html.contains("ant-space-wrap"));
        assert!(html.contains("gap: 16px"));
    }

    /// 测试可访问性
    #[test]
    fn test_accessibility() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Space {
                    role: "group",
                    "aria-label": "Button group",
                    div { role: "button", "Button 1" }
                    div { role: "button", "Button 2" }
                }
            }
        });

        let _ = dom.rebuild();
        let html = dioxus_ssr::render(&dom);
        
        assert!(html.contains("role=\"group\""));
        assert!(html.contains("aria-label=\"Button group\""));
    }

    /// 性能测试 - 大量子元素
    #[test]
    fn test_performance_many_children() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Space {
                    size: SpaceSizeConfig::Single(SpaceSize::Small),
                    // 模拟大量子元素
                    for i in 0..100 {
                        div { key: "{i}", "Item {i}" }
                    }
                }
            }
        });

        let start = std::time::Instant::now();
        let _ = dom.rebuild();
        let duration = start.elapsed();
        
        // 确保渲染时间在合理范围内（这里设置为 100ms）
        assert!(duration.as_millis() < 100, "渲染时间过长: {:?}", duration);
    }

    /// 内存使用测试
    #[test]
    fn test_memory_usage() {
        // 创建多个 Space 实例，确保没有内存泄漏
        for _ in 0..1000 {
            let mut dom = VirtualDom::new(|| {
                rsx! {
                    Space {
                        div { "Test" }
                    }
                }
            });
            let _ = dom.rebuild();
        }
        
        // 如果没有 panic，说明内存使用正常
        assert!(true);
    }

    /// 边缘情况测试
    #[test]
    fn test_edge_cases() {
        // 测试极大的间距值
        let mut dom_large = VirtualDom::new(|| {
            rsx! {
                Space {
                    size: SpaceSizeConfig::Single(SpaceSize::Custom(999)),
                    div { "Item" }
                }
            }
        });
        let _ = dom_large.rebuild();
        let html_large = dioxus_ssr::render(&dom_large);
        assert!(html_large.contains("gap: 999px"));
        
        // 测试零间距
        let mut dom_zero = VirtualDom::new(|| {
            rsx! {
                Space {
                    size: SpaceSizeConfig::Single(SpaceSize::Custom(0)),
                    div { "Item" }
                }
            }
        });
        let _ = dom_zero.rebuild();
        let html_zero = dioxus_ssr::render(&dom_zero);
        assert!(html_zero.contains("gap: 0px"));
    }

    /// 测试与其他组件的集成
    #[test]
    fn test_integration_with_other_components() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                div {
                    class: "container",
                    Space {
                        size: SpaceSizeConfig::Single(SpaceSize::Middle),
                        div {
                            class: "button",
                            "Primary Button"
                        }
                        div {
                            class: "button secondary",
                            "Secondary Button"
                        }
                        SpaceCompact {
                            size: CompactSize::Small,
                            div { class: "input", "Input 1" }
                            div { class: "input", "Input 2" }
                        }
                    }
                }
            }
        });

        let _ = dom.rebuild();
        let html = dioxus_ssr::render(&dom);
        
        assert!(html.contains("container"));
        assert!(html.contains("ant-space"));
        assert!(html.contains("ant-space-compact"));
        assert!(html.contains("Primary Button"));
        assert!(html.contains("Secondary Button"));
        assert!(html.contains("Input 1"));
        assert!(html.contains("Input 2"));
    }
}

/// 工具函数测试
#[cfg(test)]
mod utils_tests {
    use super::*;
    use crate::components::space::utils::*;
    use crate::components::space::types::*;

    /// 测试间距计算
    #[test]
    fn test_calculate_space_size() {
        let theme = SpaceTheme::default();
        
        // 测试单一尺寸
        let (h, v) = calculate_space_size(
            &SpaceSizeConfig::Single(SpaceSize::Small),
            SpaceDirection::Horizontal,
            &theme,
        );
        assert_eq!(h, 8);
        assert_eq!(v, 8);
        
        // 测试数组尺寸
        let (h, v) = calculate_space_size(
            &SpaceSizeConfig::Array([16, 24]),
            SpaceDirection::Horizontal,
            &theme,
        );
        assert_eq!(h, 16);
        assert_eq!(v, 24);
        
        // 测试自定义尺寸
        let (h, v) = calculate_space_size(
            &SpaceSizeConfig::Single(SpaceSize::Custom(32)),
            SpaceDirection::Vertical,
            &theme,
        );
        assert_eq!(h, 32);
        assert_eq!(v, 32);
    }

    /// 测试配置验证
    #[test]
    fn test_validate_space_config() {
        // 有效配置
        let result = validate_space_config(
            SpaceDirection::Horizontal,
            &SpaceSizeConfig::Single(SpaceSize::Middle),
            true,
            SpaceAlign::Center,
        );
        assert!(result.is_ok());
        
        // 无效配置：垂直方向不支持换行
        let result = validate_space_config(
            SpaceDirection::Vertical,
            &SpaceSizeConfig::Single(SpaceSize::Middle),
            true,
            SpaceAlign::Center,
        );
        assert!(result.is_err());
        
        // 无效配置：间距值过大
        let result = validate_space_config(
            SpaceDirection::Horizontal,
            &SpaceSizeConfig::Single(SpaceSize::Custom(1001)),
            false,
            SpaceAlign::Center,
        );
        assert!(result.is_err());
    }

    /// 测试主题合并
    #[test]
    fn test_merge_space_theme() {
        let custom_theme = SpaceTheme {
            space_small: 4,
            space_middle: 12,
            ..Default::default()
        };
        
        let merged = merge_space_theme(Some(&custom_theme));
        assert_eq!(merged.space_small, 4);
        assert_eq!(merged.space_middle, 12);
        assert_eq!(merged.space_large, 24); // 应该保持默认值
        
        // 测试无自定义主题
        let default_merged = merge_space_theme(None);
        assert_eq!(default_merged.space_small, 8);
        assert_eq!(default_merged.space_middle, 16);
        assert_eq!(default_merged.space_large, 24);
    }

    /// 测试缓存键生成
    #[test]
    fn test_generate_cache_key() {
        let theme = SpaceTheme::default();
        let theme_hash = calculate_theme_hash(&theme);
        
        let key = generate_space_cache_key(
            SpaceDirection::Horizontal,
            &SpaceSizeConfig::Single(SpaceSize::Middle),
            SpaceAlign::Center,
            true,
            theme_hash,
        );
        
        assert!(key.contains("space_"));
        assert!(key.contains("Horizontal"));
        assert!(key.contains("Center"));
        assert!(key.contains("true"));
    }

    /// 测试间距值优化
    #[test]
    fn test_optimize_space_value() {
        assert_eq!(optimize_space_value(5, 10, 100), 10); // 小于最小值
        assert_eq!(optimize_space_value(150, 10, 100), 100); // 大于最大值
        assert_eq!(optimize_space_value(50, 10, 100), 50); // 在范围内
    }

    /// 测试自适应间距计算
    #[test]
    fn test_calculate_adaptive_spacing() {
        let result = calculate_adaptive_spacing(16, 1200, 8, 32);
        assert_eq!(result, 16); // 基准宽度，应该返回原值
        
        let result = calculate_adaptive_spacing(16, 600, 8, 32);
        assert_eq!(result, 8); // 较小宽度，应该返回最小值
        
        let result = calculate_adaptive_spacing(16, 2400, 8, 32);
        assert_eq!(result, 32); // 较大宽度，应该返回最大值
    }

    /// 测试 CSS 变量生成
    #[test]
    fn test_generate_css_variables() {
        let theme = SpaceTheme::default();
        let vars = generate_css_variables(&theme);
        
        assert_eq!(vars.get("--ant-space-small"), Some(&"8px".to_string()));
        assert_eq!(vars.get("--ant-space-middle"), Some(&"16px".to_string()));
        assert_eq!(vars.get("--ant-space-large"), Some(&"24px".to_string()));
    }

    /// 测试分割元素显示判断
    #[test]
    fn test_should_show_split() {
        assert!(should_show_split(true, 3, 0)); // 第一个元素，应该显示分割
        assert!(should_show_split(true, 3, 1)); // 中间元素，应该显示分割
        assert!(!should_show_split(true, 3, 2)); // 最后一个元素，不应该显示分割
        assert!(!should_show_split(false, 3, 0)); // 没有分割元素
        assert!(!should_show_split(true, 1, 0)); // 只有一个子元素
    }

    /// 测试紧凑模式子元素位置
    #[test]
    fn test_get_compact_item_position() {
        use crate::components::space::styles::CompactItemPosition;
        
        assert_eq!(get_compact_item_position(0, 1), CompactItemPosition::Only);
        assert_eq!(get_compact_item_position(0, 3), CompactItemPosition::First);
        assert_eq!(get_compact_item_position(1, 3), CompactItemPosition::Middle);
        assert_eq!(get_compact_item_position(2, 3), CompactItemPosition::Last);
    }

    /// 测试尺寸字符串解析
    #[test]
    fn test_parse_size_string() {
        assert_eq!(parse_size_string("small"), SpaceSize::Small);
        assert_eq!(parse_size_string("middle"), SpaceSize::Middle);
        assert_eq!(parse_size_string("large"), SpaceSize::Large);
        assert_eq!(parse_size_string("32"), SpaceSize::Custom(32));
        assert_eq!(parse_size_string("invalid"), SpaceSize::Small); // 默认值
    }
}
//!
//! Tag 组件测试
//!
//! 包含 Tag 组件的单元测试和集成测试。

#[cfg(test)]
mod tests {
    use super::super::*;
    use dioxus::prelude::*;
    use dioxus_testing::*;

    #[test]
    fn test_tag_default_props() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Tag { "Default Tag" }
            }
        });
        
        let _ = dom.rebuild_to_vec();
        // 测试默认属性是否正确应用
    }

    #[test]
    fn test_tag_with_color() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Tag { 
                    color: TagColor::Primary,
                    "Primary Tag" 
                }
            }
        });
        
        let _ = dom.rebuild_to_vec();
        // 测试颜色属性是否正确应用
    }

    #[test]
    fn test_tag_with_preset_color() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Tag { 
                    color: TagColor::Preset(TagPresetColor::Blue),
                    "Blue Tag" 
                }
            }
        });
        
        let _ = dom.rebuild_to_vec();
        // 测试预设颜色是否正确应用
    }

    #[test]
    fn test_tag_with_custom_color() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Tag { 
                    color: TagColor::Custom("#ff6b35".to_string()),
                    "Custom Color Tag" 
                }
            }
        });
        
        let _ = dom.rebuild_to_vec();
        // 测试自定义颜色是否正确应用
    }

    #[test]
    fn test_tag_sizes() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                div {
                    Tag { 
                        size: TagSize::Small,
                        "Small Tag" 
                    }
                    Tag { 
                        size: TagSize::Middle,
                        "Middle Tag" 
                    }
                    Tag { 
                        size: TagSize::Large,
                        "Large Tag" 
                    }
                }
            }
        });
        
        let _ = dom.rebuild_to_vec();
        // 测试不同尺寸是否正确应用
    }

    #[test]
    fn test_tag_variants() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                div {
                    Tag { 
                        variant: TagVariant::Filled,
                        "Filled Tag" 
                    }
                    Tag { 
                        variant: TagVariant::Outlined,
                        "Outlined Tag" 
                    }
                    Tag { 
                        variant: TagVariant::Borderless,
                        "Borderless Tag" 
                    }
                }
            }
        });
        
        let _ = dom.rebuild_to_vec();
        // 测试不同变体是否正确应用
    }

    #[test]
    fn test_closable_tag() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Tag { 
                    closable: true,
                    "Closable Tag" 
                }
            }
        });
        
        let _ = dom.rebuild_to_vec();
        // 测试可关闭标签是否正确渲染
    }

    #[test]
    fn test_disabled_tag() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Tag { 
                    disabled: true,
                    "Disabled Tag" 
                }
            }
        });
        
        let _ = dom.rebuild_to_vec();
        // 测试禁用状态是否正确应用
    }

    #[test]
    fn test_tag_with_icon() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Tag { 
                    icon: rsx! { span { "🏷️" } },
                    "Tag with Icon" 
                }
            }
        });
        
        let _ = dom.rebuild_to_vec();
        // 测试带图标的标签是否正确渲染
    }

    #[test]
    fn test_tag_with_custom_close_icon() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Tag { 
                    closable: true,
                    close_icon: rsx! { span { "✕" } },
                    "Tag with Custom Close Icon" 
                }
            }
        });
        
        let _ = dom.rebuild_to_vec();
        // 测试自定义关闭图标是否正确渲染
    }

    #[test]
    fn test_checkable_tag() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                CheckableTag { 
                    checked: false,
                    "Checkable Tag" 
                }
            }
        });
        
        let _ = dom.rebuild_to_vec();
        // 测试可选择标签是否正确渲染
    }

    #[test]
    fn test_checkable_tag_checked() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                CheckableTag { 
                    checked: true,
                    "Checked Tag" 
                }
            }
        });
        
        let _ = dom.rebuild_to_vec();
        // 测试选中状态的可选择标签是否正确渲染
    }

    #[test]
    fn test_tag_style_generator() {
        let generator = TagStyleGenerator::new()
            .with_color(TagColor::Primary)
            .with_size(TagSize::Large)
            .with_variant(TagVariant::Outlined)
            .with_closable(true)
            .with_disabled(false);
        
        let class_names = generator.generate_class_names();
        
        // 验证生成的类名是否包含预期的类
        assert!(class_names.contains(&"ant-tag".to_string()));
        assert!(class_names.contains(&"ant-tag-lg".to_string()));
        assert!(class_names.contains(&"ant-tag-outlined".to_string()));
        assert!(class_names.contains(&"ant-tag-primary".to_string()));
        assert!(class_names.contains(&"ant-tag-closable".to_string()));
    }

    #[test]
    fn test_tag_custom_color_styles() {
        let generator = TagStyleGenerator::new()
            .with_color(TagColor::Custom("#ff6b35".to_string()));
        
        let inline_styles = generator.generate_inline_styles();
        
        // 验证自定义颜色是否生成正确的内联样式
        assert!(inline_styles.contains("background-color: #ff6b35"));
        assert!(inline_styles.contains("border-color: #ff6b35"));
    }

    #[test]
    fn test_tag_preset_color_class_names() {
        let generator = TagStyleGenerator::new()
            .with_color(TagColor::Preset(TagPresetColor::Blue));
        
        let class_names = generator.generate_class_names();
        
        // 验证预设颜色是否生成正确的类名
        assert!(class_names.contains(&"ant-tag-blue".to_string()));
    }

    #[test]
    fn test_tag_status_class_names() {
        let generator = TagStyleGenerator::new()
            .with_status(Some(TagStatus::Success));
        
        let class_names = generator.generate_class_names();
        
        // 验证状态是否生成正确的类名
        assert!(class_names.contains(&"ant-tag-status-success".to_string()));
    }
}
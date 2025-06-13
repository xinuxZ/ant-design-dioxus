//! Space 组件的测试
//!
//! 包含 Space 组件的单元测试和集成测试，验证组件的各种功能和样式。

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::space::*;
    use dioxus::prelude::*;
    use dioxus_testing::*;

    /// 测试默认 Space 组件
    #[test]
    fn test_default_space() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Space {
                    div { "Item 1" }
                    div { "Item 2" }
                    div { "Item 3" }
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        
        // 验证基本结构
        let html = dom.base_scope().root_node();
        // 这里可以添加更具体的断言来验证渲染结果
    }

    /// 测试垂直方向的 Space 组件
    #[test]
    fn test_vertical_space() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Space {
                    direction: SpaceDirection::Vertical,
                    div { "Item 1" }
                    div { "Item 2" }
                    div { "Item 3" }
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        
        // 验证垂直布局
        let html = dom.base_scope().root_node();
        // 验证是否包含垂直方向的类名
    }

    /// 测试不同尺寸的 Space 组件
    #[test]
    fn test_space_sizes() {
        // 测试小尺寸
        let mut dom_small = VirtualDom::new(|| {
            rsx! {
                Space {
                    size: SpaceSize::Small,
                    div { "Item 1" }
                    div { "Item 2" }
                }
            }
        });
        let _ = dom_small.rebuild_to_vec();

        // 测试大尺寸
        let mut dom_large = VirtualDom::new(|| {
            rsx! {
                Space {
                    size: SpaceSize::Large,
                    div { "Item 1" }
                    div { "Item 2" }
                }
            }
        });
        let _ = dom_large.rebuild_to_vec();

        // 测试自定义尺寸
        let mut dom_custom = VirtualDom::new(|| {
            rsx! {
                Space {
                    size: SpaceSize::Custom(32),
                    div { "Item 1" }
                    div { "Item 2" }
                }
            }
        });
        let _ = dom_custom.rebuild_to_vec();
    }

    /// 测试不同对齐方式的 Space 组件
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
        let _ = dom_center.rebuild_to_vec();

        // 测试结束对齐
        let mut dom_end = VirtualDom::new(|| {
            rsx! {
                Space {
                    align: SpaceAlign::End,
                    div { "Item 1" }
                    div { "Item 2" }
                }
            }
        });
        let _ = dom_end.rebuild_to_vec();

        // 测试基线对齐
        let mut dom_baseline = VirtualDom::new(|| {
            rsx! {
                Space {
                    align: SpaceAlign::Baseline,
                    div { "Item 1" }
                    div { "Item 2" }
                }
            }
        });
        let _ = dom_baseline.rebuild_to_vec();
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
                    div { "Item 4" }
                    div { "Item 5" }
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        
        // 验证换行样式
        let html = dom.base_scope().root_node();
        // 验证是否包含换行的类名
    }

    /// 测试自定义样式
    #[test]
    fn test_custom_styles() {
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

        let _ = dom.rebuild_to_vec();
        
        // 验证自定义样式
        let html = dom.base_scope().root_node();
        // 验证是否包含自定义类名和样式
    }

    /// 测试前缀类名
    #[test]
    fn test_custom_prefix() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Space {
                    prefix_cls: "custom",
                    div { "Item 1" }
                    div { "Item 2" }
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        
        // 验证自定义前缀
        let html = dom.base_scope().root_node();
        // 验证是否使用了自定义前缀类名
    }
}

#[cfg(test)]
mod style_tests {
    use super::*;
    use crate::components::space::styles::*;

    /// 测试样式生成器的基本功能
    #[test]
    fn test_style_generator_basic() {
        let generator = SpaceStyleGenerator::new();
        let class_name = generator.generate_class_name();
        
        assert!(class_name.contains("ant-space"));
        assert!(class_name.contains("ant-space-horizontal"));
        assert!(class_name.contains("ant-space-middle"));
        assert!(class_name.contains("ant-space-align-start"));
    }

    /// 测试垂直方向样式生成
    #[test]
    fn test_style_generator_vertical() {
        let generator = SpaceStyleGenerator::new()
            .with_direction(SpaceDirection::Vertical);
        let class_name = generator.generate_class_name();
        
        assert!(class_name.contains("ant-space-vertical"));
    }

    /// 测试不同尺寸样式生成
    #[test]
    fn test_style_generator_sizes() {
        // 测试小尺寸
        let generator_small = SpaceStyleGenerator::new()
            .with_size(SpaceSize::Small);
        let class_name_small = generator_small.generate_class_name();
        assert!(class_name_small.contains("ant-space-small"));

        // 测试大尺寸
        let generator_large = SpaceStyleGenerator::new()
            .with_size(SpaceSize::Large);
        let class_name_large = generator_large.generate_class_name();
        assert!(class_name_large.contains("ant-space-large"));

        // 测试自定义尺寸
        let generator_custom = SpaceStyleGenerator::new()
            .with_size(SpaceSize::Custom(32));
        let class_name_custom = generator_custom.generate_class_name();
        assert!(class_name_custom.contains("ant-space-custom"));
        
        let inline_style = generator_custom.generate_inline_styles();
        assert!(inline_style.contains("--ant-space-gap: 32px"));
    }

    /// 测试对齐方式样式生成
    #[test]
    fn test_style_generator_align() {
        let generator = SpaceStyleGenerator::new()
            .with_align(SpaceAlign::Center);
        let class_name = generator.generate_class_name();
        
        assert!(class_name.contains("ant-space-align-center"));
    }

    /// 测试换行样式生成
    #[test]
    fn test_style_generator_wrap() {
        let generator = SpaceStyleGenerator::new()
            .with_wrap(true);
        let class_name = generator.generate_class_name();
        
        assert!(class_name.contains("ant-space-wrap"));
    }

    /// 测试分割样式生成
    #[test]
    fn test_style_generator_split() {
        let generator = SpaceStyleGenerator::new()
            .with_split(true);
        let class_name = generator.generate_class_name();
        
        assert!(class_name.contains("ant-space-split"));
    }

    /// 测试自定义前缀
    #[test]
    fn test_style_generator_custom_prefix() {
        let generator = SpaceStyleGenerator::new()
            .with_prefix_cls("custom");
        let class_name = generator.generate_class_name();
        
        assert!(class_name.contains("custom-space"));
        assert!(class_name.contains("custom-space-horizontal"));
    }

    /// 测试基础样式生成
    #[test]
    fn test_base_styles() {
        let styles = SpaceStyles::new();
        let base_styles = styles.base_styles();
        
        assert!(base_styles.contains(".ant-space"));
        assert!(base_styles.contains("display: inline-flex"));
        assert!(base_styles.contains(".ant-space-vertical"));
        assert!(base_styles.contains(".ant-space-horizontal"));
        assert!(base_styles.contains(".ant-space-small"));
        assert!(base_styles.contains(".ant-space-middle"));
        assert!(base_styles.contains(".ant-space-large"));
    }
}

#[cfg(test)]
mod type_tests {
    use super::*;
    use crate::components::space::types::*;

    /// 测试 SpaceDirection 类型
    #[test]
    fn test_space_direction() {
        assert_eq!(SpaceDirection::Horizontal.to_class(), "horizontal");
        assert_eq!(SpaceDirection::Vertical.to_class(), "vertical");
        assert_eq!(SpaceDirection::default(), SpaceDirection::Horizontal);
    }

    /// 测试 SpaceAlign 类型
    #[test]
    fn test_space_align() {
        assert_eq!(SpaceAlign::Start.to_class(), "start");
        assert_eq!(SpaceAlign::End.to_class(), "end");
        assert_eq!(SpaceAlign::Center.to_class(), "center");
        assert_eq!(SpaceAlign::Baseline.to_class(), "baseline");
        assert_eq!(SpaceAlign::default(), SpaceAlign::Start);
    }

    /// 测试 SpaceSize 类型
    #[test]
    fn test_space_size() {
        assert_eq!(SpaceSize::Small.to_class(), "small");
        assert_eq!(SpaceSize::Middle.to_class(), "middle");
        assert_eq!(SpaceSize::Large.to_class(), "large");
        assert_eq!(SpaceSize::Custom(32).to_class(), "custom");
        
        assert_eq!(SpaceSize::Small.to_pixels(), 8);
        assert_eq!(SpaceSize::Middle.to_pixels(), 16);
        assert_eq!(SpaceSize::Large.to_pixels(), 24);
        assert_eq!(SpaceSize::Custom(32).to_pixels(), 32);
        
        assert_eq!(SpaceSize::default(), SpaceSize::Middle);
    }
}
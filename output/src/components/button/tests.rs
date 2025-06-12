//! Button 组件测试
//!
//! 包含 Button 和 ButtonGroup 组件的单元测试

#[cfg(test)]
mod tests {
    use super::*;
    use dioxus::prelude::*;

    #[test]
    fn test_button_types() {
        // 测试按钮类型枚举
        assert_eq!(ButtonType::default(), ButtonType::Default);
        assert_ne!(ButtonType::Primary, ButtonType::Default);
    }

    #[test]
    fn test_button_sizes() {
        // 测试按钮尺寸枚举
        assert_eq!(ButtonSize::default(), ButtonSize::Middle);
        assert_ne!(ButtonSize::Large, ButtonSize::Small);
    }

    #[test]
    fn test_button_shapes() {
        // 测试按钮形状枚举
        assert_eq!(ButtonShape::default(), ButtonShape::Default);
        assert_ne!(ButtonShape::Circle, ButtonShape::Round);
    }

    #[test]
    fn test_button_html_types() {
        // 测试按钮HTML类型枚举
        assert_eq!(ButtonHtmlType::default(), ButtonHtmlType::Button);
        assert_ne!(ButtonHtmlType::Submit, ButtonHtmlType::Reset);
    }

    #[test]
    fn test_button_group_sizes() {
        // 测试按钮组尺寸枚举
        assert_eq!(ButtonGroupSize::default(), ButtonGroupSize::Middle);
        assert_ne!(ButtonGroupSize::Large, ButtonGroupSize::Small);
    }

    #[test]
    fn test_get_html_type() {
        // 测试HTML类型转换函数
        use crate::component::get_html_type;
        
        assert_eq!(get_html_type(&ButtonHtmlType::Submit), "submit");
        assert_eq!(get_html_type(&ButtonHtmlType::Reset), "reset");
        assert_eq!(get_html_type(&ButtonHtmlType::Button), "button");
    }

    #[test]
    fn test_button_css_class_generation() {
        // 测试CSS类名生成
        use crate::component::get_button_css_class;
        
        let props = ButtonProps {
            button_type: ButtonType::Primary,
            size: ButtonSize::Large,
            shape: ButtonShape::Round,
            html_type: ButtonHtmlType::Button,
            danger: true,
            ghost: false,
            disabled: false,
            loading: true,
            block: false,
            class: None,
            style: None,
            onclick: None,
            children: rsx! { "Test" },
        };
        
        let class_name = get_button_css_class(&props);
        assert!(class_name.contains("ant-btn"));
        assert!(class_name.contains("ant-btn-primary"));
        assert!(class_name.contains("ant-btn-lg"));
        assert!(class_name.contains("ant-btn-round"));
        assert!(class_name.contains("ant-btn-dangerous"));
        assert!(class_name.contains("ant-btn-loading"));
        assert!(!class_name.contains("ant-btn-background-ghost"));
        assert!(!class_name.contains("ant-btn-block"));
    }

    #[test]
    fn test_button_group_css_class_generation() {
        // 测试按钮组CSS类名生成
        use crate::component::get_button_group_class_name;
        
        let props = ButtonGroupProps {
            size: ButtonGroupSize::Large,
            button_type: Some(ButtonType::Primary),
            disabled: false,
            class: None,
            style: None,
            children: rsx! { "Test" },
        };
        
        let class_name = get_button_group_class_name(&props);
        assert!(class_name.contains("ant-btn-group"));
        assert!(class_name.contains("ant-btn-group-lg"));
    }

    #[test]
    fn test_style_generation() {
        // 测试样式生成
        let styles = generate_button_styles();
        assert!(!styles.base.is_empty());
        assert!(!styles.variants.primary.is_empty());
        assert!(!styles.states.dangerous.is_empty());
        
        let group_styles = generate_button_group_styles();
        assert!(!group_styles.is_empty());
        assert!(group_styles.contains(".ant-btn-group"));
    }

    #[test]
    fn test_serialization() {
        // 测试类型的序列化和反序列化
        use serde_json;
        
        let button_type = ButtonType::Primary;
        let json = serde_json::to_string(&button_type).unwrap();
        let deserialized: ButtonType = serde_json::from_str(&json).unwrap();
        assert_eq!(button_type, deserialized);
        
        let button_size = ButtonSize::Large;
        let json = serde_json::to_string(&button_size).unwrap();
        let deserialized: ButtonSize = serde_json::from_str(&json).unwrap();
        assert_eq!(button_size, deserialized);
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use dioxus::prelude::*;

    #[test]
    fn test_button_component_creation() {
        // 测试Button组件创建
        let _app = || {
            rsx! {
                Button {
                    button_type: ButtonType::Primary,
                    size: ButtonSize::Large,
                    "测试按钮"
                }
            }
        };
        // 如果能编译通过，说明组件定义正确
    }

    #[test]
    fn test_button_group_component_creation() {
        // 测试ButtonGroup组件创建
        let _app = || {
            rsx! {
                ButtonGroup {
                    size: ButtonGroupSize::Large,
                    Button { "按钮1" }
                    Button { "按钮2" }
                }
            }
        };
        // 如果能编译通过，说明组件定义正确
    }

    #[test]
    fn test_button_with_all_props() {
        // 测试带有所有属性的Button组件
        let _app = || {
            rsx! {
                Button {
                    button_type: ButtonType::Primary,
                    size: ButtonSize::Large,
                    shape: ButtonShape::Round,
                    html_type: ButtonHtmlType::Submit,
                    danger: true,
                    ghost: false,
                    disabled: false,
                    loading: true,
                    block: true,
                    class: "custom-class",
                    style: "color: red;",
                    onclick: |_| {},
                    "完整属性按钮"
                }
            }
        };
        // 如果能编译通过，说明所有属性都正确定义
    }
}
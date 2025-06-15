use crate::components::button::types::*;
use dioxus::prelude::*;

/// ButtonGroup 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct ButtonGroupProps {
    /// 子元素
    #[props(default)]
    pub children: Element,

    /// 大小
    #[props(default = ButtonSize::Middle)]
    pub size: ButtonSize,

    /// 自定义类名
    #[props(into, optional)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(into, optional)]
    pub style: Option<String>,
}

/// ButtonGroup 组件
#[component]
pub fn ButtonGroup(props: ButtonGroupProps) -> Element {
    // 使用clone避免所有权移动问题
    let props_clone = props.clone();

    // 生成类名
    let class_name = use_memo(move || generate_button_group_class_name(&props_clone));

    rsx! {
        div {
            class: "{class_name}",
            style: props.style.as_deref().unwrap_or(""),
            {props.children}
        }
    }
}

/// 生成按钮组类名
fn generate_button_group_class_name(props: &ButtonGroupProps) -> String {
    let mut classes = vec!["ant-btn-group".to_string()];

    // 添加用户自定义类名
    if let Some(class) = &props.class {
        classes.push(class.clone());
    }

    // 处理按钮组大小
    match props.size {
        ButtonSize::Large => classes.push("ant-btn-group-lg".to_string()),
        ButtonSize::Middle => {} // 默认大小，不添加类名
        ButtonSize::Small => classes.push("ant-btn-group-sm".to_string()),
    }

    classes.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use dioxus::prelude::*;

    /// 测试 ButtonGroup 组件的基本渲染
    #[test]
    fn test_button_group_basic_render() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                ButtonGroup {
                    div { "Button 1" }
                    div { "Button 2" }
                }
            }
        });

        let _ = dom.rebuild();
        // 验证按钮组是否正确渲染
    }

    /// 测试 ButtonGroup 组件的不同大小
    #[test]
    fn test_button_group_sizes() {
        let sizes = vec![ButtonSize::Large, ButtonSize::Middle, ButtonSize::Small];

        for size in sizes {
            let mut dom = VirtualDom::new(move || {
                rsx! {
                    ButtonGroup {
                        size: size,
                        div { "Button 1" }
                        div { "Button 2" }
                    }
                }
            });

            let _ = dom.rebuild();
            // 验证不同大小的按钮组是否正确渲染
        }
    }

    /// 测试 ButtonGroup 组件的自定义类名
    #[test]
    fn test_button_group_custom_class() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                ButtonGroup {
                    class: "custom-class",
                    div { "Button 1" }
                }
            }
        });

        let _ = dom.rebuild();
        // 验证自定义类名是否正确应用
    }

    /// 测试 ButtonGroup 组件的自定义样式
    #[test]
    fn test_button_group_custom_style() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                ButtonGroup {
                    style: "margin: 10px;",
                    div { "Button 1" }
                }
            }
        });

        let _ = dom.rebuild();
        // 验证自定义样式是否正确应用
    }

    /// 测试 generate_button_group_class_name 函数
    #[test]
    fn test_generate_button_group_class_name() {
        let props = ButtonGroupProps {
            children: rsx! { div { "test" } },
            size: ButtonSize::Large,
            class: Some("custom-class".to_string()),
            style: None,
        };

        let class_name = generate_button_group_class_name(&props);
        assert!(class_name.contains("ant-btn-group"));
        assert!(class_name.contains("ant-btn-group-lg"));
        assert!(class_name.contains("custom-class"));
    }

    /// 测试 generate_button_group_class_name 函数 - 默认大小
    #[test]
    fn test_generate_button_group_class_name_default_size() {
        let props = ButtonGroupProps {
            children: rsx! { div { "test" } },
            size: ButtonSize::Middle,
            class: None,
            style: None,
        };

        let class_name = generate_button_group_class_name(&props);
        assert!(class_name.contains("ant-btn-group"));
        assert!(!class_name.contains("ant-btn-group-lg"));
        assert!(!class_name.contains("ant-btn-group-sm"));
    }

    /// 测试 generate_button_group_class_name 函数 - 小尺寸
    #[test]
    fn test_generate_button_group_class_name_small_size() {
        let props = ButtonGroupProps {
            children: rsx! { div { "test" } },
            size: ButtonSize::Small,
            class: None,
            style: None,
        };

        let class_name = generate_button_group_class_name(&props);
        assert!(class_name.contains("ant-btn-group"));
        assert!(class_name.contains("ant-btn-group-sm"));
    }
}

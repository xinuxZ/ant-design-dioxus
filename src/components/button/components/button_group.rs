use crate::components::button::styles::button_group_styles;
use crate::components::button::types::*;
use dioxus::prelude::*;

/// ButtonGroup 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct ButtonGroupProps {
    /// 设置按钮大小
    #[props(default)]
    pub size: ButtonSize,

    /// 设置按钮类型
    #[props(into, default)]
    pub button_type: Option<ButtonType>,

    /// CSS 类名
    #[props(into, default)]
    pub class: Option<String>,

    /// 内联样式
    #[props(into, default)]
    pub style: Option<String>,

    /// 子元素
    #[props(default)]
    pub children: Element,
}

/// ButtonGroup 组件
#[component]
pub fn ButtonGroup(props: ButtonGroupProps) -> Element {
    // 注入样式
    // 使用 provide_context 替代 use_shared_state_provider
    provide_context(button_group_styles());

    // 使用 memo 优化类名生成，避免不必要的重新计算
    let class_name = use_memo(move || generate_button_group_class_name(&props));

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

    // 处理按钮大小
    match props.size {
        ButtonSize::Large => classes.push("ant-btn-group-lg".to_string()),
        ButtonSize::Middle => {} // 默认大小，不添加类名
        ButtonSize::Small => classes.push("ant-btn-group-sm".to_string()),
    }

    classes.join(" ")
}

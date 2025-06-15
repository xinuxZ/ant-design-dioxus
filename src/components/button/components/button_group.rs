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

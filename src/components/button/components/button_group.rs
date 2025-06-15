use crate::components::button::styles::button_group_styles;
use crate::components::button::types::*;
use dioxus::prelude::*;

/// ButtonGroup 属性
#[derive(Props, PartialEq, Clone)]
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
    use_shared_state_provider(|| button_group_styles());

    // 使用 memo 优化类名生成
    let class_name = use_memo(move || generate_group_class_name(&props));

    rsx! {
        div {
            class: "{class_name}",
            style: props.style.as_deref().unwrap_or(""),
            role: "group",
            "aria-label": "Button group",
            {props.children}
        }
    }
}

/// 生成按钮组类名
fn generate_group_class_name(props: &ButtonGroupProps) -> String {
    let mut classes = vec!["ant-btn-group".to_string()];

    // 添加用户自定义类名
    if let Some(class) = &props.class {
        classes.push(class.clone());
    }

    // 处理按钮大小
    match props.size {
        ButtonSize::Large => classes.push("ant-btn-group-large".to_string()),
        ButtonSize::Middle => {} // 默认大小，不添加类名
        ButtonSize::Small => classes.push("ant-btn-group-small".to_string()),
    }

    classes.join(" ")
}

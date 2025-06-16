use crate::components::button::styles::ButtonGroupStyleGenerator;
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
    let class_name = use_memo(move || {
        let size_str = match props_clone.size {
            ButtonSize::Large => "large",
            ButtonSize::Small => "small",
            ButtonSize::Middle => "middle",
        };
        
        // 生成样式（CSS-in-Rust会自动注入）
        let _style_generator = ButtonGroupStyleGenerator::new()
            .with_size(size_str);
        let _generated_styles = _style_generator.generate();
        
        let mut classes = vec!["ant-btn-group".to_string()];
        
        // 添加尺寸类名
        match props_clone.size {
            ButtonSize::Large => classes.push("ant-btn-group-lg".to_string()),
            ButtonSize::Small => classes.push("ant-btn-group-sm".to_string()),
            ButtonSize::Middle => {}, // 默认大小，不添加类名
        }
        
        // 添加用户自定义类名
        if let Some(class) = &props_clone.class {
            classes.push(class.clone());
        }
        
        classes.join(" ")
    });

    rsx! {
        div {
            class: "{class_name}",
            style: props.style.as_deref().unwrap_or(""),
            {props.children}
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_group_style_generator() {
        let generator = ButtonGroupStyleGenerator::new();
        let style = generator.generate();
        assert!(!style.is_empty());
    }

    #[test]
    fn test_button_group_style_generator_with_size() {
        let generator = ButtonGroupStyleGenerator::new().with_size("large");
        let style = generator.generate();
        assert!(!style.is_empty());
    }
}

//! BackTop 回到顶部组件
//!
//! 返回页面顶部的操作按钮。
//!
//! ## 何时使用
//!
//! - 当页面内容区域比较长时；
//! - 当用户需要频繁返回顶部查看相关内容时。

use dioxus::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, Element as WebElement, ScrollToOptions};

use crate::components::icon::Icon;
use crate::prelude::conditional_class_names_array;

/// BackTop 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct BackTopProps {
    /// 自定义内容，默认为向上箭头图标
    pub children: Option<Element>,
    /// 滚动高度达到此参数值才出现 BackTop
    #[props(default = 400)]
    pub visibility_height: i32,
    /// 设置需要监听其滚动事件的元素，值为一个返回对应 DOM 元素的函数
    pub target: Option<fn() -> Option<WebElement>>,
    /// 点击按钮的回调函数
    pub on_click: Option<EventHandler<()>>,
    /// 回到顶部所需时间（ms）
    #[props(default = 450)]
    pub duration: u32,
    /// 自定义样式类名
    #[props(default = String::new())]
    pub class: String,
    /// 自定义样式
    #[props(default = String::new())]
    pub style: String,
}

/// BackTop 回到顶部组件
#[component]
pub fn BackTop(props: BackTopProps) -> Element {
    let mut visible = use_signal(|| false);
    let mut back_top_ref = use_signal(|| None::<WebElement>);

    // 检查是否应该显示回到顶部按钮
    let check_visibility = use_callback(move |_| {
        let scroll_top = if let Some(target_fn) = props.target {
            if let Some(container) = target_fn() {
                container.scroll_top()
            } else {
                window().unwrap().page_y_offset().unwrap() as i32
            }
        } else {
            window().unwrap().page_y_offset().unwrap() as i32
        };

        let should_show = scroll_top >= props.visibility_height;
        if should_show != visible.read().clone() {
            visible.set(should_show);
        }
    });

    // 滚动到顶部
    let scroll_to_top = use_callback(move |_| {
        let options = ScrollToOptions::new();
        options.set_top(0.0);
        options.set_behavior(web_sys::ScrollBehavior::Smooth);

        if let Some(target_fn) = props.target {
            if let Some(container) = target_fn() {
                container.scroll_to_with_scroll_to_options(&options);
            }
        } else {
            window().unwrap().scroll_to_with_scroll_to_options(&options);
        }

        // 触发回调
        if let Some(callback) = &props.on_click {
            callback.call(());
        }
    });

    // 监听滚动事件
    use_effect(move || {
        let target_element = if let Some(target_fn) = props.target {
            target_fn()
        } else {
            window().unwrap().document().unwrap().document_element()
        };

        if let Some(target) = target_element {
            let check_visibility_clone = check_visibility.clone();
            let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |_: web_sys::Event| {
                check_visibility_clone(());
            }) as Box<dyn FnMut(_)>);

            target
                .add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref())
                .ok();

            // 初始检查
            check_visibility(());

            closure.forget();
        }
    });

    let back_top_class = conditional_class_names_array(&[
        ("ant-back-top", true),
        ("ant-back-top-show", visible.read().clone()),
        (&props.class, !props.class.is_empty()),
    ]);

    let back_top_style = format!(
        "position: fixed; right: 24px; bottom: 50px; z-index: 10; width: 40px; height: 40px; cursor: pointer; {}",
        props.style
    );

    rsx! {
        if visible.read().clone() {
            div {
                class: "{back_top_class}",
                style: "{back_top_style}",
                onclick: move |_| scroll_to_top(()),
                onmounted: move |evt| {
                    back_top_ref.set(Some(evt.data().downcast::<WebElement>().unwrap().clone()));
                },

                div { class: "ant-back-top-content",
                    if let Some(children) = &props.children {
                        {children.clone()}
                    } else {
                        div { class: "ant-back-top-icon",
                            Icon {
                                icon_type: "VerticalAlignTopOutlined",
                                style: "font-size: 16px; color: #fff;"
                            }
                        }
                    }
                }
            }
        }
    }
}

/// BackTop 组件的便捷构造函数
impl BackTopProps {
    /// 创建一个默认的 BackTop 组件
    pub fn default() -> Self {
        Self {
            children: None,
            visibility_height: 400,
            target: None,
            on_click: None,
            duration: 450,
            class: String::new(),
            style: String::new(),
        }
    }

    /// 设置可见高度
    pub fn with_visibility_height(mut self, height: i32) -> Self {
        self.visibility_height = height;
        self
    }

    /// 设置自定义内容
    pub fn with_children(mut self, children: Element) -> Self {
        self.children = Some(children);
        self
    }

    /// 设置目标容器
    pub fn with_target(mut self, target: fn() -> Option<WebElement>) -> Self {
        self.target = Some(target);
        self
    }

    /// 设置点击回调
    pub fn with_on_click(mut self, callback: EventHandler<()>) -> Self {
        self.on_click = Some(callback);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_back_top_props_default() {
        let props = BackTopProps::default();
        assert_eq!(props.visibility_height, 400);
        assert_eq!(props.duration, 450);
        assert!(props.children.is_none());
        assert!(props.target.is_none());
    }

    #[test]
    fn test_back_top_props_with_visibility_height() {
        let props = BackTopProps::default().with_visibility_height(200);
        assert_eq!(props.visibility_height, 200);
    }

    #[test]
    fn test_back_top_props_builder() {
        let props = BackTopProps::default().with_visibility_height(300);

        assert_eq!(props.visibility_height, 300);
        assert_eq!(props.duration, 450);
    }
}

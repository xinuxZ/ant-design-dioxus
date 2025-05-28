//! Affix 固钉组件
//!
//! 将页面元素钉在可视范围。
//!
//! ## 何时使用
//!
//! 当内容区域比较长，需要滚动页面时，这部分内容对应的操作或者导航需要在滚动过程中始终展现。常用于侧边菜单和按钮组合。
//! 页面可视范围过小时，慎用此功能以免遮挡页面内容。

use crate::utils::class_names::conditional_class_names_array;
use dioxus::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, Element as WebElement};

const AFFIX_STYLE: &str = include_str!("./style.css");

/// 固钉位置
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AffixPosition {
    /// 固定在顶部
    Top,
    /// 固定在底部
    Bottom,
}

impl Default for AffixPosition {
    fn default() -> Self {
        AffixPosition::Top
    }
}

/// Affix 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct AffixProps {
    /// 子组件
    pub children: Element,
    /// 距离窗口顶部达到指定偏移量后触发
    #[props(default = 0)]
    pub offset_top: i32,
    /// 距离窗口底部达到指定偏移量后触发
    pub offset_bottom: Option<i32>,
    /// 设置 Affix 需要监听其滚动事件的元素，值为一个返回对应 DOM 元素的函数
    pub target: Option<fn() -> Option<WebElement>>,
    /// 固定状态改变时触发的回调函数
    pub on_change: Option<EventHandler<bool>>,
    /// 自定义样式类名
    #[props(default = String::new())]
    pub class: String,
    /// 自定义样式
    #[props(default = String::new())]
    pub style: String,
}

/// Affix 固钉组件
#[component]
pub fn Affix(props: AffixProps) -> Element {
    let mut is_fixed = use_signal(|| false);
    let mut placeholder_height = use_signal(|| 0);
    let mut placeholder_width = use_signal(|| 0);
    let mut affix_ref = use_signal(|| None::<WebElement>);
    let mut placeholder_ref = use_signal(|| None::<WebElement>);

    let position = if props.offset_bottom.is_some() {
        AffixPosition::Bottom
    } else {
        AffixPosition::Top
    };

    // 计算是否需要固定
    let check_position = use_callback(move |_| {
        if let Some(element) = affix_ref.read().as_ref() {
            let rect = element.get_bounding_client_rect();
            let window_height = window().unwrap().inner_height().unwrap().as_f64().unwrap();

            let should_fix = match position {
                AffixPosition::Top => rect.top() <= props.offset_top as f64,
                AffixPosition::Bottom => {
                    if let Some(offset_bottom) = props.offset_bottom {
                        rect.bottom() >= window_height - offset_bottom as f64
                    } else {
                        false
                    }
                }
            };

            if should_fix != is_fixed.read().clone() {
                is_fixed.set(should_fix);

                // 设置占位符尺寸
                if should_fix {
                    placeholder_height.set(rect.height() as i32);
                    placeholder_width.set(rect.width() as i32);
                }

                // 触发回调
                if let Some(callback) = &props.on_change {
                    callback.call(should_fix);
                }
            }
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
            let check_position_clone = check_position.clone();
            let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |_: web_sys::Event| {
                check_position_clone(());
            }) as Box<dyn FnMut(_)>);

            target
                .add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref())
                .ok();
            window()
                .unwrap()
                .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
                .ok();

            // 初始检查
            check_position(());

            closure.forget();
        }
    });

    let affix_class = conditional_class_names_array(&[
        ("ant-affix", true),
        (&props.class, !props.class.is_empty()),
    ]);

    let affix_style = if is_fixed.read().clone() {
        let mut style = props.style.clone();
        match position {
            AffixPosition::Top => {
                style.push_str(&format!(
                    "position: fixed; top: {}px; z-index: 10;",
                    props.offset_top
                ));
            }
            AffixPosition::Bottom => {
                if let Some(offset_bottom) = props.offset_bottom {
                    style.push_str(&format!(
                        "position: fixed; bottom: {}px; z-index: 10;",
                        offset_bottom
                    ));
                }
            }
        }
        style
    } else {
        props.style.clone()
    };

    rsx! {
        style { {AFFIX_STYLE} }

        if is_fixed.read().clone() {
            div {
                class: "ant-affix-placeholder",
                style: format!("height: {}px; width: {}px;", placeholder_height.read(), placeholder_width.read()),
                onmounted: move |evt| {
                    placeholder_ref.set(Some(evt.data().downcast::<WebElement>().unwrap().clone()));
                }
            }
        }

        div {
            class: affix_class,
            style: affix_style,
            onmounted: move |evt| {
                affix_ref.set(Some(evt.data().downcast::<WebElement>().unwrap().clone()));
            },
            {props.children}
        }
    }
}

/// Affix 组件的便捷构造函数
impl AffixProps {
    /// 创建一个固定在顶部的 Affix
    pub fn top(offset: i32) -> Self {
        Self {
            children: rsx! { div {} },
            offset_top: offset,
            offset_bottom: None,
            target: None,
            on_change: None,
            class: String::new(),
            style: String::new(),
        }
    }

    /// 创建一个固定在底部的 Affix
    pub fn bottom(offset: i32) -> Self {
        Self {
            children: rsx! { div {} },
            offset_top: 0,
            offset_bottom: Some(offset),
            target: None,
            on_change: None,
            class: String::new(),
            style: String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_affix_position_default() {
        assert_eq!(AffixPosition::default(), AffixPosition::Top);
    }

    #[test]
    fn test_affix_props_top() {
        let props = AffixProps::top(10);
        assert_eq!(props.offset_top, 10);
        assert_eq!(props.offset_bottom, None);
    }

    #[test]
    fn test_affix_props_bottom() {
        let props = AffixProps::bottom(20);
        assert_eq!(props.offset_bottom, Some(20));
    }
}

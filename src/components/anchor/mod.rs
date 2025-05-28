//! Anchor 锚点组件
//!
//! 用于跳转到页面指定位置。
//!
//! ## 何时使用
//!
//! 需要展现当前页面上可供跳转的锚点链接，以及快速在锚点之间跳转。

use dioxus::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, Element as WebElement, ScrollToOptions};

use crate::utils::class_names::conditional_class_names_array;

/// 锚点链接项
#[derive(Debug, Clone, PartialEq)]
pub struct AnchorLink {
    /// 锚点链接地址
    pub href: String,
    /// 锚点链接文字
    pub title: String,
    /// 子锚点
    pub children: Vec<AnchorLink>,
}

impl AnchorLink {
    /// 创建新的锚点链接
    pub fn new(href: &str, title: &str) -> Self {
        Self {
            href: href.to_string(),
            title: title.to_string(),
            children: Vec::new(),
        }
    }

    /// 添加子锚点
    pub fn with_children(mut self, children: Vec<AnchorLink>) -> Self {
        self.children = children;
        self
    }
}

/// Anchor 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct AnchorProps {
    /// 锚点链接数据
    pub links: Vec<AnchorLink>,
    /// 距离窗口顶部达到指定偏移量后触发
    #[props(default = 0)]
    pub offset_top: i32,
    /// 设置监听容器
    pub target: Option<fn() -> Option<WebElement>>,
    /// 自定义高亮的锚点
    pub current_anchor: Option<String>,
    /// 点击锚点时触发
    pub on_click: Option<EventHandler<String>>,
    /// 监听锚点链接改变
    pub on_change: Option<EventHandler<String>>,
    /// 是否显示小圆点
    #[props(default = true)]
    pub show_ink_in_fixed: bool,
    /// 自定义样式类名
    #[props(default = String::new())]
    pub class: String,
    /// 自定义样式
    #[props(default = String::new())]
    pub style: String,
}

/// Anchor 锚点组件
#[component]
pub fn Anchor(props: AnchorProps) -> Element {
    let mut active_link = use_signal(|| props.current_anchor.clone().unwrap_or_default());
    let ink_top = use_signal(|| 0);
    let mut anchor_ref = use_signal(|| None::<WebElement>);

    // 滚动到指定锚点
    let scroll_to_anchor = use_callback(move |href: String| {
        if let Some(target_id) = href.strip_prefix('#') {
            if let Some(document) = window().unwrap().document() {
                if let Some(element) = document.get_element_by_id(target_id) {
                    if let Ok(html_element) = element.dyn_into::<web_sys::HtmlElement>() {
                        let options = ScrollToOptions::new();
                        options.set_top(html_element.offset_top() as f64 - props.offset_top as f64);
                        options.set_behavior(web_sys::ScrollBehavior::Smooth);

                        if let Some(target_fn) = props.target {
                            if let Some(container) = target_fn() {
                                container.scroll_to_with_scroll_to_options(&options);
                            }
                        } else {
                            window().unwrap().scroll_to_with_scroll_to_options(&options);
                        }
                    }
                }
            }
        }
    });

    // 处理锚点点击
    let handle_click = use_callback(move |href: String| {
        active_link.set(href.clone());
        scroll_to_anchor(href.clone());

        if let Some(callback) = &props.on_click {
            callback.call(href.clone());
        }

        if let Some(callback) = &props.on_change {
            callback.call(href);
        }
    });

    // 监听滚动事件，更新当前激活的锚点
    let update_active_link = use_callback({
        let links = props.links.clone();
        move |_| {
            if let Some(_document) = window().unwrap().document() {
                let scroll_top = if let Some(target_fn) = props.target {
                    if let Some(container) = target_fn() {
                        container.scroll_top()
                    } else {
                        window().unwrap().page_y_offset().unwrap() as i32
                    }
                } else {
                    window().unwrap().page_y_offset().unwrap() as i32
                };

                let mut current_anchor = String::new();
                let mut min_distance = i32::MAX;

                // 递归检查所有锚点
                fn check_links(
                    links: &[AnchorLink],
                    scroll_top: i32,
                    offset_top: i32,
                    current: &mut String,
                    min_dist: &mut i32,
                ) {
                    for link in links {
                        if let Some(target_id) = link.href.strip_prefix('#') {
                            if let Some(document) = window().unwrap().document() {
                                if let Some(element) = document.get_element_by_id(target_id) {
                                    if let Ok(html_element) =
                                        element.dyn_into::<web_sys::HtmlElement>()
                                    {
                                        let element_top = html_element.offset_top() - offset_top;
                                        let distance = (scroll_top - element_top).abs();

                                        if distance < *min_dist && scroll_top >= element_top - 10 {
                                            *min_dist = distance;
                                            *current = link.href.clone();
                                        }
                                    }
                                }
                            }
                        }

                        check_links(&link.children, scroll_top, offset_top, current, min_dist);
                    }
                }

                check_links(
                    &links,
                    scroll_top,
                    props.offset_top,
                    &mut current_anchor,
                    &mut min_distance,
                );

                if !current_anchor.is_empty() && current_anchor != active_link.read().clone() {
                    active_link.set(current_anchor.clone());

                    if let Some(callback) = &props.on_change {
                        callback.call(current_anchor);
                    }
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
            let update_active_link_clone = update_active_link.clone();
            let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |_: web_sys::Event| {
                update_active_link_clone(());
            }) as Box<dyn FnMut(_)>);

            target
                .add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref())
                .ok();

            // 初始检查
            update_active_link(());

            closure.forget();
        }
    });

    let anchor_class = conditional_class_names_array(&[
        ("ant-anchor", true),
        (&props.class, !props.class.is_empty()),
    ]);

    // 渲染锚点链接的递归函数
    fn render_anchor_links(
        links: &[AnchorLink],
        level: usize,
        active_link: String,
        handle_click: impl Fn(String) + Clone + 'static,
    ) -> Element {
        rsx! {
            for link in links {
                div {
                    class: conditional_class_names_array(&[
                        ("ant-anchor-link", true),
                        ("ant-anchor-link-active", active_link == link.href),
                    ]),
                    style: format!("padding-left: {}px;", level * 16),

                    a {
                        class: "ant-anchor-link-title",
                        href: "{link.href}",
                        onclick: {
                            let href = link.href.clone();
                            let handle_click = handle_click.clone();
                            move |evt: Event<MouseData>| {
                                evt.prevent_default();
                                handle_click(href.clone());
                            }
                        },
                        "{link.title}"
                    }

                    if !link.children.is_empty() {
                        {render_anchor_links(&link.children, level + 1, active_link.clone(), handle_click.clone())}
                    }
                }
            }
        }
    }

    rsx! {
        div {
            class: "{anchor_class}",
            style: "{props.style}",
            onmounted: move |evt| {
                anchor_ref.set(Some(evt.data().downcast::<WebElement>().unwrap().clone()));
            },

            div { class: "ant-anchor-wrapper",
                if props.show_ink_in_fixed {
                    div {
                        class: "ant-anchor-ink",
                        div {
                            class: "ant-anchor-ink-ball",
                            style: format!("top: {}px;", ink_top.read()),
                        }
                    }
                }

                div { class: "ant-anchor-content",
                    {render_anchor_links(&props.links, 0, active_link.read().clone(), move |href| handle_click.call(href))}
                }
            }
        }
    }
}

/// 锚点链接构建器
pub struct AnchorLinkBuilder {
    link: AnchorLink,
}

impl AnchorLinkBuilder {
    pub fn new(href: &str, title: &str) -> Self {
        Self {
            link: AnchorLink::new(href, title),
        }
    }

    pub fn with_children(mut self, children: Vec<AnchorLink>) -> Self {
        self.link.children = children;
        self
    }

    pub fn build(self) -> AnchorLink {
        self.link
    }
}

/// 便捷的锚点链接创建宏
#[macro_export]
macro_rules! anchor_link {
    ($href:expr, $title:expr) => {
        AnchorLink::new($href, $title)
    };
    ($href:expr, $title:expr, [$($children:expr),*]) => {
        AnchorLink::new($href, $title).with_children(vec![$($children),*])
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anchor_link_new() {
        let link = AnchorLink::new("#section1", "Section 1");
        assert_eq!(link.href, "#section1");
        assert_eq!(link.title, "Section 1");
        assert!(link.children.is_empty());
    }

    #[test]
    fn test_anchor_link_with_children() {
        let child = AnchorLink::new("#subsection", "Subsection");
        let parent = AnchorLink::new("#section", "Section").with_children(vec![child]);

        assert_eq!(parent.children.len(), 1);
        assert_eq!(parent.children[0].href, "#subsection");
    }

    #[test]
    fn test_anchor_link_builder() {
        let link = AnchorLinkBuilder::new("#test", "Test")
            .with_children(vec![AnchorLink::new("#child", "Child")])
            .build();

        assert_eq!(link.href, "#test");
        assert_eq!(link.title, "Test");
        assert_eq!(link.children.len(), 1);
    }
}

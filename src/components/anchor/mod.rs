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

mod styles;
use styles::generate_anchor_style;

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
    /// 是否使用暗色主题
    #[props(default = false)]
    pub dark_theme: bool,
    /// 是否使用紧凑主题
    #[props(default = false)]
    pub compact_theme: bool,
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
            window()
                .unwrap()
                .document()
                .and_then(|doc| doc.document_element())
        };

        if let Some(target) = target_element {
            let update_active_link_clone = update_active_link.clone();
            let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |_: web_sys::Event| {
                update_active_link_clone(());
            }) as Box<dyn FnMut(_)>);

            target
                .add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref())
                .unwrap();

            update_active_link(());

            let target_clone = target.clone();
            return move || {
                target_clone
                    .remove_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref())
                    .unwrap();
            };
        }

        || {}
    });

    // 更新墨水球位置
    use_effect(move || {
        if let Some(anchor_element) = anchor_ref.read().as_ref() {
            if let Some(active) = active_link.read().strip_prefix('#') {
                if let Some(document) = window().unwrap().document() {
                    if let Some(link_element) = document
                        .query_selector(&format!(
                            ".ant-anchor-link[href='#{}'] .ant-anchor-link-title",
                            active
                        ))
                        .ok()
                        .flatten()
                    {
                        if let Ok(html_element) = link_element.dyn_into::<web_sys::HtmlElement>() {
                            let top = html_element.offset_top()
                                - anchor_element.get_bounding_client_rect().top() as i32;
                            ink_top.set(top);
                        }
                    }
                }
            }
        }
    });

    // 生成样式
    let anchor_style = generate_anchor_style(props.dark_theme, props.compact_theme);

    // 构建类名
    let class = conditional_class_names_array(&[
        ("ant-anchor", true),
        (props.class.as_str(), !props.class.is_empty()),
        ("ant-theme-dark", props.dark_theme),
        ("ant-theme-compact", props.compact_theme),
    ]);

    rsx! {
        style { {anchor_style} }
        div {
            class: "ant-anchor-wrapper",
            style: props.style.clone(),
            div {
                class: class,
                ref_: move |el| anchor_ref.set(el),
                div {
                    class: "ant-anchor-ink",
                    span {
                        class: format!("ant-anchor-ink-ball {}", if props.show_ink_in_fixed { "visible" } else { "" }),
                        style: format!("top: {}px", ink_top.get()),
                    }
                }
                div {
                    class: "ant-anchor-content",
                    render_anchor_links(&props.links, 0, active_link.read().clone(), handle_click.clone())
                }
            }
        }
    }
}

// 递归渲染锚点链接
fn render_anchor_links(
    links: &[AnchorLink],
    level: usize,
    active_link: String,
    handle_click: impl Fn(String) + Clone + 'static,
) -> Element {
    rsx! {
        for link in links {
            div {
                class: format!(
                    "ant-anchor-link {}",
                    if active_link == link.href { "ant-anchor-link-active" } else { "" }
                ),
                a {
                    class: "ant-anchor-link-title",
                    href: link.href.clone(),
                    onclick: move |evt| {
                        evt.stop_propagation();
                        evt.prevent_default();
                        let href = link.href.clone();
                        handle_click(href);
                    },
                    title: link.title.clone(),
                    {link.title.clone()}
                }
                if !link.children.is_empty() {
                    render_anchor_links(&link.children, level + 1, active_link.clone(), handle_click.clone())
                }
            }
        }
    }
}

/// Anchor 链接构建器
pub struct AnchorLinkBuilder {
    link: AnchorLink,
}

impl AnchorLinkBuilder {
    /// 创建新的链接构建器
    pub fn new(href: &str, title: &str) -> Self {
        Self {
            link: AnchorLink::new(href, title),
        }
    }

    /// 添加子链接
    pub fn with_children(mut self, children: Vec<AnchorLink>) -> Self {
        self.link.children = children;
        self
    }

    /// 构建链接
    pub fn build(self) -> AnchorLink {
        self.link
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anchor_link_new() {
        let link = AnchorLink::new("#test", "Test Link");
        assert_eq!(link.href, "#test");
        assert_eq!(link.title, "Test Link");
        assert!(link.children.is_empty());
    }

    #[test]
    fn test_anchor_link_with_children() {
        let child = AnchorLink::new("#child", "Child Link");
        let link = AnchorLink::new("#test", "Test Link").with_children(vec![child.clone()]);
        assert_eq!(link.href, "#test");
        assert_eq!(link.title, "Test Link");
        assert_eq!(link.children.len(), 1);
        assert_eq!(link.children[0], child);
    }

    #[test]
    fn test_anchor_link_builder() {
        let child = AnchorLink::new("#child", "Child Link");
        let link = AnchorLinkBuilder::new("#test", "Test Link")
            .with_children(vec![child.clone()])
            .build();
        assert_eq!(link.href, "#test");
        assert_eq!(link.title, "Test Link");
        assert_eq!(link.children.len(), 1);
        assert_eq!(link.children[0], child);
    }
}

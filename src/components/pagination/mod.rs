//! Pagination 分页组件
//!
//! 采用分页的形式分隔长列表，每次只加载一个页面。
//!
//! ## 何时使用
//!
//! - 当加载/渲染所有数据将花费很多时间时；
//! - 可切换页码浏览数据。

use crate::utils::class_names::conditional_class_names_array;
use dioxus::events::Key;
use dioxus::prelude::*;

/// 分页大小选择器选项
#[derive(Debug, Clone, PartialEq)]
pub struct PageSizeOption {
    /// 页面大小
    pub size: usize,
    /// 显示标签
    pub label: String,
}

impl PageSizeOption {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            label: format!("{} / 页", size),
        }
    }

    pub fn with_label(size: usize, label: &str) -> Self {
        Self {
            size,
            label: label.to_string(),
        }
    }
}

/// 分页组件大小
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PaginationSize {
    /// 默认大小
    Default,
    /// 小尺寸
    Small,
}

impl Default for PaginationSize {
    fn default() -> Self {
        PaginationSize::Default
    }
}

/// Pagination 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct PaginationProps {
    /// 当前页数
    #[props(default = 1)]
    pub current: usize,
    /// 数据总数
    pub total: usize,
    /// 每页条数
    #[props(default = 10)]
    pub page_size: usize,
    /// 指定可以设置的页面大小选项
    pub page_size_options: Option<Vec<PageSizeOption>>,
    /// 是否可以改变 pageSize
    #[props(default = false)]
    pub show_size_changer: bool,
    /// 是否显示快速跳转至某页
    #[props(default = false)]
    pub show_quick_jumper: bool,
    /// 是否显示总数和当前数据顺序
    #[props(default = false)]
    pub show_total: bool,
    /// 当只有一页时是否隐藏分页器
    #[props(default = false)]
    pub hide_on_single_page: bool,
    /// 是否显示较少页面内容
    #[props(default = false)]
    pub simple: bool,
    /// 当为小尺寸分页时，是否显示快速跳转和页面大小选择器
    #[props(default = false)]
    pub show_less_items: bool,
    /// 是否禁用分页
    #[props(default = false)]
    pub disabled: bool,
    /// 分页器大小
    #[props(default = PaginationSize::Default)]
    pub size: PaginationSize,
    /// 页码改变的回调
    pub on_change: Option<EventHandler<usize>>,
    /// pageSize 变化的回调
    pub on_show_size_change: Option<EventHandler<(usize, usize)>>,
    /// 自定义总数显示格式
    pub show_total_render: Option<fn(usize, (usize, usize)) -> String>,
    /// 自定义样式类名
    #[props(default = String::new())]
    pub class: String,
    /// 自定义样式
    #[props(default = String::new())]
    pub style: String,
}

/// Pagination 分页组件
#[component]
pub fn Pagination(props: PaginationProps) -> Element {
    let mut current_page = use_signal(|| props.current);
    let mut current_page_size = use_signal(|| props.page_size);
    let mut jump_page = use_signal(|| String::new());

    // 计算总页数
    let total_pages =
        (props.total + current_page_size.read().clone() - 1) / current_page_size.read().clone();

    // 如果只有一页且设置了隐藏，则不显示分页器
    if props.hide_on_single_page && total_pages <= 1 {
        return rsx! { div {} };
    }

    // 处理页码变化
    let mut handle_page_change = move |page: usize| {
        if page != current_page.read().clone() && page >= 1 && page <= total_pages {
            current_page.set(page);
            if let Some(callback) = &props.on_change {
                callback.call(page);
            }
        }
    };

    // 处理页面大小变化
    let mut handle_page_size_change = move |size: usize| {
        if size != current_page_size.read().clone() {
            let _old_size = current_page_size.read().clone();
            current_page_size.set(size);

            // 重新计算当前页
            let new_total_pages = (props.total + size - 1) / size;
            let new_current = if current_page.read().clone() > new_total_pages {
                new_total_pages.max(1)
            } else {
                current_page.read().clone()
            };

            current_page.set(new_current);

            if let Some(callback) = &props.on_show_size_change {
                callback.call((new_current, size));
            }

            if new_current != current_page.read().clone() {
                if let Some(callback) = &props.on_change {
                    callback.call(new_current);
                }
            }
        }
    };

    // 处理快速跳转
    let mut handle_quick_jump = move |_| {
        let page_str = jump_page.read().clone();
        if let Ok(page) = page_str.parse::<usize>() {
            handle_page_change(page);
            jump_page.set(String::new());
        }
    };

    // 生成页码按钮
    let generate_page_items = || -> Vec<Element> {
        let mut items = Vec::new();
        let current = current_page.read().clone();

        if props.simple {
            // 简单模式
            items.push(rsx! {
                span { class: "ant-pagination-simple-pager",
                    input {
                        r#type: "number",
                        value: "{current}",
                        min: "1",
                        max: "{total_pages}",
                        onchange: move |evt| {
                            if let Ok(page) = evt.value().parse::<usize>() {
                                handle_page_change(page);
                            }
                        }
                    }
                    span { class: "ant-pagination-slash", "/" }
                    span { "{total_pages}" }
                }
            });
        } else {
            // 标准模式
            let start_page = if current <= 3 {
                1
            } else if current >= total_pages - 2 {
                (total_pages - 4).max(1)
            } else {
                current - 2
            };

            let end_page = (start_page + 4).min(total_pages);

            // 首页
            if start_page > 1 {
                items.push(rsx! {
                    li {
                        class: conditional_class_names_array(&[
                            ("ant-pagination-item", true),
                            ("ant-pagination-item-1", true),
                        ]),
                        onclick: move |_| handle_page_change(1),
                        a { "1" }
                    }
                });

                if start_page > 2 {
                    items.push(rsx! {
                        li { class: "ant-pagination-jump-prev",
                            onclick: move |_| handle_page_change((current - 5).max(1)),
                            a { "•••" }
                        }
                    });
                }
            }

            // 中间页码
            for page in start_page..=end_page {
                items.push(rsx! {
                    li {
                        class: conditional_class_names_array(&[
                            ("ant-pagination-item", true),
                            (&format!("ant-pagination-item-{}", page), true),
                            ("ant-pagination-item-active", page == current),
                        ]),
                        onclick: move |_| handle_page_change(page),
                        a { "{page}" }
                    }
                });
            }

            // 尾页
            if end_page < total_pages {
                if end_page < total_pages - 1 {
                    items.push(rsx! {
                        li { class: "ant-pagination-jump-next",
                            onclick: move |_| handle_page_change((current + 5).min(total_pages)),
                            a { "•••" }
                        }
                    });
                }

                items.push(rsx! {
                    li {
                        class: conditional_class_names_array(&[
                            ("ant-pagination-item", true),
                            (&format!("ant-pagination-item-{}", total_pages), true),
                        ]),
                        onclick: move |_| handle_page_change(total_pages),
                        a { "{total_pages}" }
                    }
                });
            }
        }

        items
    };

    let pagination_class = conditional_class_names_array(&[
        ("ant-pagination", true),
        ("ant-pagination-simple", props.simple),
        ("ant-pagination-disabled", props.disabled),
        ("ant-pagination-mini", props.size == PaginationSize::Small),
        (&props.class, !props.class.is_empty()),
    ]);

    let current = current_page.read().clone();
    let page_size = current_page_size.read().clone();
    let start_index = (current - 1) * page_size + 1;
    let end_index = (current * page_size).min(props.total);

    rsx! {
        ul {
            class: "{pagination_class}",
            style: "{props.style}",

            // 总数显示
            if props.show_total {
                li { class: "ant-pagination-total-text",
                    if let Some(render_fn) = props.show_total_render {
                        "{render_fn(props.total, (start_index, end_index))}"
                    } else {
                        "共 {props.total} 条"
                    }
                }
            }

            // 上一页按钮
            li {
                class: conditional_class_names_array(&[
                    ("ant-pagination-prev", true),
                    ("ant-pagination-disabled", current <= 1 || props.disabled),
                ]),
                onclick: move |_| {
                    if current > 1 && !props.disabled {
                        handle_page_change(current - 1);
                    }
                },
                a { "‹" }
            }

            // 页码
            for item in generate_page_items() {
                {item}
            }

            // 下一页按钮
            li {
                class: conditional_class_names_array(&[
                    ("ant-pagination-next", true),
                    ("ant-pagination-disabled", current >= total_pages || props.disabled),
                ]),
                onclick: move |_| {
                    if current < total_pages && !props.disabled {
                        handle_page_change(current + 1);
                    }
                },
                a { "›" }
            }

            // 页面大小选择器
            if props.show_size_changer {
                if let Some(options) = &props.page_size_options {
                    li { class: "ant-pagination-options",
                        div { class: "ant-pagination-options-size-changer",
                            select {
                                value: "{page_size}",
                                onchange: move |evt| {
                                    if let Ok(size) = evt.value().parse::<usize>() {
                                        handle_page_size_change(size);
                                    }
                                },

                                for option in options {
                                    option {
                                        value: "{option.size}",
                                        selected: option.size == page_size,
                                        "{option.label}"
                                    }
                                }
                            }
                        }
                    }
                } else {
                    li { class: "ant-pagination-options",
                        div { class: "ant-pagination-options-size-changer",
                            select {
                                value: "{page_size}",
                                onchange: move |evt| {
                                    if let Ok(size) = evt.value().parse::<usize>() {
                                        handle_page_size_change(size);
                                    }
                                },

                                option { value: "10", "10 / 页" }
                                option { value: "20", "20 / 页" }
                                option { value: "50", "50 / 页" }
                                option { value: "100", "100 / 页" }
                            }
                        }
                    }
                }
            }

            // 快速跳转
            if props.show_quick_jumper {
                li { class: "ant-pagination-options",
                    div { class: "ant-pagination-options-quick-jumper",
                        "跳至"
                        input {
                            r#type: "number",
                            min: "1",
                            max: "{total_pages}",
                            value: "{jump_page.read()}",
                            oninput: move |evt| jump_page.set(evt.value()),
                            onkeypress: move |evt| {
                                if evt.key() == Key::Enter {
                                    handle_quick_jump(());
                                }
                            }
                        }
                        "页"
                    }
                }
            }
        }
    }
}

/// 分页大小选项构建器
pub struct PageSizeOptionsBuilder {
    options: Vec<PageSizeOption>,
}

impl PageSizeOptionsBuilder {
    pub fn new() -> Self {
        Self {
            options: Vec::new(),
        }
    }

    pub fn add_option(mut self, size: usize) -> Self {
        self.options.push(PageSizeOption::new(size));
        self
    }

    pub fn add_option_with_label(mut self, size: usize, label: &str) -> Self {
        self.options.push(PageSizeOption::with_label(size, label));
        self
    }

    pub fn build(self) -> Vec<PageSizeOption> {
        self.options
    }
}

impl Default for PageSizeOptionsBuilder {
    fn default() -> Self {
        Self::new()
            .add_option(10)
            .add_option(20)
            .add_option(50)
            .add_option(100)
    }
}

/// 便捷的分页大小选项创建宏
#[macro_export]
macro_rules! page_size_options {
    [$($size:expr),*] => {
        vec![$(
            PageSizeOption::new($size)
        ),*]
    };
    [$($size:expr => $label:expr),*] => {
        vec![$(
            PageSizeOption::with_label($size, $label)
        ),*]
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_size_option_new() {
        let option = PageSizeOption::new(10);
        assert_eq!(option.size, 10);
        assert_eq!(option.label, "10 / 页");
    }

    #[test]
    fn test_page_size_option_with_label() {
        let option = PageSizeOption::with_label(20, "每页20条");
        assert_eq!(option.size, 20);
        assert_eq!(option.label, "每页20条");
    }

    #[test]
    fn test_pagination_size_default() {
        assert_eq!(PaginationSize::default(), PaginationSize::Default);
    }

    #[test]
    fn test_page_size_options_builder() {
        let options = PageSizeOptionsBuilder::new()
            .add_option(10)
            .add_option_with_label(20, "每页20条")
            .build();

        assert_eq!(options.len(), 2);
        assert_eq!(options[0].size, 10);
        assert_eq!(options[1].size, 20);
        assert_eq!(options[1].label, "每页20条");
    }
}

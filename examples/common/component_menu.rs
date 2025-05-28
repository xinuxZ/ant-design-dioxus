//! 组件菜单组件
//!
//! 提供组件分类导航菜单

use dioxus::prelude::*;

/// 组件菜单属性
#[derive(Props, Clone, PartialEq)]
pub struct ComponentMenuProps {
    /// 当前选中的组件
    pub selected: String,
    /// 选择事件处理器
    pub on_select: EventHandler<String>,
}

/// 组件菜单
///
/// 按分类展示所有可用的组件，支持点击切换
#[component]
pub fn ComponentMenu(props: ComponentMenuProps) -> Element {
    let components = vec![
        ("通用", vec!["Button", "Icon", "Typography"]),
        ("布局", vec!["Grid", "Layout", "Space"]),
        ("导航", vec!["Menu", "Breadcrumb", "Pagination"]),
        ("数据录入", vec!["Input", "Select", "Form"]),
        ("数据展示", vec!["Table", "List", "Card"]),
        ("反馈", vec!["Alert", "Message", "Modal"]),
    ];

    rsx! {
        div {
            class: "component-menu",

            for (category, items) in components {
                div {
                    class: "menu-category",
                    style: "margin-bottom: 16px;",

                    h4 {
                        style: "margin: 0 0 8px 0; color: #666; font-size: 12px; text-transform: uppercase;",
                        "{category}"
                    }

                    for item in items {
                        div {
                            class: "menu-item",
                            style: format!(
                                "padding: 8px 12px; cursor: pointer; border-radius: 4px; margin-bottom: 2px; transition: background-color 0.2s; {}",
                                if *item == props.selected { "background: #1890ff; color: white;" } else { "color: #333; hover:background: #e6f7ff;" }
                            ),
                            onclick: move |_| {
                                props.on_select.call(item.to_string());
                            },
                            "{item}"
                        }
                    }
                }
            }
        }
    }
}

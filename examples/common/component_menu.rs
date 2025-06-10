//! 组件菜单
//!
//! 提供组件选择菜单功能

use dioxus::prelude::*;

/// 组件菜单属性
#[derive(Props, Clone, PartialEq)]
pub struct ComponentMenuProps {
    /// 当前选中的组件
    pub selected: String,
    /// 选择组件时的回调
    pub on_select: EventHandler<String>,
}

/// 组件菜单
#[component]
pub fn ComponentMenu(props: ComponentMenuProps) -> Element {
    let components = vec![
        ("通用", vec!["Button", "Icon", "Typography", "ThemeSwitch"]),
        ("布局", vec!["Grid", "Layout", "Flex", "Space", "Divider"]),
        (
            "表单",
            vec![
                "Form",
                "AutoComplete",
                "Cascader",
                "Checkbox",
                "ColorPicker",
                "DatePicker",
                "Input",
                "InputNumber",
                "Mentions",
                "Radio",
                "Rate",
                "Select",
                "Slider",
                "Switch",
                "TimePicker",
                "Transfer",
                "TreeSelect",
                "Upload",
            ],
        ),
        (
            "导航",
            vec![
                "Affix",
                "Anchor",
                "BackTop",
                "Breadcrumb",
                "Dropdown",
                "Pagination",
                "Menu",
                "Steps",
            ],
        ),
        (
            "其他",
            vec![
                "App",
                "Calendar",
                "FloatButton",
                "I18nDemo",
                "QRCode",
                "Segmented",
                "Splitter",
                "Watermark",
            ],
        ),
        (
            "数据展示",
            vec![
                "Avatar",
                "Badge",
                "Card",
                "Carousel",
                "Collapse",
                "Descriptions",
                "Empty",
                "Image",
                "List",
                "Popover",
                "Statistic",
                "Timeline",
                "Table",
                "Tabs",
                "Tag",
                "Tooltip",
                "Tree",
            ],
        ),
        (
            "反馈",
            vec![
                "Alert",
                "Progress",
                "Spin",
                "Drawer",
                "Message",
                "Modal",
                "Notification",
                "Popconfirm",
                "Result",
                "Skeleton",
                "Tour",
            ],
        ),
    ];
    /*

    */
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

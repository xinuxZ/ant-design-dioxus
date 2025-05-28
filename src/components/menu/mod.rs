//! Menu 导航菜单组件
//!
//! 为页面和功能提供导航的菜单列表。
//!
//! ## 何时使用
//!
//! 导航菜单是一个网站的灵魂，用户依赖导航在各个页面中进行跳转。一般分为顶部导航和侧边导航，顶部导航提供全局性的类目和功能，侧边导航提供多级结构来收纳和排列网站架构。
//!
//! ## 代码演示
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::{Menu, MenuItem, SubMenu};
//!
//! #[component]
//! fn App() -> Element {
//!     rsx! {
//!         Menu {
//!             mode: MenuMode::Horizontal,
//!             MenuItem {
//!                 key: "mail",
//!                 icon: "📧",
//!                 "Navigation One"
//!             }
//!             SubMenu {
//!                 key: "sub1",
//!                 title: "Navigation Two",
//!                 icon: "📁",
//!                 MenuItem {
//!                     key: "1",
//!                     "Option 1"
//!                 }
//!                 MenuItem {
//!                     key: "2",
//!                     "Option 2"
//!                 }
//!             }
//!         }
//!     }
//! }
//! ```

use dioxus::prelude::*;
use std::collections::HashSet;

/// 菜单模式
#[derive(Debug, Clone, PartialEq)]
pub enum MenuMode {
    /// 垂直菜单
    Vertical,
    /// 水平菜单
    Horizontal,
    /// 内嵌菜单
    Inline,
}

/// 菜单主题
#[derive(Debug, Clone, PartialEq)]
pub enum MenuTheme {
    /// 亮色主题
    Light,
    /// 暗色主题
    Dark,
}

/// 菜单项数据结构
#[derive(Debug, Clone, PartialEq)]
pub struct MenuItemData {
    /// 菜单项的唯一标识
    pub key: String,
    /// 菜单项标题
    pub title: String,
    /// 菜单项图标
    pub icon: Option<String>,
    /// 是否禁用
    pub disabled: bool,
    /// 链接地址
    pub href: Option<String>,
    /// 子菜单项
    pub children: Option<Vec<MenuItemData>>,
    /// 是否为分组
    pub is_group: bool,
    /// 分组标题
    pub group_title: Option<String>,
}

/// MenuItem 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct MenuItemProps {
    /// 菜单项的唯一标识
    pub menu_key: String,
    /// 菜单项图标
    pub icon: Option<String>,
    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,
    /// 链接地址
    pub href: Option<String>,
    /// 点击事件回调
    pub onclick: Option<EventHandler<String>>,
    /// 自定义样式类名
    #[props(default = String::new())]
    pub class: String,
    /// 自定义内联样式
    #[props(default = String::new())]
    pub style: String,
    /// 子元素
    pub children: Element,
}

/// SubMenu 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct SubMenuProps {
    /// 子菜单的唯一标识
    pub menu_key: String,
    /// 子菜单标题
    pub title: String,
    /// 子菜单图标
    pub icon: Option<String>,
    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,
    /// 弹出菜单的位置
    #[props(default = "bottomLeft".to_string())]
    pub popup_placement: String,
    /// 自定义样式类名
    #[props(default = String::new())]
    pub class: String,
    /// 自定义内联样式
    #[props(default = String::new())]
    pub style: String,
    /// 子元素
    pub children: Element,
}

/// MenuGroup 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct MenuGroupProps {
    /// 分组的唯一标识
    pub menu_key: String,
    /// 分组标题
    pub title: String,
    /// 自定义样式类名
    #[props(default = String::new())]
    pub class: String,
    /// 自定义内联样式
    #[props(default = String::new())]
    pub style: String,
    /// 子元素
    pub children: Element,
}

/// Menu 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct MenuProps {
    /// 菜单数据
    #[props(default = vec![])]
    pub items: Vec<MenuItemData>,
    /// 菜单类型
    #[props(default = MenuMode::Vertical)]
    pub mode: MenuMode,
    /// 主题颜色
    #[props(default = MenuTheme::Light)]
    pub theme: MenuTheme,
    /// 是否允许多选
    #[props(default = false)]
    pub multiple: bool,
    /// 是否允许选中
    #[props(default = true)]
    pub selectable: bool,
    /// 当前选中的菜单项 key 数组
    #[props(default = vec![])]
    pub selected_keys: Vec<String>,
    /// 初始选中的菜单项 key 数组
    #[props(default = vec![])]
    pub default_selected_keys: Vec<String>,
    /// 当前展开的 SubMenu 菜单项 key 数组
    #[props(default = vec![])]
    pub open_keys: Vec<String>,
    /// 初始展开的 SubMenu 菜单项 key 数组
    #[props(default = vec![])]
    pub default_open_keys: Vec<String>,
    /// 是否收起当前展开的 SubMenu
    #[props(default = true)]
    pub accordion: bool,
    /// 用户点击菜单项时的回调
    pub on_click: Option<EventHandler<MenuItemData>>,
    /// 被选中时调用
    pub on_select: Option<EventHandler<(Vec<String>, MenuItemData)>>,
    /// 取消选中时调用
    pub on_deselect: Option<EventHandler<(Vec<String>, MenuItemData)>>,
    /// SubMenu 展开/关闭的回调
    pub on_open_change: Option<EventHandler<Vec<String>>>,
    /// 自定义样式类名
    #[props(default = String::new())]
    pub class: String,
    /// 自定义内联样式
    #[props(default = String::new())]
    pub style: String,
    /// 子元素
    pub children: Element,
}

/// MenuItem 菜单项组件
#[component]
pub fn MenuItem(props: MenuItemProps) -> Element {
    let handle_click = {
        let menu_key = props.menu_key.clone();
        let disabled = props.disabled;
        let onclick = props.onclick.clone();
        move |_| {
            if !disabled {
                if let Some(handler) = &onclick {
                    handler.call(menu_key.clone());
                }
            }
        }
    };

    rsx! {
        li {
            class: format!(
                "ant-menu-item {} {}",
                if props.disabled { "ant-menu-item-disabled" } else { "" },
                props.class
            ),
            style: props.style.clone(),
            role: "menuitem",
            "data-menu-id": props.menu_key.clone(),
            onclick: handle_click,

            if let Some(href) = &props.href {
                a {
                    href: href.clone(),
                    class: "ant-menu-item-link",

                    if let Some(icon) = &props.icon {
                        span {
                            class: "ant-menu-item-icon",
                            {icon.clone()}
                        }
                    }

                    span {
                        class: "ant-menu-title-content",
                        {props.children}
                    }
                }
            } else {
                if let Some(icon) = &props.icon {
                    span {
                        class: "ant-menu-item-icon",
                        {icon.clone()}
                    }
                }

                span {
                    class: "ant-menu-title-content",
                    {props.children}
                }
            }
        }
    }
}

/// SubMenu 子菜单组件
#[component]
pub fn SubMenu(props: SubMenuProps) -> Element {
    let mut is_open = use_signal(|| false);

    let toggle_submenu = move |_| {
        if !props.disabled {
            is_open.set(!is_open());
        }
    };

    rsx! {
        li {
            class: format!(
                "ant-menu-submenu {} {} {}",
                if is_open() { "ant-menu-submenu-open" } else { "" },
                if props.disabled { "ant-menu-submenu-disabled" } else { "" },
                props.class
            ),
            style: props.style.clone(),
            role: "menuitem",
            "data-menu-id": props.menu_key.clone(),

            // 子菜单标题
            div {
                class: "ant-menu-submenu-title",
                onclick: toggle_submenu,

                if let Some(icon) = &props.icon {
                    span {
                        class: "ant-menu-item-icon",
                        {icon.clone()}
                    }
                }

                span {
                    class: "ant-menu-title-content",
                    {props.title.clone()}
                }

                span {
                    class: format!(
                        "ant-menu-submenu-arrow {}",
                        if is_open() { "ant-menu-submenu-arrow-open" } else { "" }
                    ),
                    "▶"
                }
            }

            // 子菜单内容
            if is_open() {
                ul {
                    class: "ant-menu ant-menu-sub ant-menu-inline",
                    role: "menu",
                    {props.children}
                }
            }
        }
    }
}

/// MenuGroup 菜单分组组件
#[component]
pub fn MenuGroup(props: MenuGroupProps) -> Element {
    rsx! {
        li {
            class: format!("ant-menu-item-group {}", props.class),
            style: props.style.clone(),
            role: "presentation",

            div {
                class: "ant-menu-item-group-title",
                {props.title.clone()}
            }

            ul {
                class: "ant-menu-item-group-list",
                {props.children}
            }
        }
    }
}

/// Menu 导航菜单组件
///
/// 为页面和功能提供导航的菜单列表。
#[component]
pub fn Menu(props: MenuProps) -> Element {
    let mut selected_keys = use_signal(|| {
        if !props.selected_keys.is_empty() {
            props
                .selected_keys
                .iter()
                .cloned()
                .collect::<HashSet<String>>()
        } else {
            props
                .default_selected_keys
                .iter()
                .cloned()
                .collect::<HashSet<String>>()
        }
    });

    let open_keys = use_signal(|| {
        if !props.open_keys.is_empty() {
            props.open_keys.iter().cloned().collect::<HashSet<String>>()
        } else {
            props
                .default_open_keys
                .iter()
                .cloned()
                .collect::<HashSet<String>>()
        }
    });

    // 处理菜单项点击
    let handle_menu_click = move |item: MenuItemData| {
        if !props.selectable || item.disabled {
            return;
        }

        let mut current_selected = selected_keys();
        let key = item.key.clone();

        if props.multiple {
            if current_selected.contains(&key) {
                current_selected.remove(&key);
                if let Some(handler) = &props.on_deselect {
                    handler.call((current_selected.iter().cloned().collect(), item.clone()));
                }
            } else {
                current_selected.insert(key.clone());
                if let Some(handler) = &props.on_select {
                    handler.call((current_selected.iter().cloned().collect(), item.clone()));
                }
            }
        } else {
            current_selected.clear();
            current_selected.insert(key.clone());
            if let Some(handler) = &props.on_select {
                handler.call((current_selected.iter().cloned().collect(), item.clone()));
            }
        }

        selected_keys.set(current_selected);

        if let Some(handler) = &props.on_click {
            handler.call(item);
        }
    };

    // 渲染菜单项的辅助函数
    let render_menu_items = {
        let _selected_keys = selected_keys.clone();
        let _open_keys = open_keys.clone();
        let handle_menu_click = handle_menu_click.clone();
        move |items: &Vec<MenuItemData>| -> Vec<Element> {
            items.iter().map(|item| {
                if item.is_group {
                    rsx! {
                        MenuGroup {
                            menu_key: item.key.clone(),
                            title: item.group_title.clone().unwrap_or_default(),

                            if let Some(children) = &item.children {
                                for child_item in children {
                                    if child_item.children.is_some() {
                                        SubMenu {
                                            menu_key: child_item.key.clone(),
                                            title: child_item.title.clone(),
                                            icon: child_item.icon.clone(),
                                            disabled: child_item.disabled,
                                        }
                                    } else {
                                        MenuItem {
                                            menu_key: child_item.key.clone(),
                                            icon: child_item.icon.clone(),
                                            disabled: child_item.disabled,
                                            href: child_item.href.clone(),
                                            onclick: {
                                                let item = child_item.clone();
                                                let mut handle_menu_click = handle_menu_click.clone();
                                                move |_| {
                                                    handle_menu_click(item.clone());
                                                }
                                            },
                                            {child_item.title.clone()}
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else if item.children.is_some() {
                    rsx! {
                        SubMenu {
                            menu_key: item.key.clone(),
                            title: item.title.clone(),
                            icon: item.icon.clone(),
                            disabled: item.disabled,
                        }
                    }
                } else {
                    rsx! {
                        MenuItem {
                            menu_key: item.key.clone(),
                            icon: item.icon.clone(),
                            disabled: item.disabled,
                            href: item.href.clone(),
                            onclick: {
                                let item = item.clone();
                                let mut handle_menu_click = handle_menu_click.clone();
                                move |_| {
                                    handle_menu_click(item.clone());
                                }
                            },
                            {item.title.clone()}
                        }
                    }
                }
            }).collect()
        }
    };

    let mode_class = match props.mode {
        MenuMode::Horizontal => "ant-menu-horizontal",
        MenuMode::Vertical => "ant-menu-vertical",
        MenuMode::Inline => "ant-menu-inline",
    };

    let theme_class = match props.theme {
        MenuTheme::Light => "ant-menu-light",
        MenuTheme::Dark => "ant-menu-dark",
    };

    rsx! {
        ul {
            class: format!(
                "ant-menu ant-menu-root {} {} {}",
                mode_class,
                theme_class,
                props.class
            ),
            style: props.style.clone(),
            role: "menu",

            // 如果有 items 数据，使用数据渲染，否则使用子元素
            if !props.items.is_empty() {
                for element in render_menu_items(&props.items) {
                    {element}
                }
            } else {
                {props.children}
            }
        }
    }
}

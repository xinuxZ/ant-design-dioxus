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

mod styles;

use dioxus::prelude::*;
use std::collections::HashSet;

use self::styles::{generate_menu_style, MenuMode as StyleMenuMode, MenuTheme as StyleMenuTheme};

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

impl From<MenuMode> for StyleMenuMode {
    fn from(mode: MenuMode) -> Self {
        match mode {
            MenuMode::Vertical => StyleMenuMode::Vertical,
            MenuMode::Horizontal => StyleMenuMode::Horizontal,
            MenuMode::Inline => StyleMenuMode::Inline,
        }
    }
}

/// 菜单主题
#[derive(Debug, Clone, PartialEq)]
pub enum MenuTheme {
    /// 亮色主题
    Light,
    /// 暗色主题
    Dark,
}

impl From<MenuTheme> for StyleMenuTheme {
    fn from(theme: MenuTheme) -> Self {
        match theme {
            MenuTheme::Light => StyleMenuTheme::Light,
            MenuTheme::Dark => StyleMenuTheme::Dark,
        }
    }
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
    /// 是否内联折叠
    #[props(default = false)]
    pub inline_collapsed: bool,
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
    let menu_ctx = use_context::<MenuContext>();
    let is_selected = menu_ctx.selected_keys.contains(&props.menu_key);

    let handle_click = move |_| {
        if !props.disabled {
            if let Some(handler) = &props.onclick {
                handler.call(props.menu_key.clone());
            }

            let item_data = MenuItemData {
                key: props.menu_key.clone(),
                title: props.children.to_string(),
                icon: props.icon.clone(),
                disabled: props.disabled,
                href: props.href.clone(),
                children: None,
                is_group: false,
                group_title: None,
            };

            menu_ctx.on_click.call(item_data.clone());

            if menu_ctx.selectable {
                let mut new_selected_keys = menu_ctx.selected_keys.clone();

                if !menu_ctx.multiple {
                    new_selected_keys.clear();
                }

                if new_selected_keys.contains(&props.menu_key) {
                    new_selected_keys.retain(|k| k != &props.menu_key);
                    menu_ctx
                        .on_deselect
                        .call((new_selected_keys.clone(), item_data.clone()));
                } else {
                    new_selected_keys.push(props.menu_key.clone());
                    menu_ctx
                        .on_select
                        .call((new_selected_keys.clone(), item_data.clone()));
                }

                menu_ctx.set_selected_keys(new_selected_keys);
            }
        }
    };

    let classes = format!(
        "ant-menu-item {} {} {}",
        if is_selected {
            "ant-menu-item-selected"
        } else {
            ""
        },
        if props.disabled {
            "ant-menu-item-disabled"
        } else {
            ""
        },
        props.class
    );

    match &props.href {
        Some(href) => rsx! {
            li {
                class: "{classes}",
                style: "{props.style}",
                onclick: handle_click,
                a {
                    href: "{href}",
                    if let Some(icon) = &props.icon {
                        span {
                            class: "ant-menu-item-icon",
                            "{icon}"
                        }
                    }
                    span {
                        {props.children}
                    }
                }
            }
        },
        None => rsx! {
            li {
                class: "{classes}",
                style: "{props.style}",
                onclick: handle_click,
                if let Some(icon) = &props.icon {
                    span {
                        class: "ant-menu-item-icon",
                        "{icon}"
                    }
                }
                span {
                    {props.children}
                }
            }
        },
    }
}

/// SubMenu 子菜单组件
#[component]
pub fn SubMenu(props: SubMenuProps) -> Element {
    let menu_ctx = use_context::<MenuContext>();
    let is_open = use_state(|| menu_ctx.open_keys.contains(&props.menu_key));
    let is_inline = menu_ctx.mode == MenuMode::Inline;

    // 处理子菜单展开/关闭
    let toggle_submenu = move |_| {
        if !props.disabled {
            let mut new_open_keys = menu_ctx.open_keys.clone();

            if is_open.get() {
                new_open_keys.retain(|k| k != &props.menu_key);
            } else {
                if menu_ctx.accordion {
                    new_open_keys.clear();
                }
                new_open_keys.push(props.menu_key.clone());
            }

            is_open.set(!is_open.get());
            menu_ctx.set_open_keys(new_open_keys.clone());
            menu_ctx.on_open_change.call(new_open_keys);
        }
    };

    let submenu_class = format!(
        "ant-menu-submenu {} {} {}",
        if *is_open.get() {
            "ant-menu-submenu-open"
        } else {
            ""
        },
        if props.disabled {
            "ant-menu-submenu-disabled"
        } else {
            ""
        },
        props.class
    );

    rsx! {
        li {
            class: "{submenu_class}",
            style: "{props.style}",
            div {
                class: "ant-menu-submenu-title",
                onclick: toggle_submenu,
                if let Some(icon) = &props.icon {
                    span {
                        class: "ant-menu-item-icon",
                        "{icon}"
                    }
                }
                span {
                    "{props.title}"
                }
                span {
                    class: "ant-menu-submenu-arrow"
                }
            }

            if *is_open.get() || !is_inline {
                ul {
                    class: format!(
                        "ant-menu ant-menu-sub {}",
                        if is_inline { "ant-menu-inline" } else { "ant-menu-vertical" }
                    ),
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
            class: "ant-menu-item-group {props.class}",
            style: "{props.style}",
            div {
                class: "ant-menu-item-group-title",
                "{props.title}"
            }
            ul {
                class: "ant-menu-item-group-list",
                {props.children}
            }
        }
    }
}

/// 菜单上下文
#[derive(Clone)]
struct MenuContext {
    mode: MenuMode,
    theme: MenuTheme,
    multiple: bool,
    selectable: bool,
    selected_keys: Vec<String>,
    open_keys: Vec<String>,
    accordion: bool,
    inline_collapsed: bool,
    on_click: EventHandler<MenuItemData>,
    on_select: EventHandler<(Vec<String>, MenuItemData)>,
    on_deselect: EventHandler<(Vec<String>, MenuItemData)>,
    on_open_change: EventHandler<Vec<String>>,
    set_selected_keys: UseStateSetter<Vec<String>>,
    set_open_keys: UseStateSetter<Vec<String>>,
}

/// Menu 菜单组件
#[component]
pub fn Menu(props: MenuProps) -> Element {
    // 状态管理
    let selected_keys = use_state(|| {
        if !props.selected_keys.is_empty() {
            props.selected_keys.clone()
        } else {
            props.default_selected_keys.clone()
        }
    });

    let open_keys = use_state(|| {
        if !props.open_keys.is_empty() {
            props.open_keys.clone()
        } else {
            props.default_open_keys.clone()
        }
    });

    // 事件处理
    let on_click = EventHandler::new(move |item: MenuItemData| {
        if let Some(handler) = &props.on_click {
            handler.call(item);
        }
    });

    let on_select = EventHandler::new(move |data: (Vec<String>, MenuItemData)| {
        if let Some(handler) = &props.on_select {
            handler.call(data);
        }
    });

    let on_deselect = EventHandler::new(move |data: (Vec<String>, MenuItemData)| {
        if let Some(handler) = &props.on_deselect {
            handler.call(data);
        }
    });

    let on_open_change = EventHandler::new(move |keys: Vec<String>| {
        if let Some(handler) = &props.on_open_change {
            handler.call(keys);
        }
    });

    // 创建菜单上下文
    let menu_context = MenuContext {
        mode: props.mode.clone(),
        theme: props.theme.clone(),
        multiple: props.multiple,
        selectable: props.selectable,
        selected_keys: selected_keys.get().clone(),
        open_keys: open_keys.get().clone(),
        accordion: props.accordion,
        inline_collapsed: props.inline_collapsed,
        on_click,
        on_select,
        on_deselect,
        on_open_change,
        set_selected_keys: selected_keys.setter(),
        set_open_keys: open_keys.setter(),
    };

    // 提供菜单上下文
    use_context_provider(|| menu_context);

    // 生成菜单样式
    let menu_style = generate_menu_style(
        props.mode.clone().into(),
        props.theme.clone().into(),
        props.inline_collapsed,
    );

    // 菜单类名
    let menu_class = format!(
        "ant-menu {} {} {}",
        match props.mode {
            MenuMode::Horizontal => "ant-menu-horizontal",
            MenuMode::Vertical => "ant-menu-vertical",
            MenuMode::Inline => "ant-menu-inline",
        },
        match props.theme {
            MenuTheme::Light => "ant-menu-light",
            MenuTheme::Dark => "ant-menu-dark",
        },
        if props.inline_collapsed && props.mode == MenuMode::Inline {
            "ant-menu-inline-collapsed"
        } else {
            ""
        }
    );

    // 渲染菜单项
    let render_menu_items = |items: &[MenuItemData]| {
        items.iter().map(|item| {
            if item.is_group {
                rsx! {
                    MenuGroup {
                        key: "{item.key}",
                        menu_key: "{item.key}",
                        title: "{item.group_title.clone().unwrap_or_default()}",
                        if let Some(children) = &item.children {
                            {render_menu_items(children)}
                        }
                    }
                }
            } else if let Some(children) = &item.children {
                rsx! {
                    SubMenu {
                        key: "{item.key}",
                        menu_key: "{item.key}",
                        title: "{item.title}",
                        icon: item.icon.clone(),
                        disabled: item.disabled,
                        {render_menu_items(children)}
                    }
                }
            } else {
                rsx! {
                    MenuItem {
                        key: "{item.key}",
                        menu_key: "{item.key}",
                        icon: item.icon.clone(),
                        disabled: item.disabled,
                        href: item.href.clone(),
                        "{item.title}"
                    }
                }
            }
        })
    };

    rsx! {
        style { {menu_style} }
        ul {
            class: "{menu_class} {props.class}",
            style: "{props.style}",
            if !props.items.is_empty() {
                {render_menu_items(&props.items)}
            } else {
                {props.children}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_menu_item_data() {
        let item = MenuItemData {
            key: "1".to_string(),
            title: "Item 1".to_string(),
            icon: Some("icon".to_string()),
            disabled: false,
            href: None,
            children: None,
            is_group: false,
            group_title: None,
        };

        assert_eq!(item.key, "1");
        assert_eq!(item.title, "Item 1");
        assert_eq!(item.icon, Some("icon".to_string()));
        assert!(!item.disabled);
        assert!(item.href.is_none());
        assert!(item.children.is_none());
        assert!(!item.is_group);
        assert!(item.group_title.is_none());
    }

    #[test]
    fn test_menu_item_with_children() {
        let item = MenuItemData {
            key: "parent".to_string(),
            title: "Parent".to_string(),
            icon: None,
            disabled: false,
            href: None,
            children: Some(vec![
                MenuItemData {
                    key: "child1".to_string(),
                    title: "Child 1".to_string(),
                    icon: None,
                    disabled: false,
                    href: None,
                    children: None,
                    is_group: false,
                    group_title: None,
                },
                MenuItemData {
                    key: "child2".to_string(),
                    title: "Child 2".to_string(),
                    icon: None,
                    disabled: false,
                    href: None,
                    children: None,
                    is_group: false,
                    group_title: None,
                },
            ]),
            is_group: false,
            group_title: None,
        };

        assert_eq!(item.key, "parent");
        assert_eq!(item.title, "Parent");
        assert!(item.children.is_some());
        assert_eq!(item.children.as_ref().unwrap().len(), 2);
        assert_eq!(item.children.as_ref().unwrap()[0].key, "child1");
        assert_eq!(item.children.as_ref().unwrap()[1].key, "child2");
    }

    #[test]
    fn test_menu_group() {
        let group = MenuItemData {
            key: "group1".to_string(),
            title: "".to_string(),
            icon: None,
            disabled: false,
            href: None,
            children: Some(vec![MenuItemData {
                key: "item1".to_string(),
                title: "Item 1".to_string(),
                icon: None,
                disabled: false,
                href: None,
                children: None,
                is_group: false,
                group_title: None,
            }]),
            is_group: true,
            group_title: Some("Group 1".to_string()),
        };

        assert_eq!(group.key, "group1");
        assert!(group.is_group);
        assert_eq!(group.group_title, Some("Group 1".to_string()));
        assert!(group.children.is_some());
        assert_eq!(group.children.as_ref().unwrap().len(), 1);
        assert_eq!(group.children.as_ref().unwrap()[0].key, "item1");
    }
}

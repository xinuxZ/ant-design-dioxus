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
                title: props.children.clone().into_any().to_string(),
                icon: props.icon.clone(),
                disabled: props.disabled,
                href: props.href.clone(),
                children: None,
                is_group: false,
                group_title: None,
            };

            // 更新选中状态
            if menu_ctx.selectable {
                let mut new_selected_keys = menu_ctx.selected_keys.clone();

                if menu_ctx.multiple {
                    if new_selected_keys.contains(&props.menu_key) {
                        new_selected_keys.retain(|key| key != &props.menu_key);
                        menu_ctx
                            .on_deselect
                            .call((new_selected_keys.clone(), item_data.clone()));
                    } else {
                        new_selected_keys.push(props.menu_key.clone());
                        menu_ctx
                            .on_select
                            .call((new_selected_keys.clone(), item_data.clone()));
                    }
                } else {
                    if !new_selected_keys.contains(&props.menu_key) {
                        new_selected_keys = vec![props.menu_key.clone()];
                        menu_ctx
                            .on_select
                            .call((new_selected_keys.clone(), item_data.clone()));
                    }
                }

                menu_ctx.selected_keys = new_selected_keys;
            }

            menu_ctx.on_click.call(item_data);
        }
    };

    // 构建菜单项类名
    let mut classes = vec!["ant-menu-item"];

    if is_selected {
        classes.push("ant-menu-item-selected");
    }

    if props.disabled {
        classes.push("ant-menu-item-disabled");
    }

    if !props.class.is_empty() {
        classes.push(&props.class);
    }

    let class_name = classes.join(" ");
    let style_str = if props.style.is_empty() {
        String::new()
    } else {
        props.style.clone()
    };

    // 渲染菜单项
    if let Some(href) = &props.href {
        rsx! {
            li {
                class: "{class_name}",
                style: "{style_str}",
                role: "menuitem",
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
                        class: "ant-menu-title-content",
                        {props.children}
                    }
                }
            }
        }
    } else {
        rsx! {
            li {
                class: "{class_name}",
                style: "{style_str}",
                role: "menuitem",
                onclick: handle_click,
                if let Some(icon) = &props.icon {
                    span {
                        class: "ant-menu-item-icon",
                        "{icon}"
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
    let menu_ctx = use_context::<MenuContext>();
    let is_open = create_signal(menu_ctx.open_keys.contains(&props.menu_key));

    // 处理点击事件
    let handle_title_click = move |_| {
        if !props.disabled && menu_ctx.mode == MenuMode::Inline {
            let mut new_open_keys = menu_ctx.open_keys.clone();

            if menu_ctx.accordion {
                // 手风琴模式下，同一级别只能展开一个
                if is_open() {
                    new_open_keys.retain(|key| key != &props.menu_key);
                } else {
                    // 在手风琴模式下，找到同级所有已展开的菜单项并关闭它们
                    // 这里简单处理，直接替换为当前点击的菜单项
                    new_open_keys = vec![props.menu_key.clone()];
                }
            } else {
                // 非手风琴模式下，可以同时展开多个子菜单
                if is_open() {
                    new_open_keys.retain(|key| key != &props.menu_key);
                } else {
                    new_open_keys.push(props.menu_key.clone());
                }
            }

            // 更新展开状态
            menu_ctx.open_keys = new_open_keys.clone();
            is_open.set(!is_open());

            // 触发展开状态变更回调
            menu_ctx.on_open_change.call(new_open_keys.clone());
        }
    };

    // 构建子菜单类名
    let mut classes = vec!["ant-menu-submenu"];

    match menu_ctx.mode {
        MenuMode::Vertical => classes.push("ant-menu-submenu-vertical"),
        MenuMode::Horizontal => classes.push("ant-menu-submenu-horizontal"),
        MenuMode::Inline => classes.push("ant-menu-submenu-inline"),
    }

    if is_open() {
        classes.push("ant-menu-submenu-open");
    }

    if props.disabled {
        classes.push("ant-menu-submenu-disabled");
    }

    if !props.class.is_empty() {
        classes.push(&props.class);
    }

    let class_name = classes.join(" ");
    let style_str = if props.style.is_empty() {
        String::new()
    } else {
        props.style.clone()
    };

    // 渲染子菜单
    rsx! {
        li {
            class: "{class_name}",
            style: "{style_str}",
            role: "menuitem",
            div {
                class: "ant-menu-submenu-title",
                onclick: handle_title_click,
                if let Some(icon) = &props.icon {
                    span {
                        class: "ant-menu-item-icon",
                        "{icon}"
                    }
                }
                span {
                    class: "ant-menu-title-content",
                    "{props.title}"
                }
                span {
                    class: "ant-menu-submenu-arrow"
                }
            }

            if is_open() || menu_ctx.mode != MenuMode::Inline {
                ul {
                    class: "ant-menu ant-menu-sub",
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
}

/// Menu 菜单组件
#[component]
pub fn Menu(props: MenuProps) -> Element {
    // 初始化选中的菜单项
    let selected_keys = create_signal({
        if !props.selected_keys.is_empty() {
            props.selected_keys.clone()
        } else if !props.default_selected_keys.is_empty() {
            props.default_selected_keys.clone()
        } else {
            vec![]
        }
    });

    // 初始化展开的子菜单
    let open_keys = create_signal({
        if !props.open_keys.is_empty() {
            props.open_keys.clone()
        } else if !props.default_open_keys.is_empty() {
            props.default_open_keys.clone()
        } else {
            vec![]
        }
    });

    // 处理菜单项点击事件
    let handle_item_click = move |item: MenuItemData| {
        if let Some(handler) = &props.on_click {
            handler.call(item);
        }
    };

    // 处理菜单项选中事件
    let handle_select = move |(keys, item): (Vec<String>, MenuItemData)| {
        if let Some(handler) = &props.on_select {
            handler.call((keys, item));
        }
    };

    // 处理菜单项取消选中事件
    let handle_deselect = move |(keys, item): (Vec<String>, MenuItemData)| {
        if let Some(handler) = &props.on_deselect {
            handler.call((keys, item));
        }
    };

    // 处理子菜单展开状态变更事件
    let handle_open_change = move |keys: Vec<String>| {
        if let Some(handler) = &props.on_open_change {
            handler.call(keys);
        }
    };

    // 创建菜单上下文
    let menu_context = MenuContext {
        mode: props.mode.clone(),
        theme: props.theme.clone(),
        multiple: props.multiple,
        selectable: props.selectable,
        selected_keys: selected_keys.read().clone(),
        open_keys: open_keys.read().clone(),
        accordion: props.accordion,
        inline_collapsed: props.inline_collapsed,
        on_click: EventHandler::new(move |item| handle_item_click(item)),
        on_select: EventHandler::new(move |args| handle_select(args)),
        on_deselect: EventHandler::new(move |args| handle_deselect(args)),
        on_open_change: EventHandler::new(move |keys| handle_open_change(keys)),
    };

    // 构建菜单类名
    let mut classes = vec!["ant-menu"];

    match props.mode {
        MenuMode::Vertical => classes.push("ant-menu-vertical"),
        MenuMode::Horizontal => classes.push("ant-menu-horizontal"),
        MenuMode::Inline => classes.push("ant-menu-inline"),
    }

    match props.theme {
        MenuTheme::Light => classes.push("ant-menu-light"),
        MenuTheme::Dark => classes.push("ant-menu-dark"),
    }

    if props.inline_collapsed && props.mode == MenuMode::Inline {
        classes.push("ant-menu-inline-collapsed");
    }

    if !props.class.is_empty() {
        classes.push(&props.class);
    }

    let class_name = classes.join(" ");

    // 注入样式
    let style_id = format!(
        "ant-menu-{}-{}",
        match props.mode {
            MenuMode::Vertical => "vertical",
            MenuMode::Horizontal => "horizontal",
            MenuMode::Inline => "inline",
        },
        match props.theme {
            MenuTheme::Light => "light",
            MenuTheme::Dark => "dark",
        }
    );

    let _style = generate_menu_style(props.mode.clone().into(), props.theme.clone().into());

    // 渲染菜单
    let style_str = if props.style.is_empty() {
        String::new()
    } else {
        props.style.clone()
    };

    provide_context(menu_context);

    rsx! {
        ul {
            id: "{style_id}",
            class: "{class_name}",
            style: "{style_str}",
            role: "menu",
            if !props.items.is_empty() {
                render_menu_items(&props.items)
            } else {
                {props.children}
            }
        }
    }
}

/// 渲染菜单项
fn render_menu_items(items: &[MenuItemData]) -> Element {
    rsx! {
        {items.iter().map(|item| {
            if item.is_group {
                rsx! {
                    MenuGroup {
                        key: "{item.key}",
                        menu_key: "{item.key}",
                        title: "{item.group_title.clone().unwrap_or_default()}",
                        if let Some(children) = &item.children {
                            render_menu_items(children)
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
                        render_menu_items(children)
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
        })}
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

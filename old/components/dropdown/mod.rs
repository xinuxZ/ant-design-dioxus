//! Dropdown 下拉菜单组件
//!
//! 向下弹出的列表。
//!
//! ## 何时使用
//!
//! 当页面上的操作命令过多时，用此组件可以收纳操作元素。点击或移入触点，会出现一个下拉菜单。可在列表中进行选择，并执行相应的命令。
//! - 用于收罗一组命令操作。
//! - Select 用于选择，而 Dropdown 是命令集合。

mod styles;

// use crate::components::menu::{Menu, MenuItem, MenuItemProps}; // 暂时注释掉未使用的导入
use crate::utils::class_names::conditional_class_names_array;
use dioxus::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, Element as WebElement};
// use web_sys::MouseEvent; // 暂时注释掉未使用的导入

use self::styles::{
    generate_dropdown_style, DropdownPlacement as StyleDropdownPlacement, DropdownSize,
    DropdownTheme, DropdownTrigger as StyleDropdownTrigger,
};

/// 下拉菜单触发方式
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DropdownTrigger {
    /// 点击触发
    Click,
    /// 悬停触发
    Hover,
    /// 右键触发
    ContextMenu,
}

impl Default for DropdownTrigger {
    fn default() -> Self {
        DropdownTrigger::Hover
    }
}

impl From<DropdownTrigger> for StyleDropdownTrigger {
    fn from(trigger: DropdownTrigger) -> Self {
        match trigger {
            DropdownTrigger::Click => StyleDropdownTrigger::Click,
            DropdownTrigger::Hover => StyleDropdownTrigger::Hover,
            DropdownTrigger::ContextMenu => StyleDropdownTrigger::ContextMenu,
        }
    }
}

/// 下拉菜单弹出位置
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DropdownPlacement {
    /// 下方左对齐
    BottomLeft,
    /// 下方居中
    Bottom,
    /// 下方右对齐
    BottomRight,
    /// 上方左对齐
    TopLeft,
    /// 上方居中
    Top,
    /// 上方右对齐
    TopRight,
}

impl Default for DropdownPlacement {
    fn default() -> Self {
        DropdownPlacement::BottomLeft
    }
}

impl From<DropdownPlacement> for StyleDropdownPlacement {
    fn from(placement: DropdownPlacement) -> Self {
        match placement {
            DropdownPlacement::BottomLeft => StyleDropdownPlacement::BottomLeft,
            DropdownPlacement::Bottom => StyleDropdownPlacement::Bottom,
            DropdownPlacement::BottomRight => StyleDropdownPlacement::BottomRight,
            DropdownPlacement::TopLeft => StyleDropdownPlacement::TopLeft,
            DropdownPlacement::Top => StyleDropdownPlacement::Top,
            DropdownPlacement::TopRight => StyleDropdownPlacement::TopRight,
        }
    }
}

/// 下拉菜单项
#[derive(Debug, Clone, PartialEq)]
pub struct DropdownMenuItem {
    /// 菜单项键值
    pub key: String,
    /// 菜单项标签
    pub label: String,
    /// 是否禁用
    pub disabled: bool,
    /// 是否显示分割线
    pub divider: bool,
    /// 图标
    pub icon: Option<String>,
    /// 子菜单项
    pub children: Vec<DropdownMenuItem>,
}

impl DropdownMenuItem {
    pub fn new(key: &str, label: &str) -> Self {
        Self {
            key: key.to_string(),
            label: label.to_string(),
            disabled: false,
            divider: false,
            icon: None,
            children: Vec::new(),
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn with_divider(mut self) -> Self {
        self.divider = true;
        self
    }

    pub fn with_icon(mut self, icon: &str) -> Self {
        self.icon = Some(icon.to_string());
        self
    }

    pub fn with_children(mut self, children: Vec<DropdownMenuItem>) -> Self {
        self.children = children;
        self
    }
}

/// Dropdown 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct DropdownProps {
    /// 触发下拉的元素
    pub children: Element,
    /// 下拉菜单内容
    pub overlay: Option<Element>,
    /// 菜单项数据（与 overlay 二选一）
    pub menu_items: Option<Vec<DropdownMenuItem>>,
    /// 触发下拉的行为
    #[props(default = DropdownTrigger::Hover)]
    pub trigger: DropdownTrigger,
    /// 下拉根元素的类名称
    #[props(default = String::new())]
    pub overlay_class_name: String,
    /// 下拉根元素的样式
    #[props(default = String::new())]
    pub overlay_style: String,
    /// 菜单弹出位置
    #[props(default = DropdownPlacement::BottomLeft)]
    pub placement: DropdownPlacement,
    /// 菜单是否禁用
    #[props(default = false)]
    pub disabled: bool,
    /// 菜单是否显示箭头
    #[props(default = false)]
    pub arrow: bool,
    /// 点击菜单项后是否收起菜单
    #[props(default = true)]
    pub close_on_select: bool,
    /// 菜单主题
    #[props(default = DropdownTheme::Light)]
    pub theme: DropdownTheme,
    /// 菜单尺寸
    #[props(default = DropdownSize::Middle)]
    pub size: DropdownSize,
    /// 菜单项点击事件
    pub on_menu_click: Option<EventHandler<String>>,
    /// 菜单显示状态变化事件
    pub on_visible_change: Option<EventHandler<bool>>,
    /// 自定义样式类名
    #[props(default = String::new())]
    pub class: String,
    /// 自定义样式
    #[props(default = String::new())]
    pub style: String,
}

/// Dropdown 下拉菜单组件
#[component]
pub fn Dropdown(props: DropdownProps) -> Element {
    let mut visible = use_signal(|| false);
    let mut dropdown_ref = use_signal(|| None::<WebElement>);
    let mut overlay_ref = use_signal(|| None::<WebElement>);

    // 生成样式
    let dropdown_style = generate_dropdown_style(
        props.placement.into(),
        props.theme.clone(),
        props.size.clone(),
        props.arrow,
        props.disabled,
    );

    // 显示下拉菜单
    let show_dropdown = use_callback(move |_| {
        if !props.disabled {
            visible.set(true);
            if let Some(callback) = &props.on_visible_change {
                callback.call(true);
            }
        }
    });

    // 隐藏下拉菜单
    let hide_dropdown = use_callback(move |_| {
        visible.set(false);
        if let Some(callback) = &props.on_visible_change {
            callback.call(false);
        }
    });

    // 处理菜单项点击
    let handle_menu_click = move |key: String| {
        if let Some(callback) = &props.on_menu_click {
            callback.call(key);
        }

        if props.close_on_select {
            hide_dropdown(());
        }
    };

    // 处理点击外部区域
    use_effect(move || {
        if visible.read().clone() {
            let hide_dropdown_clone = hide_dropdown.clone();
            let closure =
                wasm_bindgen::closure::Closure::wrap(Box::new(move |event: web_sys::Event| {
                    if let Some(target) = event.target() {
                        let target_element = target.dyn_into::<WebElement>().unwrap();

                        // 检查点击是否在下拉菜单外部
                        let mut is_outside = true;
                        if let Some(dropdown) = dropdown_ref.read().as_ref() {
                            if dropdown.contains(Some(&target_element)) {
                                is_outside = false;
                            }
                        }
                        if let Some(overlay) = overlay_ref.read().as_ref() {
                            if overlay.contains(Some(&target_element)) {
                                is_outside = false;
                            }
                        }

                        // 隐藏菜单
                        if is_outside {
                            hide_dropdown_clone.call(());
                        }
                    }
                }) as Box<dyn FnMut(_)>);

            // 添加全局点击监听器
            if let Some(window) = window() {
                window
                    .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
                    .unwrap();

                return closure;
            }
        }

        // 返回空闭包
        wasm_bindgen::closure::Closure::wrap(Box::new(|_: web_sys::Event| {}) as Box<dyn FnMut(_)>)
    });

    // 绑定触发事件
    let events = match props.trigger {
        DropdownTrigger::Click => use_memo(move || {
            vec![
                ("onclick", show_dropdown.clone()),
                ("onblur", hide_dropdown.clone()),
            ]
        }),
        DropdownTrigger::Hover => use_memo(move || {
            vec![
                ("onmouseenter", show_dropdown.clone()),
                ("onmouseleave", hide_dropdown.clone()),
            ]
        }),
        DropdownTrigger::ContextMenu => use_memo(move || {
            vec![
                ("oncontextmenu", show_dropdown.clone()),
                ("onblur", hide_dropdown.clone()),
            ]
        }),
    };

    // 类名生成
    let class_names = conditional_class_names_array(&[
        ("ant-dropdown", true),
        ("ant-dropdown-disabled", props.disabled),
        ("ant-dropdown-show", *visible.read()),
        (&props.class, !props.class.is_empty()),
    ]);

    // 渲染菜单内容
    let menu_content = if let Some(items) = &props.menu_items {
        let handle_menu_click_event = EventHandler::new(move |key: String| handle_menu_click(key));
        render_menu_items_recursive(items, &handle_menu_click_event)
    } else if let Some(overlay) = &props.overlay {
        overlay.clone()
    } else {
        rsx! { div { class: "ant-dropdown-menu", "无菜单内容" } }
    };

    // 渲染菜单箭头
    let arrow = if props.arrow {
        rsx! { div { class: "ant-dropdown-arrow" } }
    } else {
        rsx! { "" }
    };

    rsx! {
        style { {dropdown_style} }
        div {
            class: class_names,
            style: props.style.clone(),
            ref: |el| dropdown_ref.set(el),

            // 触发器
            div {
                class: "ant-dropdown-trigger",
                onclick: if props.trigger == DropdownTrigger::Click && !props.disabled { move |_| show_dropdown(()) } else { move |_| {} },
                onmouseenter: if props.trigger == DropdownTrigger::Hover && !props.disabled { move |_| show_dropdown(()) } else { move |_| {} },
                onmouseleave: if props.trigger == DropdownTrigger::Hover && !props.disabled { move |_| hide_dropdown(()) } else { move |_| {} },
                oncontextmenu: if props.trigger == DropdownTrigger::ContextMenu && !props.disabled {
                    move |e: MouseEvent| {
                        e.prevent_default();
                        e.stop_propagation();
                        show_dropdown(());
                    }
                } else {
                    move |_| {}
                },

                {props.children}
            }

            // 下拉菜单
            div {
                class: if *visible.read() {
                    format!("ant-dropdown-wrap {}", props.overlay_class_name)
                } else {
                    format!("ant-dropdown-wrap ant-dropdown-hidden {}", props.overlay_class_name)
                },
                style: props.overlay_style.clone(),
                ref: |el| overlay_ref.set(el),

                {arrow}
                {menu_content}
            }
        }
    }
}

/// 递归渲染菜单项
fn render_menu_items_recursive(
    items: &[DropdownMenuItem],
    handle_menu_click: &EventHandler<String>,
) -> Element {
    rsx! {
        div {
            class: "ant-dropdown-menu ant-dropdown-menu-root ant-dropdown-menu-vertical",

            for item in items.iter() {
                if item.divider {
                    div {
                        class: "ant-dropdown-menu-item-divider",
                        key: "{item.key}-divider"
                    }
                }

                div {
                    key: "{item.key}",
                    class: if item.disabled {
                        "ant-dropdown-menu-item ant-dropdown-menu-item-disabled"
                    } else {
                        "ant-dropdown-menu-item"
                    },
                    onclick: if !item.disabled {
                        let key = item.key.clone();
                        let handle_menu_click = handle_menu_click.clone();
                        move |e: web_sys::MouseEvent| {
                            e.stop_propagation();
                            handle_menu_click.call(key.clone());
                        }
                    } else {
                        move |_| {}
                    },

                    if let Some(icon) = &item.icon {
                        span {
                            class: "ant-dropdown-menu-item-icon",
                            "{icon}"
                        }
                    }

                    span {
                        class: "ant-dropdown-menu-title-content",
                        "{item.label}"
                    }

                    if !item.children.is_empty() {
                        span {
                            class: "ant-dropdown-menu-submenu-arrow",
                            "›"
                        }

                        // 子菜单
                        div {
                            class: "ant-dropdown-menu-sub",
                            {render_menu_items_recursive(&item.children, handle_menu_click)}
                        }
                    }
                }
            }
        }
    }
}

/// 下拉菜单项构建器
#[derive(Debug, Clone)]
pub struct DropdownMenuItemBuilder {
    item: DropdownMenuItem,
}

impl DropdownMenuItemBuilder {
    pub fn new(key: &str, label: &str) -> Self {
        Self {
            item: DropdownMenuItem::new(key, label),
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.item = self.item.disabled(disabled);
        self
    }

    pub fn with_divider(mut self) -> Self {
        self.item = self.item.with_divider();
        self
    }

    pub fn with_icon(mut self, icon: &str) -> Self {
        self.item = self.item.with_icon(icon);
        self
    }

    pub fn with_children(mut self, children: Vec<DropdownMenuItem>) -> Self {
        self.item = self.item.with_children(children);
        self
    }

    pub fn build(self) -> DropdownMenuItem {
        self.item
    }
}

/// 便捷的下拉菜单项创建宏
#[macro_export]
macro_rules! dropdown_item {
    ($key:expr, $label:expr) => {
        DropdownMenuItem::new($key, $label)
    };
    ($key:expr, $label:expr, disabled: $disabled:expr) => {
        DropdownMenuItem::new($key, $label).disabled($disabled)
    };
    ($key:expr, $label:expr, icon: $icon:expr) => {
        DropdownMenuItem::new($key, $label).with_icon($icon)
    };
    ($key:expr, $label:expr, children: [$($children:expr),*]) => {
        DropdownMenuItem::new($key, $label).with_children(vec![$($children),*])
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dropdown_trigger_default() {
        let trigger = DropdownTrigger::default();
        assert_eq!(trigger, DropdownTrigger::Hover);
    }

    #[test]
    fn test_dropdown_placement_default() {
        let placement = DropdownPlacement::default();
        assert_eq!(placement, DropdownPlacement::BottomLeft);
    }

    #[test]
    fn test_dropdown_menu_item_new() {
        let item = DropdownMenuItem::new("key1", "Label 1");
        assert_eq!(item.key, "key1");
        assert_eq!(item.label, "Label 1");
        assert_eq!(item.disabled, false);
        assert_eq!(item.divider, false);
        assert_eq!(item.icon, None);
        assert!(item.children.is_empty());
    }

    #[test]
    fn test_dropdown_menu_item_builder() {
        let item = DropdownMenuItemBuilder::new("key1", "Label 1")
            .disabled(true)
            .with_icon("icon")
            .with_divider()
            .build();

        assert_eq!(item.key, "key1");
        assert_eq!(item.label, "Label 1");
        assert_eq!(item.disabled, true);
        assert_eq!(item.divider, true);
        assert_eq!(item.icon, Some("icon".to_string()));
    }
}

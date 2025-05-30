//! Tabs 标签页组件
//!
//! 选项卡切换组件。
//!
//! ## 何时使用
//!
//! 提供平级的区域将大块内容进行收纳和展现，保持界面整洁。
//! Ant Design 依次提供了三级选项卡，分别用于不同的场景。
//! - 卡片式的页签，提供可关闭的样式，常用于容器顶部。
//! - 既可用于容器顶部，也可用于容器内部，是最通用的 Tabs。
//! - Radio.Button 可作为更次级的页签来使用。

use crate::utils::class_names::class_names;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

const TABS_STYLES: &str = include_str!("./style.css");

/// Tabs 标签页组件属性
#[derive(Props, Clone, PartialEq)]
pub struct TabsProps {
    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 当前激活 tab 面板的 key
    #[props(default)]
    pub active_key: Option<String>,

    /// 初始化选中面板的 key，如果没有设置 activeKey
    #[props(default)]
    pub default_active_key: Option<String>,

    /// 页签的基本样式
    #[props(default = TabsType::Line)]
    pub tab_type: TabsType,

    /// 页签位置
    #[props(default = TabsPosition::Top)]
    pub tab_position: TabsPosition,

    /// 大小
    #[props(default = TabsSize::Default)]
    pub size: TabsSize,

    /// 是否隐藏加号图标，在 type="editable-card" 时有效
    #[props(default = false)]
    pub hide_add: bool,

    /// 是否可以增减页签
    #[props(default = false)]
    pub editable: bool,

    /// 切换面板的回调
    #[props(default)]
    pub on_change: Option<EventHandler<String>>,

    /// 新增和删除页签的回调，在 type="editable-card" 时有效
    #[props(default)]
    pub on_edit: Option<EventHandler<(String, String)>>,

    /// tab bar 上额外的元素
    #[props(default)]
    pub tab_bar_extra_content: Option<Element>,

    /// 标签页项目列表（优先级高于 children）
    #[props(default)]
    pub items: Option<Vec<TabItem>>,

    /// 子元素
    #[props(default)]
    pub children: Option<Element>,
}

/// 页签的基本样式
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum TabsType {
    Line,
    Card,
    EditableCard,
}

impl Default for TabsType {
    fn default() -> Self {
        Self::Line
    }
}

/// 页签位置
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum TabsPosition {
    Top,
    Right,
    Bottom,
    Left,
}

impl Default for TabsPosition {
    fn default() -> Self {
        Self::Top
    }
}

/// 大小
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum TabsSize {
    Large,
    Default,
    Small,
}

impl Default for TabsSize {
    fn default() -> Self {
        Self::Default
    }
}

/// 编辑操作类型
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum TabsEditAction {
    Add,
    Remove,
}

/// 标签页项目
#[derive(Clone, PartialEq)]
pub struct TabItem {
    /// 对应 activeKey
    pub key: String,
    /// 选项卡头显示文字
    pub label: String,
    /// 标签页内容
    pub children: Element,
    /// 是否禁用
    pub disabled: Option<bool>,
    /// 是否可关闭
    pub closable: Option<bool>,
}

impl Default for TabItem {
    fn default() -> Self {
        Self {
            key: String::new(),
            label: String::new(),
            children: rsx! { "" },
            disabled: None,
            closable: None,
        }
    }
}

/// Tabs 标签页组件
#[component]
pub fn Tabs(props: TabsProps) -> Element {
    let mut current_key = use_signal(|| {
        props.active_key.clone().unwrap_or_else(|| {
            props
                .default_active_key
                .clone()
                .unwrap_or_else(|| "1".to_string())
        })
    });

    // 监听 active_key 变化
    {
        let mut current_key = current_key.clone();
        let active_key = props.active_key.clone();
        use_effect(move || {
            if let Some(active_key) = &active_key {
                current_key.set(active_key.clone());
            }
        });
    }

    // 新增标签页
    let add_tab = move |_| {
        if let Some(on_edit) = &props.on_edit {
            on_edit.call(("add".to_string(), "add".to_string()));
        }
    };

    let class_name = format!(
        "ant-tabs ant-tabs-{} ant-tabs-{} ant-tabs-{} {}",
        match props.tab_position {
            TabsPosition::Top => "top",
            TabsPosition::Right => "right",
            TabsPosition::Bottom => "bottom",
            TabsPosition::Left => "left",
        },
        match props.tab_type {
            TabsType::Line => "line",
            TabsType::Card => "card",
            TabsType::EditableCard => "editable-card",
        },
        match props.size {
            TabsSize::Large => "large",
            TabsSize::Default => "default",
            TabsSize::Small => "small",
        },
        props.class
    );

    rsx! {
        style { {TABS_STYLES} }

        div {
            class: class_name.clone(),
            style: props.style.clone(),

            // 标签栏
            div {
                class: "ant-tabs-tab-bar",

                div {
                    class: "ant-tabs-nav-container",

                    div {
                        class: "ant-tabs-nav-wrap",

                        div {
                            class: "ant-tabs-nav-scroll",

                            div {
                                class: "ant-tabs-nav",

                                // 渲染标签页
                                {render_tab_nav(&props, &current_key(), &mut current_key, &props.active_key, &props.on_change, &props.on_edit)}

                                // 墨水条
                                if props.tab_type == TabsType::Line {
                                    div {
                                        class: "ant-tabs-ink-bar ant-tabs-ink-bar-animated"
                                    }
                                }
                            }
                        }
                    }

                    // 新增按钮
                    if props.editable && !props.hide_add {
                        div {
                            class: "ant-tabs-nav-add",
                            onclick: add_tab,

                            "+"
                        }
                    }
                }

                // 额外内容
                if let Some(extra_content) = &props.tab_bar_extra_content {
                    div {
                        class: "ant-tabs-extra-content",

                        {extra_content.clone()}
                    }
                }
            }

            // 内容区域
            div {
                class: "ant-tabs-content ant-tabs-content-animated",

                {render_tab_content(&props, &current_key)}
            }
        }

        style { {include_str!("./style.css")} }
    }
}

/// 渲染标签页导航
fn render_tab_nav(
    props: &TabsProps,
    current_key: &str,
    current_key_signal: &mut Signal<String>,
    active_key: &Option<String>,
    on_change: &Option<Callback<String>>,
    on_edit: &Option<Callback<(String, String)>>,
) -> Element {
    let current_key_value = current_key;

    if let Some(items) = &props.items {
        // 使用 items 渲染
        rsx! {
            for item in items.iter() {
                div {
                    key: "{item.key}",
                    class: if *current_key_value == item.key {
                        "ant-tabs-tab ant-tabs-tab-active"
                    } else {
                        "ant-tabs-tab"
                    },
                    class: if item.disabled.unwrap_or(false) { " ant-tabs-tab-disabled" } else { "" },
                    onclick: {
                        let key = item.key.clone();
                        let disabled = item.disabled.unwrap_or(false);
                        let mut current_key_signal = current_key_signal.clone();
                        let active_key = active_key.clone();
                        let on_change = on_change.clone();
                        move |_| {
                            if !disabled {
                                if active_key.is_none() {
                                    *current_key_signal.write() = key.clone();
                                }
                                if let Some(on_change) = &on_change {
                                    on_change.call(key.clone());
                                }
                            }
                        }
                    },

                    div {
                        class: "ant-tabs-tab-btn",
                        "{item.label}"
                    }

                    if props.editable && item.closable.unwrap_or(true) {
                        button {
                            class: "ant-tabs-tab-remove",
                            onclick: {
                                let key = item.key.clone();
                                let on_edit = on_edit.clone();
                                move |_| {
                                    if let Some(on_edit) = &on_edit {
                                        on_edit.call(("remove".to_string(), key.clone()));
                                    }
                                }
                            },
                            "×"
                        }
                    }
                }
            }
        }
    } else {
        // 使用 children 渲染（简化实现）
        rsx! {
            div {
                class: "ant-tabs-tab ant-tabs-tab-active",
                onclick: {
                    let mut current_key_signal = current_key_signal.clone();
                    let active_key = active_key.clone();
                    let on_change = on_change.clone();
                    move |_| {
                        let key = "1".to_string();
                        if active_key.is_none() {
                            *current_key_signal.write() = key.clone();
                        }
                        if let Some(on_change) = &on_change {
                            on_change.call(key);
                        }
                    }
                },

                div {
                    class: "ant-tabs-tab-btn",
                    "Tab 1"
                }

                if props.editable {
                    button {
                        class: "ant-tabs-tab-remove",
                        onclick: {
                            let on_edit = on_edit.clone();
                            move |_| {
                                if let Some(on_edit) = &on_edit {
                                    on_edit.call(("remove".to_string(), "1".to_string()));
                                }
                            }
                        },
                        "×"
                    }
                }
            }
        }
    }
}

/// 渲染标签页内容
fn render_tab_content(props: &TabsProps, current_key: &Signal<String>) -> Element {
    let current_key_value = current_key.read();

    if let Some(items) = &props.items {
        // 使用 items 渲染
        rsx! {
            for item in items.iter() {
                div {
                    key: "{item.key}",
                    class: if *current_key_value == item.key {
                        "ant-tabs-tabpane ant-tabs-tabpane-active"
                    } else {
                        "ant-tabs-tabpane"
                    },
                    style: if *current_key_value == item.key {
                        "display: block;"
                    } else {
                        "display: none;"
                    },

                    {item.children.clone()}
                }
            }
        }
    } else {
        // 使用 children 渲染
        rsx! {
            div {
                class: "ant-tabs-tabpane ant-tabs-tabpane-active",

                {props.children.clone()}
            }
        }
    }
}

/// TabPane 标签页面板属性
#[derive(Props, Clone, PartialEq)]
pub struct TabPaneProps {
    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 对应 activeKey
    pub tab_key: String,

    /// 选项卡头显示文字
    pub tab: String,

    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,

    /// 是否可关闭
    #[props(default = true)]
    pub closable: bool,

    /// 子元素
    pub children: Element,
}

/// TabPane 标签页面板组件
#[component]
pub fn TabPane(props: TabPaneProps) -> Element {
    let class_name = format!("ant-tabs-tabpane {}", props.class);

    rsx! {
        div {
            class: class_name.clone(),
            style: props.style.clone(),
            "data-key": props.tab_key.clone(),
            "data-tab": props.tab.clone(),
            "data-disabled": props.disabled.to_string(),
            "data-closable": props.closable.to_string(),

            {props.children}
        }
    }
}

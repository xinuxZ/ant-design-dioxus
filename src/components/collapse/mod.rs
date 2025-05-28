//! Collapse 折叠面板组件
//!
//! 可以折叠/展开的内容区域。
//!
//! ## 何时使用
//!
//! - 对复杂区域进行分组和隐藏，保持页面的整洁。
//! - 手风琴是一种特殊的折叠面板，只允许单个内容区域展开。

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

// 引入折叠面板样式
const COLLAPSE_STYLE: &str = include_str!("style.css");

/// 折叠面板尺寸
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CollapseSize {
    /// 大尺寸
    Large,
    /// 中等尺寸
    Middle,
    /// 小尺寸
    Small,
}

impl Default for CollapseSize {
    fn default() -> Self {
        Self::Middle
    }
}

/// 展开图标位置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExpandIconPosition {
    /// 左侧
    Start,
    /// 右侧
    End,
}

impl Default for ExpandIconPosition {
    fn default() -> Self {
        Self::Start
    }
}

/// Collapse 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct CollapseProps {
    /// 当前激活 tab 面板的 key
    #[props(default)]
    pub active_key: Option<Vec<String>>,

    /// 默认激活 tab 面板的 key
    #[props(default)]
    pub default_active_key: Option<Vec<String>>,

    /// 是否有边框
    #[props(default = true)]
    pub bordered: bool,

    /// 手风琴模式
    #[props(default = false)]
    pub accordion: bool,

    /// 设置图标位置
    #[props(default)]
    pub expand_icon_position: ExpandIconPosition,

    /// 是否显示箭头图标
    #[props(default = true)]
    pub show_arrow: bool,

    /// 设置折叠面板大小
    #[props(default)]
    pub size: CollapseSize,

    /// 是否可折叠或指定可折叠触发区域
    #[props(default = false)]
    pub collapsible: bool,

    /// 切换面板的回调
    #[props(default)]
    pub on_change: Option<EventHandler<Vec<String>>>,

    /// 自定义 CSS 类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,

    /// 子元素
    children: Element,
}

/// Collapse 组件
///
/// 可以折叠/展开的内容区域
///
/// # Props
/// - `active_key`: 当前激活 tab 面板的 key
/// - `default_active_key`: 默认激活 tab 面板的 key
/// - `bordered`: 是否有边框，默认为 true
/// - `accordion`: 手风琴模式，默认为 false
/// - `expand_icon_position`: 设置图标位置，默认为 Start
/// - `show_arrow`: 是否显示箭头图标，默认为 true
/// - `size`: 设置折叠面板大小，默认为 Middle
/// - `collapsible`: 是否可折叠，默认为 false
/// - `on_change`: 切换面板的回调
/// - `class`: 自定义 CSS 类名
/// - `style`: 自定义样式
/// - `children`: 子元素
#[component]
pub fn Collapse(props: CollapseProps) -> Element {
    let mut active_keys = use_signal(|| {
        props
            .active_key
            .clone()
            .unwrap_or_else(|| props.default_active_key.clone().unwrap_or_default())
    });

    let mut class_list = vec!["ant-collapse"];

    // 添加尺寸类
    match props.size {
        CollapseSize::Large => class_list.push("ant-collapse-large"),
        CollapseSize::Small => class_list.push("ant-collapse-small"),
        CollapseSize::Middle => {}
    }

    // 添加边框类
    if !props.bordered {
        class_list.push("ant-collapse-borderless");
    }

    // 添加图标位置类
    match props.expand_icon_position {
        ExpandIconPosition::End => class_list.push("ant-collapse-icon-position-end"),
        ExpandIconPosition::Start => class_list.push("ant-collapse-icon-position-start"),
    }

    // 添加自定义类
    if let Some(custom_class) = &props.class {
        class_list.push(custom_class);
    }

    let class_str = class_list.join(" ");

    // 处理面板切换
    let handle_panel_click = move |key: String| {
        let mut current_keys = active_keys.read().clone();

        if props.accordion {
            // 手风琴模式：只能展开一个面板
            if current_keys.contains(&key) {
                current_keys.clear();
            } else {
                current_keys.clear();
                current_keys.push(key);
            }
        } else {
            // 普通模式：可以展开多个面板
            if current_keys.contains(&key) {
                current_keys.retain(|k| k != &key);
            } else {
                current_keys.push(key);
            }
        }

        active_keys.set(current_keys.clone());

        if let Some(on_change) = &props.on_change {
            on_change.call(current_keys);
        }
    };

    rsx! {
        style { {COLLAPSE_STYLE} }
        div {
            class: "{class_str}",
            style: props.style.unwrap_or_default(),
            role: "tablist",

            {props.children}
        }
    }
}

/// CollapsePanel 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct CollapsePanelProps {
    /// 对应 activeKey
    pub panel_key: String,

    /// 面板头内容
    pub header: String,

    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,

    /// 是否显示箭头图标
    #[props(default = true)]
    pub show_arrow: bool,

    /// 自定义切换图标
    #[props(default)]
    pub expand_icon: Option<Element>,

    /// 面板右上角的操作区域
    #[props(default)]
    pub extra: Option<Element>,

    /// 是否可折叠或指定可折叠触发区域
    #[props(default)]
    pub collapsible: Option<String>,

    /// 自定义 CSS 类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,

    /// 子元素
    children: Element,
}

/// CollapsePanel 组件
///
/// 折叠面板的子面板
///
/// # Props
/// - `key`: 对应 activeKey
/// - `header`: 面板头内容
/// - `disabled`: 是否禁用，默认为 false
/// - `show_arrow`: 是否显示箭头图标，默认为 true
/// - `expand_icon`: 自定义切换图标
/// - `extra`: 面板右上角的操作区域
/// - `collapsible`: 是否可折叠或指定可折叠触发区域
/// - `class`: 自定义 CSS 类名
/// - `style`: 自定义样式
/// - `children`: 子元素
#[component]
pub fn CollapsePanel(props: CollapsePanelProps) -> Element {
    let mut class_list = vec!["ant-collapse-item"];

    if props.disabled {
        class_list.push("ant-collapse-item-disabled");
    }

    // 添加自定义类
    if let Some(custom_class) = &props.class {
        class_list.push(custom_class);
    }

    let class_str = class_list.join(" ");

    rsx! {
        div {
            class: "{class_str}",
            style: props.style.unwrap_or_default(),

            // 面板头部
            div {
                class: "ant-collapse-header",
                role: "tab",
                "aria-expanded": "false",
                tabindex: if props.disabled { "-1" } else { "0" },

                // 展开图标
                if props.show_arrow {
                    div { class: "ant-collapse-expand-icon",
                        if let Some(icon) = &props.expand_icon {
                            {icon}
                        } else {
                            span { class: "ant-collapse-arrow",
                                svg {
                                    width: "1em",
                                    height: "1em",
                                    fill: "currentColor",
                                    "aria-hidden": "true",
                                    "focusable": "false",
                                    "viewBox": "64 64 896 896",
                                    path { d: "M765.7 486.8L314.9 134.7A7.97 7.97 0 00302 141v77.3c0 4.9 2.3 9.6 6.1 12.6l360 281.1-360 281.1c-3.9 3-6.1 7.7-6.1 12.6V883c0 6.7 7.7 10.4 12.9 6.3l450.8-352.1a31.96 31.96 0 000-50.4z" }
                                }
                            }
                        }
                    }
                }

                // 标题内容
                div { class: "ant-collapse-header-text", "{props.header}" }

                // 额外操作区域
                if let Some(extra) = &props.extra {
                    div { class: "ant-collapse-extra", {extra} }
                }
            }

            // 面板内容
            div {
                class: "ant-collapse-content",
                role: "tabpanel",

                div { class: "ant-collapse-content-box",
                    {props.children}
                }
            }
        }
    }
}

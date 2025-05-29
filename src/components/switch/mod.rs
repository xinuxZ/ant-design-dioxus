//! Switch 开关组件
//!
//! 开关选择器。
//!
//! ## 何时使用
//!
//! - 需要表示开关状态/两种状态之间的切换时；
//! - 和 `checkbox` 的区别是，切换 `switch` 会直接触发状态改变，而 `checkbox` 一般用于状态标记，需要和提交操作配合。
//!
//! ## 示例
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::Switch;
//!
//! #[component]
//! fn App() -> Element {
//!     let mut checked = use_signal(|| false);
//!
//!     rsx! {
//!         Switch {
//!             checked: checked(),
//!             onchange: move |value| {
//!                 checked.set(value);
//!             },
//!         }
//!     }
//! }
//! ```

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

// 引入样式
const STYLE: &str = include_str!("./style.css");

/// Switch 组件尺寸
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SwitchSize {
    /// 默认尺寸
    Default,
    /// 小尺寸
    Small,
}

impl Default for SwitchSize {
    fn default() -> Self {
        Self::Default
    }
}

impl SwitchSize {
    pub fn to_class(&self) -> &'static str {
        match self {
            SwitchSize::Default => "",
            SwitchSize::Small => "ant-switch-small",
        }
    }
}

/// Switch 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct SwitchProps {
    /// 指定当前是否选中
    #[props(default = false)]
    pub checked: bool,

    /// 初始是否选中
    #[props(default = false)]
    pub default_checked: bool,

    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,

    /// 加载中的开关
    #[props(default = false)]
    pub loading: bool,

    /// 开关大小
    #[props(default = SwitchSize::Default)]
    pub size: SwitchSize,

    /// 选中时的内容
    #[props(default = None)]
    pub checked_children: Option<String>,

    /// 非选中时的内容
    #[props(default = None)]
    pub unchecked_children: Option<String>,

    /// 变化时的回调函数
    #[props(default = None)]
    pub onchange: Option<EventHandler<bool>>,

    /// 点击时的回调函数
    #[props(default = None)]
    pub onclick: Option<EventHandler<MouseEvent>>,

    /// 自动获取焦点
    #[props(default = false)]
    pub auto_focus: bool,

    /// 自定义类名
    #[props(default = None)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default = None)]
    pub style: Option<String>,

    /// 自定义 id
    #[props(default = None)]
    pub id: Option<String>,
}

/// Switch 开关组件
///
/// ## 属性
///
/// - `checked`: 指定当前是否选中
/// - `default_checked`: 初始是否选中
/// - `disabled`: 是否禁用
/// - `loading`: 加载中的开关
/// - `size`: 开关大小，可选 `Default` 或 `Small`
/// - `checked_children`: 选中时的内容
/// - `unchecked_children`: 非选中时的内容
/// - `onchange`: 变化时的回调函数
/// - `onclick`: 点击时的回调函数
/// - `auto_focus`: 自动获取焦点
/// - `class`: 自定义类名
/// - `style`: 自定义样式
/// - `id`: 自定义 id
#[component]
pub fn Switch(props: SwitchProps) -> Element {
    // 内部状态管理
    let mut internal_checked = use_signal(|| props.default_checked);

    // 使用受控模式还是非受控模式
    let is_controlled = props.onchange.is_some();
    let current_checked = if is_controlled {
        props.checked
    } else {
        internal_checked()
    };

    // 处理点击事件
    let handle_click = move |evt: MouseEvent| {
        if props.disabled || props.loading {
            return;
        }

        let new_checked = !current_checked;

        // 如果是非受控模式，更新内部状态
        if !is_controlled {
            internal_checked.set(new_checked);
        }

        // 触发回调
        if let Some(onchange) = &props.onchange {
            onchange.call(new_checked);
        }

        if let Some(onclick) = &props.onclick {
            onclick.call(evt);
        }
    };

    // 构建类名
    let mut class_names = vec!["ant-switch"];

    if current_checked {
        class_names.push("ant-switch-checked");
    }

    if props.disabled {
        class_names.push("ant-switch-disabled");
    }

    if props.loading {
        class_names.push("ant-switch-loading");
    }

    let size_class = props.size.to_class();
    if !size_class.is_empty() {
        class_names.push(size_class);
    }

    if let Some(custom_class) = &props.class {
        class_names.push(custom_class);
    }

    let class_str = class_names.join(" ");

    rsx! {
        style { {STYLE} }
        button {
            class: "{class_str}",
            style: props.style.as_deref().unwrap_or(""),
            id: props.id.as_deref().unwrap_or(""),
            r#type: "button",
            role: "switch",
            "aria-checked": "{current_checked}",
            disabled: props.disabled || props.loading,
            autofocus: props.auto_focus,
            onclick: handle_click,

            div {
                class: "ant-switch-handle",
                if props.loading {
                    span {
                        class: "ant-switch-loading-icon",
                        // 这里可以添加加载图标
                        "⟳"
                    }
                }
            }

            span {
                class: "ant-switch-inner",
                if current_checked {
                    if let Some(checked_children) = &props.checked_children {
                        span {
                            class: "ant-switch-inner-checked",
                            "{checked_children}"
                        }
                    }
                } else {
                    if let Some(unchecked_children) = &props.unchecked_children {
                        span {
                            class: "ant-switch-inner-unchecked",
                            "{unchecked_children}"
                        }
                    }
                }
            }
        }
    }
}

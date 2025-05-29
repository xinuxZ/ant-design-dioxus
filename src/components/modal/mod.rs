//! Modal 对话框
//!
//! 模态对话框。
//!
//! ## 何时使用
//!
//! 需要用户处理事务，又不希望跳转页面以致打断工作流程时，可以使用 Modal 在当前页面正中打开一个浮层，承载相应的操作。
//!
//! 另外当需要一个简洁的确认框询问用户时，可以使用 Modal.confirm() 等语法糖方法。
//!
//! ## 代码演示
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::Modal;
//!
//! #[component]
//! fn App() -> Element {
//!     let mut visible = use_signal(|| false);
//!
//!     rsx! {
//!         Modal {
//!             title: "基本对话框",
//!             open: visible(),
//!             on_ok: move |_| {
//!                 visible.set(false);
//!             },
//!             on_cancel: move |_| {
//!                 visible.set(false);
//!             },
//!             "这是一个基本的对话框内容。"
//!         }
//!     }
//! }
//! ```

use dioxus::events::MouseData;
use dioxus::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::window;

const MODAL_STYLES: &str = include_str!("./style.css");

/// Modal 组件尺寸
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ModalSize {
    /// 小尺寸
    Small,
    /// 默认尺寸
    Default,
    /// 大尺寸
    Large,
}

impl Default for ModalSize {
    fn default() -> Self {
        Self::Default
    }
}

impl ModalSize {
    /// 获取对应的宽度
    pub fn to_width(&self) -> &'static str {
        match self {
            ModalSize::Small => "416px",
            ModalSize::Default => "520px",
            ModalSize::Large => "800px",
        }
    }
}

/// Modal 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct ModalProps {
    /// 对话框是否可见
    #[props(default = false)]
    pub open: bool,

    /// 标题
    #[props(default)]
    pub title: Option<String>,

    /// 宽度
    #[props(default)]
    pub width: Option<String>,

    /// 高度
    #[props(default)]
    pub height: Option<String>,

    /// 是否显示右上角的关闭按钮
    #[props(default = true)]
    pub closable: bool,

    /// 点击蒙层是否允许关闭
    #[props(default = true)]
    pub mask_closable: bool,

    /// 是否显示遮罩
    #[props(default = true)]
    pub mask: bool,

    /// 是否支持键盘 esc 关闭
    #[props(default = true)]
    pub keyboard: bool,

    /// 垂直居中展示 Modal
    #[props(default = true)]
    pub centered: bool,

    /// 确认按钮文字
    #[props(default)]
    pub ok_text: Option<String>,

    /// 取消按钮文字
    #[props(default)]
    pub cancel_text: Option<String>,

    /// 确认按钮 loading
    #[props(default = false)]
    pub confirm_loading: bool,

    /// 是否显示确认按钮
    #[props(default = true)]
    pub show_ok_button: bool,

    /// 是否显示取消按钮
    #[props(default = true)]
    pub show_cancel_button: bool,

    /// 点击确定回调
    #[props(default)]
    pub on_ok: Option<EventHandler<()>>,

    /// 点击取消回调
    #[props(default)]
    pub on_cancel: Option<EventHandler<()>>,

    /// Modal 完全关闭后的回调
    #[props(default)]
    pub after_close: Option<EventHandler<()>>,

    /// 自定义页脚
    #[props(default)]
    pub footer: Option<Element>,

    /// 对话框外层容器的类名
    #[props(default)]
    pub wrap_class_name: Option<String>,

    /// 对话框的类名
    #[props(default)]
    pub class_name: Option<String>,

    /// 可用于设置浮层的样式，调整浮层位置等
    #[props(default)]
    pub style: Option<String>,

    /// 设置 Modal 的 z-index
    #[props(default = 1000)]
    pub z_index: i32,

    /// 对话框内容
    #[props(default)]
    pub children: Option<Element>,
}

/// Modal 对话框组件
///
/// 模态对话框，在当前页面正中打开一个浮层，承载相应的操作
#[component]
pub fn Modal(props: ModalProps) -> Element {
    let is_closing = use_signal(|| false);

    // 处理键盘事件
    use_effect(move || {
        if props.open && props.keyboard {
            let closure = wasm_bindgen::closure::Closure::wrap(Box::new(
                move |event: web_sys::KeyboardEvent| {
                    if event.key() == "Escape" {
                        if let Some(on_cancel) = &props.on_cancel {
                            // 直接调用回调函数
                            on_cancel.call(());
                        }
                    }
                },
            ) as Box<dyn FnMut(_)>);

            if let Some(window) = window() {
                if let Some(document) = window.document() {
                    let _ = document.add_event_listener_with_callback(
                        "keydown",
                        closure.as_ref().unchecked_ref(),
                    );
                }
            }

            closure.forget();
        }
    });

    let modal_width = props
        .width
        .as_deref()
        .unwrap_or(ModalSize::default().to_width());
    let modal_height = props.height.as_deref().unwrap_or("auto");

    let modal_style = {
        let mut styles = vec![
            format!("width: {}", modal_width),
            format!("height: {}", modal_height),
            format!("z-index: {}", props.z_index),
        ];

        if let Some(style) = &props.style {
            styles.push(style.clone());
        }

        styles.join("; ")
    };

    let wrap_class = {
        let mut classes = vec!["ant-modal-wrap"];

        if props.centered {
            classes.push("ant-modal-centered");
        }

        if let Some(wrap_class_name) = &props.wrap_class_name {
            classes.push(wrap_class_name);
        }

        classes.join(" ")
    };

    let modal_class = {
        let mut classes = vec!["ant-modal"];

        if let Some(class_name) = &props.class_name {
            classes.push(class_name);
        }

        classes.join(" ")
    };

    let handle_mask_click = move |_event: Event<MouseData>| {
        if props.mask_closable {
            // 在 Dioxus 中，通过检查事件目标来处理遮罩点击
            // 这里简化处理，直接调用取消回调
            if let Some(on_cancel) = &props.on_cancel {
                on_cancel.call(());
            }
        }
    };

    let handle_close = move |_event: Event<MouseData>| {
        if let Some(on_cancel) = &props.on_cancel {
            on_cancel.call(());
        }
    };

    let handle_ok = move |_event: Event<MouseData>| {
        if let Some(on_ok) = &props.on_ok {
            on_ok.call(());
        }
    };

    let handle_cancel = move |_event: Event<MouseData>| {
        if let Some(on_cancel) = &props.on_cancel {
            on_cancel.call(());
        }
    };

    rsx! {
        style { {MODAL_STYLES} }
        if props.open {
            div {
                class: "ant-modal-root",
                // 遮罩层
                if props.mask {
                    div {
                        class: "ant-modal-mask",
                        style: format!("z-index: {}", props.z_index - 1),
                    }
                }
                // 对话框容器
                div {
                    class: "{wrap_class}",
                    style: format!("z-index: {}", props.z_index),
                    onclick: handle_mask_click,
                    div {
                        class: "{modal_class}",
                        style: "{modal_style}",
                        onclick: |event| event.stop_propagation(),

                        // 对话框内容
                        div {
                            class: "ant-modal-content",

                            // 关闭按钮
                            if props.closable {
                                button {
                                    class: "ant-modal-close",
                                    r#type: "button",
                                    "aria-label": "Close",
                                    onclick: handle_close,
                                    span {
                                        class: "ant-modal-close-x",
                                        span {
                                            class: "ant-modal-close-icon",
                                            "×"
                                        }
                                    }
                                }
                            }

                            // 标题
                            if let Some(title) = &props.title {
                                div {
                                    class: "ant-modal-header",
                                    div {
                                        class: "ant-modal-title",
                                        "{title}"
                                    }
                                }
                            }

                            // 主体内容
                            div {
                                class: "ant-modal-body",
                                if let Some(children) = &props.children {
                                    {children}
                                }
                            }

                            // 页脚
                            if let Some(footer) = &props.footer {
                                div {
                                    class: "ant-modal-footer",
                                    {footer}
                                }
                            } else if props.show_ok_button || props.show_cancel_button {
                                div {
                                    class: "ant-modal-footer",
                                    if props.show_cancel_button {
                                        button {
                                            class: "ant-btn ant-btn-default",
                                            r#type: "button",
                                            onclick: handle_cancel,
                                            "{props.cancel_text.as_deref().unwrap_or(\"取消\")}"
                                        }
                                    }
                                    if props.show_ok_button {
                                        button {
                                            class: if props.confirm_loading {
                                                "ant-btn ant-btn-primary ant-btn-loading"
                                            } else {
                                                "ant-btn ant-btn-primary"
                                            },
                                            r#type: "button",
                                            disabled: props.confirm_loading,
                                            onclick: handle_ok,
                                            if props.confirm_loading {
                                                span {
                                                    class: "ant-btn-loading-icon",
                                                    "⟳"
                                                }
                                            }
                                            "{props.ok_text.as_deref().unwrap_or(\"确定\")}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// 确认对话框属性
#[derive(Props, Clone, PartialEq)]
pub struct ConfirmModalProps {
    /// 对话框是否可见
    #[props(default = false)]
    pub open: bool,

    /// 标题
    #[props(default)]
    pub title: Option<String>,

    /// 内容
    pub content: String,

    /// 图标类型
    #[props(default)]
    pub icon_type: Option<String>,

    /// 确认按钮文字
    #[props(default)]
    pub ok_text: Option<String>,

    /// 取消按钮文字
    #[props(default)]
    pub cancel_text: Option<String>,

    /// 点击确定回调
    #[props(default)]
    pub on_ok: Option<EventHandler<()>>,

    /// 点击取消回调
    #[props(default)]
    pub on_cancel: Option<EventHandler<()>>,
}

/// 确认对话框组件
#[component]
pub fn ConfirmModal(props: ConfirmModalProps) -> Element {
    let icon = props.icon_type.as_deref().unwrap_or("?");

    rsx! {
        Modal {
            open: props.open,
            title: props.title.clone(),
            width: Some("416px".to_string()),
            closable: false,
            mask_closable: false,
            on_ok: props.on_ok.clone(),
            on_cancel: props.on_cancel.clone(),
            ok_text: props.ok_text.clone(),
            cancel_text: props.cancel_text.clone(),

            div {
                class: "ant-modal-confirm-body",
                span {
                    class: "ant-modal-confirm-icon",
                    "{icon}"
                }
                div {
                    class: "ant-modal-confirm-content",
                    "{props.content}"
                }
            }
        }
    }
}

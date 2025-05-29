//! Drawer 抽屉
//!
//! 屏幕边缘滑出的浮层面板。
//!
//! ## 何时使用
//!
//! 抽屉从父窗体边缘滑入，覆盖住部分父窗体内容。用户在抽屉内操作时不必离开当前任务，操作完成后，可以平滑地回到原任务。
//!
//! - 当需要一个附加的面板来控制父窗体内容，这个面板在需要时呼出。比如，控制界面展示样式，往界面中添加内容。
//! - 当需要在当前任务流中插入临时任务，创建或预览附加内容。比如展示协议条款，创建子对象。
//!
//! ## 代码演示
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::{Drawer, DrawerPlacement};
//!
//! #[component]
//! fn App() -> Element {
//!     let mut open = use_signal(|| false);
//!
//!     rsx! {
//!         Drawer {
//!             title: "基本抽屉",
//!             placement: DrawerPlacement::Right,
//!             open: open(),
//!             on_close: move |_| {
//!                 open.set(false);
//!             },
//!             "这是抽屉的内容。"
//!         }
//!     }
//! }
//! ```

use dioxus::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::window;

const DRAWER_STYLES: &str = include_str!("./style.css");

/// Drawer 抽屉位置
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DrawerPlacement {
    /// 从顶部滑入
    Top,
    /// 从右侧滑入
    Right,
    /// 从底部滑入
    Bottom,
    /// 从左侧滑入
    Left,
}

impl Default for DrawerPlacement {
    fn default() -> Self {
        Self::Right
    }
}

impl DrawerPlacement {
    /// 获取位置对应的CSS类名
    pub fn to_class(&self) -> &'static str {
        match self {
            DrawerPlacement::Top => "ant-drawer-top",
            DrawerPlacement::Right => "ant-drawer-right",
            DrawerPlacement::Bottom => "ant-drawer-bottom",
            DrawerPlacement::Left => "ant-drawer-left",
        }
    }

    /// 获取默认尺寸
    pub fn default_size(&self) -> &'static str {
        match self {
            DrawerPlacement::Top | DrawerPlacement::Bottom => "378px",
            DrawerPlacement::Left | DrawerPlacement::Right => "378px",
        }
    }

    /// 判断是否为水平方向
    pub fn is_horizontal(&self) -> bool {
        matches!(self, DrawerPlacement::Left | DrawerPlacement::Right)
    }
}

/// Drawer 抽屉尺寸
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DrawerSize {
    /// 默认尺寸
    Default,
    /// 大尺寸
    Large,
}

impl Default for DrawerSize {
    fn default() -> Self {
        Self::Default
    }
}

impl DrawerSize {
    /// 获取水平方向的宽度
    pub fn to_width(&self) -> &'static str {
        match self {
            DrawerSize::Default => "378px",
            DrawerSize::Large => "736px",
        }
    }

    /// 获取垂直方向的高度
    pub fn to_height(&self) -> &'static str {
        match self {
            DrawerSize::Default => "378px",
            DrawerSize::Large => "736px",
        }
    }
}

/// Drawer 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct DrawerProps {
    /// 抽屉是否可见
    #[props(default = false)]
    pub open: bool,

    /// 标题
    #[props(default)]
    pub title: Option<String>,

    /// 抽屉的方向
    #[props(default)]
    pub placement: DrawerPlacement,

    /// 宽度（水平方向）或高度（垂直方向）
    #[props(default)]
    pub size: Option<String>,

    /// 预设抽屉尺寸
    #[props(default)]
    pub drawer_size: DrawerSize,

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

    /// 关闭时销毁 Drawer 里的子元素
    #[props(default = false)]
    pub destroy_on_close: bool,

    /// 是否将抽屉渲染在 body 内
    #[props(default = true)]
    pub get_container: bool,

    /// 设置 Drawer 的 z-index
    #[props(default = 1000)]
    pub z_index: i32,

    /// 关闭时的回调
    #[props(default)]
    pub on_close: Option<EventHandler<()>>,

    /// Drawer 完全关闭后的回调
    #[props(default)]
    pub after_open_change: Option<EventHandler<bool>>,

    /// 自定义页脚
    #[props(default)]
    pub footer: Option<Element>,

    /// 抽屉的类名
    #[props(default)]
    pub class_name: Option<String>,

    /// 用于设置 Drawer 弹出层的样式
    #[props(default)]
    pub style: Option<String>,

    /// 设置 Drawer 容器的样式
    #[props(default)]
    pub body_style: Option<String>,

    /// 设置 Drawer 头部的样式
    #[props(default)]
    pub header_style: Option<String>,

    /// 抽屉内容
    #[props(default)]
    pub children: Option<Element>,

    /// 操作区域，位于抽屉底部
    #[props(default)]
    pub extra: Option<Element>,
}

/// Drawer 抽屉组件
///
/// 屏幕边缘滑出的浮层面板
#[component]
pub fn Drawer(props: DrawerProps) -> Element {
    let mut is_animating = use_signal(|| false);

    // 处理键盘事件
    use_effect(move || {
        if props.open && props.keyboard {
            let closure = wasm_bindgen::closure::Closure::wrap(Box::new(
                move |event: web_sys::KeyboardEvent| {
                    if event.key() == "Escape" {
                        if let Some(on_close) = &props.on_close {
                            // 创建一个模拟的鼠标事件
                            if let Some(window) = window() {
                                // dyn_into 在 JsCate 中是 dyn_into 方法，用于将一个动态类型转换为指定的类型
                                if let Some(mouse_event) = window
                                    .document()
                                    .and_then(|doc| doc.create_event("MouseEvent").ok())
                                    .and_then(|event| event.dyn_into::<web_sys::MouseEvent>().ok())
                                {
                                    on_close.call(());
                                }
                            }
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

    // 处理打开/关闭状态变化
    use_effect(move || {
        if let Some(callback) = &props.after_open_change {
            callback.call(props.open);
        }
    });

    let drawer_size = props.size.as_deref().unwrap_or_else(|| {
        if props.placement.is_horizontal() {
            props.drawer_size.to_width()
        } else {
            props.drawer_size.to_height()
        }
    });

    let drawer_style = {
        let mut styles = vec![format!("z-index: {}", props.z_index)];

        // 根据位置设置尺寸
        match props.placement {
            DrawerPlacement::Left | DrawerPlacement::Right => {
                styles.push(format!("width: {}", drawer_size));
            }
            DrawerPlacement::Top | DrawerPlacement::Bottom => {
                styles.push(format!("height: {}", drawer_size));
            }
        }

        if let Some(style) = &props.style {
            styles.push(style.clone());
        }

        styles.join("; ")
    };

    let drawer_class = {
        let mut classes = vec!["ant-drawer"];
        classes.push(props.placement.to_class());

        if props.open {
            classes.push("ant-drawer-open");
        }

        if let Some(class_name) = &props.class_name {
            classes.push(class_name);
        }

        classes.join(" ")
    };

    let handle_mask_click = move |_event: Event<MouseData>| {
        if props.mask_closable {
            // 简化处理，直接调用关闭回调
            if let Some(on_close) = &props.on_close {
                on_close.call(());
            }
        }
    };

    let handle_close = move |_event: MouseEvent| {
        if let Some(on_close) = &props.on_close {
            on_close.call(());
        }
    };

    rsx! {
        style { {DRAWER_STYLES} }
        if props.open || (!props.destroy_on_close && is_animating()) {
            div {
                class: "ant-drawer-root",

                // 遮罩层
                if props.mask {
                    div {
                        class: "ant-drawer-mask",
                        style: format!("z-index: {}", props.z_index - 1),
                        onclick: handle_mask_click,
                    }
                }

                // 抽屉容器
                div {
                    class: "ant-drawer-content-wrapper",
                    style: format!("z-index: {}", props.z_index),

                    div {
                        class: "{drawer_class}",
                        style: "{drawer_style}",

                        div {
                            class: "ant-drawer-content",

                            // 抽屉头部
                            if props.title.is_some() || props.closable || props.extra.is_some() {
                                div {
                                    class: "ant-drawer-header",
                                    style: props.header_style.as_deref().unwrap_or(""),

                                    div {
                                        class: "ant-drawer-header-title",

                                        // 标题
                                        if let Some(title) = &props.title {
                                            div {
                                                class: "ant-drawer-title",
                                                "{title}"
                                            }
                                        }

                                        // 操作区域
                                        if props.extra.is_some() || props.closable {
                                            div {
                                                class: "ant-drawer-extra",

                                                // 自定义操作区域
                                                if let Some(extra) = &props.extra {
                                                    {extra}
                                                }

                                                // 关闭按钮
                                                if props.closable {
                                                    button {
                                                        class: "ant-drawer-close",
                                                        r#type: "button",
                                                        "aria-label": "Close",
                                                        onclick: handle_close,
                                                        span {
                                                            class: "ant-drawer-close-x",
                                                            span {
                                                                class: "ant-drawer-close-icon",
                                                                "×"
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            // 抽屉主体
                            div {
                                class: "ant-drawer-body",
                                style: props.body_style.as_deref().unwrap_or(""),
                                if let Some(children) = &props.children {
                                    {children}
                                }
                            }

                            // 抽屉页脚
                            if let Some(footer) = &props.footer {
                                div {
                                    class: "ant-drawer-footer",
                                    {footer}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

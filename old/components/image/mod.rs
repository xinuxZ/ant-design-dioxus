//! Image 图片组件
//!
//! 可预览的图片。
//!
//! ## 何时使用
//!
//! - 需要展示图片时使用。
//! - 加载大图时显示 loading 或加载失败时容错处理。

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

mod styles;
use styles::use_image_style;

/// 图片适应方式
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ImageFit {
    /// 填充
    Fill,
    /// 包含
    Contain,
    /// 覆盖
    Cover,
    /// 无
    None,
    /// 缩小
    ScaleDown,
}

impl Default for ImageFit {
    fn default() -> Self {
        Self::Fill
    }
}

impl ImageFit {
    fn to_css_value(&self) -> &'static str {
        match self {
            ImageFit::Fill => "fill",
            ImageFit::Contain => "contain",
            ImageFit::Cover => "cover",
            ImageFit::None => "none",
            ImageFit::ScaleDown => "scale-down",
        }
    }
}

/// Image 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct ImageProps {
    /// 图片地址
    pub src: String,

    /// 图片描述
    #[props(default)]
    pub alt: Option<String>,

    /// 图片宽度
    #[props(default)]
    pub width: Option<String>,

    /// 图片高度
    #[props(default)]
    pub height: Option<String>,

    /// 是否开启预览功能
    #[props(default = true)]
    pub preview: bool,

    /// 占位内容
    #[props(default)]
    pub placeholder: Option<Element>,

    /// 加载失败容错地址
    #[props(default)]
    pub fallback: Option<String>,

    /// 图片适应方式
    #[props(default)]
    pub fit: ImageFit,

    /// 加载完成回调
    #[props(default)]
    pub on_load: Option<EventHandler<Event<ImageData>>>,

    /// 加载失败回调
    #[props(default)]
    pub on_error: Option<EventHandler<Event<ImageData>>>,

    /// 预览回调
    #[props(default)]
    pub on_preview: Option<EventHandler<()>>,

    /// 自定义 CSS 类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,
}

/// Image 组件
///
/// 可预览的图片组件
///
/// # Props
/// - `src`: 图片地址
/// - `alt`: 图片描述
/// - `width`: 图片宽度
/// - `height`: 图片高度
/// - `preview`: 是否开启预览功能，默认为 true
/// - `placeholder`: 占位内容
/// - `fallback`: 加载失败容错地址
/// - `fit`: 图片适应方式，默认为 Fill
/// - `on_load`: 加载完成回调
/// - `on_error`: 加载失败回调
/// - `on_preview`: 预览回调
/// - `class`: 自定义 CSS 类名
/// - `style`: 自定义样式
#[component]
pub fn Image(props: ImageProps) -> Element {
    let mut loading = use_signal(|| true);
    let mut error = use_signal(|| false);
    let mut preview_visible = use_signal(|| false);
    let mut current_src = use_signal(|| props.src.clone());

    // 获取样式
    let style_class = use_image_style();
    let mut class_list = vec![&style_class, "ant-image"];

    // 添加自定义类
    if let Some(custom_class) = &props.class {
        class_list.push(custom_class);
    }

    let class_str = class_list.join(" ");

    // 处理图片加载完成
    let handle_load = move |evt: Event<ImageData>| {
        loading.set(false);
        error.set(false);

        if let Some(on_load) = &props.on_load {
            on_load.call(evt);
        }
    };

    // 处理图片加载失败
    let handle_error = move |evt: Event<ImageData>| {
        loading.set(false);
        error.set(true);

        // 如果有fallback地址，尝试加载fallback
        if let Some(fallback) = &props.fallback {
            if current_src.read().as_str() != fallback {
                current_src.set(fallback.clone());
                loading.set(true);
                error.set(false);
                return;
            }
        }

        if let Some(on_error) = &props.on_error {
            on_error.call(evt);
        }
    };

    // 处理预览
    let handle_preview = move |_| {
        if props.preview {
            preview_visible.set(true);

            if let Some(on_preview) = &props.on_preview {
                on_preview.call(());
            }
        }
    };

    // 关闭预览
    let close_preview = move |_| {
        preview_visible.set(false);
    };

    let image_style = format!(
        "object-fit: {}; {}",
        props.fit.to_css_value(),
        props.style.as_deref().unwrap_or("")
    );

    rsx! {
        div {
            class: class_str.clone(),

            div { class: "ant-image-img-wrapper",
                // 占位符
                if loading.read().clone() && props.placeholder.is_some() {
                    div { class: "ant-image-placeholder",
                        {props.placeholder.as_ref().unwrap()}
                    }
                }

                // 错误状态
                if error.read().clone() {
                    div { class: "ant-image-error",
                        div { class: "ant-image-error-icon",
                            svg {
                                width: "1em",
                                height: "1em",
                                fill: "currentColor",
                                "aria-hidden": "true",
                                "focusable": "false",
                                "viewBox": "64 64 896 896",
                                path { d: "M928 160H96c-17.7 0-32 14.3-32 32v640c0 17.7 14.3 32 32 32h832c17.7 0 32-14.3 32-32V192c0-17.7-14.3-32-32-32zM338 304c35.3 0 64 28.7 64 64s-28.7 64-64 64-64-28.7-64-64 28.7-64 64-64zm513.9 437.1l-192.4-192.4c-4.7-4.7-12.3-4.7-17 0l-114.8 114.8-55.6-55.6c-4.7-4.7-12.3-4.7-17 0L263.5 799.5c-4.7 4.7-4.7 12.3 0 17l.1.1c4.7 4.7 12.3 4.7 17 0l181.5-181.5 55.6 55.6c4.7 4.7 12.3 4.7 17 0l114.8-114.8 175.4 175.4c4.7 4.7 12.3 4.7 17 0l.1-.1c4.7-4.7 4.7-12.3 0-17z" }
                            }
                        }
                    }
                } else {
                    img {
                        src: current_src.read().clone(),
                        alt: props.alt.clone().unwrap_or_default(),
                        width: props.width.unwrap_or_default(),
                        height: props.height.unwrap_or_default(),
                        style: image_style.clone(),
                        class: "ant-image-img",
                        onload: handle_load,
                        onerror: handle_error,
                        onclick: handle_preview,
                    }
                }

                // 预览遮罩
                if props.preview && !loading.read().clone() && !error.read().clone() {
                    div { class: "ant-image-mask",
                        div { class: "ant-image-mask-info",
                            span { class: "ant-image-preview-icon",
                                svg {
                                    width: "1em",
                                    height: "1em",
                                    fill: "currentColor",
                                    "aria-hidden": "true",
                                    "focusable": "false",
                                    "viewBox": "64 64 896 896",
                                    path { d: "M942.2 486.2C847.4 286.5 704.1 186 512 186c-192.2 0-335.4 100.5-430.2 300.3a60.3 60.3 0 000 51.5C176.6 737.5 319.9 838 512 838c192.2 0 335.4-100.5 430.2-300.3 7.7-16.2 7.7-35 0-51.5zM512 766c-161.3 0-279.4-81.8-362.7-254C232.6 339.8 350.7 258 512 258c161.3 0 279.4 81.8 362.7 254C791.5 684.2 673.4 766 512 766zm-4-430c-97.2 0-176 78.8-176 176s78.8 176 176 176 176-78.8 176-176-78.8-176-176-176zm0 288c-61.9 0-112-50.1-112-112s50.1-112 112-112 112 50.1 112 112-50.1 112-112 112z" }
                                }
                            }
                        }
                    }
                }
            }
        }

        // 预览模态框
        if preview_visible.read().clone() {
            div { class: "ant-image-preview-wrap",
                onclick: close_preview,

                div { class: "ant-image-preview",
                    onclick: |e| e.stop_propagation(),

                    div { class: "ant-image-preview-content",
                        div { class: "ant-image-preview-body",
                            img {
                                src: current_src.read().clone(),
                                alt: props.alt.unwrap_or_default(),
                                class: "ant-image-preview-img",
                            }
                        }
                    }

                    div { class: "ant-image-preview-operations",
                        div { class: "ant-image-preview-operations-operation",
                            onclick: close_preview,
                            span { class: "ant-image-preview-close",
                                svg {
                                    width: "1em",
                                    height: "1em",
                                    fill: "currentColor",
                                    "aria-hidden": "true",
                                    "focusable": "false",
                                    "viewBox": "64 64 896 896",
                                    path { d: "M563.8 512l262.5-312.9c4.4-5.2.7-13.1-6.1-13.1h-79.8c-4.7 0-9.2 2.1-12.3 5.7L511.6 449.8 295.1 191.7c-3.1-3.6-7.6-5.7-12.3-5.7H203c-6.8 0-10.5 7.9-6.1 13.1L459.4 512 196.9 824.9A7.95 7.95 0 00203 838h79.8c4.7 0 9.2-2.1 12.3-5.7l216.5-258.1 216.5 258.1c3.1 3.6 7.6 5.7 12.3 5.7h79.8c6.8 0 10.5-7.9 6.1-13.1L563.8 512z" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// ImagePreviewGroup 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct ImagePreviewGroupProps {
    /// 是否开启预览功能
    #[props(default = true)]
    pub preview: bool,

    /// 自定义 CSS 类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,

    /// 子元素
    children: Element,
}

/// ImagePreviewGroup 组件
///
/// 图片预览组，用于多张图片的预览
///
/// # Props
/// - `preview`: 是否开启预览功能，默认为 true
/// - `class`: 自定义 CSS 类名
/// - `style`: 自定义样式
/// - `children`: 子元素
#[component]
pub fn ImagePreviewGroup(props: ImagePreviewGroupProps) -> Element {
    let mut class_list = vec!["ant-image-preview-group"];

    if let Some(custom_class) = &props.class {
        class_list.push(custom_class);
    }

    let class_str = class_list.join(" ");

    rsx! {
        div {
            class: class_str.clone(),
            style: props.style.unwrap_or_default(),
            {props.children}
        }
    }
}

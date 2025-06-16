use dioxus::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, HtmlImageElement};

use crate::components::icon::{Icon, IconType};

use super::{
    styles::{
        generate_bordered_style, generate_container_style, generate_icon_style,
        generate_refresh_button_style, generate_status_mask_style, generate_status_text_style,
        get_status_icon_style, get_status_text,
    },
    types::{QRCodeIconSettings, QRCodeProps, QRCodeStatus, QRCodeType, StatusRenderInfo},
    utils::{generate_qrcode_data_url, generate_qrcode_svg},
};

/// QRCode 二维码组件
#[component]
pub fn QRCode(props: QRCodeProps) -> Element {
    let mut container_ref = use_signal(|| None::<web_sys::Element>);
    let mut qr_code_ref = use_signal(|| None::<web_sys::Element>);
    let mut error = use_signal(|| None::<String>);
    let mut icon_loaded = use_signal(|| false);

    // 根据类型选择渲染方式
    let props_type = props.clone().r#type;
    let props_clone = props.clone();
    let mut render_qrcode = move || {
        if error.read().clone().is_some() {
            return;
        }

        let qr_element = qr_code_ref.read().as_ref().cloned();
        if qr_element.is_none() {
            return;
        }

        match props_type {
            QRCodeType::Svg => render_svg_qrcode(&props_clone, qr_element.unwrap(), error),
            QRCodeType::Canvas => render_canvas_qrcode(&props_clone, qr_element.unwrap(), error),
        }
    };

    // 处理图标加载
    let render_qrcode_clone = render_qrcode.clone();
    let handle_icon_load = move |_| {
        icon_loaded.set(true);
        render_qrcode_clone();
    };

    // 处理刷新按钮点击
    let props_clone = props.clone();
    let handle_refresh = move |_: ()| {
        if let Some(on_refresh) = &props_clone.clone().on_refresh {
            on_refresh.call(());
        }
    };

    // 当内容、大小、颜色等属性变化时重新渲染二维码
    use_effect(move || {
        render_qrcode();
    });

    // 创建图标设置
    let icon_size_value = props.clone().icon_size.unwrap_or(40);
    let icon_settings = props.clone().icon.as_ref().map(|src| QRCodeIconSettings {
        src: src.clone(),
        width: icon_size_value,
        height: icon_size_value,
        ..Default::default()
    });

    // 容器样式
    let container_class = generate_container_style();
    let bordered_class = if props.clone().bordered.unwrap_or(true) {
        generate_bordered_style()
    } else {
        String::new()
    };

    // 组合自定义样式
    let combined_style = match &props.clone().style {
        Some(style) => format!("{}; {}", style, ""),
        None => String::new(),
    };

    // 组合自定义类名
    let combined_class = match &props.clone().class {
        Some(class) => format!("{} {} {}", container_class, bordered_class, class),
        None => format!("{} {}", container_class, bordered_class),
    };

    // 渲染状态遮罩
    let status_mask = if props.clone().status != QRCodeStatus::Active {
        let status_info = StatusRenderInfo {
            status: props.clone().status,
            on_refresh: props.clone().on_refresh.clone(),
        };

        let props_status_render = props.clone().status_render;
        // 使用自定义状态渲染或默认渲染
        if let Some(custom_render) = &props_status_render {
            custom_render.clone()
        } else {
            render_status_mask(status_info)
        }
    } else {
        rsx!()
    };

    let props_icon = props.clone().icon;
    let props_icon_size = props.clone().icon_size.unwrap_or(40);

    // 渲染图标
    let icon_element = if let Some(icon_src) = props_icon {
        rsx! {
            img {
                class: generate_icon_style(),
                src: "{icon_src}",
                width: "{props_icon_size}",
                height: "{props_icon_size}",
                onload: handle_icon_load,
                alt: "QR Code Icon"
            }
        }
    } else {
        rsx!()
    };

    // 渲染错误信息
    let error_element = if let Some(err_msg) = error.read().as_ref() {
        rsx! {
            div {
                style: "color: #ff4d4f; padding: 8px;",
                "{err_msg}"
            }
        }
    } else {
        rsx!()
    };

    let props_size = props.clone().size.clone();
    let props_children = props.clone().children;

    // 渲染二维码组件
    rsx! {
        div {
            class: "{combined_class}",
            style: "{combined_style}",
            onmounted: move |el| container_ref.set(Some(el.data().downcast::<web_sys::Element>().unwrap().clone())),

            // 二维码容器
            div {
                onmounted: move |el| qr_code_ref.set(Some(el.data().downcast::<web_sys::Element>().unwrap().clone())),
                style: "width: {props_size}px; height: {props_size}px;"
            }

            // 图标
            {icon_element}

            // 状态遮罩
            {status_mask}

            // 错误信息
            {error_element}

            // 子元素
            {props_children}
        }
    }
}

/// 渲染SVG格式的二维码
fn render_svg_qrcode(
    props: &QRCodeProps,
    element: web_sys::Element,
    mut error: Signal<Option<String>>,
) {
    // 创建图标设置
    let icon_size = props.icon_size.unwrap_or(40);
    let icon_settings = props.icon.as_ref().map(|src| QRCodeIconSettings {
        src: src.clone(),
        width: icon_size,
        height: icon_size,
        ..Default::default()
    });

    // 生成SVG
    let size = match props.size {
        crate::components::qr_code::types::QRCodeSize::Small => 120,
        crate::components::qr_code::types::QRCodeSize::Medium => 160,
        crate::components::qr_code::types::QRCodeSize::Large => 200,
        crate::components::qr_code::types::QRCodeSize::Custom(s) => s,
    };
    let color = props.color.as_deref().unwrap_or("#000000");
    let bg_color = props.bg_color.as_deref().unwrap_or("transparent");

    match generate_qrcode_svg(
        &props.value,
        size,
        color,
        bg_color,
        props.error_level,
        icon_settings.as_ref(),
    ) {
        Ok(svg_content) => {
            // 设置SVG内容
            element.set_inner_html(&svg_content);
            error.set(None);
        }
        Err(err) => {
            error.set(Some(err));
        }
    }
}

/// 渲染Canvas格式的二维码
fn render_canvas_qrcode(
    props: &QRCodeProps,
    element: web_sys::Element,
    mut error: Signal<Option<String>>,
) {
    // 清空容器
    element.set_inner_html("");

    // 创建Canvas元素
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.create_element("canvas").unwrap();
    let canvas = canvas.dyn_into::<HtmlCanvasElement>().unwrap();

    // 获取尺寸和颜色
    let size = match props.size {
        crate::components::qr_code::types::QRCodeSize::Small => 120,
        crate::components::qr_code::types::QRCodeSize::Medium => 160,
        crate::components::qr_code::types::QRCodeSize::Large => 200,
        crate::components::qr_code::types::QRCodeSize::Custom(s) => s,
    };
    let color = props.color.as_deref().unwrap_or("#000000");
    let bg_color = props.bg_color.as_deref().unwrap_or("transparent");

    // 设置Canvas大小
    canvas.set_width(size);
    canvas.set_height(size);

    // 添加Canvas到容器
    element.append_child(&canvas).unwrap();

    // 获取绘图上下文
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    // 绘制背景
    context.set_fill_style(&wasm_bindgen::JsValue::from_str(bg_color));
    context.fill_rect(0.0, 0.0, size as f64, size as f64);

    // 生成二维码数据URL
    match generate_qrcode_data_url(&props.value, size, color, bg_color, props.error_level) {
        Ok(data_url) => {
            // 创建图片元素
            let img = HtmlImageElement::new().unwrap();

            // 设置图片加载完成后的回调
            let context_clone = context.clone();
            let canvas_width = size as f64;
            let canvas_height = size as f64;
            let mut error_clone = error.clone();

            let img_clone = img.clone();
            let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
                // 绘制二维码图片
                context_clone
                    .draw_image_with_html_image_element(&img_clone, canvas_width, canvas_height)
                    .unwrap();

                error_clone.set(None);
            }) as Box<dyn FnMut()>);

            img.set_onload(Some(closure.as_ref().unchecked_ref()));
            img.set_src(&data_url);

            // 保持闭包存活
            closure.forget();
        }
        Err(err) => {
            error.set(Some(err));
        }
    }
}

/// 渲染状态遮罩
fn render_status_mask(status_info: StatusRenderInfo) -> Element {
    let status = status_info.status;
    let status_text = get_status_text(status);
    let status_icon_class = get_status_icon_style(status);

    // 刷新按钮（仅在过期状态显示）
    let refresh_button = if status == QRCodeStatus::Expired && status_info.on_refresh.is_some() {
        let handle_refresh = move |_| {
            if let Some(on_refresh) = &status_info.on_refresh {
                on_refresh.call(());
            }
        };

        rsx! {
            button {
                class: generate_refresh_button_style(),
                onclick: handle_refresh,
                "刷新"
            }
        }
    } else {
        rsx!()
    };

    // 状态图标
    let status_icon = match status {
        QRCodeStatus::Loading => {
            rsx! {
                div {
                    class: "{status_icon_class}",
                }
            }
        }
        QRCodeStatus::Expired => {
            rsx! {
                Icon {
                    icon_type: Some(IconType::Close),
                    class_name: Some(status_icon_class),
                    theme: None,
                    rotate: Some(0),
                    spin: false,
                    two_tone_color: None,
                    component: None,
                    style: None,
                    on_click: None,
                    size: None,
                    disabled:false,
                }
            }
        }
        QRCodeStatus::Scanned => {
            rsx! {
                Icon {
                    icon_type: Some(IconType::Check),
                    class_name: Some(status_icon_class),
                    theme: None,
                    rotate: Some(0),
                    spin: false,
                    two_tone_color: None,
                    component: None,
                    style: None,
                    on_click: None,
                    size: None,
                    disabled:false,
                }
            }
        }
        _ => rsx!(),
    };

    // 渲染状态遮罩
    rsx! {
        div {
            class: generate_status_mask_style(),

            // 状态图标
            {status_icon}

            // 状态文本
            div {
                class: generate_status_text_style(),
                "{status_text}"
            }

            // 刷新按钮
            {refresh_button}
        }
    }
}

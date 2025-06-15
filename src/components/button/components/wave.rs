use dioxus::prelude::*;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{window, HtmlElement};

/// 波纹效果组件属性
#[derive(Props, PartialEq, Clone)]
pub struct WaveProps {
    /// 子元素
    pub children: Element,

    /// 禁用波纹效果
    #[props(default = false)]
    pub disabled: bool,

    /// 自定义波纹颜色
    #[props(into, optional)]
    pub color: Option<String>,

    /// 波纹持续时间(毫秒)
    #[props(default = 500)]
    pub duration: u32,

    /// 波纹透明度
    #[props(default = 0.6)]
    pub opacity: f32,
}

/// 波纹效果组件
#[component]
pub fn Wave(props: WaveProps) -> Element {
    let node_ref = use_node_ref();
    let ripple_active = use_state(|| false);
    let animation_in_progress = use_state(|| false);

    // 波纹元素的引用
    let ripple_ref = use_node_ref();

    // 波纹样式状态
    let ripple_style = use_state(|| String::from(""));

    // 处理点击事件
    let handle_click = move |_| {
        if props.disabled || *animation_in_progress.get() {
            return;
        }

        animation_in_progress.set(true);

        if let Some(element) = node_ref.get() {
            if let Some(dom_node) = element.get_raw_element() {
                if let Some(html_element) = dom_node.dyn_ref::<HtmlElement>() {
                    // 获取元素尺寸和位置
                    let rect = html_element.get_bounding_client_rect();
                    let width = rect.width();
                    let height = rect.height();

                    // 计算波纹尺寸 (取宽高的较大值的2.5倍，确保波纹能覆盖整个按钮)
                    let ripple_size = (width.max(height) * 2.5) as i32;

                    // 获取点击位置 (默认为中心点)
                    let x = width / 2.0;
                    let y = height / 2.0;

                    // 计算波纹位置 (居中)
                    let left = x - (ripple_size as f64) / 2.0;
                    let top = y - (ripple_size as f64) / 2.0;

                    // 设置波纹样式
                    let color = props
                        .color
                        .clone()
                        .unwrap_or_else(|| "currentColor".to_string());
                    let style = format!(
                        "position: absolute; \
                         left: {}px; \
                         top: {}px; \
                         width: {}px; \
                         height: {}px; \
                         border-radius: 50%; \
                         background-color: {}; \
                         opacity: {}; \
                         transform: scale(0); \
                         pointer-events: none; \
                         transition: transform {}ms cubic-bezier(0.08, 0.82, 0.17, 1), opacity {}ms cubic-bezier(0.08, 0.82, 0.17, 1);",
                        left, top, ripple_size, ripple_size, color, props.opacity, props.duration, props.duration
                    );

                    ripple_style.set(style);
                    ripple_active.set(true);

                    // 使用requestAnimationFrame确保DOM更新后再触发动画
                    let window = window().expect("window should be available");
                    let ripple_ref_clone = ripple_ref.clone();

                    let callback = Closure::once(move || {
                        if let Some(ripple_element) = ripple_ref_clone.get() {
                            if let Some(dom_node) = ripple_element.get_raw_element() {
                                if let Some(html_element) = dom_node.dyn_ref::<HtmlElement>() {
                                    // 触发波纹扩散动画
                                    let _ =
                                        html_element.style().set_property("transform", "scale(1)");
                                    let _ = html_element.style().set_property("opacity", "0");
                                }
                            }
                        }
                    });

                    let _ = window.request_animation_frame(callback.as_ref().unchecked_ref());
                    callback.forget();

                    // 动画结束后清理
                    let animation_in_progress = animation_in_progress.clone();
                    let ripple_active = ripple_active.clone();
                    let duration = props.duration;

                    // 使用JavaScript的setTimeout代替tokio::time::sleep
                    let cleanup_callback = Closure::once(move || {
                        ripple_active.set(false);
                        animation_in_progress.set(false);
                    });

                    let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                        cleanup_callback.as_ref().unchecked_ref(),
                        duration,
                    );
                    cleanup_callback.forget();
                }
            }
        }
    };

    rsx! {
        div {
            ref: node_ref,
            class: "ant-wave-wrapper",
            style: "position: relative; display: inline-block; width: 100%; height: 100%; overflow: hidden;",
            onclick: handle_click,

            // 渲染波纹元素
            {
                if *ripple_active.get() {
                    rsx! {
                        div {
                            ref: ripple_ref,
                            class: "ant-wave-ripple",
                            style: "{ripple_style}",
                        }
                    }
                }
            }

            // 渲染子元素
            {props.children}
        }
    }
}

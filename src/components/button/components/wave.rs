use dioxus::prelude::*;
use std::rc::Rc;
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
    #[props(default = None)]
    pub color: Option<String>,

    /// 波纹持续时间(毫秒)
    #[props(default = 500)]
    pub duration: u32,

    /// 波纹透明度
    #[props(default = 0.2)]
    pub opacity: f32,
}

/// 波纹效果组件
#[component]
pub fn Wave(props: WaveProps) -> Element {
    let mut node_ref = use_signal(|| None::<HtmlElement>);
    let mut ripple_active = use_signal(|| false);
    let mut animation_in_progress = use_signal(|| false);

    // 波纹元素的引用
    let mut ripple_ref = use_signal(|| None::<HtmlElement>);

    // 波纹样式状态
    let mut ripple_style = use_signal(|| String::from(""));

    // 处理点击事件
    let handle_click = move |_: MouseEvent| {
        if props.disabled || animation_in_progress() {
            return;
        }

        animation_in_progress.set(true);

        if let Some(element) = node_ref() {
            // 获取元素尺寸和位置
            let rect = element.get_bounding_client_rect();

            // 计算波纹大小和位置
            let ripple_size = calculate_ripple_size(&rect);
            let ripple_position = calculate_ripple_position(&rect);

            // 设置波纹样式
            let color = props
                .color
                .clone()
                .unwrap_or_else(|| "rgba(0, 0, 0, 0.15)".to_string());
            let duration = props.duration;
            let opacity = props.opacity;

            ripple_style.set(format!(
                "position: absolute; \
                top: {}px; \
                left: {}px; \
                width: {}px; \
                height: {}px; \
                border-radius: 50%; \
                background-color: {}; \
                opacity: {}; \
                transform: scale(0); \
                transition: transform {}ms cubic-bezier(0.08, 0.82, 0.17, 1), opacity {}ms cubic-bezier(0.08, 0.82, 0.17, 1); \
                pointer-events: none; \
                z-index: 0;",
                ripple_position.0, ripple_position.1, ripple_size, ripple_size,
                color, opacity, duration, duration
            ));

            // 激活波纹
            ripple_active.set(true);

            // 使用requestAnimationFrame确保DOM更新后执行动画
            if let Some(window) = window() {
                let ripple_ref_clone = ripple_ref.clone();
                let animation_frame_callback = Closure::once(move || {
                    if let Some(ripple_element) = ripple_ref_clone() {
                        // 触发重排并应用变换
                        let _ = ripple_element.offset_height();
                        let _ = ripple_element.style().set_property("transform", "scale(1)");
                    }
                });

                let _ = window
                    .request_animation_frame(animation_frame_callback.as_ref().unchecked_ref());
                animation_frame_callback.forget();

                // 设置动画结束后的清理
                let cleanup_callback = Closure::once(move || {
                    ripple_active.set(false);
                    animation_in_progress.set(false);
                });

                let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                    cleanup_callback.as_ref().unchecked_ref(),
                    duration as i32,
                );
                cleanup_callback.forget();
            }
        }
    };

    // 在组件挂载时保存元素引用
    let on_mount = move |event: Event<MountedData>| {
        let element = event.data();
        if let Some(html_element) = element.downcast::<HtmlElement>() {
            node_ref.set(Some(html_element.clone()));
        }
    };

    // 在波纹元素挂载时保存引用
    let on_ripple_mount = move |event: Event<MountedData>| {
        // 获取挂载的元素
        if let Some(element) = event.data().downcast::<HtmlElement>() {
            ripple_ref.set(Some(element.clone()));
        }
    };

    rsx! {
        div {
            class: "ant-wave-wrapper",
            style: "position: relative; display: inline-block; width: 100%;",
            onclick: handle_click,
            onmounted: on_mount,

            // 渲染子元素
            {props.children}

            // 渲染波纹元素
            {
                if ripple_active() {
                    rsx! {
                        div {
                            class: "ant-wave-ripple",
                            style: "{ripple_style}",
                            onmounted: on_ripple_mount,
                        }
                    }
                } else {
                    rsx! {}
                }
            }
        }
    }
}

/// 计算波纹大小
fn calculate_ripple_size(rect: &web_sys::DomRect) -> f64 {
    let width = rect.width();
    let height = rect.height();
    (width.max(height) * 2.5).max(60.0) // 确保最小尺寸
}

/// 计算波纹位置
fn calculate_ripple_position(rect: &web_sys::DomRect) -> (f64, f64) {
    let width = rect.width();
    let height = rect.height();

    // 居中放置波纹
    let x = (width / 2.0) - (width * 1.25);
    let y = (height / 2.0) - (height * 1.25);

    (x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试 Wave 组件的基本属性
    #[test]
    fn test_wave_basic_props() {
        let props = WaveProps {
            disabled: false,
            children: VNode::empty(),
            color: None,
            duration: 500,
            opacity: 0.2,
        };
        assert_eq!(props.disabled, false);
        assert_eq!(props.duration, 500);
        assert_eq!(props.opacity, 0.2);
    }

    /// 测试 Wave 组件的禁用状态
    #[test]
    fn test_wave_disabled() {
        let props = WaveProps {
            disabled: true,
            children: VNode::empty(),
            color: None,
            duration: 500,
            opacity: 0.2,
        };
        assert_eq!(props.disabled, true);
    }

    /// 测试 Wave 组件的启用状态
    #[test]
    fn test_wave_enabled() {
        let props = WaveProps {
            disabled: false,
            children: VNode::empty(),
            color: None,
            duration: 500,
            opacity: 0.2,
        };
        assert_eq!(props.disabled, false);
    }

    /// 测试 WaveProps 的自定义颜色
    #[test]
    fn test_wave_custom_color() {
        let props = WaveProps {
            disabled: false,
            children: VNode::empty(),
            color: Some("rgba(255, 0, 0, 0.3)".to_string()),
            duration: 500,
            opacity: 0.2,
        };
        assert_eq!(props.color, Some("rgba(255, 0, 0, 0.3)".to_string()));
    }

    /// 测试 calculate_ripple_size 函数
    #[test]
    fn test_calculate_ripple_size() {
        // 创建一个模拟的 DomRect
        let rect = web_sys::DomRect::new().unwrap();
        // 注意：在实际测试中，需要设置 rect 的宽度和高度
        // 这里只是演示测试结构
        let size = calculate_ripple_size(&rect);
        assert!(size >= 60.0); // 确保最小尺寸
    }

    /// 测试 calculate_ripple_position 函数
    #[test]
    fn test_calculate_ripple_position() {
        // 创建一个模拟的 DomRect
        let rect = web_sys::DomRect::new().unwrap();
        // 注意：在实际测试中，需要设置 rect 的宽度和高度
        // 这里只是演示测试结构
        let (x, y) = calculate_ripple_position(&rect);
        // 验证位置计算是否正确
        // 由于 DomRect::new() 创建的矩形默认宽高为0，所以x和y都应该是0
        assert_eq!(x, 0.0);
        assert_eq!(y, 0.0);
    }

    /// 测试 Wave 组件的复杂配置属性
    #[test]
    fn test_wave_complex_configuration() {
        let props = WaveProps {
            disabled: false,
            children: VNode::empty(),
            color: Some("rgba(0, 255, 0, 0.4)".to_string()),
            duration: 800,
            opacity: 0.3,
        };
        assert_eq!(props.disabled, false);
        assert_eq!(props.color, Some("rgba(0, 255, 0, 0.4)".to_string()));
        assert_eq!(props.duration, 800);
        assert_eq!(props.opacity, 0.3);
    }
}

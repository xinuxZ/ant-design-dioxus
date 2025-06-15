use dioxus::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element as WebElement;

/// Wave 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct WaveProps {
    /// 子元素
    #[props(default)]
    pub children: Element,

    /// 波纹颜色
    #[props(into, default = "rgba(255, 255, 255, 0.3)".to_string())]
    pub color: String,

    /// 禁用波纹效果
    #[props(default)]
    pub disabled: bool,
}

/// Wave 组件 - 实现波纹效果
#[component]
pub fn Wave(props: WaveProps) -> Element {
    let target_ref = use_ref(|| None::<WebElement>);

    // 使用 use_effect 在组件挂载后设置事件监听
    use_effect(move || {
        if props.disabled {
            return (|| {})();
        }

        let current_target = target_ref.read().clone();

        if let Some(element) = current_target {
            let element_clone = element.clone();

            // 添加点击事件监听器
            let listener = move |event: web_sys::MouseEvent| {
                create_ripple(&element_clone, event, &props.color);
            };

            let closure = wasm_bindgen::closure::Closure::wrap(
                Box::new(listener) as Box<dyn FnMut(web_sys::MouseEvent)>
            );

            element
                .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
                .expect("Failed to add event listener");

            // 保持闭包活跃，防止被回收
            closure.forget();
        }

        (|| {})()
    });

    rsx! {
        div {
            ref: move |el| {
                if let Some(element) = el {
                    if let Some(web_element) = element.get_raw_element() {
                        *target_ref.write() = Some(web_element.clone());
                    }
                }
            },
            style: "position: relative; overflow: hidden; display: inline-block;",
            {props.children}
        }
    }
}

/// 创建波纹效果
fn create_ripple(element: &WebElement, event: web_sys::MouseEvent, color: &str) {
    // 获取元素的位置和大小
    let rect = element.get_bounding_client_rect();

    // 计算点击位置相对于元素的坐标
    let x = event.client_x() as f64 - rect.left();
    let y = event.client_y() as f64 - rect.top();

    // 计算波纹大小 (取元素宽高的较大值)
    let size = f64::max(rect.width(), rect.height());
    let diameter = size * 2.0;

    // 创建波纹元素
    let window = web_sys::window().expect("No global window exists");
    let document = window.document().expect("No document exists");
    let ripple = document
        .create_element("span")
        .expect("Failed to create element");

    // 设置波纹样式
    let style = ripple.dyn_into::<web_sys::HtmlElement>().unwrap().style();
    style.set_property("position", "absolute").unwrap();
    style.set_property("border-radius", "50%").unwrap();
    style.set_property("pointer-events", "none").unwrap();
    style.set_property("transform", "scale(0)").unwrap();
    style.set_property("background-color", color).unwrap();
    style.set_property("opacity", "1").unwrap();
    style
        .set_property("transition", "transform 0.6s, opacity 0.6s")
        .unwrap();
    style
        .set_property("width", &format!("{}px", diameter))
        .unwrap();
    style
        .set_property("height", &format!("{}px", diameter))
        .unwrap();
    style
        .set_property("left", &format!("{}px", x - diameter / 2.0))
        .unwrap();
    style
        .set_property("top", &format!("{}px", y - diameter / 2.0))
        .unwrap();

    // 添加波纹元素到父元素
    element
        .append_child(&ripple)
        .expect("Failed to append child");

    // 触发动画
    window.request_animation_frame(move |_| {
        let style = ripple.style();
        style.set_property("transform", "scale(1)").unwrap();
        style.set_property("opacity", "0").unwrap();
    });

    // 动画结束后移除波纹元素
    let ripple_clone = ripple.clone();
    let window_clone = window.clone();

    let cleanup = move || {
        if let Some(parent) = ripple_clone.parent_node() {
            let _ = parent.remove_child(&ripple_clone);
        }
    };

    let cleanup_closure =
        wasm_bindgen::closure::Closure::wrap(Box::new(cleanup) as Box<dyn FnMut()>);

    window_clone
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            cleanup_closure.as_ref().unchecked_ref(),
            650, // 略长于动画时间，确保动画完成
        )
        .expect("Failed to set timeout");

    cleanup_closure.forget();
}

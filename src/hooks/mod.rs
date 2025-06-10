//! 通用钩子函数模块
//!
//! 提供组件库中使用的通用钩子函数

use dioxus::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures;

/// 使用防抖函数
///
/// 防止函数在短时间内被多次调用
///
/// # 参数
///
/// * `func` - 要防抖的函数
/// * `delay` - 延迟时间（毫秒）
///
/// # 返回值
///
/// 防抖后的函数
pub fn use_debounce<F>(func: F, delay: u32) -> impl FnMut() + 'static
where
    F: Fn() + 'static,
{
    let func = std::rc::Rc::new(func);
    let mut timeout_id = use_signal(|| None::<i32>);

    move || {
        let func = func.clone();
        if let Some(id) = *timeout_id.read() {
            // 清除之前的定时器
            let window = web_sys::window().expect("no global `window` exists");
            window.clear_timeout_with_handle(id);
        }

        let window = web_sys::window().expect("no global `window` exists");
        let closure = wasm_bindgen::closure::Closure::once(move || {
            func();
        });

        let id = window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                closure.as_ref().unchecked_ref(),
                delay as i32,
            )
            .expect("failed to set timeout");

        timeout_id.set(Some(id));
    }
}

/// 使用节流函数
///
/// 限制函数在一定时间内只能被调用一次
///
/// # 参数
///
/// * `func` - 要节流的函数
/// * `delay` - 延迟时间（毫秒）
///
/// # 返回值
///
/// 节流后的函数
pub fn use_throttle<F>(func: F, delay: u32) -> impl FnMut() + 'static
where
    F: Fn() + 'static,
{
    let func = std::rc::Rc::new(func);
    let mut last_run = use_signal(|| 0.0);

    move || {
        let now = js_sys::Date::now();
        if now - *last_run.read() > delay as f64 {
            func();
            last_run.set(now);
        }
    }
}

/// 使用本地存储
///
/// 在浏览器的 localStorage 中存储状态
///
/// # 参数
///
/// * `key` - 存储键名
/// * `initial_value` - 初始值
///
/// # 返回值
///
/// 存储的值和更新函数
pub fn use_local_storage<T>(key: &str, initial_value: T) -> (Signal<T>, impl FnMut(T))
where
    T: Clone + serde::Serialize + for<'de> serde::Deserialize<'de> + 'static,
{
    let key = key.to_string();
    let value = use_signal(|| {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(value_str)) = storage.get_item(&key) {
                    if let Ok(value) = serde_json::from_str(&value_str) {
                        return value;
                    }
                }
            }
        }
        initial_value.clone()
    });

    let set_value = {
        let key = key.clone();
        let mut value = value.clone();
        move |new_value: T| {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    if let Ok(value_str) = serde_json::to_string(&new_value) {
                        let _ = storage.set_item(&key, &value_str);
                    }
                }
            }
            value.set(new_value);
        }
    };

    (value, set_value)
}

/// 使用会话存储
///
/// 在浏览器的 sessionStorage 中存储状态
///
/// # 参数
///
/// * `key` - 存储键名
/// * `initial_value` - 初始值
///
/// # 返回值
///
/// 存储的值和更新函数
pub fn use_session_storage<T>(key: &str, initial_value: T) -> (Signal<T>, impl FnMut(T))
where
    T: Clone + serde::Serialize + for<'de> serde::Deserialize<'de> + 'static,
{
    let key = key.to_string();
    let value = use_signal(|| {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.session_storage() {
                if let Ok(Some(value_str)) = storage.get_item(&key) {
                    if let Ok(value) = serde_json::from_str(&value_str) {
                        return value;
                    }
                }
            }
        }
        initial_value.clone()
    });

    let set_value = {
        let key = key.clone();
        let mut value = value.clone();
        move |new_value: T| {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.session_storage() {
                    if let Ok(value_str) = serde_json::to_string(&new_value) {
                        let _ = storage.set_item(&key, &value_str);
                    }
                }
            }
            value.set(new_value);
        }
    };

    (value, set_value)
}

/// 使用媒体查询
///
/// 检测媒体查询是否匹配
///
/// # 参数
///
/// * `query` - CSS 媒体查询字符串
///
/// # 返回值
///
/// 是否匹配
pub fn use_media_query(query: &str) -> bool {
    let matches = use_signal(|| false);
    let query = query.to_string();

    use_effect(move || {
        if let Some(window) = web_sys::window() {
            if let Ok(media_query_list) = window.match_media(&query) {
                if let Some(media_query_list) = media_query_list {
                    let mut matches = matches.clone();
                    matches.set(media_query_list.matches());

                    let mut matches_clone = matches.clone();
                    let listener = wasm_bindgen::closure::Closure::wrap(Box::new(
                        move |e: web_sys::MediaQueryListEvent| {
                            matches_clone.set(e.matches());
                        },
                    )
                        as Box<dyn FnMut(_)>);

                    media_query_list
                        .add_event_listener_with_callback(
                            "change",
                            listener.as_ref().unchecked_ref(),
                        )
                        .ok();

                    // 保持闭包活着
                    listener.forget();
                }
            }
        }
    });

    let value = *matches.read();
    value
}

/// 使用窗口尺寸
///
/// 获取当前窗口的宽度和高度
///
/// # 返回值
///
/// 窗口宽度和高度的信号
pub fn use_window_size() -> (Signal<u32>, Signal<u32>) {
    let width = use_signal(|| {
        web_sys::window()
            .map(|window| window.inner_width().ok())
            .flatten()
            .and_then(|width| width.as_f64())
            .map(|w| w as u32)
            .unwrap_or(0)
    });

    let height = use_signal(|| {
        web_sys::window()
            .map(|window| window.inner_height().ok())
            .flatten()
            .and_then(|height| height.as_f64())
            .map(|h| h as u32)
            .unwrap_or(0)
    });

    use_effect(move || {
        let window = match web_sys::window() {
            Some(window) => window,
            None => return,
        };

        let mut width_clone = width.clone();
        let mut height_clone = height.clone();

        let resize_callback = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
            if let Some(window) = web_sys::window() {
                if let Ok(w) = window.inner_width() {
                    if let Some(w) = w.as_f64() {
                        width_clone.set(w as u32);
                    }
                }

                if let Ok(h) = window.inner_height() {
                    if let Some(h) = h.as_f64() {
                        height_clone.set(h as u32);
                    }
                }
            }
        }) as Box<dyn FnMut()>);

        window
            .add_event_listener_with_callback("resize", resize_callback.as_ref().unchecked_ref())
            .expect("failed to add resize event listener");

        // 返回清理函数
        move || {
            let cleanup_callback = resize_callback.as_ref().unchecked_ref();
            if let Some(window) = web_sys::window() {
                window
                    .remove_event_listener_with_callback("resize", cleanup_callback)
                    .expect("failed to remove resize event listener");
            }
        }
    });

    (width, height)
}

/// 使用文档可见性
///
/// 检测文档是否可见（页面是否在前台）
///
/// # 返回值
///
/// 文档是否可见
pub fn use_document_visibility() -> Signal<bool> {
    let is_visible = use_signal(|| {
        web_sys::window()
            .and_then(|window| window.document())
            .map(|document| {
                let visibility_state = document.visibility_state();
                visibility_state == web_sys::VisibilityState::Visible
            })
            .unwrap_or(true)
    });

    use_effect(move || {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                let mut is_visible_clone = is_visible.clone();
                let callback = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
                    if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                        let visibility_state = document.visibility_state();
                        is_visible_clone.set(visibility_state == web_sys::VisibilityState::Visible);
                    }
                })
                    as Box<dyn FnMut()>);

                document
                    .add_event_listener_with_callback(
                        "visibilitychange",
                        callback.as_ref().unchecked_ref(),
                    )
                    .ok();

                // 保持闭包活着
                callback.forget();
            }
        }
    });

    is_visible
}

/// 使用在线状态
///
/// 检测用户是否在线
///
/// # 返回值
///
/// 用户是否在线
pub fn use_online_status() -> Signal<bool> {
    let is_online = use_signal(|| {
        web_sys::window()
            .and_then(|window| {
                let navigator = window.navigator();
                Some(navigator.on_line())
            })
            .unwrap_or(true)
    });

    use_effect(move || {
        if let Some(window) = web_sys::window() {
            // 在线事件处理
            let mut is_online_clone = is_online.clone();
            let online_callback = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
                is_online_clone.set(true);
            })
                as Box<dyn FnMut()>);

            // 离线事件处理
            let mut is_online_clone = is_online.clone();
            let offline_callback = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
                is_online_clone.set(false);
            })
                as Box<dyn FnMut()>);

            // 添加事件监听
            window
                .add_event_listener_with_callback(
                    "online",
                    online_callback.as_ref().unchecked_ref(),
                )
                .ok();
            window
                .add_event_listener_with_callback(
                    "offline",
                    offline_callback.as_ref().unchecked_ref(),
                )
                .ok();

            // 保持闭包活着
            online_callback.forget();
            offline_callback.forget();
        }
    });

    is_online
}

/// 使用剪贴板
///
/// 提供复制文本到剪贴板的功能
///
/// # 返回值
///
/// 复制函数和复制状态
pub fn use_clipboard() -> (impl Fn(&str), Signal<bool>) {
    let copied = use_signal(|| false);

    let copy_to_clipboard = {
        let copied = copied.clone();
        move |text: &str| {
            let mut copied = copied.clone();
            copied.set(false);

            if let Some(window) = web_sys::window() {
                let navigator = window.navigator();
                let clipboard = navigator.clipboard();
                let text = text.to_string();
                wasm_bindgen_futures::spawn_local({
                    let mut copied = copied.clone();
                    async move {
                        let result = js_sys::Promise::from(clipboard.write_text(&text));
                        let _ = wasm_bindgen_futures::JsFuture::from(result).await;
                        copied.set(true);

                        // 2秒后重置状态
                        let window = web_sys::window().unwrap();
                        let mut copied_clone = copied.clone();
                        let closure = wasm_bindgen::closure::Closure::once(move || {
                            copied_clone.set(false);
                        });

                        window
                            .set_timeout_with_callback_and_timeout_and_arguments_0(
                                closure.as_ref().unchecked_ref(),
                                2000,
                            )
                            .ok();
                    }
                });
            }
        }
    };

    (copy_to_clipboard, copied)
}

/// 使用滚动位置
///
/// 获取元素的滚动位置
///
/// # 参数
///
/// * `target_ref` - 目标元素的引用
///
/// # 返回值
///
/// 水平和垂直滚动位置
pub fn use_scroll_position(target_ref: web_sys::HtmlDivElement) -> (Signal<f64>, Signal<f64>) {
    let mut scroll_x = use_signal(|| 0.0);
    let mut scroll_y = use_signal(|| 0.0);

    use_effect(move || {
        // 初始化滚动位置
        scroll_x.set(target_ref.scroll_left() as f64);
        scroll_y.set(target_ref.scroll_top() as f64);

        // 监听滚动事件
        let mut scroll_x_clone = scroll_x.clone();
        let mut scroll_y_clone = scroll_y.clone();
        let target_ref_clone = target_ref.clone();

        let callback = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
            scroll_x_clone.set(target_ref_clone.scroll_left() as f64);
            scroll_y_clone.set(target_ref_clone.scroll_top() as f64);
        }) as Box<dyn FnMut()>);

        target_ref
            .add_event_listener_with_callback("scroll", callback.as_ref().unchecked_ref())
            .ok();

        // 保持闭包活着
        callback.forget();
    });

    (scroll_x, scroll_y)
}

/// 使用长按
///
/// 检测元素是否被长按
///
/// # 参数
///
/// * `delay` - 长按所需的时间（毫秒）
///
/// # 返回值
///
/// 长按状态信号，开始长按函数，结束长按函数
pub fn use_long_press(
    delay: u32,
) -> (Signal<bool>, impl FnMut() + 'static, impl FnMut() + 'static) {
    let mut is_pressed = use_signal(|| false);
    let timeout_id = use_signal(|| None::<i32>);

    let start_press = {
        let is_pressed = is_pressed.clone();
        let mut timeout_id = timeout_id.clone();

        move || {
            // 如果已经在按下状态，不做任何处理
            if *is_pressed.read() {
                return;
            }

            let mut is_pressed_clone = is_pressed.clone();
            let window = web_sys::window().expect("no global `window` exists");

            // 创建定时器，在指定时间后将状态设置为长按
            let closure = wasm_bindgen::closure::Closure::once(move || {
                is_pressed_clone.set(true);
            });

            let id = window
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    closure.as_ref().unchecked_ref(),
                    delay as i32,
                )
                .expect("failed to set timeout");

            timeout_id.set(Some(id));
        }
    };

    let end_press = {
        let mut is_pressed = is_pressed.clone();
        let mut timeout_id = timeout_id.clone();

        move || {
            is_pressed.set(false);

            // 清除定时器
            let timeout_id_value = *timeout_id.read();
            if let Some(id) = timeout_id_value {
                let window = web_sys::window().expect("no global `window` exists");
                window.clear_timeout_with_handle(id);
                timeout_id.set(None);
            }
        }
    };

    (is_pressed, start_press, end_press)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_use_debounce() {
        // 由于涉及到 DOM API，无法在纯 Rust 环境中测试
        // 这里只是一个占位测试
        assert!(true);
    }

    #[test]
    fn test_use_throttle() {
        // 由于涉及到 DOM API，无法在纯 Rust 环境中测试
        // 这里只是一个占位测试
        assert!(true);
    }
}

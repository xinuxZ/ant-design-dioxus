//! 样式注入器
//!
//! 用于将CSS样式字符串注入到DOM文档中

use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;
use web_sys::{window, Document, Element};

/// 已注入样式的缓存
static INJECTED_STYLES: Lazy<Mutex<HashMap<String, String>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// 获取文档对象
fn get_document() -> Option<Document> {
    window().and_then(|win| win.document())
}

/// 获取head元素
fn get_head() -> Option<Element> {
    get_document().and_then(|doc| doc.head().map(|head| head.into()))
}

/// 创建style元素
fn create_style_element(id: &str, css: &str) -> Option<Element> {
    get_document().and_then(|doc| {
        let element = doc.create_element("style").ok()?;
        element.set_id(id);
        element.set_text_content(Some(css));
        Some(element)
    })
}

/// 注入CSS样式到文档头部
///
/// # 参数
///
/// * `id` - 样式元素的唯一标识符
/// * `css` - CSS样式字符串
///
/// # 返回值
///
/// 如果成功注入样式则返回 `true`，否则返回 `false`
///
/// # 示例
///
/// ```rust
/// use crate::utils::style_injector::inject_style;
///
/// let success = inject_style("my-component-styles", ".my-class { color: red; }");
/// if success {
///     println!("样式注入成功");
/// }
/// ```
pub fn inject_style(id: &str, css: &str) -> bool {
    // 检查是否已经注入过相同的样式
    if let Ok(cache) = INJECTED_STYLES.lock() {
        if let Some(cached_css) = cache.get(id) {
            if cached_css == css {
                return true; // 样式已存在且相同，无需重复注入
            }
        }
    }

    // 如果已存在同ID的样式元素，则先移除
    if let Some(doc) = get_document() {
        if let Some(existing) = doc.get_element_by_id(id) {
            if let Some(parent) = existing.parent_node() {
                parent.remove_child(&existing).ok();
            }
        }
    }

    // 创建新的style元素
    if let Some(style_element) = create_style_element(id, css) {
        // 将style元素添加到head中
        if let Some(head) = get_head() {
            if head.append_child(&style_element).is_ok() {
                // 更新缓存
                if let Ok(mut cache) = INJECTED_STYLES.lock() {
                    cache.insert(id.to_string(), css.to_string());
                }
                return true;
            }
        }
    }

    false
}

/// 移除已注入的样式
///
/// # 参数
///
/// * `id` - 要移除的样式元素的唯一标识符
///
/// # 返回值
///
/// 如果成功移除样式则返回 `true`，否则返回 `false`
pub fn remove_style(id: &str) -> bool {
    if let Some(doc) = get_document() {
        if let Some(element) = doc.get_element_by_id(id) {
            if let Some(parent) = element.parent_node() {
                if parent.remove_child(&element).is_ok() {
                    // 从缓存中移除
                    if let Ok(mut cache) = INJECTED_STYLES.lock() {
                        cache.remove(id);
                    }
                    return true;
                }
            }
        }
    }
    false
}

/// 检查样式是否已注入
///
/// # 参数
///
/// * `id` - 样式元素的唯一标识符
///
/// # 返回值
///
/// 如果样式已注入则返回 `true`，否则返回 `false`
pub fn is_style_injected(id: &str) -> bool {
    if let Ok(cache) = INJECTED_STYLES.lock() {
        cache.contains_key(id)
    } else {
        false
    }
}

/// 清除所有已注入的样式
pub fn clear_all_styles() {
    if let Ok(mut cache) = INJECTED_STYLES.lock() {
        for id in cache.keys() {
            remove_style(id);
        }
        cache.clear();
    }
}

/// 获取已注入样式的数量
pub fn get_injected_count() -> usize {
    if let Ok(cache) = INJECTED_STYLES.lock() {
        cache.len()
    } else {
        0
    }
}

/// 获取所有已注入样式的ID列表
pub fn get_injected_ids() -> Vec<String> {
    if let Ok(cache) = INJECTED_STYLES.lock() {
        cache.keys().cloned().collect()
    } else {
        Vec::new()
    }
}

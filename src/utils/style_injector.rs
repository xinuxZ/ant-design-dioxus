//! 样式注入器
//!
//! 用于将CSS样式字符串注入到DOM文档中

use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;
use wasm_bindgen::JsCast;
use web_sys::{window, Document, Element, HtmlHeadElement};

/// 已注入样式的缓存
static INJECTED_STYLES: Lazy<Mutex<HashMap<String, String>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// 获取文档对象
fn get_document() -> Option<Document> {
    window().and_then(|win| win.document())
}

/// 获取head元素
fn get_head() -> Option<HtmlHeadElement> {
    get_document().and_then(|doc| doc.head())
}

/// 创建style元素
fn create_style_element(id: &str, css: &str) -> Option<Element> {
    get_document().and_then(|doc| {
        let element = doc.create_element("style").ok()?;
        element.set_id(id);
        element.set_text_content(Some(css));
        element.set_attribute("type", "text/css").ok()?;
        Some(element)
    })
}

/// 检查样式是否已注入
pub fn is_style_injected(id: &str) -> bool {
    let styles = INJECTED_STYLES.lock().unwrap();
    styles.contains_key(id)
}

/// 将CSS样式注入到文档中
///
/// # Arguments
///
/// * `id` - 样式元素的唯一标识符
/// * `css` - CSS样式字符串
///
/// # Returns
///
/// * `bool` - 是否成功注入样式
pub fn inject_style(id: &str, css: &str) -> bool {
    // 如果已经注入过相同的样式，则不再重复注入
    {
        let mut styles = INJECTED_STYLES.lock().unwrap();
        if let Some(injected_css) = styles.get(id) {
            if injected_css == css {
                return true;
            }
        }
    }

    // 如果已存在同ID的样式元素，则先移除
    if let Some(doc) = get_document() {
        if let Ok(existing) = doc.get_element_by_id(id) {
            if let Some(parent) = existing.parent_node() {
                parent.remove_child(&existing).ok();
            }
        }
    }

    // 创建并插入新样式元素
    let result = if let Some(head) = get_head() {
        if let Some(style) = create_style_element(id, css) {
            head.append_child(&style).is_ok()
        } else {
            false
        }
    } else {
        false
    };

    // 如果成功注入，则更新缓存
    if result {
        let mut styles = INJECTED_STYLES.lock().unwrap();
        styles.insert(id.to_string(), css.to_string());
    }

    result
}

/// 从文档中移除样式
///
/// # Arguments
///
/// * `id` - 样式元素的唯一标识符
///
/// # Returns
///
/// * `bool` - 是否成功移除样式
pub fn remove_style(id: &str) -> bool {
    let result = if let Some(doc) = get_document() {
        if let Ok(element) = doc.get_element_by_id(id) {
            if let Some(parent) = element.parent_node() {
                parent.remove_child(&element).is_ok()
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    };

    if result {
        let mut styles = INJECTED_STYLES.lock().unwrap();
        styles.remove(id);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_style_injection_cache() {
        // 这里无法实际测试DOM操作，但可以测试缓存逻辑
        let style_id = "test-style";
        let css = ".test { color: red; }";

        // 确保初始状态未注入
        {
            let styles = INJECTED_STYLES.lock().unwrap();
            assert!(!styles.contains_key(style_id));
        }

        // 模拟注入成功
        {
            let mut styles = INJECTED_STYLES.lock().unwrap();
            styles.insert(style_id.to_string(), css.to_string());
        }

        assert!(is_style_injected(style_id));

        // 模拟移除
        {
            let mut styles = INJECTED_STYLES.lock().unwrap();
            styles.remove(style_id);
        }

        assert!(!is_style_injected(style_id));
    }
}

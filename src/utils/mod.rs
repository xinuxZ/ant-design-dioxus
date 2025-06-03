//! 工具函数模块
//!
//! 提供组件库中常用的工具函数和类型定义

pub mod class_names;
pub mod responsive;

// 重新导出常用工具
pub use class_names::*;
pub use responsive::*;

/// 检查字符串是否为两个中文字符
///
/// # Arguments
///
/// * `text` - 要检查的文本
///
/// # Returns
///
/// 如果文本恰好包含两个中文字符则返回 true
pub fn is_two_cn_char(text: &str) -> bool {
    let chars: Vec<char> = text.chars().collect();
    chars.len() == 2 && chars.iter().all(|c| is_chinese_char(*c))
}

/// 检查字符是否为中文字符
///
/// # Arguments
///
/// * `c` - 要检查的字符
///
/// # Returns
///
/// 如果是中文字符则返回 true
pub fn is_chinese_char(c: char) -> bool {
    matches!(c as u32,
        0x4E00..=0x9FFF |   // CJK Unified Ideographs
        0x3400..=0x4DBF |   // CJK Extension A
        0x20000..=0x2A6DF |  // CJK Extension B
        0x2A700..=0x2B73F |  // CJK Extension C
        0x2B740..=0x2B81F |  // CJK Extension D
        0x2B820..=0x2CEAF |  // CJK Extension E
        0xF900..=0xFAFF |    // CJK Compatibility Ideographs
        0x2F800..=0x2FA1F    // CJK Compatibility Supplement
    )
}

/// 防抖函数类型
pub type DebounceCallback = Box<dyn Fn() + 'static>;

/// 节流函数类型
pub type ThrottleCallback = Box<dyn Fn() + 'static>;

/// 将像素值转换为字符串
///
/// # Arguments
///
/// * `value` - 像素值
///
/// # Returns
///
/// 格式化的像素字符串
pub fn to_px(value: i32) -> String {
    format!("{}px", value)
}

/// 将数值转换为百分比字符串
///
/// # Arguments
///
/// * `value` - 数值（0.0 到 1.0）
///
/// # Returns
///
/// 格式化的百分比字符串
pub fn to_percent(value: f64) -> String {
    format!("{}%", (value * 100.0).round())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_two_cn_char() {
        assert!(is_two_cn_char("中文"));
        assert!(is_two_cn_char("测试"));
        assert!(!is_two_cn_char("中"));
        assert!(!is_two_cn_char("中文字"));
        assert!(!is_two_cn_char("ab"));
        assert!(!is_two_cn_char("a中"));
    }

    #[test]
    fn test_is_chinese_char() {
        assert!(is_chinese_char('中'));
        assert!(is_chinese_char('文'));
        assert!(!is_chinese_char('a'));
        assert!(!is_chinese_char('1'));
    }

    #[test]
    fn test_to_px() {
        assert_eq!(to_px(10), "10px");
        assert_eq!(to_px(0), "0px");
        assert_eq!(to_px(-5), "-5px");
    }

    #[test]
    fn test_to_percent() {
        assert_eq!(to_percent(0.5), "50%");
        assert_eq!(to_percent(1.0), "100%");
        assert_eq!(to_percent(0.0), "0%");
    }
}

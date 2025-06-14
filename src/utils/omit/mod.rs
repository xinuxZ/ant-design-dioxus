//! 属性忽略工具

use std::collections::HashMap;

/// 忽略特定属性的特征
pub trait Omit {
    /// 返回类型
    type Output;

    /// 忽略指定的属性，返回剩余属性
    fn omit(self) -> Self::Output;
}

/// 为AlertProps实现Omit特征
impl Omit for crate::components::alert::types::AlertProps {
    type Output = HashMap<String, String>;

    fn omit(self) -> Self::Output {
        let mut result = HashMap::new();

        // 将非标准属性添加到结果中
        if let Some(prefix_cls) = self.prefix_cls {
            result.insert("prefix_cls".to_string(), prefix_cls);
        }

        // 可以添加其他需要保留的属性

        result
    }
}

/// 为AlertProps实现Omit特征的引用版本
impl Omit for &crate::components::alert::types::AlertProps {
    type Output = HashMap<String, String>;

    fn omit(self) -> Self::Output {
        let mut result = HashMap::new();

        // 将非标准属性添加到结果中
        if let Some(prefix_cls) = &self.prefix_cls {
            result.insert("prefix_cls".to_string(), prefix_cls.clone());
        }

        // 可以添加其他需要保留的属性

        result
    }
}

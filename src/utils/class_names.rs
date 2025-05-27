//! CSS 类名工具模块
//!
//! 提供类似 JavaScript classNames 库的功能，用于动态组合 CSS 类名

use std::collections::HashMap;

/// CSS 类名构建器
///
/// 用于动态组合和管理 CSS 类名
#[derive(Debug, Clone, Default)]
pub struct ClassNames {
    classes: Vec<String>,
}

impl ClassNames {
    /// 创建新的类名构建器
    ///
    /// # Returns
    ///
    /// 新的 ClassNames 实例
    pub fn new() -> Self {
        Self {
            classes: Vec::new(),
        }
    }

    /// 添加一个类名
    ///
    /// # Arguments
    ///
    /// * `class` - 要添加的类名
    ///
    /// # Returns
    ///
    /// 返回 self 以支持链式调用
    pub fn add<S: Into<String>>(mut self, class: S) -> Self {
        let class_str = class.into();
        if !class_str.is_empty() {
            self.classes.push(class_str);
        }
        self
    }

    /// 条件性添加类名
    ///
    /// # Arguments
    ///
    /// * `class` - 要添加的类名
    /// * `condition` - 添加条件
    ///
    /// # Returns
    ///
    /// 返回 self 以支持链式调用
    pub fn add_if<S: Into<String>>(self, class: S, condition: bool) -> Self {
        if condition {
            self.add(class)
        } else {
            self
        }
    }

    /// 从 Option 添加类名
    ///
    /// # Arguments
    ///
    /// * `class` - 可选的类名
    ///
    /// # Returns
    ///
    /// 返回 self 以支持链式调用
    pub fn add_option<S: Into<String>>(self, class: Option<S>) -> Self {
        if let Some(class_str) = class {
            self.add(class_str)
        } else {
            self
        }
    }

    /// 从映射表添加类名
    ///
    /// # Arguments
    ///
    /// * `map` - 类名到条件的映射
    ///
    /// # Returns
    ///
    /// 返回 self 以支持链式调用
    pub fn add_map<S: Into<String>>(mut self, map: HashMap<S, bool>) -> Self {
        for (class, condition) in map {
            if condition {
                self = self.add(class);
            }
        }
        self
    }

    /// 构建最终的类名字符串
    ///
    /// # Returns
    ///
    /// 用空格分隔的类名字符串
    pub fn build(self) -> String {
        self.classes.join(" ")
    }

    /// 检查是否包含指定类名
    ///
    /// # Arguments
    ///
    /// * `class` - 要检查的类名
    ///
    /// # Returns
    ///
    /// 如果包含该类名则返回 true
    pub fn contains(&self, class: &str) -> bool {
        self.classes.iter().any(|c| c == class)
    }

    /// 获取类名数量
    ///
    /// # Returns
    ///
    /// 类名的数量
    pub fn len(&self) -> usize {
        self.classes.len()
    }

    /// 检查是否为空
    ///
    /// # Returns
    ///
    /// 如果没有类名则返回 true
    pub fn is_empty(&self) -> bool {
        self.classes.is_empty()
    }
}

impl From<&str> for ClassNames {
    fn from(class: &str) -> Self {
        ClassNames::new().add(class)
    }
}

impl From<String> for ClassNames {
    fn from(class: String) -> Self {
        ClassNames::new().add(class)
    }
}

impl From<Vec<String>> for ClassNames {
    fn from(classes: Vec<String>) -> Self {
        let mut cn = ClassNames::new();
        for class in classes {
            cn = cn.add(class);
        }
        cn
    }
}

/// 便捷宏，用于创建类名
///
/// # Examples
///
/// ```rust
/// use ant_design_dioxus::class_names;
///
/// let classes = class_names![
///     "btn",
///     "btn-primary" => true,
///     "btn-disabled" => false,
///     Some("btn-large")
/// ];
/// ```
#[macro_export]
macro_rules! class_names {
    () => {
        $crate::utils::ClassNames::new()
    };
    ($($class:expr),* $(,)?) => {
        {
            let mut cn = $crate::utils::ClassNames::new();
            $(
                cn = cn.add($class);
            )*
            cn
        }
    };
    ($($class:expr => $condition:expr),* $(,)?) => {
        {
            let mut cn = $crate::utils::ClassNames::new();
            $(
                cn = cn.add_if($class, $condition);
            )*
            cn
        }
    };
}

/// 便捷函数，用于快速创建类名字符串
///
/// # Arguments
///
/// * `classes` - 类名数组
///
/// # Returns
///
/// 用空格分隔的类名字符串
pub fn class_names<S: Into<String>>(classes: &[S]) -> String
where
    S: Clone,
{
    classes
        .iter()
        .map(|c| c.clone().into())
        .filter(|c| !c.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

/// 条件性类名函数
///
/// # Arguments
///
/// * `base` - 基础类名
/// * `conditional` - 条件类名和条件的元组数组
///
/// # Returns
///
/// 用空格分隔的类名字符串
pub fn conditional_class_names<S: Into<String>>(base: S, conditional: &[(S, bool)]) -> String
where
    S: Clone,
{
    let mut classes = vec![base.into()];

    for (class, condition) in conditional {
        if *condition {
            classes.push(class.clone().into());
        }
    }

    classes.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_class_names_basic() {
        let cn = ClassNames::new().add("btn").add("btn-primary").build();

        assert_eq!(cn, "btn btn-primary");
    }

    #[test]
    fn test_class_names_conditional() {
        let cn = ClassNames::new()
            .add("btn")
            .add_if("btn-primary", true)
            .add_if("btn-disabled", false)
            .build();

        assert_eq!(cn, "btn btn-primary");
    }

    #[test]
    fn test_class_names_option() {
        let cn = ClassNames::new()
            .add("btn")
            .add_option(Some("btn-large"))
            .add_option(None::<String>)
            .build();

        assert_eq!(cn, "btn btn-large");
    }

    #[test]
    fn test_class_names_map() {
        let mut map = HashMap::new();
        map.insert("btn-primary", true);
        map.insert("btn-disabled", false);

        let cn = ClassNames::new().add("btn").add_map(map).build();

        assert!(cn.contains("btn"));
        assert!(cn.contains("btn-primary"));
        assert!(!cn.contains("btn-disabled"));
    }

    #[test]
    fn test_class_names_from_str() {
        let cn = ClassNames::from("btn").add("btn-primary").build();
        assert_eq!(cn, "btn btn-primary");
    }

    #[test]
    fn test_class_names_function() {
        let classes = class_names(&["btn", "btn-primary", ""]);
        assert_eq!(classes, "btn btn-primary");
    }

    #[test]
    fn test_conditional_class_names() {
        let classes =
            conditional_class_names("btn", &[("btn-primary", true), ("btn-disabled", false)]);
        assert_eq!(classes, "btn btn-primary");
    }
}

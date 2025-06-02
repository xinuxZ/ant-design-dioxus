//! Ant Design 设计令牌值定义
//!
//! 本模块定义了 Ant Design 设计系统的具体令牌值，包括颜色、字体、间距等。
//! 这些值基于 Ant Design 5.x 的设计规范。

use css_in_rust::theme::{DesignTokens, TokenPath, TokenValue};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Ant Design 设计令牌
///
/// 这个结构体包含了 Ant Design 的所有设计令牌，包括颜色、字体、间距等。
/// 它提供了浅色和深色主题的令牌值。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AntDesignTokens {
    /// 令牌映射表
    pub tokens: HashMap<String, TokenValue>,
}

impl Default for AntDesignTokens {
    fn default() -> Self {
        Self {
            tokens: Self::get_light_theme_tokens(),
        }
    }
}

impl AntDesignTokens {
    /// 创建新的 Ant Design 令牌实例
    pub fn new() -> Self {
        Self::default()
    }
    /// 获取浅色主题的令牌
    pub fn get_light_theme_tokens() -> HashMap<String, TokenValue> {
        let mut values = HashMap::new();

        // 基础颜色 - 主色调
        values.insert(
            "color.primary.50".to_string(),
            TokenValue::String("#e6f4ff".to_string()),
        );
        values.insert(
            "color.primary.100".to_string(),
            TokenValue::String("#bae0ff".to_string()),
        );
        values.insert(
            "color.primary.200".to_string(),
            TokenValue::String("#91caff".to_string()),
        );
        values.insert(
            "color.primary.300".to_string(),
            TokenValue::String("#69b1ff".to_string()),
        );
        values.insert(
            "color.primary.400".to_string(),
            TokenValue::String("#4096ff".to_string()),
        );
        values.insert(
            "color.primary.500".to_string(),
            TokenValue::String("#1677ff".to_string()),
        );
        values.insert(
            "color.primary.600".to_string(),
            TokenValue::String("#0958d9".to_string()),
        );
        values.insert(
            "color.primary.700".to_string(),
            TokenValue::String("#003eb3".to_string()),
        );
        values.insert(
            "color.primary.800".to_string(),
            TokenValue::String("#002c8c".to_string()),
        );
        values.insert(
            "color.primary.900".to_string(),
            TokenValue::String("#001d66".to_string()),
        );

        // 功能色
        values.insert(
            "color.success.500".to_string(),
            TokenValue::String("#52c41a".to_string()),
        );
        values.insert(
            "color.warning.500".to_string(),
            TokenValue::String("#faad14".to_string()),
        );
        values.insert(
            "color.error.500".to_string(),
            TokenValue::String("#ff4d4f".to_string()),
        );
        values.insert(
            "color.info.500".to_string(),
            TokenValue::String("#1677ff".to_string()),
        );

        // 文本颜色
        values.insert(
            "color.text.primary".to_string(),
            TokenValue::String("rgba(0, 0, 0, 0.88)".to_string()),
        );
        values.insert(
            "color.text.secondary".to_string(),
            TokenValue::String("rgba(0, 0, 0, 0.65)".to_string()),
        );
        values.insert(
            "color.text.tertiary".to_string(),
            TokenValue::String("rgba(0, 0, 0, 0.45)".to_string()),
        );
        values.insert(
            "color.text.quaternary".to_string(),
            TokenValue::String("rgba(0, 0, 0, 0.25)".to_string()),
        );

        // 背景颜色
        values.insert(
            "color.background.default".to_string(),
            TokenValue::String("#ffffff".to_string()),
        );
        values.insert(
            "color.background.container".to_string(),
            TokenValue::String("#ffffff".to_string()),
        );
        values.insert(
            "color.background.elevated".to_string(),
            TokenValue::String("#ffffff".to_string()),
        );
        values.insert(
            "color.background.layout".to_string(),
            TokenValue::String("#f5f5f5".to_string()),
        );

        // 边框颜色
        values.insert(
            "color.border.default".to_string(),
            TokenValue::String("#d9d9d9".to_string()),
        );
        values.insert(
            "color.border.secondary".to_string(),
            TokenValue::String("#f0f0f0".to_string()),
        );

        // 字体大小
        values.insert(
            "typography.fontSize.xs".to_string(),
            TokenValue::Number(12.0),
        );
        values.insert(
            "typography.fontSize.sm".to_string(),
            TokenValue::Number(14.0),
        );
        values.insert(
            "typography.fontSize.base".to_string(),
            TokenValue::Number(14.0),
        );
        values.insert(
            "typography.fontSize.lg".to_string(),
            TokenValue::Number(16.0),
        );
        values.insert(
            "typography.fontSize.xl".to_string(),
            TokenValue::Number(20.0),
        );
        values.insert(
            "typography.fontSize.2xl".to_string(),
            TokenValue::Number(24.0),
        );
        values.insert(
            "typography.fontSize.3xl".to_string(),
            TokenValue::Number(30.0),
        );
        values.insert(
            "typography.fontSize.4xl".to_string(),
            TokenValue::Number(38.0),
        );

        // 字体权重
        values.insert(
            "typography.fontWeight.normal".to_string(),
            TokenValue::Number(400.0),
        );
        values.insert(
            "typography.fontWeight.medium".to_string(),
            TokenValue::Number(500.0),
        );
        values.insert(
            "typography.fontWeight.semibold".to_string(),
            TokenValue::Number(600.0),
        );
        values.insert(
            "typography.fontWeight.bold".to_string(),
            TokenValue::Number(700.0),
        );

        // 行高
        values.insert(
            "typography.lineHeight.tight".to_string(),
            TokenValue::Number(1.2),
        );
        values.insert(
            "typography.lineHeight.normal".to_string(),
            TokenValue::Number(1.5),
        );
        values.insert(
            "typography.lineHeight.relaxed".to_string(),
            TokenValue::Number(1.75),
        );

        // 间距
        values.insert("spacing.xs".to_string(), TokenValue::Number(4.0));
        values.insert("spacing.sm".to_string(), TokenValue::Number(8.0));
        values.insert("spacing.md".to_string(), TokenValue::Number(16.0));
        values.insert("spacing.lg".to_string(), TokenValue::Number(24.0));
        values.insert("spacing.xl".to_string(), TokenValue::Number(32.0));
        values.insert("spacing.2xl".to_string(), TokenValue::Number(48.0));
        values.insert("spacing.3xl".to_string(), TokenValue::Number(64.0));

        // 边框半径
        values.insert("border.radius.none".to_string(), TokenValue::Number(0.0));
        values.insert("border.radius.sm".to_string(), TokenValue::Number(2.0));
        values.insert("border.radius.base".to_string(), TokenValue::Number(6.0));
        values.insert("border.radius.lg".to_string(), TokenValue::Number(8.0));
        values.insert("border.radius.xl".to_string(), TokenValue::Number(12.0));
        values.insert(
            "border.radius.full".to_string(),
            TokenValue::String("50%".to_string()),
        );

        // 边框宽度
        values.insert("border.width.none".to_string(), TokenValue::Number(0.0));
        values.insert("border.width.thin".to_string(), TokenValue::Number(1.0));
        values.insert("border.width.thick".to_string(), TokenValue::Number(2.0));

        values
    }

    /// 获取深色主题的令牌
    pub fn get_dark_theme_tokens() -> HashMap<String, TokenValue> {
        let mut values = Self::get_light_theme_tokens();

        // 覆盖深色主题特定的值

        // 文本颜色
        values.insert(
            "color.text.primary".to_string(),
            TokenValue::String("rgba(255, 255, 255, 0.85)".to_string()),
        );
        values.insert(
            "color.text.secondary".to_string(),
            TokenValue::String("rgba(255, 255, 255, 0.65)".to_string()),
        );
        values.insert(
            "color.text.tertiary".to_string(),
            TokenValue::String("rgba(255, 255, 255, 0.45)".to_string()),
        );
        values.insert(
            "color.text.quaternary".to_string(),
            TokenValue::String("rgba(255, 255, 255, 0.25)".to_string()),
        );

        // 背景颜色
        values.insert(
            "color.background.default".to_string(),
            TokenValue::String("#141414".to_string()),
        );
        values.insert(
            "color.background.container".to_string(),
            TokenValue::String("#1f1f1f".to_string()),
        );
        values.insert(
            "color.background.elevated".to_string(),
            TokenValue::String("#262626".to_string()),
        );
        values.insert(
            "color.background.layout".to_string(),
            TokenValue::String("#000000".to_string()),
        );

        // 边框颜色
        values.insert(
            "color.border.default".to_string(),
            TokenValue::String("#434343".to_string()),
        );
        values.insert(
            "color.border.secondary".to_string(),
            TokenValue::String("#303030".to_string()),
        );

        values
    }

    /// 合并令牌
    pub fn merge(&mut self, other: HashMap<String, TokenValue>) {
        self.tokens.extend(other);
    }

    /// 获取指定路径的令牌值
    pub fn get_token(&self, path: &str) -> Option<&TokenValue> {
        self.tokens.get(path)
    }

    /// 设置令牌值
    pub fn set_token(&mut self, path: String, value: TokenValue) {
        self.tokens.insert(path, value);
    }

    /// 创建默认的设计令牌存储（兼容 css-in-rust）
    pub fn create_default_store() -> DesignTokens {
        DesignTokens::new()
    }

    /// 获取默认的浅色主题令牌值（兼容 css-in-rust）
    pub fn get_light_theme_values() -> HashMap<TokenPath, TokenValue> {
        let mut values = HashMap::new();

        // 基础颜色 - 主色调
        values.insert(
            TokenPath::from_str("color.primary.50"),
            TokenValue::String("#e6f4ff".to_string()),
        );
        values.insert(
            TokenPath::from_str("color.primary.100"),
            TokenValue::String("#bae0ff".to_string()),
        );
        values.insert(
            TokenPath::from_str("color.primary.200"),
            TokenValue::String("#91caff".to_string()),
        );
        values.insert(
            TokenPath::from_str("color.primary.300"),
            TokenValue::String("#69b1ff".to_string()),
        );
        values.insert(
            TokenPath::from_str("color.primary.400"),
            TokenValue::String("#4096ff".to_string()),
        );
        values.insert(
            TokenPath::from_str("color.primary.500"),
            TokenValue::String("#1677ff".to_string()),
        );
        values.insert(
            TokenPath::from_str("color.primary.600"),
            TokenValue::String("#0958d9".to_string()),
        );
        values.insert(
            TokenPath::from_str("color.primary.700"),
            TokenValue::String("#003eb3".to_string()),
        );
        values.insert(
            TokenPath::from_str("color.primary.800"),
            TokenValue::String("#002c8c".to_string()),
        );
        values.insert(
            TokenPath::from_str("color.primary.900"),
            TokenValue::String("#001d66".to_string()),
        );

        // 功能色
        values.insert(
            TokenPath::from_str("color.success.500"),
            TokenValue::String("#52c41a".to_string()),
        );
        values.insert(
            TokenPath::from_str("color.warning.500"),
            TokenValue::String("#faad14".to_string()),
        );
        values.insert(
            TokenPath::from_str("color.error.500"),
            TokenValue::String("#ff4d4f".to_string()),
        );
        values.insert(
            TokenPath::from_str("color.info.500"),
            TokenValue::String("#1677ff".to_string()),
        );

        // 文本颜色
        values.insert(
            TokenPath::from_str("color.text.primary"),
            TokenValue::String("rgba(0, 0, 0, 0.88)".to_string()),
        );
        values.insert(
            TokenPath::from_str("color.text.secondary"),
            TokenValue::String("rgba(0, 0, 0, 0.65)".to_string()),
        );
        values.insert(
            TokenPath::from_str("color.text.tertiary"),
            TokenValue::String("rgba(0, 0, 0, 0.45)".to_string()),
        );
        values.insert(
            TokenPath::from_str("color.text.quaternary"),
            TokenValue::String("rgba(0, 0, 0, 0.25)".to_string()),
        );

        // 背景颜色
        values.insert(
            TokenPath::from_str("color.background.default"),
            TokenValue::String("#ffffff".to_string()),
        );
        values.insert(
            TokenPath::from_str("color.background.container"),
            TokenValue::String("#ffffff".to_string()),
        );
        values.insert(
            TokenPath::from_str("color.background.elevated"),
            TokenValue::String("#ffffff".to_string()),
        );
        values.insert(
            TokenPath::from_str("color.background.layout"),
            TokenValue::String("#f5f5f5".to_string()),
        );

        // 边框颜色
        values.insert(
            TokenPath::from_str("color.border.default"),
            TokenValue::String("#d9d9d9".to_string()),
        );
        values.insert(
            TokenPath::from_str("color.border.secondary"),
            TokenValue::String("#f0f0f0".to_string()),
        );

        values
    }

    /// 获取深色主题令牌值（兼容 css-in-rust）
    pub fn get_dark_theme_values() -> HashMap<TokenPath, TokenValue> {
        let mut values = HashMap::new();

        // 文本颜色
        values.insert(
            TokenPath::from_str("color.text.primary"),
            TokenValue::String("rgba(255, 255, 255, 0.85)".to_string()),
        );
        values.insert(
            TokenPath::from_str("color.text.secondary"),
            TokenValue::String("rgba(255, 255, 255, 0.65)".to_string()),
        );
        values.insert(
            TokenPath::from_str("color.text.tertiary"),
            TokenValue::String("rgba(255, 255, 255, 0.45)".to_string()),
        );
        values.insert(
            TokenPath::from_str("color.text.quaternary"),
            TokenValue::String("rgba(255, 255, 255, 0.25)".to_string()),
        );

        // 背景颜色
        values.insert(
            TokenPath::from_str("color.background.default"),
            TokenValue::String("#141414".to_string()),
        );
        values.insert(
            TokenPath::from_str("color.background.container"),
            TokenValue::String("#1f1f1f".to_string()),
        );
        values.insert(
            TokenPath::from_str("color.background.elevated"),
            TokenValue::String("#262626".to_string()),
        );
        values.insert(
            TokenPath::from_str("color.background.layout"),
            TokenValue::String("#000000".to_string()),
        );

        // 边框颜色
        values.insert(
            TokenPath::from_str("color.border.default"),
            TokenValue::String("#434343".to_string()),
        );
        values.insert(
            TokenPath::from_str("color.border.secondary"),
            TokenValue::String("#303030".to_string()),
        );

        values
    }
}

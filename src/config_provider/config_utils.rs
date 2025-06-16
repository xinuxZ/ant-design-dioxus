//! 配置工具模块
//!
//! 提供配置合并、验证、转换和管理的实用工具函数

use crate::config_provider::{ComponentConfig, PopupConfig, SecurityConfig, VirtualScrollConfig};
use crate::locale::LocaleConfig;
use crate::theme::ThemeConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 配置合并特征
pub trait ConfigMerge<T> {
    /// 合并配置，优先使用other中的非None值
    fn merge_with(&self, other: &T) -> T;

    /// 深度合并配置
    fn deep_merge_with(&self, other: &T) -> T;
}

/// 配置验证特征
pub trait ConfigValidate {
    type Error;

    /// 验证配置的有效性
    fn validate(&self) -> Result<(), Self::Error>;

    /// 获取验证警告
    fn get_warnings(&self) -> Vec<String>;
}

/// 配置转换特征
pub trait ConfigTransform<T> {
    /// 转换为目标类型
    fn transform_to(&self) -> Result<T, String>;

    /// 从源类型转换
    fn transform_from(source: &T) -> Result<Self, String>
    where
        Self: Sized;
}

/// 配置错误类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfigError {
    /// 验证错误
    ValidationError(String),
    /// 合并错误
    MergeError(String),
    /// 转换错误
    TransformError(String),
    /// 序列化错误
    SerializationError(String),
    /// 不兼容的配置
    IncompatibleConfig(String),
    /// 缺失必需字段
    MissingRequiredField(String),
    /// 无效的值
    InvalidValue(String, String), // field, reason

    /// IO错误
    IoError(String),

    /// 反序列化错误
    DeserializationError(String),
}

/// 配置合并策略
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MergeStrategy {
    /// 覆盖策略：完全使用新配置
    Override,
    /// 合并策略：合并非空值
    Merge,
    /// 深度合并策略：递归合并嵌套对象
    DeepMerge,
    /// 保守策略：只在目标为空时使用新值
    Conservative,
}

/// 配置优先级
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ConfigPriority {
    /// 默认配置
    Default = 0,
    /// 主题配置
    Theme = 1,
    /// 全局配置
    Global = 2,
    /// 组件配置
    Component = 3,
    /// 用户配置
    User = 4,
    /// 运行时配置
    Runtime = 5,
}

/// 配置源信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigSource {
    /// 配置来源名称
    pub name: String,
    /// 配置优先级
    pub priority: ConfigPriority,
    /// 配置版本
    pub version: String,
    /// 创建时间戳
    pub created_at: u64,
    /// 更新时间戳
    pub updated_at: u64,
}

/// 配置管理器
#[derive(Debug, Clone)]
pub struct ConfigManager {
    /// 配置层级
    config_layers: Vec<(ConfigSource, serde_json::Value)>,
    /// 合并策略
    merge_strategy: MergeStrategy,
    /// 缓存的合并结果
    cached_result: Option<serde_json::Value>,
    /// 是否需要重新计算
    needs_recalculation: bool,
}

/// 配置构建器
#[derive(Debug, Default)]
pub struct ConfigBuilder {
    theme_config: Option<ThemeConfig>,
    locale_config: Option<LocaleConfig>,
    component_config: Option<ComponentConfig>,
    security_config: Option<SecurityConfig>,
    popup_config: Option<PopupConfig>,
    virtual_scroll_config: Option<VirtualScrollConfig>,
    custom_configs: HashMap<String, serde_json::Value>,
}

/// 配置快照
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigSnapshot {
    /// 快照ID
    pub id: String,
    /// 快照名称
    pub name: String,
    /// 配置数据
    pub config_data: serde_json::Value,
    /// 创建时间
    pub created_at: u64,
    /// 描述
    pub description: Option<String>,
    /// 标签
    pub tags: Vec<String>,
}

/// 配置差异
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigDiff {
    /// 添加的字段
    pub added: Vec<String>,
    /// 删除的字段
    pub removed: Vec<String>,
    /// 修改的字段
    pub modified: Vec<ConfigFieldDiff>,
}

/// 配置字段差异
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigFieldDiff {
    /// 字段路径
    pub path: String,
    /// 旧值
    pub old_value: serde_json::Value,
    /// 新值
    pub new_value: serde_json::Value,
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::ValidationError(msg) => write!(f, "配置验证错误: {}", msg),
            ConfigError::MergeError(msg) => write!(f, "配置合并错误: {}", msg),
            ConfigError::TransformError(msg) => write!(f, "配置转换错误: {}", msg),
            ConfigError::SerializationError(msg) => write!(f, "配置序列化错误: {}", msg),
            ConfigError::IncompatibleConfig(msg) => write!(f, "不兼容的配置: {}", msg),
            ConfigError::MissingRequiredField(field) => write!(f, "缺失必需字段: {}", field),
            ConfigError::InvalidValue(field, reason) => {
                write!(f, "无效的值 '{}': {}", field, reason)
            }
            ConfigError::IoError(msg) => write!(f, "IO错误: {}", msg),
            ConfigError::DeserializationError(msg) => write!(f, "反序列化错误: {}", msg),
        }
    }
}

impl std::error::Error for ConfigError {}

impl ConfigManager {
    /// 创建新的配置管理器
    pub fn new(merge_strategy: MergeStrategy) -> Self {
        Self {
            config_layers: Vec::new(),
            merge_strategy,
            cached_result: None,
            needs_recalculation: true,
        }
    }

    /// 添加配置层
    pub fn add_layer(&mut self, source: ConfigSource, config: serde_json::Value) {
        self.config_layers.push((source, config));
        self.config_layers
            .sort_by(|a, b| a.0.priority.cmp(&b.0.priority));
        self.needs_recalculation = true;
        self.cached_result = None;
    }

    /// 移除配置层
    pub fn remove_layer(&mut self, source_name: &str) -> bool {
        let initial_len = self.config_layers.len();
        self.config_layers
            .retain(|(source, _)| source.name != source_name);
        let removed = self.config_layers.len() != initial_len;
        if removed {
            self.needs_recalculation = true;
            self.cached_result = None;
        }
        removed
    }

    /// 更新配置层
    pub fn update_layer(&mut self, source_name: &str, config: serde_json::Value) -> bool {
        if let Some((_, layer_config)) = self
            .config_layers
            .iter_mut()
            .find(|(source, _)| source.name == source_name)
        {
            *layer_config = config;
            self.needs_recalculation = true;
            self.cached_result = None;
            true
        } else {
            false
        }
    }

    /// 获取合并后的配置
    pub fn get_merged_config(&mut self) -> Result<&serde_json::Value, ConfigError> {
        if self.needs_recalculation || self.cached_result.is_none() {
            self.recalculate_merged_config()?;
        }
        Ok(self.cached_result.as_ref().unwrap())
    }

    /// 重新计算合并配置
    fn recalculate_merged_config(&mut self) -> Result<(), ConfigError> {
        if self.config_layers.is_empty() {
            self.cached_result = Some(serde_json::Value::Object(serde_json::Map::new()));
            self.needs_recalculation = false;
            return Ok(());
        }

        let mut result = serde_json::Value::Object(serde_json::Map::new());

        for (_, config) in &self.config_layers {
            result = self.merge_values(&result, config)?;
        }

        self.cached_result = Some(result);
        self.needs_recalculation = false;
        Ok(())
    }

    /// 合并两个JSON值
    fn merge_values(
        &self,
        base: &serde_json::Value,
        overlay: &serde_json::Value,
    ) -> Result<serde_json::Value, ConfigError> {
        match self.merge_strategy {
            MergeStrategy::Override => Ok(overlay.clone()),
            MergeStrategy::Merge => self.merge_values_shallow(base, overlay),
            MergeStrategy::DeepMerge => self.merge_values_deep(base, overlay),
            MergeStrategy::Conservative => self.merge_values_conservative(base, overlay),
        }
    }

    /// 浅层合并
    fn merge_values_shallow(
        &self,
        base: &serde_json::Value,
        overlay: &serde_json::Value,
    ) -> Result<serde_json::Value, ConfigError> {
        match (base, overlay) {
            (serde_json::Value::Object(base_obj), serde_json::Value::Object(overlay_obj)) => {
                let mut result = base_obj.clone();
                for (key, value) in overlay_obj {
                    if !value.is_null() {
                        result.insert(key.clone(), value.clone());
                    }
                }
                Ok(serde_json::Value::Object(result))
            }
            _ => Ok(overlay.clone()),
        }
    }

    /// 深层合并
    fn merge_values_deep(
        &self,
        base: &serde_json::Value,
        overlay: &serde_json::Value,
    ) -> Result<serde_json::Value, ConfigError> {
        match (base, overlay) {
            (serde_json::Value::Object(base_obj), serde_json::Value::Object(overlay_obj)) => {
                let mut result = base_obj.clone();
                for (key, overlay_value) in overlay_obj {
                    if overlay_value.is_null() {
                        continue;
                    }

                    if let Some(base_value) = result.get(key) {
                        result.insert(
                            key.clone(),
                            self.merge_values_deep(base_value, overlay_value)?,
                        );
                    } else {
                        result.insert(key.clone(), overlay_value.clone());
                    }
                }
                Ok(serde_json::Value::Object(result))
            }
            (serde_json::Value::Array(base_arr), serde_json::Value::Array(overlay_arr)) => {
                let mut result = base_arr.clone();
                result.extend(overlay_arr.iter().cloned());
                Ok(serde_json::Value::Array(result))
            }
            _ => Ok(overlay.clone()),
        }
    }

    /// 保守合并
    fn merge_values_conservative(
        &self,
        base: &serde_json::Value,
        overlay: &serde_json::Value,
    ) -> Result<serde_json::Value, ConfigError> {
        match (base, overlay) {
            (serde_json::Value::Object(base_obj), serde_json::Value::Object(overlay_obj)) => {
                let mut result = base_obj.clone();
                for (key, overlay_value) in overlay_obj {
                    if !result.contains_key(key) && !overlay_value.is_null() {
                        result.insert(key.clone(), overlay_value.clone());
                    }
                }
                Ok(serde_json::Value::Object(result))
            }
            _ => {
                if base.is_null() {
                    Ok(overlay.clone())
                } else {
                    Ok(base.clone())
                }
            }
        }
    }

    /// 获取配置层信息
    pub fn get_layers(&self) -> &[(ConfigSource, serde_json::Value)] {
        &self.config_layers
    }

    /// 清空所有配置层
    pub fn clear(&mut self) {
        self.config_layers.clear();
        self.cached_result = None;
        self.needs_recalculation = true;
    }
}

impl ConfigBuilder {
    /// 创建新的配置构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置主题配置
    pub fn with_theme_config(mut self, config: ThemeConfig) -> Self {
        self.theme_config = Some(config);
        self
    }

    /// 设置语言配置
    pub fn with_locale_config(mut self, config: LocaleConfig) -> Self {
        self.locale_config = Some(config);
        self
    }

    /// 设置组件配置
    pub fn with_component_config(mut self, config: ComponentConfig) -> Self {
        self.component_config = Some(config);
        self
    }

    /// 设置安全配置
    pub fn with_security_config(mut self, config: SecurityConfig) -> Self {
        self.security_config = Some(config);
        self
    }

    /// 设置弹出层配置
    pub fn with_popup_config(mut self, config: PopupConfig) -> Self {
        self.popup_config = Some(config);
        self
    }

    /// 设置虚拟滚动配置
    pub fn with_virtual_scroll_config(mut self, config: VirtualScrollConfig) -> Self {
        self.virtual_scroll_config = Some(config);
        self
    }

    /// 添加自定义配置
    pub fn with_custom_config(mut self, key: String, config: serde_json::Value) -> Self {
        self.custom_configs.insert(key, config);
        self
    }

    /// 构建最终配置
    pub fn build(self) -> Result<serde_json::Value, ConfigError> {
        let mut config_map = serde_json::Map::new();

        if let Some(theme_config) = self.theme_config {
            let theme_value = serde_json::to_value(theme_config)
                .map_err(|e| ConfigError::SerializationError(e.to_string()))?;
            config_map.insert("theme".to_string(), theme_value);
        }

        if let Some(locale_config) = self.locale_config {
            let locale_value = serde_json::to_value(locale_config)
                .map_err(|e| ConfigError::SerializationError(e.to_string()))?;
            config_map.insert("locale".to_string(), locale_value);
        }

        if let Some(component_config) = self.component_config {
            let component_value = serde_json::to_value(component_config)
                .map_err(|e| ConfigError::SerializationError(e.to_string()))?;
            config_map.insert("component".to_string(), component_value);
        }

        if let Some(security_config) = self.security_config {
            let security_value = serde_json::to_value(security_config)
                .map_err(|e| ConfigError::SerializationError(e.to_string()))?;
            config_map.insert("security".to_string(), security_value);
        }

        if let Some(popup_config) = self.popup_config {
            let popup_value = serde_json::to_value(popup_config)
                .map_err(|e| ConfigError::SerializationError(e.to_string()))?;
            config_map.insert("popup".to_string(), popup_value);
        }

        if let Some(virtual_scroll_config) = self.virtual_scroll_config {
            let virtual_scroll_value = serde_json::to_value(virtual_scroll_config)
                .map_err(|e| ConfigError::SerializationError(e.to_string()))?;
            config_map.insert("virtualScroll".to_string(), virtual_scroll_value);
        }

        for (key, value) in self.custom_configs {
            config_map.insert(key, value);
        }

        Ok(serde_json::Value::Object(config_map))
    }
}

/// 配置工具函数
pub mod utils {
    use super::*;

    /// 比较两个配置的差异
    pub fn diff_configs(old: &serde_json::Value, new: &serde_json::Value) -> ConfigDiff {
        let mut diff = ConfigDiff {
            added: Vec::new(),
            removed: Vec::new(),
            modified: Vec::new(),
        };

        diff_values_recursive(old, new, "", &mut diff);
        diff
    }

    /// 递归比较值的差异
    fn diff_values_recursive(
        old: &serde_json::Value,
        new: &serde_json::Value,
        path: &str,
        diff: &mut ConfigDiff,
    ) {
        match (old, new) {
            (serde_json::Value::Object(old_obj), serde_json::Value::Object(new_obj)) => {
                // 检查删除的字段
                for key in old_obj.keys() {
                    if !new_obj.contains_key(key) {
                        let field_path = if path.is_empty() {
                            key.clone()
                        } else {
                            format!("{}.{}", path, key)
                        };
                        diff.removed.push(field_path);
                    }
                }

                // 检查添加和修改的字段
                for (key, new_value) in new_obj {
                    let field_path = if path.is_empty() {
                        key.clone()
                    } else {
                        format!("{}.{}", path, key)
                    };

                    if let Some(old_value) = old_obj.get(key) {
                        if old_value != new_value {
                            if old_value.is_object() && new_value.is_object() {
                                diff_values_recursive(old_value, new_value, &field_path, diff);
                            } else {
                                diff.modified.push(ConfigFieldDiff {
                                    path: field_path,
                                    old_value: old_value.clone(),
                                    new_value: new_value.clone(),
                                });
                            }
                        }
                    } else {
                        diff.added.push(field_path);
                    }
                }
            }
            _ => {
                if old != new {
                    diff.modified.push(ConfigFieldDiff {
                        path: path.to_string(),
                        old_value: old.clone(),
                        new_value: new.clone(),
                    });
                }
            }
        }
    }

    /// 验证配置的完整性
    pub fn validate_config_integrity(config: &serde_json::Value) -> Result<(), Vec<ConfigError>> {
        let mut errors = Vec::new();

        // 基本结构验证
        if !config.is_object() {
            errors.push(ConfigError::ValidationError(
                "配置必须是对象类型".to_string(),
            ));
            return Err(errors);
        }

        // 验证各个配置模块
        if let Some(theme_config) = config.get("theme") {
            if let Err(e) = validate_theme_config(theme_config) {
                errors.push(e);
            }
        }

        if let Some(component_config) = config.get("component") {
            if let Err(e) = validate_component_config(component_config) {
                errors.push(e);
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// 验证主题配置
    fn validate_theme_config(config: &serde_json::Value) -> Result<(), ConfigError> {
        if !config.is_object() {
            return Err(ConfigError::ValidationError(
                "主题配置必须是对象类型".to_string(),
            ));
        }
        Ok(())
    }

    /// 验证组件配置
    fn validate_component_config(config: &serde_json::Value) -> Result<(), ConfigError> {
        if !config.is_object() {
            return Err(ConfigError::ValidationError(
                "组件配置必须是对象类型".to_string(),
            ));
        }
        Ok(())
    }

    /// 创建配置快照
    pub fn create_snapshot(
        config: &serde_json::Value,
        name: String,
        description: Option<String>,
    ) -> ConfigSnapshot {
        use std::time::{SystemTime, UNIX_EPOCH};

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        ConfigSnapshot {
            id: format!("snapshot_{}", timestamp),
            name,
            config_data: config.clone(),
            created_at: timestamp,
            description,
            tags: Vec::new(),
        }
    }

    /// 从快照恢复配置
    pub fn restore_from_snapshot(snapshot: &ConfigSnapshot) -> serde_json::Value {
        snapshot.config_data.clone()
    }

    /// 合并多个配置
    pub fn merge_configs(
        configs: &[serde_json::Value],
        strategy: MergeStrategy,
    ) -> Result<serde_json::Value, ConfigError> {
        if configs.is_empty() {
            return Ok(serde_json::Value::Object(serde_json::Map::new()));
        }

        let mut manager = ConfigManager::new(strategy);

        for (index, config) in configs.iter().enumerate() {
            let source = ConfigSource {
                name: format!("config_{}", index),
                priority: ConfigPriority::Default,
                version: "1.0.0".to_string(),
                created_at: 0,
                updated_at: 0,
            };
            manager.add_layer(source, config.clone());
        }

        manager.get_merged_config().map(|v| v.clone())
    }

    /// 扁平化配置对象
    pub fn flatten_config(config: &serde_json::Value) -> HashMap<String, serde_json::Value> {
        let mut result = HashMap::new();
        flatten_recursive(config, "", &mut result);
        result
    }

    /// 递归扁平化
    fn flatten_recursive(
        value: &serde_json::Value,
        prefix: &str,
        result: &mut HashMap<String, serde_json::Value>,
    ) {
        match value {
            serde_json::Value::Object(obj) => {
                for (key, val) in obj {
                    let new_key = if prefix.is_empty() {
                        key.clone()
                    } else {
                        format!("{}.{}", prefix, key)
                    };
                    flatten_recursive(val, &new_key, result);
                }
            }
            _ => {
                result.insert(prefix.to_string(), value.clone());
            }
        }
    }

    /// 从扁平化配置重建嵌套结构
    pub fn unflatten_config(flat_config: &HashMap<String, serde_json::Value>) -> serde_json::Value {
        let mut result = serde_json::Map::new();

        for (key, value) in flat_config {
            set_nested_value(&mut result, key, value.clone());
        }

        serde_json::Value::Object(result)
    }

    /// 设置嵌套值
    fn set_nested_value(
        obj: &mut serde_json::Map<String, serde_json::Value>,
        path: &str,
        value: serde_json::Value,
    ) {
        let parts: Vec<&str> = path.split('.').collect();
        let mut current = obj;

        for (i, part) in parts.iter().enumerate() {
            if i == parts.len() - 1 {
                current.insert(part.to_string(), value);
                return;
            }

            let entry = current
                .entry(part.to_string())
                .or_insert_with(|| serde_json::Value::Object(serde_json::Map::new()));

            if let serde_json::Value::Object(ref mut nested) = entry {
                current = nested;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::utils::*;
    use super::*;

    #[test]
    fn test_config_manager() {
        let mut manager = ConfigManager::new(MergeStrategy::DeepMerge);

        let source1 = ConfigSource {
            name: "base".to_string(),
            priority: ConfigPriority::Default,
            version: "1.0.0".to_string(),
            created_at: 0,
            updated_at: 0,
        };

        let config1 = serde_json::json!({
            "theme": {
                "primaryColor": "#1890ff"
            }
        });

        manager.add_layer(source1, config1);

        let merged = manager.get_merged_config().unwrap();
        assert_eq!(merged["theme"]["primaryColor"], "#1890ff");
    }

    #[test]
    fn test_config_builder() {
        let builder = ConfigBuilder::new();
        let config = builder.build().unwrap();
        assert!(config.is_object());
    }

    #[test]
    fn test_config_diff() {
        let old_config = serde_json::json!({
            "theme": {
                "primaryColor": "#1890ff"
            },
            "removed": "value"
        });

        let new_config = serde_json::json!({
            "theme": {
                "primaryColor": "#ff0000"
            },
            "added": "new_value"
        });

        let diff = diff_configs(&old_config, &new_config);
        assert_eq!(diff.added.len(), 1);
        assert_eq!(diff.removed.len(), 1);
        assert_eq!(diff.modified.len(), 1);
    }

    #[test]
    fn test_flatten_unflatten() {
        let config = serde_json::json!({
            "theme": {
                "colors": {
                    "primary": "#1890ff"
                }
            }
        });

        let flattened = flatten_config(&config);
        assert!(flattened.contains_key("theme.colors.primary"));

        let unflattened = unflatten_config(&flattened);
        assert_eq!(unflattened["theme"]["colors"]["primary"], "#1890ff");
    }
}

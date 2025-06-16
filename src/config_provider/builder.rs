//! ConfigProvider构建器模块
//!
//! 提供链式API来构建ConfigProvider配置

use serde_json::Value;
use std::rc::Rc;

use crate::config_provider::{
    config_utils::{ConfigError, MergeStrategy},
    ComponentConfig, ComponentSize, Direction, PopupConfig, SecurityConfig, VirtualScrollConfig,
};
use crate::locale::LocaleConfig;
use crate::theme::ThemeConfig;

/// ConfigProvider构建器
#[derive(Debug, Clone)]
pub struct ConfigProviderBuilder {
    /// 主题配置
    theme_config: Option<ThemeConfig>,
    /// 语言配置
    locale_config: Option<LocaleConfig>,
    /// 组件配置
    component_config: Option<ComponentConfig>,
    /// 设置紧凑模式
    theme_compact: bool,

    /// 设置组件尺寸
    component_size: ComponentSize,

    /// 设置方向
    direction: Direction,

    /// 设置前缀类名
    prefix_cls: String,

    /// 设置自动插入空格
    auto_insert_space_in_button: bool,

    /// 安全配置
    security_config: Option<SecurityConfig>,
    /// 弹出层配置
    popup_config: Option<PopupConfig>,
    /// 虚拟滚动配置
    virtual_scroll_config: Option<VirtualScrollConfig>,
    /// 合并策略
    merge_strategy: MergeStrategy,
    /// 配置变更回调
    on_config_change: Option<Rc<dyn Fn(&str, &Value)>>,
    /// 是否启用配置验证
    enable_validation: bool,
    /// 是否启用配置缓存
    enable_cache: bool,
    /// 是否启用配置同步
    enable_sync: bool,
    /// 自定义配置
    custom_configs: std::collections::HashMap<String, Value>,
}

impl Default for ConfigProviderBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigProviderBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self {
            theme_config: None,
            locale_config: None,
            component_config: None,
            theme_compact: false,
            component_size: ComponentSize::default(),
            direction: Direction::Ltr,
            prefix_cls: "ant-".to_string(),
            auto_insert_space_in_button: false,
            security_config: None,
            popup_config: None,
            virtual_scroll_config: None,
            merge_strategy: MergeStrategy::DeepMerge,
            on_config_change: None,
            enable_validation: true,
            enable_cache: false,
            enable_sync: false,
            custom_configs: std::collections::HashMap::new(),
        }
    }

    /// 设置主题配置
    pub fn theme_config(mut self, config: ThemeConfig) -> Self {
        self.theme_config = Some(config);
        self
    }

    /// 设置语言配置
    pub fn locale_config(mut self, config: LocaleConfig) -> Self {
        self.locale_config = Some(config);
        self
    }

    /// 设置组件配置
    pub fn component_config(mut self, config: ComponentConfig) -> Self {
        self.component_config = Some(config);
        self
    }

    /// 设置组件尺寸
    pub fn component_size(mut self, size: ComponentSize) -> Self {
        self.component_size = size;
        self
    }

    /// 设置紧凑模式
    pub fn compact(mut self, compact: bool) -> Self {
        self.theme_compact = compact;
        self
    }

    /// 设置方向
    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    /// 设置前缀类名
    pub fn prefix_cls(mut self, prefix_cls: impl Into<String>) -> Self {
        self.prefix_cls = prefix_cls.into();
        self
    }

    /// 设置自动插入空格
    pub fn auto_insert_space_in_button(mut self, auto_insert: bool) -> Self {
        self.auto_insert_space_in_button = auto_insert;
        self
    }

    /// 设置安全配置
    pub fn security_config(mut self, config: SecurityConfig) -> Self {
        self.security_config = Some(config);
        self
    }

    /// 设置弹出层配置
    pub fn popup_config(mut self, config: PopupConfig) -> Self {
        self.popup_config = Some(config);
        self
    }

    /// 设置虚拟滚动配置
    pub fn virtual_scroll_config(mut self, config: VirtualScrollConfig) -> Self {
        self.virtual_scroll_config = Some(config);
        self
    }

    /// 设置合并策略
    pub fn merge_strategy(mut self, strategy: MergeStrategy) -> Self {
        self.merge_strategy = strategy;
        self
    }

    /// 设置配置变更回调
    pub fn on_config_change<F>(mut self, callback: F) -> Self
    where
        F: Fn(&str, &Value) + 'static,
    {
        self.on_config_change = Some(Rc::new(callback));
        self
    }

    /// 启用配置验证
    pub fn enable_validation(mut self, enable: bool) -> Self {
        self.enable_validation = enable;
        self
    }

    /// 启用配置缓存
    pub fn enable_cache(mut self, enable: bool) -> Self {
        self.enable_cache = enable;
        self
    }

    /// 启用配置同步
    pub fn enable_sync(mut self, enable: bool) -> Self {
        self.enable_sync = enable;
        self
    }

    /// 添加自定义配置
    pub fn custom_config<T: serde::Serialize>(
        mut self,
        key: &str,
        config: T,
    ) -> Result<Self, ConfigError> {
        let value = serde_json::to_value(config)
            .map_err(|e| ConfigError::SerializationError(e.to_string()))?;
        self.custom_configs.insert(key.to_string(), value);
        Ok(self)
    }

    /// 从JSON配置构建
    pub fn from_json(mut self, json: Value) -> Result<Self, ConfigError> {
        if let Some(obj) = json.as_object() {
            // 解析主题配置
            if let Some(theme_value) = obj.get("theme") {
                let theme_config: ThemeConfig = serde_json::from_value(theme_value.clone())
                    .map_err(|e| ConfigError::DeserializationError(e.to_string()))?;
                self.theme_config = Some(theme_config);
            }

            // 解析语言配置
            if let Some(locale_value) = obj.get("locale") {
                let locale_config: LocaleConfig = serde_json::from_value(locale_value.clone())
                    .map_err(|e| ConfigError::DeserializationError(e.to_string()))?;
                self.locale_config = Some(locale_config);
            }

            // 解析组件配置
            if let Some(component_value) = obj.get("component") {
                let component_config: ComponentConfig =
                    serde_json::from_value(component_value.clone())
                        .map_err(|e| ConfigError::DeserializationError(e.to_string()))?;
                self.component_config = Some(component_config);
            }

            // 解析安全配置
            if let Some(security_value) = obj.get("security") {
                let security_config: SecurityConfig =
                    serde_json::from_value(security_value.clone())
                        .map_err(|e| ConfigError::DeserializationError(e.to_string()))?;
                self.security_config = Some(security_config);
            }

            // 解析弹出层配置
            if let Some(popup_value) = obj.get("popup") {
                let popup_config: PopupConfig = serde_json::from_value(popup_value.clone())
                    .map_err(|e| ConfigError::DeserializationError(e.to_string()))?;
                self.popup_config = Some(popup_config);
            }

            // 解析虚拟滚动配置
            if let Some(virtual_scroll_value) = obj.get("virtualScroll") {
                let virtual_scroll_config: VirtualScrollConfig =
                    serde_json::from_value(virtual_scroll_value.clone())
                        .map_err(|e| ConfigError::DeserializationError(e.to_string()))?;
                self.virtual_scroll_config = Some(virtual_scroll_config);
            }

            // 解析合并策略
            if let Some(merge_strategy_value) = obj.get("mergeStrategy") {
                let merge_strategy: MergeStrategy =
                    serde_json::from_value(merge_strategy_value.clone())
                        .map_err(|e| ConfigError::DeserializationError(e.to_string()))?;
                self.merge_strategy = merge_strategy;
            }

            // 解析其他配置选项
            if let Some(enable_validation) = obj.get("enableValidation").and_then(|v| v.as_bool()) {
                self.enable_validation = enable_validation;
            }

            if let Some(enable_cache) = obj.get("enableCache").and_then(|v| v.as_bool()) {
                self.enable_cache = enable_cache;
            }

            if let Some(enable_sync) = obj.get("enableSync").and_then(|v| v.as_bool()) {
                self.enable_sync = enable_sync;
            }

            // 解析自定义配置
            if let Some(custom_configs) = obj.get("customConfigs").and_then(|v| v.as_object()) {
                for (key, value) in custom_configs {
                    self.custom_configs.insert(key.clone(), value.clone());
                }
            }
        }

        Ok(self)
    }

    /// 从文件加载配置
    pub fn from_file(self, file_path: &str) -> Result<Self, ConfigError> {
        let content =
            std::fs::read_to_string(file_path).map_err(|e| ConfigError::IoError(e.to_string()))?;

        let json: Value = serde_json::from_str(&content)
            .map_err(|e| ConfigError::DeserializationError(e.to_string()))?;

        self.from_json(json)
    }

    /// 合并另一个构建器的配置
    pub fn merge(mut self, other: ConfigProviderBuilder) -> Self {
        // 合并配置（后者优先）
        if other.theme_config.is_some() {
            self.theme_config = other.theme_config;
        }
        if other.locale_config.is_some() {
            self.locale_config = other.locale_config;
        }
        if other.component_config.is_some() {
            self.component_config = other.component_config;
        }
        if other.security_config.is_some() {
            self.security_config = other.security_config;
        }
        if other.popup_config.is_some() {
            self.popup_config = other.popup_config;
        }
        if other.virtual_scroll_config.is_some() {
            self.virtual_scroll_config = other.virtual_scroll_config;
        }
        if other.on_config_change.is_some() {
            self.on_config_change = other.on_config_change;
        }

        // 合并自定义配置
        for (key, value) in other.custom_configs {
            self.custom_configs.insert(key, value);
        }

        // 其他配置直接覆盖
        self.merge_strategy = other.merge_strategy;
        self.enable_validation = other.enable_validation;
        self.enable_cache = other.enable_cache;
        self.enable_sync = other.enable_sync;

        self
    }

    /// 验证配置
    pub fn validate(&self) -> Result<(), Vec<ConfigError>> {
        if !self.enable_validation {
            return Ok(());
        }

        let mut errors = Vec::new();

        // 验证主题配置
        if let Some(theme_config) = &self.theme_config {
            if let Err(e) = self.validate_theme_config(theme_config) {
                errors.push(e);
            }
        }

        // 验证组件配置
        if let Some(component_config) = &self.component_config {
            if let Err(e) = self.validate_component_config(component_config) {
                errors.push(e);
            }
        }

        // 验证安全配置
        if let Some(security_config) = &self.security_config {
            if let Err(e) = self.validate_security_config(security_config) {
                errors.push(e);
            }
        }

        // 验证弹出层配置
        if let Some(popup_config) = &self.popup_config {
            if let Err(e) = self.validate_popup_config(popup_config) {
                errors.push(e);
            }
        }

        // 验证虚拟滚动配置
        if let Some(virtual_scroll_config) = &self.virtual_scroll_config {
            if let Err(e) = self.validate_virtual_scroll_config(virtual_scroll_config) {
                errors.push(e);
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// 构建配置
    pub fn build(self) -> Result<ConfigProviderBuilderResult, Vec<ConfigError>> {
        // 验证配置
        self.validate()?;

        Ok(ConfigProviderBuilderResult {
            theme_config: self.theme_config,
            locale_config: self.locale_config,
            component_config: self.component_config,
            security_config: self.security_config,
            popup_config: self.popup_config,
            virtual_scroll_config: self.virtual_scroll_config,
            merge_strategy: self.merge_strategy,
            on_config_change: self.on_config_change,
            enable_validation: self.enable_validation,
            enable_cache: self.enable_cache,
            enable_sync: self.enable_sync,
            custom_configs: self.custom_configs,
        })
    }

    /// 导出为JSON
    pub fn to_json(&self) -> Result<Value, ConfigError> {
        let mut config_map = serde_json::Map::new();

        if let Some(theme_config) = &self.theme_config {
            let theme_value = serde_json::to_value(theme_config)
                .map_err(|e| ConfigError::SerializationError(e.to_string()))?;
            config_map.insert("theme".to_string(), theme_value);
        }

        if let Some(locale_config) = &self.locale_config {
            let locale_value = serde_json::to_value(locale_config)
                .map_err(|e| ConfigError::SerializationError(e.to_string()))?;
            config_map.insert("locale".to_string(), locale_value);
        }

        if let Some(component_config) = &self.component_config {
            let component_value = serde_json::to_value(component_config)
                .map_err(|e| ConfigError::SerializationError(e.to_string()))?;
            config_map.insert("component".to_string(), component_value);
        }

        if let Some(security_config) = &self.security_config {
            let security_value = serde_json::to_value(security_config)
                .map_err(|e| ConfigError::SerializationError(e.to_string()))?;
            config_map.insert("security".to_string(), security_value);
        }

        if let Some(popup_config) = &self.popup_config {
            let popup_value = serde_json::to_value(popup_config)
                .map_err(|e| ConfigError::SerializationError(e.to_string()))?;
            config_map.insert("popup".to_string(), popup_value);
        }

        if let Some(virtual_scroll_config) = &self.virtual_scroll_config {
            let virtual_scroll_value = serde_json::to_value(virtual_scroll_config)
                .map_err(|e| ConfigError::SerializationError(e.to_string()))?;
            config_map.insert("virtualScroll".to_string(), virtual_scroll_value);
        }

        let merge_strategy_value = serde_json::to_value(&self.merge_strategy)
            .map_err(|e| ConfigError::SerializationError(e.to_string()))?;
        config_map.insert("mergeStrategy".to_string(), merge_strategy_value);

        config_map.insert(
            "enableValidation".to_string(),
            Value::Bool(self.enable_validation),
        );
        config_map.insert("enableCache".to_string(), Value::Bool(self.enable_cache));
        config_map.insert("enableSync".to_string(), Value::Bool(self.enable_sync));

        if !self.custom_configs.is_empty() {
            let custom_configs_value = serde_json::to_value(&self.custom_configs)
                .map_err(|e| ConfigError::SerializationError(e.to_string()))?;
            config_map.insert("customConfigs".to_string(), custom_configs_value);
        }

        Ok(Value::Object(config_map))
    }

    /// 保存到文件
    pub fn save_to_file(&self, file_path: &str) -> Result<(), ConfigError> {
        let json = self.to_json()?;
        let content = serde_json::to_string_pretty(&json)
            .map_err(|e| ConfigError::SerializationError(e.to_string()))?;

        std::fs::write(file_path, content).map_err(|e| ConfigError::IoError(e.to_string()))?;

        Ok(())
    }

    // 私有验证方法
    fn validate_theme_config(&self, _theme_config: &ThemeConfig) -> Result<(), ConfigError> {
        // 实现主题配置验证逻辑
        Ok(())
    }

    fn validate_component_config(
        &self,
        _component_config: &ComponentConfig,
    ) -> Result<(), ConfigError> {
        // 实现组件配置验证逻辑
        Ok(())
    }

    fn validate_security_config(
        &self,
        security_config: &SecurityConfig,
    ) -> Result<(), ConfigError> {
        // 验证CSP配置
        let csp_config = &security_config.csp;

        if csp_config.nonce.is_some() && csp_config.nonce.as_ref().unwrap().len() < 16 {
            return Err(ConfigError::ValidationError(
                "CSP nonce长度不能少于16个字符".to_string(),
            ));
        }
        Ok(())
    }

    fn validate_popup_config(&self, popup_config: &PopupConfig) -> Result<(), ConfigError> {
        // 验证z-index基础值
        if popup_config.z_index_base < 1 {
            return Err(ConfigError::ValidationError(
                "z-index基础值必须大于0".to_string(),
            ));
        }
        Ok(())
    }

    fn validate_virtual_scroll_config(
        &self,
        virtual_scroll_config: &VirtualScrollConfig,
    ) -> Result<(), ConfigError> {
        // 验证缓冲区配置
        if virtual_scroll_config.buffer_config.buffer_size_before
            > virtual_scroll_config.buffer_config.max_buffer_size
        {
            return Err(ConfigError::ValidationError(
                "前置缓冲区大小不能超过最大缓冲区大小".to_string(),
            ));
        }

        if virtual_scroll_config.buffer_config.buffer_size_after
            > virtual_scroll_config.buffer_config.max_buffer_size
        {
            return Err(ConfigError::ValidationError(
                "后置缓冲区大小不能超过最大缓冲区大小".to_string(),
            ));
        }

        // 验证项目大小配置
        if virtual_scroll_config.item_size_config.estimated_height <= 0.0 {
            return Err(ConfigError::ValidationError(
                "估算项目高度必须大于0".to_string(),
            ));
        }

        if virtual_scroll_config.item_size_config.estimated_width <= 0.0 {
            return Err(ConfigError::ValidationError(
                "估算项目宽度必须大于0".to_string(),
            ));
        }

        Ok(())
    }
}

/// 构建器结果
#[derive(Debug, Clone)]
pub struct ConfigProviderBuilderResult {
    /// 主题配置
    pub theme_config: Option<ThemeConfig>,
    /// 语言配置
    pub locale_config: Option<LocaleConfig>,
    /// 组件配置
    pub component_config: Option<ComponentConfig>,
    /// 安全配置
    pub security_config: Option<SecurityConfig>,
    /// 弹出层配置
    pub popup_config: Option<PopupConfig>,
    /// 虚拟滚动配置
    pub virtual_scroll_config: Option<VirtualScrollConfig>,
    /// 合并策略
    pub merge_strategy: MergeStrategy,
    /// 配置变更回调
    pub on_config_change: Option<Rc<dyn Fn(&str, &Value)>>,
    /// 是否启用配置验证
    pub enable_validation: bool,
    /// 是否启用配置缓存
    pub enable_cache: bool,
    /// 是否启用配置同步
    pub enable_sync: bool,
    /// 自定义配置
    pub custom_configs: std::collections::HashMap<String, Value>,
}

/// 预设配置构建器
pub struct PresetConfigBuilder;

impl PresetConfigBuilder {
    /// 创建默认配置
    pub fn default() -> ConfigProviderBuilder {
        ConfigProviderBuilder::new()
    }

    /// 创建开发环境配置
    pub fn development() -> ConfigProviderBuilder {
        ConfigProviderBuilder::new()
            .enable_validation(true)
            .enable_cache(false)
            .enable_sync(false)
            .merge_strategy(MergeStrategy::DeepMerge)
    }

    /// 创建生产环境配置
    pub fn production() -> ConfigProviderBuilder {
        ConfigProviderBuilder::new()
            .enable_validation(false)
            .enable_cache(true)
            .enable_sync(true)
            .merge_strategy(MergeStrategy::Conservative)
    }

    /// 创建测试环境配置
    pub fn testing() -> ConfigProviderBuilder {
        ConfigProviderBuilder::new()
            .enable_validation(true)
            .enable_cache(false)
            .enable_sync(false)
            .merge_strategy(MergeStrategy::Override)
    }

    /// 创建高性能配置
    pub fn high_performance() -> ConfigProviderBuilder {
        ConfigProviderBuilder::new()
            .enable_validation(false)
            .enable_cache(true)
            .enable_sync(false)
            .merge_strategy(MergeStrategy::Conservative)
            .virtual_scroll_config(VirtualScrollConfig {
                buffer_config: crate::config_provider::virtual_scroll_config::BufferConfig {
                    buffer_size_before: 10,
                    buffer_size_after: 10,
                    max_buffer_size: 100,
                    min_buffer_size: 20,
                    dynamic_buffer: true,
                },
                ..VirtualScrollConfig::new()
            })
    }

    /// 创建安全配置
    pub fn secure() -> ConfigProviderBuilder {
        ConfigProviderBuilder::new()
            .enable_validation(true)
            .security_config(SecurityConfig {
                csp: crate::config_provider::security_config::CspConfig {
                    nonce: Some("secure-nonce-1234567890abcdef".to_string()),
                    strict_mode: true,
                    ..Default::default()
                },
                ..SecurityConfig::default()
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_creation() {
        let builder = ConfigProviderBuilder::new();
        assert_eq!(builder.merge_strategy, MergeStrategy::DeepMerge);
        assert!(builder.enable_validation);
        assert!(!builder.enable_cache);
        assert!(!builder.enable_sync);
    }

    #[test]
    fn test_builder_chain() {
        let builder = ConfigProviderBuilder::new()
            .enable_validation(false)
            .enable_cache(true)
            .merge_strategy(MergeStrategy::Conservative);

        assert!(!builder.enable_validation);
        assert!(builder.enable_cache);
        assert_eq!(builder.merge_strategy, MergeStrategy::Conservative);
    }

    #[test]
    fn test_preset_builders() {
        let dev_builder = PresetConfigBuilder::development();
        assert!(dev_builder.enable_validation);
        assert!(!dev_builder.enable_cache);

        let prod_builder = PresetConfigBuilder::production();
        assert!(!prod_builder.enable_validation);
        assert!(prod_builder.enable_cache);

        let test_builder = PresetConfigBuilder::testing();
        assert!(test_builder.enable_validation);
        assert_eq!(test_builder.merge_strategy, MergeStrategy::Override);
    }

    #[test]
    fn test_builder_merge() {
        let builder1 = ConfigProviderBuilder::new()
            .enable_validation(true)
            .enable_cache(false);

        let builder2 = ConfigProviderBuilder::new()
            .enable_validation(false)
            .enable_sync(true);

        let merged = builder1.merge(builder2);
        assert!(!merged.enable_validation); // 后者优先
        assert!(!merged.enable_cache); // 保持原值
        assert!(merged.enable_sync); // 新值
    }

    #[test]
    fn test_custom_config() {
        let mut builder = ConfigProviderBuilder::new();

        #[derive(serde::Serialize)]
        struct CustomConfig {
            value: i32,
        }

        let custom = CustomConfig { value: 42 };
        builder = builder.custom_config("test", custom).unwrap();

        assert!(builder.custom_configs.contains_key("test"));
    }

    #[test]
    fn test_json_serialization() {
        let builder = ConfigProviderBuilder::new()
            .enable_validation(false)
            .enable_cache(true);

        let json = builder.to_json().unwrap();
        assert_eq!(json["enableValidation"], Value::Bool(false));
        assert_eq!(json["enableCache"], Value::Bool(true));
    }
}

//! 配置Hooks模块
//!
//! 提供Dioxus特有的配置钩子函数，用于在组件中访问和管理配置

use crate::config_provider::{
    config_utils::{ConfigError, ConfigManager, MergeStrategy},
    ComponentConfig, PopupConfig, SecurityConfig, VirtualScrollConfig,
};
use crate::locale::LocaleConfig;
use crate::theme::ThemeConfig;
use dioxus::prelude::*;
use std::rc::Rc;

/// 全局配置上下文
#[derive(Clone, Copy)]
pub struct GlobalConfigContext {
    /// 主题配置
    pub theme_config: Signal<Option<ThemeConfig>>,
    /// 语言配置
    pub locale_config: Signal<Option<LocaleConfig>>,
    /// 组件配置
    pub component_config: Signal<Option<ComponentConfig>>,
    /// 安全配置
    pub security_config: Signal<Option<SecurityConfig>>,
    /// 弹出层配置
    pub popup_config: Signal<Option<PopupConfig>>,
    /// 虚拟滚动配置
    pub virtual_scroll_config: Signal<Option<VirtualScrollConfig>>,
    /// 配置管理器
    pub config_manager: Signal<Rc<ConfigManager>>,
    /// 配置更新回调
    pub on_config_change: Signal<Option<Rc<dyn Fn(&str, &serde_json::Value)>>>,
}

impl std::fmt::Debug for GlobalConfigContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GlobalConfigContext")
            .field("theme_config", &self.theme_config)
            .field("locale_config", &self.locale_config)
            .field("component_config", &self.component_config)
            .field("security_config", &self.security_config)
            .field("popup_config", &self.popup_config)
            .field("virtual_scroll_config", &self.virtual_scroll_config)
            .field("config_manager", &self.config_manager)
            .field("on_config_change", &"<function>")
            .finish()
    }
}

/// 配置提供者属性
#[derive(Props, Clone, PartialEq)]
pub struct ConfigProviderProps {
    /// 子组件
    pub children: Element,
    /// 主题配置
    #[props(optional)]
    pub theme_config: Option<ThemeConfig>,
    /// 语言配置
    #[props(optional)]
    pub locale_config: Option<LocaleConfig>,
    /// 组件配置
    #[props(optional)]
    pub component_config: Option<ComponentConfig>,
    /// 安全配置
    #[props(optional)]
    pub security_config: Option<SecurityConfig>,
    /// 弹出层配置
    #[props(optional)]
    pub popup_config: Option<PopupConfig>,
    /// 虚拟滚动配置
    #[props(optional)]
    pub virtual_scroll_config: Option<VirtualScrollConfig>,
    /// 合并策略
    #[props(default = MergeStrategy::DeepMerge)]
    pub merge_strategy: MergeStrategy,
    /// 配置变更回调
    #[props(optional)]
    pub on_config_change: Option<Rc<dyn Fn(&str, &serde_json::Value)>>,
}

impl std::fmt::Debug for ConfigProviderProps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConfigProviderProps")
            .field("children", &"<Element>")
            .field("theme_config", &self.theme_config)
            .field("locale_config", &self.locale_config)
            .field("component_config", &self.component_config)
            .field("security_config", &self.security_config)
            .field("popup_config", &self.popup_config)
            .field("virtual_scroll_config", &self.virtual_scroll_config)
            .field("merge_strategy", &self.merge_strategy)
            .field("on_config_change", &"<function>")
            .finish()
    }
}

/// 配置提供者组件
#[component]
pub fn ConfigProvider(props: ConfigProviderProps) -> Element {
    let theme_config = use_signal(|| props.theme_config.clone());
    let locale_config = use_signal(|| props.locale_config.clone());
    let component_config = use_signal(|| props.component_config.clone());
    let security_config = use_signal(|| props.security_config.clone());
    let popup_config = use_signal(|| props.popup_config.clone());
    let virtual_scroll_config = use_signal(|| props.virtual_scroll_config.clone());
    let config_manager = use_signal(|| Rc::new(ConfigManager::new(props.merge_strategy)));
    let on_config_change = use_signal(|| props.on_config_change.clone());

    let context = GlobalConfigContext {
        theme_config,
        locale_config,
        component_config,
        security_config,
        popup_config,
        virtual_scroll_config,
        config_manager,
        on_config_change,
    };

    use_context_provider(|| context);

    rsx! {
        {props.children}
    }
}

/// 使用全局配置
pub fn use_config() -> GlobalConfigContext {
    use_context::<GlobalConfigContext>()
}

/// 使用主题配置
pub fn use_theme_config() -> Signal<Option<ThemeConfig>> {
    let mut config = use_config();
    config.theme_config
}

/// 使用语言配置
pub fn use_locale_config() -> Signal<Option<LocaleConfig>> {
    let mut config = use_config();
    config.locale_config
}

/// 使用组件配置
pub fn use_component_config() -> Signal<Option<ComponentConfig>> {
    let config = use_config();
    config.component_config
}

/// 使用安全配置
pub fn use_security_config() -> Signal<Option<SecurityConfig>> {
    let config = use_config();
    config.security_config
}

/// 使用弹出层配置
pub fn use_popup_config() -> Signal<Option<PopupConfig>> {
    let config = use_config();
    config.popup_config
}

/// 使用虚拟滚动配置
pub fn use_virtual_scroll_config() -> Signal<Option<VirtualScrollConfig>> {
    let config = use_config();
    config.virtual_scroll_config
}

/// 使用配置管理器
pub fn use_config_manager() -> Signal<Rc<ConfigManager>> {
    let config = use_config();
    config.config_manager
}

/// 配置导入Hook
pub fn use_config_import() -> impl Fn(&str) -> Result<(), ConfigError> {
    move |json_str: &str| {
        let mut config = use_config();

        let import_data: serde_json::Value = serde_json::from_str(json_str)
            .map_err(|e| ConfigError::TransformError(format!("解析配置失败: {}", e)))?;

        if let Some(theme) = import_data.get("theme") {
            if let Ok(theme_config) = serde_json::from_value::<ThemeConfig>(theme.clone()) {
                config.theme_config.set(Some(theme_config));
            }
        }

        if let Some(locale) = import_data.get("locale") {
            if let Ok(locale_config) = serde_json::from_value::<LocaleConfig>(locale.clone()) {
                config.locale_config.set(Some(locale_config));
            }
        }

        if let Some(component) = import_data.get("component") {
            if let Ok(component_config) =
                serde_json::from_value::<ComponentConfig>(component.clone())
            {
                config.component_config.set(Some(component_config));
            }
        }

        if let Some(security) = import_data.get("security") {
            if let Ok(security_config) = serde_json::from_value::<SecurityConfig>(security.clone())
            {
                config.security_config.set(Some(security_config));
            }
        }

        if let Some(popup) = import_data.get("popup") {
            if let Ok(popup_config) = serde_json::from_value::<PopupConfig>(popup.clone()) {
                config.popup_config.set(Some(popup_config));
            }
        }

        if let Some(virtual_scroll) = import_data.get("virtualScroll") {
            if let Ok(virtual_scroll_config) =
                serde_json::from_value::<VirtualScrollConfig>(virtual_scroll.clone())
            {
                config
                    .virtual_scroll_config
                    .set(Some(virtual_scroll_config));
            }
        }

        Ok(())
    }
}

/// 配置更新Hook
pub fn use_config_updater() -> impl Fn(&str, serde_json::Value) -> Result<(), ConfigError> {
    move |config_type: &str, new_config: serde_json::Value| {
        let mut config = use_config();

        match config_type {
            "theme" => {
                if let Ok(theme_config) = serde_json::from_value::<ThemeConfig>(new_config.clone())
                {
                    config.theme_config.set(Some(theme_config));
                } else {
                    return Err(ConfigError::TransformError("无法解析主题配置".to_string()));
                }
            }
            "locale" => {
                if let Ok(locale_config) =
                    serde_json::from_value::<LocaleConfig>(new_config.clone())
                {
                    config.locale_config.set(Some(locale_config));
                } else {
                    return Err(ConfigError::TransformError("无法解析语言配置".to_string()));
                }
            }
            "component" => {
                if let Ok(component_config) =
                    serde_json::from_value::<ComponentConfig>(new_config.clone())
                {
                    config.component_config.set(Some(component_config));
                } else {
                    return Err(ConfigError::TransformError("无法解析组件配置".to_string()));
                }
            }
            "security" => {
                if let Ok(security_config) =
                    serde_json::from_value::<SecurityConfig>(new_config.clone())
                {
                    config.security_config.set(Some(security_config));
                } else {
                    return Err(ConfigError::TransformError("无法解析安全配置".to_string()));
                }
            }
            "popup" => {
                if let Ok(popup_config) = serde_json::from_value::<PopupConfig>(new_config.clone())
                {
                    config.popup_config.set(Some(popup_config));
                } else {
                    return Err(ConfigError::TransformError(
                        "无法解析弹出层配置".to_string(),
                    ));
                }
            }
            "virtualScroll" => {
                if let Ok(virtual_scroll_config) =
                    serde_json::from_value::<VirtualScrollConfig>(new_config.clone())
                {
                    config
                        .virtual_scroll_config
                        .set(Some(virtual_scroll_config));
                } else {
                    return Err(ConfigError::TransformError(
                        "无法解析虚拟滚动配置".to_string(),
                    ));
                }
            }
            _ => {
                return Err(ConfigError::InvalidValue(
                    "config_type".to_string(),
                    format!("未知的配置类型: {}", config_type),
                ));
            }
        }

        // 触发配置变更回调
        if let Some(callback) = config.on_config_change.read().as_ref() {
            callback(config_type, &new_config);
        }

        Ok(())
    }
}

/// 配置重置Hook
pub fn use_config_reset() -> impl FnMut() {
    move || {
        let mut config = use_config();

        config.theme_config.set(None);
        config.locale_config.set(None);
        config.component_config.set(None);
        config.security_config.set(None);
        config.popup_config.set(None);
        config.virtual_scroll_config.set(None);
    }
}

/// 配置验证Hook
pub fn use_config_validator() -> impl Fn() -> Result<(), Vec<ConfigError>> {
    let config = use_config();

    move || {
        let mut errors = Vec::new();

        // 验证主题配置
        if let Some(theme_config) = config.theme_config.read().as_ref() {
            if let Err(e) = validate_theme_config(theme_config) {
                errors.push(e);
            }
        }

        // 验证组件配置
        if let Some(component_config) = config.component_config.read().as_ref() {
            if let Err(e) = validate_component_config(component_config) {
                errors.push(e);
            }
        }

        // 验证安全配置
        if let Some(security_config) = config.security_config.read().as_ref() {
            if let Err(e) = validate_security_config(security_config) {
                errors.push(e);
            }
        }

        // 验证弹出层配置
        if let Some(popup_config) = config.popup_config.read().as_ref() {
            if let Err(e) = validate_popup_config(popup_config) {
                errors.push(e);
            }
        }

        // 验证虚拟滚动配置
        if let Some(virtual_scroll_config) = config.virtual_scroll_config.read().as_ref() {
            if let Err(e) = validate_virtual_scroll_config(virtual_scroll_config) {
                errors.push(e);
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

/// 配置导出Hook
pub fn use_config_export() -> impl Fn() -> Result<serde_json::Value, ConfigError> {
    let config = use_config();

    move || {
        let mut config_map = serde_json::Map::new();

        if let Some(theme_config) = config.theme_config.read().as_ref() {
            let theme_value = serde_json::to_value(theme_config)
                .map_err(|e| ConfigError::SerializationError(e.to_string()))?;
            config_map.insert("theme".to_string(), theme_value);
        }

        if let Some(locale_config) = config.locale_config.read().as_ref() {
            let locale_value = serde_json::to_value(locale_config)
                .map_err(|e| ConfigError::SerializationError(e.to_string()))?;
            config_map.insert("locale".to_string(), locale_value);
        }

        if let Some(component_config) = config.component_config.read().as_ref() {
            let component_value = serde_json::to_value(component_config)
                .map_err(|e| ConfigError::SerializationError(e.to_string()))?;
            config_map.insert("component".to_string(), component_value);
        }

        if let Some(security_config) = config.security_config.read().as_ref() {
            let security_value = serde_json::to_value(security_config)
                .map_err(|e| ConfigError::SerializationError(e.to_string()))?;
            config_map.insert("security".to_string(), security_value);
        }

        if let Some(popup_config) = config.popup_config.read().as_ref() {
            let popup_value = serde_json::to_value(popup_config)
                .map_err(|e| ConfigError::SerializationError(e.to_string()))?;
            config_map.insert("popup".to_string(), popup_value);
        }

        if let Some(virtual_scroll_config) = config.virtual_scroll_config.read().as_ref() {
            let virtual_scroll_value = serde_json::to_value(virtual_scroll_config)
                .map_err(|e| ConfigError::SerializationError(e.to_string()))?;
            config_map.insert("virtualScroll".to_string(), virtual_scroll_value);
        }

        Ok(serde_json::Value::Object(config_map))
    }
}

// /// 配置导入Hook
// pub fn use_config_import() -> impl Fn(serde_json::Value) -> Result<(), ConfigError> {
//     let config_updater = use_config_updater();

//     move |config_data: serde_json::Value| {
//         if let Some(config_obj) = config_data.as_object() {
//             for (key, value) in config_obj {
//                 config_updater(key, value.clone())?;
//             }
//             Ok(())
//         } else {
//             Err(ConfigError::ValidationError(
//                 "配置数据必须是对象类型".to_string(),
//             ))
//         }
//     }
// }

/// 配置监听Hook
pub fn use_config_watcher<T: Clone + 'static>(
    config_type: &str,
    selector: impl Fn(&GlobalConfigContext) -> T + 'static,
    callback: impl Fn(T) + 'static,
) {
    let config = use_config();
    let config_type = config_type.to_string();

    use_effect(move || {
        let current_value = selector(&config);
        callback(current_value);
    });
}

/// 配置缓存Hook
pub fn use_config_cache() -> (
    impl Fn() -> Option<serde_json::Value>,
    impl Fn(serde_json::Value),
) {
    let cached_config = use_signal(|| None::<serde_json::Value>);

    let get_cache = {
        let cached_config = cached_config.clone();
        move || cached_config.read().clone()
    };

    let set_cache = {
        let cached_config = cached_config.clone();
        move |config: serde_json::Value| {
            cached_config.set(Some(config));
        }
    };

    (get_cache, set_cache)
}

/// 配置同步Hook
pub fn use_config_sync() -> impl Fn() -> Result<(), ConfigError> {
    let config = use_config();
    let config_export = use_config_export();

    move || {
        let exported_config = config_export()?;

        // 这里可以添加同步到外部存储的逻辑
        // 例如：localStorage、sessionStorage、服务器等

        // 示例：同步到localStorage
        if let Ok(config_str) = serde_json::to_string(&exported_config) {
            // 在实际应用中，这里需要使用web-sys来操作localStorage
            // web_sys::window()
            //     .unwrap()
            //     .local_storage()
            //     .unwrap()
            //     .unwrap()
            //     .set_item("ant_design_config", &config_str)
            //     .map_err(|_| ConfigError::SerializationError("无法保存到localStorage".to_string()))?;
        }

        Ok(())
    }
}

// 配置验证函数
fn validate_theme_config(_theme_config: &ThemeConfig) -> Result<(), ConfigError> {
    // 实现主题配置验证逻辑
    Ok(())
}

fn validate_component_config(_component_config: &ComponentConfig) -> Result<(), ConfigError> {
    // 实现组件配置验证逻辑
    Ok(())
}

fn validate_security_config(security_config: &SecurityConfig) -> Result<(), ConfigError> {
    // 验证CSP配置
    if let Some(csp_config) = &security_config.csp_config {
        if csp_config.nonce.is_some() && csp_config.nonce.as_ref().unwrap().len() < 16 {
            return Err(ConfigError::ValidationError(
                "CSP nonce长度不能少于16个字符".to_string(),
            ));
        }
    }
    Ok(())
}

fn validate_popup_config(popup_config: &PopupConfig) -> Result<(), ConfigError> {
    // 验证z-index基础值
    if popup_config.z_index_base < 1 {
        return Err(ConfigError::ValidationError(
            "z-index基础值必须大于0".to_string(),
        ));
    }
    Ok(())
}

fn validate_virtual_scroll_config(
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

#[cfg(test)]
mod tests {
    use super::*;
    use dioxus::prelude::*;

    #[test]
    fn test_config_validation() {
        let security_config = SecurityConfig::default();
        assert!(validate_security_config(&security_config).is_ok());

        let popup_config = PopupConfig::new();
        assert!(validate_popup_config(&popup_config).is_ok());

        let virtual_scroll_config = VirtualScrollConfig::new();
        assert!(validate_virtual_scroll_config(&virtual_scroll_config).is_ok());
    }

    // 注意：由于Dioxus的测试需要特殊的运行时环境，
    // 实际的组件测试需要在集成测试中进行
}

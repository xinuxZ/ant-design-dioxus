//! ConfigProvider使用示例
//!
//! 展示如何使用ConfigProvider的各种功能和配置选项

use dioxus::prelude::*;
use crate::config_provider::{
    ConfigProvider, ConfigProviderBuilder, PresetConfigBuilder,
    ComponentConfig, SecurityConfig, PopupConfig, VirtualScrollConfig,
    use_config, use_theme_config, use_component_config,
    use_config_updater, use_config_validator,
    ButtonConfig, InputConfig, TableConfig, FormConfig,
    CspConfig, PopupPlacement, VirtualScrollDirection,
    MergeStrategy, ComponentSize, Direction,
};
use crate::theme::ThemeConfig;
use crate::locale::LocaleConfig;
use std::rc::Rc;

/// 基础ConfigProvider使用示例
#[component]
pub fn BasicConfigProviderExample() -> Element {
    rsx! {
        ConfigProvider {
            theme_config: ThemeConfig::default(),
            locale_config: LocaleConfig::default(),
            component_config: ComponentConfig::default(),
            
            div { class: "app",
                h1 { "基础ConfigProvider示例" }
                BasicConfigConsumer {}
            }
        }
    }
}

/// 基础配置消费者组件
#[component]
fn BasicConfigConsumer() -> Element {
    let config = use_config();
    let theme_config = use_theme_config();
    let component_config = use_component_config();
    
    rsx! {
        div { class: "config-consumer",
            h2 { "当前配置信息" }
            
            div { class: "config-info",
                p { "主题配置: {theme_config.read().is_some()}" }
                p { "组件配置: {component_config.read().is_some()}" }
            }
        }
    }
}

/// 使用构建器模式的ConfigProvider示例
#[component]
pub fn BuilderConfigProviderExample() -> Element {
    let config_result = ConfigProviderBuilder::new()
        .theme_config(ThemeConfig::default())
        .component_config(ComponentConfig {
            button: Some(ButtonConfig {
                default_size: ComponentSize::Middle,
                default_type: crate::config_provider::component_config::ButtonType::Default,
                loading_delay: 200,
                auto_focus: false,
                block: false,
                danger: false,
                disabled: false,
                ghost: false,
                href: None,
                html_type: crate::config_provider::component_config::ButtonHtmlType::Button,
                icon: None,
                loading: false,
                shape: crate::config_provider::component_config::ButtonShape::Default,
                target: None,
            }),
            input: Some(InputConfig {
                default_size: ComponentSize::Middle,
                placeholder: Some("请输入内容".to_string()),
                allow_clear: false,
                bordered: true,
                disabled: false,
                max_length: None,
                show_count: false,
                status: None,
                prefix: None,
                suffix: None,
                addon_before: None,
                addon_after: None,
            }),
            ..ComponentConfig::default()
        })
        .security_config(SecurityConfig {
            csp_config: Some(CspConfig {
                nonce: Some("example-nonce-1234567890abcdef".to_string()),
                strict_mode: true,
                style_src: vec!["'self'".to_string(), "'unsafe-inline'".to_string()],
                script_src: vec!["'self'".to_string()],
                ..CspConfig::default()
            }),
            ..SecurityConfig::default()
        })
        .popup_config(PopupConfig {
            get_popup_container: None,
            auto_adjust_overflow: true,
            placement: PopupPlacement::BottomLeft,
            trigger: vec!["click".to_string()],
            z_index_base: 1000,
            ..PopupConfig::new()
        })
        .virtual_scroll_config(VirtualScrollConfig {
            buffer_config: crate::config_provider::virtual_scroll_config::BufferConfig {
                buffer_size_before: 5,
                buffer_size_after: 5,
                max_buffer_size: 50,
                dynamic_buffer: true,
            },
            item_size_config: crate::config_provider::virtual_scroll_config::ItemSizeConfig {
                estimated_height: 32.0,
                estimated_width: 200.0,
                dynamic_height: true,
                dynamic_width: false,
                min_height: 20.0,
                max_height: 100.0,
                min_width: 100.0,
                max_width: 500.0,
            },
            scroll_config: crate::config_provider::virtual_scroll_config::ScrollConfig {
                direction: VirtualScrollDirection::Vertical,
                smooth_scroll: true,
                scroll_to_alignment: crate::config_provider::virtual_scroll_config::ScrollAlignment::Auto,
                overscan_count: 3,
            },
            ..VirtualScrollConfig::new()
        })
        .merge_strategy(MergeStrategy::DeepMerge)
        .enable_validation(true)
        .enable_cache(false)
        .on_config_change(|config_type, new_value| {
            println!("配置变更: {} = {:?}", config_type, new_value);
        })
        .build();
    
    match config_result {
        Ok(config) => {
            rsx! {
                ConfigProvider {
                    theme_config: config.theme_config,
                    locale_config: config.locale_config,
                    component_config: config.component_config,
                    security_config: config.security_config,
                    popup_config: config.popup_config,
                    virtual_scroll_config: config.virtual_scroll_config,
                    merge_strategy: config.merge_strategy,
                    on_config_change: config.on_config_change,
                    
                    div { class: "app",
                        h1 { "构建器模式ConfigProvider示例" }
                        BuilderConfigConsumer {}
                    }
                }
            }
        }
        Err(errors) => {
            rsx! {
                div { class: "error",
                    h1 { "配置验证失败" }
                    ul {
                        for error in errors {
                            li { "{error:?}" }
                        }
                    }
                }
            }
        }
    }
}

/// 构建器配置消费者组件
#[component]
fn BuilderConfigConsumer() -> Element {
    let config = use_config();
    let config_updater = use_config_updater();
    let config_validator = use_config_validator();
    
    let update_theme = move |_| {
        let new_theme = ThemeConfig::default(); // 这里应该是新的主题配置
        let theme_value = serde_json::to_value(&new_theme).unwrap();
        if let Err(e) = config_updater("theme", theme_value) {
            println!("更新主题配置失败: {:?}", e);
        }
    };
    
    let validate_config = move |_| {
        match config_validator() {
            Ok(_) => println!("配置验证通过"),
            Err(errors) => {
                println!("配置验证失败:");
                for error in errors {
                    println!("  - {:?}", error);
                }
            }
        }
    };
    
    rsx! {
        div { class: "builder-config-consumer",
            h2 { "构建器配置消费者" }
            
            div { class: "config-actions",
                button { onclick: update_theme, "更新主题配置" }
                button { onclick: validate_config, "验证配置" }
            }
            
            div { class: "config-display",
                ConfigDisplay {}
            }
        }
    }
}

/// 配置显示组件
#[component]
fn ConfigDisplay() -> Element {
    let theme_config = use_theme_config();
    let component_config = use_component_config();
    let security_config = use_security_config();
    let popup_config = use_popup_config();
    let virtual_scroll_config = use_virtual_scroll_config();
    
    rsx! {
        div { class: "config-display",
            h3 { "当前配置状态" }
            
            div { class: "config-section",
                h4 { "主题配置" }
                p { "已配置: {theme_config.read().is_some()}" }
            }
            
            div { class: "config-section",
                h4 { "组件配置" }
                if let Some(comp_config) = component_config.read().as_ref() {
                    div {
                        p { "按钮配置: {comp_config.button.is_some()}" }
                        p { "输入框配置: {comp_config.input.is_some()}" }
                        p { "表格配置: {comp_config.table.is_some()}" }
                        p { "表单配置: {comp_config.form.is_some()}" }
                    }
                } else {
                    p { "未配置" }
                }
            }
            
            div { class: "config-section",
                h4 { "安全配置" }
                if let Some(sec_config) = security_config.read().as_ref() {
                    div {
                        p { "CSP配置: {sec_config.csp_config.is_some()}" }
                        p { "XSS防护: {sec_config.xss_protection}" }
                        p { "样式隔离: {sec_config.style_isolation}" }
                    }
                } else {
                    p { "未配置" }
                }
            }
            
            div { class: "config-section",
                h4 { "弹出层配置" }
                if let Some(popup_conf) = popup_config.read().as_ref() {
                    div {
                        p { "自动调整溢出: {popup_conf.auto_adjust_overflow}" }
                        p { "z-index基础值: {popup_conf.z_index_base}" }
                        p { "放置位置: {popup_conf.placement:?}" }
                    }
                } else {
                    p { "未配置" }
                }
            }
            
            div { class: "config-section",
                h4 { "虚拟滚动配置" }
                if let Some(vs_config) = virtual_scroll_config.read().as_ref() {
                    div {
                        p { "前置缓冲区: {vs_config.buffer_config.buffer_size_before}" }
                        p { "后置缓冲区: {vs_config.buffer_config.buffer_size_after}" }
                        p { "估算高度: {vs_config.item_size_config.estimated_height}" }
                        p { "滚动方向: {vs_config.scroll_config.direction:?}" }
                    }
                } else {
                    p { "未配置" }
                }
            }
        }
    }
}

/// 预设配置示例
#[component]
pub fn PresetConfigExample() -> Element {
    let preset_type = use_signal(|| "development");
    
    let config_result = match preset_type.read().as_str() {
        "development" => PresetConfigBuilder::development().build(),
        "production" => PresetConfigBuilder::production().build(),
        "testing" => PresetConfigBuilder::testing().build(),
        "high_performance" => PresetConfigBuilder::high_performance().build(),
        "secure" => PresetConfigBuilder::secure().build(),
        _ => PresetConfigBuilder::default().build(),
    };
    
    match config_result {
        Ok(config) => {
            rsx! {
                ConfigProvider {
                    theme_config: config.theme_config,
                    locale_config: config.locale_config,
                    component_config: config.component_config,
                    security_config: config.security_config,
                    popup_config: config.popup_config,
                    virtual_scroll_config: config.virtual_scroll_config,
                    merge_strategy: config.merge_strategy,
                    on_config_change: config.on_config_change,
                    
                    div { class: "app",
                        h1 { "预设配置示例" }
                        
                        div { class: "preset-selector",
                            h2 { "选择预设配置" }
                            select {
                                onchange: move |evt| preset_type.set(evt.value()),
                                option { value: "development", "开发环境" }
                                option { value: "production", "生产环境" }
                                option { value: "testing", "测试环境" }
                                option { value: "high_performance", "高性能" }
                                option { value: "secure", "安全模式" }
                            }
                        }
                        
                        PresetConfigDisplay { preset_type: preset_type.read().clone() }
                    }
                }
            }
        }
        Err(errors) => {
            rsx! {
                div { class: "error",
                    h1 { "预设配置加载失败" }
                    ul {
                        for error in errors {
                            li { "{error:?}" }
                        }
                    }
                }
            }
        }
    }
}

/// 预设配置显示组件
#[component]
fn PresetConfigDisplay(preset_type: String) -> Element {
    let config = use_config();
    
    rsx! {
        div { class: "preset-config-display",
            h3 { "当前预设: {preset_type}" }
            
            div { class: "preset-info",
                match preset_type.as_str() {
                    "development" => rsx! {
                        div {
                            p { "✅ 启用配置验证" }
                            p { "❌ 禁用配置缓存" }
                            p { "❌ 禁用配置同步" }
                            p { "🔄 深度合并策略" }
                        }
                    },
                    "production" => rsx! {
                        div {
                            p { "❌ 禁用配置验证" }
                            p { "✅ 启用配置缓存" }
                            p { "✅ 启用配置同步" }
                            p { "🔄 浅层合并策略" }
                        }
                    },
                    "testing" => rsx! {
                        div {
                            p { "✅ 启用配置验证" }
                            p { "❌ 禁用配置缓存" }
                            p { "❌ 禁用配置同步" }
                            p { "🔄 替换合并策略" }
                        }
                    },
                    "high_performance" => rsx! {
                        div {
                            p { "❌ 禁用配置验证" }
                            p { "✅ 启用配置缓存" }
                            p { "❌ 禁用配置同步" }
                            p { "🔄 浅层合并策略" }
                            p { "⚡ 优化虚拟滚动配置" }
                        }
                    },
                    "secure" => rsx! {
                        div {
                            p { "✅ 启用配置验证" }
                            p { "🔒 启用CSP安全策略" }
                            p { "🛡️ 严格模式" }
                            p { "🔐 安全nonce" }
                        }
                    },
                    _ => rsx! {
                        div {
                            p { "默认配置" }
                        }
                    }
                }
            }
        }
    }
}

/// 动态配置更新示例
#[component]
pub fn DynamicConfigExample() -> Element {
    let current_size = use_signal(|| ComponentSize::Middle);
    let current_direction = use_signal(|| Direction::Ltr);
    
    // 动态创建组件配置
    let component_config = use_memo(move || {
        ComponentConfig {
            button: Some(ButtonConfig {
                default_size: current_size.read().clone(),
                ..ButtonConfig::default()
            }),
            input: Some(InputConfig {
                default_size: current_size.read().clone(),
                ..InputConfig::default()
            }),
            ..ComponentConfig::default()
        }
    });
    
    rsx! {
        ConfigProvider {
            component_config: component_config.read().clone(),
            
            div { class: "app",
                h1 { "动态配置更新示例" }
                
                div { class: "controls",
                    div { class: "control-group",
                        label { "组件尺寸:" }
                        select {
                            onchange: move |evt| {
                                let size = match evt.value().as_str() {
                                    "Small" => ComponentSize::Small,
                                    "Large" => ComponentSize::Large,
                                    _ => ComponentSize::Middle,
                                };
                                current_size.set(size);
                            },
                            option { value: "Small", "小" }
                            option { value: "Middle", selected: true, "中" }
                            option { value: "Large", "大" }
                        }
                    }
                    
                    div { class: "control-group",
                        label { "文本方向:" }
                        select {
                            onchange: move |evt| {
                                let direction = match evt.value().as_str() {
                                    "Rtl" => Direction::Rtl,
                                    _ => Direction::Ltr,
                                };
                                current_direction.set(direction);
                            },
                            option { value: "Ltr", selected: true, "从左到右" }
                            option { value: "Rtl", "从右到左" }
                        }
                    }
                }
                
                DynamicConfigDisplay {
                    size: current_size.read().clone(),
                    direction: current_direction.read().clone(),
                }
            }
        }
    }
}

/// 动态配置显示组件
#[component]
fn DynamicConfigDisplay(size: ComponentSize, direction: Direction) -> Element {
    let component_config = use_component_config();
    
    rsx! {
        div { 
            class: "dynamic-config-display",
            dir: match direction {
                Direction::Ltr => "ltr",
                Direction::Rtl => "rtl",
            },
            
            h3 { "当前配置效果" }
            
            div { class: "config-preview",
                p { "当前尺寸: {size:?}" }
                p { "当前方向: {direction:?}" }
                
                if let Some(comp_config) = component_config.read().as_ref() {
                    div { class: "component-examples",
                        if let Some(button_config) = &comp_config.button {
                            div { class: "example-section",
                                h4 { "按钮示例" }
                                div { 
                                    class: format!("button button-{:?}", button_config.default_size).to_lowercase(),
                                    "示例按钮 ({button_config.default_size:?})"
                                }
                            }
                        }
                        
                        if let Some(input_config) = &comp_config.input {
                            div { class: "example-section",
                                h4 { "输入框示例" }
                                input { 
                                    class: format!("input input-{:?}", input_config.default_size).to_lowercase(),
                                    placeholder: input_config.placeholder.as_deref().unwrap_or("请输入内容"),
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// 配置导入导出示例
#[component]
pub fn ConfigImportExportExample() -> Element {
    let exported_config = use_signal(|| String::new());
    let import_config = use_signal(|| String::new());
    let import_status = use_signal(|| String::new());
    
    rsx! {
        ConfigProvider {
            theme_config: ThemeConfig::default(),
            component_config: ComponentConfig::default(),
            
            div { class: "app",
                h1 { "配置导入导出示例" }
                
                ConfigImportExportControls {
                    exported_config,
                    import_config,
                    import_status,
                }
            }
        }
    }
}

/// 配置导入导出控制组件
#[component]
fn ConfigImportExportControls(
    exported_config: Signal<String>,
    import_config: Signal<String>,
    import_status: Signal<String>,
) -> Element {
    let config_export = use_config_export();
    let config_import = use_config_import();
    
    let export_config = move |_| {
        match config_export() {
            Ok(config_json) => {
                let config_str = serde_json::to_string_pretty(&config_json).unwrap_or_default();
                exported_config.set(config_str);
            }
            Err(e) => {
                exported_config.set(format!("导出失败: {:?}", e));
            }
        }
    };
    
    let import_config_fn = move |_| {
        let config_str = import_config.read();
        if config_str.is_empty() {
            import_status.set("请输入配置JSON".to_string());
            return;
        }
        
        match serde_json::from_str::<serde_json::Value>(&config_str) {
            Ok(config_json) => {
                match config_import(config_json) {
                    Ok(_) => {
                        import_status.set("配置导入成功".to_string());
                    }
                    Err(e) => {
                        import_status.set(format!("配置导入失败: {:?}", e));
                    }
                }
            }
            Err(e) => {
                import_status.set(format!("JSON解析失败: {:?}", e));
            }
        }
    };
    
    rsx! {
        div { class: "import-export-controls",
            div { class: "export-section",
                h3 { "导出配置" }
                button { onclick: export_config, "导出当前配置" }
                textarea {
                    value: "{exported_config.read()}",
                    readonly: true,
                    rows: 10,
                    cols: 80,
                    placeholder: "导出的配置将显示在这里",
                }
            }
            
            div { class: "import-section",
                h3 { "导入配置" }
                textarea {
                    value: "{import_config.read()}",
                    oninput: move |evt| import_config.set(evt.value()),
                    rows: 10,
                    cols: 80,
                    placeholder: "在这里粘贴配置JSON",
                }
                button { onclick: import_config_fn, "导入配置" }
                if !import_status.read().is_empty() {
                    p { class: "import-status", "{import_status.read()}" }
                }
            }
        }
    }
}

/// 完整功能演示应用
#[component]
pub fn FullFeatureDemo() -> Element {
    let demo_type = use_signal(|| "basic");
    
    rsx! {
        div { class: "full-demo",
            nav { class: "demo-nav",
                h1 { "ConfigProvider完整功能演示" }
                
                div { class: "nav-buttons",
                    button {
                        class: if demo_type.read() == "basic" { "active" } else { "" },
                        onclick: move |_| demo_type.set("basic"),
                        "基础示例"
                    }
                    button {
                        class: if demo_type.read() == "builder" { "active" } else { "" },
                        onclick: move |_| demo_type.set("builder"),
                        "构建器模式"
                    }
                    button {
                        class: if demo_type.read() == "preset" { "active" } else { "" },
                        onclick: move |_| demo_type.set("preset"),
                        "预设配置"
                    }
                    button {
                        class: if demo_type.read() == "dynamic" { "active" } else { "" },
                        onclick: move |_| demo_type.set("dynamic"),
                        "动态配置"
                    }
                    button {
                        class: if demo_type.read() == "import_export" { "active" } else { "" },
                        onclick: move |_| demo_type.set("import_export"),
                        "导入导出"
                    }
                }
            }
            
            main { class: "demo-content",
                match demo_type.read().as_str() {
                    "basic" => rsx! { BasicConfigProviderExample {} },
                    "builder" => rsx! { BuilderConfigProviderExample {} },
                    "preset" => rsx! { PresetConfigExample {} },
                    "dynamic" => rsx! { DynamicConfigExample {} },
                    "import_export" => rsx! { ConfigImportExportExample {} },
                    _ => rsx! { BasicConfigProviderExample {} },
                }
            }
        }
    }
}
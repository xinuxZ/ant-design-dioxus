//! ConfigProvider测试模块
//!
//! 提供ConfigProvider各个功能模块的完整测试覆盖

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config_provider::{
        ConfigProviderBuilder, PresetConfigBuilder,
        ComponentConfig, SecurityConfig, PopupConfig, VirtualScrollConfig,
        ButtonConfig, InputConfig, TableConfig, FormConfig,
        CspConfig, PopupPlacement, VirtualScrollDirection,
        MergeStrategy, ComponentSize, Direction,
        ConfigError, ConfigValidate, ConfigMerge,
        ConfigManager, ConfigBuilder, ConfigSnapshot,
        VirtualScrollManager, PopupManager,
        StyleSecurityValidator, NonceGenerator,
    };
    use crate::theme::ThemeConfig;
    use crate::locale::LocaleConfig;
    use std::collections::HashMap;
    use serde_json::json;

    /// 测试基础配置结构
    #[test]
    fn test_basic_config_structures() {
        // 测试ComponentSize枚举
        let small = ComponentSize::Small;
        let middle = ComponentSize::Middle;
        let large = ComponentSize::Large;
        
        assert_ne!(small, middle);
        assert_ne!(middle, large);
        assert_eq!(ComponentSize::default(), ComponentSize::Middle);
        
        // 测试Direction枚举
        let ltr = Direction::Ltr;
        let rtl = Direction::Rtl;
        
        assert_ne!(ltr, rtl);
        assert_eq!(Direction::default(), Direction::Ltr);
    }
    
    /// 测试组件配置
    #[test]
    fn test_component_config() {
        let button_config = ButtonConfig {
            default_size: ComponentSize::Large,
            loading_delay: 300,
            auto_focus: true,
            ..ButtonConfig::default()
        };
        
        let input_config = InputConfig {
            default_size: ComponentSize::Small,
            placeholder: Some("测试占位符".to_string()),
            allow_clear: true,
            ..InputConfig::default()
        };
        
        let component_config = ComponentConfig {
            button: Some(button_config.clone()),
            input: Some(input_config.clone()),
            ..ComponentConfig::default()
        };
        
        assert!(component_config.button.is_some());
        assert!(component_config.input.is_some());
        assert!(component_config.table.is_none());
        
        let button = component_config.button.unwrap();
        assert_eq!(button.default_size, ComponentSize::Large);
        assert_eq!(button.loading_delay, 300);
        assert!(button.auto_focus);
        
        let input = component_config.input.unwrap();
        assert_eq!(input.default_size, ComponentSize::Small);
        assert_eq!(input.placeholder, Some("测试占位符".to_string()));
        assert!(input.allow_clear);
    }
    
    /// 测试安全配置
    #[test]
    fn test_security_config() {
        let csp_config = CspConfig {
            nonce: Some("test-nonce-123".to_string()),
            strict_mode: true,
            style_src: vec!["'self'".to_string(), "'unsafe-inline'".to_string()],
            script_src: vec!["'self'".to_string()],
            ..CspConfig::default()
        };
        
        let security_config = SecurityConfig {
            csp_config: Some(csp_config.clone()),
            xss_protection: true,
            style_isolation: true,
            dynamic_style_validation: true,
            ..SecurityConfig::default()
        };
        
        assert!(security_config.csp_config.is_some());
        assert!(security_config.xss_protection);
        assert!(security_config.style_isolation);
        
        let csp = security_config.csp_config.unwrap();
        assert_eq!(csp.nonce, Some("test-nonce-123".to_string()));
        assert!(csp.strict_mode);
        assert_eq!(csp.style_src.len(), 2);
    }
    
    /// 测试弹出层配置
    #[test]
    fn test_popup_config() {
        let popup_config = PopupConfig {
            auto_adjust_overflow: true,
            placement: PopupPlacement::TopLeft,
            trigger: vec!["hover".to_string(), "click".to_string()],
            z_index_base: 2000,
            ..PopupConfig::new()
        };
        
        assert!(popup_config.auto_adjust_overflow);
        assert_eq!(popup_config.placement, PopupPlacement::TopLeft);
        assert_eq!(popup_config.trigger.len(), 2);
        assert_eq!(popup_config.z_index_base, 2000);
    }
    
    /// 测试虚拟滚动配置
    #[test]
    fn test_virtual_scroll_config() {
        let vs_config = VirtualScrollConfig {
            buffer_config: crate::config_provider::virtual_scroll_config::BufferConfig {
                buffer_size_before: 10,
                buffer_size_after: 10,
                max_buffer_size: 100,
                dynamic_buffer: true,
            },
            item_size_config: crate::config_provider::virtual_scroll_config::ItemSizeConfig {
                estimated_height: 40.0,
                estimated_width: 300.0,
                dynamic_height: true,
                dynamic_width: false,
                min_height: 30.0,
                max_height: 200.0,
                min_width: 200.0,
                max_width: 800.0,
            },
            scroll_config: crate::config_provider::virtual_scroll_config::ScrollConfig {
                direction: VirtualScrollDirection::Horizontal,
                smooth_scroll: false,
                scroll_to_alignment: crate::config_provider::virtual_scroll_config::ScrollAlignment::Center,
                overscan_count: 5,
            },
            ..VirtualScrollConfig::new()
        };
        
        assert_eq!(vs_config.buffer_config.buffer_size_before, 10);
        assert_eq!(vs_config.item_size_config.estimated_height, 40.0);
        assert_eq!(vs_config.scroll_config.direction, VirtualScrollDirection::Horizontal);
        assert!(!vs_config.scroll_config.smooth_scroll);
    }
    
    /// 测试配置构建器
    #[test]
    fn test_config_builder() {
        let result = ConfigProviderBuilder::new()
            .theme_config(ThemeConfig::default())
            .locale_config(LocaleConfig::default())
            .component_config(ComponentConfig {
                button: Some(ButtonConfig {
                    default_size: ComponentSize::Large,
                    ..ButtonConfig::default()
                }),
                ..ComponentConfig::default()
            })
            .security_config(SecurityConfig {
                xss_protection: true,
                ..SecurityConfig::default()
            })
            .merge_strategy(MergeStrategy::DeepMerge)
            .enable_validation(true)
            .enable_cache(false)
            .build();
        
        assert!(result.is_ok());
        
        let config = result.unwrap();
        assert!(config.theme_config.is_some());
        assert!(config.component_config.is_some());
        assert!(config.security_config.is_some());
        assert_eq!(config.merge_strategy, Some(MergeStrategy::DeepMerge));
        assert_eq!(config.enable_validation, Some(true));
        assert_eq!(config.enable_cache, Some(false));
    }
    
    /// 测试预设配置构建器
    #[test]
    fn test_preset_config_builder() {
        // 测试开发环境预设
        let dev_result = PresetConfigBuilder::development().build();
        assert!(dev_result.is_ok());
        let dev_config = dev_result.unwrap();
        assert_eq!(dev_config.enable_validation, Some(true));
        assert_eq!(dev_config.enable_cache, Some(false));
        
        // 测试生产环境预设
        let prod_result = PresetConfigBuilder::production().build();
        assert!(prod_result.is_ok());
        let prod_config = prod_result.unwrap();
        assert_eq!(prod_config.enable_validation, Some(false));
        assert_eq!(prod_config.enable_cache, Some(true));
        
        // 测试安全模式预设
        let secure_result = PresetConfigBuilder::secure().build();
        assert!(secure_result.is_ok());
        let secure_config = secure_result.unwrap();
        assert!(secure_config.security_config.is_some());
        
        let security = secure_config.security_config.unwrap();
        assert!(security.xss_protection);
        assert!(security.style_isolation);
        assert!(security.csp_config.is_some());
    }
    
    /// 测试配置合并
    #[test]
    fn test_config_merge() {
        let base_config = ComponentConfig {
            button: Some(ButtonConfig {
                default_size: ComponentSize::Middle,
                loading_delay: 200,
                ..ButtonConfig::default()
            }),
            ..ComponentConfig::default()
        };
        
        let override_config = ComponentConfig {
            button: Some(ButtonConfig {
                default_size: ComponentSize::Large,
                auto_focus: true,
                ..ButtonConfig::default()
            }),
            input: Some(InputConfig {
                default_size: ComponentSize::Small,
                ..InputConfig::default()
            }),
            ..ComponentConfig::default()
        };
        
        let merged = base_config.merge(&override_config, &MergeStrategy::DeepMerge).unwrap();
        
        assert!(merged.button.is_some());
        assert!(merged.input.is_some());
        
        let button = merged.button.unwrap();
        assert_eq!(button.default_size, ComponentSize::Large); // 被覆盖
        assert_eq!(button.loading_delay, 200); // 保留原值
        assert!(button.auto_focus); // 新增值
    }
    
    /// 测试配置验证
    #[test]
    fn test_config_validation() {
        // 测试有效配置
        let valid_config = ComponentConfig {
            button: Some(ButtonConfig {
                loading_delay: 100,
                ..ButtonConfig::default()
            }),
            ..ComponentConfig::default()
        };
        
        assert!(valid_config.validate().is_ok());
        
        // 测试无效配置
        let invalid_config = ComponentConfig {
            button: Some(ButtonConfig {
                loading_delay: 10000, // 超出合理范围
                ..ButtonConfig::default()
            }),
            ..ComponentConfig::default()
        };
        
        assert!(invalid_config.validate().is_err());
    }
    
    /// 测试配置管理器
    #[test]
    fn test_config_manager() {
        let mut manager = ConfigManager::new();
        
        // 添加配置源
        let config_value = json!({
            "button": {
                "default_size": "Large",
                "loading_delay": 300
            }
        });
        
        manager.add_source("test", config_value.clone(), crate::config_provider::config_utils::ConfigPriority::Normal);
        
        // 获取合并后的配置
        let merged_result = manager.get_merged_config();
        assert!(merged_result.is_ok());
        
        // 移除配置源
        manager.remove_source("test");
        let empty_result = manager.get_merged_config();
        assert!(empty_result.is_ok());
    }
    
    /// 测试配置快照
    #[test]
    fn test_config_snapshot() {
        let config = ComponentConfig {
            button: Some(ButtonConfig {
                default_size: ComponentSize::Large,
                ..ButtonConfig::default()
            }),
            ..ComponentConfig::default()
        };
        
        let snapshot = ConfigSnapshot::create(&config, "test_snapshot").unwrap();
        
        assert_eq!(snapshot.name, "test_snapshot");
        assert!(snapshot.timestamp > 0);
        
        let restored: ComponentConfig = snapshot.restore().unwrap();
        assert!(restored.button.is_some());
        assert_eq!(restored.button.unwrap().default_size, ComponentSize::Large);
    }
    
    /// 测试虚拟滚动管理器
    #[test]
    fn test_virtual_scroll_manager() {
        let config = VirtualScrollConfig::new();
        let mut manager = VirtualScrollManager::new(config);
        
        // 测试滚动到指定位置
        manager.scroll_to_index(10, crate::config_provider::virtual_scroll_config::ScrollAlignment::Center);
        assert_eq!(manager.get_current_index(), 10);
        
        // 测试获取可见范围
        let visible_range = manager.get_visible_range();
        assert!(visible_range.start <= visible_range.end);
        
        // 测试更新配置
        let new_config = VirtualScrollConfig {
            buffer_config: crate::config_provider::virtual_scroll_config::BufferConfig {
                buffer_size_before: 20,
                ..crate::config_provider::virtual_scroll_config::BufferConfig::default()
            },
            ..VirtualScrollConfig::new()
        };
        
        manager.update_config(new_config);
        assert_eq!(manager.config.buffer_config.buffer_size_before, 20);
    }
    
    /// 测试弹出层管理器
    #[test]
    fn test_popup_manager() {
        let config = PopupConfig::new();
        let mut manager = PopupManager::new(config);
        
        // 注册弹出层
        let popup_id = "test_popup";
        manager.register_popup(popup_id.to_string(), PopupPlacement::Bottom);
        
        // 显示弹出层
        manager.show_popup(popup_id);
        assert!(manager.is_popup_visible(popup_id));
        
        // 隐藏弹出层
        manager.hide_popup(popup_id);
        assert!(!manager.is_popup_visible(popup_id));
        
        // 移除弹出层
        manager.remove_popup(popup_id);
        assert!(!manager.is_popup_registered(popup_id));
    }
    
    /// 测试样式安全验证器
    #[test]
    fn test_style_security_validator() {
        let validator = StyleSecurityValidator::new(
            vec!["color".to_string(), "background".to_string()],
            vec!["javascript".to_string(), "expression".to_string()],
        );
        
        // 测试安全样式
        assert!(validator.validate_style_property("color", "red"));
        assert!(validator.validate_style_property("background", "#ffffff"));
        
        // 测试不安全样式
        assert!(!validator.validate_style_property("behavior", "url(#default#VML)"));
        assert!(!validator.validate_style_property("color", "javascript:alert(1)"));
        
        // 测试CSS规则
        let safe_css = ".button { color: red; background: blue; }";
        assert!(validator.validate_css_rules(safe_css));
        
        let unsafe_css = ".button { behavior: url(#default#VML); }";
        assert!(!validator.validate_css_rules(unsafe_css));
    }
    
    /// 测试Nonce生成器
    #[test]
    fn test_nonce_generator() {
        let generator = NonceGenerator::new();
        
        let nonce1 = generator.generate();
        let nonce2 = generator.generate();
        
        // 确保每次生成的nonce都不同
        assert_ne!(nonce1, nonce2);
        
        // 确保nonce长度正确
        assert_eq!(nonce1.len(), 32);
        assert_eq!(nonce2.len(), 32);
        
        // 确保nonce只包含有效字符
        assert!(nonce1.chars().all(|c| c.is_ascii_alphanumeric()));
        assert!(nonce2.chars().all(|c| c.is_ascii_alphanumeric()));
    }
    
    /// 测试配置错误处理
    #[test]
    fn test_config_error_handling() {
        // 测试验证错误
        let validation_error = ConfigError::ValidationError("测试验证错误".to_string());
        assert!(matches!(validation_error, ConfigError::ValidationError(_)));
        
        // 测试合并错误
        let merge_error = ConfigError::MergeError("测试合并错误".to_string());
        assert!(matches!(merge_error, ConfigError::MergeError(_)));
        
        // 测试序列化错误
        let serialization_error = ConfigError::SerializationError("测试序列化错误".to_string());
        assert!(matches!(serialization_error, ConfigError::SerializationError(_)));
        
        // 测试错误显示
        let error_message = format!("{}", validation_error);
        assert!(error_message.contains("测试验证错误"));
    }
    
    /// 测试配置差异比较
    #[test]
    fn test_config_diff() {
        let config1 = ComponentConfig {
            button: Some(ButtonConfig {
                default_size: ComponentSize::Middle,
                loading_delay: 200,
                ..ButtonConfig::default()
            }),
            ..ComponentConfig::default()
        };
        
        let config2 = ComponentConfig {
            button: Some(ButtonConfig {
                default_size: ComponentSize::Large,
                loading_delay: 200,
                auto_focus: true,
                ..ButtonConfig::default()
            }),
            input: Some(InputConfig::default()),
            ..ComponentConfig::default()
        };
        
        let diff = crate::config_provider::config_utils::utils::compare_configs(&config1, &config2).unwrap();
        
        assert!(!diff.changes.is_empty());
        
        // 检查是否检测到了按钮配置的变化
        let button_changes: Vec<_> = diff.changes.iter()
            .filter(|change| change.field_path.starts_with("button"))
            .collect();
        assert!(!button_changes.is_empty());
    }
    
    /// 测试配置扁平化和反扁平化
    #[test]
    fn test_config_flatten_unflatten() {
        let config = ComponentConfig {
            button: Some(ButtonConfig {
                default_size: ComponentSize::Large,
                loading_delay: 300,
                auto_focus: true,
                ..ButtonConfig::default()
            }),
            ..ComponentConfig::default()
        };
        
        // 扁平化
        let flattened = crate::config_provider::config_utils::utils::flatten_config(&config).unwrap();
        assert!(!flattened.is_empty());
        
        // 检查扁平化结果
        assert!(flattened.contains_key("button.default_size"));
        assert!(flattened.contains_key("button.loading_delay"));
        assert!(flattened.contains_key("button.auto_focus"));
        
        // 反扁平化
        let unflattened: ComponentConfig = crate::config_provider::config_utils::utils::unflatten_config(&flattened).unwrap();
        
        assert!(unflattened.button.is_some());
        let button = unflattened.button.unwrap();
        assert_eq!(button.default_size, ComponentSize::Large);
        assert_eq!(button.loading_delay, 300);
        assert!(button.auto_focus);
    }
    
    /// 测试多配置合并
    #[test]
    fn test_multi_config_merge() {
        let configs = vec![
            ComponentConfig {
                button: Some(ButtonConfig {
                    default_size: ComponentSize::Small,
                    ..ButtonConfig::default()
                }),
                ..ComponentConfig::default()
            },
            ComponentConfig {
                button: Some(ButtonConfig {
                    loading_delay: 300,
                    ..ButtonConfig::default()
                }),
                input: Some(InputConfig::default()),
                ..ComponentConfig::default()
            },
            ComponentConfig {
                button: Some(ButtonConfig {
                    auto_focus: true,
                    ..ButtonConfig::default()
                }),
                table: Some(TableConfig::default()),
                ..ComponentConfig::default()
            },
        ];
        
        let merged = crate::config_provider::config_utils::utils::merge_multiple_configs(
            &configs,
            &MergeStrategy::DeepMerge
        ).unwrap();
        
        assert!(merged.button.is_some());
        assert!(merged.input.is_some());
        assert!(merged.table.is_some());
        
        let button = merged.button.unwrap();
        assert_eq!(button.default_size, ComponentSize::Small);
        assert_eq!(button.loading_delay, 300);
        assert!(button.auto_focus);
    }
    
    /// 性能测试
    #[test]
    fn test_performance() {
        use std::time::Instant;
        
        // 测试大量配置合并的性能
        let start = Instant::now();
        
        let mut configs = Vec::new();
        for i in 0..1000 {
            configs.push(ComponentConfig {
                button: Some(ButtonConfig {
                    loading_delay: 100 + i,
                    ..ButtonConfig::default()
                }),
                ..ComponentConfig::default()
            });
        }
        
        let _merged = crate::config_provider::config_utils::utils::merge_multiple_configs(
            &configs,
            &MergeStrategy::DeepMerge
        ).unwrap();
        
        let duration = start.elapsed();
        
        // 确保合并1000个配置在合理时间内完成（这里设置为1秒）
        assert!(duration.as_secs() < 1, "配置合并性能测试失败，耗时: {:?}", duration);
    }
    
    /// 集成测试
    #[test]
    fn test_integration() {
        // 创建完整的配置
        let result = ConfigProviderBuilder::new()
            .theme_config(ThemeConfig::default())
            .locale_config(LocaleConfig::default())
            .component_config(ComponentConfig {
                button: Some(ButtonConfig {
                    default_size: ComponentSize::Large,
                    loading_delay: 300,
                    auto_focus: true,
                    ..ButtonConfig::default()
                }),
                input: Some(InputConfig {
                    default_size: ComponentSize::Middle,
                    placeholder: Some("集成测试".to_string()),
                    allow_clear: true,
                    ..InputConfig::default()
                }),
                ..ComponentConfig::default()
            })
            .security_config(SecurityConfig {
                csp_config: Some(CspConfig {
                    nonce: Some("integration-test-nonce".to_string()),
                    strict_mode: true,
                    ..CspConfig::default()
                }),
                xss_protection: true,
                style_isolation: true,
                ..SecurityConfig::default()
            })
            .popup_config(PopupConfig {
                auto_adjust_overflow: true,
                placement: PopupPlacement::Bottom,
                z_index_base: 1500,
                ..PopupConfig::new()
            })
            .virtual_scroll_config(VirtualScrollConfig {
                buffer_config: crate::config_provider::virtual_scroll_config::BufferConfig {
                    buffer_size_before: 8,
                    buffer_size_after: 8,
                    ..crate::config_provider::virtual_scroll_config::BufferConfig::default()
                },
                ..VirtualScrollConfig::new()
            })
            .merge_strategy(MergeStrategy::DeepMerge)
            .enable_validation(true)
            .enable_cache(true)
            .build();
        
        assert!(result.is_ok());
        
        let config = result.unwrap();
        
        // 验证所有配置都正确设置
        assert!(config.theme_config.is_some());
        assert!(config.locale_config.is_some());
        assert!(config.component_config.is_some());
        assert!(config.security_config.is_some());
        assert!(config.popup_config.is_some());
        assert!(config.virtual_scroll_config.is_some());
        
        // 验证组件配置
        let comp_config = config.component_config.unwrap();
        assert!(comp_config.button.is_some());
        assert!(comp_config.input.is_some());
        
        let button = comp_config.button.unwrap();
        assert_eq!(button.default_size, ComponentSize::Large);
        assert_eq!(button.loading_delay, 300);
        assert!(button.auto_focus);
        
        let input = comp_config.input.unwrap();
        assert_eq!(input.default_size, ComponentSize::Middle);
        assert_eq!(input.placeholder, Some("集成测试".to_string()));
        assert!(input.allow_clear);
        
        // 验证安全配置
        let sec_config = config.security_config.unwrap();
        assert!(sec_config.csp_config.is_some());
        assert!(sec_config.xss_protection);
        assert!(sec_config.style_isolation);
        
        let csp = sec_config.csp_config.unwrap();
        assert_eq!(csp.nonce, Some("integration-test-nonce".to_string()));
        assert!(csp.strict_mode);
        
        // 验证弹出层配置
        let popup_conf = config.popup_config.unwrap();
        assert!(popup_conf.auto_adjust_overflow);
        assert_eq!(popup_conf.placement, PopupPlacement::Bottom);
        assert_eq!(popup_conf.z_index_base, 1500);
        
        // 验证虚拟滚动配置
        let vs_config = config.virtual_scroll_config.unwrap();
        assert_eq!(vs_config.buffer_config.buffer_size_before, 8);
        assert_eq!(vs_config.buffer_config.buffer_size_after, 8);
        
        // 验证其他设置
        assert_eq!(config.merge_strategy, Some(MergeStrategy::DeepMerge));
        assert_eq!(config.enable_validation, Some(true));
        assert_eq!(config.enable_cache, Some(true));
    }
}
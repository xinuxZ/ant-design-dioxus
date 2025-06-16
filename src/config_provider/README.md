# ConfigProvider - 全局配置提供者

`ConfigProvider` 是 ant-design-dioxus 的核心配置管理组件，提供了统一的全局配置管理能力，支持主题、语言、组件、安全、弹出层和虚拟滚动等多种配置。

## 功能特性

### 🎨 主题配置
- 支持亮色/暗色主题切换
- 自定义主题色彩
- 动态主题变更
- CSS变量注入

### 🌍 国际化配置
- 多语言支持
- 动态语言切换
- 自定义语言包
- RTL/LTR文本方向

### 🧩 组件配置
- 统一组件默认属性
- 组件尺寸规范
- 组件行为定制
- 批量组件配置

### 🔒 安全配置
- 内容安全策略(CSP)
- XSS防护
- 样式隔离
- 动态样式验证

### 📋 弹出层配置
- 全局弹出层管理
- 位置自动调整
- z-index管理
- 容器配置

### 📜 虚拟滚动配置
- 大数据列表优化
- 缓冲区管理
- 动态高度支持
- 性能优化

## 快速开始

### 基础使用

```rust
use dioxus::prelude::*;
use ant_design_dioxus::config_provider::{
    ConfigProvider, ComponentConfig, ComponentSize
};

#[component]
fn App() -> Element {
    rsx! {
        ConfigProvider {
            component_config: ComponentConfig {
                button: Some(ButtonConfig {
                    default_size: ComponentSize::Large,
                    ..ButtonConfig::default()
                }),
                ..ComponentConfig::default()
            },
            
            div { class: "app",
                // 你的应用内容
                MyComponent {}
            }
        }
    }
}

#[component]
fn MyComponent() -> Element {
    let component_config = use_component_config();
    
    rsx! {
        div {
            "组件配置已生效"
        }
    }
}
```

### 使用构建器模式

```rust
use ant_design_dioxus::config_provider::{
    ConfigProviderBuilder, MergeStrategy, ComponentSize
};

let config = ConfigProviderBuilder::new()
    .theme_config(ThemeConfig::default())
    .component_config(ComponentConfig {
        button: Some(ButtonConfig {
            default_size: ComponentSize::Large,
            loading_delay: 300,
            ..ButtonConfig::default()
        }),
        ..ComponentConfig::default()
    })
    .security_config(SecurityConfig {
        xss_protection: true,
        style_isolation: true,
        ..SecurityConfig::default()
    })
    .merge_strategy(MergeStrategy::DeepMerge)
    .enable_validation(true)
    .build()
    .unwrap();
```

### 使用预设配置

```rust
use ant_design_dioxus::config_provider::PresetConfigBuilder;

// 开发环境配置
let dev_config = PresetConfigBuilder::development().build().unwrap();

// 生产环境配置
let prod_config = PresetConfigBuilder::production().build().unwrap();

// 安全模式配置
let secure_config = PresetConfigBuilder::secure().build().unwrap();

// 高性能配置
let perf_config = PresetConfigBuilder::high_performance().build().unwrap();
```

## 配置类型详解

### 组件配置 (ComponentConfig)

支持以下组件的全局配置：

#### 按钮配置 (ButtonConfig)
```rust
ButtonConfig {
    default_size: ComponentSize::Middle,    // 默认尺寸
    default_type: ButtonType::Default,      // 默认类型
    loading_delay: 200,                     // 加载延迟(ms)
    auto_focus: false,                      // 自动聚焦
    block: false,                          // 块级按钮
    danger: false,                         // 危险按钮
    disabled: false,                       // 禁用状态
    ghost: false,                          // 幽灵按钮
    loading: false,                        // 加载状态
    shape: ButtonShape::Default,           // 按钮形状
    // ...
}
```

#### 输入框配置 (InputConfig)
```rust
InputConfig {
    default_size: ComponentSize::Middle,    // 默认尺寸
    placeholder: Some("请输入".to_string()), // 占位符
    allow_clear: false,                     // 允许清除
    bordered: true,                         // 显示边框
    disabled: false,                        // 禁用状态
    max_length: None,                       // 最大长度
    show_count: false,                      // 显示字符计数
    // ...
}
```

#### 表格配置 (TableConfig)
```rust
TableConfig {
    default_size: ComponentSize::Middle,    // 默认尺寸
    bordered: false,                        // 显示边框
    show_header: true,                      // 显示表头
    pagination: Some(PaginationConfig {     // 分页配置
        page_size: 10,
        show_size_changer: true,
        show_quick_jumper: false,
        // ...
    }),
    // ...
}
```

### 安全配置 (SecurityConfig)

#### CSP配置 (CspConfig)
```rust
CspConfig {
    nonce: Some("random-nonce-123".to_string()), // CSP nonce
    strict_mode: true,                            // 严格模式
    style_src: vec!["'self'".to_string()],        // 样式源
    script_src: vec!["'self'".to_string()],       // 脚本源
    img_src: vec!["'self'".to_string()],          // 图片源
    font_src: vec!["'self'".to_string()],         // 字体源
    connect_src: vec!["'self'".to_string()],      // 连接源
    // ...
}
```

#### 安全特性
```rust
SecurityConfig {
    csp_config: Some(csp_config),           // CSP配置
    xss_protection: true,                    // XSS防护
    style_isolation: true,                   // 样式隔离
    dynamic_style_validation: true,          // 动态样式验证
    css_property_whitelist: vec![            // CSS属性白名单
        "color".to_string(),
        "background".to_string(),
        // ...
    ],
    css_property_blacklist: vec![            // CSS属性黑名单
        "javascript".to_string(),
        "expression".to_string(),
        // ...
    ],
    // ...
}
```

### 弹出层配置 (PopupConfig)

```rust
PopupConfig {
    get_popup_container: None,               // 弹出层容器
    auto_adjust_overflow: true,              // 自动调整溢出
    placement: PopupPlacement::BottomLeft,   // 默认位置
    trigger: vec!["click".to_string()],      // 触发方式
    z_index_base: 1000,                      // z-index基础值
    virtual_positioning: false,              // 虚拟定位
    margin: 4.0,                            // 边距
    // ...
}
```

### 虚拟滚动配置 (VirtualScrollConfig)

#### 缓冲区配置
```rust
BufferConfig {
    buffer_size_before: 5,                   // 前置缓冲区大小
    buffer_size_after: 5,                    // 后置缓冲区大小
    max_buffer_size: 50,                     // 最大缓冲区大小
    dynamic_buffer: true,                    // 动态缓冲区
}
```

#### 项目尺寸配置
```rust
ItemSizeConfig {
    estimated_height: 32.0,                  // 估算高度
    estimated_width: 200.0,                  // 估算宽度
    dynamic_height: true,                    // 动态高度
    dynamic_width: false,                    // 动态宽度
    min_height: 20.0,                        // 最小高度
    max_height: 100.0,                       // 最大高度
    min_width: 100.0,                        // 最小宽度
    max_width: 500.0,                        // 最大宽度
}
```

## Hooks API

### 基础配置Hooks

```rust
// 获取全局配置
let config = use_config();

// 获取主题配置
let theme_config = use_theme_config();

// 获取语言配置
let locale_config = use_locale_config();

// 获取组件配置
let component_config = use_component_config();

// 获取安全配置
let security_config = use_security_config();

// 获取弹出层配置
let popup_config = use_popup_config();

// 获取虚拟滚动配置
let virtual_scroll_config = use_virtual_scroll_config();
```

### 配置管理Hooks

```rust
// 获取配置管理器
let config_manager = use_config_manager();

// 获取配置更新器
let config_updater = use_config_updater();

// 获取配置重置器
let config_reset = use_config_reset();

// 获取配置验证器
let config_validator = use_config_validator();

// 获取配置导出器
let config_export = use_config_export();

// 获取配置导入器
let config_import = use_config_import();

// 获取配置监听器
let config_watcher = use_config_watcher();

// 获取配置缓存
let config_cache = use_config_cache();

// 获取配置同步器
let config_sync = use_config_sync();
```

### 使用示例

```rust
#[component]
fn ConfigManager() -> Element {
    let config_updater = use_config_updater();
    let config_validator = use_config_validator();
    let config_export = use_config_export();
    
    let update_theme = move |_| {
        let new_theme = ThemeConfig::default();
        let theme_value = serde_json::to_value(&new_theme).unwrap();
        if let Err(e) = config_updater("theme", theme_value) {
            println!("更新主题失败: {:?}", e);
        }
    };
    
    let validate_config = move |_| {
        match config_validator() {
            Ok(_) => println!("配置验证通过"),
            Err(errors) => println!("配置验证失败: {:?}", errors),
        }
    };
    
    let export_config = move |_| {
        match config_export() {
            Ok(config) => {
                let json = serde_json::to_string_pretty(&config).unwrap();
                println!("导出配置: {}", json);
            }
            Err(e) => println!("导出失败: {:?}", e),
        }
    };
    
    rsx! {
        div {
            button { onclick: update_theme, "更新主题" }
            button { onclick: validate_config, "验证配置" }
            button { onclick: export_config, "导出配置" }
        }
    }
}
```

## 配置合并策略

### MergeStrategy枚举

```rust
pub enum MergeStrategy {
    Replace,     // 替换策略：新配置完全替换旧配置
    ShallowMerge, // 浅层合并：只合并第一层属性
    DeepMerge,   // 深度合并：递归合并所有层级
}
```

### 使用示例

```rust
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
    input: Some(InputConfig::default()),
    ..ComponentConfig::default()
};

// 深度合并
let merged = base_config.merge(&override_config, &MergeStrategy::DeepMerge).unwrap();
// 结果：button配置被合并，保留loading_delay=200，更新default_size=Large，添加auto_focus=true
//      同时添加input配置
```

## 配置验证

### 内置验证规则

- **按钮配置**：loading_delay范围验证(0-5000ms)
- **输入框配置**：max_length范围验证
- **表格配置**：page_size范围验证
- **安全配置**：CSP规则格式验证
- **弹出层配置**：z_index范围验证
- **虚拟滚动配置**：缓冲区大小验证

### 自定义验证

```rust
impl ConfigValidate for MyCustomConfig {
    fn validate(&self) -> Result<(), Vec<ConfigError>> {
        let mut errors = Vec::new();
        
        if self.some_value < 0 {
            errors.push(ConfigError::ValidationError(
                "some_value must be non-negative".to_string()
            ));
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
```

## 配置持久化

### 导出配置

```rust
let config_export = use_config_export();

match config_export() {
    Ok(config_json) => {
        let json_string = serde_json::to_string_pretty(&config_json).unwrap();
        // 保存到文件或发送到服务器
        save_to_file("config.json", &json_string);
    }
    Err(e) => println!("导出失败: {:?}", e),
}
```

### 导入配置

```rust
let config_import = use_config_import();

// 从文件加载配置
let config_json: serde_json::Value = load_from_file("config.json").unwrap();

match config_import(config_json) {
    Ok(_) => println!("配置导入成功"),
    Err(e) => println!("导入失败: {:?}", e),
}
```

### 使用构建器持久化

```rust
let config = ConfigProviderBuilder::new()
    .load_from_file("config.json")  // 从文件加载
    .theme_config(ThemeConfig::default())
    .build()
    .unwrap();

// 保存配置
config.save_to_file("updated_config.json").unwrap();
```

## 性能优化

### 配置缓存

```rust
let config = ConfigProviderBuilder::new()
    .enable_cache(true)  // 启用配置缓存
    .cache_ttl(300)      // 缓存TTL(秒)
    .build()
    .unwrap();
```

### 配置同步

```rust
let config = ConfigProviderBuilder::new()
    .enable_sync(true)           // 启用配置同步
    .sync_interval(60)           // 同步间隔(秒)
    .sync_endpoint("/api/config") // 同步端点
    .build()
    .unwrap();
```

### 虚拟滚动优化

```rust
let vs_config = VirtualScrollConfig {
    performance_config: PerformanceConfig {
        enable_optimization: true,    // 启用性能优化
        throttle_scroll: true,        // 滚动节流
        throttle_delay: 16,          // 节流延迟(ms)
        use_transform: true,         // 使用transform
        use_will_change: true,       // 使用will-change
    },
    ..VirtualScrollConfig::new()
};
```

## 安全最佳实践

### CSP配置

```rust
let csp_config = CspConfig {
    nonce: Some(generate_secure_nonce()),  // 使用安全的nonce
    strict_mode: true,                     // 启用严格模式
    style_src: vec![
        "'self'".to_string(),
        "'nonce-{nonce}".to_string(),      // 使用nonce
    ],
    script_src: vec![
        "'self'".to_string(),
        "'nonce-{nonce}".to_string(),
    ],
    object_src: vec!["'none'".to_string()], // 禁用object
    base_uri: vec!["'self'".to_string()],   // 限制base URI
    ..CspConfig::default()
};
```

### 样式安全

```rust
let security_config = SecurityConfig {
    style_isolation: true,               // 启用样式隔离
    dynamic_style_validation: true,      // 启用动态样式验证
    css_property_whitelist: vec![        // 只允许安全的CSS属性
        "color".to_string(),
        "background-color".to_string(),
        "font-size".to_string(),
        "margin".to_string(),
        "padding".to_string(),
    ],
    css_property_blacklist: vec![        // 禁用危险的CSS属性
        "javascript".to_string(),
        "expression".to_string(),
        "behavior".to_string(),
        "binding".to_string(),
    ],
    ..SecurityConfig::default()
};
```

## 故障排除

### 常见问题

1. **配置不生效**
   - 检查ConfigProvider是否正确包装了组件
   - 确认使用了正确的hooks
   - 验证配置格式是否正确

2. **性能问题**
   - 启用配置缓存
   - 减少不必要的配置更新
   - 使用浅层合并策略

3. **安全警告**
   - 检查CSP配置
   - 验证样式安全设置
   - 确认nonce生成正确

### 调试技巧

```rust
// 启用调试模式
let config = ConfigProviderBuilder::new()
    .debug_mode(true)  // 启用调试输出
    .build()
    .unwrap();

// 使用配置监听器
let config_watcher = use_config_watcher();
config_watcher.on_change(|config_type, old_value, new_value| {
    println!("配置变更: {} {} -> {}", config_type, old_value, new_value);
});

// 验证配置
let config_validator = use_config_validator();
match config_validator() {
    Ok(_) => println!("✅ 配置验证通过"),
    Err(errors) => {
        println!("❌ 配置验证失败:");
        for error in errors {
            println!("  - {:?}", error);
        }
    }
}
```

## 示例项目

查看 `examples.rs` 文件获取完整的使用示例，包括：

- 基础ConfigProvider使用
- 构建器模式配置
- 预设配置使用
- 动态配置更新
- 配置导入导出
- 完整功能演示

## API参考

详细的API文档请参考各个模块的文档注释：

- `component_config.rs` - 组件配置
- `security_config.rs` - 安全配置
- `popup_config.rs` - 弹出层配置
- `virtual_scroll_config.rs` - 虚拟滚动配置
- `config_utils.rs` - 配置工具
- `hooks.rs` - Hooks API
- `builder.rs` - 构建器API

## 贡献指南

欢迎贡献代码！请遵循以下步骤：

1. Fork项目
2. 创建功能分支
3. 编写测试
4. 提交代码
5. 创建Pull Request

### 开发环境

```bash
# 运行测试
cargo test config_provider

# 运行示例
cargo run --example config_provider_demo

# 生成文档
cargo doc --open
```

## 许可证

MIT License
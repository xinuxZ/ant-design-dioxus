# Ant Design Dioxus 主题系统

本文档详细介绍了 Ant Design Dioxus 的主题系统设计和使用方法。

## 设计理念

Ant Design Dioxus 主题系统基于 css-in-rust 的主题功能构建，专为 Dioxus 框架优化，提供了完整的主题定制和切换能力。主题系统遵循 Ant Design 的设计规范，实现了三层设计令牌体系：

1. **种子令牌 (Seed Tokens)**: 基础设计变量，如主色、成功色、边框圆角等
2. **映射令牌 (Map Tokens)**: 由种子令牌派生的设计变量，如颜色调色板、文本颜色等
3. **别名令牌 (Alias Tokens)**: 组件使用的特定设计变量，引用映射令牌

## 主题系统架构

```
ant-design-dioxus/src/theme/
├── mod.rs                 # 主题模块入口
├── core/                  # 核心类型和功能
│   ├── mod.rs
│   └── types.rs           # 定义核心类型（Size、ColorType等）
├── algorithm/             # 主题算法
│   └── mod.rs             # 颜色生成和主题派生算法
├── provider/              # 主题提供者
│   └── mod.rs             # ThemeProvider组件和相关Hook
├── tokens/                # 主题令牌
│   ├── mod.rs             # 令牌生成和管理
│   └── presets/           # 预设令牌
│       └── mod.rs         # 预设令牌配置
├── presets/               # 预设主题
│   └── mod.rs             # 预设主题配置
├── hooks/                 # 主题钩子
│   └── mod.rs             # 主题相关的Hook函数
└── README.md              # 文档说明
```

## 核心类型

### ThemeConfig

主题配置是主题系统的核心，包含了主题实例、紧凑模式标志、自定义令牌、组件令牌和算法配置：

```rust
pub struct ThemeConfig {
    /// 主题实例
    pub theme: Theme,
    /// 是否启用紧凑模式
    pub compact: bool,
    /// 自定义令牌
    pub token: HashMap<String, String>,
    /// 组件令牌
    pub components: HashMap<String, HashMap<String, String>>,
    /// 算法配置
    pub algorithm: Vec<Algorithm>,
}
```

### 主题令牌

主题系统定义了三层令牌：

```rust
// 种子令牌
pub struct SeedToken {
    pub color_primary: String,
    pub color_success: String,
    pub color_warning: String,
    pub color_error: String,
    pub color_info: String,
    pub font_size: f32,
    pub border_radius: f32,
    pub wireframe: bool,
}

// 映射令牌
pub struct MapToken {
    pub color_primary_palette: Vec<String>,
    pub color_bg_base: String,
    pub color_text_base: String,
    // ...其他派生颜色和尺寸
}

// 别名令牌
pub struct AliasToken {
    pub component_background: String,
    pub component_text_color: String,
    pub component_border_color: String,
    pub button_height_sm: f32,
    pub button_height: f32,
    pub button_height_lg: f32,
    // ...其他组件特定变量
}
```

### 主题算法

主题系统支持多种算法，用于生成完整的主题令牌：

```rust
pub enum Algorithm {
    /// 默认算法
    Default,
    /// 暗色算法
    Dark,
    /// 紧凑算法
    Compact,
    /// 自定义算法
    Custom(String),
}
```

## 使用方法

### 基本用法

使用预设主题：

```rust
use dioxus::prelude::*;
use ant_design_dioxus::theme::{ThemeProvider, presets::light_theme};

#[component]
fn App() -> Element {
    let theme_config = use_signal(|| light_theme());

    rsx! {
        ThemeProvider {
            config: theme_config,
            div {
                "Hello, Ant Design Dioxus!"
            }
        }
    }
}
```

### 主题切换

动态切换主题：

```rust
use dioxus::prelude::*;
use ant_design_dioxus::theme::{
    ThemeProvider,
    presets::{light_theme, dark_theme},
    provider::use_theme_switch
};

#[component]
fn ThemeSwitcher() -> Element {
    let theme_config = use_signal(|| light_theme());
    let theme_switch = use_theme_switch();

    let toggle_theme = move |_| {
        if theme_config.read().algorithm.iter().any(|alg| matches!(alg, Algorithm::Dark)) {
            theme_switch(light_theme());
        } else {
            theme_switch(dark_theme());
        }
    };

    rsx! {
        ThemeProvider {
            config: theme_config,
            div {
                button {
                    onclick: toggle_theme,
                    "切换主题"
                }
                div {
                    "当前内容"
                }
            }
        }
    }
}
```

### 自定义主题

创建自定义主题：

```rust
use ant_design_dioxus::theme::ThemeConfig;

// 创建自定义主题
let custom_theme = ThemeConfig::light()
    .with_primary_color("#f5222d")
    .with_success_color("#52c41a")
    .with_warning_color("#faad14")
    .with_error_color("#ff4d4f")
    .with_border_radius(4)
    .with_font_size(16);

// 添加自定义令牌
let custom_theme = custom_theme
    .with_token("colorLink", "#1890ff")
    .with_token("colorLinkHover", "#40a9ff")
    .with_token("colorLinkActive", "#096dd9");
```

### 组件令牌

设置组件特定的令牌：

```rust
use std::collections::HashMap;
use ant_design_dioxus::theme::ThemeConfig;

let mut button_tokens = HashMap::new();
button_tokens.insert("buttonBorderRadius".to_string(), "4px".to_string());
button_tokens.insert("buttonPrimaryBg".to_string(), "#722ed1".to_string());

let custom_theme = ThemeConfig::light()
    .with_component_token("button", button_tokens);
```

### 使用主题令牌

在组件中访问主题令牌：

```rust
use dioxus::prelude::*;
use ant_design_dioxus::theme::provider::use_theme_token;

#[component]
fn StyledButton() -> Element {
    let primary_color = use_theme_token("colorPrimary");
    let border_radius = use_theme_token("borderRadius");

    rsx! {
        button {
            style: "background-color: {primary_color}; border-radius: {border_radius}; color: white;",
            "主题按钮"
        }
    }
}
```

### 预设主题

使用内置的预设主题：

```rust
use ant_design_dioxus::theme::presets::{
    light_theme,
    dark_theme,
    compact_theme,
    compact_dark_theme,
    blue_theme,
    green_theme,
    red_theme,
    yellow_theme,
    purple_theme,
    cyan_theme,
    wireframe_theme,
};
```

## 与 css-in-rust 集成

Ant Design Dioxus 主题系统与 css-in-rust 深度集成，利用其主题变量和 CSS 生成功能：

```rust
use css_in_rust::theme_bridge::ThemeBridge;
use ant_design_dioxus::theme::ThemeConfig;

// 创建主题配置
let theme_config = ThemeConfig::light();

// 创建主题桥接器
let mut bridge = ThemeBridge::new(
    theme_config.theme.clone(),
    css_in_rust::theme::core::css::variables::InjectionStrategy::Replace,
    true,
);

// 注入主题变量
let _ = bridge.sync_theme_variables();
```

## 最佳实践

1. **使用预设主题**: 对于大多数应用，使用预设主题是最简单的方式
2. **组件级定制**: 对特定组件的样式进行定制，而不是全局修改
3. **语义化令牌**: 使用语义化的令牌名称，如 `colorPrimary` 而不是 `blue`
4. **主题切换**: 支持亮色/暗色主题切换，提升用户体验
5. **响应式设计**: 结合响应式钩子，适配不同屏幕尺寸

## 扩展阅读

- [Ant Design 设计令牌](https://ant.design/docs/react/customize-theme-cn)
- [css-in-rust 主题系统](../../../css-in-rust/css-in-rust/src/theme/README.md)
- [Dioxus 框架文档](https://dioxuslabs.com/docs/)

# 主题系统架构设计

## 概述

主题系统是 Ant Design Dioxus 组件库的核心部分，它提供了一套完整的设计系统令牌（Design Tokens）和主题配置接口，使组件能够适应不同的视觉风格和交互模式。通过重构，我们引入了基于泛型和 trait 的设计，使主题系统更加灵活和可扩展。

## 核心抽象

### ColorConfig Trait

`ColorConfig` trait 是颜色配置的核心抽象，它定义了颜色系统应该提供的基本功能：

```rust
pub trait ColorConfig {
    /// 创建浅色主题颜色配置
    fn light() -> Self;

    /// 创建深色主题颜色配置
    fn dark() -> Self;

    /// 获取指定类型的颜色
    fn get_color(&self, color_type: ColorType) -> Option<RgbColor>;

    /// 获取指定类型的颜色调色板
    fn get_palette(&self, color_type: ColorType) -> Option<&ColorPalette>;

    /// 生成 CSS 变量
    fn to_css_variables(&self) -> HashMap<String, String>;

    /// 生成 CSS 样式
    fn generate_css(&self) -> String;

    /// 是否为深色主题
    fn is_dark(&self) -> bool;
}
```

### GenericThemeEngine

`GenericThemeEngine` 是一个通用的主题引擎实现，它使用泛型参数和 trait bound 来支持不同的颜色配置系统：

```rust
pub struct GenericThemeEngine<C>
where
    C: ColorConfig + Clone + PartialEq,
{
    pub name: String,
    pub dark: bool,
    pub compact: bool,
    pub colors: C,
    pub typography_type: String,
    pub spacing_type: String,
    pub sizing_type: String,
    pub animations_type: String,
    pub properties: HashMap<String, String>,
}
```

## 使用方法

### 使用现有的 AntDesignColors

```rust
use ant_design_dioxus::theme::{
    AntDesignColors,
    GenericThemeEngine,
    ColorType,
};

// 创建使用 Ant Design 颜色系统的主题
let theme = GenericThemeEngine::new(
    "AntTheme".to_string(),
    false,
    false,
    AntDesignColors::light(),
    "default".to_string(),
    "default".to_string(),
    "default".to_string(),
    "default".to_string(),
);

// 使用主题
let primary_color = theme.get_color(ColorType::Primary);
```

### 创建自定义颜色配置

要创建自定义颜色配置，只需实现 `ColorConfig` trait：

```rust
use ant_design_dioxus::theme::{
    color::{ColorConfig, ColorPalette, ColorType, RgbColor},
    GenericThemeEngine,
};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
struct CustomColors {
    primary: ColorPalette,
    // 其他颜色字段...
}

impl ColorConfig for CustomColors {
    fn light() -> Self {
        // 创建浅色主题颜色配置
        Self {
            primary: ColorPalette::from_base(RgbColor::from_hex("#1890ff").unwrap()),
            // 初始化其他字段...
        }
    }

    fn dark() -> Self {
        // 创建深色主题颜色配置
        Self {
            primary: ColorPalette::from_base(RgbColor::from_hex("#177ddc").unwrap()),
            // 初始化其他字段...
        }
    }

    fn get_color(&self, color_type: ColorType) -> Option<RgbColor> {
        match color_type {
            ColorType::Primary => Some(self.primary.base),
            // 处理其他颜色类型...
            _ => None,
        }
    }

    fn get_palette(&self, color_type: ColorType) -> Option<&ColorPalette> {
        match color_type {
            ColorType::Primary => Some(&self.primary),
            // 处理其他颜色类型...
            _ => None,
        }
    }

    fn to_css_variables(&self) -> HashMap<String, String> {
        let mut variables = HashMap::new();
        // 生成 CSS 变量...
        variables
    }

    fn generate_css(&self) -> String {
        let mut css = String::new();
        // 生成 CSS 样式...
        css
    }

    fn is_dark(&self) -> bool {
        // 判断是否为深色主题
        false
    }
}

// 创建使用自定义颜色系统的主题
let theme = GenericThemeEngine::new(
    "CustomTheme".to_string(),
    false,
    false,
    CustomColors::light(),
    "default".to_string(),
    "default".to_string(),
    "default".to_string(),
    "default".to_string(),
);
```

## 主题系统架构

主题系统的架构如下：

```
┌─────────────────────────────┐
│     ThemeConfigInterface    │
└─────────────────────────────┘
              ▲
              │
┌─────────────────────────────┐
│       GenericThemeEngine    │
└─────────────────────────────┘
              │
              │ 使用
              ▼
┌─────────────────────────────┐
│        ColorConfig          │
└─────────────────────────────┘
              ▲
              │ 实现
              │
┌─────────────┬───────────────┐
│AntDesignColors│  CustomColors │
└─────────────┴───────────────┘
```

## 优势

1. **灵活性**：通过 trait 和泛型，可以轻松支持不同的颜色系统和主题配置。
2. **可扩展性**：新的颜色系统只需实现 `ColorConfig` trait，就可以与主题引擎无缝集成。
3. **类型安全**：使用泛型和 trait bound 确保类型安全，编译时就能发现潜在问题。
4. **代码复用**：通用主题引擎封装了主题的通用逻辑，避免代码重复。
5. **向后兼容**：现有的 `AntDesignColors` 实现了 `ColorConfig` trait，保持向后兼容。

## 未来扩展

1. **更多设计系统支持**：可以轻松添加对 Material Design、Fluent Design 等其他设计系统的支持。
2. **主题切换**：基于通用主题引擎，可以实现更灵活的主题切换机制。
3. **自定义主题构建器**：可以开发主题构建器，让用户更容易创建自定义主题。

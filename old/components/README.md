# Ant Design Dioxus 组件库

这个目录包含了 Ant Design Dioxus 的所有组件实现。

## 组件主题和国际化支持

所有组件都支持主题切换和语言切换功能，通过 `ConfigProvider` 组件进行全局配置。

### 主题配置

组件库使用三层设计令牌体系：

1. **种子令牌（Seed Token）**：基础设计变量，如主色、圆角等
2. **映射令牌（Map Token）**：由种子令牌派生的令牌，如成功色、警告色等
3. **别名令牌（Alias Token）**：组件使用的具体样式令牌，如按钮背景色、输入框边框色等

#### 主题配置示例

```rust
use ant_design_dioxus::prelude::*;

#[component]
fn App() -> Element {
    // 使用主题构建器创建主题配置
    let theme_config = ThemeBuilder::new()
        .add_token("colorPrimary", "#1890ff")
        .add_token("colorSuccess", "#52c41a")
        .add_token("colorWarning", "#faad14")
        .add_token("colorError", "#ff4d4f")
        .add_token("borderRadius", "4px")
        .compact(true) // 启用紧凑模式
        .build();

    rsx! {
        ConfigProvider {
            theme: Some(theme_config),
            div {
                Button { "按钮" }
                Input { placeholder: "请输入" }
            }
        }
    }
}
```

#### 在组件中使用主题

在自定义组件中，可以通过 Hook 函数获取当前的主题配置：

```rust
use ant_design_dioxus::prelude::*;

#[component]
fn MyComponent() -> Element {
    // 获取主题上下文
    let theme_context = use_theme();

    // 获取特定主题令牌
    let primary_color = use_theme_token("colorPrimary");

    // 获取组件特定令牌
    let button_height = use_component_token("Button", "height");

    rsx! {
        div {
            style: "color: {primary_color}; height: {button_height};",
            "自定义组件"
        }
    }
}
```

### 国际化配置

组件库支持多种语言，通过 `ConfigProvider` 组件的 `locale` 属性进行配置。

#### 语言配置示例

```rust
use ant_design_dioxus::prelude::*;

#[component]
fn App() -> Element {
    rsx! {
        ConfigProvider {
            locale: Some(Locale::En), // 使用英文
            div {
                Button { "Button" }
                DatePicker {}
                Pagination {
                    total: 100,
                    show_size_changer: true
                }
            }
        }
    }
}
```

#### 在组件中使用国际化

在自定义组件中，可以通过 Hook 函数获取当前的语言配置和翻译函数：

```rust
use ant_design_dioxus::prelude::*;

#[component]
fn MyComponent() -> Element {
    // 获取当前语言
    let locale_context = use_locale();

    // 获取翻译函数
    let translate = use_translate();

    // 根据当前语言翻译文本
    let ok_text = translate("ok"); // 返回"确定"或"OK"等
    let cancel_text = translate("cancel"); // 返回"取消"或"Cancel"等

    rsx! {
        div {
            Button { "{ok_text}" }
            Button { "{cancel_text}" }
        }
    }
}
```

### 组件尺寸

组件支持三种尺寸：小号（small）、中号（middle，默认）和大号（large），可以通过 `ConfigProvider` 组件的 `component_size` 属性进行全局配置。

```rust
use ant_design_dioxus::prelude::*;

#[component]
fn App() -> Element {
    rsx! {
        ConfigProvider {
            component_size: Some(ComponentSize::Large),
            div {
                Button { "大号按钮" }
                Input { placeholder: "大号输入框" }
            }
        }
    }
}
```

### 方向配置

组件支持从左到右（LTR）和从右到左（RTL）两种方向，适用于不同的语言环境。

```rust
use ant_design_dioxus::prelude::*;

#[component]
fn App() -> Element {
    rsx! {
        ConfigProvider {
            direction: Some(Direction::Rtl),
            locale: Some(Locale::Ar), // 阿拉伯语
            div {
                Button { "زر" }
                Input { placeholder: "إدخال" }
            }
        }
    }
}
```

## 组件列表

组件库提供了丰富的组件，包括：

- 通用组件：Button、Icon、Typography
- 布局组件：Grid、Layout、Space、Divider
- 导航组件：Menu、Pagination、Steps、Breadcrumb
- 数据录入组件：Form、Input、Select、Checkbox、Radio、Switch
- 数据展示组件：Table、List、Card、Calendar、Descriptions
- 反馈组件：Modal、Drawer、Alert、Message、Notification
- 其他组件：ConfigProvider、Anchor、BackTop

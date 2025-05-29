# 主题与国际化系统改进

本文档详细介绍了 Ant Design Dioxus 主题系统和国际化系统的最新改进，包括新增功能、API 变更和使用示例。

## 🎨 主题系统改进

### 新增功能

#### 1. 主题算法与颜色计算

新增了强大的主题算法功能，支持动态生成颜色调色板：

```rust
use ant_design_dioxus::theme::{ThemeConfig, ColorType};

// 生成完整的颜色调色板
let palette = ThemeConfig::generate_color_palette("#1890ff", ColorType::Primary);

// 生成亮色主题调色板
let light_palette = ThemeConfig::generate_light_palette("#1890ff");

// 生成暗色主题调色板
let dark_palette = ThemeConfig::generate_dark_palette("#1890ff");

// 颜色计算工具
let lighter_color = ThemeConfig::lighten_color("#1890ff", 0.2);
let darker_color = ThemeConfig::darken_color("#1890ff", 0.2);
```

#### 2. 动态主题切换

支持运行时动态切换主题，包括亮色、暗色和紧凑模式：

```rust
use ant_design_dioxus::theme::{use_theme_switch, ThemeConfig};

#[component]
fn ThemeSwitcher() -> Element {
    let theme_switch = use_theme_switch();

    rsx! {
        button {
            onclick: move |_| theme_switch(ThemeConfig::light()),
            "亮色主题"
        }
        button {
            onclick: move |_| theme_switch(ThemeConfig::dark()),
            "暗色主题"
        }
        button {
            onclick: move |_| theme_switch(ThemeConfig::compact()),
            "紧凑主题"
        }
    }
}
```

#### 3. 主题令牌访问

新增便捷的主题令牌访问 Hook：

```rust
use ant_design_dioxus::theme::use_theme_token;

#[component]
fn StyledComponent() -> Element {
    let primary_color = use_theme_token("color-primary");
    let border_radius = use_theme_token("border-radius-base");

    rsx! {
        div {
            style: "background: {primary_color}; border-radius: {border_radius};",
            "使用主题令牌的组件"
        }
    }
}
```

#### 4. 自定义主题创建

支持创建完全自定义的主题配置：

```rust
use ant_design_dioxus::theme::{ThemeConfig, ColorTheme};
use std::collections::HashMap;

let mut custom_colors = HashMap::new();
custom_colors.insert("primary".to_string(), "#ff6b35".to_string());
custom_colors.insert("success".to_string(), "#52c41a".to_string());

let custom_theme = ThemeConfig::create_custom_theme(
    ColorTheme { colors: custom_colors },
    Some("MyCustomTheme".to_string())
);
```

### 设计令牌扩展

新增了更多设计令牌类别：

- **颜色系统**：主色、成功色、警告色、错误色、信息色、文本色、背景色、边框色、填充色
- **尺寸系统**：字体大小、行高、边框圆角
- **间距系统**：内边距、外边距、组件间距
- **阴影系统**：基础阴影、小阴影、大阴影、超大阴影
- **动画系统**：持续时间、缓动函数

## 🌍 国际化系统改进

### 新增功能

#### 1. 扩展的语言包

大幅扩展了内置翻译文本，涵盖更多组件场景：

```rust
// 表单相关
"required" => "此项为必填项"
"invalid_email" => "请输入有效的邮箱地址"
"min_length" => "至少输入 {min} 个字符"

// 日期时间
"today" => "今天"
"select_date" => "选择日期"
"start_date" => "开始日期"

// 表格相关
"select_all" => "全选"
"sort_asc" => "升序"
"filter_menu" => "筛选菜单"

// 上传相关
"upload_drag" => "点击或拖拽文件到此区域上传"
"file_size_limit" => "文件大小不能超过 {size}"

// 步骤条
"step_next" => "下一步"
"step_finish" => "完成"
```

#### 2. 日期时间本地化

新增完整的日期时间本地化支持：

```rust
use ant_design_dioxus::locale::{
    use_datetime_format, use_date_format, use_time_format, use_relative_time_format
};
use chrono::Local;

#[component]
fn DateTimeExample() -> Element {
    let datetime_format = use_datetime_format();
    let date_format = use_date_format();
    let time_format = use_time_format();
    let relative_time_format = use_relative_time_format();

    let now = Local::now();

    rsx! {
        div {
            p { "完整日期时间: {datetime_format(&now)}" }
            p { "日期: {date_format(&now)}" }
            p { "时间: {time_format(&now)}" }
            p { "相对时间: {relative_time_format(&now)}" }
        }
    }
}
```

#### 3. 动态语言切换

支持运行时动态切换语言，自动更新 HTML 文档属性：

```rust
use ant_design_dioxus::locale::{use_locale_switch, Locale};

#[component]
fn LanguageSwitcher() -> Element {
    let locale_switch = use_locale_switch();

    rsx! {
        button {
            onclick: move |_| locale_switch(Locale::ZhCN),
            "中文"
        }
        button {
            onclick: move |_| locale_switch(Locale::En),
            "English"
        }
        button {
            onclick: move |_| locale_switch(Locale::Ar),
            "العربية"
        }
    }
}
```

#### 4. RTL 语言支持

新增对从右到左（RTL）语言的完整支持：

```rust
use ant_design_dioxus::locale::use_is_rtl;

#[component]
fn RTLAwareComponent() -> Element {
    let is_rtl = use_is_rtl();

    rsx! {
        div {
            dir: if is_rtl { "rtl" } else { "ltr" },
            style: if is_rtl { "text-align: right;" } else { "text-align: left;" },
            "支持 RTL 的组件内容"
        }
    }
}
```

#### 5. 数字和货币格式化

新增本地化的数字和货币格式化功能：

```rust
use ant_design_dioxus::locale::{use_number_format, use_currency_format};

#[component]
fn NumberExample() -> Element {
    let number_format = use_number_format();
    let currency_format = use_currency_format();

    rsx! {
        div {
            p { "数字: {number_format(1234.56)}" }
            p { "货币: {currency_format(99.99)}" }
        }
    }
}
```

### 新增 Hooks

#### 国际化相关 Hooks

- `use_locale_switch()` - 动态切换语言
- `use_datetime_format()` - 日期时间格式化
- `use_date_format()` - 日期格式化
- `use_time_format()` - 时间格式化
- `use_relative_time_format()` - 相对时间格式化
- `use_number_format()` - 数字格式化
- `use_currency_format()` - 货币格式化
- `use_is_rtl()` - 检查是否为 RTL 语言
- `use_locale_code()` - 获取当前语言代码
- `use_locale_name()` - 获取当前语言名称

#### 主题相关 Hooks

- `use_theme_switch()` - 动态切换主题
- `use_theme_token()` - 访问主题令牌值

## 🚀 使用示例

### 完整应用示例

```rust
use dioxus::prelude::*;
use ant_design_dioxus::{
    theme::{ThemeProvider, ThemeConfig, use_theme_switch},
    locale::{LocaleProvider, LocaleConfig, Locale, use_locale_switch, use_translate}
};

fn main() {
    dioxus_web::launch(App);
}

#[component]
fn App() -> Element {
    let theme_config = use_signal(|| ThemeConfig::light());
    let locale_config = use_signal(|| LocaleConfig::new(Locale::ZhCN));

    rsx! {
        ThemeProvider {
            config: theme_config,
            LocaleProvider {
                config: locale_config,
                MainContent {}
            }
        }
    }
}

#[component]
fn MainContent() -> Element {
    let theme_switch = use_theme_switch();
    let locale_switch = use_locale_switch();
    let translate = use_translate();

    rsx! {
        div {
            class: "app-container",

            header {
                class: "app-header",
                h1 { "Ant Design Dioxus" }

                div {
                    class: "controls",

                    // 主题切换
                    select {
                        onchange: move |evt| {
                            match evt.value().as_str() {
                                "light" => theme_switch(ThemeConfig::light()),
                                "dark" => theme_switch(ThemeConfig::dark()),
                                "compact" => theme_switch(ThemeConfig::compact()),
                                _ => {}
                            }
                        },
                        option { value: "light", "亮色主题" }
                        option { value: "dark", "暗色主题" }
                        option { value: "compact", "紧凑主题" }
                    }

                    // 语言切换
                    select {
                        onchange: move |evt| {
                            match evt.value().as_str() {
                                "zh-CN" => locale_switch(Locale::ZhCN),
                                "en" => locale_switch(Locale::En),
                                "ar" => locale_switch(Locale::Ar),
                                _ => {}
                            }
                        },
                        option { value: "zh-CN", "中文" }
                        option { value: "en", "English" }
                        option { value: "ar", "العربية" }
                    }
                }
            }

            main {
                class: "app-main",

                button {
                    class: "primary-button",
                    "{translate(\"ok\")}"
                }

                button {
                    class: "secondary-button",
                    "{translate(\"cancel\")}"
                }
            }
        }
    }
}
```

## 📦 依赖更新

为了支持新功能，需要在 `Cargo.toml` 中添加以下依赖：

```toml
[dependencies]
chrono = { version = "0.4", features = ["serde", "wasm-bindgen"] }
wasm-bindgen = "0.2"
js-sys = "0.3"
web-sys = { version = "0.3", features = [
    "console",
    "Document",
    "Element",
    "HtmlElement",
    "Window",
] }
```

## 🔧 配置选项

### 主题配置

```rust
use ant_design_dioxus::theme::ThemeConfig;

// 使用预设主题
let light_theme = ThemeConfig::light();
let dark_theme = ThemeConfig::dark();
let compact_theme = ThemeConfig::compact();

// 自定义主题
let custom_theme = ThemeConfig::new()
    .with_primary_color("#ff6b35")
    .with_success_color("#52c41a")
    .with_border_radius(8)
    .with_font_size_base(14);
```

### 国际化配置

```rust
use ant_design_dioxus::locale::{LocaleConfig, Locale};
use std::collections::HashMap;

// 使用预设语言
let zh_config = LocaleConfig::new(Locale::ZhCN);
let en_config = LocaleConfig::new(Locale::En);

// 自定义翻译
let mut custom_messages = HashMap::new();
custom_messages.insert("welcome".to_string(), "欢迎使用".to_string());

let custom_config = LocaleConfig::new(Locale::ZhCN)
    .with_custom_messages(custom_messages)
    .with_date_format("%Y年%m月%d日".to_string())
    .with_time_format("%H:%M:%S".to_string());
```

## 🎯 最佳实践

### 1. 主题设计

- 使用语义化的颜色命名（primary、success、warning、error）
- 保持设计令牌的一致性
- 考虑无障碍访问（对比度、色盲友好）
- 支持暗色模式

### 2. 国际化

- 使用语义化的翻译键名
- 支持参数化翻译文本
- 考虑 RTL 语言的布局适配
- 提供回退翻译

### 3. 性能优化

- 使用 `use_memo` 缓存计算结果
- 避免频繁的主题切换
- 按需加载语言包
- 使用 CSS 变量减少重渲染

## 🔄 迁移指南

### 从旧版本迁移

1. **主题系统**：
   - 旧的 `ThemeProvider` 保持兼容
   - 新增的 `use_theme_switch` 和 `use_theme_token` 可直接使用
   - CSS 变量名保持不变

2. **国际化系统**：
   - 旧的 `LocaleProvider` 和基础 hooks 保持兼容
   - 新增的日期时间和数字格式化功能可选择性使用
   - 翻译键名向后兼容

### 升级步骤

1. 更新依赖版本
2. 添加新的依赖项（chrono、wasm-bindgen 等）
3. 可选：使用新的 hooks 和功能
4. 可选：扩展自定义翻译文本
5. 测试主题切换和语言切换功能

## 📚 API 参考

详细的 API 文档请参考：
- [主题系统 API](./src/theme/mod.rs)
- [国际化系统 API](./src/locale/mod.rs)
- [示例代码](./examples/theme_i18n_demo.rs)

## 🤝 贡献

欢迎提交 Issue 和 Pull Request 来改进主题和国际化系统！

### 贡献指南

1. Fork 项目
2. 创建功能分支
3. 提交更改
4. 添加测试
5. 提交 Pull Request

### 开发环境

```bash
# 克隆项目
git clone https://github.com/your-username/ant-design-dioxus.git

# 安装依赖
cd ant-design-dioxus
cargo build

# 运行示例
cargo run --example theme_i18n_demo

# 运行测试
cargo test
```

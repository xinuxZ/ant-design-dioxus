# Ant Design Dioxus 组件库复刻策略
1. 时刻牢记项目的目标和愿景： 复刻 ant-design 框架; 组件样式完全使用 ant-design 框架的样式; 组件的使用方法和注意事项完全使用 ant-design 框架的使用方法和注意事项。

## 组件复杂度分析

基于对现有组件结构的分析，我们将 70+ 组件按复杂度分为三个等级：

### Level 1: 基础组件（简单结构）

**特征**：
- 单一文件实现（mod.rs）
- 简单的 Props 结构
- 基础样式系统
- 无子组件或复杂状态管理

**组件列表**：
- [x] **icon** ⭐         # 图标 (最高优先级 - 被大量组件依赖)
- [x] **skeleton**        # 骨架屏 (被数据加载组件依赖)
- [x] **space**           # 间距 (被布局组件依赖)
- [x] **spin**            # 加载中 (被异步组件依赖)
- [x] **flex**            # 弹性布局 (被布局组件依赖)
- [x] **divider**         # 分割线组件
<!-- - [] **back_top**        # 回到顶部 暂时忽略，不实现；后续使用 float button 代替 -->
- [ ] **statistic**       # 统计数值
- [ ] **watermark**       # 水印
- [ ] **image**           # 图片
- [ ] **result**          # 结果页
- [x] **qr_code**         # 二维码

**推荐结构**：
```
component_name/
├── mod.rs              # 主要组件导出和公共API
├── component.rs        # 核心组件实现
├── types.rs           # 类型定义（Props、枚举等）
├── styles.rs          # CSS-in-Rust 样式实现
├── tests.rs           # 单元测试
└── TODO.md            # 功能差异分析
```

### Level 2: 复杂组件（中等结构）

**特征**：
- 包含子组件或子模块
- 复杂的状态管理
- 多种变体和配置
- 事件处理和交互逻辑

**组件列表**：
- [ ] **button** ⭐       # 按钮（含 ButtonGroup）(高优先级 - 被大量组件依赖) - 依赖: icon, spin
- [ ] **tooltip** ⭐      # 文字提示 (高优先级 - 被大量组件依赖) - 依赖: 无
- [ ] **input** ⭐        # 输入框（需要 TextArea、Search、Password 等子组件）(高优先级 - 被表单组件依赖) - 依赖: icon
- [ ] **table**          # 表格 - 依赖: checkbox, radio, button, pagination, spin
- [ ] **menu**           # 导航菜单 - 依赖: icon, tooltip
- [ ] **tabs**           # 标签页 - 依赖: icon
- [ ] **modal**          # 对话框 - 依赖: button, icon
- [ ] **drawer**         # 抽屉 - 依赖: button, icon
- [ ] **dropdown**       # 下拉菜单 - 依赖: button, popover
- [ ] **popover**        # 气泡卡片 - 依赖: tooltip
- [ ] **date_picker**    # 日期选择器 - 依赖: input, dropdown, button
- [ ] **time_picker**    # 时间选择器 - 依赖: input, dropdown
- [ ] **calendar**       # 日历 - 依赖: button, select, date_picker
- [ ] **tree**           # 树形控件 - 依赖: checkbox, icon
- [ ] **tree_select**    # 树选择 - 依赖: tree, select
- [ ] **cascader**       # 级联选择 - 依赖: input, dropdown, tag
- [ ] **auto_complete**  # 自动完成 - 依赖: input, dropdown
- [ ] **upload**         # 上传 - 依赖: button, progress, icon
- [ ] **steps**          # 步骤条 - 依赖: icon
- [ ] **breadcrumb**     # 面包屑 - 依赖: icon
- [ ] **pagination**     # 分页 - 依赖: button, select, input
- [ ] **list**           # 列表 - 依赖: spin, pagination, avatar
- [ ] **card**           # 卡片 - 依赖: 无
- [ ] **collapse**       # 折叠面板 - 依赖: icon
- [ ] **carousel**       # 走马灯 - 依赖: button, icon
- [ ] **anchor**         # 锚点 - 依赖: 无
- [ ] **affix**          # 固钉 - 依赖: 无
- [ ] **badge**          # 徽标数 - 依赖: 无
- [ ] **avatar**         # 头像 - 依赖: icon, image
- [ ] **tag**            # 标签 - 依赖: icon
- [ ] **alert**          # 警告提示 (已实现) - 依赖: icon, button
- [ ] **progress**       # 进度条 - 依赖: 无
- [ ] **rate**           # 评分 - 依赖: icon
- [ ] **slider**         # 滑动输入条 - 依赖: tooltip
- [ ] **switch**         # 开关 - 依赖: 无
- [ ] **checkbox**       # 多选框 - 依赖: 无
- [ ] **radio**          # 单选框 - 依赖: 无
- [ ] **input_number**   # 数字输入框 - 依赖: input, button
- [ ] **select**         # 选择器 - 依赖: input, dropdown, tag
- [ ] **mentions**       # 提及 - 依赖: input, dropdown
- [ ] **color_picker**   # 颜色选择器 - 依赖: input, dropdown
- [ ] **segmented**      # 分段控制器 - 依赖: 无
- [ ] **splitter**       # 分割面板 - 依赖: 无
- [ ] **transfer**       # 穿梭框 - 依赖: checkbox, button, input
- [ ] **timeline**       # 时间轴 - 依赖: icon
- [ ] **descriptions**   # 描述列表 - 依赖: 无
- [ ] **float_button**   # 悬浮按钮 - 依赖: button, tooltip
- [ ] **tour**           # 漫游式引导 - 依赖: button, popover

**推荐结构**：
```
component_name/
├── mod.rs              # 主要组件导出
├── components/         # 子组件目录
│   ├── main_component.rs
│   ├── sub_component.rs
│   └── mod.rs
├── types/              # 类型定义目录
│   ├── props.rs
│   ├── enums.rs
│   └── mod.rs
├── styles/             # 样式模块目录
│   ├── base.rs
│   ├── variants.rs
│   └── mod.rs
├── hooks/              # 自定义 hooks 目录（如果需要）
├── utils/              # 工具函数目录
├── tests/              # 测试目录
└── TODO.md
```

### Level 3: 模块化组件（复杂结构）

**特征**：
- 多个功能模块
- 复杂的业务逻辑
- 多层嵌套结构
- 高度可配置

**组件列表**：
- [ ] **layout**         # 布局（含 Header、Sider、Content、Footer）- 依赖: 无 (但被其他组件依赖)
- [ ] **grid**           # 栅格（含 Row、Col）- 依赖: 无 (但被其他组件依赖)
- [ ] **form** ⭐         # 表单（含 FormItem、FormList 等）(核心组件) - 依赖: input, select, checkbox, radio, button, date_picker, upload, etc.
- [ ] **message**        # 全局提示 - 依赖: icon, button
- [ ] **notification**   # 通知提醒框 - 依赖: icon, button
- [ ] **popconfirm**     # 气泡确认框 - 依赖: popover, button
- [ ] **app**            # 应用包裹组件 - 依赖: config_provider, message, notification
- [ ] **config_provider**# 全局配置 - 依赖: theme, locale
- [ ] **typography**     # 排版（含 Title、Text、Paragraph）- 依赖: tooltip, button
- [ ] **empty**          # 空状态 - 依赖: image, button

**推荐结构**：
```
component_name/
├── mod.rs
├── core/               # 核心功能模块
│   ├── components/
│   ├── types/
│   ├── hooks/
│   └── utils/
├── features/           # 功能特性模块
│   ├── validation/     # 表单验证
│   ├── layout/         # 布局管理
│   └── interaction/    # 交互逻辑
├── styles/
├── tests/
└── TODO.md
```


## 任务计划
### 1. 编码规范

**文件命名规范**：
```
# 组件文件
component.rs      # 主组件实现
types.rs         # 类型定义
styles.rs        # 样式实现
hooks.rs         # 自定义 hooks
utils.rs         # 工具函数
tests.rs         # 单元测试

# 子组件目录
components/
├── main.rs      # 主要子组件
├── item.rs      # 项目子组件
├── group.rs     # 组合子组件
└── mod.rs       # 模块导出
```

**Props 结构规范**：
```rust
#[derive(Props, Clone, PartialEq)]
pub struct ComponentProps {
    // 1. 核心功能属性
    #[props(default)]
    pub variant: ComponentVariant,

    #[props(default)]
    pub size: ComponentSize,

    // 2. 状态属性
    #[props(default = false)]
    pub disabled: bool,

    #[props(default = false)]
    pub loading: bool,

    // 3. 事件处理
    pub on_click: Option<EventHandler<MouseEvent>>,

    // 4. 样式定制
    pub class: Option<String>,
    pub style: Option<String>,

    // 5. 子元素
    pub children: Element,
}
```

**样式生成器规范**：
```rust
pub struct ComponentStyleGenerator {
    variant: ComponentVariant,
    size: ComponentSize,
    disabled: bool,
    // ... 其他状态
}

impl ComponentStyleGenerator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn variant(mut self, variant: ComponentVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn generate(&self) -> String {
        // 生成样式逻辑
    }
}
```


**CSS-in-Rust 集成方案**
基于 `css-in-rust` 库的特性，采用以下样式管理策略：

```rust
// styles.rs 示例结构
use css_in_rust::{css, css_if, theme};

// 基础样式定义
pub struct ComponentStyles {
    pub base: String,
    pub variants: VariantStyles,
    pub states: StateStyles,
}

// 变体样式
pub struct VariantStyles {
    pub size_small: String,
    pub size_medium: String,
    pub size_large: String,
    pub type_primary: String,
    pub type_secondary: String,
}

// 状态样式
pub struct StateStyles {
    pub hover: String,
    pub active: String,
    pub disabled: String,
    pub loading: String,
}

// 样式生成器
pub fn generate_component_styles() -> ComponentStyles {
    ComponentStyles {
        base: css!(r#"
            display: inline-flex;
            align-items: center;
            justify-content: center;
            border: none;
            cursor: pointer;
            transition: all 0.2s;
        "#),
        variants: VariantStyles {
            size_small: css!("padding: 4px 8px; font-size: 12px;"),
            size_medium: css!("padding: 8px 16px; font-size: 14px;"),
            size_large: css!("padding: 12px 24px; font-size: 16px;"),
            // ...
        },
        // ...
    }
}
```

#### 2. 技术规范

##### 代码质量标准

1. **类型安全**
   - 所有公共 API 必须有明确的类型定义
   - 使用 `#[derive(Props, Clone, PartialEq)]` 标准化 Props

3. **性能优化**
   - 使用 `memo` 优化不必要的重渲染
   - 实现懒加载和虚拟滚动
   - 优化样式计算和应用

3. **可访问性**
   - 遵循 WCAG 2.1 AA 标准
   - 提供适当的 ARIA 属性
   - 支持键盘导航

##### 样式规范

1. **主题系统**
   - 支持亮色/暗色主题
   - 提供主题定制接口
   - 实现主题切换动画

2. **响应式设计**
   - 支持移动端适配
   - 实现断点系统
   - 优化触摸交互

3. **动画效果**
   - 使用 CSS 动画而非 JavaScript
   - 提供动画开关选项
   - 遵循 Material Design 动画原则

## 实施计划
### 1. 计划执行步骤 [步骤](./COMPONENT_TASK_STEP)
### 2. 工具链和自动化支持

#### 2.1 开发工具链

##### 代码生成工具
```bash
# 组件结构生成器
./tools/structure-generator --component Button --level 2 --with-tests

# 样式生成器
./tools/style-generator --component Button --theme default

# API 文档生成器
./tools/doc-generator --component Button --output docs/
```

##### 质量检查工具
```bash
# 代码质量检查
cargo clippy --all-targets --all-features

# 格式化检查
cargo fmt --check

# 测试覆盖率
cargo tarpaulin --out Html --output-dir coverage/

# 性能基准测试
cargo bench --bench component_benchmarks
```

##### 视觉回归测试工具
```bash
# 截图对比测试
./tools/visual-regression --component Button --browsers chrome,firefox,safari

# 响应式测试
./tools/responsive-test --component Button --viewports mobile,tablet,desktop
```

#### 2.2 CI/CD 自动化流程

##### 持续集成检查清单
- [ ] **代码质量**：Clippy 检查通过
- [ ] **格式化**：代码格式符合规范
- [ ] **类型检查**：所有类型检查通过
- [ ] **单元测试**：所有单元测试通过
- [ ] **集成测试**：所有集成测试通过
- [ ] **性能测试**：性能基准测试通过
- [ ] **视觉回归**：视觉对比测试通过
- [ ] **可访问性**：可访问性测试通过
- [ ] **文档生成**：API 文档生成成功

##### 自动化部署流程
```yaml
# .github/workflows/component-release.yml
name: Component Release
on:
  push:
    paths:
      - 'src/components/**'
      - 'tests/**'

jobs:
  quality-check:
    runs-on: ubuntu-latest
    steps:
      - name: Quality Check
        run: |
          cargo clippy --all-targets
          cargo test --all
          cargo bench

  visual-regression:
    runs-on: ubuntu-latest
    steps:
      - name: Visual Regression Test
        run: |
          ./tools/visual-regression --all-components

  documentation:
    runs-on: ubuntu-latest
    steps:
      - name: Generate Documentation
        run: |
          cargo doc --no-deps
          ./tools/doc-generator --all-components
```

### 3. 质量度量和监控体系

#### 3.1 质量度量指标

##### 功能完整性指标
- **API 覆盖率**：实现的 API 数量 / Ant Design 官方 API 总数
- **功能覆盖率**：实现的功能数量 / 官方功能总数
- **测试覆盖率**：测试代码覆盖率 ≥ 90%
- **文档覆盖率**：有文档的 API 数量 / 总 API 数量

##### 质量指标
- **视觉一致性**：与官方组件的视觉相似度 ≥ 95%
- **性能指标**：渲染时间、内存使用、包大小
- **可访问性得分**：WCAG 2.1 AA 标准合规性
- **跨浏览器兼容性**：主要浏览器支持率

##### 用户体验指标
- **API 易用性**：开发者使用反馈
- **学习曲线**：从 React Ant Design 迁移的难易程度
- **错误率**：运行时错误发生频率
- **社区活跃度**：GitHub stars, issues, PRs

#### 3.2 监控和报告系统

##### 自动化质量报告
```rust
// tools/quality-reporter/src/main.rs
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct QualityReport {
    pub component_name: String,
    pub api_coverage: f64,
    pub test_coverage: f64,
    pub visual_similarity: f64,
    pub performance_score: f64,
    pub accessibility_score: f64,
    pub issues: Vec<QualityIssue>,
}

#[derive(Serialize, Deserialize)]
pub struct QualityIssue {
    pub severity: IssueSeverity,
    pub category: IssueCategory,
    pub description: String,
    pub file_path: String,
    pub line_number: Option<u32>,
}

pub fn generate_quality_report(component: &str) -> QualityReport {
    // 生成质量报告逻辑
}
```

##### 质量仪表板
- **实时质量监控**：显示所有组件的质量状态
- **趋势分析**：质量指标的历史趋势
- **问题追踪**：自动发现和跟踪质量问题
- **性能监控**：组件性能的实时监控

### 4. 国际化和主题系统完善

#### 4.1 国际化系统增强

##### 多语言支持策略
```rust
// src/locale/mod.rs
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct LocaleConfig {
    pub locale: String,
    pub messages: HashMap<String, String>,
    pub date_format: String,
    pub number_format: NumberFormat,
    pub rtl: bool,
}

#[derive(Clone, Debug)]
pub struct NumberFormat {
    pub decimal_separator: String,
    pub thousands_separator: String,
    pub currency_symbol: String,
}

pub trait LocaleProvider {
    fn get_message(&self, key: &str) -> Option<&str>;
    fn format_date(&self, date: &str) -> String;
    fn format_number(&self, number: f64) -> String;
    fn is_rtl(&self) -> bool;
}
```

##### 本地化最佳实践
- **文本外部化**：所有用户可见文本都通过国际化系统
- **日期时间格式**：支持不同地区的日期时间格式
- **数字格式**：支持不同地区的数字和货币格式
- **RTL 支持**：支持从右到左的语言
- **复数形式**：支持复数形式的正确处理

#### 4.2 主题系统深度定制

##### 设计令牌系统
```rust
// src/theme/tokens.rs
use css_in_rust::theme_var;

#[derive(Clone, Debug)]
pub struct DesignTokens {
    // 颜色系统
    pub colors: ColorTokens,
    // 尺寸系统
    pub sizes: SizeTokens,
    // 字体系统
    pub typography: TypographyTokens,
    // 阴影系统
    pub shadows: ShadowTokens,
    // 动画系统
    pub motion: MotionTokens,
}

#[derive(Clone, Debug)]
pub struct ColorTokens {
    pub primary: ColorPalette,
    pub success: ColorPalette,
    pub warning: ColorPalette,
    pub error: ColorPalette,
    pub neutral: ColorPalette,
}

#[derive(Clone, Debug)]
pub struct ColorPalette {
    pub color_50: String,
    pub color_100: String,
    pub color_200: String,
    pub color_300: String,
    pub color_400: String,
    pub color_500: String,
    pub color_600: String,
    pub color_700: String,
    pub color_800: String,
    pub color_900: String,
}

pub fn generate_theme_css(tokens: &DesignTokens) -> String {
    css!(r#"
        :root {
            --color-primary: ${tokens.colors.primary.color_500};
            --color-primary-hover: ${tokens.colors.primary.color_600};
            --color-primary-active: ${tokens.colors.primary.color_700};
            /* 更多主题变量 */
        }

        [data-theme="dark"] {
            --color-primary: ${tokens.colors.primary.color_400};
            --color-primary-hover: ${tokens.colors.primary.color_300};
            --color-primary-active: ${tokens.colors.primary.color_200};
            /* 暗色主题变量 */
        }
    "#)
}
```

##### 动态主题切换
```rust
// src/theme/provider.rs
use dioxus::prelude::*;

#[derive(Clone, Copy)]
pub struct ThemeContext {
    pub current_theme: Signal<ThemeMode>,
    pub custom_tokens: Signal<Option<DesignTokens>>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum ThemeMode {
    Light,
    Dark,
    Auto, // 跟随系统
    Custom(String),
}

#[component]
pub fn ThemeProvider(children: Element) -> Element {
    let theme = use_signal(|| ThemeMode::Light);
    let tokens = use_signal(|| None::<DesignTokens>);

    // 监听系统主题变化
    use_effect(move || {
        if theme() == ThemeMode::Auto {
            // 监听系统主题变化逻辑
        }
    });

    use_context_provider(|| ThemeContext {
        current_theme: theme,
        custom_tokens: tokens,
    });

    rsx! {
        div {
            "data-theme": match theme() {
                ThemeMode::Light => "light",
                ThemeMode::Dark => "dark",
                ThemeMode::Auto => "auto",
                ThemeMode::Custom(name) => &name,
            },
            {children}
        }
    }
}
```

### 5. 性能优化深度策略

#### 5.1 渲染性能优化

##### 虚拟化和懒加载
```rust
// src/utils/virtualization.rs
use dioxus::prelude::*;

#[component]
pub fn VirtualList<T: Clone + PartialEq + 'static>(
    items: Vec<T>,
    item_height: f64,
    container_height: f64,
    render_item: fn(T, usize) -> Element,
) -> Element {
    let scroll_top = use_signal(|| 0.0);

    let visible_range = use_memo(move || {
        let start = (scroll_top() / item_height).floor() as usize;
        let visible_count = (container_height / item_height).ceil() as usize + 1;
        let end = (start + visible_count).min(items.len());
        start..end
    });

    rsx! {
        div {
            style: "height: {container_height}px; overflow-y: auto;",
            onscroll: move |evt| {
                scroll_top.set(evt.data.scroll_top());
            },
            div {
                style: "height: {items.len() as f64 * item_height}px; position: relative;",
                for (index, item) in items.iter().enumerate() {
                    if visible_range().contains(&index) {
                        div {
                            key: "{index}",
                            style: "position: absolute; top: {index as f64 * item_height}px; width: 100%;",
                            {render_item(item.clone(), index)}
                        }
                    }
                }
            }
        }
    }
}
```

##### 样式优化策略
```rust
// src/utils/style_optimization.rs
use css_in_rust::{css, css_cache};
use std::collections::HashMap;

// 样式缓存系统
static STYLE_CACHE: once_cell::sync::Lazy<std::sync::Mutex<HashMap<String, String>>> =
    once_cell::sync::Lazy::new(|| std::sync::Mutex::new(HashMap::new()));

pub fn cached_css(key: &str, generator: impl FnOnce() -> String) -> String {
    let mut cache = STYLE_CACHE.lock().unwrap();

    if let Some(cached) = cache.get(key) {
        return cached.clone();
    }

    let styles = generator();
    cache.insert(key.to_string(), styles.clone());
    styles
}

// 条件样式优化
pub fn conditional_styles(conditions: &[(bool, &str)]) -> String {
    conditions
        .iter()
        .filter(|(condition, _)| *condition)
        .map(|(_, style)| *style)
        .collect::<Vec<_>>()
        .join(" ")
}
```

#### 5.2 包大小优化

##### Tree Shaking 优化
```rust
// src/lib.rs
// 使用特性门控制组件包含
#[cfg(feature = "button")]
pub mod button;

#[cfg(feature = "input")]
pub mod input;

#[cfg(feature = "table")]
pub mod table;

// 预设组合
#[cfg(feature = "basic")]
pub mod basic {
    pub use crate::button::*;
    pub use crate::input::*;
    pub use crate::icon::*;
}

#[cfg(feature = "form")]
pub mod form {
    pub use crate::basic::*;
    pub use crate::form::*;
    pub use crate::select::*;
    pub use crate::date_picker::*;
}

#[cfg(feature = "full")]
pub mod full {
    pub use crate::*;
}
```

##### 动态导入支持
```rust
// src/utils/dynamic_import.rs
use dioxus::prelude::*;
use std::future::Future;

#[component]
pub fn LazyComponent<F, Fut>(
    loader: F,
    fallback: Element,
) -> Element
where
    F: Fn() -> Fut + 'static,
    Fut: Future<Output = Element> + 'static,
{
    let component = use_signal(|| None::<Element>);
    let loading = use_signal(|| true);

    use_effect(move || {
        spawn(async move {
            let loaded_component = loader().await;
            component.set(Some(loaded_component));
            loading.set(false);
        });
    });

    if loading() {
        fallback
    } else {
        component().unwrap_or(fallback)
    }
}
```

### 6. 社区和生态系统建设

#### 6.1 开发者体验优化

##### 开发工具支持
- **IDE 插件**：VSCode/IntelliJ 插件支持
- **代码片段**：常用组件的代码片段
- **类型提示**：完整的 TypeScript 类型定义
- **调试工具**：组件状态调试工具

##### 文档和教程
- **交互式文档**：可运行的代码示例
- **迁移指南**：从 React Ant Design 迁移
- **最佳实践**：组件使用的最佳实践
- **视频教程**：组件使用的视频教程

#### 6.2 社区贡献指南

##### 贡献流程
1. **问题报告**：使用标准化的问题模板
2. **功能请求**：使用功能请求模板
3. **代码贡献**：遵循代码贡献指南
4. **文档贡献**：改进文档和示例
5. **测试贡献**：添加测试用例

##### 质量标准
- **代码质量**：通过所有质量检查
- **测试覆盖**：新功能必须有测试
- **文档完整**：新功能必须有文档
- **向后兼容**：保持 API 向后兼容

### 7. 版本管理和发布策略

#### 7.1 语义化版本控制

- **主版本号**：不兼容的 API 修改
- **次版本号**：向下兼容的功能性新增
- **修订号**：向下兼容的问题修正

#### 7.2 发布流程

1. **预发布测试**：在测试环境进行全面测试
2. **变更日志**：生成详细的变更日志
3. **文档更新**：更新相关文档
4. **社区通知**：通知社区即将发布的变更
5. **正式发布**：发布到包管理器
6. **发布后监控**：监控发布后的问题

这个完善的策略文档为 ant-design-dioxus 项目提供了全面的指导，确保能够高质量地复刻 Ant Design 组件库，同时建立完善的开发、测试、发布和维护流程。

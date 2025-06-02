# CSS-in-Rust 方案实施规划

## 项目概述

本文档详细规划了将 `css-in-rust` 方案集成到 `ant-design-dioxus` 项目中的完整实施计划。

### 当前状态分析

1. **ant-design-dioxus 项目现状**：
   - 已完成大部分组件的基础实现
   - 使用传统的外部 CSS 文件方案（每个组件包含 `style.css`）
   - 拥有完整的主题系统（`src/theme/mod.rs`）
   - 支持多种主题模式（Light、Dark、Compact、Custom）

2. **css-in-rust 项目现状**：
   - 独立的工作区项目，包含核心库和宏库
   - 支持高性能的 CSS-in-Rust 解决方案
   - 基于 lightningcss 实现
   - 提供完整的主题系统和样式生成能力

### 集成目标

1. **功能目标**：
   - 将所有组件样式从外部 CSS 迁移到 CSS-in-Rust
   - 保持与 Ant Design 5.25.3 的完全视觉一致性
   - 支持动态主题切换和自定义主题
   - 提升样式的类型安全性和开发体验

2. **性能目标**：
   - 构建时间增加不超过 20%
   - 运行时性能无明显下降
   - 支持样式的按需加载和缓存

## 实施阶段规划

### 第一阶段：基础设施集成（1-2 天）

#### 任务 1.1：配置项目依赖
- **目标**：在 ant-design-dioxus 项目中添加 css-in-rust 依赖
- **具体操作**：
  - 修改 `Cargo.toml`，添加路径依赖
  - 配置工作区依赖关系
  - 验证依赖解析正确性

#### 任务 1.2：创建样式系统桥接模块
- **目标**：建立 ant-design-dioxus 主题系统与 css-in-rust 的桥接
- **具体操作**：
  - 在 `src/theme/` 下创建 `css_bridge.rs`
  - 实现主题数据转换逻辑
  - 建立 Design Token 映射关系

#### 任务 1.3：建立 CSS-in-Rust 基础架构
- **目标**：创建统一的样式生成和管理基础设施
- **具体操作**：
  - 创建 `src/utils/styles.rs` 模块
  - 实现样式缓存和优化机制
  - 建立样式调试和开发工具

### 第二阶段：试点组件迁移（2-3 天）

#### 任务 2.1：Button 组件迁移
- **目标**：将 Button 组件作为试点，完全迁移到 CSS-in-Rust
- **具体操作**：
  - 分析现有 `button/style.css` 的所有样式规则
  - 使用 css-in-rust 宏重新实现所有样式
  - 保持 API 兼容性，确保功能无损
  - 实现动态样式和主题支持

#### 任务 2.2：建立迁移模板和最佳实践
- **目标**：基于 Button 组件迁移经验，建立标准化流程
- **具体操作**：
  - 创建组件迁移模板
  - 编写迁移指南和最佳实践文档
  - 建立样式测试和验证流程

#### 任务 2.3：性能和功能验证
- **目标**：验证 CSS-in-Rust 方案的性能和功能正确性
- **具体操作**：
  - 进行构建性能基准测试
  - 验证运行时性能表现
  - 测试主题切换功能
  - 进行视觉回归测试

### 第三阶段：批量组件迁移（5-7 天）

#### 任务 3.1：基础组件迁移
- **目标**：迁移基础 UI 组件
- **组件列表**：Typography, Icon, Grid, Layout, Space, Divider
- **优先级**：高（这些组件被其他组件广泛依赖）

#### 任务 3.2：表单组件迁移
- **目标**：迁移表单相关组件
- **组件列表**：Input, Select, Checkbox, Radio, Switch, Form
- **优先级**：高（核心交互组件）

#### 任务 3.3：反馈组件迁移
- **目标**：迁移用户反馈组件
- **组件列表**：Alert, Message, Notification, Modal, Drawer
- **优先级**：中（重要但相对独立）

#### 任务 3.4：导航组件迁移
- **目标**：迁移导航相关组件
- **组件列表**：Menu, Breadcrumb, Pagination, Steps
- **优先级**：中

#### 任务 3.5：数据展示组件迁移
- **目标**：迁移数据展示组件
- **组件列表**：Table, List, Card, Collapse, Tabs
- **优先级**：中

#### 任务 3.6：其他组件迁移
- **目标**：迁移剩余组件
- **组件列表**：其他所有组件
- **优先级**：低

### 第四阶段：优化和完善（2-3 天）

#### 任务 4.1：性能优化
- **目标**：优化整体性能表现
- **具体操作**：
  - 样式缓存优化
  - 构建时间优化
  - 包大小优化

#### 任务 4.2：开发体验优化
- **目标**：提升开发者使用体验
- **具体操作**：
  - 完善类型定义
  - 优化错误提示
  - 改进调试工具

#### 任务 4.3：文档和测试完善
- **目标**：完善项目文档和测试覆盖
- **具体操作**：
  - 更新组件文档
  - 完善单元测试
  - 添加集成测试

## 技术实现要点

### 依赖配置

```toml
# 在 ant-design-dioxus/Cargo.toml 中添加
[dependencies]
css-in-rust = { path = "../css-in-rust/css-in-rust", features = ["macros", "runtime", "theme"] }
css-in-rust-macros = { path = "../css-in-rust/css-in-rust-macros" }
```

### 样式迁移模式

**原有方式**：
```rust
// 外部 CSS + 类名
const BUTTON_STYLE: &str = include_str!("style.css");

rsx! {
    button {
        class: "ant-btn ant-btn-primary",
        // ...
    }
}
```

**CSS-in-Rust 方式**：
```rust
use css_in_rust::css;

#[component]
fn Button(props: ButtonProps) -> Element {
    let theme = use_theme();

    let button_styles = css! {
        position: relative;
        display: inline-block;
        font-weight: 400;
        background: ${theme.colors.primary.base};
        color: ${theme.colors.text.primary};
        border-radius: ${theme.border_radius.base};
        // ...
    };

    rsx! {
        button {
            class: "{button_styles}",
            // ...
        }
    }
}
```

### 主题系统集成

```rust
// src/theme/css_bridge.rs
use css_in_rust::theme::ThemeProvider;
use crate::theme::{Theme, ColorTheme};

pub fn create_css_theme_provider(theme: &Theme) -> ThemeProvider {
    // 将 ant-design-dioxus 主题转换为 css-in-rust 主题
    // ...
}
```

## 风险评估和应对策略

### 主要风险

1. **性能风险**：CSS-in-Rust 可能增加构建时间和运行时开销
   - **应对策略**：建立性能基准，持续监控，必要时进行优化

2. **兼容性风险**：新旧样式系统可能存在不兼容
   - **应对策略**：采用渐进式迁移，保持向后兼容

3. **开发体验风险**：开发者需要学习新的样式编写方式
   - **应对策略**：提供详细文档和迁移指南

### 质量保证措施

1. **视觉回归测试**：确保迁移后的组件视觉效果与原版一致
2. **性能基准测试**：监控构建时间和运行时性能变化
3. **功能测试**：确保所有组件功能正常
4. **主题测试**：验证主题切换功能正常工作

## 成功标准

1. **功能标准**：
   - 所有组件完全使用 CSS-in-Rust
   - 功能和样式与原版完全一致
   - 主题切换正常工作

2. **性能标准**：
   - 构建时间增加不超过 20%
   - 运行时性能无明显下降
   - 包大小增加控制在合理范围内

3. **开发体验标准**：
   - 样式编写更加类型安全
   - 支持更好的 IDE 支持
   - 调试和开发工具完善

## 下一步行动

立即开始执行 **任务 1.1：配置项目依赖**，这是整个实施计划的基础。

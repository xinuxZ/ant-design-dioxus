# Ant Design Dioxus 主题系统重构方案

## 1. 重构核心思想

### 1.1 职责分离原则

- **css-in-rust**: 提供通用的 CSS-in-Rust 基础设施
  - 通用主题管理器 (`ThemeManager`)
  - CSS 变量生成和注入 (`CssGenerator`, `CssVariableManager`)
  - 设计令牌系统基础架构 (`TokenSystem`, `TokenResolver`)
  - 主题提供者基础设施 (`ThemeProvider`)

- **ant-design-dioxus**: 专注于 Ant Design 组件实现
  - 使用 css-in-rust 提供的基础设施
  - 定义 Ant Design 特定的设计规范
  - 实现 Dioxus 框架集成
  - 提供组件级别的主题 API

### 1.2 避免重复建设

不再重复实现以下已在 css-in-rust 中提供的功能：
- 主题管理器和历史记录
- CSS 变量生成和注入逻辑
- 设计令牌解析系统
- 主题提供者基础架构

## 2. 现状分析

### 2.1 当前重复问题

#### 与 css-in-rust 重复的功能
- `src/theme/hooks.rs` 中的 `ThemeProvider` 重复了 css-in-rust 的主题提供者
- `src/theme/css_bridge.rs` 重复了 CSS 变量生成逻辑
- 主题管理逻辑与 css-in-rust 的 `ThemeManager` 重复

#### 内部重复问题
- `src/theme/tokens/` 与 `src/utils/` 中的颜色、动画、尺寸定义重复
- 多个文件中存在相似的类型定义

### 2.2 架构问题

- 缺乏对 css-in-rust 基础设施的有效利用
- 主题系统与组件系统耦合过紧
- 缺乏清晰的 Ant Design 设计规范抽象

### 🎯 **重构方案**

#### **阶段一：核心抽象层重构**
```
src/theme/
├── core/                    # 核心抽象层
│   ├── mod.rs              # 导出核心类型
│   ├── types.rs            # 基础类型定义 (Size, ColorType)
│   ├── color.rs            # 颜色核心抽象 (RgbColor, ColorPalette)
│   ├── motion.rs           # 动画核心抽象 (Duration, Easing)
│   └── config.rs           # 主题配置抽象
├── tokens/                  # 设计令牌层
│   ├── colors.rs           # Ant Design 颜色预设
│   ├── animations.rs       # Ant Design 动画预设
│   ├── sizes.rs            # Ant Design 尺寸预设
│   └── presets.rs          # 完整主题预设
├── providers/               # 主题提供者层
│   ├── context.rs          # 主题上下文
│   ├── provider.rs         # ThemeProvider 组件
│   └── hooks.rs            # 主题相关 Hooks
├── bridge/                  # 外部集成层
│   └── css_bridge.rs       # CSS-in-Rust 桥接
└── mod.rs                   # 统一导出
```

#### **阶段二：依赖关系优化**
```
core (基础抽象)
  ↓
tokens (设计令牌)
  ↓
providers (主题提供)
  ↓
bridge (外部集成)
  ↓
components (组件层)
```

#### **阶段三：utils 目录重构**
```
src/utils/
├── mod.rs
├── class_names.rs          # 保留：CSS 类名工具
├── responsive.rs           # 保留：响应式工具
└── dom.rs                  # 新增：DOM 操作工具
```

### 📋 **具体实施步骤**

#### **Step 1: 创建核心抽象层**
1. 创建 `theme/core/` 目录
2. 将 `utils/color.rs` 重构为 `theme/core/color.rs`
3. 将 `utils/motion.rs` 重构为 `theme/core/motion.rs`
4. 统一 `Size` 类型定义到 `theme/core/types.rs`

#### **Step 2: 重构设计令牌层**
1. 合并 `theme/tokens/color_presets.rs` 和颜色相关逻辑
2. 统一 `theme/tokens/animation_presets.rs` 和动画配置
3. 创建统一的 `theme/tokens/presets.rs`

#### **Step 3: 分离主题提供者**
1. 将 `theme/hooks.rs` 中的 `ThemeProvider` 移至 `theme/providers/provider.rs`
2. 将主题 Hooks 移至 `theme/providers/hooks.rs`
3. 创建独立的 `theme/providers/context.rs`

#### **Step 4: 清理重复代码**
1. 删除 `utils/color.rs`、`utils/motion.rs`、`utils/size.rs`
2. 更新所有导入路径
3. 统一 API 接口

### 🚀 **预期收益**

1. **代码复用率提升 60%**：消除重复定义
2. **维护成本降低**：单一数据源，统一修改点
3. **扩展性增强**：清晰的分层架构支持新功能
4. **类型安全**：统一的类型系统避免转换错误
5. **开发体验优化**：清晰的模块边界和职责划分

### ⚠️ **迁移注意事项**

1. **渐进式迁移**：先创建新架构，再逐步迁移现有代码
2. **向后兼容**：保持公共 API 稳定，内部重构
3. **测试覆盖**：确保重构过程中功能不受影响
4. **文档更新**：同步更新使用文档和示例代码

这个重构方案将显著提升代码质量，建立清晰的架构边界，为后续的组件开发奠定坚实基础。

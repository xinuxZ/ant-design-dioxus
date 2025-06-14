# Space 组件嵌套功能文档

## 概述

Space 组件支持嵌套使用，特别是 `Space.Compact` 组件可以实现复杂的紧凑布局。这个功能参考了 Ant Design 官方的 Space 组件设计，支持多层嵌套和混合使用。

## 功能特性

### 1. Space.Compact 嵌套

`Space.Compact` 组件支持多层嵌套，可以创建复杂的紧凑布局：

```rust
use dioxus::prelude::*;
use ant_design_dioxus::{Space, SpaceCompact};

#[component]
fn NestedCompactExample() -> Element {
    rsx! {
        SpaceCompact {
            block: true,
            
            // 第一层嵌套
            SpaceCompact {
                Input { placeholder: "输入框1" }
                Button { "按钮1" }
            }
            
            // 分隔元素
            Button {
                r#type: "primary",
                "分隔符"
            }
            
            // 第二层嵌套
            SpaceCompact {
                Input { placeholder: "输入框2" }
                Button { "按钮2" }
            }
        }
    }
}
```

### 2. 多层嵌套

支持任意层级的嵌套，每一层都会自动应用适当的样式：

```rust
#[component]
fn MultiLevelNesting() -> Element {
    rsx! {
        SpaceCompact {
            SpaceCompact {
                SpaceCompact {
                    Input { style: "width: 90px;", placeholder: "深层输入" }
                    Button { "🔍" }
                }
                InputNumber { default_value: 12 }
            }
            Button { r#type: "primary", "提交" }
        }
    }
}
```

### 3. 混合嵌套

`Space` 和 `SpaceCompact` 可以混合使用：

```rust
#[component]
fn MixedNesting() -> Element {
    rsx! {
        Space {
            direction: SpaceDirection::Vertical,
            
            SpaceCompact {
                Button { "紧凑按钮1" }
                Button { "紧凑按钮2" }
            }
            
            div { "普通元素" }
            
            SpaceCompact {
                Input { placeholder: "紧凑输入" }
                Select { "选择器" }
            }
        }
    }
}
```

## 样式系统

### 嵌套样式处理

嵌套的 `Space.Compact` 组件会自动应用以下样式特性：

1. **层级管理**：每个嵌套层级都有独立的 z-index
2. **边框圆角**：根据嵌套层级调整圆角大小
3. **位置定位**：使用相对定位确保正确的层叠顺序

### CSS 类名规则

嵌套组件会生成以下类名：

- `ant-space-compact`：基础类名
- `ant-space-compact-horizontal/vertical`：方向类名
- `ant-space-compact-small/middle/large`：尺寸类名
- `ant-space-compact-nested-{level}`：嵌套层级类名

## API 参考

### 新增工具函数

#### `get_nested_space_compact_container_class`

生成嵌套 Space.Compact 容器的 CSS 类名。

```rust
pub fn get_nested_space_compact_container_class(
    direction: SpaceDirection,
    size: CompactSize,
    nesting_level: u32,
    theme: &SpaceTheme,
) -> String
```

#### `detect_space_compact_nesting_level`

检测当前 Space.Compact 组件的嵌套层级。

```rust
pub fn detect_space_compact_nesting_level() -> u32
```

#### `validate_space_compact_nesting`

验证 Space.Compact 嵌套配置是否合法。

```rust
pub fn validate_space_compact_nesting(
    nesting_level: u32,
    max_nesting_level: u32,
) -> Result<(), String>
```

### 新增样式函数

#### `generate_nested_space_compact_styles`

生成嵌套 Space.Compact 容器的样式。

```rust
pub fn generate_nested_space_compact_styles(
    direction: SpaceDirection,
    nesting_level: u32,
    theme: &SpaceTheme,
) -> String
```

## 使用注意事项

### 1. 性能考虑

- 避免过深的嵌套层级（建议不超过 5 层）
- 大量嵌套可能影响渲染性能

### 2. 样式冲突

- 嵌套组件的样式会自动处理，避免手动覆盖
- 如需自定义样式，建议使用主题系统

### 3. 响应式设计

- 嵌套组件在不同屏幕尺寸下可能需要调整
- 建议测试移动端的显示效果

## 示例项目

完整的嵌套使用示例可以在以下文件中找到：

- `examples/space_nested.rs`：完整的嵌套使用示例
- `src/components/space/tests/nested_tests.rs`：嵌套功能测试

## 与 Ant Design 的对比

本实现参考了 Ant Design 官方的 Space 组件设计：

- ✅ 支持 `Space.Compact` 嵌套
- ✅ 多层嵌套样式处理
- ✅ 混合使用 Space 和 SpaceCompact
- ✅ 自动边框和圆角处理
- ✅ 响应式布局支持

与原版的主要区别：

- 使用 Rust/Dioxus 实现而非 React/TypeScript
- 样式系统基于 CSS-in-Rust
- 类型安全的属性定义

## 未来改进

1. **上下文支持**：通过 Context API 自动检测嵌套层级
2. **动画效果**：添加嵌套组件的过渡动画
3. **主题增强**：更丰富的嵌套主题配置选项
4. **性能优化**：优化深层嵌套的渲染性能
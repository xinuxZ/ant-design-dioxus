# Space 间距组件

设置组件之间的间距，避免组件紧贴在一起。

## 目录结构

```
space/
├── mod.rs              # 模块入口文件
├── component.rs        # 组件实现
├── types.rs           # 类型定义
├── styles.rs          # 样式生成器
├── migrated_styles.rs # 迁移后的CSS-in-Rust样式
├── tests.rs           # 单元测试
├── examples.rs        # 使用示例
└── README.md          # 文档说明
```

## 重构改进

### 1. 结构优化
- **模块化设计**: 将组件拆分为独立的模块，提高代码可维护性
- **类型分离**: 将类型定义独立到 `types.rs` 文件
- **样式分离**: 采用 CSS-in-Rust 方案，移除外部 `style.css` 文件

### 2. 样式系统
- **CSS-in-Rust**: 使用 `css_in_rust` 宏实现样式定义
- **按需加载**: 根据组件属性动态注册相关样式
- **变体支持**: 支持不同方向、尺寸、对齐方式的样式变体

### 3. 代码质量
- **完整文档**: 提供详细的文档注释和使用示例
- **单元测试**: 包含全面的测试用例
- **类型安全**: 使用强类型定义确保API安全

## 何时使用

- 避免组件紧贴在一起，拉开统一的空间
- 在某组件的某个方向上，保持统一的间距
- 支持水平、垂直方向
- 支持自动换行和对齐方式

## API

### SpaceProps

| 属性 | 说明 | 类型 | 默认值 |
|------|------|------|--------|
| children | 子组件 | `Element` | - |
| direction | 间距方向 | `SpaceDirection` | `Horizontal` |
| size | 间距大小 | `SpaceSize` | `Small` |
| align | 对齐方式 | `SpaceAlign` | `Start` |
| wrap | 是否自动换行 | `bool` | `false` |
| split | 分隔符 | `Option<Element>` | - |
| prefix_cls | 自定义CSS类名前缀 | `Option<String>` | - |
| class | 自定义CSS类名 | `Option<String>` | - |
| style | 自定义样式 | `Option<String>` | - |

### SpaceDirection

- `Horizontal`: 水平间距
- `Vertical`: 垂直间距

### SpaceSize

- `Small`: 小间距 (8px)
- `Middle`: 中间距 (16px)
- `Large`: 大间距 (24px)
- `Custom(u32)`: 自定义间距

### SpaceAlign

- `Start`: 起始对齐
- `End`: 结束对齐
- `Center`: 居中对齐
- `Baseline`: 基线对齐

## 示例

### 基础用法

```rust
use dioxus::prelude::*;
use ant_design_dioxus::{Space, Button};

fn app() -> Element {
    rsx! {
        Space {
            Button { "按钮1" }
            Button { "按钮2" }
            Button { "按钮3" }
        }
    }
}
```

### 垂直间距

```rust
use dioxus::prelude::*;
use ant_design_dioxus::{Space, SpaceDirection, Button};

fn app() -> Element {
    rsx! {
        Space {
            direction: SpaceDirection::Vertical,
            Button { "按钮1" }
            Button { "按钮2" }
            Button { "按钮3" }
        }
    }
}
```

### 自定义间距大小

```rust
use dioxus::prelude::*;
use ant_design_dioxus::{Space, SpaceSize, Button};

fn app() -> Element {
    rsx! {
        Space {
            size: SpaceSize::Large,
            Button { "按钮1" }
            Button { "按钮2" }
            Button { "按钮3" }
        }
    }
}
```

### 对齐方式

```rust
use dioxus::prelude::*;
use ant_design_dioxus::{Space, SpaceAlign, Button};

fn app() -> Element {
    rsx! {
        Space {
            align: SpaceAlign::Center,
            Button { "按钮1" }
            Button { "按钮2" }
            Button { "按钮3" }
        }
    }
}
```

### 自动换行

```rust
use dioxus::prelude::*;
use ant_design_dioxus::{Space, Button};

fn app() -> Element {
    rsx! {
        Space {
            wrap: true,
            Button { "按钮1" }
            Button { "按钮2" }
            Button { "按钮3" }
            Button { "按钮4" }
            Button { "按钮5" }
        }
    }
}
```

### 分隔符

```rust
use dioxus::prelude::*;
use ant_design_dioxus::{Space, Button, Divider};

fn app() -> Element {
    rsx! {
        Space {
            split: rsx! { Divider { r#type: DividerType::Vertical } },
            Button { "按钮1" }
            Button { "按钮2" }
            Button { "按钮3" }
        }
    }
}
```

## 注意事项

1. **性能优化**: 组件使用 CSS-in-Rust 实现，样式按需加载
2. **响应式**: 支持响应式设计，在不同屏幕尺寸下自适应
3. **无障碍**: 遵循无障碍设计规范，支持键盘导航和屏幕阅读器
4. **主题**: 支持主题定制，可通过 ConfigProvider 全局配置
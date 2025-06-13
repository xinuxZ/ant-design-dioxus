# Divider 分割线组件

区隔内容的分割线。

## 目录结构

```
divider/
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
- **变体支持**: 支持不同类型、尺寸的样式变体

### 3. 代码质量
- **完整文档**: 提供详细的文档注释和使用示例
- **单元测试**: 包含全面的测试用例
- **类型安全**: 使用强类型定义确保API安全

## 何时使用

- 对不同章节的文本段落进行分割
- 对行内文字/链接进行分割，例如表格的操作列

## API

### DividerProps

| 属性 | 说明 | 类型 | 默认值 |
|------|------|------|--------|
| children | 分割线中的文字 | `Option<Element>` | - |
| type | 分割线类型 | `DividerType` | `Horizontal` |
| size | 分割线尺寸 | `DividerSize` | `Default` |
| dashed | 是否为虚线 | `bool` | `false` |
| plain | 是否为简洁模式 | `bool` | `false` |
| orientation | 分割线文字的位置 | `DividerOrientation` | `Center` |
| class | 自定义样式类名 | `Option<String>` | - |
| style | 自定义样式 | `Option<String>` | - |

### DividerType

- `Horizontal`: 水平分割线
- `Vertical`: 垂直分割线

### DividerSize

- `Small`: 小号分割线
- `Default`: 默认尺寸
- `Large`: 大号分割线

### DividerOrientation

- `Left`: 文字居左
- `Center`: 文字居中
- `Right`: 文字居右

## 基础用法

```rust
use dioxus::prelude::*;
use ant_design_dioxus::Divider;

fn app() -> Element {
    rsx! {
        div {
            p { "Lorem ipsum dolor sit amet, consectetur adipiscing elit." }
            Divider {}
            p { "Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua." }
        }
    }
}
```

## 带文字的分割线

```rust
Divider {
    "Text"
}

Divider {
    orientation: DividerOrientation::Left,
    "Left Text"
}

Divider {
    orientation: DividerOrientation::Right,
    "Right Text"
}
```

## 垂直分割线

```rust
div {
    style: "display: flex; align-items: center;",
    "Text"
    Divider {
        r#type: DividerType::Vertical
    }
    a { href: "#", "Link" }
    Divider {
        r#type: DividerType::Vertical
    }
    a { href: "#", "Link" }
}
```

## 虚线

```rust
Divider {
    dashed: true
}
```

## 不同尺寸

```rust
Divider {
    size: DividerSize::Small,
    "Small Size"
}

Divider {
    "Default Size"
}

Divider {
    size: DividerSize::Large,
    "Large Size"
}
```

## 简洁模式

```rust
Divider {
    plain: true,
    "Text"
}
```

## 性能优化

1. **按需样式注册**: 只注册当前组件实际使用的样式
2. **CSS-in-Rust**: 编译时优化，减少运行时开销
3. **模块化加载**: 支持按需导入，减少包体积

## 迁移指南

### 从旧版本迁移

1. **导入路径**: 保持不变，仍然从 `ant_design_dioxus` 导入
2. **API兼容**: 所有现有API保持向后兼容
3. **样式**: 自动使用新的CSS-in-Rust样式系统

### 注意事项

- 垂直分割线不支持文字内容
- 自定义样式会覆盖默认样式
- 建议使用枚举类型而不是字符串来设置属性

## 测试

运行测试:

```bash
cargo test divider
```

查看示例:

```bash
cargo run --example divider
```
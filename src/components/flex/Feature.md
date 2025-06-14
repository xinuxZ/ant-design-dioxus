# Flex 组件功能清单

## 组件概述

Flex 组件是 Ant Design 5.10.0+ 版本引入的现代布局容器，基于 CSS Flexbox 提供强大的对齐和分布能力。与 Space 组件不同，Flex 用于设置块级元素的布局，不添加包装元素，适用于垂直或水平方向的子元素布局，提供更多的灵活性和控制。

## 核心功能清单

### 基础布局功能
- [ ] 水平布局 (默认)
- [ ] 垂直布局 (vertical)
- [ ] 换行控制 (wrap)
- [ ] 间距设置 (gap)

### 对齐控制
- [ ] 主轴对齐 (justify)
  - [ ] flex-start
  - [ ] center
  - [ ] flex-end
  - [ ] space-between
  - [ ] space-around
  - [ ] space-evenly
- [ ] 交叉轴对齐 (align)
  - [ ] flex-start
  - [ ] center
  - [ ] flex-end
  - [ ] stretch
  - [ ] baseline

### Flex 属性
- [ ] flex 简写属性支持
- [ ] flex-grow 控制
- [ ] flex-shrink 控制
- [ ] flex-basis 控制

### 间距系统
- [ ] 预设间距 (small, middle, large)
- [ ] 自定义间距 (数字/字符串)
- [ ] 双轴间距 [horizontal, vertical]
- [ ] 响应式间距

### 样式系统
- [ ] 主题支持
- [ ] CSS-in-Rust 实现
- [ ] 响应式设计
- [ ] 自定义类名和样式

### 可访问性
- [ ] 语义化 HTML 结构
- [ ] ARIA 标签支持
- [ ] 键盘导航支持

## 实现完成情况

### 技术特性
- **类型安全**: 使用 Rust 类型系统确保 Props 类型安全
- **性能优化**: 基于 CSS Flexbox 的高性能布局
- **主题支持**: 完整的主题定制和暗色模式支持
- **响应式**: 支持不同屏幕尺寸的自适应布局
- **可访问性**: 遵循 WCAG 2.1 标准的可访问性实现
- **测试覆盖**: 全面的单元测试和集成测试

### 文件结构
```
src/components/flex/
├── mod.rs           # 模块入口和文档
├── types.rs         # 类型定义和 Props
├── styles.rs        # 样式生成函数
├── utils.rs         # 工具函数
├── component.rs     # 主要组件实现
├── tests.rs         # 测试用例
└── Feature.md       # 功能清单 (本文件)
```

### API 设计

#### FlexProps
```rust
#[derive(Props, Clone, PartialEq)]
pub struct FlexProps {
    /// 是否垂直布局
    #[props(default = false)]
    pub vertical: bool,
    
    /// 换行方式
    #[props(default = FlexWrap::NoWrap)]
    pub wrap: FlexWrap,
    
    /// 主轴对齐方式
    #[props(default = FlexJustify::Normal)]
    pub justify: FlexJustify,
    
    /// 交叉轴对齐方式
    #[props(default = FlexAlign::Normal)]
    pub align: FlexAlign,
    
    /// flex 简写属性
    #[props(optional)]
    pub flex: Option<String>,
    
    /// 间距设置
    #[props(optional)]
    pub gap: Option<FlexGap>,
    
    /// 自定义类名
    #[props(optional)]
    pub class: Option<String>,
    
    /// 自定义样式
    #[props(optional)]
    pub style: Option<String>,
    
    /// 自定义组件类型
    #[props(default = "div")]
    pub component: &'static str,
    
    /// 子元素
    #[props(optional)]
    pub children: Option<Element>,
}
```

#### 枚举类型
```rust
#[derive(Clone, PartialEq, Debug)]
pub enum FlexWrap {
    NoWrap,
    Wrap,
    WrapReverse,
}

#[derive(Clone, PartialEq, Debug)]
pub enum FlexJustify {
    Normal,
    FlexStart,
    Center,
    FlexEnd,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

#[derive(Clone, PartialEq, Debug)]
pub enum FlexAlign {
    Normal,
    FlexStart,
    Center,
    FlexEnd,
    Stretch,
    Baseline,
}

#[derive(Clone, PartialEq, Debug)]
pub enum FlexGap {
    Small,
    Middle,
    Large,
    Custom(String),
    Array([String; 2]), // [horizontal, vertical]
}
```

## 使用示例

### 基础用法
```rust
use dioxus::prelude::*;
use ant_design_dioxus::Flex;

fn app() -> Element {
    rsx! {
        // 水平布局
        Flex {
            gap: FlexGap::Middle,
            div { "Item 1" }
            div { "Item 2" }
            div { "Item 3" }
        }
        
        // 垂直布局
        Flex {
            vertical: true,
            gap: FlexGap::Large,
            div { "Item A" }
            div { "Item B" }
            div { "Item C" }
        }
    }
}
```

### 对齐控制
```rust
fn alignment_demo() -> Element {
    rsx! {
        // 居中对齐
        Flex {
            justify: FlexJustify::Center,
            align: FlexAlign::Center,
            style: "height: 200px; border: 1px solid #ddd;",
            div { "Centered Content" }
        }
        
        // 两端对齐
        Flex {
            justify: FlexJustify::SpaceBetween,
            align: FlexAlign::Center,
            div { "Left" }
            div { "Center" }
            div { "Right" }
        }
    }
}
```

### 响应式布局
```rust
fn responsive_demo() -> Element {
    rsx! {
        Flex {
            wrap: FlexWrap::Wrap,
            gap: FlexGap::Array(["16px".to_string(), "24px".to_string()]),
            
            for i in 1..=6 {
                div {
                    key: "{i}",
                    style: "flex: 1 1 200px; padding: 16px; background: #f5f5f5;",
                    "Item {i}"
                }
            }
        }
    }
}
```

### 嵌套布局
```rust
fn nested_demo() -> Element {
    rsx! {
        Flex {
            vertical: true,
            gap: FlexGap::Large,
            style: "height: 100vh;",
            
            // Header
            Flex {
                justify: FlexJustify::SpaceBetween,
                align: FlexAlign::Center,
                style: "height: 64px; background: #001529; color: white; padding: 0 24px;",
                
                div { "Logo" }
                Flex {
                    gap: FlexGap::Middle,
                    button { "Notifications" }
                    button { "Profile" }
                }
            }
            
            // Main Content
            Flex {
                flex: "1",
                gap: FlexGap::Middle,
                
                // Sidebar
                div {
                    style: "width: 200px; background: #f0f2f5; padding: 16px;",
                    "Sidebar"
                }
                
                // Content
                div {
                    style: "flex: 1; background: white; padding: 24px;",
                    "Main Content"
                }
            }
        }
    }
}
```

## 实施状态

### 基础布局 ✅
- [x] 水平布局（默认）
- [x] 垂直布局
- [x] 布局方向切换
- [x] 基础容器样式

### 对齐控制 ✅
- [x] 主轴对齐（justify-content）
  - [x] flex-start（默认）
  - [x] flex-end
  - [x] center
  - [x] space-between
  - [x] space-around
  - [x] space-evenly
- [x] 交叉轴对齐（align-items）
  - [x] flex-start（默认）
  - [x] flex-end
  - [x] center
  - [x] baseline
  - [x] stretch

### Flex 属性 ✅
- [x] flex 简写属性
- [x] flex-grow 控制
- [x] flex-shrink 控制
- [x] flex-basis 控制
- [x] 自动计算和优化

### 间距系统 ✅
- [x] 预设间距（small, middle, large）
- [x] 自定义间距
- [x] 响应式间距
- [x] 主题化间距

### 样式系统 ✅
- [x] CSS-in-Rust 实现
- [x] 主题支持
- [x] 暗色模式
- [x] 紧凑模式
- [x] 自定义样式覆盖
- [x] 响应式样式

### 可访问性 ✅
- [x] 语义化 HTML
- [x] ARIA 属性支持
- [x] 键盘导航
- [x] 屏幕阅读器支持

- [x] 功能分析完成
- [x] API 设计完成
- [x] 类型定义完成
- [x] 样式系统完成
- [x] 组件实现完成
- [x] 测试用例完成
- [x] 文档编写完成

## 技术难点

1. **CSS Flexbox 映射**: 将 CSS Flexbox 属性正确映射到 Rust 类型系统
2. **间距系统**: 实现灵活的间距配置，支持预设值和自定义值
3. **响应式支持**: 提供响应式间距和布局配置
4. **性能优化**: 确保大量子元素时的渲染性能
5. **类型安全**: 保证所有 Flex 属性的类型安全

## 依赖关系

- **基础依赖**: dioxus, css-in-rust
- **样式依赖**: 主题系统
- **工具依赖**: 响应式工具函数
- **测试依赖**: dioxus-testing

## 参考资料

- [Ant Design Flex 官方文档](https://ant.design/components/flex/)
- [CSS Flexbox 规范](https://www.w3.org/TR/css-flexbox-1/)
- [MDN Flexbox 指南](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Flexible_Box_Layout)
- [Ant Design 设计规范](https://ant.design/docs/spec/introduce)
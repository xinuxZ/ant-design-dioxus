# Divider 分割线组件

## 功能清单

### 基础分割线功能 ✅

#### 官方实现分析
- **核心逻辑**：提供水平和垂直分割线，用于分隔不同内容区域 <mcreference link="https://ant.design/components/divider/?locale=en-US" index="1">1</mcreference>
- **关键技术点**：支持实线、虚线、点线三种样式变体 <mcreference link="https://ant.design/components/divider/?locale=en-US" index="1">1</mcreference>
- **API 设计**：简洁的属性接口，支持类型、方向、样式等配置 <mcreference link="https://github.com/ant-design/ant-design/blob/master/components/divider/index.tsx" index="4">4</mcreference>

#### Rust + Dioxus 实现方案
- **类型设计**：`DividerProps` 结构体包含所有必要属性
- **状态管理**：无状态组件，纯展示型
- **事件处理**：无需事件处理
- **样式实现**：CSS-in-Rust 实现所有样式变体

### 文本分割线功能 ✅

#### 官方实现分析
- **核心逻辑**：支持在分割线中嵌入文本内容 <mcreference link="https://www.geeksforgeeks.org/reactjs-ui-ant-design-divider-component/" index="2">2</mcreference>
- **关键技术点**：文本位置可配置（左、中、右、start、end） <mcreference link="https://github.com/ant-design/ant-design/blob/master/components/divider/index.tsx" index="4">4</mcreference>
- **API 设计**：`orientation` 属性控制文本位置，`orientationMargin` 控制边距

#### Rust + Dioxus 实现方案
- **类型设计**：`DividerOrientation` 枚举定义文本位置
- **状态管理**：根据 children 判断是否有文本
- **事件处理**：无需事件处理
- **样式实现**：动态生成带文本的分割线样式

### 样式变体功能 ✅

#### 官方实现分析
- **核心逻辑**：支持 solid（实线）、dashed（虚线）、dotted（点线）三种样式 <mcreference link="https://ant.design/components/divider/?locale=en-US" index="1">1</mcreference>
- **关键技术点**：`variant` 属性控制线条样式，`dashed` 属性向后兼容 <mcreference link="https://github.com/ant-design/ant-design/blob/master/components/divider/index.tsx" index="4">4</mcreference>
- **API 设计**：默认为 solid，可通过属性切换

#### Rust + Dioxus 实现方案
- **类型设计**：`DividerVariant` 枚举定义样式类型
- **状态管理**：无状态，纯样式控制
- **事件处理**：无需事件处理
- **样式实现**：CSS border-style 属性控制

### 方向控制功能 ✅

#### 官方实现分析
- **核心逻辑**：支持水平（horizontal）和垂直（vertical）两种方向 <mcreference link="https://ant.design/components/divider/?locale=en-US" index="1">1</mcreference>
- **关键技术点**：垂直分割线常用于行内元素分隔 <mcreference link="https://ant.design/components/divider/?locale=en-US" index="1">1</mcreference>
- **API 设计**：`type` 属性控制方向，默认为 horizontal

#### Rust + Dioxus 实现方案
- **类型设计**：`DividerType` 枚举定义方向
- **状态管理**：无状态组件
- **事件处理**：无需事件处理
- **样式实现**：不同方向使用不同的 CSS 样式

### 文本样式功能 ✅

#### 官方实现分析
- **核心逻辑**：支持普通文本和朴素文本两种样式 <mcreference link="https://www.geeksforgeeks.org/reactjs-ui-ant-design-divider-component/" index="2">2</mcreference>
- **关键技术点**：`plain` 属性控制文本是否使用朴素样式 <mcreference link="https://4x.ant.design/components/divider/" index="5">5</mcreference>
- **API 设计**：默认为标题样式，plain=true 时为普通文本样式

#### Rust + Dioxus 实现方案
- **类型设计**：`plain` 布尔属性控制文本样式
- **状态管理**：无状态组件
- **事件处理**：无需事件处理
- **样式实现**：条件样式应用

### 尺寸控制功能 ✅

#### 官方实现分析
- **核心逻辑**：支持不同尺寸的间距控制 <mcreference link="https://ant.design/components/divider/?locale=en-US" index="1">1</mcreference>
- **关键技术点**：`size` 属性控制分割线的间距大小
- **API 设计**：继承全局尺寸配置

#### Rust + Dioxus 实现方案
- **类型设计**：`DividerSize` 枚举定义尺寸
- **状态管理**：无状态组件
- **事件处理**：无需事件处理
- **样式实现**：不同尺寸对应不同的 margin 值

## 技术特性

### 核心特性
- **轻量级**：纯展示组件，无复杂逻辑
- **灵活性**：支持多种样式和配置
- **语义化**：使用合适的 HTML 元素（hr、div）
- **可访问性**：提供适当的 ARIA 属性

### 性能特性
- **零状态**：无状态管理开销
- **CSS 优化**：最小化样式计算
- **渲染优化**：简单 DOM 结构

## 文件结构

```
divider/
├── mod.rs              # 模块导出和公共API
├── component.rs        # 核心组件实现
├── types.rs           # 类型定义（Props、枚举等）
├── styles.rs          # CSS-in-Rust 样式实现
├── tests.rs           # 单元测试
└── Feature.md         # 功能差异分析（本文件）
```

## API 设计

### DividerProps 结构体
```rust
#[derive(Props, Clone, PartialEq)]
pub struct DividerProps {
    // 基础属性
    #[props(default = DividerType::Horizontal)]
    pub r#type: DividerType,
    
    #[props(default = DividerOrientation::Center)]
    pub orientation: DividerOrientation,
    
    #[props(default)]
    pub orientation_margin: Option<String>,
    
    // 样式属性
    #[props(default = DividerVariant::Solid)]
    pub variant: DividerVariant,
    
    #[props(default = false)]
    pub dashed: bool,  // 向后兼容
    
    #[props(default = false)]
    pub plain: bool,
    
    // 尺寸属性
    #[props(default)]
    pub size: Option<DividerSize>,
    
    // 样式定制
    #[props(default)]
    pub class: Option<String>,
    
    #[props(default)]
    pub style: Option<String>,
    
    // 子元素
    #[props(default)]
    pub children: Option<Element>,
}
```

### 枚举类型
```rust
#[derive(Clone, PartialEq, Debug)]
pub enum DividerType {
    Horizontal,
    Vertical,
}

#[derive(Clone, PartialEq, Debug)]
pub enum DividerOrientation {
    Left,
    Right,
    Center,
    Start,  // 5.24.0+
    End,    // 5.24.0+
}

#[derive(Clone, PartialEq, Debug)]
pub enum DividerVariant {
    Solid,
    Dashed,
    Dotted,
}

#[derive(Clone, PartialEq, Debug)]
pub enum DividerSize {
    Small,
    Middle,
    Large,
}
```

## 使用示例

### 基础用法
```rust
use dioxus::prelude::*;
use ant_design_dioxus::Divider;

#[component]
fn App() -> Element {
    rsx! {
        div {
            p { "Lorem ipsum dolor sit amet..." }
            Divider {}
            p { "Lorem ipsum dolor sit amet..." }
            Divider { dashed: true }
            p { "Lorem ipsum dolor sit amet..." }
        }
    }
}
```

### 带文本分割线
```rust
Divider { "Text" }
Divider { orientation: DividerOrientation::Left, "Left Text" }
Divider { orientation: DividerOrientation::Right, "Right Text" }
```

### 垂直分割线
```rust
span {
    "Text"
    Divider { r#type: DividerType::Vertical }
    a { href: "#", "Link" }
    Divider { r#type: DividerType::Vertical }
    a { href: "#", "Link" }
}
```

### 样式变体
```rust
Divider { variant: DividerVariant::Solid, "Solid" }
Divider { variant: DividerVariant::Dashed, "Dashed" }
Divider { variant: DividerVariant::Dotted, "Dotted" }
```

### 朴素文本
```rust
Divider { plain: true, "Plain Text" }
Divider { orientation: DividerOrientation::Left, plain: true, "Left Plain Text" }
```

## 实施状态

### 基础功能 ✅
- [x] 水平分割线
- [x] 垂直分割线
- [x] 基础样式系统

### 文本功能 ✅
- [x] 文本嵌入
- [x] 文本位置控制（left, center, right, start, end）
- [x] 文本边距控制
- [x] 朴素文本样式

### 样式变体 ✅
- [x] 实线样式（默认）
- [x] 虚线样式
- [x] 点线样式
- [x] 向后兼容的 dashed 属性

### 尺寸控制 ✅
- [x] 小尺寸
- [x] 中等尺寸
- [x] 大尺寸
- [x] 自定义间距

### 样式系统 ✅
- [x] CSS-in-Rust 实现
- [x] 主题支持
- [x] 暗色模式
- [x] 自定义样式覆盖

### 可访问性 ✅
- [x] 语义化 HTML（hr/div）
- [x] ARIA 属性支持
- [x] 屏幕阅读器支持

- [x] 功能分析完成
- [x] API 设计完成
- [x] 类型定义完成
- [x] 样式系统完成
- [x] 组件实现完成
- [x] 测试用例完成
- [x] 文档编写完成

## 技术难点

### 1. 文本位置计算
- **问题**：需要精确控制文本在分割线中的位置
- **解决方案**：使用 flexbox 布局和伪元素实现

### 2. 垂直分割线适配
- **问题**：垂直分割线在不同容器中的高度适配
- **解决方案**：使用 CSS 的 align-self 和 height 属性

### 3. 样式变体实现
- **问题**：不同线条样式的 CSS 实现
- **解决方案**：使用 border-style 属性和条件样式

### 4. 响应式适配
- **问题**：在不同屏幕尺寸下的显示效果
- **解决方案**：使用相对单位和媒体查询

## 依赖关系

### 内部依赖
- **无**：Divider 是基础组件，不依赖其他组件

### 外部依赖
- `dioxus`: 核心框架
- `css-in-rust`: 样式系统

### 被依赖情况
- **Layout 组件**：用于页面布局分隔
- **List 组件**：用于列表项分隔
- **Menu 组件**：用于菜单项分隔
- **Card 组件**：用于内容区域分隔

## 参考资料

- [Ant Design Divider 官方文档](https://ant.design/components/divider/)
- [Ant Design Divider 源码](https://github.com/ant-design/ant-design/blob/master/components/divider/index.tsx)
- [React UI Ant Design Divider Component](https://www.geeksforgeeks.org/reactjs-ui-ant-design-divider-component/)
- [CSS-in-Rust 文档](https://github.com/lukidoescode/css-in-rust)
- [Dioxus 官方文档](https://dioxuslabs.com/)
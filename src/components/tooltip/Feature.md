# Tooltip - 功能实现清单

## 组件概览
### 官方组件信息
- **组件类型**：基础组件
- **依赖组件**：无
- **官方文档链接**：https://ant.design/components/tooltip/
- **设计规范**：Ant Design 设计系统

### 复刻目标
- **功能完整度目标**：100% API 兼容
- **样式还原度目标**：95%+ 视觉一致性
- **性能目标**：渲染时间 < 16ms，内存使用 < 1MB
- **可访问性目标**：WCAG 2.1 AA 标准

## 核心功能分析
### 1. 基础提示显示功能
#### 官方实现深度分析
- **功能描述**：在鼠标悬停、聚焦或点击时显示简单文本提示框
- **核心逻辑**：基于 rc-trigger 组件实现，支持多种触发方式
- **关键技术点**：
  - 位置计算和自动调整
  - 延迟显示/隐藏机制
  - 箭头指向目标元素
  - 边界检测和自适应定位
- **API 设计**：
  - `title`: 提示文本内容 (ReactNode | () => ReactNode)
  - `placement`: 位置 (top|left|right|bottom|topLeft|topRight|bottomLeft|bottomRight|leftTop|leftBottom|rightTop|rightBottom)
  - `trigger`: 触发方式 (hover|focus|click|contextMenu)
  - `open`: 受控显示状态
  - `defaultOpen`: 默认显示状态
- **边界条件**：
  - title 为空时不显示
  - 屏幕边缘自动调整位置
  - 滚动容器内的定位处理
- **性能考量**：
  - 内容缓存机制
  - 延迟渲染
  - DOM 事件优化

#### Rust + Dioxus 实现方案
- **类型设计**：
```rust
#[derive(Props, Clone, PartialEq)]
pub struct TooltipProps {
    /// 提示文本内容
    pub title: Option<String>,
    /// 位置
    #[props(default = TooltipPlacement::Top)]
    pub placement: TooltipPlacement,
    /// 触发方式
    #[props(default = TooltipTrigger::Hover)]
    pub trigger: TooltipTrigger,
    /// 受控显示状态
    pub open: Option<bool>,
    /// 默认显示状态
    #[props(default = false)]
    pub default_open: bool,
    /// 鼠标进入延迟（秒）
    #[props(default = 0.1)]
    pub mouse_enter_delay: f64,
    /// 鼠标离开延迟（秒）
    #[props(default = 0.1)]
    pub mouse_leave_delay: f64,
    /// 是否显示箭头
    #[props(default = true)]
    pub arrow: bool,
    /// 背景颜色
    pub color: Option<String>,
    /// 自动调整溢出
    #[props(default = true)]
    pub auto_adjust_overflow: bool,
    /// z-index
    pub z_index: Option<i32>,
    /// 显示状态变化回调
    pub on_open_change: Option<EventHandler<bool>>,
    /// 子元素
    pub children: Element,
    /// CSS 类名
    pub class: Option<String>,
    /// 内联样式
    pub style: Option<String>,
}

#[derive(Clone, PartialEq)]
pub enum TooltipPlacement {
    Top,
    Left,
    Right,
    Bottom,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    LeftTop,
    LeftBottom,
    RightTop,
    RightBottom,
}

#[derive(Clone, PartialEq)]
pub enum TooltipTrigger {
    Hover,
    Focus,
    Click,
    ContextMenu,
}
```
- **状态管理**：使用 use_signal 管理显示状态、位置信息、延迟定时器
- **事件处理**：鼠标事件、键盘事件、焦点事件的统一处理
- **样式实现**：CSS-in-Rust 实现动态样式生成
- **生命周期管理**：use_effect 处理延迟显示/隐藏逻辑
- **错误处理**：边界条件检查和降级处理

#### 技术难点与解决方案
- **难点1**：精确的位置计算和自动调整
  - **解决方案**：使用 getBoundingClientRect 获取元素位置，实现智能位置调整算法
  - **备选方案**：使用 CSS transform 和 position 属性进行定位
- **难点2**：延迟显示/隐藏的定时器管理
  - **解决方案**：使用 use_signal 管理定时器状态，确保正确清理
- **难点3**：事件冒泡和阻止默认行为
  - **解决方案**：合理使用 preventDefault 和 stopPropagation

#### 实现计划
1. [x] **类型定义**
   - [x] Props 结构设计
   - [x] 枚举类型定义
   - [x] 事件类型定义
2. [ ] **核心逻辑实现**
   - [ ] 显示/隐藏状态管理
   - [ ] 位置计算逻辑
   - [ ] 延迟机制实现
3. [ ] **样式集成**
   - [ ] 基础样式
   - [ ] 箭头样式
   - [ ] 位置变体样式
   - [ ] 主题适配
4. [ ] **事件处理**
   - [ ] 鼠标悬停事件
   - [ ] 焦点事件
   - [ ] 点击事件
   - [ ] 键盘导航
5. [ ] **测试编写**
   - [ ] 单元测试
   - [ ] 集成测试
   - [ ] 视觉回归测试
6. [ ] **文档更新**
   - [ ] API 文档
   - [ ] 使用示例
   - [ ] 迁移指南

### 2. 高级定位功能
#### 官方实现深度分析
- **功能描述**：12种位置选项，自动边界检测和位置调整
- **核心逻辑**：基于 @rc-component/trigger 的新对齐算法
- **关键技术点**：
  - 视口边界检测
  - 自动翻转和偏移
  - 缩放适配
  - 滚动容器处理

#### Rust + Dioxus 实现方案
- **位置计算算法**：实现智能位置调整逻辑
- **边界检测**：检测视口边界并自动调整
- **响应式适配**：支持不同屏幕尺寸

### 3. 样式和主题功能
#### 官方实现深度分析
- **功能描述**：预设颜色样式，自定义背景色，主题适配
- **核心逻辑**：CSS 变量和主题令牌系统
- **关键技术点**：
  - 设计令牌集成
  - 动态样式生成
  - 暗色模式支持

## API 兼容性检查清单
### Props 属性对比
| Ant Design 属性 | 类型 | 默认值 | 实现状态 | Dioxus 对应属性 | 备注 |
| --------------- | ---- | ------ | -------- | --------------- | ---- |
| title | ReactNode \| () => ReactNode | - | [x] | title | 提示文本 |
| placement | string | "top" | [x] | placement | 位置 |
| trigger | string \| string[] | "hover" | [x] | trigger | 触发方式 |
| open | boolean | false | [x] | open | 受控显示 |
| defaultOpen | boolean | false | [x] | default_open | 默认显示 |
| mouseEnterDelay | number | 0.1 | [x] | mouse_enter_delay | 进入延迟 |
| mouseLeaveDelay | number | 0.1 | [x] | mouse_leave_delay | 离开延迟 |
| arrow | boolean \| object | true | [x] | arrow | 箭头显示 |
| color | string | - | [x] | color | 背景颜色 |
| autoAdjustOverflow | boolean | true | [x] | auto_adjust_overflow | 自动调整 |
| zIndex | number | - | [x] | z_index | 层级 |
| onOpenChange | function | - | [x] | on_open_change | 状态变化回调 |

### 事件对比
| Ant Design 事件 | 参数 | 实现状态 | Dioxus 对应 | 备注 |
| --------------- | ---- | -------- | ----------- | ---- |
| onOpenChange | (open: boolean) => void | [x] | on_open_change | 显示状态变化 |

## 样式实现清单
### 基础样式
- [ ] **默认样式**：基础外观和布局
- [ ] **箭头样式**：指向目标的三角箭头
- [ ] **位置变体**：12种位置的样式适配
- [ ] **状态样式**：显示/隐藏动画效果

### 主题适配
- [ ] **亮色主题**：默认主题样式
- [ ] **暗色主题**：暗色模式适配
- [ ] **自定义主题**：主题变量支持
- [ ] **颜色预设**：预设颜色样式

### 响应式设计
- [ ] **移动端适配**：触摸友好的交互
- [ ] **断点响应**：不同屏幕尺寸适配
- [ ] **高 DPI 支持**：Retina 显示屏优化

## 可访问性实现清单
- [ ] **键盘导航**：Tab, Escape 键支持
- [ ] **屏幕阅读器**：ARIA 标签和角色
- [ ] **焦点管理**：焦点指示器
- [ ] **语义化标记**：正确的 HTML 语义

## 性能优化清单
- [ ] **渲染优化**：memo 和条件渲染
- [ ] **样式优化**：CSS-in-Rust 性能优化
- [ ] **事件优化**：事件委托和防抖
- [ ] **内存管理**：定时器清理
- [ ] **内容缓存**：避免重复计算

## 测试策略
### 单元测试
- [ ] **Props 测试**：所有属性的正确处理
- [ ] **事件测试**：触发方式的正确性
- [ ] **状态测试**：显示/隐藏状态变化
- [ ] **位置测试**：位置计算的准确性

### 集成测试
- [ ] **组件交互**：与其他组件的交互
- [ ] **主题集成**：主题系统的集成
- [ ] **事件冒泡**：事件处理的正确性

### 视觉测试
- [ ] **视觉回归**：与 Ant Design 的视觉对比
- [ ] **跨浏览器**：不同浏览器的兼容性
- [ ] **响应式测试**：不同屏幕尺寸的显示

## 质量保证
### 代码质量
- [ ] **类型安全**：完整的类型定义
- [ ] **错误处理**：优雅的错误处理
- [ ] **代码规范**：遵循项目编码规范
- [ ] **文档完整**：完整的代码注释

### 用户体验
- [ ] **交互一致性**：与 Ant Design 行为一致
- [ ] **性能表现**：流畅的显示/隐藏动画
- [ ] **错误提示**：友好的错误信息
- [ ] **延迟处理**：合理的延迟时间

## 发布检查清单
- [ ] **功能完整性**：所有功能都已实现
- [ ] **API 兼容性**：与 Ant Design API 兼容
- [ ] **样式一致性**：视觉效果与官方一致
- [ ] **测试覆盖率**：达到 90%+ 测试覆盖率
- [ ] **文档完整性**：API 文档和示例完整
- [ ] **性能基准**：满足性能要求
- [ ] **可访问性验证**：通过可访问性测试

## 已知限制和未来改进
### 当前限制
- 暂不支持复杂内容（仅支持文本）
- 暂不支持自定义容器
- 暂不支持虚拟元素定位

### 未来改进计划
- 支持 ReactNode 类型的复杂内容
- 支持 getPopupContainer 自定义容器
- 支持更多触发方式组合
- 支持虚拟滚动场景

## 参考资料
- [Ant Design Tooltip 官方文档](https://ant.design/components/tooltip/)
- [Ant Design Tooltip 对齐更新](https://ant.design/docs/blog/tooltip-align/)
- [WCAG 2.1 可访问性指南](https://www.w3.org/WAI/WCAG21/quickref/)
- [CSS-in-Rust 最佳实践](https://github.com/lukidoescode/css-in-rust)
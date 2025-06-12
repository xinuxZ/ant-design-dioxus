# Splitter 组件分析报告

## 组件概述

Splitter 是一个分割面板组件，用于在水平或垂直方向上分割区域，支持拖拽调整大小、面板折叠、尺寸限制等功能。该组件常用于构建可调整布局的界面，如代码编辑器、文件管理器等应用场景。

## 类型定义

### 核心枚举
```rust
// 分割器布局方向
pub enum SplitterLayout {
    Horizontal, // 水平分割
    Vertical,   // 垂直分割
}

// 尺寸类型
pub enum SizeType {
    Pixel(i32),      // 像素值
    Percentage(f32), // 百分比值
}

// 折叠配置
pub enum Collapsible {
    Disabled,                    // 禁用折叠
    Start,                      // 开始位置可折叠
    End,                        // 结束位置可折叠
    Header(CollapsibleConfig),   // 自定义折叠配置
}

// 折叠配置详情
pub struct CollapsibleConfig {
    pub start: bool,  // 开始位置折叠
    pub end: bool,    // 结束位置折叠
}
```

### 组件属性
```rust
// Splitter 主组件属性
pub struct SplitterProps {
    pub layout: SplitterLayout,                                    // 布局方向
    pub on_resize_start: Option<EventHandler<(usize, SizeType)>>, // 开始调整回调
    pub on_resize: Option<EventHandler<(usize, SizeType)>>,       // 调整中回调
    pub on_resize_end: Option<EventHandler<(usize, SizeType)>>,   // 结束调整回调
    pub lazy: bool,                                               // 懒加载模式
    pub children: Element,                                        // 子元素
    pub class: Option<String>,                                    // CSS 类名
    pub style: Option<String>,                                    // 内联样式
    pub id: Option<String>,                                       // 元素 ID
}

// SplitterPanel 面板属性
pub struct SplitterPanelProps {
    pub default_size: Option<SizeType>,  // 默认尺寸
    pub size: Option<SizeType>,          // 当前尺寸
    pub min: Option<SizeType>,           // 最小尺寸
    pub max: Option<SizeType>,           // 最大尺寸
    pub collapsible: Collapsible,        // 折叠配置
    pub resizable: bool,                 // 是否可调整
    pub children: Element,               // 子元素
    pub class: Option<String>,           // CSS 类名
    pub style: Option<String>,           // 内联样式
    pub id: Option<String>,              // 元素 ID
}
```

## 已实现功能

### ✅ 核心功能
1. **基础分割布局**
   - 水平分割 (`SplitterLayout::Horizontal`)
   - 垂直分割 (`SplitterLayout::Vertical`)
   - 面板容器和分割条渲染

2. **拖拽交互框架**
   - 鼠标按下事件处理 (`_handle_mouse_down`)
   - 鼠标移动事件处理 (`_handle_mouse_move`)
   - 鼠标释放事件处理 (`_handle_mouse_up`)
   - 全局事件监听器管理

3. **面板配置**
   - 尺寸设置 (`SizeType::Pixel`, `SizeType::Percentage`)
   - 最小/最大尺寸限制
   - 可调整性控制 (`resizable`)

### ✅ 样式功能
1. **基础样式**
   - 分割器容器样式 (`.ant-splitter`)
   - 水平/垂直布局样式
   - 面板样式 (`.ant-splitter-panel`)
   - 分割条样式 (`.ant-splitter-bar`)

2. **响应式设计**
   - 不同尺寸下的样式适配
   - 禁用状态样式
   - 悬停状态样式

3. **折叠功能样式**
   - 折叠按钮样式
   - 折叠状态指示
   - 动画过渡效果

### ✅ 架构功能
1. **组件设计**
   - 主组件 `Splitter`
   - 面板组件 `SplitterPanel`
   - 分割条组件 `SplitterBar`
   - 辅助函数 `horizontal_splitter`, `vertical_splitter`

2. **状态管理**
   - 拖拽状态 (`is_dragging`)
   - 拖拽起始位置 (`drag_start_position`)
   - 面板尺寸状态 (`panel_sizes`)
   - 活动分割器 (`active_splitter`)

3. **类型系统**
   - 尺寸类型转换 (`SizeType::to_css_string`, `SizeType::from_string`)
   - 布局方向枚举
   - 折叠配置类型

## 缺失功能

### 🔴 高优先级

1. **完整拖拽实现**
   ```rust
   // 当前：简化的鼠标事件处理
   fn _handle_mouse_move(evt: MouseEvent) {
       let delta = 100.0; // ❌ 硬编码值
       // 需要：实际坐标计算和面板尺寸更新
   }
   ```

2. **面板尺寸计算**
   ```rust
   // 缺失：动态尺寸计算逻辑
   // 缺失：约束条件验证 (min/max)
   // 缺失：百分比和像素值转换
   ```

3. **折叠功能实现**
   ```rust
   // 当前：基础折叠状态管理
   let is_collapsed = use_signal(|| false);
   // 缺失：折叠动画
   // 缺失：折叠按钮交互
   // 缺失：折叠状态持久化
   ```

4. **回调事件触发**
   ```rust
   // 缺失：on_resize_start 事件触发
   // 缺失：on_resize 事件触发
   // 缺失：on_resize_end 事件触发
   ```

### 🟡 中优先级

5. **多面板支持**
   ```rust
   // 当前：基础双面板支持
   // 缺失：动态面板数量
   // 缺失：面板插入/删除
   // 缺失：复杂布局嵌套
   ```

6. **键盘导航**
   ```rust
   // 缺失：Tab 键焦点管理
   // 缺失：箭头键调整尺寸
   // 缺失：Enter/Space 键折叠
   ```

7. **触摸设备支持**
   ```rust
   // 缺失：触摸事件处理
   // 缺失：手势识别
   // 缺失：移动端优化
   ```

8. **懒加载模式**
   ```rust
   // 当前：lazy 属性已定义
   pub lazy: bool, // ❌ 功能未实现
   // 缺失：延迟更新逻辑
   ```

### 🟢 低优先级

9. **高级动画**
   ```rust
   // 缺失：平滑调整动画
   // 缺失：弹性效果
   // 缺失：自定义动画曲线
   ```

10. **主题定制**
    ```rust
    // 缺失：Design Token 支持
    // 缺失：自定义颜色主题
    // 缺失：暗色模式适配
    ```

11. **性能优化**
    ```rust
    // 缺失：大量面板时的优化
    // 缺失：频繁调整时的节流
    // 缺失：内存使用优化
    ```

## 实现建议

### 第一阶段：核心拖拽功能
1. **实现真实坐标计算**
   ```rust
   fn handle_mouse_move(evt: MouseEvent, layout: SplitterLayout) -> f64 {
       match layout {
           SplitterLayout::Horizontal => evt.client_x() as f64,
           SplitterLayout::Vertical => evt.client_y() as f64,
       }
   }
   ```

2. **面板尺寸更新逻辑**
   ```rust
   fn update_panel_sizes(
       panel_sizes: &mut Signal<Vec<SizeType>>,
       delta: f64,
       active_index: usize,
       constraints: &[SizeConstraint],
   ) {
       // 计算新尺寸
       // 验证约束条件
       // 更新相邻面板
   }
   ```

3. **约束条件验证**
   ```rust
   struct SizeConstraint {
       min: Option<SizeType>,
       max: Option<SizeType>,
   }
   
   impl SizeConstraint {
       fn validate(&self, size: SizeType) -> SizeType {
           // 应用最小/最大限制
       }
   }
   ```

### 第二阶段：折叠功能
4. **折叠状态管理**
   ```rust
   struct CollapseState {
       collapsed_panels: HashSet<usize>,
       original_sizes: HashMap<usize, SizeType>,
   }
   ```

5. **折叠动画实现**
   ```rust
   fn animate_collapse(
       panel_index: usize,
       target_size: SizeType,
       duration: Duration,
   ) {
       // 使用 CSS 动画或 JS 动画
   }
   ```

### 第三阶段：高级功能
6. **多面板支持**
   ```rust
   fn render_panels(panels: &[SplitterPanelProps]) -> Vec<Element> {
       panels.iter().enumerate().map(|(index, panel)| {
           // 渲染面板和分割条
       }).collect()
   }
   ```

7. **键盘导航**
   ```rust
   fn handle_keyboard_event(evt: KeyboardEvent, active_splitter: usize) {
       match evt.key().as_str() {
           "ArrowLeft" | "ArrowUp" => adjust_size(-10),
           "ArrowRight" | "ArrowDown" => adjust_size(10),
           "Enter" | " " => toggle_collapse(),
           _ => {}
       }
   }
   ```

## 技术方案

### 拖拽实现方案
```rust
struct DragState {
    is_dragging: bool,
    start_position: f64,
    start_sizes: Vec<SizeType>,
    active_splitter: usize,
}

impl DragState {
    fn start_drag(&mut self, position: f64, splitter_index: usize) {
        self.is_dragging = true;
        self.start_position = position;
        self.active_splitter = splitter_index;
        // 保存初始尺寸
    }
    
    fn update_drag(&mut self, current_position: f64) -> Vec<SizeType> {
        let delta = current_position - self.start_position;
        // 计算新的面板尺寸
        self.calculate_new_sizes(delta)
    }
    
    fn end_drag(&mut self) {
        self.is_dragging = false;
        // 清理状态
    }
}
```

### 尺寸计算方案
```rust
struct SizeCalculator {
    container_size: f64,
    panel_count: usize,
}

impl SizeCalculator {
    fn pixel_to_percentage(&self, pixel: i32) -> f32 {
        (pixel as f64 / self.container_size * 100.0) as f32
    }
    
    fn percentage_to_pixel(&self, percentage: f32) -> i32 {
        (self.container_size * percentage as f64 / 100.0) as i32
    }
    
    fn distribute_remaining_space(&self, sizes: &mut [SizeType]) {
        // 分配剩余空间
    }
}
```

### 折叠动画方案
```css
.ant-splitter-panel {
    transition: width 0.3s ease, height 0.3s ease;
}

.ant-splitter-panel-collapsed {
    width: 0 !important;
    min-width: 0 !important;
    overflow: hidden;
}

.ant-splitter-panel-collapsed.vertical {
    height: 0 !important;
    min-height: 0 !important;
}
```

## 参考实现

1. [Ant Design Splitter 官方文档](https://ant.design/components/splitter/)
2. [React Split Pane 实现](https://github.com/tomkp/react-split-pane)
3. [Allotment React 组件](https://github.com/johnwalley/allotment)
4. [VS Code Split View 实现](https://github.com/microsoft/vscode)

## 约束条件

### 技术约束
- Dioxus 框架事件系统限制
- WASM 环境下的性能考虑
- CSS-in-Rust 样式系统约束

### 性能约束
- 频繁拖拽时的渲染性能
- 大量面板时的内存使用
- 动画效果的流畅性

### 兼容性约束
- 不同浏览器的鼠标事件支持
- 触摸设备的手势识别
- 屏幕阅读器的可访问性

## 代码质量问题

### 当前问题
1. **硬编码值**
   ```rust
   let delta = 100.0; // ❌ 应该从实际事件计算
   ```

2. **未实现的 TODO**
   ```rust
   // TODO: Implement actual mouse event handling
   // TODO: Add proper size calculations
   ```

3. **状态管理复杂性**
   - 多个 `use_signal` 状态
   - 状态同步问题
   - 副作用管理

4. **测试覆盖不足**
   - 缺少拖拽交互测试
   - 缺少尺寸计算测试
   - 缺少边界条件测试

### 改进建议
1. **引入状态管理器**
   ```rust
   struct SplitterState {
       layout: SplitterLayout,
       panel_sizes: Vec<SizeType>,
       drag_state: Option<DragState>,
       collapsed_panels: HashSet<usize>,
   }
   ```

2. **抽象事件处理**
   ```rust
   trait SplitterEventHandler {
       fn handle_drag_start(&mut self, position: f64, splitter_index: usize);
       fn handle_drag_move(&mut self, position: f64);
       fn handle_drag_end(&mut self);
   }
   ```

3. **增加测试覆盖**
   ```rust
   #[cfg(test)]
   mod tests {
       #[test]
       fn test_size_calculation() { /* ... */ }
       
       #[test]
       fn test_constraint_validation() { /* ... */ }
       
       #[test]
       fn test_collapse_functionality() { /* ... */ }
   }
   ```

## 性能优化建议

### 拖拽性能优化
```rust
// 使用 requestAnimationFrame 优化拖拽更新
fn throttled_drag_update(callback: impl Fn(f64)) {
    // 限制更新频率到 60fps
}

// 使用 CSS transform 代替直接修改尺寸
fn apply_drag_transform(element: &Element, delta: f64) {
    element.style().set_property("transform", &format!("translateX({}px)", delta));
}
```

### 内存优化
```rust
// 使用对象池复用拖拽状态
struct DragStatePool {
    pool: Vec<DragState>,
}

// 延迟计算面板尺寸
struct LazyPanelSize {
    calculator: Option<Box<dyn Fn() -> SizeType>>,
}
```

## 总结

Splitter 组件已建立了良好的架构基础，包括类型系统、组件结构和基础样式。组件设计思路清晰，支持水平/垂直布局、面板配置和折叠功能。

**优势：**
- ✅ 完整的类型系统设计
- ✅ 灵活的布局配置
- ✅ 良好的组件架构
- ✅ 基础的样式系统

**待改进：**
- 🔴 完整的拖拽功能实现
- 🔴 面板尺寸计算逻辑
- 🔴 折叠功能完善
- 🔴 回调事件触发
- 🟡 多面板支持和键盘导航

当前实现完成度约为 **40%**，架构设计完整但核心交互功能需要大量开发工作。建议优先实现拖拽功能和尺寸计算，这是组件的核心价值所在。
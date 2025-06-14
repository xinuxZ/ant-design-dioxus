# Drawer 组件开发计划

## 组件概述

Drawer（抽屉）是一个从屏幕边缘滑出的浮层面板组件，通常用于显示详细信息或执行操作，而无需离开当前页面。

## 类型定义

### 已实现的类型
- `DrawerPlacement`: 抽屉位置枚举（Top, Right, Bottom, Left）
- `DrawerSize`: 抽屉尺寸枚举（Default, Large）
- `DrawerProps`: 组件属性结构体
- `DrawerStyleGenerator`: 样式生成器

## 功能实现状态

### ✅ 已实现的核心功能
1. **基础显示控制**
   - `open`: 控制抽屉显示/隐藏
   - `placement`: 抽屉出现位置（上下左右）
   - `size`/`drawer_size`: 预设尺寸控制

2. **内容结构**
   - `title`: 抽屉标题
   - `children`: 抽屉主体内容
   - `footer`: 自定义页脚
   - `extra`: 操作区域

3. **交互控制**
   - `closable`: 显示关闭按钮
   - `mask_closable`: 点击遮罩关闭
   - `keyboard`: ESC键关闭支持
   - `on_close`: 关闭回调

4. **样式定制**
   - `class_name`: 自定义类名
   - `style`: 抽屉样式
   - `body_style`: 主体样式
   - `header_style`: 头部样式

5. **高级特性**
   - `mask`: 遮罩层控制
   - `z_index`: 层级控制
   - `destroy_on_close`: 关闭时销毁内容
   - `get_container`: 渲染容器控制
   - `after_open_change`: 状态变化回调

### ❌ 缺失的功能

#### 高优先级
1. **动画系统**
   - 滑入/滑出动画效果
   - 动画持续时间配置
   - 动画缓动函数
   - 动画状态管理

2. **嵌套抽屉支持**
   - `push`: 嵌套抽屉推拉行为
   - 多层级抽屉管理
   - 层级关系处理

3. **加载状态**
   - `loading`: 加载状态显示
   - Skeleton 骨架屏支持

#### 中优先级
1. **尺寸控制增强**
   - `width`: 自定义宽度
   - `height`: 自定义高度
   - 响应式尺寸适配

2. **渲染优化**
   - `force_render`: 强制预渲染
   - `get_container`: 自定义容器函数
   - Portal 渲染支持

3. **可访问性**
   - `auto_focus`: 自动聚焦
   - ARIA 属性支持
   - 键盘导航
   - 焦点管理

#### 低优先级
1. **语义化结构**
   - `class_names`: 语义化类名配置
   - `styles`: 语义化样式配置

2. **自定义渲染**
   - `drawer_render`: 自定义抽屉内容渲染
   - 自定义关闭图标

## 实现建议

### 1. 动画系统实现
```rust
// 添加动画相关属性
pub struct DrawerProps {
    // ... 现有属性
    
    /// 动画持续时间（毫秒）
    #[props(default = 300)]
    pub animation_duration: u32,
    
    /// 动画缓动函数
    #[props(default = "ease".to_string())]
    pub animation_timing: String,
    
    /// 是否禁用动画
    #[props(default = false)]
    pub disable_animation: bool,
}

// 动画状态管理
#[derive(Debug, Clone, PartialEq)]
pub enum DrawerAnimationState {
    Closed,
    Opening,
    Open,
    Closing,
}
```

### 2. 嵌套抽屉支持
```rust
// 添加嵌套相关属性
pub struct DrawerProps {
    // ... 现有属性
    
    /// 嵌套抽屉推拉行为
    #[props(default)]
    pub push: Option<DrawerPushConfig>,
    
    /// 父抽屉引用
    #[props(default)]
    pub parent_drawer: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DrawerPushConfig {
    pub distance: String, // 推拉距离
}
```

### 3. 加载状态实现
```rust
// 添加加载相关属性
pub struct DrawerProps {
    // ... 现有属性
    
    /// 加载状态
    #[props(default = false)]
    pub loading: bool,
    
    /// 加载指示器类型
    #[props(default = LoadingType::Skeleton)]
    pub loading_type: LoadingType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LoadingType {
    Skeleton,
    Spin,
}
```

## 技术方案

### 动画实现方案
1. **CSS Transition**
   - 使用 CSS transform 实现滑动效果
   - 通过类名切换控制动画状态
   - 支持自定义动画参数

2. **状态机管理**
   - 实现动画状态机
   - 处理动画生命周期
   - 防止动画冲突

### 嵌套抽屉方案
1. **上下文管理**
   - 使用 Context 管理抽屉层级
   - 实现父子抽屉通信
   - 处理层级关系

2. **推拉效果**
   - 计算推拉距离
   - 动态调整父抽屉位置
   - 处理多层嵌套

### 渲染优化方案
1. **Portal 渲染**
   - 实现 Portal 组件
   - 支持自定义容器
   - 处理事件冒泡

2. **条件渲染**
   - 优化渲染性能
   - 实现懒加载
   - 内存管理

## 参考实现

### Ant Design React 版本
- 动画系统：使用 rc-motion 库
- 嵌套抽屉：push 属性配置
- 加载状态：Skeleton 组件集成
- 可访问性：完整的 ARIA 支持

### 关键特性对比
| 特性 | React 版本 | 当前实现 | 优先级 |
|------|------------|----------|--------|
| 基础显示 | ✅ | ✅ | - |
| 动画效果 | ✅ | ❌ | 高 |
| 嵌套抽屉 | ✅ | ❌ | 高 |
| 加载状态 | ✅ | ❌ | 高 |
| 自定义尺寸 | ✅ | 部分 | 中 |
| 可访问性 | ✅ | 部分 | 中 |
| 语义化结构 | ✅ | ❌ | 低 |

## 约束条件

### 技术约束
1. **Dioxus 框架限制**
   - 动画实现依赖 CSS
   - 事件处理机制
   - 组件生命周期

2. **Web 平台限制**
   - DOM 操作能力
   - CSS 动画支持
   - 浏览器兼容性

### 性能约束
1. **渲染性能**
   - 大型抽屉内容渲染
   - 多层嵌套性能
   - 动画流畅度

2. **内存使用**
   - 组件实例管理
   - 事件监听器清理
   - 动画资源释放

## 代码质量问题

### 当前问题
1. **键盘事件处理**
   - 事件监听器未正确清理
   - 可能导致内存泄漏
   - 需要实现 cleanup 机制

2. **样式系统**
   - 样式生成器实现不完整
   - 缺少主题令牌集成
   - 需要完善样式架构

3. **错误处理**
   - 缺少边界情况处理
   - 错误状态管理不足
   - 需要增强错误恢复

### 改进建议
1. **重构事件处理**
   - 实现事件监听器自动清理
   - 使用 use_effect cleanup
   - 优化事件处理逻辑

2. **完善样式系统**
   - 实现完整的样式生成器
   - 集成主题令牌系统
   - 支持动态样式计算

3. **增强类型安全**
   - 完善类型定义
   - 增加运行时检查
   - 改进错误提示

## 总结

Drawer 组件已实现基础功能，包括显示控制、内容结构、交互控制和样式定制。主要缺失动画系统、嵌套抽屉支持和加载状态等高级特性。建议优先实现动画系统以提升用户体验，然后逐步完善嵌套抽屉和加载状态功能。同时需要重构事件处理机制和完善样式系统以提高代码质量。
# Breadcrumb 组件 TODO 清单

## 已实现功能 ✅

### 基础功能
- [x] 基本面包屑显示
- [x] 面包屑项（BreadcrumbItem）
- [x] 链接支持（href）
- [x] 禁用状态
- [x] 图标支持
- [x] 点击事件处理

### 样式和主题
- [x] 基础样式系统
- [x] 自定义类名和样式
- [x] CSS-in-Rust 样式注入

### 数据驱动
- [x] items 属性支持
- [x] 数据结构定义（BreadcrumbItemData）
- [x] 子元素和数据两种使用方式

### 分隔符
- [x] 自定义分隔符文本
- [x] 自定义分隔符元素
- [x] 分隔符自动插入

### 可访问性
- [x] aria-label 支持
- [x] aria-hidden 分隔符
- [x] 语义化 HTML 结构（ol/li）

### 开发工具
- [x] Props 结构定义
- [x] 类型安全的数据结构
- [x] 组件文档和示例

## 缺失的核心功能 ❌

### 高级功能
- [ ] **下拉菜单支持**
  - [ ] menu 属性（替代 overlay）
  - [ ] 下拉菜单项配置
  - [ ] 菜单事件处理
  - [ ] 菜单样式定制

### 路由集成
- [ ] **itemRender 自定义渲染**
  - [ ] 自定义渲染函数
  - [ ] 路由参数传递
  - [ ] 路径构建逻辑
  - [ ] 与路由库集成

### 高级配置
- [ ] **params 路由参数**
  - [ ] 参数对象支持
  - [ ] 参数传递机制
  - [ ] 动态路径生成

### 分隔符增强
- [ ] **Breadcrumb.Separator 组件**
  - [ ] 独立分隔符组件
  - [ ] 自定义分隔符位置
  - [ ] 分隔符样式控制

### 响应式支持
- [ ] **移动端适配**
  - [ ] 响应式布局
  - [ ] 移动端交互优化
  - [ ] 触摸友好的尺寸

### 主题系统
- [ ] **Design Token 集成**
  - [ ] 颜色 token（itemColor, lastItemColor, linkColor, linkHoverColor, separatorColor）
  - [ ] 尺寸 token（iconFontSize, separatorMargin）
  - [ ] 主题切换支持

### 性能优化
- [ ] **渲染优化**
  - [ ] 虚拟化支持（长面包屑）
  - [ ] 懒加载
  - [ ] 重渲染控制

### 交互增强
- [ ] **键盘导航**
  - [ ] Tab 键导航
  - [ ] Enter/Space 键激活
  - [ ] 方向键导航

### 可访问性增强
- [ ] **完整 ARIA 支持**
  - [ ] aria-current 当前页面标识
  - [ ] 屏幕阅读器优化
  - [ ] 高对比度模式支持

### 国际化
- [ ] **多语言支持**
  - [ ] RTL 布局支持
  - [ ] 本地化文本
  - [ ] 文本方向适配

### 高级样式
- [ ] **样式定制**
  - [ ] 自定义主题
  - [ ] 样式变量
  - [ ] CSS 变量支持

### 事件系统
- [ ] **完整事件支持**
  - [ ] onMouseEnter/onMouseLeave
  - [ ] onFocus/onBlur
  - [ ] 自定义事件处理

### 开发体验
- [ ] **文档和示例**
  - [ ] 完整的 API 文档
  - [ ] 交互式示例
  - [ ] 最佳实践指南
- [ ] **测试覆盖**
  - [ ] 单元测试
  - [ ] 集成测试
  - [ ] 可访问性测试

### API 兼容性
- [ ] **Ant Design 5.x 兼容**
  - [ ] items API 完整支持
  - [ ] 废弃 API 警告
  - [ ] 迁移指南

## 优先级建议

### 高优先级 🔥
1. **下拉菜单支持** - Ant Design 的核心功能 <mcreference link="https://ant.design/components/breadcrumb/" index="1">1</mcreference>
2. **itemRender 自定义渲染** - 路由集成的关键功能 <mcreference link="https://ant.design/components/breadcrumb/" index="1">1</mcreference>
3. **Breadcrumb.Separator 组件** - API 完整性
4. **Design Token 集成** - 主题系统支持 <mcreference link="https://ant.design/components/breadcrumb/" index="1">1</mcreference>

### 中优先级 ⚡
1. **params 路由参数** - 动态路由支持
2. **键盘导航** - 可访问性增强
3. **响应式支持** - 移动端友好
4. **性能优化** - 大规模应用需求

### 低优先级 📝
1. **高级样式定制** - 特殊场景需求
2. **国际化** - 特定地区需求
3. **开发工具增强** - 开发体验优化

## 实现建议

### 技术考虑
1. **参考 Ant Design 实现**：严格按照官方 API 设计 <mcreference link="https://ant.design/components/breadcrumb/" index="1">1</mcreference>
2. **Dioxus 约束**：考虑 Rust/WASM 的性能特点
3. **样式系统**：使用 CSS-in-Rust 方案
4. **状态管理**：合理使用 use_signal 和 use_effect
5. **类型安全**：充分利用 Rust 的类型系统

### 架构建议
1. **模块化设计**：Breadcrumb、BreadcrumbItem、BreadcrumbSeparator 分离
2. **样式分离**：样式逻辑独立模块
3. **工具函数**：可复用的辅助函数
4. **测试驱动**：先写测试再实现功能

### 兼容性
- 保持与 Ant Design 5.x API 兼容 <mcreference link="https://github.com/ant-design/ant-design/issues/41147" index="3">3</mcreference>
- 支持新的 items API，废弃旧的 Breadcrumb.Item 用法
- 考虑未来版本的迁移路径
- 支持主流浏览器

### 注意事项
- Ant Design 5.x 中 `Breadcrumb.Item` 和 `Breadcrumb.Separator` 已被废弃，推荐使用 `items` API <mcreference link="https://github.com/ant-design/ant-design/issues/41147" index="3">3</mcreference>
- 需要支持下拉菜单的新 API（menu 替代 overlay）<mcreference link="https://4x.ant.design/components/breadcrumb/" index="2">2</mcreference>
- 考虑与 React Router 等路由库的集成模式 <mcreference link="https://3x.ant.design/components/breadcrumb/" index="5">5</mcreference>
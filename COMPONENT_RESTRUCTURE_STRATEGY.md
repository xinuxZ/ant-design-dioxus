# Ant Design Dioxus 组件库复刻策略
1. 时刻牢记项目的目标和愿景： 复刻 ant-design 框架; 组件样式完全使用 ant-design 框架的样式; 组件的使用方法和注意事项完全使用 ant-design 框架的使用方法和注意事项。

## 组件复杂度分析

基于对现有组件结构的分析，我们将 70+ 组件按复杂度分为三个等级：

### Level 1: 基础组件（简单结构）

**特征**：
- 单一文件实现（mod.rs）
- 简单的 Props 结构
- 基础样式系统
- 无子组件或复杂状态管理

**组件列表**：
- [x] **icon** ⭐         # 图标 (最高优先级 - 被大量组件依赖)
- [x] **skeleton**        # 骨架屏 (被数据加载组件依赖)
- [x] **space**           # 间距 (被布局组件依赖)
- [x] **spin**            # 加载中 (被异步组件依赖)
- [x] **flex**            # 弹性布局 (被布局组件依赖)
- [x] **divider**         # 分割线组件
<!-- - [] **back_top**        # 回到顶部 暂时忽略，不实现；后续使用 float button 代替 -->
- [ ] **statistic**       # 统计数值
- [ ] **watermark**       # 水印
- [ ] **image**           # 图片
- [ ] **result**          # 结果页
- [x] **qr_code**         # 二维码

**推荐结构**：
```
component_name/
├── mod.rs              # 主要组件导出和公共API
├── component.rs        # 核心组件实现
├── types.rs           # 类型定义（Props、枚举等）
├── styles.rs          # CSS-in-Rust 样式实现
├── tests.rs           # 单元测试
└── TODO.md            # 功能差异分析
```

### Level 2: 复杂组件（中等结构）

**特征**：
- 包含子组件或子模块
- 复杂的状态管理
- 多种变体和配置
- 事件处理和交互逻辑

**组件列表**：
- [ ] **button** ⭐       # 按钮（含 ButtonGroup）(高优先级 - 被大量组件依赖) - 依赖: icon, spin
- [ ] **tooltip** ⭐      # 文字提示 (高优先级 - 被大量组件依赖) - 依赖: 无
- [ ] **input** ⭐        # 输入框（需要 TextArea、Search、Password 等子组件）(高优先级 - 被表单组件依赖) - 依赖: icon
- [ ] **table**          # 表格 - 依赖: checkbox, radio, button, pagination, spin
- [ ] **menu**           # 导航菜单 - 依赖: icon, tooltip
- [ ] **tabs**           # 标签页 - 依赖: icon
- [ ] **modal**          # 对话框 - 依赖: button, icon
- [ ] **drawer**         # 抽屉 - 依赖: button, icon
- [ ] **dropdown**       # 下拉菜单 - 依赖: button, popover
- [ ] **popover**        # 气泡卡片 - 依赖: tooltip
- [ ] **date_picker**    # 日期选择器 - 依赖: input, dropdown, button
- [ ] **time_picker**    # 时间选择器 - 依赖: input, dropdown
- [ ] **calendar**       # 日历 - 依赖: button, select, date_picker
- [ ] **tree**           # 树形控件 - 依赖: checkbox, icon
- [ ] **tree_select**    # 树选择 - 依赖: tree, select
- [ ] **cascader**       # 级联选择 - 依赖: input, dropdown, tag
- [ ] **auto_complete**  # 自动完成 - 依赖: input, dropdown
- [ ] **upload**         # 上传 - 依赖: button, progress, icon
- [ ] **steps**          # 步骤条 - 依赖: icon
- [ ] **breadcrumb**     # 面包屑 - 依赖: icon
- [ ] **pagination**     # 分页 - 依赖: button, select, input
- [ ] **list**           # 列表 - 依赖: spin, pagination, avatar
- [ ] **card**           # 卡片 - 依赖: 无
- [ ] **collapse**       # 折叠面板 - 依赖: icon
- [ ] **carousel**       # 走马灯 - 依赖: button, icon
- [ ] **anchor**         # 锚点 - 依赖: 无
- [ ] **affix**          # 固钉 - 依赖: 无
- [ ] **badge**          # 徽标数 - 依赖: 无
- [ ] **avatar**         # 头像 - 依赖: icon, image
- [ ] **tag**            # 标签 - 依赖: icon
- [ ] **alert**          # 警告提示 (已实现) - 依赖: icon, button
- [ ] **progress**       # 进度条 - 依赖: 无
- [ ] **rate**           # 评分 - 依赖: icon
- [ ] **slider**         # 滑动输入条 - 依赖: tooltip
- [ ] **switch**         # 开关 - 依赖: 无
- [ ] **checkbox**       # 多选框 - 依赖: 无
- [ ] **radio**          # 单选框 - 依赖: 无
- [ ] **input_number**   # 数字输入框 - 依赖: input, button
- [ ] **select**         # 选择器 - 依赖: input, dropdown, tag
- [ ] **mentions**       # 提及 - 依赖: input, dropdown
- [ ] **color_picker**   # 颜色选择器 - 依赖: input, dropdown
- [ ] **segmented**      # 分段控制器 - 依赖: 无
- [ ] **splitter**       # 分割面板 - 依赖: 无
- [ ] **transfer**       # 穿梭框 - 依赖: checkbox, button, input
- [ ] **timeline**       # 时间轴 - 依赖: icon
- [ ] **descriptions**   # 描述列表 - 依赖: 无
- [ ] **float_button**   # 悬浮按钮 - 依赖: button, tooltip
- [ ] **tour**           # 漫游式引导 - 依赖: button, popover

**推荐结构**：
```
component_name/
├── mod.rs              # 主要组件导出
├── components/         # 子组件目录
│   ├── main_component.rs
│   ├── sub_component.rs
│   └── mod.rs
├── types/              # 类型定义目录
│   ├── props.rs
│   ├── enums.rs
│   └── mod.rs
├── styles/             # 样式模块目录
│   ├── base.rs
│   ├── variants.rs
│   └── mod.rs
├── hooks/              # 自定义 hooks 目录（如果需要）
├── utils/              # 工具函数目录
├── tests/              # 测试目录
└── TODO.md
```

### Level 3: 模块化组件（复杂结构）

**特征**：
- 多个功能模块
- 复杂的业务逻辑
- 多层嵌套结构
- 高度可配置

**组件列表**：
- [ ] **layout**         # 布局（含 Header、Sider、Content、Footer）- 依赖: 无 (但被其他组件依赖)
- [ ] **grid**           # 栅格（含 Row、Col）- 依赖: 无 (但被其他组件依赖)
- [ ] **form** ⭐         # 表单（含 FormItem、FormList 等）(核心组件) - 依赖: input, select, checkbox, radio, button, date_picker, upload, etc.
- [ ] **message**        # 全局提示 - 依赖: icon, button
- [ ] **notification**   # 通知提醒框 - 依赖: icon, button
- [ ] **popconfirm**     # 气泡确认框 - 依赖: popover, button
- [ ] **app**            # 应用包裹组件 - 依赖: config_provider, message, notification
- [ ] **config_provider**# 全局配置 - 依赖: theme, locale
- [ ] **typography**     # 排版（含 Title、Text、Paragraph）- 依赖: tooltip, button
- [ ] **empty**          # 空状态 - 依赖: image, button

**推荐结构**：
```
component_name/
├── mod.rs
├── core/               # 核心功能模块
│   ├── components/
│   ├── types/
│   ├── hooks/
│   └── utils/
├── features/           # 功能特性模块
│   ├── validation/     # 表单验证
│   ├── layout/         # 布局管理
│   └── interaction/    # 交互逻辑
├── styles/
├── tests/
└── TODO.md
```


## 任务计划
### 1. 编码规范

**文件命名规范**：
```
# 组件文件
component.rs      # 主组件实现
types.rs         # 类型定义
styles.rs        # 样式实现
hooks.rs         # 自定义 hooks
utils.rs         # 工具函数
tests.rs         # 单元测试

# 子组件目录
components/
├── main.rs      # 主要子组件
├── item.rs      # 项目子组件
├── group.rs     # 组合子组件
└── mod.rs       # 模块导出
```

**Props 结构规范**：
```rust
#[derive(Props, Clone, PartialEq)]
pub struct ComponentProps {
    // 1. 核心功能属性
    #[props(default)]
    pub variant: ComponentVariant,

    #[props(default)]
    pub size: ComponentSize,

    // 2. 状态属性
    #[props(default = false)]
    pub disabled: bool,

    #[props(default = false)]
    pub loading: bool,

    // 3. 事件处理
    pub on_click: Option<EventHandler<MouseEvent>>,

    // 4. 样式定制
    pub class: Option<String>,
    pub style: Option<String>,

    // 5. 子元素
    pub children: Element,
}
```

**样式生成器规范**：
```rust
pub struct ComponentStyleGenerator {
    variant: ComponentVariant,
    size: ComponentSize,
    disabled: bool,
    // ... 其他状态
}

impl ComponentStyleGenerator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn variant(mut self, variant: ComponentVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn generate(&self) -> String {
        // 生成样式逻辑
    }
}
```


**CSS-in-Rust 集成方案**
基于 `css-in-rust` 库的特性，采用以下样式管理策略：

```rust
// styles.rs 示例结构
use css_in_rust::{css, css_if, theme};

// 基础样式定义
pub struct ComponentStyles {
    pub base: String,
    pub variants: VariantStyles,
    pub states: StateStyles,
}

// 变体样式
pub struct VariantStyles {
    pub size_small: String,
    pub size_medium: String,
    pub size_large: String,
    pub type_primary: String,
    pub type_secondary: String,
}

// 状态样式
pub struct StateStyles {
    pub hover: String,
    pub active: String,
    pub disabled: String,
    pub loading: String,
}

// 样式生成器
pub fn generate_component_styles() -> ComponentStyles {
    ComponentStyles {
        base: css!(r#"
            display: inline-flex;
            align-items: center;
            justify-content: center;
            border: none;
            cursor: pointer;
            transition: all 0.2s;
        "#),
        variants: VariantStyles {
            size_small: css!("padding: 4px 8px; font-size: 12px;"),
            size_medium: css!("padding: 8px 16px; font-size: 14px;"),
            size_large: css!("padding: 12px 24px; font-size: 16px;"),
            // ...
        },
        // ...
    }
}
```

#### 2. 技术规范

##### 代码质量标准

1. **类型安全**
   - 所有公共 API 必须有明确的类型定义
   - 使用 `#[derive(Props, Clone, PartialEq)]` 标准化 Props

3. **性能优化**
   - 使用 `memo` 优化不必要的重渲染
   - 实现懒加载和虚拟滚动
   - 优化样式计算和应用

3. **可访问性**
   - 遵循 WCAG 2.1 AA 标准
   - 提供适当的 ARIA 属性
   - 支持键盘导航

##### 样式规范

1. **主题系统**
   - 支持亮色/暗色主题
   - 提供主题定制接口
   - 实现主题切换动画

2. **响应式设计**
   - 支持移动端适配
   - 实现断点系统
   - 优化触摸交互

3. **动画效果**
   - 使用 CSS 动画而非 JavaScript
   - 提供动画开关选项
   - 遵循 Material Design 动画原则

## 实施计划
### 1. 任务执行步骤
#### **Step 0: 前置功能分析（必须步骤）**

任务的首要目标是组件功能与 Ant Design 官方组件保持功能一致。
- **调用 deepwiki 工具分析组件在 Ant Design 官方实现细节**，研究官方文档和源码实现细节
- **生成组件完整功能清单 Feature.md 文件**：记录组件应有的功能与API细节
- **制定技术方案和实现计划**: 确保所有核心功能、API、属性、事件处理都得到完整实现, 确保与 Ant Design 官方组件 API 一致

##### Feature.md 文件内容格式
```markdown
# [组件名称]

## 功能清单
### [功能名称] 实现分析

#### 官方实现分析
- **核心逻辑**：[通过 deepwiki 分析得出]
- **关键技术点**：[列出重要实现细节]
- **API 设计**：[接口设计分析]

#### Rust + Dioxus 实现方案
- **类型设计**：[Rust + Dioxus 类型系统设计]
- **状态管理**：[状态管理策略]
- **事件处理**：[事件处理机制]
- **样式实现**：[CSS-in-Rust 实现]

#### 实现计划
1. [ ] 类型定义
2. [ ] 核心逻辑实现
3. [ ] 样式集成
4. [ ] 事件处理
5. [ ] 测试编写
6. [ ] 文档更新

```

#### **Step 1: 创建组件目录结构**
```bash
# 使用结构生成器
./structure_generator --component button --level 2
```

#### **Step 2: 代码实现**
- 按照 Feature.md 功能清单实现组件所有功能

#### **Step 3: 编写测试用例**

#### **Step 4: 验证功能**
- 测试策略：制定功能验证测试计划,通过测试用例验证功能的正确性和完整性
- 生成单元测试
- 运行现有测试
- 性能对比测试

#### **Step 5 : 更新文档与进度记录**
- **标记完成状态**：在实现功能后，必须在对应组件的 Feature.md 文件中标记已完成的功能项
- **文档更新**：更新组件文档，反映新增的功能特性

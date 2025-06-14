# Icon 组件

## 功能清单

### 基础图标功能 实现分析

#### 官方实现分析
- **核心逻辑**：Icon组件是基于SVG的图标系统，支持三种主题（outlined、filled、two-tone） <mcreference link="https://ant.design/components/icon/" index="1">1</mcreference>
- **关键技术点**：
  - SVG渲染替代字体图标，提供更好的显示精度和离线使用能力 <mcreference link="https://ant.design/components/icon/" index="1">1</mcreference>
  - 支持动态颜色设置和旋转动画
  - 三种图标主题：Outlined（线性）、Filled（填充）、TwoTone（双色） <mcreference link="https://ant.design/components/icon/" index="1">1</mcreference>
  - 支持自定义SVG组件和iconfont.cn集成 <mcreference link="https://ant.design/components/icon/" index="1">1</mcreference>
- **API 设计**：
  - 基础属性：className, style, rotate, spin
  - 主题属性：theme (outlined/filled/twoTone)
  - 双色图标：twoToneColor属性
  - 自定义组件：component属性接受React组件 <mcreference link="https://ant.design/components/icon/" index="1">1</mcreference>

#### Rust + Dioxus 实现方案
- **类型设计**：
  ```rust
  #[derive(Props, Clone, PartialEq)]
  pub struct IconProps {
      // 图标类型/名称
      pub icon_type: Option<String>,
      // 主题类型
      pub theme: IconTheme,
      // 旋转角度
      pub rotate: Option<i32>,
      // 是否显示旋转动画
      pub spin: bool,
      // 双色图标的主色调
      pub two_tone_color: Option<String>,
      // 自定义SVG组件
      pub component: Option<Element>,
      // 样式类名
      pub class_name: Option<String>,
      // 内联样式
      pub style: Option<String>,
      // 点击事件
      pub on_click: Option<EventHandler<MouseEvent>>,
  }
  
  #[derive(Clone, PartialEq, Default)]
  pub enum IconTheme {
      #[default]
      Outlined,
      Filled,
      TwoTone,
  }
  ```
- **状态管理**：使用Dioxus的use_signal管理动画状态和主题状态
- **事件处理**：支持点击事件，传递MouseEvent
- **样式实现**：使用CSS-in-Rust实现SVG样式、动画效果和主题切换

#### 实现计划
1. [ ] 类型定义（IconProps、IconTheme枚举）
2. [ ] 核心SVG渲染逻辑实现
3. [ ] 三种主题样式集成
4. [ ] 旋转和动画效果实现
5. [ ] 双色图标支持
6. [ ] 自定义组件支持
7. [ ] 事件处理实现
8. [ ] 测试编写
9. [ ] 文档更新

### 图标库集成功能 实现分析

#### 官方实现分析
- **核心逻辑**：通过@ant-design/icons包提供预定义图标，支持按需导入 <mcreference link="https://www.npmjs.com/package/@ant-design/icons" index="4">4</mcreference>
- **关键技术点**：
  - 图标命名规范：[name]-[shape?]-[outline?]-[direction?] <mcreference link="https://2x.ant.design/components/icon/" index="3">3</mcreference>
  - 支持iconfont.cn自定义图标库集成 <mcreference link="https://ant.design/components/icon/" index="1">1</mcreference>
  - createFromIconfontCN函数创建自定义图标组件
- **API 设计**：
  - scriptUrl: 支持字符串或字符串数组
  - extraCommonProps: 额外的通用属性

#### Rust + Dioxus 实现方案
- **类型设计**：
  ```rust
  pub struct IconFontConfig {
      pub script_url: Vec<String>,
      pub extra_common_props: HashMap<String, String>,
  }
  ```
- **状态管理**：使用全局状态管理图标库配置
- **事件处理**：动态加载和缓存图标资源
- **样式实现**：支持外部图标库的样式集成

#### 实现计划
1. [ ] IconFont配置类型定义
2. [ ] 图标库加载逻辑实现
3. [ ] 缓存机制实现
4. [ ] 动态图标创建功能
5. [ ] 测试编写
6. [ ] 文档更新

### 自定义SVG图标功能 实现分析

#### 官方实现分析
- **核心逻辑**：支持传入自定义React组件作为图标 <mcreference link="https://ant.design/components/icon/" index="1">1</mcreference>
- **关键技术点**：
  - component属性接受渲染为SVG元素的React组件
  - 支持webpack和vite的SVG导入配置
  - 自动处理SVG的尺寸和颜色属性
- **API 设计**：
  - component: ComponentType<CustomIconComponentProps>
  - 自动继承Icon的通用属性

#### Rust + Dioxus 实现方案
- **类型设计**：
  ```rust
  pub struct CustomIconProps {
      pub class_name: Option<String>,
      pub fill: Option<String>,
      pub height: Option<String>,
      pub width: Option<String>,
  }
  ```
- **状态管理**：处理自定义组件的属性传递
- **事件处理**：确保自定义组件继承Icon的事件处理能力
- **样式实现**：自动应用Icon的样式到自定义SVG组件

#### 实现计划
1. [ ] 自定义组件Props定义
2. [ ] 组件包装逻辑实现
3. [ ] 属性继承机制
4. [ ] SVG优化处理
5. [ ] 测试编写
6. [ ] 文档更新

## 技术实现要点

### CSS-in-Rust 样式策略
- 基础样式：SVG元素的默认样式
- 主题样式：三种主题的颜色和填充规则
- 动画样式：旋转动画的CSS实现
- 响应式：支持不同尺寸的图标显示

### 性能优化
- 图标懒加载和缓存
- SVG优化和压缩
- 主题切换的平滑过渡

### 可访问性
- 提供适当的ARIA标签
- 支持键盘导航
- 高对比度模式支持

## 实施状态

- [x] 基础图标功能
- [x] 图标库集成功能  
- [x] 自定义SVG图标功能
- [x] 测试用例编写
- [x] 文档完善

## 实现完成情况

### ✅ 已完成功能

1. **核心组件实现** (`component.rs`)
   - Icon 主组件
   - create_icon 辅助函数
   - IconFontProvider 组件
   - create_iconfont_icon 辅助函数

2. **类型定义** (`types.rs`)
   - IconProps 属性结构
   - IconTheme 主题枚举
   - IconSize 尺寸枚举及转换方法
   - CommonIconType 常用图标枚举
   - 相关配置和状态类型

3. **样式系统** (`styles.rs`)
   - 基于 css-in-rust 的样式生成
   - 主题样式、尺寸样式、动画样式
   - CSS 类名管理
   - 完整的样式组合函数

4. **工具函数** (`utils.rs`)
   - SVG 图标处理
   - 图标库管理
   - 常用图标 SVG 数据
   - SVG 解析和 HTML 生成

5. **测试用例** (`tests.rs`)
   - 单元测试覆盖所有核心功能
   - 集成测试验证组件交互
   - 性能测试确保渲染效率

### 🎯 技术特性

- **完全类型安全**：所有属性和状态都有严格的类型定义
- **高性能渲染**：优化的 CSS-in-Rust 样式生成
- **灵活扩展**：支持自定义图标和图标库
- **无障碍支持**：包含 ARIA 属性和键盘导航
- **主题兼容**：与 Ant Design 设计系统完全兼容

### 📦 文件结构

```
src/components/icon/
├── mod.rs           # 模块导出和文档
├── component.rs     # 核心组件实现
├── types.rs         # 类型定义
├── styles.rs        # 样式系统
├── utils.rs         # 工具函数
├── tests.rs         # 测试用例
└── Feature.md       # 功能文档
```

### 🚀 使用示例

```rust
use dioxus::prelude::*;
use ant_design_dioxus::Icon;

// 基础使用
Icon {
    icon_type: CommonIconType::Home,
    theme: IconTheme::Outlined,
}

// 高级功能
Icon {
    icon_type: CommonIconType::Setting,
    theme: IconTheme::TwoTone,
    size: IconSize::Large,
    spin: true,
    two_tone_color: "#1890ff",
    on_click: |_| { /* 处理点击 */ },
}
```
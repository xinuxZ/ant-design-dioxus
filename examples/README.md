# Examples 目录结构说明

本目录包含 Ant Design Dioxus 组件库的所有示例代码，按照组件类型进行分类组织。

## 目录结构

```
examples/
├── README.md                    # 本说明文件
├── showcase.rs                  # 统一展示入口（类似官方文档）
├── common/                      # 公共组件和工具
│   ├── mod.rs
│   ├── demo_section.rs         # 演示区块组件
│   ├── api_documentation.rs    # API文档组件
│   └── component_menu.rs       # 组件菜单
├── general/                     # 通用组件
│   ├── mod.rs
│   ├── button/
│   │   ├── mod.rs
│   │   ├── basic.rs            # 基础用法
│   │   ├── size.rs             # 尺寸示例
│   │   ├── type.rs             # 类型示例
│   │   ├── disabled.rs         # 禁用状态
│   │   ├── loading.rs          # 加载状态
│   │   ├── danger.rs           # 危险按钮
│   │   └── group.rs            # 按钮组
│   ├── icon/
│   │   ├── mod.rs
│   │   ├── basic.rs            # 基础图标
│   │   ├── theme.rs            # 主题风格
│   │   ├── size.rs             # 尺寸示例
│   │   ├── color.rs            # 颜色示例
│   │   └── rotate.rs           # 旋转示例
│   └── typography/
│       ├── mod.rs
│       ├── title.rs            # 标题组件
│       ├── text.rs             # 文本组件
│       ├── paragraph.rs        # 段落组件
│       └── interactive.rs      # 交互功能
├── layout/                      # 布局组件
│   ├── mod.rs
│   ├── grid/
│   │   ├── mod.rs
│   │   ├── basic.rs            # 基础栅格
│   │   ├── gutter.rs           # 栅格间隔
│   │   ├── offset.rs           # 栅格偏移
│   │   ├── responsive.rs       # 响应式
│   │   └── flex.rs             # Flex布局
│   ├── space/
│   │   └── mod.rs
│   └── layout/
│       └── mod.rs
├── navigation/                  # 导航组件
│   ├── mod.rs
│   ├── menu/
│   ├── breadcrumb/
│   └── pagination/
├── data_entry/                  # 数据录入组件
│   ├── mod.rs
│   ├── input/
│   ├── select/
│   └── form/
├── data_display/                # 数据展示组件
│   ├── mod.rs
│   ├── table/
│   ├── list/
│   └── card/
└── feedback/                    # 反馈组件
    ├── mod.rs
    ├── alert/
    ├── message/
    └── modal/
```

## 使用方式

### 1. 统一展示（推荐）

运行统一的组件展示页面，包含所有组件的演示和文档：

```bash
dx serve --example showcase
```

### 2. 独立组件演示

运行特定组件的独立演示：

```bash
# 按钮组件演示
dx serve --example general_button

# 图标组件演示
dx serve --example general_icon

# 栅格系统演示
dx serve --example layout_grid
```

### 3. 特定功能演示

运行组件的特定功能演示：

```bash
# 按钮组演示
dx serve --example general_button_group

# 栅格响应式演示
dx serve --example layout_grid_responsive
```

## 开发指南

### 添加新组件示例

1. 在对应分类目录下创建组件文件夹
2. 创建 `mod.rs` 文件定义组件模块
3. 按功能创建具体的演示文件
4. 在 `showcase.rs` 中添加组件展示
5. 更新对应的 `mod.rs` 文件导出新组件

### 示例文件命名规范

- 使用小写字母和下划线
- 按功能命名：`basic.rs`、`size.rs`、`type.rs` 等
- 独立运行的示例：`{category}_{component}_{feature}.rs`

### 代码规范

- 每个示例文件都应该是可独立运行的
- 包含完整的文档注释
- 遵循项目的代码风格
- 添加必要的依赖引用

## 迁移说明

从旧的扁平结构迁移到新的分层结构：

- `button_demo.rs` → `general/button/mod.rs` + 独立示例文件
- `button_group_demo.rs` → `general/button/group.rs`
- `icon_demo.rs` → `general/icon/mod.rs` + 独立示例文件
- `typography_demo.rs` → `general/typography/mod.rs` + 独立示例文件
- `grid_demo.rs` → `layout/grid/mod.rs` + 独立示例文件
- `grid_demo_simple.rs` → `layout/grid/basic.rs`
- `showcase.rs` → 保留并增强功能

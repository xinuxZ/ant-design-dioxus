<div align="center">
  <img height="180" src="https://gw.alipayobjects.com/zos/rmsportal/KDpgvguMpGfqaHPjicRK.svg">
  <h1>Ant Design Dioxus</h1>
  <p>企业级 UI 设计语言和 Dioxus 组件库实现</p>
</div>

[![Crates.io](https://img.shields.io/crates/v/ant-design-dioxus.svg)](https://crates.io/crates/ant-design-dioxus)
[![Documentation](https://docs.rs/ant-design-dioxus/badge.svg)](https://docs.rs/ant-design-dioxus)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org)
[![Dioxus](https://img.shields.io/badge/dioxus-0.6.3-green.svg)](https://dioxuslabs.com)

[English](./README.md) · [中文](./README-zh_CN.md)

## ✨ 特性

- 🌈 **企业级设计语言**：专为 Web 应用程序设计的企业级 UI 设计语言
- 📦 **开箱即用**：60+ 个高质量 Dioxus 组件，覆盖所有常见使用场景
- 🛡 **类型安全**：使用 Rust 编写，具有可预测的静态类型检查
- ⚙️ **完整工具链**：完整的设计资源和开发工具包
- 🌍 **国际化**：内置国际化支持，支持数十种语言
- 🎨 **主题定制**：基于 CSS-in-Rust 的强大主题定制能力
- 🚀 **高性能**：基于 Rust 和 WebAssembly 的高性能实现
- 📱 **响应式设计**：移动端友好的响应式设计
- ♿ **无障碍访问**：完善的可访问性支持

## 🏗️ 架构特色

### 完整的组件生态系统

**布局组件**
- Grid 栅格系统、Layout 布局、Space 间距、Divider 分割线、Flex 弹性布局

**通用组件**
- Button 按钮、Icon 图标、Typography 排版

**导航组件**
- Affix 固钉、Breadcrumb 面包屑、Dropdown 下拉菜单、Menu 导航菜单、Pagination 分页、Steps 步骤条

**数据录入**
- AutoComplete 自动完成、Cascader 级联选择、Checkbox 多选框、DatePicker 日期选择器、Form 表单、Input 输入框、InputNumber 数字输入框、Mentions 提及、Radio 单选框、Rate 评分、Select 选择器、Slider 滑动输入条、Switch 开关、TimePicker 时间选择器、Transfer 穿梭框、TreeSelect 树选择、Upload 上传

**数据展示**
- Avatar 头像、Badge 徽标数、Calendar 日历、Card 卡片、Carousel 走马灯、Collapse 折叠面板、Descriptions 描述列表、Empty 空状态、Image 图片、List 列表、Popover 气泡卡片、Segmented 分段控制器、Statistic 统计数值、Table 表格、Tabs 标签页、Tag 标签、Timeline 时间轴、Tooltip 文字提示、Tour 漫游式引导、Tree 树形控件

**反馈组件**
- Alert 警告提示、Drawer 抽屉、Message 全局提示、Modal 对话框、Notification 通知提醒框、Popconfirm 气泡确认框、Progress 进度条、Result 结果、Skeleton 骨架屏、Spin 加载中

**其他组件**
- Anchor 锚点、BackTop 回到顶部、ConfigProvider 全局化配置、FloatButton 悬浮按钮、QRCode 二维码、Watermark 水印

### 先进的技术架构

- **模块化设计**：每个组件独立封装，支持按需引入
- **主题系统**：支持亮色/暗色主题，完全可定制的设计令牌
- **响应式系统**：内置断点系统，支持多设备适配
- **状态管理**：基于 Dioxus 的响应式状态管理
- **样式方案**：CSS-in-Rust 实现，零运行时开销

## 🖥 环境支持

- 现代浏览器
- 服务端渲染 (SSR)
- WebAssembly
- 桌面应用程序 (通过 Tauri)
- 移动应用程序 (通过 Capacitor)

| [<img src="https://raw.githubusercontent.com/alrra/browser-logos/master/src/edge/edge_48x48.png" alt="Edge" width="24px" height="24px" />](http://godban.github.io/browsers-support-badges/)<br/>Edge | [<img src="https://raw.githubusercontent.com/alrra/browser-logos/master/src/firefox/firefox_48x48.png" alt="Firefox" width="24px" height="24px" />](http://godban.github.io/browsers-support-badges/)<br/>Firefox | [<img src="https://raw.githubusercontent.com/alrra/browser-logos/master/src/chrome/chrome_48x48.png" alt="Chrome" width="24px" height="24px" />](http://godban.github.io/browsers-support-badges/)<br/>Chrome | [<img src="https://raw.githubusercontent.com/alrra/browser-logos/master/src/safari/safari_48x48.png" alt="Safari" width="24px" height="24px" />](http://godban.github.io/browsers-support-badges/)<br/>Safari |
| --- | --- | --- | --- |
| Edge | 最新 2 个版本 | 最新 2 个版本 | 最新 2 个版本 |

## 📦 安装

在你的 `Cargo.toml` 中添加：

```toml
[dependencies]
ant-design-dioxus = "0.1"
dioxus = "0.6.3"
```

## 🔨 使用方法

### 基础使用

```rust
use dioxus::prelude::*;
use ant_design_dioxus::prelude::*;

fn App() -> Element {
    rsx! {
        ConfigProvider {
            Button {
                button_type: ButtonType::Primary,
                "Hello Ant Design Dioxus!"
            }
            DatePicker {
                placeholder: "选择日期"
            }
        }
    }
}

fn main() {
    dioxus::launch(App);
}
```

### 表单示例

```rust
use dioxus::prelude::*;
use ant_design_dioxus::prelude::*;

fn LoginForm() -> Element {
    let mut username = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());

    rsx! {
        Form {
            layout: FormLayout::Vertical,

            FormItem {
                label: "用户名",
                Input {
                    placeholder: "请输入用户名",
                    value: username.read().clone(),
                    oninput: move |e| username.set(e.value().clone())
                }
            }

            FormItem {
                label: "密码",
                Input {
                    input_type: InputType::Password,
                    placeholder: "请输入密码",
                    value: password.read().clone(),
                    oninput: move |e| password.set(e.value().clone())
                }
            }

            FormItem {
                Button {
                    button_type: ButtonType::Primary,
                    html_type: "submit",
                    "登录"
                }
            }
        }
    }
}
```

### 数据展示示例

```rust
use dioxus::prelude::*;
use ant_design_dioxus::prelude::*;

fn DataDisplay() -> Element {
    rsx! {
        Space {
            direction: SpaceDirection::Vertical,
            size: SpaceSize::Large,

            Card {
                title: "用户统计",
                Statistic {
                    title: "活跃用户",
                    value: 1128,
                    suffix: "人"
                }
            }

            Table {
                columns: vec![
                    TableColumn::new("name", "姓名"),
                    TableColumn::new("age", "年龄"),
                    TableColumn::new("address", "地址"),
                ],
                data_source: vec![
                    // 表格数据
                ]
            }
        }
    }
}
```

## 🎨 主题定制

```rust
use ant_design_dioxus::prelude::*;

fn App() -> Element {
    rsx! {
        ConfigProvider {
            theme: Theme {
                token: ThemeToken {
                    color_primary: "#1890ff".to_string(),
                    border_radius: 6,
                    ..Default::default()
                },
                algorithm: vec![ThemeAlgorithm::Dark]
            },

            // 你的应用组件
            MyApp {}
        }
    }
}
```

## 🌍 国际化

```rust
use ant_design_dioxus::prelude::*;

fn App() -> Element {
    rsx! {
        ConfigProvider {
            locale: Locale::ZhCN,

            DatePicker {
                placeholder: "请选择日期"
            }
        }
    }
}
```

## 🔗 相关链接

- [官方网站](https://ant-design-dioxus.rs)
- [组件文档](https://ant-design-dioxus.rs/components/overview)
- [Ant Design](https://ant.design) - 原始 React 版本
- [Dioxus](https://dioxuslabs.com) - 我们基于的 Rust 框架
- [示例项目](./examples) - 完整的使用示例

## ⌨️ 本地开发

```bash
# 克隆仓库
git clone https://github.com/ant-design/ant-design-dioxus.git
cd ant-design-dioxus

# 运行示例
dx serve --example main
```

## 📋 开发路线图

- [x] **阶段一**：项目基础设施搭建
- [x] **阶段二**：基础组件实现（布局、通用组件）
- [x] **阶段三**：数据展示组件
- [x] **阶段四**：数据录入组件
- [x] **阶段五**：导航组件
- [x] **阶段六**：反馈组件
- [x] **阶段七**：其他组件
- [ ] **阶段八**：组件细节完善和 Ant Design 对齐
  - [ ] Button 组件增强（加载状态、幽灵按钮、危险类型）
  - [ ] Form 组件验证和布局改进
  - [ ] Table 组件排序、筛选和分页功能
  - [ ] DatePicker 组件范围选择和本地化支持
  - [ ] Input 组件前缀/后缀图标和验证状态
  - [ ] Select 组件搜索、多选和自定义渲染
  - [ ] Modal 组件可拖拽、可调整大小和嵌套模态框
  - [ ] Menu 组件子菜单、面包屑集成
  - [ ] Tree 组件拖拽和虚拟滚动
  - [ ] Upload 组件拖拽上传和进度跟踪
- [ ] **阶段九**：高级功能和性能优化
  - [ ] 大数据集虚拟滚动
  - [ ] SSR（服务端渲染）支持
  - [ ] 包体积优化
  - [ ] 性能基准测试
- [ ] **阶段十**：文档和测试完善
  - [ ] 全面的组件文档
  - [ ] 交互式演示平台
  - [ ] 单元测试和集成测试
  - [ ] 无障碍访问测试
- [ ] **阶段十一**：发布和社区建设
  - [ ] 稳定版 API 发布
  - [ ] 迁移指南
  - [ ] 社区示例和模板

查看详细的 [开发任务清单](./docs/task.md)

## 🤝 参与贡献

我们欢迎所有形式的贡献。请先阅读我们的 [贡献指南](./CONTRIBUTING.md)。

### 贡献方式

- 🐛 报告 Bug
- 💡 提出新功能建议
- 📝 改进文档
- 🔧 提交代码
- 🎨 设计改进

### 开发者

感谢所有为这个项目做出贡献的开发者！

## 📄 许可证

MIT 许可证。详情请查看 [LICENSE](./LICENSE) 文件。

## 🙏 致谢

- [Ant Design](https://ant.design) - 原始设计系统
- [Dioxus](https://dioxuslabs.com) - 优秀的 Rust Web 框架
- 所有让这个项目成为可能的贡献者们

---

<div align="center">
  <sub>使用 ❤️ 和 🦀 构建</sub>
</div>

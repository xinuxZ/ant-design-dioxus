<div align="center">
  <img height="180" src="https://gw.alipayobjects.com/zos/rmsportal/KDpgvguMpGfqaHPjicRK.svg">
  <h1>Ant Design Dioxus</h1>
  <p>An enterprise-class UI design language and Dioxus components implementation</p>
</div>

[![Crates.io](https://img.shields.io/crates/v/ant-design-dioxus.svg)](https://crates.io/crates/ant-design-dioxus)
[![Documentation](https://docs.rs/ant-design-dioxus/badge.svg)](https://docs.rs/ant-design-dioxus)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org)
[![Dioxus](https://img.shields.io/badge/dioxus-0.6.3-green.svg)](https://dioxuslabs.com)

[English](./README.md) · [中文](./README-zh_CN.md)

## ✨ Features

- 🌈 **Enterprise-class Design Language**: An enterprise-class UI design language for web applications
- 📦 **Out of the Box**: 60+ high-quality Dioxus components covering all common use cases
- 🛡 **Type Safety**: Written in Rust with predictable static type checking
- ⚙️ **Complete Toolchain**: Whole package of design resources and development tools
- 🌍 **Internationalization**: Built-in i18n support for dozens of languages
- 🎨 **Theme Customization**: Powerful theme customization based on CSS-in-Rust
- 🚀 **High Performance**: High-performance implementation based on Rust and WebAssembly
- 📱 **Responsive Design**: Mobile-friendly responsive design
- ♿ **Accessibility**: Comprehensive accessibility support

## 🏗️ Architecture Highlights

### Complete Component Ecosystem

**Layout Components**
- Grid System, Layout, Space, Divider, Flex

**General Components**
- Button, Icon, Typography

**Navigation Components**
- Affix, Breadcrumb, Dropdown, Menu, Pagination, Steps

**Data Entry**
- AutoComplete, Cascader, Checkbox, DatePicker, Form, Input, InputNumber, Mentions, Radio, Rate, Select, Slider, Switch, TimePicker, Transfer, TreeSelect, Upload

**Data Display**
- Avatar, Badge, Calendar, Card, Carousel, Collapse, Descriptions, Empty, Image, List, Popover, Segmented, Statistic, Table, Tabs, Tag, Timeline, Tooltip, Tour, Tree

**Feedback Components**
- Alert, Drawer, Message, Modal, Notification, Popconfirm, Progress, Result, Skeleton, Spin

**Other Components**
- Anchor, BackTop, ConfigProvider, FloatButton, QRCode, Watermark

### Advanced Technical Architecture

- **Modular Design**: Each component is independently encapsulated with tree-shaking support
- **Theme System**: Support for light/dark themes with fully customizable design tokens
- **Responsive System**: Built-in breakpoint system supporting multi-device adaptation
- **State Management**: Reactive state management based on Dioxus
- **Styling Solution**: CSS-in-Rust implementation with zero runtime overhead

## 🖥 Environment Support

- Modern browsers
- Server-side Rendering (SSR)
- WebAssembly
- Desktop applications (via Tauri)
- Mobile applications (via Capacitor)

| [<img src="https://raw.githubusercontent.com/alrra/browser-logos/master/src/edge/edge_48x48.png" alt="Edge" width="24px" height="24px" />](http://godban.github.io/browsers-support-badges/)<br/>Edge | [<img src="https://raw.githubusercontent.com/alrra/browser-logos/master/src/firefox/firefox_48x48.png" alt="Firefox" width="24px" height="24px" />](http://godban.github.io/browsers-support-badges/)<br/>Firefox | [<img src="https://raw.githubusercontent.com/alrra/browser-logos/master/src/chrome/chrome_48x48.png" alt="Chrome" width="24px" height="24px" />](http://godban.github.io/browsers-support-badges/)<br/>Chrome | [<img src="https://raw.githubusercontent.com/alrra/browser-logos/master/src/safari/safari_48x48.png" alt="Safari" width="24px" height="24px" />](http://godban.github.io/browsers-support-badges/)<br/>Safari |
| --- | --- | --- | --- |
| Edge | last 2 versions | last 2 versions | last 2 versions |

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
ant-design-dioxus = "0.1"
dioxus = "0.6.3"
```

## 🔨 Usage

### Basic Usage

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
                placeholder: "Select date"
            }
        }
    }
}

fn main() {
    dioxus::launch(App);
}
```

### Form Example

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
                label: "Username",
                Input {
                    placeholder: "Enter username",
                    value: username.read().clone(),
                    oninput: move |e| username.set(e.value().clone())
                }
            }

            FormItem {
                label: "Password",
                Input {
                    input_type: InputType::Password,
                    placeholder: "Enter password",
                    value: password.read().clone(),
                    oninput: move |e| password.set(e.value().clone())
                }
            }

            FormItem {
                Button {
                    button_type: ButtonType::Primary,
                    html_type: "submit",
                    "Login"
                }
            }
        }
    }
}
```

### Data Display Example

```rust
use dioxus::prelude::*;
use ant_design_dioxus::prelude::*;

fn DataDisplay() -> Element {
    rsx! {
        Space {
            direction: SpaceDirection::Vertical,
            size: SpaceSize::Large,

            Card {
                title: "User Statistics",
                Statistic {
                    title: "Active Users",
                    value: 1128,
                    suffix: "users"
                }
            }

            Table {
                columns: vec![
                    TableColumn::new("name", "Name"),
                    TableColumn::new("age", "Age"),
                    TableColumn::new("address", "Address"),
                ],
                data_source: vec![
                    // Table data
                ]
            }
        }
    }
}
```

## 🎨 Theme Customization

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

            // Your app components
            MyApp {}
        }
    }
}
```

## 🌍 Internationalization

```rust
use ant_design_dioxus::prelude::*;

fn App() -> Element {
    rsx! {
        ConfigProvider {
            locale: Locale::EnUS,

            DatePicker {
                placeholder: "Select date"
            }
        }
    }
}
```

## 🔗 Links

- [Homepage](https://ant-design-dioxus.rs)
- [Components Documentation](https://ant-design-dioxus.rs/components/overview)
- [Ant Design](https://ant.design) - Original React version
- [Dioxus](https://dioxuslabs.com) - The Rust framework we're built on
- [Examples](./examples) - Complete usage examples

## ⌨️ Development

```bash
# Clone the repository
git clone https://github.com/ant-design/ant-design-dioxus.git
cd ant-design-dioxus

# Run examples
dx serve --example main

```

## 📋 Roadmap

- [x] **Phase 1**: Project infrastructure setup
- [x] **Phase 2**: Basic components implementation (Layout, General)
- [x] **Phase 3**: Data display components
- [x] **Phase 4**: Data entry components
- [x] **Phase 5**: Navigation components
- [x] **Phase 6**: Feedback components
- [x] **Phase 7**: Other components
- [ ] **Phase 8**: Component refinement and Ant Design alignment
  - [ ] Button component enhancement (loading states, ghost style, danger type)
  - [ ] Form component validation and layout improvements
  - [ ] Table component sorting, filtering, and pagination
  - [ ] DatePicker component range selection and locale support
  - [ ] Input component prefix/suffix icons and validation states
  - [ ] Select component search, multiple selection, and custom rendering
  - [ ] Modal component draggable, resizable, and nested modals
  - [ ] Menu component submenu, breadcrumb integration
  - [ ] Tree component drag & drop, virtual scrolling
  - [ ] Upload component drag & drop, progress tracking
- [ ] **Phase 9**: Advanced features and performance optimization
  - [ ] Virtual scrolling for large datasets
  - [ ] SSR (Server-Side Rendering) support
  - [ ] Bundle size optimization
  - [ ] Performance benchmarking
- [ ] **Phase 10**: Documentation and testing improvement
  - [ ] Comprehensive component documentation
  - [ ] Interactive playground
  - [ ] Unit and integration tests
  - [ ] Accessibility testing
- [ ] **Phase 11**: Release and community building
  - [ ] Stable API release
  - [ ] Migration guides
  - [ ] Community examples and templates

See detailed [Development Task List](./docs/task.md)

## 🤝 Contributing

We welcome all contributions. Please read our [Contributing Guide](./CONTRIBUTING.md) first.

### Ways to Contribute

- 🐛 Report bugs
- 💡 Suggest new features
- 📝 Improve documentation
- 🔧 Submit code
- 🎨 Design improvements

### Contributors

Thanks to all the contributors who make this project possible!

## 📄 License

MIT License. See [LICENSE](./LICENSE) for details.

## 🙏 Acknowledgments

- [Ant Design](https://ant.design) - The original design system
- [Dioxus](https://dioxuslabs.com) - The amazing Rust web framework
- All the contributors who make this project possible

---

<div align="center">
  <sub>Built with ❤️ and 🦀</sub>
</div>

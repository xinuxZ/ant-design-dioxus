# Ant Design Dioxus

<div align="center">
  <img height="180" src="https://gw.alipayobjects.com/zos/rmsportal/KDpgvguMpGfqaHPjicRK.svg">
  <h1>Ant Design Dioxus</h1>
  <p>An enterprise-class UI design language and Dioxus components implementation</p>
</div>

[![Crates.io](https://img.shields.io/crates/v/ant-design-dioxus.svg)](https://crates.io/crates/ant-design-dioxus)
[![Documentation](https://docs.rs/ant-design-dioxus/badge.svg)](https://docs.rs/ant-design-dioxus)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[English](./README.md) · [中文](./README-zh_CN.md)

## ✨ Features

- 🌈 Enterprise-class UI designed for web applications
- 📦 A set of high-quality Dioxus components out of the box
- 🛡 Written in Rust with predictable static types
- ⚙️ Whole package of design resources and development tools
- 🌍 Internationalization support for dozens of languages
- 🎨 Powerful theme customization based on CSS-in-Rust
- 🚀 High performance with Rust and WebAssembly

## 🖥 Environment Support

- Modern browsers
- Server-side Rendering
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
dioxus = "0.5"
dioxus-web = "0.5"
```

## 🔨 Usage

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
    dioxus_web::launch(App);
}
```

## 🔗 Links

- [Homepage](https://ant-design-dioxus.rs)
- [Components Overview](https://ant-design-dioxus.rs/components/overview)
- [Ant Design](https://ant.design) - Original React version
- [Dioxus](https://dioxuslabs.com) - The Rust framework we're built on

## ⌨️ Development

```bash
# Clone the repository
git clone https://github.com/ant-design/ant-design-dioxus.git
cd ant-design-dioxus

# Install dependencies
cargo build

# Run examples
cargo run --example button

# Run tests
cargo test

# Build documentation
cargo doc --open
```

## 🤝 Contributing

We welcome all contributions. Please read our [Contributing Guide](./CONTRIBUTING.md) first.

## 📄 License

MIT License. See [LICENSE](./LICENSE) for details.

## 🙏 Acknowledgments

- [Ant Design](https://ant.design) - The original design system
- [Dioxus](https://dioxuslabs.com) - The amazing Rust web framework
- All the contributors who make this project possible

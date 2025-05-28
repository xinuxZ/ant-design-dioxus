//! Ant Design Dioxus 组件库展示示例
//! 参照 Ant Design 官方文档格式，左侧展示组件列表，右侧展示组件使用示例和参数说明

use ant_design_dioxus::prelude::*;
use dioxus as _;
use once_cell as _;
use serde as _;
use serde_json as _;
use wasm_bindgen_test as _;
use web_sys as _;

/// 主应用组件
#[component]
fn App() -> Element {
    let mut selected_component = use_signal(|| "Button".to_string());

    rsx! {
        div {
            class: "showcase-container",
            style: "display: flex; min-height: 100vh; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;",

            // 左侧导航栏
            div {
                class: "sidebar",
                style: "width: 240px; background: #f5f5f5; border-right: 1px solid #d9d9d9; padding: 16px; overflow-y: auto;",

                h2 {
                    style: "margin: 0 0 16px 0; color: #1890ff; font-size: 18px;",
                    "Ant Design Dioxus"
                }

                ComponentMenu {
                    selected: selected_component.read().clone(),
                    on_select: move |component: String| {
                        selected_component.set(component);
                    }
                }
            }

            // 右侧内容区域
            div {
                class: "content",
                style: "flex: 1; padding: 24px; overflow-y: auto;",

                match selected_component.read().as_str() {
                    "Button" => rsx! { ButtonShowcase {} },
                    "Icon" => rsx! { IconShowcase {} },
                    "Typography" => rsx! { TypographyShowcase {} },
                    _ => rsx! { div { "组件开发中..." } }
                }
            }
        }
    }
}

/// 组件菜单
#[component]
fn ComponentMenu(selected: String, on_select: EventHandler<String>) -> Element {
    let components = vec![
        ("通用", vec!["Button", "Icon", "Typography"]),
        ("布局", vec!["Grid", "Layout", "Space"]),
        ("导航", vec!["Menu", "Breadcrumb", "Pagination"]),
        ("数据录入", vec!["Input", "Select", "Form"]),
        ("数据展示", vec!["Table", "List", "Card"]),
        ("反馈", vec!["Alert", "Message", "Modal"]),
    ];

    rsx! {
        div {
            class: "component-menu",

            for (category, items) in components {
                div {
                    class: "menu-category",
                    style: "margin-bottom: 16px;",

                    h4 {
                        style: "margin: 0 0 8px 0; color: #666; font-size: 12px; text-transform: uppercase;",
                        "{category}"
                    }

                    for item in items {
                        div {
                            class: "menu-item",
                            style: format!(
                                "padding: 8px 12px; cursor: pointer; border-radius: 4px; margin-bottom: 2px; transition: background-color 0.2s; {}",
                                if *item == selected { "background: #1890ff; color: white;" } else { "color: #333; hover:background: #e6f7ff;" }
                            ),
                            onclick: move |_| {
                                on_select.call(item.to_string());
                            },
                            "{item}"
                        }
                    }
                }
            }
        }
    }
}

/// Button 组件展示
#[component]
fn ButtonShowcase() -> Element {
    rsx! {
        div {
            class: "component-showcase",

            h1 {
                style: "margin: 0 0 16px 0; color: #262626; font-size: 28px;",
                "Button 按钮"
            }

            p {
                style: "margin: 0 0 24px 0; color: #666; font-size: 14px;",
                "按钮用于开始一个即时操作。"
            }

            // 基础用法
            DemoSection {
                title: "基础用法",
                description: "按钮有五种类型：主按钮、次按钮、虚线按钮、文本按钮和链接按钮。",

                div {
                    style: "display: flex; gap: 8px; flex-wrap: wrap;",

                    Button {
                        button_type: ButtonType::Primary,
                        "Primary Button"
                    }

                    Button {
                        button_type: ButtonType::Default,
                        "Default Button"
                    }

                    Button {
                        button_type: ButtonType::Dashed,
                        "Dashed Button"
                    }

                    Button {
                        button_type: ButtonType::Text,
                        "Text Button"
                    }

                    Button {
                        button_type: ButtonType::Link,
                        "Link Button"
                    }
                }
            }

            // 不同尺寸
            DemoSection {
                title: "三种尺寸",
                description: "按钮有大、中、小三种尺寸。",

                div {
                    style: "display: flex; gap: 8px; align-items: center;",

                    Button {
                        button_type: ButtonType::Primary,
                        size: ButtonSize::Large,
                        "Large"
                    }

                    Button {
                        button_type: ButtonType::Primary,
                        size: ButtonSize::Middle,
                        "Middle"
                    }

                    Button {
                        button_type: ButtonType::Primary,
                        size: ButtonSize::Small,
                        "Small"
                    }
                }
            }

            // 不可用状态
            DemoSection {
                title: "不可用状态",
                description: "添加 disabled 属性即可让按钮处于不可用状态。",

                div {
                    style: "display: flex; gap: 8px;",

                    Button {
                        button_type: ButtonType::Primary,
                        disabled: true,
                        "Primary(disabled)"
                    }

                    Button {
                        button_type: ButtonType::Default,
                        disabled: true,
                        "Default(disabled)"
                    }
                }
            }

            // 危险按钮
            DemoSection {
                title: "危险按钮",
                description: "删除/移动/修改权限等危险操作，一般需要二次确认。",

                div {
                    style: "display: flex; gap: 8px;",

                    Button {
                        button_type: ButtonType::Primary,
                        danger: true,
                        "Primary Danger"
                    }

                    Button {
                        button_type: ButtonType::Default,
                        danger: true,
                        "Default Danger"
                    }
                }
            }

            // API 文档
            ApiDocumentation {
                component_name: "Button",
                props: vec![
                    PropDoc {
                        name: "button_type".to_string(),
                        prop_type: "ButtonType".to_string(),
                        default: "ButtonType::Default".to_string(),
                        description: "设置按钮类型".to_string(),
                    },
                    PropDoc {
                        name: "size".to_string(),
                        prop_type: "ButtonSize".to_string(),
                        default: "ButtonSize::Middle".to_string(),
                        description: "设置按钮大小".to_string(),
                    },
                    PropDoc {
                        name: "disabled".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "按钮失效状态".to_string(),
                    },
                    PropDoc {
                        name: "danger".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "设置危险按钮".to_string(),
                    },
                    PropDoc {
                        name: "loading".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "设置按钮载入状态".to_string(),
                    },
                    PropDoc {
                        name: "block".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "将按钮宽度调整为其父宽度的选项".to_string(),
                    },
                ]
            }
        }
    }
}

/// Icon 组件展示
#[component]
fn IconShowcase() -> Element {
    rsx! {
        div {
            class: "component-showcase",

            h1 {
                style: "margin: 0 0 16px 0; color: #262626; font-size: 28px;",
                "Icon 图标"
            }

            p {
                style: "margin: 0 0 24px 0; color: #666; font-size: 14px;",
                "语义化的矢量图形。使用图标组件，你需要安装 @ant-design/icons 图标组件包。"
            }

            // 基础用法
            DemoSection {
                title: "基础用法",
                description: "最简单的用法。",

                div {
                    style: "display: flex; gap: 16px; align-items: center;",

                    Icon {
                        icon_type: "home",
                        theme: IconTheme::Outlined,
                    }

                    Icon {
                        icon_type: "setting",
                        theme: IconTheme::Filled,
                    }

                    Icon {
                        icon_type: "smile",
                        theme: IconTheme::TwoTone,
                    }
                }
            }

            // 不同主题
            DemoSection {
                title: "图标主题",
                description: "图标有三种主题：线性、实心、双色。",

                div {
                    style: "display: flex; gap: 24px;",

                    div {
                        style: "text-align: center;",
                        Icon {
                            icon_type: "home",
                            theme: IconTheme::Outlined,
                            style: "font-size: 24px;"
                        }
                        div { style: "margin-top: 8px; font-size: 12px;", "Outlined" }
                    }

                    div {
                        style: "text-align: center;",
                        Icon {
                            icon_type: "home",
                            theme: IconTheme::Filled,
                            style: "font-size: 24px;"
                        }
                        div { style: "margin-top: 8px; font-size: 12px;", "Filled" }
                    }

                    div {
                        style: "text-align: center;",
                        Icon {
                            icon_type: "home",
                            theme: IconTheme::TwoTone,
                            style: "font-size: 24px;"
                        }
                        div { style: "margin-top: 8px; font-size: 12px;", "TwoTone" }
                    }
                }
            }

            // 旋转
            DemoSection {
                title: "旋转角度",
                description: "图标可以旋转任意角度。",

                div {
                    style: "display: flex; gap: 16px; align-items: center;",

                    Icon {
                        icon_type: "reload",
                        rotate: IconRotate::Rotate90,
                        style: "font-size: 24px;"
                    }

                    Icon {
                        icon_type: "reload",
                        rotate: IconRotate::Rotate180,
                        style: "font-size: 24px;"
                    }

                    Icon {
                        icon_type: "reload",
                        rotate: IconRotate::Rotate270,
                        style: "font-size: 24px;"
                    }
                }
            }

            // API 文档
            ApiDocumentation {
                component_name: "Icon",
                props: vec![
                    PropDoc {
                        name: "icon_type".to_string(),
                        prop_type: "&str".to_string(),
                        default: "-".to_string(),
                        description: "图标类型".to_string(),
                    },
                    PropDoc {
                        name: "theme".to_string(),
                        prop_type: "IconTheme".to_string(),
                        default: "IconTheme::Outlined".to_string(),
                        description: "图标主题风格".to_string(),
                    },
                    PropDoc {
                        name: "rotate".to_string(),
                        prop_type: "IconRotate".to_string(),
                        default: "IconRotate::None".to_string(),
                        description: "图标旋转角度".to_string(),
                    },
                    PropDoc {
                        name: "style".to_string(),
                        prop_type: "&str".to_string(),
                        default: "\"\"".to_string(),
                        description: "设置图标的样式，例如 fontSize 和 color".to_string(),
                    },
                ]
            }
        }
    }
}

/// Typography 组件展示
#[component]
fn TypographyShowcase() -> Element {
    rsx! {
        div {
            class: "component-showcase",

            h1 {
                style: "margin: 0 0 16px 0; color: #262626; font-size: 28px;",
                "Typography 排版"
            }

            p {
                style: "margin: 0 0 24px 0; color: #666; font-size: 14px;",
                "文本的基本格式。"
            }

            // 标题
            DemoSection {
                title: "标题组件",
                description: "展示不同级别的标题。",

                div {
                    style: "display: flex; flex-direction: column; gap: 8px;",

                    Title { level: HeadingLevel::H1, "h1. Ant Design" }
                    Title { level: HeadingLevel::H2, "h2. Ant Design" }
                    Title { level: HeadingLevel::H3, "h3. Ant Design" }
                    Title { level: HeadingLevel::H4, "h4. Ant Design" }
                    Title { level: HeadingLevel::H5, "h5. Ant Design" }
                    // Note: HeadingLevel only supports H1-H5
                    // Title { level: HeadingLevel::H6, "h6. Ant Design" }
                }
            }

            // 文本
            DemoSection {
                title: "文本组件",
                description: "内置不同样式的文本。",

                div {
                    style: "display: flex; flex-direction: column; gap: 8px;",

                    Text { "Ant Design (default)" }
                    Text { text_type: TextType::Secondary, "Ant Design (secondary)" }
                    Text { text_type: TextType::Success, "Ant Design (success)" }
                    Text { text_type: TextType::Warning, "Ant Design (warning)" }
                    Text { text_type: TextType::Danger, "Ant Design (danger)" }
                    Text { disabled: true, "Ant Design (disabled)" }
                }
            }

            // 文本修饰
            DemoSection {
                title: "文本修饰",
                description: "提供加粗、斜体、下划线、删除线、代码等文本修饰。",

                div {
                    style: "display: flex; flex-direction: column; gap: 8px;",

                    Text { strong: true, "Ant Design (strong)" }
                    Text { italic: true, "Ant Design (italic)" }
                    Text { underline: true, "Ant Design (underline)" }
                    Text { delete: true, "Ant Design (delete)" }
                    Text { code: true, "Ant Design (code)" }
                }
            }

            // 段落
            DemoSection {
                title: "段落组件",
                description: "段落组件。",

                Paragraph {
                    "在中台产品的研发过程中，会出现不同的设计规范和实现方式，但其中往往存在很多类似的页面和组件，给设计师和开发者带来很多困扰和重复建设，大大降低了产品的研发效率。"
                }
            }

            // API 文档
            ApiDocumentation {
                component_name: "Typography",
                props: vec![
                    PropDoc {
                        name: "level".to_string(),
                        prop_type: "HeadingLevel".to_string(),
                        default: "HeadingLevel::H1".to_string(),
                        description: "重要程度，相当于 h1、h2、h3、h4、h5、h6".to_string(),
                    },
                    PropDoc {
                        name: "text_type".to_string(),
                        prop_type: "TextType".to_string(),
                        default: "TextType::Default".to_string(),
                        description: "文本类型".to_string(),
                    },
                    PropDoc {
                        name: "disabled".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "禁用文本".to_string(),
                    },
                    PropDoc {
                        name: "strong".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "是否加粗".to_string(),
                    },
                    PropDoc {
                        name: "italic".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "是否斜体".to_string(),
                    },
                    PropDoc {
                        name: "underline".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "是否下划线".to_string(),
                    },
                    PropDoc {
                        name: "delete".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "是否删除线".to_string(),
                    },
                    PropDoc {
                        name: "code".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "是否代码样式".to_string(),
                    },
                ]
            }
        }
    }
}

/// 演示区域组件
#[component]
fn DemoSection(title: String, description: String, children: Element) -> Element {
    rsx! {
        div {
            class: "demo-section",
            style: "margin-bottom: 32px; border: 1px solid #f0f0f0; border-radius: 6px;",

            div {
                class: "demo-content",
                style: "padding: 24px;",
                {children}
            }

            div {
                class: "demo-meta",
                style: "border-top: 1px solid #f0f0f0; padding: 16px 24px; background: #fafafa;",

                h4 {
                    style: "margin: 0 0 8px 0; color: #262626; font-size: 14px; font-weight: 500;",
                    "{title}"
                }

                p {
                    style: "margin: 0; color: #666; font-size: 12px; line-height: 1.5;",
                    "{description}"
                }
            }
        }
    }
}

/// 属性文档结构
#[derive(Clone, Debug, PartialEq)]
struct PropDoc {
    name: String,
    prop_type: String,
    default: String,
    description: String,
}

/// API 文档组件
#[component]
fn ApiDocumentation(component_name: String, props: Vec<PropDoc>) -> Element {
    rsx! {
        div {
            class: "api-documentation",
            style: "margin-top: 48px;",

            h2 {
                style: "margin: 0 0 16px 0; color: #262626; font-size: 20px;",
                "API"
            }

            h3 {
                style: "margin: 0 0 16px 0; color: #262626; font-size: 16px;",
                "{component_name}"
            }

            div {
                class: "api-table",
                style: "border: 1px solid #f0f0f0; border-radius: 6px; overflow: hidden;",

                div {
                    class: "table-header",
                    style: "display: grid; grid-template-columns: 1fr 1fr 1fr 2fr; background: #fafafa; border-bottom: 1px solid #f0f0f0;",

                    div { style: "padding: 12px 16px; font-weight: 500; color: #262626;", "参数" }
                    div { style: "padding: 12px 16px; font-weight: 500; color: #262626;", "类型" }
                    div { style: "padding: 12px 16px; font-weight: 500; color: #262626;", "默认值" }
                    div { style: "padding: 12px 16px; font-weight: 500; color: #262626;", "说明" }
                }

                for prop in props {
                    div {
                        class: "table-row",
                        style: "display: grid; grid-template-columns: 1fr 1fr 1fr 2fr; border-bottom: 1px solid #f0f0f0;",

                        div {
                            style: "padding: 12px 16px; color: #262626; font-family: 'SFMono-Regular', Consolas, 'Liberation Mono', Menlo, Courier, monospace;",
                            "{prop.name}"
                        }
                        div {
                            style: "padding: 12px 16px; color: #1890ff; font-family: 'SFMono-Regular', Consolas, 'Liberation Mono', Menlo, Courier, monospace;",
                            "{prop.prop_type}"
                        }
                        div {
                            style: "padding: 12px 16px; color: #666; font-family: 'SFMono-Regular', Consolas, 'Liberation Mono', Menlo, Courier, monospace;",
                            "{prop.default}"
                        }
                        div {
                            style: "padding: 12px 16px; color: #666;",
                            "{prop.description}"
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    dioxus::launch(App);
}

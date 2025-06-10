#![allow(non_snake_case)]
//!
//! 展示国际化和主题在组件内的使用方法

use crate::common::*;
use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

/// 国际化和主题组件演示
#[component]
pub fn I18nAndThemeDemo() -> Element {
    rsx! {
        div {
            class: "component-showcase",

            h1 {
                style: "margin: 0 0 16px 0; color: #262626; font-size: 28px;",
                "国际化和主题系统"
            }

            p {
                style: "margin: 0 0 24px 0; color: #666; font-size: 14px;",
                "演示如何在组件内部使用国际化和主题功能。"
            }

            // 配置提供者
            ConfigProvider {
                locale: Some(Locale::ZhCN),
                theme: Some(Theme::light()),
                direction: Some(Direction::Ltr),

                // 国际化控制
                DemoSection {
                    title: "国际化切换",
                    description: "通过ConfigProvider切换语言，组件会自动使用对应语言的文本。",

                    I18nSwitchDemo {}
                }

                // 主题控制
                DemoSection {
                    title: "主题切换",
                    description: "通过ConfigProvider切换主题，组件会自动使用对应主题的样式。",

                    ThemeSwitchDemo {}
                }

                // 组件内使用
                DemoSection {
                    title: "组件内部使用国际化和主题",
                    description: "自定义组件可以使用Hooks访问当前的语言和主题设置。",

                    ComponentDemo {}
                }
            }
        }
    }
}

/// 国际化切换示例
#[component]
fn I18nSwitchDemo() -> Element {
    let mut locale = use_signal(|| Locale::ZhCN);
    let translate = use_translate();

    // 获取当前区域的按钮文本
    let ok_text = translate("ok");
    let cancel_text = translate("cancel");
    let confirm_text = translate("confirm");

    rsx! {
        div {
            class: "demo-container",
            style: "display: flex; gap: 16px; flex-wrap: wrap; margin-bottom: 16px;",

            // 语言切换
            div {
                style: "margin-bottom: 16px;",

                // 中文按钮
                Button {
                    button_type: if locale() == Locale::ZhCN { ButtonType::Primary } else { ButtonType::Default },
                    onclick: move |_| locale.set(Locale::ZhCN),
                    "中文"
                }

                // 英文按钮
                Button {
                    button_type: if locale() == Locale::En { ButtonType::Primary } else { ButtonType::Default },
                    onclick: move |_| locale.set(Locale::En),
                    "English"
                }

                // 阿拉伯语按钮
                Button {
                    button_type: if locale() == Locale::Ar { ButtonType::Primary } else { ButtonType::Default },
                    onclick: move |_| locale.set(Locale::Ar),
                    "العربية"
                }
            }

            // 使用翻译的组件
            div {
                style: "border: 1px solid #eee; padding: 16px; border-radius: 8px; width: 100%;",

                // 使用LocaleProvider更改当前上下文的语言
                LocaleProvider {
                    locale: locale,
                    div {
                        h3 { "当前语言翻译示例" }

                        // 显示当前语言的按钮文本
                        div {
                            style: "display: flex; gap: 8px; margin-bottom: 16px;",

                            Button { "{ok_text}" }
                            Button { "{cancel_text}" }
                            Button { "{confirm_text}" }
                        }

                        // 模态框示例
                        ModalDemo {}
                    }
                }
            }
        }
    }
}

/// 模态框示例（使用国际化）
#[component]
fn ModalDemo() -> Element {
    let mut visible = use_signal(|| false);

    rsx! {
        div {
            Button {
                button_type: ButtonType::Primary,
                onclick: move |_| visible.set(true),
                "打开模态框"
            }

            Modal {
                open: visible(),
                title: Some("国际化模态框".to_string()),
                on_ok: move |_| visible.set(false),
                on_cancel: move |_| visible.set(false),

                "模态框内容会根据当前语言环境自动切换确认和取消按钮的文本。"
            }
        }
    }
}

/// 主题切换示例
#[component]
fn ThemeSwitchDemo() -> Element {
    let mut theme = use_signal(|| Theme::light());

    rsx! {
        div {
            class: "demo-container",
            style: "display: flex; gap: 16px; flex-wrap: wrap; margin-bottom: 16px;",

            // 主题切换
            div {
                style: "margin-bottom: 16px;",

                // 亮色主题按钮
                Button {
                    button_type: if theme().is_light() { ButtonType::Primary } else { ButtonType::Default },
                    onclick: move |_| theme.set(Theme::light()),
                    "亮色主题"
                }

                // 暗色主题按钮
                Button {
                    button_type: if theme().is_dark() { ButtonType::Primary } else { ButtonType::Default },
                    onclick: move |_| theme.set(Theme::dark()),
                    "暗色主题"
                }

                // 紧凑主题按钮
                Button {
                    button_type: if theme().is_compact() { ButtonType::Primary } else { ButtonType::Default },
                    onclick: move |_| theme.set(Theme::compact()),
                    "紧凑主题"
                }
            }

            // 使用主题的组件
            div {
                style: "border: 1px solid #eee; padding: 16px; border-radius: 8px; width: 100%;",

                // 使用ThemeProvider更改当前上下文的主题
                ThemeProvider {
                    theme: theme,
                    div {
                        h3 { "当前主题示例" }

                        // 显示不同类型的按钮
                        div {
                            style: "display: flex; gap: 8px; margin-bottom: 16px;",

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

                        // 使用ThemeTokenDemo显示当前主题的令牌
                        ThemeTokenDemo {}
                    }
                }
            }
        }
    }
}

/// 主题令牌演示
#[component]
fn ThemeTokenDemo() -> Element {
    // 使用主题令牌
    let primary_color = use_theme_token("colorPrimary");
    let success_color = use_theme_token("colorSuccess");
    let warning_color = use_theme_token("colorWarning");
    let error_color = use_theme_token("colorError");
    let font_size = use_theme_token("fontSize");
    let border_radius = use_theme_token("borderRadius");

    rsx! {
        div {
            style: "margin-top: 16px;",

            div {
                style: "display: flex; flex-direction: column; gap: 8px;",

                div {
                    style: "display: flex; align-items: center;",
                    span {
                        style: "width: 150px;",
                        "Primary Color:"
                    }
                    div {
                        style: "width: 20px; height: 20px; background-color: {primary_color}; border-radius: 4px; margin-right: 8px;"
                    }
                    span { "{primary_color}" }
                }

                div {
                    style: "display: flex; align-items: center;",
                    span {
                        style: "width: 150px;",
                        "Success Color:"
                    }
                    div {
                        style: "width: 20px; height: 20px; background-color: {success_color}; border-radius: 4px; margin-right: 8px;"
                    }
                    span { "{success_color}" }
                }

                div {
                    style: "display: flex; align-items: center;",
                    span {
                        style: "width: 150px;",
                        "Warning Color:"
                    }
                    div {
                        style: "width: 20px; height: 20px; background-color: {warning_color}; border-radius: 4px; margin-right: 8px;"
                    }
                    span { "{warning_color}" }
                }

                div {
                    style: "display: flex; align-items: center;",
                    span {
                        style: "width: 150px;",
                        "Error Color:"
                    }
                    div {
                        style: "width: 20px; height: 20px; background-color: {error_color}; border-radius: 4px; margin-right: 8px;"
                    }
                    span { "{error_color}" }
                }

                div {
                    style: "display: flex; align-items: center;",
                    span {
                        style: "width: 150px;",
                        "Font Size:"
                    }
                    span { "{font_size}" }
                }

                div {
                    style: "display: flex; align-items: center;",
                    span {
                        style: "width: 150px;",
                        "Border Radius:"
                    }
                    span { "{border_radius}" }
                }
            }
        }
    }
}

/// 自定义组件示例
#[component]
fn ComponentDemo() -> Element {
    rsx! {
        div {
            class: "demo-container",
            style: "display: flex; gap: 16px; flex-wrap: wrap; margin-bottom: 16px;",

            div {
                style: "border: 1px solid #eee; padding: 16px; border-radius: 8px; width: 100%;",

                h3 { "自定义组件" }
                p { "这个组件内部使用了国际化和主题功能。" }

                CustomComponentWithI18nAndTheme {}
            }
        }
    }
}

/// 自定义组件（使用国际化和主题）
#[component]
fn CustomComponentWithI18nAndTheme() -> Element {
    // 获取国际化和主题信息
    let translate = use_translate();
    let is_rtl = use_is_rtl();
    let locale_name = use_locale_name();

    // 获取主题令牌
    let primary_color = use_theme_token("colorPrimary");
    let bg_color = use_theme_token("colorBgContainer");
    let border_color = use_theme_token("colorBorder");
    let text_color = use_theme_token("colorText");
    let border_radius = use_theme_token("borderRadius");

    // 获取翻译文本
    let submit_text = translate("submit");
    let reset_text = translate("reset");
    let loading_text = translate("loading");

    rsx! {
        div {
            style: "border: 1px solid {border_color}; background-color: {bg_color}; color: {text_color}; padding: 16px; border-radius: {border_radius};",
            dir: if is_rtl { "rtl" } else { "ltr" },

            h4 {
                style: "color: {primary_color}; margin-top: 0;",
                "当前语言: {locale_name}"
            }

            div {
                style: "display: flex; gap: 8px; margin-bottom: 16px;",

                Button {
                    button_type: ButtonType::Primary,
                    "{submit_text}"
                }

                Button {
                    "{reset_text}"
                }

                Button {
                    loading: true,
                    "{loading_text}"
                }
            }

            div {
                p { "这个组件会根据当前语言环境自动调整文本内容，并根据当前主题调整样式。" }
                p { "它使用了use_translate、use_is_rtl、use_locale_name和use_theme_token等Hook来获取当前的国际化和主题设置。" }
            }
        }
    }
}

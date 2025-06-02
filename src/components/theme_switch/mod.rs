//! ThemeSwitch 主题切换组件
//!
//! 提供主题切换功能的用户界面组件，支持在不同主题间切换。
//! 符合 Ant Design 的设计规范，提供优雅的主题切换体验。

use crate::theme::{tokens::theme_presets::AntDesignTheme, Theme, ThemeContext};
use css_in_rust::css;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

/// 主题切换组件的显示模式
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ThemeSwitchMode {
    /// 按钮模式
    Button,
    /// 开关模式
    Switch,
    /// 下拉选择模式
    Select,
}

impl Default for ThemeSwitchMode {
    fn default() -> Self {
        Self::Switch
    }
}

/// 主题切换组件属性
#[derive(Props, Clone, PartialEq)]
pub struct ThemeSwitchProps {
    /// 显示模式
    #[props(default = ThemeSwitchMode::Switch)]
    pub mode: ThemeSwitchMode,

    /// 是否显示标签
    #[props(default = true)]
    pub show_label: bool,

    /// 自定义标签文本
    #[props(default)]
    pub label: Option<String>,

    /// 组件尺寸
    #[props(default = "default".to_string())]
    pub size: String,

    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,

    /// 主题切换回调
    #[props(default)]
    pub on_change: Option<EventHandler<Theme>>,
}

/// 主题切换组件
///
/// 提供用户友好的主题切换界面，支持多种显示模式。
/// 自动与主题上下文集成，确保主题切换的一致性。
#[component]
pub fn ThemeSwitch(props: ThemeSwitchProps) -> Element {
    // 获取主题上下文
    let mut theme_context = use_context::<Signal<ThemeContext>>();
    let current_theme = theme_context.read().current_theme.clone();

    // 提取 props 中的值，避免闭包捕获问题
    let mode = props.mode.clone();
    let show_label = props.show_label;
    let label = props.label.clone();
    let size = props.size.clone();
    let disabled = props.disabled;
    let on_change = props.on_change.clone();

    // 获取当前主题类型
    let current_theme_type = get_theme_type(&current_theme);
    let is_dark = current_theme_type == Theme::Dark;

    match mode {
        ThemeSwitchMode::Switch => {
            let switch_class = get_switch_styles(&size, disabled);

            rsx! {
                div {
                    class: "ant-theme-switch-container",
                    style: get_container_styles(),

                    if show_label {
                        span {
                            class: "ant-theme-switch-label",
                            style: get_label_styles(),
                            {label.as_deref().unwrap_or("主题")}
                        }
                    }

                    div {
                        class: "ant-theme-switch {switch_class}",
                        onclick: move |_| {
                            if !disabled {
                                let new_theme = if is_dark { Theme::Light } else { Theme::Dark };

                                // 更新主题上下文
                                theme_context.write().current_theme = match new_theme {
                                    Theme::Light => AntDesignTheme::light(),
                                    Theme::Dark => AntDesignTheme::dark(),
                                    Theme::Compact => AntDesignTheme::compact(),
                                    Theme::Custom => AntDesignTheme::light(),
                                };

                                // 重新注入 CSS 变量
                                let css_vars = theme_context.read().current_theme.to_css_variables();
                                inject_theme_variables(&css_vars);

                                // 触发回调
                                if let Some(callback) = &on_change {
                                    callback.call(new_theme);
                                }
                            }
                        },

                        div {
                            class: "ant-theme-switch-handle",
                            style: get_handle_styles(is_dark),
                            if is_dark { "🌙" } else { "☀️" }
                        }

                        div {
                            class: "ant-theme-switch-inner",
                            style: get_inner_styles(),
                            span { class: "ant-theme-switch-inner-checked", "🌙" }
                            span { class: "ant-theme-switch-inner-unchecked", "☀️" }
                        }
                    }
                }
            }
        }

        ThemeSwitchMode::Button => {
            let button_class = get_button_styles(&size, disabled);

            rsx! {
                div {
                    class: "ant-theme-switch-buttons",
                    style: get_buttons_container_styles(),

                    if show_label {
                        span {
                            class: "ant-theme-switch-label",
                            style: get_label_styles(),
                            {label.as_deref().unwrap_or("主题")}
                        }
                    }

                    div {
                        class: "ant-theme-switch-button-group",
                        style: get_button_group_styles(),

                        button {
                            class: "ant-btn {button_class}",
                            class: if current_theme_type == Theme::Light { "ant-btn-primary" } else { "" },
                            disabled: disabled,
                            onclick: move |_| {
                                // 更新主题上下文
                                theme_context.write().current_theme = AntDesignTheme::light();
                                let css_vars = theme_context.read().current_theme.to_css_variables();
                                inject_theme_variables(&css_vars);
                                if let Some(callback) = &on_change {
                                    callback.call(Theme::Light);
                                }
                            },
                            "☀️ 亮色"
                        }

                        button {
                            class: "ant-btn {button_class}",
                            class: if current_theme_type == Theme::Dark { "ant-btn-primary" } else { "" },
                            disabled: disabled,
                            onclick: move |_| {
                                // 更新主题上下文
                                theme_context.write().current_theme = AntDesignTheme::dark();
                                let css_vars = theme_context.read().current_theme.to_css_variables();
                                inject_theme_variables(&css_vars);
                                if let Some(callback) = &on_change {
                                    callback.call(Theme::Dark);
                                }
                            },
                            "🌙 暗色"
                        }
                    }
                }
            }
        }

        ThemeSwitchMode::Select => {
            let select_class = get_select_styles(&size, disabled);

            rsx! {
                div {
                    class: "ant-theme-switch-select",
                    style: get_select_container_styles(),

                    if show_label {
                        label {
                            class: "ant-theme-switch-label",
                            style: get_label_styles(),
                            {label.as_deref().unwrap_or("主题")}
                        }
                    }

                    select {
                        class: "ant-select {select_class}",
                        disabled: disabled,
                        value: theme_to_string(current_theme_type),
                        onchange: move |evt| {
                            if let Ok(new_theme) = string_to_theme(&evt.value()) {
                                // 更新主题上下文
                                theme_context.write().current_theme = match new_theme {
                                    Theme::Light => AntDesignTheme::light(),
                                    Theme::Dark => AntDesignTheme::dark(),
                                    Theme::Compact => AntDesignTheme::compact(),
                                    Theme::Custom => AntDesignTheme::light(),
                                };
                                let css_vars = theme_context.read().current_theme.to_css_variables();
                                inject_theme_variables(&css_vars);
                                if let Some(callback) = &on_change {
                                    callback.call(new_theme);
                                }
                            }
                        },

                        option { value: "light", "☀️ 亮色主题" }
                        option { value: "dark", "🌙 暗色主题" }
                        option { value: "compact", "📱 紧凑主题" }
                    }
                }
            }
        }
    }
}

// 辅助函数

/// 获取主题类型
fn get_theme_type(theme: &AntDesignTheme) -> Theme {
    // 根据主题的变体判断类型
    match theme.variant {
        css_in_rust::theme::ThemeVariant::Dark => Theme::Dark,
        css_in_rust::theme::ThemeVariant::Light => Theme::Light,
        css_in_rust::theme::ThemeVariant::Auto => Theme::Light, // 默认为亮色
    }
}

/// 主题枚举转字符串
fn theme_to_string(theme: Theme) -> String {
    match theme {
        Theme::Light => "light".to_string(),
        Theme::Dark => "dark".to_string(),
        Theme::Compact => "compact".to_string(),
        Theme::Custom => "custom".to_string(),
    }
}

/// 字符串转主题枚举
fn string_to_theme(s: &str) -> Result<Theme, ()> {
    match s {
        "light" => Ok(Theme::Light),
        "dark" => Ok(Theme::Dark),
        "compact" => Ok(Theme::Compact),
        "custom" => Ok(Theme::Custom),
        _ => Err(()),
    }
}

/// 注入主题变量到 DOM
fn inject_theme_variables(css_vars: &str) {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = document)]
        fn getElementById(id: &str) -> Option<web_sys::Element>;

        #[wasm_bindgen(js_namespace = document)]
        fn createElement(tag: &str) -> web_sys::Element;

        #[wasm_bindgen(js_namespace = document, js_name = head)]
        static HEAD: web_sys::Element;
    }

    // 移除旧的样式
    if let Some(old_style) = getElementById("ant-theme-vars") {
        old_style.remove();
    }

    // 创建新的样式元素
    let style_element = createElement("style");
    style_element.set_attribute("id", "ant-theme-vars").ok();
    style_element.set_text_content(Some(&format!(":root {{{}}}", css_vars)));

    // 添加到 head
    HEAD.append_child(&style_element).ok();
}

// 样式生成函数

fn get_container_styles() -> String {
    css!(
        r#"
        display: flex;
        align-items: center;
        gap: 8px;
        "#
    )
    .to_string()
}

fn get_label_styles() -> String {
    css!(
        r#"
        color: var(--ant-text-color);
        font-size: var(--ant-font-size-base);
        margin-right: 8px;
        "#
    )
    .to_string()
}

fn get_switch_styles(size: &str, disabled: bool) -> String {
    let base_styles = css!(
        r#"
        position: relative;
        display: inline-block;
        box-sizing: border-box;
        min-width: 44px;
        height: 22px;
        line-height: 20px;
        vertical-align: middle;
        background-color: rgba(0, 0, 0, 0.25);
        border: 1px solid transparent;
        border-radius: 100px;
        cursor: pointer;
        transition: all 0.2s;
        user-select: none;
        "#
    )
    .to_string();

    let size_styles = match size {
        "small" => css!("min-width: 28px; height: 16px;").to_string(),
        "large" => css!("min-width: 56px; height: 28px;").to_string(),
        _ => css!("").to_string(),
    };

    let disabled_styles = if disabled {
        css!("cursor: not-allowed; opacity: 0.4;").to_string()
    } else {
        css!("").to_string()
    };

    format!("{} {} {}", base_styles, size_styles, disabled_styles,)
}

fn get_handle_styles(is_dark: bool) -> String {
    let transform = if is_dark {
        "translateX(22px)"
    } else {
        "translateX(2px)"
    };

    css!(&format!(
        r#"
        position: absolute;
        top: 1px;
        left: 1px;
        width: 18px;
        height: 18px;
        background-color: #fff;
        border-radius: 9px;
        cursor: pointer;
        transition: all 0.2s ease-in-out;
        transform: {};
        display: flex;
        align-items: center;
        justify-content: center;
        font-size: 10px;
        "#,
        transform
    ))
    .to_string()
}

fn get_inner_styles() -> String {
    css!(
        r#"
        display: block;
        margin: 0 7px 0 25px;
        color: #fff;
        font-size: 12px;
        transition: margin 0.2s;
        "#
    )
    .to_string()
}

fn get_buttons_container_styles() -> String {
    css!(
        r#"
        display: flex;
        align-items: center;
        gap: 8px;
        "#
    )
    .to_string()
}

fn get_button_group_styles() -> String {
    css!(
        r#"
        display: flex;
        gap: 4px;
        "#
    )
    .to_string()
}

fn get_button_styles(size: &str, disabled: bool) -> String {
    let base_styles = css!(
        r#"
        border-radius: 6px;
        transition: all 0.2s;
        "#
    );

    let size_styles = match size {
        "small" => css!("padding: 4px 8px; font-size: 12px;"),
        "large" => css!("padding: 8px 16px; font-size: 16px;"),
        _ => css!("padding: 6px 12px; font-size: 14px;"),
    };

    let disabled_styles = if disabled {
        css!("cursor: not-allowed; opacity: 0.4;")
    } else {
        css!("")
    };

    format!("{} {} {}", base_styles, size_styles, disabled_styles)
}

fn get_select_container_styles() -> String {
    css!(
        r#"
        display: flex;
        align-items: center;
        gap: 8px;
        "#
    )
    .to_string()
}

fn get_select_styles(size: &str, disabled: bool) -> String {
    let base_styles = css!(
        r#"
        border: 1px solid var(--ant-border-color);
        border-radius: 6px;
        background-color: var(--ant-bg-color);
        color: var(--ant-text-color);
        transition: all 0.2s;
        "#
    )
    .to_string();

    let size_styles = match size {
        "small" => css!("padding: 4px 8px; font-size: 12px;").to_string(),
        "large" => css!("padding: 8px 12px; font-size: 16px;").to_string(),
        _ => css!("padding: 6px 10px; font-size: 14px;").to_string(),
    };

    let disabled_styles = if disabled {
        css!("cursor: not-allowed; opacity: 0.4;").to_string()
    } else {
        css!("").to_string()
    };

    format!("{} {} {}", base_styles, size_styles, disabled_styles)
}

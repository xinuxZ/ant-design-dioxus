//! 主题切换示例应用
//!
//! 本示例展示如何在 Dioxus 应用中使用通用主题引擎和自定义颜色配置，
//! 并实现主题切换功能。

use ant_design_dioxus::theme::{
    color::{ColorConfig, ColorPalette, ColorType, RgbColor},
    core::generic_theme::GenericThemeEngine,
    providers::ThemeProvider,
    DesignTokens, ThemeConfigInterface,
};
use dioxus::prelude::*;
use std::collections::HashMap;
use std::rc::Rc;

/// 自定义颜色配置
#[derive(Debug, Clone, PartialEq)]
struct CustomColors {
    primary: ColorPalette,
    success: ColorPalette,
    warning: ColorPalette,
    error: ColorPalette,
    info: ColorPalette,
    text_color: RgbColor,
    text_color_secondary: RgbColor,
    background_color: RgbColor,
    is_dark: bool,
}

impl CustomColors {
    /// 创建浅色主题颜色配置
    fn light() -> Self {
        Self {
            primary: ColorPalette::from_base(RgbColor::from_hex("#1890ff").unwrap()),
            success: ColorPalette::from_base(RgbColor::from_hex("#52c41a").unwrap()),
            warning: ColorPalette::from_base(RgbColor::from_hex("#faad14").unwrap()),
            error: ColorPalette::from_base(RgbColor::from_hex("#f5222d").unwrap()),
            info: ColorPalette::from_base(RgbColor::from_hex("#1890ff").unwrap()),
            text_color: RgbColor::from_hex("#000000").unwrap().with_alpha(0.85),
            text_color_secondary: RgbColor::from_hex("#000000").unwrap().with_alpha(0.45),
            background_color: RgbColor::from_hex("#ffffff").unwrap(),
            is_dark: false,
        }
    }

    /// 创建深色主题颜色配置
    fn dark() -> Self {
        Self {
            primary: ColorPalette::from_base(RgbColor::from_hex("#177ddc").unwrap()),
            success: ColorPalette::from_base(RgbColor::from_hex("#49aa19").unwrap()),
            warning: ColorPalette::from_base(RgbColor::from_hex("#d89614").unwrap()),
            error: ColorPalette::from_base(RgbColor::from_hex("#a61d24").unwrap()),
            info: ColorPalette::from_base(RgbColor::from_hex("#177ddc").unwrap()),
            text_color: RgbColor::from_hex("#ffffff").unwrap().with_alpha(0.85),
            text_color_secondary: RgbColor::from_hex("#ffffff").unwrap().with_alpha(0.45),
            background_color: RgbColor::from_hex("#141414").unwrap(),
            is_dark: true,
        }
    }
}

impl ColorConfig for CustomColors {
    fn light() -> Self {
        CustomColors::light()
    }

    fn dark() -> Self {
        CustomColors::dark()
    }

    fn get_color(&self, color_type: ColorType) -> Option<RgbColor> {
        match color_type {
            ColorType::Primary => Some(self.primary.base),
            ColorType::Success => Some(self.success.base),
            ColorType::Warning => Some(self.warning.base),
            ColorType::Error => Some(self.error.base),
            ColorType::Info => Some(self.info.base),
            ColorType::TextColor => Some(self.text_color),
            ColorType::TextColorSecondary => Some(self.text_color_secondary),
            ColorType::BackgroundColor => Some(self.background_color),
            _ => None,
        }
    }

    fn get_palette(&self, color_type: ColorType) -> Option<&ColorPalette> {
        match color_type {
            ColorType::Primary => Some(&self.primary),
            ColorType::Success => Some(&self.success),
            ColorType::Warning => Some(&self.warning),
            ColorType::Error => Some(&self.error),
            ColorType::Info => Some(&self.info),
            _ => None,
        }
    }

    fn to_css_variables(&self) -> HashMap<String, String> {
        let mut variables = HashMap::new();

        // 添加基础颜色变量
        if let Some(color) = self.get_color(ColorType::Primary) {
            variables.insert("--custom-primary-color".to_string(), color.to_rgb_string());
        }
        if let Some(color) = self.get_color(ColorType::Success) {
            variables.insert("--custom-success-color".to_string(), color.to_rgb_string());
        }
        if let Some(color) = self.get_color(ColorType::Warning) {
            variables.insert("--custom-warning-color".to_string(), color.to_rgb_string());
        }
        if let Some(color) = self.get_color(ColorType::Error) {
            variables.insert("--custom-error-color".to_string(), color.to_rgb_string());
        }
        if let Some(color) = self.get_color(ColorType::Info) {
            variables.insert("--custom-info-color".to_string(), color.to_rgb_string());
        }

        // 添加文本颜色变量
        if let Some(color) = self.get_color(ColorType::TextColor) {
            variables.insert("--custom-text-color".to_string(), color.to_rgb_string());
        }
        if let Some(color) = self.get_color(ColorType::TextColorSecondary) {
            variables.insert(
                "--custom-text-color-secondary".to_string(),
                color.to_rgb_string(),
            );
        }

        // 添加背景颜色变量
        if let Some(color) = self.get_color(ColorType::BackgroundColor) {
            variables.insert("--custom-bg-color".to_string(), color.to_rgb_string());
        }

        variables
    }

    fn generate_css(&self) -> String {
        let mut css = String::new();

        // 生成颜色类
        for color_type in [
            ColorType::Primary,
            ColorType::Success,
            ColorType::Warning,
            ColorType::Error,
            ColorType::Info,
        ] {
            if let Some(color) = self.get_color(color_type) {
                let class_name = match color_type {
                    ColorType::Primary => "custom-primary",
                    ColorType::Success => "custom-success",
                    ColorType::Warning => "custom-warning",
                    ColorType::Error => "custom-error",
                    ColorType::Info => "custom-info",
                    _ => continue,
                };

                css.push_str(&format!(
                    ".{} {{ color: {}; }}\n",
                    class_name,
                    color.to_rgb_string()
                ));

                css.push_str(&format!(
                    ".{}-bg {{ background-color: {}; }}\n",
                    class_name,
                    color.to_rgb_string()
                ));
            }
        }

        css
    }

    fn is_dark(&self) -> bool {
        self.is_dark
    }
}

/// 自定义主题提供者
#[derive(Debug, Clone, PartialEq)]
struct CustomThemeProvider {
    light_theme: Rc<GenericThemeEngine<CustomColors>>,
    dark_theme: Rc<GenericThemeEngine<CustomColors>>,
    current_theme: Rc<GenericThemeEngine<CustomColors>>,
}

impl CustomThemeProvider {
    fn new() -> Self {
        let light_theme = Rc::new(GenericThemeEngine::new(
            "CustomLight".to_string(),
            false,
            false,
            CustomColors::light(),
            "default".to_string(),
            "default".to_string(),
            "default".to_string(),
            "default".to_string(),
        ));

        let dark_theme = Rc::new(GenericThemeEngine::new(
            "CustomDark".to_string(),
            true,
            false,
            CustomColors::dark(),
            "default".to_string(),
            "default".to_string(),
            "default".to_string(),
            "default".to_string(),
        ));

        Self {
            light_theme: light_theme.clone(),
            dark_theme: dark_theme.clone(),
            current_theme: light_theme,
        }
    }

    fn toggle_theme(&mut self) {
        if self.current_theme.is_dark() {
            self.current_theme = self.light_theme.clone();
        } else {
            self.current_theme = self.dark_theme.clone();
        }
    }
}

impl ThemeProvider for CustomThemeProvider {
    type Theme = GenericThemeEngine<CustomColors>;

    fn current_theme(&self) -> Rc<Self::Theme> {
        self.current_theme.clone()
    }
}

/// 主题切换按钮组件
#[component]
fn ThemeSwitcher(cx: Scope) -> Element {
    let theme_provider = use_context::<CustomThemeProvider>(cx).unwrap();
    let is_dark = theme_provider.current_theme().is_dark();

    let toggle_theme = move |_| {
        let mut provider = theme_provider.clone();
        provider.toggle_theme();
        cx.provide_context(provider);
    };

    cx.render(rsx! {
        button {
            onclick: toggle_theme,
            style: "padding: 8px 16px; border-radius: 4px; cursor: pointer; background-color: var(--custom-primary-color); color: white; border: none;",
            if is_dark {
                "切换到浅色主题"
            } else {
                "切换到深色主题"
            }
        }
    })
}

/// 颜色展示组件
#[component]
fn ColorDisplay(cx: Scope) -> Element {
    let theme_provider = use_context::<CustomThemeProvider>(cx).unwrap();
    let theme = theme_provider.current_theme();

    let color_types = vec![
        ("主色", ColorType::Primary),
        ("成功色", ColorType::Success),
        ("警告色", ColorType::Warning),
        ("错误色", ColorType::Error),
        ("信息色", ColorType::Info),
    ];

    cx.render(rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 16px;",
            for (name, color_type) in color_types {
                if let Some(color) = theme.get_color(color_type) {
                    div {
                        style: "display: flex; align-items: center; gap: 8px;",
                        div {
                            style: "width: 24px; height: 24px; border-radius: 4px; background-color: {color};",
                            style: "background-color: {}",
                                color.to_rgb_string(),
                        },
                        span {
                            style: "color: var(--custom-text-color);",
                            "{name}: {}",
                                color.to_hex_string(),
                        }
                    }
                }
            }
        }
    })
}

/// 应用组件
#[component]
fn App(cx: Scope) -> Element {
    let theme_provider = use_context::<CustomThemeProvider>(cx).unwrap();
    let theme = theme_provider.current_theme();
    let bg_color = theme.get_color(ColorType::BackgroundColor).unwrap();
    let text_color = theme.get_color(ColorType::TextColor).unwrap();

    // 生成 CSS 变量
    let css_variables = theme.to_css_variables();
    let css_vars = css_variables
        .iter()
        .map(|(k, v)| format!("{}: {};", k, v))
        .collect::<Vec<_>>()
        .join("\n");

    cx.render(rsx! {
        style { "{css_vars}" },
        div {
            style: "font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif; padding: 24px; min-height: 100vh; transition: all 0.3s;",
            style: "background-color: {}; color: {};",
                bg_color.to_rgb_string(),
                text_color.to_rgb_string(),
            h1 {
                style: "margin-bottom: 24px; color: var(--custom-text-color);",
                "自定义主题示例"
            },
            div {
                style: "margin-bottom: 24px;",
                ThemeSwitcher {}
            },
            div {
                style: "display: flex; flex-direction: column; gap: 24px;",
                div {
                    style: "padding: 16px; border-radius: 8px; background-color: var(--custom-primary-color); color: white;",
                    "这是使用主色的区块"
                },
                div {
                    style: "padding: 16px; border-radius: 8px; background-color: var(--custom-success-color); color: white;",
                    "这是使用成功色的区块"
                },
                div {
                    style: "padding: 16px; border-radius: 8px; background-color: var(--custom-warning-color); color: white;",
                    "这是使用警告色的区块"
                },
                div {
                    style: "padding: 16px; border-radius: 8px; background-color: var(--custom-error-color); color: white;",
                    "这是使用错误色的区块"
                }
            },
            h2 {
                style: "margin: 24px 0; color: var(--custom-text-color);",
                "颜色展示"
            },
            ColorDisplay {}
        }
    })
}

fn main() {
    // 创建自定义主题提供者
    let theme_provider = CustomThemeProvider::new();

    // 启动应用
    dioxus_desktop::launch_with_props(
        |cx| {
            cx.provide_context(theme_provider);
            cx.render(rsx! { App {} })
        },
        dioxus_desktop::Config::default(),
    );
}

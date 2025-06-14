//! 通用主题引擎示例
//!
//! 本示例展示如何使用 GenericThemeEngine 和自定义颜色配置

use ant_design_dioxus::theme::{
    color::{ColorConfig, ColorPalette, ColorType, RgbColor},
    core::generic_theme::GenericThemeEngine,
    DesignTokens, ThemeConfigInterface,
};
use std::collections::HashMap;

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

        // 添加调色板变量
        for color_type in [
            ColorType::Primary,
            ColorType::Success,
            ColorType::Warning,
            ColorType::Error,
            ColorType::Info,
        ] {
            if let Some(palette) = self.get_palette(color_type) {
                let prefix = match color_type {
                    ColorType::Primary => "--custom-primary",
                    ColorType::Success => "--custom-success",
                    ColorType::Warning => "--custom-warning",
                    ColorType::Error => "--custom-error",
                    ColorType::Info => "--custom-info",
                    _ => continue,
                };

                variables.insert(
                    format!("{}-color-light", prefix),
                    palette.light.to_rgb_string(),
                );
                variables.insert(
                    format!("{}-color-lighter", prefix),
                    palette.lighter.to_rgb_string(),
                );
                variables.insert(
                    format!("{}-color-dark", prefix),
                    palette.dark.to_rgb_string(),
                );
                variables.insert(
                    format!("{}-color-darker", prefix),
                    palette.darker.to_rgb_string(),
                );
            }
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

        // 生成文本颜色类
        if let Some(color) = self.get_color(ColorType::TextColor) {
            css.push_str(&format!(
                ".custom-text {{ color: {}; }}\n",
                color.to_rgb_string()
            ));
        }

        if let Some(color) = self.get_color(ColorType::TextColorSecondary) {
            css.push_str(&format!(
                ".custom-text-secondary {{ color: {}; }}\n",
                color.to_rgb_string()
            ));
        }

        css
    }

    fn is_dark(&self) -> bool {
        self.is_dark
    }
}

fn main() {
    // 创建自定义颜色配置
    let custom_colors = CustomColors::light();

    // 创建通用主题引擎
    let theme = GenericThemeEngine::new(
        "CustomTheme".to_string(),
        false,
        false,
        custom_colors,
        "default".to_string(),
        "default".to_string(),
        "default".to_string(),
        "default".to_string(),
    );

    // 生成 CSS 变量
    let css_variables = theme.to_css_variables();
    println!("CSS Variables:\n");
    for (key, value) in &css_variables {
        println!("{}: {}", key, value);
    }

    // 生成 CSS
    let css = theme.generate_css();
    println!("\n\nGenerated CSS:\n");
    println!("{}", css);

    // 获取颜色
    if let Some(primary_color) = theme.get_color(ColorType::Primary) {
        println!("\n\nPrimary Color: {}", primary_color.to_hex_string());
    }

    // 创建深色主题
    let dark_theme = GenericThemeEngine::dark(
        "DarkCustomTheme".to_string(),
        "default".to_string(),
        "default".to_string(),
        "default".to_string(),
        "default".to_string(),
    );

    println!("\n\nDark Theme Name: {}", dark_theme.name());
    println!("Is Dark: {}", dark_theme.is_dark());
}

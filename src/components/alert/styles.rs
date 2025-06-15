//! Alert 组件的样式定义

use crate::components::alert::types::*;
use css_in_rust::css;
use std::collections::HashMap;

/// Alert 样式管理器
pub struct AlertStyles;

impl AlertStyles {
    /// 获取基础样式
    pub fn base() -> String {
        css! (
            ".ant-alert": {
                "box-sizing": "border-box",
                "margin": "0",
                "padding": "8px 15px",
                "color": "rgba(0, 0, 0, 0.85)",
                "font-size": "14px",
                "font-variant": "tabular-nums",
                "line-height": "1.5715",
                "list-style": "none",
                "font-feature-settings": "'tnum'",
                "position": "relative",
                "display": "flex",
                "align-items": "center",
                "padding": "8px 15px",
                "word-wrap": "break-word",
                "border-radius": "6px",
                "border": "1px solid transparent",
                "transition": "all 0.3s cubic-bezier(0.645, 0.045, 0.355, 1)"
            },
            ".ant-alert-content": {
                "flex": "1",
                "min-width": "0"
            },
            ".ant-alert-icon": {
                "margin-right": "8px",
                "line-height": "0"
            },
            ".ant-alert-description": {
                "display": "block",
                "margin-top": "4px",
                "color": "rgba(0, 0, 0, 0.65)",
                "font-size": "14px",
                "line-height": "1.5715"
            },
            ".ant-alert-message": {
                "color": "rgba(0, 0, 0, 0.85)",
                "font-size": "14px",
                "line-height": "1.5715",
                "display": "block"
            },
            ".ant-alert-action": {
                "margin-left": "8px"
            },
            ".ant-alert-close-icon": {
                "position": "absolute",
                "top": "8px",
                "right": "8px",
                "padding": "0",
                "overflow": "hidden",
                "font-size": "12px",
                "line-height": "22px",
                "background-color": "transparent",
                "border": "none",
                "outline": "none",
                "cursor": "pointer",
                "color": "rgba(0, 0, 0, 0.45)",
                "transition": "color 0.3s"
            },
            ".ant-alert-close-icon:hover": {
                "color": "rgba(0, 0, 0, 0.75)"
            },
            ".ant-alert-close-icon:focus": {
                "outline": "2px solid #1890ff",
                "outline-offset": "2px"
            }
        )
        .to_string()
    }

    /// 获取类型样式
    pub fn type_styles() -> String {
        css! {
            ".ant-alert-success": {
                "background-color": "#f6ffed",
                "border-color": "#b7eb8f",
                "color": "#52c41a"
            },
            ".ant-alert-success .ant-alert-icon": {
                "color": "#52c41a"
            },
            ".ant-alert-info": {
                "background-color": "#e6f7ff",
                "border-color": "#91d5ff",
                "color": "#1890ff"
            },
            ".ant-alert-info .ant-alert-icon": {
                "color": "#1890ff"
            },
            ".ant-alert-warning": {
                "background-color": "#fffbe6",
                "border-color": "#ffe58f",
                "color": "#faad14"
            },
            ".ant-alert-warning .ant-alert-icon": {
                "color": "#faad14"
            },
            ".ant-alert-error": {
                "background-color": "#fff2f0",
                "border-color": "#ffccc7",
                "color": "#ff4d4f"
            },
            ".ant-alert-error .ant-alert-icon": {
                "color": "#ff4d4f"
            }
        }
        .to_string()
    }

    /// 获取尺寸样式
    pub fn size_styles() -> String {
        css! {
            ".ant-alert-small": {
                "padding": "4px 8px",
                "font-size": "12px"
            },
            ".ant-alert-small .ant-alert-icon": {
                "font-size": "14px",
                "margin-right": "6px"
            },
            ".ant-alert-small .ant-alert-close-icon": {
                "top": "4px",
                "right": "4px",
                "font-size": "10px",
                "line-height": "16px"
            },
            ".ant-alert-large": {
                "padding": "12px 20px",
                "font-size": "16px"
            },
            ".ant-alert-large .ant-alert-icon": {
                "font-size": "18px",
                "margin-right": "10px"
            },
            ".ant-alert-large .ant-alert-close-icon": {
                "top": "12px",
                "right": "12px",
                "font-size": "14px",
                "line-height": "24px"
            }
        }
        .to_string()
    }

    /// 获取状态样式
    pub fn state_styles() -> String {
        css! {
            ".ant-alert-closable": {
                "padding-right": "40px"
            },
            ".ant-alert-with-icon": {
                "padding": "8px 15px 8px 47px"
            },
            ".ant-alert-with-icon.ant-alert-small": {
                "padding": "4px 8px 4px 35px"
            },
            ".ant-alert-with-icon.ant-alert-large": {
                "padding": "12px 20px 12px 55px"
            },
            ".ant-alert-with-icon .ant-alert-icon": {
                "position": "absolute",
                "left": "15px",
                "top": "50%",
                "transform": "translateY(-50%)"
            },
            ".ant-alert-with-icon.ant-alert-small .ant-alert-icon": {
                "left": "8px"
            },
            ".ant-alert-with-icon.ant-alert-large .ant-alert-icon": {
                "left": "20px"
            },
            ".ant-alert-with-description": {
                "padding": "15px 15px 15px 64px",
                "align-items": "flex-start"
            },
            ".ant-alert-with-description.ant-alert-small": {
                "padding": "8px 8px 8px 45px"
            },
            ".ant-alert-with-description.ant-alert-large": {
                "padding": "20px 20px 20px 75px"
            },
            ".ant-alert-with-description .ant-alert-icon": {
                "margin-right": "0",
                "font-size": "24px",
                "top": "15px"
            },
            ".ant-alert-with-description.ant-alert-small .ant-alert-icon": {
                "font-size": "18px",
                "top": "8px"
            },
            ".ant-alert-with-description.ant-alert-large .ant-alert-icon": {
                "font-size": "28px",
                "top": "20px"
            },
            ".ant-alert-with-description .ant-alert-message": {
                "color": "rgba(0, 0, 0, 0.85)",
                "font-size": "16px",
                "margin-bottom": "4px",
                "display": "block"
            },
            ".ant-alert-with-description.ant-alert-small .ant-alert-message": {
                "font-size": "14px"
            },
            ".ant-alert-with-description.ant-alert-large .ant-alert-message": {
                "font-size": "18px"
            },
            ".ant-alert-banner": {
                "border": "0",
                "border-radius": "0",
                "margin-bottom": "0"
            },
            ".ant-alert-no-border": {
                "border": "none"
            }
        }
        .to_string()
    }

    /// 获取动画样式
    pub fn animation_styles() -> String {
        css! {
            "@keyframes ant-alert-motion-in": {
                "0%": {
                    "opacity": "0",
                    "transform": "scaleY(0)",
                    "transform-origin": "0 0"
                },
                "100%": {
                    "opacity": "1",
                    "transform": "scaleY(1)",
                    "transform-origin": "0 0"
                }
            },
            "@keyframes ant-alert-motion-out": {
                "0%": {
                    "opacity": "1",
                    "transform": "scaleY(1)",
                    "transform-origin": "0 0"
                },
                "100%": {
                    "opacity": "0",
                    "transform": "scaleY(0)",
                    "transform-origin": "0 0"
                }
            },
            ".ant-alert-motion-enter": {
                "animation": "ant-alert-motion-in 0.3s cubic-bezier(0.645, 0.045, 0.355, 1)"
            },
            ".ant-alert-motion-leave": {
                "animation": "ant-alert-motion-out 0.3s cubic-bezier(0.645, 0.045, 0.355, 1)"
            }
        }
        .to_string()
    }

    /// 获取暗色主题样式
    pub fn dark_theme_styles() -> String {
        css! {
            "[data-theme='dark'] .ant-alert": {
                "background-color": "#1f1f1f",
                "border-color": "#434343",
                "color": "rgba(255, 255, 255, 0.85)"
            },
            "[data-theme='dark'] .ant-alert-message": {
                "color": "rgba(255, 255, 255, 0.85)"
            },
            "[data-theme='dark'] .ant-alert-description": {
                "color": "rgba(255, 255, 255, 0.65)"
            },
            "[data-theme='dark'] .ant-alert-close-icon": {
                "color": "rgba(255, 255, 255, 0.45)"
            },
            "[data-theme='dark'] .ant-alert-close-icon:hover": {
                "color": "rgba(255, 255, 255, 0.75)"
            },
            "[data-theme='dark'] .ant-alert-success": {
                "background-color": "#162312",
                "border-color": "#274916"
            },
            "[data-theme='dark'] .ant-alert-info": {
                "background-color": "#111a2c",
                "border-color": "#153450"
            },
            "[data-theme='dark'] .ant-alert-warning": {
                "background-color": "#2b2111",
                "border-color": "#594214"
            },
            "[data-theme='dark'] .ant-alert-error": {
                "background-color": "#2a1215",
                "border-color": "#58181c"
            }
        }
        .to_string()
    }

    /// 获取RTL样式
    pub fn rtl_styles() -> String {
        css! {
            "[dir='rtl'] .ant-alert-icon": {
                "margin-right": "0",
                "margin-left": "8px"
            },
            "[dir='rtl'] .ant-alert-action": {
                "margin-left": "0",
                "margin-right": "8px"
            },
            "[dir='rtl'] .ant-alert-close-icon": {
                "right": "auto",
                "left": "8px"
            },
            "[dir='rtl'] .ant-alert-with-icon": {
                "padding": "8px 47px 8px 15px"
            },
            "[dir='rtl'] .ant-alert-with-icon .ant-alert-icon": {
                "left": "auto",
                "right": "15px"
            },
            "[dir='rtl'] .ant-alert-with-description": {
                "padding": "15px 64px 15px 15px"
            },
            "[dir='rtl'] .ant-alert-with-description .ant-alert-icon": {
                "left": "auto",
                "right": "15px"
            }
        }
        .to_string()
    }

    /// 获取响应式样式
    pub fn responsive_styles() -> String {
        css! {
            "@media (max-width: 575px)": {
                ".ant-alert": {
                    "padding": "6px 12px",
                    "font-size": "13px"
                },
                ".ant-alert-with-icon": {
                    "padding": "6px 12px 6px 40px"
                },
                ".ant-alert-with-icon .ant-alert-icon": {
                    "left": "12px",
                    "font-size": "14px"
                },
                ".ant-alert-with-description": {
                    "padding": "12px 12px 12px 50px"
                },
                ".ant-alert-with-description .ant-alert-icon": {
                    "font-size": "20px",
                    "top": "12px"
                },
                ".ant-alert-close-icon": {
                    "top": "6px",
                    "right": "6px"
                }
            },
            "@media (min-width: 576px) and (max-width: 767px)": {
                ".ant-alert": {
                    "padding": "7px 13px"
                }
            }
        }
        .to_string()
    }

    /// 获取高对比度样式
    pub fn high_contrast_styles() -> String {
        css! {
            "@media (prefers-contrast: high)": {
                ".ant-alert": {
                    "border-width": "2px"
                },
                ".ant-alert-success": {
                    "background-color": "#e6ffe6",
                    "border-color": "#00aa00",
                    "color": "#006600"
                },
                ".ant-alert-info": {
                    "background-color": "#e6f3ff",
                    "border-color": "#0066cc",
                    "color": "#003d7a"
                },
                ".ant-alert-warning": {
                    "background-color": "#fff8e6",
                    "border-color": "#cc8800",
                    "color": "#995500"
                },
                ".ant-alert-error": {
                    "background-color": "#ffe6e6",
                    "border-color": "#cc0000",
                    "color": "#990000"
                }
            }
        }
        .to_string()
    }

    /// 获取减少动画样式
    pub fn reduced_motion_styles() -> String {
        css! {
            "@media (prefers-reduced-motion: reduce)": {
                ".ant-alert": {
                    "transition": "none"
                },
                ".ant-alert-entering, .ant-alert-exiting": {
                    "animation": "none"
                }
            }
        }
        .to_string()
    }

    /// 获取打印样式
    pub fn print_styles() -> String {
        css! {
            "@media print": {
                ".ant-alert": {
                    "color-adjust": "exact",
                    "print-color-adjust": "exact",
                    "box-shadow": "none",
                    "page-break-inside": "avoid"
                },
                ".ant-alert-close-icon": {
                    "display": "none"
                }
            }
        }
        .to_string()
    }

    /// 获取完整样式
    pub fn get_styles() -> String {
        let mut combined = Vec::new();
        combined.push(Self::base());
        combined.push(Self::type_styles());
        combined.push(Self::size_styles());
        combined.push(Self::state_styles());
        combined.push(Self::animation_styles());
        combined.push(Self::dark_theme_styles());
        combined.push(Self::rtl_styles());
        combined.push(Self::responsive_styles());
        combined.push(Self::high_contrast_styles());
        combined.push(Self::reduced_motion_styles());
        combined.push(Self::print_styles());
        combined.join("\n")
    }

    /// 根据主题生成自定义样式
    pub fn generate_theme_styles(theme: &AlertTheme) -> String {
        css! {
            ".ant-alert": {
                "border-radius": theme.border_radius.clone(),
                "font-size": theme.font_size.clone(),
                "font-family": theme.font_family.clone(),
                "line-height": theme.line_height.clone(),
                "box-shadow": theme.box_shadow.clone(),
                "transition-duration": theme.animation_duration.clone(),
                "transition-timing-function": theme.animation_timing_function.clone()
            },
            ".ant-alert-success": {
                "color": theme.success_color.clone()
            },
            ".ant-alert-info": {
                "color": theme.info_color.clone()
            },
            ".ant-alert-warning": {
                "color": theme.warning_color.clone()
            },
            ".ant-alert-error": {
                "color": theme.error_color.clone()
            }
        }
        .to_string()
    }

    /// 生成动态样式
    pub fn generate_dynamic_styles(props: &AlertProps) -> String {
        let mut styles = Vec::new();

        // 自定义颜色
        if let Some(ref color) = props.color {
            styles.push(format!("color: {}", color));
        }

        // 自定义背景色
        if let Some(ref bg_color) = props.background_color {
            styles.push(format!("background-color: {}", bg_color));
        }

        // 自定义边框颜色
        if let Some(ref border_color) = props.border_color {
            styles.push(format!("border-color: {}", border_color));
        }

        // 自定义圆角
        if let Some(ref border_radius) = props.border_radius {
            styles.push(format!("border-radius: {}", border_radius));
        }

        // 动画持续时间
        if props.enable_animation {
            styles.push(format!(
                "transition-duration: {}ms",
                props.animation_duration
            ));
        } else {
            styles.push("transition: none".to_string());
        }

        // 边框显示
        if !props.show_border {
            styles.push("border: none".to_string());
        }

        styles.join("; ")
    }

    /// 获取CSS变量
    pub fn get_css_variables() -> HashMap<String, String> {
        let mut vars = HashMap::new();

        // 颜色变量
        vars.insert(
            "--ant-alert-success-color".to_string(),
            "#52c41a".to_string(),
        );
        vars.insert("--ant-alert-info-color".to_string(), "#1890ff".to_string());
        vars.insert(
            "--ant-alert-warning-color".to_string(),
            "#faad14".to_string(),
        );
        vars.insert("--ant-alert-error-color".to_string(), "#ff4d4f".to_string());

        // 背景色变量
        vars.insert("--ant-alert-success-bg".to_string(), "#f6ffed".to_string());
        vars.insert("--ant-alert-info-bg".to_string(), "#e6f7ff".to_string());
        vars.insert("--ant-alert-warning-bg".to_string(), "#fffbe6".to_string());
        vars.insert("--ant-alert-error-bg".to_string(), "#fff2f0".to_string());

        // 边框色变量
        vars.insert(
            "--ant-alert-success-border".to_string(),
            "#b7eb8f".to_string(),
        );
        vars.insert("--ant-alert-info-border".to_string(), "#91d5ff".to_string());
        vars.insert(
            "--ant-alert-warning-border".to_string(),
            "#ffe58f".to_string(),
        );
        vars.insert(
            "--ant-alert-error-border".to_string(),
            "#ffccc7".to_string(),
        );

        // 尺寸变量
        vars.insert("--ant-alert-padding".to_string(), "8px 15px".to_string());
        vars.insert("--ant-alert-border-radius".to_string(), "6px".to_string());
        vars.insert("--ant-alert-font-size".to_string(), "14px".to_string());
        vars.insert("--ant-alert-line-height".to_string(), "1.5715".to_string());

        // 动画变量
        vars.insert(
            "--ant-alert-animation-duration".to_string(),
            "0.3s".to_string(),
        );
        vars.insert(
            "--ant-alert-animation-timing".to_string(),
            "cubic-bezier(0.645, 0.045, 0.355, 1)".to_string(),
        );

        vars
    }

    /// 生成CSS变量样式
    pub fn generate_css_variables_style() -> String {
        let vars = Self::get_css_variables();
        let mut css_content = String::from(":root {\n");

        for (key, value) in vars {
            css_content.push_str(&format!("  {}: {};\n", key, value));
        }

        css_content.push_str("}");

        // 注意：这里需要根据实际的css_in_rust库API调整
        css_content
    }
}

/// 样式工具函数
pub struct StyleUtils;

impl StyleUtils {
    /// 合并样式字符串
    pub fn merge_styles(styles: &[&str]) -> String {
        styles.join("; ")
    }

    /// 解析颜色值
    pub fn parse_color(color: &str) -> Result<String, String> {
        if color.starts_with('#') {
            if color.len() == 4 || color.len() == 7 {
                Ok(color.to_string())
            } else {
                Err("Invalid hex color format".to_string())
            }
        } else if color.starts_with("rgb") || color.starts_with("hsl") {
            Ok(color.to_string())
        } else {
            // 命名颜色
            Ok(color.to_string())
        }
    }

    /// 计算对比色
    pub fn get_contrast_color(background: &str) -> String {
        // 简单的对比色计算
        if background.contains("dark")
            || background.starts_with("#0")
            || background.starts_with("#1")
        {
            "#ffffff".to_string()
        } else {
            "#000000".to_string()
        }
    }

    /// 生成渐变背景
    pub fn generate_gradient(start_color: &str, end_color: &str, direction: &str) -> String {
        format!(
            "linear-gradient({}, {}, {})",
            direction, start_color, end_color
        )
    }

    /// 生成阴影
    pub fn generate_box_shadow(x: i32, y: i32, blur: i32, spread: i32, color: &str) -> String {
        format!("{}px {}px {}px {}px {}", x, y, blur, spread, color)
    }

    /// 转换尺寸单位
    pub fn convert_size_unit(value: f32, from_unit: &str, to_unit: &str) -> String {
        match (from_unit, to_unit) {
            ("px", "rem") => format!("{}rem", value / 16.0),
            ("rem", "px") => format!("{}px", value * 16.0),
            ("px", "em") => format!("{}em", value / 16.0),
            ("em", "px") => format!("{}px", value * 16.0),
            _ => format!("{}{}", value, to_unit),
        }
    }

    /// 生成媒体查询
    pub fn generate_media_query(min_width: Option<u32>, max_width: Option<u32>) -> String {
        match (min_width, max_width) {
            (Some(min), Some(max)) => {
                format!("@media (min-width: {}px) and (max-width: {}px)", min, max)
            }
            (Some(min), None) => format!("@media (min-width: {}px)", min),
            (None, Some(max)) => format!("@media (max-width: {}px)", max),
            (None, None) => "@media all".to_string(),
        }
    }

    /// 优化CSS
    pub fn optimize_css(css: &str) -> String {
        // 简单的CSS优化：移除多余空格和换行
        css.lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join(" ")
            .replace("; ", ";")
            .replace(" {", "{")
            .replace("{ ", "{")
            .replace(" }", "}")
            .replace("} ", "}")
    }
}

/// 主题管理器
pub struct ThemeManager {
    themes: HashMap<String, AlertTheme>,
    current_theme: String,
}

impl ThemeManager {
    /// 创建新的主题管理器
    pub fn new() -> Self {
        let mut themes = HashMap::new();
        themes.insert("default".to_string(), AlertTheme::default());

        Self {
            themes,
            current_theme: "default".to_string(),
        }
    }

    /// 添加主题
    pub fn add_theme(&mut self, name: String, theme: AlertTheme) {
        self.themes.insert(name, theme);
    }

    /// 设置当前主题
    pub fn set_current_theme(&mut self, name: String) -> Result<(), String> {
        if self.themes.contains_key(&name) {
            self.current_theme = name;
            Ok(())
        } else {
            Err(format!("Theme '{}' not found", name))
        }
    }

    /// 获取当前主题
    pub fn get_current_theme(&self) -> Option<&AlertTheme> {
        self.themes.get(&self.current_theme)
    }

    /// 获取主题样式
    pub fn get_theme_styles(&self, theme_name: &str) -> Option<String> {
        self.themes
            .get(theme_name)
            .map(|theme| AlertStyles::generate_theme_styles(theme))
    }
}

impl Default for ThemeManager {
    fn default() -> Self {
        Self::new()
    }
}

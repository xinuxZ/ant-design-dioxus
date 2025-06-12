//! App 组件样式模块
//!
//! 本模块包含 App 组件的所有样式定义，从组件逻辑中分离出来，
//! 提高代码的可维护性和复用性。

use crate::theme::AliasToken;
use css_in_rust::css;

/// App 主题类型
#[derive(Debug, Clone, PartialEq)]
pub enum AppTheme {
    /// 默认主题
    Default,
    /// 暗色主题
    Dark,
    /// 紧凑主题
    Compact,
    /// 暗色+紧凑主题
    DarkCompact,
}

impl Default for AppTheme {
    fn default() -> Self {
        Self::Default
    }
}

/// App 组件样式生成器
pub struct AppStyleGenerator {
    /// 主题类型
    pub theme: AppTheme,
    /// 主题令牌
    pub token: AliasToken,
}

impl AppStyleGenerator {
    /// 创建新的 App 样式生成器
    pub fn new() -> Self {
        Self {
            theme: AppTheme::Default,
            token: AliasToken::default(),
        }
    }

    /// 设置主题
    pub fn with_theme(mut self, theme: AppTheme) -> Self {
        self.theme = theme;
        self
    }

    /// 设置主题令牌
    pub fn with_token(mut self, token: AliasToken) -> Self {
        self.token = token;
        self
    }

    /// 生成完整的 App 样式
    pub fn generate(&self) -> String {
        let mut classes = vec!["ant-app".to_string()];

        match self.theme {
            AppTheme::Default => {}
            AppTheme::Dark => classes.push("ant-app-dark".to_string()),
            AppTheme::Compact => classes.push("ant-app-compact".to_string()),
            AppTheme::DarkCompact => {
                classes.push("ant-app-dark".to_string());
                classes.push("ant-app-compact".to_string());
            }
        }

        classes.join(" ")
    }

    /// 生成 CSS 样式
    pub fn generate_css(&self) -> String {
        css!(
            r#"
            .ant-app {
                position: relative;
                width: 100%;
                height: 100%;
                color: ${color_text};
                font-size: ${font_size}px;
                font-family: ${font_family};
                line-height: ${line_height};
                font-variant: tabular-nums;
                font-feature-settings: 'tnum';
                background: ${bg_color_container};
                box-sizing: border-box;
            }

            .ant-app *, .ant-app *::before, .ant-app *::after {
                box-sizing: border-box;
            }

            .ant-app-dark {
                background: ${bg_color_dark};
                color: ${color_text_light};
            }

            .ant-app-dark .ant-app-header {
                background: ${bg_color_dark_header};
                border-bottom-color: ${border_color_dark};
            }

            .ant-app-dark .ant-app-footer {
                background: ${bg_color_dark_header};
                border-top-color: ${border_color_dark};
                color: ${color_text_light_secondary};
            }

            .ant-app-dark .ant-app-sidebar {
                background: ${bg_color_dark_header};
                border-right-color: ${border_color_dark};
            }

            .ant-app-dark .ant-app-main {
                background: ${bg_color_dark};
            }

            .ant-app-compact {
                font-size: ${font_size_sm}px;
            }

            .ant-app-compact .ant-app-main {
                padding: ${padding_sm}px;
            }
            "#,
            color_text = self.token.color_text,
            font_size = self.token.font_size,
            font_family = self.token.font_family,
            line_height = self.token.line_height,
            bg_color_container = self.token.color_bg_container,
            bg_color_dark = "#141414",
            color_text_light = "rgba(255, 255, 255, 0.85)",
            bg_color_dark_header = "#1f1f1f",
            border_color_dark = "#303030",
            color_text_light_secondary = "rgba(255, 255, 255, 0.45)",
            font_size_sm = self.token.font_size_sm,
            padding_sm = self.token.padding_sm
        )
        .to_string()
    }
}

/// App 容器样式
pub fn app_container_style() -> String {
    css!(
        r#"
        .ant-app-container {
            position: relative;
            width: 100%;
            height: 100%;
            min-height: 100vh;
            display: flex;
            flex-direction: column;
        }
        "#
    )
    .to_string()
}

/// App 内容区域样式
pub fn app_content_style() -> String {
    css!(
        r#"
        .ant-app-content {
            flex: 1;
            position: relative;
        }
        "#
    )
    .to_string()
}

/// App 头部样式
pub fn app_header_style() -> String {
    css!(
        r#"
        .ant-app-header {
            position: relative;
            background: #fff;
            border-bottom: 1px solid #f0f0f0;
        }
        "#
    )
    .to_string()
}

/// App 底部样式
pub fn app_footer_style() -> String {
    css!(
        r#"
        .ant-app-footer {
            position: relative;
            background: #fafafa;
            border-top: 1px solid #f0f0f0;
            text-align: center;
            color: rgba(0, 0, 0, 0.45);
            padding: 24px 50px;
        }
        "#
    )
    .to_string()
}

/// App 侧边栏样式
pub fn app_sidebar_style() -> String {
    css!(
        r#"
        .ant-app-sidebar {
            position: relative;
            background: #fff;
            border-right: 1px solid #f0f0f0;
        }
        "#
    )
    .to_string()
}

/// App 主体区域样式
pub fn app_main_style() -> String {
    css!(
        r#"
        .ant-app-main {
            position: relative;
            flex: 1;
            background: #fff;
            padding: 24px;
        }
        "#
    )
    .to_string()
}

/// 响应式样式
pub fn app_responsive_style() -> String {
    css!(
        r#"
        @media (max-width: 768px) {
            .ant-app-main {
                padding: 16px;
            }

            .ant-app-footer {
                padding: 16px 24px;
            }
        }

        @media (max-width: 576px) {
            .ant-app-main {
                padding: 12px;
            }

            .ant-app-footer {
                padding: 12px 16px;
            }
        }
        "#
    )
    .to_string()
}

/// 生成 App 样式
pub fn generate_app_style(theme: AppTheme) -> String {
    AppStyleGenerator::new().with_theme(theme).generate()
}

/// 生成 App CSS 样式
pub fn generate_app_css(theme: AppTheme) -> String {
    AppStyleGenerator::new().with_theme(theme).generate_css()
}

/// 默认 App 样式
pub fn default_app_style() -> String {
    AppStyleGenerator::new().generate()
}

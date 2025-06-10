//! App 组件样式模块
//!
//! 本模块包含 App 组件的所有样式定义，从组件逻辑中分离出来，
//! 提高代码的可维护性和复用性。

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
}

impl AppStyleGenerator {
    /// 创建新的 App 样式生成器
    pub fn new() -> Self {
        Self {
            theme: AppTheme::Default,
        }
    }

    /// 设置主题
    pub fn with_theme(mut self, theme: AppTheme) -> Self {
        self.theme = theme;
        self
    }

    /// 生成完整的 App 样式
    pub fn generate(&self) -> String {
        let mut classes = vec![self.base_style()];

        match self.theme {
            AppTheme::Default => {}
            AppTheme::Dark => classes.push(self.dark_theme_style()),
            AppTheme::Compact => classes.push(self.compact_theme_style()),
            AppTheme::DarkCompact => {
                classes.push(self.dark_theme_style());
                classes.push(self.compact_theme_style());
            }
        }

        classes.join(" ")
    }

    /// 基础样式
    fn base_style(&self) -> String {
        css!(
            r#"
            position: relative;
            width: 100%;
            height: 100%;
            color: rgba(0, 0, 0, 0.88);
            font-size: 14px;
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Noto Sans', sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol', 'Noto Color Emoji';
            line-height: 1.5714285714285714;
            font-variant: tabular-nums;
            font-feature-settings: 'tnum';
            background: #fff;
            box-sizing: border-box;

            &, * {
                box-sizing: border-box;
            }
            "#
        )
    }

    /// 暗色主题样式
    fn dark_theme_style(&self) -> String {
        css!(
            r#"
            background: #141414;
            color: rgba(255, 255, 255, 0.85);

            .ant-app-header {
                background: #1f1f1f;
                border-bottom-color: #303030;
            }

            .ant-app-footer {
                background: #1f1f1f;
                border-top-color: #303030;
                color: rgba(255, 255, 255, 0.45);
            }

            .ant-app-sidebar {
                background: #1f1f1f;
                border-right-color: #303030;
            }

            .ant-app-main {
                background: #141414;
            }
            "#
        )
    }

    /// 紧凑主题样式
    fn compact_theme_style(&self) -> String {
        css!(
            r#"
            font-size: 12px;

            .ant-app-main {
                padding: 16px;
            }
            "#
        )
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
}

/// 生成完整的 App 组件样式
pub fn generate_app_style(theme: AppTheme) -> String {
    format!(
        "{} {} {} {} {} {} {} {}",
        AppStyleGenerator::new().with_theme(theme).generate(),
        app_container_style(),
        app_content_style(),
        app_header_style(),
        app_footer_style(),
        app_sidebar_style(),
        app_main_style(),
        app_responsive_style(),
    )
}

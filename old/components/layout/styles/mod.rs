//! Layout 布局组件样式模块
//!
//! 本模块定义了Layout组件的样式，包括Layout、Header、Sider、Content和Footer的样式。
//! 使用CSS-in-Rust方式实现，替代原有的CSS文件。

use crate::shared::styles::mixins;
use crate::shared::styles::tokens::DesignToken;
use css_in_rust::css;

/// Sider主题
pub enum SiderTheme {
    /// 亮色主题
    Light,
    /// 暗色主题
    Dark,
}

impl Default for SiderTheme {
    fn default() -> Self {
        Self::Dark
    }
}

/// Layout样式生成器
pub struct LayoutStyleGenerator {
    /// 是否有Sider
    has_sider: bool,
}

impl LayoutStyleGenerator {
    /// 创建新的样式生成器
    pub fn new() -> Self {
        Self { has_sider: false }
    }

    /// 设置是否有Sider
    pub fn with_has_sider(mut self, has_sider: bool) -> Self {
        self.has_sider = has_sider;
        self
    }

    /// 生成Layout样式
    pub fn generate(&self) -> String {
        let mut classes = vec!["ant-layout".to_string()];

        if self.has_sider {
            classes.push("ant-layout-has-sider".to_string());
        }

        classes.join(" ")
    }

    /// 基础样式
    pub fn base_style() -> String {
        css!(
            r#"
            display: flex;
            flex: auto;
            flex-direction: column;
            min-height: 0;
            background: #f0f2f5;
            box-sizing: border-box;

            &.ant-layout-has-sider {
                flex-direction: row;
            }
            "#
        )
        .to_string()
    }
}

/// Header样式生成器
pub struct HeaderStyleGenerator {
    /// 是否为亮色主题
    light: bool,
    /// 是否固定头部
    fixed: bool,
}

impl HeaderStyleGenerator {
    /// 创建新的样式生成器
    pub fn new() -> Self {
        Self {
            light: false,
            fixed: false,
        }
    }

    /// 设置是否为亮色主题
    pub fn with_light(mut self, light: bool) -> Self {
        self.light = light;
        self
    }

    /// 设置是否固定头部
    pub fn with_fixed(mut self, fixed: bool) -> Self {
        self.fixed = fixed;
        self
    }

    /// 生成Header样式
    pub fn generate(&self) -> String {
        let mut classes = vec!["ant-layout-header".to_string()];

        if self.light {
            classes.push("ant-layout-header-light".to_string());
        }

        if self.fixed {
            classes.push("ant-layout-header-fixed".to_string());
        }

        classes.join(" ")
    }

    /// 基础样式
    pub fn base_style() -> String {
        css!(
            r#"
            height: 64px;
            padding: 0 50px;
            color: rgba(255, 255, 255, 0.85);
            line-height: 64px;
            background: #001529;

            &.ant-layout-header-light {
                background: #fff;
                color: rgba(0, 0, 0, 0.85);
                border-bottom: 1px solid #f0f0f0;
            }

            &.ant-layout-header-fixed {
                position: fixed;
                top: 0;
                z-index: 1;
                width: 100%;
            }

            @media screen and (max-width: 576px) {
                padding: 0 20px;
            }
            "#
        )
        .to_string()
    }
}

/// Sider样式生成器
pub struct SiderStyleGenerator {
    /// 主题
    theme: SiderTheme,
    /// 是否可折叠
    collapsible: bool,
    /// 是否已折叠
    collapsed: bool,
    /// 是否有触发器
    has_trigger: bool,
    /// 是否为左侧布局
    left: bool,
}

impl SiderStyleGenerator {
    /// 创建新的样式生成器
    pub fn new() -> Self {
        Self {
            theme: SiderTheme::Dark,
            collapsible: false,
            collapsed: false,
            has_trigger: false,
            left: true,
        }
    }

    /// 设置主题
    pub fn with_theme(mut self, theme: SiderTheme) -> Self {
        self.theme = theme;
        self
    }

    /// 设置是否可折叠
    pub fn with_collapsible(mut self, collapsible: bool) -> Self {
        self.collapsible = collapsible;
        self
    }

    /// 设置是否已折叠
    pub fn with_collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = collapsed;
        self
    }

    /// 设置是否有触发器
    pub fn with_has_trigger(mut self, has_trigger: bool) -> Self {
        self.has_trigger = has_trigger;
        self
    }

    /// 设置是否为左侧布局
    pub fn with_left(mut self, left: bool) -> Self {
        self.left = left;
        self
    }

    /// 生成Sider样式
    pub fn generate(&self) -> String {
        let mut classes = vec!["ant-layout-sider".to_string()];

        match self.theme {
            SiderTheme::Light => classes.push("ant-layout-sider-light".to_string()),
            SiderTheme::Dark => classes.push("ant-layout-sider-dark".to_string()),
        }

        if self.collapsible {
            classes.push("ant-layout-sider-collapsible".to_string());
        }

        if self.collapsed {
            classes.push("ant-layout-sider-collapsed".to_string());
        }

        if self.has_trigger {
            classes.push("ant-layout-sider-has-trigger".to_string());
        }

        if !self.left {
            classes.push("ant-layout-sider-right".to_string());
        }

        classes.join(" ")
    }

    /// 基础样式
    pub fn base_style() -> String {
        css!(
            r#"
            position: relative;
            min-width: 0;
            background: #001529;
            transition: all 0.2s;

            /* 子元素盒模型设置 */
            * {
                box-sizing: border-box;
            }

            &.ant-layout-sider-collapsed {
                min-width: 80px !important;
                max-width: 80px !important;
                width: 80px !important;
            }

            &.ant-layout-sider-has-trigger {
                padding-bottom: 48px;
            }

            &.ant-layout-sider-right {
                order: 1;
            }

            &.ant-layout-sider-light {
                background: #fff;

                .ant-layout-sider-trigger {
                    color: rgba(0, 0, 0, 0.85);
                    background: #fff;
                    border-top: 1px solid #f0f0f0;
                }
            }

            .ant-layout-sider-trigger {
                position: absolute;
                bottom: 0;
                z-index: 1;
                height: 48px;
                color: #fff;
                line-height: 48px;
                text-align: center;
                background: #002140;
                cursor: pointer;
                transition: all 0.2s;

                &:hover {
                    background: rgba(0, 0, 0, 0.1);
                }
            }
            "#
        )
        .to_string()
    }
}

/// Content样式生成器
pub struct ContentStyleGenerator {
    /// 内边距大小
    padding: Option<String>,
    /// 背景色
    background: Option<String>,
    /// 是否有边框
    bordered: bool,
    /// 是否有阴影
    shadow: bool,
}

impl ContentStyleGenerator {
    /// 创建新的样式生成器
    pub fn new() -> Self {
        Self {
            padding: None,
            background: None,
            bordered: false,
            shadow: false,
        }
    }

    /// 设置内边距大小
    pub fn with_padding(mut self, padding: Option<String>) -> Self {
        self.padding = padding;
        self
    }

    /// 设置背景色
    pub fn with_background(mut self, background: Option<String>) -> Self {
        self.background = background;
        self
    }

    /// 设置是否有边框
    pub fn with_bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }

    /// 设置是否有阴影
    pub fn with_shadow(mut self, shadow: bool) -> Self {
        self.shadow = shadow;
        self
    }

    /// 生成Content样式
    pub fn generate(&self) -> String {
        let mut classes = vec!["ant-layout-content".to_string()];

        if let Some(padding) = &self.padding {
            match padding.as_str() {
                "small" => classes.push("ant-layout-content-padded-sm".to_string()),
                "large" => classes.push("ant-layout-content-padded-lg".to_string()),
                _ => classes.push("ant-layout-content-padded".to_string()),
            }
        }

        if let Some(background) = &self.background {
            match background.as_str() {
                "white" => classes.push("ant-layout-content-white".to_string()),
                "gray" => classes.push("ant-layout-content-gray".to_string()),
                "transparent" => classes.push("ant-layout-content-transparent".to_string()),
                _ => {}
            }
        }

        if self.bordered {
            classes.push("ant-layout-content-bordered".to_string());
        }

        if self.shadow {
            classes.push("ant-layout-content-shadow".to_string());
        }

        classes.join(" ")
    }

    /// 基础样式
    pub fn base_style() -> String {
        css!(
            r#"
            flex: auto;
            min-height: 0;
            color: rgba(0, 0, 0, 0.85);
            background: transparent;
            box-sizing: border-box;

            &.ant-layout-content-padded {
                padding: 24px;
            }

            &.ant-layout-content-padded-sm {
                padding: 12px;
            }

            &.ant-layout-content-padded-lg {
                padding: 32px;
            }

            &.ant-layout-content-white {
                background: #fff;
            }

            &.ant-layout-content-gray {
                background: #f0f2f5;
            }

            &.ant-layout-content-transparent {
                background: transparent;
            }

            &.ant-layout-content-bordered {
                border: 1px solid #f0f0f0;
            }

            &.ant-layout-content-shadow {
                box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
            }
            "#
        )
        .to_string()
    }
}

/// Footer样式生成器
pub struct FooterStyleGenerator {
    /// 主题
    theme: Option<String>,
}

impl FooterStyleGenerator {
    /// 创建新的样式生成器
    pub fn new() -> Self {
        Self { theme: None }
    }

    /// 设置主题
    pub fn with_theme(mut self, theme: Option<String>) -> Self {
        self.theme = theme;
        self
    }

    /// 生成Footer样式
    pub fn generate(&self) -> String {
        let mut classes = vec!["ant-layout-footer".to_string()];

        if let Some(theme) = &self.theme {
            match theme.as_str() {
                "light" => classes.push("ant-layout-footer-light".to_string()),
                "dark" => classes.push("ant-layout-footer-dark".to_string()),
                "transparent" => classes.push("ant-layout-footer-transparent".to_string()),
                _ => {}
            }
        }

        classes.join(" ")
    }

    /// 基础样式
    pub fn base_style() -> String {
        css!(
            r#"
            padding: 24px 50px;
            color: rgba(0, 0, 0, 0.85);
            background: #f0f2f5;
            display: flex;
            flex-direction: column;
            box-sizing: border-box;

            &.ant-layout-footer-light {
                background: #fff;
                border-top: 1px solid #f0f0f0;
            }

            &.ant-layout-footer-dark {
                background: #001529;
                color: rgba(255, 255, 255, 0.85);
            }

            &.ant-layout-footer-transparent {
                background: transparent;
            }

            .ant-layout-footer-content {
                display: flex;
                justify-content: center;
                align-items: flex-start;
            }

            @media screen and (max-width: 576px) {
                padding: 24px 20px;
            }
            "#
        )
        .to_string()
    }
}

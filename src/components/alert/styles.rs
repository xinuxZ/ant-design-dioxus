//! Alert 组件的样式定义
//!
//! Alert组件样式管理
//!
//! 使用样式生成器模式管理Alert组件的样式
//! 使用 css! 宏的样式生成器模式，提供 Alert 组件的完整样式生成功能

use crate::components::alert::types::*;
use crate::shared::styles::mixins::*;
use css_in_rust::css;

/// 警告提示样式生成器
pub struct AlertStyleGenerator {
    pub alert_type: AlertType,
    pub size: AlertSize,
    pub closable: bool,
    pub show_icon: bool,
    pub banner: bool,
    pub action: bool,
    pub description: bool,
}

impl AlertStyleGenerator {
    /// 创建新的警告提示样式生成器
    pub fn new() -> Self {
        Self {
            alert_type: AlertType::Info,
            size: AlertSize::Default,
            closable: false,
            show_icon: false,
            banner: false,
            action: false,
            description: false,
        }
    }

    /// 设置警告提示类型
    pub fn with_type(mut self, alert_type: AlertType) -> Self {
        self.alert_type = alert_type;
        self
    }

    /// 设置警告提示尺寸
    pub fn with_size(mut self, size: AlertSize) -> Self {
        self.size = size;
        self
    }

    /// 设置是否可关闭
    pub fn with_closable(mut self, closable: bool) -> Self {
        self.closable = closable;
        self
    }

    /// 设置是否显示图标
    pub fn with_show_icon(mut self, show_icon: bool) -> Self {
        self.show_icon = show_icon;
        self
    }

    /// 设置是否为横幅模式
    pub fn with_banner(mut self, banner: bool) -> Self {
        self.banner = banner;
        self
    }

    /// 设置是否有操作
    pub fn with_action(mut self, action: bool) -> Self {
        self.action = action;
        self
    }

    /// 设置是否有描述
    pub fn with_description(mut self, description: bool) -> Self {
        self.description = description;
        self
    }

    /// 生成完整的警告提示样式类名
    pub fn generate(&self) -> String {
        let mut classes = vec![self.base_style(), self.type_style(), self.size_style()];

        if self.closable {
            classes.push(self.closable_style());
        }

        if self.show_icon {
            classes.push(self.icon_style());
        }

        if self.banner {
            classes.push(self.banner_style());
        }

        if self.action {
            classes.push(self.action_style());
        }

        if self.description {
            classes.push(self.description_style());
        }

        classes.join(" ")
    }

    /// 基础警告提示样式
    fn base_style(&self) -> String {
        css!(
            r#"
            position: relative;
            padding: 8px 15px;
            margin-bottom: 16px;
            border: 1px solid transparent;
            border-radius: 6px;
            transition: all 0.3s;
            font-size: 14px;
            line-height: 1.5715;
            word-wrap: break-word;
            display: flex;
            align-items: flex-start;
            "#
        )
        .to_string()
    }

    /// 警告提示类型样式
    fn type_style(&self) -> String {
        match self.alert_type {
            AlertType::Success => css!(
                r#"
                background-color: #f6ffed;
                border-color: #b7eb8f;
                color: #52c41a;
                "#
            ),
            AlertType::Info => css!(
                r#"
                background-color: #e6f7ff;
                border-color: #91d5ff;
                color: #1890ff;
                "#
            ),
            AlertType::Warning => css!(
                r#"
                background-color: #fffbe6;
                border-color: #ffe58f;
                color: #faad14;
                "#
            ),
            AlertType::Error => css!(
                r#"
                background-color: #fff2f0;
                border-color: #ffccc7;
                color: #ff4d4f;
                "#
            ),
        }
        .to_string()
    }

    /// 警告提示尺寸样式
    fn size_style(&self) -> String {
        match self.size {
            AlertSize::Small => css!(
                r#"
                padding: 4px 8px;
                font-size: 12px;
                "#
            ),
            AlertSize::Default => String::new(),
            AlertSize::Large => css!(
                r#"
                padding: 12px 20px;
                font-size: 16px;
                "#
            ),
        }
        .to_string()
    }

    /// 可关闭样式
    fn closable_style(&self) -> String {
        css!(
            r#"
            padding-right: 24px;
            "#
        )
        .to_string()
    }

    /// 图标样式
    fn icon_style(&self) -> String {
        css!(
            r#"
            padding-left: 38px;
            "#
        )
        .to_string()
    }

    /// 横幅样式
    fn banner_style(&self) -> String {
        css!(
            r#"
            border: 0;
            border-radius: 0;
            margin-bottom: 0;
            "#
        )
        .to_string()
    }

    /// 操作样式
    fn action_style(&self) -> String {
        css!(
            r#"
            align-items: center;
            "#
        )
        .to_string()
    }

    /// 描述样式
    fn description_style(&self) -> String {
        css!(
            r#"
            align-items: flex-start;
            padding: 12px 15px;
            "#
        )
        .to_string()
    }
}

impl Default for AlertStyleGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// 生成主题样式
///
/// 根据主题配置生成自定义样式

pub fn generate_theme_styles(theme: &AlertTheme) -> String {
    format!(
        r#"
        --alert-success-color: {};
        --alert-info-color: {};
        --alert-warning-color: {};
        --alert-error-color: {};
        --alert-success-bg: #f6ffed;
        --alert-info-bg: #e6f7ff;
        --alert-warning-bg: #fffbe6;
        --alert-error-bg: #fff2f0;
        --alert-success-border: #b7eb8f;
        --alert-info-border: #91d5ff;
        --alert-warning-border: #ffe58f;
        --alert-error-border: #ffccc7;
        --alert-font-size: {};
        --alert-icon-size: {};
        --alert-close-size: 12px;
        --alert-padding: 8px 15px;
        --alert-margin: 0 0 16px 0;
        --alert-border-radius: {};
        --alert-transition: all 0.3s;
        "#,
        theme.success_color,
        theme.info_color,
        theme.warning_color,
        theme.error_color,
        theme.font_size,
        theme.font_size,
        theme.border_radius
    )
}

/// 生成动画样式
pub fn animation_styles() -> String {
    css!(
        r#"
        opacity: 0;
        transform: scaleY(0);
        transform-origin: 0 0;
        transition: opacity 0.3s, transform 0.3s;
        "#
    )
    .to_string()
}

/// 生成暗色主题样式
pub fn dark_theme_styles() -> String {
    css!(
        r#"
        background-color: #1f1f1f;
        border-color: #434343;
        color: rgba(255, 255, 255, 0.85);
        "#
    )
}

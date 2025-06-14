//! Badge 徽标数组件样式

use crate::theme::AliasToken;
use css_in_rust::css;

/// 徽标状态类型
#[derive(Debug, Clone, PartialEq)]
pub enum BadgeStatus {
    /// 成功状态
    Success,
    /// 处理中状态
    Processing,
    /// 默认状态
    Default,
    /// 错误状态
    Error,
    /// 警告状态
    Warning,
}

impl Default for BadgeStatus {
    fn default() -> Self {
        Self::Default
    }
}

/// 徽标尺寸
#[derive(Debug, Clone, PartialEq)]
pub enum BadgeSize {
    /// 默认尺寸
    Default,
    /// 小尺寸
    Small,
}

impl Default for BadgeSize {
    fn default() -> Self {
        Self::Default
    }
}

/// Badge 样式生成器
pub struct BadgeStyleGenerator {
    /// 徽标状态
    pub status: Option<BadgeStatus>,
    /// 徽标尺寸
    pub size: BadgeSize,
    /// 是否为小红点模式
    pub dot: bool,
    /// 徽标颜色
    pub color: Option<String>,
    /// 是否为 RTL 模式
    pub rtl: bool,
    /// 是否显示零值
    pub show_zero: bool,
    /// 主题令牌
    pub token: AliasToken,
}

impl Default for BadgeStyleGenerator {
    fn default() -> Self {
        Self {
            status: None,
            size: BadgeSize::Default,
            dot: false,
            color: None,
            rtl: false,
            show_zero: false,
            token: AliasToken::default(),
        }
    }
}

impl BadgeStyleGenerator {
    /// 创建一个新的 Badge 样式生成器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置徽标状态
    pub fn with_status(mut self, status: Option<BadgeStatus>) -> Self {
        self.status = status;
        self
    }

    /// 设置徽标尺寸
    pub fn with_size(mut self, size: BadgeSize) -> Self {
        self.size = size;
        self
    }

    /// 设置是否为小红点模式
    pub fn with_dot(mut self, dot: bool) -> Self {
        self.dot = dot;
        self
    }

    /// 设置徽标颜色
    pub fn with_color(mut self, color: Option<String>) -> Self {
        self.color = color;
        self
    }

    /// 设置是否为 RTL 模式
    pub fn with_rtl(mut self, rtl: bool) -> Self {
        self.rtl = rtl;
        self
    }

    /// 设置是否显示零值
    pub fn with_show_zero(mut self, show_zero: bool) -> Self {
        self.show_zero = show_zero;
        self
    }

    /// 设置主题令牌
    pub fn with_token(mut self, token: AliasToken) -> Self {
        self.token = token;
        self
    }

    /// 生成样式类名
    pub fn generate(&self) -> String {
        let mut classes = vec!["ant-badge"];

        // 添加尺寸相关的类名
        if self.size == BadgeSize::Small {
            classes.push("ant-badge-small");
        }

        // 添加状态相关的类名
        if let Some(status) = &self.status {
            classes.push("ant-badge-status");
            match status {
                BadgeStatus::Success => classes.push("ant-badge-status-success"),
                BadgeStatus::Processing => classes.push("ant-badge-status-processing"),
                BadgeStatus::Default => classes.push("ant-badge-status-default"),
                BadgeStatus::Error => classes.push("ant-badge-status-error"),
                BadgeStatus::Warning => classes.push("ant-badge-status-warning"),
            }
        }

        // 添加点模式类名
        if self.dot {
            classes.push("ant-badge-dot");
        }

        // 添加RTL类名
        if self.rtl {
            classes.push("ant-badge-rtl");
        }

        // 添加自定义颜色类名
        if self.color.is_some() {
            classes.push("ant-badge-custom-color");
        }

        classes.join(" ")
    }

    /// 生成 CSS 样式
    pub fn generate_css(&self) -> String {
        css!(
            r#"
            .ant-badge {
                position: relative;
                display: inline-block;
                line-height: 1;
            }

            .ant-badge-not-a-wrapper {
                vertical-align: middle;
            }

            .ant-badge-count {
                position: absolute;
                top: 0;
                right: 0;
                z-index: ${z_index_badge};
                min-width: 20px;
                height: 20px;
                padding: 0 6px;
                color: ${color_text_light};
                font-weight: normal;
                font-size: ${font_size_sm}px;
                line-height: 20px;
                white-space: nowrap;
                text-align: center;
                background: ${color_error};
                border-radius: 10px;
                box-shadow: 0 0 0 1px ${color_bg_container};
                transform: translate(50%, -50%);
                transform-origin: 100% 0%;
            }

            .ant-badge-count-content {
                display: inline-block;
            }

            .ant-badge-dot {
                position: absolute;
                top: 0;
                right: 0;
                z-index: ${z_index_badge};
                width: 6px;
                height: 6px;
                background: ${color_error};
                border-radius: 100%;
                box-shadow: 0 0 0 1px ${color_bg_container};
                transform: translate(50%, -50%);
                transform-origin: 100% 0%;
            }

            .ant-badge-small .ant-badge-count {
                min-width: 14px;
                height: 14px;
                padding: 0 4px;
                font-size: ${font_size_sm}px;
                line-height: 14px;
                border-radius: 7px;
            }

            .ant-badge-small .ant-badge-dot {
                width: 4px;
                height: 4px;
            }

            .ant-badge-status {
                line-height: inherit;
                vertical-align: baseline;
            }

            .ant-badge-status-dot {
                position: relative;
                top: -1px;
                display: inline-block;
                width: 6px;
                height: 6px;
                border-radius: 50%;
                vertical-align: middle;
            }

            .ant-badge-status-success .ant-badge-status-dot {
                background: ${color_success};
            }

            .ant-badge-status-processing .ant-badge-status-dot {
                position: relative;
                background: ${color_primary};
            }

            .ant-badge-status-processing .ant-badge-status-dot::after {
                position: absolute;
                top: 0;
                left: 0;
                width: 100%;
                height: 100%;
                border: 1px solid ${color_primary};
                border-radius: 50%;
                animation: antStatusProcessing 1.2s infinite ease-in-out;
                content: '';
            }

            .ant-badge-status-default .ant-badge-status-dot {
                background: ${color_text_secondary};
            }

            .ant-badge-status-error .ant-badge-status-dot {
                background: ${color_error};
            }

            .ant-badge-status-warning .ant-badge-status-dot {
                background: ${color_warning};
            }

            .ant-badge-status-text {
                margin-left: ${margin_xs}px;
                color: ${color_text};
                font-size: ${font_size}px;
            }

            .ant-badge-zoom-appear,
            .ant-badge-zoom-enter {
                animation: antZoomBadgeIn 0.3s cubic-bezier(0.12, 0.4, 0.29, 1.46);
                animation-fill-mode: both;
            }

            .ant-badge-zoom-leave {
                animation: antZoomBadgeOut 0.3s cubic-bezier(0.71, -0.46, 0.88, 0.6);
                animation-fill-mode: both;
            }

            .ant-badge-not-a-wrapper .ant-badge-count {
                transform: none;
            }

            .ant-badge-count-overflow-max {
                color: ${color_text_light};
            }

            .ant-badge-multiple-words {
                padding: 0 8px;
            }

            .ant-badge-rtl {
                direction: rtl;
            }

            .ant-badge-rtl .ant-badge-count,
            .ant-badge-rtl .ant-badge-dot {
                right: auto;
                left: 0;
                transform: translate(-50%, -50%);
                transform-origin: 0% 0%;
            }

            .ant-badge-rtl.ant-badge:not(.ant-badge-not-a-wrapper) .ant-badge-count {
                transform: translate(-50%, -50%);
                transform-origin: 0% 0%;
            }

            .ant-badge-rtl .ant-badge-status-text {
                margin-right: ${margin_xs}px;
                margin-left: 0;
            }

            .ant-badge:not(.ant-badge-status):hover .ant-badge-count {
                box-shadow: 0 0 0 1px ${color_error_hover};
            }

            .ant-badge:not(.ant-badge-status):hover .ant-badge-dot {
                box-shadow: 0 0 0 1px ${color_error_hover};
            }

            @media (prefers-contrast: high) {
                .ant-badge-count {
                    box-shadow: 0 0 0 2px #fff;
                }

                .ant-badge-dot {
                    box-shadow: 0 0 0 2px #fff;
                }
            }

            @keyframes antStatusProcessing {
                0% {
                    transform: scale(0.8);
                    opacity: 0.5;
                }
                100% {
                    transform: scale(2.4);
                    opacity: 0;
                }
            }

            @keyframes antZoomBadgeIn {
                0% {
                    transform: scale(0) translate(50%, -50%);
                    opacity: 0;
                }
                100% {
                    transform: scale(1) translate(50%, -50%);
                }
            }

            @keyframes antZoomBadgeOut {
                0% {
                    transform: scale(1) translate(50%, -50%);
                }
                100% {
                    transform: scale(0) translate(50%, -50%);
                    opacity: 0;
                }
            }

            ${custom_color_style}
            "#,
            z_index_badge = self.token.z_index_badge,
            color_text_light = self.token.color_text_light,
            font_size_sm = self.token.font_size_sm,
            color_error = self.token.color_error,
            color_bg_container = self.token.color_bg_container,
            color_success = self.token.color_success,
            color_primary = self.token.color_primary,
            color_text_secondary = self.token.color_text_secondary,
            color_warning = self.token.color_warning,
            margin_xs = self.token.margin_xs,
            color_text = self.token.color_text,
            font_size = self.token.font_size,
            color_error_hover = self.token.color_error_hover,
            custom_color_style = self.generate_custom_color_style()
        )
        .to_string()
    }

    /// 生成自定义颜色样式
    fn generate_custom_color_style(&self) -> String {
        if let Some(color) = &self.color {
            css!(
                r#"
                .ant-badge-custom-color .ant-badge-count,
                .ant-badge-custom-color .ant-badge-dot {
                    background: ${color};
                }
                "#,
                color = color
            )
            .to_string()
        } else {
            String::new()
        }
    }
}

/// 生成徽标样式
pub fn generate_badge_style(
    status: Option<BadgeStatus>,
    size: BadgeSize,
    dot: bool,
    color: Option<String>,
    rtl: bool,
    show_zero: bool,
) -> String {
    BadgeStyleGenerator::new()
        .with_status(status)
        .with_size(size)
        .with_dot(dot)
        .with_color(color)
        .with_rtl(rtl)
        .with_show_zero(show_zero)
        .generate()
}

/// 生成徽标 CSS 样式
pub fn generate_badge_css(
    status: Option<BadgeStatus>,
    size: BadgeSize,
    dot: bool,
    color: Option<String>,
    rtl: bool,
    show_zero: bool,
) -> String {
    BadgeStyleGenerator::new()
        .with_status(status)
        .with_size(size)
        .with_dot(dot)
        .with_color(color)
        .with_rtl(rtl)
        .with_show_zero(show_zero)
        .generate_css()
}

/// 默认徽标样式
pub fn default_badge_style() -> String {
    BadgeStyleGenerator::new().generate()
}

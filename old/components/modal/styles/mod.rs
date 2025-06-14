//! Modal 对话框组件样式

use crate::theme::AliasToken;
use css_in_rust::css;

/// Modal 样式生成器
pub struct ModalStyleGenerator {
    /// 是否垂直居中
    pub centered: bool,
    /// z-index 值
    pub z_index: i32,
    /// 宽度
    pub width: Option<String>,
    /// 主题令牌
    pub token: AliasToken,
}

impl Default for ModalStyleGenerator {
    fn default() -> Self {
        Self {
            centered: true,
            z_index: 1000,
            width: None,
            token: AliasToken::default(),
        }
    }
}

impl ModalStyleGenerator {
    /// 创建新的样式生成器
    pub fn new() -> Self {
        Default::default()
    }

    /// 设置是否垂直居中
    pub fn with_centered(mut self, centered: bool) -> Self {
        self.centered = centered;
        self
    }

    /// 设置z-index值
    pub fn with_z_index(mut self, z_index: i32) -> Self {
        self.z_index = z_index;
        self
    }

    /// 设置宽度
    pub fn with_width(mut self, width: Option<String>) -> Self {
        self.width = width;
        self
    }

    /// 设置主题令牌
    pub fn with_token(mut self, token: AliasToken) -> Self {
        self.token = token;
        self
    }

    /// 生成样式类名
    pub fn generate(&self) -> String {
        let mut classes = vec!["ant-modal".to_string()];

        if self.centered {
            classes.push("ant-modal-centered".to_string());
        }

        classes.join(" ")
    }

    /// 生成 CSS 样式
    pub fn generate_css(&self) -> String {
        // 根据传入的宽度设置弹窗宽度样式
        let width_style = match &self.width {
            Some(w) => w.to_string(),
            None => "520px".to_string(),
        };

        css!(
            r#"
            .ant-modal-root {
                position: fixed;
                top: 0;
                left: 0;
                right: 0;
                bottom: 0;
                z-index: ${z_index};
            }

            .ant-modal-mask {
                position: fixed;
                top: 0;
                left: 0;
                right: 0;
                bottom: 0;
                background-color: ${mask_bg_color};
                z-index: ${z_index};
                animation: ant-modal-mask-fade-in 0.2s ease-out;
            }

            .ant-modal-wrap {
                position: fixed;
                top: 0;
                left: 0;
                right: 0;
                bottom: 0;
                overflow: auto;
                outline: 0;
                z-index: ${z_index};
                display: flex;
                align-items: flex-start;
                justify-content: center;
                padding: 100px 0 40px;
            }

            .ant-modal-wrap.ant-modal-centered {
                align-items: center;
                padding: 0;
            }

            .ant-modal {
                position: relative;
                width: ${width};
                max-width: calc(100vw - ${margin_lg}px);
                margin: 0 auto;
                animation: ant-modal-zoom-in 0.2s ease-out;
            }

            .ant-modal-content {
                position: relative;
                background-color: ${bg_color_container};
                background-clip: padding-box;
                border: 0;
                border-radius: ${border_radius}px;
                box-shadow: ${box_shadow};
                pointer-events: auto;
            }

            .ant-modal-close {
                position: absolute;
                top: 0;
                right: 0;
                z-index: 10;
                padding: 0;
                color: ${color_text_secondary};
                font-weight: 700;
                line-height: 1;
                text-decoration: none;
                background: transparent;
                border: 0;
                outline: 0;
                cursor: pointer;
                transition: color 0.2s;
                width: ${height_lg}px;
                height: ${height_lg}px;
                display: flex;
                align-items: center;
                justify-content: center;
            }

            .ant-modal-close:hover {
                color: ${color_text};
            }

            .ant-modal-close-x {
                display: flex;
                align-items: center;
                justify-content: center;
                width: ${font_size_lg}px;
                height: ${font_size_lg}px;
                font-size: ${font_size}px;
                font-style: normal;
                line-height: ${font_size_lg}px;
                text-align: center;
                text-transform: none;
                text-rendering: auto;
            }

            .ant-modal-header {
                padding: ${padding_md}px ${padding_lg}px;
                color: ${color_text};
                background: ${bg_color_container};
                border-bottom: 1px solid ${color_split};
                border-radius: ${border_radius}px ${border_radius}px 0 0;
            }

            .ant-modal-title {
                margin: 0;
                color: ${color_text};
                font-weight: 600;
                font-size: ${font_size_lg}px;
                line-height: ${line_height};
                word-wrap: break-word;
            }

            .ant-modal-body {
                padding: ${padding_lg}px;
                font-size: ${font_size}px;
                line-height: ${line_height};
                word-wrap: break-word;
            }

            .ant-modal-footer {
                padding: ${padding_sm}px ${padding_lg}px;
                text-align: right;
                background: transparent;
                border-top: 1px solid ${color_split};
                border-radius: 0 0 ${border_radius}px ${border_radius}px;
            }

            .ant-modal-footer .ant-btn + .ant-btn:not(.ant-dropdown-trigger) {
                margin-bottom: 0;
                margin-left: ${margin_sm}px;
            }

            .ant-modal-open {
                overflow: hidden;
            }

            .ant-modal-confirm .ant-modal-header {
                display: none;
            }

            .ant-modal-confirm .ant-modal-body {
                padding: ${padding_lg}px ${padding_lg}px ${padding_md}px;
            }

            .ant-modal-confirm-body-wrapper {
                display: flex;
            }

            .ant-modal-confirm-body {
                display: flex;
                flex-direction: column;
                flex-grow: 1;
            }

            .ant-modal-confirm-body > .anticon {
                display: inline-block;
                flex-shrink: 0;
                margin-right: ${margin_sm}px;
                font-size: ${font_size_lg}px;
            }

            .ant-modal-confirm-title {
                color: ${color_text};
                font-weight: 500;
                font-size: ${font_size_lg}px;
                line-height: 1.4;
            }

            .ant-modal-confirm-content {
                margin-top: ${margin_sm}px;
                color: ${color_text};
                font-size: ${font_size}px;
            }

            .ant-modal-confirm-btns {
                margin-top: ${margin_lg}px;
                text-align: right;
            }

            .ant-modal-confirm-btns .ant-btn + .ant-btn {
                margin-bottom: 0;
                margin-left: ${margin_sm}px;
            }

            .ant-modal-confirm-error .ant-modal-confirm-body > .anticon {
                color: ${color_error};
            }

            .ant-modal-confirm-warning .ant-modal-confirm-body > .anticon,
            .ant-modal-confirm-confirm .ant-modal-confirm-body > .anticon {
                color: ${color_warning};
            }

            .ant-modal-confirm-info .ant-modal-confirm-body > .anticon {
                color: ${color_info};
            }

            .ant-modal-confirm-success .ant-modal-confirm-body > .anticon {
                color: ${color_success};
            }

            @keyframes ant-modal-mask-fade-in {
                from {
                    opacity: 0;
                }
                to {
                    opacity: 1;
                }
            }

            @keyframes ant-modal-zoom-in {
                from {
                    transform: scale(0.8);
                    opacity: 0;
                }
                to {
                    transform: scale(1);
                    opacity: 1;
                }
            }

            @keyframes ant-modal-zoom-out {
                from {
                    transform: scale(1);
                    opacity: 1;
                }
                to {
                    transform: scale(0.8);
                    opacity: 0;
                }
            }

            @media (max-width: 575px) {
                .ant-modal {
                    max-width: calc(100vw - ${margin_sm * 2}px);
                    margin: 0 auto;
                }

                .ant-modal-centered .ant-modal {
                    flex: 1;
                }
            }

            .ant-modal-rtl .ant-modal-close {
                right: auto;
                left: 0;
            }

            .ant-modal-rtl .ant-modal-footer {
                text-align: left;
            }

            .ant-modal-rtl .ant-modal-footer .ant-btn + .ant-btn {
                margin-right: ${margin_sm}px;
                margin-left: 0;
            }

            .ant-modal-rtl .ant-modal-confirm-body {
                direction: rtl;
            }

            .ant-modal-rtl .ant-modal-confirm-body > .anticon {
                margin-right: 0;
                margin-left: ${margin_sm}px;
            }

            .ant-modal-rtl .ant-modal-confirm-btns {
                text-align: left;
            }

            .ant-modal-rtl .ant-modal-confirm-btns .ant-btn + .ant-btn {
                margin-right: ${margin_sm}px;
                margin-left: 0;
            }

            @media (prefers-color-scheme: dark) {
                .ant-modal-content {
                    background-color: ${bg_color_container_dark};
                }

                .ant-modal-header {
                    background-color: ${bg_color_container_dark};
                    border-bottom-color: ${color_split_dark};
                }

                .ant-modal-footer {
                    border-top-color: ${color_split_dark};
                }

                .ant-modal-title,
                .ant-modal-close,
                .ant-modal-confirm-title {
                    color: ${color_text_dark};
                }

                .ant-modal-close:hover {
                    color: ${color_text_secondary_dark};
                }

                .ant-modal-body,
                .ant-modal-confirm-content {
                    color: ${color_text_secondary_dark};
                }
            }

            @media (prefers-contrast: high) {
                .ant-modal {
                    outline: 2px solid #000;
                }
            }

            @media (prefers-reduced-motion: reduce) {
                .ant-modal-mask,
                .ant-modal {
                    animation: none !important;
                }

                .ant-modal-close,
                .ant-modal-footer .ant-btn {
                    transition: none !important;
                }
            }

            @media print {
                .ant-modal-mask,
                .ant-modal-wrap {
                    display: none !important;
                }
            }
            "#,
            z_index = self.z_index,
            mask_bg_color = "rgba(0, 0, 0, 0.45)",
            width = width_style,
            margin_lg = self.token.margin_lg,
            bg_color_container = self.token.color_bg_container,
            border_radius = self.token.border_radius,
            box_shadow = self.token.box_shadow,
            color_text_secondary = self.token.color_text_secondary,
            color_text = self.token.color_text,
            height_lg = self.token.height_lg,
            font_size_lg = self.token.font_size_lg,
            font_size = self.token.font_size,
            padding_md = self.token.padding_md,
            padding_lg = self.token.padding_lg,
            color_split = self.token.color_split,
            line_height = self.token.line_height,
            padding_sm = self.token.padding_sm,
            margin_sm = self.token.margin_sm,
            color_error = self.token.color_error,
            color_warning = self.token.color_warning,
            color_info = self.token.color_info,
            color_success = self.token.color_success,
            bg_color_container_dark = "#1f1f1f",
            color_split_dark = "#424242",
            color_text_dark = "rgba(255, 255, 255, 0.85)",
            color_text_secondary_dark = "rgba(255, 255, 255, 0.65)"
        )
        .to_string()
    }
}

/// 生成 Modal 样式
pub fn generate_modal_style(centered: bool, z_index: i32, width: Option<String>) -> String {
    ModalStyleGenerator::new()
        .with_centered(centered)
        .with_z_index(z_index)
        .with_width(width)
        .generate()
}

/// 生成 Modal CSS 样式
pub fn generate_modal_css(centered: bool, z_index: i32, width: Option<String>) -> String {
    ModalStyleGenerator::new()
        .with_centered(centered)
        .with_z_index(z_index)
        .with_width(width)
        .generate_css()
}

/// 默认 Modal 样式
pub fn default_modal_style() -> String {
    ModalStyleGenerator::new().generate()
}

/// Modal 样式构建器别名
pub type ModalStyleBuilder = ModalStyleGenerator;

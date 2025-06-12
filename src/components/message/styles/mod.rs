use crate::theme::AliasToken;
use css_in_rust::css;

pub struct MessageStyleGenerator {
    token: AliasToken,
}

impl MessageStyleGenerator {
    pub fn new() -> Self {
        Self {
            token: AliasToken::default(),
        }
    }

    pub fn generate_style(&self) -> String {
        css!(
            r#"
            /* Message 全局提示样式 */

            .ant-message {
              position: fixed;
              top: 8px;
              left: 0;
              right: 0;
              z-index: 1010;
              pointer-events: none;
              display: flex;
              flex-direction: column;
              align-items: center;
            }

            .ant-message-container {
              position: fixed;
              left: 0;
              right: 0;
              z-index: 1010;
              pointer-events: none;
              display: flex;
              flex-direction: column;
              align-items: center;
            }

            .ant-message-notice-wrapper {
              display: flex;
              justify-content: center;
              margin-bottom: ${margin_sm}px;
            }

            .ant-message-notice {
              position: relative;
              display: inline-block;
              padding: ${padding_sm}px ${padding_md}px;
              background: ${bg_color};
              border-radius: ${border_radius}px;
              box-shadow: ${box_shadow};
              pointer-events: auto;
              animation: ant-message-move-up-in 0.3s ease-out;
              transition: all 0.3s;
            }

            .ant-message-notice-content {
              display: flex;
              align-items: center;
            }

            .ant-message-custom-content {
              display: flex;
              align-items: center;
              font-size: ${font_size}px;
              line-height: 1.5715;
              color: ${color_text};
            }

            .ant-message-icon {
              display: inline-flex;
              align-items: center;
              margin-right: ${margin_sm}px;
              font-size: ${font_size_lg}px;
              line-height: 1;
            }

            .ant-message-icon-default {
              display: inline-block;
            }

            .ant-message-icon-loading {
              display: inline-block;
              animation: ant-message-loading-rotate 1s linear infinite;
            }

            .ant-message-content-text {
              flex: 1;
            }

            /* 消息类型样式 */
            .ant-message-success .ant-message-icon {
              color: ${color_success};
            }

            .ant-message-error .ant-message-icon {
              color: ${color_error};
            }

            .ant-message-warning .ant-message-icon {
              color: ${color_warning};
            }

            .ant-message-info .ant-message-icon {
              color: ${color_info};
            }

            .ant-message-loading .ant-message-icon {
              color: ${color_primary};
            }

            /* 隐藏状态 */
            .ant-message-hidden {
              opacity: 0;
              transform: translateY(-100%);
              pointer-events: none;
            }

            /* 动画效果 */
            @keyframes ant-message-move-up-in {
              0% {
                opacity: 0;
                transform: translateY(-100%);
              }

              100% {
                opacity: 1;
                transform: translateY(0);
              }
            }

            @keyframes ant-message-move-up-out {
              0% {
                opacity: 1;
                transform: translateY(0);
              }

              100% {
                opacity: 0;
                transform: translateY(-100%);
              }
            }

            @keyframes ant-message-loading-rotate {
              0% {
                transform: rotate(0deg);
              }

              100% {
                transform: rotate(360deg);
              }
            }

            /* 响应式设计 */
            @media (max-width: 768px) {
              .ant-message-notice {
                margin: 0 ${margin_md}px;
                max-width: calc(100vw - ${padding_md_2}px);
              }
            }

            /* RTL 支持 */
            .ant-message[dir="rtl"] .ant-message-icon {
              margin-right: 0;
              margin-left: ${margin_sm}px;
            }

            /* 高对比度模式支持 */
            @media (prefers-contrast: high) {
              .ant-message-notice {
                border: 1px solid rgba(0, 0, 0, 0.2);
              }
            }

            /* 减少动画模式支持 */
            @media (prefers-reduced-motion: reduce) {
              .ant-message-notice {
                animation: none;
              }

              .ant-message-icon-loading {
                animation: none;
              }
            }

            /* 深色主题支持 */
            @media (prefers-color-scheme: dark) {
              .ant-message-notice {
                background: ${bg_color_dark};
                color: ${color_text};
                box-shadow: ${box_shadow_dark};
              }

              .ant-message-custom-content {
                color: ${color_text};
              }
            }
            "#,
            margin_sm = self.token.margin_sm,
            padding_sm = self.token.padding_sm,
            padding_md = self.token.padding_md,
            bg_color = self.token.color_bg_container,
            border_radius = self.token.border_radius,
            box_shadow = self.token.box_shadow,
            font_size = self.token.font_size,
            color_text = self.token.color_text,
            font_size_lg = self.token.font_size_lg,
            color_success = self.token.color_success,
            color_error = self.token.color_error,
            color_warning = self.token.color_warning,
            color_info = self.token.color_info,
            color_primary = self.token.color_primary,
            margin_md = self.token.margin_md,
            padding_md_2 = self.token.padding_md * 2.0,
            bg_color_dark = self.token.color_bg_container,
            box_shadow_dark = self.token.box_shadow_secondary
        )
    }
}

pub fn use_message_style() -> String {
    MessageStyleGenerator::new().generate_style()
}

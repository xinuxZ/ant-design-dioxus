use crate::theme::{use_token, AliasToken};
use css_in_rust::Style;

pub struct MessageStyleGenerator {
    token: AliasToken,
}

impl MessageStyleGenerator {
    pub fn new() -> Self {
        Self { token: use_token() }
    }

    pub fn generate_style(&self) -> String {
        let style = Style::new(format!(
            r#"
            /* Message 全局提示样式 */

            .ant-message {{
              position: fixed;
              top: 8px;
              left: 0;
              right: 0;
              z-index: 1010;
              pointer-events: none;
              display: flex;
              flex-direction: column;
              align-items: center;
            }}

            .ant-message-container {{
              position: fixed;
              left: 0;
              right: 0;
              z-index: 1010;
              pointer-events: none;
              display: flex;
              flex-direction: column;
              align-items: center;
            }}

            .ant-message-notice-wrapper {{
              display: flex;
              justify-content: center;
              margin-bottom: {}px;
            }}

            .ant-message-notice {{
              position: relative;
              display: inline-block;
              padding: {}px {}px;
              background: {};
              border-radius: {}px;
              box-shadow: {};
              pointer-events: auto;
              animation: ant-message-move-up-in 0.3s ease-out;
              transition: all 0.3s;
            }}

            .ant-message-notice-content {{
              display: flex;
              align-items: center;
            }}

            .ant-message-custom-content {{
              display: flex;
              align-items: center;
              font-size: {}px;
              line-height: 1.5715;
              color: {};
            }}

            .ant-message-icon {{
              display: inline-flex;
              align-items: center;
              margin-right: {}px;
              font-size: {}px;
              line-height: 1;
            }}

            .ant-message-icon-default {{
              display: inline-block;
            }}

            .ant-message-icon-loading {{
              display: inline-block;
              animation: ant-message-loading-rotate 1s linear infinite;
            }}

            .ant-message-content-text {{
              flex: 1;
            }}

            /* 消息类型样式 */
            .ant-message-success .ant-message-icon {{
              color: {};
            }}

            .ant-message-error .ant-message-icon {{
              color: {};
            }}

            .ant-message-warning .ant-message-icon {{
              color: {};
            }}

            .ant-message-info .ant-message-icon {{
              color: {};
            }}

            .ant-message-loading .ant-message-icon {{
              color: {};
            }}

            /* 隐藏状态 */
            .ant-message-hidden {{
              opacity: 0;
              transform: translateY(-100%);
              pointer-events: none;
            }}

            /* 动画效果 */
            @keyframes ant-message-move-up-in {{
              0% {{
                opacity: 0;
                transform: translateY(-100%);
              }}

              100% {{
                opacity: 1;
                transform: translateY(0);
              }}
            }}

            @keyframes ant-message-move-up-out {{
              0% {{
                opacity: 1;
                transform: translateY(0);
              }}

              100% {{
                opacity: 0;
                transform: translateY(-100%);
              }}
            }}

            @keyframes ant-message-loading-rotate {{
              0% {{
                transform: rotate(0deg);
              }}

              100% {{
                transform: rotate(360deg);
              }}
            }}

            /* 响应式设计 */
            @media (max-width: 768px) {{
              .ant-message-notice {{
                margin: 0 {}px;
                max-width: calc(100vw - {}px);
              }}
            }}

            /* RTL 支持 */
            .ant-message[dir="rtl"] .ant-message-icon {{
              margin-right: 0;
              margin-left: {}px;
            }}

            /* 高对比度模式支持 */
            @media (prefers-contrast: high) {{
              .ant-message-notice {{
                border: 1px solid rgba(0, 0, 0, 0.2);
              }}
            }}

            /* 减少动画模式支持 */
            @media (prefers-reduced-motion: reduce) {{
              .ant-message-notice {{
                animation: none;
              }}

              .ant-message-icon-loading {{
                animation: none;
              }}
            }}

            /* 深色主题支持 */
            @media (prefers-color-scheme: dark) {{
              .ant-message-notice {{
                background: {};
                color: {};
                box-shadow: {};
              }}

              .ant-message-custom-content {{
                color: {};
              }}
            }}
            "#,
            self.token.margin_xs,                 // 消息项底部间距
            self.token.padding_xs,                // 消息项内边距(垂直)
            self.token.padding_md,                // 消息项内边距(水平)
            self.token.color_bg_container,        // 消息项背景色
            self.token.border_radius,             // 消息项边框圆角
            self.token.box_shadow_secondary,      // 消息项阴影
            self.token.font_size,                 // 消息内容字体大小
            self.token.color_text,                // 消息内容文本颜色
            self.token.margin_xs,                 // 消息图标右边距
            self.token.font_size + 2.0,           // 消息图标字体大小
            self.token.color_success,             // 成功消息图标颜色
            self.token.color_error,               // 错误消息图标颜色
            self.token.color_warning,             // 警告消息图标颜色
            self.token.color_primary,             // 信息消息图标颜色
            self.token.color_primary,             // 加载消息图标颜色
            self.token.padding_md,                // 移动端消息项边距
            self.token.padding_md * 2,            // 移动端消息项最大宽度计算
            self.token.margin_xs,                 // RTL模式图标左边距
            self.token.color_bg_container_dark,   // 深色主题背景色
            self.token.color_text_dark,           // 深色主题文本颜色
            self.token.box_shadow_secondary_dark, // 深色主题阴影
            self.token.color_text_dark,           // 深色主题消息内容文本颜色
        ))
        .unwrap();

        style.get_class_name()
    }
}

pub fn use_message_style() -> String {
    let style_generator = MessageStyleGenerator::new();
    style_generator.generate_style()
}

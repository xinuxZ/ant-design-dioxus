use crate::theme::AliasToken;
use css_in_rust::css;

pub struct InputNumberStyleGenerator {
    token: AliasToken,
}

impl InputNumberStyleGenerator {
    pub fn new() -> Self {
        Self {
            token: AliasToken::default(),
        }
    }

    pub fn generate_style(&self) -> String {
        css!(
            r#"
            /* InputNumber 数字输入框组件样式 */

            /* 基础容器样式 */
            .ant-input-number {
              position: relative;
              display: inline-block;
              width: 90px;
              margin: 0;
              padding: 0;
              color: ${color_text};
              font-size: ${font_size}px;
              line-height: 1.5714285714285714;
              list-style: none;
              font-family: ${font_family};
              background-color: ${bg_color};
              background-image: none;
              transition: all 0.2s;
              border: 1px solid ${border_color};
              border-radius: ${border_radius}px;
            }

            .ant-input-number:hover {
              border-color: ${hover_border_color};
              border-right-width: 1px;
            }

            .ant-input-number-focused {
              border-color: ${focus_border_color};
              box-shadow: 0 0 0 2px ${focus_shadow_color};
              border-right-width: 1px;
              outline: 0;
            }

            /* 小尺寸 */
            .ant-input-number-small {
              padding: 0;
              font-size: ${font_size_sm}px;
              border-radius: ${border_radius_sm}px;
            }

            .ant-input-number-small .ant-input-number-input {
              height: 22px;
              padding: 0 7px;
            }

            .ant-input-number-small .ant-input-number-handler-wrap {
              width: 22px;
            }

            .ant-input-number-small .ant-input-number-handler {
              height: 11px;
            }

            /* 大尺寸 */
            .ant-input-number-large {
              padding: 0;
              font-size: ${font_size_lg}px;
              border-radius: ${border_radius_lg}px;
            }

            .ant-input-number-large .ant-input-number-input {
              height: 38px;
              padding: 6.5px 11px;
            }

            .ant-input-number-large .ant-input-number-handler-wrap {
              width: 32px;
            }

            .ant-input-number-large .ant-input-number-handler {
              height: 19px;
            }

            /* 无边框 */
            .ant-input-number-borderless {
              border-color: transparent !important;
              box-shadow: none !important;
            }

            /* 禁用状态 */
            .ant-input-number-disabled {
              color: ${disabled_color};
              background-color: ${disabled_bg};
              border-color: ${disabled_border};
              box-shadow: none;
              cursor: not-allowed;
            }

            .ant-input-number-disabled:hover {
              border-color: ${disabled_border};
            }

            .ant-input-number-disabled .ant-input-number-input {
              cursor: not-allowed;
            }

            .ant-input-number-disabled .ant-input-number-handler-wrap {
              display: none;
            }

            /* 错误状态 */
            .ant-input-number-status-error {
              border-color: ${error_color};
            }

            .ant-input-number-status-error:hover {
              border-color: ${error_hover_color};
            }

            .ant-input-number-status-error.ant-input-number-focused {
              border-color: ${error_color};
              box-shadow: 0 0 0 2px ${error_shadow_color};
            }

            /* 警告状态 */
            .ant-input-number-status-warning {
              border-color: ${warning_color};
            }

            .ant-input-number-status-warning:hover {
              border-color: ${warning_hover_color};
            }

            .ant-input-number-status-warning.ant-input-number-focused {
              border-color: ${warning_color};
              box-shadow: 0 0 0 2px ${warning_shadow_color};
            }

            /* 输入框包装器 */
            .ant-input-number-input-wrap {
              position: relative;
              display: flex;
              align-items: center;
              width: 100%;
              height: 30px;
              padding: 0;
            }

            /* 输入框 */
            .ant-input-number-input {
              position: relative;
              display: inline-block;
              width: 100%;
              height: 30px;
              padding: 4px 11px;
              color: ${color_text};
              font-size: inherit;
              line-height: 1.5714285714285714;
              background-color: transparent;
              background-image: none;
              border: none;
              border-radius: ${border_radius}px;
              outline: none;
              transition: all 0.3s;
              appearance: textfield;
              -moz-appearance: textfield;
            }

            .ant-input-number-input::-webkit-outer-spin-button,
            .ant-input-number-input::-webkit-inner-spin-button {
              margin: 0;
              -webkit-appearance: none;
            }

            .ant-input-number-input::placeholder {
              color: ${placeholder_color};
              user-select: none;
            }

            .ant-input-number-input:placeholder-shown {
              text-overflow: ellipsis;
            }

            /* 前缀和后缀 */
            .ant-input-number-prefix,
            .ant-input-number-suffix {
              display: flex;
              flex: none;
              align-items: center;
              color: ${addon_color};
              line-height: 1;
              white-space: nowrap;
            }

            .ant-input-number-prefix {
              margin-right: 4px;
            }

            .ant-input-number-suffix {
              margin-left: 4px;
            }

            /* 处理器包装器 */
            .ant-input-number-handler-wrap {
              position: absolute;
              top: 0;
              right: 0;
              width: 22px;
              height: 100%;
              background: ${bg_color};
              border-left: 1px solid ${border_color};
              border-radius: 0 ${border_radius}px ${border_radius}px 0;
              opacity: 0;
              transition: opacity 0.2s;
            }

            .ant-input-number:hover .ant-input-number-handler-wrap,
            .ant-input-number-focused .ant-input-number-handler-wrap {
              opacity: 1;
            }

            /* 处理器按钮 */
            .ant-input-number-handler {
              position: relative;
              display: block;
              width: 100%;
              height: 15px;
              overflow: hidden;
              color: ${handler_color};
              font-weight: bold;
              line-height: 0;
              text-align: center;
              border-left: 0;
              cursor: pointer;
              transition: all 0.1s linear;
            }

            .ant-input-number-handler:active {
              background: ${handler_active_bg};
            }

            .ant-input-number-handler:hover {
              height: 40%;
            }

            .ant-input-number-handler-up {
              border-top-right-radius: ${border_radius}px;
              border-bottom: 1px solid ${border_color};
            }

            .ant-input-number-handler-down {
              border-bottom-right-radius: ${border_radius}px;
              border-top: 1px solid ${border_color};
            }

            /* 处理器图标 */
            .ant-input-number-handler-up-inner,
            .ant-input-number-handler-down-inner {
              display: inline-block;
              font-size: 7px;
              font-weight: bold;
              line-height: 15px;
              color: ${handler_icon_color};
              transition: all 0.1s linear;
            }

            .ant-input-number-handler:hover .ant-input-number-handler-up-inner,
            .ant-input-number-handler:hover .ant-input-number-handler-down-inner {
              color: ${handler_hover_color};
            }

            .ant-input-number-handler-up-disabled,
            .ant-input-number-handler-down-disabled {
              cursor: not-allowed;
            }

            .ant-input-number-handler-up-disabled .ant-input-number-handler-up-inner,
            .ant-input-number-handler-down-disabled .ant-input-number-handler-down-inner {
              color: ${handler_disabled_color};
            }

            /* 紧凑模式 */
            .ant-input-number-compact-item:not(.ant-input-number-compact-first-item):not(.ant-input-number-compact-last-item) {
              border-right-width: 1px;
            }

            .ant-input-number-compact-item:not(.ant-input-number-compact-last-item) {
              border-right-width: 0;
            }

            /* 响应式设计 */
            @media (max-width: 768px) {
              .ant-input-number {
                width: 80px;
              }
            }

            /* 高对比度模式支持 */
            @media (prefers-contrast: high) {
              .ant-input-number {
                border: 1px solid #000;
              }
              .ant-input-number-handler-wrap {
                border-left: 1px solid #000;
              }
              .ant-input-number-handler-up {
                border-bottom: 1px solid #000;
              }
              .ant-input-number-handler-down {
                border-top: 1px solid #000;
              }
            }

            /* 减少动画模式支持 */
            @media (prefers-reduced-motion: reduce) {
              .ant-input-number,
              .ant-input-number-input,
              .ant-input-number-handler-wrap,
              .ant-input-number-handler,
              .ant-input-number-handler-up-inner,
              .ant-input-number-handler-down-inner {
                transition: none;
              }
            }
            "#,
            color_text = self.token.color_text,
            font_size = self.token.font_size,
            font_family = self.token.font_family,
            bg_color = self.token.color_bg_container,
            border_color = self.token.color_border,
            border_radius = self.token.border_radius,
            hover_border_color = self.token.color_primary_hover,
            focus_border_color = self.token.color_primary,
            focus_shadow_color = self.token.color_primary_bg_hover,
            font_size_sm = self.token.font_size,
            border_radius_sm = self.token.border_radius_sm,
            font_size_lg = self.token.font_size_lg,
            border_radius_lg = self.token.border_radius,
            disabled_color = self.token.color_text_disabled,
            disabled_bg = self.token.color_bg_container,
            disabled_border = self.token.color_border,
            error_color = self.token.color_error,
            error_hover_color = self.token.color_border_hover,
            error_shadow_color = self.token.color_error_bg_hover,
            warning_color = self.token.color_warning,
            warning_hover_color = self.token.color_warning_hover,
            warning_shadow_color = self.token.color_warning_bg_hover,
            placeholder_color = self.token.color_text_placeholder,
            addon_color = self.token.color_text_secondary,
            handler_color = self.token.color_text_secondary,
            handler_active_bg = self.token.color_fill_tertiary,
            handler_icon_color = self.token.color_text_secondary,
            handler_hover_color = self.token.color_primary,
            handler_disabled_color = self.token.color_text_disabled
        )
    }
}

pub fn use_input_number_style() -> String {
    InputNumberStyleGenerator::new().generate_style()
}

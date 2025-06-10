use crate::theme::{use_token, AliasToken};
use css_in_rust::Style;

pub struct InputNumberStyleGenerator {
    token: AliasToken,
}

impl InputNumberStyleGenerator {
    pub fn new() -> Self {
        Self { token: use_token() }
    }

    pub fn generate_style(&self) -> String {
        let style = Style::new(format!(
            r#"
            /* InputNumber 数字输入框组件样式 */

            /* 基础容器样式 */
            .ant-input-number {{
              position: relative;
              display: inline-block;
              width: 90px;
              margin: 0;
              padding: 0;
              color: {};
              font-size: {}px;
              line-height: 1.5714285714285714;
              list-style: none;
              font-family: {};
              background-color: {};
              background-image: none;
              transition: all 0.2s;
              border: 1px solid {};
              border-radius: {}px;
            }}

            .ant-input-number:hover {{
              border-color: {};
              border-right-width: 1px;
            }}

            .ant-input-number-focused {{
              border-color: {};
              box-shadow: 0 0 0 2px {};
              border-right-width: 1px;
              outline: 0;
            }}

            /* 小尺寸 */
            .ant-input-number-small {{
              padding: 0;
              font-size: {}px;
              border-radius: {}px;
            }}

            .ant-input-number-small .ant-input-number-input {{
              height: 22px;
              padding: 0 7px;
            }}

            .ant-input-number-small .ant-input-number-handler-wrap {{
              width: 22px;
            }}

            .ant-input-number-small .ant-input-number-handler {{
              height: 11px;
            }}

            /* 大尺寸 */
            .ant-input-number-large {{
              padding: 0;
              font-size: {}px;
              border-radius: {}px;
            }}

            .ant-input-number-large .ant-input-number-input {{
              height: 38px;
              padding: 6.5px 11px;
            }}

            .ant-input-number-large .ant-input-number-handler-wrap {{
              width: 32px;
            }}

            .ant-input-number-large .ant-input-number-handler {{
              height: 19px;
            }}

            /* 无边框 */
            .ant-input-number-borderless {{
              border-color: transparent !important;
              box-shadow: none !important;
            }}

            /* 禁用状态 */
            .ant-input-number-disabled {{
              color: {};
              background-color: {};
              border-color: {};
              box-shadow: none;
              cursor: not-allowed;
            }}

            .ant-input-number-disabled:hover {{
              border-color: {};
            }}

            .ant-input-number-disabled .ant-input-number-input {{
              cursor: not-allowed;
            }}

            .ant-input-number-disabled .ant-input-number-handler-wrap {{
              display: none;
            }}

            /* 错误状态 */
            .ant-input-number-status-error {{
              border-color: {};
            }}

            .ant-input-number-status-error:hover {{
              border-color: {};
            }}

            .ant-input-number-status-error.ant-input-number-focused {{
              border-color: {};
              box-shadow: 0 0 0 2px {};
            }}

            /* 警告状态 */
            .ant-input-number-status-warning {{
              border-color: {};
            }}

            .ant-input-number-status-warning:hover {{
              border-color: {};
            }}

            .ant-input-number-status-warning.ant-input-number-focused {{
              border-color: {};
              box-shadow: 0 0 0 2px {};
            }}

            /* 输入框包装器 */
            .ant-input-number-input-wrap {{
              position: relative;
              display: flex;
              align-items: center;
              width: 100%;
              height: 30px;
              padding: 0;
            }}

            /* 输入框 */
            .ant-input-number-input {{
              position: relative;
              display: inline-block;
              width: 100%;
              height: 30px;
              padding: 4px 11px;
              color: {};
              font-size: inherit;
              line-height: 1.5714285714285714;
              background-color: transparent;
              background-image: none;
              border: none;
              border-radius: {}px;
              outline: none;
              transition: all 0.3s;
              appearance: textfield;
              -moz-appearance: textfield;
            }}

            .ant-input-number-input::-webkit-outer-spin-button,
            .ant-input-number-input::-webkit-inner-spin-button {{
              margin: 0;
              -webkit-appearance: none;
            }}

            .ant-input-number-input::placeholder {{
              color: {};
              user-select: none;
            }}

            .ant-input-number-input:placeholder-shown {{
              text-overflow: ellipsis;
            }}

            /* 前缀和后缀 */
            .ant-input-number-prefix,
            .ant-input-number-suffix {{
              display: flex;
              flex: none;
              align-items: center;
              color: {};
              line-height: 1;
              white-space: nowrap;
            }}

            .ant-input-number-prefix {{
              margin-right: 4px;
            }}

            .ant-input-number-suffix {{
              margin-left: 4px;
            }}

            /* 控制按钮包装器 */
            .ant-input-number-handler-wrap {{
              position: absolute;
              top: 0;
              right: 0;
              width: 22px;
              height: 100%;
              background: {};
              border-left: 1px solid {};
              border-radius: 0 {}px {}px 0;
              opacity: 0;
              transition: opacity 0.2s;
            }}

            .ant-input-number:hover .ant-input-number-handler-wrap,
            .ant-input-number-focused .ant-input-number-handler-wrap {{
              opacity: 1;
            }}

            .ant-input-number-handler-wrap-hidden {{
              display: none;
            }}

            /* 控制按钮 */
            .ant-input-number-handler {{
              position: relative;
              display: block;
              width: 100%;
              height: 15px;
              overflow: hidden;
              color: {};
              font-weight: bold;
              line-height: 0;
              text-align: center;
              cursor: pointer;
              transition: all 0.1s linear;
            }}

            .ant-input-number-handler:active {{
              background: {};
            }}

            .ant-input-number-handler:hover {{
              height: 40%;
            }}

            .ant-input-number-handler:hover .ant-input-number-handler-up-inner,
            .ant-input-number-handler:hover .ant-input-number-handler-down-inner {{
              color: {};
            }}

            .ant-input-number-handler-up-inner,
            .ant-input-number-handler-down-inner {{
              display: inline-block;
              color: {};
              font-style: normal;
              line-height: 0;
              text-align: center;
              text-transform: none;
              vertical-align: -0.125em;
              text-rendering: optimizeLegibility;
              -webkit-font-smoothing: antialiased;
              -moz-osx-font-smoothing: grayscale;
              position: absolute;
              right: 4px;
              width: 12px;
              height: 12px;
              transition: all 0.1s linear;
              user-select: none;
            }}

            .ant-input-number-handler-up-inner {{
              top: 0;
            }}

            .ant-input-number-handler-down-inner {{
              bottom: 0;
            }}

            .ant-input-number-handler-up {{
              border-bottom: 1px solid {};
            }}

            .ant-input-number-handler-up-disabled,
            .ant-input-number-handler-down-disabled {{
              cursor: not-allowed;
            }}

            .ant-input-number-handler-up-disabled:hover,
            .ant-input-number-handler-down-disabled:hover {{
              color: {};
              border-color: {};
            }}

            .ant-input-number-handler-up-disabled .ant-input-number-handler-up-inner,
            .ant-input-number-handler-down-disabled .ant-input-number-handler-down-inner {{
              color: {};
            }}
            "#,
            self.token.color_text,                  // 基础文本颜色
            self.token.font_size,                   // 基础字体大小
            self.token.font_family,                 // 字体族
            self.token.color_bg_container,          // 背景颜色
            self.token.color_border,                // 边框颜色
            self.token.border_radius,               // 边框圆角
            self.token.color_primary_hover,         // 悬浮边框颜色
            self.token.color_primary,               // 聚焦边框颜色
            self.token.color_primary_bg_hover,      // 聚焦阴影颜色
            self.token.font_size,                   // 小尺寸字体大小
            self.token.border_radius_sm,            // 小尺寸边框圆角
            self.token.font_size_lg,                // 大尺寸字体大小
            self.token.border_radius,               // 大尺寸边框圆角
            self.token.color_text_disabled,         // 禁用状态文本颜色
            self.token.color_bg_container_disabled, // 禁用状态背景色
            self.token.color_border,                // 禁用状态边框颜色
            self.token.color_border,                // 禁用状态悬浮边框颜色
            self.token.color_error,                 // 错误状态边框颜色
            self.token.color_error,                 // 错误状态悬浮边框颜色
            self.token.color_error_hover,           // 错误状态聚焦边框颜色
            self.token.color_error_bg_hover,        // 错误状态聚焦阴影颜色
            self.token.color_warning,               // 警告状态边框颜色
            self.token.color_warning,               // 警告状态悬浮边框颜色
            self.token.color_warning_hover,         // 警告状态聚焦边框颜色
            self.token.color_warning_bg_hover,      // 警告状态聚焦阴影颜色
            self.token.color_text,                  // 输入框文本颜色
            self.token.border_radius,               // 输入框边框圆角
            self.token.color_text_placeholder,      // 占位符颜色
            self.token.color_text_quaternary,       // 前缀后缀颜色
            self.token.color_bg_container,          // 控制按钮包装器背景色
            self.token.color_border,                // 控制按钮包装器边框颜色
            self.token.border_radius,               // 控制按钮包装器右上圆角
            self.token.border_radius,               // 控制按钮包装器右下圆角
            self.token.color_text_quaternary,       // 控制按钮颜色
            self.token.color_bg_container_active,   // 控制按钮激活背景色
            self.token.color_primary,               // 控制按钮悬浮颜色
            self.token.color_text_quaternary,       // 控制按钮内部图标颜色
            self.token.color_border,                // 上控制按钮底部边框颜色
            self.token.color_text_disabled,         // 禁用控制按钮悬浮颜色
            self.token.color_border,                // 禁用控制按钮悬浮边框颜色
            self.token.color_text_disabled,         // 禁用控制按钮内部图标颜色
        ))
        .unwrap();

        style.get_class_name()
    }
}

pub fn use_input_number_style() -> String {
    let style_generator = InputNumberStyleGenerator::new();
    style_generator.generate_style()
}

use crate::theme::{use_token, AliasToken};
use css_in_rust::Style;

pub struct CascaderStyleGenerator {
    token: AliasToken,
}

impl CascaderStyleGenerator {
    pub fn new() -> Self {
        Self { token: use_token() }
    }

    pub fn generate_style(&self) -> String {
        let style = Style::new(format!(
            r#"
            /* Cascader 级联选择组件样式 */

            /* 基础容器样式 */
            .ant-cascader {{
              position: relative;
              display: inline-block;
              width: 100%;
              box-sizing: border-box;
              margin: 0;
              padding: 0;
              color: {};
              font-size: {}px;
              line-height: 1.5714285714285714;
              list-style: none;
              font-family: {};
            }}

            /* 选择器样式 */
            .ant-cascader-selector {{
              position: relative;
              display: flex;
              align-items: center;
              min-height: 32px;
              padding: 4px 11px;
              color: {};
              font-size: {}px;
              line-height: 1.5714285714285714;
              background-color: {};
              background-image: none;
              border: 1px solid {};
              border-radius: {}px;
              cursor: pointer;
              transition: all 0.2s;
            }}

            .ant-cascader-selector:hover {{
              border-color: {};
            }}

            .ant-cascader-open .ant-cascader-selector {{
              border-color: {};
              box-shadow: 0 0 0 2px {};
              outline: 0;
            }}

            /* 小尺寸选择器 */
            .ant-cascader-small .ant-cascader-selector,
            .ant-cascader-selector-small {{
              min-height: 24px;
              padding: 0px 7px;
              font-size: {}px;
              border-radius: {}px;
            }}

            /* 大尺寸选择器 */
            .ant-cascader-large .ant-cascader-selector,
            .ant-cascader-selector-large {{
              min-height: 40px;
              padding: 6.5px 11px;
              font-size: {}px;
              border-radius: {}px;
            }}

            /* 禁用状态 */
            .ant-cascader-disabled .ant-cascader-selector,
            .ant-cascader-selector-disabled {{
              color: {};
              background-color: {};
              border-color: {};
              box-shadow: none;
              cursor: not-allowed;
            }}

            .ant-cascader-disabled .ant-cascader-selector:hover,
            .ant-cascader-selector-disabled:hover {{
              border-color: {};
            }}

            /* 错误状态 */
            .ant-cascader-status-error .ant-cascader-selector,
            .ant-cascader-selector-status-error {{
              border-color: {};
            }}

            .ant-cascader-status-error .ant-cascader-selector:hover,
            .ant-cascader-selector-status-error:hover {{
              border-color: {};
            }}

            .ant-cascader-status-error.ant-cascader-open .ant-cascader-selector,
            .ant-cascader-open .ant-cascader-selector-status-error {{
              border-color: {};
              box-shadow: 0 0 0 2px {};
            }}

            /* 警告状态 */
            .ant-cascader-status-warning .ant-cascader-selector,
            .ant-cascader-selector-status-warning {{
              border-color: {};
            }}

            .ant-cascader-status-warning .ant-cascader-selector:hover,
            .ant-cascader-selector-status-warning:hover {{
              border-color: {};
            }}

            .ant-cascader-status-warning.ant-cascader-open .ant-cascader-selector,
            .ant-cascader-open .ant-cascader-selector-status-warning {{
              border-color: {};
              box-shadow: 0 0 0 2px {};
            }}

            /* 选中项显示 */
            .ant-cascader-selection-item {{
              flex: 1;
              overflow: hidden;
              white-space: nowrap;
              text-overflow: ellipsis;
            }}

            /* 搜索输入框 */
            .ant-cascader-input {{
              flex: 1;
              width: 100%;
              min-width: 0;
              padding: 0;
              color: inherit;
              font-size: inherit;
              line-height: inherit;
              background: transparent;
              border: none;
              outline: none;
              cursor: inherit;
            }}

            .ant-cascader-input::placeholder {{
              color: {};
            }}

            /* 清除按钮 */
            .ant-cascader-clear {{
              position: absolute;
              top: 50%;
              right: 24px;
              z-index: 1;
              display: inline-block;
              width: 12px;
              height: 12px;
              margin-top: -6px;
              color: {};
              font-size: 12px;
              font-style: normal;
              line-height: 1;
              text-align: center;
              text-transform: none;
              background: {};
              cursor: pointer;
              opacity: 0;
              transition: color 0.3s ease, opacity 0.15s ease;
              text-rendering: auto;
            }}

            .ant-cascader:hover .ant-cascader-clear {{
              opacity: 1;
            }}

            .ant-cascader-clear:hover {{
              color: {};
            }}

            .ant-cascader-clear:active {{
              color: {};
            }}

            /* 下拉箭头 */
            .ant-cascader-arrow {{
              position: absolute;
              top: 50%;
              right: 8px;
              width: 12px;
              height: 12px;
              margin-top: -6px;
              color: {};
              font-size: 12px;
              line-height: 1;
              text-align: center;
              pointer-events: none;
              transition: transform 0.2s;
            }}

            .ant-cascader-open .ant-cascader-arrow {{
              transform: rotate(180deg);
            }}

            /* 下拉菜单 */
            .ant-cascader-dropdown {{
              position: absolute;
              top: 100%;
              left: 0;
              z-index: 1050;
              width: auto;
              min-width: 100%;
              padding: 4px 0;
              margin: 4px 0 0;
              background-color: {};
              box-shadow: 0 3px 6px -4px rgba(0, 0, 0, 0.12), 0 6px 16px 0 rgba(0, 0, 0, 0.08), 0 9px 28px 8px rgba(0, 0, 0, 0.05);
              border-radius: {}px;
            }}

            .ant-cascader-menus {{
              display: flex;
              align-items: flex-start;
              box-sizing: border-box;
            }}

            .ant-cascader-menu {{
              min-width: 200px;
              height: 256px;
              margin: 0;
              padding: 0 0;
              overflow-x: hidden;
              overflow-y: auto;
              list-style: none;
              outline: none;
              border-right: 1px solid {};
            }}

            .ant-cascader-menu:last-child {{
              margin-right: 0;
              border-right: 0;
            }}

            .ant-cascader-menu-item {{
              display: flex;
              align-items: center;
              padding: 5px 12px;
              overflow: hidden;
              line-height: 1.5714285714285714;
              white-space: nowrap;
              text-overflow: ellipsis;
              cursor: pointer;
              transition: all 0.2s;
            }}

            .ant-cascader-menu-item:hover {{
              background-color: {};
            }}

            .ant-cascader-menu-item-active {{
              background-color: {};
            }}

            .ant-cascader-menu-item-disabled {{
              color: {};
              cursor: not-allowed;
            }}

            .ant-cascader-menu-item-disabled:hover {{
              background-color: transparent;
            }}

            .ant-cascader-menu-item-content {{
              flex: auto;
              overflow: hidden;
              text-overflow: ellipsis;
            }}

            .ant-cascader-menu-item-expand-icon {{
              margin-left: 8px;
              color: {};
              font-size: 10px;
            }}

            .ant-cascader-menu-item-expand .ant-cascader-menu-item-expand-icon {{
              color: {};
            }}

            .ant-cascader-menu-item-loading-icon {{
              margin-right: 4px;
              color: {};
              animation: loadingCircle 1s infinite linear;
            }}

            @keyframes loadingCircle {{
              100% {{
                transform: rotate(360deg);
              }}
            }}

            .ant-cascader-multiple .ant-cascader-selection-item {{
              display: inline-block;
              margin-inline-end: 4px;
              padding: 0 4px;
              background: {};
              border-radius: 2px;
            }}

            .ant-cascader-multiple .ant-cascader-selection-item:last-child {{
              margin-inline-end: 0;
            }}

            @media (max-width: 575px) {{
              .ant-cascader-dropdown {{
                width: 100%;
                max-width: 296px;
              }}

              .ant-cascader-menu {{
                min-width: 120px;
              }}

              .ant-cascader-menu-item {{
                padding: 8px 8px;
              }}
            }}
            "#,
            self.token.color_text,                  // 基础文字颜色
            self.token.font_size,                   // 字体大小
            self.token.font_family,                 // 字体
            self.token.color_text,                  // 选择器文字颜色
            self.token.font_size,                   // 选择器字体大小
            self.token.color_bg_container,          // 选择器背景色
            self.token.color_border,                // 选择器边框色
            self.token.border_radius,               // 选择器圆角
            self.token.color_primary_hover,         // 悬停边框色
            self.token.color_primary,               // 选中边框色
            self.token.color_primary_bg_hover,      // 选中阴影色
            self.token.font_size_sm,                // 小号选择器字体
            self.token.border_radius_sm,            // 小号选择器圆角
            self.token.font_size_lg,                // 大号选择器字体
            self.token.border_radius_lg,            // 大号选择器圆角
            self.token.color_text_disabled,         // 禁用文字颜色
            self.token.color_bg_container_disabled, // 禁用背景色
            self.token.color_border,                // 禁用边框色
            self.token.color_border,                // 禁用悬停边框色
            self.token.color_error,                 // 错误边框色
            self.token.color_error,                 // 错误悬停边框色
            self.token.color_error_hover,           // 错误选中边框色
            self.token.color_error_bg_hover,        // 错误选中阴影色
            self.token.color_warning,               // 警告边框色
            self.token.color_warning,               // 警告悬停边框色
            self.token.color_warning_hover,         // 警告选中边框色
            self.token.color_warning_bg_hover,      // 警告选中阴影色
            self.token.color_text_placeholder,      // 占位符颜色
            self.token.color_text_quaternary,       // 清除按钮颜色
            self.token.color_bg_container,          // 清除按钮背景色
            self.token.color_text_tertiary,         // 清除按钮悬停颜色
            self.token.color_text,                  // 清除按钮激活颜色
            self.token.color_text_quaternary,       // 下拉箭头颜色
            self.token.color_bg_container,          // 下拉菜单背景色
            self.token.border_radius,               // 下拉菜单圆角
            self.token.color_border,                // 菜单右边框色
            self.token.color_bg_text_hover,         // 菜单项悬停背景色
            self.token.color_bg_text_active,        // 菜单项激活背景色
            self.token.color_text_disabled,         // 禁用菜单项颜色
            self.token.color_text_quaternary,       // 展开图标颜色
            self.token.color_text_secondary,        // 展开状态图标颜色
            self.token.color_primary,               // 加载图标颜色
            self.token.color_bg_container_disabled, // 多选标签背景色
        ))
        .unwrap();

        style.get_class_name()
    }
}

pub fn use_cascader_style() -> String {
    let style_generator = CascaderStyleGenerator::new();
    style_generator.generate_style()
}

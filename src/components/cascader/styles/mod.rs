use crate::theme::AliasToken;
use css_in_rust::css;

pub struct CascaderStyleGenerator {
    token: AliasToken,
}

impl CascaderStyleGenerator {
    pub fn new() -> Self {
        Self {
            token: AliasToken::default(),
        }
    }

    pub fn generate_style(&self) -> String {
        css!(
            r#"
            /* Cascader 级联选择组件样式 */

            /* 基础容器样式 */
            .ant-cascader {
              position: relative;
              display: inline-block;
              width: 100%;
              box-sizing: border-box;
              margin: 0;
              padding: 0;
              color: ${text_color};
              font-size: ${font_size}px;
              line-height: 1.5714285714285714;
              list-style: none;
              font-family: ${font_family};
            }

            /* 选择器样式 */
            .ant-cascader-selector {
              position: relative;
              display: flex;
              align-items: center;
              min-height: 32px;
              padding: 4px 11px;
              color: ${selector_color};
              font-size: ${font_size}px;
              line-height: 1.5714285714285714;
              background-color: ${selector_bg};
              background-image: none;
              border: 1px solid ${border_color};
              border-radius: ${border_radius}px;
              cursor: pointer;
              transition: all 0.2s;
            }

            .ant-cascader-selector:hover {
              border-color: ${hover_border_color};
            }

            .ant-cascader-open .ant-cascader-selector {
              border-color: ${focus_border_color};
              box-shadow: 0 0 0 2px rgba(24, 144, 255, 0.2);
              outline: 0;
            }

            /* 小尺寸选择器 */
            .ant-cascader-small .ant-cascader-selector,
            .ant-cascader-selector-small {
              min-height: 24px;
              padding: 0px 7px;
              font-size: ${sm_font_size}px;
              border-radius: ${sm_border_radius}px;
            }

            /* 大尺寸选择器 */
            .ant-cascader-large .ant-cascader-selector,
            .ant-cascader-selector-large {
              min-height: 40px;
              padding: 6.5px 11px;
              font-size: ${lg_font_size}px;
              border-radius: ${lg_border_radius}px;
            }

            /* 禁用状态 */
            .ant-cascader-disabled .ant-cascader-selector,
            .ant-cascader-selector-disabled {
              color: ${disabled_color};
              background-color: ${disabled_bg};
              border-color: ${disabled_border_color};
              box-shadow: none;
              cursor: not-allowed;
            }

            .ant-cascader-disabled .ant-cascader-selector:hover,
            .ant-cascader-selector-disabled:hover {
              border-color: ${disabled_border_color};
            }

            /* 错误状态 */
            .ant-cascader-status-error .ant-cascader-selector,
            .ant-cascader-selector-status-error {
              border-color: ${error_color};
            }

            .ant-cascader-status-error .ant-cascader-selector:hover,
            .ant-cascader-selector-status-error:hover {
              border-color: ${error_color};
            }

            .ant-cascader-status-error.ant-cascader-open .ant-cascader-selector,
            .ant-cascader-open .ant-cascader-selector-status-error {
              border-color: ${error_color};
              box-shadow: 0 0 0 2px rgba(255, 77, 79, 0.2);
            }

            /* 警告状态 */
            .ant-cascader-status-warning .ant-cascader-selector,
            .ant-cascader-selector-status-warning {
              border-color: ${warning_color};
            }

            .ant-cascader-status-warning .ant-cascader-selector:hover,
            .ant-cascader-selector-status-warning:hover {
              border-color: ${warning_color};
            }

            .ant-cascader-status-warning.ant-cascader-open .ant-cascader-selector,
            .ant-cascader-open .ant-cascader-selector-status-warning {
              border-color: ${warning_color};
              box-shadow: 0 0 0 2px rgba(250, 173, 20, 0.2);
            }

            /* 选中项显示 */
            .ant-cascader-selection-item {
              flex: 1;
              overflow: hidden;
              white-space: nowrap;
              text-overflow: ellipsis;
            }

            /* 搜索输入框 */
            .ant-cascader-input {
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
            }

            .ant-cascader-input::placeholder {
              color: ${placeholder_color};
            }

            /* 清除按钮 */
            .ant-cascader-clear {
              position: absolute;
              top: 50%;
              right: 24px;
              z-index: 1;
              display: inline-block;
              width: 12px;
              height: 12px;
              margin-top: -6px;
              color: ${clear_icon_color};
              font-size: 12px;
              font-style: normal;
              line-height: 1;
              text-align: center;
              text-transform: none;
              background: ${clear_bg_color};
              cursor: pointer;
              opacity: 0;
              transition: color 0.3s ease, opacity 0.15s ease;
              text-rendering: auto;
            }

            .ant-cascader:hover .ant-cascader-clear {
              opacity: 1;
            }

            .ant-cascader-clear:hover {
              color: ${clear_hover_color};
            }

            .ant-cascader-clear:active {
              color: ${clear_active_color};
            }

            /* 下拉箭头 */
            .ant-cascader-arrow {
              position: absolute;
              top: 50%;
              right: 8px;
              width: 12px;
              height: 12px;
              margin-top: -6px;
              color: ${arrow_color};
              font-size: 12px;
              line-height: 1;
              text-align: center;
              pointer-events: none;
            }

            .ant-cascader-arrow .anticon {
              display: inline-block;
              font-size: 12px;
              font-size: 10px \9;
              transform: scale(0.83333333) rotate(0deg);
              vertical-align: middle;
            }

            /* 下拉菜单 */
            .ant-cascader-dropdown {
              position: absolute;
              top: 100%;
              left: 0;
              z-index: 1050;
              width: 100%;
              margin-top: 4px;
              padding: 4px 0;
              overflow: hidden;
              font-size: ${font_size}px;
              font-variant: initial;
              background-color: ${dropdown_bg};
              border-radius: ${border_radius}px;
              box-shadow: ${dropdown_shadow};
              box-sizing: border-box;
            }

            .ant-cascader-dropdown-hidden {
              display: none;
            }

            .ant-cascader-dropdown-placement-bottomLeft {
              left: 0;
            }

            .ant-cascader-dropdown-placement-bottomRight {
              right: 0;
            }

            .ant-cascader-menu {
              display: inline-block;
              min-width: 111px;
              height: 180px;
              margin: 0;
              padding: 0;
              overflow: auto;
              vertical-align: top;
              list-style: none;
              border-right: 1px solid ${menu_border_color};
              -ms-overflow-style: none;
              scrollbar-width: none;
            }

            .ant-cascader-menu::-webkit-scrollbar {
              display: none;
            }

            .ant-cascader-menu:last-child {
              margin-right: 0;
              border-right: 0;
            }

            .ant-cascader-menu-item {
              display: flex;
              align-items: center;
              padding: 5px 12px;
              overflow: hidden;
              line-height: 22px;
              white-space: nowrap;
              text-overflow: ellipsis;
              cursor: pointer;
              transition: all 0.3s;
            }

            .ant-cascader-menu-item:hover {
              background: ${item_hover_bg};
            }

            .ant-cascader-menu-item-disabled {
              color: ${disabled_color};
              cursor: not-allowed;
            }

            .ant-cascader-menu-item-disabled:hover {
              background: transparent;
            }

            .ant-cascader-menu-item-active:not(.ant-cascader-menu-item-disabled) {
              font-weight: 600;
              background: ${item_active_bg};
            }

            .ant-cascader-menu-item-content {
              flex: auto;
            }

            .ant-cascader-menu-item-expand .ant-cascader-menu-item-expand-icon,
            .ant-cascader-menu-item-loading-icon {
              margin-left: 4px;
              color: ${expand_icon_color};
              font-size: 10px;
            }

            .ant-cascader-menu-item-disabled.ant-cascader-menu-item-expand .ant-cascader-menu-item-expand-icon,
            .ant-cascader-menu-item-disabled .ant-cascader-menu-item-loading-icon {
              color: ${disabled_color};
            }

            .ant-cascader-menu-item .ant-cascader-menu-item-keyword {
              color: ${keyword_color};
            }
            "#,
            text_color = self.token.color_text,
            font_size = self.token.font_size,
            font_family = self.token.font_family,
            selector_color = self.token.color_text,
            selector_bg = self.token.color_bg_container,
            border_color = self.token.color_border,
            border_radius = self.token.border_radius,
            hover_border_color = self.token.color_primary_hover,
            focus_border_color = self.token.color_primary,
            sm_font_size = self.token.font_size_sm,
            sm_border_radius = self.token.border_radius_sm,
            lg_font_size = self.token.font_size_lg,
            lg_border_radius = self.token.border_radius_lg,
            disabled_color = self.token.color_text_disabled,
            disabled_bg = self.token.color_bg_disabled,
            disabled_border_color = self.token.color_border,
            error_color = self.token.color_error,
            warning_color = self.token.color_warning,
            placeholder_color = self.token.input_placeholder_color,
            clear_icon_color = "rgba(0, 0, 0, 0.25)",
            clear_bg_color = self.token.color_bg_container,
            clear_hover_color = "rgba(0, 0, 0, 0.45)",
            clear_active_color = "rgba(0, 0, 0, 0.65)",
            arrow_color = "rgba(0, 0, 0, 0.25)",
            dropdown_bg = self.token.color_bg_container,
            dropdown_shadow = self.token.box_shadow_secondary,
            menu_border_color = self.token.color_border_secondary,
            item_hover_bg = self.token.color_bg,
            item_active_bg = self.token.color_primary_bg,
            expand_icon_color = self.token.color_text_secondary,
            keyword_color = self.token.color_error
        )
    }
}

pub fn use_cascader_style() -> String {
    CascaderStyleGenerator::new().generate_style()
}

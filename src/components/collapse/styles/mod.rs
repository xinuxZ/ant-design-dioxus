use crate::theme::{use_token, AliasToken};
use css_in_rust::Style;

pub struct CollapseStyleGenerator {
    token: AliasToken,
}

impl CollapseStyleGenerator {
    pub fn new() -> Self {
        Self { token: use_token() }
    }

    pub fn generate_style(&self) -> String {
        let style = Style::new(format!(
            r#"
            /* Collapse 折叠面板组件样式 */

            .ant-collapse {{
              margin: 0;
              padding: 0;
              color: {};
              font-size: {}px;
              font-variant: tabular-nums;
              line-height: 1.5715;
              list-style: none;
              font-feature-settings: 'tnum';
              background-color: {};
              border: 1px solid {};
              border-bottom: 0;
              border-radius: {}px;
            }}

            .ant-collapse>.ant-collapse-item {{
              border-bottom: 1px solid {};
            }}

            .ant-collapse>.ant-collapse-item:last-child {{
              border-radius: 0 0 {}px {}px;
            }}

            .ant-collapse>.ant-collapse-item:last-child>.ant-collapse-content {{
              border-radius: 0 0 {}px {}px;
            }}

            .ant-collapse-borderless {{
              background-color: transparent;
              border: 0;
            }}

            .ant-collapse-borderless>.ant-collapse-item {{
              border-bottom: 1px solid {};
            }}

            .ant-collapse-borderless>.ant-collapse-item:last-child {{
              border-bottom: 0;
            }}

            .ant-collapse-borderless>.ant-collapse-item>.ant-collapse-header {{
              padding-left: 0;
              padding-right: 0;
            }}

            .ant-collapse-borderless>.ant-collapse-item>.ant-collapse-content {{
              background-color: transparent;
              border-top: 0;
            }}

            .ant-collapse-borderless>.ant-collapse-item>.ant-collapse-content>.ant-collapse-content-box {{
              padding-left: 0;
              padding-right: 0;
            }}

            .ant-collapse-item {{
              border-bottom: 0;
            }}

            .ant-collapse-item-active>.ant-collapse-header {{
              color: {};
            }}

            .ant-collapse-item-active>.ant-collapse-header .ant-collapse-arrow {{
              transform: rotate(90deg);
            }}

            .ant-collapse-item-disabled>.ant-collapse-header {{
              color: {};
              cursor: not-allowed;
            }}

            .ant-collapse-item-disabled>.ant-collapse-header .ant-collapse-arrow {{
              cursor: not-allowed;
            }}

            .ant-collapse-header {{
              position: relative;
              display: flex;
              flex-wrap: nowrap;
              align-items: flex-start;
              padding: 12px 16px;
              color: {};
              line-height: 1.5715;
              cursor: pointer;
              transition: all 0.3s, visibility 0s;
            }}

            .ant-collapse-header:focus {{
              outline: none;
            }}

            .ant-collapse-header-text {{
              flex: auto;
            }}

            .ant-collapse-expand-icon {{
              height: 22px;
              color: {};
              line-height: 22px;
              cursor: pointer;
              transition: all 0.3s;
            }}

            .ant-collapse-icon-position-start .ant-collapse-expand-icon {{
              margin-right: 12px;
            }}

            .ant-collapse-icon-position-end .ant-collapse-expand-icon {{
              margin-left: 12px;
              order: 1;
            }}

            .ant-collapse-arrow {{
              display: inline-block;
              transition: transform 0.24s;
            }}

            .ant-collapse-content {{
              color: {};
              background-color: {};
              border-top: 1px solid {};
              transition: all 0.3s ease-out;
              will-change: height;
            }}

            .ant-collapse-content-active {{
              height: auto;
            }}

            .ant-collapse-content-inactive {{
              height: 0;
              overflow: hidden;
            }}

            .ant-collapse-content-box {{
              padding: 16px;
            }}

            .ant-collapse-extra {{
              margin-left: auto;
            }}

            .ant-collapse-large {{
              font-size: {}px;
            }}

            .ant-collapse-large>.ant-collapse-item>.ant-collapse-header {{
              padding: 16px 24px;
              font-size: {}px;
              line-height: 1.5;
            }}

            .ant-collapse-large>.ant-collapse-item>.ant-collapse-content>.ant-collapse-content-box {{
              padding: 24px;
            }}

            .ant-collapse-small {{
              font-size: {}px;
            }}

            .ant-collapse-small>.ant-collapse-item>.ant-collapse-header {{
              padding: 8px 12px;
              font-size: {}px;
            }}

            .ant-collapse-small>.ant-collapse-item>.ant-collapse-content>.ant-collapse-content-box {{
              padding: 12px;
            }}

            .ant-collapse-ghost {{
              background-color: transparent;
              border: 0;
            }}

            .ant-collapse-ghost>.ant-collapse-item {{
              border-bottom: 0;
            }}

            .ant-collapse-ghost>.ant-collapse-item>.ant-collapse-content {{
              background-color: transparent;
              border: 0;
            }}

            .ant-collapse-rtl {{
              direction: rtl;
            }}

            .ant-collapse-rtl .ant-collapse-expand-icon {{
              transform: rotate(180deg);
            }}

            .ant-collapse-rtl .ant-collapse-icon-position-start .ant-collapse-expand-icon {{
              margin-right: 0;
              margin-left: 12px;
            }}

            .ant-collapse-rtl .ant-collapse-icon-position-end .ant-collapse-expand-icon {{
              margin-right: 12px;
              margin-left: 0;
            }}

            .ant-collapse-rtl .ant-collapse-extra {{
              margin-right: auto;
              margin-left: 0;
            }}

            /* 动画效果 */
            @keyframes ant-collapse-motion {{
              0% {{
                max-height: 0;
                opacity: 0;
              }}

              100% {{
                max-height: 1000px;
                opacity: 1;
              }}
            }}

            .ant-collapse-content-active {{
              animation: ant-collapse-motion 0.3s ease-out;
            }}

            .ant-collapse-content-inactive {{
              animation: ant-collapse-motion 0.3s ease-out reverse;
            }}
            "#,
            self.token.color_text,               // 文本颜色
            self.token.font_size,                // 字体大小
            self.token.color_fill_tertiary,      // 背景色
            self.token.color_border,             // 边框颜色
            self.token.border_radius,            // 边框圆角
            self.token.color_border,             // 面板项边框颜色
            self.token.border_radius,            // 最后一个面板项圆角
            self.token.border_radius,            // 最后一个面板项圆角
            self.token.border_radius,            // 最后一个面板内容圆角
            self.token.border_radius,            // 最后一个面板内容圆角
            self.token.color_split,              // 无边框模式下的边框颜色
            self.token.color_primary,            // 活动面板标题颜色
            self.token.color_text_disabled,      // 禁用面板颜色
            self.token.color_text,               // 面板标题颜色
            self.token.color_text_quaternary,    // 展开图标颜色
            self.token.color_text,               // 内容文本颜色
            self.token.color_bg_container,       // 内容背景颜色
            self.token.color_border,             // 内容上边框颜色
            self.token.font_size_lg,             // 大尺寸字体
            self.token.font_size_lg,             // 大尺寸标题字体
            self.token.font_size,                // 小尺寸字体
            self.token.font_size,                // 小尺寸标题字体
        ))
        .unwrap();

        style.get_class_name()
    }
}

pub fn use_collapse_style() -> String {
    let style_generator = CollapseStyleGenerator::new();
    style_generator.generate_style()
}

use crate::theme::{use_token, AliasToken};
use css_in_rust::Style;

pub struct DrawerStyleGenerator {
    token: AliasToken,
}

impl DrawerStyleGenerator {
    pub fn new() -> Self {
        Self { token: use_token() }
    }

    pub fn generate_style(&self) -> String {
        let style = Style::new(format!(
            r#"
            /* Drawer 抽屉样式 */

            /* 抽屉根容器 */
            .ant-drawer-root {{
              position: fixed;
              inset: 0;
              z-index: 1000;
              pointer-events: none;
            }}

            /* 遮罩层 */
            .ant-drawer-mask {{
              position: absolute;
              inset: 0;
              background-color: {};
              pointer-events: auto;
              animation: ant-drawer-mask-fade-in 0.3s cubic-bezier(0.78, 0.14, 0.15, 0.86);
            }}

            /* 抽屉内容包装器 */
            .ant-drawer-content-wrapper {{
              position: absolute;
              pointer-events: auto;
            }}

            /* 抽屉基础样式 */
            .ant-drawer {{
              position: relative;
              background: {};
              box-shadow: {};
              transition: transform 0.3s cubic-bezier(0.78, 0.14, 0.15, 0.86);
              display: flex;
              flex-direction: column;
              height: 100%;
            }}

            /* 右侧抽屉 */
            .ant-drawer-right {{
              top: 0;
              right: 0;
              height: 100%;
              transform: translateX(100%);
            }}

            .ant-drawer-right.ant-drawer-open {{
              transform: translateX(0);
            }}

            .ant-drawer-content-wrapper .ant-drawer-right {{
              right: 0;
              top: 0;
              bottom: 0;
            }}

            /* 左侧抽屉 */
            .ant-drawer-left {{
              top: 0;
              left: 0;
              height: 100%;
              transform: translateX(-100%);
            }}

            .ant-drawer-left.ant-drawer-open {{
              transform: translateX(0);
            }}

            .ant-drawer-content-wrapper .ant-drawer-left {{
              left: 0;
              top: 0;
              bottom: 0;
            }}

            /* 顶部抽屉 */
            .ant-drawer-top {{
              top: 0;
              left: 0;
              right: 0;
              width: 100%;
              transform: translateY(-100%);
            }}

            .ant-drawer-top.ant-drawer-open {{
              transform: translateY(0);
            }}

            .ant-drawer-content-wrapper .ant-drawer-top {{
              top: 0;
              left: 0;
              right: 0;
            }}

            /* 底部抽屉 */
            .ant-drawer-bottom {{
              bottom: 0;
              left: 0;
              right: 0;
              width: 100%;
              transform: translateY(100%);
            }}

            .ant-drawer-bottom.ant-drawer-open {{
              transform: translateY(0);
            }}

            .ant-drawer-content-wrapper .ant-drawer-bottom {{
              bottom: 0;
              left: 0;
              right: 0;
            }}

            /* 抽屉内容 */
            .ant-drawer-content {{
              display: flex;
              flex-direction: column;
              width: 100%;
              height: 100%;
              overflow: hidden;
            }}

            /* 抽屉头部 */
            .ant-drawer-header {{
              flex-shrink: 0;
              padding: {}px {}px;
              border-bottom: 1px solid {};
              background: {};
            }}

            .ant-drawer-header-title {{
              display: flex;
              align-items: center;
              justify-content: space-between;
              min-height: 22px;
            }}

            .ant-drawer-title {{
              flex: 1;
              margin: 0;
              color: {};
              font-weight: 600;
              font-size: {}px;
              line-height: 1.375;
            }}

            .ant-drawer-extra {{
              display: flex;
              align-items: center;
              gap: 8px;
            }}

            /* 关闭按钮 */
            .ant-drawer-close {{
              position: relative;
              display: inline-block;
              width: 22px;
              height: 22px;
              padding: 0;
              color: {};
              font-weight: 600;
              font-size: {}px;
              font-style: normal;
              line-height: 22px;
              text-align: center;
              text-transform: none;
              text-decoration: none;
              background: transparent;
              border: 0;
              border-radius: {}px;
              outline: 0;
              cursor: pointer;
              transition: color 0.2s, background-color 0.2s;
              user-select: none;
            }}

            .ant-drawer-close:hover {{
              color: {};
              background-color: {};
            }}

            .ant-drawer-close:active {{
              background-color: {};
            }}

            .ant-drawer-close-x {{
              display: block;
              width: 100%;
              height: 100%;
            }}

            .ant-drawer-close-icon {{
              display: inline-block;
              font-size: {}px;
              line-height: 1;
            }}

            /* 抽屉主体 */
            .ant-drawer-body {{
              flex: 1;
              padding: {}px;
              overflow: auto;
              background: {};
              word-wrap: break-word;
            }}

            /* 抽屉页脚 */
            .ant-drawer-footer {{
              flex-shrink: 0;
              padding: 10px 16px;
              border-top: 1px solid {};
              background: {};
            }}

            /* 动画效果 */
            @keyframes ant-drawer-mask-fade-in {{
              0% {{
                opacity: 0;
              }}

              100% {{
                opacity: 1;
              }}
            }}

            @keyframes ant-drawer-mask-fade-out {{
              0% {{
                opacity: 1;
              }}

              100% {{
                opacity: 0;
              }}
            }}

            /* 响应式设计 */
            @media (max-width: 768px) {{

              .ant-drawer-right,
              .ant-drawer-left {{
                width: 100% !important;
              }}

              .ant-drawer-header {{
                padding: 12px 16px;
              }}

              .ant-drawer-body {{
                padding: 16px;
              }}

              .ant-drawer-footer {{
                padding: 8px 12px;
              }}
            }}

            /* RTL支持 */
            [dir="rtl"] .ant-drawer-right {{
              right: auto;
              left: 0;
              transform: translateX(-100%);
            }}

            [dir="rtl"] .ant-drawer-right.ant-drawer-open {{
              transform: translateX(0);
            }}

            [dir="rtl"] .ant-drawer-left {{
              right: 0;
              left: auto;
              transform: translateX(100%);
            }}

            [dir="rtl"] .ant-drawer-left.ant-drawer-open {{
              transform: translateX(0);
            }}

            /* 高对比度模式 */
            @media (prefers-contrast: high) {{
              .ant-drawer {{
                outline: solid 2px;
                border-radius: {}px;
              }}

              .ant-drawer-header {{
                border-bottom: solid 2px;
              }}

              .ant-drawer-footer {{
                border-top: solid 2px;
              }}
            }}

            /* 减少动画模式 */
            @media (prefers-reduced-motion: reduce) {{
              .ant-drawer,
              .ant-drawer-mask {{
                transition: none;
                animation: none;
              }}
            }}

            /* 深色模式 */
            @media (prefers-color-scheme: dark) {{
              .ant-drawer {{
                background: {};
                box-shadow: {};
              }}

              .ant-drawer-header {{
                border-bottom: 1px solid {};
                background: {};
              }}

              .ant-drawer-body {{
                background: {};
              }}

              .ant-drawer-footer {{
                border-top: 1px solid {};
                background: {};
              }}

              .ant-drawer-title {{
                color: {};
              }}

              .ant-drawer-close {{
                color: {};
              }}

              .ant-drawer-close:hover {{
                color: {};
                background-color: {};
              }}

              .ant-drawer-close:active {{
                background-color: {};
              }}

              .ant-drawer-mask {{
                background-color: {};
              }}
            }}
            "#,
            self.token.color_bg_mask,              // 遮罩层背景色
            self.token.color_bg_container,         // 抽屉背景色
            self.token.box_shadow_secondary,       // 抽屉阴影
            self.token.padding_md,                 // 抽屉头部内边距(垂直)
            self.token.padding_lg,                 // 抽屉头部内边距(水平)
            self.token.color_split,                // 头部分割线颜色
            self.token.color_bg_container,         // 头部背景色
            self.token.color_text,                 // 标题文本颜色
            self.token.font_size_lg,               // 标题字体大小
            self.token.color_text_quaternary,      // 关闭按钮颜色
            self.token.font_size_lg,               // 关闭按钮字体大小
            self.token.border_radius,              // 关闭按钮边框圆角
            self.token.color_text,                 // 关闭按钮悬浮文本颜色
            self.token.color_bg_text_hover,        // 关闭按钮悬浮背景色
            self.token.color_bg_text_active,       // 关闭按钮激活背景色
            self.token.font_size,                  // 关闭图标字体大小
            self.token.padding_lg,                 // 抽屉内容内边距
            self.token.color_bg_container,         // 抽屉内容背景色
            self.token.color_split,                // 页脚分割线颜色
            self.token.color_bg_container,         // 页脚背景色
            self.token.border_radius,              // 高对比度模式边框圆角
            self.token.color_bg_container_dark,    // 深色模式背景色
            self.token.box_shadow_secondary_dark,  // 深色模式阴影
            self.token.color_split_dark,           // 深色模式头部分割线颜色
            self.token.color_bg_container_dark,    // 深色模式头部背景色
            self.token.color_bg_container_dark,    // 深色模式内容背景色
            self.token.color_split_dark,           // 深色模式页脚分割线颜色
            self.token.color_bg_container_dark,    // 深色模式页脚背景色
            self.token.color_text_dark,            // 深色模式标题文本颜色
            self.token.color_text_quaternary_dark, // 深色模式关闭按钮颜色
            self.token.color_text_dark,            // 深色模式关闭按钮悬浮文本颜色
            self.token.color_bg_text_hover_dark,   // 深色模式关闭按钮悬浮背景色
            self.token.color_bg_text_active_dark,  // 深色模式关闭按钮激活背景色
            self.token.color_bg_mask_dark,         // 深色模式遮罩层背景色
        ))
        .unwrap();

        style.get_class_name()
    }
}

pub fn use_drawer_style() -> String {
    let style_generator = DrawerStyleGenerator::new();
    style_generator.generate_style()
}

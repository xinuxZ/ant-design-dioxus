use crate::theme::{use_token, AliasToken};
use css_in_rust::Style;

pub struct ImageStyleGenerator {
    token: AliasToken,
}

impl ImageStyleGenerator {
    pub fn new() -> Self {
        Self { token: use_token() }
    }

    pub fn generate_style(&self) -> String {
        let style = Style::new(format!(
            r#"
            /* Image 图片组件样式 */

            .ant-image {{
              position: relative;
              display: inline-block;
            }}

            .ant-image-img {{
              width: 100%;
              height: auto;
              vertical-align: middle;
              border-style: none;
            }}

            .ant-image-img-wrapper {{
              position: relative;
              display: inline-block;
              overflow: hidden;
            }}

            .ant-image-placeholder {{
              position: absolute;
              top: 0;
              right: 0;
              bottom: 0;
              left: 0;
              display: flex;
              align-items: center;
              justify-content: center;
              color: {};
              font-size: {}px;
              background-color: {};
              border-radius: {}px;
            }}

            .ant-image-error {{
              position: absolute;
              top: 0;
              right: 0;
              bottom: 0;
              left: 0;
              display: flex;
              align-items: center;
              justify-content: center;
              color: {};
              font-size: {}px;
              background-color: {};
              border-radius: {}px;
            }}

            .ant-image-error-icon {{
              font-size: {}px;
            }}

            .ant-image-mask {{
              position: absolute;
              top: 0;
              right: 0;
              bottom: 0;
              left: 0;
              display: flex;
              align-items: center;
              justify-content: center;
              color: {};
              background: {};
              cursor: pointer;
              opacity: 0;
              transition: opacity 0.3s;
            }}

            .ant-image:hover .ant-image-mask {{
              opacity: 1;
            }}

            .ant-image-mask-info {{
              padding: 0 4px;
              overflow: hidden;
              white-space: nowrap;
              text-overflow: ellipsis;
            }}

            .ant-image-preview-icon {{
              display: inline-block;
              font-size: {}px;
            }}

            .ant-image-preview-wrap {{
              position: fixed;
              top: 0;
              right: 0;
              bottom: 0;
              left: 0;
              z-index: 1000;
              background: {};
              text-align: center;
              opacity: 1;
              animation: ant-fade-in 0.3s;
            }}

            .ant-image-preview {{
              position: relative;
              top: 50%;
              display: inline-block;
              padding: 0 {}px;
              text-align: left;
              vertical-align: middle;
              transform: translateY(-50%);
            }}

            .ant-image-preview-content {{
              position: relative;
            }}

            .ant-image-preview-body {{
              overflow: hidden;
              text-align: center;
            }}

            .ant-image-preview-img {{
              max-width: 100%;
              max-height: 70vh;
              vertical-align: middle;
              transform: scale3d(1, 1, 1);
              cursor: grab;
              transition: transform 0.3s cubic-bezier(0.215, 0.61, 0.355, 1) 0s;
              user-select: none;
              pointer-events: auto;
            }}

            .ant-image-preview-img:active {{
              cursor: grabbing;
            }}

            .ant-image-preview-operations {{
              position: absolute;
              top: {}px;
              right: {}px;
              z-index: 1;
              display: flex;
              flex-direction: row-reverse;
              align-items: center;
              color: {};
              list-style: none;
              background: {};
              border-radius: {}px;
              pointer-events: auto;
            }}

            .ant-image-preview-operations-operation {{
              margin-left: {}px;
              padding: {}px;
              cursor: pointer;
              transition: all 0.3s;
            }}

            .ant-image-preview-operations-operation:hover {{
              color: {};
            }}

            .ant-image-preview-operations-operation:first-child {{
              margin-left: 0;
            }}

            .ant-image-preview-close {{
              font-size: {}px;
            }}

            .ant-image-preview-group {{
              display: inline-block;
            }}

            /* 动画效果 */
            @keyframes ant-fade-in {{
              0% {{
                opacity: 0;
              }}

              100% {{
                opacity: 1;
              }}
            }}

            @keyframes ant-fade-out {{
              0% {{
                opacity: 1;
              }}

              100% {{
                opacity: 0;
              }}
            }}

            /* 响应式设计 */
            @media (max-width: 768px) {{
              .ant-image-preview {{
                padding: 0 {}px;
              }}

              .ant-image-preview-img {{
                max-height: 60vh;
              }}

              .ant-image-preview-operations {{
                top: {}px;
                right: {}px;
              }}

              .ant-image-preview-operations-operation {{
                margin-left: {}px;
                padding: {}px;
              }}
            }}

            /* RTL 支持 */
            .ant-image-rtl .ant-image-preview-operations {{
              right: auto;
              left: {}px;
              flex-direction: row;
            }}

            .ant-image-rtl .ant-image-preview-operations-operation {{
              margin-right: {}px;
              margin-left: 0;
            }}

            .ant-image-rtl .ant-image-preview-operations-operation:first-child {{
              margin-right: 0;
            }}
            "#,
            self.token.color_text_quaternary, // 占位符颜色
            self.token.font_size,             // 占位符字体大小
            self.token.color_bg_layout,       // 占位符背景色
            self.token.border_radius_sm,      // 占位符边框圆角
            self.token.color_text_quaternary, // 错误状态颜色
            self.token.font_size,             // 错误状态字体大小
            self.token.color_bg_layout,       // 错误状态背景色
            self.token.border_radius_sm,      // 错误状态边框圆角
            self.token.font_size_lg + self.token.font_size_sm, // 错误图标大小
            self.token.color_white,           // 遮罩层文本颜色
            self.token.color_bg_mask,         // 遮罩层背景色
            self.token.font_size,             // 预览图标大小
            self.token.color_bg_mask,         // 预览包装器背景色
            self.token.padding_lg * 2,        // 预览内边距
            self.token.padding_md,            // 操作区顶部边距
            self.token.padding_md,            // 操作区右侧边距
            self.token.color_white,           // 操作区文本颜色
            self.token.color_bg_mask,         // 操作区背景色
            self.token.border_radius,         // 操作区边框圆角
            self.token.margin_md,             // 操作项间距
            self.token.padding_md,            // 操作项内边距
            self.token.color_white,           // 操作项悬浮文本颜色
            self.token.font_size_lg,          // 关闭按钮字体大小
            self.token.padding_md,            // 移动设备预览内边距
            self.token.padding_xs,            // 移动设备操作区顶部边距
            self.token.padding_xs,            // 移动设备操作区右侧边距
            self.token.margin_xs,             // 移动设备操作项间距
            self.token.padding_xs,            // 移动设备操作项内边距
            self.token.padding_md,            // RTL模式下操作区左侧边距
            self.token.margin_md,             // RTL模式下操作项右侧间距
        ))
        .unwrap();

        style.get_class_name()
    }
}

pub fn use_image_style() -> String {
    let style_generator = ImageStyleGenerator::new();
    style_generator.generate_style()
}

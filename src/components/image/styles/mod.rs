use crate::theme::AliasToken;
use css_in_rust::css;

pub struct ImageStyleGenerator {
    token: AliasToken,
}

impl ImageStyleGenerator {
    pub fn new() -> Self {
        Self {
            token: AliasToken::default(),
        }
    }

    pub fn generate_style(&self) -> String {
        css!(
            r#"
            /* Image 图片组件样式 */

            .ant-image {
              position: relative;
              display: inline-block;
            }

            .ant-image-img {
              width: 100%;
              height: auto;
              vertical-align: middle;
              border-style: none;
            }

            .ant-image-img-wrapper {
              position: relative;
              display: inline-block;
              overflow: hidden;
            }

            .ant-image-placeholder {
              position: absolute;
              top: 0;
              right: 0;
              bottom: 0;
              left: 0;
              display: flex;
              align-items: center;
              justify-content: center;
              color: ${placeholder_color};
              font-size: ${font_size}px;
              background-color: ${placeholder_bg};
              border-radius: ${border_radius_sm}px;
            }

            .ant-image-error {
              position: absolute;
              top: 0;
              right: 0;
              bottom: 0;
              left: 0;
              display: flex;
              align-items: center;
              justify-content: center;
              color: ${error_color};
              font-size: ${font_size}px;
              background-color: ${error_bg};
              border-radius: ${border_radius_sm}px;
            }

            .ant-image-error-icon {
              font-size: ${error_icon_size}px;
            }

            .ant-image-mask {
              position: absolute;
              top: 0;
              right: 0;
              bottom: 0;
              left: 0;
              display: flex;
              align-items: center;
              justify-content: center;
              color: ${mask_text_color};
              background: ${mask_bg};
              cursor: pointer;
              opacity: 0;
              transition: opacity 0.3s;
            }

            .ant-image:hover .ant-image-mask {
              opacity: 1;
            }

            .ant-image-mask-info {
              padding: 0 4px;
              overflow: hidden;
              white-space: nowrap;
              text-overflow: ellipsis;
            }

            .ant-image-preview-icon {
              display: inline-block;
              font-size: ${preview_icon_size}px;
            }

            .ant-image-preview-wrap {
              position: fixed;
              top: 0;
              right: 0;
              bottom: 0;
              left: 0;
              z-index: 1000;
              background: ${preview_bg};
              text-align: center;
              opacity: 1;
              animation: ant-fade-in 0.3s;
            }

            .ant-image-preview {
              position: relative;
              top: 50%;
              display: inline-block;
              padding: 0 ${padding_md}px;
              text-align: left;
              vertical-align: middle;
              transform: translateY(-50%);
            }

            .ant-image-preview-content {
              position: relative;
            }

            .ant-image-preview-body {
              overflow: hidden;
              text-align: center;
            }

            .ant-image-preview-img {
              max-width: 100%;
              max-height: 70vh;
              vertical-align: middle;
              transform: scale3d(1, 1, 1);
              cursor: grab;
              transition: transform 0.3s cubic-bezier(0.215, 0.61, 0.355, 1) 0s;
              user-select: none;
              pointer-events: auto;
            }

            .ant-image-preview-img:active {
              cursor: grabbing;
            }

            .ant-image-preview-operations {
              position: absolute;
              top: ${padding_md}px;
              right: ${padding_md}px;
              z-index: 1;
              display: flex;
              flex-direction: row-reverse;
              align-items: center;
              color: ${operations_color};
              list-style: none;
              background: ${operations_bg};
              border-radius: ${border_radius}px;
              pointer-events: auto;
            }

            .ant-image-preview-operations-operation {
              margin-left: ${margin_xs}px;
              padding: ${padding_xs}px;
              cursor: pointer;
              transition: all 0.3s;
            }

            .ant-image-preview-operations-operation:hover {
              color: ${operations_hover_color};
            }

            .ant-image-preview-operations-operation:first-child {
              margin-left: 0;
            }

            .ant-image-preview-close {
              font-size: ${close_size}px;
            }

            .ant-image-preview-group {
              display: inline-block;
            }

            /* 动画效果 */
            @keyframes ant-fade-in {
              0% {
                opacity: 0;
              }

              100% {
                opacity: 1;
              }
            }

            @keyframes ant-fade-out {
              0% {
                opacity: 1;
              }

              100% {
                opacity: 0;
              }
            }

            /* 预览动画 */
            .ant-image-preview-moving .ant-image-preview-img {
              cursor: grabbing;
            }

            /* 预览导航 */
            .ant-image-preview-switch-left,
            .ant-image-preview-switch-right {
              position: absolute;
              top: 50%;
              z-index: 1;
              display: flex;
              align-items: center;
              justify-content: center;
              width: 44px;
              height: 44px;
              margin-top: -22px;
              color: ${switch_color};
              background: ${switch_bg};
              border-radius: 50%;
              cursor: pointer;
              transition: all 0.3s;
              pointer-events: auto;
            }

            .ant-image-preview-switch-left:hover,
            .ant-image-preview-switch-right:hover {
              color: ${switch_hover_color};
            }

            .ant-image-preview-switch-left {
              left: ${padding_lg}px;
            }

            .ant-image-preview-switch-right {
              right: ${padding_lg}px;
            }

            /* 响应式设计 */
            @media (max-width: 768px) {
              .ant-image-preview-operations {
                top: ${padding_xs}px;
                right: ${padding_xs}px;
              }

              .ant-image-preview-switch-left {
                left: ${padding_xs}px;
              }

              .ant-image-preview-switch-right {
                right: ${padding_xs}px;
              }
            }

            /* 高对比度模式支持 */
            @media (prefers-contrast: high) {
              .ant-image-preview-img {
                border: 1px solid #fff;
              }
            }

            /* 减少动画模式支持 */
            @media (prefers-reduced-motion: reduce) {
              .ant-image-preview-wrap,
              .ant-image-preview-img,
              .ant-image-mask,
              .ant-image-preview-operations-operation {
                animation: none;
                transition: none;
              }
            }
            "#,
            placeholder_color = self.token.color_text_secondary,
            font_size = self.token.font_size,
            placeholder_bg = self.token.color_bg_layout,
            border_radius_sm = self.token.border_radius_sm,
            error_color = self.token.color_text_secondary,
            error_bg = self.token.color_bg_layout,
            error_icon_size = self.token.font_size_lg + self.token.font_size_sm,
            mask_text_color = self.token.color_text,
            mask_bg = self.token.component_background,
            preview_icon_size = self.token.font_size,
            preview_bg = self.token.component_background,
            padding_md = self.token.padding_md,
            operations_color = self.token.color_text,
            operations_bg = self.token.color_bg_container,
            border_radius = self.token.border_radius,
            margin_xs = self.token.margin_xs,
            padding_xs = self.token.padding_xs,
            operations_hover_color = self.token.color_primary,
            close_size = self.token.font_size_lg,
            switch_color = self.token.color_text,
            switch_bg = self.token.color_bg_container,
            switch_hover_color = self.token.color_primary,
            padding_lg = self.token.padding_lg
        )
    }
}

pub fn use_image_style() -> String {
    ImageStyleGenerator::new().generate_style()
}

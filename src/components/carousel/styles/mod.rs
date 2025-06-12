use crate::theme::AliasToken;
use css_in_rust::css;

pub struct CarouselStyleGenerator {
    token: AliasToken,
}

impl CarouselStyleGenerator {
    pub fn new() -> Self {
        Self {
            token: AliasToken::default(),
        }
    }

    pub fn generate_style(&self) -> String {
        css!(
            r#"
            /* Carousel 走马灯组件样式 */

            .ant-carousel {
              position: relative;
              overflow: hidden;
              width: 100%;
              height: 100%;
              box-sizing: border-box;
            }

            .ant-carousel-inner {
              display: flex;
              width: 100%;
              height: 100%;
              transition: transform 0.5s ease;
            }

            .ant-carousel-item {
              flex: 0 0 100%;
              width: 100%;
              height: 100%;
              display: flex;
              align-items: center;
              justify-content: center;
            }

            /* 箭头样式 */
            .ant-carousel-arrow {
              position: absolute;
              top: 50%;
              transform: translateY(-50%);
              z-index: 2;
              width: 36px;
              height: 36px;
              border: none;
              border-radius: 50%;
              background: rgba(0, 0, 0, 0.2);
              color: ${arrow_color};
              font-size: 18px;
              font-weight: bold;
              cursor: pointer;
              display: flex;
              align-items: center;
              justify-content: center;
              transition: all 0.3s ease;
              opacity: 0;
              visibility: hidden;
            }

            .ant-carousel:hover .ant-carousel-arrow {
              opacity: 1;
              visibility: visible;
            }

            .ant-carousel-arrow:hover {
              background: rgba(0, 0, 0, 0.5);
              transform: translateY(-50%) scale(1.1);
            }

            .ant-carousel-arrow-left {
              left: 16px;
            }

            .ant-carousel-arrow-right {
              right: 16px;
            }

            /* 指示器样式 */
            .ant-carousel-dots {
              position: absolute;
              margin: 0;
              padding: 0;
              list-style: none;
              display: flex;
              z-index: 2;
            }

            .ant-carousel-dots-bottom {
              bottom: 16px;
              left: 50%;
              transform: translateX(-50%);
              flex-direction: row;
            }

            .ant-carousel-dots-top {
              top: 16px;
              left: 50%;
              transform: translateX(-50%);
              flex-direction: row;
            }

            .ant-carousel-dots-left {
              left: 16px;
              top: 50%;
              transform: translateY(-50%);
              flex-direction: column;
            }

            .ant-carousel-dots-right {
              right: 16px;
              top: 50%;
              transform: translateY(-50%);
              flex-direction: column;
            }

            .ant-carousel-dot {
              margin: 0 3px;
              cursor: pointer;
            }

            .ant-carousel-dots-left .ant-carousel-dot,
            .ant-carousel-dots-right .ant-carousel-dot {
              margin: 3px 0;
            }

            .ant-carousel-dot-button {
              width: 16px;
              height: 3px;
              border: none;
              border-radius: 1px;
              background: rgba(255, 255, 255, 0.3);
              cursor: pointer;
              transition: all 0.3s ease;
              display: block;
            }

            .ant-carousel-dots-left .ant-carousel-dot-button,
            .ant-carousel-dots-right .ant-carousel-dot-button {
              width: 3px;
              height: 16px;
            }

            .ant-carousel-dot:hover .ant-carousel-dot-button {
              background: rgba(255, 255, 255, 0.5);
            }

            .ant-carousel-dot-active .ant-carousel-dot-button {
              background: #fff;
              width: 24px;
            }

            .ant-carousel-dots-left .ant-carousel-dot-active .ant-carousel-dot-button,
            .ant-carousel-dots-right .ant-carousel-dot-active .ant-carousel-dot-button {
              width: 3px;
              height: 24px;
            }

            /* 淡入淡出效果 */
            .ant-carousel-fade .ant-carousel-inner {
              position: relative;
            }

            .ant-carousel-fade .ant-carousel-item {
              position: absolute;
              top: 0;
              left: 0;
              opacity: 0;
              transition: opacity 0.5s ease;
            }

            .ant-carousel-fade .ant-carousel-item-active {
              opacity: 1;
            }

            /* 响应式设计 */
            @media (max-width: 768px) {
              .ant-carousel-arrow {
                width: 32px;
                height: 32px;
                font-size: 16px;
              }

              .ant-carousel-arrow-left {
                left: 8px;
              }

              .ant-carousel-arrow-right {
                right: 8px;
              }

              .ant-carousel-dots-bottom {
                bottom: 8px;
              }

              .ant-carousel-dots-top {
                top: 8px;
              }

              .ant-carousel-dots-left {
                left: 8px;
              }

              .ant-carousel-dots-right {
                right: 8px;
              }

              .ant-carousel-dot-button {
                width: 12px;
                height: 2px;
              }

              .ant-carousel-dots-left .ant-carousel-dot-button,
              .ant-carousel-dots-right .ant-carousel-dot-button {
                width: 2px;
                height: 12px;
              }

              .ant-carousel-dot-active .ant-carousel-dot-button {
                width: 18px;
              }

              .ant-carousel-dots-left .ant-carousel-dot-active .ant-carousel-dot-button,
              .ant-carousel-dots-right .ant-carousel-dot-active .ant-carousel-dot-button {
                width: 2px;
                height: 18px;
              }
            }

            /* 垂直方向的走马灯 */
            .ant-carousel-vertical .ant-carousel-inner {
              flex-direction: column;
            }

            /* 自动播放指示器 */
            .ant-carousel-autoplay-indicator {
              position: absolute;
              bottom: 0;
              left: 0;
              height: 3px;
              background: ${primary_color};
              transition: width 0.3s linear;
              z-index: 3;
            }

            /* 暗色主题适配 */
            .ant-carousel-dark .ant-carousel-arrow {
              background: rgba(255, 255, 255, 0.2);
              color: rgba(0, 0, 0, 0.85);
            }

            .ant-carousel-dark .ant-carousel-arrow:hover {
              background: rgba(255, 255, 255, 0.5);
            }

            .ant-carousel-dark .ant-carousel-dot-button {
              background: rgba(0, 0, 0, 0.3);
            }

            .ant-carousel-dark .ant-carousel-dot:hover .ant-carousel-dot-button {
              background: rgba(0, 0, 0, 0.5);
            }

            .ant-carousel-dark .ant-carousel-dot-active .ant-carousel-dot-button {
              background: #000;
            }
            "#,
            arrow_color = self.token.color_white,
            primary_color = self.token.color_primary
        )
    }
}

pub fn use_carousel_style() -> String {
    CarouselStyleGenerator::new().generate_style()
}

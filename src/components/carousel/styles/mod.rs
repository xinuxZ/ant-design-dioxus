use crate::theme::{use_token, AliasToken};
use css_in_rust::Style;

pub struct CarouselStyleGenerator {
    token: AliasToken,
}

impl CarouselStyleGenerator {
    pub fn new() -> Self {
        Self { token: use_token() }
    }

    pub fn generate_style(&self) -> String {
        let style = Style::new(format!(
            r#"
            /* Carousel 走马灯组件样式 */

            .ant-carousel {{
                position: relative;
                overflow: hidden;
                width: 100%;
                height: 100%;
                box-sizing: border-box;
            }}

            .ant-carousel-inner {{
                display: flex;
                width: 100%;
                height: 100%;
                transition: transform 0.5s ease;
            }}

            .ant-carousel-item {{
                flex: 0 0 100%;
                width: 100%;
                height: 100%;
                display: flex;
                align-items: center;
                justify-content: center;
            }}

            /* 箭头样式 */
            .ant-carousel-arrow {{
                position: absolute;
                top: 50%;
                transform: translateY(-50%);
                z-index: 2;
                width: 36px;
                height: 36px;
                border: none;
                border-radius: 50%;
                background: rgba(0, 0, 0, 0.2);
                color: {};
                font-size: 18px;
                font-weight: bold;
                cursor: pointer;
                display: flex;
                align-items: center;
                justify-content: center;
                transition: all 0.3s ease;
                opacity: 0;
                visibility: hidden;
            }}

            .ant-carousel:hover .ant-carousel-arrow {{
                opacity: 1;
                visibility: visible;
            }}

            .ant-carousel-arrow:hover {{
                background: rgba(0, 0, 0, 0.5);
                transform: translateY(-50%) scale(1.1);
            }}

            .ant-carousel-arrow-left {{
                left: 16px;
            }}

            .ant-carousel-arrow-right {{
                right: 16px;
            }}

            /* 指示器样式 */
            .ant-carousel-dots {{
                position: absolute;
                margin: 0;
                padding: 0;
                list-style: none;
                display: flex;
                z-index: 2;
            }}

            .ant-carousel-dots-bottom {{
                bottom: 16px;
                left: 50%;
                transform: translateX(-50%);
                flex-direction: row;
            }}

            .ant-carousel-dots-top {{
                top: 16px;
                left: 50%;
                transform: translateX(-50%);
                flex-direction: row;
            }}

            .ant-carousel-dots-left {{
                left: 16px;
                top: 50%;
                transform: translateY(-50%);
                flex-direction: column;
            }}

            .ant-carousel-dots-right {{
                right: 16px;
                top: 50%;
                transform: translateY(-50%);
                flex-direction: column;
            }}

            .ant-carousel-dot {{
                margin: 0 3px;
                cursor: pointer;
            }}

            .ant-carousel-dots-left .ant-carousel-dot,
            .ant-carousel-dots-right .ant-carousel-dot {{
                margin: 3px 0;
            }}

            .ant-carousel-dot-button {{
                width: 16px;
                height: 3px;
                border: none;
                border-radius: 1px;
                background: rgba(255, 255, 255, 0.3);
                cursor: pointer;
                transition: all 0.3s ease;
                display: block;
            }}

            .ant-carousel-dots-left .ant-carousel-dot-button,
            .ant-carousel-dots-right .ant-carousel-dot-button {{
                width: 3px;
                height: 16px;
            }}

            .ant-carousel-dot:hover .ant-carousel-dot-button {{
                background: rgba(255, 255, 255, 0.5);
            }}

            .ant-carousel-dot-active .ant-carousel-dot-button {{
                background: #fff;
                width: 24px;
            }}

            .ant-carousel-dots-left .ant-carousel-dot-active .ant-carousel-dot-button,
            .ant-carousel-dots-right .ant-carousel-dot-active .ant-carousel-dot-button {{
                width: 3px;
                height: 24px;
            }}

            /* 淡入淡出效果 */
            .ant-carousel-fade .ant-carousel-inner {{
                position: relative;
            }}

            .ant-carousel-fade .ant-carousel-item {{
                position: absolute;
                top: 0;
                left: 0;
                opacity: 0;
                transition: opacity 0.5s ease;
            }}

            .ant-carousel-fade .ant-carousel-item-active {{
                opacity: 1;
            }}

            /* 响应式设计 */
            @media (max-width: 768px) {{
                .ant-carousel-arrow {{
                    width: 32px;
                    height: 32px;
                    font-size: 16px;
                }}

                .ant-carousel-arrow-left {{
                    left: 8px;
                }}

                .ant-carousel-arrow-right {{
                    right: 8px;
                }}

                .ant-carousel-dots-bottom {{
                    bottom: 8px;
                }}

                .ant-carousel-dots-top {{
                    top: 8px;
                }}

                .ant-carousel-dots-left {{
                    left: 8px;
                }}

                .ant-carousel-dots-right {{
                    right: 8px;
                }}
            }}

            @media (max-width: 480px) {{
                .ant-carousel-arrow {{
                    width: 28px;
                    height: 28px;
                    font-size: 14px;
                }}

                .ant-carousel-dot-button {{
                    width: 12px;
                    height: 2px;
                }}

                .ant-carousel-dots-left .ant-carousel-dot-button,
                .ant-carousel-dots-right .ant-carousel-dot-button {{
                    width: 2px;
                    height: 12px;
                }}

                .ant-carousel-dot-active .ant-carousel-dot-button {{
                    width: 18px;
                }}

                .ant-carousel-dots-left .ant-carousel-dot-active .ant-carousel-dot-button,
                .ant-carousel-dots-right .ant-carousel-dot-active .ant-carousel-dot-button {{
                    width: 2px;
                    height: 18px;
                }}
            }}

            /* RTL 支持 */
            .ant-carousel-rtl .ant-carousel-arrow-left {{
                right: 16px;
                left: auto;
            }}

            .ant-carousel-rtl .ant-carousel-arrow-right {{
                left: 16px;
                right: auto;
            }}

            .ant-carousel-rtl .ant-carousel-dots-left {{
                right: 16px;
                left: auto;
            }}

            .ant-carousel-rtl .ant-carousel-dots-right {{
                left: 16px;
                right: auto;
            }}

            /* 加载状态 */
            .ant-carousel-loading {{
                opacity: 0.6;
            }}

            .ant-carousel-loading::after {{
                content: "";
                position: absolute;
                top: 50%;
                left: 50%;
                width: 30px;
                height: 30px;
                margin: -15px 0 0 -15px;
                border: 4px solid #f3f3f3;
                border-top: 4px solid {};
                border-radius: 50%;
                animation: ant-carousel-loading 1.2s infinite linear;
            }}

            @keyframes ant-carousel-loading {{
                0% {{
                    transform: rotate(0deg);
                }}
                100% {{
                    transform: rotate(360deg);
                }}
            }}

            .ant-carousel-dark {{
                color: {};
            }}

            .ant-carousel-dark .ant-carousel-arrow {{
                background: rgba(255, 255, 255, 0.2);
                color: {};
            }}

            .ant-carousel-dark .ant-carousel-arrow:hover {{
                background: rgba(255, 255, 255, 0.5);
            }}

            .ant-carousel-dark .ant-carousel-dot-button {{
                background: rgba(0, 0, 0, 0.3);
            }}

            .ant-carousel-dark .ant-carousel-dot:hover .ant-carousel-dot-button {{
                background: rgba(0, 0, 0, 0.5);
            }}

            .ant-carousel-dark .ant-carousel-dot-active .ant-carousel-dot-button {{
                background: #000;
            }}
            "#,
            self.token.color_white,   // 箭头颜色
            self.token.color_primary, // 加载动画颜色
            self.token.color_text,    // 暗色主题文本颜色
            self.token.color_text,    // 暗色主题箭头颜色
        ))
        .unwrap();

        style.get_class_name()
    }
}

pub fn use_carousel_style() -> String {
    let style_generator = CarouselStyleGenerator::new();
    style_generator.generate_style()
}

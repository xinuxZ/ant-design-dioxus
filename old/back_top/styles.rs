//! BackTop 组件样式系统
//!
//! 本模块实现了 BackTop 组件的 CSS-in-Rust 样式系统，
//! 提供了完整的样式生成、主题支持和响应式设计。

use crate::components::back_top::types::*;
use std::collections::HashMap;

/// 生成 BackTop 容器样式
pub fn generate_back_top_container_style(
    config: &BackTopConfig,
    is_visible: bool,
    is_mobile: bool,
) -> String {
    let theme = config.effective_theme(is_mobile);
    let bottom = config.effective_bottom(is_mobile);
    let right = config.effective_right(is_mobile);

    let visibility = if is_visible { "visible" } else { "hidden" };
    let opacity = if is_visible { "1" } else { "0" };
    let pointer_events = if is_visible { "auto" } else { "none" };

    let transform = match config.animation {
        BackTopAnimation::Slide | BackTopAnimation::FadeSlide => {
            if is_visible {
                "translateY(0)"
            } else {
                "translateY(20px)"
            }
        }
        BackTopAnimation::Scale | BackTopAnimation::FadeScale => {
            if is_visible {
                "scale(1)"
            } else {
                "scale(0.8)"
            }
        }
        BackTopAnimation::Bounce => {
            if is_visible {
                "scale(1)"
            } else {
                "scale(0)"
            }
        }
        _ => "none",
    };

    format!(
        "
        position: fixed;
        bottom: {}px;
        right: {}px;
        z-index: {};
        visibility: {};
        opacity: {};
        pointer-events: {};
        transform: {};
        transition: {};
        ",
        bottom,
        right,
        theme.z_index,
        visibility,
        opacity,
        pointer_events,
        transform,
        theme.transition
    )
}

/// 生成 BackTop 按钮样式
pub fn generate_back_top_button_style(
    config: &BackTopConfig,
    is_mobile: bool,
    is_hover: bool,
    is_active: bool,
    is_disabled: bool,
) -> String {
    let theme = config.effective_theme(is_mobile);

    let (background_color, icon_color, box_shadow, border) = if is_disabled {
        (
            &theme.disabled_background_color,
            &theme.disabled_icon_color,
            &"none".to_string(),
            &theme.border,
        )
    } else if is_active {
        (
            &theme.active_background_color,
            &theme.hover_icon_color,
            &theme.hover_box_shadow,
            &theme.hover_border,
        )
    } else if is_hover {
        (
            &theme.hover_background_color,
            &theme.hover_icon_color,
            &theme.hover_box_shadow,
            &theme.hover_border,
        )
    } else {
        (
            &theme.background_color,
            &theme.icon_color,
            &theme.box_shadow,
            &theme.border,
        )
    };

    let cursor = if is_disabled {
        "not-allowed"
    } else {
        "pointer"
    };

    format!(
        "
        width: {};
        height: {};
        background-color: {};
        color: {};
        border: {};
        border-radius: {};
        box-shadow: {};
        font-size: {};
        display: flex;
        align-items: center;
        justify-content: center;
        cursor: {};
        outline: none;
        user-select: none;
        transition: {};
        text-decoration: none;
        ",
        theme.size,
        theme.size,
        background_color,
        icon_color,
        border,
        theme.border_radius,
        box_shadow,
        theme.font_size,
        cursor,
        theme.transition
    )
}

/// 生成默认图标样式
pub fn generate_back_top_icon_style() -> String {
    "
    width: 1em;
    height: 1em;
    fill: currentColor;
    display: inline-block;
    "
    .to_string()
}

/// 生成 BackTop CSS 类名
pub fn generate_back_top_class_name(
    config: &BackTopConfig,
    is_visible: bool,
    is_mobile: bool,
) -> String {
    let mut classes = vec!["ant-back-top"];

    if is_visible {
        classes.push("ant-back-top-visible");
    } else {
        classes.push("ant-back-top-hidden");
    }

    if is_mobile {
        classes.push("ant-back-top-mobile");
    }

    match config.animation {
        BackTopAnimation::Fade => classes.push("ant-back-top-fade"),
        BackTopAnimation::Slide => classes.push("ant-back-top-slide"),
        BackTopAnimation::FadeSlide => classes.push("ant-back-top-fade-slide"),
        BackTopAnimation::Scale => classes.push("ant-back-top-scale"),
        BackTopAnimation::FadeScale => classes.push("ant-back-top-fade-scale"),
        BackTopAnimation::Bounce => classes.push("ant-back-top-bounce"),
        BackTopAnimation::None => classes.push("ant-back-top-no-animation"),
    }

    if config.has_custom_content {
        classes.push("ant-back-top-custom");
    }

    classes.join(" ")
}

/// 生成响应式样式
pub fn generate_responsive_style(config: &BackTopConfig) -> String {
    let mobile_config = config.mobile_config.as_ref();

    let mobile_styles = if let Some(mobile) = mobile_config {
        format!(
            "
            @media (max-width: 768px) {{
                .ant-back-top {{
                    bottom: {}px !important;
                    right: {}px !important;
                }}
            }}
            ",
            mobile.bottom, mobile.right
        )
    } else {
        "
        @media (max-width: 768px) {
            .ant-back-top {
                bottom: 20px !important;
                right: 20px !important;
            }
        }
        "
        .to_string()
    };

    format!(
        "
        {}

        @media (max-width: 480px) {{
            .ant-back-top {{
                bottom: 16px !important;
                right: 16px !important;
            }}
            .ant-back-top button {{
                width: 36px !important;
                height: 36px !important;
                font-size: 16px !important;
            }}
        }}

        @media (prefers-reduced-motion: reduce) {{
            .ant-back-top {{
                transition: none !important;
            }}
            .ant-back-top button {{
                transition: none !important;
            }}
        }}
        ",
        mobile_styles
    )
}

/// 生成动画样式
pub fn generate_animation_style(animation: &BackTopAnimation, easing: &EasingFunction) -> String {
    let easing_str = easing.to_string();

    match animation {
        BackTopAnimation::Fade => format!(
            "
            .ant-back-top-fade {{
                transition: opacity 0.3s {};
            }}
            .ant-back-top-fade.ant-back-top-hidden {{
                opacity: 0;
            }}
            .ant-back-top-fade.ant-back-top-visible {{
                opacity: 1;
            }}
            ",
            easing_str
        ),
        BackTopAnimation::Slide => format!(
            "
            .ant-back-top-slide {{
                transition: transform 0.3s {};
            }}
            .ant-back-top-slide.ant-back-top-hidden {{
                transform: translateY(20px);
            }}
            .ant-back-top-slide.ant-back-top-visible {{
                transform: translateY(0);
            }}
            ",
            easing_str
        ),
        BackTopAnimation::FadeSlide => format!(
            "
            .ant-back-top-fade-slide {{
                transition: opacity 0.3s {}, transform 0.3s {};
            }}
            .ant-back-top-fade-slide.ant-back-top-hidden {{
                opacity: 0;
                transform: translateY(20px);
            }}
            .ant-back-top-fade-slide.ant-back-top-visible {{
                opacity: 1;
                transform: translateY(0);
            }}
            ",
            easing_str, easing_str
        ),
        BackTopAnimation::Scale => format!(
            "
            .ant-back-top-scale {{
                transition: transform 0.3s {};
            }}
            .ant-back-top-scale.ant-back-top-hidden {{
                transform: scale(0.8);
            }}
            .ant-back-top-scale.ant-back-top-visible {{
                transform: scale(1);
            }}
            ",
            easing_str
        ),
        BackTopAnimation::FadeScale => format!(
            "
            .ant-back-top-fade-scale {{
                transition: opacity 0.3s {}, transform 0.3s {};
            }}
            .ant-back-top-fade-scale.ant-back-top-hidden {{
                opacity: 0;
                transform: scale(0.8);
            }}
            .ant-back-top-fade-scale.ant-back-top-visible {{
                opacity: 1;
                transform: scale(1);
            }}
            ",
            easing_str, easing_str
        ),
        BackTopAnimation::Bounce => format!(
            "
            .ant-back-top-bounce {{
                transition: transform 0.4s {};
            }}
            .ant-back-top-bounce.ant-back-top-hidden {{
                transform: scale(0);
            }}
            .ant-back-top-bounce.ant-back-top-visible {{
                transform: scale(1);
            }}
            .ant-back-top-bounce:active {{
                transform: scale(0.95);
            }}
            ",
            easing_str
        ),
        BackTopAnimation::None => "
            .ant-back-top-no-animation {
                transition: none;
            }
            "
        .to_string(),
    }
}

/// 生成可访问性样式
pub fn generate_accessibility_style() -> String {
    "
    .ant-back-top:focus-visible {
        outline: 2px solid #1890ff;
        outline-offset: 2px;
    }

    .ant-back-top button:focus-visible {
        outline: 2px solid #1890ff;
        outline-offset: 2px;
    }

    @media (prefers-reduced-motion: reduce) {
        .ant-back-top {
            transition: none !important;
        }
        .ant-back-top button {
            transition: none !important;
        }
    }

    @media (prefers-contrast: high) {
        .ant-back-top button {
            border: 2px solid currentColor;
        }
    }
    "
    .to_string()
}

/// 生成暗色主题样式
pub fn generate_dark_theme_style() -> String {
    let dark_theme = BackTopDarkTheme::theme();

    format!(
        "
        [data-theme='dark'] .ant-back-top button,
        .ant-back-top.ant-back-top-dark button {{
            background-color: {};
            color: {};
            box-shadow: {};
        }}

        [data-theme='dark'] .ant-back-top button:hover,
        .ant-back-top.ant-back-top-dark button:hover {{
            background-color: {};
            color: {};
            box-shadow: {};
        }}

        [data-theme='dark'] .ant-back-top button:active,
        .ant-back-top.ant-back-top-dark button:active {{
            background-color: {};
        }}
        ",
        dark_theme.background_color,
        dark_theme.icon_color,
        dark_theme.box_shadow,
        dark_theme.hover_background_color,
        dark_theme.hover_icon_color,
        dark_theme.hover_box_shadow,
        dark_theme.active_background_color
    )
}

/// 生成紧凑主题样式
pub fn generate_compact_theme_style() -> String {
    let compact_theme = BackTopCompactTheme::theme();

    format!(
        "
        .ant-back-top.ant-back-top-compact button {{
            width: {};
            height: {};
            font-size: {};
            border-radius: {};
            box-shadow: {};
            transition: {};
        }}

        .ant-back-top.ant-back-top-compact button:hover {{
            box-shadow: {};
        }}
        ",
        compact_theme.size,
        compact_theme.size,
        compact_theme.font_size,
        compact_theme.border_radius,
        compact_theme.box_shadow,
        compact_theme.transition,
        compact_theme.hover_box_shadow
    )
}

/// 生成完整的 CSS 样式表
pub fn generate_back_top_stylesheet(config: &BackTopConfig) -> String {
    format!(
        "
        /* BackTop 基础样式 */
        .ant-back-top {{
            position: fixed;
            z-index: {};
            cursor: pointer;
        }}

        .ant-back-top button {{
            border: none;
            outline: none;
            cursor: pointer;
            user-select: none;
            display: flex;
            align-items: center;
            justify-content: center;
            text-decoration: none;
        }}

        .ant-back-top-hidden {{
            visibility: hidden;
            opacity: 0;
            pointer-events: none;
        }}

        .ant-back-top-visible {{
            visibility: visible;
            opacity: 1;
            pointer-events: auto;
        }}

        /* 动画样式 */
        {}

        /* 响应式样式 */
        {}

        /* 可访问性样式 */
        {}

        /* 暗色主题样式 */
        {}

        /* 紧凑主题样式 */
        {}
        ",
        config.theme.z_index,
        generate_animation_style(&config.animation, &config.easing),
        generate_responsive_style(config),
        generate_accessibility_style(),
        generate_dark_theme_style(),
        generate_compact_theme_style()
    )
}

/// BackTop 样式生成器
#[derive(Clone, Debug)]
pub struct BackTopStyleGenerator {
    config: BackTopConfig,
    cache: HashMap<String, String>,
}

impl BackTopStyleGenerator {
    /// 创建新的样式生成器
    pub fn new(config: BackTopConfig) -> Self {
        Self {
            config,
            cache: HashMap::new(),
        }
    }

    /// 生成容器样式
    pub fn container_style(&mut self, is_visible: bool, is_mobile: bool) -> &str {
        let key = format!("container_{}_{}", is_visible, is_mobile);

        if !self.cache.contains_key(&key) {
            let style = generate_back_top_container_style(&self.config, is_visible, is_mobile);
            self.cache.insert(key.clone(), style);
        }

        self.cache.get(&key).unwrap()
    }

    /// 生成按钮样式
    pub fn button_style(
        &mut self,
        is_mobile: bool,
        is_hover: bool,
        is_active: bool,
        is_disabled: bool,
    ) -> &str {
        let key = format!(
            "button_{}_{}_{}_{}",
            is_mobile, is_hover, is_active, is_disabled
        );

        if !self.cache.contains_key(&key) {
            let style = generate_back_top_button_style(
                &self.config,
                is_mobile,
                is_hover,
                is_active,
                is_disabled,
            );
            self.cache.insert(key.clone(), style);
        }

        self.cache.get(&key).unwrap()
    }

    /// 生成图标样式
    pub fn icon_style(&mut self) -> &str {
        let key = "icon".to_string();

        if !self.cache.contains_key(&key) {
            let style = generate_back_top_icon_style();
            self.cache.insert(key.clone(), style);
        }

        self.cache.get(&key).unwrap()
    }

    /// 生成类名
    pub fn class_name(&mut self, is_visible: bool, is_mobile: bool) -> &str {
        let key = format!("class_{}_{}", is_visible, is_mobile);

        if !self.cache.contains_key(&key) {
            let class = generate_back_top_class_name(&self.config, is_visible, is_mobile);
            self.cache.insert(key.clone(), class);
        }

        self.cache.get(&key).unwrap()
    }

    /// 生成完整样式表
    pub fn stylesheet(&mut self) -> &str {
        let key = "stylesheet".to_string();

        if !self.cache.contains_key(&key) {
            let stylesheet = generate_back_top_stylesheet(&self.config);
            self.cache.insert(key.clone(), stylesheet);
        }

        self.cache.get(&key).unwrap()
    }

    /// 更新配置
    pub fn update_config(&mut self, config: BackTopConfig) {
        self.config = config;
        self.cache.clear();
    }

    /// 清除缓存
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    /// 获取缓存大小
    pub fn cache_size(&self) -> usize {
        self.cache.len()
    }

    /// 获取配置引用
    pub fn config(&self) -> &BackTopConfig {
        &self.config
    }
}

impl Default for BackTopStyleGenerator {
    fn default() -> Self {
        Self::new(BackTopConfig::default())
    }
}

/// 生成内联样式字符串
pub fn generate_inline_style(
    config: &BackTopConfig,
    is_visible: bool,
    is_mobile: bool,
    is_hover: bool,
    is_active: bool,
    is_disabled: bool,
) -> String {
    let container_style = generate_back_top_container_style(config, is_visible, is_mobile);
    let button_style =
        generate_back_top_button_style(config, is_mobile, is_hover, is_active, is_disabled);

    format!("{}{}", container_style, button_style)
}

/// 生成 CSS 变量样式
pub fn generate_css_variables_style(theme: &BackTopTheme) -> String {
    format!(
        "
        --back-top-bg-color: {};
        --back-top-hover-bg-color: {};
        --back-top-icon-color: {};
        --back-top-hover-icon-color: {};
        --back-top-border-radius: {};
        --back-top-box-shadow: {};
        --back-top-hover-box-shadow: {};
        --back-top-transition: {};
        --back-top-z-index: {};
        --back-top-size: {};
        --back-top-font-size: {};
        --back-top-border: {};
        --back-top-hover-border: {};
        --back-top-active-bg-color: {};
        --back-top-disabled-bg-color: {};
        --back-top-disabled-icon-color: {};
        ",
        theme.background_color,
        theme.hover_background_color,
        theme.icon_color,
        theme.hover_icon_color,
        theme.border_radius,
        theme.box_shadow,
        theme.hover_box_shadow,
        theme.transition,
        theme.z_index,
        theme.size,
        theme.font_size,
        theme.border,
        theme.hover_border,
        theme.active_background_color,
        theme.disabled_background_color,
        theme.disabled_icon_color
    )
}

/// 生成默认图标 SVG
pub fn generate_default_icon_svg() -> String {
    r#"
    <svg viewBox="0 0 1024 1024" fill="currentColor">
        <path d="M868 545.5L536.1 163a31.96 31.96 0 0 0-48.3 0L156 545.5a7.97 7.97 0 0 0 6 13.2h81c4.6 0 9-2 12.1-5.5L474 300.9V864c0 4.4 3.6 8 8 8h60c4.4 0 8-3.6 8-8V300.9l218.9 252.3c3.1 3.5 7.5 5.5 12.1 5.5h81c6.8 0 10.5-8 6-13.2z"/>
    </svg>
    "#.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_back_top_container_style() {
        let config = BackTopConfig::default();
        let style = generate_back_top_container_style(&config, true, false);

        assert!(style.contains("position: fixed"));
        assert!(style.contains("bottom: 50px"));
        assert!(style.contains("right: 50px"));
        assert!(style.contains("opacity: 1"));
        assert!(style.contains("visibility: visible"));
    }

    #[test]
    fn test_generate_back_top_button_style() {
        let config = BackTopConfig::default();
        let style = generate_back_top_button_style(&config, false, false, false, false);

        assert!(style.contains("width: 40px"));
        assert!(style.contains("height: 40px"));
        assert!(style.contains("display: flex"));
        assert!(style.contains("cursor: pointer"));
    }

    #[test]
    fn test_generate_back_top_class_name() {
        let config = BackTopConfig::default();
        let class_name = generate_back_top_class_name(&config, true, false);

        assert!(class_name.contains("ant-back-top"));
        assert!(class_name.contains("ant-back-top-visible"));
        assert!(class_name.contains("ant-back-top-fade-slide"));
    }

    #[test]
    fn test_generate_animation_style() {
        let fade_style =
            generate_animation_style(&BackTopAnimation::Fade, &EasingFunction::EaseOut);

        assert!(fade_style.contains(".ant-back-top-fade"));
        assert!(fade_style.contains("opacity"));
        assert!(fade_style.contains("transition"));

        let slide_style =
            generate_animation_style(&BackTopAnimation::Slide, &EasingFunction::EaseOut);

        assert!(slide_style.contains(".ant-back-top-slide"));
        assert!(slide_style.contains("transform"));
        assert!(slide_style.contains("translateY"));
    }

    #[test]
    fn test_generate_responsive_style() {
        let config = BackTopConfig::default();
        let style = generate_responsive_style(&config);

        assert!(style.contains("@media (max-width: 768px)"));
        assert!(style.contains("@media (max-width: 480px)"));
        assert!(style.contains("@media (prefers-reduced-motion: reduce)"));
    }

    #[test]
    fn test_generate_accessibility_style() {
        let style = generate_accessibility_style();

        assert!(style.contains(":focus-visible"));
        assert!(style.contains("outline"));
        assert!(style.contains("@media (prefers-reduced-motion: reduce)"));
        assert!(style.contains("@media (prefers-contrast: high)"));
    }

    #[test]
    fn test_generate_dark_theme_style() {
        let style = generate_dark_theme_style();

        assert!(style.contains("[data-theme='dark']"));
        assert!(style.contains(".ant-back-top-dark"));
        assert!(style.contains(":hover"));
        assert!(style.contains(":active"));
    }

    #[test]
    fn test_generate_compact_theme_style() {
        let style = generate_compact_theme_style();

        assert!(style.contains(".ant-back-top-compact"));
        assert!(style.contains("32px"));
        assert!(style.contains("16px"));
    }

    #[test]
    fn test_back_top_style_generator() {
        let mut generator = BackTopStyleGenerator::new(BackTopConfig::default());

        let container_style = generator.container_style(true, false);
        assert!(container_style.contains("position: fixed"));

        let button_style = generator.button_style(false, false, false, false);
        assert!(button_style.contains("display: flex"));

        let icon_style = generator.icon_style();
        assert!(icon_style.contains("fill: currentColor"));

        let class_name = generator.class_name(true, false);
        assert!(class_name.contains("ant-back-top"));

        let stylesheet = generator.stylesheet();
        assert!(stylesheet.contains(".ant-back-top"));

        assert_eq!(generator.cache_size(), 5);

        generator.clear_cache();
        assert_eq!(generator.cache_size(), 0);
    }

    #[test]
    fn test_generate_inline_style() {
        let config = BackTopConfig::default();
        let style = generate_inline_style(&config, true, false, false, false, false);

        assert!(style.contains("position: fixed"));
        assert!(style.contains("display: flex"));
    }

    #[test]
    fn test_generate_css_variables_style() {
        let theme = BackTopTheme::default();
        let style = generate_css_variables_style(&theme);

        assert!(style.contains("--back-top-bg-color"));
        assert!(style.contains("--back-top-hover-bg-color"));
        assert!(style.contains("--back-top-icon-color"));
        assert!(style.contains("--back-top-z-index"));
    }

    #[test]
    fn test_generate_default_icon_svg() {
        let svg = generate_default_icon_svg();

        assert!(svg.contains("<svg"));
        assert!(svg.contains("viewBox"));
        assert!(svg.contains("fill=\"currentColor\""));
        assert!(svg.contains("<path"));
    }

    #[test]
    fn test_style_generator_config_update() {
        let mut generator = BackTopStyleGenerator::new(BackTopConfig::default());

        // 生成一些样式以填充缓存
        let _ = generator.container_style(true, false);
        let _ = generator.button_style(false, false, false, false);
        assert!(generator.cache_size() > 0);

        // 更新配置应该清除缓存
        let new_config = BackTopConfig {
            bottom: 100,
            right: 100,
            ..Default::default()
        };
        generator.update_config(new_config);
        assert_eq!(generator.cache_size(), 0);
        assert_eq!(generator.config().bottom, 100);
        assert_eq!(generator.config().right, 100);
    }
}

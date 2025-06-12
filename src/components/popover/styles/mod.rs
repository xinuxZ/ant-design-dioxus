//! Popover 气泡卡片组件样式

use crate::theme::{get_theme, ThemeMode};

/// Popover 气泡卡片组件的位置
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PopoverPlacement {
    Top,
    TopLeft,
    TopRight,
    Bottom,
    BottomLeft,
    BottomRight,
    Left,
    LeftTop,
    LeftBottom,
    Right,
    RightTop,
    RightBottom,
}

impl Default for PopoverPlacement {
    fn default() -> Self {
        Self::Top
    }
}

/// Popover 气泡卡片组件的触发方式
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PopoverTrigger {
    Hover,
    Click,
    Focus,
    ContextMenu,
}

impl Default for PopoverTrigger {
    fn default() -> Self {
        Self::Hover
    }
}

/// Popover 气泡卡片组件的样式生成器
pub struct PopoverStyleGenerator {
    placement: PopoverPlacement,
    trigger: PopoverTrigger,
    arrow: bool,
    dark_theme: bool,
    with_title: bool,
    is_rtl: bool,
    theme_mode: ThemeMode,
    custom_width: Option<String>,
    custom_z_index: Option<u32>,
    high_contrast: bool,
    reduced_motion: bool,
}

impl PopoverStyleGenerator {
    /// 创建一个新的 Popover 样式生成器
    pub fn new() -> Self {
        Self {
            placement: PopoverPlacement::default(),
            trigger: PopoverTrigger::default(),
            arrow: true,
            dark_theme: false,
            with_title: false,
            is_rtl: false,
            theme_mode: get_theme(),
            custom_width: None,
            custom_z_index: None,
            high_contrast: false,
            reduced_motion: false,
        }
    }

    /// 设置位置
    pub fn with_placement(mut self, placement: PopoverPlacement) -> Self {
        self.placement = placement;
        self
    }

    /// 设置触发方式
    pub fn with_trigger(mut self, trigger: PopoverTrigger) -> Self {
        self.trigger = trigger;
        self
    }

    /// 设置是否显示箭头
    pub fn with_arrow(mut self, arrow: bool) -> Self {
        self.arrow = arrow;
        self
    }

    /// 设置是否使用深色主题
    pub fn with_dark_theme(mut self, dark_theme: bool) -> Self {
        self.dark_theme = dark_theme;
        self
    }

    /// 设置是否有标题
    pub fn with_title(mut self, with_title: bool) -> Self {
        self.with_title = with_title;
        self
    }

    /// 设置是否是 RTL 方向
    pub fn with_rtl(mut self, is_rtl: bool) -> Self {
        self.is_rtl = is_rtl;
        self
    }

    /// 设置主题模式
    pub fn with_theme_mode(mut self, theme_mode: ThemeMode) -> Self {
        self.theme_mode = theme_mode;
        self
    }

    /// 设置自定义宽度
    pub fn with_custom_width(mut self, width: impl Into<String>) -> Self {
        self.custom_width = Some(width.into());
        self
    }

    /// 设置自定义 z-index
    pub fn with_custom_z_index(mut self, z_index: u32) -> Self {
        self.custom_z_index = Some(z_index);
        self
    }

    /// 设置是否启用高对比度模式
    pub fn with_high_contrast(mut self, high_contrast: bool) -> Self {
        self.high_contrast = high_contrast;
        self
    }

    /// 设置是否启用减少动画模式
    pub fn with_reduced_motion(mut self, reduced_motion: bool) -> Self {
        self.reduced_motion = reduced_motion;
        self
    }

    /// 生成基础样式
    fn generate_base_style(&self) -> String {
        let z_index = self.custom_z_index.unwrap_or(1030);
        let min_width = self.custom_width.as_deref().unwrap_or("177px");

        let bg_color = if self.dark_theme || self.theme_mode == ThemeMode::Dark {
            "#1f1f1f"
        } else {
            "#fff"
        };

        let text_color = if self.dark_theme || self.theme_mode == ThemeMode::Dark {
            "rgba(255, 255, 255, 0.85)"
        } else {
            "rgba(0, 0, 0, 0.85)"
        };

        let shadow = if self.dark_theme || self.theme_mode == ThemeMode::Dark {
            "0 6px 16px 0 rgba(0, 0, 0, 0.18), 0 3px 6px -4px rgba(0, 0, 0, 0.22), 0 9px 28px 8px rgba(0, 0, 0, 0.15)"
        } else {
            "0 6px 16px 0 rgba(0, 0, 0, 0.08), 0 3px 6px -4px rgba(0, 0, 0, 0.12), 0 9px 28px 8px rgba(0, 0, 0, 0.05)"
        };

        let border_color = if self.dark_theme || self.theme_mode == ThemeMode::Dark {
            "#303030"
        } else {
            "rgba(5, 5, 5, 0.06)"
        };

        let mut css = String::new();

        css.push_str(&format!(
            r#"
/* Popover 气泡卡片组件样式 */

.ant-popover {{
  position: absolute;
  top: 0;
  left: 0;
  z-index: {z_index};
  font-weight: normal;
  white-space: normal;
  text-align: {text_align};
  cursor: auto;
  user-select: text;
  pointer-events: auto;
  box-sizing: border-box;
}}

.ant-popover-wrapper {{
  position: relative;
  display: inline-block;
}}

.ant-popover-trigger {{
  display: inline-block;
}}

.ant-popover-hidden {{
  display: none;
}}

.ant-popover-content {{
  position: relative;
}}

.ant-popover-inner {{
  background-color: {bg_color};
  background-clip: padding-box;
  border-radius: 6px;
  box-shadow: {shadow};
  padding: 12px 16px;
  color: {text_color};
  text-decoration: none;
  word-wrap: break-word;
  min-width: {min_width};
  min-height: 32px;
}}

.ant-popover-title {{
  min-width: {min_width};
  margin-bottom: 8px;
  color: {text_color};
  font-weight: 600;
  font-size: 14px;
  line-height: 1.5;
}}

.ant-popover-inner-content {{
  color: {text_color};
  font-size: 14px;
  line-height: 1.5715;
}}
"#,
            z_index = z_index,
            text_align = if self.is_rtl { "right" } else { "left" },
            bg_color = bg_color,
            shadow = shadow,
            text_color = text_color,
            min_width = min_width,
        ));

        // 如果有标题，添加标题样式
        if self.with_title {
            css.push_str(&format!(
                r#"
.ant-popover-with-title .ant-popover-title {{
  padding-bottom: 8px;
  border-bottom: 1px solid {border_color};
}}

.ant-popover-with-title .ant-popover-inner-content {{
  padding-top: 8px;
}}
"#,
                border_color = border_color
            ));
        }

        css
    }

    /// 生成箭头样式
    fn generate_arrow_style(&self) -> String {
        if !self.arrow {
            return String::new();
        }

        let bg_color = if self.dark_theme || self.theme_mode == ThemeMode::Dark {
            "#1f1f1f"
        } else {
            "#fff"
        };

        let shadow = if self.dark_theme || self.theme_mode == ThemeMode::Dark {
            "3px 3px 7px rgba(0, 0, 0, 0.17)"
        } else {
            "3px 3px 7px rgba(0, 0, 0, 0.07)"
        };

        format!(
            r#"
/* 箭头样式 */
.ant-popover-arrow {{
  position: absolute;
  display: block;
  pointer-events: none;
  width: 16px;
  height: 16px;
  overflow: hidden;
}}

.ant-popover-arrow-content {{
  position: absolute;
  top: 0;
  right: 0;
  bottom: 0;
  left: 0;
  display: block;
  width: 8.48528137px;
  height: 8.48528137px;
  margin: auto;
  background-color: {bg_color};
  border-style: solid;
  border-width: 1px;
  border-color: transparent;
  transform: rotate(45deg);
  box-shadow: {shadow};
  z-index: 1;
}}
"#,
            bg_color = bg_color,
            shadow = shadow
        )
    }

    /// 生成位置样式
    fn generate_placement_style(&self) -> String {
        let mut css = String::new();

        // 顶部位置样式
        css.push_str(
            r#"
/* 位置样式 */
.ant-popover-placement-top {
  padding-bottom: 10px;
}

.ant-popover-placement-top .ant-popover-arrow {
  bottom: 6px;
  left: 50%;
  transform: translateX(-50%);
}

.ant-popover-placement-topLeft {
  padding-bottom: 10px;
}

.ant-popover-placement-topLeft .ant-popover-arrow {
  bottom: 6px;
  left: 16px;
}

.ant-popover-placement-topRight {
  padding-bottom: 10px;
}

.ant-popover-placement-topRight .ant-popover-arrow {
  bottom: 6px;
  right: 16px;
}
"#,
        );

        // 底部位置样式
        css.push_str(
            r#"
.ant-popover-placement-bottom {
  padding-top: 10px;
}

.ant-popover-placement-bottom .ant-popover-arrow {
  top: 6px;
  left: 50%;
  transform: translateX(-50%);
}

.ant-popover-placement-bottomLeft {
  padding-top: 10px;
}

.ant-popover-placement-bottomLeft .ant-popover-arrow {
  top: 6px;
  left: 16px;
}

.ant-popover-placement-bottomRight {
  padding-top: 10px;
}

.ant-popover-placement-bottomRight .ant-popover-arrow {
  top: 6px;
  right: 16px;
}
"#,
        );

        // 左侧位置样式
        css.push_str(
            r#"
.ant-popover-placement-left {
  padding-right: 10px;
}

.ant-popover-placement-left .ant-popover-arrow {
  right: 6px;
  top: 50%;
  transform: translateY(-50%);
}

.ant-popover-placement-leftTop {
  padding-right: 10px;
}

.ant-popover-placement-leftTop .ant-popover-arrow {
  right: 6px;
  top: 16px;
}

.ant-popover-placement-leftBottom {
  padding-right: 10px;
}

.ant-popover-placement-leftBottom .ant-popover-arrow {
  right: 6px;
  bottom: 16px;
}
"#,
        );

        // 右侧位置样式
        css.push_str(
            r#"
.ant-popover-placement-right {
  padding-left: 10px;
}

.ant-popover-placement-right .ant-popover-arrow {
  left: 6px;
  top: 50%;
  transform: translateY(-50%);
}

.ant-popover-placement-rightTop {
  padding-left: 10px;
}

.ant-popover-placement-rightTop .ant-popover-arrow {
  left: 6px;
  top: 16px;
}

.ant-popover-placement-rightBottom {
  padding-left: 10px;
}

.ant-popover-placement-rightBottom .ant-popover-arrow {
  left: 6px;
  bottom: 16px;
}
"#,
        );

        css
    }

    /// 生成动画样式
    fn generate_animation_style(&self) -> String {
        if self.reduced_motion {
            return String::new();
        }

        let mut css = String::new();

        css.push_str(
            r#"
/* 动画效果 */
.ant-popover {
  animation: ant-popover-fade-in 0.2s ease-out;
}

.ant-popover-hidden {
  animation: ant-popover-fade-out 0.2s ease-out;
}

@keyframes ant-popover-fade-in {
  0% {
    opacity: 0;
    transform: scale(0.8);
  }

  100% {
    opacity: 1;
    transform: scale(1);
  }
}

@keyframes ant-popover-fade-out {
  0% {
    opacity: 1;
    transform: scale(1);
  }

  100% {
    opacity: 0;
    transform: scale(0.8);
  }
}
"#,
        );

        css
    }

    /// 生成响应式样式
    fn generate_responsive_style(&self) -> String {
        let mut css = String::new();

        css.push_str(
            r#"
/* 响应式设计 */
@media (max-width: 575px) {
  .ant-popover-inner {
    min-width: 120px;
    padding: 8px 12px;
  }

  .ant-popover-title {
    min-width: 120px;
    margin-bottom: 6px;
    font-size: 13px;
  }

  .ant-popover-inner-content {
    font-size: 13px;
  }
}

@media (max-width: 480px) {
  .ant-popover-inner {
    min-width: 100px;
    padding: 6px 10px;
  }

  .ant-popover-title {
    min-width: 100px;
    margin-bottom: 4px;
    font-size: 12px;
  }

  .ant-popover-inner-content {
    font-size: 12px;
  }

  .ant-popover-arrow {
    width: 12px;
    height: 12px;
  }

  .ant-popover-arrow-content {
    width: 6.48528137px;
    height: 6.48528137px;
  }
}
"#,
        );

        css
    }

    /// 生成 RTL 样式
    fn generate_rtl_style(&self) -> String {
        if !self.is_rtl {
            return String::new();
        }

        let mut css = String::new();

        css.push_str(
            r#"
.ant-popover-rtl {
  direction: rtl;
  text-align: right;
}

.ant-popover-rtl .ant-popover-placement-topLeft .ant-popover-arrow {
  left: auto;
  right: 16px;
}

.ant-popover-rtl .ant-popover-placement-topRight .ant-popover-arrow {
  right: auto;
  left: 16px;
}

.ant-popover-rtl .ant-popover-placement-bottomLeft .ant-popover-arrow {
  left: auto;
  right: 16px;
}

.ant-popover-rtl .ant-popover-placement-bottomRight .ant-popover-arrow {
  right: auto;
  left: 16px;
}

.ant-popover-rtl .ant-popover-placement-leftTop .ant-popover-arrow,
.ant-popover-rtl .ant-popover-placement-leftBottom .ant-popover-arrow {
  right: auto;
  left: 6px;
  transform: rotate(45deg);
}

.ant-popover-rtl .ant-popover-placement-rightTop .ant-popover-arrow,
.ant-popover-rtl .ant-popover-placement-rightBottom .ant-popover-arrow {
  left: auto;
  right: 6px;
  transform: rotate(45deg);
}
"#,
        );

        css
    }

    /// 生成深色主题样式
    fn generate_dark_theme_style(&self) -> String {
        if !self.dark_theme && self.theme_mode != ThemeMode::Dark {
            return String::new();
        }

        let mut css = String::new();

        css.push_str(
            r#"
.ant-popover-dark .ant-popover-inner {
  background-color: #1f1f1f;
  box-shadow: 0 6px 16px 0 rgba(0, 0, 0, 0.18), 0 3px 6px -4px rgba(0, 0, 0, 0.22), 0 9px 28px 8px rgba(0, 0, 0, 0.15);
}

.ant-popover-dark .ant-popover-title {
  color: rgba(255, 255, 255, 0.85);
  border-bottom-color: #303030;
}

.ant-popover-dark .ant-popover-inner-content {
  color: rgba(255, 255, 255, 0.85);
}

.ant-popover-dark .ant-popover-arrow-content {
  background-color: #1f1f1f;
  box-shadow: 3px 3px 7px rgba(0, 0, 0, 0.17);
}
"#,
        );

        css
    }

    /// 生成高对比度模式样式
    fn generate_high_contrast_style(&self) -> String {
        if !self.high_contrast {
            return String::new();
        }

        let mut css = String::new();

        css.push_str(
            r#"
/* 高对比度模式 */
@media (forced-colors: active) {
  .ant-popover-inner {
    border: 1px solid CanvasText;
  }

  .ant-popover-arrow-content {
    border-color: CanvasText;
  }

  .ant-popover-title {
    border-bottom: 1px solid CanvasText;
  }
}
"#,
        );

        css
    }

    /// 生成额外样式
    fn generate_extra_style(&self) -> String {
        let mut css = String::new();

        let primary_color = if self.dark_theme || self.theme_mode == ThemeMode::Dark {
            "#177ddc"
        } else {
            "#1677ff"
        };

        css.push_str(&format!(
            r#"
.ant-popover-disabled {{
  cursor: not-allowed;
}}

.ant-popover-disabled .ant-popover-trigger {{
  pointer-events: none;
}}

.ant-popover-loading {{
  position: relative;
}}

.ant-popover-loading .ant-popover-inner {{
  padding: 24px;
}}

.ant-popover-loading .ant-popover-inner::after {{
  content: "";
  position: absolute;
  top: 50%;
  left: 50%;
  width: 20px;
  height: 20px;
  margin-top: -10px;
  margin-left: -10px;
  border: 2px solid {primary_color};
  border-radius: 50%;
  border-top-color: transparent;
  animation: {animation};
}}

@keyframes ant-popover-loading {{
  from {{
    transform: rotate(0deg);
  }}
  to {{
    transform: rotate(360deg);
  }}
}}
"#,
            primary_color = primary_color,
            animation = if self.reduced_motion {
                "none"
            } else {
                "ant-popover-loading 1s linear infinite"
            }
        ));

        css
    }

    /// 生成完整样式
    pub fn generate(&self) -> String {
        let mut css = String::new();

        css.push_str(&self.generate_base_style());
        css.push_str(&self.generate_arrow_style());
        css.push_str(&self.generate_placement_style());
        css.push_str(&self.generate_animation_style());
        css.push_str(&self.generate_responsive_style());
        css.push_str(&self.generate_rtl_style());
        css.push_str(&self.generate_dark_theme_style());
        css.push_str(&self.generate_high_contrast_style());
        css.push_str(&self.generate_extra_style());

        css
    }
}

/// 生成 Popover 样式
pub fn generate_popover_style(
    placement: PopoverPlacement,
    trigger: PopoverTrigger,
    arrow: bool,
    dark_theme: bool,
    with_title: bool,
    is_rtl: bool,
) -> String {
    let theme_mode = get_theme();
    let reduced_motion = crate::theme::prefers_reduced_motion();
    let high_contrast = crate::theme::prefers_high_contrast();

    PopoverStyleGenerator::new()
        .with_placement(placement)
        .with_trigger(trigger)
        .with_arrow(arrow)
        .with_dark_theme(dark_theme)
        .with_title(with_title)
        .with_rtl(is_rtl)
        .with_theme_mode(theme_mode)
        .with_high_contrast(high_contrast)
        .with_reduced_motion(reduced_motion)
        .generate()
}

/// 默认 Popover 样式
pub fn default_popover_style() -> String {
    let theme_mode = get_theme();
    let reduced_motion = crate::theme::prefers_reduced_motion();
    let high_contrast = crate::theme::prefers_high_contrast();

    PopoverStyleGenerator::new()
        .with_theme_mode(theme_mode)
        .with_high_contrast(high_contrast)
        .with_reduced_motion(reduced_motion)
        .generate()
}

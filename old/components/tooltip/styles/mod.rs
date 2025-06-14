//! Tooltip 文字提示组件样式

/// Tooltip 触发方式
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TooltipTrigger {
    /// 鼠标移入时触发
    Hover,
    /// 鼠标点击时触发
    Click,
    /// 获得焦点时触发
    Focus,
    /// 手动触发
    Manual,
}

/// Tooltip 弹出位置
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TooltipPlacement {
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

/// Tooltip 主题
#[derive(Debug, Clone, PartialEq)]
pub enum TooltipColor {
    /// 默认主题
    Default,
    /// 自定义颜色
    Custom(String),
}

/// Tooltip 样式生成器
pub struct TooltipStyleGenerator {
    placement: TooltipPlacement,
    trigger: TooltipTrigger,
    color: TooltipColor,
    arrow: bool,
    high_contrast: bool,
    dark_mode: bool,
    mobile: bool,
}

impl TooltipStyleGenerator {
    /// 创建一个新的 Tooltip 样式生成器
    pub fn new() -> Self {
        Self {
            placement: TooltipPlacement::Top,
            trigger: TooltipTrigger::Hover,
            color: TooltipColor::Default,
            arrow: true,
            high_contrast: false,
            dark_mode: false,
            mobile: false,
        }
    }

    /// 设置弹出位置
    pub fn with_placement(mut self, placement: TooltipPlacement) -> Self {
        self.placement = placement;
        self
    }

    /// 设置触发方式
    pub fn with_trigger(mut self, trigger: TooltipTrigger) -> Self {
        self.trigger = trigger;
        self
    }

    /// 设置颜色
    pub fn with_color(mut self, color: TooltipColor) -> Self {
        self.color = color;
        self
    }

    /// 设置是否显示箭头
    pub fn with_arrow(mut self, arrow: bool) -> Self {
        self.arrow = arrow;
        self
    }

    /// 设置是否高对比度模式
    pub fn with_high_contrast(mut self, high_contrast: bool) -> Self {
        self.high_contrast = high_contrast;
        self
    }

    /// 设置是否暗黑模式
    pub fn with_dark_mode(mut self, dark_mode: bool) -> Self {
        self.dark_mode = dark_mode;
        self
    }

    /// 设置是否移动端
    pub fn with_mobile(mut self, mobile: bool) -> Self {
        self.mobile = mobile;
        self
    }

    /// 生成基础样式
    fn generate_base_style(&self) -> String {
        let mut css = String::new();

        css.push_str(
            r#"
/* Tooltip 文字提示组件样式 */

.ant-tooltip-wrapper {
  position: relative;
  display: inline-block;
}

.ant-tooltip-trigger {
  display: inline-block;
  cursor: pointer;
}

.ant-tooltip {
  position: absolute;
  z-index: 1070;
  display: block;
  visibility: visible;
  font-size: 14px;
  font-style: normal;
  font-weight: 400;
  line-height: 1.5715;
  text-align: start;
  text-decoration: none;
  text-shadow: none;
  text-transform: none;
  letter-spacing: normal;
  word-break: normal;
  word-spacing: normal;
  word-wrap: normal;
  white-space: normal;
  line-break: auto;
  font-size: 14px;
  line-height: 1.5715;
  text-align: start;
  list-style: none;
  font-feature-settings: 'tnum';
  color: rgba(0, 0, 0, 0.88);
  box-sizing: border-box;
  margin: 0;
  padding: 0;
  pointer-events: none;
}

.ant-tooltip-content {
  position: relative;
}

.ant-tooltip-arrow {
  position: absolute;
  width: 16px;
  height: 16px;
  overflow: hidden;
  background: transparent;
  pointer-events: none;
}

.ant-tooltip-arrow::before {
  position: absolute;
  top: 0;
  inset-inline-start: 0;
  width: 16px;
  height: 8px;
  background-color: rgba(0, 0, 0, 0.85);
  clip-path: path('M 0 8 A 4 4 0 0 0 2.82842712474619 6.82842712474619 L 6.585786437626905 3.0710678118654755 A 2 2 0 0 1 9.414213562373096 3.0710678118654755 L 13.17157287525381 6.82842712474619 A 4 4 0 0 0 16 8 Z');
  content: '';
}

.ant-tooltip-inner {
  min-width: 32px;
  min-height: 32px;
  padding: 6px 8px;
  color: #fff;
  text-align: start;
  text-decoration: none;
  word-wrap: break-word;
  background-color: rgba(0, 0, 0, 0.85);
  border-radius: 6px;
  box-shadow: 0 6px 16px 0 rgba(0, 0, 0, 0.08), 0 3px 6px -4px rgba(0, 0, 0, 0.12), 0 9px 28px 8px rgba(0, 0, 0, 0.05);
}
"#,
        );

        css
    }

    /// 生成位置样式
    fn generate_placement_style(&self) -> String {
        let mut css = String::new();

        css.push_str(
            r#"
/* 位置样式 */
.ant-tooltip-placement-top {
  padding-bottom: 8px;
}

.ant-tooltip-placement-top .ant-tooltip-content {
  transform-origin: 50% 100%;
}

.ant-tooltip-placement-top .ant-tooltip-arrow {
  bottom: 0;
  left: 50%;
  transform: translateX(-50%) translateY(100%) rotate(180deg);
}

.ant-tooltip-placement-topLeft {
  padding-bottom: 8px;
}

.ant-tooltip-placement-topLeft .ant-tooltip-content {
  transform-origin: 0 100%;
}

.ant-tooltip-placement-topLeft .ant-tooltip-arrow {
  bottom: 0;
  left: 16px;
  transform: translateY(100%) rotate(180deg);
}

.ant-tooltip-placement-topRight {
  padding-bottom: 8px;
}

.ant-tooltip-placement-topRight .ant-tooltip-content {
  transform-origin: 100% 100%;
}

.ant-tooltip-placement-topRight .ant-tooltip-arrow {
  bottom: 0;
  right: 16px;
  transform: translateY(100%) rotate(180deg);
}

.ant-tooltip-placement-bottom {
  padding-top: 8px;
}

.ant-tooltip-placement-bottom .ant-tooltip-content {
  transform-origin: 50% 0;
}

.ant-tooltip-placement-bottom .ant-tooltip-arrow {
  top: 0;
  left: 50%;
  transform: translateX(-50%) translateY(-100%);
}

.ant-tooltip-placement-bottomLeft {
  padding-top: 8px;
}

.ant-tooltip-placement-bottomLeft .ant-tooltip-content {
  transform-origin: 0 0;
}

.ant-tooltip-placement-bottomLeft .ant-tooltip-arrow {
  top: 0;
  left: 16px;
  transform: translateY(-100%);
}

.ant-tooltip-placement-bottomRight {
  padding-top: 8px;
}

.ant-tooltip-placement-bottomRight .ant-tooltip-content {
  transform-origin: 100% 0;
}

.ant-tooltip-placement-bottomRight .ant-tooltip-arrow {
  top: 0;
  right: 16px;
  transform: translateY(-100%);
}

.ant-tooltip-placement-left {
  padding-right: 8px;
}

.ant-tooltip-placement-left .ant-tooltip-content {
  transform-origin: 100% 50%;
}

.ant-tooltip-placement-left .ant-tooltip-arrow {
  right: 0;
  top: 50%;
  transform: translateX(100%) translateY(-50%) rotate(90deg);
}

.ant-tooltip-placement-leftTop {
  padding-right: 8px;
}

.ant-tooltip-placement-leftTop .ant-tooltip-content {
  transform-origin: 100% 0;
}

.ant-tooltip-placement-leftTop .ant-tooltip-arrow {
  right: 0;
  top: 8px;
  transform: translateX(100%) rotate(90deg);
}

.ant-tooltip-placement-leftBottom {
  padding-right: 8px;
}

.ant-tooltip-placement-leftBottom .ant-tooltip-content {
  transform-origin: 100% 100%;
}

.ant-tooltip-placement-leftBottom .ant-tooltip-arrow {
  right: 0;
  bottom: 8px;
  transform: translateX(100%) rotate(90deg);
}

.ant-tooltip-placement-right {
  padding-left: 8px;
}

.ant-tooltip-placement-right .ant-tooltip-content {
  transform-origin: 0 50%;
}

.ant-tooltip-placement-right .ant-tooltip-arrow {
  left: 0;
  top: 50%;
  transform: translateX(-100%) translateY(-50%) rotate(-90deg);
}

.ant-tooltip-placement-rightTop {
  padding-left: 8px;
}

.ant-tooltip-placement-rightTop .ant-tooltip-content {
  transform-origin: 0 0;
}

.ant-tooltip-placement-rightTop .ant-tooltip-arrow {
  left: 0;
  top: 8px;
  transform: translateX(-100%) rotate(-90deg);
}

.ant-tooltip-placement-rightBottom {
  padding-left: 8px;
}

.ant-tooltip-placement-rightBottom .ant-tooltip-content {
  transform-origin: 0 100%;
}

.ant-tooltip-placement-rightBottom .ant-tooltip-arrow {
  left: 0;
  bottom: 8px;
  transform: translateX(-100%) rotate(-90deg);
}
"#,
        );

        css
    }

    /// 生成动画样式
    fn generate_animation_style(&self) -> String {
        let mut css = String::new();

        css.push_str(
            r#"
/* 动画效果 */
@keyframes antTooltipIn {
  0% {
    opacity: 0;
    transform: scale(0.8);
  }
  100% {
    opacity: 1;
    transform: scale(1);
  }
}

@keyframes antTooltipOut {
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

    /// 生成移动端样式
    fn generate_mobile_style(&self) -> String {
        if !self.mobile {
            return String::new();
        }

        let mut css = String::new();

        css.push_str(
            r#"
@media (max-width: 576px) {
  .ant-tooltip {
    max-width: 250px;
  }

  .ant-tooltip-inner {
    padding: 4px 6px;
    min-height: 24px;
    min-width: 24px;
    font-size: 12px;
  }
}
"#,
        );

        css
    }

    /// 生成高对比度样式
    fn generate_high_contrast_style(&self) -> String {
        if !self.high_contrast {
            return String::new();
        }

        let mut css = String::new();

        css.push_str(
            r#"
@media (prefers-contrast: high) {
  .ant-tooltip-inner {
    background-color: #000;
    outline: 2px solid #fff;
    outline-offset: -2px;
  }

  .ant-tooltip-arrow::before {
    background-color: #000;
  }
}
"#,
        );

        css
    }

    /// 生成暗黑模式样式
    fn generate_dark_mode_style(&self) -> String {
        if !self.dark_mode {
            return String::new();
        }

        let mut css = String::new();

        css.push_str(
            r#"
@media (prefers-color-scheme: dark) {
  .ant-tooltip-inner {
    background-color: rgba(255, 255, 255, 0.85);
    color: rgba(0, 0, 0, 0.88);
  }

  .ant-tooltip-arrow::before {
    background-color: rgba(255, 255, 255, 0.85);
  }
}
"#,
        );

        css
    }

    /// 生成自定义颜色样式
    fn generate_custom_color_style(&self) -> String {
        if let TooltipColor::Custom(color) = &self.color {
            format!(
                r#"
.ant-tooltip-inner {{
  background-color: {0};
}}

.ant-tooltip-arrow::before {{
  background-color: {0};
}}
"#,
                color
            )
        } else {
            String::new()
        }
    }

    /// 生成完整样式
    pub fn generate(&self) -> String {
        let mut css = String::new();

        css.push_str(&self.generate_base_style());
        css.push_str(&self.generate_placement_style());
        css.push_str(&self.generate_animation_style());
        css.push_str(&self.generate_mobile_style());
        css.push_str(&self.generate_high_contrast_style());
        css.push_str(&self.generate_dark_mode_style());
        css.push_str(&self.generate_custom_color_style());

        css
    }
}

/// 生成 Tooltip 样式
pub fn generate_tooltip_style(
    placement: TooltipPlacement,
    trigger: TooltipTrigger,
    color: TooltipColor,
    arrow: bool,
    high_contrast: bool,
    dark_mode: bool,
    mobile: bool,
) -> String {
    TooltipStyleGenerator::new()
        .with_placement(placement)
        .with_trigger(trigger)
        .with_color(color)
        .with_arrow(arrow)
        .with_high_contrast(high_contrast)
        .with_dark_mode(dark_mode)
        .with_mobile(mobile)
        .generate()
}

/// 默认 Tooltip 样式
pub fn default_tooltip_style() -> String {
    TooltipStyleGenerator::new().generate()
}

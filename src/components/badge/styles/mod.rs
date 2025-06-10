//! Badge 徽标数组件样式

/// 徽标状态类型
#[derive(Debug, Clone, PartialEq)]
pub enum BadgeStatus {
    /// 成功状态
    Success,
    /// 处理中状态
    Processing,
    /// 默认状态
    Default,
    /// 错误状态
    Error,
    /// 警告状态
    Warning,
}

impl Default for BadgeStatus {
    fn default() -> Self {
        Self::Default
    }
}

/// 徽标尺寸
#[derive(Debug, Clone, PartialEq)]
pub enum BadgeSize {
    /// 默认尺寸
    Default,
    /// 小尺寸
    Small,
}

impl Default for BadgeSize {
    fn default() -> Self {
        Self::Default
    }
}

/// Badge 样式生成器
pub struct BadgeStyleGenerator {
    status: Option<BadgeStatus>,
    size: BadgeSize,
    dot: bool,
    color: Option<String>,
    rtl: bool,
    show_zero: bool,
}

impl BadgeStyleGenerator {
    /// 创建一个新的 Badge 样式生成器
    pub fn new() -> Self {
        Self {
            status: None,
            size: BadgeSize::Default,
            dot: false,
            color: None,
            rtl: false,
            show_zero: false,
        }
    }

    /// 设置徽标状态
    pub fn with_status(mut self, status: Option<BadgeStatus>) -> Self {
        self.status = status;
        self
    }

    /// 设置徽标尺寸
    pub fn with_size(mut self, size: BadgeSize) -> Self {
        self.size = size;
        self
    }

    /// 设置是否为小红点模式
    pub fn with_dot(mut self, dot: bool) -> Self {
        self.dot = dot;
        self
    }

    /// 设置徽标颜色
    pub fn with_color(mut self, color: Option<String>) -> Self {
        self.color = color;
        self
    }

    /// 设置是否为 RTL 模式
    pub fn with_rtl(mut self, rtl: bool) -> Self {
        self.rtl = rtl;
        self
    }

    /// 设置是否显示零值
    pub fn with_show_zero(mut self, show_zero: bool) -> Self {
        self.show_zero = show_zero;
        self
    }

    /// 生成基础样式
    fn generate_base_style(&self) -> String {
        let mut css = String::new();

        css.push_str(
            r#"
/* Badge 徽标数样式 */

.ant-badge {
  position: relative;
  display: inline-block;
  line-height: 1;
}

.ant-badge-not-a-wrapper {
  vertical-align: middle;
}

.ant-badge-count {
  position: absolute;
  top: 0;
  right: 0;
  z-index: 10;
  min-width: 20px;
  height: 20px;
  padding: 0 6px;
  color: #fff;
  font-weight: normal;
  font-size: 12px;
  line-height: 20px;
  white-space: nowrap;
  text-align: center;
  background: #ff4d4f;
  border-radius: 10px;
  box-shadow: 0 0 0 1px #fff;
  transform: translate(50%, -50%);
  transform-origin: 100% 0%;
}

.ant-badge-count-content {
  display: inline-block;
}

.ant-badge-dot {
  position: absolute;
  top: 0;
  right: 0;
  z-index: 10;
  width: 6px;
  height: 6px;
  background: #ff4d4f;
  border-radius: 100%;
  box-shadow: 0 0 0 1px #fff;
  transform: translate(50%, -50%);
  transform-origin: 100% 0%;
}
"#,
        );

        css
    }

    /// 生成尺寸样式
    fn generate_size_style(&self) -> String {
        if self.size == BadgeSize::Small {
            return r#"
.ant-badge-small .ant-badge-count {
  min-width: 14px;
  height: 14px;
  padding: 0 4px;
  font-size: 12px;
  line-height: 14px;
  border-radius: 7px;
}

.ant-badge-small .ant-badge-dot {
  width: 4px;
  height: 4px;
}
"#
            .to_string();
        }
        String::new()
    }

    /// 生成状态样式
    fn generate_status_style(&self) -> String {
        let mut css = String::new();

        css.push_str(
            r#"
/* 状态点样式 */
.ant-badge-status {
  line-height: inherit;
  vertical-align: baseline;
}

.ant-badge-status-dot {
  position: relative;
  top: -1px;
  display: inline-block;
  width: 6px;
  height: 6px;
  vertical-align: middle;
  border-radius: 50%;
}

.ant-badge-status-success {
  background-color: #52c41a;
}

.ant-badge-status-processing {
  position: relative;
  background-color: #1890ff;
}

.ant-badge-status-processing::after {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  border: 1px solid #1890ff;
  border-radius: 50%;
  animation: antStatusProcessing 1.2s infinite ease-in-out;
  content: '';
}

.ant-badge-status-default {
  background-color: #d9d9d9;
}

.ant-badge-status-error {
  background-color: #ff4d4f;
}

.ant-badge-status-warning {
  background-color: #faad14;
}

.ant-badge-status-text {
  margin-left: 8px;
  color: rgba(0, 0, 0, 0.85);
  font-size: 14px;
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
@keyframes antStatusProcessing {
  0% {
    transform: scale(0.8);
    opacity: 0.5;
  }

  100% {
    transform: scale(2.4);
    opacity: 0;
  }
}
"#,
        );

        css
    }

    /// 生成显示零值样式
    fn generate_zero_style(&self) -> String {
        if !self.show_zero {
            return r#"
/* 数字为0时隐藏 */
.ant-badge-count-zero {
  display: none;
}
"#
            .to_string();
        }
        String::new()
    }

    /// 生成多字样式
    fn generate_multiple_words_style(&self) -> String {
        let mut css = String::new();

        css.push_str(
            r#"
/* 多位数字样式调整 */
.ant-badge-multiple-words {
  padding: 0 8px;
}
"#,
        );

        css
    }

    /// 生成 RTL 样式
    fn generate_rtl_style(&self) -> String {
        if self.rtl {
            return r#"
/* RTL 支持 */
.ant-badge-rtl {
  direction: rtl;
}

.ant-badge-rtl .ant-badge-count {
  right: auto;
  left: 0;
  transform: translate(-50%, -50%);
  transform-origin: 0% 0%;
}

.ant-badge-rtl .ant-badge-dot {
  right: auto;
  left: 0;
  transform: translate(-50%, -50%);
  transform-origin: 0% 0%;
}

.ant-badge-rtl .ant-badge-status-text {
  margin-right: 8px;
  margin-left: 0;
}
"#
            .to_string();
        }
        String::new()
    }

    /// 生成悬浮效果样式
    fn generate_hover_style(&self) -> String {
        let mut css = String::new();

        css.push_str(
            r#"
/* 悬浮效果 */
.ant-badge:hover .ant-badge-count {
  transform: translate(50%, -50%) scale(1.1);
  transition: transform 0.2s ease;
}

.ant-badge:hover .ant-badge-dot {
  transform: translate(50%, -50%) scale(1.2);
  transition: transform 0.2s ease;
}
"#,
        );

        css
    }

    /// 生成无障碍样式
    fn generate_accessibility_style(&self) -> String {
        let mut css = String::new();

        css.push_str(
            r#"
/* 无障碍支持 */
.ant-badge-count[aria-hidden="true"] {
  position: absolute;
  clip: rect(0 0 0 0);
  width: 1px;
  height: 1px;
  margin: -1px;
  padding: 0;
  overflow: hidden;
  white-space: nowrap;
  border: 0;
}
"#,
        );

        css
    }

    /// 生成自定义颜色样式
    fn generate_custom_color_style(&self) -> String {
        if let Some(color) = &self.color {
            return format!(
                r#"
.ant-badge-dot, .ant-badge-count {{
  background: {0};
}}
"#,
                color
            );
        }
        String::new()
    }

    /// 生成完整样式
    pub fn generate(&self) -> String {
        let mut css = String::new();

        css.push_str(&self.generate_base_style());
        css.push_str(&self.generate_size_style());
        css.push_str(&self.generate_status_style());
        css.push_str(&self.generate_animation_style());
        css.push_str(&self.generate_zero_style());
        css.push_str(&self.generate_multiple_words_style());
        css.push_str(&self.generate_rtl_style());
        css.push_str(&self.generate_hover_style());
        css.push_str(&self.generate_accessibility_style());
        css.push_str(&self.generate_custom_color_style());

        css
    }
}

/// 生成 Badge 样式
pub fn generate_badge_style(
    status: Option<BadgeStatus>,
    size: BadgeSize,
    dot: bool,
    color: Option<String>,
    rtl: bool,
    show_zero: bool,
) -> String {
    BadgeStyleGenerator::new()
        .with_status(status)
        .with_size(size)
        .with_dot(dot)
        .with_color(color)
        .with_rtl(rtl)
        .with_show_zero(show_zero)
        .generate()
}

/// 默认 Badge 样式
pub fn default_badge_style() -> String {
    BadgeStyleGenerator::new().generate()
}

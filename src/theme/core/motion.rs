//! 动画核心抽象
//!
//! 提供动画系统的基础类型和核心功能，包括：
//! - 动画持续时间定义
//! - 缓动函数定义
//! - 过渡效果类型
//! - 动画配置抽象

use serde::{Deserialize, Serialize};
use std::fmt;

/// 动画持续时间枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum Duration {
    /// 快速动画 (100ms)
    #[default]
    Fast,
    /// 中等动画 (200ms)
    Mid,
    /// 慢速动画 (300ms)
    Slow,
    /// 自定义时长
    Custom(u32),
}

impl Duration {
    /// 获取持续时间的毫秒值
    pub fn to_ms(&self) -> u32 {
        match self {
            Duration::Fast => 100,
            Duration::Mid => 200,
            Duration::Slow => 300,
            Duration::Custom(ms) => *ms,
        }
    }

    /// 获取持续时间的秒值
    pub fn to_seconds(&self) -> f32 {
        self.to_ms() as f32 / 1000.0
    }

    /// 转换为 CSS 时间字符串
    pub fn to_css_string(&self) -> String {
        format!("{}ms", self.to_ms())
    }
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_css_string())
    }
}

/// 缓动函数枚举
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum Easing {
    /// 线性
    #[default]
    Linear,
    /// 缓入
    EaseIn,
    /// 缓出
    EaseOut,
    /// 缓入缓出
    EaseInOut,
    /// 缓入（二次方）
    EaseInQuad,
    /// 缓出（二次方）
    EaseOutQuad,
    /// 缓入缓出（二次方）
    EaseInOutQuad,
    /// 缓入（三次方）
    EaseInCubic,
    /// 缓出（三次方）
    EaseOutCubic,
    /// 缓入缓出（三次方）
    EaseInOutCubic,
    /// 缓入（四次方）
    EaseInQuart,
    /// 缓出（四次方）
    EaseOutQuart,
    /// 缓入缓出（四次方）
    EaseInOutQuart,
    /// 缓入（指数）
    EaseInExpo,
    /// 缓出（指数）
    EaseOutExpo,
    /// 缓入缓出（指数）
    EaseInOutExpo,
    /// 缓入（圆形）
    EaseInCirc,
    /// 缓出（圆形）
    EaseOutCirc,
    /// 缓入缓出（圆形）
    EaseInOutCirc,
    /// 回弹
    EaseInBack,
    /// 回弹缓出
    EaseOutBack,
    /// 回弹缓入缓出
    EaseInOutBack,
    /// 自定义贝塞尔曲线
    CubicBezier(f32, f32, f32, f32),
}

impl Easing {
    /// 转换为 CSS 缓动函数字符串
    pub fn to_css_string(&self) -> String {
        match self {
            Easing::Linear => "linear".to_string(),
            Easing::EaseIn => "ease-in".to_string(),
            Easing::EaseOut => "ease-out".to_string(),
            Easing::EaseInOut => "ease-in-out".to_string(),
            Easing::EaseInQuad => "cubic-bezier(0.55, 0.085, 0.68, 0.53)".to_string(),
            Easing::EaseOutQuad => "cubic-bezier(0.25, 0.46, 0.45, 0.94)".to_string(),
            Easing::EaseInOutQuad => "cubic-bezier(0.455, 0.03, 0.515, 0.955)".to_string(),
            Easing::EaseInCubic => "cubic-bezier(0.55, 0.055, 0.675, 0.19)".to_string(),
            Easing::EaseOutCubic => "cubic-bezier(0.215, 0.61, 0.355, 1)".to_string(),
            Easing::EaseInOutCubic => "cubic-bezier(0.645, 0.045, 0.355, 1)".to_string(),
            Easing::EaseInQuart => "cubic-bezier(0.895, 0.03, 0.685, 0.22)".to_string(),
            Easing::EaseOutQuart => "cubic-bezier(0.165, 0.84, 0.44, 1)".to_string(),
            Easing::EaseInOutQuart => "cubic-bezier(0.77, 0, 0.175, 1)".to_string(),
            Easing::EaseInExpo => "cubic-bezier(0.95, 0.05, 0.795, 0.035)".to_string(),
            Easing::EaseOutExpo => "cubic-bezier(0.19, 1, 0.22, 1)".to_string(),
            Easing::EaseInOutExpo => "cubic-bezier(1, 0, 0, 1)".to_string(),
            Easing::EaseInCirc => "cubic-bezier(0.6, 0.04, 0.98, 0.335)".to_string(),
            Easing::EaseOutCirc => "cubic-bezier(0.075, 0.82, 0.165, 1)".to_string(),
            Easing::EaseInOutCirc => "cubic-bezier(0.785, 0.135, 0.15, 0.86)".to_string(),
            Easing::EaseInBack => "cubic-bezier(0.6, -0.28, 0.735, 0.045)".to_string(),
            Easing::EaseOutBack => "cubic-bezier(0.175, 0.885, 0.32, 1.275)".to_string(),
            Easing::EaseInOutBack => "cubic-bezier(0.68, -0.55, 0.265, 1.55)".to_string(),
            Easing::CubicBezier(x1, y1, x2, y2) => {
                format!("cubic-bezier({}, {}, {}, {})", x1, y1, x2, y2)
            }
        }
    }
}

impl fmt::Display for Easing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_css_string())
    }
}

/// 过渡效果类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TransitionType {
    /// 淡入淡出
    Fade,
    /// 滑动
    Slide,
    /// 缩放
    Scale,
    /// 旋转
    Rotate,
    /// 移动
    Move,
    /// 翻转
    Flip,
    /// 弹跳
    Bounce,
    /// 摇摆
    Swing,
    /// 脉冲
    Pulse,
    /// 闪烁
    Flash,
    /// 抖动
    Shake,
    /// 橡皮筋
    RubberBand,
    /// 果冻
    Jello,
    /// 心跳
    HeartBeat,
}

/// 过渡方向
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    /// 上
    Up,
    /// 下
    Down,
    /// 左
    Left,
    /// 右
    Right,
    /// 上左
    UpLeft,
    /// 上右
    UpRight,
    /// 下左
    DownLeft,
    /// 下右
    DownRight,
}

/// 动画配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnimationConfig {
    /// 动画类型
    pub transition_type: TransitionType,
    /// 持续时间
    pub duration: Duration,
    /// 缓动函数
    pub easing: Easing,
    /// 延迟时间
    pub delay: Duration,
    /// 方向（可选）
    pub direction: Option<Direction>,
    /// 是否无限循环
    pub infinite: bool,
    /// 循环次数（如果不是无限循环）
    pub iteration_count: Option<u32>,
    /// 是否反向播放
    pub reverse: bool,
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            transition_type: TransitionType::Fade,
            duration: Duration::Mid,
            easing: Easing::EaseInOut,
            delay: Duration::Custom(0),
            direction: None,
            infinite: false,
            iteration_count: Some(1),
            reverse: false,
        }
    }
}

impl AnimationConfig {
    /// 创建新的动画配置
    pub fn new(transition_type: TransitionType) -> Self {
        Self {
            transition_type,
            ..Default::default()
        }
    }

    /// 设置持续时间
    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    /// 设置缓动函数
    pub fn easing(mut self, easing: Easing) -> Self {
        self.easing = easing;
        self
    }

    /// 设置延迟时间
    pub fn delay(mut self, delay: Duration) -> Self {
        self.delay = delay;
        self
    }

    /// 设置方向
    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = Some(direction);
        self
    }

    /// 设置为无限循环
    pub fn infinite(mut self) -> Self {
        self.infinite = true;
        self.iteration_count = None;
        self
    }

    /// 设置循环次数
    pub fn repeat(mut self, count: u32) -> Self {
        self.infinite = false;
        self.iteration_count = Some(count);
        self
    }

    /// 设置反向播放
    pub fn reverse(mut self) -> Self {
        self.reverse = true;
        self
    }

    /// 生成 CSS 动画属性
    pub fn to_css_animation(&self, name: &str) -> String {
        let mut parts = vec![
            name.to_string(),
            self.duration.to_css_string(),
            self.easing.to_css_string(),
        ];

        if self.delay.to_ms() > 0 {
            parts.push(self.delay.to_css_string());
        }

        if self.infinite {
            parts.push("infinite".to_string());
        } else if let Some(count) = self.iteration_count {
            if count != 1 {
                parts.push(count.to_string());
            }
        }

        if self.reverse {
            parts.push("reverse".to_string());
        }

        parts.join(" ")
    }

    /// 生成 CSS 过渡属性
    pub fn to_css_transition(&self, property: &str) -> String {
        let mut parts = vec![
            property.to_string(),
            self.duration.to_css_string(),
            self.easing.to_css_string(),
        ];

        if self.delay.to_ms() > 0 {
            parts.push(self.delay.to_css_string());
        }

        parts.join(" ")
    }
}

/// 动画工具函数
pub mod utils {
    use super::*;

    /// 生成关键帧动画的 CSS
    pub fn generate_keyframes(name: &str, keyframes: &[(u8, &str)]) -> String {
        let mut css = format!("@keyframes {} {{\n", name);

        for (percentage, styles) in keyframes {
            css.push_str(&format!("  {}% {{\n", percentage));
            css.push_str(&format!("    {}\n", styles));
            css.push_str("  }\n");
        }

        css.push_str("}\n");
        css
    }

    /// 生成常用的淡入动画关键帧
    pub fn fade_in_keyframes() -> String {
        generate_keyframes("fadeIn", &[(0, "opacity: 0;"), (100, "opacity: 1;")])
    }

    /// 生成常用的滑入动画关键帧
    pub fn slide_in_up_keyframes() -> String {
        generate_keyframes(
            "slideInUp",
            &[
                (0, "transform: translateY(100%); opacity: 0;"),
                (100, "transform: translateY(0); opacity: 1;"),
            ],
        )
    }

    /// 生成常用的缩放动画关键帧
    pub fn scale_in_keyframes() -> String {
        generate_keyframes(
            "scaleIn",
            &[
                (0, "transform: scale(0); opacity: 0;"),
                (100, "transform: scale(1); opacity: 1;"),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::utils::*;
    use super::*;

    #[test]
    fn test_duration() {
        assert_eq!(Duration::Fast.to_ms(), 100);
        assert_eq!(Duration::Mid.to_ms(), 200);
        assert_eq!(Duration::Slow.to_ms(), 300);
        assert_eq!(Duration::Custom(500).to_ms(), 500);

        assert_eq!(Duration::Fast.to_seconds(), 0.1);
        assert_eq!(Duration::Mid.to_css_string(), "200ms");
    }

    #[test]
    fn test_easing() {
        assert_eq!(Easing::Linear.to_css_string(), "linear");
        assert_eq!(Easing::EaseIn.to_css_string(), "ease-in");
        assert_eq!(
            Easing::CubicBezier(0.1, 0.2, 0.3, 0.4).to_css_string(),
            "cubic-bezier(0.1, 0.2, 0.3, 0.4)"
        );
    }

    #[test]
    fn test_animation_config() {
        let config = AnimationConfig::new(TransitionType::Fade)
            .duration(Duration::Fast)
            .easing(Easing::EaseOut)
            .delay(Duration::Custom(100));

        assert_eq!(config.duration, Duration::Fast);
        assert_eq!(config.easing, Easing::EaseOut);
        assert_eq!(config.delay, Duration::Custom(100));
    }

    #[test]
    fn test_css_animation_generation() {
        let config = AnimationConfig::new(TransitionType::Fade)
            .duration(Duration::Mid)
            .easing(Easing::EaseInOut);

        let css = config.to_css_animation("fadeIn");
        assert!(css.contains("fadeIn"));
        assert!(css.contains("200ms"));
        assert!(css.contains("ease-in-out"));
    }

    #[test]
    fn test_keyframes_generation() {
        let keyframes = generate_keyframes("test", &[(0, "opacity: 0;"), (100, "opacity: 1;")]);

        assert!(keyframes.contains("@keyframes test"));
        assert!(keyframes.contains("0%"));
        assert!(keyframes.contains("100%"));
        assert!(keyframes.contains("opacity: 0;"));
        assert!(keyframes.contains("opacity: 1;"));
    }
}

//! Ant Design 动画预设
//!
//! 本模块定义了 Ant Design 设计系统的动画缓动函数和相关配置。
//! 这些动画预设基于 Ant Design 的动效设计原则。

use crate::theme::core::motion::{AnimationConfig, Duration, Easing, TransitionType};
use css_in_rust::animation::easing::EasingFunction;
use serde::{Deserialize, Serialize};

/// Ant Design 标准缓动函数
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AntDesignEasing {
    /// 标准缓动 - 用于大多数动画
    Standard,
    /// 强调缓动 - 用于重要元素
    Emphasized,
    /// 减速缓动 - 用于进入动画
    Decelerated,
    /// 加速缓动 - 用于退出动画
    Accelerated,
    /// 弹性缓动 - 用于反馈动画
    Bounce,
    /// 回弹缓动 - 用于交互反馈
    Spring,
}

impl AntDesignEasing {
    /// 转换为 CSS 贝塞尔曲线
    pub fn to_css(&self) -> String {
        match self {
            AntDesignEasing::Standard => "cubic-bezier(0.34, 0.69, 0.1, 1)".to_string(),
            AntDesignEasing::Emphasized => "cubic-bezier(0.05, 0.7, 0.1, 1)".to_string(),
            AntDesignEasing::Decelerated => "cubic-bezier(0.0, 0.0, 0.2, 1)".to_string(),
            AntDesignEasing::Accelerated => "cubic-bezier(0.4, 0.0, 1, 1)".to_string(),
            AntDesignEasing::Bounce => "cubic-bezier(0.68, -0.55, 0.265, 1.55)".to_string(),
            AntDesignEasing::Spring => "cubic-bezier(0.175, 0.885, 0.32, 1.275)".to_string(),
        }
    }

    /// 获取建议持续时间（毫秒）
    pub fn suggested_duration_ms(&self) -> u64 {
        match self {
            AntDesignEasing::Standard => 200,
            AntDesignEasing::Emphasized => 300,
            AntDesignEasing::Decelerated => 150,
            AntDesignEasing::Accelerated => 100,
            AntDesignEasing::Bounce => 400,
            AntDesignEasing::Spring => 350,
        }
    }

    /// 获取适用场景描述
    pub fn description(&self) -> &'static str {
        match self {
            AntDesignEasing::Standard => "标准缓动，适用于大多数动画场景",
            AntDesignEasing::Emphasized => "强调缓动，适用于重要元素的动画",
            AntDesignEasing::Decelerated => "减速缓动，适用于元素进入动画",
            AntDesignEasing::Accelerated => "加速缓动，适用于元素退出动画",
            AntDesignEasing::Bounce => "弹性缓动，适用于反馈和确认动画",
            AntDesignEasing::Spring => "回弹缓动，适用于交互反馈动画",
        }
    }

    /// 转换为 css-in-rust 的 EasingFunction
    pub fn to_easing_function(&self) -> EasingFunction {
        match self {
            AntDesignEasing::Standard => EasingFunction::CubicBezier(0.34, 0.69, 0.1, 1.0),
            AntDesignEasing::Emphasized => EasingFunction::CubicBezier(0.05, 0.7, 0.1, 1.0),
            AntDesignEasing::Decelerated => EasingFunction::CubicBezier(0.0, 0.0, 0.2, 1.0),
            AntDesignEasing::Accelerated => EasingFunction::CubicBezier(0.4, 0.0, 1.0, 1.0),
            AntDesignEasing::Bounce => EasingFunction::CubicBezier(0.68, -0.55, 0.265, 1.55),
            AntDesignEasing::Spring => EasingFunction::CubicBezier(0.175, 0.885, 0.32, 1.275),
        }
    }

    /// 转换为通用的 Easing 枚举
    pub fn to_easing(&self) -> Easing {
        match self {
            AntDesignEasing::Standard => Easing::CubicBezier(0.34, 0.69, 0.1, 1.0),
            AntDesignEasing::Emphasized => Easing::CubicBezier(0.05, 0.7, 0.1, 1.0),
            AntDesignEasing::Decelerated => Easing::CubicBezier(0.0, 0.0, 0.2, 1.0),
            AntDesignEasing::Accelerated => Easing::CubicBezier(0.4, 0.0, 1.0, 1.0),
            AntDesignEasing::Bounce => Easing::CubicBezier(0.68, -0.55, 0.265, 1.55),
            AntDesignEasing::Spring => Easing::CubicBezier(0.175, 0.885, 0.32, 1.275),
        }
    }

    /// 转换为通用的 Duration
    pub fn to_duration(&self) -> Duration {
        Duration::Custom(self.suggested_duration_ms() as u32)
    }

    /// 创建标准缓动
    pub fn standard() -> Self {
        AntDesignEasing::Standard
    }

    /// 创建强调缓动
    pub fn emphasized() -> Self {
        AntDesignEasing::Emphasized
    }

    /// 创建进入动画缓动
    pub fn enter() -> Self {
        AntDesignEasing::Decelerated
    }

    /// 创建退出动画缓动
    pub fn exit() -> Self {
        AntDesignEasing::Accelerated
    }

    /// 创建弹性缓动
    pub fn bounce() -> Self {
        AntDesignEasing::Bounce
    }

    /// 创建回弹缓动
    pub fn spring() -> Self {
        AntDesignEasing::Spring
    }
}

/// Ant Design 动画配置构建器
/// 提供便捷的方法创建基于 Ant Design 设计规范的动画配置
pub struct AntDesignAnimationBuilder;

impl AntDesignAnimationBuilder {
    /// 基于 Ant Design 缓动创建动画配置
    pub fn from_easing(easing: AntDesignEasing) -> AnimationConfig {
        AnimationConfig::new(TransitionType::Fade)
            .easing(easing.to_easing())
            .duration(easing.to_duration())
    }

    /// 创建带有指定过渡类型的动画配置
    pub fn with_transition(
        easing: AntDesignEasing,
        transition_type: TransitionType,
    ) -> AnimationConfig {
        AnimationConfig::new(transition_type)
            .easing(easing.to_easing())
            .duration(easing.to_duration())
    }

    /// 创建自定义持续时间的动画配置
    pub fn with_custom_duration(easing: AntDesignEasing, duration_ms: u32) -> AnimationConfig {
        AnimationConfig::new(TransitionType::Fade)
            .easing(easing.to_easing())
            .duration(Duration::Custom(duration_ms))
    }
}

/// Ant Design 动画预设工厂
/// 提供符合 Ant Design 设计规范的预设动画配置
pub struct AntDesignAnimationFactory;

impl AntDesignAnimationFactory {
    /// 创建标准动画配置
    pub fn standard() -> AnimationConfig {
        AntDesignAnimationBuilder::from_easing(AntDesignEasing::Standard)
    }

    /// 创建强调动画配置
    pub fn emphasized() -> AnimationConfig {
        AntDesignAnimationBuilder::from_easing(AntDesignEasing::Emphasized)
    }

    /// 创建进入动画配置
    pub fn enter() -> AnimationConfig {
        AntDesignAnimationBuilder::from_easing(AntDesignEasing::Decelerated)
    }

    /// 创建退出动画配置
    pub fn exit() -> AnimationConfig {
        AntDesignAnimationBuilder::from_easing(AntDesignEasing::Accelerated)
    }

    /// 创建反馈动画配置
    pub fn feedback() -> AnimationConfig {
        AntDesignAnimationBuilder::from_easing(AntDesignEasing::Bounce)
    }

    /// 创建交互动画配置
    pub fn interaction() -> AnimationConfig {
        AntDesignAnimationBuilder::from_easing(AntDesignEasing::Spring)
    }
}

/// 常用动画预设
/// 提供开箱即用的动画配置，符合 Ant Design 设计规范
pub mod animation_presets {
    use super::*;

    /// 淡入动画
    pub fn fade_in() -> AnimationConfig {
        AntDesignAnimationBuilder::with_custom_duration(AntDesignEasing::Decelerated, 150)
    }

    /// 淡出动画
    pub fn fade_out() -> AnimationConfig {
        AntDesignAnimationBuilder::with_custom_duration(AntDesignEasing::Accelerated, 100)
    }

    /// 滑入动画
    pub fn slide_in() -> AnimationConfig {
        AntDesignAnimationBuilder::with_transition(AntDesignEasing::Standard, TransitionType::Slide)
            .duration(Duration::Custom(200))
    }

    /// 滑出动画
    pub fn slide_out() -> AnimationConfig {
        AntDesignAnimationBuilder::with_transition(AntDesignEasing::Standard, TransitionType::Slide)
            .duration(Duration::Custom(200))
    }

    /// 缩放进入动画
    pub fn scale_in() -> AnimationConfig {
        AntDesignAnimationBuilder::with_transition(
            AntDesignEasing::Decelerated,
            TransitionType::Scale,
        )
        .duration(Duration::Custom(150))
    }

    /// 缩放退出动画
    pub fn scale_out() -> AnimationConfig {
        AntDesignAnimationBuilder::with_transition(
            AntDesignEasing::Accelerated,
            TransitionType::Scale,
        )
        .duration(Duration::Custom(100))
    }

    /// 弹跳反馈动画
    pub fn bounce_feedback() -> AnimationConfig {
        AntDesignAnimationBuilder::with_transition(AntDesignEasing::Bounce, TransitionType::Bounce)
            .duration(Duration::Custom(400))
    }
}

//! Progress 组件单元测试

#[cfg(test)]
mod tests {
    use super::*;
    use dioxus::prelude::*;
    use dioxus_testing::*;

    /// 测试 ProgressType 枚举
    #[test]
    fn test_progress_type() {
        assert_eq!(ProgressType::default(), ProgressType::Line);
        assert_eq!(ProgressType::Line, ProgressType::Line);
        assert_ne!(ProgressType::Line, ProgressType::Circle);
        assert_ne!(ProgressType::Circle, ProgressType::Dashboard);
    }

    /// 测试 ProgressSize 枚举
    #[test]
    fn test_progress_size() {
        assert_eq!(ProgressSize::default(), ProgressSize::Default);
        assert_eq!(ProgressSize::Default, ProgressSize::Default);
        assert_ne!(ProgressSize::Default, ProgressSize::Small);
    }

    /// 测试 ProgressStatus 枚举
    #[test]
    fn test_progress_status() {
        assert_eq!(ProgressStatus::default(), ProgressStatus::Normal);
        
        // 测试 to_class 方法
        assert_eq!(ProgressStatus::Normal.to_class(), "");
        assert_eq!(ProgressStatus::Exception.to_class(), "ant-progress-status-exception");
        assert_eq!(ProgressStatus::Success.to_class(), "ant-progress-status-success");
        assert_eq!(ProgressStatus::Active.to_class(), "ant-progress-status-active");
        
        // 测试 to_color 方法
        assert_eq!(ProgressStatus::Normal.to_color(), "#1677ff");
        assert_eq!(ProgressStatus::Exception.to_color(), "#ff4d4f");
        assert_eq!(ProgressStatus::Success.to_color(), "#52c41a");
        assert_eq!(ProgressStatus::Active.to_color(), "#1677ff");
    }

    /// 测试 StrokeLinecap 枚举
    #[test]
    fn test_stroke_linecap() {
        assert_eq!(StrokeLinecap::default(), StrokeLinecap::Round);
        
        assert_eq!(StrokeLinecap::Round.to_str(), "round");
        assert_eq!(StrokeLinecap::Square.to_str(), "square");
        assert_eq!(StrokeLinecap::Butt.to_str(), "butt");
    }

    /// 测试 PercentPosition 枚举
    #[test]
    fn test_percent_position() {
        assert_eq!(PercentPosition::default(), PercentPosition::Top);
        assert_eq!(PercentPosition::Top, PercentPosition::Top);
        assert_ne!(PercentPosition::Top, PercentPosition::Bottom);
    }

    /// 测试 GradientConfig 结构体
    #[test]
    fn test_gradient_config() {
        let gradient = GradientConfig::new("#ff0000", "#00ff00");
        assert_eq!(gradient.from, "#ff0000");
        assert_eq!(gradient.to, "#00ff00");
        assert_eq!(gradient.direction, None);
        
        let gradient_with_direction = gradient.with_direction(45.0);
        assert_eq!(gradient_with_direction.direction, Some(45.0));
        
        let css = gradient_with_direction.to_css();
        assert_eq!(css, "linear-gradient(45deg, #ff0000, #00ff00)");
        
        // 测试默认方向
        let default_gradient = GradientConfig::new("red", "blue");
        let default_css = default_gradient.to_css();
        assert_eq!(default_css, "linear-gradient(90deg, red, blue)");
    }

    /// 测试 SuccessConfig 结构体
    #[test]
    fn test_success_config() {
        let success = SuccessConfig::new(80);
        assert_eq!(success.percent, 80);
        assert_eq!(success.stroke_color, None);
        
        let success_with_color = success.with_color("#52c41a");
        assert_eq!(success_with_color.stroke_color, Some("#52c41a".to_string()));
    }

    /// 测试 StepConfig 结构体
    #[test]
    fn test_step_config() {
        let step = StepConfig::new("#ff0000", 25);
        assert_eq!(step.color, "#ff0000");
        assert_eq!(step.percent, 25);
    }

    /// 测试样式函数
    #[test]
    fn test_style_functions() {
        // 测试渐变色样式
        let gradient_style = get_gradient_style("#ff0000", "#00ff00", Some(45.0));
        assert_eq!(gradient_style, "linear-gradient(45deg, #ff0000, #00ff00)");
        
        let default_gradient_style = get_gradient_style("red", "blue", None);
        assert_eq!(default_gradient_style, "linear-gradient(90deg, red, blue)");
        
        // 测试成功进度段样式
        let success_style = get_success_segment_style(80, "#52c41a", 8);
        assert!(success_style.contains("width: 80%"));
        assert!(success_style.contains("height: 8px"));
        assert!(success_style.contains("background-color: #52c41a"));
        
        // 测试多色彩分段样式
        let segment_style = get_color_segment_style(20, 60, "#ff0000", 8);
        assert!(segment_style.contains("width: 40%"));
        assert!(segment_style.contains("left: 20%"));
        assert!(segment_style.contains("background-color: #ff0000"));
        
        // 测试响应式样式类名
        assert_eq!(get_responsive_class(true), "ant-progress-responsive");
        assert_eq!(get_responsive_class(false), "");
        
        // 测试动画样式类名
        assert_eq!(get_animation_class(true), "ant-progress-no-animation");
        assert_eq!(get_animation_class(false), "");
        
        // 测试主题色样式类名
        assert_eq!(get_theme_class(Some("#ff0000")), "ant-progress-custom-theme");
        assert_eq!(get_theme_class(None), "");
        
        // 测试主题色CSS变量
        let theme_vars = get_theme_css_vars(Some("#ff0000"));
        assert_eq!(theme_vars, "--progress-theme-color: #ff0000;");
        
        let empty_theme_vars = get_theme_css_vars(None);
        assert_eq!(empty_theme_vars, "");
    }

    /// 测试样式生成
    #[test]
    fn test_progress_styles() {
        let styles = get_progress_styles();
        
        // 检查基础样式是否包含
        assert!(styles.contains(".ant-progress"));
        assert!(styles.contains(".ant-progress-line"));
        assert!(styles.contains(".ant-progress-circle"));
        assert!(styles.contains(".ant-progress-dashboard"));
        
        // 检查状态样式是否包含
        assert!(styles.contains(".ant-progress-status-exception"));
        assert!(styles.contains(".ant-progress-status-success"));
        assert!(styles.contains(".ant-progress-status-active"));
        
        // 检查动画是否包含
        assert!(styles.contains("@keyframes ant-progress-appear"));
        assert!(styles.contains("@keyframes ant-progress-active"));
        
        // 检查响应式样式是否包含
        assert!(styles.contains("@media (max-width: 575px)"));
    }

    /// 测试边界值处理
    #[test]
    fn test_boundary_values() {
        // 测试百分比边界值
        let success_style_over = get_success_segment_style(150, "#52c41a", 8);
        assert!(success_style_over.contains("width: 100%"));
        
        let success_style_under = get_success_segment_style(-10, "#52c41a", 8);
        assert!(success_style_under.contains("width: 0%"));
        
        // 测试分段样式边界值
        let segment_style_over = get_color_segment_style(150, 200, "#ff0000", 8);
        assert!(segment_style_over.contains("width: 50%"));
        assert!(segment_style_over.contains("left: 100%"));
        
        let segment_style_under = get_color_segment_style(-10, 20, "#ff0000", 8);
        assert!(segment_style_under.contains("width: 30%"));
        assert!(segment_style_under.contains("left: 0%"));
    }

    /// 测试组件属性默认值
    #[test]
    fn test_progress_props_defaults() {
        // 由于 ProgressProps 使用了 Props derive macro，
        // 我们无法直接测试默认值，但可以测试相关类型的默认值
        assert_eq!(ProgressType::default(), ProgressType::Line);
        assert_eq!(ProgressStatus::default(), ProgressStatus::Normal);
        assert_eq!(ProgressSize::default(), ProgressSize::Default);
        assert_eq!(StrokeLinecap::default(), StrokeLinecap::Round);
        assert_eq!(PercentPosition::default(), PercentPosition::Top);
    }

    /// 测试克隆和相等性
    #[test]
    fn test_clone_and_equality() {
        let gradient1 = GradientConfig::new("red", "blue").with_direction(45.0);
        let gradient2 = gradient1.clone();
        assert_eq!(gradient1, gradient2);
        
        let success1 = SuccessConfig::new(80).with_color("green");
        let success2 = success1.clone();
        assert_eq!(success1, success2);
        
        let step1 = StepConfig::new("red", 25);
        let step2 = step1.clone();
        assert_eq!(step1, step2);
    }

    /// 测试调试输出
    #[test]
    fn test_debug_output() {
        let progress_type = ProgressType::Line;
        let debug_str = format!("{:?}", progress_type);
        assert!(debug_str.contains("Line"));
        
        let progress_status = ProgressStatus::Success;
        let debug_str = format!("{:?}", progress_status);
        assert!(debug_str.contains("Success"));
        
        let gradient = GradientConfig::new("red", "blue");
        let debug_str = format!("{:?}", gradient);
        assert!(debug_str.contains("red"));
        assert!(debug_str.contains("blue"));
    }

    /// 测试字符串转换
    #[test]
    fn test_string_conversions() {
        let gradient = GradientConfig::new("#ff0000".to_string(), "#00ff00".to_string());
        assert_eq!(gradient.from, "#ff0000");
        assert_eq!(gradient.to, "#00ff00");
        
        let success = SuccessConfig::new(80).with_color("#52c41a".to_string());
        assert_eq!(success.stroke_color, Some("#52c41a".to_string()));
        
        let step = StepConfig::new("#ff0000".to_string(), 25);
        assert_eq!(step.color, "#ff0000");
    }
}
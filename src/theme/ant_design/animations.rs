//! Ant Design 动画规范
//!
//! 实现 Ant Design 的动画系统，包括：
//! - 动画预设
//! - 过渡效果
//! - 缓动函数
//! - 动画时长

use crate::theme::core::motion::{AnimationConfig, Direction, Duration, Easing, TransitionType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 动画场景枚举
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AnimationScene {
    /// 进入动画
    Enter,
    /// 退出动画
    Exit,
    /// 悬停动画
    Hover,
    /// 激活动画
    Active,
    /// 焦点动画
    Focus,
    /// 加载动画
    Loading,
    /// 错误动画
    Error,
    /// 成功动画
    Success,
    /// 警告动画
    Warning,
    /// 信息动画
    Info,
}

/// 组件动画类型
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ComponentAnimation {
    /// 模态框
    Modal,
    /// 抽屉
    Drawer,
    /// 下拉菜单
    Dropdown,
    /// 工具提示
    Tooltip,
    /// 气泡确认框
    Popover,
    /// 通知
    Notification,
    /// 消息
    Message,
    /// 标签页
    Tabs,
    /// 折叠面板
    Collapse,
    /// 手风琴
    Accordion,
    /// 轮播图
    Carousel,
    /// 进度条
    Progress,
    /// 加载中
    Spin,
    /// 按钮
    Button,
    /// 输入框
    Input,
    /// 表格
    Table,
    /// 列表
    List,
    /// 卡片
    Card,
    /// 标签
    Tag,
    /// 徽章
    Badge,
    /// 头像
    Avatar,
    /// 开关
    Switch,
    /// 滑块
    Slider,
    /// 步骤条
    Steps,
    /// 时间轴
    Timeline,
    /// 树形控件
    Tree,
    /// 菜单
    Menu,
    /// 面包屑
    Breadcrumb,
    /// 分页
    Pagination,
    /// 锚点
    Anchor,
    /// 回到顶部
    BackTop,
}

/// Ant Design 动画规范
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AntDesignAnimations {
    /// 动画预设配置
    pub presets: HashMap<String, AnimationConfig>,
    /// 组件动画配置
    pub component_animations: HashMap<ComponentAnimation, HashMap<AnimationScene, AnimationConfig>>,
    /// 全局动画开关
    pub motion_enabled: bool,
    /// 减少动画（无障碍）
    pub reduce_motion: bool,
    /// 默认动画时长
    pub default_duration: Duration,
    /// 默认缓动函数
    pub default_easing: Easing,
}

impl AntDesignAnimations {
    /// 获取动画持续时间
    pub fn get_duration(&self, duration: &Duration) -> Option<u32> {
        Some(duration.to_ms())
    }

    /// 获取缓动函数
    pub fn get_easing(&self, easing: &Easing) -> Option<String> {
        Some(easing.to_css_string())
    }

    /// 创建默认动画规范
    pub fn default() -> Self {
        let mut presets = HashMap::new();

        // 基础动画预设
        presets.insert(
            "fade-in".to_string(),
            AnimationConfig::new(TransitionType::Fade)
                .duration(Duration::Mid)
                .easing(Easing::EaseOut),
        );
        presets.insert(
            "fade-out".to_string(),
            AnimationConfig::new(TransitionType::Fade)
                .duration(Duration::Mid)
                .easing(Easing::EaseIn),
        );
        presets.insert(
            "slide-up".to_string(),
            AnimationConfig::new(TransitionType::Slide)
                .duration(Duration::Mid)
                .easing(Easing::EaseOut)
                .direction(Direction::Up),
        );
        presets.insert(
            "slide-down".to_string(),
            AnimationConfig::new(TransitionType::Slide)
                .duration(Duration::Mid)
                .easing(Easing::EaseOut)
                .direction(Direction::Down),
        );
        presets.insert(
            "slide-left".to_string(),
            AnimationConfig::new(TransitionType::Slide)
                .duration(Duration::Mid)
                .easing(Easing::EaseOut)
                .direction(Direction::Left),
        );
        presets.insert(
            "slide-right".to_string(),
            AnimationConfig::new(TransitionType::Slide)
                .duration(Duration::Mid)
                .easing(Easing::EaseOut)
                .direction(Direction::Right),
        );
        presets.insert(
            "scale-in".to_string(),
            AnimationConfig::new(TransitionType::Scale)
                .duration(Duration::Mid)
                .easing(Easing::EaseOut),
        );
        presets.insert(
            "scale-out".to_string(),
            AnimationConfig::new(TransitionType::Scale)
                .duration(Duration::Mid)
                .easing(Easing::EaseIn),
        );
        presets.insert(
            "zoom-in".to_string(),
            AnimationConfig::new(TransitionType::Scale)
                .duration(Duration::Fast)
                .easing(Easing::EaseOut),
        );
        presets.insert(
            "zoom-out".to_string(),
            AnimationConfig::new(TransitionType::Scale)
                .duration(Duration::Fast)
                .easing(Easing::EaseIn),
        );
        presets.insert(
            "bounce-in".to_string(),
            AnimationConfig::new(TransitionType::Bounce)
                .duration(Duration::Slow)
                .easing(Easing::EaseOut),
        );
        presets.insert(
            "pulse".to_string(),
            AnimationConfig::new(TransitionType::Pulse)
                .duration(Duration::Slow)
                .easing(Easing::EaseInOut)
                .infinite(),
        );
        presets.insert(
            "shake".to_string(),
            AnimationConfig::new(TransitionType::Shake)
                .duration(Duration::Fast)
                .easing(Easing::EaseInOut)
                .repeat(3),
        );
        presets.insert(
            "heartbeat".to_string(),
            AnimationConfig::new(TransitionType::HeartBeat)
                .duration(Duration::Slow)
                .easing(Easing::EaseInOut)
                .infinite(),
        );

        // 快速动画预设
        presets.insert(
            "fade-in-fast".to_string(),
            AnimationConfig::new(TransitionType::Fade)
                .duration(Duration::Fast)
                .easing(Easing::EaseOut),
        );
        presets.insert(
            "fade-out-fast".to_string(),
            AnimationConfig::new(TransitionType::Fade)
                .duration(Duration::Fast)
                .easing(Easing::EaseIn),
        );
        presets.insert(
            "slide-up-fast".to_string(),
            AnimationConfig::new(TransitionType::Slide)
                .duration(Duration::Fast)
                .easing(Easing::EaseOut)
                .direction(Direction::Up),
        );
        presets.insert(
            "scale-in-fast".to_string(),
            AnimationConfig::new(TransitionType::Scale)
                .duration(Duration::Fast)
                .easing(Easing::EaseOut),
        );

        // 慢速动画预设
        presets.insert(
            "fade-in-slow".to_string(),
            AnimationConfig::new(TransitionType::Fade)
                .duration(Duration::Slow)
                .easing(Easing::EaseOut),
        );
        presets.insert(
            "slide-up-slow".to_string(),
            AnimationConfig::new(TransitionType::Slide)
                .duration(Duration::Slow)
                .easing(Easing::EaseOut)
                .direction(Direction::Up),
        );

        let mut component_animations = HashMap::new();

        // 模态框动画
        let mut modal_animations = HashMap::new();
        modal_animations.insert(
            AnimationScene::Enter,
            AnimationConfig::new(TransitionType::Fade)
                .duration(Duration::Mid)
                .easing(Easing::EaseOut),
        );
        modal_animations.insert(
            AnimationScene::Exit,
            AnimationConfig::new(TransitionType::Fade)
                .duration(Duration::Mid)
                .easing(Easing::EaseIn),
        );
        component_animations.insert(ComponentAnimation::Modal, modal_animations);

        // 抽屉动画
        let mut drawer_animations = HashMap::new();
        drawer_animations.insert(
            AnimationScene::Enter,
            AnimationConfig::new(TransitionType::Slide)
                .duration(Duration::Mid)
                .easing(Easing::EaseOut)
                .direction(Direction::Right),
        );
        drawer_animations.insert(
            AnimationScene::Exit,
            AnimationConfig::new(TransitionType::Slide)
                .duration(Duration::Mid)
                .easing(Easing::EaseIn)
                .direction(Direction::Right),
        );
        component_animations.insert(ComponentAnimation::Drawer, drawer_animations);

        // 下拉菜单动画
        let mut dropdown_animations = HashMap::new();
        dropdown_animations.insert(
            AnimationScene::Enter,
            AnimationConfig::new(TransitionType::Slide)
                .duration(Duration::Fast)
                .easing(Easing::EaseOut)
                .direction(Direction::Up),
        );
        dropdown_animations.insert(
            AnimationScene::Exit,
            AnimationConfig::new(TransitionType::Slide)
                .duration(Duration::Fast)
                .easing(Easing::EaseIn)
                .direction(Direction::Up),
        );
        component_animations.insert(ComponentAnimation::Dropdown, dropdown_animations);

        // 工具提示动画
        let mut tooltip_animations = HashMap::new();
        tooltip_animations.insert(
            AnimationScene::Enter,
            AnimationConfig::new(TransitionType::Scale)
                .duration(Duration::Fast)
                .easing(Easing::EaseOut),
        );
        tooltip_animations.insert(
            AnimationScene::Exit,
            AnimationConfig::new(TransitionType::Scale)
                .duration(Duration::Fast)
                .easing(Easing::EaseIn),
        );
        component_animations.insert(ComponentAnimation::Tooltip, tooltip_animations);

        // 通知动画
        let mut notification_animations = HashMap::new();
        notification_animations.insert(
            AnimationScene::Enter,
            AnimationConfig::new(TransitionType::Slide)
                .duration(Duration::Mid)
                .easing(Easing::EaseOut)
                .direction(Direction::Right),
        );
        notification_animations.insert(
            AnimationScene::Exit,
            AnimationConfig::new(TransitionType::Slide)
                .duration(Duration::Mid)
                .easing(Easing::EaseIn)
                .direction(Direction::Right),
        );
        component_animations.insert(ComponentAnimation::Notification, notification_animations);

        // 消息动画
        let mut message_animations = HashMap::new();
        message_animations.insert(
            AnimationScene::Enter,
            AnimationConfig::new(TransitionType::Slide)
                .duration(Duration::Fast)
                .easing(Easing::EaseOut)
                .direction(Direction::Down),
        );
        message_animations.insert(
            AnimationScene::Exit,
            AnimationConfig::new(TransitionType::Slide)
                .duration(Duration::Fast)
                .easing(Easing::EaseIn)
                .direction(Direction::Up),
        );
        component_animations.insert(ComponentAnimation::Message, message_animations);

        // 按钮动画
        let mut button_animations = HashMap::new();
        button_animations.insert(
            AnimationScene::Hover,
            AnimationConfig::new(TransitionType::Scale)
                .duration(Duration::Fast)
                .easing(Easing::EaseOut),
        );
        button_animations.insert(
            AnimationScene::Active,
            AnimationConfig::new(TransitionType::Scale)
                .duration(Duration::Fast)
                .easing(Easing::EaseIn),
        );
        component_animations.insert(ComponentAnimation::Button, button_animations);

        // 加载动画
        let mut spin_animations = HashMap::new();
        spin_animations.insert(
            AnimationScene::Loading,
            AnimationConfig::new(TransitionType::Rotate)
                .duration(Duration::Slow)
                .easing(Easing::Linear)
                .infinite(),
        );
        component_animations.insert(ComponentAnimation::Spin, spin_animations);

        Self {
            presets,
            component_animations,
            motion_enabled: true,
            reduce_motion: false,
            default_duration: Duration::Mid,
            default_easing: Easing::EaseOut,
        }
    }

    /// 创建无动画规范（无障碍模式）
    pub fn no_motion() -> Self {
        let mut animations = Self::default();
        animations.motion_enabled = false;
        animations.reduce_motion = true;
        animations.default_duration = Duration::Fast;

        // 将所有动画时长设为最短
        for config in animations.presets.values_mut() {
            config.duration = Duration::Fast;
        }

        for component_map in animations.component_animations.values_mut() {
            for config in component_map.values_mut() {
                config.duration = Duration::Fast;
            }
        }

        animations
    }

    /// 获取动画预设
    pub fn get_preset(&self, name: &str) -> Option<&AnimationConfig> {
        if !self.motion_enabled {
            return None;
        }
        self.presets.get(name)
    }

    /// 获取组件动画配置
    pub fn get_component_animation(
        &self,
        component: ComponentAnimation,
        scene: AnimationScene,
    ) -> Option<&AnimationConfig> {
        if !self.motion_enabled {
            return None;
        }

        self.component_animations
            .get(&component)
            .and_then(|scenes| scenes.get(&scene))
    }

    /// 添加自定义动画预设
    pub fn add_preset(&mut self, name: String, config: AnimationConfig) {
        self.presets.insert(name, config);
    }

    /// 添加组件动画配置
    pub fn add_component_animation(
        &mut self,
        component: ComponentAnimation,
        scene: AnimationScene,
        config: AnimationConfig,
    ) {
        self.component_animations
            .entry(component)
            .or_insert_with(HashMap::new)
            .insert(scene, config);
    }

    /// 设置动画开关
    pub fn set_motion_enabled(&mut self, enabled: bool) {
        self.motion_enabled = enabled;
    }

    /// 设置减少动画模式
    pub fn set_reduce_motion(&mut self, reduce: bool) {
        self.reduce_motion = reduce;
        if reduce {
            self.default_duration = Duration::Fast;
        }
    }

    /// 生成 CSS 动画类
    pub fn generate_css_classes(&self) -> String {
        let mut css = String::new();

        // 生成预设动画类
        for (name, config) in &self.presets {
            let class_name = format!(".ant-motion-{}", name.replace('_', "-"));
            css.push_str(&format!(
                "{} {{\n  {}\n}}\n\n",
                class_name,
                config.to_css_animation(name)
            ));
        }

        // 生成组件动画类
        for (component, scenes) in &self.component_animations {
            let component_name = format!("{:?}", component).to_lowercase();

            for (scene, config) in scenes {
                let scene_name = format!("{:?}", scene).to_lowercase();
                let class_name = format!(".ant-{}-{}", component_name, scene_name);

                css.push_str(&format!(
                    "{} {{\n  {}\n}}\n\n",
                    class_name,
                    config.to_css_animation(&scene_name.clone())
                ));
            }
        }

        // 添加减少动画的媒体查询
        css.push_str("@media (prefers-reduced-motion: reduce) {\n");
        css.push_str("  * {\n");
        css.push_str("    animation-duration: 0.01ms !important;\n");
        css.push_str("    animation-iteration-count: 1 !important;\n");
        css.push_str("    transition-duration: 0.01ms !important;\n");
        css.push_str("  }\n");
        css.push_str("}\n\n");

        // 添加动画禁用类
        css.push_str(".ant-motion-disabled * {\n");
        css.push_str("  animation: none !important;\n");
        css.push_str("  transition: none !important;\n");
        css.push_str("}\n");

        css
    }

    /// 生成 CSS 变量
    pub fn to_css_variables(&self) -> HashMap<String, String> {
        let mut variables = HashMap::new();

        // 默认动画配置变量
        variables.insert(
            "--ant-motion-duration-fast".to_string(),
            Duration::Fast.to_css_string(),
        );
        variables.insert(
            "--ant-motion-duration-mid".to_string(),
            Duration::Mid.to_css_string(),
        );
        variables.insert(
            "--ant-motion-duration-slow".to_string(),
            Duration::Slow.to_css_string(),
        );

        // 缓动函数变量
        variables.insert(
            "--ant-motion-ease-linear".to_string(),
            Easing::Linear.to_css_string(),
        );
        variables.insert(
            "--ant-motion-ease-in".to_string(),
            Easing::EaseIn.to_css_string(),
        );
        variables.insert(
            "--ant-motion-ease-out".to_string(),
            Easing::EaseOut.to_css_string(),
        );
        variables.insert(
            "--ant-motion-ease-in-out".to_string(),
            Easing::EaseInOut.to_css_string(),
        );

        // 动画开关变量
        variables.insert(
            "--ant-motion-enabled".to_string(),
            if self.motion_enabled { "1" } else { "0" }.to_string(),
        );
        variables.insert(
            "--ant-motion-reduce".to_string(),
            if self.reduce_motion { "1" } else { "0" }.to_string(),
        );

        variables
    }

    /// 生成关键帧动画
    pub fn generate_keyframes(&self) -> String {
        let mut keyframes = String::new();

        // 淡入动画
        keyframes.push_str("@keyframes ant-fade-in {\n");
        keyframes.push_str("  from { opacity: 0; }\n");
        keyframes.push_str("  to { opacity: 1; }\n");
        keyframes.push_str("}\n\n");

        // 淡出动画
        keyframes.push_str("@keyframes ant-fade-out {\n");
        keyframes.push_str("  from { opacity: 1; }\n");
        keyframes.push_str("  to { opacity: 0; }\n");
        keyframes.push_str("}\n\n");

        // 向上滑入
        keyframes.push_str("@keyframes ant-slide-up-in {\n");
        keyframes.push_str("  from {\n");
        keyframes.push_str("    opacity: 0;\n");
        keyframes.push_str("    transform: translateY(100%);\n");
        keyframes.push_str("  }\n");
        keyframes.push_str("  to {\n");
        keyframes.push_str("    opacity: 1;\n");
        keyframes.push_str("    transform: translateY(0);\n");
        keyframes.push_str("  }\n");
        keyframes.push_str("}\n\n");

        // 向下滑入
        keyframes.push_str("@keyframes ant-slide-down-in {\n");
        keyframes.push_str("  from {\n");
        keyframes.push_str("    opacity: 0;\n");
        keyframes.push_str("    transform: translateY(-100%);\n");
        keyframes.push_str("  }\n");
        keyframes.push_str("  to {\n");
        keyframes.push_str("    opacity: 1;\n");
        keyframes.push_str("    transform: translateY(0);\n");
        keyframes.push_str("  }\n");
        keyframes.push_str("}\n\n");

        // 缩放进入
        keyframes.push_str("@keyframes ant-scale-in {\n");
        keyframes.push_str("  from {\n");
        keyframes.push_str("    opacity: 0;\n");
        keyframes.push_str("    transform: scale(0.8);\n");
        keyframes.push_str("  }\n");
        keyframes.push_str("  to {\n");
        keyframes.push_str("    opacity: 1;\n");
        keyframes.push_str("    transform: scale(1);\n");
        keyframes.push_str("  }\n");
        keyframes.push_str("}\n\n");

        // 弹跳进入
        keyframes.push_str("@keyframes ant-bounce-in {\n");
        keyframes.push_str("  0% {\n");
        keyframes.push_str("    opacity: 0;\n");
        keyframes.push_str("    transform: scale(0.3);\n");
        keyframes.push_str("  }\n");
        keyframes.push_str("  50% {\n");
        keyframes.push_str("    opacity: 1;\n");
        keyframes.push_str("    transform: scale(1.05);\n");
        keyframes.push_str("  }\n");
        keyframes.push_str("  70% {\n");
        keyframes.push_str("    transform: scale(0.9);\n");
        keyframes.push_str("  }\n");
        keyframes.push_str("  100% {\n");
        keyframes.push_str("    opacity: 1;\n");
        keyframes.push_str("    transform: scale(1);\n");
        keyframes.push_str("  }\n");
        keyframes.push_str("}\n\n");

        // 脉冲动画
        keyframes.push_str("@keyframes ant-pulse {\n");
        keyframes.push_str("  0%, 100% {\n");
        keyframes.push_str("    transform: scale(1);\n");
        keyframes.push_str("  }\n");
        keyframes.push_str("  50% {\n");
        keyframes.push_str("    transform: scale(1.05);\n");
        keyframes.push_str("  }\n");
        keyframes.push_str("}\n\n");

        // 摇摆动画
        keyframes.push_str("@keyframes ant-shake {\n");
        keyframes.push_str("  0%, 100% {\n");
        keyframes.push_str("    transform: translateX(0);\n");
        keyframes.push_str("  }\n");
        keyframes.push_str("  10%, 30%, 50%, 70%, 90% {\n");
        keyframes.push_str("    transform: translateX(-10px);\n");
        keyframes.push_str("  }\n");
        keyframes.push_str("  20%, 40%, 60%, 80% {\n");
        keyframes.push_str("    transform: translateX(10px);\n");
        keyframes.push_str("  }\n");
        keyframes.push_str("}\n\n");

        // 旋转动画
        keyframes.push_str("@keyframes ant-spin {\n");
        keyframes.push_str("  from {\n");
        keyframes.push_str("    transform: rotate(0deg);\n");
        keyframes.push_str("  }\n");
        keyframes.push_str("  to {\n");
        keyframes.push_str("    transform: rotate(360deg);\n");
        keyframes.push_str("  }\n");
        keyframes.push_str("}\n\n");

        keyframes
    }
}

impl Default for AntDesignAnimations {
    fn default() -> Self {
        Self::default()
    }
}

/// 动画工具函数
pub mod utils {
    use super::*;

    /// 检查是否应该禁用动画
    pub fn should_disable_animation() -> bool {
        // 在实际应用中，这里可以检查用户偏好设置或系统设置
        false
    }

    /// 根据组件类型获取推荐的动画时长
    pub fn get_recommended_duration(component: ComponentAnimation) -> Duration {
        match component {
            ComponentAnimation::Tooltip | ComponentAnimation::Dropdown => Duration::Fast,
            ComponentAnimation::Modal | ComponentAnimation::Drawer => Duration::Mid,
            ComponentAnimation::Notification | ComponentAnimation::Message => Duration::Mid,
            ComponentAnimation::Spin | ComponentAnimation::Progress => Duration::Slow,
            _ => Duration::Mid,
        }
    }

    /// 根据动画场景获取推荐的缓动函数
    pub fn get_recommended_easing(scene: AnimationScene) -> Easing {
        match scene {
            AnimationScene::Enter => Easing::EaseOut,
            AnimationScene::Exit => Easing::EaseIn,
            AnimationScene::Hover | AnimationScene::Active => Easing::EaseInOut,
            AnimationScene::Loading => Easing::Linear,
            _ => Easing::EaseOut,
        }
    }

    /// 生成动画类名
    pub fn generate_animation_class_name(
        component: ComponentAnimation,
        scene: AnimationScene,
    ) -> String {
        let component_name = format!("{:?}", component).to_lowercase();
        let scene_name = format!("{:?}", scene).to_lowercase();
        format!("ant-{}-{}", component_name, scene_name)
    }

    /// 创建自定义动画配置
    pub fn create_custom_animation(
        transition_type: TransitionType,
        duration_ms: u32,
        easing: Easing,
    ) -> AnimationConfig {
        AnimationConfig::new(transition_type)
            .duration(Duration::Custom(duration_ms))
            .easing(easing)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ant_design_animations() {
        let animations = AntDesignAnimations::default();

        // 测试预设动画
        assert!(animations.get_preset("fade-in").is_some());
        assert!(animations.get_preset("slide-up").is_some());
        assert!(animations.get_preset("scale-in").is_some());

        // 测试组件动画
        assert!(animations
            .get_component_animation(ComponentAnimation::Modal, AnimationScene::Enter)
            .is_some());
        assert!(animations
            .get_component_animation(ComponentAnimation::Button, AnimationScene::Hover)
            .is_some());
    }

    #[test]
    fn test_no_motion() {
        let animations = AntDesignAnimations::no_motion();

        assert!(!animations.motion_enabled);
        assert!(animations.reduce_motion);
        assert_eq!(animations.default_duration, Duration::Fast);

        // 动画被禁用时应该返回 None
        assert!(animations.get_preset("fade-in").is_none());
        assert!(animations
            .get_component_animation(ComponentAnimation::Modal, AnimationScene::Enter)
            .is_none());
    }

    #[test]
    fn test_custom_animations() {
        let mut animations = AntDesignAnimations::default();

        // 添加自定义预设
        let custom_config = AnimationConfig::new(TransitionType::Flip)
            .duration(Duration::Slow)
            .easing(Easing::EaseInOut);
        animations.add_preset("custom-flip".to_string(), custom_config.clone());

        assert_eq!(animations.get_preset("custom-flip"), Some(&custom_config));

        // 添加自定义组件动画
        animations.add_component_animation(
            ComponentAnimation::Card,
            AnimationScene::Hover,
            custom_config.clone(),
        );

        assert_eq!(
            animations.get_component_animation(ComponentAnimation::Card, AnimationScene::Hover),
            Some(&custom_config)
        );
    }

    #[test]
    fn test_css_generation() {
        let animations = AntDesignAnimations::default();

        let css_classes = animations.generate_css_classes();
        assert!(css_classes.contains(".ant-motion-fade-in"));
        assert!(css_classes.contains(".ant-modal-enter"));
        assert!(css_classes.contains("@media (prefers-reduced-motion: reduce)"));

        let variables = animations.to_css_variables();
        assert!(variables.contains_key("--ant-motion-duration-fast"));
        assert!(variables.contains_key("--ant-motion-ease-out"));

        let keyframes = animations.generate_keyframes();
        assert!(keyframes.contains("@keyframes ant-fade-in"));
        assert!(keyframes.contains("@keyframes ant-slide-up-in"));
    }

    #[test]
    fn test_utils() {
        assert_eq!(
            utils::get_recommended_duration(ComponentAnimation::Tooltip),
            Duration::Fast
        );
        assert_eq!(
            utils::get_recommended_duration(ComponentAnimation::Modal),
            Duration::Mid
        );

        assert_eq!(
            utils::get_recommended_easing(AnimationScene::Enter),
            Easing::EaseOut
        );
        assert_eq!(
            utils::get_recommended_easing(AnimationScene::Exit),
            Easing::EaseIn
        );

        assert_eq!(
            utils::generate_animation_class_name(ComponentAnimation::Button, AnimationScene::Hover),
            "ant-button-hover"
        );

        let custom = utils::create_custom_animation(TransitionType::Scale, 500, Easing::EaseOut);
        assert_eq!(custom.transition_type, TransitionType::Scale);
        assert_eq!(custom.duration, Duration::Custom(500));
        assert_eq!(custom.easing, Easing::EaseOut);
    }
}

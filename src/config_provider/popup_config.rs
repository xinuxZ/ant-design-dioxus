//! 弹出层配置
//!
//! 提供统一的弹出层行为配置，包括定位、溢出处理、容器管理等

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 弹出层配置
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PopupConfig {
    /// 弹出层是否匹配选择器宽度
    pub match_select_width: Option<bool>,
    /// 弹出层溢出处理方式
    pub overflow: PopupOverflow,
    /// 弹出层放置策略
    pub placement_strategy: PlacementStrategy,
    /// 弹出层容器配置
    pub container_config: ContainerConfig,
    /// 弹出层动画配置
    pub animation_config: AnimationConfig,
    /// 弹出层z-index基础值
    pub z_index_base: u32,
    /// 是否启用虚拟定位
    pub virtual_positioning: bool,
    /// 弹出层边距
    pub margin: PopupMargin,
    /// 获取弹出容器的函数
    pub get_popup_container: Option<String>,
    /// 自动调整溢出
    pub auto_adjust_overflow: Option<bool>,
    /// 弹出位置
    pub placement: Option<PopupPlacement>,
    /// 触发方式
    pub trigger: Option<String>,
}

/// 弹出层溢出处理方式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PopupOverflow {
    /// 自动调整位置
    Auto,
    /// 隐藏溢出部分
    Hidden,
    /// 显示滚动条
    Scroll,
    /// 自适应大小
    Adaptive,
    /// 翻转到对面
    Flip,
}

/// 弹出层放置策略
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlacementStrategy {
    /// 首选放置位置
    pub preferred_placement: PopupPlacement,
    /// 备选放置位置
    pub fallback_placements: Vec<PopupPlacement>,
    /// 是否允许自动调整
    pub auto_adjust: bool,
    /// 边界检测配置
    pub boundary_detection: BoundaryDetection,
}

/// 弹出层放置位置
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PopupPlacement {
    Top,
    TopStart,
    TopEnd,
    Bottom,
    BottomStart,
    BottomEnd,
    BottomLeft,
    Left,
    LeftStart,
    LeftEnd,
    Right,
    RightStart,
    RightEnd,
}

/// 边界检测配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BoundaryDetection {
    /// 检测边界元素选择器
    pub boundary_selector: Option<String>,
    /// 边界偏移量
    pub boundary_offset: BoundaryOffset,
    /// 是否检测视口边界
    pub check_viewport: bool,
    /// 是否检测滚动容器边界
    pub check_scroll_container: bool,
}

/// 边界偏移量
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BoundaryOffset {
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
    pub left: i32,
}

/// 容器配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContainerConfig {
    /// 获取弹出容器的函数
    pub get_popup_container: Option<String>, // 函数名或选择器
    /// 获取目标容器的函数
    pub get_target_container: Option<String>, // 函数名或选择器
    /// 默认容器选择器
    pub default_container: String,
    /// 是否创建新的容器层
    pub create_container_layer: bool,
    /// 容器类名
    pub container_class_name: Option<String>,
    /// 容器样式
    pub container_style: Option<HashMap<String, String>>,
}

/// 动画配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnimationConfig {
    /// 进入动画名称
    pub enter_animation: Option<String>,
    /// 退出动画名称
    pub exit_animation: Option<String>,
    /// 动画持续时间（毫秒）
    pub duration: u32,
    /// 动画缓动函数
    pub easing: String,
    /// 是否启用动画
    pub enabled: bool,
}

/// 弹出层边距
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PopupMargin {
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
    pub left: i32,
}

/// 弹出层管理器
#[derive(Debug, Clone)]
pub struct PopupManager {
    config: PopupConfig,
    active_popups: Vec<PopupInstance>,
    z_index_counter: u32,
}

/// 弹出层实例
#[derive(Debug, Clone)]
pub struct PopupInstance {
    pub id: String,
    pub placement: PopupPlacement,
    pub z_index: u32,
    pub trigger_element: Option<String>, // 触发元素的ID
    pub popup_element: Option<String>,   // 弹出层元素的ID
    pub is_visible: bool,
}

/// 弹出层定位结果
#[derive(Debug, Clone)]
pub struct PopupPositionResult {
    pub x: f64,
    pub y: f64,
    pub placement: PopupPlacement,
    pub arrow_position: Option<ArrowPosition>,
    pub overflow_adjustments: Vec<OverflowAdjustment>,
}

/// 箭头位置
#[derive(Debug, Clone)]
pub struct ArrowPosition {
    pub x: f64,
    pub y: f64,
    pub rotation: f64,
}

/// 溢出调整
#[derive(Debug, Clone)]
pub struct OverflowAdjustment {
    pub direction: OverflowDirection,
    pub adjustment: f64,
}

/// 溢出方向
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverflowDirection {
    Top,
    Right,
    Bottom,
    Left,
}

impl Default for PopupOverflow {
    fn default() -> Self {
        Self::Auto
    }
}

impl Default for PlacementStrategy {
    fn default() -> Self {
        Self {
            preferred_placement: PopupPlacement::Bottom,
            fallback_placements: vec![
                PopupPlacement::Top,
                PopupPlacement::Right,
                PopupPlacement::Left,
            ],
            auto_adjust: true,
            boundary_detection: BoundaryDetection::default(),
        }
    }
}

impl Default for BoundaryDetection {
    fn default() -> Self {
        Self {
            boundary_selector: None,
            boundary_offset: BoundaryOffset {
                top: 8,
                right: 8,
                bottom: 8,
                left: 8,
            },
            check_viewport: true,
            check_scroll_container: true,
        }
    }
}

impl Default for ContainerConfig {
    fn default() -> Self {
        Self {
            get_popup_container: None,
            get_target_container: None,
            default_container: "body".to_string(),
            create_container_layer: true,
            container_class_name: Some("ant-popup-container".to_string()),
            container_style: None,
        }
    }
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            enter_animation: Some("fadeIn".to_string()),
            exit_animation: Some("fadeOut".to_string()),
            duration: 200,
            easing: "ease-out".to_string(),
            enabled: true,
        }
    }
}

impl Default for PopupMargin {
    fn default() -> Self {
        Self {
            top: 4,
            right: 4,
            bottom: 4,
            left: 4,
        }
    }
}

impl PopupConfig {
    /// 创建新的弹出层配置
    pub fn new() -> Self {
        Self {
            match_select_width: Some(true),
            overflow: PopupOverflow::Auto,
            placement_strategy: PlacementStrategy::default(),
            container_config: ContainerConfig::default(),
            animation_config: AnimationConfig::default(),
            z_index_base: 1000,
            virtual_positioning: false,
            margin: PopupMargin::default(),
            get_popup_container: None,
            auto_adjust_overflow: None,
            placement: None,
            trigger: None,
        }
    }

    /// 设置是否匹配选择器宽度
    pub fn with_match_select_width(mut self, match_width: bool) -> Self {
        self.match_select_width = Some(match_width);
        self
    }

    /// 设置溢出处理方式
    pub fn with_overflow(mut self, overflow: PopupOverflow) -> Self {
        self.overflow = overflow;
        self
    }

    /// 设置放置策略
    pub fn with_placement_strategy(mut self, strategy: PlacementStrategy) -> Self {
        self.placement_strategy = strategy;
        self
    }

    /// 设置z-index基础值
    pub fn with_z_index_base(mut self, z_index: u32) -> Self {
        self.z_index_base = z_index;
        self
    }

    /// 启用虚拟定位
    pub fn with_virtual_positioning(mut self, enabled: bool) -> Self {
        self.virtual_positioning = enabled;
        self
    }
}

impl PopupManager {
    /// 创建新的弹出层管理器
    pub fn new(config: PopupConfig) -> Self {
        Self {
            z_index_counter: config.z_index_base,
            config,
            active_popups: Vec::new(),
        }
    }

    /// 注册新的弹出层
    pub fn register_popup(&mut self, id: String, trigger_element: Option<String>) -> PopupInstance {
        self.z_index_counter += 1;

        let instance = PopupInstance {
            id: id.clone(),
            placement: self.config.placement_strategy.preferred_placement,
            z_index: self.z_index_counter,
            trigger_element,
            popup_element: None,
            is_visible: false,
        };

        self.active_popups.push(instance.clone());
        instance
    }

    /// 显示弹出层
    pub fn show_popup(&mut self, id: &str) -> Result<(), String> {
        if let Some(popup) = self.active_popups.iter_mut().find(|p| p.id == id) {
            popup.is_visible = true;
            Ok(())
        } else {
            Err(format!("弹出层 '{}' 未找到", id))
        }
    }

    /// 隐藏弹出层
    pub fn hide_popup(&mut self, id: &str) -> Result<(), String> {
        if let Some(popup) = self.active_popups.iter_mut().find(|p| p.id == id) {
            popup.is_visible = false;
            Ok(())
        } else {
            Err(format!("弹出层 '{}' 未找到", id))
        }
    }

    /// 移除弹出层
    pub fn remove_popup(&mut self, id: &str) {
        self.active_popups.retain(|p| p.id != id);
    }

    /// 获取可见的弹出层列表
    pub fn get_visible_popups(&self) -> Vec<&PopupInstance> {
        self.active_popups.iter().filter(|p| p.is_visible).collect()
    }

    /// 计算弹出层位置
    pub fn calculate_position(
        &self,
        trigger_rect: (f64, f64, f64, f64), // x, y, width, height
        popup_size: (f64, f64),             // width, height
        placement: PopupPlacement,
    ) -> PopupPositionResult {
        let (trigger_x, trigger_y, trigger_width, trigger_height) = trigger_rect;
        let (popup_width, popup_height) = popup_size;
        let margin = &self.config.margin;

        let (x, y) = match placement {
            PopupPlacement::Top => (
                trigger_x + (trigger_width - popup_width) / 2.0,
                trigger_y - popup_height - margin.bottom as f64,
            ),
            PopupPlacement::TopStart => {
                (trigger_x, trigger_y - popup_height - margin.bottom as f64)
            }
            PopupPlacement::TopEnd => (
                trigger_x + trigger_width - popup_width,
                trigger_y - popup_height - margin.bottom as f64,
            ),
            PopupPlacement::Bottom => (
                trigger_x + (trigger_width - popup_width) / 2.0,
                trigger_y + trigger_height + margin.top as f64,
            ),
            PopupPlacement::BottomStart => {
                (trigger_x, trigger_y + trigger_height + margin.top as f64)
            }
            PopupPlacement::BottomEnd => (
                trigger_x + trigger_width - popup_width,
                trigger_y + trigger_height + margin.top as f64,
            ),
            PopupPlacement::BottomLeft => (
                trigger_x - popup_width - margin.right as f64,
                trigger_y + trigger_height + margin.top as f64,
            ),
            PopupPlacement::Left => (
                trigger_x - popup_width - margin.right as f64,
                trigger_y + (trigger_height - popup_height) / 2.0,
            ),
            PopupPlacement::LeftStart => (trigger_x - popup_width - margin.right as f64, trigger_y),
            PopupPlacement::LeftEnd => (
                trigger_x - popup_width - margin.right as f64,
                trigger_y + trigger_height - popup_height,
            ),
            PopupPlacement::Right => (
                trigger_x + trigger_width + margin.left as f64,
                trigger_y + (trigger_height - popup_height) / 2.0,
            ),
            PopupPlacement::RightStart => {
                (trigger_x + trigger_width + margin.left as f64, trigger_y)
            }
            PopupPlacement::RightEnd => (
                trigger_x + trigger_width + margin.left as f64,
                trigger_y + trigger_height - popup_height,
            ),
        };

        PopupPositionResult {
            x,
            y,
            placement,
            arrow_position: self.calculate_arrow_position(trigger_rect, (x, y), placement),
            overflow_adjustments: Vec::new(), // TODO: 实现溢出检测
        }
    }

    /// 计算箭头位置
    fn calculate_arrow_position(
        &self,
        trigger_rect: (f64, f64, f64, f64),
        popup_position: (f64, f64),
        placement: PopupPlacement,
    ) -> Option<ArrowPosition> {
        let (trigger_x, trigger_y, trigger_width, trigger_height) = trigger_rect;
        let (popup_x, popup_y) = popup_position;

        match placement {
            PopupPlacement::Top | PopupPlacement::TopStart | PopupPlacement::TopEnd => {
                Some(ArrowPosition {
                    x: trigger_x + trigger_width / 2.0 - popup_x,
                    y: 0.0, // 箭头在弹出层底部
                    rotation: 180.0,
                })
            }
            PopupPlacement::Bottom | PopupPlacement::BottomStart | PopupPlacement::BottomEnd => {
                Some(ArrowPosition {
                    x: trigger_x + trigger_width / 2.0 - popup_x,
                    y: 0.0, // 箭头在弹出层顶部
                    rotation: 0.0,
                })
            }
            PopupPlacement::Left | PopupPlacement::LeftStart | PopupPlacement::LeftEnd => {
                Some(ArrowPosition {
                    x: 0.0, // 箭头在弹出层右侧
                    y: trigger_y + trigger_height / 2.0 - popup_y,
                    rotation: 90.0,
                })
            }
            PopupPlacement::Right | PopupPlacement::RightStart | PopupPlacement::RightEnd => {
                Some(ArrowPosition {
                    x: 0.0, // 箭头在弹出层左侧
                    y: trigger_y + trigger_height / 2.0 - popup_y,
                    rotation: -90.0,
                })
            }
            PopupPlacement::BottomLeft => {
                Some(ArrowPosition {
                    x: 0.0, // 箭头在弹出层右侧
                    y: trigger_y + trigger_height / 2.0 - popup_y,
                    rotation: 90.0,
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_popup_config_creation() {
        let config = PopupConfig::new()
            .with_match_select_width(false)
            .with_overflow(PopupOverflow::Scroll)
            .with_z_index_base(2000);

        assert_eq!(config.match_select_width, Some(false));
        assert_eq!(config.overflow, PopupOverflow::Scroll);
        assert_eq!(config.z_index_base, 2000);
    }

    #[test]
    fn test_popup_manager() {
        let config = PopupConfig::new();
        let mut manager = PopupManager::new(config);

        let popup = manager.register_popup("test-popup".to_string(), None);
        assert_eq!(popup.id, "test-popup");
        assert!(!popup.is_visible);

        manager.show_popup(&popup.id).unwrap();
        let visible_popups = manager.get_visible_popups();
        assert_eq!(visible_popups.len(), 1);
    }

    #[test]
    fn test_position_calculation() {
        let config = PopupConfig::new();
        let manager = PopupManager::new(config);

        let trigger_rect = (100.0, 100.0, 200.0, 50.0); // x, y, width, height
        let popup_size = (150.0, 100.0); // width, height

        let result = manager.calculate_position(trigger_rect, popup_size, PopupPlacement::Bottom);

        // 弹出层应该在触发器下方居中
        assert_eq!(result.x, 125.0); // 100 + (200 - 150) / 2
        assert_eq!(result.y, 154.0); // 100 + 50 + 4 (margin)
        assert_eq!(result.placement, PopupPlacement::Bottom);
    }
}

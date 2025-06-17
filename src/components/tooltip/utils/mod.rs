//! Tooltip 组件工具函数

use crate::components::tooltip::types::*;
use web_sys::{window, Element, HtmlElement};
use wasm_bindgen::JsCast;

/// 获取元素的边界信息
pub fn get_element_bounds(element: &Element) -> Result<ElementBounds, String> {
    let rect = element.get_bounding_client_rect();
    
    Ok(ElementBounds {
        left: rect.left(),
        top: rect.top(),
        width: rect.width(),
        height: rect.height(),
    })
}

/// 获取视口尺寸
pub fn get_viewport_size() -> Result<(f64, f64), String> {
    let window = window().ok_or("无法获取 window 对象")?;
    
    let width = window
        .inner_width()
        .map_err(|_| "无法获取视口宽度")?
        .as_f64()
        .ok_or("视口宽度不是有效数字")?;
        
    let height = window
        .inner_height()
        .map_err(|_| "无法获取视口高度")?
        .as_f64()
        .ok_or("视口高度不是有效数字")?;
    
    Ok((width, height))
}

/// 获取滚动偏移量
pub fn get_scroll_offset() -> Result<(f64, f64), String> {
    let window = window().ok_or("无法获取 window 对象")?;
    
    let scroll_x = window.page_x_offset().unwrap_or(0.0);
    let scroll_y = window.page_y_offset().unwrap_or(0.0);
    
    Ok((scroll_x, scroll_y))
}

/// 计算 Tooltip 的位置
pub fn calculate_tooltip_position(
    target_bounds: &ElementBounds,
    tooltip_size: (f64, f64),
    placement: &TooltipPlacement,
    auto_adjust: bool,
) -> Result<TooltipPosition, String> {
    let (tooltip_width, tooltip_height) = tooltip_size;
    let (viewport_width, viewport_height) = get_viewport_size()?;
    let (scroll_x, scroll_y) = get_scroll_offset()?;
    
    // 计算基础位置
    let (mut x, mut y) = calculate_base_position(target_bounds, tooltip_size, placement);
    
    // 添加滚动偏移
    x += scroll_x;
    y += scroll_y;
    
    let mut actual_placement = placement.clone();
    
    // 自动调整位置以避免溢出
    if auto_adjust {
        let adjusted = adjust_position_for_overflow(
            (x, y),
            tooltip_size,
            (viewport_width, viewport_height),
            target_bounds,
            placement,
        )?;
        
        x = adjusted.x;
        y = adjusted.y;
        actual_placement = adjusted.actual_placement;
    }
    
    Ok(TooltipPosition {
        x,
        y,
        actual_placement,
    })
}

/// 计算基础位置（不考虑边界调整）
fn calculate_base_position(
    target_bounds: &ElementBounds,
    tooltip_size: (f64, f64),
    placement: &TooltipPlacement,
) -> (f64, f64) {
    let (tooltip_width, tooltip_height) = tooltip_size;
    let (offset_x, offset_y) = placement.get_position_offset();
    
    match placement {
        TooltipPlacement::Top => (
            target_bounds.center_x() - tooltip_width / 2.0,
            target_bounds.top - tooltip_height + offset_y,
        ),
        TooltipPlacement::TopLeft => (
            target_bounds.left + offset_x,
            target_bounds.top - tooltip_height + offset_y,
        ),
        TooltipPlacement::TopRight => (
            target_bounds.right() - tooltip_width + offset_x,
            target_bounds.top - tooltip_height + offset_y,
        ),
        TooltipPlacement::Bottom => (
            target_bounds.center_x() - tooltip_width / 2.0,
            target_bounds.bottom() + offset_y,
        ),
        TooltipPlacement::BottomLeft => (
            target_bounds.left + offset_x,
            target_bounds.bottom() + offset_y,
        ),
        TooltipPlacement::BottomRight => (
            target_bounds.right() - tooltip_width + offset_x,
            target_bounds.bottom() + offset_y,
        ),
        TooltipPlacement::Left => (
            target_bounds.left - tooltip_width + offset_x,
            target_bounds.center_y() - tooltip_height / 2.0,
        ),
        TooltipPlacement::LeftTop => (
            target_bounds.left - tooltip_width + offset_x,
            target_bounds.top + offset_y,
        ),
        TooltipPlacement::LeftBottom => (
            target_bounds.left - tooltip_width + offset_x,
            target_bounds.bottom() - tooltip_height + offset_y,
        ),
        TooltipPlacement::Right => (
            target_bounds.right() + offset_x,
            target_bounds.center_y() - tooltip_height / 2.0,
        ),
        TooltipPlacement::RightTop => (
            target_bounds.right() + offset_x,
            target_bounds.top + offset_y,
        ),
        TooltipPlacement::RightBottom => (
            target_bounds.right() + offset_x,
            target_bounds.bottom() - tooltip_height + offset_y,
        ),
    }
}

/// 调整位置以避免溢出
fn adjust_position_for_overflow(
    position: (f64, f64),
    tooltip_size: (f64, f64),
    viewport_size: (f64, f64),
    target_bounds: &ElementBounds,
    original_placement: &TooltipPlacement,
) -> Result<TooltipPosition, String> {
    let (mut x, mut y) = position;
    let (tooltip_width, tooltip_height) = tooltip_size;
    let (viewport_width, viewport_height) = viewport_size;
    let mut actual_placement = original_placement.clone();
    
    // 检查水平溢出
    if x < 0.0 {
        x = 8.0; // 最小边距
    } else if x + tooltip_width > viewport_width {
        x = viewport_width - tooltip_width - 8.0;
    }
    
    // 检查垂直溢出
    if y < 0.0 {
        y = 8.0;
    } else if y + tooltip_height > viewport_height {
        y = viewport_height - tooltip_height - 8.0;
    }
    
    // 如果调整后仍然溢出，尝试翻转位置
    if should_flip_placement(position, tooltip_size, viewport_size, original_placement) {
        let flipped_placement = get_flipped_placement(original_placement);
        let (flipped_x, flipped_y) = calculate_base_position(target_bounds, tooltip_size, &flipped_placement);
        
        // 检查翻转后的位置是否更好
        if is_position_better((flipped_x, flipped_y), position, tooltip_size, viewport_size) {
            x = flipped_x;
            y = flipped_y;
            actual_placement = flipped_placement;
        }
    }
    
    Ok(TooltipPosition {
        x,
        y,
        actual_placement,
    })
}

/// 判断是否应该翻转位置
fn should_flip_placement(
    position: (f64, f64),
    tooltip_size: (f64, f64),
    viewport_size: (f64, f64),
    placement: &TooltipPlacement,
) -> bool {
    let (x, y) = position;
    let (tooltip_width, tooltip_height) = tooltip_size;
    let (viewport_width, viewport_height) = viewport_size;
    
    match placement {
        TooltipPlacement::Top | TooltipPlacement::TopLeft | TooltipPlacement::TopRight => {
            y < 0.0
        }
        TooltipPlacement::Bottom | TooltipPlacement::BottomLeft | TooltipPlacement::BottomRight => {
            y + tooltip_height > viewport_height
        }
        TooltipPlacement::Left | TooltipPlacement::LeftTop | TooltipPlacement::LeftBottom => {
            x < 0.0
        }
        TooltipPlacement::Right | TooltipPlacement::RightTop | TooltipPlacement::RightBottom => {
            x + tooltip_width > viewport_width
        }
    }
}

/// 获取翻转后的位置
fn get_flipped_placement(placement: &TooltipPlacement) -> TooltipPlacement {
    match placement {
        TooltipPlacement::Top => TooltipPlacement::Bottom,
        TooltipPlacement::TopLeft => TooltipPlacement::BottomLeft,
        TooltipPlacement::TopRight => TooltipPlacement::BottomRight,
        TooltipPlacement::Bottom => TooltipPlacement::Top,
        TooltipPlacement::BottomLeft => TooltipPlacement::TopLeft,
        TooltipPlacement::BottomRight => TooltipPlacement::TopRight,
        TooltipPlacement::Left => TooltipPlacement::Right,
        TooltipPlacement::LeftTop => TooltipPlacement::RightTop,
        TooltipPlacement::LeftBottom => TooltipPlacement::RightBottom,
        TooltipPlacement::Right => TooltipPlacement::Left,
        TooltipPlacement::RightTop => TooltipPlacement::LeftTop,
        TooltipPlacement::RightBottom => TooltipPlacement::LeftBottom,
    }
}

/// 判断位置是否更好
fn is_position_better(
    new_position: (f64, f64),
    old_position: (f64, f64),
    tooltip_size: (f64, f64),
    viewport_size: (f64, f64),
) -> bool {
    let new_overflow = calculate_overflow(new_position, tooltip_size, viewport_size);
    let old_overflow = calculate_overflow(old_position, tooltip_size, viewport_size);
    
    new_overflow < old_overflow
}

/// 计算溢出面积
fn calculate_overflow(
    position: (f64, f64),
    tooltip_size: (f64, f64),
    viewport_size: (f64, f64),
) -> f64 {
    let (x, y) = position;
    let (tooltip_width, tooltip_height) = tooltip_size;
    let (viewport_width, viewport_height) = viewport_size;
    
    let left_overflow = if x < 0.0 { -x } else { 0.0 };
    let right_overflow = if x + tooltip_width > viewport_width {
        x + tooltip_width - viewport_width
    } else {
        0.0
    };
    let top_overflow = if y < 0.0 { -y } else { 0.0 };
    let bottom_overflow = if y + tooltip_height > viewport_height {
        y + tooltip_height - viewport_height
    } else {
        0.0
    };
    
    left_overflow + right_overflow + top_overflow + bottom_overflow
}

/// 估算 Tooltip 内容尺寸
pub fn estimate_tooltip_size(content: &str, max_width: Option<f64>) -> (f64, f64) {
    // 简单的尺寸估算，实际项目中可能需要更精确的计算
    let char_width = 8.0; // 平均字符宽度
    let line_height = 22.0; // 行高
    let padding = 16.0; // 内边距
    
    let max_width = max_width.unwrap_or(300.0);
    let content_width = (content.len() as f64 * char_width).min(max_width - padding);
    let lines = ((content.len() as f64 * char_width) / (max_width - padding)).ceil().max(1.0);
    let content_height = lines * line_height;
    
    (content_width + padding, content_height + padding)
}

/// 创建延迟执行器
pub struct DelayExecutor {
    timeout_id: Option<i32>,
}

impl DelayExecutor {
    pub fn new() -> Self {
        Self { timeout_id: None }
    }
    
    /// 延迟执行函数
    pub fn execute_after_delay<F>(&mut self, delay_ms: f64, callback: F) -> Result<(), String>
    where
        F: FnOnce() + 'static,
    {
        // 清除之前的定时器
        self.clear();
        
        let window = window().ok_or("无法获取 window 对象")?;
        
        let closure = wasm_bindgen::closure::Closure::once_into_js(callback);
        
        let timeout_id = window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                closure.as_ref().unchecked_ref(),
                delay_ms as i32,
            )
            .map_err(|_| "无法设置定时器")?;
            
        self.timeout_id = Some(timeout_id);
        
        Ok(())
    }
    
    /// 清除定时器
    pub fn clear(&mut self) {
        if let Some(timeout_id) = self.timeout_id.take() {
            if let Some(window) = window() {
                let _ = window.clear_timeout_with_handle(timeout_id);
            }
        }
    }
}

impl Drop for DelayExecutor {
    fn drop(&mut self) {
        self.clear();
    }
}

/// 检查元素是否在视口内
pub fn is_element_in_viewport(element: &Element) -> Result<bool, String> {
    let bounds = get_element_bounds(element)?;
    let (viewport_width, viewport_height) = get_viewport_size()?;
    
    Ok(bounds.left >= 0.0
        && bounds.top >= 0.0
        && bounds.right() <= viewport_width
        && bounds.bottom() <= viewport_height)
}

/// 获取最近的滚动容器
pub fn get_scroll_container(element: &Element) -> Option<Element> {
    let mut current = element.parent_element();
    
    while let Some(parent) = current {
        if let Ok(html_element) = parent.dyn_into::<HtmlElement>() {
            let style = window()?
                .get_computed_style(&html_element)
                .ok()?
                .flatten()?;
                
            let overflow = style.get_property_value("overflow").ok()?;
            let overflow_x = style.get_property_value("overflow-x").ok()?;
            let overflow_y = style.get_property_value("overflow-y").ok()?;
            
            if overflow.contains("auto")
                || overflow.contains("scroll")
                || overflow_x.contains("auto")
                || overflow_x.contains("scroll")
                || overflow_y.contains("auto")
                || overflow_y.contains("scroll")
            {
                return Some(parent);
            }
        }
        
        current = parent.parent_element();
    }
    
    None
}
//! Spin 组件的工具函数

use dioxus::prelude::*;
use std::collections::HashMap;
use super::types::{SpinProps, SpinState, SpinConfig, SpinSize, SpinTheme};

/// 创建 Spin 状态
pub fn create_spin_state(spinning: bool, delay: Option<u32>) -> SpinState {
    SpinState {
        visible: spinning && delay.is_none(),
        delayed: delay.is_some(),
        delay_timer: None,
    }
}

/// 验证 Spin 组件的 Props
pub fn validate_spin_props(props: &SpinProps) -> Result<(), String> {
    // 验证延迟时间
    if let Some(delay) = props.delay {
        if delay > 10000 {
            return Err("延迟时间不应超过 10 秒".to_string());
        }
    }

    // 验证提示文本长度
    if let Some(ref tip) = props.tip {
        if tip.len() > 100 {
            return Err("提示文本长度不应超过 100 个字符".to_string());
        }
    }

    Ok(())
}

/// 计算延迟显示逻辑
pub fn should_show_with_delay(spinning: bool, delay: Option<u32>, elapsed_time: u32) -> bool {
    if !spinning {
        return false;
    }

    match delay {
        Some(delay_ms) => elapsed_time >= delay_ms,
        None => true,
    }
}

/// 生成缓存键
pub fn generate_cache_key(config: &SpinConfig) -> String {
    format!(
        "spin_{}_{}_{}_{}",
        config.size,
        config.spinning,
        config.delay.unwrap_or(0),
        config.has_children
    )
}

/// 计算主题哈希值
pub fn calculate_theme_hash(theme: &SpinTheme) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    theme.color_primary.hash(&mut hasher);
    theme.color_text.hash(&mut hasher);
    theme.color_bg_container.hash(&mut hasher);
    theme.color_bg_mask.hash(&mut hasher);
    hasher.finish()
}

/// 合并用户主题与默认主题
pub fn merge_theme_with_default(user_theme: Option<SpinTheme>) -> SpinTheme {
    match user_theme {
        Some(mut theme) => {
            let default_theme = SpinTheme::default();
            
            // 如果用户主题的某些字段为空，使用默认值
            if theme.color_primary.is_empty() {
                theme.color_primary = default_theme.color_primary;
            }
            if theme.color_text.is_empty() {
                theme.color_text = default_theme.color_text;
            }
            if theme.color_bg_container.is_empty() {
                theme.color_bg_container = default_theme.color_bg_container;
            }
            if theme.color_bg_mask.is_empty() {
                theme.color_bg_mask = default_theme.color_bg_mask;
            }
            
            theme
        }
        None => SpinTheme::default(),
    }
}

/// 检查是否有子元素
pub fn has_children(children: &Element) -> bool {
    // 这里需要根据 Dioxus 的实际 API 来实现
    // 暂时返回 true，实际实现需要检查 children 是否为空
    true
}

/// 判断是否为移动设备
pub fn is_mobile_device() -> bool {
    // 这里可以通过 JavaScript 互操作来检测
    // 暂时返回 false
    false
}

/// 获取响应式配置
pub fn get_responsive_config(size: &SpinSize) -> HashMap<String, String> {
    let mut config = HashMap::new();
    
    match size {
        SpinSize::Small => {
            config.insert("mobile_size".to_string(), "12px".to_string());
            config.insert("tablet_size".to_string(), "14px".to_string());
        }
        SpinSize::Default => {
            config.insert("mobile_size".to_string(), "16px".to_string());
            config.insert("tablet_size".to_string(), "20px".to_string());
        }
        SpinSize::Large => {
            config.insert("mobile_size".to_string(), "24px".to_string());
            config.insert("tablet_size".to_string(), "32px".to_string());
        }
    }
    
    config
}

/// 优化延迟时间
pub fn optimize_delay_time(delay: Option<u32>, has_children: bool) -> Option<u32> {
    match delay {
        Some(d) => {
            // 如果有子元素，建议最小延迟时间为 300ms
            if has_children && d < 300 {
                Some(300)
            } else {
                Some(d)
            }
        }
        None => {
            // 如果没有设置延迟但有子元素，建议设置默认延迟
            if has_children {
                Some(300)
            } else {
                None
            }
        }
    }
}

/// 计算自适应尺寸
pub fn calculate_adaptive_size(container_width: f64, container_height: f64) -> SpinSize {
    let min_dimension = container_width.min(container_height);
    
    if min_dimension < 100.0 {
        SpinSize::Small
    } else if min_dimension > 300.0 {
        SpinSize::Large
    } else {
        SpinSize::Default
    }
}

/// 生成 CSS 变量映射
pub fn generate_css_variables(theme: &SpinTheme, size: &SpinSize) -> HashMap<String, String> {
    let mut vars = HashMap::new();
    
    vars.insert("--spin-color-primary".to_string(), theme.color_primary.clone());
    vars.insert("--spin-color-text".to_string(), theme.color_text.clone());
    vars.insert("--spin-color-bg".to_string(), theme.color_bg_container.clone());
    vars.insert("--spin-color-mask".to_string(), theme.color_bg_mask.clone());
    vars.insert("--spin-duration".to_string(), theme.motion_duration_slow.clone());
    
    // 根据尺寸设置字体大小
    let font_size = match size {
        SpinSize::Small => &theme.font_size_sm,
        SpinSize::Default => &theme.font_size,
        SpinSize::Large => &theme.font_size_lg,
    };
    vars.insert("--spin-font-size".to_string(), font_size.clone());
    
    vars
}

/// 检查是否需要显示提示文本
pub fn should_show_tip(tip: &Option<String>, has_children: bool) -> bool {
    tip.is_some() && (has_children || tip.as_ref().unwrap().len() > 0)
}

/// 格式化提示文本
pub fn format_tip_text(tip: &Option<String>) -> Option<String> {
    tip.as_ref().map(|t| {
        let trimmed = t.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        }
    }).flatten()
}

/// 计算动画延迟
pub fn calculate_animation_delay(index: usize, total: usize) -> f64 {
    let base_delay = 0.1; // 100ms
    let step = 0.4 / total as f64; // 总共 400ms 分布
    base_delay + (index as f64 * step)
}

/// 生成唯一 ID
pub fn generate_unique_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    
    format!("spin_{}", timestamp)
}

/// 解析尺寸字符串
pub fn parse_size_string(size_str: &str) -> Result<SpinSize, String> {
    match size_str.to_lowercase().as_str() {
        "small" | "sm" => Ok(SpinSize::Small),
        "default" | "medium" | "md" => Ok(SpinSize::Default),
        "large" | "lg" => Ok(SpinSize::Large),
        _ => Err(format!("无效的尺寸字符串: {}", size_str)),
    }
}

/// 创建默认配置
pub fn create_default_config() -> SpinConfig {
    SpinConfig {
        size: SpinSize::Default,
        spinning: true,
        delay: None,
        tip: None,
        has_children: false,
    }
}

/// 更新 Spin 状态
pub fn update_spin_state(
    current_state: &mut SpinState,
    spinning: bool,
    delay: Option<u32>,
) {
    current_state.visible = spinning && delay.is_none();
    current_state.delayed = delay.is_some();
    
    // 如果不再旋转，清除延迟计时器
    if !spinning {
        current_state.delay_timer = None;
    }
}

/// 检查性能模式
pub fn is_performance_mode() -> bool {
    // 可以通过环境变量或配置来控制
    std::env::var("SPIN_PERFORMANCE_MODE")
        .map(|v| v == "true")
        .unwrap_or(false)
}

/// 获取最佳动画帧率
pub fn get_optimal_frame_rate() -> u32 {
    if is_performance_mode() {
        30 // 性能模式使用较低帧率
    } else {
        60 // 正常模式使用标准帧率
    }
}

/// 计算内存使用情况
pub fn estimate_memory_usage(config: &SpinConfig) -> usize {
    let base_size = std::mem::size_of::<SpinConfig>();
    let tip_size = config.tip.as_ref().map(|t| t.len()).unwrap_or(0);
    
    base_size + tip_size
}
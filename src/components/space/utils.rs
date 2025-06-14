//! Space 组件工具函数
//!
//! 提供 Space 组件相关的工具函数，包括间距计算、配置验证、主题合并等功能。

use dioxus::prelude::*;

use crate::components::space::types::*;
use std::collections::HashMap;

/// 计算间距大小
pub fn calculate_space_size(
    size: &SpaceSizeConfig,
    direction: SpaceDirection,
    theme: &SpaceTheme,
) -> (u32, u32) {
    match size {
        SpaceSizeConfig::Single(space_size) => {
            let value = match space_size {
                SpaceSize::Small => theme.space_small,
                SpaceSize::Middle => theme.space_middle,
                SpaceSize::Large => theme.space_large,
                SpaceSize::Custom(val) => *val,
            };
            (value, value)
        }
        SpaceSizeConfig::Array([horizontal, vertical]) => (*horizontal, *vertical),
        SpaceSizeConfig::String(s) => {
            let value = match s.as_str() {
                "small" => theme.space_small,
                "middle" => theme.space_middle,
                "large" => theme.space_large,
                _ => {
                    // 尝试解析为数字
                    s.parse::<u32>().unwrap_or(theme.space_small)
                }
            };
            (value, value)
        }
    }
}

/// 判断是否应该包装子元素
pub fn should_wrap_children(direction: SpaceDirection, wrap: bool, children_count: usize) -> bool {
    match direction {
        SpaceDirection::Horizontal => wrap && children_count > 1,
        SpaceDirection::Vertical => false, // 垂直方向不支持换行
    }
}

/// 获取间距值（用于 CSS gap 属性）
pub fn get_space_gap_value(
    size: &SpaceSizeConfig,
    direction: SpaceDirection,
    theme: &SpaceTheme,
) -> String {
    let (horizontal, vertical) = calculate_space_size(size, direction, theme);

    match direction {
        SpaceDirection::Horizontal => format!("{}px", horizontal),
        SpaceDirection::Vertical => format!("{}px", vertical),
    }
}

/// 获取响应式间距值
pub fn get_responsive_gap_value(
    size: &SpaceSizeConfig,
    direction: SpaceDirection,
    theme: &SpaceTheme,
    is_mobile: bool,
) -> String {
    let (horizontal, vertical) = calculate_space_size(size, direction, theme);

    // 移动端使用较小的间距
    let mobile_factor = if is_mobile { 0.75 } else { 1.0 };

    match direction {
        SpaceDirection::Horizontal => {
            format!("{}px", (horizontal as f32 * mobile_factor) as u32)
        }
        SpaceDirection::Vertical => {
            format!("{}px", (vertical as f32 * mobile_factor) as u32)
        }
    }
}

/// 验证 Space 配置
pub fn validate_space_config(
    direction: SpaceDirection,
    size: &SpaceSizeConfig,
    wrap: bool,
    align: SpaceAlign,
) -> Result<(), String> {
    // 验证间距配置
    match size {
        SpaceSizeConfig::Single(space_size) => {
            if let SpaceSize::Custom(val) = space_size {
                if *val > 1000 {
                    return Err("间距值不能超过 1000px".to_string());
                }
            }
        }
        SpaceSizeConfig::Array([horizontal, vertical]) => {
            if *horizontal > 1000 || *vertical > 1000 {
                return Err("间距值不能超过 1000px".to_string());
            }
        }
        SpaceSizeConfig::String(_) => {}
    }

    // 验证换行配置
    if wrap && direction == SpaceDirection::Vertical {
        return Err("垂直方向不支持换行".to_string());
    }

    // 验证对齐配置
    if direction == SpaceDirection::Vertical && align == SpaceAlign::Baseline {
        return Err("垂直方向不建议使用 baseline 对齐".to_string());
    }

    Ok(())
}

/// 验证 Space.Compact 配置
pub fn validate_space_compact_config(
    direction: SpaceDirection,
    size: CompactSize,
    block: bool,
) -> Result<(), String> {
    // 垂直方向的限制
    if direction == SpaceDirection::Vertical {
        // 垂直模式下的特殊限制可以在这里添加
    }

    // 块级模式的验证
    if block && direction == SpaceDirection::Vertical {
        // 垂直块级模式的特殊处理
    }

    Ok(())
}

/// 生成缓存键
pub fn generate_space_cache_key(
    direction: SpaceDirection,
    size: &SpaceSizeConfig,
    align: SpaceAlign,
    wrap: bool,
    theme_hash: u64,
) -> String {
    format!(
        "space_{}_{:?}_{}_{}_{}",
        direction, size, align, wrap, theme_hash
    )
}

/// 生成 Space.Compact 缓存键
pub fn generate_space_compact_cache_key(
    direction: SpaceDirection,
    size: CompactSize,
    block: bool,
    theme_hash: u64,
) -> String {
    format!(
        "space_compact_{}_{}_{}_{}",
        direction, size, block, theme_hash
    )
}

/// 计算主题哈希值
pub fn calculate_theme_hash(theme: &SpaceTheme) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    theme.space_small.hash(&mut hasher);
    theme.space_middle.hash(&mut hasher);
    theme.space_large.hash(&mut hasher);
    theme.compact_border_color.hash(&mut hasher);
    theme.compact_border_width.hash(&mut hasher);
    theme.compact_border_radius.hash(&mut hasher);

    hasher.finish()
}

/// 合并用户主题与默认主题
pub fn merge_space_theme(user_theme: Option<&SpaceTheme>) -> SpaceTheme {
    let default_theme = SpaceTheme::default();

    let mut merged = default_theme.clone();
    match user_theme {
        Some(theme) => {
            // 合并基础间距值
            if theme.space_small != default_theme.space_small {
                merged.space_small = theme.space_small;
            }
            if theme.space_middle != default_theme.space_middle {
                merged.space_middle = theme.space_middle;
            }
            if theme.space_large != default_theme.space_large {
                merged.space_large = theme.space_large;
            }

            // 合并紧凑模式配置
            if theme.compact_border_color != default_theme.compact_border_color {
                merged.compact_border_color = theme.compact_border_color.clone();
            }
            if theme.compact_border_width != default_theme.compact_border_width {
                merged.compact_border_width = theme.compact_border_width;
            }
            if theme.compact_border_radius != default_theme.compact_border_radius {
                merged.compact_border_radius = theme.compact_border_radius;
            }

            // 合并 CSS 变量
            for (key, value) in &theme.css_vars {
                merged.css_vars.insert(key.clone(), value.clone());
            }

            merged
        }
        None => default_theme,
    }
}

/// 获取子元素数量
pub fn count_children(children: &Option<Element>) -> usize {
    match children {
        Some(_) => {
            // 这里需要根据实际的 Dioxus Element 结构来计算
            // 暂时返回一个默认值
            1
        }
        None => 0,
    }
}

/// 判断是否为移动设备
pub fn is_mobile_device() -> bool {
    // 这里可以通过 web-sys 获取 window.innerWidth 或使用媒体查询
    // 暂时返回 false，实际实现需要根据具体需求
    false
}

/// 获取响应式配置
pub fn get_responsive_config() -> HashMap<String, u32> {
    let mut breakpoints = HashMap::new();
    breakpoints.insert("xs".to_string(), 480);
    breakpoints.insert("sm".to_string(), 576);
    breakpoints.insert("md".to_string(), 768);
    breakpoints.insert("lg".to_string(), 992);
    breakpoints.insert("xl".to_string(), 1200);
    breakpoints.insert("xxl".to_string(), 1600);
    breakpoints
}

/// 优化间距值（确保合理的间距范围）
pub fn optimize_space_value(value: u32, min_value: u32, max_value: u32) -> u32 {
    if value < min_value {
        min_value
    } else if value > max_value {
        max_value
    } else {
        value
    }
}

/// 计算自适应间距
pub fn calculate_adaptive_spacing(
    base_size: u32,
    container_width: u32,
    min_spacing: u32,
    max_spacing: u32,
) -> u32 {
    // 根据容器宽度自适应调整间距
    let ratio = container_width as f32 / 1200.0; // 以 1200px 为基准
    let adaptive_size = (base_size as f32 * ratio) as u32;

    optimize_space_value(adaptive_size, min_spacing, max_spacing)
}

/// 生成 CSS 变量映射
pub fn generate_css_variables(theme: &SpaceTheme) -> HashMap<String, String> {
    let mut vars = theme.css_vars.clone();

    // 添加基础变量
    vars.insert(
        "--ant-space-small".to_string(),
        format!("{}px", theme.space_small),
    );
    vars.insert(
        "--ant-space-middle".to_string(),
        format!("{}px", theme.space_middle),
    );
    vars.insert(
        "--ant-space-large".to_string(),
        format!("{}px", theme.space_large),
    );
    vars.insert(
        "--ant-space-compact-border-color".to_string(),
        theme.compact_border_color.clone(),
    );
    vars.insert(
        "--ant-space-compact-border-width".to_string(),
        format!("{}px", theme.compact_border_width),
    );
    vars.insert(
        "--ant-space-compact-border-radius".to_string(),
        format!("{}px", theme.compact_border_radius),
    );

    vars
}

/// 检查是否需要分割元素
pub fn should_show_split(has_split: bool, children_count: usize, current_index: usize) -> bool {
    has_split && children_count > 1 && current_index < children_count - 1
}

/// 获取 Compact 子元素位置
pub fn get_compact_item_position(
    index: usize,
    total: usize,
) -> crate::components::space::styles::CompactItemPosition {
    use crate::components::space::styles::CompactItemPosition;

    if total == 1 {
        CompactItemPosition::Only
    } else if index == 0 {
        CompactItemPosition::First
    } else if index == total - 1 {
        CompactItemPosition::Last
    } else {
        CompactItemPosition::Middle
    }
}

/// 解析尺寸字符串
pub fn parse_size_string(size_str: &str) -> SpaceSize {
    match size_str {
        "small" => SpaceSize::Small,
        "middle" => SpaceSize::Middle,
        "large" => SpaceSize::Large,
        _ => {
            // 尝试解析为数字
            if let Ok(num) = size_str.parse::<u32>() {
                SpaceSize::Custom(num)
            } else {
                SpaceSize::Small // 默认值
            }
        }
    }
}

/// 创建默认的 Space 状态
pub fn create_default_space_state(
    direction: SpaceDirection,
    size: &SpaceSizeConfig,
    theme: &SpaceTheme,
) -> SpaceState {
    let (gap, _) = calculate_space_size(size, direction, theme);

    SpaceState {
        initialized: true,
        current_direction: direction,
        current_gap: gap,
        is_wrapped: false,
        children_count: 0,
    }
}

/// 更新 Space 状态
pub fn update_space_state(
    state: &mut SpaceState,
    direction: SpaceDirection,
    size: &SpaceSizeConfig,
    wrap: bool,
    children_count: usize,
    theme: &SpaceTheme,
) {
    let (gap, _) = calculate_space_size(size, direction, theme);

    state.current_direction = direction;
    state.current_gap = gap;
    state.is_wrapped = wrap;
    state.children_count = children_count;
    state.initialized = true;
}

/// 获取 Space 容器的 CSS 类名
pub fn get_space_container_class(
    direction: SpaceDirection,
    align: SpaceAlign,
    wrap: bool,
    theme: &SpaceTheme,
) -> String {
    let mut classes = vec!["ant-space".to_string()];

    // 添加方向类名
    match direction {
        SpaceDirection::Horizontal => classes.push("ant-space-horizontal".to_string()),
        SpaceDirection::Vertical => classes.push("ant-space-vertical".to_string()),
    }

    // 添加对齐类名
    match align {
        SpaceAlign::Start => classes.push("ant-space-align-start".to_string()),
        SpaceAlign::End => classes.push("ant-space-align-end".to_string()),
        SpaceAlign::Center => classes.push("ant-space-align-center".to_string()),
        SpaceAlign::Baseline => classes.push("ant-space-align-baseline".to_string()),
    }

    // 添加换行类名
    if wrap {
        classes.push("ant-space-wrap".to_string());
    }

    classes.join(" ")
}

/// 获取 Space.Compact 容器类名
pub fn get_space_compact_container_class(
    direction: SpaceDirection,
    size: CompactSize,
    block: bool,
    theme: &SpaceTheme,
) -> String {
    let base_class = "ant-space-compact";
    let direction_class = match direction {
        SpaceDirection::Horizontal => "ant-space-compact-horizontal",
        SpaceDirection::Vertical => "ant-space-compact-vertical",
    };
    let size_class = match size {
        CompactSize::Small => "ant-space-compact-small",
        CompactSize::Middle => "ant-space-compact-middle",
        CompactSize::Large => "ant-space-compact-large",
    };
    let block_class = if block {
        "ant-space-compact-block"
    } else {
        ""
    };
    
    format!(
        "{} {} {} {}",
        base_class, direction_class, size_class, block_class
    )
    .trim()
    .to_string()
}

/// 获取嵌套 Space.Compact 容器类名
pub fn get_nested_space_compact_container_class(
    direction: SpaceDirection,
    size: CompactSize,
    nesting_level: u32,
    theme: &SpaceTheme,
) -> String {
    let base_class = get_space_compact_container_class(direction, size, false, theme);
    let nested_class = format!("ant-space-compact-nested-{}", nesting_level);
    
    format!("{} {}", base_class, nested_class)
}

/// 检测 Space.Compact 组件的嵌套层级
pub fn detect_space_compact_nesting_level() -> u32 {
    // 这里可以通过上下文或其他方式检测嵌套层级
    // 暂时返回默认值，后续可以通过 Context 或其他方式实现
    0
}

/// 验证 Space.Compact 嵌套配置
pub fn validate_space_compact_nesting(
    nesting_level: u32,
    max_nesting_level: u32,
) -> Result<(), String> {
    if nesting_level > max_nesting_level {
        Err(format!(
            "Space.Compact 嵌套层级 {} 超过最大允许层级 {}",
            nesting_level, max_nesting_level
        ))
    } else {
        Ok(())
    }
}

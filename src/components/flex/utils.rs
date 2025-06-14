//! Flex 组件的工具函数模块
//!
//! 提供 Flex 组件相关的工具函数，包括配置创建、验证、计算等功能。

use crate::components::flex::types::*;
use std::collections::HashMap;

/// 创建 Flex 配置
pub fn create_flex_config(props: &FlexProps) -> FlexConfig {
    FlexConfig {
        vertical: props.vertical,
        wrap: props.wrap.clone(),
        justify: props.justify.clone(),
        align: props.align.clone(),
        flex: props.flex.clone(),
        gap: props.gap.clone(),
        component: props.component.to_string(),
        has_children: props.children.is_ok(),
    }
}

/// 验证 Flex Props
pub fn validate_flex_props(props: &FlexProps) -> Result<(), String> {
    // 验证 flex 属性格式
    if let Some(ref flex_value) = props.flex {
        if !is_valid_flex_value(flex_value) {
            return Err(format!("Invalid flex value: {}", flex_value));
        }
    }

    // 验证间距值
    if let Some(ref gap) = props.gap {
        if !is_valid_gap_value(gap) {
            return Err(format!("Invalid gap value: {:?}", gap));
        }
    }

    // 验证组件类型
    if !is_valid_component_type(props.component) {
        return Err(format!("Invalid component type: {}", props.component));
    }

    Ok(())
}

/// 检查是否为有效的 flex 值
fn is_valid_flex_value(value: &str) -> bool {
    // 支持的 flex 值格式:
    // - 数字: "1", "2", "0.5"
    // - 关键字: "auto", "initial", "none"
    // - 复合值: "1 1 auto", "0 0 200px"

    if value.is_empty() {
        return false;
    }

    // 检查关键字
    if matches!(value, "auto" | "initial" | "none" | "inherit" | "unset") {
        return true;
    }

    // 检查数字或复合值
    let parts: Vec<&str> = value.split_whitespace().collect();

    match parts.len() {
        1 => {
            // 单个值：数字或长度单位
            is_valid_number_or_length(parts[0])
        }
        2 => {
            // 两个值：flex-grow flex-shrink
            is_valid_number(parts[0]) && is_valid_number(parts[1])
        }
        3 => {
            // 三个值：flex-grow flex-shrink flex-basis
            is_valid_number(parts[0])
                && is_valid_number(parts[1])
                && is_valid_number_or_length(parts[2])
        }
        _ => false,
    }
}

/// 检查是否为有效的间距值
fn is_valid_gap_value(gap: &FlexGap) -> bool {
    match gap {
        FlexGap::Small | FlexGap::Middle | FlexGap::Large => true,
        FlexGap::Custom(value) => is_valid_length_value(value),
        FlexGap::Array([h, v]) => is_valid_length_value(h) && is_valid_length_value(v),
    }
}

/// 检查是否为有效的组件类型
fn is_valid_component_type(component: &str) -> bool {
    // 允许的 HTML 标签
    matches!(
        component,
        "div"
            | "section"
            | "article"
            | "aside"
            | "header"
            | "footer"
            | "main"
            | "nav"
            | "span"
            | "p"
            | "ul"
            | "ol"
            | "li"
            | "dl"
            | "dt"
            | "dd"
    )
}

/// 检查是否为有效的数字
fn is_valid_number(value: &str) -> bool {
    value.parse::<f32>().is_ok()
}

/// 检查是否为有效的数字或长度单位
fn is_valid_number_or_length(value: &str) -> bool {
    if value == "auto" || value == "content" {
        return true;
    }

    is_valid_length_value(value)
}

/// 检查是否为有效的长度值
fn is_valid_length_value(value: &str) -> bool {
    if value.is_empty() {
        return false;
    }

    // 检查是否以有效的单位结尾
    let valid_units = [
        "px", "em", "rem", "vh", "vw", "vmin", "vmax", "%", "ch", "ex", "cm", "mm", "in", "pt",
        "pc",
    ];

    for unit in &valid_units {
        if value.ends_with(unit) {
            let number_part = &value[..value.len() - unit.len()];
            return number_part.parse::<f32>().is_ok();
        }
    }

    // 检查是否为纯数字 (会被当作 px)
    value.parse::<f32>().is_ok()
}

/// 计算间距值
pub fn calculate_gap_value(gap: &FlexGap, theme: &FlexTheme) -> String {
    match gap {
        FlexGap::Small => theme.gap_small.clone(),
        FlexGap::Middle => theme.gap_middle.clone(),
        FlexGap::Large => theme.gap_large.clone(),
        FlexGap::Custom(value) => value.clone(),
        FlexGap::Array([h, v]) => format!("{} {}", h, v),
    }
}

/// 合并 Flex 主题
pub fn merge_flex_theme(base: &FlexTheme, override_theme: &FlexTheme) -> FlexTheme {
    FlexTheme {
        gap_small: if override_theme.gap_small != base.gap_small {
            override_theme.gap_small.clone()
        } else {
            base.gap_small.clone()
        },
        gap_middle: if override_theme.gap_middle != base.gap_middle {
            override_theme.gap_middle.clone()
        } else {
            base.gap_middle.clone()
        },
        gap_large: if override_theme.gap_large != base.gap_large {
            override_theme.gap_large.clone()
        } else {
            base.gap_large.clone()
        },
        font_size: if override_theme.font_size != base.font_size {
            override_theme.font_size.clone()
        } else {
            base.font_size.clone()
        },
        line_height: if override_theme.line_height != base.line_height {
            override_theme.line_height.clone()
        } else {
            base.line_height.clone()
        },
        color: if override_theme.color != base.color {
            override_theme.color.clone()
        } else {
            base.color.clone()
        },
        background_color: if override_theme.background_color != base.background_color {
            override_theme.background_color.clone()
        } else {
            base.background_color.clone()
        },
        border_color: if override_theme.border_color != base.border_color {
            override_theme.border_color.clone()
        } else {
            base.border_color.clone()
        },
        border_radius: if override_theme.border_radius != base.border_radius {
            override_theme.border_radius.clone()
        } else {
            base.border_radius.clone()
        },
        box_shadow: if override_theme.box_shadow != base.box_shadow {
            override_theme.box_shadow.clone()
        } else {
            base.box_shadow.clone()
        },
        transition: if override_theme.transition != base.transition {
            override_theme.transition.clone()
        } else {
            base.transition.clone()
        },
        breakpoints: {
            let mut merged_breakpoints = base.breakpoints.clone();
            for (key, value) in &override_theme.breakpoints {
                merged_breakpoints.insert(key.clone(), value.clone());
            }
            merged_breakpoints
        },
    }
}

/// 检查是否为响应式间距
pub fn is_responsive_gap(gap: &FlexGap) -> bool {
    matches!(gap, FlexGap::Array(_))
}

/// 获取响应式配置
pub fn get_responsive_config(breakpoints: HashMap<String, FlexConfig>) -> ResponsiveConfig {
    ResponsiveConfig {
        breakpoints,
        current_breakpoint: detect_current_breakpoint(),
        enabled: true,
    }
}

/// 检测当前断点
fn detect_current_breakpoint() -> String {
    // 在实际应用中，这里应该通过 JavaScript 获取窗口宽度
    // 这里返回默认值
    "md".to_string()
}

/// 优化 Flex 性能
pub fn optimize_flex_performance(config: &FlexConfig, item_count: usize) -> PerformanceConfig {
    PerformanceConfig {
        virtualization: item_count > 1000,
        max_render_items: if item_count > 1000 { 100 } else { item_count },
        lazy_loading: item_count > 500,
        memory_limit: calculate_memory_limit(item_count),
    }
}

/// 计算内存限制
fn calculate_memory_limit(item_count: usize) -> usize {
    // 基于项目数量估算内存需求
    let base_memory = 50; // 50MB 基础内存
    let per_item_memory = 0.1; // 每个项目 0.1MB

    (base_memory as f32 + item_count as f32 * per_item_memory) as usize
}

/// 计算 flex-basis 值
pub fn calculate_flex_basis(basis: &str, container_size: f32, item_count: usize) -> String {
    if basis.ends_with('%') {
        basis.to_string()
    } else if basis.ends_with("px") {
        basis.to_string()
    } else if basis == "auto" {
        "auto".to_string()
    } else if basis == "content" {
        "content".to_string()
    } else {
        // 尝试解析为数字，按比例计算
        if let Ok(ratio) = basis.parse::<f32>() {
            let calculated_size = container_size * ratio / item_count as f32;
            format!("{}px", calculated_size)
        } else {
            "auto".to_string()
        }
    }
}

/// 生成 CSS 变量映射
pub fn generate_css_variables(theme: &FlexTheme) -> HashMap<String, String> {
    let mut variables = HashMap::new();

    variables.insert("--ant-flex-gap-small".to_string(), theme.gap_small.clone());
    variables.insert(
        "--ant-flex-gap-middle".to_string(),
        theme.gap_middle.clone(),
    );
    variables.insert("--ant-flex-gap-large".to_string(), theme.gap_large.clone());
    variables.insert("--ant-flex-font-size".to_string(), theme.font_size.clone());
    variables.insert(
        "--ant-flex-line-height".to_string(),
        theme.line_height.clone(),
    );
    variables.insert("--ant-flex-color".to_string(), theme.color.clone());
    variables.insert(
        "--ant-flex-background-color".to_string(),
        theme.background_color.clone(),
    );
    variables.insert(
        "--ant-flex-border-color".to_string(),
        theme.border_color.clone(),
    );
    variables.insert(
        "--ant-flex-border-radius".to_string(),
        theme.border_radius.clone(),
    );
    variables.insert(
        "--ant-flex-box-shadow".to_string(),
        theme.box_shadow.clone(),
    );
    variables.insert(
        "--ant-flex-transition".to_string(),
        theme.transition.clone(),
    );

    variables
}

/// 检查是否应该换行
pub fn should_wrap(config: &FlexConfig) -> bool {
    matches!(config.wrap, FlexWrap::Wrap | FlexWrap::WrapReverse)
}

/// 格式化 flex 值
pub fn format_flex_value(flex: &str) -> String {
    // 标准化 flex 值格式
    let trimmed = flex.trim();

    if trimmed.is_empty() {
        return "0 1 auto".to_string();
    }

    // 处理关键字
    match trimmed {
        "auto" => "1 1 auto".to_string(),
        "initial" => "0 1 auto".to_string(),
        "none" => "0 0 auto".to_string(),
        _ => {
            // 处理数字值
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            match parts.len() {
                1 => {
                    if let Ok(num) = parts[0].parse::<f32>() {
                        format!("{} 1 0%", num)
                    } else {
                        trimmed.to_string()
                    }
                }
                2 => format!("{} {} 0%", parts[0], parts[1]),
                3 => trimmed.to_string(),
                _ => "0 1 auto".to_string(),
            }
        }
    }
}

/// 计算项目尺寸
pub fn calculate_item_size(config: &FlexConfig, container_size: f32, item_count: usize) -> f32 {
    if config.vertical {
        // 垂直布局时计算高度
        if should_wrap(config) {
            container_size / (item_count as f32).sqrt().ceil()
        } else {
            container_size / item_count as f32
        }
    } else {
        // 水平布局时计算宽度
        if should_wrap(config) {
            container_size / (item_count as f32).sqrt().ceil()
        } else {
            container_size / item_count as f32
        }
    }
}

/// 创建默认配置
pub fn create_default_config() -> FlexConfig {
    FlexConfig::default()
}

/// 检查性能模式
pub fn check_performance_mode(item_count: usize) -> bool {
    item_count > 100
}

/// 获取最佳渲染策略
pub fn get_optimal_render_strategy(item_count: usize) -> String {
    if item_count < 50 {
        "direct".to_string()
    } else if item_count < 500 {
        "batched".to_string()
    } else {
        "virtualized".to_string()
    }
}

/// 估算内存使用
pub fn estimate_memory_usage(config: &FlexConfig, item_count: usize) -> usize {
    let base_size = 1024; // 1KB 基础大小
    let per_item_size = if config.has_children { 512 } else { 256 }; // 每个项目的大小

    base_size + per_item_size * item_count
}

/// 生成缓存键
pub fn generate_cache_key(config: &FlexConfig) -> String {
    format!(
        "flex_{}_{}_{}_{}_{}_{}",
        config.vertical,
        config.wrap,
        config.justify,
        config.align,
        config.flex.as_deref().unwrap_or("none"),
        config
            .gap
            .as_ref()
            .map(|g| format!("{:?}", g))
            .unwrap_or_else(|| "none".to_string())
    )
}

/// 计算主题哈希值
pub fn calculate_theme_hash(theme: &FlexTheme) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    theme.gap_small.hash(&mut hasher);
    theme.gap_middle.hash(&mut hasher);
    theme.gap_large.hash(&mut hasher);
    theme.font_size.hash(&mut hasher);
    theme.color.hash(&mut hasher);
    hasher.finish()
}

/// 解析尺寸字符串
pub fn parse_size_string(size: &str) -> Result<(f32, String), String> {
    let size = size.trim();

    if size.is_empty() {
        return Err("Empty size string".to_string());
    }

    // 查找数字部分和单位部分
    let mut number_end = 0;
    for (i, ch) in size.char_indices() {
        if ch.is_ascii_digit() || ch == '.' || ch == '-' {
            number_end = i + 1;
        } else {
            break;
        }
    }

    if number_end == 0 {
        return Err(format!("Invalid size format: {}", size));
    }

    let number_part = &size[..number_end];
    let unit_part = &size[number_end..];

    match number_part.parse::<f32>() {
        Ok(value) => Ok((value, unit_part.to_string())),
        Err(_) => Err(format!("Invalid number: {}", number_part)),
    }
}

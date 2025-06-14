//! Flex 组件的样式生成模块
//!
//! 提供 Flex 组件的 CSS 样式生成功能，包括容器样式、项目样式等。

use crate::components::flex::types::*;
use std::collections::HashMap;

/// 生成 Flex 容器样式
pub fn generate_flex_container_styles(config: &FlexConfig, theme: &FlexTheme) -> String {
    let mut styles = Vec::new();
    
    // 基础 flex 样式
    styles.push("display: flex".to_string());
    
    // 方向
    if config.vertical {
        styles.push("flex-direction: column".to_string());
    } else {
        styles.push("flex-direction: row".to_string());
    }
    
    // 换行
    styles.push(format!("flex-wrap: {}", config.wrap));
    
    // 主轴对齐
    if config.justify != FlexJustify::Normal {
        styles.push(format!("justify-content: {}", config.justify));
    }
    
    // 交叉轴对齐
    if config.align != FlexAlign::Normal {
        styles.push(format!("align-items: {}", config.align));
    }
    
    // flex 属性
    if let Some(ref flex_value) = config.flex {
        styles.push(format!("flex: {}", flex_value));
    }
    
    // 间距
    if let Some(ref gap) = config.gap {
        let gap_value = calculate_gap_value(gap, theme);
        styles.push(format!("gap: {}", gap_value));
    }
    
    // 基础样式
    styles.push(format!("font-size: {}", theme.font_size));
    styles.push(format!("line-height: {}", theme.line_height));
    styles.push(format!("color: {}", theme.color));
    
    // 过渡动画
    styles.push(format!("transition: {}", theme.transition));
    
    styles.join("; ")
}

/// 生成 Flex 项目样式
pub fn generate_flex_item_styles(config: &FlexItemConfig, theme: &FlexTheme) -> String {
    let mut styles = Vec::new();
    
    // flex 属性
    if let Some(ref flex_value) = config.flex {
        styles.push(format!("flex: {}", flex_value));
    } else {
        // 分别设置 flex-grow, flex-shrink, flex-basis
        if let Some(grow) = config.flex_grow {
            styles.push(format!("flex-grow: {}", grow));
        }
        if let Some(shrink) = config.flex_shrink {
            styles.push(format!("flex-shrink: {}", shrink));
        }
        if let Some(ref basis) = config.flex_basis {
            styles.push(format!("flex-basis: {}", basis));
        }
    }
    
    // align-self
    if let Some(ref align_self) = config.align_self {
        styles.push(format!("align-self: {}", align_self));
    }
    
    // order
    if let Some(order) = config.order {
        styles.push(format!("order: {}", order));
    }
    
    // 过渡动画
    styles.push(format!("transition: {}", theme.transition));
    
    styles.join("; ")
}

/// 获取 Flex 容器的 CSS 类名
pub fn get_flex_container_class(config: &FlexConfig) -> String {
    let mut classes = vec!["ant-flex".to_string()];
    
    if config.vertical {
        classes.push("ant-flex-vertical".to_string());
    }
    
    match config.wrap {
        FlexWrap::Wrap => classes.push("ant-flex-wrap".to_string()),
        FlexWrap::WrapReverse => classes.push("ant-flex-wrap-reverse".to_string()),
        _ => {}
    }
    
    match config.justify {
        FlexJustify::Center => classes.push("ant-flex-justify-center".to_string()),
        FlexJustify::FlexEnd => classes.push("ant-flex-justify-end".to_string()),
        FlexJustify::SpaceBetween => classes.push("ant-flex-justify-between".to_string()),
        FlexJustify::SpaceAround => classes.push("ant-flex-justify-around".to_string()),
        FlexJustify::SpaceEvenly => classes.push("ant-flex-justify-evenly".to_string()),
        _ => {}
    }
    
    match config.align {
        FlexAlign::Center => classes.push("ant-flex-align-center".to_string()),
        FlexAlign::FlexEnd => classes.push("ant-flex-align-end".to_string()),
        FlexAlign::Stretch => classes.push("ant-flex-align-stretch".to_string()),
        FlexAlign::Baseline => classes.push("ant-flex-align-baseline".to_string()),
        _ => {}
    }
    
    if config.gap.is_some() {
        classes.push("ant-flex-gap".to_string());
    }
    
    classes.join(" ")
}

/// 获取 Flex 项目的 CSS 类名
pub fn get_flex_item_class(config: &FlexItemConfig) -> String {
    let mut classes = vec!["ant-flex-item".to_string()];
    
    if config.flex.is_some() || config.flex_grow.is_some() || config.flex_shrink.is_some() || config.flex_basis.is_some() {
        classes.push("ant-flex-item-flex".to_string());
    }
    
    if let Some(ref align_self) = config.align_self {
        match align_self {
            FlexAlign::Center => classes.push("ant-flex-item-align-center".to_string()),
            FlexAlign::FlexEnd => classes.push("ant-flex-item-align-end".to_string()),
            FlexAlign::Stretch => classes.push("ant-flex-item-align-stretch".to_string()),
            FlexAlign::Baseline => classes.push("ant-flex-item-align-baseline".to_string()),
            _ => {}
        }
    }
    
    if config.order.is_some() {
        classes.push("ant-flex-item-ordered".to_string());
    }
    
    classes.join(" ")
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

/// 生成响应式样式
pub fn generate_responsive_styles(config: &ResponsiveConfig, theme: &FlexTheme) -> String {
    let mut styles = Vec::new();
    
    for (breakpoint, flex_config) in &config.breakpoints {
        if let Some(media_query) = theme.breakpoints.get(breakpoint) {
            let container_styles = generate_flex_container_styles(flex_config, theme);
            let media_style = format!(
                "@media (min-width: {}) {{ {} }}",
                media_query,
                container_styles
            );
            styles.push(media_style);
        }
    }
    
    styles.join("\n")
}

/// 生成 CSS 变量映射
pub fn generate_css_variables(theme: &FlexTheme) -> HashMap<String, String> {
    let mut variables = HashMap::new();
    
    variables.insert("--ant-flex-gap-small".to_string(), theme.gap_small.clone());
    variables.insert("--ant-flex-gap-middle".to_string(), theme.gap_middle.clone());
    variables.insert("--ant-flex-gap-large".to_string(), theme.gap_large.clone());
    variables.insert("--ant-flex-font-size".to_string(), theme.font_size.clone());
    variables.insert("--ant-flex-line-height".to_string(), theme.line_height.clone());
    variables.insert("--ant-flex-color".to_string(), theme.color.clone());
    variables.insert("--ant-flex-background-color".to_string(), theme.background_color.clone());
    variables.insert("--ant-flex-border-color".to_string(), theme.border_color.clone());
    variables.insert("--ant-flex-border-radius".to_string(), theme.border_radius.clone());
    variables.insert("--ant-flex-box-shadow".to_string(), theme.box_shadow.clone());
    variables.insert("--ant-flex-transition".to_string(), theme.transition.clone());
    
    variables
}

/// 检查是否应该换行
pub fn should_wrap(config: &FlexConfig) -> bool {
    matches!(config.wrap, FlexWrap::Wrap | FlexWrap::WrapReverse)
}

/// 获取 flex 方向
pub fn get_flex_direction(vertical: bool) -> &'static str {
    if vertical {
        "column"
    } else {
        "row"
    }
}

/// 获取主轴属性名
pub fn get_main_axis_property(vertical: bool) -> &'static str {
    if vertical {
        "height"
    } else {
        "width"
    }
}

/// 获取交叉轴属性名
pub fn get_cross_axis_property(vertical: bool) -> &'static str {
    if vertical {
        "width"
    } else {
        "height"
    }
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

/// 生成网格样式 (用于 FlexGrid 组件)
pub fn generate_grid_styles(cols: usize, gap: &FlexGap, theme: &FlexTheme) -> String {
    let gap_value = calculate_gap_value(gap, theme);
    let flex_basis = format!("calc({}% - {})", 100.0 / cols as f32, gap_value);
    
    format!(
        "display: flex; flex-wrap: wrap; gap: {}; > * {{ flex: 0 0 {}; }}",
        gap_value,
        flex_basis
    )
}

/// 生成布局样式 (用于 FlexLayout 组件)
pub fn generate_layout_styles(areas: &[&str], theme: &FlexTheme) -> String {
    let mut styles = Vec::new();
    
    styles.push("display: flex".to_string());
    styles.push("flex-direction: column".to_string());
    styles.push("min-height: 100vh".to_string());
    
    for (index, area) in areas.iter().enumerate() {
        let area_style = match *area {
            "header" => "flex: 0 0 auto",
            "footer" => "flex: 0 0 auto",
            "content" => "flex: 1 1 auto",
            "sidebar" => "flex: 0 0 200px",
            _ => "flex: 0 0 auto",
        };
        
        styles.push(format!("> :nth-child({}) {{ {} }}", index + 1, area_style));
    }
    
    styles.push(format!("gap: {}", theme.gap_middle));
    
    styles.join("; ")
}

/// 获取性能优化的样式
pub fn get_performance_optimized_styles(config: &PerformanceConfig) -> String {
    let mut styles = Vec::new();
    
    if config.virtualization {
        styles.push("contain: layout style paint".to_string());
        styles.push("will-change: transform".to_string());
    }
    
    if config.lazy_loading {
        styles.push("content-visibility: auto".to_string());
        styles.push("contain-intrinsic-size: 200px".to_string());
    }
    
    styles.join("; ")
}

/// 生成暗色主题样式
pub fn generate_dark_theme_styles(theme: &FlexTheme) -> FlexTheme {
    FlexTheme {
        color: "rgba(255, 255, 255, 0.88)".to_string(),
        background_color: "#141414".to_string(),
        border_color: "#424242".to_string(),
        box_shadow: "0 2px 8px rgba(0, 0, 0, 0.45)".to_string(),
        ..theme.clone()
    }
}

/// 生成紧凑主题样式
pub fn generate_compact_theme_styles(theme: &FlexTheme) -> FlexTheme {
    FlexTheme {
        gap_small: "4px".to_string(),
        gap_middle: "8px".to_string(),
        gap_large: "12px".to_string(),
        font_size: "12px".to_string(),
        line_height: "1.5".to_string(),
        ..theme.clone()
    }
}
//! Divider 组件的工具函数

use crate::components::divider::types::*;
use std::collections::HashMap;

/// 创建 Divider 配置
pub fn create_divider_config(
    r#type: DividerType,
    orientation: DividerOrientation,
    variant: DividerVariant,
    plain: bool,
    size: DividerSize,
    has_text: bool,
) -> DividerConfig {
    DividerConfig {
        r#type,
        orientation,
        orientation_margin: None,
        variant,
        plain,
        size,
        has_text,
        class: None,
        style: None,
    }
}

/// 验证 Divider Props
pub fn validate_divider_props(props: &DividerProps) -> Result<(), String> {
    // 垂直分割线不应该有文本内容
    if props.r#type == DividerType::Vertical && props.children.is_some() {
        return Err("Vertical divider should not have text content".to_string());
    }

    // 验证文本边距格式
    if let Some(ref margin) = props.orientation_margin {
        if !is_valid_margin(margin) {
            return Err(format!("Invalid orientation margin: {}", margin));
        }
    }

    Ok(())
}

/// 检查边距格式是否有效
pub fn is_valid_margin(margin: &str) -> bool {
    // 支持的格式：数字、数字+px、数字+em、数字+rem、数字+%
    let margin = margin.trim();

    if margin.is_empty() {
        return false;
    }

    // 纯数字
    if margin.parse::<f64>().is_ok() {
        return true;
    }

    // 数字+单位
    let units = [
        "px", "em", "rem", "%", "vh", "vw", "pt", "pc", "in", "cm", "mm",
    ];
    for unit in &units {
        if margin.ends_with(unit) {
            let number_part = &margin[..margin.len() - unit.len()];
            if number_part.parse::<f64>().is_ok() {
                return true;
            }
        }
    }

    false
}

/// 计算文本边距
pub fn calculate_text_margin(
    orientation: &DividerOrientation,
    custom_margin: Option<&String>,
    size: &DividerSize,
) -> String {
    if let Some(margin) = custom_margin {
        return margin.clone();
    }

    let base_margin = match size {
        DividerSize::Small => 12,
        DividerSize::Middle => 16,
        DividerSize::Large => 20,
    };

    match orientation {
        DividerOrientation::Left | DividerOrientation::Start => {
            format!("0 {}px 0 0", base_margin)
        }
        DividerOrientation::Right | DividerOrientation::End => {
            format!("0 0 0 {}px", base_margin)
        }
        DividerOrientation::Center => {
            format!("0 {}px", base_margin)
        }
    }
}

/// 判断是否为 RTL 布局
pub fn is_rtl_layout() -> bool {
    // 在实际应用中，这里应该从上下文或全局状态获取
    // 目前返回 false 作为默认值
    false
}

/// 获取实际的文本对齐方式（考虑 RTL）
pub fn get_actual_orientation(orientation: &DividerOrientation) -> DividerOrientation {
    if is_rtl_layout() {
        match orientation {
            DividerOrientation::Start => DividerOrientation::Right,
            DividerOrientation::End => DividerOrientation::Left,
            _ => orientation.clone(),
        }
    } else {
        match orientation {
            DividerOrientation::Start => DividerOrientation::Left,
            DividerOrientation::End => DividerOrientation::Right,
            _ => orientation.clone(),
        }
    }
}

/// 生成 CSS 变量映射
pub fn generate_css_variables(theme: &DividerTheme) -> HashMap<String, String> {
    let mut variables = HashMap::new();

    variables.insert(
        "--divider-color-split".to_string(),
        theme.color_split.clone(),
    );
    variables.insert("--divider-color-text".to_string(), theme.color_text.clone());
    variables.insert(
        "--divider-color-text-secondary".to_string(),
        theme.color_text_secondary.clone(),
    );
    variables.insert("--divider-font-size".to_string(), theme.font_size.clone());
    variables.insert(
        "--divider-font-size-sm".to_string(),
        theme.font_size_sm.clone(),
    );
    variables.insert(
        "--divider-font-size-lg".to_string(),
        theme.font_size_lg.clone(),
    );
    variables.insert(
        "--divider-line-height".to_string(),
        theme.line_height.clone(),
    );
    variables.insert("--divider-margin".to_string(), theme.margin.clone());
    variables.insert("--divider-margin-sm".to_string(), theme.margin_sm.clone());
    variables.insert("--divider-margin-lg".to_string(), theme.margin_lg.clone());
    variables.insert(
        "--divider-text-padding".to_string(),
        theme.text_padding.clone(),
    );
    variables.insert(
        "--divider-vertical-height".to_string(),
        theme.vertical_height.clone(),
    );
    variables.insert(
        "--divider-vertical-margin".to_string(),
        theme.vertical_margin.clone(),
    );
    variables.insert(
        "--divider-border-width".to_string(),
        theme.border_width.clone(),
    );
    variables.insert("--divider-transition".to_string(), theme.transition.clone());

    variables
}

/// 合并主题配置
pub fn merge_theme(base: &DividerTheme, override_theme: &DividerTheme) -> DividerTheme {
    DividerTheme {
        color_split: if override_theme.color_split.is_empty() {
            base.color_split.clone()
        } else {
            override_theme.color_split.clone()
        },
        color_text: if override_theme.color_text.is_empty() {
            base.color_text.clone()
        } else {
            override_theme.color_text.clone()
        },
        color_text_secondary: if override_theme.color_text_secondary.is_empty() {
            base.color_text_secondary.clone()
        } else {
            override_theme.color_text_secondary.clone()
        },
        font_size: if override_theme.font_size.is_empty() {
            base.font_size.clone()
        } else {
            override_theme.font_size.clone()
        },
        font_size_sm: if override_theme.font_size_sm.is_empty() {
            base.font_size_sm.clone()
        } else {
            override_theme.font_size_sm.clone()
        },
        font_size_lg: if override_theme.font_size_lg.is_empty() {
            base.font_size_lg.clone()
        } else {
            override_theme.font_size_lg.clone()
        },
        line_height: if override_theme.line_height.is_empty() {
            base.line_height.clone()
        } else {
            override_theme.line_height.clone()
        },
        margin: if override_theme.margin.is_empty() {
            base.margin.clone()
        } else {
            override_theme.margin.clone()
        },
        margin_sm: if override_theme.margin_sm.is_empty() {
            base.margin_sm.clone()
        } else {
            override_theme.margin_sm.clone()
        },
        margin_lg: if override_theme.margin_lg.is_empty() {
            base.margin_lg.clone()
        } else {
            override_theme.margin_lg.clone()
        },
        text_padding: if override_theme.text_padding.is_empty() {
            base.text_padding.clone()
        } else {
            override_theme.text_padding.clone()
        },
        vertical_height: if override_theme.vertical_height.is_empty() {
            base.vertical_height.clone()
        } else {
            override_theme.vertical_height.clone()
        },
        vertical_margin: if override_theme.vertical_margin.is_empty() {
            base.vertical_margin.clone()
        } else {
            override_theme.vertical_margin.clone()
        },
        border_width: if override_theme.border_width.is_empty() {
            base.border_width.clone()
        } else {
            override_theme.border_width.clone()
        },
        transition: if override_theme.transition.is_empty() {
            base.transition.clone()
        } else {
            override_theme.transition.clone()
        },
    }
}

/// 应用暗色主题
pub fn apply_dark_theme(base: &DividerTheme, dark: &DividerDarkTheme) -> DividerTheme {
    DividerTheme {
        color_split: dark.color_split.clone(),
        color_text: dark.color_text.clone(),
        color_text_secondary: dark.color_text_secondary.clone(),
        ..base.clone()
    }
}

/// 应用紧凑主题
pub fn apply_compact_theme(base: &DividerTheme, compact: &DividerCompactTheme) -> DividerTheme {
    DividerTheme {
        margin: compact.margin.clone(),
        margin_sm: compact.margin_sm.clone(),
        margin_lg: compact.margin_lg.clone(),
        text_padding: compact.text_padding.clone(),
        vertical_margin: compact.vertical_margin.clone(),
        ..base.clone()
    }
}

/// 计算分割线厚度
pub fn calculate_divider_thickness(size: &DividerSize, variant: &DividerVariant) -> String {
    let base_thickness = match size {
        DividerSize::Small => 0.5,
        DividerSize::Middle => 1.0,
        DividerSize::Large => 2.0,
    };

    let thickness = match variant {
        DividerVariant::Dotted => base_thickness * 1.5, // 点线需要更粗一些
        _ => base_thickness,
    };

    format!("{}px", thickness)
}

/// 生成分割线样式
pub fn generate_divider_style(theme: &DividerTheme) -> String {
    // 生成基础样式
    let styles = vec![
        "box-sizing: border-box",
        "margin: 0",
        "padding: 0",
        "color: rgba(0, 0, 0, 0.88)",
        "font-size: 14px",
        "line-height: 1.5714285714285714",
        "list-style: none",
        "font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Noto Sans', sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol', 'Noto Color Emoji'",
    ];

    styles.join("; ")
}

/// 生成文本样式字符串
pub fn generate_text_style(config: &DividerConfig, theme: &DividerTheme) -> String {
    let mut styles = Vec::new();

    // 字体大小
    let font_size = match config.size {
        DividerSize::Small => &theme.font_size_sm,
        DividerSize::Middle => &theme.font_size,
        DividerSize::Large => &theme.font_size_lg,
    };
    styles.push(format!("font-size: {}", font_size));

    // 文本颜色
    let color = if config.plain {
        &theme.color_text_secondary
    } else {
        &theme.color_text
    };
    styles.push(format!("color: {}", color));

    // 行高
    styles.push(format!("line-height: {}", theme.line_height));

    // 内边距
    let padding = calculate_text_margin(
        &config.orientation,
        config.orientation_margin.as_ref(),
        &config.size,
    );
    styles.push(format!("padding: {}", padding));

    styles.join("; ")
}

/// 解析尺寸字符串
pub fn parse_size_string(size_str: &str) -> Option<DividerSize> {
    match size_str.to_lowercase().as_str() {
        "small" | "sm" => Some(DividerSize::Small),
        "middle" | "medium" | "md" => Some(DividerSize::Middle),
        "large" | "lg" => Some(DividerSize::Large),
        _ => None,
    }
}

/// 解析类型字符串
pub fn parse_type_string(type_str: &str) -> Option<DividerType> {
    match type_str.to_lowercase().as_str() {
        "horizontal" | "h" => Some(DividerType::Horizontal),
        "vertical" | "v" => Some(DividerType::Vertical),
        _ => None,
    }
}

/// 解析方向字符串
pub fn parse_orientation_string(orientation_str: &str) -> Option<DividerOrientation> {
    match orientation_str.to_lowercase().as_str() {
        "left" | "l" => Some(DividerOrientation::Left),
        "right" | "r" => Some(DividerOrientation::Right),
        "center" | "c" => Some(DividerOrientation::Center),
        "start" | "s" => Some(DividerOrientation::Start),
        "end" | "e" => Some(DividerOrientation::End),
        _ => None,
    }
}

/// 解析变体字符串
pub fn parse_variant_string(variant_str: &str) -> Option<DividerVariant> {
    match variant_str.to_lowercase().as_str() {
        "solid" => Some(DividerVariant::Solid),
        "dashed" | "dash" => Some(DividerVariant::Dashed),
        "dotted" | "dot" => Some(DividerVariant::Dotted),
        _ => None,
    }
}

/// 生成主题哈希
pub fn generate_theme_hash(theme: &DividerTheme) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    theme.color_split.hash(&mut hasher);
    theme.color_text.hash(&mut hasher);
    theme.color_text_secondary.hash(&mut hasher);
    theme.font_size.hash(&mut hasher);
    theme.margin.hash(&mut hasher);
    hasher.finish()
}

/// 检查是否为有效的 CSS 颜色值
pub fn is_valid_css_color(color: &str) -> bool {
    let color = color.trim();

    // 十六进制颜色
    if color.starts_with('#') {
        let hex = &color[1..];
        return hex.len() == 3 || hex.len() == 6 || hex.len() == 8;
    }

    // RGB/RGBA
    if color.starts_with("rgb(") || color.starts_with("rgba(") {
        return true;
    }

    // HSL/HSLA
    if color.starts_with("hsl(") || color.starts_with("hsla(") {
        return true;
    }

    // CSS 颜色关键字
    let css_colors = [
        "transparent",
        "black",
        "white",
        "red",
        "green",
        "blue",
        "yellow",
        "cyan",
        "magenta",
        "gray",
        "grey",
        "orange",
        "purple",
        "pink",
        "brown",
        "navy",
        "teal",
        "olive",
    ];

    css_colors.contains(&color.to_lowercase().as_str())
}

/// 获取默认配置
pub fn get_default_config() -> DividerConfig {
    DividerConfig::default()
}

/// 创建水平分割线配置
pub fn horizontal_divider() -> DividerConfig {
    DividerConfig {
        r#type: DividerType::Horizontal,
        ..Default::default()
    }
}

/// 创建垂直分割线配置
pub fn vertical_divider() -> DividerConfig {
    DividerConfig {
        r#type: DividerType::Vertical,
        ..Default::default()
    }
}

/// 创建虚线分割线配置
pub fn dashed_divider() -> DividerConfig {
    DividerConfig {
        variant: DividerVariant::Dashed,
        ..Default::default()
    }
}

/// 创建点线分割线配置
pub fn dotted_divider() -> DividerConfig {
    DividerConfig {
        variant: DividerVariant::Dotted,
        ..Default::default()
    }
}

/// 创建带文本的分割线配置
pub fn text_divider(orientation: DividerOrientation) -> DividerConfig {
    DividerConfig {
        orientation,
        has_text: true,
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_margin() {
        assert!(is_valid_margin("10px"));
        assert!(is_valid_margin("1.5em"));
        assert!(is_valid_margin("100%"));
        assert!(is_valid_margin("0"));
        assert!(!is_valid_margin(""));
        assert!(!is_valid_margin("invalid"));
    }

    #[test]
    fn test_calculate_text_margin() {
        let margin = calculate_text_margin(&DividerOrientation::Center, None, &DividerSize::Middle);
        assert_eq!(margin, "0 16px");

        let margin = calculate_text_margin(&DividerOrientation::Left, None, &DividerSize::Small);
        assert_eq!(margin, "0 12px 0 0");
    }

    #[test]
    fn test_parse_size_string() {
        assert_eq!(parse_size_string("small"), Some(DividerSize::Small));
        assert_eq!(parse_size_string("LARGE"), Some(DividerSize::Large));
        assert_eq!(parse_size_string("invalid"), None);
    }

    #[test]
    fn test_is_valid_css_color() {
        assert!(is_valid_css_color("#ff0000"));
        assert!(is_valid_css_color("rgb(255, 0, 0)"));
        assert!(is_valid_css_color("red"));
        assert!(!is_valid_css_color("invalid-color"));
    }
}

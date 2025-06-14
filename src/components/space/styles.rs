//! Space 组件样式生成
//!
//! 提供 Space 组件的 CSS 样式生成功能，包括基础样式、主题样式和响应式样式。

use crate::components::space::types::*;
use std::collections::HashMap;

/// 生成 Space 容器样式
pub fn generate_space_container_styles(
    direction: SpaceDirection,
    align: SpaceAlign,
    wrap: bool,
    gap: u32,
    theme: &SpaceTheme,
) -> String {
    let flex_direction = match direction {
        SpaceDirection::Horizontal => "row",
        SpaceDirection::Vertical => "column",
    };
    
    let align_items = match align {
        SpaceAlign::Start => "flex-start",
        SpaceAlign::End => "flex-end",
        SpaceAlign::Center => "center",
        SpaceAlign::Baseline => "baseline",
    };
    
    let flex_wrap = if wrap && direction == SpaceDirection::Horizontal {
        "wrap"
    } else {
        "nowrap"
    };
    
    format!(
        r#"
        display: flex;
        flex-direction: {};
        align-items: {};
        flex-wrap: {};
        gap: {}px;
        box-sizing: border-box;
        "#,
        flex_direction, align_items, flex_wrap, gap
    )
}

/// 生成 Space 子元素包装器样式
pub fn generate_space_item_styles(
    direction: SpaceDirection,
    is_last: bool,
) -> String {
    let base_styles = r#"
        display: inline-flex;
        flex-shrink: 0;
        box-sizing: border-box;
    "#;
    
    // 如果不是最后一个元素且不使用 gap，则添加边距
    if !is_last {
        match direction {
            SpaceDirection::Horizontal => {
                format!("{}", base_styles)
            }
            SpaceDirection::Vertical => {
                format!("{}", base_styles)
            }
        }
    } else {
        base_styles.to_string()
    }
}

/// 生成 Space.Compact 容器样式
pub fn generate_space_compact_container_styles(
    direction: SpaceDirection,
    block: bool,
    size: CompactSize,
    theme: &SpaceTheme,
) -> String {
    let flex_direction = match direction {
        SpaceDirection::Horizontal => "row",
        SpaceDirection::Vertical => "column",
    };
    
    let display = if block { "flex" } else { "inline-flex" };
    let width = if block { "100%" } else { "auto" };
    
    format!(
        r#"
        display: {};
        flex-direction: {};
        width: {};
        border-radius: {}px;
        box-sizing: border-box;
        position: relative;
        "#,
        display, flex_direction, width, theme.compact_border_radius
    )
}

/// 生成嵌套 Space.Compact 容器样式
pub fn generate_nested_space_compact_styles(
    direction: SpaceDirection,
    nesting_level: u32,
    theme: &SpaceTheme,
) -> String {
    let flex_direction = match direction {
        SpaceDirection::Horizontal => "row",
        SpaceDirection::Vertical => "column",
    };
    
    // 根据嵌套层级调整样式
    let z_index = 100 + nesting_level;
    let border_radius = theme.compact_border_radius.saturating_sub(nesting_level);
    
    format!(
        r#"
        display: inline-flex;
        flex-direction: {};
        border-radius: {}px;
        box-sizing: border-box;
        position: relative;
        z-index: {};
        "#,
        flex_direction, border_radius, z_index
    )
}

/// 生成 Space.Compact 子元素样式
pub fn generate_space_compact_item_styles(
    direction: SpaceDirection,
    position: CompactItemPosition,
    size: CompactSize,
    theme: &SpaceTheme,
) -> String {
    let border_radius = match (direction, position) {
        (SpaceDirection::Horizontal, CompactItemPosition::First) => {
            format!("{}px 0 0 {}px", theme.compact_border_radius, theme.compact_border_radius)
        }
        (SpaceDirection::Horizontal, CompactItemPosition::Last) => {
            format!("0 {}px {}px 0", theme.compact_border_radius, theme.compact_border_radius)
        }
        (SpaceDirection::Horizontal, CompactItemPosition::Middle) => "0".to_string(),
        (SpaceDirection::Horizontal, CompactItemPosition::Only) => {
            format!("{}px", theme.compact_border_radius)
        }
        (SpaceDirection::Vertical, CompactItemPosition::First) => {
            format!("{}px {}px 0 0", theme.compact_border_radius, theme.compact_border_radius)
        }
        (SpaceDirection::Vertical, CompactItemPosition::Last) => {
            format!("0 0 {}px {}px", theme.compact_border_radius, theme.compact_border_radius)
        }
        (SpaceDirection::Vertical, CompactItemPosition::Middle) => "0".to_string(),
        (SpaceDirection::Vertical, CompactItemPosition::Only) => {
            format!("{}px", theme.compact_border_radius)
        }
    };
    
    let border_style = match position {
        CompactItemPosition::First => {
            if direction == SpaceDirection::Horizontal {
                format!("border-right: 0; border-radius: {};", border_radius)
            } else {
                format!("border-bottom: 0; border-radius: {};", border_radius)
            }
        }
        CompactItemPosition::Middle => {
            if direction == SpaceDirection::Horizontal {
                "border-left: 0; border-right: 0; border-radius: 0;".to_string()
            } else {
                "border-top: 0; border-bottom: 0; border-radius: 0;".to_string()
            }
        }
        CompactItemPosition::Last => {
            if direction == SpaceDirection::Horizontal {
                format!("border-left: 0; border-radius: {};", border_radius)
            } else {
                format!("border-top: 0; border-radius: {};", border_radius)
            }
        }
        CompactItemPosition::Only => {
            format!("border-radius: {};", border_radius)
        }
    };
    
    format!(
        r#"
        flex-shrink: 0;
        {};
        box-sizing: border-box;
        "#,
        border_style
    )
}

/// 生成分割元素样式
pub fn generate_space_split_styles(
    direction: SpaceDirection,
    theme: &SpaceTheme,
) -> String {
    match direction {
        SpaceDirection::Horizontal => {
            format!(
                r#"
                display: inline-flex;
                align-items: center;
                margin: 0 {}px;
                "#,
                theme.space_small / 2
            )
        }
        SpaceDirection::Vertical => {
            format!(
                r#"
                display: flex;
                justify-content: center;
                margin: {}px 0;
                "#,
                theme.space_small / 2
            )
        }
    }
}

/// 生成响应式样式
pub fn generate_space_responsive_styles(
    breakpoints: &HashMap<String, u32>,
    theme: &SpaceTheme,
) -> String {
    let mut styles = String::new();
    
    for (breakpoint, width) in breakpoints {
        styles.push_str(&format!(
            r#"
            @media (max-width: {}px) {{
                .ant-space {{
                    gap: {}px;
                }}
                .ant-space-compact {{
                    flex-direction: column;
                }}
            }}
            "#,
            width, theme.space_small
        ));
    }
    
    styles
}

/// 获取 Space CSS 类名
pub fn get_space_class_name(
    direction: SpaceDirection,
    align: SpaceAlign,
    size: &SpaceSizeConfig,
    wrap: bool,
    custom_class: Option<&str>,
) -> String {
    let mut classes = vec!["ant-space".to_string()];
    
    // 方向类名
    classes.push(format!("ant-space-{}", direction));
    
    // 对齐类名
    if align != SpaceAlign::Start {
        classes.push(format!("ant-space-align-{}", align));
    }
    
    // 尺寸类名
    match size {
        SpaceSizeConfig::Single(space_size) => {
            match space_size {
                SpaceSize::Small => classes.push("ant-space-small".to_string()),
                SpaceSize::Middle => classes.push("ant-space-middle".to_string()),
                SpaceSize::Large => classes.push("ant-space-large".to_string()),
                SpaceSize::Custom(_) => classes.push("ant-space-custom".to_string()),
            }
        }
        SpaceSizeConfig::Array(_) => classes.push("ant-space-array".to_string()),
        SpaceSizeConfig::String(s) => classes.push(format!("ant-space-{}", s)),
    }
    
    // 换行类名
    if wrap {
        classes.push("ant-space-wrap".to_string());
    }
    
    // 自定义类名
    if let Some(custom) = custom_class {
        classes.push(custom.to_string());
    }
    
    classes.join(" ")
}

/// 获取 Space.Compact CSS 类名
pub fn get_space_compact_class_name(
    direction: SpaceDirection,
    size: CompactSize,
    block: bool,
    custom_class: Option<&str>,
) -> String {
    let mut classes = vec!["ant-space-compact".to_string()];
    
    // 方向类名
    classes.push(format!("ant-space-compact-{}", direction));
    
    // 尺寸类名
    classes.push(format!("ant-space-compact-{}", size));
    
    // 块级类名
    if block {
        classes.push("ant-space-compact-block".to_string());
    }
    
    // 自定义类名
    if let Some(custom) = custom_class {
        classes.push(custom.to_string());
    }
    
    classes.join(" ")
}

/// 生成完整的 Space 样式表
pub fn generate_complete_space_styles(theme: &SpaceTheme) -> String {
    format!(
        r#"
        /* Space 基础样式 */
        .ant-space {{
            display: inline-flex;
            box-sizing: border-box;
        }}
        
        .ant-space-horizontal {{
            flex-direction: row;
        }}
        
        .ant-space-vertical {{
            flex-direction: column;
        }}
        
        .ant-space-align-start {{
            align-items: flex-start;
        }}
        
        .ant-space-align-end {{
            align-items: flex-end;
        }}
        
        .ant-space-align-center {{
            align-items: center;
        }}
        
        .ant-space-align-baseline {{
            align-items: baseline;
        }}
        
        .ant-space-wrap {{
            flex-wrap: wrap;
        }}
        
        .ant-space-small {{
            gap: {}px;
        }}
        
        .ant-space-middle {{
            gap: {}px;
        }}
        
        .ant-space-large {{
            gap: {}px;
        }}
        
        .ant-space-item {{
            display: inline-flex;
            flex-shrink: 0;
        }}
        
        /* Space.Compact 样式 */
        .ant-space-compact {{
            display: inline-flex;
            border-radius: {}px;
        }}
        
        .ant-space-compact-block {{
            display: flex;
            width: 100%;
        }}
        
        .ant-space-compact-horizontal {{
            flex-direction: row;
        }}
        
        .ant-space-compact-vertical {{
            flex-direction: column;
        }}
        
        .ant-space-compact-item {{
            flex-shrink: 0;
            border: {}px solid {};
        }}
        
        .ant-space-compact-item:not(:first-child) {{
            margin-left: -{}px;
        }}
        
        .ant-space-compact-vertical .ant-space-compact-item:not(:first-child) {{
            margin-left: 0;
            margin-top: -{}px;
        }}
        
        .ant-space-compact-item:first-child {{
            border-top-left-radius: {}px;
            border-bottom-left-radius: {}px;
        }}
        
        .ant-space-compact-item:last-child {{
            border-top-right-radius: {}px;
            border-bottom-right-radius: {}px;
        }}
        
        .ant-space-compact-vertical .ant-space-compact-item:first-child {{
            border-top-left-radius: {}px;
            border-top-right-radius: {}px;
            border-bottom-left-radius: 0;
        }}
        
        .ant-space-compact-vertical .ant-space-compact-item:last-child {{
            border-bottom-left-radius: {}px;
            border-bottom-right-radius: {}px;
            border-top-right-radius: 0;
        }}
        
        /* 分割线样式 */
        .ant-space-split {{
            display: inline-flex;
            align-items: center;
        }}
        
        .ant-space-vertical .ant-space-split {{
            justify-content: center;
        }}
        "#,
        theme.space_small,
        theme.space_middle,
        theme.space_large,
        theme.compact_border_radius,
        theme.compact_border_width,
        theme.compact_border_color,
        theme.compact_border_width,
        theme.compact_border_width,
        theme.compact_border_radius,
        theme.compact_border_radius,
        theme.compact_border_radius,
        theme.compact_border_radius,
        theme.compact_border_radius,
        theme.compact_border_radius,
        theme.compact_border_radius,
        theme.compact_border_radius,
    )
}

/// Space.Compact 子元素位置枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompactItemPosition {
    /// 第一个元素
    First,
    /// 中间元素
    Middle,
    /// 最后一个元素
    Last,
    /// 唯一元素
    Only,
}

/// 生成 Space 样式
pub fn generate_space_styles(
    direction: SpaceDirection,
    align: SpaceAlign,
    size: &SpaceSizeConfig,
    wrap: bool,
    theme: &SpaceTheme,
) -> String {
    let gap = calculate_gap_value(size, theme);
    generate_space_container_styles(direction, align, wrap, gap, theme)
}

/// 生成 Space.Compact 样式
pub fn generate_space_compact_styles(
    direction: SpaceDirection,
    block: bool,
    size: CompactSize,
    theme: &SpaceTheme,
) -> String {
    generate_space_compact_container_styles(direction, block, size, theme)
}

/// 计算间距值
fn calculate_gap_value(size: &SpaceSizeConfig, theme: &SpaceTheme) -> u32 {
    match size {
        SpaceSizeConfig::Single(space_size) => match space_size {
            SpaceSize::Small => theme.space_small,
            SpaceSize::Middle => theme.space_middle,
            SpaceSize::Large => theme.space_large,
            SpaceSize::Custom(value) => *value,
        },
        SpaceSizeConfig::Array(values) => values[0], // 使用水平间距作为默认值
        SpaceSizeConfig::String(s) => {
            match s.as_str() {
                "small" => theme.space_small,
                "middle" => theme.space_middle,
                "large" => theme.space_large,
                _ => theme.space_small,
            }
        }
    }
}
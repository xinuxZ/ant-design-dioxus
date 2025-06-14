use super::types::*;

/// 计算头像属性
/// 根据是否有标题和段落自动调整头像样式
pub fn calculate_avatar_props(
    avatar_config: &SkeletonAvatarConfig,
    has_title: bool,
    has_paragraph: bool,
) -> SkeletonAvatarProps {
    let base_props = match avatar_config {
        SkeletonAvatarConfig::Boolean(_) => SkeletonAvatarProps {
            active: None,
            shape: None,
            size: None,
        },
        SkeletonAvatarConfig::Config(props) => props.clone(),
    };

    // 根据布局自动调整头像属性
    let auto_shape = if has_title && !has_paragraph {
        // 只有标题时使用方形头像
        Some(AvatarShape::Square)
    } else {
        // 其他情况使用圆形头像
        Some(AvatarShape::Circle)
    };

    let auto_size = Some(AvatarSize::Large);

    SkeletonAvatarProps {
        active: base_props.active,
        shape: base_props.shape.or(auto_shape),
        size: base_props.size.or(auto_size),
    }
}

/// 计算标题属性
/// 根据是否有头像和段落自动调整标题宽度
pub fn calculate_title_props(
    title_config: &SkeletonTitleConfig,
    has_avatar: bool,
    has_paragraph: bool,
) -> SkeletonTitleProps {
    let base_props = match title_config {
        SkeletonTitleConfig::Boolean(_) => SkeletonTitleProps {
            width: None,
        },
        SkeletonTitleConfig::Config(props) => props.clone(),
    };

    // 根据布局自动调整标题宽度
    let auto_width = if !has_avatar && has_paragraph {
        // 没有头像但有段落时，标题宽度为 38%
        Some(SkeletonWidth::Percentage(38))
    } else if has_avatar && has_paragraph {
        // 有头像和段落时，标题宽度为 50%
        Some(SkeletonWidth::Percentage(50))
    } else {
        // 其他情况使用默认宽度
        None
    };

    SkeletonTitleProps {
        width: base_props.width.or(auto_width),
    }
}

/// 计算段落属性
/// 根据是否有头像和标题自动调整段落样式
pub fn calculate_paragraph_props(
    paragraph_config: &SkeletonParagraphConfig,
    has_avatar: bool,
    has_title: bool,
) -> SkeletonParagraphProps {
    let base_props = match paragraph_config {
        SkeletonParagraphConfig::Boolean(_) => SkeletonParagraphProps {
            rows: None,
            width: None,
        },
        SkeletonParagraphConfig::Config(props) => props.clone(),
    };

    // 根据布局自动调整段落属性
    let auto_rows = if !has_avatar && has_title {
        // 没有头像但有标题时，显示 3 行
        Some(3)
    } else {
        // 其他情况显示 2 行
        Some(2)
    };

    let auto_width = if !has_avatar || !has_title {
        // 没有头像或标题时，最后一行宽度为 61%
        Some(SkeletonWidthConfig::Single(SkeletonWidth::Percentage(61)))
    } else {
        None
    };

    SkeletonParagraphProps {
        rows: base_props.rows.or(auto_rows),
        width: base_props.width.or(auto_width),
    }
}

/// 生成段落宽度数组
/// 根据行数和宽度配置生成每行的宽度
pub fn generate_paragraph_widths(
    rows: u32,
    width_config: Option<&SkeletonWidthConfig>,
) -> Vec<SkeletonWidth> {
    let mut widths = Vec::new();

    match width_config {
        Some(SkeletonWidthConfig::Single(width)) => {
            // 单一宽度配置：前面的行使用 100%，最后一行使用指定宽度
            for i in 0..rows {
                if i == rows - 1 {
                    widths.push(width.clone());
                } else {
                    widths.push(SkeletonWidth::Percentage(100));
                }
            }
        },
        Some(SkeletonWidthConfig::Multiple(width_array)) => {
            // 多宽度配置：使用数组中的宽度，不足的用 100% 填充
            for i in 0..rows {
                if let Some(width) = width_array.get(i as usize) {
                    widths.push(width.clone());
                } else {
                    widths.push(SkeletonWidth::Percentage(100));
                }
            }
        },
        None => {
            // 默认配置：所有行都使用 100%
            for _ in 0..rows {
                widths.push(SkeletonWidth::Percentage(100));
            }
        },
    }

    widths
}

/// 验证骨架屏配置
/// 检查配置是否有效
pub fn validate_skeleton_config(props: &SkeletonProps) -> Result<(), String> {
    // 检查头像配置
    if let Some(avatar_config) = &props.avatar {
        if let SkeletonAvatarConfig::Config(avatar_props) = avatar_config {
            if let Some(AvatarSize::Custom(size)) = &avatar_props.size {
                if *size == 0 {
                    return Err("Avatar size cannot be zero".to_string());
                }
                if *size > 1000 {
                    return Err("Avatar size too large (max: 1000px)".to_string());
                }
            }
        }
    }

    // 检查段落配置
    if let Some(paragraph_config) = &props.paragraph {
        if let SkeletonParagraphConfig::Config(paragraph_props) = paragraph_config {
            if let Some(rows) = paragraph_props.rows {
                if rows == 0 {
                    return Err("Paragraph rows cannot be zero".to_string());
                }
                if rows > 20 {
                    return Err("Too many paragraph rows (max: 20)".to_string());
                }
            }

            // 检查宽度配置
            if let Some(width_config) = &paragraph_props.width {
                match width_config {
                    SkeletonWidthConfig::Single(width) => {
                        validate_skeleton_width(width)?;
                    },
                    SkeletonWidthConfig::Multiple(widths) => {
                        if widths.is_empty() {
                            return Err("Width array cannot be empty".to_string());
                        }
                        for width in widths {
                            validate_skeleton_width(width)?;
                        }
                    },
                }
            }
        }
    }

    // 检查主题配置
    if let Some(theme) = &props.theme {
        if let Some(radius) = theme.block_radius {
            if radius > 50 {
                return Err("Block radius too large (max: 50px)".to_string());
            }
        }

        if let Some(line_height) = theme.paragraph_line_height {
            if line_height == 0 {
                return Err("Paragraph line height cannot be zero".to_string());
            }
            if line_height > 100 {
                return Err("Paragraph line height too large (max: 100px)".to_string());
            }
        }

        if let Some(title_height) = theme.title_height {
            if title_height == 0 {
                return Err("Title height cannot be zero".to_string());
            }
            if title_height > 100 {
                return Err("Title height too large (max: 100px)".to_string());
            }
        }
    }

    Ok(())
}

/// 验证宽度值
fn validate_skeleton_width(width: &SkeletonWidth) -> Result<(), String> {
    match width {
        SkeletonWidth::Pixels(px) => {
            if *px == 0 {
                return Err("Width cannot be zero pixels".to_string());
            }
            if *px > 10000 {
                return Err("Width too large (max: 10000px)".to_string());
            }
        },
        SkeletonWidth::Percentage(pct) => {
            if *pct == 0 {
                return Err("Width cannot be zero percent".to_string());
            }
            if *pct > 100 {
                return Err("Percentage width cannot exceed 100%".to_string());
            }
        },
        SkeletonWidth::String(s) => {
            if s.is_empty() {
                return Err("Width string cannot be empty".to_string());
            }
        },
    }
    Ok(())
}

/// 检查是否应该显示头像
pub fn should_show_avatar(avatar_config: Option<&SkeletonAvatarConfig>) -> bool {
    match avatar_config {
        Some(SkeletonAvatarConfig::Boolean(show)) => *show,
        Some(SkeletonAvatarConfig::Config(_)) => true,
        None => false,
    }
}

/// 检查是否应该显示标题
pub fn should_show_title(title_config: Option<&SkeletonTitleConfig>) -> bool {
    match title_config {
        Some(SkeletonTitleConfig::Boolean(show)) => *show,
        Some(SkeletonTitleConfig::Config(_)) => true,
        None => true, // 默认显示标题
    }
}

/// 检查是否应该显示段落
pub fn should_show_paragraph(paragraph_config: Option<&SkeletonParagraphConfig>) -> bool {
    match paragraph_config {
        Some(SkeletonParagraphConfig::Boolean(show)) => *show,
        Some(SkeletonParagraphConfig::Config(_)) => true,
        None => true, // 默认显示段落
    }
}

/// 生成缓存键
/// 用于样式缓存
pub fn generate_cache_key(
    component_type: &str,
    active: bool,
    size: Option<&str>,
    shape: Option<&str>,
    theme_hash: u64,
) -> String {
    format!(
        "{}:{}:{}:{}:{}",
        component_type,
        active,
        size.unwrap_or("default"),
        shape.unwrap_or("default"),
        theme_hash
    )
}

/// 计算主题哈希值
/// 用于缓存键生成
pub fn calculate_theme_hash(theme: &SkeletonTheme) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    
    theme.block_radius.hash(&mut hasher);
    theme.gradient_from_color.hash(&mut hasher);
    theme.gradient_to_color.hash(&mut hasher);
    theme.paragraph_line_height.hash(&mut hasher);
    theme.paragraph_margin_top.hash(&mut hasher);
    theme.title_height.hash(&mut hasher);
    
    hasher.finish()
}

/// 合并主题配置
/// 将用户主题与默认主题合并
pub fn merge_theme(user_theme: Option<&SkeletonTheme>) -> SkeletonTheme {
    let default_theme = SkeletonTheme::default();
    
    match user_theme {
        Some(theme) => SkeletonTheme {
            block_radius: theme.block_radius.or(default_theme.block_radius),
            gradient_from_color: theme.gradient_from_color.clone()
                .or(default_theme.gradient_from_color),
            gradient_to_color: theme.gradient_to_color.clone()
                .or(default_theme.gradient_to_color),
            paragraph_line_height: theme.paragraph_line_height
                .or(default_theme.paragraph_line_height),
            paragraph_margin_top: theme.paragraph_margin_top
                .or(default_theme.paragraph_margin_top),
            title_height: theme.title_height.or(default_theme.title_height),
        },
        None => default_theme,
    }
}

/// 优化段落行数
/// 根据内容长度建议合适的行数
pub fn optimize_paragraph_rows(content_length: Option<usize>) -> u32 {
    match content_length {
        Some(len) => {
            if len < 50 {
                1
            } else if len < 150 {
                2
            } else if len < 300 {
                3
            } else {
                4
            }
        },
        None => 2, // 默认 2 行
    }
}

/// 检查是否为移动设备
/// 用于响应式调整
pub fn is_mobile_device() -> bool {
    // 在实际应用中，这里应该检查 window.innerWidth 或使用媒体查询
    // 这里提供一个简单的实现
    false // 默认为桌面设备
}

/// 获取响应式配置
/// 根据设备类型调整配置
pub fn get_responsive_config(is_mobile: bool) -> (u32, u32) {
    if is_mobile {
        // 移动设备：更小的间距和高度
        (12, 20) // (line_height, margin_top)
    } else {
        // 桌面设备：标准间距和高度
        (16, 28) // (line_height, margin_top)
    }
}
use css_in_rust::css;

use super::types::*;

/// 生成骨架屏容器样式
pub fn generate_skeleton_style(
    active: bool,
    round: bool,
    theme: &SkeletonTheme,
    class_name: Option<&str>,
) -> String {
    let base_style = css! {
        display: block;
        width: 100%;
    };

    let round_style = if round {
        css! {
            border-radius: {theme.block_radius.unwrap_or(4)}px;
        }
        .to_string()
    } else {
        String::new()
    };

    let custom_class = class_name.unwrap_or("");

    format!("{} {} {}", base_style, round_style, custom_class)
}

/// 生成骨架屏头像样式
pub fn generate_skeleton_avatar_style(
    shape: &AvatarShape,
    size: &AvatarSize,
    active: bool,
    theme: &SkeletonTheme,
) -> String {
    let size_px = size.to_css();

    let base_style = css! {
        display: inline-block;
        width: {size_px};
        height: {size_px};
        vertical-align: top;
    };

    let shape_style = match shape {
        AvatarShape::Circle => css! (
            border-radius: 50%;
        )
        .to_string(),
        AvatarShape::Square => css! (
            border-radius: {theme.block_radius.unwrap_or(4)}px;
        )
        .to_string(),
    };

    let animation_style = generate_skeleton_animation(active, theme);

    format!("{} {} {}", base_style, shape_style, animation_style)
}

/// 生成骨架屏标题样式
pub fn generate_skeleton_title_style(
    width: Option<&SkeletonWidth>,
    active: bool,
    round: bool,
    theme: &SkeletonTheme,
) -> String {
    let height = theme.title_height.unwrap_or(16);
    let width_css = width
        .map(|w| w.to_css())
        .unwrap_or_else(|| "100%".to_string());

    let base_style = css! {
        height: {height}px;
        width: {width_css};
        margin: 0;
        margin-bottom: 8px;
    };

    let round_style = if round {
        css! (
            border-radius: {theme.block_radius.unwrap_or(4)}px;
        )
        .to_string()
    } else {
        String::new()
    };

    let animation_style = generate_skeleton_animation(active, theme);

    format!("{} {} {}", base_style, round_style, animation_style)
}

/// 生成骨架屏段落样式
pub fn generate_skeleton_paragraph_style(
    rows: u32,
    widths: &[SkeletonWidth],
    active: bool,
    round: bool,
    theme: &SkeletonTheme,
) -> String {
    let line_height = theme.paragraph_line_height.unwrap_or(16);
    let margin_top = theme.paragraph_margin_top.unwrap_or(28);

    let container_style = css! {
        margin: 0;
        margin-top: {margin_top}px;
    };

    let mut line_styles = Vec::new();

    for (index, width) in widths.iter().enumerate() {
        let width_css = width.to_css();
        let margin_bottom = if index < widths.len() - 1 { "8px" } else { "0" };

        let line_style = css! {
            height: {line_height}px;
            width: {width_css};
            margin-bottom: {margin_bottom};
        };

        let round_style = if round {
            css! {
                border-radius: {theme.block_radius.unwrap_or(4)}px;
            }
            .to_string()
        } else {
            String::new()
        };

        let animation_style = generate_skeleton_animation(active, theme);

        line_styles.push(format!(
            "{} {} {}",
            line_style, round_style, animation_style
        ));
    }

    format!("{} /* lines: {} */", container_style, line_styles.join(" "))
}

/// 生成骨架屏按钮样式
pub fn generate_skeleton_button_style(
    shape: &ButtonShape,
    size: &ButtonSize,
    block: bool,
    active: bool,
    theme: &SkeletonTheme,
) -> String {
    let height = size.to_height_css();
    let width = if block {
        "100%".to_string()
    } else {
        size.to_width_css()
    };

    let base_style = css! {
        display: inline-block;
        height: {height};
        width: {width};
        vertical-align: top;
    };

    let shape_style = match shape {
        ButtonShape::Circle => {
            let size_px = size.to_height_css();
            css! {
                border-radius: 50%;
                width: {size_px};
            }
            .to_string()
        }
        ButtonShape::Round => css! {
            border-radius: {height};
        }
        .to_string(),
        ButtonShape::Square => css! {
            border-radius: 0;
        }
        .to_string(),
        ButtonShape::Default => css! {
            border-radius: {theme.block_radius.unwrap_or(4)}px;
        }
        .to_string(),
    };

    let animation_style = generate_skeleton_animation(active, theme);

    format!("{} {} {}", base_style, shape_style, animation_style)
}

/// 生成骨架屏输入框样式
pub fn generate_skeleton_input_style(
    size: &InputSize,
    active: bool,
    theme: &SkeletonTheme,
) -> String {
    let height = size.to_height_css();

    let base_style = css! {
        display: inline-block;
        height: {height};
        width: 100%;
        border-radius: {theme.block_radius.unwrap_or(4)}px;
    };

    let animation_style = generate_skeleton_animation(active, theme);

    format!("{} {}", base_style, animation_style)
}

/// 生成骨架屏图片样式
pub fn generate_skeleton_image_style(
    width: Option<&SkeletonWidth>,
    height: Option<&SkeletonWidth>,
    active: bool,
    theme: &SkeletonTheme,
) -> String {
    let width_css = width
        .map(|w| w.to_css())
        .unwrap_or_else(|| "100%".to_string());
    let height_css = height
        .map(|h| h.to_css())
        .unwrap_or_else(|| "200px".to_string());

    let base_style = css! {
        display: inline-block;
        width: {width_css};
        height: {height_css};
        border-radius: {theme.block_radius.unwrap_or(4)}px;
    };

    let animation_style = generate_skeleton_animation(active, theme);

    format!("{} {}", base_style, animation_style)
}

/// 生成骨架屏动画样式
pub fn generate_skeleton_animation(active: bool, theme: &SkeletonTheme) -> String {
    if active {
        let from_color = theme
            .gradient_from_color
            .as_ref()
            .unwrap_or(&"rgba(0,0,0,0.06)".to_string());
        let to_color = theme
            .gradient_to_color
            .as_ref()
            .unwrap_or(&"rgba(0,0,0,0.15)".to_string());

        css! {
            background: linear-gradient(90deg, {from_color} 25%, {to_color} 37%, {from_color} 63%);
            background-size: 400% 100%;
            animation: skeleton-loading 1.4s ease-in-out infinite;
        }
        .to_string()
    } else {
        let from_color = theme
            .gradient_from_color
            .as_ref()
            .unwrap_or(&"rgba(0,0,0,0.06)".to_string());

        css! {
            background: {from_color};
        }
        .to_string()
    }
}

/// 生成骨架屏关键帧动画
pub fn generate_skeleton_keyframes() -> String {
    css! {
        @keyframes skeleton-loading {
            0% {
                background-position: 100% 50%;
            }
            100% {
                background-position: 0 50%;
            }
        }
    }
    .to_string()
}

/// 获取骨架屏 CSS 类名
pub fn get_skeleton_class_name(
    loading: bool,
    active: bool,
    round: bool,
    custom_class: Option<&str>,
) -> String {
    let mut classes = vec!["ant-skeleton"];

    if loading {
        classes.push("ant-skeleton-loading");
    }

    if active {
        classes.push("ant-skeleton-active");
    }

    if round {
        classes.push("ant-skeleton-round");
    }

    if let Some(custom) = custom_class {
        classes.push(custom);
    }

    classes.join(" ")
}

/// 获取骨架屏头像 CSS 类名
pub fn get_skeleton_avatar_class_name(
    shape: &AvatarShape,
    size: &AvatarSize,
    active: bool,
    custom_class: Option<&str>,
) -> String {
    let mut classes = vec!["ant-skeleton-avatar"];

    let shape_c = shape.clone();
    let shape_class = format!("ant-skeleton-avatar-{}", shape_c);
    classes.push(&shape_class);

    match size {
        AvatarSize::Large => classes.push("ant-skeleton-avatar-lg"),
        AvatarSize::Small => classes.push("ant-skeleton-avatar-sm"),
        AvatarSize::Default => {}
        AvatarSize::Custom(_) => classes.push("ant-skeleton-avatar-custom"),
    }

    if active {
        classes.push("ant-skeleton-avatar-active");
    }

    if let Some(custom) = custom_class {
        classes.push(custom);
    }

    classes.join(" ")
}

/// 获取骨架屏按钮 CSS 类名
pub fn get_skeleton_button_class_name(
    shape: &ButtonShape,
    size: &ButtonSize,
    block: bool,
    active: bool,
    custom_class: Option<&str>,
) -> String {
    let mut classes = vec!["ant-skeleton-button"];

    let shape_class = format!("ant-skeleton-button-{}", shape);
    classes.push(&shape_class);

    match size {
        ButtonSize::Large => classes.push("ant-skeleton-button-lg"),
        ButtonSize::Small => classes.push("ant-skeleton-button-sm"),
        ButtonSize::Default => {}
    }

    if block {
        classes.push("ant-skeleton-button-block");
    }

    if active {
        classes.push("ant-skeleton-button-active");
    }

    if let Some(custom) = custom_class {
        classes.push(custom);
    }

    classes.join(" ")
}

/// 获取骨架屏输入框 CSS 类名
pub fn get_skeleton_input_class_name(
    size: &InputSize,
    active: bool,
    custom_class: Option<&str>,
) -> String {
    let mut classes = vec!["ant-skeleton-input"];

    match size {
        InputSize::Large => classes.push("ant-skeleton-input-lg"),
        InputSize::Small => classes.push("ant-skeleton-input-sm"),
        InputSize::Default => {}
    }

    if active {
        classes.push("ant-skeleton-input-active");
    }

    if let Some(custom) = custom_class {
        classes.push(custom);
    }

    classes.join(" ")
}

/// 获取骨架屏图片 CSS 类名
pub fn get_skeleton_image_class_name(active: bool, custom_class: Option<&str>) -> String {
    let mut classes = vec!["ant-skeleton-image"];

    if active {
        classes.push("ant-skeleton-image-active");
    }

    if let Some(custom) = custom_class {
        classes.push(custom);
    }

    classes.join(" ")
}

/// 生成完整的骨架屏样式表
pub fn generate_complete_skeleton_styles(theme: &SkeletonTheme) -> String {
    let keyframes = generate_skeleton_keyframes();

    let base_styles = css! {
        .ant-skeleton {
            display: table;
            width: 100%;
        }

        .ant-skeleton-header {
            display: table-cell;
            padding-right: 16px;
            vertical-align: top;
        }

        .ant-skeleton-content {
            display: table-cell;
            width: 100%;
            vertical-align: top;
        }

        .ant-skeleton-avatar {
            display: inline-block;
            vertical-align: top;
            background: {theme.gradient_from_color.as_ref().unwrap_or(&"rgba(0,0,0,0.06)".to_string())};
        }

        .ant-skeleton-title {
            width: 100%;
            height: {theme.title_height.unwrap_or(16)}px;
            background: {theme.gradient_from_color.as_ref().unwrap_or(&"rgba(0,0,0,0.06)".to_string())};
            border-radius: {theme.block_radius.unwrap_or(4)}px;
        }

        .ant-skeleton-paragraph {
            padding: 0;
            margin: 0;
            margin-top: {theme.paragraph_margin_top.unwrap_or(28)}px;
        }

        .ant-skeleton-paragraph > li {
            width: 100%;
            height: {theme.paragraph_line_height.unwrap_or(16)}px;
            list-style: none;
            background: {theme.gradient_from_color.as_ref().unwrap_or(&"rgba(0,0,0,0.06)".to_string())};
            border-radius: {theme.block_radius.unwrap_or(4)}px;
        }

        .ant-skeleton-paragraph > li + li {
            margin-top: 8px;
        }

        .ant-skeleton-button {
            display: inline-block;
            vertical-align: top;
            background: {theme.gradient_from_color.as_ref().unwrap_or(&"rgba(0,0,0,0.06)".to_string())};
            border-radius: {theme.block_radius.unwrap_or(4)}px;
        }

        .ant-skeleton-input {
            display: inline-block;
            width: 100%;
            background: {theme.gradient_from_color.as_ref().unwrap_or(&"rgba(0,0,0,0.06)".to_string())};
            border-radius: {theme.block_radius.unwrap_or(4)}px;
        }

        .ant-skeleton-image {
            display: flex;
            align-items: center;
            justify-content: center;
            background: {theme.gradient_from_color.as_ref().unwrap_or(&"rgba(0,0,0,0.06)".to_string())};
            border-radius: {theme.block_radius.unwrap_or(4)}px;
        }
    };

    format!("{} {}", keyframes, base_styles)
}

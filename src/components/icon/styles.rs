//! Icon 组件的样式实现

use css_in_rust::css;
use super::types::{IconTheme, IconSize};

/// 生成Icon组件的基础样式
pub fn generate_base_style() -> String {
    css!(
        r#"
        display: inline-block;
        color: inherit;
        font-style: normal;
        line-height: 0;
        text-align: center;
        text-transform: none;
        vertical-align: -0.125em;
        text-rendering: optimizeLegibility;
        -webkit-font-smoothing: antialiased;
        -moz-osx-font-smoothing: grayscale;
        
        svg {
            display: inline-block;
            width: 1em;
            height: 1em;
            fill: currentColor;
            vertical-align: -0.125em;
        }
        "#
    )
}

/// 生成Icon组件的尺寸样式
pub fn generate_size_style(size: &IconSize) -> String {
    let pixel_size = size.to_css();
    css!(
        r#"
        font-size: {pixel_size};
        width: {pixel_size};
        height: {pixel_size};
        "#
    )
}

/// 生成Icon组件的旋转样式
pub fn generate_rotation_style(degrees: i32) -> String {
    css!(
        r#"
        transform: rotate({degrees}deg);
        "#
    )
}

/// 生成Icon组件的旋转动画样式
pub fn generate_spin_style() -> String {
    css!(
        r#"
        animation: antIconSpin 1s infinite linear;
        
        @keyframes antIconSpin {
            from {
                transform: rotate(0deg);
            }
            to {
                transform: rotate(360deg);
            }
        }
        "#
    )
}

/// 生成Icon组件的主题样式
pub fn generate_theme_style(theme: &IconTheme) -> String {
    match theme {
        IconTheme::Outlined => css!(
            r#"
            svg {
                fill: none;
                stroke: currentColor;
                stroke-width: 1;
            }
            "#
        ),
        IconTheme::Filled => css!(
            r#"
            svg {
                fill: currentColor;
                stroke: none;
            }
            "#
        ),
        IconTheme::TwoTone => css!(
            r#"
            svg {
                fill: currentColor;
            }
            
            .ant-icon-two-tone-primary {
                fill: var(--ant-primary-color, #1890ff);
            }
            
            .ant-icon-two-tone-secondary {
                fill: var(--ant-primary-color-hover, #40a9ff);
                opacity: 0.3;
            }
            "#
        ),
    }
}

/// 生成双色图标的自定义颜色样式
pub fn generate_two_tone_color_style(color: &str) -> String {
    css!(
        r#"
        .ant-icon-two-tone-primary {
            fill: {color};
        }
        "#
    )
}

/// 生成禁用状态的样式
pub fn generate_disabled_style() -> String {
    css!(
        r#"
        color: rgba(0, 0, 0, 0.25);
        cursor: not-allowed;
        pointer-events: none;
        "#
    )
}

/// 生成可点击状态的样式
pub fn generate_clickable_style() -> String {
    css!(
        r#"
        cursor: pointer;
        transition: color 0.3s;
        
        &:hover {
            color: var(--ant-primary-color-hover, #40a9ff);
        }
        
        &:active {
            color: var(--ant-primary-color-active, #096dd9);
        }
        "#
    )
}

/// 生成Icon组件的完整样式
pub fn generate_icon_style(
    theme: &IconTheme,
    size: Option<&IconSize>,
    rotation: Option<i32>,
    spin: bool,
    disabled: bool,
    clickable: bool,
    two_tone_color: Option<&str>,
) -> String {
    let mut styles = vec![generate_base_style()];
    
    // 添加主题样式
    styles.push(generate_theme_style(theme));
    
    // 添加尺寸样式
    if let Some(size) = size {
        styles.push(generate_size_style(size));
    }
    
    // 添加旋转样式
    if let Some(degrees) = rotation {
        if degrees != 0 {
            styles.push(generate_rotation_style(degrees));
        }
    }
    
    // 添加旋转动画样式
    if spin {
        styles.push(generate_spin_style());
    }
    
    // 添加禁用样式
    if disabled {
        styles.push(generate_disabled_style());
    }
    
    // 添加可点击样式
    if clickable && !disabled {
        styles.push(generate_clickable_style());
    }
    
    // 添加双色图标自定义颜色样式
    if let (IconTheme::TwoTone, Some(color)) = (theme, two_tone_color) {
        styles.push(generate_two_tone_color_style(color));
    }
    
    styles.join(" ")
}

/// Icon组件的CSS类名常量
pub mod class_names {
    pub const ICON: &str = "ant-icon";
    pub const ICON_OUTLINED: &str = "ant-icon-outlined";
    pub const ICON_FILLED: &str = "ant-icon-filled";
    pub const ICON_TWO_TONE: &str = "ant-icon-two-tone";
    pub const ICON_SPIN: &str = "ant-icon-spin";
    pub const ICON_DISABLED: &str = "ant-icon-disabled";
    pub const ICON_CLICKABLE: &str = "ant-icon-clickable";
}

/// 生成Icon组件的CSS类名
pub fn generate_class_names(
    theme: &IconTheme,
    spin: bool,
    disabled: bool,
    clickable: bool,
    custom_class: Option<&str>,
) -> String {
    let mut classes = vec![class_names::ICON];
    
    // 添加主题类名
    match theme {
        IconTheme::Outlined => classes.push(class_names::ICON_OUTLINED),
        IconTheme::Filled => classes.push(class_names::ICON_FILLED),
        IconTheme::TwoTone => classes.push(class_names::ICON_TWO_TONE),
    }
    
    // 添加状态类名
    if spin {
        classes.push(class_names::ICON_SPIN);
    }
    
    if disabled {
        classes.push(class_names::ICON_DISABLED);
    }
    
    if clickable && !disabled {
        classes.push(class_names::ICON_CLICKABLE);
    }
    
    // 添加自定义类名
    if let Some(custom) = custom_class {
        classes.push(custom);
    }
    
    classes.join(" ")
}
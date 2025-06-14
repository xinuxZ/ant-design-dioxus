//! Spin 组件的样式实现

use crate::theme::use_theme;
use crate::utils::class_names::class_names;
use super::types::{SpinSize, SpinTheme};
use std::collections::HashMap;

/// 生成 Spin 容器的样式
pub fn generate_spin_container_styles(size: &SpinSize, has_children: bool) -> String {
    let base_styles = r#"
        .ant-spin {
            color: var(--ant-primary-color);
            vertical-align: middle;
            text-align: center;
            opacity: 0;
            position: absolute;
            transition: transform 0.3s cubic-bezier(0.78, 0.14, 0.15, 0.86);
            font-size: 14px;
            display: inline-block;
            line-height: 1;
        }
        
        .ant-spin-spinning {
            opacity: 1;
            position: static;
        }
        
        .ant-spin-nested-loading {
            position: relative;
        }
        
        .ant-spin-nested-loading > div > .ant-spin {
            position: absolute;
            top: 0;
            left: 0;
            z-index: 4;
            display: block;
            width: 100%;
            height: 100%;
            max-height: 400px;
        }
        
        .ant-spin-nested-loading > div > .ant-spin .ant-spin-dot {
            position: absolute;
            top: 50%;
            left: 50%;
            margin: -10px;
        }
        
        .ant-spin-nested-loading > div > .ant-spin .ant-spin-text {
            position: absolute;
            top: 50%;
            width: 100%;
            padding-top: 5px;
            text-shadow: 0 1px 2px var(--ant-color-bg-container);
        }
        
        .ant-spin-nested-loading > div > .ant-spin.ant-spin-show-text .ant-spin-dot {
            margin-top: -20px;
        }
        
        .ant-spin-nested-loading > div > .ant-spin-sm .ant-spin-dot {
            margin: -7px;
        }
        
        .ant-spin-nested-loading > div > .ant-spin-sm .ant-spin-text {
            padding-top: 2px;
            font-size: 12px;
        }
        
        .ant-spin-nested-loading > div > .ant-spin-sm.ant-spin-show-text .ant-spin-dot {
            margin-top: -17px;
        }
        
        .ant-spin-nested-loading > div > .ant-spin-lg .ant-spin-dot {
            margin: -16px;
        }
        
        .ant-spin-nested-loading > div > .ant-spin-lg .ant-spin-text {
            padding-top: 8px;
            font-size: 16px;
        }
        
        .ant-spin-nested-loading > div > .ant-spin-lg.ant-spin-show-text .ant-spin-dot {
            margin-top: -26px;
        }
    "#;

    let size_styles = match size {
        SpinSize::Small => r#"
            .ant-spin-sm .ant-spin-dot {
                font-size: 14px;
                width: 14px;
                height: 14px;
            }
            
            .ant-spin-sm .ant-spin-dot i {
                width: 6px;
                height: 6px;
            }
        "#,
        SpinSize::Default => r#"
            .ant-spin .ant-spin-dot {
                font-size: 20px;
                width: 20px;
                height: 20px;
            }
            
            .ant-spin .ant-spin-dot i {
                width: 9px;
                height: 9px;
            }
        "#,
        SpinSize::Large => r#"
            .ant-spin-lg .ant-spin-dot {
                font-size: 32px;
                width: 32px;
                height: 32px;
            }
            
            .ant-spin-lg .ant-spin-dot i {
                width: 14px;
                height: 14px;
            }
        "#,
    };

    let container_styles = if has_children {
        r#"
            .ant-spin-container {
                position: relative;
                transition: opacity 0.3s;
            }
            
            .ant-spin-blur {
                clear: both;
                opacity: 0.5;
                user-select: none;
                pointer-events: none;
            }
            
            .ant-spin-blur::after {
                opacity: 0.4;
                pointer-events: none;
            }
        "#
    } else {
        ""
    };

    format!("{}{}{}", base_styles, size_styles, container_styles)
}

/// 生成 Spin 指示器的样式
pub fn generate_spin_dot_styles() -> String {
    r#"
        .ant-spin-dot {
            position: relative;
            display: inline-block;
            font-size: 20px;
            width: 1em;
            height: 1em;
        }
        
        .ant-spin-dot-item {
            position: absolute;
            display: block;
            width: 9px;
            height: 9px;
            background-color: var(--ant-primary-color);
            border-radius: 100%;
            transform: scale(0.75);
            transform-origin: 50% 50%;
            opacity: 0.3;
            animation: antSpinMove 1s infinite linear alternate;
        }
        
        .ant-spin-dot-item:nth-child(1) {
            top: 0;
            left: 0;
        }
        
        .ant-spin-dot-item:nth-child(2) {
            top: 0;
            right: 0;
            animation-delay: 0.4s;
        }
        
        .ant-spin-dot-item:nth-child(3) {
            right: 0;
            bottom: 0;
            animation-delay: 0.8s;
        }
        
        .ant-spin-dot-item:nth-child(4) {
            bottom: 0;
            left: 0;
            animation-delay: 1.2s;
        }
        
        @keyframes antSpinMove {
            to {
                opacity: 1;
            }
        }
        
        .ant-spin-dot-spin {
            transform: rotate(45deg);
            animation: antRotate 1.2s infinite linear;
        }
        
        @keyframes antRotate {
            to {
                transform: rotate(405deg);
            }
        }
    "#.to_string()
}

/// 生成 Spin 文本的样式
pub fn generate_spin_text_styles() -> String {
    r#"
        .ant-spin-text {
            padding-top: 5px;
            color: var(--ant-primary-color);
            font-size: 14px;
        }
        
        .ant-spin-sm .ant-spin-text {
            font-size: 12px;
        }
        
        .ant-spin-lg .ant-spin-text {
            font-size: 16px;
        }
    "#.to_string()
}

/// 生成遮罩层样式
pub fn generate_spin_mask_styles() -> String {
    r#"
        .ant-spin-mask {
            position: absolute;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
            background: var(--ant-color-bg-mask);
            z-index: 3;
        }
    "#.to_string()
}

/// 生成完整的 Spin 样式表
pub fn generate_spin_styles(size: &SpinSize, has_children: bool) -> String {
    let container_styles = generate_spin_container_styles(size, has_children);
    let dot_styles = generate_spin_dot_styles();
    let text_styles = generate_spin_text_styles();
    let mask_styles = if has_children {
        generate_spin_mask_styles()
    } else {
        String::new()
    };

    format!("{}{}{}{}", container_styles, dot_styles, text_styles, mask_styles)
}

/// 获取 Spin 组件的 CSS 类名
pub fn get_spin_class_name(
    size: &SpinSize,
    spinning: bool,
    has_children: bool,
    has_tip: bool,
    custom_class: Option<&str>,
) -> String {
    let mut classes = vec!["ant-spin"];

    // 添加尺寸类名
    match size {
        SpinSize::Small => classes.push("ant-spin-sm"),
        SpinSize::Large => classes.push("ant-spin-lg"),
        SpinSize::Default => {}
    }

    // 添加状态类名
    if spinning {
        classes.push("ant-spin-spinning");
    }

    if has_tip {
        classes.push("ant-spin-show-text");
    }

    // 添加自定义类名
    let base_class = class_names(&classes);
    if let Some(custom) = custom_class {
        format!("{} {}", base_class, custom)
    } else {
        base_class
    }
}

/// 获取 Spin 容器的 CSS 类名
pub fn get_spin_container_class_name(
    spinning: bool,
    wrapper_class_name: Option<&str>,
) -> String {
    let mut classes = vec!["ant-spin-nested-loading"];

    let base_class = class_names(&classes);
    if let Some(wrapper) = wrapper_class_name {
        format!("{} {}", base_class, wrapper)
    } else {
        base_class
    }
}

/// 获取 Spin 内容容器的 CSS 类名
pub fn get_spin_content_class_name(spinning: bool) -> String {
    let mut classes = vec!["ant-spin-container"];
    
    if spinning {
        classes.push("ant-spin-blur");
    }
    
    class_names(&classes)
}

/// 计算 Spin 指示器的尺寸
pub fn calculate_spin_size(size: &SpinSize) -> (u32, u32) {
    match size {
        SpinSize::Small => (14, 14),
        SpinSize::Default => (20, 20),
        SpinSize::Large => (32, 32),
    }
}

/// 获取 Spin 动画持续时间
pub fn get_spin_duration(size: &SpinSize) -> &'static str {
    match size {
        SpinSize::Small => "0.8s",
        SpinSize::Default => "1s",
        SpinSize::Large => "1.2s",
    }
}

/// 生成 CSS 变量映射
pub fn generate_spin_css_vars(theme: &SpinTheme) -> String {
    format!(
        r#"
        :root {{
            --ant-primary-color: {};
            --ant-color-text: {};
            --ant-color-bg-container: {};
            --ant-color-bg-mask: {};
            --ant-motion-duration-slow: {};
            --ant-font-size-sm: {};
            --ant-font-size: {};
            --ant-font-size-lg: {};
        }}
        "#,
        theme.color_primary,
        theme.color_text,
        theme.color_bg_container,
        theme.color_bg_mask,
        theme.motion_duration_slow,
        theme.font_size_sm,
        theme.font_size,
        theme.font_size_lg
    )
}

/// 检查是否需要显示遮罩
pub fn should_show_mask(spinning: bool, has_children: bool) -> bool {
    spinning && has_children
}

/// 获取指示器的变换样式
pub fn get_indicator_transform(size: &SpinSize) -> String {
    let (width, height) = calculate_spin_size(size);
    let offset = width / 2;
    format!("translate(-{}px, -{}px)", offset, offset)
}

/// 生成响应式样式
pub fn generate_responsive_styles() -> String {
    r#"
        @media (max-width: 576px) {
            .ant-spin-lg .ant-spin-dot {
                font-size: 24px;
                width: 24px;
                height: 24px;
            }
            
            .ant-spin-lg .ant-spin-text {
                font-size: 14px;
            }
        }
        
        @media (prefers-reduced-motion: reduce) {
            .ant-spin-dot-item {
                animation: none;
            }
            
            .ant-spin-dot-spin {
                animation: none;
            }
        }
    "#.to_string()
}

/// 获取 Spin 指示器的 CSS 类名
pub fn get_spin_indicator_class(size: &SpinSize) -> String {
    let size_class = match size {
        SpinSize::Small => "ant-spin-sm",
        SpinSize::Default => "",
        SpinSize::Large => "ant-spin-lg",
    };
    
    class_names(&[
        "ant-spin-spinning",
        size_class,
    ])
}

/// 获取 Spin 文本的 CSS 类名
pub fn get_spin_text_class() -> String {
    "ant-spin-text".to_string()
}

/// 获取 Spin 遮罩的 CSS 类名
pub fn get_spin_mask_class() -> String {
    "ant-spin-mask".to_string()
}

/// 生成 Spin 指示器的内联样式
pub fn generate_spin_indicator_style(theme: &SpinTheme, size: &SpinSize) -> String {
    let (width, height) = calculate_spin_size(size);
    format!(
        "width: {}px; height: {}px; color: {};",
        width, height, theme.color_primary
    )
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

/// 生成 Spin 主题样式
pub fn generate_spin_theme_styles(theme: &SpinTheme) -> String {
    format!(
        r#"
        .ant-spin {{
            --ant-primary-color: {};
            --ant-text-color: {};
            --ant-bg-color: {};
            --ant-mask-color: {};
            --ant-motion-duration: {};
        }}
        "#,
        theme.color_primary,
        theme.color_text,
        theme.color_bg_container,
        theme.color_bg_mask,
        theme.motion_duration_slow
    )
}
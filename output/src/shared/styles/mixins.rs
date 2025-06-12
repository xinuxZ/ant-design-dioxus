//! 样式混合器(Mixins)模块
//!
//! 本模块提供了可复用的样式片段，类似于SCSS的mixins功能。
//! 这些混合器可以在多个组件中复用，减少代码重复，提高一致性。

use css_in_rust::css;

/// 清除浮动混合器
pub fn clearfix() -> String {
    css!(
        r#"
        &::before,
        &::after {
            content: '';
            display: table;
        }
        &::after {
            clear: both;
        }
        "#
    )
    .to_string()
}

/// 文本省略混合器
pub fn text_ellipsis() -> String {
    css!(
        r#"
        overflow: hidden;
        white-space: nowrap;
        text-overflow: ellipsis;
        "#
    )
    .to_string()
}

/// 多行文本省略混合器
pub fn text_ellipsis_multiline(lines: u32) -> String {
    css!(&format!(
        r#"
            display: -webkit-box;
            -webkit-line-clamp: {};
            -webkit-box-orient: vertical;
            overflow: hidden;
            "#,
        lines
    ))
    .to_string()
}

/// 居中对齐混合器
pub fn center_flex() -> String {
    css!(
        r#"
        display: flex;
        align-items: center;
        justify-content: center;
        "#
    )
    .to_string()
}

/// 垂直居中混合器
pub fn center_vertical() -> String {
    css!(
        r#"
        display: flex;
        align-items: center;
        "#
    )
    .to_string()
}

/// 水平居中混合器
pub fn center_horizontal() -> String {
    css!(
        r#"
        display: flex;
        justify-content: center;
        "#
    )
    .to_string()
}

/// 绝对定位居中混合器
pub fn center_absolute() -> String {
    css!(
        r#"
        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        "#
    )
    .to_string()
}

/// 重置按钮样式混合器
pub fn reset_button() -> String {
    css!(
        r#"
        border: none;
        background: none;
        padding: 0;
        margin: 0;
        cursor: pointer;
        outline: none;
        "#
    )
    .to_string()
}

/// 重置列表样式混合器
pub fn reset_list() -> String {
    css!(
        r#"
        list-style: none;
        padding: 0;
        margin: 0;
        "#
    )
    .to_string()
}

/// 隐藏滚动条混合器
pub fn hide_scrollbar() -> String {
    css!(
        r#"
        -ms-overflow-style: none;
        scrollbar-width: none;
        &::-webkit-scrollbar {
            display: none;
        }
        "#
    )
    .to_string()
}

/// 自定义滚动条混合器
pub fn custom_scrollbar(width: &str, track_color: &str, thumb_color: &str) -> String {
    css!(&format!(
        r#"
            &::-webkit-scrollbar {{
                width: {};
            }}
            &::-webkit-scrollbar-track {{
                background: {};
            }}
            &::-webkit-scrollbar-thumb {{
                background: {};
                border-radius: 4px;
            }}
            &::-webkit-scrollbar-thumb:hover {{
                background: {};
            }}
            "#,
        width, track_color, thumb_color, thumb_color
    ))
    .to_string()
}

/// 过渡动画混合器
pub fn transition(property: &str, duration: &str, timing: &str) -> String {
    css!(&format!(
        r#"
            transition: {} {} {};
            "#,
        property, duration, timing
    ))
    .to_string()
}

/// 标准过渡动画混合器
pub fn transition_standard(property: &str) -> String {
    transition(
        property,
        "var(--ant-motion-duration-mid)",
        "var(--ant-motion-ease-in-out)",
    )
}

/// 悬停效果混合器
pub fn hover_effect() -> String {
    css!(
        r#"
        transition: all var(--ant-motion-duration-mid) var(--ant-motion-ease-in-out);
        &:hover {
            transform: translateY(-2px);
            box-shadow: var(--ant-box-shadow-base);
        }
        "#
    )
    .to_string()
}

/// 焦点效果混合器
pub fn focus_effect(color: &str) -> String {
    css!(&format!(
        r#"
            &:focus {{
                outline: 0;
                box-shadow: 0 0 0 2px {};
            }}
            "#,
        color
    ))
    .to_string()
}

/// 禁用状态混合器
pub fn disabled_state() -> String {
    css!(
        r#"
        &:disabled,
        &[disabled] {
            cursor: not-allowed;
            opacity: 0.6;
            pointer-events: none;
        }
        "#
    )
    .to_string()
}

/// 加载状态混合器
pub fn loading_state() -> String {
    css!(
        r#"
        &.loading {
            position: relative;
            pointer-events: none;
            &::before {
                content: '';
                position: absolute;
                top: 0;
                left: 0;
                right: 0;
                bottom: 0;
                background: rgba(255, 255, 255, 0.7);
                z-index: 1;
            }
        }
        "#
    )
    .to_string()
}

/// 响应式混合器
pub fn responsive(breakpoint: &str, styles: &str) -> String {
    css!(&format!(
        r#"
            @media (min-width: {}) {{
                {}
            }}
            "#,
        breakpoint, styles
    ))
    .to_string()
}

/// 暗色主题混合器
pub fn dark_theme(styles: &str) -> String {
    css!(&format!(
        r#"
            [data-theme="dark"] & {{
                {}
            }}
            "#,
        styles
    ))
    .to_string()
}

/// 高对比度主题混合器
pub fn high_contrast_theme(styles: &str) -> String {
    css!(&format!(
        r#"
            @media (prefers-contrast: high) {{
                {}
            }}
            "#,
        styles
    ))
    .to_string()
}

/// 减少动画混合器
pub fn reduced_motion(styles: &str) -> String {
    css!(&format!(
        r#"
            @media (prefers-reduced-motion: reduce) {{
                {}
            }}
            "#,
        styles
    ))
    .to_string()
}

/// 打印样式混合器
pub fn print_styles(styles: &str) -> String {
    css!(&format!(
        r#"
            @media print {{
                {}
            }}
            "#,
        styles
    ))
    .to_string()
}

/// 卡片样式混合器
pub fn card_style() -> String {
    css!(
        r#"
        background: var(--ant-bg-color);
        border: 1px solid var(--ant-border-color);
        border-radius: var(--ant-border-radius);
        box-shadow: var(--ant-box-shadow-card);
        overflow: hidden;
        "#
    )
    .to_string()
}

/// 输入框样式混合器
pub fn input_style() -> String {
    css!(
        r#"
        display: inline-block;
        width: 100%;
        min-width: 0;
        padding: 4px 11px;
        color: var(--ant-text-color);
        font-size: var(--ant-font-size-base);
        line-height: var(--ant-line-height-base);
        background-color: var(--ant-bg-color);
        background-image: none;
        border: 1px solid var(--ant-border-color);
        border-radius: var(--ant-border-radius);
        transition: all var(--ant-motion-duration-mid) var(--ant-motion-ease-in-out);

        &:hover {
            border-color: var(--ant-primary-color-hover);
        }

        &:focus {
            border-color: var(--ant-primary-color);
            outline: 0;
            box-shadow: 0 0 0 2px rgba(24, 144, 255, 0.2);
        }
        "#
    )
    .to_string()
}

/// 按钮基础样式混合器
pub fn button_base() -> String {
    css!(
        r#"
        position: relative;
        display: inline-block;
        font-weight: 400;
        white-space: nowrap;
        text-align: center;
        background-image: none;
        border: 1px solid transparent;
        box-shadow: 0 2px 0 rgba(0, 0, 0, 0.02);
        cursor: pointer;
        transition: all var(--ant-motion-duration-mid) var(--ant-motion-ease-in-out);
        user-select: none;
        touch-action: manipulation;
        height: 32px;
        padding: 4px 15px;
        font-size: var(--ant-font-size-base);
        border-radius: var(--ant-border-radius);
        outline: none;
        text-decoration: none;
        line-height: var(--ant-line-height-base);
        "#
    )
    .to_string()
}

/// 表格样式混合器
pub fn table_style() -> String {
    css!(
        r#"
        width: 100%;
        border-collapse: collapse;
        border-spacing: 0;

        th, td {
            padding: 16px;
            text-align: left;
            border-bottom: 1px solid var(--ant-border-color-split);
        }

        th {
            background: var(--ant-bg-color-container);
            font-weight: 500;
            color: var(--ant-text-color);
        }

        tbody tr:hover {
            background: var(--ant-bg-color-container);
        }
        "#
    )
    .to_string()
}

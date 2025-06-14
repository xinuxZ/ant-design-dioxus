//! Divider 组件的样式系统

use crate::components::divider::types::*;
use crate::components::divider::utils::*;

/// 生成 Divider 容器样式
pub fn generate_divider_container_style(config: &DividerConfig, theme: &DividerTheme) -> String {
    let mut styles = Vec::new();

    // 基础样式
    styles.push("box-sizing: border-box".to_string());
    styles.push("position: relative".to_string());
    styles.push("display: flex".to_string());

    if config.is_vertical() {
        // 垂直分割线样式
        styles.push("flex-direction: column".to_string());
        styles.push("align-items: center".to_string());
        styles.push("justify-content: center".to_string());
        styles.push(format!("height: {}", theme.vertical_height));
        styles.push(format!("margin: {}", theme.vertical_margin));
        styles.push("display: inline-flex".to_string());
    } else {
        // 水平分割线样式
        styles.push("flex-direction: row".to_string());
        styles.push("align-items: center".to_string());
        styles.push("width: 100%".to_string());

        let margin = match config.size {
            DividerSize::Small => &theme.margin_sm,
            DividerSize::Middle => &theme.margin,
            DividerSize::Large => &theme.margin_lg,
        };
        styles.push(format!("margin: {}", margin));

        if config.has_text {
            styles.push("justify-content: center".to_string());
        }
    }

    // 过渡动画
    styles.push(format!("transition: {}", theme.transition));

    styles.join("; ")
}

/// 生成分割线样式
pub fn generate_divider_line_style(
    config: &DividerConfig,
    theme: &DividerTheme,
    position: Option<&str>, // "before" | "after" | None
) -> String {
    let mut styles = Vec::new();

    // 基础样式
    styles.push("border: 0".to_string());
    styles.push("background: transparent".to_string());

    // 边框颜色和样式
    styles.push(format!("border-color: {}", theme.color_split));
    styles.push(format!("border-style: {}", config.get_border_style()));

    // 厚度
    let thickness = calculate_divider_thickness(&config.size, &config.variant);

    if config.is_vertical() {
        styles.push(format!("border-left-width: {}", thickness));
        styles.push("border-top-width: 0".to_string());
        styles.push("width: 0".to_string());
        styles.push("height: 100%".to_string());
    } else {
        styles.push(format!("border-top-width: {}", thickness));
        styles.push("border-left-width: 0".to_string());
        styles.push("height: 0".to_string());

        if config.has_text {
            // 有文本时，分割线分为两部分
            match position {
                Some("before") => match get_actual_orientation(&config.orientation) {
                    DividerOrientation::Left => styles.push("width: 5%".to_string()),
                    DividerOrientation::Right => styles.push("flex: 1".to_string()),
                    _ => styles.push("flex: 1".to_string()),
                },
                Some("after") => match get_actual_orientation(&config.orientation) {
                    DividerOrientation::Left => styles.push("flex: 1".to_string()),
                    DividerOrientation::Right => styles.push("width: 5%".to_string()),
                    _ => styles.push("flex: 1".to_string()),
                },
                _ => styles.push("width: 100%".to_string()),
            }
        } else {
            styles.push("width: 100%".to_string());
        }
    }

    styles.join("; ")
}

/// 生成文本样式
pub fn generate_divider_text_style(config: &DividerConfig, theme: &DividerTheme) -> String {
    let mut styles = Vec::new();

    // 基础样式
    styles.push("display: inline-block".to_string());
    styles.push("white-space: nowrap".to_string());
    styles.push("text-align: center".to_string());
    styles.push("background: inherit".to_string());
    styles.push("font-weight: 500".to_string());

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

    // 朴素样式调整
    if config.plain {
        styles.push("font-weight: normal".to_string());
        styles.push("font-size: 0.875em".to_string());
    }

    styles.join("; ")
}

/// 生成 CSS 类名
pub fn generate_divider_class_name(config: &DividerConfig) -> String {
    let mut classes = vec!["ant-divider".to_string()];

    // 类型
    classes.push(format!("ant-divider-{}", config.r#type));

    // 样式变体
    if config.variant != DividerVariant::Solid {
        classes.push(format!("ant-divider-{}", config.variant));
    }

    // 尺寸
    if config.size != DividerSize::Middle {
        classes.push(format!("ant-divider-{}", config.size));
    }

    // 文本相关
    if config.has_text {
        classes.push("ant-divider-with-text".to_string());
        classes.push(format!(
            "ant-divider-with-text-{}",
            get_actual_orientation(&config.orientation)
        ));

        if config.plain {
            classes.push("ant-divider-plain".to_string());
        }
    }

    // 自定义类名
    if let Some(ref custom_class) = config.class {
        classes.push(custom_class.clone());
    }

    classes.join(" ")
}

/// 生成响应式样式
pub fn generate_responsive_style(
    config: &DividerConfig,
    theme: &DividerTheme,
    breakpoint: &str,
) -> String {
    let mut styles = Vec::new();

    match breakpoint {
        "xs" => {
            // 超小屏幕
            if config.has_text {
                styles.push("font-size: 12px".to_string());
                styles.push("padding: 0 8px".to_string());
            }
            if config.is_vertical() {
                styles.push("height: 0.8em".to_string());
                styles.push("margin: 0 4px".to_string());
            }
        }
        "sm" => {
            // 小屏幕
            if config.has_text {
                styles.push("font-size: 13px".to_string());
                styles.push("padding: 0 12px".to_string());
            }
            if config.is_vertical() {
                styles.push("height: 0.85em".to_string());
                styles.push("margin: 0 6px".to_string());
            }
        }
        "md" => {
            // 中等屏幕 - 使用默认样式
        }
        "lg" => {
            // 大屏幕
            if config.has_text {
                styles.push("font-size: 15px".to_string());
                styles.push("padding: 0 20px".to_string());
            }
            if config.is_vertical() {
                styles.push("height: 1.1em".to_string());
                styles.push("margin: 0 10px".to_string());
            }
        }
        "xl" => {
            // 超大屏幕
            if config.has_text {
                styles.push("font-size: 16px".to_string());
                styles.push("padding: 0 24px".to_string());
            }
            if config.is_vertical() {
                styles.push("height: 1.2em".to_string());
                styles.push("margin: 0 12px".to_string());
            }
        }
        _ => {}
    }

    if styles.is_empty() {
        String::new()
    } else {
        format!(
            "@media (min-width: {}) {{ {} }}",
            get_breakpoint_value(breakpoint),
            styles.join("; ")
        )
    }
}

/// 获取断点值
fn get_breakpoint_value(breakpoint: &str) -> &'static str {
    match breakpoint {
        "xs" => "0px",
        "sm" => "576px",
        "md" => "768px",
        "lg" => "992px",
        "xl" => "1200px",
        _ => "0px",
    }
}

/// 生成暗色主题样式
pub fn generate_dark_theme_style(config: &DividerConfig, dark_theme: &DividerDarkTheme) -> String {
    let mut styles = Vec::new();

    // 分割线颜色
    styles.push(format!("border-color: {}", dark_theme.color_split));

    // 文本颜色
    if config.has_text {
        let color = if config.plain {
            &dark_theme.color_text_secondary
        } else {
            &dark_theme.color_text
        };
        styles.push(format!("color: {}", color));
    }

    styles.join("; ")
}

/// 生成紧凑主题样式
pub fn generate_compact_theme_style(
    config: &DividerConfig,
    compact_theme: &DividerCompactTheme,
) -> String {
    let mut styles = Vec::new();

    if config.is_vertical() {
        styles.push(format!("margin: {}", compact_theme.vertical_margin));
    } else {
        let margin = match config.size {
            DividerSize::Small => &compact_theme.margin_sm,
            DividerSize::Middle => &compact_theme.margin,
            DividerSize::Large => &compact_theme.margin_lg,
        };
        styles.push(format!("margin: {}", margin));
    }

    if config.has_text {
        styles.push(format!("padding: {}", compact_theme.text_padding));
    }

    styles.join("; ")
}

/// 生成动画样式
pub fn generate_animation_style(config: &DividerConfig, theme: &DividerTheme) -> String {
    let mut styles = Vec::new();

    // 基础过渡
    styles.push(format!("transition: {}", theme.transition));

    // 悬停效果（仅对有文本的分割线）
    if config.has_text {
        styles.push("cursor: default".to_string());
    }

    styles.join("; ")
}

/// 生成可访问性样式
pub fn generate_accessibility_style(config: &DividerConfig) -> String {
    let mut styles = Vec::new();

    // ARIA 属性
    if config.has_text {
        // 有文本的分割线作为标题
        styles.push("role: separator".to_string());
    } else {
        // 纯分割线作为装饰
        styles.push("role: presentation".to_string());
        styles.push("aria-hidden: true".to_string());
    }

    // 高对比度模式支持
    styles.push("@media (prefers-contrast: high) { border-color: currentColor }".to_string());

    // 减少动画模式支持
    styles.push("@media (prefers-reduced-motion: reduce) { transition: none }".to_string());

    styles.join("; ")
}

/// 生成完整的 CSS 样式表
pub fn generate_divider_stylesheet(theme: &DividerTheme) -> String {
    let mut css = Vec::new();

    // 基础样式
    css.push(format!(r#"
.ant-divider {{
    box-sizing: border-box;
    margin: {};
    color: {};
    font-size: {};
    line-height: {};
    list-style: none;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Noto Sans', sans-serif;
    border-top: {} solid {};
    transition: {};
}}
"#,
        theme.margin,
        theme.color_text,
        theme.font_size,
        theme.line_height,
        theme.border_width,
        theme.color_split,
        theme.transition
    ));

    // 垂直分割线
    css.push(format!(
        r#"
.ant-divider-vertical {{
    position: relative;
    top: -0.06em;
    display: inline-block;
    height: {};
    margin: {};
    vertical-align: middle;
    border-top: 0;
    border-left: {} solid {};
}}
"#,
        theme.vertical_height, theme.vertical_margin, theme.border_width, theme.color_split
    ));

    // 带文本的分割线
    css.push(format!(
        r#"
.ant-divider-with-text {{
    display: flex;
    margin: {};
    color: {};
    font-weight: 500;
    font-size: {};
    white-space: nowrap;
    text-align: center;
    border-top: 0;
    align-items: center;
}}

.ant-divider-with-text::before,
.ant-divider-with-text::after {{
    position: relative;
    width: 50%;
    border-top: {} solid transparent;
    border-top-color: inherit;
    border-bottom: 0;
    transform: translateY(50%);
    content: '';
}}
"#,
        theme.margin, theme.color_text, theme.font_size, theme.border_width
    ));

    // 文本位置样式
    css.push(
        r#"
.ant-divider-with-text-left::before {
    width: 5%;
}

.ant-divider-with-text-left::after {
    width: 95%;
}

.ant-divider-with-text-right::before {
    width: 95%;
}

.ant-divider-with-text-right::after {
    width: 5%;
}

.ant-divider-inner-text {
    display: inline-block;
    padding: 0 16px;
}
"#
        .to_string(),
    );

    // 朴素文本样式
    css.push(format!(
        r#"
.ant-divider-plain.ant-divider-with-text {{
    color: {};
    font-weight: normal;
    font-size: {};
}}
"#,
        theme.color_text_secondary, theme.font_size_sm
    ));

    // 样式变体
    css.push(format!(
        r#"
.ant-divider-dashed {{
    background: none;
    border-color: {};
    border-style: dashed;
    border-width: {} 0 0;
}}

.ant-divider-dashed.ant-divider-vertical {{
    border-width: 0 0 0 {};
}}

.ant-divider-dotted {{
    background: none;
    border-color: {};
    border-style: dotted;
    border-width: {} 0 0;
}}

.ant-divider-dotted.ant-divider-vertical {{
    border-width: 0 0 0 {};
}}
"#,
        theme.color_split,
        theme.border_width,
        theme.border_width,
        theme.color_split,
        theme.border_width,
        theme.border_width
    ));

    // 尺寸变体
    css.push(format!(
        r#"
.ant-divider-small {{
    margin: {};
    font-size: {};
}}

.ant-divider-large {{
    margin: {};
    font-size: {};
}}
"#,
        theme.margin_sm, theme.font_size_sm, theme.margin_lg, theme.font_size_lg
    ));

    // 响应式样式
    css.push(
        r#"
@media (max-width: 575px) {
    .ant-divider-with-text {
        font-size: 12px;
    }

    .ant-divider-inner-text {
        padding: 0 8px;
    }

    .ant-divider-vertical {
        height: 0.8em;
        margin: 0 4px;
    }
}

@media (min-width: 1200px) {
    .ant-divider-with-text {
        font-size: 16px;
    }

    .ant-divider-inner-text {
        padding: 0 24px;
    }

    .ant-divider-vertical {
        height: 1.2em;
        margin: 0 12px;
    }
}
"#
        .to_string(),
    );

    // 可访问性样式
    css.push(
        r#"
@media (prefers-contrast: high) {
    .ant-divider {
        border-color: currentColor;
    }
}

@media (prefers-reduced-motion: reduce) {
    .ant-divider {
        transition: none;
    }
}
"#
        .to_string(),
    );

    css.join("\n")
}

/// 样式生成器结构体
pub struct DividerStyleGenerator {
    theme: DividerTheme,
    dark_theme: Option<DividerDarkTheme>,
    compact_theme: Option<DividerCompactTheme>,
}

impl DividerStyleGenerator {
    /// 创建新的样式生成器
    pub fn new(theme: DividerTheme) -> Self {
        Self {
            theme,
            dark_theme: None,
            compact_theme: None,
        }
    }

    /// 设置暗色主题
    pub fn with_dark_theme(mut self, dark_theme: DividerDarkTheme) -> Self {
        self.dark_theme = Some(dark_theme);
        self
    }

    /// 设置紧凑主题
    pub fn with_compact_theme(mut self, compact_theme: DividerCompactTheme) -> Self {
        self.compact_theme = Some(compact_theme);
        self
    }

    /// 生成组件样式
    pub fn generate_component_style(&self, config: &DividerConfig) -> String {
        let mut styles = Vec::new();

        // 基础样式
        styles.push(generate_divider_container_style(config, &self.theme));

        // 暗色主题样式
        if let Some(ref dark_theme) = self.dark_theme {
            styles.push(generate_dark_theme_style(config, dark_theme));
        }

        // 紧凑主题样式
        if let Some(ref compact_theme) = self.compact_theme {
            styles.push(generate_compact_theme_style(config, compact_theme));
        }

        // 动画样式
        styles.push(generate_animation_style(config, &self.theme));

        // 可访问性样式
        styles.push(generate_accessibility_style(config));

        styles.join("; ")
    }

    /// 生成完整样式表
    pub fn generate_stylesheet(&self) -> String {
        generate_divider_stylesheet(&self.theme)
    }
}

impl Default for DividerStyleGenerator {
    fn default() -> Self {
        Self::new(DividerTheme::default())
    }
}

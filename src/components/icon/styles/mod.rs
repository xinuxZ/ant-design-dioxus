//! Icon 组件样式模块
//!
//! 本模块包含 Icon 组件的所有样式定义，从组件逻辑中分离出来，
//! 提高代码的可维护性和复用性。

// use crate::shared::styles::mixins::*;
// use crate::shared::styles::tokens::DesignToken;
use css_in_rust::css;

/// 图标主题枚举
#[derive(Debug, Clone, PartialEq)]
pub enum IconTheme {
    Outlined,
    Filled,
    TwoTone,
}

/// 图标旋转方向枚举
#[derive(Debug, Clone, PartialEq)]
pub enum IconRotate {
    None,
    Rotate90,
    Rotate180,
    Rotate270,
}

/// 图标尺寸枚举
#[derive(Debug, Clone, PartialEq)]
pub enum IconSize {
    Small,
    Medium,
    Large,
    ExtraLarge,
    Custom(String),
}

/// 图标样式生成器
pub struct IconStyleGenerator {
    pub theme: IconTheme,
    pub rotate: IconRotate,
    pub size: IconSize,
    pub color: Option<String>,
    pub spin: bool,
    pub disabled: bool,
    pub is_rtl: bool,
}

impl IconStyleGenerator {
    /// 创建新的图标样式生成器
    pub fn new() -> Self {
        Self {
            theme: IconTheme::Outlined,
            rotate: IconRotate::None,
            size: IconSize::Medium,
            color: None,
            spin: false,
            disabled: false,
            is_rtl: false,
        }
    }

    /// 设置图标主题
    pub fn with_theme(mut self, theme: IconTheme) -> Self {
        self.theme = theme;
        self
    }

    /// 设置图标旋转方向
    pub fn with_rotate(mut self, rotate: IconRotate) -> Self {
        self.rotate = rotate;
        self
    }

    /// 设置图标尺寸
    pub fn with_size(mut self, size: IconSize) -> Self {
        self.size = size;
        self
    }

    /// 设置图标颜色
    pub fn with_color(mut self, color: Option<String>) -> Self {
        self.color = color;
        self
    }

    /// 设置旋转动画
    pub fn with_spin(mut self, spin: bool) -> Self {
        self.spin = spin;
        self
    }

    /// 设置禁用状态
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// 设置 RTL 方向
    pub fn with_rtl(mut self, is_rtl: bool) -> Self {
        self.is_rtl = is_rtl;
        self
    }

    /// 生成完整的图标样式类名
    pub fn generate(&self) -> String {
        let mut classes = vec![self.base_style()];

        // 添加主题样式
        classes.push(self.theme_style());

        // 添加尺寸样式
        classes.push(self.size_style());

        // 添加旋转样式
        if self.spin {
            classes.push(self.spin_style());
        } else {
            classes.push(self.rotate_style());
        }

        // 添加禁用样式
        if self.disabled {
            classes.push(self.disabled_style());
        }

        classes.join(" ")
    }

    /// 基础图标样式
    fn base_style(&self) -> String {
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
            cursor: pointer;
            transition: all var(--ant-motion-duration-mid) var(--ant-motion-ease-in-out);

            & > * {
                line-height: 1;
            }

            & svg {
                display: inline-block;
            }

            &::before {
                display: none;
            }

            & .anticon-icon {
                display: block;
            }

            &:hover {
                color: var(--ant-primary-color-hover);
            }
            "#
        )
    }

    /// 主题样式
    fn theme_style(&self) -> String {
        match self.theme {
            IconTheme::Outlined => css!("/* 线框风格不需要额外样式 */").to_string(),
            IconTheme::Filled => css!("color: var(--ant-primary-color);").to_string(),
            IconTheme::TwoTone => css!("color: var(--ant-primary-color);").to_string(),
        }
    }

    /// 尺寸样式
    fn size_style(&self) -> String {
        match &self.size {
            IconSize::Small => css!("font-size: 12px;").to_string(),
            IconSize::Medium => css!("font-size: 16px;").to_string(),
            IconSize::Large => css!("font-size: 18px;").to_string(),
            IconSize::ExtraLarge => css!("font-size: 24px;").to_string(),
            IconSize::Custom(_size) => css!(&format!("font-size: {};", _size)).to_string(),
        }
    }

    /// 旋转动画样式
    fn spin_style(&self) -> String {
        css!(
            r#"
            animation: loadingCircle 1s infinite linear;

            @keyframes loadingCircle {
                100% {
                    transform: rotate(360deg);
                }
            }

            @media (prefers-reduced-motion: reduce) {
                animation: none;
            }
            "#
        )
    }

    /// 旋转角度样式
    fn rotate_style(&self) -> String {
        match self.rotate {
            IconRotate::None => css!("").to_string(),
            IconRotate::Rotate90 => css!(
                r#"
                transform: rotate(90deg);

                [dir="rtl"] & {
                    transform: rotate(-90deg);
                }
                "#
            )
            .to_string(),
            IconRotate::Rotate180 => css!("transform: rotate(180deg);").to_string(),
            IconRotate::Rotate270 => css!(
                r#"
                transform: rotate(270deg);

                [dir="rtl"] & {
                    transform: rotate(-270deg);
                }
                "#
            )
            .to_string(),
        }
    }

    /// 禁用样式
    fn disabled_style(&self) -> String {
        css!(
            r#"
            color: var(--ant-disabled-color);
            cursor: not-allowed;
            "#
        )
    }
}

/// 高对比度模式样式
pub fn high_contrast_style() -> String {
    css!(
        r#"
        @media (prefers-contrast: high) {
            .anticon {
                filter: contrast(1.2);
            }
        }
        "#
    )
}

/// 减少动画模式样式
pub fn reduced_motion_style() -> String {
    css!(
        r#"
        @media (prefers-reduced-motion: reduce) {
            .anticon-spin {
                animation: none;
            }

            .anticon {
                transition: none;
            }
        }
        "#
    )
}

/// 生成图标的 CSS 样式
pub fn generate_icon_style(
    theme: IconTheme,
    rotate: IconRotate,
    size: IconSize,
    color: Option<String>,
    spin: bool,
    is_rtl: bool,
    disabled: bool,
) -> String {
    let mut styles = String::new();

    // 基础样式
    styles.push_str(
        r#"
        .anticon {
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
            cursor: pointer;
            transition: all 0.3s;
        }

        .anticon > * {
            line-height: 1;
        }

        .anticon svg {
            display: inline-block;
        }

        .anticon::before {
            display: none;
        }

        .anticon .anticon-icon {
            display: block;
        }

        .anticon:hover {
            color: #40a9ff;
        }
        "#,
    );

    // 主题样式
    match theme {
        IconTheme::Outlined => {} // 线框风格不需要额外样式
        IconTheme::Filled => {
            styles.push_str(
                r#"
                .anticon {
                    color: #1890ff;
                }
                "#,
            );
        }
        IconTheme::TwoTone => {
            styles.push_str(
                r#"
                .anticon {
                    color: #1890ff;
                }
                "#,
            );
        }
    }

    // 尺寸样式
    match size {
        IconSize::Small => {
            styles.push_str(
                r#"
                .anticon {
                    font-size: 12px;
                }
                "#,
            );
        }
        IconSize::Medium => {
            styles.push_str(
                r#"
                .anticon {
                    font-size: 16px;
                }
                "#,
            );
        }
        IconSize::Large => {
            styles.push_str(
                r#"
                .anticon {
                    font-size: 18px;
                }
                "#,
            );
        }
        IconSize::ExtraLarge => {
            styles.push_str(
                r#"
                .anticon {
                    font-size: 24px;
                }
                "#,
            );
        }
        IconSize::Custom(size_value) => {
            styles.push_str(&format!(
                r#"
                .anticon {{
                    font-size: {};
                }}
                "#,
                size_value
            ));
        }
    }

    // 旋转样式
    if spin {
        styles.push_str(
            r#"
            .anticon {
                animation: loadingCircle 1s infinite linear;
            }

            @keyframes loadingCircle {
                100% {
                    transform: rotate(360deg);
                }
            }
            "#,
        );
    } else {
        match rotate {
            IconRotate::None => {} // 不旋转不需要额外样式
            IconRotate::Rotate90 => {
                if is_rtl {
                    styles.push_str(
                        r#"
                        .anticon {
                            transform: rotate(-90deg);
                        }
                        "#,
                    );
                } else {
                    styles.push_str(
                        r#"
                        .anticon {
                            transform: rotate(90deg);
                        }
                        "#,
                    );
                }
            }
            IconRotate::Rotate180 => {
                styles.push_str(
                    r#"
                    .anticon {
                        transform: rotate(180deg);
                    }
                    "#,
                );
            }
            IconRotate::Rotate270 => {
                if is_rtl {
                    styles.push_str(
                        r#"
                        .anticon {
                            transform: rotate(-270deg);
                        }
                        "#,
                    );
                } else {
                    styles.push_str(
                        r#"
                        .anticon {
                            transform: rotate(270deg);
                        }
                        "#,
                    );
                }
            }
        }
    }

    // 禁用样式
    if disabled {
        styles.push_str(
            r#"
            .anticon {
                color: rgba(0, 0, 0, 0.25);
                cursor: not-allowed;
            }
            "#,
        );
    }

    // 自定义颜色
    if let Some(color_value) = color {
        styles.push_str(&format!(
            r#"
            .anticon {{
                color: {};
            }}
            "#,
            color_value
        ));
    }

    styles
}

/// 默认的图标样式
pub fn default_icon_style() -> String {
    generate_icon_style(
        IconTheme::Outlined,
        IconRotate::None,
        IconSize::Medium,
        None,
        false,
        false,
        false,
    )
}

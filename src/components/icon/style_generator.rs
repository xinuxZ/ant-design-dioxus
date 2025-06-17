//!
//! Icon组件样式管理
//!
//! 使用样式生成器模式管理Icon组件的样式
//! 使用 css! 宏的样式生成器模式，提供 Icon 组件的完整样式生成功能

use crate::components::icon::types::*;
use crate::shared::styles::mixins::*;
use css_in_rust::css;

/// Icon样式生成器
pub struct IconStyleGenerator {
    pub size: IconSize,
    pub color: Option<String>,
    pub spin: bool,
    pub rotate: Option<i32>,
}

impl IconStyleGenerator {
    /// 创建新的Icon样式生成器
    pub fn new() -> Self {
        Self {
            size: IconSize::Default,
            color: None,
            spin: false,
            rotate: None,
        }
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

    /// 设置是否旋转
    pub fn with_spin(mut self, spin: bool) -> Self {
        self.spin = spin;
        self
    }

    /// 设置旋转角度
    pub fn with_rotate(mut self, rotate: Option<i32>) -> Self {
        self.rotate = rotate;
        self
    }

    /// 生成完整的Icon样式类名
    pub fn generate(&self) -> String {
        let mut classes = vec![self.base_style()];

        classes.push(self.size_style());

        if self.color.is_some() {
            classes.push(self.color_style());
        }

        if self.spin {
            classes.push(self.spin_style());
        }

        if self.rotate.is_some() {
            classes.push(self.rotate_style());
        }

        classes.join(" ")
    }

    /// 基础Icon样式
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
            transition: all 0.3s;
            "#
        )
        .to_string()
    }

    /// 图标尺寸样式
    fn size_style(&self) -> String {
        match self.size {
            IconSize::Small => css!(
                r#"
                font-size: 12px;
                "#
            )
            .to_string(),
            IconSize::Default => css!(
                r#"
                font-size: 14px;
                "#
            )
            .to_string(),
            IconSize::Medium => css!(
                r#"
                font-size: 16px;
                "#
            )
            .to_string(),
            IconSize::Large => css!(
                r#"
                font-size: 20px;
                "#
            )
            .to_string(),
            IconSize::Custom(size) => css!(
                r#"
                font-size: {}px;
                "#,
                size
            )
            .to_string(),
        }
    }

    /// 图标颜色样式
    fn color_style(&self) -> String {
        if let Some(ref color) = self.color {
            css!(
                r#"
                color: {};
                "#,
                color
            )
            .to_string()
        } else {
            String::new()
        }
    }

    /// 旋转动画样式
    fn spin_style(&self) -> String {
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
        .to_string()
    }

    /// 静态旋转样式
    fn rotate_style(&self) -> String {
        if let Some(rotate) = self.rotate {
            css!(
                r#"
                transform: rotate({}deg);
                "#,
                rotate
            )
            .to_string()
        } else {
            String::new()
        }
    }
}

impl Default for IconStyleGenerator {
    fn default() -> Self {
        Self::new()
    }
}

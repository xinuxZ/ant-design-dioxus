//! Divider组件的样式生成器
//!
//! 提供Divider组件的样式生成功能，包括水平/垂直分割线、带文字的分割线、虚线等样式的生成。

use crate::theme::AliasToken;
use css_in_rust::css;

/// 分割线类型
#[derive(Debug, Clone, PartialEq)]
pub enum DividerType {
    /// 水平分割线
    Horizontal,
    /// 垂直分割线
    Vertical,
}

impl Default for DividerType {
    fn default() -> Self {
        Self::Horizontal
    }
}

/// 分割线文字位置
#[derive(Debug, Clone, PartialEq)]
pub enum DividerOrientation {
    /// 左侧
    Left,
    /// 居中
    Center,
    /// 右侧
    Right,
}

impl Default for DividerOrientation {
    fn default() -> Self {
        Self::Center
    }
}

/// 分割线尺寸
#[derive(Debug, Clone, PartialEq)]
pub enum DividerSize {
    /// 小号
    Small,
    /// 默认
    Default,
    /// 大号
    Large,
}

impl Default for DividerSize {
    fn default() -> Self {
        Self::Default
    }
}

/// Divider组件样式生成器
#[derive(Debug, Clone)]
pub struct DividerStyleGenerator {
    /// 分割线类型
    pub divider_type: DividerType,
    /// 分割线尺寸
    pub size: DividerSize,
    /// 是否虚线
    pub dashed: bool,
    /// 是否简洁模式
    pub plain: bool,
    /// 是否有文字
    pub has_text: bool,
    /// 文字位置
    pub orientation: DividerOrientation,
    /// 主题令牌
    pub token: AliasToken,
}

impl Default for DividerStyleGenerator {
    fn default() -> Self {
        Self {
            divider_type: DividerType::default(),
            size: DividerSize::default(),
            dashed: false,
            plain: false,
            has_text: false,
            orientation: DividerOrientation::default(),
            token: AliasToken::default(),
        }
    }
}

impl DividerStyleGenerator {
    /// 创建新的Divider样式生成器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置分割线类型
    pub fn with_type(mut self, divider_type: DividerType) -> Self {
        self.divider_type = divider_type;
        self
    }

    /// 设置分割线尺寸
    pub fn with_size(mut self, size: DividerSize) -> Self {
        self.size = size;
        self
    }

    /// 设置是否为虚线
    pub fn with_dashed(mut self, dashed: bool) -> Self {
        self.dashed = dashed;
        self
    }

    /// 设置是否为简洁模式
    pub fn with_plain(mut self, plain: bool) -> Self {
        self.plain = plain;
        self
    }

    /// 设置是否有文字
    pub fn with_has_text(mut self, has_text: bool) -> Self {
        self.has_text = has_text;
        self
    }

    /// 设置文字位置
    pub fn with_orientation(mut self, orientation: DividerOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// 设置主题令牌
    pub fn with_token(mut self, token: AliasToken) -> Self {
        self.token = token;
        self
    }

    /// 生成样式类名
    pub fn generate(&self) -> String {
        let mut classes = vec!["ant-divider"];

        // 添加类型相关的类名
        match self.divider_type {
            DividerType::Horizontal => classes.push("ant-divider-horizontal"),
            DividerType::Vertical => classes.push("ant-divider-vertical"),
        }

        // 添加尺寸相关的类名
        match self.size {
            DividerSize::Small => classes.push("ant-divider-small"),
            DividerSize::Default => {}
            DividerSize::Large => classes.push("ant-divider-large"),
        }

        // 添加虚线相关的类名
        if self.dashed {
            classes.push("ant-divider-dashed");
        }

        // 添加简洁模式相关的类名
        if self.plain {
            classes.push("ant-divider-plain");
        }

        // 添加文字相关的类名
        if self.has_text {
            classes.push("ant-divider-with-text");
            match self.orientation {
                DividerOrientation::Left => classes.push("ant-divider-with-text-left"),
                DividerOrientation::Center => classes.push("ant-divider-with-text-center"),
                DividerOrientation::Right => classes.push("ant-divider-with-text-right"),
            }
        }

        classes.join(" ")
    }

    /// 生成 CSS 样式
    pub fn generate_css(&self) -> String {
        css!(
            r#"
            .ant-divider {
                box-sizing: border-box;
                margin: ${margin_md}px 0;
                padding: 0;
                color: ${color_text};
                font-size: ${font_size}px;
                font-variant: tabular-nums;
                line-height: ${line_height};
                list-style: none;
                border-top: 1px solid ${border_color};
                transition: all 0.2s;
            }

            .ant-divider-horizontal {
                display: flex;
                clear: both;
                width: 100%;
                min-width: 100%;
                margin: ${margin_lg}px 0;
            }

            .ant-divider-vertical {
                position: relative;
                top: -0.06em;
                display: inline-block;
                height: 0.9em;
                margin: 0 ${margin_xs}px;
                vertical-align: middle;
                border-top: 0;
                border-left: 1px solid ${border_color};
            }

            .ant-divider-small {
                margin: ${margin_xs}px 0;
            }

            .ant-divider-large {
                margin: ${margin_xl}px 0;
            }

            .ant-divider-dashed {
                background: none;
                border-style: dashed;
                border-width: 1px 0 0;
            }

            .ant-divider-with-text {
                display: flex;
                margin: ${margin_md}px 0;
                color: ${color_heading};
                font-weight: 500;
                font-size: ${font_size_lg}px;
                white-space: nowrap;
                text-align: center;
                border-top: 0;
                border-top-color: ${border_color};
            }

            .ant-divider-plain {
                font-weight: normal;
            }

            .ant-divider-with-text::before,
            .ant-divider-with-text::after {
                position: relative;
                top: 50%;
                width: 50%;
                border-top: 1px solid transparent;
                border-top-color: inherit;
                border-bottom: 0;
                transform: translateY(50%);
                content: "";
            }

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
                padding: 0 ${padding_md}px;
            }

            .ant-divider-rtl {
                direction: rtl;
            }

            .ant-divider-horizontal.ant-divider-with-text-left.ant-divider-rtl::before {
                width: 95%;
            }

            .ant-divider-horizontal.ant-divider-with-text-left.ant-divider-rtl::after {
                width: 5%;
            }

            .ant-divider-horizontal.ant-divider-with-text-right.ant-divider-rtl::before {
                width: 5%;
            }

            .ant-divider-horizontal.ant-divider-with-text-right.ant-divider-rtl::after {
                width: 95%;
            }

            /* 高对比度模式支持 */
            @media (prefers-contrast: high) {
                .ant-divider {
                    border-color: black;
                }
            }

            /* 减少动画模式支持 */
            @media (prefers-reduced-motion: reduce) {
                .ant-divider {
                    transition: none;
                }
            }
            "#,
            margin_xs = self.token.margin_xs,
            margin_md = self.token.margin_md,
            margin_lg = self.token.margin_lg,
            margin_xl = self.token.margin_lg * 2,
            color_text = self.token.color_text,
            font_size = self.token.font_size,
            line_height = self.token.line_height,
            border_color = self.token.color_split,
            color_heading = self.token.color_text_heading,
            font_size_lg = self.token.font_size_lg,
            padding_md = self.token.padding_md
        )
        .to_string()
    }
}

/// 生成分割线样式
pub fn generate_divider_style(
    divider_type: DividerType,
    size: DividerSize,
    dashed: bool,
    plain: bool,
    has_text: bool,
    orientation: DividerOrientation,
) -> String {
    DividerStyleGenerator::new()
        .with_type(divider_type)
        .with_size(size)
        .with_dashed(dashed)
        .with_plain(plain)
        .with_has_text(has_text)
        .with_orientation(orientation)
        .generate()
}

/// 生成分割线CSS样式
pub fn generate_divider_css(
    divider_type: DividerType,
    size: DividerSize,
    dashed: bool,
    plain: bool,
    has_text: bool,
    orientation: DividerOrientation,
) -> String {
    DividerStyleGenerator::new()
        .with_type(divider_type)
        .with_size(size)
        .with_dashed(dashed)
        .with_plain(plain)
        .with_has_text(has_text)
        .with_orientation(orientation)
        .generate_css()
}

/// 默认分割线样式
pub fn default_divider_style() -> String {
    DividerStyleGenerator::new().generate()
}

//!
//! Flex组件样式管理
//!
//! 使用样式生成器模式管理Flex组件的样式
//! 使用 css! 宏的样式生成器模式，提供 Flex 组件的完整样式生成功能

use crate::components::flex::types::*;
use crate::shared::styles::mixins::*;
use css_in_rust::css;

/// Flex样式生成器
pub struct FlexStyleGenerator {
    pub vertical: bool,
    pub wrap: FlexWrap,
    pub justify: FlexJustify,
    pub align: FlexAlign,
    pub gap: Option<FlexGap>,
    pub flex: Option<String>,
}

impl FlexStyleGenerator {
    /// 创建新的Flex样式生成器
    pub fn new() -> Self {
        Self {
            vertical: false,
            wrap: FlexWrap::NoWrap,
            justify: FlexJustify::Normal,
            align: FlexAlign::Normal,
            gap: None,
            flex: None,
        }
    }

    /// 设置方向
    pub fn with_vertical(mut self, vertical: bool) -> Self {
        self.vertical = vertical;
        self
    }

    /// 设置换行
    pub fn with_wrap(mut self, wrap: FlexWrap) -> Self {
        self.wrap = wrap;
        self
    }

    /// 设置主轴对齐
    pub fn with_justify(mut self, justify: FlexJustify) -> Self {
        self.justify = justify;
        self
    }

    /// 设置交叉轴对齐
    pub fn with_align(mut self, align: FlexAlign) -> Self {
        self.align = align;
        self
    }

    /// 设置间距
    pub fn with_gap(mut self, gap: Option<FlexGap>) -> Self {
        self.gap = gap;
        self
    }

    /// 设置flex属性
    pub fn with_flex(mut self, flex: Option<String>) -> Self {
        self.flex = flex;
        self
    }

    /// 生成完整的Flex样式类名
    pub fn generate(&self) -> String {
        let mut classes = vec![self.base_style()];

        if self.vertical {
            classes.push(self.vertical_style());
        }

        classes.push(self.wrap_style());
        classes.push(self.justify_style());
        classes.push(self.align_style());

        if self.gap.is_some() {
            classes.push(self.gap_style());
        }

        if self.flex.is_some() {
            classes.push(self.flex_style());
        }

        classes.join(" ")
    }

    /// 基础Flex样式
    fn base_style(&self) -> String {
        css!(
            r#"
            display: flex;
            font-size: 14px;
            line-height: 1.5715;
            color: rgba(0, 0, 0, 0.88);
            transition: all 0.2s;
            "#
        )
        .to_string()
    }

    /// 垂直方向样式
    fn vertical_style(&self) -> String {
        css!(
            r#"
            flex-direction: column;
            "#
        )
        .to_string()
    }

    /// 换行样式
    fn wrap_style(&self) -> String {
        let wrap_value = match self.wrap {
            FlexWrap::NoWrap => "nowrap",
            FlexWrap::Wrap => "wrap",
            FlexWrap::WrapReverse => "wrap-reverse",
        };

        css!(
            r#"
            flex-wrap: {};
            "#,
            wrap_value
        )
        .to_string()
    }

    /// 主轴对齐样式
    fn justify_style(&self) -> String {
        if self.justify == FlexJustify::Normal {
            return String::new();
        }

        let justify_value = match self.justify {
            FlexJustify::Normal => return String::new(),
            FlexJustify::FlexStart => "flex-start",
            FlexJustify::FlexEnd => "flex-end",
            FlexJustify::Center => "center",
            FlexJustify::SpaceBetween => "space-between",
            FlexJustify::SpaceAround => "space-around",
            FlexJustify::SpaceEvenly => "space-evenly",
        };

        css!(
            r#"
            justify-content: {};
            "#,
            justify_value
        )
        .to_string()
    }

    /// 交叉轴对齐样式
    fn align_style(&self) -> String {
        if self.align == FlexAlign::Normal {
            return String::new();
        }

        let align_value = match self.align {
            FlexAlign::Normal => return String::new(),
            FlexAlign::FlexStart => "flex-start",
            FlexAlign::FlexEnd => "flex-end",
            FlexAlign::Center => "center",
            FlexAlign::Baseline => "baseline",
            FlexAlign::Stretch => "stretch",
        };

        css!(
            r#"
            align-items: {};
            "#,
            align_value
        )
        .to_string()
    }

    /// 间距样式
    fn gap_style(&self) -> String {
        if let Some(ref gap) = self.gap {
            let gap_value = match gap {
                FlexGap::Small => "8px",
                FlexGap::Middle => "16px",
                FlexGap::Large => "24px",
                FlexGap::Custom(value) => value,
                FlexGap::Array(values) => {
                    return css!(
                        r#"
                        row-gap: {};
                        column-gap: {};
                        "#,
                        values[1], values[0]
                    ).to_string();
                }
            };

            css!(
                r#"
                gap: {};
                "#,
                gap_value
            )
            .to_string()
        } else {
            String::new()
        }
    }

    /// Flex属性样式
    fn flex_style(&self) -> String {
        if let Some(ref flex_value) = self.flex {
            css!(
                r#"
                flex: {};
                "#,
                flex_value
            )
            .to_string()
        } else {
            String::new()
        }
    }
}

impl Default for FlexStyleGenerator {
    fn default() -> Self {
        Self::new()
    }
}

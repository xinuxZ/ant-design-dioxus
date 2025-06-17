//!
//! Space组件样式管理
//!
//! 使用样式生成器模式管理Space组件的样式
//! 使用 css! 宏的样式生成器模式，提供 Space 组件的完整样式生成功能

use crate::components::space::types::*;
use crate::shared::styles::mixins::*;
use css_in_rust::css;

/// Space样式生成器
pub struct SpaceStyleGenerator {
    pub direction: SpaceDirection,
    pub size: SpaceSizeConfig,
    pub align: Option<SpaceAlign>,
    pub wrap: bool,
    pub split: bool,
}

impl SpaceStyleGenerator {
    /// 创建新的Space样式生成器
    pub fn new() -> Self {
        Self {
            direction: SpaceDirection::Horizontal,
            size: SpaceSizeConfig::Single(SpaceSize::Small),
            align: None,
            wrap: false,
            split: false,
        }
    }

    /// 设置方向
    pub fn with_direction(mut self, direction: SpaceDirection) -> Self {
        self.direction = direction;
        self
    }

    /// 设置间距大小
    pub fn with_size(mut self, size: SpaceSizeConfig) -> Self {
        self.size = size;
        self
    }

    /// 设置对齐方式
    pub fn with_align(mut self, align: Option<SpaceAlign>) -> Self {
        self.align = align;
        self
    }

    /// 设置是否换行
    pub fn with_wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }

    /// 设置是否有分割线
    pub fn with_split(mut self, split: bool) -> Self {
        self.split = split;
        self
    }

    /// 生成完整的Space样式类名
    pub fn generate(&self) -> String {
        let mut classes = vec![self.base_style()];

        classes.push(self.direction_style());
        classes.push(self.size_style());

        if let Some(align) = &self.align {
            classes.push(self.align_style(align));
        }

        if self.wrap {
            classes.push(self.wrap_style());
        }

        classes.join(" ")
    }

    /// 生成Space项目样式类名
    pub fn generate_item(&self) -> String {
        let mut classes = vec![self.item_base_style()];

        if self.split {
            classes.push(self.item_split_style());
        }

        classes.join(" ")
    }

    /// 基础Space样式
    fn base_style(&self) -> String {
        css!(
            r#"
            display: inline-flex;
            "#
        )
        .to_string()
    }

    /// 方向样式
    fn direction_style(&self) -> String {
        match self.direction {
            SpaceDirection::Horizontal => css!(
                r#"
                flex-direction: row;
                "#
            ),
            SpaceDirection::Vertical => css!(
                r#"
                flex-direction: column;
                "#
            ),
        }
        .to_string()
    }

    /// 间距大小样式
    fn size_style(&self) -> String {
        let gap_value = match &self.size {
            SpaceSizeConfig::Single(space_size) => match space_size {
                SpaceSize::Small => "8px",
                SpaceSize::Middle => "16px",
                SpaceSize::Large => "24px",
                SpaceSize::Custom(size) => {
                    &format!("{}px", size)
                }
            },
            SpaceSizeConfig::Array([horizontal, vertical]) => {
                // 对于数组类型，使用水平间距作为默认值
                &format!("{}px", horizontal)
            },
            SpaceSizeConfig::String(s) => {
                // 对于字符串类型，直接使用
                s
            }
        };

        css!(
            r#"
            gap: {};
            "#,
            gap_value
        )
        .to_string()
    }

    /// 对齐方式样式
    fn align_style(&self, align: &SpaceAlign) -> String {
        match align {
            SpaceAlign::Start => css!(
                r#"
                align-items: flex-start;
                "#
            ),
            SpaceAlign::End => css!(
                r#"
                align-items: flex-end;
                "#
            ),
            SpaceAlign::Center => css!(
                r#"
                align-items: center;
                "#
            ),
            SpaceAlign::Baseline => css!(
                r#"
                align-items: baseline;
                "#
            ),
        }
        .to_string()
    }

    /// 换行样式
    fn wrap_style(&self) -> String {
        css!(
            r#"
            flex-wrap: wrap;
            "#
        )
        .to_string()
    }

    /// Space项目基础样式
    fn item_base_style(&self) -> String {
        css!(
            r#"
            display: inline-block;
            "#
        )
        .to_string()
    }

    /// Space项目分割线样式
    fn item_split_style(&self) -> String {
        match self.direction {
            SpaceDirection::Horizontal => css!(
                r#"
                position: relative;
                
                &:not(:last-child)::after {
                    content: '';
                    position: absolute;
                    top: 50%;
                    right: -50%;
                    transform: translateY(-50%);
                    width: 1px;
                    height: 100%;
                    background-color: #d9d9d9;
                }
                "#
            ),
            SpaceDirection::Vertical => css!(
                r#"
                position: relative;
                
                &:not(:last-child)::after {
                    content: '';
                    position: absolute;
                    bottom: -50%;
                    left: 0;
                    width: 100%;
                    height: 1px;
                    background-color: #d9d9d9;
                }
                "#
            ),
        }
        .to_string()
    }
}

impl Default for SpaceStyleGenerator {
    fn default() -> Self {
        Self::new()
    }
}
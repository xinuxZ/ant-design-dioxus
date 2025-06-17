//! Divider 组件样式生成器
//!
//! 使用 css! 宏的样式生成器模式，提供 Divider 组件的完整样式生成功能

use crate::components::divider::types::*;
use crate::shared::styles::mixins::*;
use css_in_rust::css;

/// Divider 样式生成器
pub struct DividerStyleGenerator {
    pub r#type: DividerType,
    pub orientation: DividerOrientation,
    pub size: DividerSize,
    pub variant: DividerVariant,
    pub plain: bool,
    pub has_text: bool,
    pub orientation_margin: Option<String>,
    pub class: Option<String>,
}

impl DividerStyleGenerator {
    /// 创建新的 Divider 样式生成器
    pub fn new() -> Self {
        Self {
            r#type: DividerType::Horizontal,
            orientation: DividerOrientation::Center,
            size: DividerSize::Middle,
            variant: DividerVariant::Solid,
            plain: false,
            has_text: false,
            orientation_margin: None,
            class: None,
        }
    }

    /// 设置分割线类型
    pub fn with_type(mut self, divider_type: DividerType) -> Self {
        self.r#type = divider_type;
        self
    }

    /// 设置文本方向
    pub fn with_orientation(mut self, orientation: DividerOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// 设置尺寸
    pub fn with_size(mut self, size: DividerSize) -> Self {
        self.size = size;
        self
    }

    /// 设置样式变体
    pub fn with_variant(mut self, variant: DividerVariant) -> Self {
        self.variant = variant;
        self
    }

    /// 设置朴素样式
    pub fn with_plain(mut self, plain: bool) -> Self {
        self.plain = plain;
        self
    }

    /// 设置是否有文本
    pub fn with_text(mut self, has_text: bool) -> Self {
        self.has_text = has_text;
        self
    }

    /// 设置自定义类名
    pub fn with_class(mut self, class: Option<String>) -> Self {
        self.class = class;
        self
    }

    /// 生成完整的 CSS 类名字符串
    pub fn generate(&self) -> String {
        let mut classes = Vec::new();

        // 基础样式
        classes.push(self.base_style());

        // 类型样式
        classes.push(self.type_style());

        // 尺寸样式
        if self.size != DividerSize::Middle {
            classes.push(self.size_style());
        }

        // 变体样式
        classes.push(self.generate_variant_style());

        // 文本相关样式
        if self.has_text {
            classes.push(self.text_style());
            classes.push(self.generate_orientation_style());

            if self.plain {
                classes.push(self.plain_style());
            }
        }

        classes.join(" ")
    }

    /// 基础分割线样式
    fn base_style(&self) -> String {
        css!(
            r#"
            box-sizing: border-box;
            position: relative;
            display: flex;
            align-items: center;
            transition: all 0.3s;
            "#
        )
    }

    /// 分割线类型样式
    fn type_style(&self) -> String {
        match self.r#type {
            DividerType::Horizontal => css!(
                r#"
                flex-direction: row;
                width: 100%;
                margin: 16px 0;

                &::before,
                &::after {
                    content: '';
                    flex: 1;
                    height: 0;
                    border-top: 1px solid var(--ant-color-border);
                }
                "#
            )
            .to_string(),
            DividerType::Vertical => css!(
                r#"
                flex-direction: column;
                height: 0.9em;
                margin: 0 8px;
                display: inline-flex;
                vertical-align: middle;

                &::before {
                    content: '';
                    width: 0;
                    height: 100%;
                    border-left: 1px solid var(--ant-color-border);
                }
                "#
            )
            .to_string(),
        }
    }

    /// 尺寸样式
    fn size_style(&self) -> String {
        match self.size {
            DividerSize::Small => css!(
                r#"
                margin: 8px 0;
                font-size: 12px;
                "#
            )
            .to_string(),
            DividerSize::Large => css!(
                r#"
                margin: 24px 0;
                font-size: 16px;
                "#
            )
            .to_string(),
            DividerSize::Middle => css!("").to_string(), // 默认样式
        }
    }

    /// 生成变体样式
    fn generate_variant_style(&self) -> String {
        match self.variant {
            DividerVariant::Dashed => css!(
                r#"
                &::before,
                &::after {
                    border-style: dashed !important;
                }
                "#
            )
            .to_string(),
            DividerVariant::Dotted => css!(
                r#"
                &::before,
                &::after {
                    border-style: dotted !important;
                }
                "#
            )
            .to_string(),
            DividerVariant::Solid => css!("").to_string(), // 默认样式
        }
    }

    /// 文本样式
    fn text_style(&self) -> String {
        css!(
            r#"
            &::before,
            &::after {
                position: relative;
                top: 50%;
                width: 50%;
                transform: translateY(50%);
            }

            .ant-divider-inner-text {
                display: inline-block;
                padding: 0 1em;
                color: var(--ant-color-text-heading);
                font-weight: 500;
                font-size: 16px;
                white-space: nowrap;
                text-align: center;
                background: var(--ant-color-bg-container);
                line-height: 1.5715;
            }
            "#
        )
    }

    /// 生成方向样式
    fn generate_orientation_style(&self) -> String {
        match self.orientation {
            DividerOrientation::Left => css!(
                r#"
                &.ant-divider-with-text-left {
                    &::before {
                        width: 5%;
                    }
                    &::after {
                        width: 95%;
                    }
                }
                "#
            ),
            DividerOrientation::Right => css!(
                r#"
                &.ant-divider-with-text-right {
                    &::before {
                        width: 95%;
                    }
                    &::after {
                        width: 5%;
                    }
                }
                "#
            ),
            DividerOrientation::Center => css!(
                r#"
                &.ant-divider-with-text-center {
                    &::before {
                        width: 50%;
                    }
                    &::after {
                        width: 50%;
                    }
                }
                "#
            ),
            DividerOrientation::Start | DividerOrientation::End => css!(
                r#"
                /* RTL支持的方向样式将在后续实现 */
                "#
            ),
        }
    }

    /// 朴素样式
    fn plain_style(&self) -> String {
        css!(
            r#"
            .ant-divider-inner-text {
                color: var(--ant-color-text);
                font-weight: normal;
                font-size: 14px;
            }
            "#
        )
    }
}

/// 生成 CSS 类名
pub fn generate_divider_class_name(
    divider_type: &DividerType,
    orientation: &DividerOrientation,
    size: &DividerSize,
    variant: &DividerVariant,
    plain: bool,
    has_text: bool,
    custom_class: Option<&str>,
) -> String {
    let mut classes = vec!["ant-divider".to_string()];

    // 类型
    classes.push(format!("ant-divider-{}", divider_type));

    // 样式变体
    if *variant != DividerVariant::Solid {
        classes.push(format!("ant-divider-{}", variant));
    }

    // 尺寸
    if *size != DividerSize::Middle {
        classes.push(format!("ant-divider-{}", size));
    }

    // 文本相关
    if has_text {
        classes.push("ant-divider-with-text".to_string());
        classes.push(format!("ant-divider-with-text-{}", orientation));

        if plain {
            classes.push("ant-divider-plain".to_string());
        }
    }

    // 自定义类名
    if let Some(custom_class) = custom_class {
        classes.push(custom_class.to_string());
    }

    classes.join(" ")
}

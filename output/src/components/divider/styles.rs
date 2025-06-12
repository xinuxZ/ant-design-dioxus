//! Divider 组件样式

//!
//! Divider 组件样式定义
//!
//! 包含 Divider 组件的所有样式生成逻辑。

use css_in_rust::{css, Style};
use super::types::*;
use crate::theme::AliasToken;

/// Divider 样式
#[derive(Debug, Clone)]
pub struct DividerStyles {
    /// 基础样式
    pub base: String,
    /// 变体样式
    pub variants: VariantStyles,
}

/// 变体样式
#[derive(Debug, Clone)]
pub struct VariantStyles {
    /// 带文字类型样式
    pub type_text: String,
    /// 虚线类型样式
    pub type_dashed: String,
    /// 点线类型样式
    pub type_dotted: String,
    /// 小尺寸样式
    pub size_small: String,
    /// 大尺寸样式
    pub size_large: String,
}

/// Divider 样式生成器
#[derive(Debug, Clone)]
pub struct DividerStyleGenerator {
    /// 分割线类型
    pub divider_type: DividerType,
    /// 分割线尺寸
    pub size: DividerSize,
    /// 分割线变体
    pub variant: DividerVariant,
    /// 是否简洁模式
    pub plain: bool,
    /// 是否有文字
    pub has_text: bool,
    /// 文字位置
    pub orientation: DividerOrientation,
    /// 方向边距
    pub orientation_margin: Option<OrientationMargin>,
    /// 前缀类名
    pub prefix_cls: String,
    /// 主题令牌
    pub token: AliasToken,
}

impl Default for DividerStyleGenerator {
    fn default() -> Self {
        Self {
            divider_type: DividerType::default(),
            size: DividerSize::default(),
            variant: DividerVariant::default(),
            plain: false,
            has_text: false,
            orientation: DividerOrientation::default(),
            orientation_margin: None,
            prefix_cls: "ant".to_string(),
            token: AliasToken::default(),
        }
    }
}

impl DividerStyleGenerator {
    /// 创建新的样式生成器
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

    /// 设置分割线变体
    pub fn with_variant(mut self, variant: DividerVariant) -> Self {
        self.variant = variant;
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

    /// 设置方向边距
    pub fn with_orientation_margin(mut self, margin: Option<OrientationMargin>) -> Self {
        self.orientation_margin = margin;
        self
    }

    /// 设置前缀类名
    pub fn with_prefix_cls(mut self, prefix_cls: String) -> Self {
        self.prefix_cls = prefix_cls;
        self
    }

    /// 设置主题令牌
    pub fn with_token(mut self, token: AliasToken) -> Self {
        self.token = token;
        self
    }

    /// 生成样式类名
    pub fn generate_class_names(&self) -> Vec<String> {
        let mut classes = vec![format!("{}-divider", self.prefix_cls)];

        // 添加类型相关的类名
        match self.divider_type {
            DividerType::Horizontal => {
                classes.push(format!("{}-divider-horizontal", self.prefix_cls));
                if self.has_text {
                    classes.push(format!("{}-divider-with-text", self.prefix_cls));
                    match self.orientation {
                        DividerOrientation::Left => classes.push(format!("{}-divider-with-text-left", self.prefix_cls)),
                        DividerOrientation::Center => classes.push(format!("{}-divider-with-text-center", self.prefix_cls)),
                        DividerOrientation::Right => classes.push(format!("{}-divider-with-text-right", self.prefix_cls)),
                    }
                }
            }
            DividerType::Vertical => {
                classes.push(format!("{}-divider-vertical", self.prefix_cls));
            }
        }

        // 添加尺寸相关的类名
        match self.size {
            DividerSize::Small => classes.push(format!("{}-divider-small", self.prefix_cls)),
            DividerSize::Default => {},
            DividerSize::Large => classes.push(format!("{}-divider-large", self.prefix_cls)),
        }

        // 添加变体相关的类名
        match self.variant {
            DividerVariant::Solid => {},
            DividerVariant::Dashed => classes.push(format!("{}-divider-dashed", self.prefix_cls)),
            DividerVariant::Dotted => classes.push(format!("{}-divider-dotted", self.prefix_cls)),
        }

        // 添加简洁模式相关的类名
        if self.plain {
            classes.push(format!("{}-divider-plain", self.prefix_cls));
        }

        classes
    }

    /// 生成方向边距样式
    pub fn generate_orientation_margin_style(&self) -> Option<String> {
        if let Some(margin) = &self.orientation_margin {
            let margin_value = match margin {
                OrientationMargin::Pixels(px) => format!("{}px", px),
                OrientationMargin::Percentage(pct) => format!("{}%", pct),
            };
            
            match self.orientation {
                DividerOrientation::Left => Some(format!(
                    ".{}-divider-with-text-left::before {{ width: {}; }} .{}-divider-with-text-left::after {{ width: calc(100% - {}); }}",
                    self.prefix_cls, margin_value, self.prefix_cls, margin_value
                )),
                DividerOrientation::Right => Some(format!(
                    ".{}-divider-with-text-right::before {{ width: calc(100% - {}); }} .{}-divider-with-text-right::after {{ width: {}; }}",
                    self.prefix_cls, margin_value, self.prefix_cls, margin_value
                )),
                DividerOrientation::Center => Some(format!(
                    ".{}-divider-inner-text {{ margin: 0 {}; }}",
                    self.prefix_cls, margin_value
                )),
            }
        } else {
            None
        }
    }
}

/// 生成 Divider 样式
pub fn generate_divider_styles() -> DividerStyles {
    // 重用迁移后的样式，并添加新的变体样式
    let base_styles = super::migrated_styles::generate_divider_styles();
    
    let type_dotted = css!(r#"
        .ant-divider-dotted {
            background: none;
            border-style: dotted;
            border-width: 1px 0 0;
        }
    "#);
    
    DividerStyles {
        base: base_styles.base,
        variants: VariantStyles {
            type_text: base_styles.variants.type_text,
            type_dashed: base_styles.variants.type_dashed,
            type_dotted,
            size_small: base_styles.variants.size_small,
            size_large: base_styles.variants.size_large,
        },
    }
}

//! Divider 组件样式
//! 
//! 此文件由样式迁移工具自动生成，请勿手动修改

use css_in_rust::css;

/// Divider 组件样式
#[derive(Debug, Clone)]
pub struct DividerStyles {
    pub base: String,
    pub variants: VariantStyles,
}

/// 变体样式
#[derive(Debug, Clone)]
pub struct VariantStyles {
    pub type_text: String,
    pub type_dashed: String,
    pub type_dotted: String,
    pub size_small: String,
    pub size_large: String,
}

/// Divider 样式生成器
#[derive(Debug, Clone)]
pub struct DividerStyleGenerator {
    pub divider_type: DividerType,
    pub size: DividerSize,
    pub variant: DividerVariant,
    pub plain: bool,
    pub has_text: bool,
    pub orientation: DividerOrientation,
    pub orientation_margin: Option<OrientationMargin>,
    pub prefix_cls: String,
    pub token: Option<crate::theme::AliasToken>,
}

impl DividerStyleGenerator {
    pub fn new() -> Self {
        Self {
            divider_type: DividerType::Horizontal,
            size: DividerSize::Default,
            variant: DividerVariant::Solid,
            plain: false,
            has_text: false,
            orientation: DividerOrientation::Center,
            orientation_margin: None,
            prefix_cls: "ant".to_string(),
            token: None,
        }
    }
    
    pub fn with_type(mut self, divider_type: DividerType) -> Self {
        self.divider_type = divider_type;
        self
    }
    
    pub fn with_size(mut self, size: DividerSize) -> Self {
        self.size = size;
        self
    }
    
    pub fn with_variant(mut self, variant: DividerVariant) -> Self {
        self.variant = variant;
        self
    }
    
    pub fn with_plain(mut self, plain: bool) -> Self {
        self.plain = plain;
        self
    }
    
    pub fn with_has_text(mut self, has_text: bool) -> Self {
        self.has_text = has_text;
        self
    }
    
    pub fn with_orientation(mut self, orientation: DividerOrientation) -> Self {
        self.orientation = orientation;
        self
    }
    
    pub fn with_orientation_margin(mut self, margin: Option<OrientationMargin>) -> Self {
        self.orientation_margin = margin;
        self
    }
    
    pub fn with_prefix_cls(mut self, prefix_cls: String) -> Self {
        self.prefix_cls = prefix_cls;
        self
    }
    
    pub fn with_token(mut self, token: Option<crate::theme::AliasToken>) -> Self {
        self.token = token;
        self
    }
    
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
            DividerVariant::Dashed => classes.push(format!("{}-divider-dashed", self.prefix_cls)),
            DividerVariant::Dotted => classes.push(format!("{}-divider-dotted", self.prefix_cls)),
            DividerVariant::Solid => {},
        }
        
        // 添加简洁模式相关的类名
        if self.plain {
            classes.push(format!("{}-divider-plain", self.prefix_cls));
        }
        
        classes
    }
    
    pub fn generate_inline_styles(&self) -> String {
        let mut styles = Vec::new();
        
        // 添加方向边距样式
        if let Some(margin) = &self.orientation_margin {
            if self.has_text && matches!(self.divider_type, DividerType::Horizontal) {
                let margin_value = match margin {
                    OrientationMargin::Pixels(px) => format!("{}px", px),
                    OrientationMargin::Percentage(pct) => format!("{}%", pct),
                };
                
                match self.orientation {
                    DividerOrientation::Left => {
                        styles.push(format!("margin-left: {}", margin_value));
                    }
                    DividerOrientation::Right => {
                        styles.push(format!("margin-right: {}", margin_value));
                    }
                    DividerOrientation::Center => {
                        styles.push(format!("margin-left: {}; margin-right: {}", margin_value, margin_value));
                    }
                }
            }
        }
        
        styles.join("; ")
    }
    
    pub fn generate_orientation_margin_style(&self) -> Option<String> {
        if let Some(margin) = &self.orientation_margin {
            if self.has_text && matches!(self.divider_type, DividerType::Horizontal) {
                let margin_value = match margin {
                    OrientationMargin::Pixels(px) => format!("{}px", px),
                    OrientationMargin::Percentage(pct) => format!("{}%", pct),
                };
                
                let selector = match self.orientation {
                    DividerOrientation::Left => format!(".{}-divider-with-text-left .{}-divider-inner-text", self.prefix_cls, self.prefix_cls),
                    DividerOrientation::Right => format!(".{}-divider-with-text-right .{}-divider-inner-text", self.prefix_cls, self.prefix_cls),
                    DividerOrientation::Center => format!(".{}-divider-with-text-center .{}-divider-inner-text", self.prefix_cls, self.prefix_cls),
                };
                
                let style = match self.orientation {
                    DividerOrientation::Left => format!("{} {{ margin-left: {}; }}", selector, margin_value),
                    DividerOrientation::Right => format!("{} {{ margin-right: {}; }}", selector, margin_value),
                    DividerOrientation::Center => format!("{} {{ margin-left: {}; margin-right: {}; }}", selector, margin_value, margin_value),
                };
                
                return Some(style);
            }
        }
        None
    }
}

impl DividerStyleGenerator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn generate(&self) -> DividerStyles {
        generate_divider_styles()
    }
}

/// 生成 Divider 组件样式
pub fn generate_divider_styles() -> DividerStyles {
    let base = css!(r#"
        .ant-divider {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Noto Sans', sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol', 'Noto Color Emoji';
            color: rgba(0, 0, 0, 0.88);
            list-style: none;
            padding: 0;
            box-sizing: border-box;
            border-block-start: 1px solid rgba(5, 5, 5, 0.06);
            line-height: 1.5714285714285714;
            font-size: 14px;
            margin: 0;
        }
        .ant-divider-horizontal {
            display: flex;
            margin: 24px 0;
            min-width: 100%;
            width: 100%;
            clear: both;
        }
        .ant-divider-vertical {
            position: relative;
            margin: 0 8px;
            border-inline-start: 1px solid rgba(5, 5, 5, 0.06);
            border-top: 0;
            top: -0.06em;
            height: 0.9em;
            display: inline-block;
            vertical-align: middle;
        }
        .ant-divider-rtl {
            direction: rtl;
        }
        .ant-divider {
            border-block-start-color: rgba(253, 253, 253, 0.12);
            color: rgba(255, 255, 255, 0.85);
        }
        .ant-divider-vertical {
            border-inline-start-color: rgba(253, 253, 253, 0.12);
        }
        .ant-divider {
            border-block-start-color: #000;
        }
        .ant-divider-vertical {
            border-inline-start-color: #000;
        }
        .ant-divider {
            border-block-start-color: #fff;
        }
        .ant-divider-vertical {
            border-inline-start-color: #fff;
        }
        .ant-divider {
            transition: none;
        }
        .ant-divider-horizontal {
            margin: 16px 0;
        }
    "#);

    let variants = VariantStyles {
        type_text: css!(r#"
        .ant-divider-horizontal.ant-divider-with-text {
            color: rgba(0, 0, 0, 0.88);
            align-items: center;
            text-align: center;
            font-weight: 500;
            white-space: nowrap;
            margin: 16px 0;
            display: flex;
            font-size: 16px;
            border-block-start: 0;
        }
        .ant-divider-horizontal.ant-divider-with-text::before,
.ant-divider-horizontal.ant-divider-with-text::after {
            border-block-start: 1px solid transparent;
            transform: translateY(50%);
            content: '';
            width: 50%;
            border-block-start-color: inherit;
            border-bottom: 0;
            position: relative;
            top: 50%;
        }
        .ant-divider-horizontal.ant-divider-with-text-left::before {
            top: 50%;
            width: 5%;
        }
        .ant-divider-horizontal.ant-divider-with-text-left::after {
            width: 95%;
            top: 50%;
        }
        .ant-divider-horizontal.ant-divider-with-text-right::before {
            top: 50%;
            width: 95%;
        }
        .ant-divider-horizontal.ant-divider-with-text-right::after {
            top: 50%;
            width: 5%;
        }
        .ant-divider-inner-text {
            display: inline-block;
            padding: 0 1em;
        }
        .ant-divider-plain.ant-divider-with-text {
            font-size: 14px;
            color: rgba(0, 0, 0, 0.45);
            font-weight: normal;
        }
        .ant-divider-rtl.ant-divider-with-text-left::before {
            width: 95%;
        }
        .ant-divider-rtl.ant-divider-with-text-left::after {
            width: 5%;
        }
        .ant-divider-rtl.ant-divider-with-text-right::before {
            width: 5%;
        }
        .ant-divider-rtl.ant-divider-with-text-right::after {
            width: 95%;
        }
        .ant-divider-with-text {
            color: rgba(255, 255, 255, 0.85);
        }
        .ant-divider-plain.ant-divider-with-text {
            color: rgba(255, 255, 255, 0.45);
        }
        .ant-divider-horizontal.ant-divider-with-text {
            margin: 12px 0;
            font-size: 14px;
        }
        "#),
        type_dashed: css!(r#"
        .ant-divider-dashed {
            background: none;
            border-style: dashed;
            border-width: 1px 0 0;
            border-color: rgba(5, 5, 5, 0.06);
        }
        .ant-divider-horizontal.ant-divider-dashed {
            border-block-start-style: dashed;
        }
        .ant-divider-vertical.ant-divider-dashed {
            border-top: 0;
            border-inline-start-style: dashed;
        }
        .ant-divider-dashed.ant-divider-with-text::before,
.ant-divider-dashed.ant-divider-with-text::after {
            border-style: dashed none none;
        }
        .ant-divider-dashed {
            border-color: rgba(253, 253, 253, 0.12);
        }
        .ant-divider-dashed {
            border-color: #000;
        }
        .ant-divider-dashed {
            border-color: #fff;
        }
        "#),
        type_dotted: css!(r#"
        .ant-divider-dotted {
            background: none;
            border-style: dotted;
            border-width: 1px 0 0;
            border-color: rgba(5, 5, 5, 0.06);
        }
        .ant-divider-horizontal.ant-divider-dotted {
            border-block-start-style: dotted;
        }
        .ant-divider-vertical.ant-divider-dotted {
            border-top: 0;
            border-inline-start-style: dotted;
        }
        .ant-divider-dotted.ant-divider-with-text::before,
.ant-divider-dotted.ant-divider-with-text::after {
            border-style: dotted none none;
        }
        .ant-divider-dotted {
            border-color: rgba(253, 253, 253, 0.12);
        }
        .ant-divider-dotted {
            border-color: #000;
        }
        .ant-divider-dotted {
            border-color: #fff;
        }
        "#),
        size_small: css!(r#"
        .ant-divider-small {
            margin: 16px 0;
        }
        .ant-divider-small.ant-divider-with-text {
            margin: 8px 0;
            font-size: 14px;
        }
        .ant-divider-small.ant-divider-vertical {
            height: 0.7em;
            margin: 0 6px;
        }
        "#),
        size_large: css!(r#"
        .ant-divider-large {
            margin: 32px 0;
        }
        .ant-divider-large.ant-divider-with-text {
            margin: 24px 0;
            font-size: 18px;
        }
        .ant-divider-large.ant-divider-vertical {
            margin: 0 12px;
            height: 1.1em;
        }
        .ant-divider-large {
            margin: 24px 0;
        }
        .ant-divider-large.ant-divider-with-text {
            margin: 16px 0;
            font-size: 16px;
        }
        "#),
    };

    DividerStyles {
        base,
        variants,
    }
}

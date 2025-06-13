//! Space 组件的样式实现
//!
//! 提供 Space 组件的 CSS-in-Rust 样式生成功能，包括基础样式、变体样式和内联样式生成。

use crate::theme::Theme;
use super::types::*;

/// Space 组件的样式定义
#[derive(Debug, Clone)]
pub struct SpaceStyles {
    /// 基础样式
    pub base: String,
    /// 变体样式
    pub variants: VariantStyles,
}

/// 变体样式定义
#[derive(Debug, Clone)]
pub struct VariantStyles {
    /// 方向样式
    pub direction: String,
    /// 对齐样式
    pub align: String,
    /// 尺寸样式
    pub size: String,
    /// 换行样式
    pub wrap: String,
    /// 分割样式
    pub split: String,
}

/// Space 样式生成器
#[derive(Debug, Clone)]
pub struct SpaceStyleGenerator {
    /// 间距方向
    direction: SpaceDirection,
    /// 间距大小
    size: SpaceSize,
    /// 对齐方式
    align: SpaceAlign,
    /// 是否换行
    wrap: bool,
    /// 是否有分割元素
    has_split: bool,
    /// 前缀类名
    prefix_cls: String,
    /// 主题令牌
    theme_token: Option<Theme>,
}

impl SpaceStyleGenerator {
    /// 创建新的样式生成器
    pub fn new() -> Self {
        Self {
            direction: SpaceDirection::default(),
            size: SpaceSize::default(),
            align: SpaceAlign::default(),
            wrap: false,
            has_split: false,
            prefix_cls: "ant".to_string(),
            theme_token: None,
        }
    }
    
    /// 设置方向
    pub fn with_direction(mut self, direction: SpaceDirection) -> Self {
        self.direction = direction;
        self
    }
    
    /// 设置尺寸
    pub fn with_size(mut self, size: SpaceSize) -> Self {
        self.size = size;
        self
    }
    
    /// 设置对齐方式
    pub fn with_align(mut self, align: SpaceAlign) -> Self {
        self.align = align;
        self
    }
    
    /// 设置是否换行
    pub fn with_wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }
    
    /// 设置是否有分割元素
    pub fn with_split(mut self, has_split: bool) -> Self {
        self.has_split = has_split;
        self
    }
    
    /// 设置前缀类名
    pub fn with_prefix_cls(mut self, prefix_cls: &str) -> Self {
        self.prefix_cls = prefix_cls.to_string();
        self
    }
    
    /// 设置主题令牌
    pub fn with_theme_token(mut self, theme: Theme) -> Self {
        self.theme_token = Some(theme);
        self
    }
    
    /// 生成CSS类名
    pub fn generate_class_name(&self) -> String {
        let mut classes = vec![format!("{}-space", self.prefix_cls)];
        
        // 添加方向类名
        classes.push(format!("{}-space-{}", self.prefix_cls, self.direction.to_class()));
        
        // 添加尺寸类名
        classes.push(format!("{}-space-{}", self.prefix_cls, self.size.to_class()));
        
        // 添加对齐类名
        classes.push(format!("{}-space-align-{}", self.prefix_cls, self.align.to_class()));
        
        // 添加换行类名
        if self.wrap {
            classes.push(format!("{}-space-wrap", self.prefix_cls));
        }
        
        // 添加分割类名
        if self.has_split {
            classes.push(format!("{}-space-split", self.prefix_cls));
        }
        
        classes.join(" ")
    }
    
    /// 生成内联样式
    pub fn generate_inline_styles(&self) -> String {
        let mut styles = Vec::new();
        
        // 如果是自定义尺寸，添加CSS变量
        if let SpaceSize::Custom(size) = self.size {
            styles.push(format!("--{}-space-gap: {}px", self.prefix_cls, size));
        }
        
        styles.join("; ")
    }
}

impl Default for SpaceStyleGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl SpaceStyles {
    /// 创建新的样式实例
    pub fn new() -> Self {
        Self {
            base: String::new(),
            variants: VariantStyles::new(),
        }
    }
    
    /// 获取基础样式
    pub fn base_styles(&self) -> String {
        generate_space_styles()
    }
    
    /// 获取变体样式
    pub fn variant_styles(&self) -> String {
        self.variants.generate_all()
    }
}

impl VariantStyles {
    /// 创建新的变体样式实例
    pub fn new() -> Self {
        Self {
            direction: String::new(),
            align: String::new(),
            size: String::new(),
            wrap: String::new(),
            split: String::new(),
        }
    }
    
    /// 生成所有变体样式
    pub fn generate_all(&self) -> String {
        // 由于样式已经在基础样式中定义，这里返回空字符串
        // 实际项目中可以根据需要添加额外的变体样式
        String::new()
    }
}

/// 生成 Space 组件的基础样式
fn generate_space_styles() -> String {
    r#"
    .ant-space {
        display: inline-flex;
        gap: 8px;
    }
    
    .ant-space-vertical {
        flex-direction: column;
    }
    
    .ant-space-horizontal {
        flex-direction: row;
    }
    
    .ant-space-align-start {
        align-items: flex-start;
    }
    
    .ant-space-align-end {
        align-items: flex-end;
    }
    
    .ant-space-align-center {
        align-items: center;
    }
    
    .ant-space-align-baseline {
        align-items: baseline;
    }
    
    .ant-space-wrap {
        flex-wrap: wrap;
    }
    
    .ant-space-small {
        gap: 8px;
    }
    
    .ant-space-middle {
        gap: 16px;
    }
    
    .ant-space-large {
        gap: 24px;
    }
    
    .ant-space-custom {
        gap: var(--ant-space-gap, 8px);
    }
    
    .ant-space-split {
        position: relative;
    }
    
    .ant-space-split > * + * {
        position: relative;
    }
    
    .ant-space-split > * + *::before {
        position: absolute;
        top: 50%;
        left: -8px;
        width: 1px;
        height: 100%;
        transform: translateY(-50%);
        content: "";
        background-color: rgba(5, 5, 5, 0.06);
    }
    
    .ant-space-vertical.ant-space-split > * + *::before {
        top: -8px;
        left: 50%;
        width: 100%;
        height: 1px;
        transform: translateX(-50%);
    }
    "#.to_string()
}
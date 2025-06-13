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
    /// 调试模式配置
    debug_config: SpaceDebugConfig,
    /// 动画配置
    animation_config: SpaceAnimationConfig,
    /// 性能优化配置
    performance_config: SpacePerformanceConfig,
    /// 国际化配置
    i18n_config: SpaceI18nConfig,
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
            debug_config: SpaceDebugConfig::default(),
            animation_config: SpaceAnimationConfig::default(),
            performance_config: SpacePerformanceConfig::default(),
            i18n_config: SpaceI18nConfig::default(),
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
    
    /// 设置调试模式配置
    pub fn with_debug_config(mut self, debug_config: SpaceDebugConfig) -> Self {
        self.debug_config = debug_config;
        self
    }
    
    /// 设置动画配置
    pub fn with_animation_config(mut self, animation_config: SpaceAnimationConfig) -> Self {
        self.animation_config = animation_config;
        self
    }
    
    /// 设置性能优化配置
    pub fn with_performance_config(mut self, performance_config: SpacePerformanceConfig) -> Self {
        self.performance_config = performance_config;
        self
    }
    
    /// 设置国际化配置
    pub fn with_i18n_config(mut self, i18n_config: SpaceI18nConfig) -> Self {
        self.i18n_config = i18n_config;
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
        
        // 添加调试模式类名
        if self.debug_config.enabled {
            classes.push(format!("{}-space-debug", self.prefix_cls));
            if self.debug_config.show_boundaries {
                classes.push(format!("{}-space-debug-boundaries", self.prefix_cls));
            }
            if self.debug_config.show_size_info {
                classes.push(format!("{}-space-debug-size-info", self.prefix_cls));
            }
        }
        
        // 添加动画类名
        if self.animation_config.enabled {
            classes.push(format!("{}-space-animated", self.prefix_cls));
            if self.animation_config.respect_reduced_motion {
                classes.push(format!("{}-space-respect-reduced-motion", self.prefix_cls));
            }
        }
        
        // 添加性能优化类名
        if self.performance_config.virtual_scroll {
            classes.push(format!("{}-space-virtual-scroll", self.prefix_cls));
        }
        if self.performance_config.lazy_loading {
            classes.push(format!("{}-space-lazy-loading", self.prefix_cls));
        }
        
        // 添加国际化类名
        if self.i18n_config.rtl {
            classes.push(format!("{}-space-rtl", self.prefix_cls));
        }
        if self.i18n_config.auto_direction {
            classes.push(format!("{}-space-auto-direction", self.prefix_cls));
        }
        
        classes.join(" ")
    }
    
    /// 生成内联样式
    pub fn generate_inline_styles(&self) -> String {
        let mut styles = Vec::new();
        
        // 处理不同类型的尺寸
        match &self.size {
            SpaceSize::Custom(size) => {
                styles.push(format!("--{}-space-gap: {}px", self.prefix_cls, size));
            }
            SpaceSize::Array(sizes) => {
                // 数组尺寸支持：第一个为水平间距，第二个为垂直间距
                if !sizes.is_empty() {
                    let horizontal_gap = match &sizes[0] {
                        SpaceSize::Small => "8px",
                        SpaceSize::Middle => "16px",
                        SpaceSize::Large => "24px",
                        SpaceSize::Custom(size) => &format!("{}px", size),
                        SpaceSize::Array(_) => "16px", // 嵌套数组使用默认值
                    };
                    
                    let vertical_gap = if sizes.len() > 1 {
                        match &sizes[1] {
                            SpaceSize::Small => "8px",
                            SpaceSize::Middle => "16px",
                            SpaceSize::Large => "24px",
                            SpaceSize::Custom(size) => &format!("{}px", size),
                            SpaceSize::Array(_) => "16px", // 嵌套数组使用默认值
                        }
                    } else {
                        horizontal_gap // 如果只有一个值，垂直间距使用相同值
                    };
                    
                    styles.push(format!("--{}-space-gap-horizontal: {}", self.prefix_cls, horizontal_gap));
                    styles.push(format!("--{}-space-gap-vertical: {}", self.prefix_cls, vertical_gap));
                }
            }
            _ => {} // 预设尺寸通过CSS类处理
        }
        
        // 添加调试模式样式
        if self.debug_config.enabled {
            if let Some(debug_color) = &self.debug_config.debug_color {
                styles.push(format!("--{}-space-debug-color: {}", self.prefix_cls, debug_color));
            }
        }
        
        // 添加动画样式
        if self.animation_config.enabled {
            styles.push(format!("--{}-space-animation-duration: {}ms", self.prefix_cls, self.animation_config.duration));
            styles.push(format!("--{}-space-animation-easing: {}", self.prefix_cls, self.animation_config.easing));
        }
        
        // 添加性能优化样式
        if self.performance_config.virtual_scroll {
            styles.push(format!("--{}-space-virtual-threshold: {}", self.prefix_cls, self.performance_config.virtual_scroll_threshold));
        }
        
        // 添加国际化样式
        if self.i18n_config.rtl {
            styles.push("direction: rtl".to_string());
        }
        
        styles.join("; ")
    }
    
    /// 生成完整的样式信息
    pub fn generate(&self) -> SpaceStyles {
        SpaceStyles {
            base: self.generate_class_name(),
            variants: VariantStyles::new(),
        }
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
    
    /* 调试模式样式 */
    .ant-space-debug {
        position: relative;
    }
    
    .ant-space-debug-boundaries {
        outline: 2px dashed var(--ant-space-debug-color, #ff4d4f);
        outline-offset: 2px;
    }
    
    .ant-space-debug-boundaries > * {
        outline: 1px solid var(--ant-space-debug-color, #ff4d4f);
        outline-offset: 1px;
    }
    
    .ant-space-debug-size-info::after {
        content: attr(data-space-size);
        position: absolute;
        top: -20px;
        left: 0;
        font-size: 12px;
        color: var(--ant-space-debug-color, #ff4d4f);
        background: rgba(255, 255, 255, 0.9);
        padding: 2px 4px;
        border-radius: 2px;
        z-index: 1000;
    }
    
    /* 动画效果样式 */
    .ant-space-animated {
        transition: gap var(--ant-space-animation-duration, 300ms) var(--ant-space-animation-easing, ease-in-out);
    }
    
    .ant-space-animated > * {
        transition: all var(--ant-space-animation-duration, 300ms) var(--ant-space-animation-easing, ease-in-out);
    }
    
    .ant-space-respect-reduced-motion {
        /* 遵循用户的减少动画偏好 */
    }
    
    @media (prefers-reduced-motion: reduce) {
        .ant-space-respect-reduced-motion,
        .ant-space-respect-reduced-motion > * {
            transition: none !important;
            animation: none !important;
        }
    }
    
    /* 性能优化样式 */
    .ant-space-virtual-scroll {
        overflow: hidden;
        contain: layout style paint;
    }
    
    .ant-space-lazy-loading > * {
        content-visibility: auto;
        contain-intrinsic-size: 0 50px;
    }
    
    /* 国际化样式 */
    .ant-space-rtl {
        direction: rtl;
    }
    
    .ant-space-rtl.ant-space-horizontal {
        flex-direction: row-reverse;
    }
    
    .ant-space-rtl .ant-space-split > * + *::before {
        left: auto;
        right: -8px;
    }
    
    .ant-space-auto-direction {
        /* 自动方向检测样式 */
    }
    
    [dir="rtl"] .ant-space-auto-direction {
        direction: rtl;
    }
    
    [dir="rtl"] .ant-space-auto-direction.ant-space-horizontal {
        flex-direction: row-reverse;
    }
    
    [dir="rtl"] .ant-space-auto-direction .ant-space-split > * + *::before {
        left: auto;
        right: -8px;
    }
    "#.to_string()
}
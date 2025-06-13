//!
//! Alert 组件样式定义
//!
//! 提供 Alert 组件的样式生成器和样式管理功能。

use css_in_rust::css;
use crate::theme::{use_theme, use_component_style, use_dark_mode, use_compact_mode, use_responsive};
use crate::theme::hooks::use_token_style;
use crate::utils::responsive::{Breakpoint, ResponsiveValue};
use std::collections::HashMap;
use super::types::*;

/// Alert 样式生成器
#[derive(Debug, Clone)]
pub struct AlertStyleGenerator {
    config: AlertStyleConfig,
    prefix_cls: String,
    theme_enabled: bool,
}

impl AlertStyleGenerator {
    /// 创建新的样式生成器
    pub fn new() -> Self {
        Self {
            config: AlertStyleConfig::default(),
            prefix_cls: "ant-alert".to_string(),
            theme_enabled: true,
        }
    }
    
    /// 设置是否启用主题
    pub fn with_theme_enabled(mut self, enabled: bool) -> Self {
        self.theme_enabled = enabled;
        self
    }

    /// 设置前缀类名
    pub fn with_prefix_cls(mut self, prefix_cls: String) -> Self {
        self.prefix_cls = prefix_cls;
        self
    }

    /// 设置 Alert 类型
    pub fn with_type(mut self, alert_type: AlertType) -> Self {
        self.config.alert_type = alert_type;
        self
    }

    /// 设置尺寸
    pub fn with_size(mut self, size: AlertSize) -> Self {
        self.config.size = size;
        self
    }

    /// 设置变体
    pub fn with_variant(mut self, variant: AlertVariant) -> Self {
        self.config.variant = variant;
        self
    }

    /// 设置是否显示图标
    pub fn with_show_icon(mut self, show_icon: bool) -> Self {
        self.config.show_icon = show_icon;
        self
    }

    /// 设置是否有描述
    pub fn with_description(mut self, has_description: bool) -> Self {
        self.config.has_description = has_description;
        self
    }

    /// 设置是否可关闭
    pub fn with_closable(mut self, closable: bool) -> Self {
        self.config.closable = closable;
        self
    }

    /// 设置是否为横幅模式
    pub fn with_banner(mut self, banner: bool) -> Self {
        self.config.banner = banner;
        self
    }

    /// 设置是否显示边框
    pub fn with_bordered(mut self, bordered: bool) -> Self {
        self.config.bordered = bordered;
        self
    }

    /// 设置是否禁用
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.config.disabled = disabled;
        self
    }

    /// 设置是否可见
    pub fn with_visible(mut self, visible: bool) -> Self {
        self.config.visible = visible;
        self
    }
    
    /// 生成响应式样式
    pub fn generate_responsive_styles(&self) -> String {
        let mut styles = Vec::new();
        
        // 响应式字体大小
        let font_size = ResponsiveValue::new("14px")
            .set(Breakpoint::XS, "12px")
            .set(Breakpoint::SM, "13px")
            .set(Breakpoint::MD, "14px")
            .set(Breakpoint::LG, "15px")
            .set(Breakpoint::XL, "16px");
            
        // 响应式内边距
        let padding = ResponsiveValue::new("8px 15px")
            .set(Breakpoint::XS, "6px 12px")
            .set(Breakpoint::SM, "7px 13px")
            .set(Breakpoint::MD, "8px 15px")
            .set(Breakpoint::LG, "10px 16px")
            .set(Breakpoint::XL, "12px 20px");
            
        // 生成媒体查询
        for breakpoint in [Breakpoint::XS, Breakpoint::SM, Breakpoint::MD, Breakpoint::LG, Breakpoint::XL, Breakpoint::XXL] {
            let min_width = crate::utils::responsive::get_breakpoint_min_width(breakpoint);
            let max_width = crate::utils::responsive::get_breakpoint_max_width(breakpoint);
            
            let media_query = if let Some(max) = max_width {
                format!("@media (min-width: {}px) and (max-width: {}px)", min_width, max)
            } else {
                format!("@media (min-width: {}px)", min_width)
            };
            
            let responsive_styles = format!(
                "{} {{ .{} {{ font-size: {}; padding: {}; }} }}",
                media_query,
                self.prefix_cls,
                font_size.values.get(&breakpoint).unwrap_or(&font_size.default),
                padding.values.get(&breakpoint).unwrap_or(&padding.default)
            );
            
            styles.push(responsive_styles);
        }
        
        format!("<style>{}</style>", styles.join("\n"))
    }

    /// 生成 CSS 类名列表
    pub fn generate_class_names(&self) -> Vec<String> {
        let mut classes = vec![self.prefix_cls.clone()];
        
        // 如果启用主题，添加主题相关的类名
        if self.theme_enabled {
            // 构建变体映射
            let mut variants = HashMap::new();
            
            // 添加类型变体
            let type_variant = match self.config.alert_type {
                AlertType::Success => "success",
                AlertType::Info => "info", 
                AlertType::Warning => "warning",
                AlertType::Error => "error",
            };
            variants.insert("type".to_string(), type_variant.to_string());
            
            // 添加尺寸变体
            let size_variant = match self.config.size {
                AlertSize::Small => "small",
                AlertSize::Default => "default",
                AlertSize::Large => "large",
            };
            variants.insert("size".to_string(), size_variant.to_string());
            
            // 添加变体类型
            let variant_type = match self.config.variant {
                AlertVariant::Filled => "filled",
                AlertVariant::Outlined => "outlined",
            };
            variants.insert("variant".to_string(), variant_type.to_string());
            
            // 使用主题系统生成类名
            let theme_classes = use_component_style("alert", Some(variants));
            classes.extend(theme_classes.split_whitespace().map(|s| s.to_string()));
        }

        // 添加类型相关类名（保持向后兼容）
        match self.config.alert_type {
            AlertType::Success => classes.push(format!("{}-success", self.prefix_cls)),
            AlertType::Info => classes.push(format!("{}-info", self.prefix_cls)),
            AlertType::Warning => classes.push(format!("{}-warning", self.prefix_cls)),
            AlertType::Error => classes.push(format!("{}-error", self.prefix_cls)),
        }

        // 添加尺寸相关类名
        match self.config.size {
            AlertSize::Small => classes.push(format!("{}-sm", self.prefix_cls)),
            AlertSize::Default => {}, // 默认尺寸不添加额外类名
            AlertSize::Large => classes.push(format!("{}-lg", self.prefix_cls)),
        }

        // 添加变体相关类名
        match self.config.variant {
            AlertVariant::Filled => {}, // 默认填充样式
            AlertVariant::Outlined => classes.push(format!("{}-outlined", self.prefix_cls)),
        }

        // 添加状态相关类名
        if self.config.show_icon {
            classes.push(format!("{}-with-icon", self.prefix_cls));
        }

        if self.config.has_description {
            classes.push(format!("{}-with-description", self.prefix_cls));
        }

        if self.config.closable {
            classes.push(format!("{}-closable", self.prefix_cls));
        }

        if self.config.banner {
            classes.push(format!("{}-banner", self.prefix_cls));
        }

        if !self.config.bordered {
            classes.push(format!("{}-no-border", self.prefix_cls));
        }

        if self.config.disabled {
            classes.push(format!("{}-disabled", self.prefix_cls));
        }

        if !self.config.visible {
            classes.push(format!("{}-hidden", self.prefix_cls));
        }

        classes
    }

    /// 生成内联样式
    pub fn generate_inline_styles(&self) -> String {
        let mut styles = Vec::new();
        
        // 如果启用主题，使用主题令牌
        if self.theme_enabled {
            // 基础样式令牌
            let mut base_tokens = HashMap::new();
            base_tokens.insert("padding", "alert-padding");
            base_tokens.insert("border-radius", "alert-border-radius");
            base_tokens.insert("font-size", "alert-font-size");
            base_tokens.insert("line-height", "alert-line-height");
            
            // 根据类型添加颜色令牌
            match self.config.alert_type {
                AlertType::Success => {
                    base_tokens.insert("background-color", "alert-success-bg");
                    base_tokens.insert("border-color", "alert-success-border");
                    base_tokens.insert("color", "alert-success-text");
                }
                AlertType::Info => {
                    base_tokens.insert("background-color", "alert-info-bg");
                    base_tokens.insert("border-color", "alert-info-border");
                    base_tokens.insert("color", "alert-info-text");
                }
                AlertType::Warning => {
                    base_tokens.insert("background-color", "alert-warning-bg");
                    base_tokens.insert("border-color", "alert-warning-border");
                    base_tokens.insert("color", "alert-warning-text");
                }
                AlertType::Error => {
                    base_tokens.insert("background-color", "alert-error-bg");
                    base_tokens.insert("border-color", "alert-error-border");
                    base_tokens.insert("color", "alert-error-text");
                }
            }
            
            // 根据尺寸调整令牌
            match self.config.size {
                AlertSize::Small => {
                    base_tokens.insert("padding", "alert-padding-sm");
                    base_tokens.insert("font-size", "alert-font-size-sm");
                }
                AlertSize::Large => {
                    base_tokens.insert("padding", "alert-padding-lg");
                    base_tokens.insert("font-size", "alert-font-size-lg");
                }
                _ => {}
            }
            
            // 生成主题样式
            let theme_style = use_token_style(base_tokens);
            if !theme_style.is_empty() {
                styles.push(theme_style);
            }
        }

        // 可见性控制
        if !self.config.visible {
            styles.push("display: none".to_string());
        }

        // 禁用状态
        if self.config.disabled {
            styles.push("opacity: 0.6".to_string());
            styles.push("pointer-events: none".to_string());
        }
        
        // 添加全局动画关键帧
        styles.push(format!("<style>{}</style>", AlertAnimationStyles::get_keyframes()));
        
        // 添加响应式样式
        if self.theme_enabled {
            styles.push(self.generate_responsive_styles());
        }

        styles.join("; ")
    }
}

impl Default for AlertStyleGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Alert 基础样式
#[derive(Debug, Clone)]
pub struct AlertStyles {
    pub base: String,
    pub message: String,
    pub description: String,
    pub icon: String,
    pub close_icon: String,
    pub action: String,
}

/// Alert 类型样式
#[derive(Debug, Clone)]
pub struct AlertTypeStyles {
    pub success: String,
    pub info: String,
    pub warning: String,
    pub error: String,
}

/// Alert 尺寸样式
#[derive(Debug, Clone)]
pub struct AlertSizeStyles {
    pub small: String,
    pub default: String,
    pub large: String,
}

/// Alert 变体样式
#[derive(Debug, Clone)]
pub struct AlertVariantStyles {
    pub filled: String,
    pub outlined: String,
}

/// Alert 状态样式
#[derive(Debug, Clone)]
pub struct AlertStateStyles {
    pub with_icon: String,
    pub with_description: String,
    pub closable: String,
    pub banner: String,
    pub no_border: String,
    pub disabled: String,
    pub hidden: String,
}

/// Alert 组件动画样式
#[derive(Debug, Clone)]
pub struct AlertAnimationStyles {
    pub entering: String,
    pub exiting: String,
}

impl Default for AlertAnimationStyles {
    fn default() -> Self {
        Self {
            entering: format!("\\n                opacity: 0;\\n                animation: ant-alert-motion-fade-in 0.3s cubic-bezier(0.645, 0.045, 0.355, 1);\\n                animation-fill-mode: forwards;\\n            "),
            exiting: format!("\\n                opacity: 1;\\n                animation: ant-alert-motion-fade-out 0.3s cubic-bezier(0.645, 0.045, 0.355, 1);\\n                animation-fill-mode: forwards;\\n            "),
        }
    }
}

impl AlertAnimationStyles {
    pub fn get_keyframes() -> String {
        format!("\\n            @keyframes ant-alert-motion-fade-in {{\\n                0% {{ opacity: 0; }}\\n                100% {{ opacity: 1; }}\\n            }}\\n            @keyframes ant-alert-motion-fade-out {{\\n                0% {{ opacity: 1; max-height: 150px; margin-bottom: 16px; padding: 8px 15px; }}\\n                100% {{ opacity: 0; max-height: 0; margin-bottom: 0; padding: 0; }}\\n            }}\\n        ")
    }
}

/// 生成 Alert 组件的完整样式
pub fn generate_alert_styles() -> (AlertStyles, AlertTypeStyles, AlertSizeStyles, AlertVariantStyles, AlertStateStyles, AlertAnimationStyles) {
    let base_styles = AlertStyles {
        base: css!("
            .ant-alert {
                position: relative;
                display: flex;
                align-items: flex-start;
                padding: 8px 15px;
                margin-bottom: 16px;
                border: 1px solid transparent;
                border-radius: 6px;
                font-size: 14px;
                line-height: 1.5715;
                word-wrap: break-word;
                transition: all 0.3s cubic-bezier(0.78, 0.14, 0.15, 0.86);
            }
        ").to_string(),
        message: css!("
            .ant-alert-message {
                color: rgba(0, 0, 0, 0.88);
                font-weight: 600;
                margin-bottom: 4px;
            }
        ").to_string(),
        description: css!("
            .ant-alert-description {
                color: rgba(0, 0, 0, 0.65);
                font-size: 14px;
                line-height: 1.5715;
            }
        ").to_string(),
        icon: css!("
            .ant-alert-icon {
                margin-right: 8px;
                font-size: 16px;
                line-height: 1;
                flex-shrink: 0;
            }
        ").to_string(),
        close_icon: css!("
            .ant-alert-close-icon {
                position: absolute;
                top: 8px;
                right: 8px;
                padding: 0;
                background: transparent;
                border: none;
                cursor: pointer;
                font-size: 12px;
                line-height: 1;
                color: rgba(0, 0, 0, 0.45);
                transition: color 0.3s;
            }
            
            .ant-alert-close-icon:hover {
                color: rgba(0, 0, 0, 0.75);
            }
        ").to_string(),
        action: css!("
            .ant-alert-action {
                margin-left: auto;
                padding-left: 16px;
            }
        ").to_string(),
    };

    let type_styles = AlertTypeStyles {
        success: css!("
            .ant-alert-success {
                background-color: #f6ffed;
                border-color: #b7eb8f;
                color: #52c41a;
            }
        ").to_string(),
        info: css!("
            .ant-alert-info {
                background-color: #e6f7ff;
                border-color: #91d5ff;
                color: #1890ff;
            }
        ").to_string(),
        warning: css!("
            .ant-alert-warning {
                background-color: #fffbe6;
                border-color: #ffe58f;
                color: #faad14;
            }
        ").to_string(),
        error: css!("
            .ant-alert-error {
                background-color: #fff2f0;
                border-color: #ffccc7;
                color: #ff4d4f;
            }
        ").to_string(),
    };

    let size_styles = AlertSizeStyles {
        small: css!("
            .ant-alert-small {
                padding: 4px 8px;
                font-size: 12px;
            }
            
            .ant-alert-small .ant-alert-icon {
                font-size: 14px;
                margin-right: 6px;
            }
            
            .ant-alert-small .ant-alert-close-icon {
                top: 4px;
                right: 4px;
                font-size: 10px;
            }
        ").to_string(),
        default: css!("
            .ant-alert-default {
                /* 默认样式已在 base 中定义 */
            }
        ").to_string(),
        large: css!("
            .ant-alert-large {
                padding: 12px 20px;
                font-size: 16px;
            }
            
            .ant-alert-large .ant-alert-icon {
                font-size: 20px;
                margin-right: 12px;
            }
            
            .ant-alert-large .ant-alert-close-icon {
                top: 12px;
                right: 12px;
                font-size: 14px;
            }
        ").to_string(),
    };

    let variant_styles = AlertVariantStyles {
        filled: css!("
            .ant-alert-filled {
                /* 默认填充样式 */
            }
        ").to_string(),
        outlined: css!("
            .ant-alert-outlined {
                background-color: transparent;
                border-width: 1px;
                border-style: solid;
            }
        ").to_string(),
    };

    let state_styles = AlertStateStyles {
        with_icon: css!("
            .ant-alert-with-icon {
                padding-left: 15px;
            }
            
            .ant-alert-with-icon .ant-alert-message {
                margin-left: 24px;
            }
            
            .ant-alert-with-icon .ant-alert-description {
                margin-left: 24px;
            }
        ").to_string(),
        with_description: css!("
            .ant-alert-with-description {
                padding: 15px 15px 15px 64px;
            }
            
            .ant-alert-with-description .ant-alert-icon {
                font-size: 24px;
                margin-top: -2px;
            }
            
            .ant-alert-with-description .ant-alert-message {
                font-size: 16px;
                margin-bottom: 4px;
            }
        ").to_string(),
        closable: css!("
            .ant-alert-closable {
                padding-right: 32px;
            }
        ").to_string(),
        banner: css!("
            .ant-alert-banner {
                margin-bottom: 0;
                border: 0;
                border-radius: 0;
            }
        ").to_string(),
        no_border: css!("
            .ant-alert-no-border {
                border: none;
            }
        ").to_string(),
        disabled: css!("
            .ant-alert-disabled {
                opacity: 0.6;
                pointer-events: none;
            }
        ").to_string(),
        hidden: css!("
            .ant-alert-hidden {
                display: none;
            }
        ").to_string(),
    };

    let animation_styles = AlertAnimationStyles::default();

    (base_styles, type_styles, size_styles, variant_styles, state_styles, animation_styles)
}
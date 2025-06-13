//!
//! Tag 组件样式定义
//!
//! 包含 Tag 组件的所有样式生成逻辑。

use crate::theme::AliasToken;
use super::types::*;

/// Tag 样式生成器
#[derive(Debug, Clone)]
pub struct TagStyleGenerator {
    pub color: TagColor,
    pub size: TagSize,
    pub variant: TagVariant,
    pub closable: bool,
    pub bordered: bool,
    pub disabled: bool,
    pub status: Option<TagStatus>,
    pub prefix_cls: String,
    pub token: AliasToken,
}

impl TagStyleGenerator {
    pub fn new() -> Self {
        Self {
            color: TagColor::default(),
            size: TagSize::default(),
            variant: TagVariant::default(),
            closable: false,
            bordered: true,
            disabled: false,
            status: None,
            prefix_cls: "ant".to_string(),
            token: AliasToken::default(),
        }
    }
    
    pub fn with_color(mut self, color: TagColor) -> Self {
        self.color = color;
        self
    }
    
    pub fn with_size(mut self, size: TagSize) -> Self {
        self.size = size;
        self
    }
    
    pub fn with_variant(mut self, variant: TagVariant) -> Self {
        self.variant = variant;
        self
    }
    
    pub fn with_closable(mut self, closable: bool) -> Self {
        self.closable = closable;
        self
    }
    
    pub fn with_bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }
    
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    
    pub fn with_status(mut self, status: Option<TagStatus>) -> Self {
        self.status = status;
        self
    }
    
    pub fn with_prefix_cls(mut self, prefix_cls: String) -> Self {
        self.prefix_cls = prefix_cls;
        self
    }
    
    pub fn with_token(mut self, token: AliasToken) -> Self {
        self.token = token;
        self
    }
    
    /// 生成CSS类名
    pub fn generate_class_names(&self) -> Vec<String> {
        let mut classes = vec![
            format!("{}-tag", self.prefix_cls),
        ];
        
        // 尺寸类名
        match self.size {
            TagSize::Small => classes.push(format!("{}-tag-sm", self.prefix_cls)),
            TagSize::Large => classes.push(format!("{}-tag-lg", self.prefix_cls)),
            _ => {}
        }
        
        // 变体类名
        match self.variant {
            TagVariant::Outlined => classes.push(format!("{}-tag-outlined", self.prefix_cls)),
            TagVariant::Borderless => classes.push(format!("{}-tag-borderless", self.prefix_cls)),
            _ => {}
        }
        
        // 颜色类名
        match &self.color {
            TagColor::Primary => classes.push(format!("{}-tag-primary", self.prefix_cls)),
            TagColor::Success => classes.push(format!("{}-tag-success", self.prefix_cls)),
            TagColor::Warning => classes.push(format!("{}-tag-warning", self.prefix_cls)),
            TagColor::Error => classes.push(format!("{}-tag-error", self.prefix_cls)),
            TagColor::Info => classes.push(format!("{}-tag-info", self.prefix_cls)),
            TagColor::Processing => classes.push(format!("{}-tag-processing", self.prefix_cls)),
            TagColor::Preset(preset) => {
                let color_name = match preset {
                    TagPresetColor::Magenta => "magenta",
                    TagPresetColor::Red => "red",
                    TagPresetColor::Volcano => "volcano",
                    TagPresetColor::Orange => "orange",
                    TagPresetColor::Gold => "gold",
                    TagPresetColor::Lime => "lime",
                    TagPresetColor::Green => "green",
                    TagPresetColor::Cyan => "cyan",
                    TagPresetColor::Blue => "blue",
                    TagPresetColor::GeekBlue => "geekblue",
                    TagPresetColor::Purple => "purple",
                };
                classes.push(format!("{}-tag-{}", self.prefix_cls, color_name));
            }
            _ => {}
        }
        
        // 状态类名
        if let Some(status) = &self.status {
            let status_name = match status {
                TagStatus::Success => "success",
                TagStatus::Processing => "processing",
                TagStatus::Error => "error",
                TagStatus::Warning => "warning",
                _ => "default",
            };
            classes.push(format!("{}-tag-status-{}", self.prefix_cls, status_name));
        }
        
        // 可关闭类名
        if self.closable {
            classes.push(format!("{}-tag-closable", self.prefix_cls));
        }
        
        // 无边框类名
        if !self.bordered {
            classes.push(format!("{}-tag-borderless", self.prefix_cls));
        }
        
        // 禁用类名
        if self.disabled {
            classes.push(format!("{}-tag-disabled", self.prefix_cls));
        }
        
        classes
    }
    
    /// 生成内联样式
    pub fn generate_inline_styles(&self) -> String {
        let mut styles = Vec::new();
        
        // 自定义颜色样式
        if let TagColor::Custom(color) = &self.color {
            if color.starts_with('#') {
                // 十六进制颜色
                styles.push(format!("background-color: {}", color));
                styles.push(format!("border-color: {}", color));
                styles.push("color: #fff".to_string());
            } else {
                // CSS颜色名称
                styles.push(format!("background-color: {}", color));
                styles.push(format!("border-color: {}", color));
            }
        }
        
        styles.join("; ")
    }
}

/// Tag 样式结构
#[derive(Debug, Clone)]
pub struct TagStyles {
    pub base: String,
    pub variants: TagVariantStyles,
    pub colors: TagColorStyles,
    pub sizes: TagSizeStyles,
}

/// Tag 变体样式
#[derive(Debug, Clone)]
pub struct TagVariantStyles {
    pub filled: String,
    pub outlined: String,
    pub borderless: String,
}

/// Tag 颜色样式
#[derive(Debug, Clone)]
pub struct TagColorStyles {
    pub default: String,
    pub primary: String,
    pub success: String,
    pub warning: String,
    pub error: String,
    pub info: String,
    pub processing: String,
    pub preset_colors: TagPresetColorStyles,
}

/// Tag 预设颜色样式
#[derive(Debug, Clone)]
pub struct TagPresetColorStyles {
    pub magenta: String,
    pub red: String,
    pub volcano: String,
    pub orange: String,
    pub gold: String,
    pub lime: String,
    pub green: String,
    pub cyan: String,
    pub blue: String,
    pub geekblue: String,
    pub purple: String,
}

/// Tag 尺寸样式
#[derive(Debug, Clone)]
pub struct TagSizeStyles {
    pub small: String,
    pub middle: String,
    pub large: String,
}

/// 生成 Tag 样式
pub fn generate_tag_styles() -> TagStyles {
    TagStyles {
        base: r#"
            .ant-tag {
                display: inline-block;
                height: auto;
                margin-inline-end: 8px;
                padding-inline: 7px;
                font-size: 12px;
                line-height: 20px;
                white-space: nowrap;
                background: rgba(0, 0, 0, 0.02);
                border: 1px solid #d9d9d9;
                border-radius: 6px;
                opacity: 1;
                transition: all 0.2s;
                text-align: start;
                position: relative;
                cursor: default;
                user-select: none;
            }
            
            .ant-tag:hover {
                opacity: 0.85;
            }
            
            .ant-tag-close-icon {
                margin-inline-start: 3px;
                color: rgba(0, 0, 0, 0.45);
                font-weight: 400;
                font-size: 10px;
                line-height: inherit;
                cursor: pointer;
                transition: all 0.2s;
            }
            
            .ant-tag-close-icon:hover {
                color: rgba(0, 0, 0, 0.85);
            }
            
            .ant-tag-disabled {
                opacity: 0.5;
                cursor: not-allowed;
                pointer-events: none;
            }
        "#.to_string(),
        
        variants: TagVariantStyles {
            filled: r#"
                .ant-tag {
                    background: rgba(0, 0, 0, 0.02);
                    border: 1px solid #d9d9d9;
                }
            "#.to_string(),
            
            outlined: r#"
                .ant-tag-outlined {
                    background: transparent;
                    border: 1px solid #d9d9d9;
                }
            "#.to_string(),
            
            borderless: r#"
                .ant-tag-borderless {
                    border: none;
                    background: rgba(0, 0, 0, 0.02);
                }
            "#.to_string(),
        },
        
        colors: TagColorStyles {
            default: r#"
                .ant-tag {
                    color: rgba(0, 0, 0, 0.88);
                    background: rgba(0, 0, 0, 0.02);
                    border-color: #d9d9d9;
                }
            "#.to_string(),
            
            primary: r#"
                .ant-tag-primary {
                    color: #1677ff;
                    background: #f0f5ff;
                    border-color: #adc6ff;
                }
            "#.to_string(),
            
            success: r#"
                .ant-tag-success {
                    color: #52c41a;
                    background: #f6ffed;
                    border-color: #b7eb8f;
                }
            "#.to_string(),
            
            warning: r#"
                .ant-tag-warning {
                    color: #faad14;
                    background: #fffbe6;
                    border-color: #ffe58f;
                }
            "#.to_string(),
            
            error: r#"
                .ant-tag-error {
                    color: #ff4d4f;
                    background: #fff2f0;
                    border-color: #ffccc7;
                }
            "#.to_string(),
            
            info: r#"
                .ant-tag-info {
                    color: #1677ff;
                    background: #f0f5ff;
                    border-color: #adc6ff;
                }
            "#.to_string(),
            
            processing: r#"
                .ant-tag-processing {
                    color: #1677ff;
                    background: #f0f5ff;
                    border-color: #adc6ff;
                }
            "#.to_string(),
            
            preset_colors: TagPresetColorStyles {
                magenta: r#"
                    .ant-tag-magenta {
                        color: #eb2f96;
                        background: #fff0f6;
                        border-color: #ffadd2;
                    }
                "#.to_string(),
                
                red: r#"
                    .ant-tag-red {
                        color: #ff4d4f;
                        background: #fff2f0;
                        border-color: #ffccc7;
                    }
                "#.to_string(),
                
                volcano: r#"
                    .ant-tag-volcano {
                        color: #fa541c;
                        background: #fff2e8;
                        border-color: #ffd8bf;
                    }
                "#.to_string(),
                
                orange: r#"
                    .ant-tag-orange {
                        color: #fa8c16;
                        background: #fff7e6;
                        border-color: #ffd591;
                    }
                "#.to_string(),
                
                gold: r#"
                    .ant-tag-gold {
                        color: #faad14;
                        background: #fffbe6;
                        border-color: #ffe58f;
                    }
                "#.to_string(),
                
                lime: r#"
                    .ant-tag-lime {
                        color: #a0d911;
                        background: #fcffe6;
                        border-color: #eaff8f;
                    }
                "#.to_string(),
                
                green: r#"
                    .ant-tag-green {
                        color: #52c41a;
                        background: #f6ffed;
                        border-color: #b7eb8f;
                    }
                "#.to_string(),
                
                cyan: r#"
                    .ant-tag-cyan {
                        color: #13c2c2;
                        background: #e6fffb;
                        border-color: #87e8de;
                    }
                "#.to_string(),
                
                blue: r#"
                    .ant-tag-blue {
                        color: #1677ff;
                        background: #f0f5ff;
                        border-color: #adc6ff;
                    }
                "#.to_string(),
                
                geekblue: r#"
                    .ant-tag-geekblue {
                        color: #2f54eb;
                        background: #f0f5ff;
                        border-color: #adc6ff;
                    }
                "#.to_string(),
                
                purple: r#"
                    .ant-tag-purple {
                        color: #722ed1;
                        background: #f9f0ff;
                        border-color: #d3adf7;
                    }
                "#.to_string(),
            },
        },
        
        sizes: TagSizeStyles {
            small: r#"
                .ant-tag-sm {
                    padding-inline: 4px;
                    font-size: 11px;
                    line-height: 16px;
                    border-radius: 4px;
                }
            "#.to_string(),
            
            middle: r#"
                .ant-tag {
                    padding-inline: 7px;
                    font-size: 12px;
                    line-height: 20px;
                    border-radius: 6px;
                }
            "#.to_string(),
            
            large: r#"
                .ant-tag-lg {
                    padding-inline: 8px;
                    font-size: 14px;
                    line-height: 22px;
                    border-radius: 8px;
                }
            "#.to_string(),
        },
    }
}
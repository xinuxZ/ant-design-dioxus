//! 按钮组样式生成器
//!
//! 提供按钮组组件的样式生成功能，包括：
//! - 基础按钮组样式
//! - 不同尺寸的按钮组样式
//! - 按钮组内按钮的排列样式

use css_in_rust::css;

/// 按钮组样式生成器
#[derive(Debug, Clone, Default)]
pub struct ButtonGroupStyleGenerator {
    /// 按钮组尺寸
    size: Option<String>,
}

impl ButtonGroupStyleGenerator {
    /// 创建新的按钮组样式生成器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置按钮组尺寸
    pub fn with_size(mut self, size: &str) -> Self {
        self.size = Some(size.to_string());
        self
    }

    /// 生成按钮组样式类名
    pub fn generate(&self) -> String {
        let base_style = self.base_style();
        let size_style = self.size_style();
        
        format!("{} {}", base_style, size_style).trim().to_string()
    }

    /// 生成基础按钮组样式
    fn base_style(&self) -> String {
        css! {
            ".ant-btn-group" {
                position: "relative";
                display: "inline-block";
                vertical-align: "middle";
            }

            ".ant-btn-group > .ant-btn" {
                position: "relative";
                z-index: "1";
            }

            ".ant-btn-group > .ant-btn:hover,
             .ant-btn-group > .ant-btn:focus,
             .ant-btn-group > .ant-btn:active" {
                z-index: "2";
            }

            ".ant-btn-group > .ant-btn:not(:first-child):not(:last-child)" {
                border-radius: "0";
            }

            ".ant-btn-group > .ant-btn:first-child:not(:last-child)" {
                border-top-right-radius: "0";
                border-bottom-right-radius: "0";
            }

            ".ant-btn-group > .ant-btn:last-child:not(:first-child)" {
                border-top-left-radius: "0";
                border-bottom-left-radius: "0";
            }

            ".ant-btn-group > .ant-btn:not(:first-child)" {
                margin-left: "-1px";
            }

            ".ant-btn-group > .ant-btn:not(:first-child):not(:last-child):not(.ant-btn-primary)" {
                border-left-color: "transparent";
                border-right-color: "transparent";
            }

            ".ant-btn-group > .ant-btn:first-child:not(:last-child):not(.ant-btn-primary)" {
                border-right-color: "transparent";
            }

            ".ant-btn-group > .ant-btn:last-child:not(:first-child):not(.ant-btn-primary)" {
                border-left-color: "transparent";
            }
        }
    }

    /// 生成尺寸相关样式
    fn size_style(&self) -> String {
        match self.size.as_deref() {
            Some("large") => css! {
                ".ant-btn-group-lg > .ant-btn" {
                    height: "40px";
                    padding: "6.4px 15px";
                    font-size: "16px";
                    line-height: "1.5";
                }
            },
            Some("small") => css! {
                ".ant-btn-group-sm > .ant-btn" {
                    height: "24px";
                    padding: "0px 7px";
                    font-size: "14px";
                    line-height: "1.5";
                }
            },
            _ => String::new(), // 默认中等尺寸，无需额外样式
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_group_style_generator_new() {
        let generator = ButtonGroupStyleGenerator::new();
        assert!(generator.size.is_none());
    }

    #[test]
    fn test_button_group_style_generator_with_size() {
        let generator = ButtonGroupStyleGenerator::new().with_size("large");
        assert_eq!(generator.size, Some("large".to_string()));
    }

    #[test]
    fn test_button_group_style_generator_generate() {
        let generator = ButtonGroupStyleGenerator::new();
        let style = generator.generate();
        assert!(!style.is_empty());
    }

    #[test]
    fn test_button_group_style_generator_with_large_size() {
        let generator = ButtonGroupStyleGenerator::new().with_size("large");
        let style = generator.generate();
        assert!(!style.is_empty());
    }

    #[test]
    fn test_button_group_style_generator_with_small_size() {
        let generator = ButtonGroupStyleGenerator::new().with_size("small");
        let style = generator.generate();
        assert!(!style.is_empty());
    }
}
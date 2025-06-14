/// Affix 组件样式模块
///
/// 本模块包含 Affix 组件的所有样式定义，从组件逻辑中分离出来，
/// 提高代码的可维护性和复用性。
use crate::theme::AliasToken;
use css_in_rust::css;

/// 固钉位置
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AffixPosition {
    /// 固定在顶部
    Top,
    /// 固定在底部
    Bottom,
}

impl Default for AffixPosition {
    fn default() -> Self {
        AffixPosition::Top
    }
}

/// Affix 样式生成器
pub struct AffixStyleGenerator {
    /// 是否处于固定状态
    pub is_fixed: bool,
    /// 固定位置
    pub position: AffixPosition,
    /// 顶部偏移量
    pub offset_top: i32,
    /// 底部偏移量
    pub offset_bottom: Option<i32>,
    /// 自定义样式
    pub custom_style: Option<String>,
    /// 主题令牌
    pub token: AliasToken,
}

impl AffixStyleGenerator {
    /// 创建新的样式生成器
    pub fn new() -> Self {
        Self {
            is_fixed: false,
            position: AffixPosition::default(),
            offset_top: 0,
            offset_bottom: None,
            custom_style: None,
            token: AliasToken::default(),
        }
    }

    /// 设置固定状态
    pub fn with_fixed(mut self, is_fixed: bool) -> Self {
        self.is_fixed = is_fixed;
        self
    }

    /// 设置固定位置
    pub fn with_position(mut self, position: AffixPosition) -> Self {
        self.position = position;
        self
    }

    /// 设置顶部偏移量
    pub fn with_offset_top(mut self, offset: i32) -> Self {
        self.offset_top = offset;
        self
    }

    /// 设置底部偏移量
    pub fn with_offset_bottom(mut self, offset: Option<i32>) -> Self {
        self.offset_bottom = offset;
        self
    }

    /// 添加自定义样式
    pub fn with_custom_style(mut self, style: &str) -> Self {
        self.custom_style = Some(style.to_string());
        self
    }

    /// 设置主题令牌
    pub fn with_token(mut self, token: AliasToken) -> Self {
        self.token = token;
        self
    }

    /// 生成 Affix 样式
    pub fn generate(&self) -> String {
        // 基础样式
        let base_style = "ant-affix".to_string();

        // 如果处于固定状态，添加固定样式
        let fixed_style = if self.is_fixed {
            "ant-affix-fixed".to_string()
        } else {
            "".to_string()
        };

        // 组合所有样式
        format!("{} {}", base_style, fixed_style).trim().to_string()
    }

    /// 生成 Affix 内联样式
    pub fn generate_inline_style(&self) -> String {
        // 基础样式
        let base_style = css!(r#"position: relative;"#);

        // 如果处于固定状态，添加固定样式
        let fixed_style = if self.is_fixed {
            let style = match self.position {
                AffixPosition::Top => {
                    css!(
                        r#"
                        position: fixed;
                        z-index: ${z_index};
                        box-shadow: ${box_shadow};
                        top: ${top_offset}px;
                        "#,
                        z_index = self.token.z_index_popup_base,
                        box_shadow = self.token.box_shadow_secondary,
                        top_offset = self.offset_top
                    )
                }
                AffixPosition::Bottom => {
                    if let Some(offset) = self.offset_bottom {
                        css!(
                            r#"
                            position: fixed;
                            z-index: ${z_index};
                            box-shadow: ${box_shadow};
                            bottom: ${bottom_offset}px;
                            "#,
                            z_index = self.token.z_index_popup_base,
                            box_shadow = self.token.box_shadow_secondary,
                            bottom_offset = offset
                        )
                    } else {
                        css!(
                            r#"
                            position: fixed;
                            z-index: ${z_index};
                            box-shadow: ${box_shadow};
                            "#,
                            z_index = self.token.z_index_popup_base,
                            box_shadow = self.token.box_shadow_secondary
                        )
                    }
                }
            };

            style.to_string()
        } else {
            "".to_string()
        };

        // 添加自定义样式
        let custom = self.custom_style.clone().unwrap_or_default();

        // 组合所有样式
        format!("{}{}{}", base_style, fixed_style, custom)
    }

    /// 生成 Affix CSS
    pub fn generate_css(&self) -> String {
        css!(
            r#"
            .ant-affix {
                position: relative;
                transition: all 0.3s cubic-bezier(0.645, 0.045, 0.355, 1);
            }

            .ant-affix-fixed {
                position: fixed;
                z-index: ${z_index};
            }

            .ant-affix-placeholder {
                display: block;
            }

            .ant-affix[style*="position: fixed"] {
                box-shadow: ${box_shadow};
                z-index: ${z_index};
            }

            @media (max-width: 768px) {
                .ant-affix[style*="position: fixed"] {
                    position: relative !important;
                    top: auto !important;
                    bottom: auto !important;
                }
                .ant-affix-placeholder {
                    display: none;
                }
            }

            @media (prefers-contrast: high) {
                .ant-affix[style*="position: fixed"] {
                    border: 1px solid;
                }
            }

            @media (prefers-reduced-motion: reduce) {
                .ant-affix {
                    transition: none;
                }
            }
            "#,
            z_index = self.token.z_index_popup_base,
            box_shadow = self.token.box_shadow_secondary
        )
        .to_string()
    }
}

/// 生成 Affix 样式
pub fn generate_affix_style(
    is_fixed: bool,
    position: AffixPosition,
    offset_top: i32,
    offset_bottom: Option<i32>,
) -> String {
    AffixStyleGenerator::new()
        .with_fixed(is_fixed)
        .with_position(position)
        .with_offset_top(offset_top)
        .with_offset_bottom(offset_bottom)
        .generate()
}

/// 生成 Affix 内联样式
pub fn generate_affix_inline_style(
    is_fixed: bool,
    position: AffixPosition,
    offset_top: i32,
    offset_bottom: Option<i32>,
) -> String {
    AffixStyleGenerator::new()
        .with_fixed(is_fixed)
        .with_position(position)
        .with_offset_top(offset_top)
        .with_offset_bottom(offset_bottom)
        .generate_inline_style()
}

/// 默认 Affix 样式
pub fn default_affix_style() -> String {
    AffixStyleGenerator::new().generate()
}

/// 生成 Affix 组件的全部 CSS
pub fn generate_affix_css() -> String {
    css!(
        r#"
        .ant-affix {
            position: relative;
            transition: all 0.3s cubic-bezier(0.645, 0.045, 0.355, 1);
        }

        .ant-affix-placeholder {
            display: block;
        }

        .ant-affix[style*="position: fixed"] {
            box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
            z-index: 10;
        }

        @media (max-width: 768px) {
            .ant-affix[style*="position: fixed"] {
                position: relative !important;
                top: auto !important;
                bottom: auto !important;
            }
            .ant-affix-placeholder {
                display: none;
            }
        }

        @media (prefers-contrast: high) {
            .ant-affix[style*="position: fixed"] {
                border: 1px solid;
            }
        }

        @media (prefers-reduced-motion: reduce) {
            .ant-affix {
                transition: none;
            }
        }
        "#
    )
    .to_string()
}

/// 注入 Affix 样式到文档
pub fn inject_affix_style() {
    use crate::utils::style_injector;
    style_injector::inject_style("ant-affix-style", &generate_affix_css());
}

/// 在组件中使用此方法来确保样式已注入
pub fn use_affix_style() {
    use dioxus::prelude::*;
    use_effect(move || inject_affix_style());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_affix_style_generator() {
        let style = AffixStyleGenerator::new()
            .with_fixed(true)
            .with_position(AffixPosition::Top)
            .with_offset_top(10)
            .generate();

        assert!(style.contains("position: fixed"));
        assert!(style.contains("top: 10px"));
        assert!(style.contains("z-index: 10"));
    }

    #[test]
    fn test_affix_position_default() {
        assert_eq!(AffixPosition::default(), AffixPosition::Top);
    }

    #[test]
    fn test_generate_affix_css() {
        let css = generate_affix_css();
        assert!(css.contains(".ant-affix"));
        assert!(css.contains(".ant-affix-placeholder"));
        assert!(css.contains("@media"));
    }
}

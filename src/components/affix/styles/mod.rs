/// Affix 组件样式生成器
///
/// 用于生成 Affix 组件的样式

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

    /// 生成 Affix 样式
    pub fn generate(&self) -> String {
        let mut style = String::new();

        // 基础样式
        style.push_str(&self.base_style());

        // 如果处于固定状态，添加固定样式
        if self.is_fixed {
            style.push_str(&self.fixed_style());
        }

        // 添加自定义样式
        if let Some(custom) = &self.custom_style {
            style.push_str(custom);
        }

        style
    }

    /// 基础样式
    fn base_style(&self) -> String {
        String::from("position: relative;")
    }

    /// 固定状态样式
    fn fixed_style(&self) -> String {
        let mut style = String::from(
            "position: fixed; z-index: 10; box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);",
        );

        match self.position {
            AffixPosition::Top => {
                style.push_str(&format!("top: {}px;", self.offset_top));
            }
            AffixPosition::Bottom => {
                if let Some(offset) = self.offset_bottom {
                    style.push_str(&format!("bottom: {}px;", offset));
                }
            }
        }

        style
    }
}

/// 生成 Affix 组件的全部 CSS
pub fn generate_affix_css() -> String {
    let mut css = String::new();

    // 基础样式
    css.push_str(".ant-affix { position: relative; transition: all 0.3s cubic-bezier(0.645, 0.045, 0.355, 1); }\n");

    // 占位符样式
    css.push_str(".ant-affix-placeholder { display: block; }\n");

    // 固定状态样式
    css.push_str(".ant-affix[style*=\"position: fixed\"] { box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15); z-index: 10; }\n");

    // 响应式设计
    css.push_str(
        "@media (max-width: 768px) {
  .ant-affix[style*=\"position: fixed\"] {
    position: relative !important;
    top: auto !important;
    bottom: auto !important;
  }
  .ant-affix-placeholder {
    display: none;
  }
}\n",
    );

    // 高对比度模式支持
    css.push_str(
        "@media (prefers-contrast: high) {
  .ant-affix[style*=\"position: fixed\"] {
    border: 1px solid;
  }
}\n",
    );

    // 减少动画模式支持
    css.push_str(
        "@media (prefers-reduced-motion: reduce) {
  .ant-affix {
    transition: none;
  }
}\n",
    );

    css
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

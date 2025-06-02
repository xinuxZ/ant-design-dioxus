//! Ant Design 间距规范
//!
//! 实现 Ant Design 的间距系统，包括：
//! - 基础间距单位
//! - 组件间距
//! - 布局间距
//! - 内边距和外边距

use crate::theme::core::{Size, SpaceSize};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 间距类型枚举
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SpacingType {
    /// 内边距
    Padding,
    /// 外边距
    Margin,
    /// 间隙
    Gap,
    /// 边框间距
    Border,
}

/// 间距方向枚举
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SpacingDirection {
    /// 全部方向
    All,
    /// 垂直方向（上下）
    Vertical,
    /// 水平方向（左右）
    Horizontal,
    /// 上方
    Top,
    /// 右方
    Right,
    /// 下方
    Bottom,
    /// 左方
    Left,
}

impl SpacingDirection {
    /// 转换为 CSS 属性后缀
    pub fn to_css_suffix(&self) -> &'static str {
        match self {
            SpacingDirection::All => "",
            SpacingDirection::Vertical => "-block",
            SpacingDirection::Horizontal => "-inline",
            SpacingDirection::Top => "-top",
            SpacingDirection::Right => "-right",
            SpacingDirection::Bottom => "-bottom",
            SpacingDirection::Left => "-left",
        }
    }
}

/// 间距配置结构
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpacingConfig {
    /// 间距类型
    pub spacing_type: SpacingType,
    /// 间距方向
    pub direction: SpacingDirection,
    /// 间距大小
    pub size: SpaceSize,
}

impl SpacingConfig {
    /// 创建新的间距配置
    pub fn new(spacing_type: SpacingType, direction: SpacingDirection, size: SpaceSize) -> Self {
        Self {
            spacing_type,
            direction,
            size,
        }
    }

    /// 生成 CSS 属性名
    pub fn to_css_property(&self) -> String {
        let base = match self.spacing_type {
            SpacingType::Padding => "padding",
            SpacingType::Margin => "margin",
            SpacingType::Gap => "gap",
            SpacingType::Border => "border-spacing",
        };

        format!("{}{}", base, self.direction.to_css_suffix())
    }

    /// 生成 CSS 值
    pub fn to_css_value(&self, spacing: &AntDesignSpacing) -> String {
        let pixels = spacing.get_space(self.size).unwrap_or(0);
        format!("{}px", pixels)
    }

    /// 生成完整的 CSS 声明
    pub fn to_css(&self, spacing: &AntDesignSpacing) -> String {
        format!(
            "{}: {};",
            self.to_css_property(),
            self.to_css_value(spacing)
        )
    }
}

/// Ant Design 间距规范
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AntDesignSpacing {
    /// 基础间距单位（像素）
    pub base_unit: u32,

    // 基础间距定义
    /// 无间距
    pub none: u32,
    /// 极小间距 - 4px
    pub xs: u32,
    /// 小间距 - 8px
    pub sm: u32,
    /// 中等间距 - 16px
    pub md: u32,
    /// 大间距 - 24px
    pub lg: u32,
    /// 极大间距 - 32px
    pub xl: u32,
    /// 超大间距 - 48px
    pub xxl: u32,

    // 组件特定间距
    /// 按钮内边距
    pub button_padding: HashMap<Size, (u32, u32)>, // (水平, 垂直)
    /// 输入框内边距
    pub input_padding: HashMap<Size, (u32, u32)>,
    /// 卡片内边距
    pub card_padding: u32,
    /// 模态框内边距
    pub modal_padding: u32,
    /// 表格单元格内边距
    pub table_cell_padding: (u32, u32),

    // 布局间距
    /// 栅格间距
    pub grid_gutter: u32,
    /// 容器内边距
    pub container_padding: u32,
    /// 节间距
    pub section_spacing: u32,
}

impl AntDesignSpacing {
    /// 创建默认间距规范
    pub fn default() -> Self {
        let base_unit = 8;

        let mut button_padding = HashMap::new();
        button_padding.insert(Size::Small, (7, 4));
        button_padding.insert(Size::Middle, (15, 8));
        button_padding.insert(Size::Large, (15, 10));

        let mut input_padding = HashMap::new();
        input_padding.insert(Size::Small, (7, 4));
        input_padding.insert(Size::Middle, (11, 8));
        input_padding.insert(Size::Large, (11, 10));

        Self {
            base_unit,

            none: 0,
            xs: base_unit / 2,  // 4px
            sm: base_unit,      // 8px
            md: base_unit * 2,  // 16px
            lg: base_unit * 3,  // 24px
            xl: base_unit * 4,  // 32px
            xxl: base_unit * 6, // 48px

            button_padding,
            input_padding,
            card_padding: base_unit * 3,                    // 24px
            modal_padding: base_unit * 3,                   // 24px
            table_cell_padding: (base_unit, base_unit * 2), // 8px, 16px

            grid_gutter: base_unit * 2,       // 16px
            container_padding: base_unit * 3, // 24px
            section_spacing: base_unit * 6,   // 48px
        }
    }

    /// 创建紧凑间距规范
    pub fn compact() -> Self {
        let base_unit = 6;

        let mut button_padding = HashMap::new();
        button_padding.insert(Size::Small, (5, 2));
        button_padding.insert(Size::Middle, (11, 6));
        button_padding.insert(Size::Large, (11, 8));

        let mut input_padding = HashMap::new();
        input_padding.insert(Size::Small, (5, 2));
        input_padding.insert(Size::Middle, (8, 6));
        input_padding.insert(Size::Large, (8, 8));

        Self {
            base_unit,

            none: 0,
            xs: base_unit / 2,  // 3px
            sm: base_unit,      // 6px
            md: base_unit * 2,  // 12px
            lg: base_unit * 3,  // 18px
            xl: base_unit * 4,  // 24px
            xxl: base_unit * 6, // 36px

            button_padding,
            input_padding,
            card_padding: base_unit * 2,                    // 12px
            modal_padding: base_unit * 2,                   // 12px
            table_cell_padding: (base_unit, base_unit * 2), // 6px, 12px

            grid_gutter: base_unit * 2,       // 12px
            container_padding: base_unit * 2, // 12px
            section_spacing: base_unit * 4,   // 24px
        }
    }

    /// 获取指定间距大小的像素值
    pub fn get_space(&self, space: SpaceSize) -> Option<u32> {
        match space {
            SpaceSize::None => Some(self.none),
            SpaceSize::XSmall => Some(self.xs),
            SpaceSize::Small => Some(self.sm),
            SpaceSize::Middle => Some(self.md),
            SpaceSize::Large => Some(self.lg),
            SpaceSize::XLarge => Some(self.xl),
            SpaceSize::Custom(size) => Some(size),
        }
    }

    /// 获取按钮内边距
    pub fn get_button_padding(&self, size: Size) -> (u32, u32) {
        self.button_padding.get(&size).copied().unwrap_or((11, 8))
    }

    /// 获取输入框内边距
    pub fn get_input_padding(&self, size: Size) -> (u32, u32) {
        self.input_padding.get(&size).copied().unwrap_or((11, 8))
    }

    /// 根据倍数获取间距
    pub fn get_spacing_by_multiple(&self, multiple: f32) -> u32 {
        (self.base_unit as f32 * multiple) as u32
    }

    /// 生成 CSS 变量
    pub fn to_css_variables(&self) -> HashMap<String, String> {
        let mut variables = HashMap::new();

        // 基础间距变量
        variables.insert(
            "--ant-spacing-base".to_string(),
            format!("{}px", self.base_unit),
        );
        variables.insert("--ant-spacing-xs".to_string(), format!("{}px", self.xs));
        variables.insert("--ant-spacing-sm".to_string(), format!("{}px", self.sm));
        variables.insert("--ant-spacing-md".to_string(), format!("{}px", self.md));
        variables.insert("--ant-spacing-lg".to_string(), format!("{}px", self.lg));
        variables.insert("--ant-spacing-xl".to_string(), format!("{}px", self.xl));
        variables.insert("--ant-spacing-xxl".to_string(), format!("{}px", self.xxl));

        // 组件间距变量
        variables.insert(
            "--ant-card-padding".to_string(),
            format!("{}px", self.card_padding),
        );
        variables.insert(
            "--ant-modal-padding".to_string(),
            format!("{}px", self.modal_padding),
        );
        variables.insert(
            "--ant-grid-gutter".to_string(),
            format!("{}px", self.grid_gutter),
        );
        variables.insert(
            "--ant-container-padding".to_string(),
            format!("{}px", self.container_padding),
        );
        variables.insert(
            "--ant-section-spacing".to_string(),
            format!("{}px", self.section_spacing),
        );

        // 按钮内边距变量
        for (size, (h, v)) in &self.button_padding {
            let size_name = match size {
                Size::Small => "sm",
                Size::Middle => "md",
                Size::Large => "lg",
            };
            variables.insert(
                format!("--ant-button-padding-{}", size_name),
                format!("{}px {}px", v, h),
            );
        }

        // 输入框内边距变量
        for (size, (h, v)) in &self.input_padding {
            let size_name = match size {
                Size::Small => "sm",
                Size::Middle => "md",
                Size::Large => "lg",
            };
            variables.insert(
                format!("--ant-input-padding-{}", size_name),
                format!("{}px {}px", v, h),
            );
        }

        variables
    }

    /// 生成 CSS 样式
    pub fn generate_css(&self) -> String {
        let mut css = String::new();

        // 间距工具类
        for (name, value) in [
            ("xs", self.xs),
            ("sm", self.sm),
            ("md", self.md),
            ("lg", self.lg),
            ("xl", self.xl),
            ("xxl", self.xxl),
        ] {
            // 外边距类
            css.push_str(&format!(".ant-m-{} {{ margin: {}px; }}\n", name, value));
            css.push_str(&format!(
                ".ant-mt-{} {{ margin-top: {}px; }}\n",
                name, value
            ));
            css.push_str(&format!(
                ".ant-mr-{} {{ margin-right: {}px; }}\n",
                name, value
            ));
            css.push_str(&format!(
                ".ant-mb-{} {{ margin-bottom: {}px; }}\n",
                name, value
            ));
            css.push_str(&format!(
                ".ant-ml-{} {{ margin-left: {}px; }}\n",
                name, value
            ));
            css.push_str(&format!(
                ".ant-mx-{} {{ margin-left: {}px; margin-right: {}px; }}\n",
                name, value, value
            ));
            css.push_str(&format!(
                ".ant-my-{} {{ margin-top: {}px; margin-bottom: {}px; }}\n",
                name, value, value
            ));

            // 内边距类
            css.push_str(&format!(".ant-p-{} {{ padding: {}px; }}\n", name, value));
            css.push_str(&format!(
                ".ant-pt-{} {{ padding-top: {}px; }}\n",
                name, value
            ));
            css.push_str(&format!(
                ".ant-pr-{} {{ padding-right: {}px; }}\n",
                name, value
            ));
            css.push_str(&format!(
                ".ant-pb-{} {{ padding-bottom: {}px; }}\n",
                name, value
            ));
            css.push_str(&format!(
                ".ant-pl-{} {{ padding-left: {}px; }}\n",
                name, value
            ));
            css.push_str(&format!(
                ".ant-px-{} {{ padding-left: {}px; padding-right: {}px; }}\n",
                name, value, value
            ));
            css.push_str(&format!(
                ".ant-py-{} {{ padding-top: {}px; padding-bottom: {}px; }}\n",
                name, value, value
            ));

            // 间隙类
            css.push_str(&format!(".ant-gap-{} {{ gap: {}px; }}\n", name, value));
        }

        // 特殊间距类
        css.push_str(&format!(
            ".ant-card-padding {{ padding: {}px; }}\n",
            self.card_padding
        ));
        css.push_str(&format!(
            ".ant-modal-padding {{ padding: {}px; }}\n",
            self.modal_padding
        ));
        css.push_str(&format!(
            ".ant-container-padding {{ padding: {}px; }}\n",
            self.container_padding
        ));

        css
    }

    /// 创建间距配置
    pub fn create_spacing_config(
        &self,
        spacing_type: SpacingType,
        direction: SpacingDirection,
        size: SpaceSize,
    ) -> SpacingConfig {
        SpacingConfig::new(spacing_type, direction, size)
    }
}

impl Default for AntDesignSpacing {
    fn default() -> Self {
        Self::default()
    }
}

/// 间距工具函数
pub mod utils {
    use super::*;

    /// 根据组件大小获取推荐间距
    pub fn get_recommended_spacing(size: Size) -> SpaceSize {
        match size {
            Size::Small => SpaceSize::Small,
            Size::Middle => SpaceSize::Middle,
            Size::Large => SpaceSize::Large,
        }
    }

    /// 计算响应式间距
    pub fn calculate_responsive_spacing(base_spacing: u32, screen_width: u32) -> u32 {
        if screen_width < 576 {
            // 小屏幕，减少间距
            (base_spacing as f32 * 0.75) as u32
        } else if screen_width < 768 {
            // 中等屏幕，稍微减少间距
            (base_spacing as f32 * 0.875) as u32
        } else {
            // 大屏幕，使用原始间距
            base_spacing
        }
    }

    /// 生成间距工具类名
    pub fn generate_spacing_class_name(
        spacing_type: SpacingType,
        direction: SpacingDirection,
        size: &str,
    ) -> String {
        let type_prefix = match spacing_type {
            SpacingType::Padding => "p",
            SpacingType::Margin => "m",
            SpacingType::Gap => "gap",
            SpacingType::Border => "border",
        };

        let direction_suffix = match direction {
            SpacingDirection::All => "",
            SpacingDirection::Vertical => "y",
            SpacingDirection::Horizontal => "x",
            SpacingDirection::Top => "t",
            SpacingDirection::Right => "r",
            SpacingDirection::Bottom => "b",
            SpacingDirection::Left => "l",
        };

        if direction_suffix.is_empty() {
            format!("ant-{}-{}", type_prefix, size)
        } else {
            format!("ant-{}{}-{}", type_prefix, direction_suffix, size)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spacing_direction() {
        assert_eq!(SpacingDirection::All.to_css_suffix(), "");
        assert_eq!(SpacingDirection::Top.to_css_suffix(), "-top");
        assert_eq!(SpacingDirection::Horizontal.to_css_suffix(), "-inline");
    }

    #[test]
    fn test_spacing_config() {
        let config = SpacingConfig::new(
            SpacingType::Padding,
            SpacingDirection::Top,
            SpaceSize::Middle,
        );

        assert_eq!(config.to_css_property(), "padding-top");

        let spacing = AntDesignSpacing::default();
        assert_eq!(config.to_css_value(&spacing), "16px");
        assert_eq!(config.to_css(&spacing), "padding-top: 16px;");
    }

    #[test]
    fn test_ant_design_spacing() {
        let spacing = AntDesignSpacing::default();

        assert_eq!(spacing.base_unit, 8);
        assert_eq!(spacing.xs, 4);
        assert_eq!(spacing.sm, 8);
        assert_eq!(spacing.md, 16);
        assert_eq!(spacing.lg, 24);

        assert_eq!(spacing.get_space(SpaceSize::Middle), Some(16));
        assert_eq!(spacing.get_space(SpaceSize::Custom(20)), Some(20));

        let (h, v) = spacing.get_button_padding(Size::Middle);
        assert_eq!((h, v), (15, 8));

        assert_eq!(spacing.get_spacing_by_multiple(1.5), 12);
    }

    #[test]
    fn test_compact_spacing() {
        let compact = AntDesignSpacing::compact();

        assert_eq!(compact.base_unit, 6);
        assert_eq!(compact.xs, 3);
        assert_eq!(compact.sm, 6);
        assert_eq!(compact.md, 12);

        let (h, v) = compact.get_button_padding(Size::Small);
        assert_eq!((h, v), (5, 2));
    }

    #[test]
    fn test_css_generation() {
        let spacing = AntDesignSpacing::default();

        let variables = spacing.to_css_variables();
        assert!(variables.contains_key("--ant-spacing-base"));
        assert!(variables.contains_key("--ant-spacing-md"));
        assert!(variables.contains_key("--ant-button-padding-md"));

        let css = spacing.generate_css();
        assert!(css.contains(".ant-m-md"));
        assert!(css.contains(".ant-p-lg"));
        assert!(css.contains(".ant-gap-sm"));
    }

    #[test]
    fn test_utils() {
        assert_eq!(
            utils::get_recommended_spacing(Size::Small),
            SpaceSize::Small
        );
        assert_eq!(
            utils::get_recommended_spacing(Size::Large),
            SpaceSize::Large
        );

        assert_eq!(utils::calculate_responsive_spacing(16, 500), 12); // 小屏幕
        assert_eq!(utils::calculate_responsive_spacing(16, 700), 14); // 中等屏幕
        assert_eq!(utils::calculate_responsive_spacing(16, 1000), 16); // 大屏幕

        assert_eq!(
            utils::generate_spacing_class_name(SpacingType::Margin, SpacingDirection::Top, "md"),
            "ant-mt-md"
        );
        assert_eq!(
            utils::generate_spacing_class_name(SpacingType::Padding, SpacingDirection::All, "lg"),
            "ant-p-lg"
        );
    }
}

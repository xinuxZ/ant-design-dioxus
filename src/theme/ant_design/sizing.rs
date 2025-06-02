//! Ant Design 尺寸规范
//!
//! 实现 Ant Design 的尺寸系统，包括：
//! - 组件尺寸
//! - 边框圆角
//! - 阴影
//! - 边框宽度

use crate::theme::core::{Size, SizeConfig};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 组件类型枚举
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ComponentType {
    /// 按钮
    Button,
    /// 输入框
    Input,
    /// 选择器
    Select,
    /// 开关
    Switch,
    /// 滑块
    Slider,
    /// 复选框
    Checkbox,
    /// 单选框
    Radio,
    /// 标签
    Tag,
    /// 徽章
    Badge,
    /// 头像
    Avatar,
    /// 图标
    Icon,
    /// 分页
    Pagination,
    /// 步骤条
    Steps,
    /// 进度条
    Progress,
    /// 卡片
    Card,
    /// 模态框
    Modal,
    /// 抽屉
    Drawer,
    /// 工具提示
    Tooltip,
    /// 气泡确认框
    Popconfirm,
    /// 下拉菜单
    Dropdown,
    /// 菜单
    Menu,
    /// 表格
    Table,
    /// 列表
    List,
    /// 树形控件
    Tree,
    /// 面包屑
    Breadcrumb,
    /// 标签页
    Tabs,
    /// 折叠面板
    Collapse,
    /// 轮播图
    Carousel,
    /// 锚点
    Anchor,
    /// 回到顶部
    BackTop,
    /// 分割线
    Divider,
}

/// 边框圆角配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BorderRadius {
    /// 小圆角
    pub small: u32,
    /// 基础圆角
    pub base: u32,
    /// 大圆角
    pub large: u32,
    /// 圆形
    pub circle: String,
}

impl BorderRadius {
    /// 创建默认边框圆角
    pub fn default() -> Self {
        Self {
            small: 2,
            base: 6,
            large: 8,
            circle: "50%".to_string(),
        }
    }

    /// 创建紧凑边框圆角
    pub fn compact() -> Self {
        Self {
            small: 1,
            base: 4,
            large: 6,
            circle: "50%".to_string(),
        }
    }

    /// 根据尺寸获取圆角值
    pub fn get_radius(&self, size: Size) -> u32 {
        match size {
            Size::Small => self.small,
            Size::Middle => self.base,
            Size::Large => self.large,
        }
    }

    /// 转换为 CSS 值
    pub fn to_css(&self, size: Size) -> String {
        format!("{}px", self.get_radius(size))
    }
}

/// 阴影配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BoxShadow {
    /// 无阴影
    pub none: String,
    /// 小阴影
    pub small: String,
    /// 基础阴影
    pub base: String,
    /// 大阴影
    pub large: String,
    /// 内阴影
    pub inset: String,
}

impl BoxShadow {
    /// 创建默认阴影
    pub fn default() -> Self {
        Self {
            none: "none".to_string(),
            small: "0 1px 2px 0 rgba(0, 0, 0, 0.03), 0 1px 6px -1px rgba(0, 0, 0, 0.02), 0 2px 4px 0 rgba(0, 0, 0, 0.02)".to_string(),
            base: "0 1px 2px 0 rgba(0, 0, 0, 0.03), 0 1px 6px -1px rgba(0, 0, 0, 0.02), 0 2px 4px 0 rgba(0, 0, 0, 0.02)".to_string(),
            large: "0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06)".to_string(),
            inset: "inset 0 2px 4px 0 rgba(0, 0, 0, 0.06)".to_string(),
        }
    }

    /// 根据尺寸获取阴影
    pub fn get_shadow(&self, size: Size) -> &String {
        match size {
            Size::Small => &self.small,
            Size::Middle => &self.base,
            Size::Large => &self.large,
        }
    }
}

/// 边框宽度配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BorderWidth {
    /// 无边框
    pub none: u32,
    /// 细边框
    pub thin: u32,
    /// 基础边框
    pub base: u32,
    /// 粗边框
    pub thick: u32,
}

impl BorderWidth {
    /// 创建默认边框宽度
    pub fn default() -> Self {
        Self {
            none: 0,
            thin: 1,
            base: 1,
            thick: 2,
        }
    }

    /// 根据尺寸获取边框宽度
    pub fn get_width(&self, size: Size) -> u32 {
        match size {
            Size::Small => self.thin,
            Size::Middle => self.base,
            Size::Large => self.thick,
        }
    }

    /// 转换为 CSS 值
    pub fn to_css(&self, size: Size) -> String {
        format!("{}px", self.get_width(size))
    }
}

/// Ant Design 尺寸规范
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AntDesignSizing {
    /// 组件尺寸配置
    pub component_sizes: HashMap<ComponentType, HashMap<Size, SizeConfig>>,
    /// 边框圆角配置
    pub border_radius: BorderRadius,
    /// 阴影配置
    pub box_shadow: BoxShadow,
    /// 边框宽度配置
    pub border_width: BorderWidth,
    /// 最小触摸目标尺寸
    pub min_touch_target: u32,
    /// 图标尺寸映射
    pub icon_sizes: HashMap<Size, u32>,
}

impl AntDesignSizing {
    /// 创建默认尺寸规范
    pub fn default() -> Self {
        let mut component_sizes = HashMap::new();

        // 按钮尺寸
        let mut button_sizes = HashMap::new();
        button_sizes.insert(Size::Small, SizeConfig::new(24, 7, 4, 14, 6));
        button_sizes.insert(Size::Middle, SizeConfig::new(32, 15, 8, 14, 6));
        button_sizes.insert(Size::Large, SizeConfig::new(40, 15, 10, 16, 6));
        component_sizes.insert(ComponentType::Button, button_sizes);

        // 输入框尺寸
        let mut input_sizes = HashMap::new();
        input_sizes.insert(Size::Small, SizeConfig::new(24, 7, 4, 14, 6));
        input_sizes.insert(Size::Middle, SizeConfig::new(32, 11, 8, 14, 6));
        input_sizes.insert(Size::Large, SizeConfig::new(40, 11, 10, 16, 6));
        component_sizes.insert(ComponentType::Input, input_sizes);

        // 选择器尺寸
        let mut select_sizes = HashMap::new();
        select_sizes.insert(Size::Small, SizeConfig::new(24, 7, 4, 14, 6));
        select_sizes.insert(Size::Middle, SizeConfig::new(32, 11, 8, 14, 6));
        select_sizes.insert(Size::Large, SizeConfig::new(40, 11, 10, 16, 6));
        component_sizes.insert(ComponentType::Select, select_sizes);

        // 标签尺寸
        let mut tag_sizes = HashMap::new();
        tag_sizes.insert(Size::Small, SizeConfig::new(20, 4, 2, 12, 2));
        tag_sizes.insert(Size::Middle, SizeConfig::new(24, 7, 4, 14, 6));
        tag_sizes.insert(Size::Large, SizeConfig::new(32, 8, 6, 16, 6));
        component_sizes.insert(ComponentType::Tag, tag_sizes);

        // 头像尺寸
        let mut avatar_sizes = HashMap::new();
        avatar_sizes.insert(Size::Small, SizeConfig::new(24, 0, 0, 14, 6));
        avatar_sizes.insert(Size::Middle, SizeConfig::new(32, 0, 0, 14, 6));
        avatar_sizes.insert(Size::Large, SizeConfig::new(40, 0, 0, 16, 6));
        component_sizes.insert(ComponentType::Avatar, avatar_sizes);

        // 图标尺寸映射
        let mut icon_sizes = HashMap::new();
        icon_sizes.insert(Size::Small, 12);
        icon_sizes.insert(Size::Middle, 14);
        icon_sizes.insert(Size::Large, 16);

        Self {
            component_sizes,
            border_radius: BorderRadius::default(),
            box_shadow: BoxShadow::default(),
            border_width: BorderWidth::default(),
            min_touch_target: 44, // 符合无障碍标准
            icon_sizes,
        }
    }

    /// 创建紧凑尺寸规范
    pub fn compact() -> Self {
        let mut sizing = Self::default();

        // 更新按钮尺寸
        let mut button_sizes = HashMap::new();
        button_sizes.insert(Size::Small, SizeConfig::new(20, 5, 2, 12, 4));
        button_sizes.insert(Size::Middle, SizeConfig::new(28, 11, 6, 14, 4));
        button_sizes.insert(Size::Large, SizeConfig::new(36, 11, 8, 16, 4));
        sizing
            .component_sizes
            .insert(ComponentType::Button, button_sizes);

        // 更新输入框尺寸
        let mut input_sizes = HashMap::new();
        input_sizes.insert(Size::Small, SizeConfig::new(20, 5, 2, 12, 4));
        input_sizes.insert(Size::Middle, SizeConfig::new(28, 8, 6, 14, 4));
        input_sizes.insert(Size::Large, SizeConfig::new(36, 8, 8, 16, 4));
        sizing
            .component_sizes
            .insert(ComponentType::Input, input_sizes);

        // 更新边框圆角
        sizing.border_radius = BorderRadius::compact();

        // 更新图标尺寸
        sizing.icon_sizes.insert(Size::Small, 10);
        sizing.icon_sizes.insert(Size::Middle, 12);
        sizing.icon_sizes.insert(Size::Large, 14);

        sizing
    }

    /// 获取组件尺寸配置
    pub fn get_component_size(&self, component: ComponentType, size: Size) -> Option<&SizeConfig> {
        self.component_sizes
            .get(&component)
            .and_then(|sizes| sizes.get(&size))
    }

    /// 获取指定尺寸的像素值
    pub fn get_size(&self, size: Size) -> Option<u32> {
        // 使用按钮作为默认尺寸参考
        self.get_component_size(ComponentType::Button, size)
            .map(|config| config.height)
    }

    /// 获取图标尺寸
    pub fn get_icon_size(&self, size: Size) -> u32 {
        self.icon_sizes.get(&size).copied().unwrap_or(14)
    }

    /// 获取边框圆角
    pub fn get_border_radius(&self, size: Size) -> u32 {
        self.border_radius.get_radius(size)
    }

    /// 获取阴影
    pub fn get_box_shadow(&self, size: Size) -> &String {
        self.box_shadow.get_shadow(size)
    }

    /// 获取边框宽度
    pub fn get_border_width(&self, size: Size) -> u32 {
        self.border_width.get_width(size)
    }

    /// 检查尺寸是否符合最小触摸目标
    pub fn is_touch_friendly(&self, component: ComponentType, size: Size) -> bool {
        if let Some(config) = self.get_component_size(component, size) {
            config.height >= self.min_touch_target
        } else {
            false
        }
    }

    /// 生成 CSS 变量
    pub fn to_css_variables(&self) -> HashMap<String, String> {
        let mut variables = HashMap::new();

        // 边框圆角变量
        variables.insert(
            "--ant-border-radius-sm".to_string(),
            format!("{}px", self.border_radius.small),
        );
        variables.insert(
            "--ant-border-radius-base".to_string(),
            format!("{}px", self.border_radius.base),
        );
        variables.insert(
            "--ant-border-radius-lg".to_string(),
            format!("{}px", self.border_radius.large),
        );
        variables.insert(
            "--ant-border-radius-circle".to_string(),
            self.border_radius.circle.clone(),
        );

        // 阴影变量
        variables.insert(
            "--ant-box-shadow-sm".to_string(),
            self.box_shadow.small.clone(),
        );
        variables.insert(
            "--ant-box-shadow-base".to_string(),
            self.box_shadow.base.clone(),
        );
        variables.insert(
            "--ant-box-shadow-lg".to_string(),
            self.box_shadow.large.clone(),
        );
        variables.insert(
            "--ant-box-shadow-inset".to_string(),
            self.box_shadow.inset.clone(),
        );

        // 边框宽度变量
        variables.insert(
            "--ant-border-width-sm".to_string(),
            format!("{}px", self.border_width.thin),
        );
        variables.insert(
            "--ant-border-width-base".to_string(),
            format!("{}px", self.border_width.base),
        );
        variables.insert(
            "--ant-border-width-lg".to_string(),
            format!("{}px", self.border_width.thick),
        );

        // 组件尺寸变量
        for (component, sizes) in &self.component_sizes {
            let component_name = format!("{:?}", component).to_lowercase();
            for (size, config) in sizes {
                let size_name = match size {
                    Size::Small => "sm",
                    Size::Middle => "md",
                    Size::Large => "lg",
                };

                variables.insert(
                    format!("--ant-{}-height-{}", component_name, size_name),
                    format!("{}px", config.height),
                );
                variables.insert(
                    format!("--ant-{}-padding-{}", component_name, size_name),
                    format!(
                        "{}px {}px",
                        config.padding_horizontal, config.padding_vertical
                    ),
                );
                variables.insert(
                    format!("--ant-{}-font-size-{}", component_name, size_name),
                    format!("{}px", config.font_size),
                );
            }
        }

        // 图标尺寸变量
        for (size, pixels) in &self.icon_sizes {
            let size_name = match size {
                Size::Small => "sm",
                Size::Middle => "md",
                Size::Large => "lg",
            };
            variables.insert(
                format!("--ant-icon-size-{}", size_name),
                format!("{}px", pixels),
            );
        }

        variables
    }

    /// 生成 CSS 样式
    pub fn generate_css(&self) -> String {
        let mut css = String::new();

        // 边框圆角工具类
        css.push_str(&format!(
            ".ant-rounded-sm {{ border-radius: {}px; }}\n",
            self.border_radius.small
        ));
        css.push_str(&format!(
            ".ant-rounded {{ border-radius: {}px; }}\n",
            self.border_radius.base
        ));
        css.push_str(&format!(
            ".ant-rounded-lg {{ border-radius: {}px; }}\n",
            self.border_radius.large
        ));
        css.push_str(&format!(
            ".ant-rounded-full {{ border-radius: {}; }}\n",
            self.border_radius.circle
        ));

        // 阴影工具类
        css.push_str(&format!(
            ".ant-shadow-sm {{ box-shadow: {}; }}\n",
            self.box_shadow.small
        ));
        css.push_str(&format!(
            ".ant-shadow {{ box-shadow: {}; }}\n",
            self.box_shadow.base
        ));
        css.push_str(&format!(
            ".ant-shadow-lg {{ box-shadow: {}; }}\n",
            self.box_shadow.large
        ));
        css.push_str(&format!(
            ".ant-shadow-inset {{ box-shadow: {}; }}\n",
            self.box_shadow.inset
        ));
        css.push_str(".ant-shadow-none { box-shadow: none; }\n");

        // 边框宽度工具类
        css.push_str(&format!(
            ".ant-border-sm {{ border-width: {}px; }}\n",
            self.border_width.thin
        ));
        css.push_str(&format!(
            ".ant-border {{ border-width: {}px; }}\n",
            self.border_width.base
        ));
        css.push_str(&format!(
            ".ant-border-lg {{ border-width: {}px; }}\n",
            self.border_width.thick
        ));
        css.push_str(".ant-border-none { border-width: 0; }\n");

        // 尺寸工具类
        for (size_name, size_enum) in [
            ("sm", Size::Small),
            ("md", Size::Middle),
            ("lg", Size::Large),
        ] {
            if let Some(config) = self.get_component_size(ComponentType::Button, size_enum) {
                css.push_str(&format!(
                    ".ant-size-{} {{ height: {}px; }}\n",
                    size_name, config.height
                ));
            }
        }

        css
    }
}

impl Default for AntDesignSizing {
    fn default() -> Self {
        Self::default()
    }
}

/// 尺寸工具函数
pub mod utils {
    use super::*;

    /// 根据内容计算推荐尺寸
    pub fn calculate_recommended_size(content_length: usize) -> Size {
        if content_length < 10 {
            Size::Small
        } else if content_length < 50 {
            Size::Middle
        } else {
            Size::Large
        }
    }

    /// 检查尺寸是否适合移动端
    pub fn is_mobile_friendly(height: u32) -> bool {
        height >= 44 // iOS 人机界面指南推荐的最小触摸目标
    }

    /// 计算响应式尺寸
    pub fn calculate_responsive_size(base_size: u32, screen_width: u32) -> u32 {
        if screen_width < 576 {
            // 小屏幕，稍微减小尺寸
            (base_size as f32 * 0.9) as u32
        } else {
            base_size
        }
    }

    /// 生成尺寸类名
    pub fn generate_size_class_name(component: ComponentType, size: Size) -> String {
        let component_name = format!("{:?}", component).to_lowercase();
        let size_name = match size {
            Size::Small => "sm",
            Size::Middle => "md",
            Size::Large => "lg",
        };

        format!("ant-{}-{}", component_name, size_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_border_radius() {
        let radius = BorderRadius::default();

        assert_eq!(radius.get_radius(Size::Small), 2);
        assert_eq!(radius.get_radius(Size::Middle), 6);
        assert_eq!(radius.get_radius(Size::Large), 8);

        assert_eq!(radius.to_css(Size::Middle), "6px");

        let compact = BorderRadius::compact();
        assert_eq!(compact.get_radius(Size::Small), 1);
    }

    #[test]
    fn test_box_shadow() {
        let shadow = BoxShadow::default();

        assert_eq!(shadow.get_shadow(Size::Small), &shadow.small);
        assert_eq!(shadow.get_shadow(Size::Middle), &shadow.base);
        assert_eq!(shadow.get_shadow(Size::Large), &shadow.large);
    }

    #[test]
    fn test_border_width() {
        let width = BorderWidth::default();

        assert_eq!(width.get_width(Size::Small), 1);
        assert_eq!(width.get_width(Size::Middle), 1);
        assert_eq!(width.get_width(Size::Large), 2);

        assert_eq!(width.to_css(Size::Large), "2px");
    }

    #[test]
    fn test_ant_design_sizing() {
        let sizing = AntDesignSizing::default();

        // 测试按钮尺寸
        let button_config: &SizeConfig = sizing
            .get_component_size(ComponentType::Button, Size::Middle)
            .unwrap();
        assert_eq!(button_config.height, 32);
        assert_eq!(
            (
                button_config.padding_horizontal,
                button_config.padding_vertical
            ),
            (15, 8)
        );

        // 测试图标尺寸
        assert_eq!(sizing.get_icon_size(Size::Middle), 14);

        // 测试边框圆角
        assert_eq!(sizing.get_border_radius(Size::Middle), 6);

        // 测试触摸友好性
        assert!(sizing.is_touch_friendly(ComponentType::Button, Size::Large));
        assert!(!sizing.is_touch_friendly(ComponentType::Button, Size::Small));
    }

    #[test]
    fn test_compact_sizing() {
        let compact = AntDesignSizing::compact();

        let button_config = compact
            .get_component_size(ComponentType::Button, Size::Middle)
            .unwrap();
        assert_eq!(button_config.height, 28);

        assert_eq!(compact.get_icon_size(Size::Middle), 12);
        assert_eq!(compact.get_border_radius(Size::Middle), 4);
    }

    #[test]
    fn test_css_generation() {
        let sizing = AntDesignSizing::default();

        let variables = sizing.to_css_variables();
        assert!(variables.contains_key("--ant-border-radius-base"));
        assert!(variables.contains_key("--ant-box-shadow-base"));
        assert!(variables.contains_key("--ant-button-height-md"));

        let css = sizing.generate_css();
        assert!(css.contains(".ant-rounded"));
        assert!(css.contains(".ant-shadow"));
        assert!(css.contains(".ant-border"));
    }

    #[test]
    fn test_utils() {
        assert_eq!(utils::calculate_recommended_size(5), Size::Small);
        assert_eq!(utils::calculate_recommended_size(25), Size::Middle);
        assert_eq!(utils::calculate_recommended_size(100), Size::Large);

        assert!(utils::is_mobile_friendly(44));
        assert!(!utils::is_mobile_friendly(30));

        assert_eq!(utils::calculate_responsive_size(40, 500), 36);
        assert_eq!(utils::calculate_responsive_size(40, 800), 40);

        assert_eq!(
            utils::generate_size_class_name(ComponentType::Button, Size::Large),
            "ant-button-lg"
        );
    }
}

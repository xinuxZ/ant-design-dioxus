//! Dropdown 下拉菜单组件样式模块
//!
//! 本模块包含 Dropdown 组件的所有样式定义，从组件逻辑中分离出来，
//! 提高代码的可维护性和复用性。

use crate::theme::AliasToken;
use css_in_rust::css;
use serde::{Deserialize, Serialize};

/// 下拉菜单触发方式
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DropdownTrigger {
    /// 点击触发
    Click,
    /// 悬停触发
    Hover,
    /// 右键触发
    ContextMenu,
}

impl Default for DropdownTrigger {
    fn default() -> Self {
        DropdownTrigger::Hover
    }
}

/// 下拉菜单弹出位置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DropdownPlacement {
    /// 下方左对齐
    BottomLeft,
    /// 下方居中
    Bottom,
    /// 下方右对齐
    BottomRight,
    /// 上方左对齐
    TopLeft,
    /// 上方居中
    Top,
    /// 上方右对齐
    TopRight,
}

impl Default for DropdownPlacement {
    fn default() -> Self {
        DropdownPlacement::BottomLeft
    }
}

/// 下拉菜单主题
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DropdownTheme {
    /// 亮色主题（默认）
    Light,
    /// 暗色主题
    Dark,
}

impl Default for DropdownTheme {
    fn default() -> Self {
        Self::Light
    }
}

/// 下拉菜单尺寸
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DropdownSize {
    /// 大尺寸
    Large,
    /// 中等尺寸（默认）
    Middle,
    /// 小尺寸
    Small,
    /// 紧凑尺寸
    Compact,
}

impl Default for DropdownSize {
    fn default() -> Self {
        Self::Middle
    }
}

/// 下拉菜单样式生成器
pub struct DropdownStyleGenerator {
    /// 下拉菜单位置
    pub placement: DropdownPlacement,
    /// 下拉菜单主题
    pub theme: DropdownTheme,
    /// 下拉菜单尺寸
    pub size: DropdownSize,
    /// 是否显示箭头
    pub arrow: bool,
    /// 是否禁用
    pub disabled: bool,
    /// 主题令牌
    pub token: AliasToken,
}

impl DropdownStyleGenerator {
    /// 创建新的下拉菜单样式生成器
    pub fn new() -> Self {
        Self {
            placement: DropdownPlacement::default(),
            theme: DropdownTheme::default(),
            size: DropdownSize::default(),
            arrow: false,
            disabled: false,
            token: AliasToken::default(),
        }
    }

    /// 设置下拉菜单位置
    pub fn with_placement(mut self, placement: DropdownPlacement) -> Self {
        self.placement = placement;
        self
    }

    /// 设置下拉菜单主题
    pub fn with_theme(mut self, theme: DropdownTheme) -> Self {
        self.theme = theme;
        self
    }

    /// 设置下拉菜单尺寸
    pub fn with_size(mut self, size: DropdownSize) -> Self {
        self.size = size;
        self
    }

    /// 设置是否显示箭头
    pub fn with_arrow(mut self, arrow: bool) -> Self {
        self.arrow = arrow;
        self
    }

    /// 设置是否禁用
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// 设置主题令牌
    pub fn with_token(mut self, token: AliasToken) -> Self {
        self.token = token;
        self
    }

    /// 生成完整的下拉菜单样式类名
    pub fn generate(&self) -> String {
        let mut classes = vec!["ant-dropdown".to_string()];

        // 添加位置类名
        match self.placement {
            DropdownPlacement::BottomLeft => {
                classes.push("ant-dropdown-placement-bottomLeft".to_string())
            }
            DropdownPlacement::Bottom => classes.push("ant-dropdown-placement-bottom".to_string()),
            DropdownPlacement::BottomRight => {
                classes.push("ant-dropdown-placement-bottomRight".to_string())
            }
            DropdownPlacement::TopLeft => {
                classes.push("ant-dropdown-placement-topLeft".to_string())
            }
            DropdownPlacement::Top => classes.push("ant-dropdown-placement-top".to_string()),
            DropdownPlacement::TopRight => {
                classes.push("ant-dropdown-placement-topRight".to_string())
            }
        }

        // 添加箭头类名
        if self.arrow {
            classes.push("ant-dropdown-arrow".to_string());
        }

        // 添加主题类名
        match self.theme {
            DropdownTheme::Light => {}
            DropdownTheme::Dark => classes.push("ant-dropdown-menu-dark".to_string()),
        }

        // 添加尺寸类名
        match self.size {
            DropdownSize::Large => classes.push("ant-dropdown-menu-large".to_string()),
            DropdownSize::Middle => {}
            DropdownSize::Small => classes.push("ant-dropdown-menu-small".to_string()),
            DropdownSize::Compact => classes.push("ant-dropdown-menu-compact".to_string()),
        }

        // 添加禁用类名
        if self.disabled {
            classes.push("ant-dropdown-disabled".to_string());
        }

        classes.join(" ")
    }

    /// 生成 CSS 样式
    pub fn generate_css(&self) -> String {
        css!(
            r#"
            .ant-dropdown {
                position: absolute;
                top: -9999px;
                left: -9999px;
                z-index: ${z_index};
                display: block;
                box-sizing: border-box;
                margin: 0;
                padding: 0;
                color: ${color_text};
                font-size: ${font_size}px;
                font-variant: tabular-nums;
                line-height: ${line_height};
                list-style: none;
                font-feature-settings: 'tnum';
                border-radius: ${border_radius}px;
                box-shadow: ${box_shadow};
                transform-origin: 50% 0;
            }

            .ant-dropdown-hidden {
                display: none;
            }

            .ant-dropdown-menu {
                position: relative;
                margin: 0;
                padding: ${padding_xs}px 0;
                text-align: left;
                background-color: ${bg_color_container};
                background-clip: padding-box;
                border-radius: ${border_radius}px;
                outline: none;
                box-shadow: ${box_shadow};
                list-style-type: none;
            }

            .ant-dropdown-menu-item {
                position: relative;
                display: flex;
                align-items: center;
                padding: ${padding_xs}px ${padding_sm}px;
                color: ${color_text};
                font-weight: normal;
                font-size: ${font_size}px;
                line-height: ${line_height};
                cursor: pointer;
                transition: all 0.3s;
            }

            .ant-dropdown-menu-item:hover {
                background-color: ${color_bg_text_hover};
            }

            .ant-dropdown-menu-item-disabled {
                color: ${color_text_disabled};
                cursor: not-allowed;
            }

            .ant-dropdown-menu-item-disabled:hover {
                color: ${color_text_disabled};
                background-color: transparent;
                cursor: not-allowed;
            }

            .ant-dropdown-menu-item-divider {
                height: 1px;
                margin: ${margin_xs}px 0;
                overflow: hidden;
                line-height: 0;
                background-color: ${color_split};
            }

            .ant-dropdown-menu-item-selected {
                color: ${color_primary};
                background-color: ${color_bg_text_active};
            }

            .ant-dropdown-menu-submenu {
                position: relative;
            }

            .ant-dropdown-menu-submenu-title {
                display: flex;
                align-items: center;
                padding: ${padding_xs}px ${padding_sm}px;
                color: ${color_text};
                font-weight: normal;
                font-size: ${font_size}px;
                line-height: ${line_height};
                cursor: pointer;
                transition: all 0.3s;
            }

            .ant-dropdown-menu-submenu-arrow {
                position: absolute;
                right: ${padding_sm}px;
                color: ${color_text_secondary};
            }

            .ant-dropdown-menu-dark {
                background-color: ${color_bg_container_dark};
            }

            .ant-dropdown-menu-dark .ant-dropdown-menu-item {
                color: ${color_text_secondary_dark};
            }

            .ant-dropdown-menu-dark .ant-dropdown-menu-item:hover {
                color: ${color_text_dark};
                background-color: ${color_bg_text_hover_dark};
            }

            .ant-dropdown-menu-dark .ant-dropdown-menu-item-selected {
                color: ${color_text_dark};
                background-color: ${color_primary};
            }

            .ant-dropdown-menu-dark .ant-dropdown-menu-submenu-title {
                color: ${color_text_secondary_dark};
            }

            .ant-dropdown-menu-dark .ant-dropdown-menu-divider {
                background-color: ${color_split_dark};
            }

            .ant-dropdown-menu-large .ant-dropdown-menu-item,
            .ant-dropdown-menu-large .ant-dropdown-menu-submenu-title {
                padding: ${padding_sm}px ${padding_md}px;
                font-size: ${font_size_lg}px;
            }

            .ant-dropdown-menu-small .ant-dropdown-menu-item,
            .ant-dropdown-menu-small .ant-dropdown-menu-submenu-title {
                padding: ${padding_xs_sm}px ${padding_xs}px;
                font-size: ${font_size_sm}px;
            }

            .ant-dropdown-menu-compact .ant-dropdown-menu-item,
            .ant-dropdown-menu-compact .ant-dropdown-menu-submenu-title {
                padding: ${padding_xs_sm}px ${padding_xs}px;
                font-size: ${font_size_sm}px;
                line-height: 1.4;
            }

            .ant-dropdown-arrow {
                position: absolute;
                display: block;
                width: 8px;
                height: 8px;
                background-color: ${bg_color_container};
                transform: rotate(45deg);
                z-index: -1;
            }

            .ant-dropdown-placement-top .ant-dropdown-arrow,
            .ant-dropdown-placement-topLeft .ant-dropdown-arrow,
            .ant-dropdown-placement-topRight .ant-dropdown-arrow {
                bottom: -4px;
                box-shadow: 3px 3px 7px -3px rgba(0, 0, 0, 0.1);
            }

            .ant-dropdown-placement-bottom .ant-dropdown-arrow,
            .ant-dropdown-placement-bottomLeft .ant-dropdown-arrow,
            .ant-dropdown-placement-bottomRight .ant-dropdown-arrow {
                top: -4px;
                box-shadow: -2px -2px 5px rgba(0, 0, 0, 0.06);
            }

            .ant-dropdown-placement-top .ant-dropdown-arrow,
            .ant-dropdown-placement-bottom .ant-dropdown-arrow {
                left: 50%;
                transform: translateX(-50%) rotate(45deg);
            }

            .ant-dropdown-placement-topLeft .ant-dropdown-arrow,
            .ant-dropdown-placement-bottomLeft .ant-dropdown-arrow {
                left: 16px;
            }

            .ant-dropdown-placement-topRight .ant-dropdown-arrow,
            .ant-dropdown-placement-bottomRight .ant-dropdown-arrow {
                right: 16px;
            }

            .ant-dropdown-disabled .ant-dropdown-trigger {
                cursor: not-allowed;
                color: ${color_text_disabled};
            }
            "#,
            z_index = self.token.z_index_popup,
            color_text = self.token.color_text,
            font_size = self.token.font_size,
            line_height = self.token.line_height,
            border_radius = self.token.border_radius,
            box_shadow = self.token.box_shadow_secondary,
            padding_xs = self.token.padding_xs,
            padding_sm = self.token.padding_sm,
            padding_md = self.token.padding_md,
            padding_xs_sm = 2,
            margin_xs = self.token.margin_xs,
            bg_color_container = self.token.color_bg_container,
            color_bg_text_hover = self.token.color_bg_text_hover,
            color_text_disabled = self.token.color_text_disabled,
            color_split = self.token.color_split,
            color_primary = self.token.color_primary,
            color_bg_text_active = self.token.color_bg_text_active,
            color_text_secondary = self.token.color_text_secondary,
            color_bg_container_dark = "#1f1f1f",
            color_text_secondary_dark = "rgba(255, 255, 255, 0.65)",
            color_text_dark = "rgba(255, 255, 255, 0.85)",
            color_bg_text_hover_dark = "rgba(255, 255, 255, 0.08)",
            color_split_dark = "#424242",
            font_size_lg = self.token.font_size_lg,
            font_size_sm = self.token.font_size_sm
        )
        .to_string()
    }
}

/// 生成 Dropdown 样式
pub fn generate_dropdown_style(
    placement: DropdownPlacement,
    theme: DropdownTheme,
    size: DropdownSize,
    arrow: bool,
    disabled: bool,
) -> String {
    DropdownStyleGenerator::new()
        .with_placement(placement)
        .with_theme(theme)
        .with_size(size)
        .with_arrow(arrow)
        .with_disabled(disabled)
        .generate()
}

/// 生成 Dropdown CSS 样式
pub fn generate_dropdown_css(
    placement: DropdownPlacement,
    theme: DropdownTheme,
    size: DropdownSize,
    arrow: bool,
    disabled: bool,
) -> String {
    DropdownStyleGenerator::new()
        .with_placement(placement)
        .with_theme(theme)
        .with_size(size)
        .with_arrow(arrow)
        .with_disabled(disabled)
        .generate_css()
}

/// 默认 Dropdown 样式
pub fn default_dropdown_style() -> String {
    DropdownStyleGenerator::new().generate()
}

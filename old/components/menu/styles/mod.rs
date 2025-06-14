//! Menu 导航菜单组件样式模块
//!
//! 本模块包含 Menu 组件的所有样式定义，从组件逻辑中分离出来，
//! 提高代码的可维护性和复用性。

use crate::theme::AliasToken;
use css_in_rust::css;
use serde::{Deserialize, Serialize};

/// 菜单模式
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MenuMode {
    /// 垂直菜单
    Vertical,
    /// 水平菜单
    Horizontal,
    /// 内嵌菜单
    Inline,
}

impl Default for MenuMode {
    fn default() -> Self {
        Self::Vertical
    }
}

/// 菜单主题
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MenuTheme {
    /// 亮色主题
    Light,
    /// 暗色主题
    Dark,
}

impl Default for MenuTheme {
    fn default() -> Self {
        Self::Light
    }
}

/// 菜单样式生成器
pub struct MenuStyleGenerator {
    /// 菜单模式
    pub mode: MenuMode,
    /// 菜单主题
    pub theme: MenuTheme,
    /// 是否折叠
    pub collapsed: bool,
    /// 主题令牌
    pub token: AliasToken,
}

impl MenuStyleGenerator {
    /// 创建新的菜单样式生成器
    pub fn new() -> Self {
        Self {
            mode: MenuMode::default(),
            theme: MenuTheme::default(),
            collapsed: false,
            token: AliasToken::default(),
        }
    }

    /// 设置菜单模式
    pub fn with_mode(mut self, mode: MenuMode) -> Self {
        self.mode = mode;
        self
    }

    /// 设置菜单主题
    pub fn with_theme(mut self, theme: MenuTheme) -> Self {
        self.theme = theme;
        self
    }

    /// 设置是否折叠
    pub fn with_collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = collapsed;
        self
    }

    /// 设置主题令牌
    pub fn with_token(mut self, token: AliasToken) -> Self {
        self.token = token;
        self
    }

    /// 生成完整的菜单样式类名
    pub fn generate(&self) -> String {
        let mut classes = vec!["ant-menu".to_string()];

        // 根据模式添加相应样式
        match self.mode {
            MenuMode::Horizontal => classes.push("ant-menu-horizontal".to_string()),
            MenuMode::Vertical => classes.push("ant-menu-vertical".to_string()),
            MenuMode::Inline => classes.push("ant-menu-inline".to_string()),
        }

        // 根据主题添加相应样式
        match self.theme {
            MenuTheme::Light => classes.push("ant-menu-light".to_string()),
            MenuTheme::Dark => classes.push("ant-menu-dark".to_string()),
        }

        // 折叠状态
        if self.collapsed {
            classes.push("ant-menu-collapsed".to_string());
        }

        classes.join(" ")
    }

    /// 生成 CSS 样式
    pub fn generate_css(&self) -> String {
        css!(
            r#"
            .ant-menu {
                margin: 0;
                padding: 0;
                color: ${color_text};
                font-size: ${font_size}px;
                line-height: ${line_height};
                list-style: none;
                font-feature-settings: 'tnum';
                background: ${bg_color_container};
                border: 0;
                border-radius: 0;
                outline: none;
                box-shadow: none;
                transition: background 0.3s, width 0.3s cubic-bezier(0.2, 0, 0, 1) 0s;
                box-sizing: border-box;
            }

            .ant-menu::before {
                display: table;
                content: '';
            }

            .ant-menu::after {
                display: table;
                clear: both;
                content: '';
            }

            .ant-menu ul,
            .ant-menu ol {
                margin: 0;
                padding: 0;
                list-style: none;
            }

            .ant-menu-hidden {
                display: none;
            }

            .ant-menu-item-group-title {
                padding: ${padding_xs}px ${padding_sm}px;
                color: ${color_text_secondary};
                font-size: ${font_size}px;
                line-height: ${line_height};
                transition: all 0.3s;
            }

            .ant-menu-submenu,
            .ant-menu-item {
                position: relative;
                display: block;
                margin: 0;
                padding: 0 ${padding_lg}px;
                color: ${color_text};
                font-weight: 400;
                font-size: ${font_size}px;
                line-height: ${height_base}px;
                cursor: pointer;
                transition: border-color 0.3s, background 0.3s, padding 0.15s cubic-bezier(0.645, 0.045, 0.355, 1);
                border-radius: ${border_radius}px;
                margin-inline: 4px;
                margin-block: 4px;
                width: calc(100% - 8px);
            }

            .ant-menu-item:active,
            .ant-menu-submenu-title:active {
                background: ${color_bg_text_active};
            }

            .ant-menu-submenu .ant-menu-sub {
                cursor: initial;
                transition: background 0.3s cubic-bezier(0.645, 0.045, 0.355, 1), padding 0.3s cubic-bezier(0.645, 0.045, 0.355, 1);
            }

            .ant-menu-item > a {
                display: block;
                color: ${color_text};
                text-decoration: none;
            }

            .ant-menu-item > a:hover {
                color: ${color_text};
            }

            .ant-menu-item > a::before {
                position: absolute;
                top: 0;
                right: 0;
                bottom: 0;
                left: 0;
                background-color: transparent;
                content: '';
            }

            .ant-menu-item-divider {
                height: 1px;
                overflow: hidden;
                line-height: 0;
                background-color: ${color_split};
                margin: ${margin_xs}px 0;
            }

            .ant-menu-item:hover,
            .ant-menu-submenu:hover,
            .ant-menu-submenu-title:hover {
                color: ${color_primary};
                background-color: ${color_bg_text_hover};
            }

            .ant-menu-item-selected {
                color: ${color_primary};
                background-color: ${color_bg_text_active};
            }

            .ant-menu-item-selected > a,
            .ant-menu-item-selected > a:hover {
                color: ${color_primary};
            }

            .ant-menu-submenu-selected {
                color: ${color_primary};
            }

            .ant-menu-submenu-title {
                position: relative;
                display: flex;
                align-items: center;
                padding: 0 ${padding_lg}px;
                height: ${height_base}px;
                line-height: ${height_base}px;
                cursor: pointer;
                transition: all 0.3s;
                border-radius: ${border_radius}px;
                margin-inline: 4px;
                margin-block: 4px;
                width: calc(100% - 8px);
            }

            .ant-menu-submenu-arrow {
                position: absolute;
                right: ${padding_md}px;
                color: ${color_text};
                transform: rotate(0deg);
                transition: transform 0.3s cubic-bezier(0.645, 0.045, 0.355, 1);
            }

            .ant-menu-submenu-open > .ant-menu-submenu-title > .ant-menu-submenu-arrow {
                transform: rotate(180deg);
            }

            .ant-menu-submenu-disabled {
                color: ${color_text_disabled} !important;
                background: none;
                cursor: not-allowed;
            }

            .ant-menu-submenu-disabled > .ant-menu-submenu-title {
                color: ${color_text_disabled} !important;
                cursor: not-allowed;
            }

            .ant-menu-submenu-disabled > .ant-menu-submenu-title > .ant-menu-submenu-arrow {
                color: ${color_text_disabled} !important;
            }

            .ant-menu-submenu-disabled:hover {
                color: ${color_text_disabled} !important;
                background: none;
            }

            .ant-menu-item-disabled,
            .ant-menu-submenu-disabled {
                color: ${color_text_disabled} !important;
                background: none;
                cursor: not-allowed;
            }

            .ant-menu-item-disabled > a,
            .ant-menu-submenu-disabled > a {
                color: ${color_text_disabled} !important;
                pointer-events: none;
            }

            .ant-menu-item-disabled:hover,
            .ant-menu-submenu-disabled:hover {
                color: ${color_text_disabled} !important;
                background: none;
            }

            /* 水平菜单样式 */
            .ant-menu-horizontal {
                display: flex;
                flex-wrap: nowrap;
                border-bottom: 1px solid ${color_split};
                line-height: ${height_base}px;
            }

            .ant-menu-horizontal > .ant-menu-item,
            .ant-menu-horizontal > .ant-menu-submenu {
                position: relative;
                top: 1px;
                display: inline-flex;
                align-items: center;
                margin-top: 0;
                margin-bottom: 0;
                padding: 0 ${padding_md}px;
                border-bottom: 2px solid transparent;
            }

            .ant-menu-horizontal > .ant-menu-item:hover,
            .ant-menu-horizontal > .ant-menu-submenu:hover,
            .ant-menu-horizontal > .ant-menu-item-active,
            .ant-menu-horizontal > .ant-menu-submenu-active,
            .ant-menu-horizontal > .ant-menu-item-open,
            .ant-menu-horizontal > .ant-menu-submenu-open,
            .ant-menu-horizontal > .ant-menu-item-selected,
            .ant-menu-horizontal > .ant-menu-submenu-selected {
                color: ${color_primary};
                border-bottom: 2px solid ${color_primary};
            }

            /* 垂直菜单样式 */
            .ant-menu-vertical {
                border-right: 1px solid ${color_split};
            }

            .ant-menu-vertical .ant-menu-item {
                margin-bottom: ${margin_xs}px;
            }

            .ant-menu-vertical .ant-menu-submenu-title {
                padding-right: ${padding_xl}px;
            }

            /* 内联菜单样式 */
            .ant-menu-inline {
                width: 100%;
                border-right: 1px solid ${color_split};
            }

            .ant-menu-inline .ant-menu-item,
            .ant-menu-inline .ant-menu-submenu-title {
                width: 100%;
            }

            .ant-menu-inline .ant-menu-submenu-title {
                padding-right: ${padding_xl}px;
            }

            .ant-menu-inline .ant-menu-item {
                margin-bottom: ${margin_xs}px;
            }

            .ant-menu-inline .ant-menu-item:not(:last-child) {
                margin-bottom: ${margin_xs}px;
            }

            .ant-menu-inline .ant-menu-submenu-arrow {
                right: ${padding_md}px;
            }

            .ant-menu-inline-collapsed {
                width: ${height_lg * 2}px;
            }

            .ant-menu-inline-collapsed > .ant-menu-item,
            .ant-menu-inline-collapsed > .ant-menu-submenu > .ant-menu-submenu-title {
                padding: 0 calc(50% - ${font_size}px / 2);
                text-overflow: clip;
            }

            .ant-menu-inline-collapsed .ant-menu-submenu-arrow {
                display: none;
            }

            .ant-menu-inline-collapsed .ant-menu-item-group-title {
                padding-right: 0;
                padding-left: 0;
                text-align: center;
            }

            /* 折叠菜单样式 */
            .ant-menu-collapsed {
                width: ${height_lg * 2}px;
            }

            .ant-menu-collapsed > .ant-menu-item,
            .ant-menu-collapsed > .ant-menu-submenu > .ant-menu-submenu-title {
                padding: 0 calc(50% - ${font_size}px / 2);
                text-overflow: clip;
            }

            .ant-menu-collapsed .ant-menu-submenu-arrow {
                display: none;
            }

            .ant-menu-collapsed .ant-menu-item-group-title {
                padding-right: 0;
                padding-left: 0;
                text-align: center;
            }

            /* 暗色主题样式 */
            .ant-menu-dark {
                color: ${color_text_secondary_dark};
                background: ${color_bg_container_dark};
            }

            .ant-menu-dark .ant-menu-item,
            .ant-menu-dark .ant-menu-submenu-title {
                color: ${color_text_secondary_dark};
            }

            .ant-menu-dark .ant-menu-item:hover,
            .ant-menu-dark .ant-menu-submenu-title:hover {
                color: ${color_text_dark};
                background-color: ${color_bg_text_hover_dark};
            }

            .ant-menu-dark .ant-menu-item-selected {
                color: ${color_text_dark};
                background-color: ${color_primary};
            }

            .ant-menu-dark .ant-menu-item-selected > a,
            .ant-menu-dark .ant-menu-item-selected > a:hover {
                color: ${color_text_dark};
            }

            .ant-menu-dark .ant-menu-submenu-selected {
                color: ${color_text_dark};
            }

            .ant-menu-dark .ant-menu-submenu-arrow {
                color: ${color_text_secondary_dark};
            }

            .ant-menu-dark .ant-menu-item-divider {
                background-color: ${color_split_dark};
            }

            .ant-menu-dark .ant-menu-submenu-disabled,
            .ant-menu-dark .ant-menu-item-disabled {
                color: ${color_text_disabled_dark} !important;
            }

            /* 响应式样式 */
            @media (max-width: 575px) {
                .ant-menu-horizontal {
                    flex-wrap: wrap;
                }
            }

            /* 动画样式 */
            .ant-menu-submenu-popup {
                position: absolute;
                z-index: ${z_index_popup};
                background: transparent;
                box-shadow: none;
                transform-origin: 0 0;
            }

            .ant-menu-submenu-popup .ant-menu {
                background-color: ${bg_color_container};
                border-radius: ${border_radius}px;
                box-shadow: ${box_shadow};
            }

            .ant-menu-submenu-popup.ant-menu-dark .ant-menu {
                background-color: ${color_bg_container_dark};
            }

            .ant-menu-submenu-popup .ant-menu-item:hover {
                background-color: ${color_bg_text_hover};
            }

            .ant-menu-submenu-popup.ant-menu-dark .ant-menu-item:hover {
                background-color: ${color_bg_text_hover_dark};
            }

            .ant-menu-submenu-popup .ant-menu-item-selected {
                background-color: ${color_bg_text_active};
            }

            .ant-menu-submenu-popup.ant-menu-dark .ant-menu-item-selected {
                background-color: ${color_primary};
            }

            .ant-menu-submenu-placement-rightTop {
                transform-origin: left top;
            }

            .ant-menu-submenu-placement-leftTop {
                transform-origin: right top;
            }

            .ant-menu-submenu-placement-bottomLeft {
                transform-origin: top left;
            }

            .ant-menu-submenu-placement-bottomRight {
                transform-origin: top right;
            }

            .ant-menu-submenu-placement-topLeft {
                transform-origin: bottom left;
            }

            .ant-menu-submenu-placement-topRight {
                transform-origin: bottom right;
            }
            "#,
            color_text = self.token.color_text,
            font_size = self.token.font_size,
            line_height = self.token.line_height,
            bg_color_container = self.token.color_bg_container,
            color_text_secondary = self.token.color_text_secondary,
            padding_xs = self.token.padding_xs,
            padding_sm = self.token.padding_sm,
            padding_md = self.token.padding_md,
            padding_lg = self.token.padding_lg,
            padding_xl = self.token.padding_xl,
            height_base = self.token.height_base,
            height_lg = self.token.height_lg,
            border_radius = self.token.border_radius,
            color_bg_text_active = self.token.color_bg_text_active,
            color_text_disabled = self.token.color_text_disabled,
            color_split = self.token.color_split,
            margin_xs = self.token.margin_xs,
            color_primary = self.token.color_primary,
            color_bg_text_hover = self.token.color_bg_text_hover,
            color_text_secondary_dark = "rgba(255, 255, 255, 0.65)",
            color_bg_container_dark = "#001529",
            color_text_dark = "#fff",
            color_bg_text_hover_dark = "rgba(255, 255, 255, 0.08)",
            color_split_dark = "rgba(255, 255, 255, 0.12)",
            color_text_disabled_dark = "rgba(255, 255, 255, 0.25)",
            z_index_popup = self.token.z_index_popup,
            box_shadow = self.token.box_shadow
        ).to_string()
    }
}

/// 生成菜单样式
pub fn generate_menu_style(mode: MenuMode, theme: MenuTheme, collapsed: bool) -> String {
    MenuStyleGenerator::new()
        .with_mode(mode)
        .with_theme(theme)
        .with_collapsed(collapsed)
        .generate()
}

/// 生成菜单 CSS 样式
pub fn generate_menu_css(mode: MenuMode, theme: MenuTheme, collapsed: bool) -> String {
    MenuStyleGenerator::new()
        .with_mode(mode)
        .with_theme(theme)
        .with_collapsed(collapsed)
        .generate_css()
}

/// 默认菜单样式
pub fn default_menu_style() -> String {
    MenuStyleGenerator::new().generate()
}

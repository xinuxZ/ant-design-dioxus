//! Tabs 标签页组件样式模块
//!
//! 本模块包含 Tabs 组件的所有样式定义，从组件逻辑中分离出来，
//! 提高代码的可维护性和复用性。

use crate::theme::AliasToken;
use css_in_rust::css;
use serde::{Deserialize, Serialize};

/// 页签的基本样式
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TabsType {
    /// 线条样式
    Line,
    /// 卡片样式
    Card,
    /// 可编辑卡片样式
    EditableCard,
}

impl Default for TabsType {
    fn default() -> Self {
        Self::Line
    }
}

/// 页签位置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TabsPosition {
    /// 顶部
    Top,
    /// 右侧
    Right,
    /// 底部
    Bottom,
    /// 左侧
    Left,
}

impl Default for TabsPosition {
    fn default() -> Self {
        Self::Top
    }
}

/// 标签页大小
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TabsSize {
    /// 大尺寸
    Large,
    /// 默认尺寸
    Default,
    /// 小尺寸
    Small,
}

impl Default for TabsSize {
    fn default() -> Self {
        Self::Default
    }
}

/// 标签页样式生成器
pub struct TabsStyleGenerator {
    /// 页签基本样式
    pub tab_type: TabsType,
    /// 页签位置
    pub tab_position: TabsPosition,
    /// 标签页大小
    pub size: TabsSize,
    /// 是否为暗色模式
    pub dark: bool,
    /// 主题令牌
    pub token: AliasToken,
}

impl TabsStyleGenerator {
    /// 创建新的标签页样式生成器
    pub fn new() -> Self {
        Self {
            tab_type: TabsType::default(),
            tab_position: TabsPosition::default(),
            size: TabsSize::default(),
            dark: false,
            token: AliasToken::default(),
        }
    }

    /// 设置页签基本样式
    pub fn with_type(mut self, tab_type: TabsType) -> Self {
        self.tab_type = tab_type;
        self
    }

    /// 设置页签位置
    pub fn with_position(mut self, tab_position: TabsPosition) -> Self {
        self.tab_position = tab_position;
        self
    }

    /// 设置标签页大小
    pub fn with_size(mut self, size: TabsSize) -> Self {
        self.size = size;
        self
    }

    /// 设置暗色模式
    pub fn with_dark(mut self, dark: bool) -> Self {
        self.dark = dark;
        self
    }

    /// 设置主题令牌
    pub fn with_token(mut self, token: AliasToken) -> Self {
        self.token = token;
        self
    }

    /// 生成完整的标签页样式类名
    pub fn generate(&self) -> String {
        let mut classes = vec!["ant-tabs"];

        // 添加位置相关的类名
        match self.tab_position {
            TabsPosition::Top => classes.push("ant-tabs-top"),
            TabsPosition::Right => classes.push("ant-tabs-right"),
            TabsPosition::Bottom => classes.push("ant-tabs-bottom"),
            TabsPosition::Left => classes.push("ant-tabs-left"),
        }

        // 添加类型相关的类名
        match self.tab_type {
            TabsType::Line => {}
            TabsType::Card => classes.push("ant-tabs-card"),
            TabsType::EditableCard => {
                classes.push("ant-tabs-card");
                classes.push("ant-tabs-editable-card");
            }
        }

        // 添加尺寸相关的类名
        match self.size {
            TabsSize::Large => classes.push("ant-tabs-large"),
            TabsSize::Default => {}
            TabsSize::Small => classes.push("ant-tabs-small"),
        }

        // 添加暗色模式类名
        if self.dark {
            classes.push("ant-tabs-dark");
        }

        classes.join(" ")
    }

    /// 生成 CSS 样式
    pub fn generate_css(&self) -> String {
        css!(
            r#"
            .ant-tabs {
                box-sizing: border-box;
                margin: 0;
                padding: 0;
                color: ${color_text};
                font-size: ${font_size}px;
                font-variant: tabular-nums;
                line-height: ${line_height};
                list-style: none;
                font-feature-settings: 'tnum';
                position: relative;
                overflow: hidden;
                zoom: 1;
            }

            .ant-tabs::before,
            .ant-tabs::after {
                display: table;
                content: '';
            }

            .ant-tabs::after {
                clear: both;
                content: '';
            }

            .ant-tabs-nav {
                position: relative;
                display: flex;
                flex: none;
                align-items: center;
                margin: 0 0 ${margin_md}px 0;
            }

            .ant-tabs-nav-wrap {
                position: relative;
                display: inline-block;
                flex: auto;
                align-self: stretch;
                overflow: hidden;
                white-space: nowrap;
                transform: translate(0);
            }

            .ant-tabs-nav-list {
                position: relative;
                display: flex;
                transition: transform 0.3s;
            }

            .ant-tabs-tab {
                position: relative;
                display: inline-flex;
                align-items: center;
                padding: ${padding_xs}px 0;
                font-size: ${font_size}px;
                background: transparent;
                border: 0;
                outline: none;
                cursor: pointer;
            }

            .ant-tabs-tab-btn {
                outline: none;
                transition: all 0.3s;
            }

            .ant-tabs-tab-btn:focus,
            .ant-tabs-tab-btn:active {
                color: ${color_primary};
            }

            .ant-tabs-tab-btn:hover {
                color: ${color_primary_hover};
            }

            .ant-tabs-tab-remove {
                margin-left: ${margin_xs}px;
                color: ${color_text_secondary};
                font-size: ${font_size}px;
                background: transparent;
                border: none;
                outline: none;
                cursor: pointer;
                transition: all 0.3s;
            }

            .ant-tabs-tab-remove:hover {
                color: ${color_text};
            }

            .ant-tabs-tab-active {
                color: ${color_primary};
                font-weight: 500;
            }

            .ant-tabs-ink-bar {
                position: absolute;
                background: ${color_primary};
                pointer-events: none;
            }

            .ant-tabs-top > .ant-tabs-nav,
            .ant-tabs-bottom > .ant-tabs-nav {
                flex-direction: row;
            }

            .ant-tabs-top > .ant-tabs-nav .ant-tabs-ink-bar,
            .ant-tabs-bottom > .ant-tabs-nav .ant-tabs-ink-bar {
                height: 2px;
            }

            .ant-tabs-top > .ant-tabs-nav .ant-tabs-ink-bar {
                bottom: 0;
            }

            .ant-tabs-bottom > .ant-tabs-nav .ant-tabs-ink-bar {
                top: 0;
            }

            .ant-tabs-left > .ant-tabs-nav,
            .ant-tabs-right > .ant-tabs-nav {
                flex-direction: column;
                min-width: 50px;
            }

            .ant-tabs-left > .ant-tabs-nav .ant-tabs-tab,
            .ant-tabs-right > .ant-tabs-nav .ant-tabs-tab {
                padding: ${padding_sm}px ${padding_md}px;
                text-align: center;
            }

            .ant-tabs-left > .ant-tabs-nav .ant-tabs-tab + .ant-tabs-tab,
            .ant-tabs-right > .ant-tabs-nav .ant-tabs-tab + .ant-tabs-tab {
                margin: ${margin_sm}px 0 0 0;
            }

            .ant-tabs-left > .ant-tabs-nav .ant-tabs-ink-bar,
            .ant-tabs-right > .ant-tabs-nav .ant-tabs-ink-bar {
                width: 2px;
            }

            .ant-tabs-left > .ant-tabs-nav .ant-tabs-ink-bar {
                right: 0;
            }

            .ant-tabs-right > .ant-tabs-nav .ant-tabs-ink-bar {
                left: 0;
            }

            .ant-tabs-content {
                display: flex;
                width: 100%;
            }

            .ant-tabs-content-holder {
                flex: auto;
                min-width: 0;
                min-height: 0;
            }

            .ant-tabs-content-animated {
                transition: margin 0.3s;
            }

            .ant-tabs-tabpane {
                flex: none;
                width: 100%;
                outline: none;
            }

            /* Card 样式 */
            .ant-tabs-card > .ant-tabs-nav .ant-tabs-tab {
                margin: 0;
                padding: ${padding_xs}px ${padding_md}px;
                background: ${color_bg_container_disabled};
                border: 1px solid ${color_border};
                transition: all 0.3s cubic-bezier(0.645, 0.045, 0.355, 1);
            }

            .ant-tabs-card > .ant-tabs-nav .ant-tabs-tab-active {
                color: ${color_primary};
                background: ${color_bg_container};
            }

            .ant-tabs-card > .ant-tabs-nav .ant-tabs-tab-disabled {
                color: ${color_text_disabled};
                cursor: not-allowed;
            }

            .ant-tabs-card > .ant-tabs-nav .ant-tabs-nav-wrap {
                margin-bottom: 0;
            }

            .ant-tabs-top.ant-tabs-card > .ant-tabs-nav .ant-tabs-tab + .ant-tabs-tab {
                margin-left: 2px;
            }

            .ant-tabs-top.ant-tabs-card > .ant-tabs-nav .ant-tabs-tab {
                border-radius: ${border_radius}px ${border_radius}px 0 0;
            }

            .ant-tabs-top.ant-tabs-card > .ant-tabs-nav .ant-tabs-tab-active {
                border-bottom-color: ${color_bg_container};
            }

            .ant-tabs-bottom.ant-tabs-card > .ant-tabs-nav .ant-tabs-tab + .ant-tabs-tab {
                margin-left: 2px;
            }

            .ant-tabs-bottom.ant-tabs-card > .ant-tabs-nav .ant-tabs-tab {
                border-radius: 0 0 ${border_radius}px ${border_radius}px;
            }

            .ant-tabs-bottom.ant-tabs-card > .ant-tabs-nav .ant-tabs-tab-active {
                border-top-color: ${color_bg_container};
            }

            .ant-tabs-left.ant-tabs-card > .ant-tabs-nav .ant-tabs-tab + .ant-tabs-tab {
                margin-top: 2px;
            }

            .ant-tabs-left.ant-tabs-card > .ant-tabs-nav .ant-tabs-tab {
                border-radius: ${border_radius}px 0 0 ${border_radius}px;
            }

            .ant-tabs-left.ant-tabs-card > .ant-tabs-nav .ant-tabs-tab-active {
                border-right-color: ${color_bg_container};
            }

            .ant-tabs-right.ant-tabs-card > .ant-tabs-nav .ant-tabs-tab + .ant-tabs-tab {
                margin-top: 2px;
            }

            .ant-tabs-right.ant-tabs-card > .ant-tabs-nav .ant-tabs-tab {
                border-radius: 0 ${border_radius}px ${border_radius}px 0;
            }

            .ant-tabs-right.ant-tabs-card > .ant-tabs-nav .ant-tabs-tab-active {
                border-left-color: ${color_bg_container};
            }

            /* 可编辑卡片样式 */
            .ant-tabs-editable-card > .ant-tabs-nav .ant-tabs-tab {
                padding: ${padding_xs}px ${padding_xs}px ${padding_xs}px ${padding_md}px;
            }

            .ant-tabs-editable-card > .ant-tabs-nav .ant-tabs-nav-add {
                padding: 0 ${padding_xs}px;
                background: ${color_bg_container_disabled};
                border: 1px solid ${color_border};
                border-radius: ${border_radius}px;
                outline: none;
                cursor: pointer;
                color: ${color_text};
                transition: all 0.3s;
            }

            .ant-tabs-editable-card > .ant-tabs-nav .ant-tabs-nav-add:hover {
                color: ${color_primary_hover};
            }

            .ant-tabs-editable-card > .ant-tabs-nav .ant-tabs-nav-add:active {
                color: ${color_primary_active};
            }

            /* 尺寸样式 */
            .ant-tabs-large > .ant-tabs-nav .ant-tabs-tab {
                padding: ${padding_md}px 0;
                font-size: ${font_size_lg}px;
            }

            .ant-tabs-small > .ant-tabs-nav .ant-tabs-tab {
                padding: ${padding_xs / 2}px 0;
                font-size: ${font_size_sm}px;
            }

            .ant-tabs-small > .ant-tabs-nav .ant-tabs-nav-more {
                font-size: ${font_size_sm}px;
            }

            .ant-tabs-card.ant-tabs-large > .ant-tabs-nav .ant-tabs-tab {
                padding: ${padding_sm}px ${padding_lg}px;
            }

            .ant-tabs-card.ant-tabs-small > .ant-tabs-nav .ant-tabs-tab {
                padding: ${padding_xs / 2}px ${padding_sm}px;
            }

            /* 响应式样式 */
            @media (max-width: 575px) {
                .ant-tabs-nav .ant-tabs-tab {
                    padding: ${padding_xs}px ${padding_xs}px;
                    margin: 0 ${margin_xs}px 0 0;
                }
                .ant-tabs-nav .ant-tabs-tab:last-child {
                    margin-right: 0;
                }
                .ant-tabs-left > .ant-tabs-nav,
                .ant-tabs-right > .ant-tabs-nav {
                    width: 100%;
                }
                .ant-tabs-left > .ant-tabs-content-holder,
                .ant-tabs-right > .ant-tabs-content-holder {
                    margin-top: ${margin_sm}px;
                }
            }

            /* RTL 样式 */
            .ant-tabs-rtl {
                direction: rtl;
            }

            .ant-tabs-rtl .ant-tabs-nav .ant-tabs-tab {
                margin: 0 0 0 ${margin_md}px;
            }

            .ant-tabs-rtl .ant-tabs-nav .ant-tabs-tab:last-child {
                margin-left: 0;
            }

            .ant-tabs-rtl .ant-tabs-nav .ant-tabs-tab .ant-tabs-tab-remove {
                margin-right: ${margin_xs}px;
                margin-left: 0;
            }

            .ant-tabs-rtl.ant-tabs-left > .ant-tabs-nav {
                order: 1;
            }

            .ant-tabs-rtl.ant-tabs-right > .ant-tabs-nav {
                order: 0;
            }

            .ant-tabs-rtl.ant-tabs-card.ant-tabs-top > .ant-tabs-nav .ant-tabs-tab + .ant-tabs-tab,
            .ant-tabs-rtl.ant-tabs-card.ant-tabs-bottom > .ant-tabs-nav .ant-tabs-tab + .ant-tabs-tab {
                margin-right: 2px;
                margin-left: 0;
            }

            /* 暗色模式 */
            .ant-tabs-dark > .ant-tabs-nav {
                background-color: ${color_bg_container_dark};
            }

            .ant-tabs-dark > .ant-tabs-nav .ant-tabs-tab {
                color: ${color_text_secondary_dark};
            }

            .ant-tabs-dark > .ant-tabs-nav .ant-tabs-tab:hover {
                color: ${color_text_dark};
            }

            .ant-tabs-dark > .ant-tabs-nav .ant-tabs-tab-active {
                color: ${color_primary};
            }

            .ant-tabs-dark > .ant-tabs-nav .ant-tabs-ink-bar {
                background: ${color_primary};
            }

            .ant-tabs-dark.ant-tabs-card > .ant-tabs-nav .ant-tabs-tab {
                background: ${color_bg_container_dark};
                border-color: ${color_split_dark};
            }

            .ant-tabs-dark.ant-tabs-card > .ant-tabs-nav .ant-tabs-tab-active {
                background: ${color_bg_text_active_dark};
                border-color: ${color_split_dark};
            }

            @media (prefers-color-scheme: dark) {
                .ant-tabs {
                    color: ${color_text_dark};
                }
                .ant-tabs-tab {
                    color: ${color_text_secondary_dark};
                }
                .ant-tabs-tab:hover {
                    color: ${color_text_dark};
                }
                .ant-tabs-tab-active {
                    color: ${color_primary};
                }
                .ant-tabs-card > .ant-tabs-nav .ant-tabs-tab {
                    background: ${color_bg_container_dark};
                    border-color: ${color_split_dark};
                }
                .ant-tabs-card > .ant-tabs-nav .ant-tabs-tab-active {
                    background: ${color_bg_text_active_dark};
                    border-color: ${color_split_dark};
                }
            }

            @media (prefers-contrast: high) {
                .ant-tabs-tab {
                    background: transparent;
                }
                .ant-tabs-tab-active {
                    outline: 2px solid;
                }
                .ant-tabs-ink-bar {
                    background: currentColor;
                    height: 3px;
                }
            }
            "#,
            color_text = self.token.color_text,
            font_size = self.token.font_size,
            line_height = self.token.line_height,
            margin_md = self.token.margin_md,
            padding_xs = self.token.padding_xs,
            color_primary = self.token.color_primary,
            color_primary_hover = self.token.color_primary_hover,
            margin_xs = self.token.margin_xs,
            color_text_secondary = self.token.color_text_secondary,
            padding_sm = self.token.padding_sm,
            padding_md = self.token.padding_md,
            margin_sm = self.token.margin_sm,
            color_bg_container_disabled = self.token.color_bg_container_disabled,
            color_border = self.token.color_border,
            color_bg_container = self.token.color_bg_container,
            color_text_disabled = self.token.color_text_disabled,
            border_radius = self.token.border_radius,
            color_primary_active = self.token.color_primary_active,
            font_size_lg = self.token.font_size_lg,
            font_size_sm = self.token.font_size_sm,
            padding_lg = self.token.padding_lg,
            color_bg_container_dark = "#141414",
            color_text_secondary_dark = "rgba(255, 255, 255, 0.65)",
            color_text_dark = "rgba(255, 255, 255, 0.85)",
            color_split_dark = "#303030",
            color_bg_text_active_dark = "#111b26"
        ).to_string()
    }
}

/// 生成 Tabs 样式
pub fn generate_tabs_style(
    tab_type: TabsType,
    tab_position: TabsPosition,
    size: TabsSize,
    dark: bool,
) -> String {
    TabsStyleGenerator::new()
        .with_type(tab_type)
        .with_position(tab_position)
        .with_size(size)
        .with_dark(dark)
        .generate()
}

/// 生成 Tabs CSS 样式
pub fn generate_tabs_css(
    tab_type: TabsType,
    tab_position: TabsPosition,
    size: TabsSize,
    dark: bool,
) -> String {
    TabsStyleGenerator::new()
        .with_type(tab_type)
        .with_position(tab_position)
        .with_size(size)
        .with_dark(dark)
        .generate_css()
}

/// 默认 Tabs 样式
pub fn default_tabs_style() -> String {
    TabsStyleGenerator::new().generate()
}

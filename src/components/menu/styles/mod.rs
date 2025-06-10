//! Menu 导航菜单组件样式模块
//!
//! 本模块包含 Menu 组件的所有样式定义，从组件逻辑中分离出来，
//! 提高代码的可维护性和复用性。

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
}

impl MenuStyleGenerator {
    /// 创建新的菜单样式生成器
    pub fn new() -> Self {
        Self {
            mode: MenuMode::default(),
            theme: MenuTheme::default(),
            collapsed: false,
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

    /// 生成完整的菜单样式
    pub fn generate(&self) -> String {
        let mut classes = vec![self.base_style()];

        // 根据模式添加相应样式
        match self.mode {
            MenuMode::Horizontal => classes.push(self.horizontal_style()),
            MenuMode::Vertical => classes.push(self.vertical_style()),
            MenuMode::Inline => classes.push(self.inline_style()),
        }

        // 根据主题添加相应样式
        match self.theme {
            MenuTheme::Light => {}
            MenuTheme::Dark => classes.push(self.dark_theme_style()),
        }

        // 折叠状态
        if self.collapsed {
            classes.push(self.collapsed_style());
        }

        classes.join(" ")
    }

    /// 基础样式
    fn base_style(&self) -> String {
        css!(
            r#"
            margin: 0;
            padding: 0;
            color: rgba(0, 0, 0, 0.88);
            font-size: 14px;
            line-height: 1.5715;
            list-style: none;
            font-feature-settings: 'tnum';
            background: #fff;
            border: 0;
            border-radius: 0;
            outline: none;
            box-shadow: none;
            transition: background 0.3s, width 0.3s cubic-bezier(0.2, 0, 0, 1) 0s;
            box-sizing: border-box;

            &::before {
                display: table;
                content: '';
            }

            &::after {
                display: table;
                clear: both;
                content: '';
            }

            ul, ol {
                margin: 0;
                padding: 0;
                list-style: none;
            }

            &-hidden {
                display: none;
            }

            &-item-group-title {
                padding: 8px 16px;
                color: rgba(0, 0, 0, 0.45);
                font-size: 14px;
                line-height: 1.5715;
                transition: all 0.3s;
            }

            &-submenu,
            &-item {
                position: relative;
                display: block;
                margin: 0;
                padding: 0 20px;
                color: rgba(0, 0, 0, 0.88);
                font-weight: 400;
                font-size: 14px;
                line-height: 40px;
                cursor: pointer;
                transition: border-color 0.3s, background 0.3s, padding 0.15s cubic-bezier(0.645, 0.045, 0.355, 1);
                border-radius: 6px;
                margin-inline: 4px;
                margin-block: 4px;
                width: calc(100% - 8px);
            }

            &-item:active,
            &-submenu-title:active {
                background: rgba(0, 0, 0, 0.15);
            }

            &-submenu .ant-menu-sub {
                cursor: initial;
                transition: background 0.3s cubic-bezier(0.645, 0.045, 0.355, 1), padding 0.3s cubic-bezier(0.645, 0.045, 0.355, 1);
            }

            &-item > a {
                display: block;
                color: rgba(0, 0, 0, 0.88);
                text-decoration: none;
            }

            &-item > a:hover {
                color: rgba(0, 0, 0, 0.88);
            }

            &-item > a::before {
                position: absolute;
                top: 0;
                right: 0;
                bottom: 0;
                left: 0;
                background-color: transparent;
                content: '';
            }

            &-item-divider {
                overflow: hidden;
                line-height: 0;
                border-color: rgba(5, 5, 5, 0.06);
                border-style: solid;
                border-width: 1px 0 0 0;
            }

            &-item:hover,
            &-item-active,
            &:not(.ant-menu-inline) .ant-menu-submenu-open,
            &-submenu-active,
            &-submenu-title:hover {
                color: #1677ff;
                background-color: rgba(0, 0, 0, 0.06);
            }

            &-item-selected {
                color: #1677ff;
                background-color: #e6f4ff;
            }

            &-item-selected > a,
            &-item-selected > a:hover {
                color: #1677ff;
            }

            &:not(.ant-menu-horizontal) .ant-menu-item-selected {
                background-color: #e6f4ff;
            }

            &-inline .ant-menu-item-selected {
                background-color: #e6f4ff;
            }

            &-item-disabled,
            &-submenu-disabled {
                color: rgba(0, 0, 0, 0.25) !important;
                background: none;
                border-color: transparent !important;
                cursor: not-allowed;
            }

            &-item-disabled > a,
            &-submenu-disabled > a {
                color: rgba(0, 0, 0, 0.25) !important;
                pointer-events: none;
            }

            &-item-disabled > .ant-menu-submenu-title,
            &-submenu-disabled > .ant-menu-submenu-title {
                color: rgba(0, 0, 0, 0.25) !important;
                cursor: not-allowed;
            }

            &-item-disabled > .ant-menu-submenu-title > .ant-menu-submenu-arrow::before,
            &-item-disabled > .ant-menu-submenu-title > .ant-menu-submenu-arrow::after,
            &-submenu-disabled > .ant-menu-submenu-title > .ant-menu-submenu-arrow::before,
            &-submenu-disabled > .ant-menu-submenu-title > .ant-menu-submenu-arrow::after {
                background: rgba(0, 0, 0, 0.25) !important;
            }

            &-item-icon,
            &-submenu-title .ant-menu-item-icon {
                min-width: 14px;
                margin-right: 10px;
                font-size: 14px;
                transition: font-size 0.15s cubic-bezier(0.215, 0.61, 0.355, 1), margin 0.3s cubic-bezier(0.645, 0.045, 0.355, 1);
            }

            &-item-icon + span,
            &-submenu-title .ant-menu-item-icon + span {
                opacity: 1;
                transition: opacity 0.3s cubic-bezier(0.645, 0.045, 0.355, 1), width 0.3s cubic-bezier(0.645, 0.045, 0.355, 1);
            }

            &-item .ant-menu-item-icon,
            &-submenu .ant-menu-item-icon {
                line-height: 40px;
            }

            &-submenu-title {
                position: relative;
                display: block;
                margin: 0;
                padding: 0 20px;
                color: rgba(0, 0, 0, 0.88);
                font-weight: 400;
                font-size: 14px;
                line-height: 40px;
                cursor: pointer;
                transition: border-color 0.3s, background 0.3s, padding 0.15s cubic-bezier(0.645, 0.045, 0.355, 1);
                border-radius: 6px;
                margin-inline: 4px;
                margin-block: 4px;
                width: calc(100% - 8px);
            }

            &-submenu-title:hover {
                color: #1677ff;
                background-color: rgba(0, 0, 0, 0.06);
            }

            &-submenu-arrow {
                position: absolute;
                top: 50%;
                right: 16px;
                width: 10px;
                transition: transform 0.3s cubic-bezier(0.645, 0.045, 0.355, 1);
            }

            &-submenu-arrow::before,
            &-submenu-arrow::after {
                position: absolute;
                width: 6px;
                height: 1.5px;
                background-color: currentColor;
                border-radius: 2px;
                transition: background 0.3s cubic-bezier(0.645, 0.045, 0.355, 1), transform 0.3s cubic-bezier(0.645, 0.045, 0.355, 1), top 0.3s cubic-bezier(0.645, 0.045, 0.355, 1), color 0.3s cubic-bezier(0.645, 0.045, 0.355, 1);
                content: '';
            }

            &-submenu-arrow::before {
                transform: rotate(45deg) translateY(-2px);
            }

            &-submenu-arrow::after {
                transform: rotate(-45deg) translateY(2px);
            }

            &-submenu-open > .ant-menu-submenu-title > .ant-menu-submenu-arrow {
                transform: translateY(-2px);
            }

            &-submenu-open > .ant-menu-submenu-title > .ant-menu-submenu-arrow::before {
                transform: rotate(45deg) translateX(2px);
            }

            &-submenu-open > .ant-menu-submenu-title > .ant-menu-submenu-arrow::after {
                transform: rotate(-45deg) translateX(-2px);
            }

            &-sub.ant-menu-inline {
                padding: 0;
                background: #fafafa;
                border: 0;
                border-radius: 0;
                box-shadow: none;
            }

            &-sub.ant-menu-inline > .ant-menu-item,
            &-sub.ant-menu-inline > .ant-menu-submenu > .ant-menu-submenu-title {
                height: 40px;
                line-height: 40px;
                list-style-position: inside;
                list-style-type: disc;
            }

            &-sub.ant-menu-inline .ant-menu-item-group-title {
                padding-left: 32px;
            }
            "#
        )
    }

    /// 水平菜单样式
    fn horizontal_style(&self) -> String {
        css!(
            r#"
            &.ant-menu-horizontal {
                line-height: 46px;
                border-bottom: 1px solid #f0f0f0;
            }

            &.ant-menu-horizontal > .ant-menu-item,
            &.ant-menu-horizontal > .ant-menu-submenu {
                position: relative;
                top: 1px;
                display: inline-block;
                vertical-align: bottom;
                border-bottom: 2px solid transparent;
            }

            &.ant-menu-horizontal > .ant-menu-item:hover,
            &.ant-menu-horizontal > .ant-menu-submenu:hover,
            &.ant-menu-horizontal > .ant-menu-item-active,
            &.ant-menu-horizontal > .ant-menu-submenu-active,
            &.ant-menu-horizontal > .ant-menu-item-open,
            &.ant-menu-horizontal > .ant-menu-submenu-open,
            &.ant-menu-horizontal > .ant-menu-item-selected,
            &.ant-menu-horizontal > .ant-menu-submenu-selected {
                color: #1677ff;
                border-bottom: 2px solid #1677ff;
            }

            &.ant-menu-horizontal > .ant-menu-item > a {
                display: block;
                color: rgba(0, 0, 0, 0.88);
            }

            &.ant-menu-horizontal > .ant-menu-item > a:hover {
                color: #1677ff;
            }

            &.ant-menu-horizontal > .ant-menu-item > a::before {
                bottom: -2px;
            }

            &.ant-menu-horizontal > .ant-menu-submenu > .ant-menu-submenu-title {
                padding: 0;
                border-radius: 0;
                margin-inline: 0;
                margin-block: 0;
                width: auto;
            }

            &.ant-menu-horizontal.ant-menu-sub {
                min-width: 114px;
                border-radius: 4px;
            }
            "#
        )
    }

    /// 垂直菜单样式
    fn vertical_style(&self) -> String {
        css!(
            r#"
            &.ant-menu-vertical,
            &.ant-menu-vertical-left,
            &.ant-menu-vertical-right {
                border-right: 1px solid #f0f0f0;
            }

            &.ant-menu-vertical-right {
                border-right: none;
                border-left: 1px solid #f0f0f0;
            }

            &.ant-menu-vertical.ant-menu-sub,
            &.ant-menu-vertical-left.ant-menu-sub,
            &.ant-menu-vertical-right.ant-menu-sub {
                min-width: 160px;
                max-height: calc(100vh - 100px);
                padding: 0;
                overflow: hidden;
                border-right: 0;
                transform-origin: 0 0;
            }

            &.ant-menu-vertical.ant-menu-sub .ant-menu-item,
            &.ant-menu-vertical-left.ant-menu-sub .ant-menu-item,
            &.ant-menu-vertical-right.ant-menu-sub .ant-menu-item {
                left: 0;
                margin-left: 0;
                border-right: 0;
            }

            &.ant-menu-vertical.ant-menu-sub .ant-menu-item:after,
            &.ant-menu-vertical-left.ant-menu-sub .ant-menu-item:after,
            &.ant-menu-vertical-right.ant-menu-sub .ant-menu-item:after {
                border-right: 0;
            }
            "#
        )
    }

    /// 内嵌菜单样式
    fn inline_style(&self) -> String {
        css!(
            r#"
            &.ant-menu-inline {
                width: 100%;
            }

            &.ant-menu-inline .ant-menu-item,
            &.ant-menu-inline .ant-menu-submenu-title {
                width: 100%;
            }
            "#
        )
    }

    /// 折叠菜单样式
    fn collapsed_style(&self) -> String {
        css!(
            r#"
            &.ant-menu-inline-collapsed {
                width: 80px;
            }

            &.ant-menu-inline-collapsed > .ant-menu-item,
            &.ant-menu-inline-collapsed > .ant-menu-submenu > .ant-menu-submenu-title {
                left: 0;
                padding: 0 32px !important;
                text-overflow: clip;
            }

            &.ant-menu-inline-collapsed > .ant-menu-item .ant-menu-item-icon,
            &.ant-menu-inline-collapsed > .ant-menu-submenu > .ant-menu-submenu-title .ant-menu-item-icon {
                margin: 0;
                font-size: 16px;
                line-height: 40px;
            }

            &.ant-menu-inline-collapsed > .ant-menu-item .ant-menu-item-icon + span,
            &.ant-menu-inline-collapsed > .ant-menu-submenu > .ant-menu-submenu-title .ant-menu-item-icon + span {
                display: inline-block;
                max-width: 0;
                opacity: 0;
            }

            &.ant-menu-inline-collapsed > .ant-menu-item .ant-menu-submenu-arrow,
            &.ant-menu-inline-collapsed > .ant-menu-submenu > .ant-menu-submenu-title .ant-menu-submenu-arrow {
                display: none;
            }
            "#
        )
    }

    /// 暗色主题样式
    fn dark_theme_style(&self) -> String {
        css!(
            r#"
            &.ant-menu-dark,
            &.ant-menu-dark .ant-menu-sub {
                color: rgba(255, 255, 255, 0.65);
                background: #001529;
            }

            &.ant-menu-dark .ant-menu-submenu-title,
            &.ant-menu-dark .ant-menu-item {
                color: rgba(255, 255, 255, 0.65);
                transition: color 0.3s;
            }

            &.ant-menu-dark.ant-menu-submenu-popup {
                background: transparent;
            }

            &.ant-menu-dark .ant-menu-item > a {
                color: rgba(255, 255, 255, 0.65);
            }

            &.ant-menu-dark .ant-menu-item > a:hover {
                color: #fff;
            }

            &.ant-menu-dark .ant-menu-item:hover,
            &.ant-menu-dark .ant-menu-submenu-active,
            &.ant-menu-dark .ant-menu-submenu-open,
            &.ant-menu-dark .ant-menu-submenu-selected,
            &.ant-menu-dark .ant-menu-submenu-title:hover {
                color: #fff;
                background-color: transparent;
            }

            &.ant-menu-dark .ant-menu-item:hover > a,
            &.ant-menu-dark .ant-menu-submenu-active > a,
            &.ant-menu-dark .ant-menu-submenu-open > a,
            &.ant-menu-dark .ant-menu-submenu-selected > a,
            &.ant-menu-dark .ant-menu-submenu-title:hover > a {
                color: #fff;
            }

            &.ant-menu-dark .ant-menu-item-selected {
                color: #fff;
                background-color: #1677ff;
            }

            &.ant-menu-dark .ant-menu-item-selected > a,
            &.ant-menu-dark .ant-menu-item-selected > a:hover {
                color: #fff;
            }

            &.ant-menu-dark .ant-menu-item-disabled,
            &.ant-menu-dark .ant-menu-submenu-disabled {
                color: rgba(255, 255, 255, 0.35) !important;
            }

            &.ant-menu-dark .ant-menu-item-disabled > a,
            &.ant-menu-dark .ant-menu-submenu-disabled > a {
                color: rgba(255, 255, 255, 0.35) !important;
            }

            &.ant-menu-dark .ant-menu-item-disabled > .ant-menu-submenu-title,
            &.ant-menu-dark .ant-menu-submenu-disabled > .ant-menu-submenu-title {
                color: rgba(255, 255, 255, 0.35) !important;
            }

            &.ant-menu-dark .ant-menu-item-disabled > .ant-menu-submenu-title > .ant-menu-submenu-arrow::before,
            &.ant-menu-dark .ant-menu-item-disabled > .ant-menu-submenu-title > .ant-menu-submenu-arrow::after,
            &.ant-menu-dark .ant-menu-submenu-disabled > .ant-menu-submenu-title > .ant-menu-submenu-arrow::before,
            &.ant-menu-dark .ant-menu-submenu-disabled > .ant-menu-submenu-title > .ant-menu-submenu-arrow::after {
                background: rgba(255, 255, 255, 0.35) !important;
            }

            &.ant-menu-dark.ant-menu-horizontal {
                border-bottom: 0;
            }

            &.ant-menu-dark.ant-menu-horizontal > .ant-menu-item,
            &.ant-menu-dark.ant-menu-horizontal > .ant-menu-submenu {
                top: 0;
                margin-top: 0;
                border-color: #001529;
                border-bottom: 0;
            }

            &.ant-menu-dark.ant-menu-horizontal > .ant-menu-item:hover,
            &.ant-menu-dark.ant-menu-horizontal > .ant-menu-submenu:hover,
            &.ant-menu-dark.ant-menu-horizontal > .ant-menu-item-active,
            &.ant-menu-dark.ant-menu-horizontal > .ant-menu-submenu-active,
            &.ant-menu-dark.ant-menu-horizontal > .ant-menu-item-open,
            &.ant-menu-dark.ant-menu-horizontal > .ant-menu-submenu-open,
            &.ant-menu-dark.ant-menu-horizontal > .ant-menu-item-selected,
            &.ant-menu-dark.ant-menu-horizontal > .ant-menu-submenu-selected {
                color: #fff;
                border-color: #001529;
            }

            &.ant-menu-dark .ant-menu-item-group-title {
                color: rgba(255, 255, 255, 0.65);
            }

            &.ant-menu-dark.ant-menu-inline .ant-menu-sub {
                background: #000c17;
            }
            "#
        )
    }
}

/// 响应式菜单样式
pub fn menu_responsive_style() -> String {
    css!(
        r#"
        @media (max-width: 576px) {
            .ant-menu-horizontal {
                overflow-x: auto;
                white-space: nowrap;
            }

            .ant-menu-horizontal > .ant-menu-item,
            .ant-menu-horizontal > .ant-menu-submenu {
                display: inline-flex;
            }

            .ant-menu-item,
            .ant-menu-submenu-title {
                padding-left: 16px;
                padding-right: 16px;
            }
        }

        .ant-menu-sub.ant-menu-inline {
            animation: antSlideUpIn 0.3s cubic-bezier(0.645, 0.045, 0.355, 1);
        }
        "#
    )
}

/// 动画样式
pub fn menu_animation_style() -> String {
    css!(
        r#"
        @keyframes antSlideUpIn {
            0% {
                transform: scaleY(0.8);
                transform-origin: 0% 0%;
                opacity: 0;
            }
            100% {
                transform: scaleY(1);
                transform-origin: 0% 0%;
                opacity: 1;
            }
        }

        @keyframes antSlideUpOut {
            0% {
                transform: scaleY(1);
                transform-origin: 0% 0%;
                opacity: 1;
            }
            100% {
                transform: scaleY(0.8);
                transform-origin: 0% 0%;
                opacity: 0;
            }
        }
        "#
    )
}

/// 可访问性样式
pub fn menu_a11y_style() -> String {
    css!(
        r#"
        .ant-menu-item:focus,
        .ant-menu-submenu-title:focus {
            outline: 2px solid #1677ff;
            outline-offset: -2px;
        }

        .ant-menu-item:focus:not(:focus-visible),
        .ant-menu-submenu-title:focus:not(:focus-visible) {
            outline: none;
        }
        "#
    )
}

/// 高对比度样式
pub fn menu_high_contrast_style() -> String {
    css!(
        r#"
        @media (prefers-contrast: high) {
            .ant-menu {
                border: 1px solid #000;
            }

            .ant-menu-item:hover,
            .ant-menu-submenu-title:hover {
                background: #000;
                color: #fff;
            }

            .ant-menu-item-selected {
                background: #000;
                color: #fff;
            }
        }
        "#
    )
}

/// 生成完整的菜单样式
pub fn generate_menu_style(mode: MenuMode, theme: MenuTheme, collapsed: bool) -> String {
    format!(
        "{} {} {} {} {}",
        MenuStyleGenerator::new()
            .with_mode(mode)
            .with_theme(theme)
            .with_collapsed(collapsed)
            .generate(),
        menu_responsive_style(),
        menu_animation_style(),
        menu_a11y_style(),
        menu_high_contrast_style(),
    )
}

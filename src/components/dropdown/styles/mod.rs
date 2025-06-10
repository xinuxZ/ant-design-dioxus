//! Dropdown 下拉菜单组件样式模块
//!
//! 本模块包含 Dropdown 组件的所有样式定义，从组件逻辑中分离出来，
//! 提高代码的可维护性和复用性。

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

    /// 生成完整的下拉菜单样式
    pub fn generate(&self) -> String {
        let mut classes = vec![
            self.base_style(),
            self.placement_style(),
            self.arrow_style(),
        ];

        match self.theme {
            DropdownTheme::Light => {}
            DropdownTheme::Dark => classes.push(self.dark_theme_style()),
        }

        match self.size {
            DropdownSize::Large => classes.push(self.large_size_style()),
            DropdownSize::Middle => {}
            DropdownSize::Small => classes.push(self.small_size_style()),
            DropdownSize::Compact => classes.push(self.compact_size_style()),
        }

        if self.disabled {
            classes.push(self.disabled_style());
        }

        classes.join(" ")
    }

    /// 基础样式
    fn base_style(&self) -> String {
        css!(
            r#"
            position: relative;
            display: inline-block;
            font-size: 14px;
            font-variant: tabular-nums;
            line-height: 1.5715;
            color: rgba(0, 0, 0, 0.85);
            font-feature-settings: 'tnum';

            .ant-dropdown-trigger {
                cursor: pointer;
                user-select: none;
            }
            "#
        )
    }

    /// 位置样式
    fn placement_style(&self) -> String {
        match self.placement {
            DropdownPlacement::BottomLeft => css!(
                r#"
                .ant-dropdown-wrap {
                    top: 100%;
                    left: 0;
                }
                "#
            ),
            DropdownPlacement::Bottom => css!(
                r#"
                .ant-dropdown-wrap {
                    top: 100%;
                    left: 50%;
                    transform: translateX(-50%);
                }
                "#
            ),
            DropdownPlacement::BottomRight => css!(
                r#"
                .ant-dropdown-wrap {
                    top: 100%;
                    right: 0;
                }
                "#
            ),
            DropdownPlacement::TopLeft => css!(
                r#"
                .ant-dropdown-wrap {
                    bottom: 100%;
                    left: 0;
                    top: auto;
                }

                .ant-dropdown-arrow {
                    top: auto;
                    bottom: -4px;
                    transform: rotate(-135deg);
                }
                "#
            ),
            DropdownPlacement::Top => css!(
                r#"
                .ant-dropdown-wrap {
                    bottom: 100%;
                    left: 50%;
                    top: auto;
                    transform: translateX(-50%);
                }

                .ant-dropdown-arrow {
                    top: auto;
                    bottom: -4px;
                    transform: rotate(-135deg);
                }
                "#
            ),
            DropdownPlacement::TopRight => css!(
                r#"
                .ant-dropdown-wrap {
                    bottom: 100%;
                    right: 0;
                    top: auto;
                }

                .ant-dropdown-arrow {
                    top: auto;
                    bottom: -4px;
                    transform: rotate(-135deg);
                }
                "#
            ),
        }
    }

    /// 箭头样式
    fn arrow_style(&self) -> String {
        if self.arrow {
            css!(
                r#"
                .ant-dropdown-arrow {
                    position: absolute;
                    top: -4px;
                    left: 16px;
                    width: 8px;
                    height: 8px;
                    background: #fff;
                    border: 1px solid rgba(0, 0, 0, 0.06);
                    border-bottom: none;
                    border-right: none;
                    transform: rotate(45deg);
                }
                "#
            )
        } else {
            String::new()
        }
    }

    /// 禁用样式
    fn disabled_style(&self) -> String {
        css!(
            r#"
            &.ant-dropdown-disabled {
                .ant-dropdown-trigger {
                    cursor: not-allowed;
                    opacity: 0.25;
                }
            }
            "#
        )
    }

    /// 暗色主题样式
    fn dark_theme_style(&self) -> String {
        css!(
            r#"
            &.ant-dropdown-dark {
                .ant-dropdown-menu {
                    background: #001529;
                }

                .ant-dropdown-menu-item {
                    color: rgba(255, 255, 255, 0.65);
                }

                .ant-dropdown-menu-item:hover {
                    color: #fff;
                    background-color: #1890ff;
                }

                .ant-dropdown-menu-item-disabled {
                    color: rgba(255, 255, 255, 0.25) !important;
                }

                .ant-dropdown-menu-item-divider {
                    background-color: rgba(255, 255, 255, 0.1);
                }

                .ant-dropdown-menu-submenu-arrow {
                    color: rgba(255, 255, 255, 0.45);
                }
            }
            "#
        )
    }

    /// 大尺寸样式
    fn large_size_style(&self) -> String {
        css!(
            r#"
            .ant-dropdown-menu-item {
                min-height: 40px;
                padding: 8px 16px;
                font-size: 16px;
                line-height: 24px;
            }
            "#
        )
    }

    /// 小尺寸样式
    fn small_size_style(&self) -> String {
        css!(
            r#"
            .ant-dropdown-menu-item {
                min-height: 24px;
                padding: 2px 8px;
                font-size: 12px;
                line-height: 20px;
            }
            "#
        )
    }

    /// 紧凑尺寸样式
    fn compact_size_style(&self) -> String {
        css!(
            r#"
            &.ant-dropdown-compact {
                .ant-dropdown-menu-item {
                    min-height: 24px;
                    padding: 2px 8px;
                    font-size: 12px;
                    line-height: 20px;
                }

                .ant-dropdown-menu {
                    padding: 2px 0;
                }

                .ant-dropdown-menu-item-icon {
                    margin-right: 4px;
                }
            }
            "#
        )
    }
}

/// 下拉菜单容器样式
pub fn dropdown_wrap_style() -> String {
    css!(
        r#"
        .ant-dropdown-wrap {
            position: absolute;
            z-index: 1050;
            min-width: 120px;
            margin: 0;
            padding: 4px 0;
            background-color: #fff;
            background-clip: padding-box;
            border-radius: 6px;
            outline: none;
            box-shadow: 0 6px 16px 0 rgba(0, 0, 0, 0.08),
                0 3px 6px -4px rgba(0, 0, 0, 0.12),
                0 9px 28px 8px rgba(0, 0, 0, 0.05);
            animation: antDropdownSlideUpIn 0.3s cubic-bezier(0.08, 0.82, 0.17, 1);
            transform-origin: 0 0;
        }

        @keyframes antDropdownSlideUpIn {
            0% {
                opacity: 0;
                transform: scaleY(0.8);
            }

            100% {
                opacity: 1;
                transform: scaleY(1);
            }
        }

        @keyframes antDropdownSlideUpOut {
            0% {
                opacity: 1;
                transform: scaleY(1);
            }

            100% {
                opacity: 0;
                transform: scaleY(0.8);
            }
        }

        .ant-dropdown-hidden {
            display: none;
        }
        "#
    )
}

/// 下拉菜单样式
pub fn dropdown_menu_style() -> String {
    css!(
        r#"
        .ant-dropdown-menu {
            position: relative;
            margin: 0;
            padding: 4px 0;
            text-align: left;
            list-style-type: none;
            background-color: #fff;
            background-clip: padding-box;
            border-radius: 6px;
            outline: none;
            box-shadow: none;
        }

        .ant-dropdown-menu-root {
            max-height: 400px;
            overflow-x: hidden;
            overflow-y: auto;
        }

        .ant-dropdown-menu-vertical {
            padding: 4px 0;
        }

        .ant-dropdown-menu-item {
            position: relative;
            display: flex;
            align-items: center;
            min-height: 32px;
            padding: 5px 12px;
            color: rgba(0, 0, 0, 0.85);
            font-weight: normal;
            font-size: 14px;
            line-height: 22px;
            cursor: pointer;
            transition: all 0.3s;
            border-radius: 0;
            user-select: none;
        }

        .ant-dropdown-menu-item:hover {
            background-color: #f5f5f5;
        }

        .ant-dropdown-menu-item:active {
            background-color: #e6f7ff;
        }

        .ant-dropdown-menu-item-disabled {
            color: rgba(0, 0, 0, 0.25);
            cursor: not-allowed;
        }

        .ant-dropdown-menu-item-disabled:hover {
            color: rgba(0, 0, 0, 0.25);
            background-color: transparent;
            cursor: not-allowed;
        }

        .ant-dropdown-menu-item-icon {
            min-width: 12px;
            margin-right: 8px;
            font-size: 12px;
        }

        .ant-dropdown-menu-title-content {
            flex: 1;
        }

        .ant-dropdown-menu-submenu-arrow {
            margin-left: auto;
            font-size: 10px;
            color: rgba(0, 0, 0, 0.45);
            transition: transform 0.3s;
        }

        .ant-dropdown-menu-item-divider {
            height: 1px;
            margin: 4px 0;
            overflow: hidden;
            line-height: 0;
            background-color: rgba(5, 5, 5, 0.06);
        }

        .ant-dropdown-menu-sub {
            position: absolute;
            top: 0;
            left: 100%;
            min-width: 120px;
            margin-left: 4px;
            background: #fff;
            border-radius: 6px;
            box-shadow: 0 6px 16px 0 rgba(0, 0, 0, 0.08),
                0 3px 6px -4px rgba(0, 0, 0, 0.12),
                0 9px 28px 8px rgba(0, 0, 0, 0.05);
            transform-origin: 0 0;
        }
        "#
    )
}

/// 响应式样式
pub fn dropdown_responsive_style() -> String {
    css!(
        r#"
        @media (max-width: 575px) {
            .ant-dropdown-wrap {
                width: 100%;
            }

            .ant-dropdown-menu {
                border-radius: 0;
                box-shadow: none;
                width: 100%;
            }
        }

        @media (prefers-contrast: high) {
            .ant-dropdown-menu {
                border: 1px solid #000;
            }

            .ant-dropdown-menu-item {
                border-bottom: 1px dotted #000;
            }

            .ant-dropdown-menu-item:hover {
                background-color: #000 !important;
                color: #fff !important;
            }
        }

        @media (prefers-reduced-motion: reduce) {
            .ant-dropdown-wrap {
                animation: none !important;
            }

            .ant-dropdown-menu-item {
                transition: none !important;
            }

            .ant-dropdown-menu-submenu-arrow {
                transition: none !important;
            }
        }

        @media print {
            .ant-dropdown-wrap {
                display: none !important;
            }
        }
        "#
    )
}

/// A11y 样式
pub fn dropdown_a11y_style() -> String {
    css!(
        r#"
        .ant-dropdown-menu-item:focus {
            outline: 2px solid #1890ff;
            outline-offset: -2px;
        }

        .ant-dropdown-trigger:focus {
            outline: 2px solid #1890ff;
            outline-offset: 1px;
        }
        "#
    )
}

/// 生成完整的下拉菜单样式
pub fn generate_dropdown_style(
    placement: DropdownPlacement,
    theme: DropdownTheme,
    size: DropdownSize,
    arrow: bool,
    disabled: bool,
) -> String {
    format!(
        "{} {} {} {} {}",
        DropdownStyleGenerator::new()
            .with_placement(placement)
            .with_theme(theme)
            .with_size(size)
            .with_arrow(arrow)
            .with_disabled(disabled)
            .generate(),
        dropdown_wrap_style(),
        dropdown_menu_style(),
        dropdown_responsive_style(),
        dropdown_a11y_style(),
    )
}

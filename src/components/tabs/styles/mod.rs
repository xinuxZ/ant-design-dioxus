//! Tabs 标签页组件样式模块
//!
//! 本模块包含 Tabs 组件的所有样式定义，从组件逻辑中分离出来，
//! 提高代码的可维护性和复用性。

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
}

impl TabsStyleGenerator {
    /// 创建新的标签页样式生成器
    pub fn new() -> Self {
        Self {
            tab_type: TabsType::default(),
            tab_position: TabsPosition::default(),
            size: TabsSize::default(),
            dark: false,
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

    /// 生成完整的标签页样式类名
    pub fn generate(&self) -> String {
        let mut classes = vec![
            self.base_style(),
            self.position_style(),
            self.type_style(),
            self.size_style(),
        ];

        if self.dark {
            classes.push(self.dark_style());
        }

        classes.join(" ")
    }

    /// 基础标签页样式
    fn base_style(&self) -> String {
        css!(
            r#"
            box-sizing: border-box;
            margin: 0;
            padding: 0;
            color: rgba(0, 0, 0, 0.85);
            font-size: 14px;
            font-variant: tabular-nums;
            line-height: 1.5715;
            list-style: none;
            font-feature-settings: 'tnum';
            position: relative;
            overflow: hidden;
            zoom: 1;

            &::before {
                display: table;
                content: '';
            }

            &::after {
                display: table;
                clear: both;
                content: '';
            }
            "#
        )
    }

    /// 标签页位置样式
    fn position_style(&self) -> String {
        match self.tab_position {
            TabsPosition::Top => css!(
                r#"
                &.ant-tabs-top {
                    .ant-tabs-bar {
                        border-bottom: 1px solid #f0f0f0;
                        margin: 0 0 16px 0;
                    }
                }
                "#
            ),
            TabsPosition::Right => css!(
                r#"
                &.ant-tabs-right {
                    .ant-tabs-bar {
                        float: right;
                        border-left: 1px solid #f0f0f0;
                        margin: 0 0 0 16px;
                    }

                    .ant-tabs-bar .ant-tabs-ink-bar {
                        left: 1px;
                        right: auto;
                    }

                    .ant-tabs-content {
                        margin-right: -1px;
                        border-right: 1px solid #f0f0f0;
                    }
                }
                "#
            ),
            TabsPosition::Bottom => css!(
                r#"
                &.ant-tabs-bottom {
                    .ant-tabs-bar {
                        border-top: 1px solid #f0f0f0;
                        border-bottom: none;
                        margin: 16px 0 0 0;
                    }

                    .ant-tabs-ink-bar {
                        top: 1px;
                        bottom: auto;
                    }

                    .ant-tabs-nav-container {
                        margin-top: -1px;
                        margin-bottom: 0;
                    }
                }
                "#
            ),
            TabsPosition::Left => css!(
                r#"
                &.ant-tabs-left {
                    .ant-tabs-bar {
                        float: left;
                        border-right: 1px solid #f0f0f0;
                        margin: 0 16px 0 0;
                    }

                    .ant-tabs-bar .ant-tabs-tab {
                        display: block;
                        margin-right: 0;
                        margin-bottom: 24px;
                        text-align: right;
                    }

                    .ant-tabs-bar .ant-tabs-nav-container {
                        margin-right: -1px;
                    }

                    .ant-tabs-bar .ant-tabs-nav-wrap {
                        margin-right: -1px;
                    }

                    .ant-tabs-bar .ant-tabs-nav {
                        padding-left: 0;
                    }

                    .ant-tabs-bar .ant-tabs-ink-bar {
                        right: 1px;
                        left: auto;
                        width: 2px;
                        height: auto;
                    }

                    .ant-tabs-bar {
                        .ant-tabs-tab:last-child {
                            margin-bottom: 0;
                        }
                    }

                    .ant-tabs-bar .ant-tabs-ink-bar {
                        right: 1px;
                        left: auto;
                    }

                    .ant-tabs-content {
                        margin-left: -1px;
                        border-left: 1px solid #f0f0f0;
                    }
                }
                "#
            ),
        }
    }

    /// 标签页类型样式
    fn type_style(&self) -> String {
        match self.tab_type {
            TabsType::Line => css!(
                r#"
                &.ant-tabs-line {
                    .ant-tabs-ink-bar {
                        position: absolute;
                        background: #1890ff;
                        pointer-events: none;
                        height: 2px;
                        bottom: 1px;
                        left: 0;
                        background-color: #1890ff;
                        transform-origin: 0 0;
                        transition: transform 0.3s cubic-bezier(0.645, 0.045, 0.355, 1), width 0.3s cubic-bezier(0.645, 0.045, 0.355, 1), left 0.3s cubic-bezier(0.645, 0.045, 0.355, 1);
                    }

                    .ant-tabs-ink-bar-animated {
                        transition: transform 0.3s cubic-bezier(0.645, 0.045, 0.355, 1), width 0.2s cubic-bezier(0.645, 0.045, 0.355, 1), left 0.3s cubic-bezier(0.645, 0.045, 0.355, 1);
                    }
                }
                "#
            ),
            TabsType::Card => css!(
                r#"
                &.ant-tabs-card {
                    .ant-tabs-bar {
                        border-bottom: 1px solid #f0f0f0;
                    }

                    .ant-tabs-nav-container {
                        height: 40px;
                    }

                    .ant-tabs-tab {
                        margin: 0;
                        padding: 8px 16px;
                        background: #fafafa;
                        border: 1px solid #f0f0f0;
                        border-bottom: 0;
                        border-radius: 4px 4px 0 0;
                        transition: all 0.3s cubic-bezier(0.645, 0.045, 0.355, 1);
                        line-height: 22px;
                    }

                    .ant-tabs-tab-active {
                        background: #fff;
                        border-color: #f0f0f0;
                        border-bottom: 1px solid #fff;
                        margin-bottom: -1px;
                    }

                    .ant-tabs-tab:hover {
                        color: #40a9ff;
                    }

                    .ant-tabs-ink-bar {
                        visibility: hidden;
                    }
                }
                "#
            ),
            TabsType::EditableCard => css!(
                r#"
                &.ant-tabs-editable-card {
                    .ant-tabs-nav-add {
                        background: #fff;
                        border: 1px dashed #d9d9d9;
                        border-radius: 2px;
                        margin: 0 10px;
                    }

                    .ant-tabs-nav-add:hover {
                        color: #40a9ff;
                        border-color: #40a9ff;
                    }

                    .ant-tabs-tab {
                        padding: 8px 8px 8px 16px;
                        background: #fafafa;
                        border: 1px solid #f0f0f0;
                        border-radius: 4px 4px 0 0;
                        margin: 0 4px 0 0;
                    }

                    .ant-tabs-tab-active {
                        background: #fff;
                    }

                    .ant-tabs-tab:hover {
                        color: #40a9ff;
                    }
                }
                "#
            ),
        }
    }

    /// 标签页大小样式
    fn size_style(&self) -> String {
        match self.size {
            TabsSize::Large => css!(
                r#"
                &.ant-tabs-large {
                    .ant-tabs-nav-container {
                        font-size: 16px;
                    }

                    .ant-tabs-tab {
                        padding: 16px 20px;
                    }
                }
                "#
            ),
            TabsSize::Default => css!(
                r#"
                &.ant-tabs-default {

                }
                "#
            ),
            TabsSize::Small => css!(
                r#"
                &.ant-tabs-small {
                    .ant-tabs-nav-container {
                        font-size: 12px;
                    }

                    .ant-tabs-tab {
                        padding: 8px 12px;
                    }

                    &.ant-tabs-card .ant-tabs-tab {
                        padding: 6px 12px;
                        font-size: 12px;
                        line-height: 18px;
                    }
                }
                "#
            ),
        }
    }

    /// 暗色模式样式
    fn dark_style(&self) -> String {
        css!(
            r#"
            &.ant-tabs-dark {
                color: #fff;
            }

            &.ant-tabs-dark .ant-tabs-bar {
                border-bottom: 1px solid #303030;
            }

            &.ant-tabs-dark .ant-tabs-tab {
                color: rgba(255, 255, 255, 0.65);
            }

            &.ant-tabs-dark .ant-tabs-tab:hover {
                color: #fff;
            }

            &.ant-tabs-dark .ant-tabs-tab-active {
                color: #fff;
            }

            &.ant-tabs-dark .ant-tabs-ink-bar {
                background-color: #fff;
            }

            &.ant-tabs-dark.ant-tabs-card .ant-tabs-tab {
                background: #262626;
                border-color: #303030;
            }

            &.ant-tabs-dark.ant-tabs-card .ant-tabs-tab-active {
                background: #1f1f1f;
                border-color: #303030;
            }
            "#
        )
    }
}

/// 标签页内容样式
pub fn tabs_content_style() -> String {
    css!(
        r#"
        .ant-tabs-content {
            width: 100%;
        }

        .ant-tabs-content-animated {
            transition: margin-left 0.3s cubic-bezier(0.645, 0.045, 0.355, 1);
            will-change: margin-left;
        }
        "#
    )
}

/// 标签页面板样式
pub fn tabs_pane_style() -> String {
    css!(
        r#"
        .ant-tabs-tabpane {
            flex-shrink: 0;
            width: 100%;
            opacity: 1;
            transition: opacity 0.45s;
        }

        .ant-tabs-tabpane-inactive {
            opacity: 0;
            height: 0;
            padding: 0 !important;
            pointer-events: none;
        }

        .ant-tabs-tabpane-inactive * {
            visibility: hidden;
        }
        "#
    )
}

/// 标签导航区域样式
pub fn tabs_nav_style() -> String {
    css!(
        r#"
        .ant-tabs-bar {
            border-bottom: 1px solid #f0f0f0;
            margin: 0 0 16px 0;
            outline: none;
            transition: padding 0.3s cubic-bezier(0.645, 0.045, 0.355, 1);
        }

        .ant-tabs-nav-container {
            overflow: hidden;
            font-size: 14px;
            line-height: 1.5715;
            box-sizing: border-box;
            position: relative;
            white-space: nowrap;
            margin-bottom: -1px;
            zoom: 1;
        }

        .ant-tabs-nav-container::before {
            display: table;
            content: '';
        }

        .ant-tabs-nav-container::after {
            display: table;
            clear: both;
            content: '';
        }

        .ant-tabs-nav-wrap {
            overflow: hidden;
            margin-bottom: -1px;
        }

        .ant-tabs-nav-scroll {
            overflow: hidden;
            white-space: nowrap;
        }

        .ant-tabs-nav {
            box-sizing: border-box;
            padding-left: 0;
            transition: transform 0.3s cubic-bezier(0.645, 0.045, 0.355, 1);
            position: relative;
            margin: 0;
            list-style: none;
            display: inline-block;
        }

        .ant-tabs-nav::before {
            display: table;
            content: '';
        }

        .ant-tabs-nav::after {
            display: table;
            clear: both;
            content: '';
        }

        .ant-tabs-extra-content {
            float: right;
            line-height: 45px;
        }
        "#
    )
}

/// 标签项样式
pub fn tabs_tab_style() -> String {
    css!(
        r#"
        .ant-tabs-tab {
            position: relative;
            display: inline-block;
            box-sizing: border-box;
            height: 100%;
            padding: 12px 16px;
            text-decoration: none;
            cursor: pointer;
            transition: color 0.3s cubic-bezier(0.645, 0.045, 0.355, 1);
            margin-right: 32px;
        }

        .ant-tabs-tab::before {
            position: absolute;
            top: -1px;
            left: 0;
            right: 0;
            bottom: 0;
            border-radius: 6px 6px 0 0;
            border: 1px solid transparent;
            content: '';
            transition: all 0.3s;
        }

        .ant-tabs-tab:last-child {
            margin-right: 0;
        }

        .ant-tabs-tab:hover {
            color: #40a9ff;
        }

        .ant-tabs-tab:active {
            color: #096dd9;
        }

        .ant-tabs-tab-btn {
            outline: none;
            transition: all 0.3s;
            border: 0;
            margin: 0;
            padding: 0;
            background: transparent;
            color: inherit;
            font-size: inherit;
            line-height: inherit;
            cursor: pointer;
        }

        .ant-tabs-tab-btn:focus,
        .ant-tabs-tab-btn:active {
            color: #096dd9;
        }

        .ant-tabs-tab-active {
            color: #1890ff;
            font-weight: 500;
        }

        .ant-tabs-tab-disabled {
            color: rgba(0, 0, 0, 0.25);
            cursor: not-allowed;
        }

        .ant-tabs-tab-disabled:hover {
            color: rgba(0, 0, 0, 0.25);
        }

        .ant-tabs-tab-remove {
            flex: none;
            margin-right: -4px;
            margin-left: 8px;
            color: rgba(0, 0, 0, 0.45);
            font-size: 12px;
            background: transparent;
            border: none;
            outline: none;
            cursor: pointer;
            font-weight: 400;
            transition: all 0.3s;
        }

        .ant-tabs-tab-remove:hover {
            color: rgba(0, 0, 0, 0.85);
        }
        "#
    )
}

/// 标签添加按钮样式
pub fn tabs_nav_add_style() -> String {
    css!(
        r#"
        .ant-tabs-nav-add {
            min-width: 40px;
            margin-left: 2px;
            padding: 0 8px;
            background: #fafafa;
            border: 1px solid #d9d9d9;
            border-radius: 2px;
            outline: none;
            cursor: pointer;
            color: rgba(0, 0, 0, 0.65);
            transition: all 0.3s cubic-bezier(0.645, 0.045, 0.355, 1);
            display: inline-flex;
            align-items: center;
            justify-content: center;
        }

        .ant-tabs-nav-add:hover {
            color: #40a9ff;
            border-color: #40a9ff;
        }

        .ant-tabs-nav-add:active {
            color: #096dd9;
            border-color: #096dd9;
        }
        "#
    )
}

/// 响应式样式
pub fn tabs_responsive_style() -> String {
    css!(
        r#"
        @media (max-width: 575px) {
            .ant-tabs-tab {
                margin-right: 12px;
                padding: 8px 8px;
            }

            .ant-tabs-large .ant-tabs-tab {
                padding: 10px 12px;
            }

            .ant-tabs-small .ant-tabs-tab {
                padding: 6px 6px;
            }

            .ant-tabs-extra-content {
                display: none;
            }
        }

        @media (max-width: 480px) {
            .ant-tabs {
                width: 100%;
            }

            .ant-tabs-tab {
                margin-right: 6px;
                font-size: 13px;
            }

            .ant-tabs-nav-add {
                padding: 0 4px;
                min-width: 32px;
            }
        }
        "#
    )
}

/// RTL 方向样式
pub fn tabs_rtl_style() -> String {
    css!(
        r#"
        .ant-tabs-rtl {
            direction: rtl;
        }

        .ant-tabs-rtl .ant-tabs-tab {
            margin-right: 0;
            margin-left: 32px;
        }

        .ant-tabs-rtl .ant-tabs-tab:last-child {
            margin-left: 0;
        }

        .ant-tabs-rtl .ant-tabs-nav-add {
            margin-right: 2px;
            margin-left: 0;
        }

        .ant-tabs-rtl .ant-tabs-extra-content {
            float: left;
        }

        .ant-tabs-rtl.ant-tabs-left .ant-tabs-bar {
            float: right;
            margin-right: 0;
            margin-left: 16px;
            border-right: none;
            border-left: 1px solid #f0f0f0;
        }

        .ant-tabs-rtl.ant-tabs-right .ant-tabs-bar {
            float: left;
            margin-right: 16px;
            margin-left: 0;
            border-right: 1px solid #f0f0f0;
            border-left: none;
        }
        "#
    )
}

/// 生成完整的标签页样式
pub fn generate_tabs_style(
    tab_type: TabsType,
    tab_position: TabsPosition,
    size: TabsSize,
    dark: bool,
) -> String {
    format!(
        "{} {} {} {} {} {} {}",
        TabsStyleGenerator::new()
            .with_type(tab_type)
            .with_position(tab_position)
            .with_size(size)
            .with_dark(dark)
            .generate(),
        tabs_content_style(),
        tabs_pane_style(),
        tabs_nav_style(),
        tabs_tab_style(),
        tabs_nav_add_style(),
        tabs_responsive_style(),
        tabs_rtl_style(),
    )
}

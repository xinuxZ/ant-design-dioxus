//! Card组件样式模块
//!
//! 本模块包含Card组件的所有样式定义，从组件逻辑中分离出来，
//! 提高代码的可维护性和复用性。

use crate::theme::AliasToken;
use css_in_rust::css;

/// 卡片尺寸枚举
#[derive(Debug, Clone, PartialEq)]
pub enum CardSize {
    /// 默认尺寸
    Default,
    /// 小尺寸
    Small,
}

impl Default for CardSize {
    fn default() -> Self {
        Self::Default
    }
}

/// 卡片类型枚举
#[derive(Debug, Clone, PartialEq)]
pub enum CardType {
    /// 默认类型
    Default,
    /// 内部卡片
    Inner,
}

impl Default for CardType {
    fn default() -> Self {
        Self::Default
    }
}

/// 卡片样式生成器
pub struct CardStyleGenerator {
    /// 卡片尺寸
    pub size: CardSize,
    /// 卡片类型
    pub card_type: CardType,
    /// 是否有边框
    pub bordered: bool,
    /// 是否可悬浮
    pub hoverable: bool,
    /// 是否加载中
    pub loading: bool,
    /// 主题令牌
    pub token: AliasToken,
}

impl Default for CardStyleGenerator {
    fn default() -> Self {
        Self {
            size: CardSize::default(),
            card_type: CardType::default(),
            bordered: true,
            hoverable: false,
            loading: false,
            token: AliasToken::default(),
        }
    }
}

impl CardStyleGenerator {
    /// 创建新的卡片样式生成器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置卡片尺寸
    pub fn with_size(mut self, size: CardSize) -> Self {
        self.size = size;
        self
    }

    /// 设置卡片类型
    pub fn with_type(mut self, card_type: CardType) -> Self {
        self.card_type = card_type;
        self
    }

    /// 设置边框状态
    pub fn with_bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }

    /// 设置悬停效果
    pub fn with_hoverable(mut self, hoverable: bool) -> Self {
        self.hoverable = hoverable;
        self
    }

    /// 设置加载状态
    pub fn with_loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    /// 设置主题令牌
    pub fn with_token(mut self, token: AliasToken) -> Self {
        self.token = token;
        self
    }

    /// 生成样式类名
    pub fn generate(&self) -> String {
        let mut classes = vec!["ant-card"];

        // 添加尺寸相关的类名
        if self.size == CardSize::Small {
            classes.push("ant-card-small");
        }

        // 添加类型相关的类名
        if self.card_type == CardType::Inner {
            classes.push("ant-card-inner");
        }

        // 添加无边框类名
        if !self.bordered {
            classes.push("ant-card-bordered-false");
        }

        // 添加悬浮类名
        if self.hoverable {
            classes.push("ant-card-hoverable");
        }

        // 添加加载中类名
        if self.loading {
            classes.push("ant-card-loading");
        }

        classes.join(" ")
    }

    /// 生成 CSS 样式
    pub fn generate_css(&self) -> String {
        css!(
            r#"
            .ant-card {
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
                background: ${color_bg_container};
                border-radius: ${border_radius}px;
                border: 1px solid ${color_border_secondary};
            }

            .ant-card-small {
                font-size: ${font_size_sm}px;
            }

            .ant-card-small .ant-card-head {
                min-height: 36px;
                padding: 0 ${padding_sm}px;
                font-size: ${font_size}px;
            }

            .ant-card-small .ant-card-head-wrapper {
                min-height: 36px;
            }

            .ant-card-small .ant-card-body {
                padding: ${padding_sm}px;
            }

            .ant-card-inner {
                background: ${color_bg_container};
            }

            .ant-card-inner .ant-card-head {
                padding: 0 ${padding_sm}px;
                background: transparent;
            }

            .ant-card-inner .ant-card-body {
                padding: ${padding_md}px;
            }

            .ant-card-inner .ant-card-extra {
                padding: 0 ${padding_sm}px;
            }

            .ant-card-bordered-false {
                border: none;
                box-shadow: none;
            }

            .ant-card-hoverable {
                cursor: pointer;
                transition: box-shadow 0.3s, border-color 0.3s;
            }

            .ant-card-hoverable:hover {
                border-color: transparent;
                box-shadow: ${box_shadow};
            }

            .ant-card-loading .ant-card-body {
                user-select: none;
            }

            .ant-card-head {
                min-height: 48px;
                margin-bottom: -1px;
                padding: 0 ${padding_lg}px;
                color: ${color_text};
                font-weight: 500;
                font-size: ${font_size_lg}px;
                background: transparent;
                border-bottom: 1px solid ${color_split};
                border-radius: ${border_radius}px ${border_radius}px 0 0;
                display: flex;
                align-items: center;
            }

            .ant-card-head-wrapper {
                display: flex;
                align-items: center;
                flex: auto;
                min-height: 48px;
            }

            .ant-card-head-title {
                display: inline-block;
                flex: 1;
                padding: ${padding_md}px 0;
                overflow: hidden;
                white-space: nowrap;
                text-overflow: ellipsis;
            }

            .ant-card-extra {
                float: right;
                margin-left: auto;
                padding: ${padding_md}px 0;
                color: ${color_text};
                font-weight: normal;
                font-size: ${font_size}px;
            }

            .ant-card-body {
                padding: ${padding_lg}px;
                zoom: 1;
            }

            .ant-card-body::before,
            .ant-card-body::after {
                display: table;
                content: '';
            }

            .ant-card-body::after {
                clear: both;
                content: '';
            }

            .ant-card-loading-content {
                width: 100%;
                height: 100%;
            }

            .ant-card-loading-block {
                height: 14px;
                margin: 4px 0;
                background: linear-gradient(90deg, rgba(207, 216, 220, 0.2), rgba(207, 216, 220, 0.4), rgba(207, 216, 220, 0.2));
                background-size: 600% 600%;
                border-radius: ${border_radius}px;
                animation: card-loading 1.4s ease infinite;
            }

            @keyframes card-loading {
                0%,
                100% {
                    background-position: 0 50%;
                }
                50% {
                    background-position: 100% 50%;
                }
            }

            .ant-card-actions {
                margin: 0;
                padding: 0;
                list-style: none;
                background: ${color_bg_container};
                border-top: 1px solid ${color_split};
                display: flex;
                justify-content: space-around;
            }

            .ant-card-actions > li {
                margin: ${margin_xs}px 0;
                color: ${color_text_secondary};
                text-align: center;
                flex: 1;
            }

            .ant-card-actions > li > span {
                position: relative;
                display: block;
                min-width: 32px;
                font-size: ${font_size}px;
                line-height: ${line_height};
                cursor: pointer;
            }

            .ant-card-actions > li > span:hover {
                color: ${color_primary};
                transition: color 0.3s;
            }

            .ant-card-actions > li > span > .anticon {
                font-size: 16px;
                line-height: 22px;
            }

            .ant-card-actions > li > span a {
                color: ${color_text_secondary};
                transition: color 0.3s;
            }

            .ant-card-actions > li > span a:hover {
                color: ${color_primary};
            }

            .ant-card-actions > li:not(:last-child) {
                border-right: 1px solid ${color_split};
            }

            .ant-card-meta {
                margin: -4px 0;
                display: flex;
                align-items: flex-start;
            }

            .ant-card-meta-detail {
                overflow: hidden;
                flex: 1;
            }

            .ant-card-meta-avatar {
                padding-right: ${padding_md}px;
            }

            .ant-card-meta-title {
                overflow: hidden;
                color: ${color_text};
                font-weight: 500;
                font-size: ${font_size_lg}px;
                white-space: nowrap;
                text-overflow: ellipsis;
                margin-bottom: ${margin_xs}px;
            }

            .ant-card-meta-description {
                color: ${color_text_secondary};
            }

            .ant-card-grid {
                float: left;
                width: 33.33%;
                padding: ${padding_lg}px;
                border: 0;
                border-radius: 0;
                box-shadow: 1px 0 0 0 ${color_split}, 0 1px 0 0 ${color_split}, 1px 1px 0 0 ${color_split}, 1px 0 0 0 ${color_split} inset, 0 1px 0 0 ${color_split} inset;
                transition: all 0.3s;
            }

            .ant-card-grid:hover {
                position: relative;
                z-index: 1;
                box-shadow: ${box_shadow};
            }

            .ant-card-rtl {
                direction: rtl;
            }

            .ant-card-rtl .ant-card-extra {
                margin-right: auto;
                margin-left: 0;
            }

            .ant-card-rtl .ant-card-meta-avatar {
                padding-right: 0;
                padding-left: ${padding_md}px;
            }

            .ant-card-rtl .ant-card-actions > li:not(:last-child) {
                border-right: none;
                border-left: 1px solid ${color_split};
            }

            @media (max-width: 575px) {
                .ant-card-head-title {
                    font-size: ${font_size}px;
                }

                .ant-card-head {
                    padding: 0 ${padding_sm}px;
                }

                .ant-card-body {
                    padding: ${padding_md}px;
                }
            }
            "#,
            color_text = self.token.color_text,
            font_size = self.token.font_size,
            line_height = self.token.line_height,
            color_bg_container = self.token.color_bg_container,
            border_radius = self.token.border_radius,
            color_border_secondary = self.token.color_border_secondary,
            font_size_sm = self.token.font_size_sm,
            padding_sm = self.token.padding_sm,
            padding_md = self.token.padding_md,
            box_shadow = self.token.box_shadow,
            padding_lg = self.token.padding_lg,
            color_split = self.token.color_split,
            font_size_lg = self.token.font_size_lg,
            margin_xs = self.token.margin_xs,
            color_text_secondary = self.token.color_text_secondary,
            color_primary = self.token.color_primary
        ).to_string()
    }
}

/// 生成 Card 样式
pub fn generate_card_style(
    size: CardSize,
    card_type: CardType,
    bordered: bool,
    hoverable: bool,
    loading: bool,
) -> String {
    CardStyleGenerator::new()
        .with_size(size)
        .with_type(card_type)
        .with_bordered(bordered)
        .with_hoverable(hoverable)
        .with_loading(loading)
        .generate()
}

/// 生成 Card CSS 样式
pub fn generate_card_css(
    size: CardSize,
    card_type: CardType,
    bordered: bool,
    hoverable: bool,
    loading: bool,
) -> String {
    CardStyleGenerator::new()
        .with_size(size)
        .with_type(card_type)
        .with_bordered(bordered)
        .with_hoverable(hoverable)
        .with_loading(loading)
        .generate_css()
}

/// 默认 Card 样式
pub fn default_card_style() -> String {
    CardStyleGenerator::new().generate()
}

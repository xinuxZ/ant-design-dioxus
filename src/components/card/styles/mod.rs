//! Card组件样式模块
//!
//! 本模块包含Card组件的所有样式定义，从组件逻辑中分离出来，
//! 提高代码的可维护性和复用性。

use crate::shared::styles::mixins::*;
use css_in_rust::css;

/// 卡片尺寸枚举
#[derive(Debug, Clone, PartialEq)]
pub enum CardSize {
    Default,
    Small,
}

/// 卡片类型枚举
#[derive(Debug, Clone, PartialEq)]
pub enum CardType {
    Default,
    Inner,
}

/// 卡片样式生成器
pub struct CardStyleGenerator {
    pub size: CardSize,
    pub card_type: CardType,
    pub bordered: bool,
    pub hoverable: bool,
    pub loading: bool,
}

impl CardStyleGenerator {
    /// 创建新的卡片样式生成器
    pub fn new() -> Self {
        Self {
            size: CardSize::Default,
            card_type: CardType::Default,
            bordered: true,
            hoverable: false,
            loading: false,
        }
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

    /// 生成完整的卡片样式类名
    pub fn generate(&self) -> String {
        let mut classes = vec![self.base_style(), self.size_style(), self.type_style()];

        if !self.bordered {
            classes.push(self.borderless_style());
        }

        if self.hoverable {
            classes.push(self.hoverable_style());
        }

        if self.loading {
            classes.push(self.loading_style());
        }

        classes.join(" ")
    }

    /// 基础卡片样式
    fn base_style(&self) -> String {
        format!(
            "{} {}",
            card_style(),
            css!(
                r#"
            position: relative;
            "#
            )
        )
    }

    /// 卡片尺寸样式
    fn size_style(&self) -> String {
        let style = match self.size {
            CardSize::Small => css!(
                r#"
                & .ant-card-head {
                    min-height: 36px;
                    padding: 0 12px;
                    font-size: var(--ant-font-size-base);
                }

                & .ant-card-body {
                    padding: 12px;
                }
                "#
            ),
            CardSize::Default => css!(
                r#"
                & .ant-card-head {
                    min-height: 48px;
                    padding: 0 24px;
                    font-size: 16px;
                }

                & .ant-card-body {
                    padding: 24px;
                }
                "#
            ),
        };

        style.to_string()
    }

    /// 卡片类型样式
    fn type_style(&self) -> String {
        match self.card_type {
            CardType::Inner => css!(
                r#"
                background: var(--ant-bg-color-container);

                & .ant-card-head {
                    padding: 0 12px;
                    background: transparent;
                }

                & .ant-card-body {
                    padding: 16px;
                }

                & .ant-card-extra {
                    padding: 0 12px;
                }
                "#
            )
            .to_string(),
            CardType::Default => String::new(),
        }
    }

    /// 无边框样式
    fn borderless_style(&self) -> String {
        css!(
            r#"
            border: none;
            box-shadow: none;
            "#
        )
        .to_string()
    }

    /// 悬停效果样式
    fn hoverable_style(&self) -> String {
        hover_effect()
    }

    /// 加载状态样式
    fn loading_style(&self) -> String {
        loading_state()
    }
}

/// 卡片头部样式
pub fn card_head_style() -> String {
    css!(
        r#"
        min-height: 48px;
        margin-bottom: -1px;
        padding: 0 24px;
        color: var(--ant-text-color);
        font-weight: 500;
        font-size: 16px;
        background: transparent;
        border-bottom: 1px solid var(--ant-border-color-split);
        border-radius: var(--ant-border-radius) var(--ant-border-radius) 0 0;
        "#
    )
    .to_string()
}

/// 卡片头部包装器样式
pub fn card_head_wrapper_style() -> String {
    css!(
        r#"
        display: flex;
        align-items: center;
        "#
    )
    .to_string()
}

/// 卡片标题样式
pub fn card_head_title_style() -> String {
    format!(
        "{} {}",
        text_ellipsis(),
        css!(
            r#"
        flex: 1;
        padding: 16px 0;
        "#
        )
    )
}

/// 卡片额外内容样式
pub fn card_extra_style() -> String {
    css!(
        r#"
        margin-left: auto;
        padding: 16px 0;
        color: var(--ant-text-color);
        font-weight: normal;
        font-size: var(--ant-font-size-base);
        "#
    )
    .to_string()
}

/// 卡片主体样式
pub fn card_body_style() -> String {
    css!(
        r#"
        padding: 24px;
        "#
    )
    .to_string()
}

/// 卡片加载内容样式
pub fn card_loading_content_style() -> String {
    css!(
        r#"
        width: 100%;
        "#
    )
    .to_string()
}

/// 卡片加载块样式
pub fn card_loading_block_style() -> String {
    css!(
        r#"
        height: 14px;
        margin: 4px 0;
        background: linear-gradient(90deg, rgba(207, 216, 220, 0.2), rgba(207, 216, 220, 0.4), rgba(207, 216, 220, 0.2));
        background-size: 200% 100%;
        border-radius: var(--ant-border-radius);
        animation: ant-skeleton-loading 1.4s ease infinite;

        @keyframes ant-skeleton-loading {
            0% {
                background-position: 200% 0;
            }
            100% {
                background-position: -200% 0;
            }
        }
        "#
    ).to_string()
}

/// 卡片操作区样式
pub fn card_actions_style() -> String {
    css!(
        r#"
        margin: 0;
        padding: 0;
        list-style: none;
        background: var(--ant-bg-color-container);
        border-top: 1px solid var(--ant-border-color-split);
        display: flex;
        border-radius: 0 0 var(--ant-border-radius) var(--ant-border-radius);

        & > li {
            flex: 1;
            margin: 12px 0;
            color: var(--ant-text-color-secondary);
            text-align: center;
            cursor: pointer;
            transition: color var(--ant-motion-duration-mid) var(--ant-motion-ease-in-out);
        }

        & > li:not(:last-child) {
            border-right: 1px solid var(--ant-border-color-split);
        }

        & > li:hover {
            color: var(--ant-primary-color);
        }
        "#
    )
    .to_string()
}

/// 卡片元信息样式
pub fn card_meta_style() -> String {
    css!(
        r#"
        margin: -4px 0;
        display: flex;
        "#
    )
    .to_string()
}

/// 卡片元信息详情样式
pub fn card_meta_detail_style() -> String {
    css!(
        r#"
        overflow: hidden;
        "#
    )
    .to_string()
}

/// 卡片元信息头像样式
pub fn card_meta_avatar_style() -> String {
    css!(
        r#"
        padding-right: 16px;
        "#
    )
    .to_string()
}

/// 卡片元信息内容样式
pub fn card_meta_content_style() -> String {
    css!(
        r#"
        flex: 1;
        "#
    )
    .to_string()
}

/// 卡片元信息标题样式
pub fn card_meta_title_style() -> String {
    format!(
        "{} {}",
        text_ellipsis(),
        css!(
            r#"
        margin-bottom: 4px;
        color: var(--ant-text-color);
        font-weight: 500;
        font-size: 16px;
        "#
        )
    )
}

/// 卡片元信息描述样式
pub fn card_meta_description_style() -> String {
    css!(
        r#"
        color: var(--ant-text-color-secondary);
        "#
    )
    .to_string()
}

/// 卡片网格样式
pub fn card_grid_style() -> String {
    css!(
        r#"
        float: left;
        width: 33.33%;
        padding: 24px;
        border: 0;
        border-radius: 0;
        box-shadow: 1px 0 0 0 var(--ant-border-color-split), 0 1px 0 0 var(--ant-border-color-split), 1px 1px 0 0 var(--ant-border-color-split), inset 1px 0 0 0 var(--ant-border-color-split), inset 0 1px 0 0 var(--ant-border-color-split);
        transition: all var(--ant-motion-duration-mid) var(--ant-motion-ease-in-out);

        &:hover {
            position: relative;
            z-index: 1;
            box-shadow: var(--ant-box-shadow-base);
        }
        "#
    ).to_string()
}

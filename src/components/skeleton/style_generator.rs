//!
//! Skeleton组件样式管理
//!
//! 使用样式生成器模式管理Skeleton组件的样式
//! 使用 css! 宏的样式生成器模式，提供 Skeleton 组件的完整样式生成功能

use crate::components::skeleton::types::*;
use crate::shared::styles::mixins::*;
use css_in_rust::css;

/// Skeleton 尺寸枚举
#[derive(Debug, Clone, PartialEq)]
pub enum SkeletonSize {
    Small,
    Default,
    Large,
}

/// Skeleton样式生成器
pub struct SkeletonStyleGenerator {
    pub active: bool,
    pub loading: bool,
    pub avatar: bool,
    pub title: bool,
    pub paragraph: bool,
    pub round: bool,
    pub size: SkeletonSize,
}

impl SkeletonStyleGenerator {
    /// 创建新的Skeleton样式生成器
    pub fn new() -> Self {
        Self {
            active: false,
            loading: true,
            avatar: false,
            title: true,
            paragraph: true,
            round: false,
            size: SkeletonSize::Default,
        }
    }

    /// 设置是否激活动画
    pub fn with_active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    /// 设置是否加载中
    pub fn with_loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    /// 设置是否显示头像
    pub fn with_avatar(mut self, avatar: bool) -> Self {
        self.avatar = avatar;
        self
    }

    /// 设置是否显示标题
    pub fn with_title(mut self, title: bool) -> Self {
        self.title = title;
        self
    }

    /// 设置是否显示段落
    pub fn with_paragraph(mut self, paragraph: bool) -> Self {
        self.paragraph = paragraph;
        self
    }

    /// 设置是否圆角
    pub fn with_round(mut self, round: bool) -> Self {
        self.round = round;
        self
    }

    /// 设置尺寸
    pub fn with_size(mut self, size: SkeletonSize) -> Self {
        self.size = size;
        self
    }

    /// 生成完整的Skeleton样式类名
    pub fn generate(&self) -> String {
        let mut classes = vec![self.base_style()];

        if self.active {
            classes.push(self.active_style());
        }

        if self.round {
            classes.push(self.round_style());
        }

        classes.push(self.size_style());

        classes.join(" ")
    }

    /// 生成头像样式类名
    pub fn generate_avatar(&self) -> String {
        let mut classes = vec![self.avatar_base_style()];

        if self.active {
            classes.push(self.active_style());
        }

        if self.round {
            classes.push(self.avatar_round_style());
        }

        classes.push(self.avatar_size_style());

        classes.join(" ")
    }

    /// 生成标题样式类名
    pub fn generate_title(&self) -> String {
        let mut classes = vec![self.title_base_style()];

        if self.active {
            classes.push(self.active_style());
        }

        if self.round {
            classes.push(self.round_style());
        }

        classes.join(" ")
    }

    /// 生成段落样式类名
    pub fn generate_paragraph(&self) -> String {
        let mut classes = vec![self.paragraph_base_style()];

        if self.active {
            classes.push(self.active_style());
        }

        if self.round {
            classes.push(self.round_style());
        }

        classes.join(" ")
    }

    /// 基础Skeleton样式
    fn base_style(&self) -> String {
        css!(
            r#"
            display: block;
            "#
        )
        .to_string()
    }

    /// 激活动画样式
    fn active_style(&self) -> String {
        css!(
            r#"
            position: relative;
            overflow: hidden;
            
            &::after {
                content: '';
                position: absolute;
                top: 0;
                left: -100%;
                width: 100%;
                height: 100%;
                background: linear-gradient(
                    90deg,
                    transparent,
                    rgba(255, 255, 255, 0.4),
                    transparent
                );
                animation: skeleton-loading 1.4s ease-in-out infinite;
            }
            
            @keyframes skeleton-loading {
                0% {
                    left: -100%;
                }
                100% {
                    left: 100%;
                }
            }
            "#
        )
        .to_string()
    }

    /// 圆角样式
    fn round_style(&self) -> String {
        css!(
            r#"
            border-radius: 6px;
            "#
        )
        .to_string()
    }

    /// 尺寸样式
    fn size_style(&self) -> String {
        match self.size {
            SkeletonSize::Small => css!(
                r#"
                font-size: 12px;
                "#
            ),
            SkeletonSize::Default => css!(
                r#"
                font-size: 14px;
                "#
            ),
            SkeletonSize::Large => css!(
                r#"
                font-size: 16px;
                "#
            ),
        }
        .to_string()
    }

    /// 头像基础样式
    fn avatar_base_style(&self) -> String {
        css!(
            r#"
            display: inline-block;
            width: 32px;
            height: 32px;
            background-color: #f5f5f5;
            border-radius: 50%;
            "#
        )
        .to_string()
    }

    /// 头像圆角样式
    fn avatar_round_style(&self) -> String {
        css!(
            r#"
            border-radius: 6px;
            "#
        )
        .to_string()
    }

    /// 头像尺寸样式
    fn avatar_size_style(&self) -> String {
        match self.size {
            SkeletonSize::Small => css!(
                r#"
                width: 24px;
                height: 24px;
                "#
            ),
            SkeletonSize::Default => css!(
                r#"
                width: 32px;
                height: 32px;
                "#
            ),
            SkeletonSize::Large => css!(
                r#"
                width: 40px;
                height: 40px;
                "#
            ),
        }
        .to_string()
    }

    /// 标题基础样式
    fn title_base_style(&self) -> String {
        css!(
            r#"
            width: 38%;
            height: 16px;
            background-color: #f5f5f5;
            margin-bottom: 16px;
            "#
        )
        .to_string()
    }

    /// 段落基础样式
    fn paragraph_base_style(&self) -> String {
        css!(
            r#"
            width: 100%;
            
            & > li {
                height: 16px;
                background-color: #f5f5f5;
                margin-bottom: 8px;
                list-style: none;
            }
            
            & > li:last-child {
                width: 61%;
                margin-bottom: 0;
            }
            "#
        )
        .to_string()
    }
}

impl Default for SkeletonStyleGenerator {
    fn default() -> Self {
        Self::new()
    }
}
//! # Skeleton 骨架屏组件
//!
//! 在内容加载时显示占位符，提供更好的用户体验。
//! 当组件包含大量信息（如列表或卡片）时特别有用，仅在首次加载数据时工作。
//!
//! ## 特性
//!
//! - 🎭 **多种占位符**: 支持头像、标题、段落等多种占位符类型
//! - 🎨 **动画效果**: 内置波浪动画和闪烁效果
//! - 🎯 **子组件**: 提供 Button、Input、Image 等专用骨架屏
//! - 🎪 **自适应**: 根据内容自动调整布局和样式
//! - 🎨 **主题支持**: 完整的 Design Token 和主题定制
//! - 📱 **响应式**: 支持不同屏幕尺寸的适配
//!
//! ## 使用示例
//!
//! ### 基础用法
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::Skeleton;
//!
//! #[component]
//! fn App() -> Element {
//!     let loading = use_signal(|| true);
//!
//!     rsx! {
//!         Skeleton {
//!             loading: loading(),
//!             active: true,
//!             "内容加载完成后显示"
//!         }
//!     }
//! }
//! ```
//!
//! ### 带头像的骨架屏
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::{Skeleton, SkeletonAvatarConfig, SkeletonAvatarProps, AvatarShape, AvatarSize};
//!
//! #[component]
//! fn App() -> Element {
//!     rsx! {
//!         Skeleton {
//!             loading: true,
//!             avatar: SkeletonAvatarConfig::Config(SkeletonAvatarProps {
//!                 shape: Some(AvatarShape::Circle),
//!                 size: Some(AvatarSize::Large),
//!                 active: Some(true),
//!             }),
//!             paragraph: SkeletonParagraphConfig::Config(SkeletonParagraphProps {
//!                 rows: Some(3),
//!                 width: None,
//!             }),
//!         }
//!     }
//! }
//! ```
//!
//! ### 子组件使用
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::{SkeletonButton, SkeletonInput, SkeletonImage};
//!
//! #[component]
//! fn App() -> Element {
//!     rsx! {
//!         div {
//!             SkeletonButton {
//!                 active: true,
//!                 size: Some(ButtonSize::Large),
//!             }
//!             SkeletonInput {
//!                 active: true,
//!                 size: Some(InputSize::Default),
//!             }
//!             SkeletonImage {
//!                 active: true,
//!             }
//!         }
//!     }
//! }
//! ```

mod component;
mod styles;
mod types;
mod utils;

// 重新导出主要组件和类型
pub use component::{Skeleton, SkeletonButton, SkeletonImage, SkeletonInput};

pub use types::{
    AvatarShape as SkeletonAvatarShape, AvatarSize as SkeletonAvatarSize,
    ButtonShape as SkeletonButtonShape, ButtonSize as SkeletonButtonSize,
    InputSize as SkeletonInputSize, SkeletonAvatarConfig, SkeletonAvatarProps, SkeletonButtonProps,
    SkeletonImageProps, SkeletonInputProps, SkeletonParagraphConfig, SkeletonParagraphProps,
    SkeletonProps, SkeletonTheme, SkeletonTitleConfig, SkeletonTitleProps, SkeletonWidth,
    SkeletonWidthConfig,
};

pub use styles::{
    generate_skeleton_animation, generate_skeleton_avatar_style, generate_skeleton_button_style,
    generate_skeleton_image_style, generate_skeleton_input_style,
    generate_skeleton_paragraph_style, generate_skeleton_style, generate_skeleton_title_style,
    get_skeleton_class_name,
};

pub use utils::{
    calculate_avatar_props, calculate_paragraph_props, calculate_title_props,
    generate_paragraph_widths, validate_skeleton_config,
};

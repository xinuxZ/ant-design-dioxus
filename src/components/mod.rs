//! # Components
//!
//! Ant Design Dioxus 组件库
//!
//! 本模块包含了所有的 UI 组件实现，遵循 Ant Design 设计规范。

// 基础组件
pub mod icon;
// pub mod typography;  // TODO: 待实现

// 通用组件
pub mod button;
pub mod divider;
// pub mod typography;

// 布局组件
// pub mod grid;
// pub mod layout;
pub mod space;

// 导航组件
// pub mod affix;
// pub mod anchor;
// pub mod breadcrumb;
// pub mod dropdown;
// pub mod menu;
// pub mod pagination;
// pub mod steps;

// 数据录入组件
// pub mod auto_complete;
// pub mod checkbox;
// pub mod date_picker;
// pub mod form;
// pub mod input;
// pub mod input_number;
// pub mod mentions;
// pub mod radio;
// pub mod rate;
// pub mod select;
// pub mod slider;
// pub mod switch;
// pub mod transfer;
// pub mod tree_select;
// pub mod upload;

// 数据展示组件
// pub mod avatar;
// pub mod badge;
// pub mod calendar;
// pub mod card;
// pub mod carousel;
// pub mod collapse;
// pub mod descriptions;
// pub mod empty;
// pub mod image;
// pub mod list;
// pub mod popover;
// pub mod segmented;
// pub mod statistic;
// pub mod table;
// pub mod tabs;
pub mod tag;
// pub mod timeline;
// pub mod tooltip;
// pub mod tour;
// pub mod tree;
pub mod typography;

// 反馈组件
pub mod alert;
// pub mod drawer;
// pub mod message;
// pub mod modal;
// pub mod notification;
// pub mod popconfirm;
// pub mod progress;
// pub mod result;
// pub mod skeleton;
// pub mod spin;

// 其他组件
// pub mod flex;
// pub mod watermark;

// 重新导出所有组件
pub use alert::*;
pub use button::*;
pub use divider::*;
pub use space::*;
pub use tag::*;

// 组件通用类型和特性
pub mod common {
    use dioxus::prelude::*;

    /// 组件大小枚举
    #[derive(Debug, Clone, PartialEq)]
    pub enum Size {
        Small,
        Middle,
        Large,
    }

    impl Default for Size {
        fn default() -> Self {
            Size::Middle
        }
    }

    impl From<&str> for Size {
        fn from(value: &str) -> Self {
            match value {
                "small" => Size::Small,
                "large" => Size::Large,
                _ => Size::Middle,
            }
        }
    }

    /// 组件状态枚举
    #[derive(Debug, Clone, PartialEq)]
    pub enum Status {
        Success,
        Warning,
        Error,
        Info,
    }

    impl From<&str> for Status {
        fn from(value: &str) -> Self {
            match value {
                "success" => Status::Success,
                "warning" => Status::Warning,
                "error" => Status::Error,
                _ => Status::Info,
            }
        }
    }

    /// 组件变体枚举
    #[derive(Debug, Clone, PartialEq)]
    pub enum Variant {
        Default,
        Primary,
        Secondary,
        Dashed,
        Text,
        Link,
    }

    impl Default for Variant {
        fn default() -> Self {
            Variant::Default
        }
    }

    impl From<&str> for Variant {
        fn from(value: &str) -> Self {
            match value {
                "primary" => Variant::Primary,
                "secondary" => Variant::Secondary,
                "dashed" => Variant::Dashed,
                "text" => Variant::Text,
                "link" => Variant::Link,
                _ => Variant::Default,
            }
        }
    }
}

// 重新导出通用类型
pub use common::*;

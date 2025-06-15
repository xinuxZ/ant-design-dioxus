//! Ant Design components for Dioxus.
//!
//! This module provides all Ant Design components implemented for Dioxus framework.

// Level 1 组件
pub mod qr_code; // 已实现 // 临时实现

// pub mod back_top;
pub mod icon;
// pub mod alert;
pub mod divider;
// pub mod breadcrumb;
// pub mod empty;
pub mod flex;
// pub mod grid;
// pub mod layout;
// pub mod popconfirm;
// pub mod popover;
// pub mod progress;
// pub mod rate;
// pub mod result;
pub mod skeleton;
pub mod space;
pub mod spin;
// pub mod switch;
// pub mod tag;
// pub mod tooltip;
// pub mod watermark;

// Level 2 组件 (依赖 Level 1)
pub mod alert; // 警告提示 - 依赖 icon

// pub mod avatar;          // 头像 - 依赖 icon
// pub mod badge;           // 徽标数 - 依赖 icon
pub mod button; // 按钮 - 依赖 icon, spin
                // pub mod card;            // 卡片 - 依赖 divider
                // pub mod collapse;        // 折叠面板 - 依赖 icon
                // pub mod empty;           // 空状态 - 依赖 icon
                // pub mod image;           // 图片 - 依赖 icon, spin
                // pub mod input;           // 输入框 - 依赖 icon
                // pub mod list;            // 列表 - 依赖 icon, spin
                // pub mod message;         // 全局提示 - 依赖 icon
                // pub mod notification;    // 通知提醒框 - 依赖 icon
                // pub mod pagination;      // 分页 - 依赖 icon
                // pub mod popover;         // 气泡卡片 - 依赖 icon
                // pub mod progress;        // 进度条 - 依赖 icon
                // pub mod result;          // 结果 - 依赖 icon, button
                // pub mod select;          // 选择器 - 依赖 icon, spin
                // pub mod slider;          // 滑动输入条 - 依赖 icon
                // pub mod steps;           // 步骤条 - 依赖 icon
                // pub mod switch;          // 开关 - 依赖 icon
                // pub mod tag;             // 标签 - 依赖 icon
                // pub mod timeline;        // 时间轴 - 依赖 icon
                // pub mod tooltip;         // 文字提示 - 依赖 icon
                // pub mod tree;            // 树形控件 - 依赖 icon

// Level 3 组件
// pub mod affix;
// pub mod anchor;
// pub mod auto_complete;
// pub mod cascader;
// pub mod checkbox;
// pub mod date_picker;
// pub mod form;
// pub mod input;
// pub mod input_number;
// pub mod mentions;
// pub mod menu;
// pub mod radio;
// pub mod select;
// pub mod slider;
// pub mod time_picker;
// pub mod transfer;
// pub mod tree_select;
// pub mod upload;

// 其他组件
// pub mod app;
// pub mod color_picker;
// pub mod drawer;
// pub mod float_button;
// pub mod message;
// pub mod notification;
// pub mod splitter;
// pub mod statistic;
// pub mod tour;

// 重新导出组件
pub use alert::{Alert, AlertProps};
pub use button::{Button, ButtonShape, ButtonSize, ButtonType, LoadingConfig};
pub use divider::{Divider, DividerProps};
pub use flex::{Flex, FlexProps};
pub use icon::{Icon, IconProps, IconType};
pub use qr_code::{QRCode, QRCodeProps};
pub use skeleton::{Skeleton, SkeletonProps};
pub use space::{Space, SpaceProps};
pub use spin::{Spin, SpinProps};

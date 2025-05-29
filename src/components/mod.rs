//! Ant Design Dioxus 组件库
//!
//! 这个模块包含了所有的 UI 组件实现
//!
//! ## 组件分类
//!
//! - **通用组件**: Button, Icon, Typography
//! - **布局组件**: Grid, Layout, Space, Divider
//! - **导航组件**: Affix, Breadcrumb, Dropdown, Menu, Pagination, Steps
//! - **数据录入**: AutoComplete, Cascader, Checkbox, DatePicker, Form, Input, InputNumber, Mentions, Radio, Rate, Select, Slider, Switch, TimePicker, Transfer, TreeSelect, Upload
//! - **数据展示**: Avatar, Badge, Calendar, Card, Carousel, Collapse, Comment, Descriptions, Empty, Image, List, Popover, Segmented, Statistic, Table, Tabs, Tag, Timeline, Tooltip, Tour, Tree
//! - **反馈**: Alert, Drawer, Message, Modal, Notification, Popconfirm, Progress, Result, Skeleton, Spin
//! - **其他**: Anchor, BackTop, ConfigProvider, FloatButton, QRCode, Theme, Watermark

// 基础导入在各个组件中按需引入

// 通用组件
pub mod button;
pub mod grid;
pub mod icon;
pub mod layout;
pub mod typography;

// 重新导出所有组件
pub use avatar::*;
pub use badge::*;

pub use breadcrumb::*;
pub use button::*;
pub use card::*;
pub use carousel::*;
pub use collapse::*;
pub use descriptions::*;
pub use divider::*;
pub use empty::*;
pub use flex::*;
pub use grid::*;
pub use icon::*;
pub use image::*;
pub use layout::*;
pub use list::*;
pub use menu::*;
pub use popover::*;
pub use select::*;
pub use space::*;
pub use spin::*;
pub use statistic::*;
pub use table::*;
pub use tabs::*;
pub use tag::*;
pub use timeline::*;
pub use tooltip::*;
pub use tree::*;
pub use typography::*;

pub mod alert;
pub use alert::*;

pub mod progress;
pub use progress::*;

// 组件模块声明（后续逐步实现）

// 通用组件
// pub mod button;
// pub mod icon;
// pub mod typography;

// 布局组件
pub mod divider;
// pub mod grid;
// pub mod layout;
pub mod flex;
pub mod space;

// 导航组件
pub mod affix;
pub use affix::*;
pub mod breadcrumb;
pub mod dropdown;
pub use dropdown::*;

pub mod menu;
pub mod pagination;
pub use pagination::*;
pub mod steps;
pub use steps::*;

// 数据录入组件
pub mod auto_complete;
pub use auto_complete::*;
pub mod cascader;
pub use cascader::*;
pub mod checkbox;
pub use checkbox::*;
pub mod date_picker;
pub use date_picker::*;
pub mod form;
pub use form::*;
pub mod input;
pub use input::*;
pub mod input_number;
pub use input_number::*;
pub mod mentions;
pub use mentions::*;
pub mod radio;
pub use radio::*;
pub mod rate;
pub use rate::*;
pub mod select;
pub mod slider;
pub use slider::*;
pub mod switch;
pub use switch::*;
pub use time_picker::*;
pub use transfer::*;
pub use tree_select::*;
pub use upload::*;
pub mod time_picker;
pub mod transfer;
pub mod tree_select;
pub mod upload;

// 数据展示
pub mod avatar;
pub mod badge;
// pub mod calendar;
pub mod card;
pub mod carousel;
pub mod collapse;
// pub mod comment;
pub mod descriptions;
pub mod empty;
pub mod image;
pub mod list;
pub mod popover;
// pub mod segmented;
pub mod statistic;
pub mod table;
pub mod tabs;
pub mod tag;
pub mod timeline;
pub mod tooltip;
// pub mod tour;
pub mod tree;

// 反馈组件
// pub mod alert;
// pub mod drawer;
// pub mod message;
// pub mod modal;
// pub mod notification;
// pub mod popconfirm;
// pub mod progress;
// pub mod result;
// pub mod skeleton;
pub mod spin;

// 其他组件
pub mod anchor;
pub use anchor::*;

pub mod back_top;
pub use back_top::*;

// pub mod config_provider;
// pub mod float_button;
// pub mod qr_code;
// pub mod theme;
// pub mod watermark;

/// 组件的通用属性特征
pub trait ComponentProps {
    /// 组件的 CSS 类名
    fn class_name(&self) -> Option<&str> {
        None
    }

    /// 组件的内联样式
    fn style(&self) -> Option<&str> {
        None
    }

    /// 组件的测试 ID
    fn test_id(&self) -> Option<&str> {
        None
    }
}

/// 尺寸相关的组件属性
pub trait SizeProps {
    /// 组件尺寸
    fn size(&self) -> crate::utils::Size {
        crate::utils::Size::Middle
    }
}

/// 状态相关的组件属性
pub trait StatusProps {
    /// 是否禁用
    fn disabled(&self) -> bool {
        false
    }

    /// 是否加载中
    fn loading(&self) -> bool {
        false
    }
}

/// 可见性相关的组件属性
pub trait VisibilityProps {
    /// 是否可见
    fn visible(&self) -> bool {
        true
    }

    /// 是否隐藏
    fn hidden(&self) -> bool {
        false
    }
}

/// 表单相关的组件属性
pub trait FormProps {
    /// 表单项名称
    fn name(&self) -> Option<&str> {
        None
    }

    /// 是否必填
    fn required(&self) -> bool {
        false
    }

    /// 验证状态
    fn validate_status(&self) -> Option<ValidateStatus> {
        None
    }

    /// 帮助文本
    fn help(&self) -> Option<&str> {
        None
    }
}

/// 验证状态
#[derive(Debug, Clone, PartialEq)]
pub enum ValidateStatus {
    /// 成功
    Success,
    /// 警告
    Warning,
    /// 错误
    Error,
    /// 验证中
    Validating,
}

/// 组件变体类型
#[derive(Debug, Clone, PartialEq)]
pub enum Variant {
    /// 默认
    Default,
    /// 主要
    Primary,
    /// 次要
    Secondary,
    /// 成功
    Success,
    /// 警告
    Warning,
    /// 错误
    Error,
    /// 信息
    Info,
}

/// 组件形状类型
#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    /// 默认
    Default,
    /// 圆形
    Circle,
    /// 圆角
    Round,
}

/// 组件位置类型
#[derive(Debug, Clone, PartialEq)]
pub enum Placement {
    /// 顶部
    Top,
    /// 顶部开始
    TopStart,
    /// 顶部结束
    TopEnd,
    /// 底部
    Bottom,
    /// 底部开始
    BottomStart,
    /// 底部结束
    BottomEnd,
    /// 左侧
    Left,
    /// 左侧开始
    LeftStart,
    /// 左侧结束
    LeftEnd,
    /// 右侧
    Right,
    /// 右侧开始
    RightStart,
    /// 右侧结束
    RightEnd,
}

/// 触发方式
#[derive(Debug, Clone, PartialEq)]
pub enum Trigger {
    /// 点击
    Click,
    /// 悬停
    Hover,
    /// 焦点
    Focus,
    /// 右键点击
    ContextMenu,
    /// 手动
    Manual,
}

/// 对齐方式
#[derive(Debug, Clone, PartialEq)]
pub enum Align {
    /// 左对齐
    Left,
    /// 居中对齐
    Center,
    /// 右对齐
    Right,
    /// 两端对齐
    Justify,
}

/// 垂直对齐方式
#[derive(Debug, Clone, PartialEq)]
pub enum VerticalAlign {
    /// 顶部对齐
    Top,
    /// 居中对齐
    Middle,
    /// 底部对齐
    Bottom,
    /// 基线对齐
    Baseline,
}

/// 方向类型
#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    /// 水平
    Horizontal,
    /// 垂直
    Vertical,
}

/// 组件通用工具函数
pub mod utils {
    use super::*;

    /// 合并 CSS 类名
    pub fn merge_class_names(base: &str, additional: Option<&str>) -> String {
        match additional {
            Some(additional) if !additional.is_empty() => {
                format!("{} {}", base, additional)
            }
            _ => base.to_string(),
        }
    }

    /// 生成组件的基础 CSS 类名
    pub fn get_component_class_name(prefix: &str, component: &str) -> String {
        format!("{}-{}", prefix, component)
    }

    /// 生成带修饰符的 CSS 类名
    pub fn get_modifier_class_name(base: &str, modifier: &str) -> String {
        format!("{}-{}", base, modifier)
    }

    /// 生成状态相关的 CSS 类名
    pub fn get_status_class_names(base: &str, disabled: bool, loading: bool) -> Vec<String> {
        let mut classes = vec![base.to_string()];

        if disabled {
            classes.push(format!("{}-disabled", base));
        }

        if loading {
            classes.push(format!("{}-loading", base));
        }

        classes
    }

    /// 生成尺寸相关的 CSS 类名
    pub fn get_size_class_name(base: &str, size: crate::utils::Size) -> String {
        match size {
            crate::utils::Size::Small => format!("{}-sm", base),
            crate::utils::Size::Middle => base.to_string(),
            crate::utils::Size::Large => format!("{}-lg", base),
        }
    }

    /// 生成变体相关的 CSS 类名
    pub fn get_variant_class_name(base: &str, variant: Variant) -> String {
        match variant {
            Variant::Default => base.to_string(),
            Variant::Primary => format!("{}-primary", base),
            Variant::Secondary => format!("{}-secondary", base),
            Variant::Success => format!("{}-success", base),
            Variant::Warning => format!("{}-warning", base),
            Variant::Error => format!("{}-error", base),
            Variant::Info => format!("{}-info", base),
        }
    }

    /// 生成形状相关的 CSS 类名
    pub fn get_shape_class_name(base: &str, shape: Shape) -> String {
        match shape {
            Shape::Default => base.to_string(),
            Shape::Circle => format!("{}-circle", base),
            Shape::Round => format!("{}-round", base),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::utils::*;
    use super::*;

    #[test]
    fn test_merge_class_names() {
        assert_eq!(merge_class_names("base", None), "base");
        assert_eq!(merge_class_names("base", Some("")), "base");
        assert_eq!(
            merge_class_names("base", Some("additional")),
            "base additional"
        );
    }

    #[test]
    fn test_get_component_class_name() {
        assert_eq!(get_component_class_name("ant", "button"), "ant-button");
    }

    #[test]
    fn test_get_modifier_class_name() {
        assert_eq!(
            get_modifier_class_name("ant-button", "primary"),
            "ant-button-primary"
        );
    }

    #[test]
    fn test_get_status_class_names() {
        let classes = get_status_class_names("ant-button", true, false);
        assert!(classes.contains(&"ant-button".to_string()));
        assert!(classes.contains(&"ant-button-disabled".to_string()));
        assert!(!classes.contains(&"ant-button-loading".to_string()));

        let classes = get_status_class_names("ant-button", false, true);
        assert!(classes.contains(&"ant-button".to_string()));
        assert!(!classes.contains(&"ant-button-disabled".to_string()));
        assert!(classes.contains(&"ant-button-loading".to_string()));
    }

    #[test]
    fn test_get_size_class_name() {
        assert_eq!(
            get_size_class_name("ant-button", crate::utils::Size::Small),
            "ant-button-sm"
        );
        assert_eq!(
            get_size_class_name("ant-button", crate::utils::Size::Middle),
            "ant-button"
        );
        assert_eq!(
            get_size_class_name("ant-button", crate::utils::Size::Large),
            "ant-button-lg"
        );
    }

    #[test]
    fn test_get_variant_class_name() {
        assert_eq!(
            get_variant_class_name("ant-button", Variant::Default),
            "ant-button"
        );
        assert_eq!(
            get_variant_class_name("ant-button", Variant::Primary),
            "ant-button-primary"
        );
        assert_eq!(
            get_variant_class_name("ant-button", Variant::Success),
            "ant-button-success"
        );
    }

    #[test]
    fn test_get_shape_class_name() {
        assert_eq!(
            get_shape_class_name("ant-button", Shape::Default),
            "ant-button"
        );
        assert_eq!(
            get_shape_class_name("ant-button", Shape::Circle),
            "ant-button-circle"
        );
        assert_eq!(
            get_shape_class_name("ant-button", Shape::Round),
            "ant-button-round"
        );
    }
}

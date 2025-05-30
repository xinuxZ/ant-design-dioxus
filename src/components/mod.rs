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
mod affix;
mod alert;
mod anchor;
mod app;
mod auto_complete;
mod avatar;
mod back_top;
mod badge;
mod breadcrumb;
pub mod button;
mod calendar;
mod card;
mod carousel;
mod cascader;
mod checkbox;
mod collapse;
mod color_picker;
mod date_picker;
mod descriptions;
mod divider;
mod drawer;
mod dropdown;
mod empty;
mod flex;
mod float_button;
mod form;
mod grid;
mod icon;
mod image;
mod input;
mod input_number;
mod layout;
mod list;
mod mentions;
mod menu;
mod message;
mod modal;
mod notification;
mod pagination;
mod popconfirm;
mod popover;
mod progress;
mod qr_code;
mod radio;
mod rate;
mod result;
mod segmented;
mod select;
mod skeleton;
mod slider;
mod space;
mod spin;
mod splitter;
mod statistic;
mod steps;
mod switch;
mod table;
// mod tabs;
mod tag;
mod time_picker;
mod timeline;
mod tooltip;
mod tour;
mod transfer;
mod tree;
mod tree_select;
mod typography;
mod upload;
mod watermark;

pub use affix::*;
pub use alert::*;
pub use anchor::*;
pub use app::*;
pub use auto_complete::*;
pub use avatar::*;
pub use back_top::*;
pub use badge::*;
pub use breadcrumb::*;
pub use button::*;
pub use calendar::*;
pub use card::*;
pub use carousel::*;
pub use cascader::*;
pub use checkbox::*;
pub use collapse::*;
pub use color_picker::*;
pub use date_picker::*;
pub use descriptions::*;
pub use divider::*;
pub use drawer::*;
pub use dropdown::*;
pub use empty::*;
pub use flex::*;
pub use float_button::*;
pub use form::*;
pub use grid::*;
pub use icon::*;
pub use image::*;
pub use input::*;
pub use input_number::*;
pub use layout::*;
pub use list::*;
pub use mentions::*;
pub use menu::*;
pub use message::*;
pub use modal::*;
pub use notification::*;
pub use pagination::*;
pub use popconfirm::*;
pub use popover::*;
pub use progress::*;
pub use qr_code::*;
pub use radio::*;
pub use rate::*;
pub use result::*;
pub use segmented::*;
pub use select::*;
pub use skeleton::*;
pub use slider::*;
pub use space::*;
pub use spin::*;
pub use splitter::*;
pub use statistic::*;
pub use steps::*;
pub use switch::*;
pub use table::*;
// pub use tabs::*;
pub use tag::*;
pub use time_picker::*;
pub use timeline::*;
pub use tooltip::*;
pub use tour::*;
pub use transfer::*;
pub use tree::*;
pub use tree_select::*;
pub use typography::*;
pub use upload::*;
pub use watermark::*;

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

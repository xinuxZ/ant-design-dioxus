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
//! - **其他**: Anchor, BackTop, ConfigProvider, FloatButton, QRCode, Theme, ThemeSwitch, Watermark

/// # 主题和语言支持
///
/// 所有组件都支持主题切换和语言切换功能，通过 `ConfigProvider` 组件进行全局配置。
///
/// ## 主题配置示例
///
/// ```rust
/// use ant_design_dioxus::prelude::*;
///
/// #[component]
/// fn App() -> Element {
///     let theme_config = ThemeBuilder::new()
///         .add_token("colorPrimary", "#1890ff")
///         .add_token("colorSuccess", "#52c41a")
///         .add_token("colorWarning", "#faad14")
///         .add_token("colorError", "#ff4d4f")
///         .add_token("colorInfo", "#1890ff")
///         .build();
///
///     rsx! {
///         ConfigProvider {
///             theme: Some(theme_config),
///             div {
///                 Button { "按钮" }
///                 Input { placeholder: "请输入" }
///             }
///         }
///     }
/// }
/// ```
///
/// ## 语言配置示例
///
/// ```rust
/// use ant_design_dioxus::prelude::*;
///
/// #[component]
/// fn App() -> Element {
///     rsx! {
///         ConfigProvider {
///             locale: Some(Locale::En),
///             div {
///                 Button { "Button" }
///                 DatePicker {}
///                 Pagination {
///                     total: 100,
///                     show_size_changer: true
///                 }
///             }
///         }
///     }
/// }
/// ```
///
/// ## 组件中使用主题和语言
///
/// 在自定义组件中，可以通过 Hook 函数获取当前的主题和语言配置：
///
/// ```rust
/// use ant_design_dioxus::prelude::*;
///
/// #[component]
/// fn MyComponent() -> Element {
///     // 获取主题上下文
///     let theme_context = use_theme();
///
///     // 获取特定主题令牌
///     let primary_color = use_theme_token("colorPrimary");
///
///     // 获取组件特定令牌
///     let button_height = use_component_token("Button", "height");
///
///     // 获取翻译函数
///     let translate = use_translate();
///     let ok_text = translate("ok"); // 根据当前语言返回"确定"或"OK"等
///
///     rsx! {
///         div {
///             style: "color: {primary_color}; height: {button_height};",
///             "{ok_text}"
///         }
///     }
/// }
/// ```
// 基础导入在各个组件中按需引入

// 通用组件
pub mod affix;
pub mod alert;
pub mod anchor;
pub mod app;
pub mod auto_complete;
pub mod avatar;
pub mod back_top;
pub mod badge;
pub mod breadcrumb;
pub mod button;
pub mod calendar;
pub mod card;
pub mod carousel;
pub mod cascader;
pub mod checkbox;
pub mod collapse;
pub mod color_picker;
pub mod date_picker;
pub mod descriptions;
pub mod divider;
pub mod drawer;
pub mod dropdown;
pub mod empty;
pub mod flex;
pub mod float_button;
pub mod form;
pub mod grid;
pub mod icon;
pub mod image;
pub mod input;
pub mod input_number;
pub mod layout;
pub mod list;
pub mod mentions;
pub mod menu;
pub mod message;
pub mod modal;
pub mod notification;
pub mod pagination;
pub mod popconfirm;
pub mod popover;
pub mod progress;
pub mod qr_code;
pub mod radio;
pub mod rate;
pub mod result;
pub mod segmented;
pub mod select;
pub mod skeleton;
pub mod slider;
pub mod space;
pub mod spin;
pub mod splitter;
pub mod statistic;
pub mod steps;
pub mod switch;
pub mod table;
pub mod tabs;
pub mod tag;
// pub mod theme_switch;
pub mod time_picker;
pub mod timeline;
pub mod tooltip;
pub mod tour;
pub mod transfer;
pub mod tree;
pub mod tree_select;
pub mod typography;
pub mod upload;
pub mod watermark;

// ================================
// 组件模块导出 - 避免命名冲突
// ================================
//
// 注意：不再使用 `pub use *` 的方式导出所有组件，
// 而是通过 prelude 模块进行分组导出，避免命名冲突。
// 如果需要直接访问组件，请使用完整路径，如：
// use ant_design_dioxus::components::button::Button;
//
// 或者通过 prelude 模块访问：
// use ant_design_dioxus::prelude::general::Button;
// use ant_design_dioxus::prelude::Button; // 常用组件直接导出

// 组件模块已通过 mod 声明公开，无需额外的 pub use
// 用户可以通过以下方式访问组件：
// 1. 直接路径：use ant_design_dioxus::components::button::Button;
// 2. 通过 prelude：use ant_design_dioxus::prelude::Button;
// 3. 通过分组模块：use ant_design_dioxus::prelude::general::Button;

/// 尺寸相关的组件属性
pub trait SizeProps {
    /// 组件尺寸
    fn size(&self) -> crate::theme::core::types::Size {
        crate::theme::core::types::Size::Middle
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
    pub fn get_size_class_name(base: &str, size: crate::theme::core::types::Size) -> String {
        match size {
            crate::theme::core::types::Size::Small => format!("{}-sm", base),
            crate::theme::core::types::Size::Middle => base.to_string(),
            crate::theme::core::types::Size::Large => format!("{}-lg", base),
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
            get_size_class_name("ant-button", crate::theme::core::types::Size::Small),
            "ant-button-sm"
        );
        assert_eq!(
            get_size_class_name("ant-button", crate::theme::core::types::Size::Middle),
            "ant-button"
        );
        assert_eq!(
            get_size_class_name("ant-button", crate::theme::core::types::Size::Large),
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

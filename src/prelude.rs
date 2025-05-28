//! Ant Design Dioxus 预导入模块
//!
//! 提供常用类型和组件的便捷导入，用户只需要 `use ant_design_dioxus::prelude::*;`
//! 即可导入大部分常用的类型和组件。

// 重新导出 Dioxus 核心类型
pub use dioxus::prelude::*;

// 导出核心配置和主题
pub use crate::config_provider::{
    use_config, use_prefix_cls, ComponentSizeConfig, ConfigProvider, ConfigProviderBuilder,
    Direction as ConfigDirection, EmptyConfig, FormConfig, GlobalConfig, LabelAlign, RequiredMark,
    TableConfig, ValidateTrigger,
};

pub use crate::theme::{
    use_theme, ColorTheme, MotionTheme, SizeTheme, Theme, ThemeConfig, ThemeProvider,
};

// 导出国际化
pub use crate::locale::{
    use_locale, use_translate, use_translate_with_args, CurrencyFormat, CurrencySymbolPosition,
    LanguagePack, Locale, LocaleConfig, LocaleProvider, NumberFormat, TranslationKey,
    TranslationValue,
};

// 导出工具类型和函数
pub use crate::utils::{
    is_chinese_char, is_two_cn_char, to_percent, to_px, DebounceCallback, Size, SpaceSize,
    ThrottleCallback,
};

pub use crate::utils::class_names::{
    class_names, class_names, conditional_class_names, ClassNames,
};

pub use crate::utils::responsive::{
    generate_media_query, get_breakpoint_max_width, get_breakpoint_min_width,
    get_current_breakpoint, Breakpoint, GridConfig, ResponsiveValue,
};

pub use crate::utils::color::{
    generate_css_var_name,
    get_contrast_color,
    is_dark_color,
    ColorPalette,
    ColorType,
    HslColor,
    RgbColor,
    // Ant Design 颜色常量
    BLUE,
    CYAN,
    GEEK_BLUE,
    GOLD,
    GREEN,
    LIME,
    MAGENTA,
    ORANGE,
    PINK,
    PURPLE,
    RED,
    VOLCANO,
    YELLOW,
};

pub use crate::utils::motion::{
    generate_keyframes_css,
    AnimationConfig,
    Direction as MotionDirection,
    Duration,
    Easing,
    TransitionType,
    // 预设动画配置
    FADE_IN,
    FADE_OUT,
    MOVE_DOWN,
    MOVE_LEFT,
    MOVE_RIGHT,
    MOVE_UP,
    SLIDE_DOWN,
    SLIDE_LEFT,
    SLIDE_RIGHT,
    SLIDE_UP,
    ZOOM_IN,
    ZOOM_OUT,
};

// 导出组件通用类型
pub use crate::components::{
    Align, ComponentProps, Direction, FormProps, Placement, Shape, SizeProps, StatusProps, Trigger,
    ValidateStatus, Variant, VerticalAlign, VisibilityProps,
};

// 导出组件工具函数
pub use crate::components::utils::{
    get_component_class_name, get_modifier_class_name, get_shape_class_name, get_size_class_name,
    get_status_class_names, get_variant_class_name, merge_class_names,
};

// 当组件实现后，可以在这里导出常用组件
// 目前先注释掉，等组件实现后再启用

// 通用组件
pub use crate::components::button::{
    Button, ButtonGroup, ButtonGroupProps, ButtonGroupSize, ButtonProps, ButtonSize, ButtonType,
};
pub use crate::components::grid::Align as GridAlign;
pub use crate::components::grid::{Col, ColProps, Justify, ResponsiveConfig, Row, RowProps};
pub use crate::components::icon::{Icon, IconProps, IconRotate, IconTheme};
pub use crate::components::layout::{
    Content, ContentProps, Footer, FooterProps, Header, HeaderProps, Layout, LayoutProps, Sider,
    SiderBreakpoint, SiderProps, SiderTheme,
};
pub use crate::components::typography::{
    HeadingLevel, Paragraph, ParagraphProps, Text, TextProps, TextType, Title, TitleProps,
};

// 布局组件
// pub use crate::components::grid::{Row, RowProps, Col, ColProps};
// pub use crate::components::layout::{
//     Layout, LayoutProps, Header, HeaderProps, Content, ContentProps,
//     Footer, FooterProps, Sider, SiderProps,
// };
// pub use crate::components::space::{Space, SpaceProps};
// pub use crate::components::divider::{Divider, DividerProps};

// 导航组件
// pub use crate::components::menu::{Menu, MenuProps, MenuItem, MenuItemProps};
// pub use crate::components::breadcrumb::{Breadcrumb, BreadcrumbProps, BreadcrumbItem};
// pub use crate::components::pagination::{Pagination, PaginationProps};
// pub use crate::components::steps::{Steps, StepsProps, Step, StepProps};

// 数据录入组件
// pub use crate::components::form::{Form, FormProps, FormItem, FormItemProps};
// pub use crate::components::input::{Input, InputProps, TextArea, TextAreaProps};
// pub use crate::components::select::{Select, SelectProps, Option, OptionProps};
// pub use crate::components::checkbox::{Checkbox, CheckboxProps, CheckboxGroup};
// pub use crate::components::radio::{Radio, RadioProps, RadioGroup, RadioGroupProps};
// pub use crate::components::switch::{Switch, SwitchProps};
// pub use crate::components::slider::{Slider, SliderProps};
// pub use crate::components::rate::{Rate, RateProps};
// pub use crate::components::upload::{Upload, UploadProps};

// 数据展示组件
// pub use crate::components::table::{Table, TableProps, Column, ColumnProps};
// pub use crate::components::list::{List, ListProps, ListItem, ListItemProps};
// pub use crate::components::card::{Card, CardProps};
// pub use crate::components::tabs::{Tabs, TabsProps, TabPane, TabPaneProps};
// pub use crate::components::tag::{Tag, TagProps};
// pub use crate::components::badge::{Badge, BadgeProps};
// pub use crate::components::avatar::{Avatar, AvatarProps};
// pub use crate::components::tooltip::{Tooltip, TooltipProps};
// pub use crate::components::popover::{Popover, PopoverProps};
// pub use crate::components::empty::{Empty, EmptyProps};

// 反馈组件
// pub use crate::components::alert::{Alert, AlertProps};
// pub use crate::components::message::{Message, MessageProps, message};
// pub use crate::components::notification::{Notification, NotificationProps, notification};
// pub use crate::components::modal::{Modal, ModalProps};
// pub use crate::components::drawer::{Drawer, DrawerProps};
// pub use crate::components::progress::{Progress, ProgressProps};
// pub use crate::components::spin::{Spin, SpinProps};
// pub use crate::components::skeleton::{Skeleton, SkeletonProps};
// pub use crate::components::result::{Result, ResultProps};

/// 版本信息
pub const VERSION: &str = crate::VERSION;

/// 默认的 CSS 类名前缀
pub const DEFAULT_PREFIX: &str = "ant";

/// 常用的 CSS 类名
pub mod css_classes {
    /// 隐藏元素
    pub const HIDDEN: &str = "ant-hidden";

    /// 禁用状态
    pub const DISABLED: &str = "ant-disabled";

    /// 加载状态
    pub const LOADING: &str = "ant-loading";

    /// 激活状态
    pub const ACTIVE: &str = "ant-active";

    /// 选中状态
    pub const SELECTED: &str = "ant-selected";

    /// 焦点状态
    pub const FOCUSED: &str = "ant-focused";

    /// 悬停状态
    pub const HOVER: &str = "ant-hover";

    /// 错误状态
    pub const ERROR: &str = "ant-error";

    /// 警告状态
    pub const WARNING: &str = "ant-warning";

    /// 成功状态
    pub const SUCCESS: &str = "ant-success";

    /// 信息状态
    pub const INFO: &str = "ant-info";

    /// 小尺寸
    pub const SIZE_SMALL: &str = "ant-sm";

    /// 大尺寸
    pub const SIZE_LARGE: &str = "ant-lg";

    /// 圆形
    pub const CIRCLE: &str = "ant-circle";

    /// 圆角
    pub const ROUND: &str = "ant-round";

    /// 块级元素
    pub const BLOCK: &str = "ant-block";

    /// 内联元素
    pub const INLINE: &str = "ant-inline";

    /// 左对齐
    pub const ALIGN_LEFT: &str = "ant-align-left";

    /// 居中对齐
    pub const ALIGN_CENTER: &str = "ant-align-center";

    /// 右对齐
    pub const ALIGN_RIGHT: &str = "ant-align-right";

    /// 两端对齐
    pub const ALIGN_JUSTIFY: &str = "ant-align-justify";
}

/// 常用的 CSS 变量名
pub mod css_vars {
    /// 主色
    pub const PRIMARY_COLOR: &str = "--ant-primary-color";

    /// 成功色
    pub const SUCCESS_COLOR: &str = "--ant-success-color";

    /// 警告色
    pub const WARNING_COLOR: &str = "--ant-warning-color";

    /// 错误色
    pub const ERROR_COLOR: &str = "--ant-error-color";

    /// 信息色
    pub const INFO_COLOR: &str = "--ant-info-color";

    /// 文本色
    pub const TEXT_COLOR: &str = "--ant-text-color";

    /// 次要文本色
    pub const TEXT_COLOR_SECONDARY: &str = "--ant-text-color-secondary";

    /// 禁用文本色
    pub const TEXT_COLOR_DISABLED: &str = "--ant-text-color-disabled";

    /// 背景色
    pub const BACKGROUND_COLOR: &str = "--ant-background-color";

    /// 边框色
    pub const BORDER_COLOR: &str = "--ant-border-color";

    /// 边框半径
    pub const BORDER_RADIUS: &str = "--ant-border-radius";

    /// 阴影
    pub const BOX_SHADOW: &str = "--ant-box-shadow";

    /// 字体大小
    pub const FONT_SIZE: &str = "--ant-font-size";

    /// 行高
    pub const LINE_HEIGHT: &str = "--ant-line-height";

    /// 动画时长
    pub const ANIMATION_DURATION: &str = "--ant-animation-duration";

    /// 缓动函数
    pub const ANIMATION_EASING: &str = "--ant-animation-easing";
}

/// 常用的断点值
pub mod breakpoints {
    /// 超小屏幕
    pub const XS: u32 = 480;

    /// 小屏幕
    pub const SM: u32 = 576;

    /// 中等屏幕
    pub const MD: u32 = 768;

    /// 大屏幕
    pub const LG: u32 = 992;

    /// 超大屏幕
    pub const XL: u32 = 1200;

    /// 超超大屏幕
    pub const XXL: u32 = 1600;
}

/// 常用的 z-index 值
pub mod z_index {
    /// 下拉菜单
    pub const DROPDOWN: i32 = 1050;

    /// 固定定位
    pub const AFFIX: i32 = 1010;

    /// 回到顶部
    pub const BACK_TOP: i32 = 1010;

    /// 抽屉
    pub const DRAWER: i32 = 1000;

    /// 模态框
    pub const MODAL: i32 = 1000;

    /// 通知
    pub const NOTIFICATION: i32 = 1010;

    /// 消息
    pub const MESSAGE: i32 = 1010;

    /// 气泡确认框
    pub const POPCONFIRM: i32 = 1030;

    /// 气泡卡片
    pub const POPOVER: i32 = 1030;

    /// 工具提示
    pub const TOOLTIP: i32 = 1070;

    /// 图片预览
    pub const IMAGE_PREVIEW: i32 = 1080;
}

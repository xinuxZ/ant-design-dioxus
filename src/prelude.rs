//! Ant Design Dioxus 预导入模块
//!
//! 提供常用类型和组件的便捷导入，用户只需要 `use ant_design_dioxus::prelude::*;`
//! 即可导入大部分常用的类型和组件。
//!
//! ## 设计原则
//!
//! 为了避免命名冲突，本模块采用以下导出策略：
//! 1. 核心配置和主题类型直接导出
//! 2. 组件按模块分组导出，避免类型名冲突
//! 3. 工具函数和常用类型直接导出
//! 4. 组件特定的类型通过模块前缀访问

// ================================
// 核心配置和主题
// ================================

/// 重新导出核心配置相关类型
pub use crate::config_provider::{
    use_config, use_prefix_cls, ComponentSizeConfig, ConfigProvider, ConfigProviderBuilder,
    Direction as ConfigDirection, EmptyConfig, FormConfig, GlobalConfig, LabelAlign, RequiredMark,
    TableConfig, ValidateTrigger,
};

/// 重新导出主题相关类型
pub use crate::theme::{
    use_theme, ColorTheme, MotionTheme, SizeTheme, Theme, ThemeConfig, ThemeProvider,
};

/// 重新导出主题切换组件
pub use crate::components::theme_switch::{ThemeSwitch, ThemeSwitchMode, ThemeSwitchProps};

/// 重新导出国际化相关类型
pub use crate::locale::{
    use_locale, use_translate, use_translate_with_args, CurrencyFormat, CurrencySymbolPosition,
    LanguagePack, Locale, LocaleConfig, LocaleProvider, NumberFormat, TranslationKey,
    TranslationValue,
};

// ================================
// 工具类型和函数
// ================================

/// 重新导出工具类型和函数
pub use crate::utils::{
    is_chinese_char, is_two_cn_char, to_percent, to_px, DebounceCallback, Size, SpaceSize,
    ThrottleCallback,
};

/// 重新导出类名工具
pub use crate::utils::class_names::{
    class_names, conditional_class_names, conditional_class_names_array, ClassNames,
};

/// 重新导出响应式工具
pub use crate::utils::responsive::{
    generate_media_query, get_breakpoint_max_width, get_breakpoint_min_width,
    get_current_breakpoint, Breakpoint, GridConfig, ResponsiveValue,
};

/// 重新导出颜色工具
pub use crate::utils::color::{
    generate_css_var_name, get_contrast_color, is_dark_color, ColorPalette, ColorType, HslColor,
    RgbColor,
};

/// 重新导出动画工具
pub use crate::utils::motion::{
    AnimationConfig, Direction as MotionDirection, Duration, Easing, TransitionType,
};

// ================================
// 组件通用类型
// ================================

/// 重新导出组件通用类型和特征
pub use crate::components::{
    Align, Direction, Placement, Shape, SizeProps, StatusProps, Trigger, ValidateStatus, Variant,
    VerticalAlign, VisibilityProps,
};

/// 重新导出组件工具函数
pub use crate::components::utils::{
    get_component_class_name, get_modifier_class_name, get_shape_class_name, get_size_class_name,
    get_status_class_names, get_variant_class_name, merge_class_names,
};

// ================================
// 组件模块 - 按功能分组导出
// ================================

/// 通用组件
// pub mod general {
pub use crate::components::button::{
    Button, ButtonGroup, ButtonHtmlType, ButtonProps, ButtonShape, ButtonSize, ButtonType,
};
pub use crate::components::icon::{Icon, IconProps, IconTheme};
pub use crate::components::typography::{
    HeadingLevel, Link, LinkProps, Paragraph, ParagraphProps, Text, TextProps, TextType, Title,
    TitleProps,
};
// }

/// 布局组件
// pub mod layout {
pub use crate::components::divider::{Divider, DividerOrientation, DividerProps, DividerType};
pub use crate::components::flex::{Flex, FlexAlign, FlexGap, FlexJustify, FlexProps, FlexWrap};
pub use crate::components::grid::{Col, ColProps, Justify as GridJustify, Row, RowProps};
pub use crate::components::layout::{
    Content, ContentProps, Footer, FooterProps, Header, HeaderProps, Layout, LayoutProps, Sider,
    SiderProps,
};
pub use crate::components::space::{Space, SpaceAlign, SpaceDirection, SpaceProps};
// }

/// 导航组件
// pub mod navigation {
pub use crate::components::affix::{Affix, AffixProps};
pub use crate::components::anchor::{Anchor, AnchorLink, AnchorProps};
pub use crate::components::back_top::{BackTop, BackTopProps};
pub use crate::components::breadcrumb::{
    Breadcrumb, BreadcrumbItem, BreadcrumbItemData, BreadcrumbProps,
};
pub use crate::components::dropdown::{
    Dropdown, DropdownMenuItem, DropdownPlacement, DropdownProps, DropdownTrigger,
};
pub use crate::components::menu::{
    Menu, MenuGroup, MenuItem, MenuItemData, MenuMode, MenuProps, MenuTheme, SubMenu,
};
pub use crate::components::pagination::{
    PageSizeOption, Pagination, PaginationProps, PaginationSize,
};
pub use crate::components::steps::{
    StepItem, StepItemBuilder, StepStatus, Steps, StepsDirection, StepsProps, StepsSize, StepsType,
};
// }

/// 数据录入组件
// pub mod data_entry {
pub use crate::components::auto_complete::{
    AutoComplete, AutoCompleteOption, AutoCompleteProps, AutoCompleteSize, AutoCompleteStatus,
};
pub use crate::components::cascader::{Cascader, CascaderOption, CascaderProps};
pub use crate::components::checkbox::{
    CheckAll, Checkbox, CheckboxGroup, CheckboxOption, CheckboxProps, CheckboxSize,
};
pub use crate::components::color_picker::{ColorPicker, ColorPickerProps};
pub use crate::components::date_picker::{
    DatePicker, DatePickerMode, DatePickerProps, DatePickerSize, DatePickerStatus,
};
pub use crate::components::form::{Form, FormItem, FormLayout, FormProps, FormRule, FormSize};
pub use crate::components::input::{Input, InputProps, InputSize, InputStatus};
pub use crate::components::input_number::{
    InputNumber, InputNumberProps, InputNumberSize, InputNumberStatus, StepInfo,
};
pub use crate::components::mentions::{
    MentionOption, Mentions, MentionsProps, MentionsSize, MentionsStatus,
};
pub use crate::components::radio::{Radio, RadioGroup, RadioOption, RadioProps, RadioSize};
pub use crate::components::rate::{Rate, RateProps, RateSize};
pub use crate::components::select::{
    OptionData, Select, SelectOptGroup, SelectOption, SelectProps, SelectSize, SelectStatus,
};
pub use crate::components::slider::{Slider, SliderMark, SliderProps, SliderSize, SliderStatus};
pub use crate::components::switch::{Switch, SwitchProps, SwitchSize};
pub use crate::components::time_picker::{
    TimeFormat, TimePicker, TimePickerProps, TimePickerSize, TimeValue,
};
pub use crate::components::transfer::{
    Transfer, TransferDirection, TransferItem, TransferListProps, TransferProps,
};
pub use crate::components::tree_select::{TreeSelect, TreeSelectProps};
pub use crate::components::upload::{
    Upload, UploadChangeInfo, UploadFile, UploadListType, UploadProps, UploadStatus,
};
// }

/// 数据展示组件
// pub mod data_display {
pub use crate::components::avatar::{Avatar, AvatarProps, AvatarShape, AvatarSize};
pub use crate::components::badge::{Badge, BadgeProps, BadgeSize, BadgeStatus};
pub use crate::components::calendar::{Calendar, CalendarProps};
pub use crate::components::card::{Card, CardProps};
pub use crate::components::carousel::{
    Carousel, CarouselProps, DotPosition, Effect as CarouselEffect,
};
pub use crate::components::collapse::{Collapse, CollapsePanel, CollapseProps};
pub use crate::components::descriptions::{
    Descriptions, DescriptionsItem, DescriptionsProps, DescriptionsSize,
};
pub use crate::components::empty::{Empty, EmptyProps};
pub use crate::components::image::{Image, ImagePreviewGroup};
pub use crate::components::list::{List, ListItem, ListItemMeta, ListLayout, ListProps, ListSize};
pub use crate::components::popover::{Popover, PopoverPlacement, PopoverProps, PopoverTrigger};
pub use crate::components::qr_code::{QRCode, QRCodeProps};
pub use crate::components::segmented::{Segmented, SegmentedItem, SegmentedOption, SegmentedProps};
pub use crate::components::statistic::{Countdown, Statistic, StatisticPrecision, StatisticProps};
pub use crate::components::table::{
    SelectionType, Table, TableAlign, TableColumn, TableFixed, TablePagination, TableProps,
    TableRowSelection, TableSize,
};
pub use crate::components::tabs::{
    TabItem, Tabs, TabsEditAction, TabsPosition, TabsProps, TabsSize, TabsType,
};
pub use crate::components::tag::{Tag, TagColor, TagProps, TagSize};
pub use crate::components::timeline::{
    Timeline, TimelineItem, TimelineItemColor, TimelineMode, TimelineProps,
};
pub use crate::components::tooltip::{
    Tooltip, TooltipColor, TooltipPlacement, TooltipProps, TooltipTrigger,
};
pub use crate::components::tour::{MaskConfig, Tour, TourProps, TourStep};
pub use crate::components::tree::{Tree, TreeNode, TreeProps};
// }

/// 反馈组件
// pub mod feedback {
pub use crate::components::alert::{Alert, AlertProps, AlertType};
pub use crate::components::drawer::{Drawer, DrawerPlacement, DrawerProps, DrawerSize};
pub use crate::components::message::{message, Message, MessageManager, MessageProps};
pub use crate::components::modal::{Modal, ModalApi, ModalConfig, ModalProps};
pub use crate::components::notification::{
    notification, NotificationConfig, NotificationItem, NotificationManager, NotificationPlacement,
    NotificationProps, NotificationType,
};
pub use crate::components::popconfirm::{Popconfirm, PopconfirmPlacement, PopconfirmProps};
pub use crate::components::progress::{
    Progress, ProgressProps, ProgressSize, ProgressStatus, ProgressType,
};
pub use crate::components::result::{Result, ResultProps, ResultStatus};
pub use crate::components::skeleton::{
    Skeleton, SkeletonButton, SkeletonButtonSize, SkeletonImage, SkeletonInput, SkeletonInputSize,
    SkeletonParagraphProps, SkeletonProps, SkeletonWidth,
};
pub use crate::components::spin::{Spin, SpinProps, SpinSize};
// }

/// 其他组件
// pub mod other {
pub use crate::components::app::{App, AppProps};
pub use crate::components::float_button::{FloatButton, FloatButtonGroup, FloatButtonProps};
pub use crate::components::splitter::{Splitter, SplitterProps};
pub use crate::components::watermark::{Watermark, WatermarkProps};
// }

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

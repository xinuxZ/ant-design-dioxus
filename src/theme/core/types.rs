//! 主题系统核心类型
//!
//! 定义主题系统使用的核心类型和枚举

use serde::{Deserialize, Serialize};

/// 组件尺寸
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Size {
    /// 小尺寸
    Small,
    /// 中等尺寸
    Middle,
    /// 大尺寸
    Large,
}

impl Default for Size {
    fn default() -> Self {
        Size::Middle
    }
}

impl std::fmt::Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Size::Small => write!(f, "small"),
            Size::Middle => write!(f, "middle"),
            Size::Large => write!(f, "large"),
        }
    }
}

/// 颜色类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ColorType {
    /// 主色
    Primary,
    /// 成功色
    Success,
    /// 警告色
    Warning,
    /// 错误色
    Error,
    /// 信息色
    Info,
}

impl std::fmt::Display for ColorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ColorType::Primary => write!(f, "primary"),
            ColorType::Success => write!(f, "success"),
            ColorType::Warning => write!(f, "warning"),
            ColorType::Error => write!(f, "error"),
            ColorType::Info => write!(f, "info"),
        }
    }
}

/// 种子令牌
///
/// Ant Design 设计系统中的基础设计变量
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SeedToken {
    /// 主色
    pub color_primary: String,
    /// 成功色
    pub color_success: String,
    /// 警告色
    pub color_warning: String,
    /// 错误色
    pub color_error: String,
    /// 信息色
    pub color_info: String,
    /// 链接色
    pub color_link: String,
    /// 文本基础色
    pub color_text_base: String,
    /// 背景基础色
    pub color_bg_base: String,
    /// 基础字体大小
    pub font_size: f32,
    /// 基础边框圆角
    pub border_radius: f32,
    /// 控件高度
    pub control_height: f32,
    /// 边框宽度
    pub line_width: f32,
    /// 边框类型
    pub line_type: String,
    /// 动画单位
    pub motion_unit: f32,
    /// 动画基础
    pub motion_base: f32,
    /// 基础z-index
    pub z_index_base: i32,
    /// 弹出层z-index基础
    pub z_index_popup_base: i32,
    /// 图片透明度
    pub opacity_image: f32,
    /// 尺寸单位
    pub size_unit: f32,
    /// 尺寸步长
    pub size_step: f32,
    /// 弹出层箭头大小
    pub size_popup_arrow: f32,
    /// 字体族
    pub font_family: String,
    /// 代码字体族
    pub font_family_code: String,
    /// 线框模式
    pub wireframe: bool,
    /// 是否启用动画
    pub motion: bool,
}

impl Default for SeedToken {
    fn default() -> Self {
        Self {
            color_primary: "#1890ff".to_string(),
            color_success: "#52c41a".to_string(),
            color_warning: "#faad14".to_string(),
            color_error: "#f5222d".to_string(),
            color_info: "#1890ff".to_string(),
            color_link: "#1890ff".to_string(),
            color_text_base: "#000000".to_string(),
            color_bg_base: "#ffffff".to_string(),
            font_size: 14.0,
            border_radius: 2.0,
            control_height: 32.0,
            line_width: 1.0,
            line_type: "solid".to_string(),
            motion_unit: 0.1,
            motion_base: 0.0,
            z_index_base: 0,
            z_index_popup_base: 1000,
            opacity_image: 1.0,
            size_unit: 4.0,
            size_step: 4.0,
            size_popup_arrow: 16.0,
            font_family: "-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Noto Sans', sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol', 'Noto Color Emoji'".to_string(),
            font_family_code: "'SFMono-Regular', Consolas, 'Liberation Mono', Menlo, Courier, monospace".to_string(),
            wireframe: false,
            motion: true,
        }
    }
}

/// 映射令牌
///
/// 从种子令牌派生的设计变量
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapToken {
    // 颜色
    /// 主色调色板
    pub color_primary_palette: Vec<String>,
    /// 成功色调色板
    pub color_success_palette: Vec<String>,
    /// 警告色调色板
    pub color_warning_palette: Vec<String>,
    /// 错误色调色板
    pub color_error_palette: Vec<String>,
    /// 信息色调色板
    pub color_info_palette: Vec<String>,

    // 功能色
    /// 主色
    pub color_primary: String,
    /// 成功色
    pub color_success: String,
    /// 警告色
    pub color_warning: String,
    /// 错误色
    pub color_error: String,
    /// 信息色
    pub color_info: String,
    /// 链接色
    pub color_link: String,

    // 中性色
    /// 背景色
    pub color_bg_base: String,
    /// 文本色
    pub color_text_base: String,

    // 派生颜色
    /// 主要文本色
    pub color_text: String,
    /// 次要文本色
    pub color_text_secondary: String,
    /// 禁用文本色
    pub color_text_disabled: String,
    /// 边框色
    pub color_border: String,
    /// 分割线色
    pub color_split: String,

    // 状态色
    /// 主色悬停状态
    pub color_primary_hover: String,
    /// 主色激活状态
    pub color_primary_active: String,
    /// 主色文本
    pub color_primary_text: String,
    /// 主色文本悬停状态
    pub color_primary_text_hover: String,
    /// 主色文本激活状态
    pub color_primary_text_active: String,
    /// 主色边框
    pub color_primary_border: String,
    /// 主色边框悬停状态
    pub color_primary_border_hover: String,
    /// 主色背景
    pub color_primary_bg: String,
    /// 主色背景悬停状态
    pub color_primary_bg_hover: String,

    /// 次要文本悬停色
    pub color_text_secondary_hover: String,
    /// 次要文本激活色
    pub color_text_secondary_active: String,

    /// 边框悬停色
    pub color_border_hover: String,
    /// 边框激活色
    pub color_border_active: String,

    // 背景色
    /// 容器背景色
    pub color_bg_container: String,
    /// 高层背景色
    pub color_bg_elevated: String,
    /// 布局背景色
    pub color_bg_layout: String,
    /// 遮罩背景色
    pub color_bg_mask: String,
    /// 聚光背景色
    pub color_bg_spotlight: String,
    /// 文本悬停背景色
    pub color_bg_text_hover: String,
    /// 文本激活背景色
    pub color_bg_text_active: String,
    /// 禁用背景色
    pub color_bg_disabled: String,

    // 边框圆角
    /// 基础边框圆角
    pub border_radius_base: f32,
    /// 小号边框圆角
    pub border_radius_sm: f32,
    /// 大号边框圆角
    pub border_radius_lg: f32,
    /// 超大边框圆角
    pub border_radius_xl: f32,
    /// 圆形边框圆角
    pub border_radius_circle: f32,

    // 边框宽度
    /// 基础边框宽度
    pub border_width_base: f32,

    // 尺寸
    /// 基础单位
    pub size_unit: f32,
    /// 尺寸步长
    pub size_step: f32,

    // 控件尺寸
    /// 小号控件高度
    pub control_height_sm: f32,
    /// 标准控件高度
    pub control_height: f32,
    /// 大号控件高度
    pub control_height_lg: f32,

    // 字体大小
    /// 小号字体大小
    pub font_size_sm: f32,
    /// 标准字体大小
    pub font_size: f32,
    /// 大号字体大小
    pub font_size_lg: f32,
    /// 超大号字体大小
    pub font_size_xl: f32,

    // 行高
    /// 标准行高
    pub line_height: f32,
    /// 小号行高
    pub line_height_sm: f32,
    /// 大号行高
    pub line_height_lg: f32,

    // 透明度
    /// 禁用状态透明度
    pub opacity_disabled: f32,

    // 动画
    /// 是否启用动画
    pub motion_enabled: bool,
    /// 动效时长 - 慢速
    pub motion_duration_slow: f32,
    /// 动效时长 - 中速
    pub motion_duration_mid: f32,
    /// 动效时长 - 快速
    pub motion_duration_fast: f32,

    // 阴影
    /// 弹出层阴影
    pub box_shadow_popup: String,
}

impl Default for MapToken {
    fn default() -> Self {
        Self {
            color_primary_palette: vec![
                "#e6f7ff".to_string(),
                "#bae7ff".to_string(),
                "#91d5ff".to_string(),
                "#69c0ff".to_string(),
                "#40a9ff".to_string(),
                "#1890ff".to_string(),
                "#096dd9".to_string(),
                "#0050b3".to_string(),
                "#003a8c".to_string(),
                "#002766".to_string(),
            ],
            color_success_palette: vec![],
            color_warning_palette: vec![],
            color_error_palette: vec![],
            color_info_palette: vec![],

            // 功能色
            color_primary: "#1890ff".to_string(),
            color_success: "#52c41a".to_string(),
            color_warning: "#faad14".to_string(),
            color_error: "#f5222d".to_string(),
            color_info: "#1890ff".to_string(),
            color_link: "#1890ff".to_string(),

            // 中性色
            color_bg_base: "#fff".to_string(),
            color_text_base: "rgba(0, 0, 0, 0.85)".to_string(),

            // 派生颜色
            color_text: "rgba(0, 0, 0, 0.85)".to_string(),
            color_text_secondary: "rgba(0, 0, 0, 0.45)".to_string(),
            color_text_disabled: "rgba(0, 0, 0, 0.25)".to_string(),
            color_border: "#d9d9d9".to_string(),
            color_split: "rgba(0, 0, 0, 0.06)".to_string(),

            // 状态色
            color_primary_hover: "#40a9ff".to_string(),
            color_primary_active: "#096dd9".to_string(),
            color_primary_text: "#1890ff".to_string(),
            color_primary_text_hover: "#40a9ff".to_string(),
            color_primary_text_active: "#096dd9".to_string(),
            color_primary_border: "#1890ff".to_string(),
            color_primary_border_hover: "#40a9ff".to_string(),
            color_primary_bg: "#e6f7ff".to_string(),
            color_primary_bg_hover: "#bae7ff".to_string(),

            color_text_secondary_hover: "rgba(0, 0, 0, 0.65)".to_string(),
            color_text_secondary_active: "rgba(0, 0, 0, 0.85)".to_string(),

            color_border_hover: "#40a9ff".to_string(),
            color_border_active: "#1890ff".to_string(),

            // 背景色
            color_bg_container: "#ffffff".to_string(),
            color_bg_elevated: "#ffffff".to_string(),
            color_bg_layout: "#f5f5f5".to_string(),
            color_bg_mask: "rgba(0, 0, 0, 0.45)".to_string(),
            color_bg_spotlight: "rgba(0, 0, 0, 0.85)".to_string(),
            color_bg_text_hover: "rgba(0, 0, 0, 0.04)".to_string(),
            color_bg_text_active: "rgba(0, 0, 0, 0.08)".to_string(),
            color_bg_disabled: "#f5f5f5".to_string(),

            // 边框圆角
            border_radius_base: 2.0,
            border_radius_sm: 2.0,
            border_radius_lg: 2.0,
            border_radius_xl: 2.0,
            border_radius_circle: 9999.0,

            // 边框宽度
            border_width_base: 1.0,

            // 尺寸
            size_unit: 4.0,
            size_step: 4.0,

            // 控件尺寸
            control_height_sm: 24.0,
            control_height: 32.0,
            control_height_lg: 40.0,

            // 字体大小
            font_size_sm: 12.0,
            font_size: 14.0,
            font_size_lg: 16.0,
            font_size_xl: 20.0,

            // 行高
            line_height: 1.5714285714285714,
            line_height_sm: 1.6666666666666667,
            line_height_lg: 1.5,

            // 透明度
            opacity_disabled: 0.25,

            // 动画
            motion_enabled: true,
            motion_duration_slow: 0.3,
            motion_duration_mid: 0.2,
            motion_duration_fast: 0.1,

            // 阴影
            box_shadow_popup: "0 6px 16px 0 rgba(0, 0, 0, 0.08), 0 3px 6px -4px rgba(0, 0, 0, 0.12), 0 9px 28px 8px rgba(0, 0, 0, 0.05)".to_string(),
        }
    }
}

/// 别名令牌
///
/// 组件使用的特定设计变量，引用映射令牌
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AliasToken {
    // 组件通用
    /// 组件背景色
    pub component_background: String,
    /// 组件文本色
    pub component_text_color: String,
    /// 组件边框色
    pub component_border_color: String,

    // 状态色
    /// 主色
    pub color_primary: String,
    /// 主色悬停状态
    pub color_primary_hover: String,
    /// 主色激活状态
    pub color_primary_active: String,
    /// 主色边框
    pub color_primary_border: String,
    /// 主色边框悬停状态
    pub color_primary_border_hover: String,
    /// 主色背景
    pub color_primary_bg: String,
    /// 主色背景悬停状态
    pub color_primary_bg_hover: String,
    /// 成功色
    pub color_success: String,
    /// 警告色
    pub color_warning: String,
    /// 错误色
    pub color_error: String,
    /// 信息色
    pub color_info: String,

    // 文本色
    /// 主要文本色
    pub color_text: String,
    /// 次要文本色
    pub color_text_secondary: String,
    /// 禁用文本色
    pub color_text_disabled: String,
    /// 主色文本
    pub color_text_primary: String,

    // 背景色
    /// 背景色
    pub color_bg: String,
    /// 容器背景色
    pub color_bg_container: String,
    /// 禁用背景色
    pub color_bg_disabled: String,
    /// 布局背景色
    pub color_bg_layout: String,

    // 边框色
    /// 边框色
    pub color_border: String,
    /// 边框悬停色
    pub color_border_hover: String,
    /// 边框次要色
    pub color_border_secondary: String,

    // 阴影
    /// 基础阴影
    pub box_shadow: String,
    /// 次要阴影
    pub box_shadow_secondary: String,
    /// 三级阴影
    pub box_shadow_tertiary: String,
    /// 弹出层阴影
    pub box_shadow_popup: String,

    // 按钮
    /// 按钮高度（小）
    pub button_height_sm: f32,
    /// 按钮高度（中）
    pub button_height: f32,
    /// 按钮高度（大）
    pub button_height_lg: f32,
    /// 按钮默认背景色
    pub button_default_bg: String,
    /// 按钮默认边框色
    pub button_default_border: String,
    /// 按钮默认文本色
    pub button_default_color: String,
    /// 按钮默认悬停背景色
    pub button_default_hover_bg: String,
    /// 按钮默认悬停边框色
    pub button_default_hover_border: String,
    /// 按钮默认悬停文本色
    pub button_default_hover_color: String,
    /// 按钮主要背景色
    pub button_primary_bg: String,
    /// 按钮主要边框色
    pub button_primary_border: String,
    /// 按钮主要文本色
    pub button_primary_color: String,
    /// 按钮主要悬停背景色
    pub button_primary_hover_bg: String,
    /// 按钮主要悬停边框色
    pub button_primary_hover_border: String,
    /// 按钮主要悬停文本色
    pub button_primary_hover_color: String,
    /// 按钮危险背景色
    pub button_danger_bg: String,
    /// 按钮危险边框色
    pub button_danger_border: String,
    /// 按钮危险文本色
    pub button_danger_color: String,
    /// 按钮危险悬停背景色
    pub button_danger_hover_bg: String,
    /// 按钮危险悬停边框色
    pub button_danger_hover_border: String,
    /// 按钮危险悬停文本色
    pub button_danger_hover_color: String,
    /// 按钮禁用背景色
    pub button_disabled_bg: String,
    /// 按钮禁用边框色
    pub button_disabled_border: String,
    /// 按钮禁用文本色
    pub button_disabled_color: String,

    // 输入框
    /// 输入框高度（小）
    pub input_height_sm: f32,
    /// 输入框高度（中）
    pub input_height: f32,
    /// 输入框高度（大）
    pub input_height_lg: f32,
    /// 输入框背景色
    pub input_bg: String,
    /// 输入框边框色
    pub input_border_color: String,
    /// 输入框文本色
    pub input_color: String,
    /// 输入框悬停边框色
    pub input_hover_border_color: String,
    /// 输入框禁用背景色
    pub input_disabled_bg: String,
    /// 输入框禁用边框色
    pub input_disabled_border_color: String,
    /// 输入框禁用文本色
    pub input_disabled_color: String,
    /// 输入框占位符文本色
    pub input_placeholder_color: String,
    /// 输入框激活状态边框色
    pub input_active_border_color: String,

    // 间距
    /// 基础间距
    pub margin: f32,
    /// 小号间距
    pub margin_xs: f32,
    /// 中小号间距
    pub margin_sm: f32,
    /// 中号间距
    pub margin_md: f32,
    /// 中大号间距
    pub margin_lg: f32,
    /// 大号间距
    pub margin_xl: f32,
    /// 超大号间距
    pub margin_xxl: f32,

    // 内边距
    /// 基础内边距
    pub padding: f32,
    /// 小号内边距
    pub padding_xs: f32,
    /// 中小号内边距
    pub padding_sm: f32,
    /// 中号内边距
    pub padding_md: f32,
    /// 中大号内边距
    pub padding_lg: f32,
    /// 大号内边距
    pub padding_xl: f32,
    /// 超大号内边距
    pub padding_xxl: f32,

    // 边框圆角
    /// 基础边框圆角
    pub border_radius: f32,
    /// 小号边框圆角
    pub border_radius_sm: f32,
    /// 大号边框圆角
    pub border_radius_lg: f32,
    /// 超大边框圆角
    pub border_radius_xl: f32,
    /// 圆形边框圆角
    pub border_radius_circle: f32,

    // 边框宽度
    /// 基础边框宽度
    pub border_width: f32,

    // 字体
    /// 基础字体大小
    pub font_size: f32,
    /// 小号字体大小
    pub font_size_sm: f32,
    /// 大号字体大小
    pub font_size_lg: f32,
    /// 超大号字体大小
    pub font_size_xl: f32,
    /// 基础行高
    pub line_height: f32,
    /// 字体族
    pub font_family: String,
    /// 代码字体族
    pub font_family_code: String,

    // 链接样式
    /// 链接色
    pub link_color: String,
    /// 链接悬停色
    pub link_hover_color: String,
    /// 链接激活色
    pub link_active_color: String,
    /// 链接装饰
    pub link_decoration: String,
    /// 链接悬浮装饰
    pub link_hover_decoration: String,
    /// 链接聚焦装饰
    pub link_focus_decoration: String,

    // 控件相关
    /// 控件交互尺寸
    pub control_interactive_size: f32,
    /// 控件轮廓宽度
    pub control_outline_width: f32,
    /// 控件水平内边距
    pub control_padding_horizontal: f32,
    /// 控件水平内边距（小）
    pub control_padding_horizontal_sm: f32,
    /// 控件垂直内边距
    pub control_padding_vertical: f32,
    /// 控件垂直内边距（小）
    pub control_padding_vertical_sm: f32,

    // 动画
    /// 是否启用动画
    pub motion_enabled: bool,
    /// 动效时长 - 慢速
    pub motion_duration_slow: f32,
    /// 动效时长 - 中速
    pub motion_duration_mid: f32,
    /// 动效时长 - 快速
    pub motion_duration_fast: f32,
}

impl Default for AliasToken {
    fn default() -> Self {
        Self {
            component_background: "#fff".to_string(),
            component_text_color: "rgba(0, 0, 0, 0.85)".to_string(),
            component_border_color: "#d9d9d9".to_string(),

            // 状态色
            color_primary: "#1890ff".to_string(),
            color_primary_hover: "#40a9ff".to_string(),
            color_primary_active: "#096dd9".to_string(),
            color_primary_border: "#1890ff".to_string(),
            color_primary_border_hover: "#40a9ff".to_string(),
            color_primary_bg: "#e6f7ff".to_string(),
            color_primary_bg_hover: "#bae7ff".to_string(),
            color_success: "#52c41a".to_string(),
            color_warning: "#faad14".to_string(),
            color_error: "#f5222d".to_string(),
            color_info: "#1890ff".to_string(),

            // 文本色
            color_text: "rgba(0, 0, 0, 0.85)".to_string(),
            color_text_secondary: "rgba(0, 0, 0, 0.45)".to_string(),
            color_text_disabled: "rgba(0, 0, 0, 0.25)".to_string(),
            color_text_primary: "#1890ff".to_string(),

            // 背景色
            color_bg: "#f5f5f5".to_string(),
            color_bg_container: "#ffffff".to_string(),
            color_bg_disabled: "#f5f5f5".to_string(),
            color_bg_layout: "#f0f2f5".to_string(),

            // 边框色
            color_border: "#d9d9d9".to_string(),
            color_border_hover: "#40a9ff".to_string(),
            color_border_secondary: "#f0f0f0".to_string(),

            box_shadow: "0 6px 16px 0 rgba(0, 0, 0, 0.08), 0 3px 6px -4px rgba(0, 0, 0, 0.12), 0 9px 28px 8px rgba(0, 0, 0, 0.05)".to_string(),
            box_shadow_secondary: "0 6px 16px 0 rgba(0, 0, 0, 0.08), 0 3px 6px -4px rgba(0, 0, 0, 0.12), 0 9px 28px 8px rgba(0, 0, 0, 0.05)".to_string(),
            box_shadow_tertiary: "0 1px 2px 0 rgba(0, 0, 0, 0.03), 0 1px 6px -1px rgba(0, 0, 0, 0.02), 0 2px 4px 0 rgba(0, 0, 0, 0.02)".to_string(),
            box_shadow_popup: "0 6px 16px 0 rgba(0, 0, 0, 0.08), 0 3px 6px -4px rgba(0, 0, 0, 0.12), 0 9px 28px 8px rgba(0, 0, 0, 0.05)".to_string(),

            // 按钮
            button_height_sm: 24.0,
            button_height: 32.0,
            button_height_lg: 40.0,
            button_default_bg: "#fff".to_string(),
            button_default_border: "#d9d9d9".to_string(),
            button_default_color: "rgba(0, 0, 0, 0.85)".to_string(),
            button_default_hover_bg: "#fff".to_string(),
            button_default_hover_border: "#40a9ff".to_string(),
            button_default_hover_color: "#40a9ff".to_string(),
            button_primary_bg: "#1890ff".to_string(),
            button_primary_border: "#1890ff".to_string(),
            button_primary_color: "#fff".to_string(),
            button_primary_hover_bg: "#40a9ff".to_string(),
            button_primary_hover_border: "#40a9ff".to_string(),
            button_primary_hover_color: "#fff".to_string(),
            button_danger_bg: "#ff4d4f".to_string(),
            button_danger_border: "#ff4d4f".to_string(),
            button_danger_color: "#fff".to_string(),
            button_danger_hover_bg: "#ff7875".to_string(),
            button_danger_hover_border: "#ff7875".to_string(),
            button_danger_hover_color: "#fff".to_string(),
            button_disabled_bg: "#f5f5f5".to_string(),
            button_disabled_border: "#d9d9d9".to_string(),
            button_disabled_color: "rgba(0, 0, 0, 0.25)".to_string(),

            // 输入框
            input_height_sm: 24.0,
            input_height: 32.0,
            input_height_lg: 40.0,
            input_bg: "#fff".to_string(),
            input_border_color: "#d9d9d9".to_string(),
            input_color: "rgba(0, 0, 0, 0.85)".to_string(),
            input_hover_border_color: "#40a9ff".to_string(),
            input_disabled_bg: "#f5f5f5".to_string(),
            input_disabled_border_color: "#d9d9d9".to_string(),
            input_disabled_color: "rgba(0, 0, 0, 0.25)".to_string(),
            input_placeholder_color: "rgba(0, 0, 0, 0.25)".to_string(),
            input_active_border_color: "#1890ff".to_string(),

            // 间距
            margin: 16.0,
            margin_xs: 8.0,
            margin_sm: 12.0,
            margin_md: 16.0,
            margin_lg: 24.0,
            margin_xl: 32.0,
            margin_xxl: 48.0,

            // 内边距
            padding: 16.0,
            padding_xs: 8.0,
            padding_sm: 12.0,
            padding_md: 16.0,
            padding_lg: 24.0,
            padding_xl: 32.0,
            padding_xxl: 48.0,

            // 边框圆角
            border_radius: 2.0,
            border_radius_sm: 2.0 * 0.75,
            border_radius_lg: 2.0 * 1.25,
            border_radius_xl: 2.0 * 1.5,
            border_radius_circle: 9999.0,

            // 边框宽度
            border_width: 1.0,

            // 字体
            font_size: 14.0,
            font_size_sm: 12.0,
            font_size_lg: 16.0,
            font_size_xl: 20.0,
            line_height: 1.5714285714285714,
            font_family: "-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Noto Sans', sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol', 'Noto Color Emoji'".to_string(),
            font_family_code: "'SFMono-Regular', Consolas, 'Liberation Mono', Menlo, Courier, monospace".to_string(),

            // 链接样式
            link_color: "#1890ff".to_string(),
            link_hover_color: "#40a9ff".to_string(),
            link_active_color: "#096dd9".to_string(),
            link_decoration: "none".to_string(),
            link_hover_decoration: "none".to_string(),
            link_focus_decoration: "none".to_string(),

            // 控件相关
            control_interactive_size: 16.0,
            control_outline_width: 2.0,
            control_padding_horizontal: 12.0,
            control_padding_horizontal_sm: 8.0,
            control_padding_vertical: 4.0,
            control_padding_vertical_sm: 0.0,

            // 动画
            motion_enabled: true,
            motion_duration_slow: 0.3,
            motion_duration_mid: 0.2,
            motion_duration_fast: 0.1,
        }
    }
}

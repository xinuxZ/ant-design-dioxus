//! 主题算法模块
//!
//! 提供颜色算法和主题派生功能，用于生成完整的主题令牌

use super::color_utils;
use super::core::types::{ColorType, MapToken, SeedToken};

/// 生成颜色调色板
///
/// 基于给定的基色生成10个色阶的调色板
///
/// # 参数
///
/// * `base_color` - 基础色值（HEX格式）
/// * `color_type` - 颜色类型
///
/// # 返回值
///
/// 包含10个色阶的调色板（从浅到深）
pub fn generate_color_palette(base_color: &str, color_type: ColorType) -> Vec<String> {
    // 使用新的颜色工具生成调色板
    match color_type {
        ColorType::Primary => color_utils::generate_ant_palette(base_color)
            .unwrap_or_else(|_| fallback_palette(base_color, color_type)),
        ColorType::Success => color_utils::generate_ant_palette(base_color)
            .unwrap_or_else(|_| fallback_palette(base_color, color_type)),
        ColorType::Warning => color_utils::generate_ant_palette(base_color)
            .unwrap_or_else(|_| fallback_palette(base_color, color_type)),
        ColorType::Error => color_utils::generate_ant_palette(base_color)
            .unwrap_or_else(|_| fallback_palette(base_color, color_type)),
        ColorType::Info => generate_color_palette(base_color, ColorType::Primary),
    }
}

/// 提供备用调色板，以防颜色解析失败
fn fallback_palette(base_color: &str, color_type: ColorType) -> Vec<String> {
    match color_type {
        ColorType::Primary => vec![
            "#e6f7ff".to_string(),
            "#bae7ff".to_string(),
            "#91d5ff".to_string(),
            "#69c0ff".to_string(),
            "#40a9ff".to_string(),
            base_color.to_string(),
            "#096dd9".to_string(),
            "#0050b3".to_string(),
            "#003a8c".to_string(),
            "#002766".to_string(),
        ],
        ColorType::Success => vec![
            "#f6ffed".to_string(),
            "#d9f7be".to_string(),
            "#b7eb8f".to_string(),
            "#95de64".to_string(),
            "#73d13d".to_string(),
            base_color.to_string(),
            "#389e0d".to_string(),
            "#237804".to_string(),
            "#135200".to_string(),
            "#092b00".to_string(),
        ],
        ColorType::Warning => vec![
            "#fffbe6".to_string(),
            "#fff1b8".to_string(),
            "#ffe58f".to_string(),
            "#ffd666".to_string(),
            "#ffc53d".to_string(),
            base_color.to_string(),
            "#d48806".to_string(),
            "#ad6800".to_string(),
            "#874d00".to_string(),
            "#613400".to_string(),
        ],
        ColorType::Error => vec![
            "#fff1f0".to_string(),
            "#ffccc7".to_string(),
            "#ffa39e".to_string(),
            "#ff7875".to_string(),
            "#ff4d4f".to_string(),
            base_color.to_string(),
            "#cf1322".to_string(),
            "#a8071a".to_string(),
            "#820014".to_string(),
            "#5c0011".to_string(),
        ],
        ColorType::Info => generate_color_palette(base_color, ColorType::Primary),
    }
}

/// 亮色主题算法
///
/// 基于种子令牌生成亮色主题的映射令牌
///
/// # 参数
///
/// * `seed` - 种子令牌
///
/// # 返回值
///
/// 生成的映射令牌
pub fn light_algorithm(seed: &SeedToken) -> MapToken {
    let mut map_token = MapToken::default();

    // 生成调色板
    map_token.color_primary_palette =
        generate_color_palette(&seed.color_primary, ColorType::Primary);
    map_token.color_success_palette =
        generate_color_palette(&seed.color_success, ColorType::Success);
    map_token.color_warning_palette =
        generate_color_palette(&seed.color_warning, ColorType::Warning);
    map_token.color_error_palette = generate_color_palette(&seed.color_error, ColorType::Error);
    map_token.color_info_palette = generate_color_palette(&seed.color_info, ColorType::Info);

    // 设置基础功能色
    map_token.color_primary = seed.color_primary.clone();
    map_token.color_success = seed.color_success.clone();
    map_token.color_warning = seed.color_warning.clone();
    map_token.color_error = seed.color_error.clone();
    map_token.color_info = seed.color_info.clone();
    map_token.color_link = seed.color_link.clone();

    // 设置基础颜色
    map_token.color_bg_base = seed.color_bg_base.clone();
    map_token.color_text_base = seed.color_text_base.clone();

    // 派生颜色
    map_token.color_text = seed.color_text_base.clone();
    map_token.color_text_secondary = color_utils::set_alpha(&seed.color_text_base, 0.65)
        .unwrap_or_else(|_| "rgba(0, 0, 0, 0.65)".to_string());
    map_token.color_text_disabled = color_utils::set_alpha(&seed.color_text_base, 0.25)
        .unwrap_or_else(|_| "rgba(0, 0, 0, 0.25)".to_string());
    map_token.color_border = "#d9d9d9".to_string();
    map_token.color_split = color_utils::set_alpha(&seed.color_text_base, 0.06)
        .unwrap_or_else(|_| "rgba(0, 0, 0, 0.06)".to_string());

    // 主色相关状态色
    if let Some(hover_color) = map_token.color_primary_palette.get(4) {
        map_token.color_primary_hover = hover_color.clone();
        map_token.color_primary_bg_hover = hover_color.clone();
    }

    if let Some(active_color) = map_token.color_primary_palette.get(6) {
        map_token.color_primary_active = active_color.clone();
    }

    if let Some(bg_color) = map_token.color_primary_palette.get(0) {
        map_token.color_primary_bg = bg_color.clone();
    }

    // 边框相关状态色
    map_token.color_primary_border = seed.color_primary.clone();
    map_token.color_border_hover = map_token.color_primary_hover.clone();
    map_token.color_border_active = seed.color_primary.clone();

    // 文本相关状态色
    map_token.color_primary_text = seed.color_primary.clone();
    map_token.color_primary_text_hover = map_token.color_primary_hover.clone();
    map_token.color_primary_text_active = map_token.color_primary_active.clone();
    map_token.color_text_secondary_hover = "rgba(0, 0, 0, 0.65)".to_string();
    map_token.color_text_secondary_active = "rgba(0, 0, 0, 0.85)".to_string();

    // 背景色
    map_token.color_bg_container = "#ffffff".to_string();
    map_token.color_bg_elevated = "#ffffff".to_string();
    map_token.color_bg_layout = "#f0f2f5".to_string();
    map_token.color_bg_mask = "rgba(0, 0, 0, 0.45)".to_string();
    map_token.color_bg_spotlight = "rgba(0, 0, 0, 0.85)".to_string();
    map_token.color_bg_text_hover = "rgba(0, 0, 0, 0.04)".to_string();
    map_token.color_bg_text_active = "rgba(0, 0, 0, 0.08)".to_string();
    map_token.color_bg_disabled = "#f5f5f5".to_string();

    // 尺寸
    map_token.size_unit = seed.size_unit;
    map_token.size_step = seed.size_step;

    // 边框宽度
    map_token.border_width_base = seed.line_width;

    // 边框圆角
    map_token.border_radius_base = seed.border_radius;
    map_token.border_radius_sm = seed.border_radius * 0.75;
    map_token.border_radius_lg = seed.border_radius * 1.25;
    map_token.border_radius_xl = seed.border_radius * 1.5;
    map_token.border_radius_circle = 9999.0;

    // 控件高度
    map_token.control_height = seed.control_height;
    map_token.control_height_sm = seed.control_height * 0.75;
    map_token.control_height_lg = seed.control_height * 1.25;

    // 字体大小
    map_token.font_size = seed.font_size;
    map_token.font_size_sm = seed.font_size * 0.85; // 通常是14 -> 12
    map_token.font_size_lg = seed.font_size * 1.14; // 通常是14 -> 16
    map_token.font_size_xl = seed.font_size * 1.43; // 通常是14 -> 20

    // 行高
    map_token.line_height = 1.5714285714285714; // 22/14
    map_token.line_height_sm = 1.6666666666666667; // 20/12
    map_token.line_height_lg = 1.5; // 24/16

    // 动画
    map_token.motion_enabled = seed.motion;
    map_token.motion_duration_slow = 0.3;
    map_token.motion_duration_mid = 0.2;
    map_token.motion_duration_fast = 0.1;

    // 阴影
    map_token.box_shadow_popup = "0 6px 16px 0 rgba(0, 0, 0, 0.08), 0 3px 6px -4px rgba(0, 0, 0, 0.12), 0 9px 28px 8px rgba(0, 0, 0, 0.05)".to_string();

    map_token
}

/// 暗色主题算法
///
/// 基于种子令牌生成暗色主题的映射令牌
///
/// # 参数
///
/// * `seed` - 种子令牌
///
/// # 返回值
///
/// 生成的映射令牌
pub fn dark_algorithm(seed: &SeedToken) -> MapToken {
    let mut map_token = MapToken::default();

    // 生成调色板
    map_token.color_primary_palette =
        generate_color_palette(&seed.color_primary, ColorType::Primary);
    map_token.color_success_palette =
        generate_color_palette(&seed.color_success, ColorType::Success);
    map_token.color_warning_palette =
        generate_color_palette(&seed.color_warning, ColorType::Warning);
    map_token.color_error_palette = generate_color_palette(&seed.color_error, ColorType::Error);
    map_token.color_info_palette = generate_color_palette(&seed.color_info, ColorType::Info);

    // 设置基础功能色
    map_token.color_primary = seed.color_primary.clone();
    map_token.color_success = seed.color_success.clone();
    map_token.color_warning = seed.color_warning.clone();
    map_token.color_error = seed.color_error.clone();
    map_token.color_info = seed.color_info.clone();
    map_token.color_link = seed.color_link.clone();

    // 设置基础颜色
    map_token.color_bg_base = "#141414".to_string();
    map_token.color_text_base = "rgba(255, 255, 255, 0.85)".to_string();

    // 派生颜色
    map_token.color_text = "rgba(255, 255, 255, 0.85)".to_string();
    map_token.color_text_secondary = "rgba(255, 255, 255, 0.45)".to_string();
    map_token.color_text_disabled = "rgba(255, 255, 255, 0.25)".to_string();
    map_token.color_border = "#434343".to_string();
    map_token.color_split = "rgba(255, 255, 255, 0.12)".to_string();

    // 主色相关状态色
    if let Some(hover_color) = map_token.color_primary_palette.get(4) {
        map_token.color_primary_hover = hover_color.clone();
        map_token.color_primary_bg_hover = hover_color.clone();
    }

    if let Some(active_color) = map_token.color_primary_palette.get(6) {
        map_token.color_primary_active = active_color.clone();
    }

    if let Some(bg_color) = map_token.color_primary_palette.get(0) {
        map_token.color_primary_bg = color_utils::set_alpha(bg_color, 0.2)
            .unwrap_or_else(|_| "rgba(24, 144, 255, 0.2)".to_string());
    }

    // 边框相关状态色
    map_token.color_primary_border = seed.color_primary.clone();
    map_token.color_border_hover = map_token.color_primary_hover.clone();
    map_token.color_border_active = seed.color_primary.clone();

    // 文本相关状态色
    map_token.color_primary_text = seed.color_primary.clone();
    map_token.color_primary_text_hover = map_token.color_primary_hover.clone();
    map_token.color_primary_text_active = map_token.color_primary_active.clone();
    map_token.color_text_secondary_hover = "rgba(255, 255, 255, 0.65)".to_string();
    map_token.color_text_secondary_active = "rgba(255, 255, 255, 0.85)".to_string();

    // 背景色
    map_token.color_bg_container = "#141414".to_string();
    map_token.color_bg_elevated = "#1f1f1f".to_string();
    map_token.color_bg_layout = "#000000".to_string();
    map_token.color_bg_mask = "rgba(0, 0, 0, 0.45)".to_string();
    map_token.color_bg_spotlight = "rgba(0, 0, 0, 0.85)".to_string();
    map_token.color_bg_text_hover = "rgba(255, 255, 255, 0.08)".to_string();
    map_token.color_bg_text_active = "rgba(255, 255, 255, 0.16)".to_string();
    map_token.color_bg_disabled = "rgba(255, 255, 255, 0.08)".to_string();

    // 尺寸
    map_token.size_unit = seed.size_unit;
    map_token.size_step = seed.size_step;

    // 边框宽度
    map_token.border_width_base = seed.line_width;

    // 边框圆角
    map_token.border_radius_base = seed.border_radius;
    map_token.border_radius_sm = seed.border_radius * 0.75;
    map_token.border_radius_lg = seed.border_radius * 1.25;
    map_token.border_radius_xl = seed.border_radius * 1.5;
    map_token.border_radius_circle = 9999.0;

    // 控件高度
    map_token.control_height = seed.control_height;
    map_token.control_height_sm = seed.control_height * 0.75;
    map_token.control_height_lg = seed.control_height * 1.25;

    // 字体大小
    map_token.font_size = seed.font_size;
    map_token.font_size_sm = seed.font_size * 0.85; // 通常是14 -> 12
    map_token.font_size_lg = seed.font_size * 1.14; // 通常是14 -> 16
    map_token.font_size_xl = seed.font_size * 1.43; // 通常是14 -> 20

    // 行高
    map_token.line_height = 1.5714285714285714; // 22/14
    map_token.line_height_sm = 1.6666666666666667; // 20/12
    map_token.line_height_lg = 1.5; // 24/16

    // 动画
    map_token.motion_enabled = seed.motion;
    map_token.motion_duration_slow = 0.3;
    map_token.motion_duration_mid = 0.2;
    map_token.motion_duration_fast = 0.1;

    // 阴影
    map_token.box_shadow_popup = "0 6px 16px 0 rgba(0, 0, 0, 0.85), 0 3px 6px -4px rgba(0, 0, 0, 0.85), 0 9px 28px 8px rgba(0, 0, 0, 0.65)".to_string();

    map_token
}

/// 紧凑算法
///
/// 基于映射令牌应用紧凑模式的调整
///
/// # 参数
///
/// * `map_token` - 映射令牌
///
/// # 返回值
///
/// 调整后的映射令牌
pub fn compact_algorithm(map_token: &mut MapToken) {
    // 调整尺寸相关的令牌
    map_token.size_unit = 4.0;
    map_token.size_step = 2.0;
}

/// 颜色调整：变亮
///
/// # 参数
///
/// * `color` - 原始颜色（HEX格式）
/// * `amount` - 变亮程度（0.0-1.0）
///
/// # 返回值
///
/// 调整后的颜色（HEX格式）
pub fn lighten_color(color: &str, amount: f32) -> String {
    color_utils::lighten(color, amount).unwrap_or_else(|_| color.to_string())
}

/// 颜色调整：变暗
///
/// # 参数
///
/// * `color` - 原始颜色（HEX格式）
/// * `amount` - 变暗程度（0.0-1.0）
///
/// # 返回值
///
/// 调整后的颜色（HEX格式）
pub fn darken_color(color: &str, amount: f32) -> String {
    color_utils::darken(color, amount).unwrap_or_else(|_| color.to_string())
}

/// 颜色调整：透明度
///
/// # 参数
///
/// * `color` - 原始颜色（HEX格式）
/// * `alpha` - 透明度（0.0-1.0）
///
/// # 返回值
///
/// 调整后的颜色（RGBA格式）
pub fn set_alpha_color(color: &str, alpha: f32) -> String {
    color_utils::set_alpha(color, alpha).unwrap_or_else(|_| format!("rgba(0, 0, 0, {})", alpha))
}

/// 颜色调整：饱和度
///
/// # 参数
///
/// * `color` - 原始颜色（HEX格式）
/// * `amount` - 饱和度调整（-1.0-1.0）
///
/// # 返回值
///
/// 调整后的颜色（HEX格式）
pub fn adjust_saturation(color: &str, amount: f32) -> String {
    color_utils::adjust_saturation(color, amount).unwrap_or_else(|_| color.to_string())
}

/// 组件级算法类型
pub enum ComponentAlgorithm {
    /// 按钮组件算法
    Button,
    /// 输入框组件算法
    Input,
    /// 选择器组件算法
    Select,
    /// 菜单组件算法
    Menu,
    /// 表格组件算法
    Table,
    /// 卡片组件算法
    Card,
    /// 模态框组件算法
    Modal,
    /// 消息组件算法
    Message,
    /// 标签组件算法
    Tag,
    /// 分页组件算法
    Pagination,
    /// 弹出框组件算法
    Popover,
    /// 抽屉组件算法
    Drawer,
    /// 下拉菜单组件算法
    Dropdown,
    /// 表单组件算法
    Form,
    /// 步骤条组件算法
    Steps,
    /// 折叠面板组件算法
    Collapse,
    /// 滑动输入组件算法
    Slider,
    /// 开关组件算法
    Switch,
    /// 日期选择器组件算法
    DatePicker,
    /// 通知组件算法
    Notification,
}

impl std::fmt::Display for ComponentAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComponentAlgorithm::Button => write!(f, "button"),
            ComponentAlgorithm::Input => write!(f, "input"),
            ComponentAlgorithm::Select => write!(f, "select"),
            ComponentAlgorithm::Menu => write!(f, "menu"),
            ComponentAlgorithm::Table => write!(f, "table"),
            ComponentAlgorithm::Card => write!(f, "card"),
            ComponentAlgorithm::Modal => write!(f, "modal"),
            ComponentAlgorithm::Message => write!(f, "message"),
            ComponentAlgorithm::Tag => write!(f, "tag"),
            ComponentAlgorithm::Pagination => write!(f, "pagination"),
            ComponentAlgorithm::Popover => write!(f, "popover"),
            ComponentAlgorithm::Drawer => write!(f, "drawer"),
            ComponentAlgorithm::Dropdown => write!(f, "dropdown"),
            ComponentAlgorithm::Form => write!(f, "form"),
            ComponentAlgorithm::Steps => write!(f, "steps"),
            ComponentAlgorithm::Collapse => write!(f, "collapse"),
            ComponentAlgorithm::Slider => write!(f, "slider"),
            ComponentAlgorithm::Switch => write!(f, "switch"),
            ComponentAlgorithm::DatePicker => write!(f, "datePicker"),
            ComponentAlgorithm::Notification => write!(f, "notification"),
        }
    }
}

/// 组件级算法
///
/// 基于全局令牌和组件特定算法生成组件级令牌
///
/// # 参数
///
/// * `component` - 组件算法类型
/// * `seed` - 种子令牌
/// * `map_token` - 全局映射令牌
/// * `is_dark` - 是否是暗色主题
///
/// # 返回值
///
/// 组件级令牌的键值对
pub fn component_algorithm(
    component: ComponentAlgorithm,
    seed: &SeedToken,
    map_token: &MapToken,
    is_dark: bool,
) -> std::collections::HashMap<String, String> {
    let mut tokens = std::collections::HashMap::new();

    match component {
        ComponentAlgorithm::Button => {
            // 基础按钮令牌
            tokens.insert("height".to_string(), map_token.control_height.to_string());
            tokens.insert(
                "heightSm".to_string(),
                map_token.control_height_sm.to_string(),
            );
            tokens.insert(
                "heightLg".to_string(),
                map_token.control_height_lg.to_string(),
            );
            tokens.insert(
                "paddingHorizontal".to_string(),
                format!(
                    "{}px",
                    map_token.control_height / 2.0 - map_token.font_size / 2.0
                ),
            );
            tokens.insert(
                "paddingHorizontalSm".to_string(),
                format!(
                    "{}px",
                    map_token.control_height_sm / 2.0 - map_token.font_size_sm / 2.0
                ),
            );
            tokens.insert(
                "paddingHorizontalLg".to_string(),
                format!(
                    "{}px",
                    map_token.control_height_lg / 2.0 - map_token.font_size_lg / 2.0
                ),
            );

            // 默认按钮
            tokens.insert(
                "defaultBg".to_string(),
                if is_dark {
                    "transparent".to_string()
                } else {
                    "#fff".to_string()
                },
            );
            tokens.insert("defaultBorder".to_string(), map_token.color_border.clone());
            tokens.insert("defaultColor".to_string(), map_token.color_text.clone());
            tokens.insert(
                "defaultHoverBg".to_string(),
                if is_dark {
                    "transparent".to_string()
                } else {
                    "#fff".to_string()
                },
            );
            tokens.insert(
                "defaultHoverBorder".to_string(),
                map_token.color_primary_hover.clone(),
            );
            tokens.insert(
                "defaultHoverColor".to_string(),
                map_token.color_primary_hover.clone(),
            );
            tokens.insert(
                "defaultActiveBg".to_string(),
                if is_dark {
                    "transparent".to_string()
                } else {
                    "#fff".to_string()
                },
            );
            tokens.insert(
                "defaultActiveBorder".to_string(),
                map_token.color_primary_active.clone(),
            );
            tokens.insert(
                "defaultActiveColor".to_string(),
                map_token.color_primary_active.clone(),
            );

            // 主要按钮
            tokens.insert("primaryBg".to_string(), map_token.color_primary.clone());
            tokens.insert("primaryBorder".to_string(), map_token.color_primary.clone());
            tokens.insert("primaryColor".to_string(), "#fff".to_string());
            tokens.insert(
                "primaryHoverBg".to_string(),
                map_token.color_primary_hover.clone(),
            );
            tokens.insert(
                "primaryHoverBorder".to_string(),
                map_token.color_primary_hover.clone(),
            );
            tokens.insert("primaryHoverColor".to_string(), "#fff".to_string());
            tokens.insert(
                "primaryActiveBg".to_string(),
                map_token.color_primary_active.clone(),
            );
            tokens.insert(
                "primaryActiveBorder".to_string(),
                map_token.color_primary_active.clone(),
            );
            tokens.insert("primaryActiveColor".to_string(), "#fff".to_string());

            // 危险按钮
            tokens.insert("dangerBg".to_string(), map_token.color_error.clone());
            tokens.insert("dangerBorder".to_string(), map_token.color_error.clone());
            tokens.insert("dangerColor".to_string(), "#fff".to_string());

            if let Some(error_hover) = map_token.color_error_palette.get(4) {
                tokens.insert("dangerHoverBg".to_string(), error_hover.clone());
                tokens.insert("dangerHoverBorder".to_string(), error_hover.clone());
            } else {
                tokens.insert("dangerHoverBg".to_string(), "#ff7875".to_string());
                tokens.insert("dangerHoverBorder".to_string(), "#ff7875".to_string());
            }
            tokens.insert("dangerHoverColor".to_string(), "#fff".to_string());

            if let Some(error_active) = map_token.color_error_palette.get(6) {
                tokens.insert("dangerActiveBg".to_string(), error_active.clone());
                tokens.insert("dangerActiveBorder".to_string(), error_active.clone());
            } else {
                tokens.insert("dangerActiveBg".to_string(), "#d9363e".to_string());
                tokens.insert("dangerActiveBorder".to_string(), "#d9363e".to_string());
            }
            tokens.insert("dangerActiveColor".to_string(), "#fff".to_string());

            // 禁用按钮
            tokens.insert(
                "disabledBg".to_string(),
                map_token.color_bg_disabled.clone(),
            );
            tokens.insert("disabledBorder".to_string(), map_token.color_border.clone());
            tokens.insert(
                "disabledColor".to_string(),
                map_token.color_text_disabled.clone(),
            );
        }
        ComponentAlgorithm::Input => {
            // 基础输入框令牌
            tokens.insert("height".to_string(), map_token.control_height.to_string());
            tokens.insert(
                "heightSm".to_string(),
                map_token.control_height_sm.to_string(),
            );
            tokens.insert(
                "heightLg".to_string(),
                map_token.control_height_lg.to_string(),
            );
            tokens.insert("paddingHorizontal".to_string(), "11px".to_string());
            tokens.insert("paddingVertical".to_string(), "4px".to_string());

            // 颜色和边框
            tokens.insert(
                "bg".to_string(),
                if is_dark {
                    "transparent".to_string()
                } else {
                    "#fff".to_string()
                },
            );
            tokens.insert("border".to_string(), map_token.color_border.clone());
            tokens.insert("color".to_string(), map_token.color_text.clone());
            tokens.insert(
                "hoverBorder".to_string(),
                map_token.color_primary_hover.clone(),
            );
            tokens.insert("activeBorder".to_string(), map_token.color_primary.clone());
            tokens.insert(
                "placeholderColor".to_string(),
                map_token.color_text_disabled.clone(),
            );

            // 禁用状态
            tokens.insert(
                "disabledBg".to_string(),
                map_token.color_bg_disabled.clone(),
            );
            tokens.insert("disabledBorder".to_string(), map_token.color_border.clone());
            tokens.insert(
                "disabledColor".to_string(),
                map_token.color_text_disabled.clone(),
            );
        }
        ComponentAlgorithm::Select => {
            // 基础选择器令牌
            tokens.insert("height".to_string(), map_token.control_height.to_string());
            tokens.insert(
                "heightSm".to_string(),
                map_token.control_height_sm.to_string(),
            );
            tokens.insert(
                "heightLg".to_string(),
                map_token.control_height_lg.to_string(),
            );
            tokens.insert("paddingHorizontal".to_string(), "11px".to_string());

            // 颜色和边框
            tokens.insert(
                "bg".to_string(),
                if is_dark {
                    "transparent".to_string()
                } else {
                    "#fff".to_string()
                },
            );
            tokens.insert("border".to_string(), map_token.color_border.clone());
            tokens.insert("color".to_string(), map_token.color_text.clone());
            tokens.insert(
                "hoverBorder".to_string(),
                map_token.color_primary_hover.clone(),
            );
            tokens.insert("activeBorder".to_string(), map_token.color_primary.clone());

            // 下拉菜单
            tokens.insert(
                "dropdownBg".to_string(),
                map_token.color_bg_container.clone(),
            );
            tokens.insert(
                "itemSelectedBg".to_string(),
                map_token.color_primary_bg.clone(),
            );
            tokens.insert(
                "itemSelectedColor".to_string(),
                map_token.color_primary.clone(),
            );
            tokens.insert(
                "itemHoverBg".to_string(),
                if is_dark {
                    "#303030".to_string()
                } else {
                    "#f5f5f5".to_string()
                },
            );

            // 禁用状态
            tokens.insert(
                "disabledBg".to_string(),
                map_token.color_bg_disabled.clone(),
            );
            tokens.insert("disabledBorder".to_string(), map_token.color_border.clone());
            tokens.insert(
                "disabledColor".to_string(),
                map_token.color_text_disabled.clone(),
            );
        }
        ComponentAlgorithm::Menu => {
            // 基础菜单令牌
            tokens.insert("bg".to_string(), map_token.color_bg_container.clone());
            tokens.insert("itemColor".to_string(), map_token.color_text.clone());
            tokens.insert("itemHeight".to_string(), "40px".to_string());
            tokens.insert("horizontalItemHeight".to_string(), "46px".to_string());
            tokens.insert("itemPaddingHorizontal".to_string(), "20px".to_string());

            // 激活和选中状态
            tokens.insert(
                "itemActiveBg".to_string(),
                map_token.color_primary_bg.clone(),
            );
            tokens.insert(
                "itemActiveColor".to_string(),
                map_token.color_primary.clone(),
            );
            tokens.insert(
                "itemSelectedBg".to_string(),
                map_token.color_primary_bg.clone(),
            );
            tokens.insert(
                "itemSelectedColor".to_string(),
                map_token.color_primary.clone(),
            );

            // 分组和折叠
            tokens.insert(
                "groupTitleColor".to_string(),
                map_token.color_text_secondary.clone(),
            );
            tokens.insert("collapsedWidth".to_string(), "80px".to_string());

            // 暗色模式特殊处理
            if is_dark {
                tokens.insert(
                    "darkItemColor".to_string(),
                    "rgba(255, 255, 255, 0.65)".to_string(),
                );
                tokens.insert(
                    "darkItemSelectedBg".to_string(),
                    map_token.color_primary.clone(),
                );
                tokens.insert("darkItemSelectedColor".to_string(), "#fff".to_string());
                tokens.insert(
                    "darkDangerItemColor".to_string(),
                    map_token.color_error.clone(),
                );
                tokens.insert(
                    "darkDangerItemSelectedBg".to_string(),
                    map_token.color_error.clone(),
                );
            }
        }
        ComponentAlgorithm::Table => {
            // 基础表格令牌
            tokens.insert("bg".to_string(), map_token.color_bg_container.clone());
            tokens.insert("border".to_string(), map_token.color_border.clone());
            tokens.insert("color".to_string(), map_token.color_text.clone());

            // 表头
            tokens.insert(
                "headerBg".to_string(),
                if is_dark {
                    "#1d1d1d".to_string()
                } else {
                    "#fafafa".to_string()
                },
            );
            tokens.insert("headerColor".to_string(), map_token.color_text.clone());
            tokens.insert(
                "headerSortActiveBg".to_string(),
                if is_dark {
                    "#303030".to_string()
                } else {
                    "#f5f5f5".to_string()
                },
            );

            // 表体
            tokens.insert(
                "rowHoverBg".to_string(),
                if is_dark {
                    "#262626".to_string()
                } else {
                    "#fafafa".to_string()
                },
            );
            tokens.insert(
                "selectedRowBg".to_string(),
                map_token.color_primary_bg.clone(),
            );
            tokens.insert(
                "expandedRowBg".to_string(),
                if is_dark {
                    "#1d1d1d".to_string()
                } else {
                    "#fbfbfb".to_string()
                },
            );
        }
        ComponentAlgorithm::Card => {
            // 基础卡片令牌
            tokens.insert("bg".to_string(), map_token.color_bg_container.clone());
            tokens.insert("color".to_string(), map_token.color_text.clone());
            tokens.insert("border".to_string(), map_token.color_border.clone());
            tokens.insert(
                "borderRadius".to_string(),
                format!("{}px", map_token.border_radius_base),
            );

            // 卡片头部
            tokens.insert(
                "headBg".to_string(),
                if is_dark {
                    "transparent".to_string()
                } else {
                    "transparent".to_string()
                },
            );
            tokens.insert("headColor".to_string(), map_token.color_text.clone());
            tokens.insert(
                "headPadding".to_string(),
                format!("{}px {}px", map_token.padding_sm, map_token.padding_lg),
            );

            // 卡片内容
            tokens.insert("padding".to_string(), format!("{}px", map_token.padding_lg));
            tokens.insert(
                "actionsBg".to_string(),
                if is_dark {
                    "#1d1d1d".to_string()
                } else {
                    "#fafafa".to_string()
                },
            );
        }
        ComponentAlgorithm::Modal => {
            // 基础模态框令牌
            tokens.insert("bg".to_string(), map_token.color_bg_container.clone());
            tokens.insert("color".to_string(), map_token.color_text.clone());
            tokens.insert(
                "borderRadius".to_string(),
                format!("{}px", map_token.border_radius_base),
            );
            tokens.insert("padding".to_string(), format!("{}px", map_token.padding_lg));

            // 模态框头部
            tokens.insert("headerBg".to_string(), map_token.color_bg_container.clone());
            tokens.insert("headerColor".to_string(), map_token.color_text.clone());
            tokens.insert(
                "headerPadding".to_string(),
                format!("{}px {}px", map_token.padding_md, map_token.padding_lg),
            );
            tokens.insert(
                "headerBorderColor".to_string(),
                if is_dark {
                    "#303030".to_string()
                } else {
                    "#f0f0f0".to_string()
                },
            );

            // 模态框底部
            tokens.insert("footerBg".to_string(), "transparent".to_string());
            tokens.insert(
                "footerBorderColor".to_string(),
                if is_dark {
                    "#303030".to_string()
                } else {
                    "#f0f0f0".to_string()
                },
            );
            tokens.insert(
                "footerPadding".to_string(),
                format!("{}px {}px", map_token.padding_md, map_token.padding_lg),
            );

            // 模态框遮罩
            tokens.insert("maskBg".to_string(), "rgba(0, 0, 0, 0.45)".to_string());
        }
        ComponentAlgorithm::Message => {
            // 基础消息令牌
            tokens.insert("bg".to_string(), map_token.color_bg_container.clone());
            tokens.insert("color".to_string(), map_token.color_text.clone());
            tokens.insert(
                "padding".to_string(),
                format!("{}px {}px", map_token.padding_sm, map_token.padding_md),
            );

            // 消息类型
            if let Some(info_bg) = map_token.color_info_palette.get(0) {
                tokens.insert(
                    "infoBg".to_string(),
                    if is_dark {
                        color_utils::set_alpha(info_bg, 0.15)
                            .unwrap_or_else(|_| "rgba(24, 144, 255, 0.15)".to_string())
                    } else {
                        info_bg.clone()
                    },
                );
            } else {
                tokens.insert(
                    "infoBg".to_string(),
                    if is_dark {
                        "rgba(24, 144, 255, 0.15)".to_string()
                    } else {
                        "#e6f7ff".to_string()
                    },
                );
            }

            if let Some(success_bg) = map_token.color_success_palette.get(0) {
                tokens.insert(
                    "successBg".to_string(),
                    if is_dark {
                        color_utils::set_alpha(success_bg, 0.15)
                            .unwrap_or_else(|_| "rgba(82, 196, 26, 0.15)".to_string())
                    } else {
                        success_bg.clone()
                    },
                );
            } else {
                tokens.insert(
                    "successBg".to_string(),
                    if is_dark {
                        "rgba(82, 196, 26, 0.15)".to_string()
                    } else {
                        "#f6ffed".to_string()
                    },
                );
            }

            if let Some(warning_bg) = map_token.color_warning_palette.get(0) {
                tokens.insert(
                    "warningBg".to_string(),
                    if is_dark {
                        color_utils::set_alpha(warning_bg, 0.15)
                            .unwrap_or_else(|_| "rgba(250, 173, 20, 0.15)".to_string())
                    } else {
                        warning_bg.clone()
                    },
                );
            } else {
                tokens.insert(
                    "warningBg".to_string(),
                    if is_dark {
                        "rgba(250, 173, 20, 0.15)".to_string()
                    } else {
                        "#fffbe6".to_string()
                    },
                );
            }

            if let Some(error_bg) = map_token.color_error_palette.get(0) {
                tokens.insert(
                    "errorBg".to_string(),
                    if is_dark {
                        color_utils::set_alpha(error_bg, 0.15)
                            .unwrap_or_else(|_| "rgba(245, 34, 45, 0.15)".to_string())
                    } else {
                        error_bg.clone()
                    },
                );
            } else {
                tokens.insert(
                    "errorBg".to_string(),
                    if is_dark {
                        "rgba(245, 34, 45, 0.15)".to_string()
                    } else {
                        "#fff1f0".to_string()
                    },
                );
            }
        }
        ComponentAlgorithm::Tag => {
            // 基础标签令牌
            tokens.insert(
                "bg".to_string(),
                if is_dark {
                    "rgba(255, 255, 255, 0.04)".to_string()
                } else {
                    "#fafafa".to_string()
                },
            );
            tokens.insert("color".to_string(), map_token.color_text.clone());
            tokens.insert(
                "border".to_string(),
                if is_dark {
                    "#434343".to_string()
                } else {
                    "#d9d9d9".to_string()
                },
            );
            tokens.insert(
                "borderRadius".to_string(),
                format!("{}px", map_token.border_radius_base),
            );
            tokens.insert("padding".to_string(), "0 7px".to_string());

            // 标签类型
            tokens.insert("primaryBg".to_string(), map_token.color_primary_bg.clone());
            tokens.insert("primaryColor".to_string(), map_token.color_primary.clone());
            tokens.insert(
                "primaryBorder".to_string(),
                map_token.color_primary_border.clone(),
            );

            tokens.insert(
                "successBg".to_string(),
                if is_dark {
                    "rgba(82, 196, 26, 0.1)".to_string()
                } else {
                    "#f6ffed".to_string()
                },
            );
            tokens.insert("successColor".to_string(), map_token.color_success.clone());
            tokens.insert(
                "successBorder".to_string(),
                if is_dark {
                    "rgba(82, 196, 26, 0.2)".to_string()
                } else {
                    "#b7eb8f".to_string()
                },
            );

            tokens.insert(
                "warningBg".to_string(),
                if is_dark {
                    "rgba(250, 173, 20, 0.1)".to_string()
                } else {
                    "#fffbe6".to_string()
                },
            );
            tokens.insert("warningColor".to_string(), map_token.color_warning.clone());
            tokens.insert(
                "warningBorder".to_string(),
                if is_dark {
                    "rgba(250, 173, 20, 0.2)".to_string()
                } else {
                    "#ffe58f".to_string()
                },
            );

            tokens.insert(
                "errorBg".to_string(),
                if is_dark {
                    "rgba(245, 34, 45, 0.1)".to_string()
                } else {
                    "#fff1f0".to_string()
                },
            );
            tokens.insert("errorColor".to_string(), map_token.color_error.clone());
            tokens.insert(
                "errorBorder".to_string(),
                if is_dark {
                    "rgba(245, 34, 45, 0.2)".to_string()
                } else {
                    "#ffa39e".to_string()
                },
            );
        }
        ComponentAlgorithm::Pagination => {
            // 基础分页令牌
            tokens.insert("itemBg".to_string(), map_token.color_bg_container.clone());
            tokens.insert("itemColor".to_string(), map_token.color_text.clone());
            tokens.insert(
                "itemSize".to_string(),
                format!("{}px", map_token.control_height),
            );
            tokens.insert(
                "itemSizeSm".to_string(),
                format!("{}px", map_token.control_height_sm),
            );
            tokens.insert(
                "borderRadius".to_string(),
                format!("{}px", map_token.border_radius_base),
            );

            // 激活状态
            tokens.insert(
                "itemActiveBg".to_string(),
                map_token.color_bg_container.clone(),
            );
            tokens.insert(
                "itemActiveColor".to_string(),
                map_token.color_primary.clone(),
            );
            tokens.insert(
                "itemActiveBorder".to_string(),
                map_token.color_primary.clone(),
            );

            // 悬停状态
            tokens.insert(
                "itemHoverBg".to_string(),
                if is_dark {
                    "transparent".to_string()
                } else {
                    "#fff".to_string()
                },
            );
            tokens.insert(
                "itemHoverColor".to_string(),
                map_token.color_primary_hover.clone(),
            );
            tokens.insert(
                "itemHoverBorder".to_string(),
                map_token.color_primary_hover.clone(),
            );

            // 禁用状态
            tokens.insert(
                "itemDisabledBg".to_string(),
                map_token.color_bg_disabled.clone(),
            );
            tokens.insert(
                "itemDisabledColor".to_string(),
                map_token.color_text_disabled.clone(),
            );
            tokens.insert(
                "itemDisabledBorder".to_string(),
                map_token.color_border.clone(),
            );
        }
        ComponentAlgorithm::Popover => {
            // 基础弹出框令牌
            tokens.insert("bg".to_string(), map_token.color_bg_container.clone());
            tokens.insert("color".to_string(), map_token.color_text.clone());
            tokens.insert(
                "borderRadius".to_string(),
                format!("{}px", map_token.border_radius_base),
            );
            tokens.insert("border".to_string(), map_token.color_border.clone());
            tokens.insert("boxShadow".to_string(), map_token.box_shadow_popup.clone());
            tokens.insert("padding".to_string(), "12px".to_string());
            tokens.insert("zIndex".to_string(), "1030".to_string());
        }
        ComponentAlgorithm::Drawer => {
            // 抽屉组件令牌
            tokens.insert("bg".to_string(), map_token.color_bg_container.clone());
            tokens.insert("color".to_string(), map_token.color_text.clone());
            tokens.insert("border".to_string(), map_token.color_border.clone());
            tokens.insert(
                "headerBg".to_string(),
                if is_dark {
                    "rgba(255, 255, 255, 0.04)".to_string()
                } else {
                    map_token.color_bg_container.clone()
                },
            );
            tokens.insert(
                "footerBg".to_string(),
                if is_dark {
                    "rgba(255, 255, 255, 0.04)".to_string()
                } else {
                    map_token.color_bg_container.clone()
                },
            );
            tokens.insert("zIndex".to_string(), "1000".to_string());
            tokens.insert("titleColor".to_string(), map_token.color_text.clone());
        }
        ComponentAlgorithm::Dropdown => {
            // 下拉菜单令牌
            tokens.insert("bg".to_string(), map_token.color_bg_container.clone());
            tokens.insert("color".to_string(), map_token.color_text.clone());
            tokens.insert(
                "borderRadius".to_string(),
                format!("{}px", map_token.border_radius_base),
            );
            tokens.insert("boxShadow".to_string(), map_token.box_shadow_popup.clone());
            tokens.insert("padding".to_string(), "4px 0".to_string());
            tokens.insert("zIndex".to_string(), "1050".to_string());

            // 下拉菜单项
            tokens.insert("itemPadding".to_string(), "5px 12px".to_string());
            tokens.insert(
                "itemHoverBg".to_string(),
                map_token.color_bg_text_hover.clone(),
            );
            tokens.insert(
                "itemActiveBg".to_string(),
                map_token.color_bg_text_active.clone(),
            );
        }
        ComponentAlgorithm::Form => {
            // 表单组件令牌
            tokens.insert("labelColor".to_string(), map_token.color_text.clone());
            tokens.insert(
                "labelRequiredMarkColor".to_string(),
                map_token.color_error.clone(),
            );
            tokens.insert(
                "labelHeight".to_string(),
                map_token.control_height.to_string(),
            );
            tokens.insert("errorColor".to_string(), map_token.color_error.clone());
            tokens.insert(
                "helpColor".to_string(),
                map_token.color_text_secondary.clone(),
            );
            tokens.insert("marginBottom".to_string(), "24px".to_string());
        }
        ComponentAlgorithm::Steps => {
            // 步骤条组件令牌
            tokens.insert("bgColor".to_string(), map_token.color_bg_container.clone());
            tokens.insert("titleColor".to_string(), map_token.color_text.clone());
            tokens.insert(
                "descriptionColor".to_string(),
                map_token.color_text_secondary.clone(),
            );
            tokens.insert("iconSize".to_string(), "32px".to_string());
            tokens.insert("iconFontSize".to_string(), "16px".to_string());
            tokens.insert("dotSize".to_string(), "8px".to_string());

            // 激活状态
            tokens.insert(
                "activeTitleColor".to_string(),
                map_token.color_primary.clone(),
            );
            tokens.insert(
                "activeIconBgColor".to_string(),
                map_token.color_primary.clone(),
            );
            tokens.insert("activeIconColor".to_string(), "#fff".to_string());

            // 完成状态
            tokens.insert("finishTitleColor".to_string(), map_token.color_text.clone());
            tokens.insert("finishIconBgColor".to_string(), "#fff".to_string());
            tokens.insert(
                "finishIconColor".to_string(),
                map_token.color_primary.clone(),
            );

            // 错误状态
            tokens.insert("errorTitleColor".to_string(), map_token.color_error.clone());
            tokens.insert(
                "errorIconBgColor".to_string(),
                map_token.color_error.clone(),
            );
            tokens.insert("errorIconColor".to_string(), "#fff".to_string());
        }
        ComponentAlgorithm::Collapse => {
            // 折叠面板组件令牌
            tokens.insert("bg".to_string(), map_token.color_bg_container.clone());
            tokens.insert("color".to_string(), map_token.color_text.clone());
            tokens.insert(
                "borderRadius".to_string(),
                format!("{}px", map_token.border_radius_base),
            );
            tokens.insert("border".to_string(), map_token.color_border.clone());
            tokens.insert("headerPadding".to_string(), "12px 16px".to_string());
            tokens.insert("contentPadding".to_string(), "16px".to_string());
            tokens.insert("headerBg".to_string(), map_token.color_bg_container.clone());
            tokens.insert(
                "activeBorderColor".to_string(),
                map_token.color_primary.clone(),
            );
        }
        ComponentAlgorithm::Slider => {
            // 滑动输入组件令牌
            tokens.insert("railBg".to_string(), map_token.color_split.clone());
            tokens.insert("railHoverBg".to_string(), "#e1e1e1".to_string());
            tokens.insert("trackBg".to_string(), map_token.color_primary.clone());
            tokens.insert(
                "trackHoverBg".to_string(),
                map_token.color_primary_hover.clone(),
            );
            tokens.insert("handleColor".to_string(), "#fff".to_string());
            tokens.insert("handleBg".to_string(), map_token.color_primary.clone());
            tokens.insert("handleBorder".to_string(), map_token.color_primary.clone());
            tokens.insert("handleSize".to_string(), "14px".to_string());
            tokens.insert("dotBg".to_string(), "#fff".to_string());
            tokens.insert("dotBorder".to_string(), "#d9d9d9".to_string());
            tokens.insert(
                "dotActiveBorder".to_string(),
                map_token.color_primary.clone(),
            );

            // 禁用状态
            tokens.insert(
                "disabledRailBg".to_string(),
                map_token.color_bg_disabled.clone(),
            );
            tokens.insert(
                "disabledTrackBg".to_string(),
                map_token.color_text_disabled.clone(),
            );
            tokens.insert("disabledHandleBg".to_string(), "#fff".to_string());
            tokens.insert("disabledHandleBorder".to_string(), "#d9d9d9".to_string());
        }
        ComponentAlgorithm::Switch => {
            // 开关组件令牌
            tokens.insert("bg".to_string(), "#bfbfbf".to_string());
            tokens.insert("bgChecked".to_string(), map_token.color_primary.clone());
            tokens.insert("borderRadius".to_string(), "100px".to_string());
            tokens.insert("handleBg".to_string(), "#fff".to_string());
            tokens.insert("handleSize".to_string(), "18px".to_string());
            tokens.insert("height".to_string(), "22px".to_string());
            tokens.insert("minWidth".to_string(), "44px".to_string());

            // 禁用状态
            tokens.insert(
                "disabledBg".to_string(),
                map_token.color_bg_disabled.clone(),
            );
            tokens.insert("disabledHandleBg".to_string(), "#fff".to_string());
        }
        ComponentAlgorithm::DatePicker => {
            // 日期选择器令牌
            tokens.insert("bg".to_string(), map_token.color_bg_container.clone());
            tokens.insert("color".to_string(), map_token.color_text.clone());
            tokens.insert("border".to_string(), map_token.color_border.clone());
            tokens.insert(
                "borderRadius".to_string(),
                format!("{}px", map_token.border_radius_base),
            );
            tokens.insert("boxShadow".to_string(), map_token.box_shadow_popup.clone());
            tokens.insert("zIndex".to_string(), "1050".to_string());
            tokens.insert("itemBg".to_string(), map_token.color_bg_container.clone());
            tokens.insert("cellHeight".to_string(), "24px".to_string());
            tokens.insert("cellWidth".to_string(), "36px".to_string());
            tokens.insert(
                "cellHoverBg".to_string(),
                map_token.color_bg_text_hover.clone(),
            );
            tokens.insert(
                "cellActiveBg".to_string(),
                map_token.color_primary_bg.clone(),
            );
            tokens.insert(
                "cellActiveColor".to_string(),
                map_token.color_primary.clone(),
            );
            tokens.insert("cellInViewColor".to_string(), map_token.color_text.clone());
            tokens.insert(
                "cellOutViewColor".to_string(),
                map_token.color_text_disabled.clone(),
            );
        }
        ComponentAlgorithm::Notification => {
            // 通知组件令牌
            tokens.insert("bg".to_string(), map_token.color_bg_container.clone());
            tokens.insert("color".to_string(), map_token.color_text.clone());
            tokens.insert(
                "borderRadius".to_string(),
                format!("{}px", map_token.border_radius_base),
            );
            tokens.insert("border".to_string(), map_token.color_border.clone());
            tokens.insert("boxShadow".to_string(), map_token.box_shadow_popup.clone());
            tokens.insert("padding".to_string(), "16px 24px".to_string());
            tokens.insert("width".to_string(), "384px".to_string());
            tokens.insert("zIndex".to_string(), "1010".to_string());
            tokens.insert("titleColor".to_string(), map_token.color_text.clone());
            tokens.insert(
                "descriptionColor".to_string(),
                map_token.color_text_secondary.clone(),
            );

            // 通知类型
            tokens.insert(
                "successIconColor".to_string(),
                map_token.color_success.clone(),
            );
            tokens.insert("infoIconColor".to_string(), map_token.color_info.clone());
            tokens.insert(
                "warningIconColor".to_string(),
                map_token.color_warning.clone(),
            );
            tokens.insert("errorIconColor".to_string(), map_token.color_error.clone());
            tokens.insert("iconSize".to_string(), "24px".to_string());
        }
    }

    tokens
}

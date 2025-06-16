//! 预设主题模块
//!
//! 提供预设的主题配置，方便用户直接使用

use super::{AntThemeType, ThemeConfig};

/// 获取亮色主题配置
///
/// # 返回值
///
/// 亮色主题配置
pub fn light_theme() -> ThemeConfig {
    ThemeConfig::new()
        .theme(AntThemeType::Light)
        .with_primary_color("#1890ff")
        .with_success_color("#52c41a")
        .with_warning_color("#faad14")
        .with_error_color("#f5222d")
        .with_info_color("#1890ff")
        .with_border_radius(2.0)
        .with_font_size(14.0)
}

/// 获取暗色主题配置
///
/// # 返回值
///
/// 暗色主题配置
pub fn dark_theme() -> ThemeConfig {
    ThemeConfig::new()
        .theme(AntThemeType::Dark)
        .with_primary_color("#1890ff")
        .with_success_color("#52c41a")
        .with_warning_color("#faad14")
        .with_error_color("#f5222d")
        .with_info_color("#1890ff")
        .with_border_radius(2.0)
        .with_font_size(14.0)
}

/// 获取紧凑主题配置
///
/// # 返回值
///
/// 紧凑主题配置
pub fn compact_theme() -> ThemeConfig {
    ThemeConfig::new()
        .theme(AntThemeType::Compact)
        .with_primary_color("#1890ff")
        .with_success_color("#52c41a")
        .with_warning_color("#faad14")
        .with_error_color("#f5222d")
        .with_info_color("#1890ff")
        .with_border_radius(2.0)
        .with_font_size(14.0)
}

/// 获取暗色紧凑主题配置
///
/// # 返回值
///
/// 暗色紧凑主题配置
pub fn compact_dark_theme() -> ThemeConfig {
    ThemeConfig::new()
        .theme(AntThemeType::Dark)
        .compact(true)
        .with_primary_color("#1890ff")
        .with_success_color("#52c41a")
        .with_warning_color("#faad14")
        .with_error_color("#f5222d")
        .with_info_color("#1890ff")
        .with_border_radius(2.0)
        .with_font_size(14.0)
}

/// 获取蓝色主题配置
///
/// # 返回值
///
/// 蓝色主题配置
pub fn blue_theme() -> ThemeConfig {
    light_theme().with_primary_color("#1890ff")
}

/// 获取绿色主题配置
///
/// # 返回值
///
/// 绿色主题配置
pub fn green_theme() -> ThemeConfig {
    light_theme().with_primary_color("#52c41a")
}

/// 获取红色主题配置
///
/// # 返回值
///
/// 红色主题配置
pub fn red_theme() -> ThemeConfig {
    light_theme().with_primary_color("#f5222d")
}

/// 获取黄色主题配置
///
/// # 返回值
///
/// 黄色主题配置
pub fn yellow_theme() -> ThemeConfig {
    light_theme().with_primary_color("#faad14")
}

/// 获取紫色主题配置
///
/// # 返回值
///
/// 紫色主题配置
pub fn purple_theme() -> ThemeConfig {
    light_theme().with_primary_color("#722ed1")
}

/// 获取青色主题配置
///
/// # 返回值
///
/// 青色主题配置
pub fn cyan_theme() -> ThemeConfig {
    light_theme().with_primary_color("#13c2c2")
}

/// 获取线框模式主题配置
///
/// # 返回值
///
/// 线框模式主题配置
pub fn wireframe_theme() -> ThemeConfig {
    light_theme().with_wireframe(true)
}

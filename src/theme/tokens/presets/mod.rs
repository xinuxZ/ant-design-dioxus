//! 预设主题令牌模块
//!
//! 提供预设的主题令牌配置

use std::collections::HashMap;

/// 获取默认主题令牌
///
/// # 返回值
///
/// 默认主题的令牌映射
pub fn default_tokens() -> HashMap<String, String> {
    let mut tokens = HashMap::new();

    // 种子令牌
    tokens.insert("colorPrimary".to_string(), "#1890ff".to_string());
    tokens.insert("colorSuccess".to_string(), "#52c41a".to_string());
    tokens.insert("colorWarning".to_string(), "#faad14".to_string());
    tokens.insert("colorError".to_string(), "#f5222d".to_string());
    tokens.insert("colorInfo".to_string(), "#1890ff".to_string());
    tokens.insert("fontSize".to_string(), "14px".to_string());
    tokens.insert("borderRadius".to_string(), "2px".to_string());
    tokens.insert("wireframe".to_string(), "false".to_string());

    // 中性色
    tokens.insert("colorBgBase".to_string(), "#fff".to_string());
    tokens.insert(
        "colorTextBase".to_string(),
        "rgba(0, 0, 0, 0.85)".to_string(),
    );

    // 派生颜色
    tokens.insert("colorText".to_string(), "rgba(0, 0, 0, 0.85)".to_string());
    tokens.insert(
        "colorTextSecondary".to_string(),
        "rgba(0, 0, 0, 0.45)".to_string(),
    );
    tokens.insert(
        "colorTextDisabled".to_string(),
        "rgba(0, 0, 0, 0.25)".to_string(),
    );
    tokens.insert("colorBorder".to_string(), "#d9d9d9".to_string());
    tokens.insert("colorSplit".to_string(), "rgba(0, 0, 0, 0.06)".to_string());

    // 尺寸
    tokens.insert("sizeUnit".to_string(), "4px".to_string());
    tokens.insert("sizeStep".to_string(), "4px".to_string());
    tokens.insert("controlHeight".to_string(), "32px".to_string());
    tokens.insert("controlHeightSM".to_string(), "24px".to_string());
    tokens.insert("controlHeightLG".to_string(), "40px".to_string());

    // 字体
    tokens.insert("fontFamily".to_string(), "-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Noto Sans', sans-serif".to_string());
    tokens.insert(
        "fontFamilyCode".to_string(),
        "'SFMono-Regular', Consolas, 'Liberation Mono', Menlo, Courier, monospace".to_string(),
    );
    tokens.insert("lineHeight".to_string(), "1.5715".to_string());

    // 动画
    tokens.insert("motionDurationSlow".to_string(), "0.3s".to_string());
    tokens.insert("motionDurationMid".to_string(), "0.2s".to_string());
    tokens.insert("motionDurationFast".to_string(), "0.1s".to_string());

    tokens
}

/// 获取暗色主题令牌
///
/// # 返回值
///
/// 暗色主题的令牌映射
pub fn dark_tokens() -> HashMap<String, String> {
    let mut tokens = default_tokens();

    // 覆盖种子令牌
    tokens.insert("colorBgBase".to_string(), "#141414".to_string());
    tokens.insert(
        "colorTextBase".to_string(),
        "rgba(255, 255, 255, 0.85)".to_string(),
    );

    // 派生颜色
    tokens.insert(
        "colorText".to_string(),
        "rgba(255, 255, 255, 0.85)".to_string(),
    );
    tokens.insert(
        "colorTextSecondary".to_string(),
        "rgba(255, 255, 255, 0.45)".to_string(),
    );
    tokens.insert(
        "colorTextDisabled".to_string(),
        "rgba(255, 255, 255, 0.25)".to_string(),
    );
    tokens.insert("colorBorder".to_string(), "#434343".to_string());
    tokens.insert(
        "colorSplit".to_string(),
        "rgba(255, 255, 255, 0.12)".to_string(),
    );

    tokens
}

/// 获取紧凑主题令牌
///
/// # 返回值
///
/// 紧凑主题的令牌映射
pub fn compact_tokens() -> HashMap<String, String> {
    let mut tokens = default_tokens();

    // 尺寸
    tokens.insert("sizeStep".to_string(), "2px".to_string());
    tokens.insert("controlHeight".to_string(), "28px".to_string());
    tokens.insert("controlHeightSM".to_string(), "22px".to_string());
    tokens.insert("controlHeightLG".to_string(), "36px".to_string());

    tokens
}

/// 获取暗色紧凑主题令牌
///
/// # 返回值
///
/// 暗色紧凑主题的令牌映射
pub fn compact_dark_tokens() -> HashMap<String, String> {
    let mut tokens = dark_tokens();

    // 尺寸
    tokens.insert("sizeStep".to_string(), "2px".to_string());
    tokens.insert("controlHeight".to_string(), "28px".to_string());
    tokens.insert("controlHeightSM".to_string(), "22px".to_string());
    tokens.insert("controlHeightLG".to_string(), "36px".to_string());

    tokens
}

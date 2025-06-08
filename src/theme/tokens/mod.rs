//! 主题令牌模块
//!
//! 定义 Ant Design 主题令牌系统

use super::core::types::{AliasToken, MapToken, SeedToken};
use std::collections::HashMap;

/// 预设令牌
pub mod presets;

/// 默认种子令牌
pub fn default_seed_token() -> SeedToken {
    SeedToken {
        color_primary: "#1890ff".to_string(),
        color_success: "#52c41a".to_string(),
        color_warning: "#faad14".to_string(),
        color_error: "#f5222d".to_string(),
        color_info: "#1890ff".to_string(),
        font_size: 14.0,
        border_radius: 2.0,
        wireframe: false,
    }
}

/// 默认映射令牌
pub fn default_map_token() -> MapToken {
    MapToken::default()
}

/// 默认别名令牌
pub fn default_alias_token() -> AliasToken {
    AliasToken::default()
}

/// 生成组件令牌
///
/// 基于全局令牌生成特定组件的令牌
///
/// # 参数
///
/// * `component` - 组件名称
/// * `seed` - 种子令牌
/// * `map` - 映射令牌
///
/// # 返回值
///
/// 组件特定的令牌映射
pub fn generate_component_token(
    component: &str,
    seed: &SeedToken,
    map: &MapToken,
) -> HashMap<String, String> {
    let mut tokens = HashMap::new();

    match component {
        "button" => {
            tokens.insert("buttonPrimaryBg".to_string(), seed.color_primary.clone());
            tokens.insert("buttonPrimaryColor".to_string(), "#fff".to_string());
            tokens.insert("buttonDefaultBg".to_string(), "#fff".to_string());
            tokens.insert("buttonDefaultColor".to_string(), map.color_text.clone());
            tokens.insert(
                "buttonDefaultBorderColor".to_string(),
                map.color_border.clone(),
            );
            tokens.insert("buttonDangerBg".to_string(), seed.color_error.clone());
            tokens.insert("buttonDangerColor".to_string(), "#fff".to_string());
        }
        "input" => {
            tokens.insert("inputBorderColor".to_string(), map.color_border.clone());
            tokens.insert("inputBg".to_string(), "#fff".to_string());
            tokens.insert("inputColor".to_string(), map.color_text.clone());
            tokens.insert(
                "inputPlaceholderColor".to_string(),
                map.color_text_secondary.clone(),
            );
            tokens.insert("inputDisabledBg".to_string(), "#f5f5f5".to_string());
        }
        "select" => {
            tokens.insert("selectBorderColor".to_string(), map.color_border.clone());
            tokens.insert("selectItemSelectedBg".to_string(), "#e6f7ff".to_string());
            tokens.insert(
                "selectItemSelectedColor".to_string(),
                seed.color_primary.clone(),
            );
            tokens.insert("selectDropdownBg".to_string(), "#fff".to_string());
        }
        "menu" => {
            tokens.insert("menuBg".to_string(), "#fff".to_string());
            tokens.insert("menuItemColor".to_string(), map.color_text.clone());
            tokens.insert("menuItemActiveBg".to_string(), "#e6f7ff".to_string());
            tokens.insert(
                "menuItemActiveColor".to_string(),
                seed.color_primary.clone(),
            );
            tokens.insert("menuItemSelectedBg".to_string(), "#e6f7ff".to_string());
            tokens.insert(
                "menuItemSelectedColor".to_string(),
                seed.color_primary.clone(),
            );
        }
        "table" => {
            tokens.insert("tableBg".to_string(), "#fff".to_string());
            tokens.insert("tableHeaderBg".to_string(), "#fafafa".to_string());
            tokens.insert("tableHeaderColor".to_string(), map.color_text.clone());
            tokens.insert("tableBorderColor".to_string(), map.color_border.clone());
            tokens.insert("tableRowHoverBg".to_string(), "#fafafa".to_string());
        }
        "card" => {
            tokens.insert("cardBg".to_string(), "#fff".to_string());
            tokens.insert("cardBorderColor".to_string(), map.color_border.clone());
            tokens.insert("cardHeadColor".to_string(), map.color_text.clone());
            tokens.insert("cardHeadBg".to_string(), "#fff".to_string());
        }
        "modal" => {
            tokens.insert("modalBg".to_string(), "#fff".to_string());
            tokens.insert("modalMaskBg".to_string(), "rgba(0, 0, 0, 0.45)".to_string());
            tokens.insert("modalHeaderBg".to_string(), "#fff".to_string());
            tokens.insert("modalHeaderColor".to_string(), map.color_text.clone());
            tokens.insert("modalContentBg".to_string(), "#fff".to_string());
        }
        "message" => {
            tokens.insert("messageInfoBg".to_string(), "#e6f7ff".to_string());
            tokens.insert("messageSuccessBg".to_string(), "#f6ffed".to_string());
            tokens.insert("messageWarningBg".to_string(), "#fffbe6".to_string());
            tokens.insert("messageErrorBg".to_string(), "#fff1f0".to_string());
        }
        _ => {}
    }

    tokens
}

/// 生成暗色组件令牌
///
/// 基于全局令牌生成特定组件的暗色主题令牌
///
/// # 参数
///
/// * `component` - 组件名称
/// * `seed` - 种子令牌
/// * `map` - 映射令牌
///
/// # 返回值
///
/// 组件特定的暗色主题令牌映射
pub fn generate_dark_component_token(
    component: &str,
    seed: &SeedToken,
    map: &MapToken,
) -> HashMap<String, String> {
    let mut tokens = HashMap::new();

    match component {
        "button" => {
            tokens.insert("buttonPrimaryBg".to_string(), seed.color_primary.clone());
            tokens.insert("buttonPrimaryColor".to_string(), "#fff".to_string());
            tokens.insert("buttonDefaultBg".to_string(), "transparent".to_string());
            tokens.insert("buttonDefaultColor".to_string(), map.color_text.clone());
            tokens.insert(
                "buttonDefaultBorderColor".to_string(),
                map.color_border.clone(),
            );
            tokens.insert("buttonDangerBg".to_string(), seed.color_error.clone());
            tokens.insert("buttonDangerColor".to_string(), "#fff".to_string());
        }
        "input" => {
            tokens.insert("inputBorderColor".to_string(), map.color_border.clone());
            tokens.insert("inputBg".to_string(), "transparent".to_string());
            tokens.insert("inputColor".to_string(), map.color_text.clone());
            tokens.insert(
                "inputPlaceholderColor".to_string(),
                map.color_text_secondary.clone(),
            );
            tokens.insert(
                "inputDisabledBg".to_string(),
                "rgba(255, 255, 255, 0.08)".to_string(),
            );
        }
        "select" => {
            tokens.insert("selectBorderColor".to_string(), map.color_border.clone());
            tokens.insert("selectItemSelectedBg".to_string(), "#111b26".to_string());
            tokens.insert(
                "selectItemSelectedColor".to_string(),
                seed.color_primary.clone(),
            );
            tokens.insert("selectDropdownBg".to_string(), "#1f1f1f".to_string());
        }
        "menu" => {
            tokens.insert("menuBg".to_string(), "#141414".to_string());
            tokens.insert("menuItemColor".to_string(), map.color_text.clone());
            tokens.insert("menuItemActiveBg".to_string(), "#111b26".to_string());
            tokens.insert(
                "menuItemActiveColor".to_string(),
                seed.color_primary.clone(),
            );
            tokens.insert("menuItemSelectedBg".to_string(), "#111b26".to_string());
            tokens.insert(
                "menuItemSelectedColor".to_string(),
                seed.color_primary.clone(),
            );
        }
        "table" => {
            tokens.insert("tableBg".to_string(), "#141414".to_string());
            tokens.insert("tableHeaderBg".to_string(), "#1d1d1d".to_string());
            tokens.insert("tableHeaderColor".to_string(), map.color_text.clone());
            tokens.insert("tableBorderColor".to_string(), map.color_border.clone());
            tokens.insert("tableRowHoverBg".to_string(), "#1d1d1d".to_string());
        }
        "card" => {
            tokens.insert("cardBg".to_string(), "#141414".to_string());
            tokens.insert("cardBorderColor".to_string(), map.color_border.clone());
            tokens.insert("cardHeadColor".to_string(), map.color_text.clone());
            tokens.insert("cardHeadBg".to_string(), "#141414".to_string());
        }
        "modal" => {
            tokens.insert("modalBg".to_string(), "#141414".to_string());
            tokens.insert("modalMaskBg".to_string(), "rgba(0, 0, 0, 0.45)".to_string());
            tokens.insert("modalHeaderBg".to_string(), "#141414".to_string());
            tokens.insert("modalHeaderColor".to_string(), map.color_text.clone());
            tokens.insert("modalContentBg".to_string(), "#141414".to_string());
        }
        "message" => {
            tokens.insert(
                "messageInfoBg".to_string(),
                "rgba(16, 142, 233, 0.15)".to_string(),
            );
            tokens.insert(
                "messageSuccessBg".to_string(),
                "rgba(82, 196, 26, 0.15)".to_string(),
            );
            tokens.insert(
                "messageWarningBg".to_string(),
                "rgba(250, 173, 20, 0.15)".to_string(),
            );
            tokens.insert(
                "messageErrorBg".to_string(),
                "rgba(245, 34, 45, 0.15)".to_string(),
            );
        }
        _ => {}
    }

    tokens
}

//! CSS变量模块
//!
//! 提供CSS变量生成和注入功能

use crate::theme::core::types::{AliasToken, MapToken, SeedToken};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// CSS变量选项
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CssVariablesOptions {
    /// 是否启用CSS变量
    pub enabled: bool,
    /// CSS变量前缀
    pub prefix: String,
    /// 根选择器
    pub root_selector: String,
    /// 是否使用哈希值
    pub hashed: bool,
    /// 哈希键
    pub hash_key: Option<String>,
}

impl Default for CssVariablesOptions {
    fn default() -> Self {
        Self {
            enabled: false,
            prefix: "ant".to_string(),
            root_selector: ":root".to_string(),
            hashed: true,
            hash_key: None,
        }
    }
}

/// 生成CSS变量
///
/// 基于令牌生成CSS变量字符串
///
/// # 参数
///
/// * `seed` - 种子令牌
/// * `map` - 映射令牌
/// * `alias` - 别名令牌
/// * `options` - CSS变量选项
///
/// # 返回值
///
/// CSS变量字符串
pub fn generate_css_variables(
    seed: &SeedToken,
    map: &MapToken,
    alias: &AliasToken,
    options: &CssVariablesOptions,
) -> String {
    if !options.enabled {
        return String::new();
    }

    let mut css_vars = HashMap::new();
    let prefix = &options.prefix;

    // 添加种子令牌变量
    add_seed_token_vars(seed, prefix, &mut css_vars);

    // 添加映射令牌变量
    add_map_token_vars(map, prefix, &mut css_vars);

    // 添加别名令牌变量
    add_alias_token_vars(alias, prefix, &mut css_vars);

    // 生成CSS变量字符串
    generate_css_string(&css_vars, &options.root_selector)
}

/// 添加种子令牌变量
fn add_seed_token_vars(seed: &SeedToken, prefix: &str, css_vars: &mut HashMap<String, String>) {
    css_vars.insert(
        format!("--{}-color-primary", prefix),
        seed.color_primary.clone(),
    );
    css_vars.insert(
        format!("--{}-color-success", prefix),
        seed.color_success.clone(),
    );
    css_vars.insert(
        format!("--{}-color-warning", prefix),
        seed.color_warning.clone(),
    );
    css_vars.insert(
        format!("--{}-color-error", prefix),
        seed.color_error.clone(),
    );
    css_vars.insert(format!("--{}-color-info", prefix), seed.color_info.clone());
    css_vars.insert(format!("--{}-color-link", prefix), seed.color_link.clone());
    css_vars.insert(
        format!("--{}-color-text-base", prefix),
        seed.color_text_base.clone(),
    );
    css_vars.insert(
        format!("--{}-color-bg-base", prefix),
        seed.color_bg_base.clone(),
    );
    css_vars.insert(
        format!("--{}-font-size", prefix),
        format!("{}px", seed.font_size),
    );
    css_vars.insert(
        format!("--{}-border-radius", prefix),
        format!("{}px", seed.border_radius),
    );
    css_vars.insert(
        format!("--{}-control-height", prefix),
        format!("{}px", seed.control_height),
    );
    css_vars.insert(
        format!("--{}-line-width", prefix),
        format!("{}px", seed.line_width),
    );
    css_vars.insert(format!("--{}-line-type", prefix), seed.line_type.clone());
    css_vars.insert(
        format!("--{}-motion-unit", prefix),
        format!("{}s", seed.motion_unit),
    );
    css_vars.insert(
        format!("--{}-motion-base", prefix),
        format!("{}", seed.motion_base),
    );
    css_vars.insert(
        format!("--{}-z-index-base", prefix),
        format!("{}", seed.z_index_base),
    );
    css_vars.insert(
        format!("--{}-z-index-popup-base", prefix),
        format!("{}", seed.z_index_popup_base),
    );
    css_vars.insert(
        format!("--{}-opacity-image", prefix),
        format!("{}", seed.opacity_image),
    );
    css_vars.insert(
        format!("--{}-size-unit", prefix),
        format!("{}px", seed.size_unit),
    );
    css_vars.insert(
        format!("--{}-size-step", prefix),
        format!("{}px", seed.size_step),
    );
    css_vars.insert(
        format!("--{}-size-popup-arrow", prefix),
        format!("{}px", seed.size_popup_arrow),
    );
    css_vars.insert(
        format!("--{}-font-family", prefix),
        seed.font_family.clone(),
    );
    css_vars.insert(
        format!("--{}-font-family-code", prefix),
        seed.font_family_code.clone(),
    );
    css_vars.insert(
        format!("--{}-wireframe", prefix),
        format!("{}", seed.wireframe),
    );
    css_vars.insert(format!("--{}-motion", prefix), format!("{}", seed.motion));
}

/// 添加映射令牌变量
fn add_map_token_vars(map: &MapToken, prefix: &str, css_vars: &mut HashMap<String, String>) {
    // 调色板
    for (i, color) in map.color_primary_palette.iter().enumerate() {
        css_vars.insert(
            format!("--{}-color-primary-{}", prefix, i + 1),
            color.clone(),
        );
    }

    // 中性色
    css_vars.insert(
        format!("--{}-color-bg-base", prefix),
        map.color_bg_base.clone(),
    );
    css_vars.insert(
        format!("--{}-color-text-base", prefix),
        map.color_text_base.clone(),
    );

    // 派生颜色
    css_vars.insert(format!("--{}-color-text", prefix), map.color_text.clone());
    css_vars.insert(
        format!("--{}-color-text-secondary", prefix),
        map.color_text_secondary.clone(),
    );
    css_vars.insert(
        format!("--{}-color-text-disabled", prefix),
        map.color_text_disabled.clone(),
    );
    css_vars.insert(
        format!("--{}-color-border", prefix),
        map.color_border.clone(),
    );
    css_vars.insert(format!("--{}-color-split", prefix), map.color_split.clone());

    // 背景色
    css_vars.insert(
        format!("--{}-color-bg-container", prefix),
        map.color_bg_container.clone(),
    );
    css_vars.insert(
        format!("--{}-color-bg-elevated", prefix),
        map.color_bg_elevated.clone(),
    );
    css_vars.insert(
        format!("--{}-color-bg-layout", prefix),
        map.color_bg_layout.clone(),
    );
    css_vars.insert(
        format!("--{}-color-bg-mask", prefix),
        map.color_bg_mask.clone(),
    );
    css_vars.insert(
        format!("--{}-color-bg-spotlight", prefix),
        map.color_bg_spotlight.clone(),
    );
    css_vars.insert(
        format!("--{}-color-bg-text-hover", prefix),
        map.color_bg_text_hover.clone(),
    );
    css_vars.insert(
        format!("--{}-color-bg-text-active", prefix),
        map.color_bg_text_active.clone(),
    );
    css_vars.insert(
        format!("--{}-color-bg-disabled", prefix),
        map.color_bg_disabled.clone(),
    );

    // 边框圆角
    css_vars.insert(
        format!("--{}-border-radius-base", prefix),
        format!("{}px", map.border_radius_base),
    );
    css_vars.insert(
        format!("--{}-border-radius-sm", prefix),
        format!("{}px", map.border_radius_sm),
    );
    css_vars.insert(
        format!("--{}-border-radius-lg", prefix),
        format!("{}px", map.border_radius_lg),
    );
    css_vars.insert(
        format!("--{}-border-radius-xl", prefix),
        format!("{}px", map.border_radius_xl),
    );

    // 尺寸
    css_vars.insert(
        format!("--{}-size-unit", prefix),
        format!("{}px", map.size_unit),
    );
    css_vars.insert(
        format!("--{}-size-step", prefix),
        format!("{}px", map.size_step),
    );

    // 控件尺寸
    css_vars.insert(
        format!("--{}-control-height-sm", prefix),
        format!("{}px", map.control_height_sm),
    );
    css_vars.insert(
        format!("--{}-control-height", prefix),
        format!("{}px", map.control_height),
    );
    css_vars.insert(
        format!("--{}-control-height-lg", prefix),
        format!("{}px", map.control_height_lg),
    );

    // 字体大小
    css_vars.insert(
        format!("--{}-font-size-sm", prefix),
        format!("{}px", map.font_size_sm),
    );
    css_vars.insert(
        format!("--{}-font-size", prefix),
        format!("{}px", map.font_size),
    );
    css_vars.insert(
        format!("--{}-font-size-lg", prefix),
        format!("{}px", map.font_size_lg),
    );
    css_vars.insert(
        format!("--{}-font-size-xl", prefix),
        format!("{}px", map.font_size_xl),
    );

    // 行高
    css_vars.insert(
        format!("--{}-line-height", prefix),
        format!("{}", map.line_height),
    );
    css_vars.insert(
        format!("--{}-line-height-sm", prefix),
        format!("{}", map.line_height_sm),
    );
    css_vars.insert(
        format!("--{}-line-height-lg", prefix),
        format!("{}", map.line_height_lg),
    );
}

/// 添加别名令牌变量
fn add_alias_token_vars(alias: &AliasToken, prefix: &str, css_vars: &mut HashMap<String, String>) {
    // 组件通用
    css_vars.insert(
        format!("--{}-component-background", prefix),
        alias.component_background.clone(),
    );
    css_vars.insert(
        format!("--{}-component-text-color", prefix),
        alias.component_text_color.clone(),
    );
    css_vars.insert(
        format!("--{}-component-border-color", prefix),
        alias.component_border_color.clone(),
    );

    // 阴影
    css_vars.insert(format!("--{}-box-shadow", prefix), alias.box_shadow.clone());
    css_vars.insert(
        format!("--{}-box-shadow-secondary", prefix),
        alias.box_shadow_secondary.clone(),
    );
    css_vars.insert(
        format!("--{}-box-shadow-tertiary", prefix),
        alias.box_shadow_tertiary.clone(),
    );

    // 按钮
    css_vars.insert(
        format!("--{}-button-height-sm", prefix),
        format!("{}px", alias.button_height_sm),
    );
    css_vars.insert(
        format!("--{}-button-height", prefix),
        format!("{}px", alias.button_height),
    );
    css_vars.insert(
        format!("--{}-button-height-lg", prefix),
        format!("{}px", alias.button_height_lg),
    );

    // 输入框
    css_vars.insert(
        format!("--{}-input-height-sm", prefix),
        format!("{}px", alias.input_height_sm),
    );
    css_vars.insert(
        format!("--{}-input-height", prefix),
        format!("{}px", alias.input_height),
    );
    css_vars.insert(
        format!("--{}-input-height-lg", prefix),
        format!("{}px", alias.input_height_lg),
    );

    // 间距
    css_vars.insert(
        format!("--{}-margin", prefix),
        format!("{}px", alias.margin),
    );
    css_vars.insert(
        format!("--{}-margin-xs", prefix),
        format!("{}px", alias.margin_xs),
    );
    css_vars.insert(
        format!("--{}-margin-sm", prefix),
        format!("{}px", alias.margin_sm),
    );
    css_vars.insert(
        format!("--{}-margin-md", prefix),
        format!("{}px", alias.margin_md),
    );
    css_vars.insert(
        format!("--{}-margin-lg", prefix),
        format!("{}px", alias.margin_lg),
    );
    css_vars.insert(
        format!("--{}-margin-xl", prefix),
        format!("{}px", alias.margin_xl),
    );
    css_vars.insert(
        format!("--{}-margin-xxl", prefix),
        format!("{}px", alias.margin_xxl),
    );

    // 内边距
    css_vars.insert(
        format!("--{}-padding", prefix),
        format!("{}px", alias.padding),
    );
    css_vars.insert(
        format!("--{}-padding-xs", prefix),
        format!("{}px", alias.padding_xs),
    );
    css_vars.insert(
        format!("--{}-padding-sm", prefix),
        format!("{}px", alias.padding_sm),
    );
    css_vars.insert(
        format!("--{}-padding-md", prefix),
        format!("{}px", alias.padding_md),
    );
    css_vars.insert(
        format!("--{}-padding-lg", prefix),
        format!("{}px", alias.padding_lg),
    );
    css_vars.insert(
        format!("--{}-padding-xl", prefix),
        format!("{}px", alias.padding_xl),
    );
    css_vars.insert(
        format!("--{}-padding-xxl", prefix),
        format!("{}px", alias.padding_xxl),
    );

    // 链接样式
    css_vars.insert(
        format!("--{}-link-decoration", prefix),
        alias.link_decoration.clone(),
    );
    css_vars.insert(
        format!("--{}-link-hover-decoration", prefix),
        alias.link_hover_decoration.clone(),
    );
    css_vars.insert(
        format!("--{}-link-focus-decoration", prefix),
        alias.link_focus_decoration.clone(),
    );

    // 控件相关
    css_vars.insert(
        format!("--{}-control-interactive-size", prefix),
        format!("{}px", alias.control_interactive_size),
    );
    css_vars.insert(
        format!("--{}-control-outline-width", prefix),
        format!("{}px", alias.control_outline_width),
    );
    css_vars.insert(
        format!("--{}-control-padding-horizontal", prefix),
        format!("{}px", alias.control_padding_horizontal),
    );
    css_vars.insert(
        format!("--{}-control-padding-horizontal-sm", prefix),
        format!("{}px", alias.control_padding_horizontal_sm),
    );
}

/// 生成CSS变量字符串
fn generate_css_string(css_vars: &HashMap<String, String>, root_selector: &str) -> String {
    let mut css = format!("{} {{\n", root_selector);

    for (key, value) in css_vars {
        css.push_str(&format!("  {}: {};\n", key, value));
    }

    css.push_str("}\n");
    css
}

/// 将CSS变量注入到DOM
pub fn inject_css_variables(css: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsCast;
        use web_sys::{window, Element, HtmlElement};

        if let Some(window) = window() {
            if let Some(document) = window.document() {
                // 查找或创建style元素
                let style_id = "ant-design-dioxus-css-variables";
                let style_element = match document.get_element_by_id(style_id) {
                    Some(element) => element,
                    None => {
                        let element = document
                            .create_element("style")
                            .expect("Failed to create style element");
                        element
                            .set_attribute("id", style_id)
                            .expect("Failed to set style id");
                        element
                            .set_attribute("type", "text/css")
                            .expect("Failed to set style type");

                        // 添加到head
                        if let Some(head) = document.head() {
                            head.append_child(&element)
                                .expect("Failed to append style to head");
                        }

                        element
                    }
                };

                // 设置CSS内容
                style_element.set_text_content(Some(css));
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        // 在非WASM环境中，只记录CSS变量
        log::debug!("CSS Variables: {}", css);
    }
}

/// 创建CSS变量样式元素
///
/// 将CSS变量字符串转换为可注入的style元素字符串
///
/// # 参数
///
/// * `css` - CSS变量字符串
///
/// # 返回值
///
/// style元素字符串
pub fn create_css_variables_style_element(css: &str) -> String {
    if css.is_empty() {
        return String::new();
    }

    format!(
        r#"<style id="ant-design-dioxus-css-variables" type="text/css">
{}
</style>"#,
        css
    )
}

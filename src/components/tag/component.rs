//!
//! Tag 标签组件
//!
//! 进行标记和分类的小标签。
//!
//! ## 何时使用
//!
//! - 用于标记事物的属性和维度。
//! - 进行分类。
//!
//! ## 示例
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::Tag;
//!
//! fn app() -> Element {
//!     rsx! {
//!         div {
//!             Tag { "Tag 1" }
//!             Tag { color: "blue", "Tag 2" }
//!             Tag { closable: true, "Closable Tag" }
//!         }
//!     }
//! }
//! ```

use super::styles::{generate_tag_styles, TagStyleGenerator};
use super::types::*;
use crate::config_provider::use_config;
use crate::theme::use_theme;
use css_in_rust::css;
use dioxus::prelude::*;

/// Tag 标签组件
///
/// 进行标记和分类的小标签。
#[component]
pub fn Tag(props: TagProps) -> Element {
    // 获取配置和主题
    let config = use_config();
    let theme = use_theme();

    // 确定前缀类名
    let prefix_cls = props
        .prefix_cls
        .clone()
        .unwrap_or_else(|| config.config.prefix_cls.clone());

    // 创建样式生成器
    let style_generator = TagStyleGenerator::new()
        .with_color(props.color.clone())
        .with_size(props.size.clone())
        .with_variant(props.variant.clone())
        .with_closable(props.closable)
        .with_bordered(props.bordered)
        .with_disabled(props.disabled)
        .with_status(props.status.clone())
        .with_prefix_cls(prefix_cls.clone())
        .with_token(crate::theme::AliasToken::default());

    // 获取样式
    let styles = generate_tag_styles();

    // 注册基础样式
    {
        let style_generator_clone = style_generator.clone();
        let color = props.color.clone();
        let size = props.size.clone();
        let variant = props.variant.clone();

        use_effect(move || {
            let _ = css!(styles.base);

            // 注册变体样式
            match variant {
                TagVariant::Outlined => {
                    let _ = css!(styles.variants.outlined);
                }
                TagVariant::Borderless => {
                    let _ = css!(styles.variants.borderless);
                }
                _ => {
                    let _ = css!(styles.variants.filled);
                }
            }

            // 注册颜色样式
            match &color {
                TagColor::Primary => {
                    let _ = css!(styles.colors.primary);
                }
                TagColor::Success => {
                    let _ = css!(styles.colors.success);
                }
                TagColor::Warning => {
                    let _ = css!(styles.colors.warning);
                }
                TagColor::Error => {
                    let _ = css!(styles.colors.error);
                }
                TagColor::Info => {
                    let _ = css!(styles.colors.info);
                }
                TagColor::Processing => {
                    let _ = css!(styles.colors.processing);
                }
                TagColor::Preset(preset) => match preset {
                    TagPresetColor::Magenta => {
                        let _ = css!(styles.colors.preset_colors.magenta);
                    }
                    TagPresetColor::Red => {
                        let _ = css!(styles.colors.preset_colors.red);
                    }
                    TagPresetColor::Volcano => {
                        let _ = css!(styles.colors.preset_colors.volcano);
                    }
                    TagPresetColor::Orange => {
                        let _ = css!(styles.colors.preset_colors.orange);
                    }
                    TagPresetColor::Gold => {
                        let _ = css!(styles.colors.preset_colors.gold);
                    }
                    TagPresetColor::Lime => {
                        let _ = css!(styles.colors.preset_colors.lime);
                    }
                    TagPresetColor::Green => {
                        let _ = css!(styles.colors.preset_colors.green);
                    }
                    TagPresetColor::Cyan => {
                        let _ = css!(styles.colors.preset_colors.cyan);
                    }
                    TagPresetColor::Blue => {
                        let _ = css!(styles.colors.preset_colors.blue);
                    }
                    TagPresetColor::GeekBlue => {
                        let _ = css!(styles.colors.preset_colors.geekblue);
                    }
                    TagPresetColor::Purple => {
                        let _ = css!(styles.colors.preset_colors.purple);
                    }
                },
                _ => {
                    let _ = css!(styles.colors.default);
                }
            }

            // 注册尺寸样式
            match size {
                TagSize::Small => {
                    let _ = css!(styles.sizes.small);
                }
                TagSize::Large => {
                    let _ = css!(styles.sizes.large);
                }
                _ => {
                    let _ = css!(styles.sizes.middle);
                }
            }
        });
    }

    // 生成CSS类名
    let mut class_names = style_generator.generate_class_names();

    // 添加自定义类名
    if let Some(custom_class) = &props.class {
        class_names.push(custom_class.clone());
    }

    // 生成内联样式
    let inline_styles = style_generator.generate_inline_styles();

    // 合并自定义样式
    let final_styles = if let Some(custom_style) = &props.style {
        if inline_styles.is_empty() {
            custom_style.clone()
        } else {
            format!("{};{}", inline_styles, custom_style)
        }
    } else {
        inline_styles
    };

    let class_name = class_names.join(" ");

    // 处理关闭事件
    let handle_close = {
        let on_close = props.on_close.clone();
        move |_| {
            if let Some(callback) = &on_close {
                callback.call(());
            }
        }
    };

    // 处理点击事件
    let handle_click = {
        let on_click = props.on_click.clone();
        let disabled = props.disabled;
        move |evt: MouseEvent| {
            if !disabled {
                if let Some(callback) = &on_click {
                    callback.call(evt);
                }
            }
        }
    };

    // 渲染组件
    rsx! {
        span {
            class: "{class_name}",
            style: if final_styles.is_empty() { None } else { Some(final_styles) },
            onclick: handle_click,

            // 自定义图标
            if let Some(icon) = props.icon {
                span {
                    class: "{prefix_cls}-tag-icon",
                    {icon}
                }
            }

            // 标签内容
            if let Some(children) = props.children {
                {children}
            }

            // 关闭按钮
            if props.closable && !props.disabled {
                if let Some(close_icon) = props.close_icon {
                    span {
                        class: "{prefix_cls}-tag-close-icon",
                        onclick: handle_close,
                        {close_icon}
                    }
                } else {
                    span {
                        class: "{prefix_cls}-tag-close-icon",
                        onclick: handle_close,
                        "×"
                    }
                }
            }
        }
    }
}

/// CheckableTag 可选择标签组件
///
/// 类似 Checkbox，点击切换选中状态。
#[component]
pub fn CheckableTag(props: CheckableTagProps) -> Element {
    // 获取配置和主题
    let config = use_config();
    let theme = use_theme();

    // 确定前缀类名
    let prefix_cls = props
        .prefix_cls
        .clone()
        .unwrap_or_else(|| config.config.prefix_cls.clone());

    // 获取样式
    let styles = generate_tag_styles();

    // 注册基础样式
    {
        use_effect(move || {
            let _ = css!(styles.base);
            let _ = css!(
                r#"
                .ant-tag-checkable {
                    background-color: transparent;
                    border-style: dashed;
                    cursor: pointer;
                }

                .ant-tag-checkable:not(.ant-tag-checkable-checked):hover {
                    color: #1677ff;
                    border-color: #1677ff;
                }

                .ant-tag-checkable-checked {
                    background-color: #1677ff;
                    border-color: #1677ff;
                    color: #fff;
                }

                .ant-tag-checkable.ant-tag-disabled {
                    cursor: not-allowed;
                    opacity: 0.5;
                }
            "#
            );
        });
    }

    // 生成CSS类名
    let mut class_names = vec![
        format!("{}-tag", prefix_cls),
        format!("{}-tag-checkable", prefix_cls),
    ];

    if props.checked {
        class_names.push(format!("{}-tag-checkable-checked", prefix_cls));
    }

    if props.disabled {
        class_names.push(format!("{}-tag-disabled", prefix_cls));
    }

    // 添加自定义类名
    if let Some(custom_class) = &props.class {
        class_names.push(custom_class.clone());
    }

    let class_name = class_names.join(" ");

    // 处理点击事件
    let handle_click = {
        let on_change = props.on_change.clone();
        let checked = props.checked;
        let disabled = props.disabled;
        move |_| {
            if !disabled {
                if let Some(callback) = &on_change {
                    callback.call(!checked);
                }
            }
        }
    };

    // 渲染组件
    rsx! {
        span {
            class: "{class_name}",
            style: props.style.clone(),
            onclick: handle_click,

            // 标签内容
            if let Some(children) = props.children {
                {children}
            }
        }
    }
}

//!
//! Tag 组件使用示例
//!
//! 展示 Tag 组件的各种使用方式和配置选项。

use dioxus::prelude::*;
use super::*;

/// 基础标签示例
#[component]
pub fn BasicTagExample() -> Element {
    rsx! {
        div { class: "tag-examples",
            h3 { "基础标签" }
            div { class: "example-row",
                Tag { "Default" }
                Tag { color: TagColor::Primary, "Primary" }
                Tag { color: TagColor::Success, "Success" }
                Tag { color: TagColor::Warning, "Warning" }
                Tag { color: TagColor::Error, "Error" }
            }
        }
    }
}

/// 预设颜色标签示例
#[component]
pub fn PresetColorTagExample() -> Element {
    rsx! {
        div { class: "tag-examples",
            h3 { "预设颜色" }
            div { class: "example-row",
                Tag { color: TagColor::Preset(TagPresetColor::Blue), "Blue" }
                Tag { color: TagColor::Preset(TagPresetColor::Purple), "Purple" }
                Tag { color: TagColor::Preset(TagPresetColor::Cyan), "Cyan" }
                Tag { color: TagColor::Preset(TagPresetColor::Green), "Green" }
                Tag { color: TagColor::Preset(TagPresetColor::Magenta), "Magenta" }
                Tag { color: TagColor::Preset(TagPresetColor::Pink), "Pink" }
                Tag { color: TagColor::Preset(TagPresetColor::Red), "Red" }
                Tag { color: TagColor::Preset(TagPresetColor::Orange), "Orange" }
                Tag { color: TagColor::Preset(TagPresetColor::Yellow), "Yellow" }
                Tag { color: TagColor::Preset(TagPresetColor::Volcano), "Volcano" }
                Tag { color: TagColor::Preset(TagPresetColor::Geekblue), "Geekblue" }
                Tag { color: TagColor::Preset(TagPresetColor::Lime), "Lime" }
                Tag { color: TagColor::Preset(TagPresetColor::Gold), "Gold" }
            }
        }
    }
}

/// 自定义颜色标签示例
#[component]
pub fn CustomColorTagExample() -> Element {
    rsx! {
        div { class: "tag-examples",
            h3 { "自定义颜色" }
            div { class: "example-row",
                Tag { color: TagColor::Custom("#f50".to_string()), "#f50" }
                Tag { color: TagColor::Custom("#2db7f5".to_string()), "#2db7f5" }
                Tag { color: TagColor::Custom("#87d068".to_string()), "#87d068" }
                Tag { color: TagColor::Custom("#108ee9".to_string()), "#108ee9" }
            }
        }
    }
}

/// 不同尺寸标签示例
#[component]
pub fn TagSizeExample() -> Element {
    rsx! {
        div { class: "tag-examples",
            h3 { "不同尺寸" }
            div { class: "example-row", style: "align-items: center;",
                Tag { size: TagSize::Small, "Small" }
                Tag { size: TagSize::Middle, "Middle" }
                Tag { size: TagSize::Large, "Large" }
            }
        }
    }
}

/// 不同变体标签示例
#[component]
pub fn TagVariantExample() -> Element {
    rsx! {
        div { class: "tag-examples",
            h3 { "不同变体" }
            div { class: "example-row",
                Tag { variant: TagVariant::Filled, color: TagColor::Primary, "Filled" }
                Tag { variant: TagVariant::Outlined, color: TagColor::Primary, "Outlined" }
                Tag { variant: TagVariant::Borderless, color: TagColor::Primary, "Borderless" }
            }
        }
    }
}

/// 可关闭标签示例
#[component]
pub fn ClosableTagExample() -> Element {
    let mut tags = use_signal(|| vec![
        "Tag 1".to_string(),
        "Tag 2".to_string(),
        "Tag 3".to_string(),
    ]);

    rsx! {
        div { class: "tag-examples",
            h3 { "可关闭标签" }
            div { class: "example-row",
                for (index, tag) in tags.read().iter().enumerate() {
                    Tag {
                        key: "{index}",
                        closable: true,
                        on_close: move |_| {
                            tags.write().remove(index);
                        },
                        "{tag}"
                    }
                }
            }
        }
    }
}

/// 带图标的标签示例
#[component]
pub fn TagWithIconExample() -> Element {
    rsx! {
        div { class: "tag-examples",
            h3 { "带图标的标签" }
            div { class: "example-row",
                Tag {
                    icon: rsx! { span { "🏷️" } },
                    "Tag"
                }
                Tag {
                    icon: rsx! { span { "⭐" } },
                    color: TagColor::Primary,
                    "Star"
                }
                Tag {
                    icon: rsx! { span { "🔥" } },
                    color: TagColor::Error,
                    "Hot"
                }
            }
        }
    }
}

/// 状态标签示例
#[component]
pub fn TagStatusExample() -> Element {
    rsx! {
        div { class: "tag-examples",
            h3 { "状态标签" }
            div { class: "example-row",
                Tag { status: Some(TagStatus::Success), "Success" }
                Tag { status: Some(TagStatus::Processing), "Processing" }
                Tag { status: Some(TagStatus::Error), "Error" }
                Tag { status: Some(TagStatus::Warning), "Warning" }
                Tag { status: Some(TagStatus::Default), "Default" }
            }
        }
    }
}

/// 可选择标签示例
#[component]
pub fn CheckableTagExample() -> Element {
    let mut checked_tags = use_signal(|| vec![false, true, false, false]);

    rsx! {
        div { class: "tag-examples",
            h3 { "可选择标签" }
            div { class: "example-row",
                for (index, &checked) in checked_tags.read().iter().enumerate() {
                    CheckableTag {
                        key: "{index}",
                        checked: checked,
                        on_change: move |new_checked| {
                            checked_tags.write()[index] = new_checked;
                        },
                        "Tag {index + 1}"
                    }
                }
            }
        }
    }
}

/// 禁用状态标签示例
#[component]
pub fn DisabledTagExample() -> Element {
    rsx! {
        div { class: "tag-examples",
            h3 { "禁用状态" }
            div { class: "example-row",
                Tag { disabled: true, "Disabled" }
                Tag { disabled: true, color: TagColor::Primary, "Disabled Primary" }
                Tag { disabled: true, closable: true, "Disabled Closable" }
                CheckableTag { disabled: true, checked: false, "Disabled Checkable" }
                CheckableTag { disabled: true, checked: true, "Disabled Checked" }
            }
        }
    }
}

/// 无边框标签示例
#[component]
pub fn BorderlessTagExample() -> Element {
    rsx! {
        div { class: "tag-examples",
            h3 { "无边框标签" }
            div { class: "example-row",
                Tag { bordered: false, "Borderless" }
                Tag { bordered: false, color: TagColor::Primary, "Primary Borderless" }
                Tag { bordered: false, color: TagColor::Success, "Success Borderless" }
                Tag { bordered: false, color: TagColor::Warning, "Warning Borderless" }
                Tag { bordered: false, color: TagColor::Error, "Error Borderless" }
            }
        }
    }
}

/// 自定义关闭图标示例
#[component]
pub fn CustomCloseIconExample() -> Element {
    rsx! {
        div { class: "tag-examples",
            h3 { "自定义关闭图标" }
            div { class: "example-row",
                Tag {
                    closable: true,
                    close_icon: rsx! { span { "✕" } },
                    "Custom Close 1"
                }
                Tag {
                    closable: true,
                    close_icon: rsx! { span { "⊗" } },
                    color: TagColor::Primary,
                    "Custom Close 2"
                }
                Tag {
                    closable: true,
                    close_icon: rsx! { span { "🗙" } },
                    color: TagColor::Error,
                    "Custom Close 3"
                }
            }
        }
    }
}

/// 综合示例
#[component]
pub fn ComprehensiveTagExample() -> Element {
    rsx! {
        div { class: "tag-examples-container",
            style: "
                padding: 20px;
                .tag-examples {
                    margin-bottom: 24px;
                }
                .example-row {
                    display: flex;
                    gap: 8px;
                    flex-wrap: wrap;
                    margin-top: 8px;
                }
                h3 {
                    margin: 0 0 8px 0;
                    font-size: 16px;
                    font-weight: 600;
                }
            ",
            
            BasicTagExample {}
            PresetColorTagExample {}
            CustomColorTagExample {}
            TagSizeExample {}
            TagVariantExample {}
            ClosableTagExample {}
            TagWithIconExample {}
            TagStatusExample {}
            CheckableTagExample {}
            DisabledTagExample {}
            BorderlessTagExample {}
            CustomCloseIconExample {}
        }
    }
}
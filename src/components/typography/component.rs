//! Typography 组件实现
//!
//! 本模块包含Typography组件的所有组件实现，包括Title、Text、Paragraph和Link等。

use super::styles::*;
use super::types::*;
use crate::theme::{use_theme, Theme};
use dioxus::prelude::*;

/// Typography 基础组件
#[component]
pub fn Typography(props: TypographyProps) -> Element {
    let theme = use_theme();
    let text_type = props.r#type.as_ref().unwrap_or(&TextType::Default);
    
    let style_generator = TypographyStyleGenerator::new()
        .with_type(text_type.clone())
        .with_disabled(props.disabled)
        .with_delete(props.delete)
        .with_underline(props.underline)
        .with_strong(props.strong)
        .with_italic(props.italic)
        .with_mark(props.mark)
        .with_code(props.code)
        .with_keyboard(props.keyboard)
        .with_copyable(props.copyable.is_some())
        .with_editable(props.editable.is_some());
    
    let class_name = format!("{} {}", 
        style_generator.generate_class(),
        props.class.clone().unwrap_or_default()
    );

    let mut style = props.style.clone().unwrap_or_default();
    if let Some(ellipsis) = &props.ellipsis {
        if let Some(suffix) = &ellipsis.suffix {
            // 使用默认宽度，实际应该从 ellipsis 配置中获取
            style.push_str("max-width: 100%;");
        }
    }

    rsx! {
        span {
            class: "{class_name}",
            style: "{style}",
            {props.children}
        }
    }
}

/// Title 组件
#[component]
pub fn Title(props: TitleProps) -> Element {
    let theme = use_theme();
    let text_type = props.r#type.as_ref().unwrap_or(&TextType::Default);
    
    let mut style_generator = TitleStyleGenerator::new(props.level.clone())
        .with_type(text_type.clone())
        .with_disabled(props.disabled);
    
    style_generator.typography_generator = style_generator.typography_generator
        .with_delete(props.delete)
        .with_underline(props.underline)
        .with_strong(props.strong)
        .with_italic(props.italic)
        .with_mark(props.mark)
        .with_code(props.code)
        .with_keyboard(props.keyboard)
        .with_copyable(props.copyable.is_some())
        .with_editable(props.editable.is_some())
        .with_ellipsis(props.ellipsis.is_some())
        .with_ellipsis_rows(props.ellipsis.as_ref().and_then(|e| e.rows));

    let class_name = format!("{} {}", 
        style_generator.generate_class(),
        props.class.clone().unwrap_or_default()
    );

    let tag = match props.level {
        HeadingLevel::H1 => "h1",
        HeadingLevel::H2 => "h2",
        HeadingLevel::H3 => "h3",
        HeadingLevel::H4 => "h4",
        HeadingLevel::H5 => "h5",
    };

    rsx! {
        {tag} {
            class: class_name,
            style: props.style.clone(),
            {props.children}
            if let Some(copyable) = &props.copyable {
                CopyButton {
                    text: copyable.text.clone(),
                    format: Some(copyable.format.clone()),
                    tooltip: copyable.tooltips.as_ref().map(|(before, _)| before.clone()),
                    on_copy: copyable.on_copy.clone(),
                }
            }
            if let Some(editable) = &props.editable {
                EditButton {
                    text: editable.text.clone(),
                    tooltip: editable.tooltip.clone(),
                    trigger_type: editable.trigger_type.first().cloned(),
                    on_start: editable.on_start.clone(),
                    on_change: editable.on_change.clone(),
                    on_cancel: editable.on_cancel.clone(),
                    on_end: editable.on_end.clone(),
                    max_length: editable.max_length,
                    auto_size: editable.auto_size.clone(),
                    enter_icon: editable.enter_icon.clone(),
                    editing: editable.editing,
                }
            }
        }
    }
}

/// Text 组件
#[component]
pub fn Text(props: TextProps) -> Element {
    let theme = use_theme();
    let text_type = props.r#type.as_ref().unwrap_or(&TextType::Default);
    
    let style_generator = TypographyStyleGenerator::new()
        .with_type(text_type.clone())
        .with_disabled(props.disabled)
        .with_delete(props.delete)
        .with_underline(props.underline)
        .with_strong(props.strong)
        .with_italic(props.italic)
        .with_mark(props.mark)
        .with_code(props.code)
        .with_keyboard(props.keyboard)
        .with_copyable(props.copyable.is_some())
        .with_editable(props.editable.is_some())
        .with_ellipsis(props.ellipsis.is_some())
        .with_ellipsis_rows(props.ellipsis.as_ref().and_then(|e| e.rows));

    let class_name = format!("{} {}", 
        style_generator.generate_class(),
        props.class.clone().unwrap_or_default()
    );

    rsx! {
        span {
            class: "{class_name}",
            style: props.style.clone(),
            {props.children}
            if let Some(copyable) = &props.copyable {
                CopyButton {
                    text: copyable.text.clone(),
                    format: Some(copyable.format.clone()),
                    tooltip: copyable.tooltips.as_ref().map(|(before, _)| before.clone()),
                    on_copy: copyable.on_copy.clone(),
                }
            }
            if let Some(editable) = &props.editable {
                EditButton {
                    text: editable.text.clone(),
                    tooltip: editable.tooltip.clone(),
                    trigger_type: editable.trigger_type.first().cloned(),
                    on_start: editable.on_start.clone(),
                    on_change: editable.on_change.clone(),
                    on_cancel: editable.on_cancel.clone(),
                    on_end: editable.on_end.clone(),
                    max_length: editable.max_length,
                    auto_size: editable.auto_size.clone(),
                    enter_icon: editable.enter_icon.clone(),
                    editing: editable.editing,
                }
            }
        }
    }
}

/// Paragraph 组件
#[component]
pub fn Paragraph(props: ParagraphProps) -> Element {
    let theme = use_theme();
    let text_type = props.r#type.as_ref().unwrap_or(&TextType::Default);
    
    let style_generator = ParagraphStyleGenerator::new()
        .with_type(text_type.clone())
        .with_disabled(props.disabled)
        .with_delete(props.delete)
        .with_underline(props.underline)
        .with_strong(props.strong)
        .with_italic(props.italic)
        .with_mark(props.mark)
        .with_code(props.code)
        .with_keyboard(props.keyboard)
        .with_copyable(props.copyable.is_some())
        .with_editable(props.editable.is_some())
        .with_ellipsis(props.ellipsis.is_some())
        .with_ellipsis_rows(props.ellipsis.as_ref().and_then(|e| e.rows));

    let class_name = format!("{} {}", 
        style_generator.generate_class(),
        props.class.clone().unwrap_or_default()
    );

    rsx! {
        p {
            class: "{class_name}",
            style: props.style.clone(),
            {props.children}
            if let Some(copyable) = &props.copyable {
                CopyButton {
                    text: copyable.text.clone(),
                    format: Some(copyable.format.clone()),
                    tooltip: copyable.tooltips.as_ref().map(|(before, _)| before.clone()),
                    on_copy: copyable.on_copy.clone(),
                }
            }
            if let Some(editable) = &props.editable {
                EditButton {
                    text: editable.text.clone(),
                    tooltip: editable.tooltip.clone(),
                    trigger_type: editable.trigger_type.first().cloned(),
                    on_start: editable.on_start.clone(),
                    on_change: editable.on_change.clone(),
                    on_cancel: editable.on_cancel.clone(),
                    on_end: editable.on_end.clone(),
                    max_length: editable.max_length,
                    auto_size: editable.auto_size.clone(),
                    enter_icon: editable.enter_icon.clone(),
                    editing: editable.editing,
                }
            }
        }
    }
}

/// Typography Link 组件
#[component]
pub fn TypographyLink(props: super::types::LinkProps) -> Element {
    let theme = use_theme();
    let link_type = props.r#type.as_ref().unwrap_or(&LinkType::Default);
    
    let style_generator = LinkStyleGenerator::new()
        .with_type(link_type.clone())
        .with_disabled(props.disabled)
        .with_block(props.block);
    
    let mut typography_generator = style_generator.typography_generator.clone()
        .with_delete(props.delete)
        .with_underline(props.underline)
        .with_strong(props.strong)
        .with_italic(props.italic)
        .with_mark(props.mark)
        .with_code(props.code)
        .with_keyboard(props.keyboard)
        .with_copyable(props.copyable.is_some())
        .with_ellipsis(props.ellipsis.is_some())
        .with_ellipsis_rows(props.ellipsis.as_ref().and_then(|e| e.rows));

    let class_name = format!("{} {} {}", 
        style_generator.generate_class(),
        typography_generator.generate_class(),
        props.class.clone().unwrap_or_default()
    );

    let onclick = move |_| {
        if !props.disabled {
            if let Some(onclick_handler) = &props.onclick {
                // 创建一个空的MouseEvent - 需要正确的构造方法
                // onclick_handler.call(MouseEvent::default());
            }
        }
    };

    rsx! {
        a {
            class: "{class_name}",
            style: props.style.clone(),
            id: props.id.clone(),
            href: props.href.clone(),
            target: props.target.as_ref().map(|t| t.as_str()),
            rel: if props.target == Some(LinkTarget::Blank) { Some("noopener noreferrer") } else { None },
            onclick: onclick,
            {props.children}
            if let Some(copyable) = &props.copyable {
                CopyButton {
                    text: copyable.text.clone(),
                    format: Some(copyable.format.clone()),
                    tooltip: copyable.tooltips.as_ref().map(|(before, _)| before.clone()),
                    on_copy: copyable.on_copy.clone(),
                }
            }
        }
    }
}

/// 复制按钮组件
#[component]
fn CopyButton(
    text: Option<String>,
    format: Option<CopyFormat>,
    tooltip: Option<String>,
    on_copy: Option<Callback<String>>,
) -> Element {
    let action_class = TypographyStyleGenerator::action_button_style();

    let onclick = move |_| {
        if let Some(text) = &text {
            // TODO: 实现实际的复制功能
            // 这里需要使用 web-sys 或其他方式访问剪贴板 API
            if let Some(on_copy) = &on_copy {
                on_copy.call(text.clone());
            }
        }
    };

    rsx! {
        button {
            class: "{action_class}",
            title: tooltip.as_deref().unwrap_or("复制"),
            onclick: onclick,
            // 复制图标 (简化版)
            "📋"
        }
    }
}

/// 编辑按钮组件
#[component]
fn EditButton(
    text: Option<String>,
    tooltip: Option<String>,
    trigger_type: Option<EditTriggerType>,
    on_start: Option<Callback<()>>,
    on_change: Option<Callback<String>>,
    on_cancel: Option<Callback<()>>,
    on_end: Option<Callback<String>>,
    max_length: Option<u32>,
    auto_size: Option<AutoSizeConfig>,
    enter_icon: Option<Element>,
    editing: bool,
) -> Element {
    let mut edit_text = use_signal(|| text.clone().unwrap_or_default());
    let mut is_editing = use_signal(|| editing);

    let action_class = TypographyStyleGenerator::action_button_style();
    let input_class = EditStyleGenerator::edit_input_style();
    let actions_class = EditStyleGenerator::edit_actions_style();
    let action_btn_class = EditStyleGenerator::edit_action_button_style();

    let start_edit = move |_| {
        is_editing.set(true);
        if let Some(on_start) = &on_start {
            on_start.call(());
        }
    };

    let confirm_edit = move |_| {
        is_editing.set(false);
        if let Some(on_end) = &on_end {
            on_end.call(edit_text.read().clone());
        }
    };

    let cancel_edit = move |_| {
        is_editing.set(false);
        edit_text.set(text.clone().unwrap_or_default());
        if let Some(on_cancel) = &on_cancel {
            on_cancel.call(());
        }
    };

    let on_input = move |evt: FormEvent| {
        let value = evt.value();
        if let Some(max_len) = max_length {
            if value.len() <= max_len as usize {
                edit_text.set(value.clone());
                if let Some(on_change) = &on_change {
                    on_change.call(value);
                }
            }
        } else {
            edit_text.set(value.clone());
            if let Some(on_change) = &on_change {
                on_change.call(value);
            }
        }
    };

    if *is_editing.read() {
        rsx! {
            span {
                class: "typography-edit-container",
                input {
                    class: "{input_class}",
                    r#type: "text",
                    value: "{edit_text.read()}",
                    oninput: on_input,
                    autofocus: true,
                }
                span {
                    class: "{actions_class}",
                    button {
                        class: "{action_btn_class}",
                        title: "确认",
                        onclick: confirm_edit,
                        "✓"
                    }
                    button {
                        class: "{action_btn_class}",
                        title: "取消",
                        onclick: cancel_edit,
                        "✕"
                    }
                }
            }
        }
    } else {
        rsx! {
            button {
                class: "{action_class}",
                title: tooltip.as_deref().unwrap_or("编辑"),
                onclick: start_edit,
                // 编辑图标 (简化版)
                "✏️"
            }
        }
    }
}

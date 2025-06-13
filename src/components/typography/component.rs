//! Typography 组件实现
//!
//! 本模块包含Typography组件的所有组件实现，包括Title、Text、Paragraph和Link等。

use super::styles::*;
use super::types::*;
use crate::theme::use_theme;
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

    let class_name = format!(
        "{} {}",
        style_generator.generate_class(),
        props.class.clone().unwrap_or_default()
    );

    let mut style = props.style.clone().unwrap_or_default();

    // 处理省略功能的样式
    if let Some(ellipsis) = &props.ellipsis {
        if let Some(suffix) = &ellipsis.suffix {
            style.push_str("max-width: 100%;");
        }
    }

    rsx! {
        if let Some(ellipsis) = &props.ellipsis {
            EllipsisWrapper {
                ellipsis: ellipsis.clone(),
                class_name: class_name.clone(),
                style: style.clone(),
                children: props.children,
            }
        } else {
            span {
                class: "{class_name}",
                style: "{style}",
                {props.children}
            }
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

    style_generator.typography_generator = style_generator
        .typography_generator
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

    let class_name = format!(
        "{} {}",
        style_generator.generate_class(),
        props.class.clone().unwrap_or_default()
    );

    rsx! {
       match props.level {
           HeadingLevel::H1 => rsx! {
               h1 {
                   class: class_name,
                   style: props.style.clone(),
                   role: "heading",
                   "aria-level": "1",
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
                           editing: editable.editing,
                       }
                   }
               }
           },
           HeadingLevel::H2 => rsx! {
               h2 {
                   class: class_name,
                   style: props.style.clone(),
                   role: "heading",
                   "aria-level": "2",
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
                           editing: editable.editing,
                       }
                   }
               }
           },
           HeadingLevel::H3 => rsx! {
               h3 {
                   class: class_name,
                   style: props.style.clone(),
                   role: "heading",
                   "aria-level": "3",
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
                           editing: editable.editing,
                       }
                   }
               }
           },
           HeadingLevel::H4 => rsx! {
               h4 {
                   class: class_name,
                   style: props.style.clone(),
                   role: "heading",
                   "aria-level": "4",
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
                           editing: editable.editing,
                       }
                   }
               }
           },
           HeadingLevel::H5 => rsx! {
               h5 {
                   class: class_name,
                   style: props.style.clone(),
                   role: "heading",
                   "aria-level": "5",
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
           },
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
        .with_ellipsis_rows(if props.ellipsis.is_some() {
            Some(1)
        } else {
            None
        });

    let class_name = format!(
        "{} {}",
        style_generator.generate_class(),
        props.class.clone().unwrap_or_default()
    );

    rsx! {
        span {
            class: "{class_name}",
            style: props.style.clone(),
            role: if props.editable.is_some() { "textbox" } else { "text" },
            "aria-readonly": if props.editable.is_some() { "false" } else { "true" },
            "aria-disabled": if props.disabled { "true" } else { "false" },
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

    let class_name = format!(
        "{} {}",
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

    let mut typography_generator = style_generator
        .typography_generator
        .clone()
        .with_delete(props.delete)
        .with_underline(props.underline)
        .with_strong(props.strong)
        .with_italic(props.italic)
        .with_mark(props.mark)
        .with_code(props.code)
        .with_keyboard(props.keyboard)
        .with_copyable(props.copyable.is_some())
        .with_ellipsis(props.ellipsis.is_some())
        .with_ellipsis_rows(if props.ellipsis.is_some() {
            Some(1)
        } else {
            None
        });

    let class_name = format!(
        "{} {} {}",
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
    let (copy_to_clipboard, copied) = crate::hooks::use_clipboard();

    let onclick = move |_| {
        if let Some(text) = &text {
            // 使用 use_clipboard hook 实现复制功能
            copy_to_clipboard(&text);

            // 调用用户提供的回调
            if let Some(on_copy) = &on_copy {
                on_copy.call(text.clone());
            }
        }
    };

    let button_title = if copied() {
        "已复制!"
    } else {
        tooltip.as_deref().unwrap_or("复制")
    };

    let button_icon = if copied() {
        "✓" // 复制成功图标
    } else {
        "📋" // 默认复制图标
    };

    rsx! {
        button {
            class: "{action_class}",
            title: "{button_title}",
            onclick: onclick,
            disabled: copied(),
            "aria-label": "{button_title}",
            "aria-pressed": "{copied()}",
            role: "button",
            "{button_icon}"
        }
    }
}

/// 省略功能包装组件
#[component]
fn EllipsisWrapper(
    ellipsis: EllipsisConfig,
    class_name: String,
    style: String,
    children: Element,
) -> Element {
    let mut is_expanded = use_signal(|| ellipsis.expanded.unwrap_or(ellipsis.default_expanded));

    let toggle_expand = move |_| {
        let new_state = !is_expanded();
        is_expanded.set(new_state);
        if let Some(on_expand) = &ellipsis.on_expand {
            on_expand.call(new_state);
        }
    };

    // 如果可展开且当前是收起状态，应用省略样式
    let should_ellipsis = ellipsis.expandable && !is_expanded();

    rsx! {
        span {
            class: "{class_name}",
            style: "{style}",
            span {
                class: if should_ellipsis { "typography-ellipsis-content" } else { "" },
                {children}
            }
            if ellipsis.expandable {
                button {
                    class: "typography-expand-button",
                    onclick: toggle_expand,
                    title: if is_expanded() { "收起" } else { "展开" },
                    if is_expanded() {
                        if ellipsis.collapsible {
                            "收起"
                        }
                    } else {
                        "展开"
                    }
                }
            }
            if let Some(suffix) = &ellipsis.suffix {
                span { class: "typography-ellipsis-suffix", "{suffix}" }
            }
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

    let text_clone = text.clone();
    let cancel_edit = move |_| {
        is_editing.set(false);
        edit_text.set(text_clone.clone().unwrap_or_default());
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

    let on_keydown = {
        let on_end = on_end.clone();
        let on_cancel = on_cancel.clone();
        let text = text.clone();
        let mut edit_text = edit_text.clone();
        let mut is_editing = is_editing.clone();

        move |evt: KeyboardEvent| match evt.key() {
            Key::Enter => {
                evt.prevent_default();
                is_editing.set(false);
                if let Some(on_end) = &on_end {
                    on_end.call(edit_text.read().clone());
                }
            }
            Key::Escape => {
                evt.prevent_default();
                is_editing.set(false);
                edit_text.set(text.clone().unwrap_or_default());
                if let Some(on_cancel) = &on_cancel {
                    on_cancel.call(());
                }
            }
            _ => {}
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
                    onkeydown: on_keydown,
                    autofocus: true,
                    maxlength: max_length.map(|len| len.to_string()),
                    "aria-label": "编辑文本",
                    "aria-describedby": "edit-help",
                    role: "textbox",
                }
                span {
                    class: "{actions_class}",
                    button {
                        class: "{action_btn_class}",
                        title: "确认",
                        onclick: confirm_edit,
                        "aria-label": "确认编辑",
                        role: "button",
                        "✓"
                    }
                    button {
                        class: "{action_btn_class}",
                        title: "取消",
                        onclick: cancel_edit,
                        "aria-label": "取消编辑",
                        role: "button",
                        "✕"
                    }
                    span {
                        id: "edit-help",
                        class: "sr-only",
                        "按Enter确认，按Escape取消"
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
                "aria-label": tooltip.as_deref().unwrap_or("编辑文本"),
                role: "button",
                // 编辑图标 (简化版)
                "✏️"
            }
        }
    }
}

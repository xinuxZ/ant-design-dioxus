//! Input 组件的核心实现
//!
//! 本模块包含 Input 组件及其子组件的核心实现逻辑，
//! 从类型定义中分离出来，提高代码的可维护性。

use dioxus::prelude::*;
use crate::components::input::types::*;
use crate::components::input::styles::InputStyleGenerator;
use crate::theme::use_theme;

/// Input 组件
#[component]
pub fn Input(props: InputProps) -> Element {
    let theme = use_theme();
    let mut internal_value = use_signal(|| props.value.clone().unwrap_or_default());
    let mut is_focused = use_signal(|| false);

    // 同步外部 value 到内部状态
    use_effect(move || {
        if let Some(value) = &props.value {
            internal_value.set(value.clone());
        }
    });

    // 事件处理器
    let handle_input = move |evt: FormEvent| {
        let value = evt.value();
        internal_value.set(value.clone());
        if let Some(on_change) = &props.on_change {
            on_change.call(value);
        }
    };

    let handle_keydown = move |evt: KeyboardEvent| {
        if evt.key() == Key::Enter {
            if let Some(on_press_enter) = &props.on_press_enter {
                on_press_enter.call(evt);
            }
        }
    };

    let handle_focus = move |evt: FocusEvent| {
        is_focused.set(true);
        if let Some(on_focus) = &props.on_focus {
            on_focus.call(evt);
        }
    };

    let handle_blur = move |evt: FocusEvent| {
        is_focused.set(false);
        if let Some(on_blur) = &props.on_blur {
            on_blur.call(evt);
        }
    };

    let handle_clear = move |evt: MouseEvent| {
        internal_value.set(String::new());
        if let Some(on_change) = &props.on_change {
            on_change.call(String::new());
        }
        if let Some(on_clear) = &props.on_clear {
            on_clear.call(evt);
        }
    };

    // 生成样式
    let style_generator = InputStyleGenerator::new()
        .with_size(props.size.clone().into())
        .with_status(props.status.clone().into())
        .with_disabled(props.disabled)
        .with_bordered(props.bordered)
        .with_allow_clear(props.allow_clear)
        .with_prefix(props.prefix.is_some())
        .with_suffix(props.suffix.is_some())
        .with_show_count(props.show_count)
        .with_addon_before(props.addon_before.is_some())
        .with_addon_after(props.addon_after.is_some())
        .with_focused(is_focused())
        .with_token(theme.alias_token.clone());

    let input_style = style_generator.generate_css();

    // 获取 CSS 类名
    let input_class = get_input_css_class(&props, is_focused());
    let wrapper_class = get_wrapper_css_class(&props, is_focused());
    let group_class = get_group_css_class(&props);

    // 计算字符数
    let char_count = internal_value().chars().count();
    let max_length = props.max_length.unwrap_or(usize::MAX);

    // 渲染逻辑
    if !group_class.is_empty() {
        // 有前置或后置标签的情况
        rsx! {
            style { {input_style} }

            div {
                class: group_class,
                style: props.style.clone().unwrap_or_default(),

                div {
                    class: "ant-input-group",

                    if let Some(addon_before) = props.addon_before.clone() {
                        div {
                            class: "ant-input-group-addon",
                            {addon_before}
                        }
                    }

                    if !wrapper_class.is_empty() {
                        span {
                            class: wrapper_class,

                            if let Some(prefix) = props.prefix.clone() {
                                span {
                                    class: "ant-input-prefix",
                                    {prefix}
                                }
                            }

                            input {
                                class: input_class,
                                r#type: props.input_type.clone(),
                                value: internal_value(),
                                placeholder: props.placeholder.clone(),
                                disabled: props.disabled,
                                readonly: props.readonly,
                                maxlength: if max_length != usize::MAX { max_length.to_string() } else { String::new() },
                                oninput: handle_input,
                                onkeydown: handle_keydown,
                                onfocus: handle_focus,
                                onblur: handle_blur
                            }

                            if props.allow_clear && !internal_value().is_empty() && !props.disabled {
                                span {
                                    class: "ant-input-clear-icon",
                                    onclick: handle_clear,
                                    "×"
                                }
                            }

                            if let Some(suffix) = props.suffix.clone() {
                                span {
                                    class: "ant-input-suffix",
                                    {suffix}
                                }
                            }
                        }
                    } else {
                        input {
                            class: input_class,
                            r#type: props.input_type.clone(),
                            value: internal_value(),
                            placeholder: props.placeholder.clone(),
                            disabled: props.disabled,
                            readonly: props.readonly,
                            maxlength: if max_length != usize::MAX { max_length.to_string() } else { String::new() },
                            oninput: handle_input,
                            onkeydown: handle_keydown,
                            onfocus: handle_focus,
                            onblur: handle_blur
                        }
                    }

                    if let Some(addon_after) = props.addon_after.clone() {
                        div {
                            class: "ant-input-group-addon",
                            {addon_after}
                        }
                    }
                }

                if props.show_count {
                    div {
                        class: "ant-input-show-count-suffix",

                        {
                            if max_length != usize::MAX {
                                format!("{} / {}", char_count, max_length)
                            } else {
                                char_count.to_string()
                            }
                        }
                    }
                }
            }
        }
    } else if !wrapper_class.is_empty() {
        // 有前缀或后缀的情况
        rsx! {
            style { {input_style} }

            div {
                class: wrapper_class,
                style: props.style.clone().unwrap_or_default(),

                if let Some(prefix) = props.prefix.clone() {
                    span {
                        class: "ant-input-prefix",
                        {prefix}
                    }
                }

                input {
                    class: input_class,
                    r#type: props.input_type.clone(),
                    value: internal_value(),
                    placeholder: props.placeholder.clone(),
                    disabled: props.disabled,
                    readonly: props.readonly,
                    maxlength: if max_length != usize::MAX { max_length.to_string() } else { String::new() },
                    oninput: handle_input,
                    onkeydown: handle_keydown,
                    onfocus: handle_focus,
                    onblur: handle_blur
                }

                if props.allow_clear && !internal_value().is_empty() && !props.disabled {
                    span {
                        class: "ant-input-clear-icon",
                        onclick: handle_clear,
                        "×"
                    }
                }

                if let Some(suffix) = props.suffix.clone() {
                    span {
                        class: "ant-input-suffix",
                        {suffix}
                    }
                }

                if props.show_count {
                    span {
                        class: "ant-input-show-count-suffix",

                        {
                            if max_length != usize::MAX {
                                format!("{} / {}", char_count, max_length)
                            } else {
                                char_count.to_string()
                            }
                        }
                    }
                }
            }
        }
    } else {
        // 基本输入框
        rsx! {
            style { {input_style} }

            input {
                class: input_class,
                style: props.style.clone().unwrap_or_default(),
                r#type: props.input_type.clone(),
                value: internal_value(),
                placeholder: props.placeholder.clone(),
                disabled: props.disabled,
                readonly: props.readonly,
                maxlength: if max_length != usize::MAX { max_length.to_string() } else { String::new() },
                oninput: handle_input,
                onkeydown: handle_keydown,
                onfocus: handle_focus,
                onblur: handle_blur
            }

            if props.show_count {
                div {
                    class: "ant-input-show-count-suffix",
                    style: "margin-top: 4px;",

                    {
                        if max_length != usize::MAX {
                            format!("{} / {}", char_count, max_length)
                        } else {
                            char_count.to_string()
                        }
                    }
                }
            }
        }
    }
}

/// Input.TextArea 组件
#[component]
pub fn TextArea(props: TextAreaProps) -> Element {
    let theme = use_theme();
    let mut internal_value = use_signal(|| props.value.clone().unwrap_or_default());
    let mut is_focused = use_signal(|| false);

    // 同步外部 value 到内部状态
    use_effect(move || {
        if let Some(value) = &props.value {
            internal_value.set(value.clone());
        }
    });

    // 事件处理器
    let handle_input = move |evt: FormEvent| {
        let value = evt.value();
        internal_value.set(value.clone());
        if let Some(on_change) = &props.on_change {
            on_change.call(value);
        }
    };

    let handle_keydown = move |evt: KeyboardEvent| {
        if evt.key() == Key::Enter {
            if let Some(on_press_enter) = &props.on_press_enter {
                on_press_enter.call(evt);
            }
        }
    };

    let handle_focus = move |evt: FocusEvent| {
        is_focused.set(true);
        if let Some(on_focus) = &props.on_focus {
            on_focus.call(evt);
        }
    };

    let handle_blur = move |evt: FocusEvent| {
        is_focused.set(false);
        if let Some(on_blur) = &props.on_blur {
            on_blur.call(evt);
        }
    };

    let handle_clear = move |evt: MouseEvent| {
        internal_value.set(String::new());
        if let Some(on_change) = &props.on_change {
            on_change.call(String::new());
        }
        if let Some(on_clear) = &props.on_clear {
            on_clear.call(evt);
        }
    };

    // 生成样式
    let style_generator = InputStyleGenerator::new()
        .with_size(props.size.clone().into())
        .with_status(props.status.clone().into())
        .with_disabled(props.disabled)
        .with_bordered(props.bordered)
        .with_allow_clear(props.allow_clear)
        .with_show_count(props.show_count)
        .with_focused(is_focused())
        .with_token(theme.alias_token.clone());

    let textarea_style = style_generator.generate_textarea_css();

    // 计算字符数
    let char_count = internal_value().chars().count();
    let max_length = props.max_length.unwrap_or(usize::MAX);

    // 计算样式
    let mut textarea_class = vec!["ant-input"];
    
    match props.size {
        InputSize::Large => textarea_class.push("ant-input-lg"),
        InputSize::Small => textarea_class.push("ant-input-sm"),
        _ => {}
    }
    
    match props.status {
        InputStatus::Error => textarea_class.push("ant-input-status-error"),
        InputStatus::Warning => textarea_class.push("ant-input-status-warning"),
        _ => {}
    }
    
    if props.disabled {
        textarea_class.push("ant-input-disabled");
    }
    
    if !props.bordered {
        textarea_class.push("ant-input-borderless");
    }
    
    if is_focused() {
        textarea_class.push("ant-input-focused");
    }
    
    if let Some(class) = &props.class {
        textarea_class.push(class);
    }

    let resize_style = if props.resize { "resize: vertical;" } else { "resize: none;" };

    rsx! {
        style { {textarea_style} }
        
        div {
            class: "ant-input-textarea",
            style: props.style.clone().unwrap_or_default(),
            
            textarea {
                class: textarea_class.join(" "),
                style: resize_style,
                value: internal_value(),
                placeholder: props.placeholder.clone(),
                disabled: props.disabled,
                readonly: props.readonly,
                rows: props.rows.to_string(),
                maxlength: if max_length != usize::MAX { max_length.to_string() } else { String::new() },
                oninput: handle_input,
                onkeydown: handle_keydown,
                onfocus: handle_focus,
                onblur: handle_blur
            }
            
            if props.allow_clear && !internal_value().is_empty() && !props.disabled {
                span {
                    class: "ant-input-clear-icon ant-input-textarea-clear-icon",
                    onclick: handle_clear,
                    "×"
                }
            }
            
            if props.show_count {
                div {
                    class: "ant-input-textarea-show-count",
                    {
                        if max_length != usize::MAX {
                            format!("{} / {}", char_count, max_length)
                        } else {
                            char_count.to_string()
                        }
                    }
                }
            }
        }
    }
}

/// Input.Search 组件
#[component]
pub fn Search(props: SearchProps) -> Element {
    let theme = use_theme();
    let mut internal_value = use_signal(|| props.value.clone().unwrap_or_default());
    let mut is_focused = use_signal(|| false);

    // 同步外部 value 到内部状态
    use_effect(move || {
        if let Some(value) = &props.value {
            internal_value.set(value.clone());
        }
    });

    // 事件处理器
    let handle_input = move |evt: FormEvent| {
        let value = evt.value();
        internal_value.set(value.clone());
        if let Some(on_change) = &props.on_change {
            on_change.call(value);
        }
    };

    let handle_search = move |_evt: MouseEvent| {
        if let Some(on_search) = &props.on_search {
            on_search.call(internal_value());
        }
    };

    let handle_keydown = move |evt: KeyboardEvent| {
        if evt.key() == Key::Enter {
            if let Some(on_search) = &props.on_search {
                on_search.call(internal_value());
            }
        }
    };

    let handle_focus = move |evt: FocusEvent| {
        is_focused.set(true);
        if let Some(on_focus) = &props.on_focus {
            on_focus.call(evt);
        }
    };

    let handle_blur = move |evt: FocusEvent| {
        is_focused.set(false);
        if let Some(on_blur) = &props.on_blur {
            on_blur.call(evt);
        }
    };

    let handle_clear = move |evt: MouseEvent| {
        internal_value.set(String::new());
        if let Some(on_change) = &props.on_change {
            on_change.call(String::new());
        }
        if let Some(on_clear) = &props.on_clear {
            on_clear.call(evt);
        }
    };

    // 生成样式
    let style_generator = InputStyleGenerator::new()
        .with_size(props.size.clone().into())
        .with_status(props.status.clone().into())
        .with_disabled(props.disabled)
        .with_bordered(props.bordered)
        .with_allow_clear(props.allow_clear)
        .with_prefix(props.prefix.is_some())
        .with_suffix(true) // Search 总是有后缀（搜索按钮）
        .with_show_count(props.show_count)
        .with_focused(is_focused())
        .with_token(theme.alias_token.clone());

    let search_style = style_generator.generate_search_css();

    // 计算字符数
    let char_count = internal_value().chars().count();
    let max_length = props.max_length.unwrap_or(usize::MAX);

    // 计算样式
    let mut input_class = vec!["ant-input"];
    let mut wrapper_class = vec!["ant-input-search", "ant-input-affix-wrapper"];
    
    match props.size {
        InputSize::Large => {
            input_class.push("ant-input-lg");
            wrapper_class.push("ant-input-lg");
        }
        InputSize::Small => {
            input_class.push("ant-input-sm");
            wrapper_class.push("ant-input-sm");
        }
        _ => {}
    }
    
    match props.status {
        InputStatus::Error => {
            input_class.push("ant-input-status-error");
            wrapper_class.push("ant-input-status-error");
        }
        InputStatus::Warning => {
            input_class.push("ant-input-status-warning");
            wrapper_class.push("ant-input-status-warning");
        }
        _ => {}
    }
    
    if props.disabled {
        input_class.push("ant-input-disabled");
        wrapper_class.push("ant-input-affix-wrapper-disabled");
    }
    
    if !props.bordered {
        input_class.push("ant-input-borderless");
        wrapper_class.push("ant-input-affix-wrapper-borderless");
    }
    
    if is_focused() {
        input_class.push("ant-input-focused");
        wrapper_class.push("ant-input-affix-wrapper-focused");
    }
    
    if let Some(class) = &props.class {
        wrapper_class.push(class);
    }

    rsx! {
        style { {search_style} }
        
        div {
            class: wrapper_class.join(" "),
            style: props.style.clone().unwrap_or_default(),
            
            if let Some(prefix) = props.prefix.clone() {
                span {
                    class: "ant-input-prefix",
                    {prefix}
                }
            }
            
            input {
                class: input_class.join(" "),
                r#type: "text",
                value: internal_value(),
                placeholder: props.placeholder.clone(),
                disabled: props.disabled,
                readonly: props.readonly,
                maxlength: if max_length != usize::MAX { max_length.to_string() } else { String::new() },
                oninput: handle_input,
                onkeydown: handle_keydown,
                onfocus: handle_focus,
                onblur: handle_blur
            }
            
            if props.allow_clear && !internal_value().is_empty() && !props.disabled {
                span {
                    class: "ant-input-clear-icon",
                    onclick: handle_clear,
                    "×"
                }
            }
            
            span {
                class: "ant-input-suffix",
                
                if props.enter_button.is_some() {
                    button {
                        class: "ant-btn ant-input-search-button",
                        r#type: "button",
                        disabled: props.disabled || props.loading,
                        onclick: handle_search,
                        
                        if props.loading {
                            span { class: "ant-btn-loading-icon", "⟳" }
                        } else {
                            {props.enter_button.clone().unwrap_or_else(|| "搜索".to_string())}
                        }
                    }
                } else if props.search_icon {
                    span {
                        class: "ant-input-search-icon",
                        onclick: handle_search,
                        
                        if props.loading {
                            span { class: "ant-btn-loading-icon", "⟳" }
                        } else {
                            "🔍"
                        }
                    }
                }
            }
            
            if props.show_count {
                span {
                    class: "ant-input-show-count-suffix",
                    {
                        if max_length != usize::MAX {
                            format!("{} / {}", char_count, max_length)
                        } else {
                            char_count.to_string()
                        }
                    }
                }
            }
        }
    }
}

/// Input.Password 组件
#[component]
pub fn Password(props: PasswordProps) -> Element {
    let theme = use_theme();
    let mut internal_value = use_signal(|| props.value.clone().unwrap_or_default());
    let mut is_focused = use_signal(|| false);
    let mut visible = use_signal(|| false);

    // 同步外部 value 到内部状态
    use_effect(move || {
        if let Some(value) = &props.value {
            internal_value.set(value.clone());
        }
    });

    // 事件处理器
    let handle_input = move |evt: FormEvent| {
        let value = evt.value();
        internal_value.set(value.clone());
        if let Some(on_change) = &props.on_change {
            on_change.call(value);
        }
    };

    let handle_keydown = move |evt: KeyboardEvent| {
        if evt.key() == Key::Enter {
            if let Some(on_press_enter) = &props.on_press_enter {
                on_press_enter.call(evt);
            }
        }
    };

    let handle_focus = move |evt: FocusEvent| {
        is_focused.set(true);
        if let Some(on_focus) = &props.on_focus {
            on_focus.call(evt);
        }
    };

    let handle_blur = move |evt: FocusEvent| {
        is_focused.set(false);
        if let Some(on_blur) = &props.on_blur {
            on_blur.call(evt);
        }
    };

    let toggle_visibility = move |_evt: MouseEvent| {
        visible.set(!visible());
    };

    // 生成样式
    let style_generator = InputStyleGenerator::new()
        .with_size(props.size.clone().into())
        .with_status(props.status.clone().into())
        .with_disabled(props.disabled)
        .with_bordered(props.bordered)
        .with_prefix(props.prefix.is_some())
        .with_suffix(props.visibility_toggle) // 密码框总是有后缀（可见性切换）
        .with_show_count(props.show_count)
        .with_focused(is_focused())
        .with_token(theme.alias_token.clone());

    let password_style = style_generator.generate_password_css();

    // 计算字符数
    let char_count = internal_value().chars().count();
    let max_length = props.max_length.unwrap_or(usize::MAX);

    // 计算样式
    let mut input_class = vec!["ant-input"];
    let mut wrapper_class = vec!["ant-input-password", "ant-input-affix-wrapper"];
    
    match props.size {
        InputSize::Large => {
            input_class.push("ant-input-lg");
            wrapper_class.push("ant-input-lg");
        }
        InputSize::Small => {
            input_class.push("ant-input-sm");
            wrapper_class.push("ant-input-sm");
        }
        _ => {}
    }
    
    match props.status {
        InputStatus::Error => {
            input_class.push("ant-input-status-error");
            wrapper_class.push("ant-input-status-error");
        }
        InputStatus::Warning => {
            input_class.push("ant-input-status-warning");
            wrapper_class.push("ant-input-status-warning");
        }
        _ => {}
    }
    
    if props.disabled {
        input_class.push("ant-input-disabled");
        wrapper_class.push("ant-input-affix-wrapper-disabled");
    }
    
    if !props.bordered {
        input_class.push("ant-input-borderless");
        wrapper_class.push("ant-input-affix-wrapper-borderless");
    }
    
    if is_focused() {
        input_class.push("ant-input-focused");
        wrapper_class.push("ant-input-affix-wrapper-focused");
    }
    
    if let Some(class) = &props.class {
        wrapper_class.push(class);
    }

    let input_type = if visible() { "text" } else { "password" };

    rsx! {
        style { {password_style} }
        
        div {
            class: wrapper_class.join(" "),
            style: props.style.clone().unwrap_or_default(),
            
            if let Some(prefix) = props.prefix.clone() {
                span {
                    class: "ant-input-prefix",
                    {prefix}
                }
            }
            
            input {
                class: input_class.join(" "),
                r#type: input_type,
                value: internal_value(),
                placeholder: props.placeholder.clone(),
                disabled: props.disabled,
                readonly: props.readonly,
                maxlength: if max_length != usize::MAX { max_length.to_string() } else { String::new() },
                oninput: handle_input,
                onkeydown: handle_keydown,
                onfocus: handle_focus,
                onblur: handle_blur
            }
            
            span {
                class: "ant-input-suffix",
                
                if props.visibility_toggle {
                    span {
                        class: "ant-input-password-icon",
                        onclick: toggle_visibility,
                        
                        if let Some(icon_render) = props.icon_render {
                            {icon_render(visible())}
                        } else {
                            if visible() {
                                "👁️"
                            } else {
                                "🙈"
                            }
                        }
                    }
                }
            }
            
            if props.show_count {
                span {
                    class: "ant-input-show-count-suffix",
                    {
                        if max_length != usize::MAX {
                            format!("{} / {}", char_count, max_length)
                        } else {
                            char_count.to_string()
                        }
                    }
                }
            }
        }
    }
}

/// Input.OTP 组件
#[component]
pub fn OTP(props: OTPProps) -> Element {
    let theme = use_theme();
    let mut internal_values = use_signal(|| {
        let value = props.value.clone().unwrap_or_default();
        let mut values = vec![String::new(); props.length];
        for (i, ch) in value.chars().enumerate() {
            if i < props.length {
                values[i] = ch.to_string();
            }
        }
        values
    });
    let mut focused_index = use_signal(|| 0usize);

    // 同步外部 value 到内部状态
    use_effect(move || {
        if let Some(value) = &props.value {
            let mut values = vec![String::new(); props.length];
            for (i, ch) in value.chars().enumerate() {
                if i < props.length {
                    values[i] = ch.to_string();
                }
            }
            internal_values.set(values);
        }
    });

    // 生成样式
    let style_generator = InputStyleGenerator::new()
        .with_size(props.size.clone().into())
        .with_status(props.status.clone().into())
        .with_disabled(props.disabled)
        .with_bordered(props.bordered)
        .with_focused(true)
        .with_token(theme.alias_token.clone());

    let otp_style = style_generator.generate_otp_css();

    // 计算样式
    let mut wrapper_class = vec!["ant-input-otp"];
    
    match props.size {
        InputSize::Large => wrapper_class.push("ant-input-otp-lg"),
        InputSize::Small => wrapper_class.push("ant-input-otp-sm"),
        _ => {}
    }
    
    match props.status {
        InputStatus::Error => wrapper_class.push("ant-input-otp-status-error"),
        InputStatus::Warning => wrapper_class.push("ant-input-otp-status-warning"),
        _ => {}
    }
    
    if props.disabled {
        wrapper_class.push("ant-input-otp-disabled");
    }
    
    if !props.bordered {
        wrapper_class.push("ant-input-otp-borderless");
    }
    
    if let Some(class) = &props.class {
        wrapper_class.push(class);
    }

    rsx! {
        style { {otp_style} }
        
        div {
            class: wrapper_class.join(" "),
            style: props.style.clone().unwrap_or_default(),
            
            for (index, value) in internal_values().iter().enumerate() {
                input {
                    key: "{index}",
                    class: "ant-input ant-input-otp-input",
                    r#type: if props.mask { "password" } else { "text" },
                    value: value.clone(),
                    disabled: props.disabled,
                    maxlength: "1",
                    oninput: move |evt: FormEvent| {
                        let new_value = evt.value();
                        let mut values = internal_values();
                        
                        if new_value.len() <= 1 {
                            values[index] = new_value.clone();
                            internal_values.set(values.clone());
                            
                            // 触发 onChange 回调
                            if let Some(on_change) = &props.on_change {
                                let full_value = values.join("");
                                on_change.call(full_value.clone());
                                
                                // 如果输入完成，触发 onFinish 回调
                                if full_value.len() == props.length {
                                    if let Some(on_finish) = &props.on_finish {
                                        on_finish.call(full_value);
                                    }
                                }
                            }
                            
                            // 自动跳转到下一个输入框
                            if !new_value.is_empty() && index < props.length - 1 {
                                focused_index.set(index + 1);
                            }
                        }
                    },
                    onkeydown: move |evt: KeyboardEvent| {
                        match evt.key() {
                            Key::Backspace => {
                                let mut values = internal_values();
                                if values[index].is_empty() && index > 0 {
                                    // 如果当前输入框为空，跳转到前一个输入框
                                    focused_index.set(index - 1);
                                } else {
                                    // 清空当前输入框
                                    values[index] = String::new();
                                    internal_values.set(values.clone());
                                    
                                    if let Some(on_change) = &props.on_change {
                                        on_change.call(values.join(""));
                                    }
                                }
                            },
                            Key::ArrowLeft => {
                                if index > 0 {
                                    focused_index.set(index - 1);
                                }
                            },
                            Key::ArrowRight => {
                                if index < props.length - 1 {
                                    focused_index.set(index + 1);
                                }
                            },
                            _ => {}
                        }
                    },
                    onfocus: move |_evt: FocusEvent| {
                        focused_index.set(index);
                    }
                }
            }
        }
    }
}

/// 获取输入框的 CSS 类名
fn get_input_css_class(props: &InputProps, is_focused: bool) -> String {
    let mut classes = vec!["ant-input"];

    // 添加尺寸相关的类名
    match props.size {
        InputSize::Large => classes.push("ant-input-lg"),
        InputSize::Small => classes.push("ant-input-sm"),
        _ => {}
    }

    // 添加状态相关的类名
    match props.status {
        InputStatus::Error => classes.push("ant-input-status-error"),
        InputStatus::Warning => classes.push("ant-input-status-warning"),
        _ => {}
    }

    // 添加禁用状态类名
    if props.disabled {
        classes.push("ant-input-disabled");
    }

    // 添加无边框类名
    if !props.bordered {
        classes.push("ant-input-borderless");
    }

    // 添加焦点类名
    if is_focused {
        classes.push("ant-input-focused");
    }

    // 添加自定义类名
    if let Some(class) = &props.class {
        classes.push(class);
    }

    classes.join(" ")
}

/// 获取输入框包装器的 CSS 类名
fn get_wrapper_css_class(props: &InputProps, is_focused: bool) -> String {
    // 如果没有前缀、后缀或清除按钮，则不需要包装器
    if props.prefix.is_none() && props.suffix.is_none() && !props.allow_clear && !props.show_count {
        return String::new();
    }

    let mut classes = vec!["ant-input-affix-wrapper"];

    // 添加尺寸相关的类名
    match props.size {
        InputSize::Large => classes.push("ant-input-lg"),
        InputSize::Small => classes.push("ant-input-sm"),
        _ => {}
    }

    // 添加状态相关的类名
    match props.status {
        InputStatus::Error => classes.push("ant-input-status-error"),
        InputStatus::Warning => classes.push("ant-input-status-warning"),
        _ => {}
    }

    // 添加禁用状态类名
    if props.disabled {
        classes.push("ant-input-affix-wrapper-disabled");
    }

    // 添加无边框类名
    if !props.bordered {
        classes.push("ant-input-affix-wrapper-borderless");
    }

    // 添加焦点类名
    if is_focused {
        classes.push("ant-input-affix-wrapper-focused");
    }

    classes.join(" ")
}

/// 获取输入框组的 CSS 类名
fn get_group_css_class(props: &InputProps) -> String {
    // 如果没有前置或后置标签，则不需要组包装器
    if props.addon_before.is_none() && props.addon_after.is_none() {
        return String::new();
    }

    let mut classes = vec!["ant-input-group-wrapper"];

    // 添加尺寸相关的类名
    match props.size {
        InputSize::Large => classes.push("ant-input-group-wrapper-lg"),
        InputSize::Small => classes.push("ant-input-group-wrapper-sm"),
        _ => {}
    }

    // 添加自定义类名
    if let Some(class) = &props.class {
        classes.push(class);
    }

    classes.join(" ")
}
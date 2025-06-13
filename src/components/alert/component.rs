//!
//! Alert 组件实现
//!
//! 提供警告提示组件，用于页面中展示重要的提示信息。

use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;
use std::time::Duration;

use crate::components::icon::Icon;
use crate::locale::{use_locale, LocaleContext};
use crate::theme::{use_compact_mode, use_dark_mode, use_theme};
use crate::utils::class_names::merge_class_names;

use super::styles::AlertAnimationStyles;
use super::styles::AlertStyleGenerator;
use super::types::*;

/// Alert 组件
///
/// 警告提示，展示需要关注的信息。
#[component]
pub fn Alert(props: AlertProps) -> Element {
    // 状态管理
    let mut alert_state = use_signal(|| AlertState {
        visible: props.visible,
        animation_state: if props.visible {
            AlertAnimationState::Entered
        } else {
            AlertAnimationState::Exited
        },
        closing: false,
    });

    // 监听visible属性变化
    use_effect(move || {
        if props.visible && alert_state.read().animation_state == AlertAnimationState::Exited {
            // 显示时添加进入动画
            alert_state.with_mut(|state| {
                state.visible = true;
                state.animation_state = AlertAnimationState::Entering;
            });

            // 延迟设置为已进入状态
            // 注意：motion 和 animation_duration 需要从 AlertProps 中添加
            let motion = true; // 默认启用动画
            let animation_duration = Duration::from_millis(300); // 默认动画时长
            if motion {
                spawn(async move {
                    gloo_timers::future::TimeoutFuture::new(animation_duration.as_millis() as u32)
                        .await;
                    alert_state.with_mut(|state| {
                        state.animation_state = AlertAnimationState::Entered;
                    });
                });
            } else {
                alert_state.with_mut(|state| {
                    state.animation_state = AlertAnimationState::Entered;
                });
            }
        } else if !props.visible
            && alert_state.read().animation_state != AlertAnimationState::Exited
        {
            // 隐藏时添加退出动画
            alert_state.with_mut(|state| {
                state.animation_state = AlertAnimationState::Exiting;
            });

            // 延迟设置为已退出状态
            let motion = true; // 默认启用动画
            let animation_duration = Duration::from_millis(300); // 默认动画时长
            if motion {
                spawn(async move {
                    gloo_timers::future::TimeoutFuture::new(animation_duration.as_millis() as u32)
                        .await;
                    alert_state.with_mut(|state| {
                        state.visible = false;
                        state.animation_state = AlertAnimationState::Exited;
                    });
                });
            } else {
                alert_state.with_mut(|state| {
                    state.visible = false;
                    state.animation_state = AlertAnimationState::Exited;
                });
            }
        }

        // 返回空闭包以满足use_effect要求
    });

    // 获取主题上下文
    let theme_context = use_theme();
    let is_dark_mode = use_dark_mode();
    let is_compact_mode = use_compact_mode();

    // 获取国际化上下文
    let locale_context = use_locale();
    let translate = crate::locale::use_translate();

    // 样式生成器
    let style_generator = AlertStyleGenerator::new()
        .with_prefix_cls(
            props
                .prefix_cls
                .clone()
                .unwrap_or_else(|| "ant-alert".to_string()),
        )
        .with_type(props.r#type.clone())
        .with_size(props.size)
        .with_variant(props.variant)
        .with_show_icon(props.show_icon.unwrap_or(false) || props.icon.is_some())
        .with_description(props.description.is_some())
        .with_closable(props.closable.unwrap_or(false))
        .with_banner(props.banner.unwrap_or(false))
        .with_theme_enabled(true)
        .with_bordered(props.bordered)
        .with_disabled(props.disabled.unwrap_or(false))
        .with_visible(alert_state.read().visible);

    // 生成类名和样式
    let class_names = style_generator.generate_class_names();
    let inline_styles = style_generator.generate_inline_styles();

    // 动画类名（暂时为空，后续可以根据动画状态添加）
    let animation_class = "";

    // 合并自定义类名和动画类名
    let final_class = if let Some(custom_class) = &props.class {
        if animation_class.is_empty() {
            format!("{} {}", class_names.join(" "), custom_class)
        } else {
            format!(
                "{} {} {}",
                class_names.join(" "),
                animation_class,
                custom_class
            )
        }
    } else {
        if animation_class.is_empty() {
            class_names.join(" ")
        } else {
            format!("{} {}", class_names.join(" "), animation_class)
        }
    };

    // 合并自定义样式
    let final_style = if let Some(custom_style) = &props.style {
        if inline_styles.is_empty() {
            custom_style.clone()
        } else {
            format!("{};{}", inline_styles.clone(), custom_style)
        }
    } else {
        inline_styles.clone()
    };

    // 关闭处理函数
    let handle_close = move |event: Event<MouseData>| {
        if props.disabled.unwrap_or(false) {
            return;
        }

        // 设置关闭状态
        alert_state.with_mut(|state| {
            state.closing = true;
            state.animation_state = AlertAnimationState::Exiting;
        });

        // 调用关闭回调
        if let Some(on_close_cb) = &props.on_close {
            on_close_cb.call(event);
        }

        // 延迟隐藏（等待动画完成）
        let motion = true; // 默认启用动画
        let animation_duration = Duration::from_millis(300); // 默认动画时长
        if motion {
            let duration_ms = animation_duration.as_millis() as u32;
            let after_close = props.after_close.clone();
            spawn(async move {
                gloo_timers::future::TimeoutFuture::new(duration_ms).await;
                alert_state.with_mut(|state| {
                    state.visible = false;
                    state.animation_state = AlertAnimationState::Exited;
                    state.closing = false;
                });

                // 调用关闭后回调
                if let Some(after_close_cb) = &after_close {
                    after_close_cb.call(());
                }
            });
        } else {
            // 无动画时立即隐藏
            alert_state.with_mut(|state| {
                state.visible = false;
                state.animation_state = AlertAnimationState::Exited;
                state.closing = false;
            });

            // 调用关闭后回调
            if let Some(after_close_cb) = &props.after_close {
                after_close_cb.call(());
            }
        }
    };

    // 获取默认图标
    let default_icon = match props.r#type {
        AlertType::Success => rsx! {
            Icon {
                icon_type: "check-circle",
                class: "ant-alert-icon"
            }
        },
        AlertType::Info => rsx! {
            Icon {
                icon_type: "info-circle",
                class: "ant-alert-icon"
            }
        },
        AlertType::Warning => rsx! {
            Icon {
                icon_type: "exclamation-circle",
                class: "ant-alert-icon"
            }
        },
        AlertType::Error => rsx! {
            Icon {
                icon_type: "close-circle",
                class: "ant-alert-icon"
            }
        },
    };

    // 如果完全退出，返回空
    if alert_state.read().animation_state == AlertAnimationState::Exited {
        return rsx! { div { style: "display: none;" } };
    }

    // 根据动画状态生成额外的类名
    let motion = true; // 默认启用动画
    let animation_class = if motion {
        match alert_state.read().animation_state {
            AlertAnimationState::Entering => "ant-alert-entering",
            AlertAnimationState::Entered => "",
            AlertAnimationState::Exiting => "ant-alert-exiting",
            AlertAnimationState::Exited => "", // 不应该到达这里
        }
    } else {
        ""
    };

    // 根据动画状态应用特定样式
    let animation_style = if motion {
        match alert_state.read().animation_state {
            AlertAnimationState::Entering => AlertAnimationStyles::default().entering,
            AlertAnimationState::Exiting => AlertAnimationStyles::default().exiting,
            _ => String::new(),
        }
    } else {
        String::new()
    };

    // 合并所有样式
    let combined_style = if animation_style.is_empty() {
        inline_styles
    } else {
        format!("{};{}", inline_styles, animation_style)
    };

    rsx! {
        div {
            class: "{final_class}",
            style: "{combined_style}",
            role: "alert",
            "aria-live": "polite",
            "data-testid": "alert",

            // 图标区域
            if props.show_icon.unwrap_or(false) || props.icon.is_some() {
                div {
                    class: "ant-alert-icon",
                    if let Some(custom_icon) = &props.icon {
                        {custom_icon.clone()}
                    } else {
                        {default_icon}
                    }
                }
            }

            // 内容区域
            div {
                class: "ant-alert-content",

                // 消息内容
                if let Some(msg) = &props.message {
                    div {
                        class: "ant-alert-message",
                        "{msg}"
                    }
                }

                // 描述内容
                if let Some(desc) = &props.description {
                    div {
                        class: "ant-alert-description",
                        "{desc}"
                    }
                }

                // 子元素
                {props.children}
            }

            // 操作区域
            if let Some(action_content) = &props.action {
                div {
                    class: "ant-alert-action",
                    {action_content.clone()}
                }
            }

            // 关闭按钮
            if props.closable.unwrap_or(false) {
                button {
                    class: "ant-alert-close-icon",
                    r#type: "button",
                    "aria-label": translate("alert.close"),
                    onclick: handle_close,
                    disabled: props.disabled.unwrap_or(false),

                    if let Some(custom_close_icon) = &props.close_icon {
                        {custom_close_icon.clone()}
                    } else if let Some(close_txt) = &props.close_text {
                        span {
                            class: "ant-alert-close-text",
                            "{close_txt}"
                        }
                    } else {
                        Icon {
                            icon_type: "close",
                            class: "ant-alert-close-icon-svg"
                        }
                    }
                }
            }
        }
    }
}

/// ErrorBoundary Alert 组件
///
/// 用于捕获和显示错误信息的特殊 Alert 组件。
#[component]
pub fn ErrorBoundaryAlert(
    error_message: String,
    error_details: Option<String>,

    #[props(default = true)] show_details: bool,

    #[props(default = true)] show_reload_button: bool,

    on_reload: Option<Callback<()>>,

    #[props(default = "ant-alert".to_string())] prefix_cls: String,

    class: Option<String>,
    style: Option<String>,
) -> Element {
    let mut show_details_state = use_signal(|| false);

    let handle_reload = move |_| {
        if let Some(reload_cb) = &on_reload {
            reload_cb.call(());
        } else {
            // 默认重新加载页面
            web_sys::window().and_then(|w| w.location().reload().ok());
        }
    };

    let toggle_details = move |_| {
        show_details_state.with_mut(|state| *state = !*state);
    };

    rsx! {
        Alert {
            r#type: AlertType::Error,
            message: error_message,
            description: if show_details && error_details.is_some() {
                error_details.clone()
            } else {
                None
            },
            show_icon: true,
            closable: false,
            prefix_cls: prefix_cls,
            class: class,
            style: style,

            action: rsx! {
                div {
                    class: "ant-alert-error-actions",
                    style: "display: flex; gap: 8px;",

                    if show_details && error_details.is_some() {
                        button {
                            class: "ant-btn ant-btn-sm ant-btn-text",
                            onclick: toggle_details,
                            if show_details_state() {
                                "隐藏详情"
                            } else {
                                "显示详情"
                            }
                        }
                    }

                    if show_reload_button {
                        button {
                            class: "ant-btn ant-btn-sm ant-btn-primary",
                            onclick: handle_reload,
                            "重新加载"
                        }
                    }
                }
            }
        }
    }
}

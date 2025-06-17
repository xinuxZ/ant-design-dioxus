//! Alert 组件实现

use dioxus::prelude::*;
use gloo_timers::callback::Timeout;

use crate::components::alert::{
    styles::AlertStyleGenerator,
    types::*,
    utils::{AlertEventHandler, AlertUtils},
};
use crate::config_provider::hooks::use_component_config;

/// Alert 组件
#[component]
pub fn Alert(props: AlertProps) -> Element {
    // 获取全局配置
    let component_config = use_component_config();
    let read_config = component_config.read();
    let alert_config = read_config.as_ref().and_then(|config| config.get_alert_config());

    // 应用全局配置
    let props = AlertProps {
        alert_type: match alert_config {
            Some(config) if props.alert_type == AlertType::Info => match &config.default_type {
                Some(config_type) => match config_type {
                    crate::config_provider::component_config::AlertType::Success => {
                        AlertType::Success
                    }
                    crate::config_provider::component_config::AlertType::Info => AlertType::Info,
                    crate::config_provider::component_config::AlertType::Warning => {
                        AlertType::Warning
                    }
                    crate::config_provider::component_config::AlertType::Error => AlertType::Error,
                },
                None => AlertType::Info,
            },
            _ => props.alert_type,
        },
        size: match alert_config {
            Some(config) if props.size == AlertSize::Default => match &config.default_size {
                Some(config_size) => match config_size {
                    crate::config_provider::component_config::AlertSize::Small => AlertSize::Small,
                    crate::config_provider::component_config::AlertSize::Default => {
                        AlertSize::Default
                    }
                    crate::config_provider::component_config::AlertSize::Large => AlertSize::Large,
                },
                None => AlertSize::Default,
            },
            _ => props.size,
        },
        closable: if let Some(config) = alert_config {
            config.closable.unwrap_or(props.closable)
        } else {
            props.closable
        },
        show_icon: if let Some(config) = alert_config {
            config.show_icon.unwrap_or(props.show_icon)
        } else {
            props.show_icon
        },
        banner: if let Some(config) = alert_config {
            config.banner.unwrap_or(props.banner)
        } else {
            props.banner
        },
        enable_animation: if let Some(config) = alert_config {
            config.enable_animation.unwrap_or(props.enable_animation)
        } else {
            props.enable_animation
        },
        animation_duration: if let Some(config) = alert_config {
            config
                .animation_duration
                .unwrap_or(props.animation_duration)
        } else {
            props.animation_duration
        },
        class_name: props
            .class_name
            .or_else(|| alert_config.and_then(|c| c.class_name.clone())),
        style: props.style.or_else(|| {
            alert_config.and_then(|c| {
                c.style.clone().map(|s| {
                    s.iter()
                        .map(|(k, v)| format!("{}: {}", k, v))
                        .collect::<Vec<_>>()
                        .join("; ")
                })
            })
        }),
        ..props
    };

    // 状态管理
    let mut alert_state = use_signal(|| AlertState {
        visible: props.visible,
        closing: false,
        mounted: false,
        animation_state: AnimationState::Idle,
    });

    // 内部可见性状态
    let mut internal_visible = use_signal(|| props.visible);

    // 动画定时器
    let mut animation_timer = use_signal(|| None::<Timeout>);

    // 组件挂载效果 - 修复 hooks 规则违规
    use_effect(move || {
        let current_state = alert_state.read();
        if !current_state.mounted {
            // 在下一个 tick 中更新状态，避免在同一个 effect 中读写
            spawn(async move {
                alert_state.write().mounted = true;
                
                if props.visible && props.enable_animation {
                    alert_state.write().animation_state = AnimationState::Entering;
                    
                    // 设置进入动画完成定时器
                    let timer = Timeout::new(props.animation_duration, move || {
                        alert_state.write().animation_state = AnimationState::Entered;
                    });
                    animation_timer.set(Some(timer));
                } else if props.visible {
                    alert_state.write().animation_state = AnimationState::Entered;
                }
            });
        }
    });

    // 克隆需要在 effect 中使用的 props 字段
    let visible = props.visible;
    let enable_animation = props.enable_animation;
    let animation_duration = props.animation_duration;
    let after_close = props.after_close.clone();

    // 监听visible属性变化 - 分离读取和写入操作
    let current_visible = *internal_visible.read();
    
    // 只在 visible 状态真正改变时更新
    if visible != current_visible {
        // 在独立的作用域中更新 internal_visible
        use_effect(move || {
            internal_visible.set(visible);
        });
        
        // 在另一个独立的作用域中处理状态变化
        use_effect(move || {
            if visible {
                // 显示Alert
                alert_state.write().visible = true;
                alert_state.write().closing = false;

                if enable_animation {
                    alert_state.write().animation_state = AnimationState::Entering;

                    let timer = Timeout::new(animation_duration, move || {
                        alert_state.write().animation_state = AnimationState::Entered;
                    });
                    animation_timer.set(Some(timer));
                }
            } else {
                // 隐藏Alert
                handle_close_animation_internal(
                    enable_animation,
                    animation_duration,
                    after_close,
                    &mut alert_state,
                    &mut animation_timer,
                );
            }
        });
    }

    // 自动聚焦
    let mut alert_ref = use_signal(|| None::<web_sys::Element>);
    let auto_focus = props.auto_focus;

    use_effect(move || {
        if auto_focus && alert_state.read().visible {
            if let Some(element) = alert_ref.read().as_ref() {
                AlertEventHandler::handle_focus(element);
            }
        }
    });

    // 克隆需要在闭包中使用的 props 字段
    let on_close_clone = props.on_close.clone();
    let on_key_down_clone = props.on_key_down.clone();
    let closable_clone = props.closable;
    let enable_animation_clone = props.enable_animation;
    let animation_duration_clone = props.animation_duration;
    let after_close_clone = props.after_close.clone();

    // 关闭处理函数
    let handle_close = move |event: MouseEvent| {
        if let Some(ref on_close) = on_close_clone {
            on_close.call(event);
        }

        handle_close_animation_internal(
            enable_animation_clone,
            animation_duration_clone,
            after_close_clone.clone(),
            &mut alert_state,
            &mut animation_timer,
        );
    };

    // 键盘事件处理
    let handle_keydown = move |event: KeyboardEvent| {
        if let Some(ref on_key_down) = on_key_down_clone {
            on_key_down.call(event.clone());
        }

        // 处理Escape键关闭
        if event.key() == Key::Escape && closable_clone {
            handle_close_animation_internal(
                enable_animation_clone,
                animation_duration_clone,
                after_close_clone.clone(),
                &mut alert_state,
                &mut animation_timer,
            );
        }

        AlertEventHandler::handle_keyboard(&event, on_close_clone.clone());
    };

    // 克隆鼠标事件处理器
    let on_mouse_enter_clone = props.on_mouse_enter.clone();
    let on_mouse_leave_clone = props.on_mouse_leave.clone();
    let on_click_clone = props.on_click.clone();

    // 鼠标事件处理
    let handle_mouse_enter = move |event: MouseEvent| {
        if let Some(ref on_mouse_enter) = on_mouse_enter_clone {
            on_mouse_enter.call(event);
        }
    };

    let handle_mouse_leave = move |event: MouseEvent| {
        if let Some(ref on_mouse_leave) = on_mouse_leave_clone {
            on_mouse_leave.call(event);
        }
    };

    let handle_click = move |event: MouseEvent| {
        if let Some(ref on_click) = on_click_clone {
            on_click.call(event);
        }
    };

    // 如果不可见且没有动画，直接返回空
    if !alert_state.read().visible && !alert_state.read().closing {
        return rsx! {};
    }

    // 使用样式生成器生成样式
    let props_clone = props.clone();
    let has_description = props.description.is_some();
    let class_name = use_memo(move || {
        // 使用样式生成器
        let alert_class = AlertStyleGenerator::new()
            .with_type(props_clone.alert_type.clone())
            .with_size(props_clone.size.clone())
            .with_closable(props_clone.closable)
            .with_show_icon(props_clone.show_icon)
            .with_banner(props_clone.banner)
            .with_action(props_clone.action.is_some())
            .with_description(has_description)
            .generate();

        // 添加自定义类名
        let mut classes = vec![alert_class];
        if let Some(custom_class) = &props_clone.class_name {
            classes.push(custom_class.clone());
        }

        // 添加动画状态类名
        match alert_state.read().animation_state {
            AnimationState::Entering => classes.push("ant-alert-entering".to_string()),
            AnimationState::Entered => classes.push("ant-alert-entered".to_string()),
            AnimationState::Exiting => classes.push("ant-alert-exiting".to_string()),
            AnimationState::Exited => classes.push("ant-alert-exited".to_string()),
            _ => {}
        }

        classes.join(" ")
    });

    // 生成内联样式
    let inline_style = props.style.clone().unwrap_or_default();
    let aria_attributes = AlertUtils::generate_aria_attributes(&props);

    // 克隆渲染相关的 props
    let show_icon_clone = props.show_icon;
    let icon_clone = props.icon.clone();
    let alert_type_clone = props.alert_type.clone();
    let close_icon_clone = props.close_icon.clone();
    let message_clone = props.message.clone();
    let description_clone = props.description.clone();
    let action_clone = props.action.clone();

    // 渲染图标
    let render_icon = move || {
        if show_icon_clone {
            if let Some(ref custom_icon) = icon_clone {
                rsx! {
                    span { class: "ant-alert-icon", {custom_icon} }
                }
            } else {
                let icon_name = AlertUtils::get_default_icon(&alert_type_clone);
                rsx! {
                    span {
                        class: "ant-alert-icon",
                        i { class: format!("anticon anticon-{}", icon_name) }
                    }
                }
            }
        } else {
            rsx! {}
        }
    };

    // 渲染关闭按钮
    let render_close_button = move || {
        if props.closable {
            if let Some(ref custom_close_icon) = close_icon_clone {
                rsx! {
                    button {
                        class: "ant-alert-close-icon",
                        "aria-label": "Close",
                        onclick: handle_close,
                        {custom_close_icon}
                    }
                }
            } else {
                rsx! {
                    button {
                        class: "ant-alert-close-icon",
                        "aria-label": "Close",
                        onclick: handle_close,
                        span { class: "anticon anticon-close" }
                    }
                }
            }
        } else {
            rsx! {}
        }
    };

    // 渲染操作项
    let render_action = move || {
        if let Some(action) = &action_clone {
            rsx! {
                span { class: "ant-alert-action", {action} }
            }
        } else {
            rsx! {}
        }
    };

    // 渲染Alert内容
    rsx! {
        div {
            class: "{class_name}",
            style: "{inline_style}",
            // id: {props.id},
            "aria-live": {aria_attributes.get("aria-live").cloned().unwrap_or_default()},
            "aria-atomic": {aria_attributes.get("aria-atomic").cloned().unwrap_or_default()},
            // r#ref: move |el| {
            //     // 保存元素引用
            //     alert_ref.set(Some(el));

            //     // 处理动画
            //     if let Some(element) = alert_ref.read().as_ref() {
            //         if props.enable_animation {
            //             AlertAnimationManager::handle_transition(
            //                 element,
            //                 alert_state.read().animation_state,
            //                 props.animation_duration,
            //                 props.alert_type,
            //             );
            //         }
            //     }
            // },
            onmouseenter: handle_mouse_enter,
            onmouseleave: handle_mouse_leave,
            onclick: handle_click,
            onkeydown: handle_keydown,

            // 渲染图标
            {render_icon()}

            // 渲染内容
            div {
                class: "ant-alert-content",

                // 渲染消息
                div {
                    class: "ant-alert-message",
                    {message_clone}
                }

                // 渲染描述
                if let Some(description) = description_clone {
                    div {
                        class: "ant-alert-description",
                        {description}
                    }
                }
            }

            // 渲染操作项
            {render_action()}

            // 渲染关闭按钮
            {render_close_button()}
        }
    }
}

/// 处理关闭动画
// fn handle_close_animation(
//     props: &AlertProps,
//     alert_state: &mut Signal<AlertState>,
//     animation_timer: &mut Signal<Option<Timeout>>,
// ) {
//     handle_close_animation_internal(
//         props.enable_animation,
//         props.animation_duration,
//         props.after_close.clone(),
//         alert_state,
//         animation_timer,
//     );
// }

/// 内部关闭动画处理函数
fn handle_close_animation_internal(
    enable_animation: bool,
    animation_duration: u32,
    after_close: Option<EventHandler<()>>,
    alert_state: &mut Signal<AlertState>,
    animation_timer: &mut Signal<Option<Timeout>>,
) {
    if enable_animation {
        alert_state.write().closing = true;
        alert_state.write().animation_state = AnimationState::Exiting;

        // 设置退出动画完成定时器
        let mut alert_state = alert_state.clone();
        let timer = Timeout::new(animation_duration, move || {
            alert_state.write().visible = false;
            alert_state.write().closing = false;
            alert_state.write().animation_state = AnimationState::Exited;

            // 调用after_close回调
            if let Some(ref after_close) = after_close {
                after_close.call(());
            }
        });
        animation_timer.set(Some(timer));
    } else {
        alert_state.write().visible = false;
        alert_state.write().closing = false;
        alert_state.write().animation_state = AnimationState::Exited;

        // 调用after_close回调
        if let Some(ref after_close) = after_close {
            after_close.call(());
        }
    }
}

/// Alert 便捷组件 - 成功提示
#[component]
pub fn AlertSuccess(message: String, #[props(optional)] description: Option<String>) -> Element {
    rsx! {
        Alert {
            message,
            description,
            alert_type: AlertType::Success,
            show_icon: true,
        }
    }
}

/// Alert 便捷组件 - 信息提示
#[component]
pub fn AlertInfo(message: String, #[props(optional)] description: Option<String>) -> Element {
    rsx! {
        Alert {
            message,
            description,
            alert_type: AlertType::Info,
            show_icon: true,
        }
    }
}

/// Alert 便捷组件 - 警告提示
#[component]
pub fn AlertWarning(message: String, #[props(optional)] description: Option<String>) -> Element {
    rsx! {
        Alert {
            message,
            description,
            alert_type: AlertType::Warning,
            show_icon: true,
        }
    }
}

/// Alert 便捷组件 - 错误提示
#[component]
pub fn AlertError(message: String, #[props(optional)] description: Option<String>) -> Element {
    rsx! {
        Alert {
            message,
            description,
            alert_type: AlertType::Error,
            show_icon: true,
        }
    }
}

/// Alert 便捷组件 - 可关闭提示
#[component]
pub fn AlertClosable(
    message: String,
    alert_type: AlertType,
    #[props(optional)] description: Option<String>,
    #[props(optional)] on_close: Option<EventHandler<MouseEvent>>,
) -> Element {
    rsx! {
        Alert {
            message,
            description,
            alert_type,
            closable: true,
            show_icon: true,
            on_close,
        }
    }
}

/// Alert 便捷组件 - 横幅提示
#[component]
pub fn AlertBanner(
    message: String,
    alert_type: AlertType,
    #[props(optional)] description: Option<String>,
    #[props(optional)] closable: Option<bool>,
) -> Element {
    rsx! {
        Alert {
            message,
            description,
            alert_type,
            banner: true,
            closable: closable.unwrap_or(false),
            show_icon: true,
        }
    }
}

/// Alert 组合组件 - 多个Alert的容器
#[component]
pub fn AlertGroup(
    children: Element,
    #[props(optional)] class_name: Option<String>,
    #[props(optional)] style: Option<String>,
) -> Element {
    let class_names = if let Some(custom_class) = class_name {
        format!("ant-alert-group {}", custom_class)
    } else {
        "ant-alert-group".to_string()
    };

    rsx! {
        div {
            class: "{class_names}",
            style: style.unwrap_or_default(),
            {children}
        }
    }
}

/// Alert 提供者组件 - 提供全局配置
#[component]
pub fn AlertProvider(children: Element, #[props(optional)] config: Option<AlertConfig>) -> Element {
    // 设置全局配置
    if let Some(cfg) = config {
        // 设置全局配置
        crate::components::alert::types::set_global_alert_config(cfg);
    }

    // 渲染子元素
    rsx! {
        div {
            class: "ant-alert-provider",
            {children}
        }
    }
}

/// Alert 钩子 - 用于程序化显示Alert
pub fn use_alert() -> AlertController {
    let alerts = use_signal(|| Vec::<AlertItem>::new());

    AlertController { alerts }
}

/// Alert 控制器
pub struct AlertController {
    alerts: Signal<Vec<AlertItem>>,
}

impl AlertController {
    /// 显示成功提示
    pub fn success(&mut self, message: &str, duration: Option<u32>) {
        self.show(AlertItem {
            id: self.generate_id(),
            message: message.to_string(),
            alert_type: AlertType::Success,
            duration,
            ..Default::default()
        });
    }

    /// 显示信息提示
    pub fn info(&mut self, message: &str, duration: Option<u32>) {
        self.show(AlertItem {
            id: self.generate_id(),
            message: message.to_string(),
            alert_type: AlertType::Info,
            duration,
            ..Default::default()
        });
    }

    /// 显示警告提示
    pub fn warning(&mut self, message: &str, duration: Option<u32>) {
        self.show(AlertItem {
            id: self.generate_id(),
            message: message.to_string(),
            alert_type: AlertType::Warning,
            duration,
            ..Default::default()
        });
    }

    /// 显示错误提示
    pub fn error(&mut self, message: &str, duration: Option<u32>) {
        self.show(AlertItem {
            id: self.generate_id(),
            message: message.to_string(),
            alert_type: AlertType::Error,
            duration,
            ..Default::default()
        });
    }

    /// 显示Alert
    pub fn show(&mut self, item: AlertItem) {
        self.alerts.write().push(item);
    }

    /// 关闭Alert
    pub fn close(&mut self, id: &str) {
        self.alerts.write().retain(|item| item.id != id);
    }

    /// 关闭所有Alert
    pub fn close_all(&mut self) {
        self.alerts.write().clear();
    }

    /// 生成唯一ID
    fn generate_id(&self) -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        format!("alert-{}", timestamp)
    }

    /// 渲染Alert列表
    pub fn render(&self) -> Element {
        let alerts = self.alerts.read();

        rsx! {
            div { class: "ant-alert-container",
                for item in alerts.iter() {
                    Alert {
                        key: "{item.id}",
                        message: item.message.clone(),
                        alert_type: item.alert_type.clone(),
                        description: item.description.clone(),
                        closable: true,
                        show_icon: true,
                        on_close: move |_| {
                            // 这里需要实现关闭逻辑
                        },
                    }
                }
            }
        }
    }
}

/// Alert 项目
#[derive(Clone, PartialEq, Debug)]
pub struct AlertItem {
    pub id: String,
    pub message: String,
    pub alert_type: AlertType,
    pub description: Option<String>,
    pub duration: Option<u32>,
    pub closable: bool,
    pub show_icon: bool,
}

impl Default for AlertItem {
    fn default() -> Self {
        Self {
            id: String::new(),
            message: String::new(),
            alert_type: AlertType::Info,
            description: None,
            duration: Some(3000),
            closable: true,
            show_icon: true,
        }
    }
}

/// 全局Alert函数
// pub mod global {
//     use super::*;

//     /// 全局成功提示
//     pub fn success(message: &str) {
//         // 这里需要实现全局Alert显示逻辑
//         // 可以通过事件系统或全局状态管理来实现
//     }

//     /// 全局信息提示
//     pub fn info(message: &str) {
//         // 实现全局信息提示
//     }

//     /// 全局警告提示
//     pub fn warning(message: &str) {
//         // 实现全局警告提示
//     }

//     /// 全局错误提示
//     pub fn error(message: &str) {
//         // 实现全局错误提示
//     }
// }

/// 创建成功类型的Alert
pub fn success_alert(message: String) -> Element {
    rsx! {
        Alert {
            alert_type: AlertType::Success,
            message: message,
        }
    }
}

/// 创建信息类型的Alert
pub fn info_alert(message: String) -> Element {
    rsx! {
        Alert {
            alert_type: AlertType::Info,
            message: message,
        }
    }
}

/// 创建警告类型的Alert
pub fn warning_alert(message: String) -> Element {
    rsx! {
        Alert {
            alert_type: AlertType::Warning,
            message: message,
        }
    }
}

/// 创建错误类型的Alert
pub fn error_alert(message: String) -> Element {
    rsx! {
        Alert {
            alert_type: AlertType::Error,
            message: message,
        }
    }
}

/// 创建可关闭的Alert
pub fn closable_alert(message: String, alert_type: AlertType) -> Element {
    rsx! {
        Alert {
            alert_type: alert_type,
            message: message,
            closable: true,
        }
    }
}

/// 创建带图标的Alert
pub fn icon_alert(message: String, alert_type: AlertType) -> Element {
    rsx! {
        Alert {
            alert_type: alert_type,
            message: message,
            show_icon: true,
        }
    }
}

/// 创建横幅样式的Alert
pub fn banner_alert(message: String, alert_type: AlertType) -> Element {
    rsx! {
        Alert {
            alert_type: alert_type,
            message: message,
            banner: true,
        }
    }
}

/// 创建带操作按钮的Alert
pub fn action_alert(message: String, alert_type: AlertType, action: Element) -> Element {
    rsx! {
        Alert {
            alert_type: alert_type,
            message: message,
            action: action,
        }
    }
}

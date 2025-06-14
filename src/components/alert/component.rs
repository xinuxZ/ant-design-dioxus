//! Alert 组件实现

use dioxus::prelude::*;
use gloo_timers::callback::Timeout;

use crate::components::alert::{
    styles::AlertStyles,
    types::*,
    utils::{AlertAnimationManager, AlertEventHandler, AlertUtils},
};

/// Alert 组件
#[component]
pub fn Alert(props: AlertProps) -> Element {
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

    // 组件挂载效果
    use_effect(move || {
        if !alert_state.read().mounted {
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
        }
    });

    // 监听visible属性变化
    use_effect(move || {
        let current_visible = internal_visible.read();
        if props.visible != *current_visible {
            internal_visible.set(props.visible);

            if props.visible {
                // 显示Alert
                alert_state.write().visible = true;
                alert_state.write().closing = false;

                if props.enable_animation {
                    alert_state.write().animation_state = AnimationState::Entering;

                    let timer = Timeout::new(props.animation_duration, move || {
                        alert_state.write().animation_state = AnimationState::Entered;
                    });
                    animation_timer.set(Some(timer));
                }
            } else {
                // 隐藏Alert
                handle_close_animation(&props, &mut alert_state, &mut animation_timer);
            }
        }
    });

    // 自动聚焦
    let alert_ref = use_signal(|| None::<web_sys::Element>);

    use_effect(move || {
        if props.auto_focus && alert_state.read().visible {
            if let Some(element) = alert_ref.read().as_ref() {
                AlertEventHandler::handle_focus(element);
            }
        }
    });

    // 关闭处理函数
    let handle_close = move |event: MouseEvent| {
        if let Some(ref on_close) = props.on_close {
            on_close.call(event);
        }

        handle_close_animation(&props, &mut alert_state, &mut animation_timer);
    };

    // 键盘事件处理
    let handle_keydown = move |event: KeyboardEvent| {
        if let Some(ref on_key_down) = props.on_key_down {
            on_key_down.call(event.clone());
        }

        // 处理Escape键关闭
        if event.key() == Key::Escape && props.closable {
            handle_close_animation(&props, &mut alert_state, &mut animation_timer);
        }

        AlertEventHandler::handle_keyboard(&event, props.on_close.clone());
    };

    // 鼠标事件处理
    let handle_mouse_enter = move |event: MouseEvent| {
        if let Some(ref on_mouse_enter) = props.on_mouse_enter {
            on_mouse_enter.call(event);
        }
    };

    let handle_mouse_leave = move |event: MouseEvent| {
        if let Some(ref on_mouse_leave) = props.on_mouse_leave {
            on_mouse_leave.call(event);
        }
    };

    let handle_click = move |event: MouseEvent| {
        if let Some(ref on_click) = props.on_click {
            on_click.call(event);
        }
    };

    // 如果不可见且没有动画，直接返回空
    if !alert_state.read().visible && !alert_state.read().closing {
        return rsx! {};
    }

    // 生成样式和类名
    let class_names = AlertUtils::generate_class_names(&props, &alert_state.read());
    let inline_styles = AlertUtils::generate_inline_styles(&props);
    let aria_attributes = AlertUtils::generate_aria_attributes(&props);

    // 渲染图标
    let render_icon = move || {
        if props.show_icon {
            if let Some(ref custom_icon) = props.icon {
                rsx! {
                    span { class: "ant-alert-icon", {custom_icon} }
                }
            } else {
                let icon_name = AlertUtils::get_default_icon(&props.alert_type);
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
            if let Some(ref custom_close_icon) = props.close_icon {
                rsx! {
                    button {
                        class: "ant-alert-close-icon",
                        r#type: "button",
                        onclick: handle_close,
                        "aria-label": "Close",
                        {custom_close_icon}
                    }
                }
            } else {
                rsx! {
                    button {
                        class: "ant-alert-close-icon",
                        r#type: "button",
                        onclick: handle_close,
                        "aria-label": "Close",
                        i { class: "anticon anticon-close" }
                    }
                }
            }
        } else {
            rsx! {}
        }
    };

    // 渲染消息内容
    let render_content = move || {
        rsx! {
            div { class: "ant-alert-content",
                div { class: "ant-alert-message", "{props.message}" }
                if let Some(ref description) = props.description {
                    div { class: "ant-alert-description", "{description}" }
                }
            }
        }
    };

    // 渲染操作区域
    let render_action = move || {
        if let Some(ref action) = props.action {
            rsx! {
                div { class: "ant-alert-action", {action} }
            }
        } else {
            rsx! {}
        }
    };

    let data_testid_clone = props.data_testid.clone();
    let tab_index_clone = props.tab_index.clone();

    // 主要渲染
    rsx! {
        div {
            class: "{class_names}",
            style: "{inline_styles}",
            role: "{props.role}",
            "aria-label": aria_attributes.get("aria-label").cloned().unwrap_or_default(),
            "aria-live": aria_attributes.get("aria-live").cloned().unwrap_or_default(),
            "aria-atomic": aria_attributes.get("aria-atomic").cloned().unwrap_or_default(),
            tabindex: tab_index_clone.unwrap_or(-1),
            "data-testid": data_testid_clone.clone().unwrap_or_default(),
            onmounted: move |event| {
                if let Some(element) = event.data.downcast::<web_sys::Element>() {
                    alert_ref.set(Some(element.clone()));
                }
            },
            onclick: handle_click,
            onmouseenter: handle_mouse_enter,
            onmouseleave: handle_mouse_leave,
            onkeydown: handle_keydown,

            {render_icon()}
            {render_content()}
            {render_action()}
            {render_close_button()}
        }
    }
}

/// 处理关闭动画
fn handle_close_animation(
    props: &AlertProps,
    alert_state: &mut Signal<AlertState>,
    animation_timer: &mut Signal<Option<Timeout>>,
) {
    if props.enable_animation {
        alert_state.write().closing = true;
        alert_state.write().animation_state = AnimationState::Exiting;

        // 设置退出动画完成定时器
        let timer = Timeout::new(props.animation_duration, move || {
            alert_state.write().visible = false;
            alert_state.write().closing = false;
            alert_state.write().animation_state = AnimationState::Exited;

            // 调用after_close回调
            if let Some(ref after_close) = props.after_close {
                after_close.call(());
            }
        });
        animation_timer.set(Some(timer));
    } else {
        alert_state.write().visible = false;
        alert_state.write().closing = false;
        alert_state.write().animation_state = AnimationState::Exited;

        // 调用after_close回调
        if let Some(ref after_close) = props.after_close {
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
        use_effect(move || {
            crate::components::alert::types::set_global_alert_config(cfg.clone());
        });
    }

    rsx! {
        div { class: "ant-alert-provider", {children} }
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
pub mod global {
    use super::*;

    /// 全局成功提示
    pub fn success(message: &str) {
        // 这里需要实现全局Alert显示逻辑
        // 可以通过事件系统或全局状态管理来实现
    }

    /// 全局信息提示
    pub fn info(message: &str) {
        // 实现全局信息提示
    }

    /// 全局警告提示
    pub fn warning(message: &str) {
        // 实现全局警告提示
    }

    /// 全局错误提示
    pub fn error(message: &str) {
        // 实现全局错误提示
    }
}

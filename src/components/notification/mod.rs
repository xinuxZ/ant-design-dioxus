//! Notification 通知提醒框
//!
//! 全局展示通知提醒信息。
//!
//! ## 何时使用
//!
//! 在系统四个角显示通知提醒信息。经常用于以下情况：
//!
//! - 较为复杂的通知内容。
//! - 带有交互的通知，给出用户下一步的行动点。
//! - 系统主动推送。
//!
//! ## 代码演示
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::{notification, NotificationPlacement};
//!
//! #[component]
//! fn App() -> Element {
//!     let handle_click = move |_| {
//!         notification::success("成功", "这是一条成功的通知信息");
//!     };
//!
//!     rsx! {
//!         button {
//!             onclick: handle_click,
//!             "显示通知"
//!         }
//!     }
//! }
//! ```

use dioxus::prelude::*;
use uuid::Uuid;

const NOTIFICATION_STYLES: &str = include_str!("./style.css");

/// 通知类型
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NotificationType {
    /// 成功
    Success,
    /// 信息
    Info,
    /// 警告
    Warning,
    /// 错误
    Error,
}

impl NotificationType {
    /// 获取类型对应的CSS类名
    pub fn to_class(&self) -> &'static str {
        match self {
            NotificationType::Success => "ant-notification-notice-success",
            NotificationType::Info => "ant-notification-notice-info",
            NotificationType::Warning => "ant-notification-notice-warning",
            NotificationType::Error => "ant-notification-notice-error",
        }
    }

    /// 获取默认图标
    pub fn default_icon(&self) -> &'static str {
        match self {
            NotificationType::Success => "✓",
            NotificationType::Info => "ℹ",
            NotificationType::Warning => "⚠",
            NotificationType::Error => "✕",
        }
    }

    /// 获取默认持续时间（秒）
    pub fn default_duration(&self) -> f64 {
        match self {
            NotificationType::Success => 4.5,
            NotificationType::Info => 4.5,
            NotificationType::Warning => 4.5,
            NotificationType::Error => 4.5,
        }
    }
}

/// 通知位置
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NotificationPlacement {
    /// 右上角
    TopRight,
    /// 左上角
    TopLeft,
    /// 右下角
    BottomRight,
    /// 左下角
    BottomLeft,
    /// 顶部居中
    Top,
    /// 底部居中
    Bottom,
}

impl Default for NotificationPlacement {
    fn default() -> Self {
        Self::TopRight
    }
}

impl NotificationPlacement {
    /// 获取位置对应的CSS类名
    pub fn to_class(&self) -> &'static str {
        match self {
            NotificationPlacement::TopRight => "ant-notification-topRight",
            NotificationPlacement::TopLeft => "ant-notification-topLeft",
            NotificationPlacement::BottomRight => "ant-notification-bottomRight",
            NotificationPlacement::BottomLeft => "ant-notification-bottomLeft",
            NotificationPlacement::Top => "ant-notification-top",
            NotificationPlacement::Bottom => "ant-notification-bottom",
        }
    }

    /// 获取容器样式
    pub fn to_style(&self) -> &'static str {
        match self {
            NotificationPlacement::TopRight => "top: 24px; right: 24px;",
            NotificationPlacement::TopLeft => "top: 24px; left: 24px;",
            NotificationPlacement::BottomRight => "bottom: 24px; right: 24px;",
            NotificationPlacement::BottomLeft => "bottom: 24px; left: 24px;",
            NotificationPlacement::Top => "top: 24px; left: 50%; transform: translateX(-50%);",
            NotificationPlacement::Bottom => {
                "bottom: 24px; left: 50%; transform: translateX(-50%);"
            }
        }
    }
}

/// 通知项数据
#[derive(Debug, Clone, PartialEq)]
pub struct NotificationItem {
    /// 唯一标识
    pub key: String,
    /// 通知类型
    pub notification_type: NotificationType,
    /// 标题
    pub title: String,
    /// 描述内容
    pub description: Option<String>,
    /// 自定义图标
    pub icon: Option<String>,
    /// 持续时间（秒），0表示不自动关闭
    pub duration: f64,
    /// 是否显示关闭按钮
    pub closable: bool,
    /// 关闭时的回调
    pub on_close: Option<String>,
    /// 点击通知时的回调
    pub on_click: Option<String>,
    /// 自定义样式
    pub style: Option<String>,
    /// 自定义类名
    pub class_name: Option<String>,
    /// 创建时间
    pub created_at: f64,
}

impl NotificationItem {
    /// 创建新的通知项
    pub fn new(
        notification_type: NotificationType,
        title: impl Into<String>,
        description: Option<impl Into<String>>,
    ) -> Self {
        Self {
            key: Uuid::new_v4().to_string(),
            notification_type,
            title: title.into(),
            description: description.map(|d| d.into()),
            icon: None,
            duration: notification_type.default_duration(),
            closable: true,
            on_close: None,
            on_click: None,
            style: None,
            class_name: None,
            created_at: js_sys::Date::now(),
        }
    }

    /// 设置自定义图标
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// 设置持续时间
    pub fn with_duration(mut self, duration: f64) -> Self {
        self.duration = duration;
        self
    }

    /// 设置是否可关闭
    pub fn with_closable(mut self, closable: bool) -> Self {
        self.closable = closable;
        self
    }

    /// 设置自定义样式
    pub fn with_style(mut self, style: impl Into<String>) -> Self {
        self.style = Some(style.into());
        self
    }

    /// 设置自定义类名
    pub fn with_class_name(mut self, class_name: impl Into<String>) -> Self {
        self.class_name = Some(class_name.into());
        self
    }
}

/// 通知配置
#[derive(Debug, Clone, PartialEq)]
pub struct NotificationConfig {
    /// 通知位置
    pub placement: NotificationPlacement,
    /// 距离顶部或底部的距离
    pub top: Option<String>,
    /// 距离底部的距离
    pub bottom: Option<String>,
    /// 最大显示数量
    pub max_count: Option<usize>,
    /// RTL 模式
    pub rtl: bool,
    /// 堆叠模式阈值
    pub stack: Option<usize>,
}

impl Default for NotificationConfig {
    fn default() -> Self {
        Self {
            placement: NotificationPlacement::TopRight,
            top: None,
            bottom: None,
            max_count: None,
            rtl: false,
            stack: None,
        }
    }
}

/// 通知容器组件属性
#[derive(Props, Clone, PartialEq)]
pub struct NotificationContainerProps {
    /// 通知列表
    #[props(default)]
    pub notifications: Vec<NotificationItem>,

    /// 通知配置
    #[props(default)]
    pub config: NotificationConfig,

    /// 关闭通知的回调
    #[props(default)]
    pub on_close: Option<EventHandler<String>>,
}

/// 单个通知组件属性
#[derive(Props, Clone, PartialEq)]
pub struct NotificationProps {
    /// 通知数据
    pub notification: NotificationItem,

    /// 关闭回调
    #[props(default)]
    pub on_close: Option<EventHandler<String>>,
}

/// 单个通知组件
#[component]
fn NotificationNotice(props: NotificationProps) -> Element {
    let mut is_closing = use_signal(|| false);

    // 提前克隆需要的值
    let notification_key = props.notification.key.clone();
    let notification_key_for_close = notification_key.clone();
    let notification_duration = props.notification.duration;
    let on_close_callback = props.on_close.clone();
    let on_close_for_handle = props.on_close.clone();

    // 自动关闭逻辑
    use_effect(move || {
        if notification_duration > 0.0 {
            let key = notification_key.clone();
            let duration = notification_duration;
            let on_close = on_close_callback.clone();

            spawn(async move {
                gloo_timers::future::TimeoutFuture::new((duration * 1000.0) as u32).await;
                if let Some(callback) = on_close {
                    callback.call(key);
                }
            });
        }
    });
    let handle_close = move |_: MouseEvent| {
        is_closing.set(true);
        if let Some(on_close) = &on_close_for_handle {
            on_close.call(notification_key_for_close.clone());
        }
    };

    let handle_click = move |_: MouseEvent| {
        // 处理点击事件
        if let Some(_on_click) = &props.notification.on_click {
            // 这里可以添加点击回调逻辑
        }
    };

    let notice_class = {
        let mut classes = vec!["ant-notification-notice"];
        classes.push(props.notification.notification_type.to_class());

        if is_closing() {
            classes.push("ant-notification-notice-close");
        }

        if let Some(class_name) = &props.notification.class_name {
            classes.push(class_name);
        }

        classes.join(" ")
    };

    let icon = props
        .notification
        .icon
        .as_deref()
        .unwrap_or_else(|| props.notification.notification_type.default_icon());

    rsx! {
        div {
            class: "{notice_class}",
            style: props.notification.style.as_deref().unwrap_or(""),
            onclick: handle_click,

            div {
                class: "ant-notification-notice-content",

                div {
                    class: "ant-notification-notice-with-icon",

                    // 图标
                    span {
                        class: "ant-notification-notice-icon",
                        "{icon}"
                    }

                    // 内容
                    div {
                        class: "ant-notification-notice-message",

                        // 标题
                        div {
                            class: "ant-notification-notice-title",
                            "{props.notification.title}"
                        }

                        // 描述
                        if let Some(description) = &props.notification.description {
                            div {
                                class: "ant-notification-notice-description",
                                "{description}"
                            }
                        }
                    }
                }
            }

            // 关闭按钮
            if props.notification.closable {
                button {
                    class: "ant-notification-notice-close",
                    r#type: "button",
                    "aria-label": "Close",
                    onclick: handle_close,
                    span {
                        class: "ant-notification-notice-close-x",
                        "×"
                    }
                }
            }
        }
    }
}

/// 通知容器组件
#[component]
pub fn NotificationContainer(props: NotificationContainerProps) -> Element {
    let container_class = {
        let mut classes = vec!["ant-notification"];
        classes.push(props.config.placement.to_class());

        if props.config.rtl {
            classes.push("ant-notification-rtl");
        }

        classes.join(" ")
    };

    let container_style = {
        let mut styles = vec![props.config.placement.to_style().to_string()];

        if let Some(top) = &props.config.top {
            styles.push(format!("top: {}", top));
        }

        if let Some(bottom) = &props.config.bottom {
            styles.push(format!("bottom: {}", bottom));
        }

        styles.join("; ")
    };

    // 处理最大数量限制
    let visible_notifications = if let Some(max_count) = props.config.max_count {
        props
            .notifications
            .iter()
            .take(max_count)
            .cloned()
            .collect::<Vec<_>>()
    } else {
        props.notifications.clone()
    };

    rsx! {
        style { {NOTIFICATION_STYLES} }
        if !visible_notifications.is_empty() {
            div {
                class: "{container_class}",
                style: "{container_style}",

                for notification in visible_notifications {
                    NotificationNotice {
                        key: "{notification.key}",
                        notification: notification,
                        on_close: props.on_close.clone(),
                    }
                }
            }
        }
    }
}

/// 全局通知管理器
pub struct NotificationManager {
    notifications: std::sync::Arc<std::sync::RwLock<Vec<NotificationItem>>>,
    config: std::sync::Arc<std::sync::RwLock<NotificationConfig>>,
}

impl NotificationManager {
    /// 创建新的通知管理器
    pub fn new() -> Self {
        Self {
            notifications: std::sync::Arc::new(std::sync::RwLock::new(Vec::new())),
            config: std::sync::Arc::new(std::sync::RwLock::new(NotificationConfig::default())),
        }
    }

    /// 显示成功通知
    pub fn success(&self, title: impl Into<String>, description: Option<impl Into<String>>) {
        let notification = NotificationItem::new(NotificationType::Success, title, description);
        self.add_notification(notification);
    }

    /// 显示信息通知
    pub fn info(&self, title: impl Into<String>, description: Option<impl Into<String>>) {
        let notification = NotificationItem::new(NotificationType::Info, title, description);
        self.add_notification(notification);
    }

    /// 显示警告通知
    pub fn warning(&self, title: impl Into<String>, description: Option<impl Into<String>>) {
        let notification = NotificationItem::new(NotificationType::Warning, title, description);
        self.add_notification(notification);
    }

    /// 显示错误通知
    pub fn error(&self, title: impl Into<String>, description: Option<impl Into<String>>) {
        let notification = NotificationItem::new(NotificationType::Error, title, description);
        self.add_notification(notification);
    }

    /// 添加自定义通知
    pub fn open(&self, notification: NotificationItem) {
        self.add_notification(notification);
    }

    /// 关闭指定通知
    pub fn close(&self, key: &str) {
        if let Ok(mut notifications) = self.notifications.write() {
            notifications.retain(|n| n.key != key);
        }
    }

    /// 关闭所有通知
    pub fn destroy(&self) {
        if let Ok(mut notifications) = self.notifications.write() {
            notifications.clear();
        }
    }

    /// 更新配置
    pub fn config(&self, config: NotificationConfig) {
        if let Ok(mut cfg) = self.config.write() {
            *cfg = config;
        }
    }

    /// 获取当前通知列表
    pub fn get_notifications(&self) -> Vec<NotificationItem> {
        if let Ok(notifications) = self.notifications.read() {
            notifications.clone()
        } else {
            Vec::new()
        }
    }

    /// 获取当前配置
    pub fn get_config(&self) -> NotificationConfig {
        if let Ok(config) = self.config.read() {
            config.clone()
        } else {
            NotificationConfig::default()
        }
    }

    /// 添加通知到列表
    fn add_notification(&self, notification: NotificationItem) {
        if let Ok(mut notifications) = self.notifications.write() {
            // 检查是否有相同key的通知，如果有则替换
            if let Some(index) = notifications.iter().position(|n| n.key == notification.key) {
                notifications[index] = notification;
            } else {
                notifications.push(notification);
            }

            // 处理最大数量限制
            if let Ok(config) = self.config.read() {
                if let Some(max_count) = config.max_count {
                    let current_len = notifications.len();
                    if current_len > max_count {
                        notifications.drain(0..current_len - max_count);
                    }
                }
            }
        }
    }
}

// 全局通知管理器实例
static GLOBAL_NOTIFICATION: once_cell::sync::Lazy<NotificationManager> =
    once_cell::sync::Lazy::new(|| NotificationManager::new());

/// 全局通知函数
pub mod notification {
    use super::*;

    /// 显示成功通知
    pub fn success(title: impl Into<String>, description: impl Into<String>) {
        GLOBAL_NOTIFICATION.success(title, Some(description));
    }

    /// 显示信息通知
    pub fn info(title: impl Into<String>, description: impl Into<String>) {
        GLOBAL_NOTIFICATION.info(title, Some(description));
    }

    /// 显示警告通知
    pub fn warning(title: impl Into<String>, description: impl Into<String>) {
        GLOBAL_NOTIFICATION.warning(title, Some(description));
    }

    /// 显示错误通知
    pub fn error(title: impl Into<String>, description: impl Into<String>) {
        GLOBAL_NOTIFICATION.error(title, Some(description));
    }

    /// 显示自定义通知
    pub fn open(notification: NotificationItem) {
        GLOBAL_NOTIFICATION.open(notification);
    }

    /// 关闭指定通知
    pub fn close(key: &str) {
        GLOBAL_NOTIFICATION.close(key);
    }

    /// 关闭所有通知
    pub fn destroy() {
        GLOBAL_NOTIFICATION.destroy();
    }

    /// 更新配置
    pub fn config(config: NotificationConfig) {
        GLOBAL_NOTIFICATION.config(config);
    }
}

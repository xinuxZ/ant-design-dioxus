//! App component module
//!
//! The App component is the root component of your application, providing
//! global configuration and context for message, notification, and modal services.

mod styles;

use dioxus::prelude::*;
use std::rc::Rc;

use self::styles::{generate_app_style, AppTheme};

/// Message configuration
#[derive(Clone, Debug, PartialEq)]
pub struct MessageConfig {
    /// Duration in seconds
    pub duration: Option<f64>,
    /// Maximum count of messages
    pub max_count: Option<usize>,
    /// Top offset
    pub top: Option<String>,
    /// Right to left
    pub rtl: Option<bool>,
    /// Prefix class name
    pub prefix_cls: Option<String>,
}

impl Default for MessageConfig {
    fn default() -> Self {
        Self {
            duration: Some(3.0),
            max_count: None,
            top: None,
            rtl: None,
            prefix_cls: None,
        }
    }
}

/// Notification configuration
#[derive(Clone, Debug, PartialEq)]
pub struct NotificationConfig {
    /// Duration in seconds
    pub duration: Option<f64>,
    /// Maximum count of notifications
    pub max_count: Option<usize>,
    /// Placement of notification
    pub placement: Option<String>,
    /// Bottom offset
    pub bottom: Option<String>,
    /// Top offset
    pub top: Option<String>,
    /// Right to left
    pub rtl: Option<bool>,
    /// Prefix class name
    pub prefix_cls: Option<String>,
}

impl Default for NotificationConfig {
    fn default() -> Self {
        Self {
            duration: Some(4.5),
            max_count: None,
            placement: Some("topRight".to_string()),
            bottom: None,
            top: None,
            rtl: None,
            prefix_cls: None,
        }
    }
}

/// App context for static methods
#[derive(Clone)]
pub struct AppContext {
    /// Message instance
    pub message: MessageInstance,
    /// Notification instance
    pub notification: NotificationInstance,
    /// Modal instance
    pub modal: ModalInstance,
}

/// Message instance with static methods
#[derive(Clone)]
pub struct MessageInstance {
    /// Show success message
    pub success: Rc<dyn Fn(String)>,
    /// Show error message
    pub error: Rc<dyn Fn(String)>,
    /// Show info message
    pub info: Rc<dyn Fn(String)>,
    /// Show warning message
    pub warning: Rc<dyn Fn(String)>,
    /// Show loading message
    pub loading: Rc<dyn Fn(String)>,
}

/// Notification instance with static methods
#[derive(Clone)]
pub struct NotificationInstance {
    /// Show success notification
    pub success: Rc<dyn Fn(String, Option<String>)>,
    /// Show error notification
    pub error: Rc<dyn Fn(String, Option<String>)>,
    /// Show info notification
    pub info: Rc<dyn Fn(String, Option<String>)>,
    /// Show warning notification
    pub warning: Rc<dyn Fn(String, Option<String>)>,
}

/// Modal instance with static methods
#[derive(Clone)]
pub struct ModalInstance {
    /// Show info modal
    pub info: Rc<dyn Fn(String, Option<String>)>,
    /// Show success modal
    pub success: Rc<dyn Fn(String, Option<String>)>,
    /// Show error modal
    pub error: Rc<dyn Fn(String, Option<String>)>,
    /// Show warning modal
    pub warning: Rc<dyn Fn(String, Option<String>)>,
    /// Show confirm modal
    pub confirm: Rc<dyn Fn(String, Option<String>)>,
}

/// Component type for App wrapper
#[derive(Clone, Debug, PartialEq)]
pub enum AppComponent {
    Div,
    Section,
    Main,
    Article,
    Custom(String),
    None, // When component is false
}

impl Default for AppComponent {
    fn default() -> Self {
        Self::Div
    }
}

/// App component properties
#[derive(Props, Clone, PartialEq)]
pub struct AppProps {
    /// Config render element, if false will not create DOM node
    #[props(default)]
    pub component: AppComponent,

    /// Global config for Message
    #[props(default)]
    pub message: Option<MessageConfig>,

    /// Global config for Notification
    #[props(default)]
    pub notification: Option<NotificationConfig>,

    /// App theme
    #[props(default = AppTheme::Default)]
    pub theme: AppTheme,

    /// Children components
    pub children: Element,

    /// CSS 类名
    #[props(default)]
    pub class: Option<String>,

    /// 内联样式
    #[props(default)]
    pub style: Option<String>,

    /// 元素 ID
    #[props(default)]
    pub id: Option<String>,
}

/// App component context
static APP_CONTEXT: GlobalSignal<Option<AppContext>> = Signal::global(|| None::<AppContext>);

/// Hook to use App context
pub fn use_app() -> AppContext {
    let context = APP_CONTEXT.read();
    context
        .as_ref()
        .expect("useApp must be used within App component")
        .clone()
}

/// App component
#[component]
pub fn App(props: AppProps) -> Element {
    // Create message handlers
    let message_success = Rc::new(|content: String| {
        // In a real implementation, this would trigger a message display
        web_sys::console::log_1(&format!("Success: {}", content).into());
    });

    let message_error = Rc::new(|content: String| {
        web_sys::console::log_1(&format!("Error: {}", content).into());
    });

    let message_info = Rc::new(|content: String| {
        web_sys::console::log_1(&format!("Info: {}", content).into());
    });

    let message_warning = Rc::new(|content: String| {
        web_sys::console::log_1(&format!("Warning: {}", content).into());
    });

    let message_loading = Rc::new(|content: String| {
        web_sys::console::log_1(&format!("Loading: {}", content).into());
    });

    // Create notification handlers
    let notification_success = Rc::new(|title: String, description: Option<String>| {
        web_sys::console::log_1(
            &format!(
                "Notification Success: {}, {}",
                title,
                description.unwrap_or_default()
            )
            .into(),
        );
    });

    let notification_error = Rc::new(|title: String, description: Option<String>| {
        web_sys::console::log_1(
            &format!(
                "Notification Error: {}, {}",
                title,
                description.unwrap_or_default()
            )
            .into(),
        );
    });

    let notification_info = Rc::new(|title: String, description: Option<String>| {
        web_sys::console::log_1(
            &format!(
                "Notification Info: {}, {}",
                title,
                description.unwrap_or_default()
            )
            .into(),
        );
    });

    let notification_warning = Rc::new(|title: String, description: Option<String>| {
        web_sys::console::log_1(
            &format!(
                "Notification Warning: {}, {}",
                title,
                description.unwrap_or_default()
            )
            .into(),
        );
    });

    // Create modal handlers
    let modal_info = Rc::new(|title: String, content: Option<String>| {
        web_sys::console::log_1(
            &format!("Modal Info: {}, {}", title, content.unwrap_or_default()).into(),
        );
    });

    let modal_success = Rc::new(|title: String, content: Option<String>| {
        web_sys::console::log_1(
            &format!("Modal Success: {}, {}", title, content.unwrap_or_default()).into(),
        );
    });

    let modal_error = Rc::new(|title: String, content: Option<String>| {
        web_sys::console::log_1(
            &format!("Modal Error: {}, {}", title, content.unwrap_or_default()).into(),
        );
    });

    let modal_warning = Rc::new(|title: String, content: Option<String>| {
        web_sys::console::log_1(
            &format!("Modal Warning: {}, {}", title, content.unwrap_or_default()).into(),
        );
    });

    let modal_confirm = Rc::new(|title: String, content: Option<String>| {
        web_sys::console::log_1(
            &format!("Modal Confirm: {}, {}", title, content.unwrap_or_default()).into(),
        );
    });

    // Create instances
    let message_instance = MessageInstance {
        success: message_success,
        error: message_error,
        info: message_info,
        warning: message_warning,
        loading: message_loading,
    };

    let notification_instance = NotificationInstance {
        success: notification_success,
        error: notification_error,
        info: notification_info,
        warning: notification_warning,
    };

    let modal_instance = ModalInstance {
        info: modal_info,
        success: modal_success,
        error: modal_error,
        warning: modal_warning,
        confirm: modal_confirm,
    };

    // Create app context
    let app_context = AppContext {
        message: message_instance,
        notification: notification_instance,
        modal: modal_instance,
    };

    // 这里应该使用 set_signal_effect 来设置全局状态
    APP_CONTEXT.write().replace(app_context);

    // 生成样式
    let app_style = generate_app_style(props.theme.clone());

    // Class name
    let class_name = format!(
        "ant-app {}",
        props.class.clone().unwrap_or_else(|| String::new())
    );

    // Style
    let style_str = props.style.clone().unwrap_or_else(|| String::new());

    match props.component {
        AppComponent::None => {
            // Just render children without container
            app_without_wrapper(props.children)
        }
        AppComponent::Div => {
            rsx! {
                style { {app_style} }
                div {
                    id: props.id.clone(),
                    class: class_name,
                    style: style_str,
                    {props.children}
                }
            }
        }
        AppComponent::Section => {
            rsx! {
                style { {app_style} }
                section {
                    id: props.id.clone(),
                    class: class_name,
                    style: style_str,
                    {props.children}
                }
            }
        }
        AppComponent::Main => {
            rsx! {
                style { {app_style} }
                main {
                    id: props.id.clone(),
                    class: class_name,
                    style: style_str,
                    {props.children}
                }
            }
        }
        AppComponent::Article => {
            rsx! {
                style { {app_style} }
                article {
                    id: props.id.clone(),
                    class: class_name,
                    style: style_str,
                    {props.children}
                }
            }
        }
        AppComponent::Custom(tag) => {
            // Dynamic element generation is more complex in Dioxus
            // For now, we'll default to div when custom is requested
            rsx! {
                style { {app_style} }
                div {
                    id: props.id.clone(),
                    class: class_name,
                    style: style_str,
                    {props.children}
                }
            }
        }
    }
}

/// Render children without wrapper
///
/// This is used when component is set to None
pub fn app_without_wrapper(children: Element) -> Element {
    rsx! {
        {children}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_config_default() {
        let config = MessageConfig::default();
        assert_eq!(config.duration, Some(3.0));
        assert_eq!(config.max_count, None);
    }

    #[test]
    fn test_notification_config_default() {
        let config = NotificationConfig::default();
        assert_eq!(config.duration, Some(4.5));
        assert_eq!(config.placement, Some("topRight".to_string()));
    }

    #[test]
    fn test_app_component_default() {
        let component = AppComponent::default();
        assert!(matches!(component, AppComponent::Div));
    }
}

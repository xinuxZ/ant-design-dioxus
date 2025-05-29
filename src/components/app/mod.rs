use dioxus::prelude::*;
use std::rc::Rc;

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
    let notification_success = Rc::new(|message: String, description: Option<String>| {
        let desc = description.unwrap_or_default();
        web_sys::console::log_1(&format!("Notification Success: {} - {}", message, desc).into());
    });

    let notification_error = Rc::new(|message: String, description: Option<String>| {
        let desc = description.unwrap_or_default();
        web_sys::console::log_1(&format!("Notification Error: {} - {}", message, desc).into());
    });

    let notification_info = Rc::new(|message: String, description: Option<String>| {
        let desc = description.unwrap_or_default();
        web_sys::console::log_1(&format!("Notification Info: {} - {}", message, desc).into());
    });

    let notification_warning = Rc::new(|message: String, description: Option<String>| {
        let desc = description.unwrap_or_default();
        web_sys::console::log_1(&format!("Notification Warning: {} - {}", message, desc).into());
    });

    // Create modal handlers
    let modal_info = Rc::new(|title: String, content: Option<String>| {
        let cont = content.unwrap_or_default();
        web_sys::console::log_1(&format!("Modal Info: {} - {}", title, cont).into());
    });

    let modal_success = Rc::new(|title: String, content: Option<String>| {
        let cont = content.unwrap_or_default();
        web_sys::console::log_1(&format!("Modal Success: {} - {}", title, cont).into());
    });

    let modal_error = Rc::new(|title: String, content: Option<String>| {
        let cont = content.unwrap_or_default();
        web_sys::console::log_1(&format!("Modal Error: {} - {}", title, cont).into());
    });

    let modal_warning = Rc::new(|title: String, content: Option<String>| {
        let cont = content.unwrap_or_default();
        web_sys::console::log_1(&format!("Modal Warning: {} - {}", title, cont).into());
    });

    let modal_confirm = Rc::new(|title: String, content: Option<String>| {
        let cont = content.unwrap_or_default();
        web_sys::console::log_1(&format!("Modal Confirm: {} - {}", title, cont).into());
    });

    // Create app context
    let app_context = AppContext {
        message: MessageInstance {
            success: message_success,
            error: message_error,
            info: message_info,
            warning: message_warning,
            loading: message_loading,
        },
        notification: NotificationInstance {
            success: notification_success,
            error: notification_error,
            info: notification_info,
            warning: notification_warning,
        },
        modal: ModalInstance {
            info: modal_info,
            success: modal_success,
            error: modal_error,
            warning: modal_warning,
            confirm: modal_confirm,
        },
    };

    // Set global context
    use_effect(move || {
        *APP_CONTEXT.write() = Some(app_context.clone());
    });

    let class_name = format!("ant-app {}", props.class.as_deref().unwrap_or(""))
        .trim()
        .to_string();

    match props.component {
        AppComponent::None => {
            // Don't create DOM node, just render children
            rsx! {
                {props.children}
            }
        }
        AppComponent::Div => {
            rsx! {
                div {
                    class: "{class_name}",
                    id: props.id,
                    style: props.style,
                    {props.children}
                }
            }
        }
        AppComponent::Section => {
            rsx! {
                section {
                    class: "{class_name}",
                    id: props.id,
                    style: props.style,
                    {props.children}
                }
            }
        }
        AppComponent::Main => {
            rsx! {
                main {
                    class: "{class_name}",
                    id: props.id,
                    style: props.style,
                    {props.children}
                }
            }
        }
        AppComponent::Article => {
            rsx! {
                article {
                    class: "{class_name}",
                    id: props.id,
                    style: props.style,
                    {props.children}
                }
            }
        }
        AppComponent::Custom(tag) => {
            // For custom tags, we'll use div as fallback since Dioxus has limited dynamic tag support
            rsx! {
                div {
                    class: "{class_name}",
                    id: props.id,
                    style: props.style,
                    "data-tag": "{tag}",
                    {props.children}
                }
            }
        }
    }
}

/// Helper function to create App component with false component prop
pub fn app_without_wrapper(children: Element) -> Element {
    rsx! {
        App {
            component: AppComponent::None,
            children
        }
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
        assert_eq!(component, AppComponent::Div);
    }
}

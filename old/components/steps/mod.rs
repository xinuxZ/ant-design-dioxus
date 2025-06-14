//! Steps 步骤条组件
//!
//! 引导用户按照流程完成任务的导航条。
//!
//! ## 何时使用
//!
//! 当任务复杂或者存在先后关系时，将其分解成一系列步骤，从而简化任务。

use crate::utils::class_names::conditional_class_names_array;
use dioxus::prelude::*;

const STEPS_STYLES: &str = include_str!("./style.css");

/// 步骤状态
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StepStatus {
    /// 等待
    Wait,
    /// 进行中
    Process,
    /// 完成
    Finish,
    /// 错误
    Error,
}

impl Default for StepStatus {
    fn default() -> Self {
        StepStatus::Wait
    }
}

/// 步骤条方向
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StepsDirection {
    /// 水平
    Horizontal,
    /// 垂直
    Vertical,
}

impl Default for StepsDirection {
    fn default() -> Self {
        StepsDirection::Horizontal
    }
}

/// 步骤条大小
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StepsSize {
    /// 默认
    Default,
    /// 小尺寸
    Small,
}

impl Default for StepsSize {
    fn default() -> Self {
        StepsSize::Default
    }
}

/// 步骤条类型
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StepsType {
    /// 默认
    Default,
    /// 导航
    Navigation,
    /// 内联
    Inline,
}

impl Default for StepsType {
    fn default() -> Self {
        StepsType::Default
    }
}

/// 步骤项数据
#[derive(Debug, Clone, PartialEq)]
pub struct StepItem {
    /// 标题
    pub title: String,
    /// 子标题
    pub subtitle: Option<String>,
    /// 描述
    pub description: Option<String>,
    /// 图标
    pub icon: Option<String>,
    /// 状态
    pub status: Option<StepStatus>,
    /// 是否禁用
    pub disabled: bool,
}

impl StepItem {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            subtitle: None,
            description: None,
            icon: None,
            status: None,
            disabled: false,
        }
    }

    pub fn with_subtitle(mut self, subtitle: &str) -> Self {
        self.subtitle = Some(subtitle.to_string());
        self
    }

    pub fn with_description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    pub fn with_icon(mut self, icon: &str) -> Self {
        self.icon = Some(icon.to_string());
        self
    }

    pub fn with_status(mut self, status: StepStatus) -> Self {
        self.status = Some(status);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// Steps 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct StepsProps {
    /// 步骤数据
    pub items: Vec<StepItem>,
    /// 当前步骤
    #[props(default = 0)]
    pub current: usize,
    /// 指定当前步骤的状态
    #[props(default = StepStatus::Process)]
    pub status: StepStatus,
    /// 步骤条方向
    #[props(default = StepsDirection::Horizontal)]
    pub direction: StepsDirection,
    /// 步骤条大小
    #[props(default = StepsSize::Default)]
    pub size: StepsSize,
    /// 步骤条类型
    #[props(default = StepsType::Default)]
    pub r#type: StepsType,
    /// 起始序号，从 0 开始记数
    #[props(default = 0)]
    pub initial: usize,
    /// 是否显示进度点
    #[props(default = false)]
    pub progress_dot: bool,
    /// 点击单个步骤时触发的回调函数
    pub on_change: Option<EventHandler<usize>>,
    /// 自定义样式类名
    #[props(default = String::new())]
    pub class: String,
    /// 自定义样式
    #[props(default = String::new())]
    pub style: String,
}

/// Steps 步骤条组件
#[component]
pub fn Steps(props: StepsProps) -> Element {
    let steps_class = conditional_class_names_array(&[
        ("ant-steps", true),
        (
            "ant-steps-horizontal",
            props.direction == StepsDirection::Horizontal,
        ),
        (
            "ant-steps-vertical",
            props.direction == StepsDirection::Vertical,
        ),
        ("ant-steps-small", props.size == StepsSize::Small),
        (
            "ant-steps-navigation",
            props.r#type == StepsType::Navigation,
        ),
        ("ant-steps-inline", props.r#type == StepsType::Inline),
        ("ant-steps-dot", props.progress_dot),
        (&props.class, !props.class.is_empty()),
    ]);

    // 计算每个步骤的状态
    let get_step_status = |index: usize, item: &StepItem| -> StepStatus {
        if let Some(status) = item.status {
            return status;
        }

        if index < props.current {
            StepStatus::Finish
        } else if index == props.current {
            props.status
        } else {
            StepStatus::Wait
        }
    };

    // 处理步骤点击
    let items_clone = props.items.clone();
    let handle_step_click = move |index: usize| {
        if let Some(callback) = &props.on_change {
            if !items_clone.get(index).map_or(true, |item| item.disabled) {
                callback.call(index);
            }
        }
    };

    rsx! {
        style { {STEPS_STYLES} }

        div {
            class: steps_class,
            style: props.style,

            for (index, item) in props.items.iter().enumerate() {
                {
                    let step_status = get_step_status(index, item);
                    let is_last = index == props.items.len() - 1;

                    let step_class = conditional_class_names_array(&[
                        ("ant-steps-item", true),
                        ("ant-steps-item-wait", step_status == StepStatus::Wait),
                        ("ant-steps-item-process", step_status == StepStatus::Process),
                        ("ant-steps-item-finish", step_status == StepStatus::Finish),
                        ("ant-steps-item-error", step_status == StepStatus::Error),
                        ("ant-steps-item-active", index == props.current),
                        ("ant-steps-item-disabled", item.disabled),
                        ("ant-steps-item-custom", item.icon.is_some()),
                    ]);

                    rsx! {
                        div {
                            class: step_class,
                            onclick: {
                                let handle_click = handle_step_click.clone();
                                move |_| handle_click(index)
                            },

                            // 步骤头部
                            div { class: "ant-steps-item-container",
                                // 连接线（非最后一个步骤）
                                if !is_last {
                                    div { class: "ant-steps-item-tail" }
                                }

                                // 步骤图标
                                div { class: "ant-steps-item-icon",
                                    if props.progress_dot {
                                        span { class: "ant-steps-icon-dot" }
                                    } else if let Some(icon) = &item.icon {
                                        span { class: "ant-steps-icon", {icon.clone()} }
                                    } else {
                                        span { class: "ant-steps-icon",
                                            {
                                                match step_status {
                                                    StepStatus::Finish => "✓".to_string(),
                                                    StepStatus::Error => "✕".to_string(),
                                                    _ => (index + 1 + props.initial).to_string(),
                                                }
                                            }
                                        }
                                    }
                                }

                                // 步骤内容
                                div { class: "ant-steps-item-content",
                                    div { class: "ant-steps-item-title",
                                        {item.title.clone()}

                                        if let Some(subtitle) = &item.subtitle {
                                            div { class: "ant-steps-item-subtitle", {subtitle.clone()} }
                                        }
                                    }

                                    if let Some(description) = &item.description {
                                        div { class: "ant-steps-item-description", {description.clone()} }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// /// Step 单个步骤组件属性
// #[derive(Props, Clone, PartialEq)]
// pub struct StepProps {
//     /// 标题
//     pub title: String,
//     /// 子标题
//     pub subtitle: Option<String>,
//     /// 描述
//     pub description: Option<String>,
//     /// 图标
//     pub icon: Option<String>,
//     /// 状态
//     pub status: Option<StepStatus>,
//     /// 是否禁用
//     #[props(default = false)]
//     pub disabled: bool,
// }

// /// Step 单个步骤组件
// #[component]
// pub fn Step(props: StepProps) -> Element {
//     // 这个组件主要用于 JSX 风格的声明，实际渲染由 Steps 组件处理
//     rsx! {
//         div { "Step component should be used within Steps" }
//     }
// }

/// 步骤项构建器
pub struct StepItemBuilder {
    item: StepItem,
}

impl StepItemBuilder {
    pub fn new(title: &str) -> Self {
        Self {
            item: StepItem::new(title),
        }
    }

    pub fn subtitle(mut self, subtitle: &str) -> Self {
        self.item.subtitle = Some(subtitle.to_string());
        self
    }

    pub fn description(mut self, description: &str) -> Self {
        self.item.description = Some(description.to_string());
        self
    }

    pub fn icon(mut self, icon: &str) -> Self {
        self.item.icon = Some(icon.to_string());
        self
    }

    pub fn status(mut self, status: StepStatus) -> Self {
        self.item.status = Some(status);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.item.disabled = disabled;
        self
    }

    pub fn build(self) -> StepItem {
        self.item
    }
}

/// 便捷的步骤项创建宏
#[macro_export]
macro_rules! step_item {
    ($title:expr) => {
        StepItem::new($title)
    };
    ($title:expr, description: $desc:expr) => {
        StepItem::new($title).with_description($desc)
    };
    ($title:expr, icon: $icon:expr) => {
        StepItem::new($title).with_icon($icon)
    };
    ($title:expr, status: $status:expr) => {
        StepItem::new($title).with_status($status)
    };
    ($title:expr, subtitle: $subtitle:expr, description: $desc:expr) => {
        StepItem::new($title)
            .with_subtitle($subtitle)
            .with_description($desc)
    };
}

/// 便捷的步骤列表创建宏
#[macro_export]
macro_rules! steps_items {
    [$($item:expr),*] => {
        vec![$($item),*]
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step_status_default() {
        assert_eq!(StepStatus::default(), StepStatus::Wait);
    }

    #[test]
    fn test_steps_direction_default() {
        assert_eq!(StepsDirection::default(), StepsDirection::Horizontal);
    }

    #[test]
    fn test_steps_size_default() {
        assert_eq!(StepsSize::default(), StepsSize::Default);
    }

    #[test]
    fn test_steps_type_default() {
        assert_eq!(StepsType::default(), StepsType::Default);
    }

    #[test]
    fn test_step_item_new() {
        let item = StepItem::new("Step 1");
        assert_eq!(item.title, "Step 1");
        assert!(item.subtitle.is_none());
        assert!(item.description.is_none());
        assert!(item.icon.is_none());
        assert!(item.status.is_none());
        assert!(!item.disabled);
    }

    #[test]
    fn test_step_item_builder() {
        let item = StepItemBuilder::new("Step 1")
            .subtitle("Subtitle")
            .description("Description")
            .icon("icon")
            .status(StepStatus::Finish)
            .disabled(true)
            .build();

        assert_eq!(item.title, "Step 1");
        assert_eq!(item.subtitle, Some("Subtitle".to_string()));
        assert_eq!(item.description, Some("Description".to_string()));
        assert_eq!(item.icon, Some("icon".to_string()));
        assert_eq!(item.status, Some(StepStatus::Finish));
        assert!(item.disabled);
    }

    #[test]
    fn test_step_item_with_methods() {
        let item = StepItem::new("Step 1")
            .with_subtitle("Subtitle")
            .with_description("Description")
            .with_icon("icon")
            .with_status(StepStatus::Process)
            .disabled(true);

        assert_eq!(item.title, "Step 1");
        assert_eq!(item.subtitle, Some("Subtitle".to_string()));
        assert_eq!(item.description, Some("Description".to_string()));
        assert_eq!(item.icon, Some("icon".to_string()));
        assert_eq!(item.status, Some(StepStatus::Process));
        assert!(item.disabled);
    }
}

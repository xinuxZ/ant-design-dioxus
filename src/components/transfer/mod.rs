//! Transfer 穿梭框组件
//!
//! 双栏穿梭选择框。
//!
//! ## 何时使用
//!
//! - 用直观的方式在两栏中移动元素，完成选择行为。
//! - 选择一个或以上的选项后，点击对应的方向键，可以把选中的选项移动到另一栏。
//! - 其中左边一栏为 `source`，右边一栏为 `target`，API 的设计也反映了这两个概念。
//!
//! ## 代码演示
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::{Transfer, TransferItem};
//!
//! #[component]
//! fn App() -> Element {
//!     let mut target_keys = use_signal(|| vec!["1".to_string(), "3".to_string()]);
//!
//!     let data_source = vec![
//!         TransferItem::new("1", "选项1"),
//!         TransferItem::new("2", "选项2"),
//!         TransferItem::new("3", "选项3"),
//!         TransferItem::new("4", "选项4"),
//!     ];
//!
//!     rsx! {
//!         Transfer {
//!             data_source: data_source,
//!             target_keys: target_keys(),
//!             on_change: move |keys, _, _| target_keys.set(keys),
//!         }
//!     }
//! }
//! ```

use dioxus::prelude::*;
use serde_json::Value;
use std::collections::HashSet;

const TRANSFER_STYLE: &str = include_str!("./style.css");

/// 穿梭框数据项
#[derive(Debug, Clone, PartialEq)]
pub struct TransferItem {
    /// 唯一标识
    pub key: String,
    /// 显示标题
    pub title: String,
    /// 描述信息
    pub description: Option<String>,
    /// 是否禁用
    pub disabled: bool,
    /// 自定义数据
    pub data: Option<Value>,
}

impl TransferItem {
    /// 创建新的穿梭框数据项
    pub fn new(key: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            title: title.into(),
            description: None,
            disabled: false,
            data: None,
        }
    }

    /// 设置描述信息
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// 设置禁用状态
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// 设置自定义数据
    pub fn with_data(mut self, data: Value) -> Self {
        self.data = Some(data);
        self
    }
}

/// 穿梭框方向
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TransferDirection {
    /// 从左到右
    Left,
    /// 从右到左
    Right,
}

impl TransferDirection {
    pub fn as_str(&self) -> &'static str {
        match self {
            TransferDirection::Left => "left",
            TransferDirection::Right => "right",
        }
    }
}

/// 穿梭框搜索配置
#[derive(Debug, Clone, PartialEq)]
pub struct TransferSearchOption {
    /// 搜索框占位符
    pub placeholder: String,
    /// 默认搜索值
    pub default_value: String,
}

impl Default for TransferSearchOption {
    fn default() -> Self {
        Self {
            placeholder: "请输入搜索内容".to_string(),
            default_value: String::new(),
        }
    }
}

/// 穿梭框本地化配置
#[derive(Debug, Clone, PartialEq)]
pub struct TransferLocale {
    /// 单个项目单位
    pub item_unit: String,
    /// 多个项目单位
    pub items_unit: String,
    /// 搜索框占位符
    pub search_placeholder: String,
    /// 空数据提示
    pub not_found_content: String,
}

impl Default for TransferLocale {
    fn default() -> Self {
        Self {
            item_unit: "项".to_string(),
            items_unit: "项".to_string(),
            search_placeholder: "请输入搜索内容".to_string(),
            not_found_content: "列表为空".to_string(),
        }
    }
}

/// 穿梭框属性
#[derive(Props, Clone, PartialEq)]
pub struct TransferProps {
    /// 数据源，其中的数据将会被渲染到左边一栏中
    #[props(default)]
    pub data_source: Vec<TransferItem>,

    /// 显示在右侧框数据的key集合
    #[props(default)]
    pub target_keys: Vec<String>,

    /// 设置选中项的key集合
    #[props(default)]
    pub selected_keys: Vec<String>,

    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,

    /// 标题集合，顺序从左至右
    #[props(default)]
    pub titles: Vec<String>,

    /// 操作文案集合，顺序从上至下
    #[props(default)]
    pub operations: Vec<String>,

    /// 是否显示搜索框
    #[props(default = false)]
    pub show_search: bool,

    /// 搜索框配置
    #[props(default)]
    pub search_option: Option<TransferSearchOption>,

    /// 是否显示全选
    #[props(default = true)]
    pub show_select_all: bool,

    /// 自定义全选标签
    #[props(default)]
    pub select_all_labels: Vec<String>,

    /// 单向样式
    #[props(default = false)]
    pub one_way: bool,

    /// 本地化配置
    #[props(default)]
    pub locale: TransferLocale,

    /// 自定义CSS类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,

    /// 列表样式
    #[props(default)]
    pub list_style: Option<String>,

    /// 操作栏样式
    #[props(default)]
    pub operation_style: Option<String>,

    /// 选项在两栏之间转移时的回调函数
    #[props(default)]
    pub on_change: Option<Callback<(Vec<String>, TransferDirection, Vec<String>)>>,

    /// 选中项发生改变时的回调函数
    #[props(default)]
    pub on_select_change: Option<Callback<(Vec<String>, Vec<String>)>>,

    /// 搜索框内容时改变时的回调函数
    #[props(default)]
    pub on_search: Option<Callback<(TransferDirection, String)>>,

    /// 滚动时的回调函数
    #[props(default)]
    pub on_scroll: Option<Callback<(TransferDirection, Event<ScrollData>)>>,

    /// 自定义过滤函数
    #[props(default)]
    pub filter_option: Option<Callback<(String, TransferItem, TransferDirection), bool>>,
}

/// 穿梭框组件
#[component]
pub fn Transfer(props: TransferProps) -> Element {
    // 内部状态
    let mut left_selected_keys = use_signal(|| Vec::<String>::new());
    let mut right_selected_keys = use_signal(|| Vec::<String>::new());
    let mut left_search_value = use_signal(|| String::new());
    let mut right_search_value = use_signal(|| String::new());

    // 计算左右两栏的数据
    let target_keys_set: HashSet<String> = props.target_keys.iter().cloned().collect();

    let left_data_source: Vec<TransferItem> = props
        .data_source
        .iter()
        .filter(|item| !target_keys_set.contains(&item.key))
        .cloned()
        .collect();

    let right_data_source: Vec<TransferItem> = props
        .data_source
        .iter()
        .filter(|item| target_keys_set.contains(&item.key))
        .cloned()
        .collect();

    // 过滤数据
    let filter_data = move |data: &[TransferItem],
                            search_value: &str,
                            direction: TransferDirection|
          -> Vec<TransferItem> {
        if search_value.is_empty() {
            return data.to_vec();
        }

        data.iter()
            .filter(|item| {
                if let Some(filter_fn) = &props.filter_option {
                    filter_fn.call((search_value.to_string(), (*item).clone(), direction))
                } else {
                    item.title
                        .to_lowercase()
                        .contains(&search_value.to_lowercase())
                        || item.description.as_ref().map_or(false, |desc| {
                            desc.to_lowercase().contains(&search_value.to_lowercase())
                        })
                }
            })
            .cloned()
            .collect()
    };

    let filtered_left_data = filter_data(
        &left_data_source,
        &left_search_value(),
        TransferDirection::Left,
    );
    let filtered_right_data = filter_data(
        &right_data_source,
        &right_search_value(),
        TransferDirection::Right,
    );

    // 移动选中项
    let target_keys = props.target_keys.clone();
    let move_to_target = move |_| {
        if !props.disabled {
            let move_keys = left_selected_keys();
            if !move_keys.is_empty() {
                let mut new_target_keys = target_keys.clone();
                new_target_keys.extend(move_keys.clone());

                left_selected_keys.set(Vec::new());

                if let Some(on_change) = &props.on_change {
                    on_change.call((new_target_keys, TransferDirection::Left, move_keys));
                }
            }
        }
    };

    let target_keys = props.target_keys.clone();
    let move_to_source = move |_| {
        if !props.disabled {
            let move_keys = right_selected_keys();
            if !move_keys.is_empty() {
                let move_keys_set: HashSet<String> = move_keys.iter().cloned().collect();
                let new_target_keys: Vec<String> = target_keys
                    .iter()
                    .filter(|key| !move_keys_set.contains(*key))
                    .cloned()
                    .collect();

                right_selected_keys.set(Vec::new());

                if let Some(on_change) = &props.on_change {
                    on_change.call((new_target_keys, TransferDirection::Right, move_keys));
                }
            }
        }
    };

    // 处理选择变化
    let handle_left_select_change = move |keys: Vec<String>| {
        left_selected_keys.set(keys.clone());
        if let Some(on_select_change) = &props.on_select_change {
            on_select_change.call((keys, right_selected_keys()));
        }
    };

    let handle_right_select_change = move |keys: Vec<String>| {
        right_selected_keys.set(keys.clone());
        if let Some(on_select_change) = &props.on_select_change {
            on_select_change.call((left_selected_keys(), keys));
        }
    };

    // 处理搜索
    let handle_left_search = move |value: String| {
        left_search_value.set(value.clone());
        if let Some(on_search) = &props.on_search {
            on_search.call((TransferDirection::Left, value));
        }
    };

    let handle_right_search = move |value: String| {
        right_search_value.set(value.clone());
        if let Some(on_search) = &props.on_search {
            on_search.call((TransferDirection::Right, value));
        }
    };

    // 计算操作按钮状态
    let left_disabled = props.disabled || left_selected_keys().is_empty();
    let right_disabled = props.disabled || right_selected_keys().is_empty() || props.one_way;

    // 获取标题
    let left_title = props
        .titles
        .get(0)
        .cloned()
        .unwrap_or_else(|| "源列表".to_string());
    let right_title = props
        .titles
        .get(1)
        .cloned()
        .unwrap_or_else(|| "目标列表".to_string());

    // 获取操作文案
    let to_right_text = props
        .operations
        .get(0)
        .cloned()
        .unwrap_or_else(|| ">".to_string());
    let to_left_text = props
        .operations
        .get(1)
        .cloned()
        .unwrap_or_else(|| "<".to_string());

    let transfer_class = format!(
        "ant-transfer {}{}{}",
        if props.disabled {
            " ant-transfer-disabled"
        } else {
            ""
        },
        if props.one_way {
            " ant-transfer-one-way"
        } else {
            ""
        },
        props.class.as_deref().unwrap_or("")
    );

    rsx! {
        style { {TRANSFER_STYLE} }

        div {
            class: transfer_class,
            style: props.style.as_deref().unwrap_or(""),

            // 左侧列表
            TransferList {
                title: left_title,
                data_source: filtered_left_data,
                selected_keys: left_selected_keys(),
                disabled: props.disabled,
                show_search: props.show_search,
                search_placeholder: props.search_option.as_ref().map(|s| s.placeholder.clone()).unwrap_or_else(|| props.locale.search_placeholder.clone()),
                search_value: left_search_value(),
                show_select_all: props.show_select_all,
                select_all_label: props.select_all_labels.get(0).cloned(),
                locale: props.locale.clone(),
                list_style: props.list_style.clone(),
                direction: TransferDirection::Left,
                on_select_change: handle_left_select_change,
                on_search: handle_left_search,
                on_scroll: props.on_scroll.clone(),
            }
            // 操作按钮
            div {
                class: "ant-transfer-operation",
                style: props.operation_style.as_deref().unwrap_or(""),

                button {
                    class: format!("ant-btn ant-transfer-operation-btn{}", if left_disabled { " ant-btn-disabled" } else { "" }),
                    disabled: left_disabled,
                    onclick: move_to_target,
                    title: "向右移动",

                    span { class: "ant-transfer-operation-btn-icon", {to_right_text} }
                }

                if !props.one_way {
                    button {
                        class: format!("ant-btn ant-transfer-operation-btn{}", if right_disabled { " ant-btn-disabled" } else { "" }),
                        disabled: right_disabled,
                        onclick: move_to_source,
                        title: "向左移动",

                        span { class: "ant-transfer-operation-btn-icon", {to_left_text} }
                    }
                }
            }

            // 右侧列表
            TransferList {
                title: right_title,
                data_source: filtered_right_data,
                selected_keys: right_selected_keys(),
                disabled: props.disabled,
                show_search: props.show_search,
                search_placeholder: props.search_option.as_ref().map(|s| s.placeholder.clone()).unwrap_or_else(|| props.locale.search_placeholder.clone()),
                search_value: right_search_value(),
                show_select_all: props.show_select_all,
                select_all_label: props.select_all_labels.get(1).cloned(),
                locale: props.locale.clone(),
                list_style: props.list_style.clone(),
                direction: TransferDirection::Right,
                on_select_change: handle_right_select_change,
                on_search: handle_right_search,
                on_scroll: props.on_scroll.clone(),
            }
        }
    }
}

/// 穿梭框列表属性
#[derive(Props, Clone, PartialEq)]
struct TransferListProps {
    title: String,
    data_source: Vec<TransferItem>,
    selected_keys: Vec<String>,
    disabled: bool,
    show_search: bool,
    search_placeholder: String,
    search_value: String,
    show_select_all: bool,
    select_all_label: Option<String>,
    locale: TransferLocale,
    list_style: Option<String>,
    direction: TransferDirection,
    on_select_change: Callback<Vec<String>>,
    on_search: Callback<String>,
    on_scroll: Option<Callback<(TransferDirection, Event<ScrollData>)>>,
}

/// 穿梭框列表组件
#[component]
fn TransferList(props: TransferListProps) -> Element {
    let selected_keys_set: HashSet<String> = props.selected_keys.iter().cloned().collect();

    // 计算全选状态
    let selectable_items: Vec<&TransferItem> = props
        .data_source
        .iter()
        .filter(|item| !item.disabled)
        .collect();
    let selected_count = selectable_items
        .iter()
        .filter(|item| selected_keys_set.contains(&item.key))
        .count();
    let total_count = selectable_items.len();

    let is_all_selected = total_count > 0 && selected_count == total_count;
    let is_indeterminate = selected_count > 0 && selected_count < total_count;

    // 处理全选
    let handle_select_all = Callback::new(move |checked: bool| {
        if !props.disabled {
            let new_keys = if checked {
                let mut keys = props.selected_keys.clone();
                for item in &selectable_items {
                    if !keys.contains(&item.key) {
                        keys.push(item.key.clone());
                    }
                }
                keys
            } else {
                let selectable_keys: HashSet<String> = selectable_items
                    .iter()
                    .map(|item| item.key.clone())
                    .collect();
                props
                    .selected_keys
                    .iter()
                    .filter(|key| !selectable_keys.contains(*key))
                    .cloned()
                    .collect()
            };

            props.on_select_change.call(new_keys);
        }
    });

    // 处理单项选择
    let handle_item_select = Callback::new(move |args: (String, bool)| {
        let (key, checked) = args;
        if !props.disabled {
            let mut new_keys = props.selected_keys.clone();
            if checked {
                if !new_keys.contains(&key) {
                    new_keys.push(key);
                }
            } else {
                new_keys.retain(|k| k != &key);
            }

            props.on_select_change.call(new_keys);
        }
    });

    // 处理搜索输入
    let handle_search_input = move |evt: Event<FormData>| {
        let value = evt.value();
        props.on_search.call(value);
    };

    // 处理滚动
    let handle_scroll = move |evt: Event<ScrollData>| {
        if let Some(on_scroll) = &props.on_scroll {
            on_scroll.call((props.direction, evt));
        }
    };

    let list_class = format!(
        "ant-transfer-list{}",
        if props.disabled {
            " ant-transfer-list-disabled"
        } else {
            ""
        }
    );

    rsx! {
        div {
            class: list_class,
            style: props.list_style.as_deref().unwrap_or(""),

            // 列表头部
            div {
                class: "ant-transfer-list-header",

                // 全选复选框
                if props.show_select_all {
                    label {
                        class: "ant-transfer-list-checkbox",

                        input {
                            r#type: "checkbox",
                            checked: is_all_selected,
                            disabled: props.disabled || total_count == 0,
                            class: if is_indeterminate { "ant-checkbox-indeterminate" } else { "" },
                            onchange: move |evt| handle_select_all(evt.checked()),
                        }
                        span { class: "ant-transfer-list-checkbox-inner" }
                    }
                }

                // 标题和统计
                span {
                    class: "ant-transfer-list-header-selected",
                    span { class: "ant-transfer-list-header-title", {props.title} }
                    span { class: "ant-transfer-list-header-count",
                        {selected_count} "/" {total_count}
                    }
                }

                // 自定义全选标签
                if let Some(label) = &props.select_all_label {
                    span { class: "ant-transfer-list-header-label", {label} }
                }
            }

            // 搜索框
            if props.show_search {
                div {
                    class: "ant-transfer-list-search",

                    input {
                        r#type: "text",
                        class: "ant-input ant-transfer-list-search-input",
                        placeholder: props.search_placeholder,
                        value: props.search_value,
                        disabled: props.disabled,
                        oninput: handle_search_input,
                    }
                    span { class: "ant-transfer-list-search-icon", "🔍" }
                }
            }

            // 列表内容
            div {
                class: "ant-transfer-list-content",
                onscroll: handle_scroll,

                if props.data_source.is_empty() {
                    div {
                        class: "ant-transfer-list-content-empty",
                        {props.locale.not_found_content},
                    }
                } else {
                    for item in &props.data_source {
                        TransferListItem {
                            key: item.key,
                            item: item.clone(),
                            selected: selected_keys_set.contains(&item.key),
                            disabled: props.disabled || item.disabled,
                            on_select: handle_item_select,
                        }
                    }
                }
            }
        }
    }
}

/// 穿梭框列表项属性
#[derive(Props, Clone, PartialEq)]
struct TransferListItemProps {
    item: TransferItem,
    selected: bool,
    disabled: bool,
    on_select: Callback<(String, bool)>,
}

/// 穿梭框列表项组件
#[component]
fn TransferListItem(props: TransferListItemProps) -> Element {
    let item_key = props.item.key.clone();

    let handle_click = {
        let key = item_key.clone();
        move |_| {
            if !props.disabled {
                props.on_select.call((key.clone(), !props.selected));
            }
        }
    };

    let handle_checkbox_change = {
        let key = item_key.clone();
        move |evt: Event<FormData>| {
            if !props.disabled {
                props.on_select.call((key.clone(), evt.checked()));
            }
        }
    };

    let item_class = format!(
        "ant-transfer-list-content-item{}{}",
        if props.selected {
            " ant-transfer-list-content-item-selected"
        } else {
            ""
        },
        if props.disabled {
            " ant-transfer-list-content-item-disabled"
        } else {
            ""
        }
    );

    rsx! {
        div {
            class: item_class,
            onclick: handle_click,

            // 复选框
            label { class: "ant-transfer-list-content-item-checkbox",
                input {
                    r#type: "checkbox",
                    checked: props.selected,
                    disabled: props.disabled,
                    onchange: handle_checkbox_change,
                }
                span { class: "ant-transfer-list-content-item-checkbox-inner" }
            }

            // 内容
            div { class: "ant-transfer-list-content-item-text",
                div { class: "ant-transfer-list-content-item-title", {props.item.title} }
                if let Some(description) = &props.item.description {
                    div { class: "ant-transfer-list-content-item-description", "{description}" }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transfer_item_new() {
        let item = TransferItem::new("1", "测试项目");
        assert_eq!(item.key, "1");
        assert_eq!(item.title, "测试项目");
        assert_eq!(item.description, None);
        assert!(!item.disabled);
        assert_eq!(item.data, None);
    }

    #[test]
    fn test_transfer_item_with_description() {
        let item = TransferItem::new("1", "测试项目").with_description("这是一个测试项目");
        assert_eq!(item.description, Some("这是一个测试项目".to_string()));
    }

    #[test]
    fn test_transfer_item_with_disabled() {
        let item = TransferItem::new("1", "测试项目").with_disabled(true);
        assert!(item.disabled);
    }

    #[test]
    fn test_transfer_direction_as_str() {
        assert_eq!(TransferDirection::Left.as_str(), "left");
        assert_eq!(TransferDirection::Right.as_str(), "right");
    }

    #[test]
    fn test_transfer_locale_default() {
        let locale = TransferLocale::default();
        assert_eq!(locale.item_unit, "项");
        assert_eq!(locale.items_unit, "项");
        assert_eq!(locale.search_placeholder, "请输入搜索内容");
        assert_eq!(locale.not_found_content, "列表为空");
    }
}

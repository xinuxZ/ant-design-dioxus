use dioxus::prelude::*;

/// Tree select size
#[derive(Clone, Debug, PartialEq)]
pub enum TreeSelectSize {
    Small,
    Middle,
    Large,
}

impl Default for TreeSelectSize {
    fn default() -> Self {
        Self::Middle
    }
}

impl TreeSelectSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            TreeSelectSize::Small => "small",
            TreeSelectSize::Middle => "middle",
            TreeSelectSize::Large => "large",
        }
    }
}

/// Tree select placement
#[derive(Clone, Debug, PartialEq)]
pub enum TreeSelectPlacement {
    BottomLeft,
    BottomRight,
    TopLeft,
    TopRight,
}

impl Default for TreeSelectPlacement {
    fn default() -> Self {
        Self::BottomLeft
    }
}

impl TreeSelectPlacement {
    pub fn as_str(&self) -> &'static str {
        match self {
            TreeSelectPlacement::BottomLeft => "bottomLeft",
            TreeSelectPlacement::BottomRight => "bottomRight",
            TreeSelectPlacement::TopLeft => "topLeft",
            TreeSelectPlacement::TopRight => "topRight",
        }
    }
}

/// Tree select status
#[derive(Clone, Debug, PartialEq)]
pub enum TreeSelectStatus {
    Error,
    Warning,
}

impl TreeSelectStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            TreeSelectStatus::Error => "error",
            TreeSelectStatus::Warning => "warning",
        }
    }
}

/// Tree select variant
#[derive(Clone, Debug, PartialEq)]
pub enum TreeSelectVariant {
    Outlined,
    Filled,
    Borderless,
    Underlined,
}

impl Default for TreeSelectVariant {
    fn default() -> Self {
        Self::Outlined
    }
}

impl TreeSelectVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            TreeSelectVariant::Outlined => "outlined",
            TreeSelectVariant::Filled => "filled",
            TreeSelectVariant::Borderless => "borderless",
            TreeSelectVariant::Underlined => "underlined",
        }
    }
}

/// Show checked strategy
#[derive(Clone, Debug, PartialEq)]
pub enum ShowCheckedStrategy {
    ShowAll,
    ShowParent,
    ShowChild,
}

impl Default for ShowCheckedStrategy {
    fn default() -> Self {
        Self::ShowChild
    }
}

impl ShowCheckedStrategy {
    pub fn as_str(&self) -> &'static str {
        match self {
            ShowCheckedStrategy::ShowAll => "SHOW_ALL",
            ShowCheckedStrategy::ShowParent => "SHOW_PARENT",
            ShowCheckedStrategy::ShowChild => "SHOW_CHILD",
        }
    }
}

/// Tree expand action
#[derive(Clone, Debug, PartialEq)]
pub enum TreeExpandAction {
    False,
    Click,
    DoubleClick,
}

impl Default for TreeExpandAction {
    fn default() -> Self {
        Self::False
    }
}

/// Field names configuration
#[derive(Clone, Debug, PartialEq)]
pub struct FieldNames {
    pub label: String,
    pub value: String,
    pub children: String,
}

impl Default for FieldNames {
    fn default() -> Self {
        Self {
            label: "label".to_string(),
            value: "value".to_string(),
            children: "children".to_string(),
        }
    }
}

/// Tree data simple mode configuration
#[derive(Clone, Debug, PartialEq)]
pub struct TreeDataSimpleMode {
    pub enabled: bool,
    pub id: String,
    pub p_id: String,
    pub root_p_id: String,
}

impl Default for TreeDataSimpleMode {
    fn default() -> Self {
        Self {
            enabled: false,
            id: "id".to_string(),
            p_id: "pId".to_string(),
            root_p_id: "rootPId".to_string(),
        }
    }
}

/// Tree node data
#[derive(Clone, Debug, PartialEq)]
pub struct TreeNodeData {
    /// Node value
    pub value: String,
    /// Node title/label
    pub title: String,
    /// Node key
    pub key: Option<String>,
    /// Child nodes
    pub children: Option<Vec<TreeNodeData>>,
    /// Whether node is disabled
    pub disabled: Option<bool>,
    /// Whether checkbox is disabled
    pub disable_checkbox: Option<bool>,
    /// Whether node is selectable
    pub selectable: Option<bool>,
    /// Whether node is checkable
    pub checkable: Option<bool>,
    /// Whether node is a leaf
    pub is_leaf: Option<bool>,
    /// Custom icon
    pub icon: Option<Element>,
    /// Custom switcher icon
    pub switcher_icon: Option<Element>,
}

impl TreeNodeData {
    pub fn new(value: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            title: title.into(),
            key: None,
            children: None,
            disabled: None,
            disable_checkbox: None,
            selectable: None,
            checkable: None,
            is_leaf: None,
            icon: None,
            switcher_icon: None,
        }
    }

    pub fn with_children(mut self, children: Vec<TreeNodeData>) -> Self {
        self.children = Some(children);
        self
    }

    pub fn with_key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = Some(disabled);
        self
    }

    pub fn is_leaf(mut self, is_leaf: bool) -> Self {
        self.is_leaf = Some(is_leaf);
        self
    }
}

/// Clear icon configuration
#[derive(Clone, Debug, PartialEq)]
pub struct ClearIconConfig {
    pub clear_icon: Option<Element>,
}

/// Tree select value type
#[derive(Clone, Debug, PartialEq)]
pub enum TreeSelectValue {
    Single(String),
    Multiple(Vec<String>),
    LabelInValue(TreeSelectLabelValue),
    MultipleLabelInValue(Vec<TreeSelectLabelValue>),
}

/// Label in value structure
#[derive(Clone, Debug, PartialEq)]
pub struct TreeSelectLabelValue {
    pub value: String,
    pub label: String,
    pub half_checked: Vec<String>,
}

/// Tree select properties
#[derive(Props, Clone, PartialEq)]
pub struct TreeSelectProps {
    /// Tree data
    #[props(default)]
    pub tree_data: Vec<TreeNodeData>,

    /// Current value
    #[props(default)]
    pub value: Option<TreeSelectValue>,

    /// Default value
    #[props(default)]
    pub default_value: Option<TreeSelectValue>,

    /// Allow clear
    #[props(default)]
    pub allow_clear: bool,

    /// Auto clear search value
    #[props(default = true)]
    pub auto_clear_search_value: bool,

    /// Default open state
    #[props(default)]
    pub default_open: bool,

    /// Disabled state
    #[props(default)]
    pub disabled: bool,

    /// Popup match select width
    #[props(default = true)]
    pub popup_match_select_width: bool,

    /// Field names configuration
    #[props(default)]
    pub field_names: FieldNames,

    /// Filter tree node function
    #[props(default)]
    pub filter_tree_node: Option<fn(&str, &TreeNodeData) -> bool>,

    /// Label in value mode
    #[props(default)]
    pub label_in_value: bool,

    /// List height
    #[props(default = 256)]
    pub list_height: i32,

    /// Load data function
    #[props(default)]
    pub load_data: Option<fn(&TreeNodeData) -> ()>,

    /// Max tag count
    #[props(default)]
    pub max_tag_count: Option<i32>,

    /// Max count
    #[props(default)]
    pub max_count: Option<i32>,

    /// Max tag text length
    #[props(default)]
    pub max_tag_text_length: Option<i32>,

    /// Multiple selection
    #[props(default)]
    pub multiple: bool,

    /// Not found content
    #[props(default)]
    pub not_found_content: Option<String>,

    /// Open state
    #[props(default)]
    pub open: Option<bool>,

    /// Placeholder
    #[props(default)]
    pub placeholder: Option<String>,

    /// Placement
    #[props(default)]
    pub placement: TreeSelectPlacement,

    /// Prefix
    pub prefix: Option<Element>,

    /// Search value
    #[props(default)]
    pub search_value: Option<String>,

    /// Show checked strategy
    #[props(default)]
    pub show_checked_strategy: ShowCheckedStrategy,

    /// Show search
    #[props(default)]
    pub show_search: Option<bool>,

    /// Size
    #[props(default)]
    pub size: TreeSelectSize,

    /// Status
    #[props(default)]
    pub status: Option<TreeSelectStatus>,

    /// Suffix icon
    pub suffix_icon: Option<Element>,

    /// Switcher icon
    pub switcher_icon: Option<Element>,

    /// Tree checkable
    #[props(default)]
    pub tree_checkable: bool,

    /// Tree check strictly
    #[props(default)]
    pub tree_check_strictly: bool,

    /// Tree data simple mode
    #[props(default)]
    pub tree_data_simple_mode: TreeDataSimpleMode,

    /// Tree default expand all
    #[props(default)]
    pub tree_default_expand_all: bool,

    /// Tree default expanded keys
    #[props(default)]
    pub tree_default_expanded_keys: Vec<String>,

    /// Tree expand action
    #[props(default)]
    pub tree_expand_action: TreeExpandAction,

    /// Tree expanded keys
    #[props(default)]
    pub tree_expanded_keys: Option<Vec<String>>,

    /// Tree icon
    #[props(default)]
    pub tree_icon: bool,

    /// Tree loaded keys
    #[props(default)]
    pub tree_loaded_keys: Vec<String>,

    /// Tree line
    #[props(default)]
    pub tree_line: bool,

    /// Variant
    #[props(default)]
    pub variant: TreeSelectVariant,

    /// Virtual scrolling
    #[props(default = true)]
    pub r#virtual: bool,

    /// Change callback
    #[props(default)]
    pub on_change: Option<EventHandler<TreeSelectValue>>,

    /// Dropdown visible change callback
    #[props(default)]
    pub on_dropdown_visible_change: Option<EventHandler<bool>>,

    /// Search callback
    #[props(default)]
    pub on_search: Option<EventHandler<String>>,

    /// Select callback
    #[props(default)]
    pub on_select: Option<EventHandler<(String, TreeNodeData)>>,

    /// Tree expand callback
    #[props(default)]
    pub on_tree_expand: Option<EventHandler<Vec<String>>>,

    /// CSS class name
    #[props(default)]
    pub class: Option<String>,

    /// Inline style
    #[props(default)]
    pub style: Option<String>,

    /// Element ID
    #[props(default)]
    pub id: Option<String>,
}

/// Tree select component
#[component]
pub fn TreeSelect(props: TreeSelectProps) -> Element {
    let is_open = use_signal(|| props.open.unwrap_or(props.default_open));
    let current_value = use_signal(|| props.value.clone().or_else(|| props.default_value.clone()));
    let search_value = use_signal(|| props.search_value.clone().unwrap_or_default());
    let expanded_keys = use_signal(|| {
        if let Some(keys) = &props.tree_expanded_keys {
            keys.clone()
        } else {
            props.tree_default_expanded_keys.clone()
        }
    });
    let filtered_tree_data = use_signal(|| props.tree_data.clone());

    let class_name = format!(
        "ant-tree-select ant-tree-select-{} ant-tree-select-{} {} {} {} {}",
        props.size.as_str(),
        props.variant.as_str(),
        if props.disabled {
            "ant-tree-select-disabled"
        } else {
            ""
        },
        if props.multiple {
            "ant-tree-select-multiple"
        } else {
            "ant-tree-select-single"
        },
        if let Some(status) = &props.status {
            format!("ant-tree-select-status-{}", status.as_str())
        } else {
            String::new()
        },
        props.class.as_deref().unwrap_or("")
    )
    .trim()
    .to_string();

    let handle_selector_click = move |_evt: MouseEvent| {
        if !props.disabled {
            let new_open = !*is_open.read();
            is_open.set(new_open);
            if let Some(on_dropdown_visible_change) = &props.on_dropdown_visible_change {
                on_dropdown_visible_change.call(new_open);
            }
        }
    };

    let handle_search = move |evt: FormEvent| {
        let value = evt.value();
        search_value.set(value.clone());

        // Filter tree data based on search
        if !value.is_empty() {
            let filtered = filter_tree_data(&props.tree_data, &value, &props.filter_tree_node);
            filtered_tree_data.set(filtered);
        } else {
            filtered_tree_data.set(props.tree_data.clone());
        }

        if let Some(on_search) = &props.on_search {
            on_search.call(value);
        }
    };

    let handle_node_select = move |node_value: String, node_data: TreeNodeData| {
        if props.multiple {
            // Handle multiple selection
            let mut new_values = match current_value.read().as_ref() {
                Some(TreeSelectValue::Multiple(values)) => values.clone(),
                _ => Vec::new(),
            };

            if let Some(pos) = new_values.iter().position(|v| v == &node_value) {
                new_values.remove(pos);
            } else {
                new_values.push(node_value.clone());
            }

            let new_value = TreeSelectValue::Multiple(new_values);
            current_value.set(Some(new_value.clone()));

            if let Some(on_change) = &props.on_change {
                on_change.call(new_value);
            }
        } else {
            // Handle single selection
            let new_value = TreeSelectValue::Single(node_value.clone());
            current_value.set(Some(new_value.clone()));
            is_open.set(false);

            if let Some(on_change) = &props.on_change {
                on_change.call(new_value);
            }

            if let Some(on_dropdown_visible_change) = &props.on_dropdown_visible_change {
                on_dropdown_visible_change.call(false);
            }
        }

        if let Some(on_select) = &props.on_select {
            on_select.call((node_value, node_data));
        }
    };

    let handle_clear = move |evt: MouseEvent| {
        evt.stop_propagation();
        current_value.set(None);
        if let Some(on_change) = &props.on_change {
            if props.multiple {
                on_change.call(TreeSelectValue::Multiple(Vec::new()));
            } else {
                on_change.call(TreeSelectValue::Single(String::new()));
            }
        }
    };

    let handle_node_expand = move |node_key: String| {
        let mut new_expanded = expanded_keys.read().clone();
        if let Some(pos) = new_expanded.iter().position(|k| k == &node_key) {
            new_expanded.remove(pos);
        } else {
            new_expanded.push(node_key);
        }
        expanded_keys.set(new_expanded.clone());

        if let Some(on_tree_expand) = &props.on_tree_expand {
            on_tree_expand.call(new_expanded);
        }
    };

    rsx! {
        div {
            class: "{class_name}",
            id: props.id,
            style: props.style,

            // Selector
            div {
                class: "ant-tree-select-selector",
                onclick: handle_selector_click,

                // Prefix
                if let Some(prefix) = props.prefix {
                    span {
                        class: "ant-tree-select-prefix",
                        {prefix}
                    }
                }

                // Selection display
                TreeSelectSelection {
                    value: current_value.read().clone(),
                    placeholder: props.placeholder.clone(),
                    multiple: props.multiple,
                    tree_data: props.tree_data.clone(),
                    max_tag_count: props.max_tag_count,
                    max_tag_text_length: props.max_tag_text_length,
                }

                // Search input
                if props.show_search.unwrap_or(!props.multiple) {
                    input {
                        class: "ant-tree-select-search-input",
                        value: "{search_value.read()}",
                        placeholder: if current_value.read().is_none() { props.placeholder.as_deref().unwrap_or("") } else { "" },
                        oninput: handle_search,
                    }
                }

                // Clear button
                if props.allow_clear && current_value.read().is_some() {
                    span {
                        class: "ant-tree-select-clear",
                        onclick: handle_clear,
                        "×"
                    }
                }

                // Suffix icon
                span {
                    class: "ant-tree-select-arrow",
                    if let Some(suffix_icon) = props.suffix_icon {
                        {suffix_icon}
                    } else {
                        "▼"
                    }
                }
            }

            // Dropdown
            if *is_open.read() {
                div {
                    class: "ant-tree-select-dropdown",
                    style: "max-height: {props.list_height}px; overflow: auto;",

                    if filtered_tree_data.read().is_empty() {
                        div {
                            class: "ant-tree-select-empty",
                            {props.not_found_content.as_deref().unwrap_or("Not Found")}
                        }
                    } else {
                        TreeSelectTree {
                            tree_data: filtered_tree_data.read().clone(),
                            selected_value: current_value.read().clone(),
                            expanded_keys: expanded_keys.read().clone(),
                            multiple: props.multiple,
                            checkable: props.tree_checkable,
                            check_strictly: props.tree_check_strictly,
                            show_icon: props.tree_icon,
                            show_line: props.tree_line,
                            switcher_icon: props.switcher_icon.clone(),
                            on_select: EventHandler::new(handle_node_select),
                            on_expand: handle_node_expand,
                        }
                    }
                }
            }
        }
    }
}

/// Tree select selection display component
#[component]
fn TreeSelectSelection(
    value: Option<TreeSelectValue>,
    placeholder: Option<String>,
    multiple: bool,
    tree_data: Vec<TreeNodeData>,
    max_tag_count: Option<i32>,
    max_tag_text_length: Option<i32>,
) -> Element {
    let get_node_title = |node_value: &str| -> String {
        find_node_by_value(&tree_data, node_value)
            .map(|node| node.title.clone())
            .unwrap_or_else(|| node_value.to_string())
    };

    match &value {
        Some(TreeSelectValue::Single(val)) => {
            rsx! {
                span {
                    class: "ant-tree-select-selection-item",
                    {get_node_title(val)}
                }
            }
        }
        Some(TreeSelectValue::Multiple(vals)) => {
            let display_count = max_tag_count.unwrap_or(vals.len() as i32) as usize;
            let visible_vals = &vals[..display_count.min(vals.len())];
            let remaining_count = vals.len().saturating_sub(display_count);

            rsx! {
                div {
                    class: "ant-tree-select-selection-overflow",
                    for val in visible_vals {
                        span {
                            key: "{val}",
                            class: "ant-tree-select-selection-item",

                            span {
                                class: "ant-tree-select-selection-item-content",
                                {truncate_text(&get_node_title(val), max_tag_text_length)}
                            }

                            span {
                                class: "ant-tree-select-selection-item-remove",
                                "×"
                            }
                        }
                    }

                    if remaining_count > 0 {
                        span {
                            class: "ant-tree-select-selection-item ant-tree-select-selection-overflow-item",
                            "+{remaining_count} ..."
                        }
                    }
                }
            }
        }
        _ => {
            rsx! {
                span {
                    class: "ant-tree-select-selection-placeholder",
                    {placeholder.as_deref().unwrap_or("Please select")}
                }
            }
        }
    }
}

/// Tree select tree component
#[component]
fn TreeSelectTree(
    tree_data: Vec<TreeNodeData>,
    selected_value: Option<TreeSelectValue>,
    expanded_keys: Vec<String>,
    multiple: bool,
    checkable: bool,
    check_strictly: bool,
    show_icon: bool,
    show_line: bool,
    switcher_icon: Option<Element>,
    on_select: EventHandler<(String, TreeNodeData)>,
    on_expand: EventHandler<String>,
) -> Element {
    let is_selected = |node_value: &str| -> bool {
        match &selected_value {
            Some(TreeSelectValue::Single(val)) => val == node_value,
            Some(TreeSelectValue::Multiple(vals)) => vals.contains(&node_value.to_string()),
            _ => false,
        }
    };

    rsx! {
        div {
            class: "ant-tree-select-tree",
            for node in &tree_data {
                TreeSelectNode {
                    key: "{node.value}",
                    node: node.clone(),
                    selected: is_selected(&node.value),
                    expanded: expanded_keys.contains(&node.key.as_ref().unwrap_or(&node.value)),
                    multiple,
                    checkable,
                    show_icon,
                    show_line,
                    switcher_icon: switcher_icon.clone(),
                    on_select,
                    on_expand,
                }
            }
        }
    }
}

/// Tree select node component
#[component]
fn TreeSelectNode(
    node: TreeNodeData,
    selected: bool,
    expanded: bool,
    multiple: bool,
    checkable: bool,
    show_icon: bool,
    show_line: bool,
    switcher_icon: Option<Element>,
    on_select: EventHandler<(String, TreeNodeData)>,
    on_expand: EventHandler<String>,
) -> Element {
    let has_children = node.children.as_ref().map_or(false, |c| !c.is_empty());
    let node_key = node.key.as_ref().unwrap_or(&node.value).clone();
    let node_clone = node.clone();

    let handle_title_click = move |_evt: MouseEvent| {
        if !node_clone.disabled.unwrap_or(false) {
            on_select.call((node_clone.value.clone(), node_clone.clone()));
        }
    };

    let handle_switcher_click = move |_evt: MouseEvent| {
        if has_children {
            on_expand.call(node_key.clone());
        }
    };

    let node_class = format!(
        "ant-tree-select-tree-treenode {} {} {}",
        if selected {
            "ant-tree-select-tree-treenode-selected"
        } else {
            ""
        },
        if node.disabled.unwrap_or(false) {
            "ant-tree-select-tree-treenode-disabled"
        } else {
            ""
        },
        if has_children && expanded {
            "ant-tree-select-tree-treenode-open"
        } else {
            ""
        }
    )
    .trim()
    .to_string();

    rsx! {
        div {
            class: "{node_class}",

            // Node content
            div {
                class: "ant-tree-select-tree-node-content-wrapper",
                onclick: handle_title_click,

                // Switcher
                if has_children {
                    span {
                        class: "ant-tree-select-tree-switcher",
                        onclick: handle_switcher_click,
                        if let Some(ref icon) = switcher_icon {
                            {icon}
                        } else {
                            if expanded { "▼" } else { "▶" }
                        }
                    }
                } else {
                    span {
                        class: "ant-tree-select-tree-switcher ant-tree-select-tree-switcher-noop",
                    }
                }

                // Checkbox
                if checkable {
                    span {
                        class: "ant-tree-select-tree-checkbox",
                        input {
                            r#type: "checkbox",
                            checked: selected,
                            disabled: node.disable_checkbox.unwrap_or(false),
                        }
                    }
                }

                // Icon
                if show_icon {
                    span {
                        class: "ant-tree-select-tree-iconEle",
                        if let Some(icon) = node.icon {
                            {icon}
                        } else {
                            "📁"
                        }
                    }
                }

                // Title
                span {
                    class: "ant-tree-select-tree-title",
                    "{node.title}"
                }
            }

            // Children
            if has_children && expanded {
                div {
                    class: "ant-tree-select-tree-child-tree",
                    for child in node.children.as_ref().unwrap() {
                        TreeSelectNode {
                            key: "{child.value}",
                            node: child.clone(),
                            selected: false, // TODO: Check if child is selected
                            expanded: false, // TODO: Check if child is expanded
                            multiple,
                            checkable,
                            show_icon,
                            show_line,
                            switcher_icon: switcher_icon.clone(),
                            on_select,
                            on_expand,
                        }
                    }
                }
            }
        }
    }
}

/// Helper function to filter tree data
fn filter_tree_data(
    tree_data: &[TreeNodeData],
    search_value: &str,
    filter_fn: &Option<fn(&str, &TreeNodeData) -> bool>,
) -> Vec<TreeNodeData> {
    let mut filtered = Vec::new();

    for node in tree_data {
        let matches = if let Some(filter_fn) = filter_fn {
            filter_fn(search_value, node)
        } else {
            node.title
                .to_lowercase()
                .contains(&search_value.to_lowercase())
        };

        if matches {
            filtered.push(node.clone());
        } else if let Some(children) = &node.children {
            let filtered_children = filter_tree_data(children, search_value, filter_fn);
            if !filtered_children.is_empty() {
                let mut node_with_filtered_children = node.clone();
                node_with_filtered_children.children = Some(filtered_children);
                filtered.push(node_with_filtered_children);
            }
        }
    }

    filtered
}

/// Helper function to find node by value
fn find_node_by_value<'a>(
    tree_data: &'a [TreeNodeData],
    value: &'a str,
) -> Option<&'a TreeNodeData> {
    for node in tree_data {
        if node.value == value {
            return Some(node);
        }
        if let Some(children) = &node.children {
            if let Some(found) = find_node_by_value(children, value) {
                return Some(found);
            }
        }
    }
    None
}

/// Helper function to truncate text
fn truncate_text(text: &str, max_length: Option<i32>) -> String {
    if let Some(max_len) = max_length {
        if text.len() > max_len as usize {
            format!("{}...", &text[..max_len as usize])
        } else {
            text.to_string()
        }
    } else {
        text.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_node_data_creation() {
        let node = TreeNodeData::new("value1", "Title 1")
            .with_key("key1")
            .disabled(true)
            .is_leaf(false);

        assert_eq!(node.value, "value1");
        assert_eq!(node.title, "Title 1");
        assert_eq!(node.key, Some("key1".to_string()));
        assert_eq!(node.disabled, Some(true));
        assert_eq!(node.is_leaf, Some(false));
    }

    #[test]
    fn test_tree_select_size_as_str() {
        assert_eq!(TreeSelectSize::Small.as_str(), "small");
        assert_eq!(TreeSelectSize::Middle.as_str(), "middle");
        assert_eq!(TreeSelectSize::Large.as_str(), "large");
    }

    #[test]
    fn test_show_checked_strategy_as_str() {
        assert_eq!(ShowCheckedStrategy::ShowAll.as_str(), "SHOW_ALL");
        assert_eq!(ShowCheckedStrategy::ShowParent.as_str(), "SHOW_PARENT");
        assert_eq!(ShowCheckedStrategy::ShowChild.as_str(), "SHOW_CHILD");
    }

    #[test]
    fn test_find_node_by_value() {
        let tree_data = vec![
            TreeNodeData::new("1", "Node 1"),
            TreeNodeData::new("2", "Node 2").with_children(vec![
                TreeNodeData::new("2-1", "Child 1"),
                TreeNodeData::new("2-2", "Child 2"),
            ]),
        ];

        assert!(find_node_by_value(&tree_data, "1").is_some());
        assert!(find_node_by_value(&tree_data, "2-1").is_some());
        assert!(find_node_by_value(&tree_data, "3").is_none());
    }

    #[test]
    fn test_truncate_text() {
        assert_eq!(truncate_text("Hello World", Some(5)), "Hello...");
        assert_eq!(truncate_text("Hi", Some(5)), "Hi");
        assert_eq!(truncate_text("Hello World", None), "Hello World");
    }
}

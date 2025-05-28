//! Form 表单组件
//!
//! 具有数据收集、校验和提交功能的表单，包含复选框、单选框、输入框、下拉选择框等元素。
//!
//! ## 何时使用
//!
//! - 用于创建一个实体或收集信息。
//! - 需要对输入的数据类型进行校验时。

use crate::utils::class_names::conditional_class_names_array;
use dioxus::prelude::*;
use std::collections::HashMap;
use std::rc::Rc;

const FORM_STYLE: &str = include_str!("./style.css");

/// 表单布局类型
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FormLayout {
    /// 水平布局
    Horizontal,
    /// 垂直布局
    Vertical,
    /// 内联布局
    Inline,
}

impl Default for FormLayout {
    fn default() -> Self {
        FormLayout::Horizontal
    }
}

/// 表单尺寸
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FormSize {
    /// 小尺寸
    Small,
    /// 中等尺寸
    Middle,
    /// 大尺寸
    Large,
}

impl Default for FormSize {
    fn default() -> Self {
        FormSize::Middle
    }
}

/// 标签对齐方式
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LabelAlign {
    /// 左对齐
    Left,
    /// 右对齐
    Right,
}

impl Default for LabelAlign {
    fn default() -> Self {
        LabelAlign::Right
    }
}

/// 校验状态
#[derive(Clone, PartialEq)]
pub enum ValidateStatus {
    /// 成功
    Success,
    /// 警告
    Warning,
    /// 错误
    Error,
    /// 校验中
    Validating,
}

/// 校验规则
#[derive(Clone)]
pub struct FormRule {
    /// 是否必填
    pub required: bool,
    /// 错误信息
    pub message: String,
    /// 最小长度
    pub min: Option<usize>,
    /// 最大长度
    pub max: Option<usize>,
    /// 正则表达式
    pub pattern: Option<String>,
    /// 自定义校验函数
    pub validator: Option<Rc<dyn Fn(&str) -> Result<(), String>>>,
}

impl FormRule {
    pub fn required(message: &str) -> Self {
        Self {
            required: true,
            message: message.to_string(),
            min: None,
            max: None,
            pattern: None,
            validator: None,
        }
    }

    pub fn min_length(min: usize, message: &str) -> Self {
        Self {
            required: false,
            message: message.to_string(),
            min: Some(min),
            max: None,
            pattern: None,
            validator: None,
        }
    }

    pub fn max_length(max: usize, message: &str) -> Self {
        Self {
            required: false,
            message: message.to_string(),
            min: None,
            max: Some(max),
            pattern: None,
            validator: None,
        }
    }

    pub fn pattern(pattern: &str, message: &str) -> Self {
        Self {
            required: false,
            message: message.to_string(),
            min: None,
            max: None,
            pattern: Some(pattern.to_string()),
            validator: None,
        }
    }

    pub fn custom<F>(validator: F, message: &str) -> Self
    where
        F: Fn(&str) -> Result<(), String> + 'static,
    {
        Self {
            required: false,
            message: message.to_string(),
            min: None,
            max: None,
            pattern: None,
            validator: Some(Rc::new(validator)),
        }
    }
}

impl PartialEq for FormRule {
    fn eq(&self, other: &Self) -> bool {
        self.required == other.required
            && self.message == other.message
            && self.min == other.min
            && self.max == other.max
            && self.pattern == other.pattern
    }
}

/// 表单字段信息
#[derive(Clone, PartialEq)]
pub struct FormField {
    /// 字段名
    pub name: String,
    /// 字段值
    pub value: String,
    /// 校验规则
    pub rules: Vec<FormRule>,
    /// 校验状态
    pub status: Option<ValidateStatus>,
    /// 错误信息
    pub error: Option<String>,
    /// 是否已校验
    pub validated: bool,
}

impl FormField {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            value: String::new(),
            rules: Vec::new(),
            status: None,
            error: None,
            validated: false,
        }
    }

    /// 校验字段
    pub fn validate(&mut self) -> bool {
        self.validated = true;
        self.error = None;
        self.status = None;

        for rule in &self.rules {
            // 必填校验
            if rule.required && self.value.trim().is_empty() {
                self.error = Some(rule.message.clone());
                self.status = Some(ValidateStatus::Error);
                return false;
            }

            // 长度校验
            if let Some(min) = rule.min {
                if self.value.len() < min {
                    self.error = Some(rule.message.clone());
                    self.status = Some(ValidateStatus::Error);
                    return false;
                }
            }

            if let Some(max) = rule.max {
                if self.value.len() > max {
                    self.error = Some(rule.message.clone());
                    self.status = Some(ValidateStatus::Error);
                    return false;
                }
            }

            // 正则校验
            if let Some(pattern) = &rule.pattern {
                // 简单的正则校验实现
                if !self.value.is_empty() {
                    // 这里可以使用 regex crate 进行更复杂的正则校验
                    // 暂时使用简单的字符串匹配
                    if pattern == "email" && !self.value.contains('@') {
                        self.error = Some(rule.message.clone());
                        self.status = Some(ValidateStatus::Error);
                        return false;
                    }
                }
            }

            // 自定义校验
            if let Some(validator) = &rule.validator {
                if let Err(err) = validator(&self.value) {
                    self.error = Some(err);
                    self.status = Some(ValidateStatus::Error);
                    return false;
                }
            }
        }

        self.status = Some(ValidateStatus::Success);
        true
    }
}

/// Form 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct FormProps {
    /// 表单子元素
    pub children: Element,
    /// 表单布局
    #[props(default = FormLayout::Horizontal)]
    pub layout: FormLayout,
    /// 表单尺寸
    #[props(default = FormSize::Middle)]
    pub size: FormSize,
    /// 标签对齐方式
    #[props(default = LabelAlign::Right)]
    pub label_align: LabelAlign,
    /// 标签宽度
    pub label_col: Option<usize>,
    /// 控件宽度
    pub wrapper_col: Option<usize>,
    /// 是否禁用表单
    #[props(default = false)]
    pub disabled: bool,
    /// 表单提交事件
    pub on_finish: Option<EventHandler<HashMap<String, String>>>,
    /// 表单提交失败事件
    pub on_finish_failed: Option<EventHandler<HashMap<String, String>>>,
    /// 字段值变化事件
    pub on_values_change: Option<EventHandler<(String, String)>>,
    /// 自定义样式类名
    #[props(default = String::new())]
    pub class: String,
    /// 自定义样式
    #[props(default = String::new())]
    pub style: String,
}

/// Form 表单组件
#[component]
pub fn Form(props: FormProps) -> Element {
    let mut form_fields = use_signal(|| HashMap::<String, FormField>::new());

    // 提供表单上下文
    use_context_provider(|| FormContext {
        layout: props.layout,
        size: props.size,
        label_align: props.label_align,
        label_col: props.label_col,
        wrapper_col: props.wrapper_col,
        disabled: props.disabled,
        fields: form_fields.clone(),
        on_field_change: Rc::new(move |name: String, value: String| {
            let mut form_fields = form_fields.clone();
            form_fields.with_mut(|fields| {
                if let Some(field) = fields.get_mut(&name) {
                    field.value = value.clone();
                    field.validate();
                } else {
                    let mut field = FormField::new(&name);
                    field.value = value.clone();
                    fields.insert(name.clone(), field);
                }
            });

            if let Some(callback) = &props.on_values_change {
                callback.call((name, value));
            }
        }),
        on_field_register: Rc::new(move |name: String, rules: Vec<FormRule>| {
            let mut form_fields = form_fields.clone();
            form_fields.with_mut(|fields| {
                if let Some(field) = fields.get_mut(&name) {
                    field.rules = rules;
                } else {
                    let mut field = FormField::new(&name);
                    field.rules = rules;
                    fields.insert(name, field);
                }
            });
        }),
    });

    // 处理表单提交
    let handle_submit = move |evt: FormEvent| {
        evt.prevent_default();

        let mut all_valid = true;
        let mut form_data = HashMap::new();

        form_fields.with_mut(|fields| {
            for (name, field) in fields.iter_mut() {
                if !field.validate() {
                    all_valid = false;
                }
                form_data.insert(name.clone(), field.value.clone());
            }
        });

        if all_valid {
            if let Some(callback) = &props.on_finish {
                callback.call(form_data);
            }
        } else {
            if let Some(callback) = &props.on_finish_failed {
                callback.call(form_data);
            }
        }
    };

    let form_class = conditional_class_names_array(&[
        ("ant-form", true),
        (
            "ant-form-horizontal",
            props.layout == FormLayout::Horizontal,
        ),
        ("ant-form-vertical", props.layout == FormLayout::Vertical),
        ("ant-form-inline", props.layout == FormLayout::Inline),
        ("ant-form-small", props.size == FormSize::Small),
        ("ant-form-large", props.size == FormSize::Large),
        ("ant-form-disabled", props.disabled),
        (&props.class, !props.class.is_empty()),
    ]);

    rsx! {
        style { {FORM_STYLE} }

        form {
            class: "{form_class}",
            style: "{props.style}",
            onsubmit: handle_submit,

            {props.children}
        }
    }
}

/// FormItem 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct FormItemProps {
    /// 表单项子元素
    pub children: Element,
    /// 字段名
    pub name: Option<String>,
    /// 标签文本
    pub label: Option<String>,
    /// 校验规则
    #[props(default = Vec::new())]
    pub rules: Vec<FormRule>,
    /// 是否必填
    #[props(default = false)]
    pub required: bool,
    /// 额外的提示信息
    pub extra: Option<String>,
    /// 配合 label 属性使用，表示是否显示 label 后面的冒号
    #[props(default = true)]
    pub colon: bool,
    /// 标签宽度
    pub label_col: Option<usize>,
    /// 控件宽度
    pub wrapper_col: Option<usize>,
    /// 自定义样式类名
    #[props(default = String::new())]
    pub class: String,
    /// 自定义样式
    #[props(default = String::new())]
    pub style: String,
}

/// FormItem 表单项组件
#[component]
pub fn FormItem(props: FormItemProps) -> Element {
    let form_context = use_context::<FormContext>();
    let mut field_status = use_signal(|| None::<ValidateStatus>);
    let mut field_error = use_signal(|| None::<String>);

    // 注册字段
    let name_clone = props.name.clone();
    use_effect(move || {
        if let Some(name) = &name_clone {
            let mut rules = props.rules.clone();
            if props.required && !rules.iter().any(|r| r.required) {
                rules.push(FormRule::required("此字段为必填项"));
            }
            (form_context.on_field_register)(name.clone(), rules);
        }
    });

    // 监听字段状态变化
    let name_clone2 = props.name.clone();
    use_effect(move || {
        if let Some(name) = &name_clone2 {
            let fields = form_context.fields.read();
            if let Some(field) = fields.get(name) {
                field_status.set(field.status.clone());
                field_error.set(field.error.clone());
            }
        }
    });

    let label_col = props.label_col.or(form_context.label_col).unwrap_or(6);
    let wrapper_col = props.wrapper_col.or(form_context.wrapper_col).unwrap_or(18);

    let item_class = conditional_class_names_array(&[
        ("ant-form-item", true),
        ("ant-form-item-required", props.required),
        (
            "ant-form-item-has-success",
            field_status.read().as_ref() == Some(&ValidateStatus::Success),
        ),
        (
            "ant-form-item-has-warning",
            field_status.read().as_ref() == Some(&ValidateStatus::Warning),
        ),
        (
            "ant-form-item-has-error",
            field_status.read().as_ref() == Some(&ValidateStatus::Error),
        ),
        (
            "ant-form-item-is-validating",
            field_status.read().as_ref() == Some(&ValidateStatus::Validating),
        ),
        (&props.class, !props.class.is_empty()),
    ]);

    rsx! {
        div {
            class: "{item_class}",
            style: "{props.style}",

            if form_context.layout == FormLayout::Horizontal {
                div { class: "ant-row ant-form-item-row",
                    // 标签列
                    if let Some(label) = &props.label {
                        div {
                            class: "ant-col ant-form-item-label",
                            style: "flex: 0 0 {label_col * 100 / 24}%;",

                            label {
                                class: conditional_class_names_array(&[
                                    ("ant-form-item-required", props.required),
                                    ("ant-form-item-no-colon", !props.colon),
                                ]),
                                title: "{label}",
                                "{label}"
                                if props.colon { ":" }
                            }
                        }
                    }

                    // 控件列
                    div {
                        class: "ant-col ant-form-item-control",
                        style: "flex: 0 0 {wrapper_col * 100 / 24}%;",

                        div { class: "ant-form-item-control-input",
                            {props.children}
                        }

                        // 错误信息
                        if let Some(error) = field_error.read().as_ref() {
                            div { class: "ant-form-item-explain ant-form-item-explain-error",
                                "{error}"
                            }
                        }

                        // 额外信息
                        if let Some(extra) = &props.extra {
                            div { class: "ant-form-item-extra", "{extra}" }
                        }
                    }
                }
            } else {
                // 垂直或内联布局
                if let Some(label) = &props.label {
                    div { class: "ant-form-item-label",
                        label {
                            class: conditional_class_names_array(&[
                                ("ant-form-item-required", props.required),
                                ("ant-form-item-no-colon", !props.colon),
                            ]),
                            title: "{label}",

                            "{label}"
                            if props.colon { ":" }
                        }
                    }
                }

                div { class: "ant-form-item-control",
                    div { class: "ant-form-item-control-input",
                        {props.children}
                    }

                    // 错误信息
                    if let Some(error) = field_error.read().as_ref() {
                        div { class: "ant-form-item-explain ant-form-item-explain-error",
                            "{error}"
                        }
                    }

                    // 额外信息
                    if let Some(extra) = &props.extra {
                        div { class: "ant-form-item-extra", "{extra}" }
                    }
                }
            }
        }
    }
}

/// 表单上下文
#[derive(Clone)]
pub struct FormContext {
    pub layout: FormLayout,
    pub size: FormSize,
    pub label_align: LabelAlign,
    pub label_col: Option<usize>,
    pub wrapper_col: Option<usize>,
    pub disabled: bool,
    pub fields: Signal<HashMap<String, FormField>>,
    pub on_field_change: Rc<dyn Fn(String, String)>,
    pub on_field_register: Rc<dyn Fn(String, Vec<FormRule>)>,
}

impl PartialEq for FormContext {
    fn eq(&self, other: &Self) -> bool {
        self.layout == other.layout
            && self.size == other.size
            && self.label_align == other.label_align
            && self.label_col == other.label_col
            && self.wrapper_col == other.wrapper_col
            && self.disabled == other.disabled
    }
}

/// 便捷的表单规则创建宏
#[macro_export]
macro_rules! form_rules {
    [required($msg:expr)] => {
        vec![FormRule::required($msg)]
    };
    [required($msg:expr), $($rule:expr),*] => {
        {
            let mut rules = vec![FormRule::required($msg)];
            rules.extend(vec![$($rule),*]);
            rules
        }
    };
    [$($rule:expr),*] => {
        vec![$($rule),*]
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_form_layout_default() {
        assert_eq!(FormLayout::default(), FormLayout::Horizontal);
    }

    #[test]
    fn test_form_size_default() {
        assert_eq!(FormSize::default(), FormSize::Middle);
    }

    #[test]
    fn test_label_align_default() {
        assert_eq!(LabelAlign::default(), LabelAlign::Right);
    }

    #[test]
    fn test_form_field_new() {
        let field = FormField::new("username");
        assert_eq!(field.name, "username");
        assert_eq!(field.value, "");
        assert!(field.rules.is_empty());
        assert!(field.status.is_none());
        assert!(field.error.is_none());
        assert!(!field.validated);
    }

    #[test]
    fn test_form_field_validate_required() {
        let mut field = FormField::new("username");
        field.rules.push(FormRule::required("用户名不能为空"));

        // 空值校验失败
        assert!(!field.validate());
        assert_eq!(field.error, Some("用户名不能为空".to_string()));
        // assert_eq!(field.status, Some(ValidateStatus::Error));

        // 有值校验成功
        field.value = "test".to_string();
        assert!(field.validate());
        assert!(field.error.is_none());
        // assert_eq!(field.status, Some(ValidateStatus::Success));
    }

    #[test]
    fn test_form_rule_creation() {
        let rule = FormRule::required("必填");
        assert!(rule.required);
        assert_eq!(rule.message, "必填");

        let rule = FormRule::min_length(3, "最少3个字符");
        assert!(!rule.required);
        assert_eq!(rule.min, Some(3));
        assert_eq!(rule.message, "最少3个字符");

        let rule = FormRule::max_length(10, "最多10个字符");
        assert_eq!(rule.max, Some(10));
        assert_eq!(rule.message, "最多10个字符");

        let rule = FormRule::pattern("email", "邮箱格式不正确");
        assert_eq!(rule.pattern, Some("email".to_string()));
        assert_eq!(rule.message, "邮箱格式不正确");
    }
}

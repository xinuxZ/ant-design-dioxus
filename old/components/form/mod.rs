//! Form 表单组件
//!
//! 具有数据收集、校验和提交功能的表单，包含复选框、单选框、输入框、下拉选择框等元素。
//!
//! ## 何时使用
//!
//! - 用于创建一个实体或收集信息。
//! - 需要对输入的数据类型进行校验时。

mod styles;

use crate::utils::class_names::conditional_class_names_array;
use dioxus::prelude::*;
use std::collections::HashMap;
use std::rc::Rc;

use self::styles::{
    generate_form_item_style, generate_form_style, FormLayout as StyleFormLayout,
    FormSize as StyleFormSize, LabelAlign as StyleLabelAlign,
    ValidateStatus as StyleValidateStatus,
};

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

impl From<FormLayout> for StyleFormLayout {
    fn from(layout: FormLayout) -> Self {
        match layout {
            FormLayout::Horizontal => StyleFormLayout::Horizontal,
            FormLayout::Vertical => StyleFormLayout::Vertical,
            FormLayout::Inline => StyleFormLayout::Inline,
        }
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

impl From<FormSize> for StyleFormSize {
    fn from(size: FormSize) -> Self {
        match size {
            FormSize::Small => StyleFormSize::Small,
            FormSize::Middle => StyleFormSize::Middle,
            FormSize::Large => StyleFormSize::Large,
        }
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

impl From<LabelAlign> for StyleLabelAlign {
    fn from(align: LabelAlign) -> Self {
        match align {
            LabelAlign::Left => StyleLabelAlign::Left,
            LabelAlign::Right => StyleLabelAlign::Right,
        }
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

impl From<ValidateStatus> for StyleValidateStatus {
    fn from(status: ValidateStatus) -> Self {
        match status {
            ValidateStatus::Success => StyleValidateStatus::Success,
            ValidateStatus::Warning => StyleValidateStatus::Warning,
            ValidateStatus::Error => StyleValidateStatus::Error,
            ValidateStatus::Validating => StyleValidateStatus::Validating,
        }
    }
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

            // 最小长度校验
            if let Some(min) = rule.min {
                if self.value.len() < min {
                    self.error = Some(rule.message.clone());
                    self.status = Some(ValidateStatus::Error);
                    return false;
                }
            }

            // 最大长度校验
            if let Some(max) = rule.max {
                if self.value.len() > max {
                    self.error = Some(rule.message.clone());
                    self.status = Some(ValidateStatus::Error);
                    return false;
                }
            }

            // 正则校验
            if let Some(pattern) = &rule.pattern {
                if let Ok(re) = regex::Regex::new(pattern) {
                    if !re.is_match(&self.value) {
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

        // 校验通过
        if !self.rules.is_empty() {
            self.status = Some(ValidateStatus::Success);
        }

        true
    }
}

/// Form 表单组件属性
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

/// 表单上下文
static FORM_CONTEXT: GlobalSignal<Option<FormContext>> = Signal::global(|| None::<FormContext>);

/// Form 表单组件
#[component]
pub fn Form(props: FormProps) -> Element {
    let fields = use_signal(HashMap::<String, FormField>::new);

    // 字段值变化处理
    let on_field_change = Rc::new(move |name: String, value: String| {
        fields.write().entry(name.clone()).and_modify(|field| {
            field.value = value.clone();
            field.validate();
        });

        if let Some(callback) = &props.on_values_change {
            callback.call((name, value));
        }
    });

    // 字段注册处理
    let on_field_register = Rc::new(move |name: String, rules: Vec<FormRule>| {
        fields.write().entry(name).or_insert_with(|| {
            let mut field = FormField::new(&name);
            field.rules = rules;
            field
        });
    });

    // 创建表单上下文
    let context = FormContext {
        layout: props.layout,
        size: props.size,
        label_align: props.label_align,
        label_col: props.label_col,
        wrapper_col: props.wrapper_col,
        disabled: props.disabled,
        fields: fields.clone(),
        on_field_change,
        on_field_register,
    };

    // 设置表单上下文
    FORM_CONTEXT.write().replace(context);

    // 生成表单样式
    let form_style = generate_form_style(
        props.layout.into(),
        props.size.into(),
        props.label_align.into(),
        props.disabled,
    );

    // 表单提交处理
    let handle_submit = move |event: FormEvent| {
        event.prevent_default();

        let mut values = HashMap::new();
        let mut is_valid = true;

        // 校验所有字段
        for (name, field) in fields.read().iter() {
            values.insert(name.clone(), field.value.clone());
        }

        for (_, field) in fields.write().iter_mut() {
            if !field.validate() {
                is_valid = false;
            }
        }

        // 触发回调
        if is_valid {
            if let Some(callback) = &props.on_finish {
                callback.call(values);
            }
        } else if let Some(callback) = &props.on_finish_failed {
            callback.call(values);
        }
    };

    // 表单类名
    let class_name = conditional_class_names_array(&[
        ("ant-form", true),
        (
            match props.layout {
                FormLayout::Horizontal => "ant-form-horizontal",
                FormLayout::Vertical => "ant-form-vertical",
                FormLayout::Inline => "ant-form-inline",
            },
            true,
        ),
        (
            match props.size {
                FormSize::Small => "ant-form-small",
                FormSize::Middle => "",
                FormSize::Large => "ant-form-large",
            },
            props.size != FormSize::Middle,
        ),
        ("ant-form-disabled", props.disabled),
        (&props.class, !props.class.is_empty()),
    ]);

    rsx! {
        style { {form_style} }
        form {
            class: class_name,
            style: props.style.clone(),
            onsubmit: handle_submit,
            {props.children}
        }
    }
}

/// FormItem 表单项属性
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
    // 获取表单上下文
    let form_ctx = FORM_CONTEXT.read();
    let form_context = form_ctx
        .as_ref()
        .expect("FormItem must be used within Form");

    // 字段名
    let field_name = props.name.clone();

    // 合并规则
    let mut rules = props.rules.clone();
    if props.required {
        rules.push(FormRule::required("This field is required"));
    }

    // 注册字段
    if let Some(name) = &field_name {
        (form_context.on_field_register)(name.clone(), rules.clone());
    }

    // 获取字段状态
    let field_status = field_name.as_ref().and_then(|name| {
        form_context
            .fields
            .read()
            .get(name)
            .and_then(|field| field.status.clone())
    });

    let field_error = field_name.as_ref().and_then(|name| {
        form_context
            .fields
            .read()
            .get(name)
            .and_then(|field| field.error.clone())
    });

    // 生成表单项样式
    let form_item_style = generate_form_item_style(
        field_status.clone().map(|s| s.into()),
        props.required,
        props.colon,
    );

    // 值变化处理
    let handle_value_change = move |value: String| {
        if let Some(name) = &field_name {
            (form_context.on_field_change)(name.clone(), value);
        }
    };

    // 标签宽度样式
    let label_style = if let Some(col) = props.label_col.or(form_context.label_col) {
        format!("width: {}%;", col)
    } else {
        String::new()
    };

    // 控件宽度样式
    let wrapper_style = if let Some(col) = props.wrapper_col.or(form_context.wrapper_col) {
        format!("width: {}%;", col)
    } else {
        String::new()
    };

    // 类名
    let class_name = conditional_class_names_array(&[
        ("ant-form-item", true),
        (
            match &field_status {
                Some(ValidateStatus::Success) => "ant-form-item-has-success",
                Some(ValidateStatus::Warning) => "ant-form-item-has-warning",
                Some(ValidateStatus::Error) => "ant-form-item-has-error",
                Some(ValidateStatus::Validating) => "ant-form-item-is-validating",
                None => "",
            },
            field_status.is_some(),
        ),
        (
            "ant-form-item-with-help",
            field_error.is_some() || props.extra.is_some(),
        ),
        (&props.class, !props.class.is_empty()),
    ]);

    // 标签类名
    let label_class = conditional_class_names_array(&[
        ("ant-form-item-required", props.required),
        ("ant-form-item-no-colon", !props.colon),
    ]);

    // 克隆子元素并注入值变更处理
    let children = if field_name.is_some() {
        props.children.clone()
    } else {
        props.children.clone()
    };

    rsx! {
        style { {form_item_style} }
        div {
            class: class_name,
            style: props.style.clone(),

            div {
                class: "ant-form-item-row",

                // 标签
                if let Some(label) = &props.label {
                    div {
                        class: "ant-form-item-label",
                        style: label_style,

                        label {
                            class: label_class,
                            "{label}"
                        }
                    }
                }

                // 控件
                div {
                    class: "ant-form-item-control",
                    style: wrapper_style,

                    div {
                        class: "ant-form-item-control-input",

                        div {
                            class: "ant-form-item-control-input-content",
                            {children}
                        }
                    }

                    // 错误信息
                    if let Some(error) = &field_error {
                        div {
                            class: "ant-form-item-explain ant-form-item-explain-error",
                            "{error}"
                        }
                    }

                    // 额外信息
                    if let Some(extra) = &props.extra {
                        div {
                            class: "ant-form-item-extra",
                            "{extra}"
                        }
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

/// 获取表单上下文
pub fn use_form_context() -> FormContext {
    let context = FORM_CONTEXT.read();
    context
        .as_ref()
        .expect("useFormContext must be used within Form")
        .clone()
}

/// 表单钩子
pub fn use_form() -> FormContext {
    use_form_context()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_form_layout_default() {
        let layout = FormLayout::default();
        assert_eq!(layout, FormLayout::Horizontal);
    }

    #[test]
    fn test_form_size_default() {
        let size = FormSize::default();
        assert_eq!(size, FormSize::Middle);
    }

    #[test]
    fn test_label_align_default() {
        let align = LabelAlign::default();
        assert_eq!(align, LabelAlign::Right);
    }

    #[test]
    fn test_form_field_new() {
        let field = FormField::new("test");
        assert_eq!(field.name, "test");
        assert_eq!(field.value, "");
        assert!(field.rules.is_empty());
        assert!(field.status.is_none());
        assert!(field.error.is_none());
        assert!(!field.validated);
    }

    #[test]
    fn test_form_field_validate_required() {
        let mut field = FormField::new("test");
        field.rules.push(FormRule::required("Required"));

        // 空值校验失败
        assert!(!field.validate());
        assert_eq!(field.error, Some("Required".to_string()));
        assert!(matches!(field.status, Some(ValidateStatus::Error)));

        // 有值校验通过
        field.value = "value".to_string();
        assert!(field.validate());
        assert_eq!(field.error, None);
        assert!(matches!(field.status, Some(ValidateStatus::Success)));
    }

    #[test]
    fn test_form_rule_creation() {
        let rule1 = FormRule::required("Required");
        assert!(rule1.required);
        assert_eq!(rule1.message, "Required");

        let rule2 = FormRule::min_length(3, "Too short");
        assert!(!rule2.required);
        assert_eq!(rule2.min, Some(3));
        assert_eq!(rule2.message, "Too short");

        let rule3 = FormRule::max_length(10, "Too long");
        assert!(!rule3.required);
        assert_eq!(rule3.max, Some(10));
        assert_eq!(rule3.message, "Too long");

        let rule4 = FormRule::pattern(r"^\d+$", "Numbers only");
        assert!(!rule4.required);
        assert_eq!(rule4.pattern, Some(r"^\d+$".to_string()));
        assert_eq!(rule4.message, "Numbers only");
    }
}

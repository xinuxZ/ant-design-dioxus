//! 组件级别配置支持
//!
//! 为各个 Ant Design 组件提供统一的配置接口和类型定义

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 组件配置集合
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ComponentConfig {
    /// 按钮组件配置
    pub button: Option<ButtonConfig>,
    /// 输入框组件配置
    pub input: Option<InputConfig>,
    /// 表格组件配置
    pub table: Option<TableConfig>,
    /// 表单组件配置
    pub form: Option<FormConfig>,
    /// 模态框组件配置
    pub modal: Option<ModalConfig>,
    /// 消息组件配置
    pub message: Option<MessageConfig>,
    /// 通知组件配置
    pub notification: Option<NotificationConfig>,
    /// 选择器组件配置
    pub select: Option<SelectConfig>,
    /// 日期选择器组件配置
    pub date_picker: Option<DatePickerConfig>,
    /// 上传组件配置
    pub upload: Option<UploadConfig>,
}

/// 按钮组件配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ButtonConfig {
    /// 自动在中文字符间插入空格
    pub auto_insert_space: Option<bool>,
    /// 默认类型
    pub default_type: Option<ButtonType>,
    /// 默认尺寸
    pub default_size: Option<ButtonSize>,
    /// 默认形状
    pub default_shape: Option<ButtonShape>,
    /// 加载延迟时间（毫秒）
    pub loading_delay: Option<u32>,
    /// 自动获取焦点
    pub auto_focus: Option<bool>,
    /// 将按钮宽度调整为其父宽度
    pub block: Option<bool>,
    /// 设置危险按钮
    pub danger: Option<bool>,
    /// 按钮失效状态
    pub disabled: Option<bool>,
    /// 幽灵属性，使按钮背景透明
    pub ghost: Option<bool>,
    /// 点击跳转的地址
    pub href: Option<String>,
    /// 设置button原生的type值
    pub html_type: Option<String>,
    /// 设置按钮的图标组件
    pub icon: Option<String>,
    /// 设置按钮载入状态
    pub loading: Option<bool>,
    /// 设置按钮形状
    pub shape: Option<ButtonShape>,
    /// 相当于a链接的target属性
    pub target: Option<String>,
    /// 自定义类名
    pub class_name: Option<String>,
    /// 自定义样式
    pub style: Option<HashMap<String, String>>,
}

impl Default for ButtonConfig {
    fn default() -> Self {
        Self {
            auto_insert_space: Some(true),
            default_type: Some(ButtonType::Default),
            default_size: Some(ButtonSize::Middle),
            default_shape: Some(ButtonShape::Default),
            loading_delay: None,
            auto_focus: None,
            block: None,
            danger: None,
            disabled: None,
            ghost: None,
            href: None,
            html_type: None,
            icon: None,
            loading: None,
            shape: None,
            target: None,
            class_name: None,
            style: None,
        }
    }
}

/// 输入框组件配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputConfig {
    /// 默认尺寸
    pub default_size: Option<InputSize>,
    /// 默认变体
    pub default_variant: Option<InputVariant>,
    /// 自动完成
    pub auto_complete: Option<String>,
    /// 占位符文本
    pub placeholder: Option<String>,
    /// 可以点击清除图标删除内容
    pub allow_clear: Option<bool>,
    /// 是否有边框
    pub bordered: Option<bool>,
    /// 是否禁用
    pub disabled: Option<bool>,
    /// 最大长度
    pub max_length: Option<usize>,
    /// 是否展示字符数
    pub show_count: Option<bool>,
    /// 设置校验状态
    pub status: Option<String>,
    /// 带有前缀图标的input
    pub prefix: Option<String>,
    /// 带有后缀图标的input
    pub suffix: Option<String>,
    /// 带标签的input，设置前置标签
    pub addon_before: Option<String>,
    /// 带标签的input，设置后置标签
    pub addon_after: Option<String>,
    /// 自定义类名
    pub class_name: Option<String>,
    /// 自定义样式
    pub style: Option<HashMap<String, String>>,
}

impl Default for InputConfig {
    fn default() -> Self {
        Self {
            default_size: Some(InputSize::Middle),
            default_variant: Some(InputVariant::Outlined),
            auto_complete: None,
            placeholder: None,
            allow_clear: None,
            bordered: Some(true),
            disabled: None,
            max_length: None,
            show_count: None,
            status: None,
            prefix: None,
            suffix: None,
            addon_before: None,
            addon_after: None,
            class_name: None,
            style: None,
        }
    }
}

/// 表格组件配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TableConfig {
    /// 默认分页大小
    pub default_page_size: Option<usize>,
    /// 显示尺寸切换器
    pub show_size_changer: Option<bool>,
    /// 显示快速跳转
    pub show_quick_jumper: Option<bool>,
    /// 显示总数
    pub show_total: Option<bool>,
    /// 默认尺寸
    pub default_size: Option<TableSize>,
    /// 自定义类名
    pub class_name: Option<String>,
    /// 自定义样式
    pub style: Option<HashMap<String, String>>,
}

/// 表单组件配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FormConfig {
    /// 验证消息
    pub validate_messages: Option<HashMap<String, String>>,
    /// 必填标记
    pub required_mark: Option<bool>,
    /// 冒号显示
    pub colon: Option<bool>,
    /// 标签对齐方式
    pub label_align: Option<LabelAlign>,
    /// 标签宽度
    pub label_width: Option<String>,
    /// 布局方式
    pub layout: Option<FormLayout>,
    /// 自定义类名
    pub class_name: Option<String>,
    /// 自定义样式
    pub style: Option<HashMap<String, String>>,
}

/// 模态框组件配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModalConfig {
    /// 默认宽度
    pub default_width: Option<String>,
    /// 是否可拖拽
    pub draggable: Option<bool>,
    /// 是否居中
    pub centered: Option<bool>,
    /// 遮罩是否可点击关闭
    pub mask_closable: Option<bool>,
    /// 自定义类名
    pub class_name: Option<String>,
    /// 自定义样式
    pub style: Option<HashMap<String, String>>,
}

/// 消息组件配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageConfig {
    /// 默认持续时间（秒）
    pub default_duration: Option<f64>,
    /// 最大显示数量
    pub max_count: Option<usize>,
    /// 距离顶部距离
    pub top: Option<String>,
    /// 自定义类名
    pub class_name: Option<String>,
    /// 自定义样式
    pub style: Option<HashMap<String, String>>,
}

/// 通知组件配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotificationConfig {
    /// 默认持续时间（秒）
    pub default_duration: Option<f64>,
    /// 最大显示数量
    pub max_count: Option<usize>,
    /// 放置位置
    pub placement: Option<NotificationPlacement>,
    /// 自定义类名
    pub class_name: Option<String>,
    /// 自定义样式
    pub style: Option<HashMap<String, String>>,
}

/// 选择器组件配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SelectConfig {
    /// 默认尺寸
    pub default_size: Option<SelectSize>,
    /// 默认变体
    pub default_variant: Option<SelectVariant>,
    /// 弹出层匹配选择器宽度
    pub popup_match_select_width: Option<bool>,
    /// 自定义类名
    pub class_name: Option<String>,
    /// 自定义样式
    pub style: Option<HashMap<String, String>>,
}

/// 日期选择器组件配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DatePickerConfig {
    /// 默认尺寸
    pub default_size: Option<DatePickerSize>,
    /// 默认变体
    pub default_variant: Option<DatePickerVariant>,
    /// 默认格式
    pub default_format: Option<String>,
    /// 自定义类名
    pub class_name: Option<String>,
    /// 自定义样式
    pub style: Option<HashMap<String, String>>,
}

/// 上传组件配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UploadConfig {
    /// 默认上传类型
    pub default_type: Option<UploadType>,
    /// 最大文件大小（字节）
    pub max_file_size: Option<usize>,
    /// 允许的文件类型
    pub accepted_file_types: Option<Vec<String>>,
    /// 自定义类名
    pub class_name: Option<String>,
    /// 自定义样式
    pub style: Option<HashMap<String, String>>,
}

/// 空状态配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmptyConfig {
    /// 默认描述文本
    pub description: Option<String>,
    /// 默认图片
    pub image: Option<String>,
    /// 图片样式
    pub image_style: HashMap<String, String>,
}

impl Default for EmptyConfig {
    fn default() -> Self {
        Self {
            description: None,
            image: None,
            image_style: HashMap::new(),
        }
    }
}

// 枚举类型定义

/// 按钮类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ButtonType {
    Primary,
    Default,
    Dashed,
    Text,
    Link,
}

/// 按钮尺寸
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ButtonSize {
    Large,
    Middle,
    Small,
}

/// 按钮形状
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ButtonShape {
    Default,
    Circle,
    Round,
}

/// 按钮HTML类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ButtonHtmlType {
    Button,
    Submit,
    Reset,
}

/// 输入框尺寸
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InputSize {
    Large,
    Middle,
    Small,
}

/// 输入框变体
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InputVariant {
    Outlined,
    Filled,
    Borderless,
}

/// 表格尺寸
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TableSize {
    Large,
    Middle,
    Small,
}

/// 标签对齐方式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LabelAlign {
    Left,
    Right,
}

/// 表单布局
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FormLayout {
    Horizontal,
    Vertical,
    Inline,
}

/// 通知放置位置
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NotificationPlacement {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Top,
    Bottom,
}

/// 选择器尺寸
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SelectSize {
    Large,
    Middle,
    Small,
}

/// 选择器变体
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SelectVariant {
    Outlined,
    Filled,
    Borderless,
}

/// 日期选择器尺寸
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DatePickerSize {
    Large,
    Middle,
    Small,
}

/// 日期选择器变体
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DatePickerVariant {
    Outlined,
    Filled,
    Borderless,
}

/// 上传类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UploadType {
    Select,
    Drag,
}

/// 组件配置合并工具
impl ComponentConfig {
    /// 合并两个组件配置
    pub fn merge(base: Self, override_config: Self) -> Self {
        Self {
            button: override_config.button.or(base.button),
            input: override_config.input.or(base.input),
            table: override_config.table.or(base.table),
            form: override_config.form.or(base.form),
            modal: override_config.modal.or(base.modal),
            message: override_config.message.or(base.message),
            notification: override_config.notification.or(base.notification),
            select: override_config.select.or(base.select),
            date_picker: override_config.date_picker.or(base.date_picker),
            upload: override_config.upload.or(base.upload),
        }
    }

    /// 获取指定组件的配置
    pub fn get_button_config(&self) -> Option<&ButtonConfig> {
        self.button.as_ref()
    }

    pub fn get_input_config(&self) -> Option<&InputConfig> {
        self.input.as_ref()
    }

    pub fn get_table_config(&self) -> Option<&TableConfig> {
        self.table.as_ref()
    }

    pub fn get_form_config(&self) -> Option<&FormConfig> {
        self.form.as_ref()
    }

    pub fn get_modal_config(&self) -> Option<&ModalConfig> {
        self.modal.as_ref()
    }

    pub fn get_message_config(&self) -> Option<&MessageConfig> {
        self.message.as_ref()
    }

    pub fn get_notification_config(&self) -> Option<&NotificationConfig> {
        self.notification.as_ref()
    }

    pub fn get_select_config(&self) -> Option<&SelectConfig> {
        self.select.as_ref()
    }

    pub fn get_date_picker_config(&self) -> Option<&DatePickerConfig> {
        self.date_picker.as_ref()
    }

    pub fn get_upload_config(&self) -> Option<&UploadConfig> {
        self.upload.as_ref()
    }
}

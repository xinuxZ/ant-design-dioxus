//! Typography 组件类型定义

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

/// 文本类型枚举
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TextType {
    /// 默认文本
    Default,
    /// 次要文本
    Secondary,
    /// 成功状态文本
    Success,
    /// 警告状态文本
    Warning,
    /// 危险状态文本
    Danger,
    /// 禁用状态文本
    Disabled,
}

impl Default for TextType {
    fn default() -> Self {
        Self::Default
    }
}

/// 标题级别枚举
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HeadingLevel {
    /// H1 标题
    H1,
    /// H2 标题
    H2,
    /// H3 标题
    H3,
    /// H4 标题
    H4,
    /// H5 标题
    H5,
}

impl Default for HeadingLevel {
    fn default() -> Self {
        Self::H1
    }
}

impl HeadingLevel {
    /// 获取对应的 HTML 标签名
    pub fn tag_name(&self) -> &'static str {
        match self {
            HeadingLevel::H1 => "h1",
            HeadingLevel::H2 => "h2",
            HeadingLevel::H3 => "h3",
            HeadingLevel::H4 => "h4",
            HeadingLevel::H5 => "h5",
        }
    }

    /// 获取数字级别
    pub fn level(&self) -> u8 {
        match self {
            HeadingLevel::H1 => 1,
            HeadingLevel::H2 => 2,
            HeadingLevel::H3 => 3,
            HeadingLevel::H4 => 4,
            HeadingLevel::H5 => 5,
        }
    }
}

/// 链接类型枚举
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LinkType {
    /// 默认链接
    Default,
    /// 次要链接
    Secondary,
    /// 成功状态链接
    Success,
    /// 警告状态链接
    Warning,
    /// 危险状态链接
    Danger,
    /// 禁用状态链接
    Disabled,
}

impl Default for LinkType {
    fn default() -> Self {
        Self::Default
    }
}

/// 链接目标枚举
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LinkTarget {
    /// 当前窗口
    #[serde(rename = "_self")]
    Self_,
    /// 新窗口
    #[serde(rename = "_blank")]
    Blank,
    /// 父窗口
    #[serde(rename = "_parent")]
    Parent,
    /// 顶层窗口
    #[serde(rename = "_top")]
    Top,
}

impl Default for LinkTarget {
    fn default() -> Self {
        Self::Self_
    }
}

impl LinkTarget {
    /// 获取对应的字符串值
    pub fn as_str(&self) -> &'static str {
        match self {
            LinkTarget::Self_ => "_self",
            LinkTarget::Blank => "_blank",
            LinkTarget::Parent => "_parent",
            LinkTarget::Top => "_top",
        }
    }
}

/// 省略配置
#[derive(Debug, Clone, PartialEq)]
pub struct EllipsisConfig {
    /// 最大行数
    pub rows: Option<u32>,
    /// 是否可展开
    pub expandable: bool,
    /// 是否可收起（需要 expandable 为 true）
    pub collapsible: bool,
    /// 省略号后缀
    pub suffix: Option<String>,
    /// 自定义展开/收起符号
    pub symbol: Option<Element>,
    /// 工具提示
    pub tooltip: Option<String>,
    /// 默认是否展开
    pub default_expanded: bool,
    /// 受控展开状态
    pub expanded: Option<bool>,
    /// 展开/收起回调
    pub on_expand: Option<EventHandler<bool>>,
    /// 省略状态变化回调
    pub on_ellipsis: Option<EventHandler<bool>>,
}

impl Default for EllipsisConfig {
    fn default() -> Self {
        Self {
            rows: None,
            expandable: false,
            collapsible: false,
            suffix: None,
            symbol: None,
            tooltip: None,
            default_expanded: false,
            expanded: None,
            on_expand: None,
            on_ellipsis: None,
        }
    }
}

/// 复制配置
#[derive(Debug, Clone, PartialEq)]
pub struct CopyableConfig {
    /// 要复制的文本（如果不指定则使用组件内容）
    pub text: Option<String>,
    /// 复制成功回调
    pub on_copy: Option<Callback<String>>,
    /// 自定义复制图标
    pub icon: Option<Element>,
    /// 工具提示文本 [复制前, 复制后]
    pub tooltips: Option<(String, String)>,
    /// 复制格式
    pub format: CopyFormat,
    /// Tab 索引
    pub tab_index: i32,
}

impl Default for CopyableConfig {
    fn default() -> Self {
        Self {
            text: None,
            on_copy: None,
            icon: None,
            tooltips: Some(("复制".to_string(), "已复制".to_string())),
            format: CopyFormat::Text,
            tab_index: 0,
        }
    }
}

/// 复制格式
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CopyFormat {
    /// 纯文本
    Text,
    /// HTML 格式
    Html,
}

impl Default for CopyFormat {
    fn default() -> Self {
        Self::Text
    }
}

/// 编辑配置
#[derive(Debug, Clone, PartialEq)]
pub struct EditableConfig {
    /// 自定义编辑图标
    pub icon: Option<Element>,
    /// 工具提示文本
    pub tooltip: Option<String>,
    /// 是否处于编辑状态
    pub editing: bool,
    /// 最大长度
    pub max_length: Option<u32>,
    /// 自动调整大小
    pub auto_size: Option<AutoSizeConfig>,
    /// 编辑文本（如果不指定则使用组件内容）
    pub text: Option<String>,
    /// 文本变化回调
    pub on_change: Option<Callback<String>>,
    /// 取消编辑回调
    pub on_cancel: Option<Callback<()>>,
    /// 开始编辑回调
    pub on_start: Option<Callback<()>>,
    /// 结束编辑回调
    pub on_end: Option<Callback<String>>,
    /// 触发类型
    pub trigger_type: Vec<EditTriggerType>,
    /// 确认图标
    pub enter_icon: Option<Element>,
    /// Tab 索引
    pub tab_index: i32,
}

impl Default for EditableConfig {
    fn default() -> Self {
        Self {
            icon: None,
            tooltip: Some("编辑".to_string()),
            editing: false,
            max_length: None,
            auto_size: None,
            text: None,
            on_change: None,
            on_cancel: None,
            on_start: None,
            on_end: None,
            trigger_type: vec![EditTriggerType::Icon],
            enter_icon: None,
            tab_index: 0,
        }
    }
}

/// 自动调整大小配置
#[derive(Debug, Clone, PartialEq)]
pub struct AutoSizeConfig {
    /// 最小行数
    pub min_rows: Option<u32>,
    /// 最大行数
    pub max_rows: Option<u32>,
}

/// 编辑触发类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EditTriggerType {
    /// 图标触发
    Icon,
    /// 文本触发
    Text,
}

/// Typography 组件的通用属性
#[derive(Debug, Clone, PartialEq, Props)]
pub struct TypographyProps {
    /// 子元素
    pub children: Element,
    /// 自定义类名
    #[props(default)]
    pub class: Option<String>,
    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,
    /// 代码样式
    #[props(default = false)]
    pub code: bool,
    /// 是否可复制
    #[props(default)]
    pub copyable: Option<CopyableConfig>,
    /// 删除线样式
    #[props(default = false)]
    pub delete: bool,
    /// 禁用状态
    #[props(default = false)]
    pub disabled: bool,
    /// 是否可编辑
    #[props(default)]
    pub editable: Option<EditableConfig>,
    /// 省略配置
    #[props(default)]
    pub ellipsis: Option<EllipsisConfig>,
    /// 键盘样式
    #[props(default = false)]
    pub keyboard: bool,
    /// 标记样式
    #[props(default = false)]
    pub mark: bool,
    /// 点击事件
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    /// 加粗样式
    #[props(default = false)]
    pub strong: bool,
    /// 斜体样式
    #[props(default = false)]
    pub italic: bool,
    /// 文本类型
    #[props(default)]
    pub r#type: Option<TextType>,
    /// 下划线样式
    #[props(default = false)]
    pub underline: bool,
}

/// Title 组件属性
#[derive(Debug, Clone, PartialEq, Props)]
pub struct TitleProps {
    /// 子元素
    pub children: Element,
    /// 自定义类名
    #[props(default)]
    pub class: Option<String>,
    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,
    /// 标题级别
    #[props(default)]
    pub level: HeadingLevel,
    /// 代码样式
    #[props(default = false)]
    pub code: bool,
    /// 是否可复制
    #[props(default)]
    pub copyable: Option<CopyableConfig>,
    /// 删除线样式
    #[props(default = false)]
    pub delete: bool,
    /// 禁用状态
    #[props(default = false)]
    pub disabled: bool,
    /// 是否可编辑
    #[props(default)]
    pub editable: Option<EditableConfig>,
    /// 省略配置
    #[props(default)]
    pub ellipsis: Option<EllipsisConfig>,
    /// 键盘样式
    #[props(default = false)]
    pub keyboard: bool,
    /// 标记样式
    #[props(default = false)]
    pub mark: bool,
    /// 点击事件
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    /// 加粗样式
    #[props(default = false)]
    pub strong: bool,
    /// 斜体样式
    #[props(default = false)]
    pub italic: bool,
    /// 文本类型
    #[props(default)]
    pub r#type: Option<TextType>,
    /// 下划线样式
    #[props(default = false)]
    pub underline: bool,
}

/// Text 组件属性
#[derive(Debug, Clone, PartialEq, Props)]
pub struct TextProps {
    /// 子元素
    pub children: Element,
    /// 自定义类名
    #[props(default)]
    pub class: Option<String>,
    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,
    /// 代码样式
    #[props(default = false)]
    pub code: bool,
    /// 是否可复制
    #[props(default)]
    pub copyable: Option<CopyableConfig>,
    /// 删除线样式
    #[props(default = false)]
    pub delete: bool,
    /// 禁用状态
    #[props(default = false)]
    pub disabled: bool,
    /// 是否可编辑
    #[props(default)]
    pub editable: Option<EditableConfig>,
    /// 省略配置（Text 组件不支持 expandable、rows 和 onExpand）
    #[props(default)]
    pub ellipsis: Option<bool>,
    /// 键盘样式
    #[props(default = false)]
    pub keyboard: bool,
    /// 标记样式
    #[props(default = false)]
    pub mark: bool,
    /// 点击事件
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    /// 加粗样式
    #[props(default = false)]
    pub strong: bool,
    /// 斜体样式
    #[props(default = false)]
    pub italic: bool,
    /// 文本类型
    #[props(default)]
    pub r#type: Option<TextType>,
    /// 下划线样式
    #[props(default = false)]
    pub underline: bool,
}

/// Paragraph 组件属性
#[derive(Debug, Clone, PartialEq, Props)]
pub struct ParagraphProps {
    /// 子元素
    pub children: Element,
    /// 自定义类名
    #[props(default)]
    pub class: Option<String>,
    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,
    /// 代码样式
    #[props(default = false)]
    pub code: bool,
    /// 是否可复制
    #[props(default)]
    pub copyable: Option<CopyableConfig>,
    /// 删除线样式
    #[props(default = false)]
    pub delete: bool,
    /// 禁用状态
    #[props(default = false)]
    pub disabled: bool,
    /// 是否可编辑
    #[props(default)]
    pub editable: Option<EditableConfig>,
    /// 省略配置
    #[props(default)]
    pub ellipsis: Option<EllipsisConfig>,
    /// 标记样式
    #[props(default = false)]
    pub mark: bool,
    /// 点击事件
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    /// 加粗样式
    #[props(default = false)]
    pub strong: bool,
    /// 斜体样式
    #[props(default = false)]
    pub italic: bool,
    /// 文本类型
    #[props(default)]
    pub r#type: Option<TextType>,
    /// 下划线样式
    #[props(default = false)]
    pub underline: bool,
    #[props(default = false)]
    pub keyboard: bool,
}

/// Link 组件属性
#[derive(Debug, Clone, PartialEq, Props)]
pub struct LinkProps {
    /// 子元素
    pub children: Element,
    /// 自定义类名
    #[props(default)]
    pub class: Option<String>,
    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,
    /// 链接地址
    #[props(default)]
    pub href: Option<String>,
    /// 链接目标
    #[props(default)]
    pub target: Option<LinkTarget>,
    /// 链接关系
    #[props(default)]
    pub rel: Option<String>,
    /// 是否为块级链接
    #[props(default = false)]
    pub block: bool,
    /// 链接类型
    #[props(default)]
    pub r#type: Option<LinkType>,
    /// 禁用状态
    #[props(default = false)]
    pub disabled: bool,
    /// 点击事件
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    /// 下划线样式
    #[props(default = false)]
    pub underline: bool,
    /// 斜体样式
    #[props(default = false)]
    pub italic: bool,
    /// 删除线样式
    #[props(default = false)]
    pub delete: bool,
    /// 标记样式
    #[props(default = false)]
    pub mark: bool,
    /// 代码样式
    #[props(default = false)]
    pub code: bool,
    /// 键盘样式
    #[props(default = false)]
    pub keyboard: bool,
    /// 加粗样式
    #[props(default = false)]
    pub strong: bool,
    /// 省略配置
    #[props(default)]
    pub ellipsis: Option<bool>,
    /// 是否可复制
    #[props(default)]
    pub copyable: Option<CopyableConfig>,
    /// 元素 ID
    #[props(default)]
    pub id: Option<String>,
}

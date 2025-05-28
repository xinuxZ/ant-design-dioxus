//! Typography 排版组件
//!
//! 文本的基本格式。
//!
//! ## 何时使用
//!
//! - 当需要展示标题、段落、列表内容时。
//! - 当需要一列基于文本的基础操作时，如拷贝、省略、可编辑。
//!
//! ## 设计指引
//!
//! ### 标题
//! 标题具有五个不同的级别，h1、h2、h3、h4、h5。在大多数情况下，
//! 我们建议使用默认标题大小。在特殊情况下，可以选择其他大小。
//!
//! ### 正文
//! 正文文本的默认字体大小为 14px，行高为 1.5715。
//! 在大多数业务情况下，我们建议使用默认文本大小。

pub mod link;

// 重新导出所有组件
pub use link::*;

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

// 引入排版样式
const TYPOGRAPHY_STYLE: &str = include_str!("style.css");

/// 标题级别
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HeadingLevel {
    /// h1 标题
    H1,
    /// h2 标题
    H2,
    /// h3 标题
    H3,
    /// h4 标题
    H4,
    /// h5 标题
    H5,
}

impl Default for HeadingLevel {
    fn default() -> Self {
        Self::H1
    }
}

/// 文本类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TextType {
    /// 默认文本
    Default,
    /// 次要文本
    Secondary,
    /// 成功文本
    Success,
    /// 警告文本
    Warning,
    /// 危险文本
    Danger,
    /// 禁用文本
    Disabled,
}

impl Default for TextType {
    fn default() -> Self {
        Self::Default
    }
}

/// 标题属性
#[derive(Props, Clone, PartialEq)]
pub struct TitleProps {
    /// 标题级别
    #[props(default)]
    pub level: HeadingLevel,
    /// 是否可复制
    #[props(default)]
    pub copyable: bool,
    /// 是否可编辑
    #[props(default)]
    pub editable: bool,
    /// 是否省略
    #[props(default)]
    pub ellipsis: bool,
    /// 省略行数
    #[props(default)]
    pub ellipsis_rows: Option<u32>,
    /// 文本类型
    #[props(default)]
    pub text_type: TextType,
    /// 是否禁用
    #[props(default)]
    pub disabled: bool,
    /// 是否删除线
    #[props(default)]
    pub delete: bool,
    /// 是否下划线
    #[props(default)]
    pub underline: bool,
    /// 是否强调
    #[props(default)]
    pub strong: bool,
    /// 是否斜体
    #[props(default)]
    pub italic: bool,
    /// 是否标记
    #[props(default)]
    pub mark: bool,
    /// 是否代码
    #[props(default)]
    pub code: bool,
    /// 自定义类名
    #[props(default)]
    pub class: Option<String>,
    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,
    /// 点击事件处理器
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    /// 子元素
    children: Element,
}

/// 文本属性
#[derive(Props, Clone, PartialEq)]
pub struct TextProps {
    /// 是否可复制
    #[props(default)]
    pub copyable: bool,
    /// 是否可编辑
    #[props(default)]
    pub editable: bool,
    /// 是否省略
    #[props(default)]
    pub ellipsis: bool,
    /// 省略行数
    #[props(default)]
    pub ellipsis_rows: Option<u32>,
    /// 文本类型
    #[props(default)]
    pub text_type: TextType,
    /// 是否禁用
    #[props(default)]
    pub disabled: bool,
    /// 是否删除线
    #[props(default)]
    pub delete: bool,
    /// 是否下划线
    #[props(default)]
    pub underline: bool,
    /// 是否强调
    #[props(default)]
    pub strong: bool,
    /// 是否斜体
    #[props(default)]
    pub italic: bool,
    /// 是否标记
    #[props(default)]
    pub mark: bool,
    /// 是否代码
    #[props(default)]
    pub code: bool,
    /// 自定义类名
    #[props(default)]
    pub class: Option<String>,
    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,
    /// 点击事件处理器
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    /// 子元素
    children: Element,
}

/// 链接属性
#[derive(Props, Clone, PartialEq)]
pub struct LinkProps {
    /// 链接地址
    #[props(default)]
    pub href: Option<String>,
    /// 链接目标
    #[props(default)]
    pub target: Option<String>,
    /// 是否可复制
    #[props(default)]
    pub copyable: bool,
    /// 是否可编辑
    #[props(default)]
    pub editable: bool,
    /// 是否省略
    #[props(default)]
    pub ellipsis: bool,
    /// 省略行数
    #[props(default)]
    pub ellipsis_rows: Option<u32>,
    /// 文本类型
    #[props(default)]
    pub text_type: TextType,
    /// 是否禁用
    #[props(default)]
    pub disabled: bool,
    /// 是否删除线
    #[props(default)]
    pub delete: bool,
    /// 是否下划线
    #[props(default)]
    pub underline: bool,
    /// 是否强调
    #[props(default)]
    pub strong: bool,
    /// 是否斜体
    #[props(default)]
    pub italic: bool,
    /// 是否标记
    #[props(default)]
    pub mark: bool,
    /// 是否代码
    #[props(default)]
    pub code: bool,
    /// 自定义类名
    #[props(default)]
    pub class: Option<String>,
    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,
    /// 点击事件处理器
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    /// 子元素
    children: Element,
}

/// 段落属性
#[derive(Props, Clone, PartialEq)]
pub struct ParagraphProps {
    /// 是否可复制
    #[props(default)]
    pub copyable: bool,
    /// 是否可编辑
    #[props(default)]
    pub editable: bool,
    /// 是否省略
    #[props(default)]
    pub ellipsis: bool,
    /// 省略行数
    #[props(default)]
    pub ellipsis_rows: Option<u32>,
    /// 文本类型
    #[props(default)]
    pub text_type: TextType,
    /// 是否禁用
    #[props(default)]
    pub disabled: bool,
    /// 是否删除线
    #[props(default)]
    pub delete: bool,
    /// 是否下划线
    #[props(default)]
    pub underline: bool,
    /// 是否强调
    #[props(default)]
    pub strong: bool,
    /// 是否斜体
    #[props(default)]
    pub italic: bool,
    /// 是否标记
    #[props(default)]
    pub mark: bool,
    /// 是否代码
    #[props(default)]
    pub code: bool,
    /// 自定义类名
    #[props(default)]
    pub class: Option<String>,
    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,
    /// 点击事件处理器
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    /// 子元素
    children: Element,
}

/// 标题组件
///
/// # 参数
///
/// * `props` - 标题属性
///
/// # 示例
///
/// ```rust
/// use dioxus::prelude::*;
/// use ant_design_dioxus::{Title, HeadingLevel};
///
/// fn app() -> Element {
///     rsx! {
///         Title {
///             level: HeadingLevel::H1,
///             "这是一个标题"
///         }
///     }
/// }
/// ```
#[component]
pub fn Title(props: TitleProps) -> Element {
    let class_name = get_typography_class_name(&props.text_type, &props.class, true, &props);
    let typography_style = get_typography_style(&props.style, &props.ellipsis_rows);

    rsx! {
        // 注入排版样式
        style { {TYPOGRAPHY_STYLE} }

        {match props.level {
            HeadingLevel::H1 => rsx! {
                h1 {
                    class: class_name.clone(),
                    style: typography_style.clone(),
                    onclick: move |evt| {
                        if let Some(handler) = &props.onclick {
                            handler.call(evt);
                        }
                    },
                    {if props.strong {
                        rsx! { strong { {props.children} } }
                    } else if props.italic {
                        rsx! { em { {props.children} } }
                    } else if props.mark {
                        rsx! { mark { {props.children} } }
                    } else if props.code {
                        rsx! { code { {props.children} } }
                    } else if props.delete {
                        rsx! { del { {props.children} } }
                    } else if props.underline {
                        rsx! { u { {props.children} } }
                    } else {
                        props.children
                    }}
                }
            },
            HeadingLevel::H2 => rsx! {
                h2 {
                    class: class_name.clone(),
                    style: typography_style.clone(),
                    onclick: move |evt| {
                        if let Some(handler) = &props.onclick {
                            handler.call(evt);
                        }
                    },
                    {if props.strong {
                        rsx! { strong { {props.children} } }
                    } else if props.italic {
                        rsx! { em { {props.children} } }
                    } else if props.mark {
                        rsx! { mark { {props.children} } }
                    } else if props.code {
                        rsx! { code { {props.children} } }
                    } else if props.delete {
                        rsx! { del { {props.children} } }
                    } else if props.underline {
                        rsx! { u { {props.children} } }
                    } else {
                        props.children
                    }}
                }
            },
            HeadingLevel::H3 => rsx! {
                h3 {
                    class: class_name.clone(),
                    style: typography_style.clone(),
                    onclick: move |evt| {
                        if let Some(handler) = &props.onclick {
                            handler.call(evt);
                        }
                    },
                    {if props.strong {
                        rsx! { strong { {props.children} } }
                    } else if props.italic {
                        rsx! { em { {props.children} } }
                    } else if props.mark {
                        rsx! { mark { {props.children} } }
                    } else if props.code {
                        rsx! { code { {props.children} } }
                    } else if props.delete {
                        rsx! { del { {props.children} } }
                    } else if props.underline {
                        rsx! { u { {props.children} } }
                    } else {
                        props.children
                    }}
                }
            },
            HeadingLevel::H4 => rsx! {
                h4 {
                    class: class_name.clone(),
                    style: typography_style.clone(),
                    onclick: move |evt| {
                        if let Some(handler) = &props.onclick {
                            handler.call(evt);
                        }
                    },
                    {if props.strong {
                        rsx! { strong { {props.children} } }
                    } else if props.italic {
                        rsx! { em { {props.children} } }
                    } else if props.mark {
                        rsx! { mark { {props.children} } }
                    } else if props.code {
                        rsx! { code { {props.children} } }
                    } else if props.delete {
                        rsx! { del { {props.children} } }
                    } else if props.underline {
                        rsx! { u { {props.children} } }
                    } else {
                        props.children
                    }}
                }
            },
            HeadingLevel::H5 => rsx! {
                h5 {
                    class: class_name.clone(),
                    style: typography_style.clone(),
                    onclick: move |evt| {
                        if let Some(handler) = &props.onclick {
                            handler.call(evt);
                        }
                    },
                    {if props.strong {
                        rsx! { strong { {props.children} } }
                    } else if props.italic {
                        rsx! { em { {props.children} } }
                    } else if props.mark {
                        rsx! { mark { {props.children} } }
                    } else if props.code {
                        rsx! { code { {props.children} } }
                    } else if props.delete {
                        rsx! { del { {props.children} } }
                    } else if props.underline {
                        rsx! { u { {props.children} } }
                    } else {
                        props.children
                    }}
                }
            },
        }}
    }
}

/// 文本组件
///
/// # 参数
///
/// * `props` - 文本属性
///
/// # 示例
///
/// ```rust
/// use dioxus::prelude::*;
/// use ant_design_dioxus::{Text, TextType};
///
/// fn app() -> Element {
///     rsx! {
///         Text {
///             text_type: TextType::Success,
///             "这是成功文本"
///         }
///     }
/// }
/// ```
#[component]
pub fn Text(props: TextProps) -> Element {
    let class_name = get_typography_class_name(&props.text_type, &props.class, false, &props);
    let typography_style = get_typography_style(&props.style, &props.ellipsis_rows);

    rsx! {
        // 注入排版样式
        style { {TYPOGRAPHY_STYLE} }

        span {
            class: class_name.clone(),
            style: typography_style.clone(),
            onclick: move |evt| {
                if let Some(handler) = &props.onclick {
                    handler.call(evt);
                }
            },
            {if props.strong {
                rsx! { strong { {props.children} } }
            } else if props.italic {
                rsx! { em { {props.children} } }
            } else if props.mark {
                rsx! { mark { {props.children} } }
            } else if props.code {
                rsx! { code { {props.children} } }
            } else if props.delete {
                rsx! { del { {props.children} } }
            } else if props.underline {
                rsx! { u { {props.children} } }
            } else {
                props.children
            }}
        }
    }
}

/// 段落组件
///
/// # 参数
///
/// * `props` - 段落属性
///
/// # 示例
///
/// ```rust
/// use dioxus::prelude::*;
/// use ant_design_dioxus::Paragraph;
///
/// fn app() -> Element {
///     rsx! {
///         Paragraph {
///             ellipsis: true,
///             ellipsis_rows: Some(3),
///             "这是一个很长的段落文本，当超过指定行数时会自动省略..."
///         }
///     }
/// }
/// ```
#[component]
pub fn Paragraph(props: ParagraphProps) -> Element {
    let class_name = get_typography_class_name(&props.text_type, &props.class, false, &props);
    let typography_style = get_typography_style(&props.style, &props.ellipsis_rows);

    rsx! {
        // 注入排版样式
        style { {TYPOGRAPHY_STYLE} }

        p {
            class: class_name.clone(),
            style: typography_style.clone(),
            onclick: move |evt| {
                if let Some(handler) = &props.onclick {
                    handler.call(evt);
                }
            },
            {if props.strong {
                rsx! { strong { {props.children} } }
            } else if props.italic {
                rsx! { em { {props.children} } }
            } else if props.mark {
                rsx! { mark { {props.children} } }
            } else if props.code {
                rsx! { code { {props.children} } }
            } else if props.delete {
                rsx! { del { {props.children} } }
            } else if props.underline {
                rsx! { u { {props.children} } }
            } else {
                props.children
            }}
        }
    }
}

/// 链接组件
///
/// # 参数
///
/// * `props` - 链接属性
///
/// # 示例
///
/// ```rust
/// use dioxus::prelude::*;
/// use ant_design_dioxus::Link;
///
/// fn app() -> Element {
///     rsx! {
///         Link {
///             href: "https://ant.design",
///             target: "_blank",
///             "Ant Design"
///         }
///     }
/// }
/// ```
#[component]
pub fn Link(props: LinkProps) -> Element {
    let class_name = get_typography_class_name(&props.text_type, &props.class, false, &props);
    let typography_style = get_typography_style(&props.style, &props.ellipsis_rows);

    // 添加链接特有的类名
    let link_class = if class_name.contains("ant-typography") {
        format!("{} ant-typography-link", class_name)
    } else {
        "ant-typography ant-typography-link".to_string()
    };

    rsx! {
        // 注入排版样式
        style { {TYPOGRAPHY_STYLE} }

        a {
            class: link_class.clone(),
            style: typography_style.clone(),
            href: props.href.as_deref().unwrap_or("#"),
            target: props.target.as_deref(),
            onclick: move |evt| {
                if props.disabled {
                    evt.prevent_default();
                    return;
                }
                if let Some(handler) = &props.onclick {
                    handler.call(evt);
                }
            },
            {if props.strong {
                rsx! { strong { {props.children} } }
            } else if props.italic {
                rsx! { em { {props.children} } }
            } else if props.mark {
                rsx! { mark { {props.children} } }
            } else if props.code {
                rsx! { code { {props.children} } }
            } else if props.delete {
                rsx! { del { {props.children} } }
            } else if props.underline {
                rsx! { u { {props.children} } }
            } else {
                props.children
            }}
        }
    }
}

/// 获取排版组件的类名
///
/// # 参数
///
/// * `text_type` - 文本类型
/// * `custom_class` - 自定义类名
/// * `is_title` - 是否为标题
/// * `props` - 组件属性（用于获取其他样式属性）
///
/// # 返回值
///
/// 返回排版组件的完整类名字符串
fn get_typography_class_name<T>(
    text_type: &TextType,
    custom_class: &Option<String>,
    is_title: bool,
    props: &T,
) -> String
where
    T: TypographyProps,
{
    let mut classes = vec!["ant-typography"];

    if is_title {
        classes.push("ant-typography-title");
    }

    // 添加文本类型类名
    match text_type {
        TextType::Default => {}
        TextType::Secondary => classes.push("ant-typography-caption"),
        TextType::Success => classes.push("ant-typography-success"),
        TextType::Warning => classes.push("ant-typography-warning"),
        TextType::Danger => classes.push("ant-typography-danger"),
        TextType::Disabled => classes.push("ant-typography-disabled"),
    }

    // 添加样式修饰类名
    if props.get_disabled() {
        classes.push("ant-typography-disabled");
    }

    if props.get_delete() {
        classes.push("ant-typography-delete");
    }

    if props.get_underline() {
        classes.push("ant-typography-underline");
    }

    if props.get_strong() {
        classes.push("ant-typography-strong");
    }

    if props.get_italic() {
        classes.push("ant-typography-italic");
    }

    if props.get_mark() {
        classes.push("ant-typography-mark");
    }

    if props.get_code() {
        classes.push("ant-typography-code");
    }

    if props.get_copyable() {
        classes.push("ant-typography-copy");
    }

    if props.get_editable() {
        classes.push("ant-typography-edit");
    }

    if props.get_ellipsis() {
        classes.push("ant-typography-ellipsis");
        if props.get_ellipsis_rows().is_some() {
            classes.push("ant-typography-ellipsis-multiple-line");
        } else {
            classes.push("ant-typography-ellipsis-single-line");
        }
    }

    // 添加自定义类名
    let mut class_string = classes.join(" ");
    if let Some(custom_class) = custom_class {
        class_string.push(' ');
        class_string.push_str(custom_class);
    }

    class_string
}

/// 获取排版组件的内联样式
///
/// # 参数
///
/// * `custom_style` - 自定义样式
/// * `ellipsis_rows` - 省略行数
///
/// # 返回值
///
/// 返回排版组件的内联样式字符串
fn get_typography_style(custom_style: &Option<String>, ellipsis_rows: &Option<u32>) -> String {
    let mut styles = Vec::new();
    // 设置多行省略样式
    if let Some(rows) = ellipsis_rows {
        styles.push(
            "-webkit-line-clamp: {}"
                .to_string()
                .replace("{}", &rows.to_string()),
        );
        styles.push("-webkit-box-orient: vertical".to_string());
        styles.push("display: -webkit-box".to_string());
    }

    // 添加自定义样式
    let mut style_string = styles.join("; ");
    if let Some(custom_style) = custom_style {
        if !style_string.is_empty() {
            style_string.push_str("; ");
        }
        style_string.push_str(custom_style);
    }

    style_string
}

/// 排版属性 trait
///
/// 用于统一获取不同排版组件的属性
trait TypographyProps {
    fn get_disabled(&self) -> bool;
    fn get_delete(&self) -> bool;
    fn get_underline(&self) -> bool;
    fn get_strong(&self) -> bool;
    fn get_italic(&self) -> bool;
    fn get_mark(&self) -> bool;
    fn get_code(&self) -> bool;
    fn get_copyable(&self) -> bool;
    fn get_editable(&self) -> bool;
    fn get_ellipsis(&self) -> bool;
    fn get_ellipsis_rows(&self) -> &Option<u32>;
}

// 为 TitleProps 实现 TypographyProps
impl TypographyProps for TitleProps {
    fn get_disabled(&self) -> bool {
        self.disabled
    }
    fn get_delete(&self) -> bool {
        self.delete
    }
    fn get_underline(&self) -> bool {
        self.underline
    }
    fn get_strong(&self) -> bool {
        self.strong
    }
    fn get_italic(&self) -> bool {
        self.italic
    }
    fn get_mark(&self) -> bool {
        self.mark
    }
    fn get_code(&self) -> bool {
        self.code
    }
    fn get_copyable(&self) -> bool {
        self.copyable
    }
    fn get_editable(&self) -> bool {
        self.editable
    }
    fn get_ellipsis(&self) -> bool {
        self.ellipsis
    }
    fn get_ellipsis_rows(&self) -> &Option<u32> {
        &self.ellipsis_rows
    }
}

// 为 TextProps 实现 TypographyProps
impl TypographyProps for TextProps {
    fn get_disabled(&self) -> bool {
        self.disabled
    }
    fn get_delete(&self) -> bool {
        self.delete
    }
    fn get_underline(&self) -> bool {
        self.underline
    }
    fn get_strong(&self) -> bool {
        self.strong
    }
    fn get_italic(&self) -> bool {
        self.italic
    }
    fn get_mark(&self) -> bool {
        self.mark
    }
    fn get_code(&self) -> bool {
        self.code
    }
    fn get_copyable(&self) -> bool {
        self.copyable
    }
    fn get_editable(&self) -> bool {
        self.editable
    }
    fn get_ellipsis(&self) -> bool {
        self.ellipsis
    }
    fn get_ellipsis_rows(&self) -> &Option<u32> {
        &self.ellipsis_rows
    }
}

// 为 LinkProps 实现 TypographyProps
impl TypographyProps for LinkProps {
    fn get_disabled(&self) -> bool {
        self.disabled
    }
    fn get_delete(&self) -> bool {
        self.delete
    }
    fn get_underline(&self) -> bool {
        self.underline
    }
    fn get_strong(&self) -> bool {
        self.strong
    }
    fn get_italic(&self) -> bool {
        self.italic
    }
    fn get_mark(&self) -> bool {
        self.mark
    }
    fn get_code(&self) -> bool {
        self.code
    }
    fn get_copyable(&self) -> bool {
        self.copyable
    }
    fn get_editable(&self) -> bool {
        self.editable
    }
    fn get_ellipsis(&self) -> bool {
        self.ellipsis
    }
    fn get_ellipsis_rows(&self) -> &Option<u32> {
        &self.ellipsis_rows
    }
}

// 为 ParagraphProps 实现 TypographyProps
impl TypographyProps for ParagraphProps {
    fn get_disabled(&self) -> bool {
        self.disabled
    }
    fn get_delete(&self) -> bool {
        self.delete
    }
    fn get_underline(&self) -> bool {
        self.underline
    }
    fn get_strong(&self) -> bool {
        self.strong
    }
    fn get_italic(&self) -> bool {
        self.italic
    }
    fn get_mark(&self) -> bool {
        self.mark
    }
    fn get_code(&self) -> bool {
        self.code
    }
    fn get_copyable(&self) -> bool {
        self.copyable
    }
    fn get_editable(&self) -> bool {
        self.editable
    }
    fn get_ellipsis(&self) -> bool {
        self.ellipsis
    }
    fn get_ellipsis_rows(&self) -> &Option<u32> {
        &self.ellipsis_rows
    }
}

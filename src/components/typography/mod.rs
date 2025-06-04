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
mod styles;

// 重新导出所有组件
use self::styles::{
    HeadingLevel as StyleHeadingLevel, LinkStyleGenerator, ParagraphStyleGenerator,
    TextType as StyleTextType, TitleStyleGenerator, TypographyStyleGenerator,
};
pub use link::*;

use css_in_rust::css;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

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
    pub children: Element,
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
    pub children: Element,
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
    let style_level = match props.level {
        HeadingLevel::H1 => StyleHeadingLevel::H1,
        HeadingLevel::H2 => StyleHeadingLevel::H2,
        HeadingLevel::H3 => StyleHeadingLevel::H3,
        HeadingLevel::H4 => StyleHeadingLevel::H4,
        HeadingLevel::H5 => StyleHeadingLevel::H5,
    };
    let style_text_type = match props.text_type {
        TextType::Default => StyleTextType::Default,
        TextType::Secondary => StyleTextType::Secondary,
        TextType::Success => StyleTextType::Success,
        TextType::Warning => StyleTextType::Warning,
        TextType::Danger => StyleTextType::Danger,
        TextType::Disabled => StyleTextType::Disabled,
    };

    let typography_generator = TypographyStyleGenerator::new()
        .with_type(style_text_type)
        .with_disabled(props.disabled)
        .with_delete(props.delete)
        .with_underline(props.underline)
        .with_strong(props.strong)
        .with_italic(props.italic)
        .with_mark(props.mark)
        .with_code(props.code)
        .with_copyable(props.copyable)
        .with_editable(props.editable)
        .with_ellipsis(props.ellipsis)
        .with_ellipsis_rows(props.ellipsis_rows);

    let style_generator =
        TitleStyleGenerator::new(style_level.clone()).with_typography(typography_generator);

    let class_name = style_generator.generate_class();

    let typography_style = get_typography_style(&props.style, &props.ellipsis_rows);

    rsx! {
        style { {style_generator.get_styles()} }

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
    let style_text_type = match props.text_type {
        TextType::Default => StyleTextType::Default,
        TextType::Secondary => StyleTextType::Secondary,
        TextType::Success => StyleTextType::Success,
        TextType::Warning => StyleTextType::Warning,
        TextType::Danger => StyleTextType::Danger,
        TextType::Disabled => StyleTextType::Disabled,
    };

    let typography_generator = TypographyStyleGenerator::new()
        .with_type(style_text_type)
        .with_disabled(props.disabled)
        .with_delete(props.delete)
        .with_underline(props.underline)
        .with_strong(props.strong)
        .with_italic(props.italic)
        .with_mark(props.mark)
        .with_code(props.code)
        .with_copyable(props.copyable)
        .with_editable(props.editable)
        .with_ellipsis(props.ellipsis)
        .with_ellipsis_rows(props.ellipsis_rows);

    let style_generator = ParagraphStyleGenerator::new().with_typography(typography_generator);

    let class_name = style_generator.generate_class();

    let typography_style = get_typography_style(&props.style, &props.ellipsis_rows);

    rsx! {
        style { {style_generator.get_styles()} }

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
    let style_text_type = match props.text_type {
        TextType::Default => StyleTextType::Default,
        TextType::Secondary => StyleTextType::Secondary,
        TextType::Success => StyleTextType::Success,
        TextType::Warning => StyleTextType::Warning,
        TextType::Danger => StyleTextType::Danger,
        TextType::Disabled => StyleTextType::Disabled,
    };

    let typography_generator = TypographyStyleGenerator::new()
        .with_type(style_text_type)
        .with_disabled(props.disabled)
        .with_delete(props.delete)
        .with_underline(props.underline)
        .with_strong(props.strong)
        .with_italic(props.italic)
        .with_mark(props.mark)
        .with_code(props.code)
        .with_copyable(props.copyable)
        .with_editable(props.editable)
        .with_ellipsis(props.ellipsis)
        .with_ellipsis_rows(props.ellipsis_rows);

    let style_generator = ParagraphStyleGenerator::new().with_typography(typography_generator);

    let class_name = style_generator.generate_class();

    let typography_style = get_typography_style(&props.style, &props.ellipsis_rows);

    rsx! {
        style { {style_generator.get_styles()} }

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
pub fn Link(props: link::LinkProps) -> Element {
    let typography_generator = TypographyStyleGenerator::new()
        .with_type(match props.r#type {
            LinkType::Default => StyleTextType::Default,
            LinkType::Secondary => StyleTextType::Secondary,
            LinkType::Success => StyleTextType::Success,
            LinkType::Warning => StyleTextType::Warning,
            LinkType::Danger => StyleTextType::Danger,
        })
        .with_disabled(props.disabled)
        .with_underline(props.underline)
        .with_strong(props.strong);

    let style_generator = LinkStyleGenerator::new()
        .with_typography(typography_generator)
        .with_href(props.href.clone())
        .with_target(props.target.as_ref().map(|t| t.as_str().to_string()))
        .with_block(props.block);

    let class_name = style_generator.generate_class();
    let typography_style = get_typography_style(&props.style, &props.ellipsis_rows);

    // 添加链接特有的类名
    let link_class = if class_name.contains("ant-typography") {
        format!("{} ant-typography-link", class_name)
    } else {
        "ant-typography ant-typography-link".to_string()
    };

    rsx! {
        // 注入排版样式
        style { {style_generator.get_styles()} }

        a {
            class: link_class.clone(),
            style: typography_style.clone(),
            href: props.href.as_deref().unwrap_or("#"),
            target: props.target.as_ref().map(|t| t.as_str()),
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
impl TypographyProps for link::LinkProps {
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

//! Typography 组件样式模块
//!
//! 本模块包含Typography组件的所有样式定义，从组件逻辑中分离出来，
//! 提高代码的可维护性和复用性。

use crate::shared::styles::mixins::*;
use crate::shared::styles::tokens::DesignToken;
use css_in_rust::css;

/// 标题级别枚举
#[derive(Debug, Clone, PartialEq)]
pub enum HeadingLevel {
    H1,
    H2,
    H3,
    H4,
    H5,
}

/// 文本类型枚举
#[derive(Debug, Clone, PartialEq)]
pub enum TextType {
    Default,
    Secondary,
    Success,
    Warning,
    Danger,
    Disabled,
}

/// Typography样式生成器
pub struct TypographyStyleGenerator {
    pub text_type: TextType,
    pub disabled: bool,
    pub delete: bool,
    pub underline: bool,
    pub strong: bool,
    pub italic: bool,
    pub mark: bool,
    pub code: bool,
    pub copyable: bool,
    pub editable: bool,
    pub ellipsis: bool,
    pub ellipsis_rows: Option<u32>,
}

impl TypographyStyleGenerator {
    /// 创建新的Typography样式生成器
    pub fn new() -> Self {
        Self {
            text_type: TextType::Default,
            disabled: false,
            delete: false,
            underline: false,
            strong: false,
            italic: false,
            mark: false,
            code: false,
            copyable: false,
            editable: false,
            ellipsis: false,
            ellipsis_rows: None,
        }
    }

    /// 设置文本类型
    pub fn with_type(mut self, text_type: TextType) -> Self {
        self.text_type = text_type;
        self
    }

    /// 设置禁用状态
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// 设置删除线
    pub fn with_delete(mut self, delete: bool) -> Self {
        self.delete = delete;
        self
    }

    /// 设置下划线
    pub fn with_underline(mut self, underline: bool) -> Self {
        self.underline = underline;
        self
    }

    /// 设置强调
    pub fn with_strong(mut self, strong: bool) -> Self {
        self.strong = strong;
        self
    }

    /// 设置斜体
    pub fn with_italic(mut self, italic: bool) -> Self {
        self.italic = italic;
        self
    }

    /// 设置标记
    pub fn with_mark(mut self, mark: bool) -> Self {
        self.mark = mark;
        self
    }

    /// 设置代码
    pub fn with_code(mut self, code: bool) -> Self {
        self.code = code;
        self
    }

    /// 设置可复制
    pub fn with_copyable(mut self, copyable: bool) -> Self {
        self.copyable = copyable;
        self
    }

    /// 设置可编辑
    pub fn with_editable(mut self, editable: bool) -> Self {
        self.editable = editable;
        self
    }

    /// 设置省略
    pub fn with_ellipsis(mut self, ellipsis: bool) -> Self {
        self.ellipsis = ellipsis;
        self
    }

    /// 设置省略行数
    pub fn with_ellipsis_rows(mut self, rows: Option<u32>) -> Self {
        self.ellipsis_rows = rows;
        self
    }

    /// 生成完整的Typography样式类名
    pub fn generate(&self) -> String {
        let mut classes = vec![self.base_style(), self.type_style()];

        if self.disabled {
            classes.push(self.disabled_style());
        }

        if self.delete {
            classes.push(self.delete_style());
        }

        if self.underline {
            classes.push(self.underline_style());
        }

        if self.strong {
            classes.push(self.strong_style());
        }

        if self.italic {
            classes.push(self.italic_style());
        }

        if self.mark {
            classes.push(self.mark_style());
        }

        if self.code {
            classes.push(self.code_style());
        }

        if self.ellipsis {
            classes.push(self.ellipsis_style());
        }

        classes.join(" ")
    }

    /// 生成样式类名（别名方法）
    pub fn generate_class(&self) -> String {
        self.generate()
    }

    /// 获取CSS样式
    pub fn get_styles(&self) -> String {
        self.generate()
    }

    /// 基础Typography样式
    fn base_style(&self) -> String {
        css!(
            r#"
            color: rgba(0, 0, 0, 0.88);
            font-size: 14px;
            line-height: 1.5715;
            list-style: none;
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Noto Sans', sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol', 'Noto Color Emoji';
            margin: 0;
            padding: 0;
            "#
        ).to_string()
    }

    /// 文本类型样式
    fn type_style(&self) -> String {
        match self.text_type {
            TextType::Default => css!(
                r#"
                color: rgba(0, 0, 0, 0.88);
                "#
            )
            .to_string(),
            TextType::Secondary => css!(
                r#"
                color: rgba(0, 0, 0, 0.45);
                "#
            )
            .to_string(),
            TextType::Success => css!(
                r#"
                color: #52c41a;
                "#
            )
            .to_string(),
            TextType::Warning => css!(
                r#"
                color: #faad14;
                "#
            )
            .to_string(),
            TextType::Danger => css!(
                r#"
                color: #ff4d4f;
                "#
            )
            .to_string(),
            TextType::Disabled => css!(
                r#"
                color: rgba(0, 0, 0, 0.25);
                cursor: not-allowed;
                "#
            )
            .to_string(),
        }
    }

    /// 禁用状态样式
    fn disabled_style(&self) -> String {
        css!(
            r#"
            color: rgba(0, 0, 0, 0.25);
            cursor: not-allowed;
            user-select: none;
            "#
        )
        .to_string()
    }

    /// 删除线样式
    fn delete_style(&self) -> String {
        css!(
            r#"
            text-decoration: line-through;
            text-decoration-color: currentColor;
            "#
        )
        .to_string()
    }

    /// 下划线样式
    fn underline_style(&self) -> String {
        css!(
            r#"
            text-decoration: underline;
            text-decoration-color: currentColor;
            text-underline-offset: 3px;
            "#
        )
        .to_string()
    }

    /// 强调样式
    fn strong_style(&self) -> String {
        css!(
            r#"
            font-weight: 600;
            "#
        )
        .to_string()
    }

    /// 斜体样式
    fn italic_style(&self) -> String {
        css!(
            r#"
            font-style: italic;
            "#
        )
        .to_string()
    }

    /// 标记样式
    fn mark_style(&self) -> String {
        css!(
            r#"
            background-color: #ffe58f;
            padding: 0.2em 0.4em;
            margin: 0;
            border-radius: 3px;
            "#
        )
        .to_string()
    }

    /// 代码样式
    fn code_style(&self) -> String {
        css!(
            r#"
            margin: 0 0.2em;
            padding: 0.2em 0.4em;
            font-size: 85%;
            background: rgba(150, 150, 150, 0.1);
            border: 1px solid rgba(100, 100, 100, 0.2);
            border-radius: 3px;
            font-family: 'SFMono-Regular', Consolas, 'Liberation Mono', Menlo, Courier, monospace;
            "#
        )
        .to_string()
    }

    /// 省略样式
    fn ellipsis_style(&self) -> String {
        if let Some(_rows) = self.ellipsis_rows {
            css!(format!(
                r#"
                    display: -webkit-box;
                    -webkit-line-clamp: {};
                    -webkit-box-orient: vertical;
                    overflow: hidden;
                    text-overflow: ellipsis;
                    word-break: break-word;
                    "#,
                rows
            ))
            .to_string()
        } else {
            css!(
                r#"
                overflow: hidden;
                white-space: nowrap;
                text-overflow: ellipsis;
                "#
            )
            .to_string()
        }
    }
}

/// 标题样式生成器
pub struct TitleStyleGenerator {
    pub level: HeadingLevel,
    pub typography_generator: TypographyStyleGenerator,
}

impl TitleStyleGenerator {
    /// 创建新的标题样式生成器
    pub fn new(level: HeadingLevel) -> Self {
        Self {
            level,
            typography_generator: TypographyStyleGenerator::new(),
        }
    }

    /// 设置Typography样式生成器
    pub fn with_typography(mut self, generator: TypographyStyleGenerator) -> Self {
        self.typography_generator = generator;
        self
    }

    /// 生成完整的标题样式类名
    pub fn generate(&self) -> String {
        let classes = vec![self.typography_generator.generate(), self.heading_style()];

        classes.join(" ")
    }

    /// 生成样式类名（别名方法）
    pub fn generate_class(&self) -> String {
        self.generate()
    }

    /// 获取CSS样式
    pub fn get_styles(&self) -> String {
        self.generate()
    }

    /// 标题级别样式
    fn heading_style(&self) -> String {
        match self.level {
            HeadingLevel::H1 => css!(
                r#"
                margin-bottom: 0.5em;
                color: rgba(0, 0, 0, 0.88);
                font-weight: 600;
                font-size: 38px;
                line-height: 1.23;
                "#
            )
            .to_string(),
            HeadingLevel::H2 => css!(
                r#"
                margin-bottom: 0.5em;
                color: rgba(0, 0, 0, 0.88);
                font-weight: 600;
                font-size: 30px;
                line-height: 1.35;
                "#
            )
            .to_string(),
            HeadingLevel::H3 => css!(
                r#"
                margin-bottom: 0.5em;
                color: rgba(0, 0, 0, 0.88);
                font-weight: 600;
                font-size: 24px;
                line-height: 1.35;
                "#
            )
            .to_string(),
            HeadingLevel::H4 => css!(
                r#"
                margin-bottom: 0.5em;
                color: rgba(0, 0, 0, 0.88);
                font-weight: 600;
                font-size: 20px;
                line-height: 1.4;
                "#
            )
            .to_string(),
            HeadingLevel::H5 => css!(
                r#"
                margin-bottom: 0.5em;
                color: rgba(0, 0, 0, 0.88);
                font-weight: 600;
                font-size: 16px;
                line-height: 1.5;
                "#
            )
            .to_string(),
        }
    }
}

/// 段落样式生成器
pub struct ParagraphStyleGenerator {
    pub typography_generator: TypographyStyleGenerator,
}

impl ParagraphStyleGenerator {
    /// 创建新的段落样式生成器
    pub fn new() -> Self {
        Self {
            typography_generator: TypographyStyleGenerator::new(),
        }
    }

    /// 设置Typography样式生成器
    pub fn with_typography(mut self, generator: TypographyStyleGenerator) -> Self {
        self.typography_generator = generator;
        self
    }

    /// 生成完整的段落样式类名
    pub fn generate(&self) -> String {
        let classes = vec![self.typography_generator.generate(), self.paragraph_style()];

        classes.join(" ")
    }

    /// 生成样式类名（别名方法）
    pub fn generate_class(&self) -> String {
        self.generate()
    }

    /// 获取CSS样式
    pub fn get_styles(&self) -> String {
        self.generate()
    }

    /// 段落样式
    fn paragraph_style(&self) -> String {
        css!(
            r#"
            margin-bottom: 1em;

            &:last-child {
                margin-bottom: 0;
            }
            "#
        )
        .to_string()
    }
}

/// 链接样式生成器
pub struct LinkStyleGenerator {
    pub typography_generator: TypographyStyleGenerator,
    pub href: Option<String>,
    pub target: Option<String>,
    pub block: bool,
}

impl LinkStyleGenerator {
    /// 创建新的链接样式生成器
    pub fn new() -> Self {
        Self {
            typography_generator: TypographyStyleGenerator::new(),
            href: None,
            target: None,
            block: false,
        }
    }

    /// 设置Typography样式生成器
    pub fn with_typography(mut self, generator: TypographyStyleGenerator) -> Self {
        self.typography_generator = generator;
        self
    }

    /// 设置链接地址
    pub fn with_href(mut self, href: Option<String>) -> Self {
        self.href = href;
        self
    }

    /// 设置链接目标
    pub fn with_target(mut self, target: Option<String>) -> Self {
        self.target = target;
        self
    }

    /// 设置块级链接
    pub fn with_block(mut self, block: bool) -> Self {
        self.block = block;
        self
    }

    /// 生成完整的链接样式类名
    pub fn generate(&self) -> String {
        let mut classes = vec![self.typography_generator.generate(), self.link_style()];

        if self.block {
            classes.push(self.block_style());
        }

        classes.join(" ")
    }

    /// 生成样式类名（别名方法）
    pub fn generate_class(&self) -> String {
        self.generate()
    }

    /// 获取CSS样式
    pub fn get_styles(&self) -> String {
        self.generate()
    }

    /// 链接样式
    fn link_style(&self) -> String {
        let _base_color = match self.typography_generator.text_type {
            TextType::Default => "#1677ff",
            TextType::Secondary => "rgba(0, 0, 0, 0.45)",
            TextType::Success => "#52c41a",
            TextType::Warning => "#faad14",
            TextType::Danger => "#ff4d4f",
            TextType::Disabled => "rgba(0, 0, 0, 0.25)",
        };

        let _hover_color = match self.typography_generator.text_type {
            TextType::Default => "#4096ff",
            TextType::Secondary => "rgba(0, 0, 0, 0.65)",
            TextType::Success => "#73d13d",
            TextType::Warning => "#ffc53d",
            TextType::Danger => "#ff7875",
            TextType::Disabled => "rgba(0, 0, 0, 0.25)",
        };

        let _active_color = match self.typography_generator.text_type {
            TextType::Default => "#0958d9",
            TextType::Secondary => "rgba(0, 0, 0, 0.85)",
            TextType::Success => "#389e0d",
            TextType::Warning => "#d48806",
            TextType::Danger => "#d9363e",
            TextType::Disabled => "rgba(0, 0, 0, 0.25)",
        };

        if self.typography_generator.disabled {
            css!(format!(
                r#"
                    color: {};
                    text-decoration: none;
                    background-color: transparent;
                    outline: none;
                    cursor: not-allowed;
                    pointer-events: none;
                    border: 0;
                    padding: 0;
                    font-size: inherit;
                    line-height: inherit;
                    text-decoration: underline;
                    text-decoration-color: transparent;
                    text-underline-offset: 3px;
                    "#,
                base_color
            ))
            .to_string()
        } else {
            css!(format!(
                r#"
                    color: {};
                    text-decoration: none;
                    background-color: transparent;
                    outline: none;
                    cursor: pointer;
                    transition: color 0.2s;
                    border: 0;
                    padding: 0;
                    font-size: inherit;
                    line-height: inherit;
                    text-decoration: underline;
                    text-decoration-color: transparent;
                    text-underline-offset: 3px;
                    transition: text-decoration-color 0.2s;
                    touch-action: manipulation;

                    &:hover {{
                        color: {};
                        text-decoration-color: currentColor;
                    }}

                    &:active {{
                        color: {};
                    }}

                    &:focus {{
                        outline: 2px solid #1677ff;
                        outline-offset: 2px;
                    }}
                    "#,
                base_color, hover_color, active_color
            ))
            .to_string()
        }
    }

    /// 块级链接样式
    fn block_style(&self) -> String {
        css!(
            r#"
            display: block;
            width: 100%;
            "#
        )
        .to_string()
    }
}

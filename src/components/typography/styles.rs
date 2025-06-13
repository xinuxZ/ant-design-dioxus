//! Typography 组件样式模块
//!
//! 本模块包含Typography组件的所有样式定义，从组件逻辑中分离出来，
//! 提高代码的可维护性和复用性。

use css_in_rust::css;

use super::types::*;
use crate::theme::Theme;

/// Typography 通用样式生成器
#[derive(Clone)]
pub struct TypographyStyleGenerator {
    pub text_type: TextType,
    pub disabled: bool,
    pub delete: bool,
    pub underline: bool,
    pub strong: bool,
    pub italic: bool,
    pub mark: bool,
    pub code: bool,
    pub keyboard: bool,
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
            keyboard: false,
            copyable: false,
            editable: false,
            ellipsis: false,
            ellipsis_rows: None,
        }
    }
    
    /// 生成可访问性样式
    pub fn generate_accessibility_styles(&self) -> String {
        r#"
        /* 屏幕阅读器专用样式 */
        .sr-only {
            position: absolute !important;
            width: 1px !important;
            height: 1px !important;
            padding: 0 !important;
            margin: -1px !important;
            overflow: hidden !important;
            clip: rect(0, 0, 0, 0) !important;
            white-space: nowrap !important;
            border: 0 !important;
        }
        
        /* 高对比度模式支持 */
        @media (prefers-contrast: high) {
            .ant-typography {
                color: CanvasText;
                background-color: Canvas;
            }
            
            .ant-typography a {
                color: LinkText;
                text-decoration: underline;
            }
            
            .ant-typography a:visited {
                color: VisitedText;
            }
            
            .ant-typography code,
            .ant-typography kbd {
                border: 1px solid CanvasText;
                background-color: Canvas;
                color: CanvasText;
            }
        }
        
        /* 减少动画偏好支持 */
        @media (prefers-reduced-motion: reduce) {
            .ant-typography *,
            .ant-typography *::before,
            .ant-typography *::after {
                animation-duration: 0.01ms !important;
                animation-iteration-count: 1 !important;
                transition-duration: 0.01ms !important;
                scroll-behavior: auto !important;
            }
        }
        
        /* 焦点指示器增强 */
        .ant-typography a:focus,
        .ant-typography button:focus,
        .ant-typography [tabindex]:focus {
            outline: 2px solid var(--ant-color-primary, #1890ff);
            outline-offset: 2px;
            border-radius: 2px;
        }
        
        /* 确保焦点指示器在高对比度模式下可见 */
        @media (prefers-contrast: high) {
            .ant-typography a:focus,
            .ant-typography button:focus,
            .ant-typography [tabindex]:focus {
                outline: 2px solid Highlight;
                outline-offset: 2px;
            }
        }
        "#.to_string()
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

    /// 设置键盘样式
    pub fn with_keyboard(mut self, keyboard: bool) -> Self {
        self.keyboard = keyboard;
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

        if self.keyboard {
            classes.push(self.keyboard_style());
        }

        if self.ellipsis {
            classes.push(self.ellipsis_style());
            classes.push(Self::ellipsis_content_style());
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
            line-height: 1.5714285714285714;
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Noto Sans', sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol', 'Noto Color Emoji';
            margin: 0;
            padding: 0;
            word-wrap: break-word;
            word-break: break-word;
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
                color: rgba(0, 0, 0, 0.65);
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
                user-select: none;
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
            "#
        )
        .to_string()
    }

    /// 下划线样式
    fn underline_style(&self) -> String {
        css!(
            r#"
            text-decoration: underline;
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
            padding: 0 2px;
            border-radius: 2px;
            "#
        )
        .to_string()
    }

    /// 代码样式
    fn code_style(&self) -> String {
        css!(
            r#"
            font-family: 'SFMono-Regular', Consolas, 'Liberation Mono', Menlo, Courier, monospace;
            background-color: rgba(150, 150, 150, 0.1);
            border: 1px solid rgba(100, 100, 100, 0.2);
            border-radius: 3px;
            padding: 0.2em 0.4em;
            font-size: 0.85em;
            "#
        )
        .to_string()
    }

    /// 键盘样式
    fn keyboard_style(&self) -> String {
        css!(
            r#"
            font-family: 'SFMono-Regular', Consolas, 'Liberation Mono', Menlo, Courier, monospace;
            background-color: #fafafa;
            border: 1px solid #d9d9d9;
            border-radius: 6px;
            box-shadow: 0 2px 0 rgba(0, 0, 0, 0.045);
            padding: 0.15em 0.4em;
            font-size: 0.85em;
            display: inline-block;
            line-height: 1.2;
            "#
        )
        .to_string()
    }

    /// 省略样式
    fn ellipsis_style(&self) -> String {
        if let Some(rows) = self.ellipsis_rows {
            if rows > 1 {
                css!(
                    r#"
                    display: -webkit-box;
                    -webkit-box-orient: vertical;
                    -webkit-line-clamp: {};
                    overflow: hidden;
                    word-break: break-all;
                    "#,
                    rows
                )
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

    /// 展开/收起按钮样式
    pub fn expand_button_style() -> String {
        css!(
            r#"
            background: none;
            border: none;
            color: #1890ff;
            cursor: pointer;
            font-size: inherit;
            margin-left: 4px;
            padding: 0;
            text-decoration: none;
            
            &:hover {
                color: #40a9ff;
                text-decoration: underline;
            }
            
            &:active {
                color: #096dd9;
            }
            "#
        )
        .to_string()
    }
    
    /// 省略内容样式
    pub fn ellipsis_content_style() -> String {
        css!(
            r#"
            .typography-ellipsis-content {
                overflow: hidden;
                white-space: nowrap;
                text-overflow: ellipsis;
            }
            
            .typography-expand-button {
                background: none;
                border: none;
                color: #1890ff;
                cursor: pointer;
                font-size: inherit;
                margin-left: 4px;
                padding: 0;
                text-decoration: none;
            }
            
            .typography-expand-button:hover {
                color: #40a9ff;
                text-decoration: underline;
            }
            
            .typography-ellipsis-suffix {
                margin-left: 4px;
                color: #8c8c8c;
            }
            "#
        )
        .to_string()
    }

    /// 生成交互按钮样式
    pub fn action_button_style() -> String {
        css!(
            r#"
            display: inline-flex;
            align-items: center;
            justify-content: center;
            width: 16px;
            height: 16px;
            margin-left: 4px;
            padding: 0;
            border: none;
            background: transparent;
            cursor: pointer;
            color: rgba(0, 0, 0, 0.45);
            border-radius: 2px;
            transition: color 0.3s;

            &:hover {
                color: rgba(0, 0, 0, 0.88);
                background-color: rgba(0, 0, 0, 0.06);
            }
            "#
        )
        .to_string()
    }
}

/// Title 样式生成器
pub struct TitleStyleGenerator {
    pub level: HeadingLevel,
    pub typography_generator: TypographyStyleGenerator,
}

impl TitleStyleGenerator {
    /// 创建新的Title样式生成器
    pub fn new(level: HeadingLevel) -> Self {
        Self {
            level,
            typography_generator: TypographyStyleGenerator::new(),
        }
    }

    /// 设置文本类型
    pub fn with_type(mut self, text_type: TextType) -> Self {
        self.typography_generator = self.typography_generator.with_type(text_type);
        self
    }

    /// 设置禁用状态
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.typography_generator = self.typography_generator.with_disabled(disabled);
        self
    }

    /// 生成完整的Title样式类名
    pub fn generate(&self) -> String {
        let mut classes = vec![self.typography_generator.generate(), self.title_style()];
        classes.join(" ")
    }

    /// 生成样式类名（别名方法）
    pub fn generate_class(&self) -> String {
        self.generate()
    }

    /// 标题样式
    fn title_style(&self) -> String {
        let (font_size, line_height, margin_top, margin_bottom, font_weight) = match self.level {
            HeadingLevel::H1 => ("38px", "1.23", "0", "0.5em", "600"),
            HeadingLevel::H2 => ("30px", "1.35", "1.2em", "0.5em", "600"),
            HeadingLevel::H3 => ("24px", "1.35", "1.2em", "0.5em", "600"),
            HeadingLevel::H4 => ("20px", "1.4", "1.2em", "0.5em", "600"),
            HeadingLevel::H5 => ("16px", "1.5", "1.2em", "0.5em", "600"),
        };

        css!(
            r#"
            color: rgba(0, 0, 0, 0.88);
            font-weight: {};
            font-size: {};
            line-height: {};
            margin-top: {};
            margin-bottom: {};
            margin-left: 0;
            margin-right: 0;
            "#,
            font_weight,
            font_size,
            line_height,
            margin_top,
            margin_bottom
        )
        .to_string()
    }
}

/// Paragraph 样式生成器
pub struct ParagraphStyleGenerator {
    pub typography_generator: TypographyStyleGenerator,
}

impl ParagraphStyleGenerator {
    /// 创建新的Paragraph样式生成器
    pub fn new() -> Self {
        Self {
            typography_generator: TypographyStyleGenerator::new(),
        }
    }

    /// 设置文本类型
    pub fn with_type(mut self, text_type: TextType) -> Self {
        self.typography_generator = self.typography_generator.with_type(text_type);
        self
    }

    /// 设置禁用状态
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.typography_generator = self.typography_generator.with_disabled(disabled);
        self
    }

    /// 设置删除线
    pub fn with_delete(mut self, delete: bool) -> Self {
        self.typography_generator = self.typography_generator.with_delete(delete);
        self
    }

    /// 设置下划线
    pub fn with_underline(mut self, underline: bool) -> Self {
        self.typography_generator = self.typography_generator.with_underline(underline);
        self
    }

    /// 设置强调
    pub fn with_strong(mut self, strong: bool) -> Self {
        self.typography_generator = self.typography_generator.with_strong(strong);
        self
    }

    /// 设置斜体
    pub fn with_italic(mut self, italic: bool) -> Self {
        self.typography_generator = self.typography_generator.with_italic(italic);
        self
    }

    /// 设置标记
    pub fn with_mark(mut self, mark: bool) -> Self {
        self.typography_generator = self.typography_generator.with_mark(mark);
        self
    }

    /// 设置代码
    pub fn with_code(mut self, code: bool) -> Self {
        self.typography_generator = self.typography_generator.with_code(code);
        self
    }

    /// 设置键盘样式
    pub fn with_keyboard(mut self, keyboard: bool) -> Self {
        self.typography_generator = self.typography_generator.with_keyboard(keyboard);
        self
    }

    /// 设置可复制
    pub fn with_copyable(mut self, copyable: bool) -> Self {
        self.typography_generator = self.typography_generator.with_copyable(copyable);
        self
    }

    /// 设置可编辑
    pub fn with_editable(mut self, editable: bool) -> Self {
        self.typography_generator = self.typography_generator.with_editable(editable);
        self
    }

    /// 设置省略
    pub fn with_ellipsis(mut self, ellipsis: bool) -> Self {
        self.typography_generator = self.typography_generator.with_ellipsis(ellipsis);
        self
    }

    /// 设置省略行数
    pub fn with_ellipsis_rows(mut self, rows: Option<u32>) -> Self {
        self.typography_generator = self.typography_generator.with_ellipsis_rows(rows);
        self
    }

    /// 生成完整的Paragraph样式类名
    pub fn generate(&self) -> String {
        let mut classes = vec![self.typography_generator.generate(), self.paragraph_style()];
        classes.join(" ")
    }

    /// 生成样式类名（别名方法）
    pub fn generate_class(&self) -> String {
        self.generate()
    }

    /// 段落样式
    fn paragraph_style(&self) -> String {
        css!(
            r#"
            margin-bottom: 1em;
            margin-top: 0;
            "#
        )
        .to_string()
    }
}

/// Link 样式生成器
pub struct LinkStyleGenerator {
    pub link_type: LinkType,
    pub disabled: bool,
    pub block: bool,
    pub typography_generator: TypographyStyleGenerator,
}

impl LinkStyleGenerator {
    /// 创建新的Link样式生成器
    pub fn new() -> Self {
        Self {
            link_type: LinkType::Default,
            disabled: false,
            block: false,
            typography_generator: TypographyStyleGenerator::new(),
        }
    }

    /// 设置链接类型
    pub fn with_type(mut self, link_type: LinkType) -> Self {
        self.link_type = link_type;
        self
    }

    /// 设置禁用状态
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// 设置块级显示
    pub fn with_block(mut self, block: bool) -> Self {
        self.block = block;
        self
    }

    /// 生成完整的Link样式类名
    pub fn generate(&self) -> String {
        let mut classes = vec![self.typography_generator.base_style(), self.link_style()];
        classes.join(" ")
    }

    /// 生成样式类名（别名方法）
    pub fn generate_class(&self) -> String {
        self.generate()
    }

    /// 链接样式
    fn link_style(&self) -> String {
        if self.disabled {
            css!(
                r#"
                color: rgba(0, 0, 0, 0.25);
                cursor: not-allowed;
                pointer-events: none;
                text-decoration: none;
                display: {};
                "#,
                if self.block { "block" } else { "inline" }
            )
            .to_string()
        } else {
            let color = match self.link_type {
                LinkType::Default => "#1677ff",
                LinkType::Secondary => "rgba(0, 0, 0, 0.45)",
                LinkType::Disabled => "rgba(0, 0, 0, 0.25)",
                LinkType::Success => "#52c41a",
                LinkType::Warning => "#faad14",
                LinkType::Danger => "#ff4d4f",
            };

            let hover_color = match self.link_type {
                LinkType::Default => "#4096ff",
                LinkType::Secondary => "rgba(0, 0, 0, 0.65)",
                LinkType::Disabled => "rgba(0, 0, 0, 0.25)",
                LinkType::Success => "#73d13d",
                LinkType::Warning => "#ffc53d",
                LinkType::Danger => "#ff7875",
            };

            let active_color = match self.link_type {
                LinkType::Default => "#0958d9",
                LinkType::Secondary => "rgba(0, 0, 0, 0.85)",
                LinkType::Disabled => "rgba(0, 0, 0, 0.25)",
                LinkType::Success => "#389e0d",
                LinkType::Warning => "#d48806",
                LinkType::Danger => "#d9363e",
            };

            css!(
                r#"
                color: {};
                text-decoration: none;
                cursor: pointer;
                transition: color 0.3s;
                display: {};

                &:hover {
                    color: {};
                }

                &:active {
                    color: {};
                }
                "#,
                color,
                if self.block { "block" } else { "inline" },
                hover_color,
                active_color
            )
            .to_string()
        }
    }
}

/// 编辑模式样式生成器
pub struct EditStyleGenerator;

impl EditStyleGenerator {
    /// 生成编辑输入框样式
    pub fn edit_input_style() -> String {
        css!(
            r#"
            width: 100%;
            border: 1px solid #d9d9d9;
            border-radius: 6px;
            padding: 4px 11px;
            font-size: inherit;
            font-family: inherit;
            line-height: inherit;
            color: inherit;
            background-color: #fff;
            outline: none;
            transition: border-color 0.3s, box-shadow 0.3s;

            &:focus {
                border-color: #4096ff;
                box-shadow: 0 0 0 2px rgba(5, 145, 255, 0.1);
            }
            "#
        )
        .to_string()
    }

    /// 生成编辑操作按钮容器样式
    pub fn edit_actions_style() -> String {
        css!(
            r#"
            display: inline-flex;
            gap: 4px;
            margin-left: 8px;
            align-items: center;
            "#
        )
        .to_string()
    }

    /// 生成编辑操作按钮样式
    pub fn edit_action_button_style() -> String {
        css!(
            r#"
            display: inline-flex;
            align-items: center;
            justify-content: center;
            width: 20px;
            height: 20px;
            padding: 0;
            border: none;
            background: transparent;
            cursor: pointer;
            color: rgba(0, 0, 0, 0.45);
            border-radius: 2px;
            transition: color 0.3s, background-color 0.3s;

            &:hover {
                color: rgba(0, 0, 0, 0.88);
                background-color: rgba(0, 0, 0, 0.06);
            }
            "#
        )
        .to_string()
    }
}

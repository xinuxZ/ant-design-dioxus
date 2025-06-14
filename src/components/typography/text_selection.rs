//! Typography 文本选择模块
//!
//! 提供文本选择、复制、高亮显示等功能

use dioxus::prelude::*;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::window;

/// 文本选择范围
#[derive(Debug, Clone, PartialEq)]
pub struct TextRange {
    pub start: usize,
    pub end: usize,
    pub text: String,
}

impl Default for TextRange {
    fn default() -> Self {
        Self {
            start: 0,
            end: 0,
            text: String::new(),
        }
    }
}

impl TextRange {
    /// 创建新的文本范围
    pub fn new(start: usize, end: usize, text: String) -> Self {
        Self { start, end, text }
    }

    /// 检查范围是否有效
    pub fn is_valid(&self) -> bool {
        self.start <= self.end && !self.text.is_empty()
    }

    /// 获取选择的文本长度
    pub fn length(&self) -> usize {
        self.end - self.start
    }

    /// 检查是否为空选择
    pub fn is_empty(&self) -> bool {
        self.start == self.end || self.text.is_empty()
    }

    /// 检查是否包含指定位置
    pub fn contains(&self, position: usize) -> bool {
        position >= self.start && position <= self.end
    }

    /// 与另一个范围合并
    pub fn merge(&self, other: &TextRange) -> TextRange {
        let start = self.start.min(other.start);
        let end = self.end.max(other.end);
        let text = if self.start <= other.start {
            format!("{}{}", self.text, other.text)
        } else {
            format!("{}{}", other.text, self.text)
        };
        TextRange::new(start, end, text)
    }
}

/// 选择模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionMode {
    /// 字符选择
    Character,
    /// 单词选择
    Word,
    /// 行选择
    Line,
    /// 段落选择
    Paragraph,
    /// 全选
    All,
}

impl Default for SelectionMode {
    fn default() -> Self {
        Self::Character
    }
}

/// 文本选择配置
#[derive(Debug, Clone, PartialEq)]
pub struct SelectionConfig {
    pub enabled: bool,
    pub mode: SelectionMode,
    pub allow_copy: bool,
    pub show_toolbar: bool,
    pub highlight_color: String,
    pub toolbar_position: ToolbarPosition,
}

impl Default for SelectionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            mode: SelectionMode::Character,
            allow_copy: true,
            show_toolbar: true,
            highlight_color: "#1890ff".to_string(),
            toolbar_position: ToolbarPosition::Auto,
        }
    }
}

/// 工具栏位置
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolbarPosition {
    /// 自动定位
    Auto,
    /// 选择区域上方
    Top,
    /// 选择区域下方
    Bottom,
    /// 选择区域左侧
    Left,
    /// 选择区域右侧
    Right,
    /// 固定位置
    Fixed,
}

/// 文本选择器
pub struct TextSelector {
    config: SelectionConfig,
    current_selection: Option<TextRange>,
    selection_history: Vec<TextRange>,
    max_history: usize,
}

impl Default for TextSelector {
    fn default() -> Self {
        Self {
            config: SelectionConfig::default(),
            current_selection: None,
            selection_history: Vec::new(),
            max_history: 10,
        }
    }
}

impl TextSelector {
    /// 创建新的文本选择器
    pub fn new(config: SelectionConfig) -> Self {
        Self {
            config,
            current_selection: None,
            selection_history: Vec::new(),
            max_history: 10,
        }
    }

    /// 设置当前选择
    pub fn set_selection(&mut self, range: TextRange) {
        if range.is_valid() {
            // 添加到历史记录
            if let Some(current) = &self.current_selection {
                self.add_to_history(current.clone());
            }
            self.current_selection = Some(range);
        }
    }

    /// 清除选择
    pub fn clear_selection(&mut self) {
        if let Some(current) = &self.current_selection {
            self.add_to_history(current.clone());
        }
        self.current_selection = None;
    }

    /// 获取当前选择
    pub fn get_selection(&self) -> Option<&TextRange> {
        self.current_selection.as_ref()
    }

    /// 添加到历史记录
    fn add_to_history(&mut self, range: TextRange) {
        self.selection_history.push(range);
        if self.selection_history.len() > self.max_history {
            self.selection_history.remove(0);
        }
    }

    /// 获取选择历史
    pub fn get_history(&self) -> &[TextRange] {
        &self.selection_history
    }

    /// 根据模式选择文本
    pub fn select_by_mode(
        &mut self,
        text: &str,
        position: usize,
        mode: SelectionMode,
    ) -> Option<TextRange> {
        match mode {
            SelectionMode::Character => self.select_character(text, position),
            SelectionMode::Word => self.select_word(text, position),
            SelectionMode::Line => self.select_line(text, position),
            SelectionMode::Paragraph => self.select_paragraph(text, position),
            SelectionMode::All => self.select_all(text),
        }
    }

    /// 选择字符
    fn select_character(&self, text: &str, position: usize) -> Option<TextRange> {
        if position < text.len() {
            let chars: Vec<char> = text.chars().collect();
            if position < chars.len() {
                let selected_char = chars[position].to_string();
                Some(TextRange::new(position, position + 1, selected_char))
            } else {
                None
            }
        } else {
            None
        }
    }

    /// 选择单词
    fn select_word(&self, text: &str, position: usize) -> Option<TextRange> {
        if position >= text.len() {
            return None;
        }

        let chars: Vec<char> = text.chars().collect();
        if position >= chars.len() {
            return None;
        }

        // 找到单词边界
        let mut start = position;
        let mut end = position;

        // 向前找到单词开始
        while start > 0 && chars[start - 1].is_alphanumeric() {
            start -= 1;
        }

        // 向后找到单词结束
        while end < chars.len() && chars[end].is_alphanumeric() {
            end += 1;
        }

        if start < end {
            let word: String = chars[start..end].iter().collect();
            Some(TextRange::new(start, end, word))
        } else {
            None
        }
    }

    /// 选择行
    fn select_line(&self, text: &str, position: usize) -> Option<TextRange> {
        if position >= text.len() {
            return None;
        }

        let lines: Vec<&str> = text.lines().collect();
        let mut current_pos = 0;

        for line in lines {
            let line_end = current_pos + line.len();
            if position >= current_pos && position <= line_end {
                return Some(TextRange::new(current_pos, line_end, line.to_string()));
            }
            current_pos = line_end + 1; // +1 for newline
        }

        None
    }

    /// 选择段落
    fn select_paragraph(&self, text: &str, position: usize) -> Option<TextRange> {
        if position >= text.len() {
            return None;
        }

        let paragraphs: Vec<&str> = text.split("\n\n").collect();
        let mut current_pos = 0;

        for paragraph in paragraphs {
            let paragraph_end = current_pos + paragraph.len();
            if position >= current_pos && position <= paragraph_end {
                return Some(TextRange::new(
                    current_pos,
                    paragraph_end,
                    paragraph.to_string(),
                ));
            }
            current_pos = paragraph_end + 2; // +2 for double newline
        }

        None
    }

    /// 全选
    fn select_all(&self, text: &str) -> Option<TextRange> {
        if !text.is_empty() {
            Some(TextRange::new(0, text.len(), text.to_string()))
        } else {
            None
        }
    }
}

/// 复制功能
pub struct CopyManager {
    copy_history: Vec<String>,
    max_history: usize,
}

impl Default for CopyManager {
    fn default() -> Self {
        Self {
            copy_history: Vec::new(),
            max_history: 20,
        }
    }
}

impl CopyManager {
    /// 创建新的复制管理器
    pub fn new() -> Self {
        Self::default()
    }

    /// 复制文本到剪贴板
    pub async fn copy_to_clipboard(&mut self, text: &str) -> Result<(), JsValue> {
        if let Some(window) = window() {
            let navigator = window.navigator().clipboard();
            let promise = navigator.write_text(text);
            wasm_bindgen_futures::JsFuture::from(promise).await?;

            // 添加到历史记录
            self.add_to_history(text.to_string());
            return Ok(());
        }

        // 降级方案：使用 execCommand
        self.copy_with_exec_command(text)
    }

    /// 使用 execCommand 复制（降级方案）
    fn copy_with_exec_command(&mut self, text: &str) -> Result<(), JsValue> {
        if let Some(window) = window() {
            if let Some(document) = window.document() {
                // 创建临时文本区域
                let textarea = document.create_element("textarea")?;
                textarea.set_attribute("value", text)?;
                textarea.set_attribute("style", "position: absolute; left: -9999px;")?;

                let textarea_clone = textarea.clone();
                if let Some(body) = document.body() {
                    body.append_child(&textarea_clone)?;

                    // 选择文本并复制
                    if let Ok(html_element) =
                        textarea_clone.dyn_into::<web_sys::HtmlTextAreaElement>()
                    {
                        html_element.select();
                        if let Ok(result) =
                            js_sys::Reflect::get(&document, &JsValue::from_str("execCommand"))
                        {
                            if let Ok(func) = result.dyn_into::<js_sys::Function>() {
                                let _ = func.call1(&document, &JsValue::from_str("copy"))?;
                            }
                        }
                    }

                    // 移除临时元素
                    body.remove_child(&textarea.clone())?;

                    // 添加到历史记录
                    self.add_to_history(text.to_string());
                    return Ok(());
                }
            }
        }

        Err(JsValue::from_str("Failed to copy text"))
    }

    /// 添加到复制历史
    fn add_to_history(&mut self, text: String) {
        // 避免重复
        if let Some(last) = self.copy_history.last() {
            if last == &text {
                return;
            }
        }

        self.copy_history.push(text);
        if self.copy_history.len() > self.max_history {
            self.copy_history.remove(0);
        }
    }

    /// 获取复制历史
    pub fn get_history(&self) -> &[String] {
        &self.copy_history
    }

    /// 清除历史
    pub fn clear_history(&mut self) {
        self.copy_history.clear();
    }
}

/// 选择工具栏组件属性
#[derive(Props, Clone, PartialEq)]
pub struct SelectionToolbarProps {
    pub selection: Option<TextRange>,
    pub position: ToolbarPosition,
    pub on_copy: Option<EventHandler<String>>,
    pub on_clear: Option<EventHandler<()>>,
    #[props(default = true)]
    pub show_copy: bool,
    #[props(default = true)]
    pub show_clear: bool,
    #[props(default = HashMap::new())]
    pub custom_actions: HashMap<String, EventHandler<String>>,
}

/// 选择工具栏组件
#[component]
pub fn SelectionToolbar(props: SelectionToolbarProps) -> Element {
    let SelectionToolbarProps {
        selection,
        position,
        on_copy,
        on_clear,
        show_copy,
        show_clear,
        custom_actions,
    } = props;

    let mut copy_manager = use_signal(|| CopyManager::new());
    let is_visible = selection.as_ref().map_or(false, |s| s.is_valid());

    if !is_visible {
        return rsx! { div { style: "display: none;" } };
    }

    let position_class = match position {
        ToolbarPosition::Auto => "ant-typography-selection-toolbar-auto",
        ToolbarPosition::Top => "ant-typography-selection-toolbar-top",
        ToolbarPosition::Bottom => "ant-typography-selection-toolbar-bottom",
        ToolbarPosition::Left => "ant-typography-selection-toolbar-left",
        ToolbarPosition::Right => "ant-typography-selection-toolbar-right",
        ToolbarPosition::Fixed => "ant-typography-selection-toolbar-fixed",
    };

    let selection_clone = selection.clone();
    let handle_copy = move |_| {
        if let Some(range) = &selection_clone {
            let text = range.text.clone();

            let text_clone = text.clone();
            // 异步复制到剪贴板
            spawn(async move {
                let mut manager = copy_manager.write();
                if let Err(e) = manager.copy_to_clipboard(&text_clone).await {
                    web_sys::console::error_1(&format!("Copy failed: {:?}", e).into());
                }
            });

            // 触发回调
            if let Some(callback) = &on_copy {
                callback.call(text);
            }
        }
    };

    let handle_clear = move |_| {
        if let Some(callback) = &on_clear {
            callback.call(());
        }
    };

    let selection_clone = selection.clone();

    rsx! {
        div {
            class: format!("ant-typography-selection-toolbar {}", position_class),

            if show_copy {
                button {
                    class: "ant-typography-toolbar-btn ant-typography-copy-btn",
                    onclick: handle_copy,
                    title: "复制",
                    "📋"
                }
            }

            if show_clear {
                button {
                    class: "ant-typography-toolbar-btn ant-typography-clear-btn",
                    onclick: handle_clear,
                    title: "清除选择",
                    "✕"
                }
            }

            // 自定义操作按钮
            for (name, handler) in custom_actions {
                button {
                    key: "{name}",
                    class: "ant-typography-toolbar-btn ant-typography-custom-btn",
                    onclick: move |_| {
                        if let Some(range) = &selection_clone {
                            handler.call(range.text.clone());
                        }
                    },
                    title: "{name}",
                    "{name}"
                }
            }
        }
    }
}

/// 文本选择 Hook
pub fn use_text_selection(
    text: String,
    config: SelectionConfig,
) -> (
    Signal<Option<TextRange>>,
    impl Fn(usize, usize),
    impl Fn(),
    impl Fn(SelectionMode),
) {
    let selector = use_signal(|| TextSelector::new(config.clone()));
    let current_selection = use_signal(|| None::<TextRange>);
    let copy_manager = use_signal(|| CopyManager::new());

    // 设置选择范围
    let set_selection = move |start: usize, end: usize| {
        if start <= end && end <= text.len() {
            let selected_text = text.chars().skip(start).take(end - start).collect();
            let range = TextRange::new(start, end, selected_text);

            selector.write().set_selection(range.clone());
            current_selection.set(Some(range));
        }
    };

    // 清除选择
    let clear_selection = move || {
        selector.write().clear_selection();
        current_selection.set(None);
    };

    // 按模式选择
    let select_by_mode = move |mode: SelectionMode| {
        if let Some(window) = window() {
            if let Some(selection) = window.document().get_selection().ok().flatten() {
                if let Ok(range_count) = selection.range_count() {
                    if range_count > 0 {
                        if let Ok(range) = selection.get_range_at(0) {
                            let start = range.start_offset() as usize;
                            let end = range.end_offset() as usize;

                            if let Some(selected_range) =
                                selector.write().select_by_mode(&text, start, mode)
                            {
                                current_selection.set(Some(selected_range));
                            }
                        }
                    }
                }
            }
        }
    };

    (
        current_selection,
        set_selection,
        clear_selection,
        select_by_mode,
    )
}

/// 选择样式生成器
pub struct SelectionStyleGenerator;

impl SelectionStyleGenerator {
    /// 生成选择相关样式
    pub fn selection_styles(config: &SelectionConfig) -> String {
        format!(
            r#"
            /* 文本选择样式 */
            .ant-typography-selectable {{
                user-select: text;
                -webkit-user-select: text;
                -moz-user-select: text;
                -ms-user-select: text;
            }}

            .ant-typography-unselectable {{
                user-select: none;
                -webkit-user-select: none;
                -moz-user-select: none;
                -ms-user-select: none;
            }}

            .ant-typography-selection-highlight {{
                background-color: {};
                color: white;
                border-radius: 2px;
                padding: 0 2px;
            }}

            /* 选择工具栏样式 */
            .ant-typography-selection-toolbar {{
                position: absolute;
                background: white;
                border: 1px solid #d9d9d9;
                border-radius: 6px;
                box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
                padding: 4px;
                display: flex;
                gap: 4px;
                z-index: 1000;
                opacity: 0;
                visibility: hidden;
                transition: opacity 0.2s, visibility 0.2s;
            }}

            .ant-typography-selection-toolbar.visible {{
                opacity: 1;
                visibility: visible;
            }}

            .ant-typography-toolbar-btn {{
                border: none;
                background: transparent;
                padding: 4px 8px;
                border-radius: 4px;
                cursor: pointer;
                font-size: 12px;
                transition: background-color 0.2s;
            }}

            .ant-typography-toolbar-btn:hover {{
                background-color: #f5f5f5;
            }}

            .ant-typography-copy-btn:hover {{
                background-color: #e6f7ff;
                color: #1890ff;
            }}

            .ant-typography-clear-btn:hover {{
                background-color: #fff2e8;
                color: #fa8c16;
            }}

            /* 工具栏位置样式 */
            .ant-typography-selection-toolbar-auto {{
                transform: translateY(-100%);
                margin-top: -8px;
            }}

            .ant-typography-selection-toolbar-top {{
                transform: translateY(-100%);
                margin-top: -8px;
            }}

            .ant-typography-selection-toolbar-bottom {{
                margin-top: 8px;
            }}

            .ant-typography-selection-toolbar-left {{
                transform: translateX(-100%);
                margin-left: -8px;
            }}

            .ant-typography-selection-toolbar-right {{
                margin-left: 8px;
            }}

            .ant-typography-selection-toolbar-fixed {{
                position: fixed;
                top: 20px;
                right: 20px;
            }}
            "#,
            config.highlight_color
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_range() {
        let range = TextRange::new(0, 5, "Hello".to_string());
        assert!(range.is_valid());
        assert_eq!(range.length(), 5);
        assert!(!range.is_empty());
        assert!(range.contains(2));
        assert!(!range.contains(6));
    }

    #[test]
    fn test_text_range_merge() {
        let range1 = TextRange::new(0, 5, "Hello".to_string());
        let range2 = TextRange::new(6, 11, "World".to_string());
        let merged = range1.merge(&range2);

        assert_eq!(merged.start, 0);
        assert_eq!(merged.end, 11);
        assert_eq!(merged.text, "HelloWorld");
    }

    #[test]
    fn test_text_selector_word_selection() {
        let selector = TextSelector::default();
        let text = "Hello World Test";

        if let Some(range) = selector.select_word(text, 6) {
            assert_eq!(range.text, "World");
            assert_eq!(range.start, 6);
            assert_eq!(range.end, 11);
        } else {
            panic!("Word selection failed");
        }
    }

    #[test]
    fn test_text_selector_line_selection() {
        let selector = TextSelector::default();
        let text = "Line 1\nLine 2\nLine 3";

        if let Some(range) = selector.select_line(text, 8) {
            assert_eq!(range.text, "Line 2");
        } else {
            panic!("Line selection failed");
        }
    }

    #[test]
    fn test_copy_manager() {
        let mut manager = CopyManager::new();
        manager.add_to_history("Test 1".to_string());
        manager.add_to_history("Test 2".to_string());
        manager.add_to_history("Test 1".to_string()); // 重复，应该被忽略

        assert_eq!(manager.get_history().len(), 2);
        assert_eq!(manager.get_history()[0], "Test 1");
        assert_eq!(manager.get_history()[1], "Test 2");
    }
}

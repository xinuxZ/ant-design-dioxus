//! Typography 文本搜索功能模块
//!
//! 提供文本搜索、高亮显示等功能

use dioxus::prelude::*;
use std::collections::HashMap;

/// 搜索结果
#[derive(Debug, Clone, PartialEq)]
pub struct SearchResult {
    pub start: usize,
    pub end: usize,
    pub matched_text: String,
}

/// 搜索选项
#[derive(Debug, Clone, PartialEq)]
pub struct SearchOptions {
    pub case_sensitive: bool,
    pub whole_word: bool,
    pub regex: bool,
    pub highlight_all: bool,
}

impl Default for SearchOptions {
    fn default() -> Self {
        Self {
            case_sensitive: false,
            whole_word: false,
            regex: false,
            highlight_all: true,
        }
    }
}

/// 文本搜索器
pub struct TextSearcher {
    text: String,
    options: SearchOptions,
}

impl TextSearcher {
    pub fn new(text: String, options: SearchOptions) -> Self {
        Self { text, options }
    }

    /// 搜索文本
    pub fn search(&self, query: &str) -> Vec<SearchResult> {
        if query.is_empty() {
            return vec![];
        }

        let search_text = if self.options.case_sensitive {
            self.text.clone()
        } else {
            self.text.to_lowercase()
        };

        let search_query = if self.options.case_sensitive {
            query.to_string()
        } else {
            query.to_lowercase()
        };

        if self.options.regex {
            self.regex_search(&search_text, &search_query)
        } else if self.options.whole_word {
            self.whole_word_search(&search_text, &search_query)
        } else {
            self.simple_search(&search_text, &search_query)
        }
    }

    /// 简单文本搜索
    fn simple_search(&self, text: &str, query: &str) -> Vec<SearchResult> {
        let mut results = Vec::new();
        let mut start = 0;

        while let Some(pos) = text[start..].find(query) {
            let actual_start = start + pos;
            let actual_end = actual_start + query.len();

            results.push(SearchResult {
                start: actual_start,
                end: actual_end,
                matched_text: self.text[actual_start..actual_end].to_string(),
            });

            start = actual_start + 1;
        }

        results
    }

    /// 整词搜索
    fn whole_word_search(&self, text: &str, query: &str) -> Vec<SearchResult> {
        let mut results = Vec::new();
        let words: Vec<&str> = text.split_whitespace().collect();
        let mut char_pos = 0;

        for word in words {
            // 找到当前单词在原文本中的位置
            if let Some(word_start) = text[char_pos..].find(word) {
                let actual_start = char_pos + word_start;

                if word == query {
                    results.push(SearchResult {
                        start: actual_start,
                        end: actual_start + word.len(),
                        matched_text: self.text[actual_start..actual_start + word.len()]
                            .to_string(),
                    });
                }

                char_pos = actual_start + word.len();
            }
        }

        results
    }

    /// 正则表达式搜索
    fn regex_search(&self, text: &str, pattern: &str) -> Vec<SearchResult> {
        // 简化的正则表达式实现，实际项目中应使用 regex crate
        // 这里只实现基本的通配符支持
        if pattern.contains('*') {
            self.wildcard_search(text, pattern)
        } else {
            self.simple_search(text, pattern)
        }
    }

    /// 通配符搜索
    fn wildcard_search(&self, text: &str, pattern: &str) -> Vec<SearchResult> {
        let mut results = Vec::new();

        // 简单的通配符实现：将 * 替换为任意字符匹配
        let parts: Vec<&str> = pattern.split('*').collect();

        if parts.len() == 2 {
            let prefix = parts[0];
            let suffix = parts[1];

            let mut start = 0;
            while let Some(prefix_pos) = text[start..].find(prefix) {
                let prefix_start = start + prefix_pos;
                let search_start = prefix_start + prefix.len();

                if let Some(suffix_pos) = text[search_start..].find(suffix) {
                    let suffix_start = search_start + suffix_pos;
                    let match_end = suffix_start + suffix.len();

                    results.push(SearchResult {
                        start: prefix_start,
                        end: match_end,
                        matched_text: self.text[prefix_start..match_end].to_string(),
                    });

                    start = match_end;
                } else {
                    break;
                }
            }
        }

        results
    }

    /// 生成高亮文本
    pub fn highlight_text(&self, query: &str, highlight_class: &str) -> String {
        let results = self.search(query);

        if results.is_empty() {
            return self.text.clone();
        }

        let mut highlighted = String::new();
        let mut last_end = 0;

        for result in results {
            // 添加匹配前的文本
            highlighted.push_str(&self.text[last_end..result.start]);

            // 添加高亮的匹配文本
            highlighted.push_str(&format!(
                "<span class=\"{}\">{}</span>",
                highlight_class,
                &self.text[result.start..result.end]
            ));

            last_end = result.end;
        }

        // 添加剩余文本
        highlighted.push_str(&self.text[last_end..]);

        highlighted
    }
}

/// 搜索高亮样式
pub struct SearchHighlightStyles;

impl SearchHighlightStyles {
    /// 默认高亮样式
    pub fn default_highlight() -> String {
        ".text-search-highlight {
            background-color: #ffe58f;
            color: #000;
            padding: 0 2px;
            border-radius: 2px;
            font-weight: 500;
        }"
        .to_string()
    }

    /// 当前匹配项高亮样式
    pub fn current_highlight() -> String {
        ".text-search-current {
            background-color: #ff9c6e;
            color: #000;
            padding: 0 2px;
            border-radius: 2px;
            font-weight: 600;
            box-shadow: 0 0 0 1px #ff7a45;
        }"
        .to_string()
    }

    /// 搜索框样式
    pub fn search_box() -> String {
        ".text-search-box {
            display: inline-flex;
            align-items: center;
            gap: 8px;
            padding: 4px 8px;
            border: 1px solid #d9d9d9;
            border-radius: 6px;
            background-color: #fff;
            font-size: 12px;
        }

        .text-search-input {
            border: none;
            outline: none;
            background: transparent;
            font-size: inherit;
            width: 120px;
        }

        .text-search-controls {
            display: flex;
            gap: 4px;
        }

        .text-search-button {
            padding: 2px 6px;
            border: 1px solid #d9d9d9;
            border-radius: 4px;
            background: #fff;
            cursor: pointer;
            font-size: 11px;
            color: #666;
        }

        .text-search-button:hover {
            background: #f5f5f5;
            border-color: #40a9ff;
        }

        .text-search-button.active {
            background: #1890ff;
            border-color: #1890ff;
            color: #fff;
        }

        .text-search-counter {
            font-size: 11px;
            color: #8c8c8c;
            white-space: nowrap;
        }"
        .to_string()
    }
}

/// 文本搜索 Hook
pub fn use_text_search(
    text: String,
) -> (
    Signal<String>,
    Signal<SearchOptions>,
    Signal<Vec<SearchResult>>,
) {
    let query = use_signal(|| String::new());
    let options = use_signal(|| SearchOptions::default());
    let mut results = use_signal(|| Vec::new());

    // 当查询或选项改变时更新结果
    use_effect(move || {
        let searcher = TextSearcher::new(text.clone(), options.read().clone());
        let search_results = searcher.search(&query.read());
        results.set(search_results);
    });

    (query, options, results)
}

/// 搜索组件属性
#[derive(Props, Clone, PartialEq)]
pub struct SearchBoxProps {
    pub query: Signal<String>,
    pub options: Signal<SearchOptions>,
    pub results: Signal<Vec<SearchResult>>,
    #[props(default = 0)]
    pub current_index: usize,
    #[props(default)]
    pub on_search: Option<EventHandler<String>>,
    #[props(default)]
    pub on_next: Option<EventHandler<()>>,
    #[props(default)]
    pub on_previous: Option<EventHandler<()>>,
}

/// 搜索框组件
#[component]
pub fn SearchBox(props: SearchBoxProps) -> Element {
    let SearchBoxProps {
        mut query,
        mut options,
        results,
        current_index,
        on_search,
        on_next,
        on_previous,
    } = props;

    rsx! {
        div {
            class: "text-search-box",
            input {
                class: "text-search-input",
                r#type: "text",
                placeholder: "搜索文本...",
                value: "{query.read()}",
                oninput: move |evt| {
                    query.set(evt.value());
                    if let Some(handler) = &on_search {
                        handler.call(evt.value());
                    }
                },
            }

            div {
                class: "text-search-controls",
                button {
                    class: if options.read().case_sensitive { "text-search-button active" } else { "text-search-button" },
                    title: "区分大小写",
                    onclick: move |_| {
                        let mut opts = options.read().clone();
                        opts.case_sensitive = !opts.case_sensitive;
                        options.set(opts);
                    },
                    "Aa"
                }

                button {
                    class: if options.read().whole_word { "text-search-button active" } else { "text-search-button" },
                    title: "全词匹配",
                    onclick: move |_| {
                        let mut opts = options.read().clone();
                        opts.whole_word = !opts.whole_word;
                        options.set(opts);
                    },
                    "Ab"
                }

                button {
                    class: if options.read().regex { "text-search-button active" } else { "text-search-button" },
                    title: "正则表达式",
                    onclick: move |_| {
                        let mut opts = options.read().clone();
                        opts.regex = !opts.regex;
                        options.set(opts);
                    },
                    ".*"
                }
            }

            if !results.read().is_empty() {
                div {
                    class: "text-search-counter",
                    "{current_index + 1}/{results.read().len()}"
                }

                button {
                    class: "text-search-button",
                    title: "上一个",
                    onclick: move |_| {
                        if let Some(handler) = &on_previous {
                            handler.call(());
                        }
                    },
                    "↑"
                }

                button {
                    class: "text-search-button",
                    title: "下一个",
                    onclick: move |_| {
                        if let Some(handler) = &on_next {
                            handler.call(());
                        }
                    },
                    "↓"
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_search() {
        let searcher = TextSearcher::new(
            "Hello world, this is a test world".to_string(),
            SearchOptions::default(),
        );

        let results = searcher.search("world");
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].start, 6);
        assert_eq!(results[0].end, 11);
        assert_eq!(results[1].start, 29);
        assert_eq!(results[1].end, 34);
    }

    #[test]
    fn test_case_sensitive_search() {
        let searcher = TextSearcher::new(
            "Hello World".to_string(),
            SearchOptions {
                case_sensitive: true,
                ..Default::default()
            },
        );

        let results = searcher.search("world");
        assert_eq!(results.len(), 0);

        let results = searcher.search("World");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_whole_word_search() {
        let searcher = TextSearcher::new(
            "Hello world wonderful".to_string(),
            SearchOptions {
                whole_word: true,
                ..Default::default()
            },
        );

        let results = searcher.search("world");
        assert_eq!(results.len(), 1);

        let results = searcher.search("wor");
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_highlight_text() {
        let searcher = TextSearcher::new("Hello world".to_string(), SearchOptions::default());

        let highlighted = searcher.highlight_text("world", "highlight");
        assert_eq!(highlighted, "Hello <span class=\"highlight\">world</span>");
    }
}

//! Typography RTL（Right-to-Left）支持模块
//!
//! 提供阿拉伯语、希伯来语等从右到左书写语言的支持

use dioxus::prelude::*;
use std::collections::HashSet;

/// RTL 语言代码
static RTL_LANGUAGES: &[&str] = &[
    "ar",  // 阿拉伯语
    "he",  // 希伯来语
    "fa",  // 波斯语
    "ur",  // 乌尔都语
    "yi",  // 意第绪语
    "iw",  // 希伯来语（旧代码）
    "ji",  // 意第绪语（旧代码）
    "arc", // 阿拉米语
    "bcc", // 俾路支语
    "bqi", // 巴赫蒂亚里语
    "ckb", // 中库尔德语
    "dv",  // 迪维希语
    "glk", // 吉拉基语
    "ku",  // 库尔德语
    "mzn", // 马赞德兰语
    "pnb", // 西旁遮普语
    "ps",  // 普什图语
    "sd",  // 信德语
    "ug",  // 维吾尔语
];

/// 文本方向
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextDirection {
    /// 从左到右
    Ltr,
    /// 从右到左
    Rtl,
    /// 自动检测
    Auto,
}

impl Default for TextDirection {
    fn default() -> Self {
        Self::Auto
    }
}

impl ToString for TextDirection {
    fn to_string(&self) -> String {
        match self {
            Self::Ltr => "ltr".to_string(),
            Self::Rtl => "rtl".to_string(),
            Self::Auto => "auto".to_string(),
        }
    }
}

/// RTL 检测器
pub struct RtlDetector {
    rtl_languages: HashSet<String>,
}

impl Default for RtlDetector {
    fn default() -> Self {
        let rtl_languages = RTL_LANGUAGES.iter().map(|&lang| lang.to_string()).collect();

        Self { rtl_languages }
    }
}

impl RtlDetector {
    /// 创建新的 RTL 检测器
    pub fn new() -> Self {
        Self::default()
    }

    /// 根据语言代码检测是否为 RTL
    pub fn is_rtl_language(&self, language: &str) -> bool {
        // 提取主要语言代码（忽略地区代码）
        let primary_lang = language
            .split('-')
            .next()
            .unwrap_or(language)
            .to_lowercase();
        self.rtl_languages.contains(&primary_lang)
    }

    /// 根据文本内容检测方向
    pub fn detect_text_direction(&self, text: &str) -> TextDirection {
        if text.is_empty() {
            return TextDirection::Ltr;
        }

        let mut rtl_chars = 0;
        let mut ltr_chars = 0;
        let mut total_directional_chars = 0;

        for ch in text.chars() {
            match self.get_char_direction(ch) {
                CharDirection::Rtl => {
                    rtl_chars += 1;
                    total_directional_chars += 1;
                }
                CharDirection::Ltr => {
                    ltr_chars += 1;
                    total_directional_chars += 1;
                }
                CharDirection::Neutral => {}
            }
        }

        if total_directional_chars == 0 {
            return TextDirection::Ltr;
        }

        // 如果 RTL 字符占比超过 30%，则认为是 RTL 文本
        if rtl_chars as f32 / total_directional_chars as f32 > 0.3 {
            TextDirection::Rtl
        } else {
            TextDirection::Ltr
        }
    }

    /// 获取字符的方向性
    fn get_char_direction(&self, ch: char) -> CharDirection {
        let code = ch as u32;

        // 阿拉伯语字符范围
        if (0x0600..=0x06FF).contains(&code)
            || (0x0750..=0x077F).contains(&code)
            || (0x08A0..=0x08FF).contains(&code)
            || (0xFB50..=0xFDFF).contains(&code)
            || (0xFE70..=0xFEFF).contains(&code)
        {
            return CharDirection::Rtl;
        }

        // 希伯来语字符范围
        if (0x0590..=0x05FF).contains(&code) {
            return CharDirection::Rtl;
        }

        // 拉丁字母
        if (0x0041..=0x005A).contains(&code) || // A-Z
           (0x0061..=0x007A).contains(&code)
        {
            // a-z
            return CharDirection::Ltr;
        }

        // 数字
        if (0x0030..=0x0039).contains(&code) {
            return CharDirection::Ltr;
        }

        // 其他欧洲语言字符
        if (0x00C0..=0x024F).contains(&code) {
            return CharDirection::Ltr;
        }

        CharDirection::Neutral
    }
}

/// 字符方向
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CharDirection {
    Ltr,
    Rtl,
    Neutral,
}

/// RTL 样式生成器
pub struct RtlStyleGenerator;

impl RtlStyleGenerator {
    /// 生成 RTL 基础样式
    pub fn base_rtl_styles() -> String {
        r#"
        /* RTL 基础样式 */
        .ant-typography[dir="rtl"] {
            direction: rtl;
            text-align: right;
        }

        .ant-typography[dir="ltr"] {
            direction: ltr;
            text-align: left;
        }

        /* RTL 文本对齐 */
        .ant-typography-rtl {
            direction: rtl;
            text-align: right;
            unicode-bidi: embed;
        }

        .ant-typography-ltr {
            direction: ltr;
            text-align: left;
            unicode-bidi: embed;
        }

        /* RTL 标题样式 */
        .ant-typography h1[dir="rtl"],
        .ant-typography h2[dir="rtl"],
        .ant-typography h3[dir="rtl"],
        .ant-typography h4[dir="rtl"],
        .ant-typography h5[dir="rtl"] {
            text-align: right;
        }

        /* RTL 段落样式 */
        .ant-typography p[dir="rtl"] {
            text-align: right;
        }

        /* RTL 链接样式 */
        .ant-typography a[dir="rtl"] {
            direction: rtl;
        }

        /* RTL 操作按钮位置调整 */
        .ant-typography[dir="rtl"] .ant-typography-copy,
        .ant-typography[dir="rtl"] .ant-typography-edit {
            margin-left: 0;
            margin-right: 4px;
        }

        /* RTL 编辑模式样式 */
        .ant-typography[dir="rtl"] .ant-typography-edit-content {
            text-align: right;
        }

        /* RTL 省略号位置 */
        .ant-typography[dir="rtl"] .ant-typography-ellipsis {
            text-align: right;
        }

        /* RTL 展开按钮位置 */
        .ant-typography[dir="rtl"] .ant-typography-expand {
            margin-left: 0;
            margin-right: 4px;
        }

        /* 双向文本支持 */
        .ant-typography-bidi {
            unicode-bidi: bidi-override;
        }

        .ant-typography-isolate {
            unicode-bidi: isolate;
        }

        .ant-typography-plaintext {
            unicode-bidi: plaintext;
        }
        "#
        .to_string()
    }

    /// 生成特定方向的样式类
    pub fn direction_class(direction: TextDirection) -> String {
        match direction {
            TextDirection::Rtl => "ant-typography-rtl".to_string(),
            TextDirection::Ltr => "ant-typography-ltr".to_string(),
            TextDirection::Auto => "ant-typography-auto".to_string(),
        }
    }

    /// 生成 dir 属性值
    pub fn dir_attribute(direction: TextDirection) -> String {
        direction.to_string()
    }
}

/// RTL 上下文
#[derive(Debug, Clone, PartialEq)]
pub struct RtlContext {
    pub direction: TextDirection,
    pub language: String,
    pub auto_detect: bool,
}

impl Default for RtlContext {
    fn default() -> Self {
        Self {
            direction: TextDirection::Auto,
            language: "en".to_string(),
            auto_detect: true,
        }
    }
}

/// RTL Hook
pub fn use_rtl(
    language: Option<String>,
    text: Option<String>,
) -> (Signal<RtlContext>, Signal<TextDirection>) {
    let mut context = use_signal(|| RtlContext::default());
    let mut computed_direction = use_signal(|| TextDirection::Ltr);
    let detector = use_signal(|| RtlDetector::new());

    // 当语言或文本改变时重新计算方向
    use_effect(move || {
        let mut ctx = context.read().clone();

        if let Some(lang) = &language {
            ctx.language = lang.clone();
        }

        let direction = if ctx.auto_detect {
            if detector.read().is_rtl_language(&ctx.language) {
                TextDirection::Rtl
            } else if let Some(text_content) = &text {
                detector.read().detect_text_direction(text_content)
            } else {
                TextDirection::Ltr
            }
        } else {
            ctx.direction
        };

        computed_direction.set(direction);
        context.set(ctx);
    });

    (context, computed_direction)
}

/// RTL 提供者组件属性
#[derive(Props, Clone, PartialEq)]
pub struct RtlProviderProps {
    #[props(default = TextDirection::Auto)]
    pub direction: TextDirection,
    #[props(default = "en".to_string())]
    pub language: String,
    #[props(default = true)]
    pub auto_detect: bool,
    pub children: Element,
}

/// RTL 提供者组件
#[component]
pub fn RtlProvider(props: RtlProviderProps) -> Element {
    let RtlProviderProps {
        direction,
        language,
        auto_detect,
        children,
    } = props;

    let context = use_signal(move || RtlContext {
        direction,
        language: language.clone(),
        auto_detect,
    });

    use_context_provider(|| context);

    rsx! {
        div {
            class: "ant-typography-rtl-provider",
            {children}
        }
    }
}

/// 获取 RTL 上下文
pub fn use_rtl_context() -> Signal<RtlContext> {
    use_context::<Signal<RtlContext>>()
}

/// 文本方向工具函数
pub struct TextDirectionUtils;

impl TextDirectionUtils {
    /// 检测文本是否包含 RTL 字符
    pub fn contains_rtl_chars(text: &str) -> bool {
        let detector = RtlDetector::new();
        for ch in text.chars() {
            if matches!(detector.get_char_direction(ch), CharDirection::Rtl) {
                return true;
            }
        }
        false
    }

    /// 检测文本是否为混合方向（包含 LTR 和 RTL 字符）
    pub fn is_mixed_direction(text: &str) -> bool {
        let detector = RtlDetector::new();
        let mut has_ltr = false;
        let mut has_rtl = false;

        for ch in text.chars() {
            match detector.get_char_direction(ch) {
                CharDirection::Ltr => has_ltr = true,
                CharDirection::Rtl => has_rtl = true,
                CharDirection::Neutral => {}
            }

            if has_ltr && has_rtl {
                return true;
            }
        }

        false
    }

    /// 为混合方向文本添加方向标记
    pub fn wrap_mixed_text(text: &str) -> String {
        if Self::is_mixed_direction(text) {
            format!("<span dir=\"auto\">{}</span>", text)
        } else {
            text.to_string()
        }
    }

    /// 获取文本的主要方向
    pub fn get_primary_direction(text: &str) -> TextDirection {
        let detector = RtlDetector::new();
        detector.detect_text_direction(text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rtl_language_detection() {
        let detector = RtlDetector::new();

        assert!(detector.is_rtl_language("ar"));
        assert!(detector.is_rtl_language("he"));
        assert!(detector.is_rtl_language("fa"));
        assert!(detector.is_rtl_language("ar-SA"));
        assert!(detector.is_rtl_language("he-IL"));

        assert!(!detector.is_rtl_language("en"));
        assert!(!detector.is_rtl_language("zh"));
        assert!(!detector.is_rtl_language("fr"));
    }

    #[test]
    fn test_text_direction_detection() {
        let detector = RtlDetector::new();

        // 英文文本
        assert_eq!(
            detector.detect_text_direction("Hello World"),
            TextDirection::Ltr
        );

        // 阿拉伯文文本
        assert_eq!(
            detector.detect_text_direction("مرحبا بالعالم"),
            TextDirection::Rtl
        );

        // 混合文本（主要是英文）
        assert_eq!(
            detector.detect_text_direction("Hello مرحبا World"),
            TextDirection::Ltr
        );

        // 空文本
        assert_eq!(detector.detect_text_direction(""), TextDirection::Ltr);
    }

    #[test]
    fn test_mixed_direction_detection() {
        assert!(TextDirectionUtils::is_mixed_direction("Hello مرحبا"));
        assert!(!TextDirectionUtils::is_mixed_direction("Hello World"));
        assert!(!TextDirectionUtils::is_mixed_direction("مرحبا بالعالم"));
    }

    #[test]
    fn test_rtl_chars_detection() {
        assert!(TextDirectionUtils::contains_rtl_chars("مرحبا"));
        assert!(TextDirectionUtils::contains_rtl_chars("Hello مرحبا"));
        assert!(!TextDirectionUtils::contains_rtl_chars("Hello World"));
    }
}

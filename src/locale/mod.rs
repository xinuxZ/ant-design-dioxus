//! 国际化模块
//!
//! 提供多语言支持，包括语言包管理、文本翻译、日期时间格式化等功能。
//! 支持动态切换语言，并提供 React Context 风格的 API。

use chrono::{DateTime, Local};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use chrono::Datelike;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Intl)]
    type DateTimeFormat;

    #[wasm_bindgen(constructor, js_namespace = Intl)]
    fn new(locale: &str, options: &JsValue) -> DateTimeFormat;

    #[wasm_bindgen(method)]
    fn format(this: &DateTimeFormat, date: &JsValue) -> String;

    #[wasm_bindgen(js_namespace = Intl)]
    type RelativeTimeFormat;

    #[wasm_bindgen(constructor, js_namespace = Intl)]
    fn new_relative(locale: &str, options: &JsValue) -> RelativeTimeFormat;

    #[wasm_bindgen(method)]
    fn format_relative(this: &RelativeTimeFormat, value: f64, unit: &str) -> String;
}

/// 支持的语言类型
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Locale {
    /// 简体中文
    ZhCN,
    /// 繁体中文
    ZhTW,
    /// 英语
    En,
    /// 日语
    Ja,
    /// 韩语
    Ko,
    /// 法语
    Fr,
    /// 德语
    De,
    /// 西班牙语
    Es,
    /// 俄语
    Ru,
    /// 意大利语
    It,
    /// 葡萄牙语
    Pt,
    /// 阿拉伯语
    Ar,
    /// 自定义语言
    Custom(String),
}

impl Locale {
    /// 获取语言代码
    pub fn code(&self) -> &str {
        match self {
            Locale::ZhCN => "zh-CN",
            Locale::ZhTW => "zh-TW",
            Locale::En => "en",
            Locale::Ja => "ja",
            Locale::Ko => "ko",
            Locale::Fr => "fr",
            Locale::De => "de",
            Locale::Es => "es",
            Locale::Ru => "ru",
            Locale::It => "it",
            Locale::Pt => "pt",
            Locale::Ar => "ar",
            Locale::Custom(code) => code,
        }
    }

    /// 获取语言名称
    pub fn name(&self) -> &str {
        match self {
            Locale::ZhCN => "简体中文",
            Locale::ZhTW => "繁體中文",
            Locale::En => "English",
            Locale::Ja => "日本語",
            Locale::Ko => "한국어",
            Locale::Fr => "Français",
            Locale::De => "Deutsch",
            Locale::Es => "Español",
            Locale::Ru => "Русский",
            Locale::It => "Italiano",
            Locale::Pt => "Português",
            Locale::Ar => "العربية",
            Locale::Custom(name) => name,
        }
    }

    /// 是否为从右到左的语言
    pub fn is_rtl(&self) -> bool {
        matches!(self, Locale::Ar)
    }
}

impl fmt::Display for Locale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code())
    }
}

impl Locale {
    /// 从语言代码创建 Locale
    pub fn from_code(code: &str) -> Self {
        match code {
            "zh-CN" | "zh-cn" => Locale::ZhCN,
            "zh-TW" | "zh-tw" => Locale::ZhTW,
            "en" => Locale::En,
            "ja" => Locale::Ja,
            "ko" => Locale::Ko,
            "fr" => Locale::Fr,
            "de" => Locale::De,
            "es" => Locale::Es,
            "ru" => Locale::Ru,
            "it" => Locale::It,
            "pt" => Locale::Pt,
            "ar" => Locale::Ar,
            _ => Locale::Custom(code.to_string()),
        }
    }
}

impl Default for Locale {
    fn default() -> Self {
        Locale::ZhCN
    }
}

/// 翻译键类型
pub type TranslationKey = &'static str;

/// 翻译值类型
pub type TranslationValue = String;

/// 语言包类型
pub type LanguagePack = HashMap<TranslationKey, TranslationValue>;

/// 国际化配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocaleConfig {
    /// 当前语言
    pub locale: Locale,
    /// 语言包
    #[serde(skip)]
    pub messages: Rc<LanguagePack>,
    /// 日期格式
    pub date_format: String,
    /// 时间格式
    pub time_format: String,
    /// 数字格式
    pub number_format: NumberFormat,
    /// 货币格式
    pub currency_format: CurrencyFormat,
}

/// 数字格式配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NumberFormat {
    /// 千位分隔符
    pub thousands_separator: String,
    /// 小数点分隔符
    pub decimal_separator: String,
    /// 小数位数
    pub decimal_places: usize,
}

/// 货币格式配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CurrencyFormat {
    /// 货币符号
    pub symbol: String,
    /// 货币代码
    pub code: String,
    /// 符号位置（前缀或后缀）
    pub symbol_position: CurrencySymbolPosition,
    /// 数字格式
    pub number_format: NumberFormat,
}

/// 货币符号位置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CurrencySymbolPosition {
    /// 前缀
    Prefix,
    /// 后缀
    Suffix,
}

impl Default for LocaleConfig {
    fn default() -> Self {
        Self {
            locale: Locale::default(),
            messages: Rc::new(get_default_messages(&Locale::default())),
            date_format: "YYYY-MM-DD".to_string(),
            time_format: "HH:mm:ss".to_string(),
            number_format: NumberFormat {
                thousands_separator: ",".to_string(),
                decimal_separator: ".".to_string(),
                decimal_places: 2,
            },
            currency_format: CurrencyFormat {
                symbol: "¥".to_string(),
                code: "CNY".to_string(),
                symbol_position: CurrencySymbolPosition::Prefix,
                number_format: NumberFormat {
                    thousands_separator: ",".to_string(),
                    decimal_separator: ".".to_string(),
                    decimal_places: 2,
                },
            },
        }
    }
}

impl LocaleConfig {
    /// 创建新的国际化配置
    pub fn new(locale: Locale) -> Self {
        let messages = get_default_messages(&locale);
        let (date_format, time_format) = get_default_datetime_format(&locale);
        let number_format = get_default_number_format(&locale);
        let currency_format = get_default_currency_format(&locale);

        Self {
            locale,
            messages: Rc::new(messages),
            date_format,
            time_format,
            number_format,
            currency_format,
        }
    }

    /// 设置自定义语言包
    pub fn with_messages(mut self, messages: LanguagePack) -> Self {
        self.messages = Rc::new(messages);
        self
    }

    /// 设置日期格式
    pub fn with_date_format(mut self, format: String) -> Self {
        self.date_format = format;
        self
    }

    /// 设置时间格式
    pub fn with_time_format(mut self, format: String) -> Self {
        self.time_format = format;
        self
    }

    /// 翻译文本
    pub fn translate(&self, key: TranslationKey) -> String {
        self.messages
            .get(key)
            .cloned()
            .unwrap_or_else(|| key.to_string())
    }

    /// 翻译文本并替换占位符
    pub fn translate_with_args(&self, key: TranslationKey, args: &[(&str, &str)]) -> String {
        let mut text = self.translate(key);
        for (placeholder, value) in args {
            text = text.replace(&format!("{{{}}}", placeholder), value);
        }
        text
    }

    /// 格式化日期时间
    pub fn format_datetime(&self, datetime: &DateTime<Local>) -> String {
        #[cfg(target_arch = "wasm32")]
        {
            use js_sys::{Date, Object, Reflect};
            use wasm_bindgen::JsValue;

            let js_date = Date::new_with_year_month_day_hr_min_sec_milli(
                datetime.year() as u32,
                (datetime.month() - 1) as i32, // JS months are 0-indexed
                datetime.day() as i32,
                datetime.hour() as i32,
                datetime.minute() as i32,
                datetime.second() as i32,
                (datetime.nanosecond() / 1_000_000) as i32,
            );

            let options = Object::new();
            let _ = Reflect::set(
                &options,
                &JsValue::from_str("year"),
                &JsValue::from_str("numeric"),
            );
            let _ = Reflect::set(
                &options,
                &JsValue::from_str("month"),
                &JsValue::from_str("2-digit"),
            );
            let _ = Reflect::set(
                &options,
                &JsValue::from_str("day"),
                &JsValue::from_str("2-digit"),
            );
            let _ = Reflect::set(
                &options,
                &JsValue::from_str("hour"),
                &JsValue::from_str("2-digit"),
            );
            let _ = Reflect::set(
                &options,
                &JsValue::from_str("minute"),
                &JsValue::from_str("2-digit"),
            );

            let formatter = DateTimeFormat::new(&self.locale.code(), &options);
            formatter.format(&js_date.into())
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            datetime.format(&self.date_format).to_string()
        }
    }

    /// 格式化日期
    pub fn format_date(&self, datetime: &DateTime<Local>) -> String {
        #[cfg(target_arch = "wasm32")]
        {
            use js_sys::{Date, Object, Reflect};
            use wasm_bindgen::JsValue;

            let js_date = Date::new_with_year_month_day(
                datetime.year() as u32,
                (datetime.month() - 1) as i32,
                datetime.day() as i32,
            );

            let options = Object::new();
            let _ = Reflect::set(
                &options,
                &JsValue::from_str("year"),
                &JsValue::from_str("numeric"),
            );
            let _ = Reflect::set(
                &options,
                &JsValue::from_str("month"),
                &JsValue::from_str("2-digit"),
            );
            let _ = Reflect::set(
                &options,
                &JsValue::from_str("day"),
                &JsValue::from_str("2-digit"),
            );

            let formatter = DateTimeFormat::new(&self.locale.code(), &options);
            formatter.format(&js_date.into())
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            datetime.format("%Y-%m-%d").to_string()
        }
    }

    /// 格式化时间
    pub fn format_time(&self, datetime: &DateTime<Local>) -> String {
        #[cfg(target_arch = "wasm32")]
        {
            use js_sys::{Date, Object, Reflect};
            use wasm_bindgen::JsValue;

            let js_date = Date::new_with_year_month_day_hr_min_sec(
                datetime.year() as u32,
                (datetime.month() - 1) as i32,
                datetime.day() as i32,
                datetime.hour() as i32,
                datetime.minute() as i32,
                datetime.second() as i32,
            );

            let options = Object::new();
            let _ = Reflect::set(
                &options,
                &JsValue::from_str("hour"),
                &JsValue::from_str("2-digit"),
            );
            let _ = Reflect::set(
                &options,
                &JsValue::from_str("minute"),
                &JsValue::from_str("2-digit"),
            );
            let _ = Reflect::set(
                &options,
                &JsValue::from_str("second"),
                &JsValue::from_str("2-digit"),
            );

            let formatter = DateTimeFormat::new(&self.locale.code(), &options);
            formatter.format(&js_date.into())
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            datetime.format(&self.time_format).to_string()
        }
    }

    /// 格式化相对时间
    pub fn format_relative_time(&self, datetime: &DateTime<Local>) -> String {
        let now = Local::now();
        let duration = now.signed_duration_since(*datetime);

        #[cfg(target_arch = "wasm32")]
        {
            use js_sys::{Object, Reflect};
            use wasm_bindgen::JsValue;

            let options = Object::new();
            let _ = Reflect::set(
                &options,
                &JsValue::from_str("numeric"),
                &JsValue::from_str("auto"),
            );

            let formatter = RelativeTimeFormat::new_relative(&self.locale.code(), &options);

            if duration.num_seconds().abs() < 60 {
                formatter.format_relative(-duration.num_seconds() as f64, "second")
            } else if duration.num_minutes().abs() < 60 {
                formatter.format_relative(-duration.num_minutes() as f64, "minute")
            } else if duration.num_hours().abs() < 24 {
                formatter.format_relative(-duration.num_hours() as f64, "hour")
            } else if duration.num_days().abs() < 30 {
                formatter.format_relative(-duration.num_days() as f64, "day")
            } else if duration.num_days().abs() < 365 {
                formatter.format_relative(-(duration.num_days() as f64 / 30.0), "month")
            } else {
                formatter.format_relative(-(duration.num_days() as f64 / 365.0), "year")
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            if duration.num_seconds().abs() < 60 {
                match self.locale {
                    Locale::ZhCN => format!("{}秒前", duration.num_seconds()),
                    _ => format!("{} seconds ago", duration.num_seconds()),
                }
            } else if duration.num_minutes().abs() < 60 {
                match self.locale {
                    Locale::ZhCN => format!("{}分钟前", duration.num_minutes()),
                    _ => format!("{} minutes ago", duration.num_minutes()),
                }
            } else if duration.num_hours().abs() < 24 {
                match self.locale {
                    Locale::ZhCN => format!("{}小时前", duration.num_hours()),
                    _ => format!("{} hours ago", duration.num_hours()),
                }
            } else {
                match self.locale {
                    Locale::ZhCN => format!("{}天前", duration.num_days()),
                    _ => format!("{} days ago", duration.num_days()),
                }
            }
        }
    }

    /// 格式化数字
    pub fn format_number(&self, number: f64) -> String {
        let formatted = format!(
            "{:.prec$}",
            number,
            prec = self.number_format.decimal_places
        );
        self.add_thousands_separator(&formatted)
    }

    /// 格式化货币
    pub fn format_currency(&self, amount: f64) -> String {
        let number = self.format_number_with_format(amount, &self.currency_format.number_format);
        match self.currency_format.symbol_position {
            CurrencySymbolPosition::Prefix => format!("{}{}", self.currency_format.symbol, number),
            CurrencySymbolPosition::Suffix => format!("{}{}", number, self.currency_format.symbol),
        }
    }

    /// 使用指定格式格式化数字
    fn format_number_with_format(&self, number: f64, format: &NumberFormat) -> String {
        let formatted = format!("{:.prec$}", number, prec = format.decimal_places);
        self.add_thousands_separator_with_format(&formatted, format)
    }

    /// 添加千位分隔符
    fn add_thousands_separator(&self, number_str: &str) -> String {
        self.add_thousands_separator_with_format(number_str, &self.number_format)
    }

    /// 使用指定格式添加千位分隔符
    fn add_thousands_separator_with_format(
        &self,
        number_str: &str,
        format: &NumberFormat,
    ) -> String {
        let parts: Vec<&str> = number_str.split(&format.decimal_separator).collect();
        let integer_part = parts[0];
        let decimal_part = parts.get(1);

        let mut result = String::new();
        let chars: Vec<char> = integer_part.chars().collect();

        for (i, ch) in chars.iter().enumerate() {
            if i > 0 && (chars.len() - i) % 3 == 0 {
                result.push_str(&format.thousands_separator);
            }
            result.push(*ch);
        }

        if let Some(decimal) = decimal_part {
            result.push_str(&format.decimal_separator);
            result.push_str(decimal);
        }

        result
    }
}

/// 国际化提供者组件属性
#[derive(Props, Clone, PartialEq)]
pub struct LocaleProviderProps {
    /// 国际化配置
    pub config: LocaleConfig,
    /// 子组件
    pub children: Element,
}

/// 国际化提供者组件
///
/// 为子组件提供国际化上下文
#[component]
pub fn LocaleProvider(props: LocaleProviderProps) -> Element {
    use_context_provider(|| Signal::new(props.config.clone()));

    rsx! {
        {props.children}
    }
}

/// 获取国际化配置的 Hook
///
/// 从上下文中获取当前的国际化配置
pub fn use_locale() -> Signal<LocaleConfig> {
    use_context::<Signal<LocaleConfig>>()
}

/// 获取翻译函数的 Hook
///
/// 返回一个翻译函数，用于翻译文本
pub fn use_translate() -> impl Fn(TranslationKey) -> String {
    let locale_config = use_context::<Signal<LocaleConfig>>();
    move |key: TranslationKey| {
        let config = locale_config.read();
        config.translate(key)
    }
}

/// 获取带参数翻译函数的 Hook
///
/// 返回一个翻译函数，支持占位符替换
pub fn use_translate_with_args() -> impl Fn(TranslationKey, &[(&str, &str)]) -> String {
    let locale_config = use_context::<Signal<LocaleConfig>>();
    move |key: TranslationKey, args: &[(&str, &str)]| {
        let config = locale_config.read();
        config.translate_with_args(key, args)
    }
}

/// 动态切换语言的Hook
pub fn use_locale_switch() -> impl FnMut(Locale) {
    let mut locale_config = use_context::<Signal<LocaleConfig>>();

    move |new_locale: Locale| {
        // 先获取需要的值，避免所有权问题
        let locale_code = new_locale.code();
        let is_rtl = new_locale.is_rtl();

        let mut config = locale_config.write();
        let messages = (*config.messages).clone();
        *config = LocaleConfig::new(new_locale.clone()).with_messages(messages);

        // 更新HTML文档的语言属性
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(document) = web_sys::window().unwrap().document() {
                if let Some(html) = document.document_element() {
                    let _ = html.set_attribute("lang", &locale_code);
                    let _ = html.set_attribute("dir", if is_rtl { "rtl" } else { "ltr" });
                }
            }
        }
    }
}

/// 获取日期时间格式化的Hook
pub fn use_datetime_format() -> impl Fn(&chrono::DateTime<chrono::Local>) -> String {
    let locale_config = use_context::<Signal<LocaleConfig>>();

    move |datetime: &chrono::DateTime<chrono::Local>| {
        let config = locale_config.read();
        config.format_datetime(datetime)
    }
}

/// 获取日期格式化的Hook
pub fn use_date_format() -> impl Fn(&chrono::DateTime<chrono::Local>) -> String {
    let locale_config = use_context::<Signal<LocaleConfig>>();

    move |datetime: &chrono::DateTime<chrono::Local>| {
        let config = locale_config.read();
        config.format_date(datetime)
    }
}

/// 获取时间格式化的Hook
pub fn use_time_format() -> impl Fn(&chrono::DateTime<chrono::Local>) -> String {
    let locale_config = use_context::<Signal<LocaleConfig>>();

    move |datetime: &chrono::DateTime<chrono::Local>| {
        let config = locale_config.read();
        config.format_time(datetime)
    }
}

/// 获取相对时间格式化的Hook
pub fn use_relative_time_format() -> impl Fn(&chrono::DateTime<chrono::Local>) -> String {
    let locale_config = use_context::<Signal<LocaleConfig>>();

    move |datetime: &chrono::DateTime<chrono::Local>| {
        let config = locale_config.read();
        config.format_relative_time(datetime)
    }
}

/// 获取数字格式化的Hook
pub fn use_number_format() -> impl Fn(f64) -> String {
    let locale_config = use_context::<Signal<LocaleConfig>>();

    move |number: f64| {
        let config = locale_config.read();
        config.format_number(number)
    }
}

/// 获取货币格式化的Hook
pub fn use_currency_format() -> impl Fn(f64) -> String {
    let locale_config = use_context::<Signal<LocaleConfig>>();

    move |amount: f64| {
        let config = locale_config.read();
        config.format_currency(amount)
    }
}

/// 检查当前语言是否为RTL的Hook
pub fn use_is_rtl() -> bool {
    let locale_config = use_context::<Signal<LocaleConfig>>();
    let config = locale_config.read();
    config.locale.is_rtl()
}

/// 获取当前语言代码的Hook
pub fn use_locale_code() -> String {
    let locale_config = use_context::<Signal<LocaleConfig>>();
    let config = locale_config.read();
    config.locale.code().to_string()
}

/// 获取当前语言名称的Hook
pub fn use_locale_name() -> String {
    let locale_config = use_context::<Signal<LocaleConfig>>();
    let config = locale_config.read();
    config.locale.name().to_string()
}

/// 获取默认语言包
fn get_default_messages(locale: &Locale) -> LanguagePack {
    let mut messages = HashMap::new();

    match locale {
        Locale::ZhCN => {
            // 基础操作
            messages.insert("ok", "确定".to_string());
            messages.insert("cancel", "取消".to_string());
            messages.insert("close", "关闭".to_string());
            messages.insert("loading", "加载中...".to_string());
            messages.insert("empty", "暂无数据".to_string());
            messages.insert("error", "错误".to_string());
            messages.insert("success", "成功".to_string());
            messages.insert("warning", "警告".to_string());
            messages.insert("info", "信息".to_string());
            messages.insert("confirm", "确认".to_string());
            messages.insert("delete", "删除".to_string());
            messages.insert("edit", "编辑".to_string());
            messages.insert("save", "保存".to_string());
            messages.insert("reset", "重置".to_string());
            messages.insert("search", "搜索".to_string());
            messages.insert("filter", "筛选".to_string());
            messages.insert("sort", "排序".to_string());
            messages.insert("refresh", "刷新".to_string());
            messages.insert("upload", "上传".to_string());
            messages.insert("download", "下载".to_string());
            messages.insert("prev", "上一页".to_string());
            messages.insert("next", "下一页".to_string());
            messages.insert("total", "共 {total} 条".to_string());
            messages.insert("page_size", "每页 {size} 条".to_string());

            // 表单相关
            messages.insert("required", "此项为必填项".to_string());
            messages.insert("invalid_email", "请输入有效的邮箱地址".to_string());
            messages.insert("invalid_url", "请输入有效的URL".to_string());
            messages.insert("invalid_phone", "请输入有效的手机号码".to_string());
            messages.insert("min_length", "至少输入 {min} 个字符".to_string());
            messages.insert("max_length", "最多输入 {max} 个字符".to_string());
            messages.insert("pattern_mismatch", "输入格式不正确".to_string());

            // 日期时间
            messages.insert("today", "今天".to_string());
            messages.insert("yesterday", "昨天".to_string());
            messages.insert("tomorrow", "明天".to_string());
            messages.insert("this_week", "本周".to_string());
            messages.insert("last_week", "上周".to_string());
            messages.insert("this_month", "本月".to_string());
            messages.insert("last_month", "上月".to_string());
            messages.insert("this_year", "今年".to_string());
            messages.insert("last_year", "去年".to_string());
            messages.insert("select_date", "选择日期".to_string());
            messages.insert("select_time", "选择时间".to_string());
            messages.insert("start_date", "开始日期".to_string());
            messages.insert("end_date", "结束日期".to_string());

            // 表格相关
            messages.insert("select_all", "全选".to_string());
            messages.insert("select_invert", "反选".to_string());
            messages.insert("select_none", "取消选择".to_string());
            messages.insert("sort_asc", "升序".to_string());
            messages.insert("sort_desc", "降序".to_string());
            messages.insert("filter_menu", "筛选菜单".to_string());
            messages.insert("filter_reset", "重置".to_string());
            messages.insert("filter_confirm", "确定".to_string());
            messages.insert("filter_checkall", "全选".to_string());

            // 上传相关
            messages.insert("upload_drag", "点击或拖拽文件到此区域上传".to_string());
            messages.insert("upload_select", "选择文件".to_string());
            messages.insert("upload_error", "上传失败".to_string());
            messages.insert("upload_success", "上传成功".to_string());
            messages.insert("upload_progress", "上传中...".to_string());
            messages.insert("file_size_limit", "文件大小不能超过 {size}".to_string());
            messages.insert("file_type_error", "不支持的文件类型".to_string());

            // 弹窗相关
            messages.insert("modal_ok", "确定".to_string());
            messages.insert("modal_cancel", "取消".to_string());
            messages.insert("confirm_delete", "确定要删除吗？".to_string());
            messages.insert("confirm_title", "确认".to_string());
            messages.insert("warning_title", "警告".to_string());
            messages.insert("error_title", "错误".to_string());
            messages.insert("info_title", "信息".to_string());

            // 步骤条
            messages.insert("step_prev", "上一步".to_string());
            messages.insert("step_next", "下一步".to_string());
            messages.insert("step_finish", "完成".to_string());
            messages.insert("step_process", "进行中".to_string());
            messages.insert("step_wait", "等待".to_string());
            messages.insert("step_error", "错误".to_string());

            // 传输穿梭框
            messages.insert("transfer_titles", "['', '']".to_string());
            messages.insert("transfer_search_placeholder", "请输入搜索内容".to_string());
            messages.insert("transfer_item_unit", "项".to_string());
            messages.insert("transfer_items_unit", "项".to_string());
            messages.insert("transfer_remove", "删除".to_string());
            messages.insert("transfer_remove_all", "删除全部".to_string());
            messages.insert("transfer_select_all", "全选".to_string());
            messages.insert("transfer_select_invert", "反选".to_string());
        }
        Locale::En => {
            // Basic operations
            messages.insert("ok", "OK".to_string());
            messages.insert("cancel", "Cancel".to_string());
            messages.insert("close", "Close".to_string());
            messages.insert("loading", "Loading...".to_string());
            messages.insert("empty", "No Data".to_string());
            messages.insert("error", "Error".to_string());
            messages.insert("success", "Success".to_string());
            messages.insert("warning", "Warning".to_string());
            messages.insert("info", "Info".to_string());
            messages.insert("confirm", "Confirm".to_string());
            messages.insert("delete", "Delete".to_string());
            messages.insert("edit", "Edit".to_string());
            messages.insert("save", "Save".to_string());
            messages.insert("reset", "Reset".to_string());
            messages.insert("search", "Search".to_string());
            messages.insert("filter", "Filter".to_string());
            messages.insert("sort", "Sort".to_string());
            messages.insert("refresh", "Refresh".to_string());
            messages.insert("upload", "Upload".to_string());
            messages.insert("download", "Download".to_string());
            messages.insert("prev", "Previous".to_string());
            messages.insert("next", "Next".to_string());
            messages.insert("total", "Total {total} items".to_string());
            messages.insert("page_size", "{size} items per page".to_string());

            // Form related
            messages.insert("required", "This field is required".to_string());
            messages.insert(
                "invalid_email",
                "Please enter a valid email address".to_string(),
            );
            messages.insert("invalid_url", "Please enter a valid URL".to_string());
            messages.insert(
                "invalid_phone",
                "Please enter a valid phone number".to_string(),
            );
            messages.insert(
                "min_length",
                "Please enter at least {min} characters".to_string(),
            );
            messages.insert(
                "max_length",
                "Please enter no more than {max} characters".to_string(),
            );
            messages.insert("pattern_mismatch", "Input format is incorrect".to_string());

            // Date and time
            messages.insert("today", "Today".to_string());
            messages.insert("yesterday", "Yesterday".to_string());
            messages.insert("tomorrow", "Tomorrow".to_string());
            messages.insert("this_week", "This Week".to_string());
            messages.insert("last_week", "Last Week".to_string());
            messages.insert("this_month", "This Month".to_string());
            messages.insert("last_month", "Last Month".to_string());
            messages.insert("this_year", "This Year".to_string());
            messages.insert("last_year", "Last Year".to_string());
            messages.insert("select_date", "Select Date".to_string());
            messages.insert("select_time", "Select Time".to_string());
            messages.insert("start_date", "Start Date".to_string());
            messages.insert("end_date", "End Date".to_string());

            // Table related
            messages.insert("select_all", "Select All".to_string());
            messages.insert("select_invert", "Invert Selection".to_string());
            messages.insert("select_none", "Deselect All".to_string());
            messages.insert("sort_asc", "Sort Ascending".to_string());
            messages.insert("sort_desc", "Sort Descending".to_string());
            messages.insert("filter_menu", "Filter Menu".to_string());
            messages.insert("filter_reset", "Reset".to_string());
            messages.insert("filter_confirm", "OK".to_string());
            messages.insert("filter_checkall", "Select All".to_string());

            // Upload related
            messages.insert(
                "upload_drag",
                "Click or drag file to this area to upload".to_string(),
            );
            messages.insert("upload_select", "Select File".to_string());
            messages.insert("upload_error", "Upload Failed".to_string());
            messages.insert("upload_success", "Upload Successful".to_string());
            messages.insert("upload_progress", "Uploading...".to_string());
            messages.insert(
                "file_size_limit",
                "File size cannot exceed {size}".to_string(),
            );
            messages.insert("file_type_error", "Unsupported file type".to_string());

            // Modal related
            messages.insert("modal_ok", "OK".to_string());
            messages.insert("modal_cancel", "Cancel".to_string());
            messages.insert(
                "confirm_delete",
                "Are you sure you want to delete?".to_string(),
            );
            messages.insert("confirm_title", "Confirm".to_string());
            messages.insert("warning_title", "Warning".to_string());
            messages.insert("error_title", "Error".to_string());
            messages.insert("info_title", "Information".to_string());

            // Steps
            messages.insert("step_prev", "Previous".to_string());
            messages.insert("step_next", "Next".to_string());
            messages.insert("step_finish", "Finish".to_string());
            messages.insert("step_process", "In Progress".to_string());
            messages.insert("step_wait", "Waiting".to_string());
            messages.insert("step_error", "Error".to_string());

            // Transfer
            messages.insert("transfer_titles", "['', '']".to_string());
            messages.insert("transfer_search_placeholder", "Search here".to_string());
            messages.insert("transfer_item_unit", "item".to_string());
            messages.insert("transfer_items_unit", "items".to_string());
            messages.insert("transfer_remove", "Remove".to_string());
            messages.insert("transfer_remove_all", "Remove All".to_string());
            messages.insert("transfer_select_all", "Select All".to_string());
            messages.insert("transfer_select_invert", "Invert Selection".to_string());
        }
        _ => {
            // 默认使用英文
            return get_default_messages(&Locale::En);
        }
    }

    messages
}

/// 获取默认日期时间格式
fn get_default_datetime_format(locale: &Locale) -> (String, String) {
    match locale {
        Locale::ZhCN => ("YYYY年MM月DD日".to_string(), "HH:mm:ss".to_string()),
        Locale::En => ("MM/DD/YYYY".to_string(), "HH:mm:ss".to_string()),
        Locale::De => ("DD.MM.YYYY".to_string(), "HH:mm:ss".to_string()),
        Locale::Fr => ("DD/MM/YYYY".to_string(), "HH:mm:ss".to_string()),
        _ => ("YYYY-MM-DD".to_string(), "HH:mm:ss".to_string()),
    }
}

/// 获取默认数字格式
fn get_default_number_format(locale: &Locale) -> NumberFormat {
    match locale {
        Locale::ZhCN => NumberFormat {
            thousands_separator: ",".to_string(),
            decimal_separator: ".".to_string(),
            decimal_places: 2,
        },
        Locale::En => NumberFormat {
            thousands_separator: ",".to_string(),
            decimal_separator: ".".to_string(),
            decimal_places: 2,
        },
        Locale::De | Locale::Fr => NumberFormat {
            thousands_separator: ".".to_string(),
            decimal_separator: ",".to_string(),
            decimal_places: 2,
        },
        _ => NumberFormat {
            thousands_separator: ",".to_string(),
            decimal_separator: ".".to_string(),
            decimal_places: 2,
        },
    }
}

/// 获取默认货币格式
fn get_default_currency_format(locale: &Locale) -> CurrencyFormat {
    match locale {
        Locale::ZhCN => CurrencyFormat {
            symbol: "¥".to_string(),
            code: "CNY".to_string(),
            symbol_position: CurrencySymbolPosition::Prefix,
            number_format: get_default_number_format(locale),
        },
        Locale::En => CurrencyFormat {
            symbol: "$".to_string(),
            code: "USD".to_string(),
            symbol_position: CurrencySymbolPosition::Prefix,
            number_format: get_default_number_format(locale),
        },
        Locale::De => CurrencyFormat {
            symbol: "€".to_string(),
            code: "EUR".to_string(),
            symbol_position: CurrencySymbolPosition::Suffix,
            number_format: get_default_number_format(locale),
        },
        Locale::Fr => CurrencyFormat {
            symbol: "€".to_string(),
            code: "EUR".to_string(),
            symbol_position: CurrencySymbolPosition::Suffix,
            number_format: get_default_number_format(locale),
        },
        Locale::Ja => CurrencyFormat {
            symbol: "¥".to_string(),
            code: "JPY".to_string(),
            symbol_position: CurrencySymbolPosition::Prefix,
            number_format: NumberFormat {
                thousands_separator: ",".to_string(),
                decimal_separator: ".".to_string(),
                decimal_places: 0, // 日元通常不显示小数
            },
        },
        _ => CurrencyFormat {
            symbol: "$".to_string(),
            code: "USD".to_string(),
            symbol_position: CurrencySymbolPosition::Prefix,
            number_format: get_default_number_format(locale),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_locale_code() {
        assert_eq!(Locale::ZhCN.code(), "zh-CN");
        assert_eq!(Locale::En.code(), "en");
        assert_eq!(Locale::Custom("test".to_string()).code(), "test");
    }

    #[test]
    fn test_locale_from_code() {
        assert_eq!(Locale::from_code("zh-CN"), Locale::ZhCN);
        assert_eq!(Locale::from_code("en"), Locale::En);
        assert_eq!(
            Locale::from_code("unknown"),
            Locale::Custom("unknown".to_string())
        );
    }

    #[test]
    fn test_locale_is_rtl() {
        assert!(!Locale::ZhCN.is_rtl());
        assert!(!Locale::En.is_rtl());
        assert!(Locale::Ar.is_rtl());
    }

    #[test]
    fn test_locale_config_translate() {
        let config = LocaleConfig::new(Locale::ZhCN);
        assert_eq!(config.translate("ok"), "确定");
        assert_eq!(config.translate("unknown_key"), "unknown_key");
    }

    #[test]
    fn test_locale_config_translate_with_args() {
        let config = LocaleConfig::new(Locale::ZhCN);
        let result = config.translate_with_args("total", &[("total", "100")]);
        assert_eq!(result, "共 100 条");
    }

    #[test]
    fn test_format_number() {
        let config = LocaleConfig::new(Locale::ZhCN);
        assert_eq!(config.format_number(1234.56), "1,234.56");
        assert_eq!(config.format_number(1000000.0), "1,000,000.00");
    }

    #[test]
    fn test_format_currency() {
        let config = LocaleConfig::new(Locale::ZhCN);
        assert_eq!(config.format_currency(1234.56), "¥1,234.56");

        let config_en = LocaleConfig::new(Locale::En);
        assert_eq!(config_en.format_currency(1234.56), "$1,234.56");
    }

    #[test]
    fn test_custom_messages() {
        let mut custom_messages = HashMap::new();
        custom_messages.insert("custom_key", "自定义值".to_string());

        let config = LocaleConfig::new(Locale::ZhCN).with_messages(custom_messages);

        assert_eq!(config.translate("custom_key"), "自定义值");
    }
}

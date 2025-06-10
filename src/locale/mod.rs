//! 国际化模块
//!
//! 提供多语言支持，包括语言包管理、文本翻译、日期时间格式化等功能。
//! 支持动态切换语言，并提供 React Context 风格的 API。

use chrono::{DateTime, Local};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use wasm_bindgen::prelude::*;

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

/// 支持的语言
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Locale {
    /// 简体中文
    ZhCN,
    /// 英文
    En,
    /// 繁体中文
    ZhTW,
    /// 日文
    Ja,
    /// 韩文
    Ko,
    /// 俄文
    Ru,
    /// 法文
    Fr,
    /// 德文
    De,
    /// 西班牙文
    Es,
    /// 葡萄牙文
    Pt,
    /// 意大利文
    It,
    /// 阿拉伯文
    Ar,
    /// 泰文
    Th,
    /// 越南文
    Vi,
}

impl Default for Locale {
    fn default() -> Self {
        Self::En
    }
}

impl std::fmt::Display for Locale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Locale::ZhCN => write!(f, "zh-CN"),
            Locale::En => write!(f, "en"),
            Locale::ZhTW => write!(f, "zh-TW"),
            Locale::Ja => write!(f, "ja"),
            Locale::Ko => write!(f, "ko"),
            Locale::Ru => write!(f, "ru"),
            Locale::Fr => write!(f, "fr"),
            Locale::De => write!(f, "de"),
            Locale::Es => write!(f, "es"),
            Locale::Pt => write!(f, "pt"),
            Locale::It => write!(f, "it"),
            Locale::Ar => write!(f, "ar"),
            Locale::Th => write!(f, "th"),
            Locale::Vi => write!(f, "vi"),
        }
    }
}

impl Locale {
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
            Locale::Th => "ไทย",
            Locale::Vi => "Tiếng Việt",
        }
    }

    /// 是否为从右到左的语言
    pub fn is_rtl(&self) -> bool {
        matches!(self, Locale::Ar)
    }
}

/// 翻译键类型
pub type TranslationKey = &'static str;

/// 翻译值类型
pub type TranslationValue = String;

/// 语言包类型
pub type LanguagePack = HashMap<TranslationKey, TranslationValue>;

/// 国际化配置
#[derive(Debug, Clone, PartialEq)]
pub struct LocaleConfig {
    /// 语言
    pub locale: Locale,
    /// 翻译消息
    pub messages: HashMap<String, String>,
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
    /// 小数位数
    pub decimal_places: usize,
    /// 千位分隔符
    pub thousands_separator: String,
    /// 小数点符号
    pub decimal_separator: String,
}

/// 货币符号位置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CurrencySymbolPosition {
    /// 前缀
    Prefix,
    /// 后缀
    Suffix,
}

/// 货币格式配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CurrencyFormat {
    /// 货币符号
    pub symbol: String,
    /// 符号位置
    pub symbol_position: CurrencySymbolPosition,
    /// 数字格式
    pub number_format: NumberFormat,
}

impl Default for NumberFormat {
    fn default() -> Self {
        Self {
            decimal_places: 2,
            thousands_separator: ",".to_string(),
            decimal_separator: ".".to_string(),
        }
    }
}

impl Default for CurrencyFormat {
    fn default() -> Self {
        Self {
            symbol: "$".to_string(),
            symbol_position: CurrencySymbolPosition::Prefix,
            number_format: NumberFormat::default(),
        }
    }
}

impl Default for LocaleConfig {
    fn default() -> Self {
        Self {
            locale: Locale::default(),
            messages: HashMap::new(),
            date_format: "YYYY-MM-DD".to_string(),
            time_format: "HH:mm:ss".to_string(),
            number_format: NumberFormat::default(),
            currency_format: CurrencyFormat::default(),
        }
    }
}

/// 国际化上下文
#[derive(Clone)]
pub struct LocaleContext {
    /// 当前语言
    pub locale: Locale,
    /// 翻译映射
    pub translations: HashMap<String, String>,
    /// 语言切换函数
    pub switch_locale: Arc<dyn FnMut(Locale) + 'static>,
}

impl PartialEq for LocaleContext {
    fn eq(&self, other: &Self) -> bool {
        self.locale == other.locale && self.translations == other.translations
        // 忽略 switch_locale 字段比较
    }
}

/// 语言提供者属性
#[derive(Props, Clone, PartialEq)]
pub struct LocaleProviderProps {
    /// 子组件
    pub children: Element,
    /// 语言配置
    #[props(into)]
    pub locale: Signal<Locale>,
}

/// 国际化提供者组件
#[component]
pub fn LocaleProvider(props: LocaleProviderProps) -> Element {
    let locale = props.locale;

    // 初始化翻译
    let mut translations = use_signal(|| get_translations(&locale.read()));

    // 当语言变化时更新翻译
    use_effect(move || {
        translations.set(get_translations(&locale.read()));
    });

    // 创建语言切换函数
    let switch_locale = {
        let mut locale = locale.clone();
        let mut translations = translations.clone();

        Arc::new(move |new_locale: Locale| {
            locale.set(new_locale.clone());
            translations.set(get_translations(&new_locale));
        })
    };

    // 创建国际化上下文
    let locale_context = LocaleContext {
        locale: locale.read().clone(),
        translations: translations.read().clone(),
        switch_locale,
    };

    // 提供国际化上下文
    use_context_provider(|| locale_context);

    props.children
}

/// 使用语言的 Hook
///
/// 获取当前的语言上下文
pub fn use_locale() -> LocaleContext {
    use_context::<LocaleContext>()
}

/// 使用语言切换的 Hook
///
/// 获取语言切换函数
pub fn use_locale_switch() -> Arc<dyn FnMut(Locale) + 'static> {
    let locale_context = use_locale();
    locale_context.switch_locale
}

/// 使用翻译的 Hook
///
/// 获取翻译函数
///
/// # 返回值
///
/// 翻译函数，接受一个键名并返回对应的翻译文本
pub fn use_translate() -> impl Fn(&str) -> String {
    let locale_context = use_locale();
    let translations = locale_context.translations.clone();

    move |key: &str| {
        translations
            .get(key)
            .cloned()
            .unwrap_or_else(|| key.to_string())
    }
}

/// 获取翻译文本
///
/// 根据语言类型获取对应的翻译文本
///
/// # 参数
///
/// * `locale` - 语言类型
///
/// # 返回值
///
/// 翻译文本映射
fn get_translations(locale: &Locale) -> HashMap<String, String> {
    match locale {
        Locale::ZhCN => zh_cn_translations(),
        Locale::En => en_translations(),
        _ => en_translations(), // 默认使用英文
    }
}

/// 简体中文翻译
fn zh_cn_translations() -> HashMap<String, String> {
    let mut translations = HashMap::new();

    // 通用
    translations.insert("ok".to_string(), "确定".to_string());
    translations.insert("cancel".to_string(), "取消".to_string());
    translations.insert("confirm".to_string(), "确认".to_string());
    translations.insert("submit".to_string(), "提交".to_string());
    translations.insert("reset".to_string(), "重置".to_string());
    translations.insert("search".to_string(), "搜索".to_string());
    translations.insert("loading".to_string(), "加载中...".to_string());
    translations.insert("edit".to_string(), "编辑".to_string());
    translations.insert("delete".to_string(), "删除".to_string());
    translations.insert("save".to_string(), "保存".to_string());

    // 分页
    translations.insert("pagination.prev".to_string(), "上一页".to_string());
    translations.insert("pagination.next".to_string(), "下一页".to_string());
    translations.insert("pagination.total".to_string(), "共 {0} 条".to_string());
    translations.insert("pagination.page".to_string(), "第 {0} 页".to_string());

    // 表单
    translations.insert("form.required".to_string(), "必填项".to_string());
    translations.insert("form.optional".to_string(), "选填项".to_string());
    translations.insert("form.error".to_string(), "输入有误".to_string());

    // 日期
    translations.insert("date.today".to_string(), "今天".to_string());
    translations.insert("date.yesterday".to_string(), "昨天".to_string());
    translations.insert("date.tomorrow".to_string(), "明天".to_string());

    translations
}

/// 英文翻译
fn en_translations() -> HashMap<String, String> {
    let mut translations = HashMap::new();

    // 通用
    translations.insert("ok".to_string(), "OK".to_string());
    translations.insert("cancel".to_string(), "Cancel".to_string());
    translations.insert("confirm".to_string(), "Confirm".to_string());
    translations.insert("submit".to_string(), "Submit".to_string());
    translations.insert("reset".to_string(), "Reset".to_string());
    translations.insert("search".to_string(), "Search".to_string());
    translations.insert("loading".to_string(), "Loading...".to_string());
    translations.insert("edit".to_string(), "Edit".to_string());
    translations.insert("delete".to_string(), "Delete".to_string());
    translations.insert("save".to_string(), "Save".to_string());

    // 分页
    translations.insert("pagination.prev".to_string(), "Previous".to_string());
    translations.insert("pagination.next".to_string(), "Next".to_string());
    translations.insert(
        "pagination.total".to_string(),
        "Total {0} items".to_string(),
    );
    translations.insert("pagination.page".to_string(), "Page {0}".to_string());

    // 表单
    translations.insert("form.required".to_string(), "Required".to_string());
    translations.insert("form.optional".to_string(), "Optional".to_string());
    translations.insert("form.error".to_string(), "Invalid input".to_string());

    // 日期
    translations.insert("date.today".to_string(), "Today".to_string());
    translations.insert("date.yesterday".to_string(), "Yesterday".to_string());
    translations.insert("date.tomorrow".to_string(), "Tomorrow".to_string());

    translations
}

impl LocaleConfig {
    /// 创建新的国际化配置
    pub fn new(locale: Locale) -> Self {
        let messages = get_translations(&locale);
        Self {
            locale,
            messages,
            date_format: "%Y-%m-%d".to_string(),
            time_format: "%H:%M:%S".to_string(),
            number_format: NumberFormat::default(),
            currency_format: CurrencyFormat::default(),
        }
    }

    /// 设置自定义语言包
    pub fn with_messages(mut self, messages: HashMap<String, String>) -> Self {
        self.messages = messages;
        self
    }

    /// 设置日期格式
    pub fn with_date_format(mut self, date_format: String) -> Self {
        self.date_format = date_format;
        self
    }

    /// 设置时间格式
    pub fn with_time_format(mut self, time_format: String) -> Self {
        self.time_format = time_format;
        self
    }

    /// 设置数字格式
    pub fn with_number_format(mut self, number_format: NumberFormat) -> Self {
        self.number_format = number_format;
        self
    }

    /// 设置货币格式
    pub fn with_currency_format(mut self, currency_format: CurrencyFormat) -> Self {
        self.currency_format = currency_format;
        self
    }

    /// 翻译文本
    pub fn translate(&self, key: &str) -> String {
        self.messages
            .get(key)
            .cloned()
            .unwrap_or_else(|| key.to_string())
    }

    /// 翻译文本并替换占位符
    pub fn translate_with_args(&self, key: &str, args: &[(&str, &str)]) -> String {
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

            let formatter = DateTimeFormat::new(&self.locale.to_string(), &options);
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

            let formatter = DateTimeFormat::new(&self.locale.to_string(), &options);
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

            let formatter = DateTimeFormat::new(&self.locale.to_string(), &options);
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

            let formatter = RelativeTimeFormat::new_relative(&self.locale.to_string(), &options);

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

/// 获取日期时间格式化的Hook
pub fn use_datetime_format() -> impl Fn(&chrono::DateTime<chrono::Local>) -> String {
    let locale_context = use_locale();

    move |datetime: &chrono::DateTime<chrono::Local>| {
        format!("{}", datetime.format("%Y-%m-%d %H:%M:%S"))
    }
}

/// 获取日期格式化的Hook
pub fn use_date_format() -> impl Fn(&chrono::DateTime<chrono::Local>) -> String {
    let locale_context = use_locale();

    move |datetime: &chrono::DateTime<chrono::Local>| format!("{}", datetime.format("%Y-%m-%d"))
}

/// 获取时间格式化的Hook
pub fn use_time_format() -> impl Fn(&chrono::DateTime<chrono::Local>) -> String {
    let locale_context = use_locale();

    move |datetime: &chrono::DateTime<chrono::Local>| format!("{}", datetime.format("%H:%M:%S"))
}

/// 获取相对时间格式化的Hook
pub fn use_relative_time_format() -> impl Fn(&chrono::DateTime<chrono::Local>) -> String {
    let locale_context = use_locale();

    move |datetime: &chrono::DateTime<chrono::Local>| {
        let now = chrono::Local::now();
        let duration = now.signed_duration_since(*datetime);

        if duration.num_days() == 0 {
            if duration.num_hours() == 0 {
                if duration.num_minutes() == 0 {
                    "刚刚".to_string()
                } else {
                    format!("{}分钟前", duration.num_minutes())
                }
            } else {
                format!("{}小时前", duration.num_hours())
            }
        } else if duration.num_days() == 1 {
            "昨天".to_string()
        } else if duration.num_days() < 7 {
            format!("{}天前", duration.num_days())
        } else if duration.num_days() < 30 {
            format!("{}周前", duration.num_days() / 7)
        } else if duration.num_days() < 365 {
            format!("{}月前", duration.num_days() / 30)
        } else {
            format!("{}年前", duration.num_days() / 365)
        }
    }
}

/// 获取数字格式化的Hook
pub fn use_number_format() -> impl Fn(f64) -> String {
    let locale_context = use_locale();

    move |number: f64| format!("{:.2}", number)
}

/// 获取货币格式化的Hook
pub fn use_currency_format() -> impl Fn(f64) -> String {
    let locale_context = use_locale();

    move |amount: f64| match locale_context.locale {
        Locale::ZhCN => format!("¥ {:.2}", amount),
        Locale::En => format!("$ {:.2}", amount),
        _ => format!("$ {:.2}", amount),
    }
}

/// 检查当前语言是否为RTL的Hook
pub fn use_is_rtl() -> bool {
    let locale_context = use_locale();
    locale_context.locale.is_rtl()
}

/// 获取当前语言代码的Hook
pub fn use_locale_code() -> String {
    let locale_context = use_locale();
    locale_context.locale.to_string()
}

/// 获取当前语言名称的Hook
pub fn use_locale_name() -> String {
    let locale_context = use_locale();
    locale_context.locale.name().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_locale_code() {
        assert_eq!(Locale::ZhCN.to_string(), "zh-CN");
        assert_eq!(Locale::En.to_string(), "en");
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
        custom_messages.insert("custom_key".to_string(), "自定义值".to_string());

        let config = LocaleConfig::new(Locale::ZhCN).with_messages(custom_messages);

        assert_eq!(config.translate("custom_key"), "自定义值");
    }
}

//! 主题和国际化系统演示
//!
//! 这个示例展示了如何使用改进后的主题系统和国际化功能，包括：
//! - 动态主题切换（亮色/暗色/紧凑模式）
//! - 多语言支持和动态语言切换
//! - 日期时间本地化格式化
//! - RTL语言支持
//! - 主题算法和颜色计算
//! - 响应式设计令牌

use ant_design_dioxus::{
    locale::{
        use_currency_format, use_date_format, use_datetime_format, use_is_rtl, use_locale_code,
        use_locale_name, use_locale_switch, use_number_format, use_relative_time_format,
        use_time_format, use_translate, Locale, LocaleConfig, LocaleProvider,
    },
    theme::{
        use_theme, use_theme_switch, use_theme_token, ColorTheme, Theme, ThemeConfig, ThemeProvider,
    },
};
use chrono::{DateTime, Local};
use dioxus::prelude::*;
use std::collections::HashMap;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // 初始化主题配置
    let theme_config = use_signal(|| ThemeConfig::light());
    let locale_config = use_signal(|| LocaleConfig::new(Locale::ZhCN));

    rsx! {
        ThemeProvider {
            config: theme_config,
            LocaleProvider {
                config: locale_config,
                div {
                    class: "demo-container",
                    style: "padding: 24px; min-height: 100vh;",

                    // 标题
                    h1 {
                        class: "demo-title",
                        style: "margin-bottom: 32px; text-align: center;",
                        "Ant Design Dioxus - 主题与国际化演示"
                    }

                    // 控制面板
                    ControlPanel {}

                    // 主题演示区域
                    ThemeDemo {}

                    // 国际化演示区域
                    I18nDemo {}

                    // 日期时间演示区域
                    DateTimeDemo {}

                    // 数字格式化演示区域
                    NumberFormatDemo {}
                }
            }
        }
    }
}

#[component]
fn ControlPanel() -> Element {
    let theme_switch = use_theme_switch();
    let locale_switch = use_locale_switch();
    let translate = use_translate();
    let is_rtl = use_is_rtl();
    let locale_name = use_locale_name();

    rsx! {
        div {
            class: "control-panel",
            style: "display: flex; gap: 16px; margin-bottom: 32px; padding: 16px; border: 1px solid var(--ant-color-border); border-radius: var(--ant-border-radius-base); background: var(--ant-color-bg-container);",
            dir: if is_rtl { "rtl" } else { "ltr" },

            // 主题切换
            div {
                class: "theme-controls",
                h3 { "{translate(\"theme\")}": "主题切换" }
                div {
                    style: "display: flex; gap: 8px;",
                    button {
                        onclick: move |_| theme_switch(ThemeConfig::light()),
                        style: "padding: 8px 16px; border: 1px solid var(--ant-color-border); border-radius: var(--ant-border-radius-base); background: var(--ant-color-bg-container); cursor: pointer;",
                        "亮色主题"
                    }
                    button {
                        onclick: move |_| theme_switch(ThemeConfig::dark()),
                        style: "padding: 8px 16px; border: 1px solid var(--ant-color-border); border-radius: var(--ant-border-radius-base); background: var(--ant-color-bg-container); cursor: pointer;",
                        "暗色主题"
                    }
                    button {
                        onclick: move |_| theme_switch(ThemeConfig::compact()),
                        style: "padding: 8px 16px; border: 1px solid var(--ant-color-border); border-radius: var(--ant-border-radius-base); background: var(--ant-color-bg-container); cursor: pointer;",
                        "紧凑主题"
                    }
                }
            }

            // 语言切换
            div {
                class: "locale-controls",
                h3 { "语言切换 (当前: {locale_name})" }
                div {
                    style: "display: flex; gap: 8px;",
                    button {
                        onclick: move |_| locale_switch(Locale::ZhCN),
                        style: "padding: 8px 16px; border: 1px solid var(--ant-color-border); border-radius: var(--ant-border-radius-base); background: var(--ant-color-bg-container); cursor: pointer;",
                        "中文"
                    }
                    button {
                        onclick: move |_| locale_switch(Locale::En),
                        style: "padding: 8px 16px; border: 1px solid var(--ant-color-border); border-radius: var(--ant-border-radius-base); background: var(--ant-color-bg-container); cursor: pointer;",
                        "English"
                    }
                    button {
                        onclick: move |_| locale_switch(Locale::Ar),
                        style: "padding: 8px 16px; border: 1px solid var(--ant-color-border); border-radius: var(--ant-border-radius-base); background: var(--ant-color-bg-container); cursor: pointer;",
                        "العربية"
                    }
                }
            }
        }
    }
}

#[component]
fn ThemeDemo() -> Element {
    let theme = use_theme();
    let primary_color = use_theme_token("color-primary");
    let success_color = use_theme_token("color-success");
    let warning_color = use_theme_token("color-warning");
    let error_color = use_theme_token("color-error");
    let translate = use_translate();

    rsx! {
        div {
            class: "theme-demo",
            style: "margin-bottom: 32px;",

            h2 { "主题演示" }

            div {
                class: "color-palette",
                style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 16px; margin-top: 16px;",

                // 主色调
                div {
                    class: "color-card",
                    style: "padding: 16px; border-radius: var(--ant-border-radius-base); background: {primary_color}; color: white;",
                    h4 { "主色调 (Primary)" }
                    p { "颜色值: {primary_color}" }
                }

                // 成功色
                div {
                    class: "color-card",
                    style: "padding: 16px; border-radius: var(--ant-border-radius-base); background: {success_color}; color: white;",
                    h4 { "成功色 (Success)" }
                    p { "颜色值: {success_color}" }
                }

                // 警告色
                div {
                    class: "color-card",
                    style: "padding: 16px; border-radius: var(--ant-border-radius-base); background: {warning_color}; color: white;",
                    h4 { "警告色 (Warning)" }
                    p { "颜色值: {warning_color}" }
                }

                // 错误色
                div {
                    class: "color-card",
                    style: "padding: 16px; border-radius: var(--ant-border-radius-base); background: {error_color}; color: white;",
                    h4 { "错误色 (Error)" }
                    p { "颜色值: {error_color}" }
                }
            }

            // 设计令牌展示
            div {
                class: "design-tokens",
                style: "margin-top: 24px; padding: 16px; border: 1px solid var(--ant-color-border); border-radius: var(--ant-border-radius-base); background: var(--ant-color-bg-container);",

                h3 { "设计令牌" }
                div {
                    style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(150px, 1fr)); gap: 12px; margin-top: 12px;",

                    div { "字体大小 XS: var(--ant-font-size-xs)" }
                    div { "字体大小 SM: var(--ant-font-size-sm)" }
                    div { "字体大小 Base: var(--ant-font-size-base)" }
                    div { "字体大小 LG: var(--ant-font-size-lg)" }
                    div { "字体大小 XL: var(--ant-font-size-xl)" }

                    div { "间距 XS: var(--ant-spacing-xs)" }
                    div { "间距 SM: var(--ant-spacing-sm)" }
                    div { "间距 Base: var(--ant-spacing-base)" }
                    div { "间距 LG: var(--ant-spacing-lg)" }
                    div { "间距 XL: var(--ant-spacing-xl)" }
                }
            }
        }
    }
}

#[component]
fn I18nDemo() -> Element {
    let translate = use_translate();
    let is_rtl = use_is_rtl();
    let locale_code = use_locale_code();

    rsx! {
        div {
            class: "i18n-demo",
            style: "margin-bottom: 32px;",
            dir: if is_rtl { "rtl" } else { "ltr" },

            h2 { "国际化演示 (当前语言: {locale_code})" }

            div {
                class: "translation-examples",
                style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 16px; margin-top: 16px;",

                div {
                    class: "translation-card",
                    style: "padding: 16px; border: 1px solid var(--ant-color-border); border-radius: var(--ant-border-radius-base); background: var(--ant-color-bg-container);",

                    h4 { "基础操作" }
                    ul {
                        li { "{translate(\"ok\")}: {translate(\"ok\")}" }
                        li { "{translate(\"cancel\")}: {translate(\"cancel\")}" }
                        li { "{translate(\"save\")}: {translate(\"save\")}" }
                        li { "{translate(\"delete\")}: {translate(\"delete\")}" }
                        li { "{translate(\"edit\")}: {translate(\"edit\")}" }
                    }
                }

                div {
                    class: "translation-card",
                    style: "padding: 16px; border: 1px solid var(--ant-color-border); border-radius: var(--ant-border-radius-base); background: var(--ant-color-bg-container);",

                    h4 { "状态信息" }
                    ul {
                        li { "{translate(\"loading\")}: {translate(\"loading\")}" }
                        li { "{translate(\"success\")}: {translate(\"success\")}" }
                        li { "{translate(\"warning\")}: {translate(\"warning\")}" }
                        li { "{translate(\"error\")}: {translate(\"error\")}" }
                        li { "{translate(\"empty\")}: {translate(\"empty\")}" }
                    }
                }

                div {
                    class: "translation-card",
                    style: "padding: 16px; border: 1px solid var(--ant-color-border); border-radius: var(--ant-border-radius-base); background: var(--ant-color-bg-container);",

                    h4 { "表单验证" }
                    ul {
                        li { "{translate(\"required\")}: {translate(\"required\")}" }
                        li { "{translate(\"invalid_email\")}: {translate(\"invalid_email\")}" }
                        li { "{translate(\"invalid_phone\")}: {translate(\"invalid_phone\")}" }
                    }
                }

                div {
                    class: "translation-card",
                    style: "padding: 16px; border: 1px solid var(--ant-color-border); border-radius: var(--ant-border-radius-base); background: var(--ant-color-bg-container);",

                    h4 { "日期时间" }
                    ul {
                        li { "{translate(\"today\")}: {translate(\"today\")}" }
                        li { "{translate(\"yesterday\")}: {translate(\"yesterday\")}" }
                        li { "{translate(\"tomorrow\")}: {translate(\"tomorrow\")}" }
                        li { "{translate(\"this_week\")}: {translate(\"this_week\")}" }
                        li { "{translate(\"select_date\")}: {translate(\"select_date\")}" }
                    }
                }
            }
        }
    }
}

#[component]
fn DateTimeDemo() -> Element {
    let datetime_format = use_datetime_format();
    let date_format = use_date_format();
    let time_format = use_time_format();
    let relative_time_format = use_relative_time_format();

    let now = Local::now();
    let yesterday = now - chrono::Duration::days(1);
    let last_week = now - chrono::Duration::weeks(1);
    let last_month = now - chrono::Duration::days(30);

    rsx! {
        div {
            class: "datetime-demo",
            style: "margin-bottom: 32px;",

            h2 { "日期时间本地化演示" }

            div {
                class: "datetime-examples",
                style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 16px; margin-top: 16px;",

                div {
                    class: "datetime-card",
                    style: "padding: 16px; border: 1px solid var(--ant-color-border); border-radius: var(--ant-border-radius-base); background: var(--ant-color-bg-container);",

                    h4 { "当前时间" }
                    ul {
                        li { "完整日期时间: {datetime_format(&now)}" }
                        li { "日期: {date_format(&now)}" }
                        li { "时间: {time_format(&now)}" }
                        li { "相对时间: {relative_time_format(&now)}" }
                    }
                }

                div {
                    class: "datetime-card",
                    style: "padding: 16px; border: 1px solid var(--ant-color-border); border-radius: var(--ant-border-radius-base); background: var(--ant-color-bg-container);",

                    h4 { "历史时间" }
                    ul {
                        li { "昨天: {relative_time_format(&yesterday)}" }
                        li { "上周: {relative_time_format(&last_week)}" }
                        li { "上月: {relative_time_format(&last_month)}" }
                    }
                }
            }
        }
    }
}

#[component]
fn NumberFormatDemo() -> Element {
    let number_format = use_number_format();
    let currency_format = use_currency_format();

    rsx! {
        div {
            class: "number-demo",
            style: "margin-bottom: 32px;",

            h2 { "数字格式化演示" }

            div {
                class: "number-examples",
                style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 16px; margin-top: 16px;",

                div {
                    class: "number-card",
                    style: "padding: 16px; border: 1px solid var(--ant-color-border); border-radius: var(--ant-border-radius-base); background: var(--ant-color-bg-container);",

                    h4 { "数字格式化" }
                    ul {
                        li { "1234.56: {number_format(1234.56)}" }
                        li { "1000000: {number_format(1000000.0)}" }
                        li { "0.123456: {number_format(0.123456)}" }
                    }
                }

                div {
                    class: "number-card",
                    style: "padding: 16px; border: 1px solid var(--ant-color-border); border-radius: var(--ant-border-radius-base); background: var(--ant-color-bg-container);",

                    h4 { "货币格式化" }
                    ul {
                        li { "99.99: {currency_format(99.99)}" }
                        li { "1234.56: {currency_format(1234.56)}" }
                        li { "1000000: {currency_format(1000000.0)}" }
                    }
                }
            }
        }
    }
}

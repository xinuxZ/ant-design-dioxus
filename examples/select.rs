//! Select 选择器组件示例
//!
//! 展示 Select 组件的各种用法和功能

use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        style { {include_str!("../src/components/select/style.css")} }

        div {
            style: "padding: 24px; max-width: 1200px; margin: 0 auto;",

            h1 { "Select 选择器示例" }

            // 基本用法
            BasicExample {}

            // 尺寸变体
            SizeExample {}

            // 状态变体
            StatusExample {}

            // 多选模式
            MultipleExample {}

            // 搜索功能
            SearchExample {}

            // 清除功能
            ClearableExample {}

            // 禁用状态
            DisabledExample {}

            // 选项组
            OptGroupExample {}

            // 无边框样式
            BorderlessExample {}
        }
    }
}

#[component]
fn BasicExample() -> Element {
    let mut value = use_signal(|| String::new());

    rsx! {
        div {
            style: "margin-bottom: 32px;",

            h2 { "基本用法" }
            p { "基本的选择器用法，支持单选。" }

            div {
                style: "display: flex; gap: 16px; flex-wrap: wrap;",

                Select {
                    value: value(),
                    placeholder: "请选择",
                    on_change: move |v| {
                        value.set(v);
                        println!("选择了: {}", value());
                    },
                    style: "width: 200px;",

                    SelectOption {
                        value: "option1",
                        "选项1"
                    }
                    SelectOption {
                        value: "option2",
                        "选项2"
                    }
                    SelectOption {
                        value: "option3",
                        "选项3"
                    }
                    SelectOption {
                        value: "option4",
                        disabled: true,
                        "禁用选项"
                    }
                }

                div {
                    style: "padding: 8px; background: #f5f5f5; border-radius: 4px;",
                    "当前值: {value()}"
                }
            }
        }
    }
}

#[component]
fn SizeExample() -> Element {
    let mut large_value = use_signal(|| String::new());
    let mut middle_value = use_signal(|| String::new());
    let mut small_value = use_signal(|| String::new());

    rsx! {
        div {
            style: "margin-bottom: 32px;",

            h2 { "不同尺寸" }
            p { "三种尺寸的选择器：大、中、小。" }

            div {
                style: "display: flex; gap: 16px; align-items: center; flex-wrap: wrap;",

                div {
                    "大尺寸:"
                    Select {
                        value: large_value(),
                        placeholder: "Large",
                        size: SelectSize::Large,
                        on_change: move |v| large_value.set(v),
                        style: "width: 200px; margin-left: 8px;",

                        SelectOption { value: "large1", "Large Option 1" }
                        SelectOption { value: "large2", "Large Option 2" }
                        SelectOption { value: "large3", "Large Option 3" }
                    }
                }

                div {
                    "中等尺寸:"
                    Select {
                        value: middle_value(),
                        placeholder: "Middle",
                        size: SelectSize::Middle,
                        on_change: move |v| middle_value.set(v),
                        style: "width: 200px; margin-left: 8px;",

                        SelectOption { value: "middle1", "Middle Option 1" }
                        SelectOption { value: "middle2", "Middle Option 2" }
                        SelectOption { value: "middle3", "Middle Option 3" }
                    }
                }

                div {
                    "小尺寸:"
                    Select {
                        value: small_value(),
                        placeholder: "Small",
                        size: SelectSize::Small,
                        on_change: move |v| small_value.set(v),
                        style: "width: 200px; margin-left: 8px;",

                        SelectOption { value: "small1", "Small Option 1" }
                        SelectOption { value: "small2", "Small Option 2" }
                        SelectOption { value: "small3", "Small Option 3" }
                    }
                }
            }
        }
    }
}

#[component]
fn StatusExample() -> Element {
    let mut normal_value = use_signal(|| String::new());
    let mut error_value = use_signal(|| String::new());
    let mut warning_value = use_signal(|| String::new());

    rsx! {
        div {
            style: "margin-bottom: 32px;",

            h2 { "不同状态" }
            p { "选择器的不同状态：正常、错误、警告。" }

            div {
                style: "display: flex; gap: 16px; align-items: center; flex-wrap: wrap;",

                div {
                    "正常状态:"
                    Select {
                        value: normal_value(),
                        placeholder: "正常状态",
                        status: SelectStatus::Normal,
                        on_change: move |v| normal_value.set(v),
                        style: "width: 200px; margin-left: 8px;",

                        SelectOption { value: "normal1", "正常选项1" }
                        SelectOption { value: "normal2", "正常选项2" }
                    }
                }

                div {
                    "错误状态:"
                    Select {
                        value: error_value(),
                        placeholder: "错误状态",
                        status: SelectStatus::Error,
                        on_change: move |v| error_value.set(v),
                        style: "width: 200px; margin-left: 8px;",

                        SelectOption { value: "error1", "错误选项1" }
                        SelectOption { value: "error2", "错误选项2" }
                    }
                }

                div {
                    "警告状态:"
                    Select {
                        value: warning_value(),
                        placeholder: "警告状态",
                        status: SelectStatus::Warning,
                        on_change: move |v| warning_value.set(v),
                        style: "width: 200px; margin-left: 8px;",

                        SelectOption { value: "warning1", "警告选项1" }
                        SelectOption { value: "warning2", "警告选项2" }
                    }
                }
            }
        }
    }
}

#[component]
fn MultipleExample() -> Element {
    let mut values = use_signal(|| vec![]);

    rsx! {
        div {
            style: "margin-bottom: 32px;",

            h2 { "多选模式" }
            p { "支持多选的选择器，可以选择多个选项。" }

            div {
                style: "display: flex; gap: 16px; align-items: flex-start; flex-wrap: wrap;",

                Select {
                    values: values(),
                    placeholder: "请选择多个选项",
                    multiple: true,
                    on_change_multiple: move |v| {
                        values.set(v.clone());
                        println!("多选值: {:?}", v);
                    },
                    style: "width: 300px;",

                    SelectOption { value: "multi1", "多选选项1" }
                    SelectOption { value: "multi2", "多选选项2" }
                    SelectOption { value: "multi3", "多选选项3" }
                    SelectOption { value: "multi4", "多选选项4" }
                    SelectOption { value: "multi5", "多选选项5" }
                }

                div {
                    style: "padding: 8px; background: #f5f5f5; border-radius: 4px; max-width: 300px;",
                    "已选择: {values().join(", ")}"
                }
            }
        }
    }
}

#[component]
fn SearchExample() -> Element {
    let mut value = use_signal(|| String::new());
    let mut search_value = use_signal(|| String::new());

    rsx! {
        div {
            style: "margin-bottom: 32px;",

            h2 { "搜索功能" }
            p { "带搜索功能的选择器，可以输入关键词过滤选项。" }

            div {
                style: "display: flex; gap: 16px; align-items: flex-start; flex-wrap: wrap;",

                Select {
                    value: value(),
                    placeholder: "搜索并选择",
                    show_search: true,
                    filter_option: true,
                    on_change: move |v| value.set(v),
                    on_search: move |v| {
                        search_value.set(v.clone());
                        println!("搜索: {}", v);
                    },
                    style: "width: 250px;",

                    SelectOption { value: "apple", "苹果 (Apple)" }
                    SelectOption { value: "banana", "香蕉 (Banana)" }
                    SelectOption { value: "cherry", "樱桃 (Cherry)" }
                    SelectOption { value: "date", "枣子 (Date)" }
                    SelectOption { value: "elderberry", "接骨木果 (Elderberry)" }
                    SelectOption { value: "fig", "无花果 (Fig)" }
                    SelectOption { value: "grape", "葡萄 (Grape)" }
                }

                div {
                    style: "padding: 8px; background: #f5f5f5; border-radius: 4px;",
                    div { "选择值: {value()}" }
                    div { "搜索值: {search_value()}" }
                }
            }
        }
    }
}

#[component]
fn ClearableExample() -> Element {
    let mut value = use_signal(|| "option1".to_string());

    rsx! {
        div {
            style: "margin-bottom: 32px;",

            h2 { "清除功能" }
            p { "支持清除的选择器，可以一键清空选择。" }

            div {
                style: "display: flex; gap: 16px; align-items: center; flex-wrap: wrap;",

                Select {
                    value: value(),
                    placeholder: "可清除的选择器",
                    allow_clear: true,
                    on_change: move |v| value.set(v),
                    on_clear: move |_| {
                        println!("清除了选择");
                    },
                    style: "width: 200px;",

                    SelectOption { value: "option1", "选项1" }
                    SelectOption { value: "option2", "选项2" }
                    SelectOption { value: "option3", "选项3" }
                }

                button {
                    onclick: move |_| value.set("option2".to_string()),
                    "设置为选项2"
                }

                div {
                    style: "padding: 8px; background: #f5f5f5; border-radius: 4px;",
                    "当前值: {value()}"
                }
            }
        }
    }
}

#[component]
fn DisabledExample() -> Element {
    let mut value = use_signal(|| "disabled1".to_string());

    rsx! {
        div {
            style: "margin-bottom: 32px;",

            h2 { "禁用状态" }
            p { "禁用的选择器和禁用的选项。" }

            div {
                style: "display: flex; gap: 16px; align-items: center; flex-wrap: wrap;",

                div {
                    "禁用选择器:"
                    Select {
                        value: value(),
                        placeholder: "禁用的选择器",
                        disabled: true,
                        style: "width: 200px; margin-left: 8px;",

                        SelectOption { value: "disabled1", "禁用选项1" }
                        SelectOption { value: "disabled2", "禁用选项2" }
                    }
                }

                div {
                    "部分禁用:"
                    Select {
                        placeholder: "部分选项禁用",
                        style: "width: 200px; margin-left: 8px;",

                        SelectOption { value: "enabled1", "可用选项1" }
                        SelectOption { value: "disabled1", disabled: true, "禁用选项1" }
                        SelectOption { value: "enabled2", "可用选项2" }
                        SelectOption { value: "disabled2", disabled: true, "禁用选项2" }
                    }
                }
            }
        }
    }
}

#[component]
fn OptGroupExample() -> Element {
    let mut value = use_signal(|| String::new());

    rsx! {
        div {
            style: "margin-bottom: 32px;",

            h2 { "选项组" }
            p { "使用选项组对选项进行分类。" }

            div {
                style: "display: flex; gap: 16px; align-items: flex-start; flex-wrap: wrap;",

                Select {
                    value: value(),
                    placeholder: "选择城市",
                    on_change: move |v| value.set(v),
                    style: "width: 250px;",

                    SelectOptGroup {
                        label: "华北地区",

                        SelectOption { value: "beijing", "北京" }
                        SelectOption { value: "tianjin", "天津" }
                        SelectOption { value: "shijiazhuang", "石家庄" }
                    }

                    SelectOptGroup {
                        label: "华东地区",

                        SelectOption { value: "shanghai", "上海" }
                        SelectOption { value: "nanjing", "南京" }
                        SelectOption { value: "hangzhou", "杭州" }
                        SelectOption { value: "suzhou", "苏州" }
                    }

                    SelectOptGroup {
                        label: "华南地区",

                        SelectOption { value: "guangzhou", "广州" }
                        SelectOption { value: "shenzhen", "深圳" }
                        SelectOption { value: "xiamen", "厦门" }
                    }
                }

                div {
                    style: "padding: 8px; background: #f5f5f5; border-radius: 4px;",
                    "选择的城市: {value()}"
                }
            }
        }
    }
}

#[component]
fn BorderlessExample() -> Element {
    let mut value = use_signal(|| String::new());

    rsx! {
        div {
            style: "margin-bottom: 32px;",

            h2 { "无边框样式" }
            p { "无边框的选择器，适用于特殊布局场景。" }

            div {
                style: "display: flex; gap: 16px; align-items: center; flex-wrap: wrap; padding: 16px; background: #fafafa; border-radius: 8px;",

                div {
                    "无边框选择器:"
                    Select {
                        value: value(),
                        placeholder: "无边框",
                        bordered: false,
                        on_change: move |v| value.set(v),
                        style: "width: 200px; margin-left: 8px;",

                        SelectOption { value: "borderless1", "无边框选项1" }
                        SelectOption { value: "borderless2", "无边框选项2" }
                        SelectOption { value: "borderless3", "无边框选项3" }
                    }
                }

                div {
                    "对比 - 有边框:"
                    Select {
                        placeholder: "有边框",
                        bordered: true,
                        style: "width: 200px; margin-left: 8px;",

                        SelectOption { value: "bordered1", "有边框选项1" }
                        SelectOption { value: "bordered2", "有边框选项2" }
                        SelectOption { value: "bordered3", "有边框选项3" }
                    }
                }
            }
        }
    }
}

// 导入组件
use ant_design_dioxus::{Select, SelectOptGroup, SelectOption, SelectSize, SelectStatus};

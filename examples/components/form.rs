//!
//! 展示 Form 组件的各种用法和样式

use crate::common::*;
use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;
use std::collections::HashMap;

/// Form 组件演示
#[component]
pub fn FormDemo() -> Element {
    let mut form_data = use_signal(|| HashMap::<String, String>::new());
    let mut validation_errors = use_signal(|| HashMap::<String, String>::new());
    let mut submit_result = use_signal(|| String::new());
    let mut form_layout = use_signal(|| FormLayout::Horizontal);
    let mut form_size = use_signal(|| FormSize::Middle);

    // 处理表单提交
    let handle_submit = move |data: HashMap<String, String>| {
        submit_result.set(format!("表单提交成功: {:?}", data));
        web_sys::console::log_1(&format!("Form submitted: {:?}", data).into());
    };

    // 处理字段变化
    let mut handle_field_change = move |name: String, value: String| {
        let mut data = form_data();
        data.insert(name.clone(), value);
        form_data.set(data);

        // 清除该字段的错误信息
        let mut errors = validation_errors();
        errors.remove(&name);
        validation_errors.set(errors);
    };

    // 处理校验失败
    let handle_validation_failed = move |errors: HashMap<String, String>| {
        validation_errors.set(errors);
        submit_result.set("表单校验失败，请检查输入".to_string());
    };

    rsx! {
        div {
            class: "component-showcase",

            h1 {
                style: "margin: 0 0 16px 0; color: #262626; font-size: 28px;",
                "Form 表单"
            }

            p {
                style: "margin: 0 0 24px 0; color: #666; font-size: 14px;",
                "具有数据收集、校验和提交功能的表单，包含复选框、单选框、输入框、下拉选择框等元素。"
            }

            // 基础用法
            DemoSection {
                title: "基础用法",
                description: "基本的表单数据域控制展示，包含布局、初始化、验证、提交。",

                Form {
                    layout: FormLayout::Horizontal,
                    on_finish: handle_submit.clone(),
                    on_finish_failed: handle_validation_failed.clone(),

                    FormItem {
                        label: "用户名",
                        name: "username",
                        rules: vec![
                            FormRule::required("请输入用户名"),
                            FormRule::min_length(3, "用户名至少3个字符"),
                        ],

                        Input {
                            placeholder: "请输入用户名",
                            on_change: move |value| {
                                handle_field_change("username".to_string(), value);
                            }
                        }
                    }

                    FormItem {
                        label: "密码",
                        name: "password",
                        rules: vec![
                            FormRule::required("请输入密码"),
                            FormRule::min_length(6, "密码至少6个字符"),
                        ],

                        Input {
                            input_type: "password",
                            placeholder: "请输入密码",
                            on_change: move |value| {
                                handle_field_change("password".to_string(), value);
                            }
                        }
                    }

                    FormItem {
                        label: "邮箱",
                        name: "email",
                        rules: vec![
                            FormRule::required("请输入邮箱"),
                            FormRule::pattern(r"^[\w\.-]+@[\w\.-]+\.[a-zA-Z]{2,}$", "请输入有效的邮箱地址"),
                        ],

                        Input {
                            placeholder: "请输入邮箱",
                            on_change: move |value| {
                                handle_field_change("email".to_string(), value);
                            }
                        }
                    }

                    FormItem {
                        Button {
                            button_type: ButtonType::Primary,
                            html_type: ButtonHtmlType::Submit,
                            "提交"
                        }

                        Button {
                            style: "margin-left: 8px;",
                            "重置"
                        }
                    }
                }

                if !submit_result().is_empty() {
                    div {
                        style: "margin-top: 16px; padding: 12px; background: #f6ffed; border: 1px solid #b7eb8f; border-radius: 6px; color: #52c41a;",
                        "{submit_result}"
                    }
                }
            }

            // 表单布局
            DemoSection {
                title: "表单布局",
                description: "表单有三种布局。",

                div {
                    style: "margin-bottom: 16px;",
                    "布局类型: "

                    label {
                        style: "margin-right: 16px;",
                        input {
                            r#type: "radio",
                            name: "layout",
                            checked: form_layout() == FormLayout::Horizontal,
                            onchange: move |_| form_layout.set(FormLayout::Horizontal)
                        }
                        " 水平"
                    }

                    label {
                        style: "margin-right: 16px;",
                        input {
                            r#type: "radio",
                            name: "layout",
                            checked: form_layout() == FormLayout::Vertical,
                            onchange: move |_| form_layout.set(FormLayout::Vertical)
                        }
                        " 垂直"
                    }

                    label {
                        input {
                            r#type: "radio",
                            name: "layout",
                            checked: form_layout() == FormLayout::Inline,
                            onchange: move |_| form_layout.set(FormLayout::Inline)
                        }
                        " 内联"
                    }
                }

                Form {
                    layout: form_layout(),

                    FormItem {
                        label: "字段A",
                        name: "fieldA",

                        Input {
                            placeholder: "输入字段A"
                        }
                    }

                    FormItem {
                        label: "字段B",
                        name: "fieldB",

                        Input {
                            placeholder: "输入字段B"
                        }
                    }

                    FormItem {
                        Button {
                            button_type: ButtonType::Primary,
                            "提交"
                        }
                    }
                }
            }

            // 表单尺寸
            DemoSection {
                title: "表单尺寸",
                description: "设置表单组件尺寸，仅对 antd 组件有效。",

                div {
                    style: "margin-bottom: 16px;",
                    "尺寸: "

                    label {
                        style: "margin-right: 16px;",
                        input {
                            r#type: "radio",
                            name: "size",
                            checked: form_size() == FormSize::Small,
                            onchange: move |_| form_size.set(FormSize::Small)
                        }
                        " 小"
                    }

                    label {
                        style: "margin-right: 16px;",
                        input {
                            r#type: "radio",
                            name: "size",
                            checked: form_size() == FormSize::Middle,
                            onchange: move |_| form_size.set(FormSize::Middle)
                        }
                        " 中"
                    }

                    label {
                        input {
                            r#type: "radio",
                            name: "size",
                            checked: form_size() == FormSize::Large,
                            onchange: move |_| form_size.set(FormSize::Large)
                        }
                        " 大"
                    }
                }

                Form {
                    size: form_size(),

                    FormItem {
                        label: "输入框",
                        name: "input",

                        Input {
                            placeholder: "请输入"
                        }
                    }

                    FormItem {
                        label: "选择框",
                        name: "select",

                        Select {
                            placeholder: "请选择",
                            SelectOption { value: "option1", "选项1" }
                            SelectOption { value: "option2", "选项2" }
                            SelectOption { value: "option3", "选项3" }
                        }
                    }

                    FormItem {
                        Button {
                            button_type: ButtonType::Primary,
                            "提交"
                        }
                    }
                }
            }

            // 校验规则
            DemoSection {
                title: "校验规则",
                description: "表单校验，在用户提交表单时，对表单数据进行校验。",

                Form {
                    FormItem {
                        label: "必填项",
                        name: "required",
                        rules: vec![FormRule::required("此项为必填项")],

                        Input {
                            placeholder: "必填项"
                        }
                    }

                    FormItem {
                        label: "长度限制",
                        name: "length",
                        rules: vec![
                            FormRule::min_length(5, "至少5个字符"),
                            FormRule::max_length(20, "最多20个字符"),
                        ],

                        Input {
                            placeholder: "5-20个字符"
                        }
                    }

                    FormItem {
                        label: "手机号",
                        name: "phone",
                        rules: vec![
                            FormRule::required("请输入手机号"),
                            FormRule::pattern(r"^1[3-9]\d{9}$", "请输入正确的手机号"),
                        ],

                        Input {
                            placeholder: "请输入手机号"
                        }
                    }

                    FormItem {
                        label: "网址",
                        name: "url",
                        rules: vec![
                            FormRule::pattern(r"^https?://[\w\.-]+\.[a-zA-Z]{2,}.*$", "请输入有效的网址"),
                        ],

                        Input {
                            placeholder: "请输入网址"
                        }
                    }

                    FormItem {
                        Button {
                            button_type: ButtonType::Primary,
                            html_type: ButtonHtmlType::Submit,
                            "校验"
                        }
                    }
                }
            }

            // 动态增减表单项
            DemoSection {
                title: "动态增减表单项",
                description: "动态增加、减少表单项。",

                Form {
                    FormItem {
                        label: "用户",
                        name: "users",

                        div {
                            style: "display: flex; flex-direction: column; gap: 8px;",

                            div {
                                style: "display: flex; gap: 8px; align-items: center;",

                                Input {
                                    placeholder: "姓名",
                                    style: "flex: 1;"
                                }

                                Input {
                                    placeholder: "年龄",
                                    style: "width: 80px;"
                                }

                                Button {
                                    danger: true,
                                    size: ButtonSize::Small,
                                    "删除"
                                }
                            }

                            div {
                                style: "display: flex; gap: 8px; align-items: center;",

                                Input {
                                    placeholder: "姓名",
                                    style: "flex: 1;"
                                }

                                Input {
                                    placeholder: "年龄",
                                    style: "width: 80px;"
                                }

                                Button {
                                    danger: true,
                                    size: ButtonSize::Small,
                                    "删除"
                                }
                            }

                            Button {
                                button_type: ButtonType::Dashed,
                                style: "width: 100%;",
                                "+ 添加用户"
                            }
                        }
                    }

                    FormItem {
                        Button {
                            button_type: ButtonType::Primary,
                            "提交"
                        }
                    }
                }
            }

            // 复杂表单
            DemoSection {
                title: "复杂表单",
                description: "包含多种表单控件的复杂表单。",

                Form {
                    layout: FormLayout::Horizontal,

                    FormItem {
                        label: "基本信息",
                        name: "basic_info",

                        div {
                            style: "border: 1px solid #d9d9d9; border-radius: 6px; padding: 16px; background: #fafafa;",

                            FormItem {
                                label: "姓名",
                                name: "name",
                                rules: vec![FormRule::required("请输入姓名")],

                                Input {
                                    placeholder: "请输入姓名"
                                }
                            }

                            FormItem {
                                label: "性别",
                                name: "gender",

                                div {
                                    style: "display: flex; gap: 16px;",

                                    label {
                                        input {
                                            r#type: "radio",
                                            name: "gender",
                                            value: "male"
                                        }
                                        " 男"
                                    }

                                    label {
                                        input {
                                            r#type: "radio",
                                            name: "gender",
                                            value: "female"
                                        }
                                        " 女"
                                    }
                                }
                            }

                            FormItem {
                                label: "爱好",
                                name: "hobbies",

                                div {
                                    style: "display: flex; gap: 16px; flex-wrap: wrap;",

                                    label {
                                        input {
                                            r#type: "checkbox",
                                            value: "reading"
                                        }
                                        " 阅读"
                                    }

                                    label {
                                        input {
                                            r#type: "checkbox",
                                            value: "music"
                                        }
                                        " 音乐"
                                    }

                                    label {
                                        input {
                                            r#type: "checkbox",
                                            value: "sports"
                                        }
                                        " 运动"
                                    }

                                    label {
                                        input {
                                            r#type: "checkbox",
                                            value: "travel"
                                        }
                                        " 旅行"
                                    }
                                }
                            }
                        }
                    }

                    FormItem {
                        label: "联系方式",
                        name: "contact",

                        div {
                            style: "border: 1px solid #d9d9d9; border-radius: 6px; padding: 16px; background: #fafafa;",

                            FormItem {
                                label: "手机号",
                                name: "phone",
                                rules: vec![
                                    FormRule::required("请输入手机号"),
                                    FormRule::pattern(r"^1[3-9]\d{9}$", "请输入正确的手机号"),
                                ],

                                Input {
                                    placeholder: "请输入手机号"
                                }
                            }

                            FormItem {
                                label: "邮箱",
                                name: "email",
                                rules: vec![
                                    FormRule::pattern(r"^[\w\.-]+@[\w\.-]+\.[a-zA-Z]{2,}$", "请输入有效的邮箱地址"),
                                ],

                                Input {
                                    placeholder: "请输入邮箱"
                                }
                            }

                            FormItem {
                                label: "地址",
                                name: "address",

                                Input {
                                    placeholder: "请输入地址"
                                }
                            }
                        }
                    }

                    FormItem {
                        label: "备注",
                        name: "remark",

                        textarea {
                            placeholder: "请输入备注",
                            rows: "4",
                            style: "width: 100%; padding: 8px; border: 1px solid #d9d9d9; border-radius: 4px; resize: vertical;"
                        }
                    }

                    FormItem {
                        div {
                            style: "display: flex; gap: 8px;",

                            Button {
                                button_type: ButtonType::Primary,
                                html_type: ButtonHtmlType::Submit,
                                "提交"
                            }

                            Button {
                                "保存草稿"
                            }

                            Button {
                                "重置"
                            }
                        }
                    }
                }
            }
        }
    }
}

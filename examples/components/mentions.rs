//!
//! 展示 Mentions 组件的各种用法和样式

use crate::common::*;
use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;
use std::rc::Rc;

/// Mentions 组件演示
#[component]
pub fn MentionsDemo() -> Element {
    let mut basic_value = use_signal(|| String::new());
    let mut multiline_value = use_signal(|| String::new());
    let mut custom_prefix_value = use_signal(|| String::new());
    let mut readonly_value = use_signal(|| "只读模式 @afc163 @zombieJ".to_string());
    let mut placement_value = use_signal(|| String::new());
    let mut async_value = use_signal(|| String::new());
    let mut event_log = use_signal(|| Vec::<String>::new());
    let mut loading = use_signal(|| false);
    let mut async_options = use_signal(|| Vec::<MentionOption>::new());

    // 基础用户选项
    let basic_options = vec![
        MentionOption::new("afc163", "afc163"),
        MentionOption::new("zombieJ", "zombieJ"),
        MentionOption::new("yesmeck", "yesmeck"),
        MentionOption::new("react", "React"),
        MentionOption::new("ant-design", "Ant Design"),
        MentionOption::new("dioxus", "Dioxus"),
        MentionOption::new("rust", "Rust"),
    ];

    // 带头像的用户选项
    let user_options = vec![
        MentionOption::new("alice", "Alice Johnson"),
        MentionOption::new("bob", "Bob Smith"),
        MentionOption::new("charlie", "Charlie Brown"),
        MentionOption::new("diana", "Diana Prince"),
        MentionOption::new("eve", "Eve Wilson"),
    ];

    // 标签选项
    let tag_options = vec![
        MentionOption::new("frontend", "前端开发"),
        MentionOption::new("backend", "后端开发"),
        MentionOption::new("design", "UI设计"),
        MentionOption::new("testing", "测试"),
        MentionOption::new("devops", "运维"),
        MentionOption::new("mobile", "移动开发"),
    ];

    // 添加事件日志
    let mut add_log = move |message: String| {
        let mut logs = event_log();
        logs.insert(0, message);
        if logs.len() > 5 {
            logs.truncate(5);
        }
        event_log.set(logs);
    };

    // 模拟异步加载
    let load_async_options: Rc<dyn Fn(String) + 'static> = {
        let basic_options = basic_options.clone();
        let loading = loading.clone();
        let async_options = async_options.clone();
        Rc::new(move |search: String| {
            let basic_options = basic_options.clone();
            let mut loading = loading.clone();
            let mut async_options = async_options.clone();

            loading.set(true);

            // 模拟网络延迟
            spawn(async move {
                gloo_timers::future::TimeoutFuture::new(500).await;

                let filtered_options = basic_options
                    .iter()
                    .filter(|option| option.label.to_lowercase().contains(&search.to_lowercase()))
                    .cloned()
                    .collect::<Vec<_>>();

                async_options.set(filtered_options);
                loading.set(false);
            });
        })
    };

    rsx! {
        div {
            class: "component-showcase",

            h1 {
                style: "margin: 0 0 16px 0; color: #262626; font-size: 28px;",
                "Mentions 提及"
            }

            p {
                style: "margin: 0 0 24px 0; color: #666; font-size: 14px;",
                "提及组件，用于在输入中提及某人或某事，常用于发布、聊天或评论功能。"
            }

            // 基础用法
            DemoSection {
                title: "基础用法",
                description: "基本使用。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    Mentions {
                        value: basic_value(),
                        placeholder: "输入 @ 来提及某人",
                        options: basic_options.clone(),
                        on_change: move |value: String| {
                            basic_value.set(value.clone());
                            add_log(format!("基础输入变化: {}", value));
                        },
                        on_select: move |option: MentionOption| {
                            add_log(format!("选择了: {} ({})", option.label, option.value));
                        }
                    }

                    div {
                        style: "color: #666; font-size: 12px;",
                        "提示: 输入 @ 符号开始提及用户"
                    }
                }
            }

            // 三种大小
            DemoSection {
                title: "三种大小",
                description: "三种大小的提及组件。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",

                        label { "大号" }
                        Mentions {
                            size: MentionsSize::Large,
                            placeholder: "大号提及组件",
                            options: basic_options.clone()
                        }
                    }

                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",

                        label { "默认" }
                        Mentions {
                            size: MentionsSize::Middle,
                            placeholder: "默认提及组件",
                            options: basic_options.clone()
                        }
                    }

                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",

                        label { "小号" }
                        Mentions {
                            size: MentionsSize::Small,
                            placeholder: "小号提及组件",
                            options: basic_options.clone()
                        }
                    }
                }
            }

            // 多行模式
            DemoSection {
                title: "多行模式",
                description: "多行输入模式。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    Mentions {
                        value: multiline_value(),
                        placeholder: "这是一个多行输入框，输入 @ 来提及用户...",
                        options: user_options.clone(),
                        rows: Some(4),
                        on_change: move |value: String| {
                            multiline_value.set(value.clone());
                            add_log(format!("多行输入变化: {}", value));
                        }
                    }

                    div {
                        style: "color: #666; font-size: 12px;",
                        "支持多行文本输入和提及功能"
                    }
                }
            }

            // 自定义触发字符
            DemoSection {
                title: "自定义触发字符",
                description: "通过 prefix 自定义触发字符。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",

                        label { "使用 # 触发标签" }
                        Mentions {
                            value: custom_prefix_value(),
                            placeholder: "输入 # 来添加标签",
                            prefix: "#".to_string(),
                            options: tag_options.clone(),
                            on_change: move |value: String| {
                                custom_prefix_value.set(value.clone());
                                add_log(format!("标签输入变化: {}", value));
                            }
                        }
                    }

                    div {
                        style: "color: #666; font-size: 12px;",
                        "提示: 输入 # 符号开始添加标签"
                    }
                }
            }

            // 状态
            DemoSection {
                title: "状态",
                description: "不同状态的提及组件。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",

                        label { "默认状态" }
                        Mentions {
                            status: MentionsStatus::Default,
                            placeholder: "默认状态",
                            options: basic_options.clone()
                        }
                    }

                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",

                        label { "错误状态" }
                        Mentions {
                            status: MentionsStatus::Error,
                            placeholder: "错误状态",
                            options: basic_options.clone()
                        }
                    }

                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",

                        label { "警告状态" }
                        Mentions {
                            status: MentionsStatus::Warning,
                            placeholder: "警告状态",
                            options: basic_options.clone()
                        }
                    }
                }
            }

            // 禁用状态
            DemoSection {
                title: "禁用状态",
                description: "禁用状态的提及组件。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    Mentions {
                        disabled: true,
                        value: "这是一个禁用的提及组件 @afc163".to_string(),
                        placeholder: "禁用状态",
                        options: basic_options.clone()
                    }

                    div {
                        style: "color: #666; font-size: 12px;",
                        "禁用状态下无法编辑内容"
                    }
                }
            }

            // 只读状态
            DemoSection {
                title: "只读状态",
                description: "只读状态的提及组件。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    Mentions {
                        readonly: true,
                        value: readonly_value(),
                        placeholder: "只读状态",
                        options: basic_options.clone()
                    }

                    div {
                        style: "color: #666; font-size: 12px;",
                        "只读状态下可以查看但无法编辑"
                    }
                }
            }

            // 无边框
            DemoSection {
                title: "无边框",
                description: "无边框样式。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    div {
                        style: "background: #f5f5f5; padding: 16px; border-radius: 6px;",

                        Mentions {
                            bordered: false,
                            placeholder: "无边框提及组件",
                            options: basic_options.clone()
                        }
                    }

                    div {
                        style: "color: #666; font-size: 12px;",
                        "适合嵌入到其他容器中使用"
                    }
                }
            }

            // 自动调整高度
            DemoSection {
                title: "自动调整高度",
                description: "根据内容自动调整高度。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    Mentions {
                        placeholder: "输入多行内容，高度会自动调整...",
                        options: basic_options.clone(),
                        auto_size: true,
                        min_rows: Some(2),
                        max_rows: Some(6)
                    }

                    div {
                        style: "color: #666; font-size: 12px;",
                        "最小2行，最大6行，根据内容自动调整"
                    }
                }
            }

            // 自定义选项渲染
            DemoSection {
                title: "自定义选项渲染",
                description: "自定义提及选项的显示方式。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    Mentions {
                        placeholder: "输入 @ 查看自定义选项样式",
                        options: user_options.clone(),
                        on_change: move |value: String| {
                            add_log(format!("自定义选项输入: {}", value));
                        }
                    }

                    div {
                        style: "color: #666; font-size: 12px;",
                        "选项可以包含头像、描述等自定义内容"
                    }
                }
            }

            // 异步加载
            DemoSection {
                title: "异步加载",
                description: "动态加载提及选项。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    Mentions {
                        value: async_value(),
                        placeholder: "输入 @ 开始搜索用户（模拟异步加载）",
                        options: async_options(),
                        on_change: {
                            let load_async_options = load_async_options.clone();
                            move |value: String| {
                                async_value.set(value.clone());

                                // 检查是否输入了@符号，触发搜索
                                if value.ends_with('@') {
                                    load_async_options(String::new());
                                } else if let Some(pos) = value.rfind('@') {
                                    let search_term = &value[pos + 1..];
                                    if !search_term.contains(' ') {
                                        load_async_options(search_term.to_string());
                                    }
                                }
                                let mut add_log_clone = add_log.clone();
                                add_log_clone(format!("异步加载输入: {}", value));
                            }
                        }
                    }

                    if loading() {
                        div {
                            style: "color: #1890ff; font-size: 12px;",
                            "🔄 正在加载选项..."
                        }
                    } else {
                        div {
                            style: "color: #666; font-size: 12px;",
                            "输入 @ 后会动态搜索匹配的用户"
                        }
                    }
                }
            }

            // 事件回调
            DemoSection {
                title: "事件回调",
                description: "演示各种事件回调。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    Mentions {
                        placeholder: "输入内容测试事件回调",
                        options: basic_options.clone(),
                        on_change: {
                            let mut add_log = add_log.clone();
                            move |value: String| {
                                add_log(format!("onChange: {}", value));
                            }
                        },
                        on_select: {
                            let mut add_log = add_log.clone();
                            move |option: MentionOption| {
                                add_log(format!("onSelect: {} ({})", option.label, option.value));
                            }
                        }
                    }

                    div {
                        style: "background: #f5f5f5; padding: 12px; border-radius: 6px; max-height: 150px; overflow-y: auto;",

                        h4 {
                            style: "margin: 0 0 8px 0; color: #333;",
                            "事件日志:"
                        }

                        if event_log().is_empty() {
                            div {
                                style: "color: #999; font-style: italic;",
                                "暂无事件"
                            }
                        } else {
                            for (index, log) in event_log().iter().enumerate() {
                                div {
                                    key: "{index}",
                                    style: "font-family: monospace; font-size: 12px; color: #666; margin-bottom: 4px;",
                                    "{log}"
                                }
                            }
                        }
                    }
                }
            }

            // 自定义样式
            DemoSection {
                title: "自定义样式",
                description: "通过 class 和 style 属性自定义样式。",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    Mentions {
                        placeholder: "自定义边框样式",
                        options: basic_options.clone(),
                        style: "border: 2px solid #1890ff; border-radius: 8px; background: #f0f8ff;"
                    }

                    Mentions {
                        placeholder: "自定义字体样式",
                        options: basic_options.clone(),
                        style: "font-family: 'Courier New', monospace; font-size: 16px; line-height: 1.8;",
                        rows: Some(3)
                    }

                    Mentions {
                        placeholder: "渐变背景样式",
                        options: basic_options.clone(),
                        style: "background: linear-gradient(45deg, #ff6b6b, #4ecdc4); color: white; border: none; border-radius: 12px;"
                    }
                }
            }
        }
    }
}

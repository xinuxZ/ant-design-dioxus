use crate::common::demo_section::DemoSection;
use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

/// TreeSelect组件示例
#[component]
pub fn TreeSelectDemo() -> Element {
    rsx! {
        div { class: "demo-container",
            h1 { "TreeSelect 树选择" }
            p { "树型选择控件。" }

            DemoSection {
                title: "基本使用",
                description: "最简单的用法。",

                div {
                    TreeSelect {
                        style: "width: 300px; height: 32px; border: 1px solid #d9d9d9; border-radius: 6px; padding: 4px 11px; background: white;",
                        placeholder: "请选择"
                    }
                }
            }

            DemoSection {
                title: "多选模式",
                description: "支持多选的树选择器。",

                div {
                    TreeSelect {
                        class: "tree-select-multiple",
                        style: "width: 300px; min-height: 32px; border: 1px solid #d9d9d9; border-radius: 6px; padding: 4px 11px; background: white;",
                        placeholder: "请选择多个选项"
                    }
                }
            }

            DemoSection {
                title: "带搜索功能",
                description: "可搜索的树选择器。",

                div {
                    TreeSelect {
                        class: "tree-select-searchable",
                        style: "width: 300px; height: 32px; border: 1px solid #d9d9d9; border-radius: 6px; padding: 4px 11px; background: white; position: relative;",
                        placeholder: "搜索并选择"
                    }
                }
            }

            DemoSection {
                title: "不同尺寸",
                description: "三种尺寸的树选择器。",

                div {
                    div { style: "display: flex; flex-direction: column; gap: 16px;",
                        TreeSelect {
                            class: "tree-select-large",
                            style: "width: 300px; height: 40px; border: 1px solid #d9d9d9; border-radius: 6px; padding: 8px 11px; background: white;",
                            placeholder: "大尺寸"
                        }
                        TreeSelect {
                            class: "tree-select-default",
                            style: "width: 300px; height: 32px; border: 1px solid #d9d9d9; border-radius: 6px; padding: 4px 11px; background: white;",
                            placeholder: "默认尺寸"
                        }
                        TreeSelect {
                            class: "tree-select-small",
                            style: "width: 300px; height: 24px; border: 1px solid #d9d9d9; border-radius: 6px; padding: 0px 7px; background: white; font-size: 12px;",
                            placeholder: "小尺寸"
                        }
                    }
                }
            }

            DemoSection {
                title: "禁用状态",
                description: "禁用状态的树选择器。",

                div {
                    TreeSelect {
                        class: "tree-select-disabled",
                        style: "width: 300px; height: 32px; border: 1px solid #d9d9d9; border-radius: 6px; padding: 4px 11px; background: #f5f5f5; color: #00000040; cursor: not-allowed;",
                        placeholder: "禁用状态"
                    }
                }
            }

            DemoSection {
                title: "自定义样式",
                description: "可以自定义树选择器的样式。",

                div {
                    TreeSelect {
                        class: "tree-select-custom",
                        style: "width: 300px; height: 32px; border: 2px solid #1677ff; border-radius: 8px; padding: 4px 11px; background: white;",
                        placeholder: "自定义样式"
                    }
                }
            }
        }
    }
}

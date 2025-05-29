#![allow(non_snake_case)]
//!
//! 展示 Collapse 组件的各种用法和样式

use crate::common::*;
use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

/// Collapse 组件演示
#[component]
pub fn CollapseDemo() -> Element {
    rsx! {
        div {
            class: "component-showcase",

            h1 {
                style: "margin: 0 0 16px 0; color: #262626; font-size: 28px;",
                "Collapse 折叠面板"
            }

            p {
                style: "margin: 0 0 24px 0; color: #666; font-size: 14px;",
                "可以折叠/展开的内容区域。"
            }

            // 基础用法
            DemoSection {
                title: "基础用法",
                description: "可以同时展开多个面板，这个例子默认展开了第一个。",

                div {
                    style: "width: 100%;",

                    Collapse {
                        default_active_key: vec!["1".to_string()],

                        CollapsePanel {
                            key: "1",
                            header: "This is panel header 1",
                            p { "A dog is a type of domesticated animal. Known for its loyalty and faithfulness, it can be found as a welcome guest in many households across the world." }
                        }

                        CollapsePanel {
                            key: "2",
                            header: "This is panel header 2",
                            p { "A dog is a type of domesticated animal. Known for its loyalty and faithfulness, it can be found as a welcome guest in many households across the world." }
                        }

                        CollapsePanel {
                            key: "3",
                            header: "This is panel header 3",
                            p { "A dog is a type of domesticated animal. Known for its loyalty and faithfulness, it can be found as a welcome guest in many households across the world." }
                        }
                    }
                }
            }

            // 手风琴
            DemoSection {
                title: "手风琴",
                description: "手风琴，每次只打开一个tab。",

                div {
                    style: "width: 100%;",

                    Collapse {
                        accordion: true,

                        CollapsePanel {
                            key: "1",
                            header: "This is panel header 1",
                            p { "A dog is a type of domesticated animal. Known for its loyalty and faithfulness, it can be found as a welcome guest in many households across the world." }
                        }

                        CollapsePanel {
                            key: "2",
                            header: "This is panel header 2",
                            p { "A dog is a type of domesticated animal. Known for its loyalty and faithfulness, it can be found as a welcome guest in many households across the world." }
                        }

                        CollapsePanel {
                            key: "3",
                            header: "This is panel header 3",
                            p { "A dog is a type of domesticated animal. Known for its loyalty and faithfulness, it can be found as a welcome guest in many households across the world." }
                        }
                    }
                }
            }

            // 面板嵌套
            DemoSection {
                title: "面板嵌套",
                description: "嵌套折叠面板。",

                div {
                    style: "width: 100%;",

                    Collapse {
                        CollapsePanel {
                            key: "1",
                            header: "This is panel header 1",

                            Collapse {
                                default_active_key: vec!["1".to_string()],

                                CollapsePanel {
                                    key: "1",
                                    header: "This is panel nest panel",
                                    p { "A dog is a type of domesticated animal. Known for its loyalty and faithfulness, it can be found as a welcome guest in many households across the world." }
                                }
                            }
                        }
                    }
                }
            }

            // 简洁风格
            DemoSection {
                title: "简洁风格",
                description: "一套没有边框的简洁样式。",

                div {
                    style: "width: 100%;",

                    Collapse {
                        bordered: false,
                        default_active_key: vec!["1".to_string()],

                        CollapsePanel {
                            key: "1",
                            header: "This is panel header 1",
                            p { "A dog is a type of domesticated animal. Known for its loyalty and faithfulness, it can be found as a welcome guest in many households across the world." }
                        }

                        CollapsePanel {
                            key: "2",
                            header: "This is panel header 2",
                            p { "A dog is a type of domesticated animal. Known for its loyalty and faithfulness, it can be found as a welcome guest in many households across the world." }
                        }

                        CollapsePanel {
                            key: "3",
                            header: "This is panel header 3",
                            p { "A dog is a type of domesticated animal. Known for its loyalty and faithfulness, it can be found as a welcome guest in many households across the world." }
                        }
                    }
                }
            }

            // API 文档
            ApiDocumentation {
                component_name: "Collapse",
                props: vec![
                    PropDoc {
                        name: "accordion".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "手风琴模式".to_string(),
                    },
                    PropDoc {
                        name: "bordered".to_string(),
                        prop_type: "bool".to_string(),
                        default: "true".to_string(),
                        description: "带边框风格的折叠面板".to_string(),
                    },
                    PropDoc {
                        name: "default_active_key".to_string(),
                        prop_type: "Vec<String>".to_string(),
                        default: "[]".to_string(),
                        description: "初始化选中面板的 key".to_string(),
                    },
                    PropDoc {
                        name: "ghost".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "使折叠面板透明且无边框".to_string(),
                    },
                ]
            }
        }
    }
}

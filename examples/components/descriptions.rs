#![allow(non_snake_case)]
//!
//! 展示 Descriptions 组件的各种用法和样式

use crate::common::*;
use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

/// Descriptions 组件演示
#[component]
pub fn DescriptionsDemo() -> Element {
    rsx! {
        div {
            class: "component-showcase",

            h1 {
                style: "margin: 0 0 16px 0; color: #262626; font-size: 28px;",
                "Descriptions 描述列表"
            }

            p {
                style: "margin: 0 0 24px 0; color: #666; font-size: 14px;",
                "成组展示多个只读字段。"
            }

            // 基础用法
            DemoSection {
                title: "基础用法",
                description: "简单的展示。",

                div {
                    style: "width: 100%;",

                    Descriptions {
                        title: "User Info",

                        DescriptionsItem {
                            label: "UserName",
                            "Zhou Maomao"
                        }

                        DescriptionsItem {
                            label: "Telephone",
                            "1810000000"
                        }

                        DescriptionsItem {
                            label: "Live",
                            "Hangzhou, Zhejiang"
                        }

                        DescriptionsItem {
                            label: "Remark",
                            "empty"
                        }

                        DescriptionsItem {
                            label: "Address",
                            "No. 18, Wantang Road, Xihu District, Hangzhou, Zhejiang, China"
                        }
                    }
                }
            }

            // 带边框的
            DemoSection {
                title: "带边框的",
                description: "带边框和背景颜色列表。",

                div {
                    style: "width: 100%;",

                    Descriptions {
                        title: "User Info",
                        bordered: true,

                        DescriptionsItem {
                            label: "Product",
                            "Cloud Database"
                        }

                        DescriptionsItem {
                            label: "Billing Mode",
                            "Prepaid"
                        }

                        DescriptionsItem {
                            label: "Automatic Renewal",
                            "YES"
                        }

                        DescriptionsItem {
                            label: "Order time",
                            "2018-04-24 18:00:00"
                        }

                        DescriptionsItem {
                            label: "Usage Time",
                            span: 2,
                            "2019-04-24 18:00:00"
                        }

                        DescriptionsItem {
                            label: "Status",
                            span: 3,
                            Badge {
                                status: "processing",
                                text: "Running"
                            }
                        }

                        DescriptionsItem {
                            label: "Negotiated Amount",
                            "$80.00"
                        }

                        DescriptionsItem {
                            label: "Discount",
                            "$20.00"
                        }

                        DescriptionsItem {
                            label: "Official Receipts",
                            "$60.00"
                        }

                        DescriptionsItem {
                            label: "Config Info",
                            "Data disk type: MongoDB"
                            br {}
                            "Database version: 3.4"
                            br {}
                            "Package: dds.mongo.mid"
                            br {}
                            "Storage space: 10 GB"
                            br {}
                            "Replication factor: 3"
                            br {}
                            "Region: East China 1"
                        }
                    }
                }
            }

            // 自定义尺寸
            DemoSection {
                title: "自定义尺寸",
                description: "自定义尺寸，适应在各种容器中展示。",

                div {
                    style: "width: 100%;",

                    Descriptions {
                        title: "Custom Size",
                        size: "small",

                        DescriptionsItem {
                            label: "Product",
                            "Cloud Database"
                        }

                        DescriptionsItem {
                            label: "Billing",
                            "Prepaid"
                        }

                        DescriptionsItem {
                            label: "Time",
                            "18:00:00"
                        }

                        DescriptionsItem {
                            label: "Amount",
                            "$80.00"
                        }

                        DescriptionsItem {
                            label: "Discount",
                            "$20.00"
                        }

                        DescriptionsItem {
                            label: "Official",
                            "$60.00"
                        }
                    }
                }
            }

            // 响应式
            DemoSection {
                title: "响应式",
                description: "通过响应式的配置可以实现在小屏幕设备上的完美呈现。",

                div {
                    style: "width: 100%;",

                    Descriptions {
                        title: "Responsive Descriptions",
                        bordered: true,
                        column: 2,

                        DescriptionsItem {
                            label: "Product",
                            "Cloud Database"
                        }

                        DescriptionsItem {
                            label: "Billing",
                            "Prepaid"
                        }

                        DescriptionsItem {
                            label: "Time",
                            "18:00:00"
                        }

                        DescriptionsItem {
                            label: "Amount",
                            "$80.00"
                        }
                    }
                }
            }

            // API 文档
            ApiDocumentation {
                component_name: "Descriptions",
                props: vec![
                    PropDoc {
                        name: "title".to_string(),
                        prop_type: "String".to_string(),
                        default: "-".to_string(),
                        description: "描述列表的标题，显示在最顶部".to_string(),
                    },
                    PropDoc {
                        name: "bordered".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "是否展示边框".to_string(),
                    },
                    PropDoc {
                        name: "column".to_string(),
                        prop_type: "u32".to_string(),
                        default: "3".to_string(),
                        description: "一行的 DescriptionItems 数量，可以写成像素值或支持响应式的对象写法".to_string(),
                    },
                    PropDoc {
                        name: "size".to_string(),
                        prop_type: "String".to_string(),
                        default: "default".to_string(),
                        description: "设置列表的大小。可以设置为 middle 、small, 或不填（只有设置 bordered={true} 生效）".to_string(),
                    },
                    PropDoc {
                        name: "layout".to_string(),
                        prop_type: "String".to_string(),
                        default: "horizontal".to_string(),
                        description: "描述布局".to_string(),
                    },
                ]
            }
        }
    }
}

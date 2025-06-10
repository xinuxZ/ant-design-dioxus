#![allow(non_snake_case)]

use dioxus::prelude::*;

// 组件页面
#[component]
pub fn ComponentsPage() -> Element {
    rsx! {
        div {
            style: "padding: 24px; max-width: 1200px; margin: 0 auto;",

            // 页面标题
            h1 {
                style: "margin-bottom: 24px; color: #262626; font-size: 28px;",
                "组件库"
            }

            // 组件分类描述
            p {
                style: "margin-bottom: 40px; color: #666; font-size: 16px;",
                "Ant Design 组件库提供了丰富的组件，涵盖了常见的交互场景和业务需求。"
            }

            // 组件列表
            div {
                style: "display: flex; flex-wrap: wrap;",

                // 常规组件区域
                div {
                    style: "width: 100%; margin-bottom: 40px;",

                    h2 {
                        style: "margin-bottom: 16px; color: #262626; font-size: 20px; border-bottom: 1px solid #f0f0f0; padding-bottom: 8px;",
                        "通用组件"
                    }

                    div {
                        style: "display: flex; flex-wrap: wrap;",

                        // 按钮
                        a {
                            class: "component-link",
                            href: "#/components/button",
                            style: "text-decoration: none; color: #1677ff; margin-right: 16px; margin-bottom: 16px; display: inline-block;",
                            "按钮 Button"
                        }

                        // 图标
                        a {
                            class: "component-link",
                            href: "#/components/icon",
                            style: "text-decoration: none; color: #1677ff; margin-right: 16px; margin-bottom: 16px; display: inline-block;",
                            "图标 Icon"
                        }

                        // 排版
                        a {
                            class: "component-link",
                            href: "#/components/typography",
                            style: "text-decoration: none; color: #1677ff; margin-right: 16px; margin-bottom: 16px; display: inline-block;",
                            "排版 Typography"
                        }
                    }
                }

                // 布局组件区域
                div {
                    style: "width: 100%; margin-bottom: 40px;",

                    h2 {
                        style: "margin-bottom: 16px; color: #262626; font-size: 20px; border-bottom: 1px solid #f0f0f0; padding-bottom: 8px;",
                        "布局组件"
                    }

                    div {
                        style: "display: flex; flex-wrap: wrap;",

                        // 布局
                        a {
                            class: "component-link",
                            href: "#/components/layout",
                            style: "text-decoration: none; color: #1677ff; margin-right: 16px; margin-bottom: 16px; display: inline-block;",
                            "布局 Layout"
                        }

                        // 栅格
                        a {
                            class: "component-link",
                            href: "#/components/grid",
                            style: "text-decoration: none; color: #1677ff; margin-right: 16px; margin-bottom: 16px; display: inline-block;",
                            "栅格 Grid"
                        }

                        // 间距
                        a {
                            class: "component-link",
                            href: "#/components/space",
                            style: "text-decoration: none; color: #1677ff; margin-right: 16px; margin-bottom: 16px; display: inline-block;",
                            "间距 Space"
                        }
                    }
                }

                // 数据输入组件区域
                div {
                    style: "width: 100%; margin-bottom: 40px;",

                    h2 {
                        style: "margin-bottom: 16px; color: #262626; font-size: 20px; border-bottom: 1px solid #f0f0f0; padding-bottom: 8px;",
                        "数据输入"
                    }

                    div {
                        style: "display: flex; flex-wrap: wrap;",

                        // 输入框
                        a {
                            class: "component-link",
                            href: "#/components/input",
                            style: "text-decoration: none; color: #1677ff; margin-right: 16px; margin-bottom: 16px; display: inline-block;",
                            "输入框 Input"
                        }

                        // 表单
                        a {
                            class: "component-link",
                            href: "#/components/form",
                            style: "text-decoration: none; color: #1677ff; margin-right: 16px; margin-bottom: 16px; display: inline-block;",
                            "表单 Form"
                        }

                        // 选择器
                        a {
                            class: "component-link",
                            href: "#/components/select",
                            style: "text-decoration: none; color: #1677ff; margin-right: 16px; margin-bottom: 16px; display: inline-block;",
                            "选择器 Select"
                        }
                    }
                }

                // 数据展示组件区域
                div {
                    style: "width: 100%; margin-bottom: 40px;",

                    h2 {
                        style: "margin-bottom: 16px; color: #262626; font-size: 20px; border-bottom: 1px solid #f0f0f0; padding-bottom: 8px;",
                        "数据展示"
                    }

                    div {
                        style: "display: flex; flex-wrap: wrap;",

                        // 表格
                        a {
                            class: "component-link",
                            href: "#/components/table",
                            style: "text-decoration: none; color: #1677ff; margin-right: 16px; margin-bottom: 16px; display: inline-block;",
                            "表格 Table"
                        }

                        // 卡片
                        a {
                            class: "component-link",
                            href: "#/components/card",
                            style: "text-decoration: none; color: #1677ff; margin-right: 16px; margin-bottom: 16px; display: inline-block;",
                            "卡片 Card"
                        }

                        // 标签页
                        a {
                            class: "component-link",
                            href: "#/components/tabs",
                            style: "text-decoration: none; color: #1677ff; margin-right: 16px; margin-bottom: 16px; display: inline-block;",
                            "标签页 Tabs"
                        }

                        // 日历
                        a {
                            class: "component-link",
                            href: "#/components/calendar",
                            style: "text-decoration: none; color: #1677ff; margin-right: 16px; margin-bottom: 16px; display: inline-block;",
                            "日历 Calendar"
                        }
                    }
                }

                // 反馈组件区域
                div {
                    style: "width: 100%; margin-bottom: 40px;",

                    h2 {
                        style: "margin-bottom: 16px; color: #262626; font-size: 20px; border-bottom: 1px solid #f0f0f0; padding-bottom: 8px;",
                        "反馈组件"
                    }

                    div {
                        style: "display: flex; flex-wrap: wrap;",

                        // 对话框
                        a {
                            class: "component-link",
                            href: "#/components/modal",
                            style: "text-decoration: none; color: #1677ff; margin-right: 16px; margin-bottom: 16px; display: inline-block;",
                            "对话框 Modal"
                        }

                        // 消息
                        a {
                            class: "component-link",
                            href: "#/components/message",
                            style: "text-decoration: none; color: #1677ff; margin-right: 16px; margin-bottom: 16px; display: inline-block;",
                            "消息 Message"
                        }

                        // 通知
                        a {
                            class: "component-link",
                            href: "#/components/notification",
                            style: "text-decoration: none; color: #1677ff; margin-right: 16px; margin-bottom: 16px; display: inline-block;",
                            "通知 Notification"
                        }
                    }
                }

                // 导航组件区域
                div {
                    style: "width: 100%; margin-bottom: 40px;",

                    h2 {
                        style: "margin-bottom: 16px; color: #262626; font-size: 20px; border-bottom: 1px solid #f0f0f0; padding-bottom: 8px;",
                        "导航组件"
                    }

                    div {
                        style: "display: flex; flex-wrap: wrap;",

                        // 菜单
                        a {
                            class: "component-link",
                            href: "#/components/menu",
                            style: "text-decoration: none; color: #1677ff; margin-right: 16px; margin-bottom: 16px; display: inline-block;",
                            "菜单 Menu"
                        }

                        // 分页
                        a {
                            class: "component-link",
                            href: "#/components/pagination",
                            style: "text-decoration: none; color: #1677ff; margin-right: 16px; margin-bottom: 16px; display: inline-block;",
                            "分页 Pagination"
                        }
                    }
                }

                // 其他组件区域
                div {
                    style: "width: 100%; margin-bottom: 40px;",

                    h2 {
                        style: "margin-bottom: 16px; color: #262626; font-size: 20px; border-bottom: 1px solid #f0f0f0; padding-bottom: 8px;",
                        "其他组件"
                    }

                    div {
                        style: "display: flex; flex-wrap: wrap;",

                        // 国际化和主题
                        a {
                            class: "component-link",
                            href: "#/components/i18n_demo",
                            style: "text-decoration: none; color: #1677ff; margin-right: 16px; margin-bottom: 16px; display: inline-block;",
                            "国际化和主题"
                        }

                        // 主题与国际化
                        a {
                            class: "component-link",
                            href: "#/components/theme_i18n",
                            style: "text-decoration: none; color: #1677ff; margin-right: 16px; margin-bottom: 16px; display: inline-block;",
                            "主题与国际化"
                        }
                    }
                }
            }
        }
    }
}

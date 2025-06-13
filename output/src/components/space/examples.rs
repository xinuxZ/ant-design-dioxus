//! Space 组件使用示例
//!
//! 展示 Space 组件的各种使用方式和配置选项。

use dioxus::prelude::*;
use super::*;

/// 基础间距示例
pub fn basic_space_example() -> Element {
    rsx! {
        Space {
            button { "按钮1" }
            button { "按钮2" }
            button { "按钮3" }
        }
    }
}

/// 垂直间距示例
pub fn vertical_space_example() -> Element {
    rsx! {
        Space {
            direction: SpaceDirection::Vertical,
            button { "按钮1" }
            button { "按钮2" }
            button { "按钮3" }
        }
    }
}

/// 不同尺寸间距示例
pub fn space_sizes_example() -> Element {
    rsx! {
        div {
            h3 { "小间距" }
            Space {
                size: SpaceSize::Small,
                button { "按钮1" }
                button { "按钮2" }
                button { "按钮3" }
            }
            
            h3 { "中间距" }
            Space {
                size: SpaceSize::Middle,
                button { "按钮1" }
                button { "按钮2" }
                button { "按钮3" }
            }
            
            h3 { "大间距" }
            Space {
                size: SpaceSize::Large,
                button { "按钮1" }
                button { "按钮2" }
                button { "按钮3" }
            }
            
            h3 { "自定义间距" }
            Space {
                size: SpaceSize::Custom(32),
                button { "按钮1" }
                button { "按钮2" }
                button { "按钮3" }
            }
        }
    }
}

/// 对齐方式示例
pub fn space_align_example() -> Element {
    rsx! {
        div {
            h3 { "起始对齐" }
            Space {
                align: SpaceAlign::Start,
                button { style: "height: 32px;", "按钮1" }
                button { style: "height: 40px;", "按钮2" }
                button { style: "height: 64px;", "按钮3" }
            }
            
            h3 { "居中对齐" }
            Space {
                align: SpaceAlign::Center,
                button { style: "height: 32px;", "按钮1" }
                button { style: "height: 40px;", "按钮2" }
                button { style: "height: 64px;", "按钮3" }
            }
            
            h3 { "结束对齐" }
            Space {
                align: SpaceAlign::End,
                button { style: "height: 32px;", "按钮1" }
                button { style: "height: 40px;", "按钮2" }
                button { style: "height: 64px;", "按钮3" }
            }
            
            h3 { "基线对齐" }
            Space {
                align: SpaceAlign::Baseline,
                button { style: "height: 32px;", "按钮1" }
                button { style: "height: 40px;", "按钮2" }
                button { style: "height: 64px;", "按钮3" }
            }
        }
    }
}

/// 自动换行示例
pub fn space_wrap_example() -> Element {
    rsx! {
        div {
            style: "width: 300px; border: 1px solid #d9d9d9; padding: 16px;",
            h3 { "自动换行" }
            Space {
                wrap: true,
                button { "按钮1" }
                button { "按钮2" }
                button { "按钮3" }
                button { "按钮4" }
                button { "按钮5" }
                button { "按钮6" }
                button { "按钮7" }
                button { "按钮8" }
            }
        }
    }
}

/// 分隔符示例
pub fn space_split_example() -> Element {
    rsx! {
        div {
            h3 { "使用分隔符" }
            Space {
                split: rsx! { span { style: "color: #d9d9d9;", "|" } },
                a { href: "#", "链接1" }
                a { href: "#", "链接2" }
                a { href: "#", "链接3" }
            }
            
            h3 { "垂直分隔符" }
            Space {
                direction: SpaceDirection::Vertical,
                split: rsx! { div { style: "width: 100%; height: 1px; background: #d9d9d9;", } },
                div { "内容1" }
                div { "内容2" }
                div { "内容3" }
            }
        }
    }
}

/// 复杂布局示例
pub fn complex_space_example() -> Element {
    rsx! {
        div {
            h3 { "复杂布局" }
            Space {
                direction: SpaceDirection::Vertical,
                size: SpaceSize::Large,
                
                // 水平按钮组
                Space {
                    button { "主要按钮" }
                    button { "次要按钮" }
                    button { "危险按钮" }
                }
                
                // 表单行
                Space {
                    align: SpaceAlign::Center,
                    label { "用户名:" }
                    input { r#type: "text", placeholder: "请输入用户名" }
                    button { "搜索" }
                }
                
                // 链接组
                Space {
                    split: rsx! { span { style: "color: #d9d9d9;", "|" } },
                    a { href: "#", "首页" }
                    a { href: "#", "产品" }
                    a { href: "#", "关于我们" }
                    a { href: "#", "联系我们" }
                }
            }
        }
    }
}

/// 响应式间距示例
pub fn responsive_space_example() -> Element {
    rsx! {
        div {
            h3 { "响应式间距" }
            Space {
                size: SpaceSize::Middle,
                wrap: true,
                
                div { 
                    style: "padding: 8px 16px; background: #f0f0f0; border-radius: 4px;",
                    "卡片1" 
                }
                div { 
                    style: "padding: 8px 16px; background: #f0f0f0; border-radius: 4px;",
                    "卡片2" 
                }
                div { 
                    style: "padding: 8px 16px; background: #f0f0f0; border-radius: 4px;",
                    "卡片3" 
                }
                div { 
                    style: "padding: 8px 16px; background: #f0f0f0; border-radius: 4px;",
                    "卡片4" 
                }
                div { 
                    style: "padding: 8px 16px; background: #f0f0f0; border-radius: 4px;",
                    "卡片5" 
                }
            }
        }
    }
}

/// 自定义样式示例
pub fn custom_space_example() -> Element {
    rsx! {
        div {
            h3 { "自定义样式" }
            Space {
                class: "custom-space",
                style: "background: #fafafa; padding: 16px; border-radius: 8px;",
                size: SpaceSize::Large,
                
                button { 
                    style: "background: #1890ff; color: white; border: none; padding: 8px 16px; border-radius: 4px;",
                    "自定义按钮1" 
                }
                button { 
                    style: "background: #52c41a; color: white; border: none; padding: 8px 16px; border-radius: 4px;",
                    "自定义按钮2" 
                }
                button { 
                    style: "background: #fa541c; color: white; border: none; padding: 8px 16px; border-radius: 4px;",
                    "自定义按钮3" 
                }
            }
        }
    }
}

/// 嵌套间距示例
pub fn nested_space_example() -> Element {
    rsx! {
        div {
            h3 { "嵌套间距" }
            Space {
                direction: SpaceDirection::Vertical,
                size: SpaceSize::Large,
                
                Space {
                    button { "第一行按钮1" }
                    button { "第一行按钮2" }
                }
                
                Space {
                    size: SpaceSize::Small,
                    span { "第二行文本1" }
                    span { "第二行文本2" }
                    span { "第二行文本3" }
                }
                
                Space {
                    align: SpaceAlign::Center,
                    input { r#type: "text", placeholder: "输入框" }
                    button { "提交" }
                }
            }
        }
    }
}

/// 所有示例的集合
pub fn all_space_examples() -> Element {
    rsx! {
        div {
            style: "padding: 24px;",
            
            h1 { "Space 组件示例" }
            
            section {
                h2 { "基础用法" }
                {basic_space_example()}
            }
            
            section {
                h2 { "垂直间距" }
                {vertical_space_example()}
            }
            
            section {
                h2 { "不同尺寸" }
                {space_sizes_example()}
            }
            
            section {
                h2 { "对齐方式" }
                {space_align_example()}
            }
            
            section {
                h2 { "自动换行" }
                {space_wrap_example()}
            }
            
            section {
                h2 { "分隔符" }
                {space_split_example()}
            }
            
            section {
                h2 { "复杂布局" }
                {complex_space_example()}
            }
            
            section {
                h2 { "响应式间距" }
                {responsive_space_example()}
            }
            
            section {
                h2 { "自定义样式" }
                {custom_space_example()}
            }
            
            section {
                h2 { "嵌套间距" }
                {nested_space_example()}
            }
        }
    }
}
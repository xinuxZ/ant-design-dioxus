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

/// 调试模式示例
pub fn debug_mode_example() -> Element {
    let debug_config = SpaceDebugConfig {
        enabled: true,
        show_boundaries: true,
        show_size_info: true,
        debug_color: Some("#ff4d4f".to_string()),
    };

    rsx! {
        div {
            h3 { "调试模式" }
            Space {
                debug_config: debug_config,
                size: SpaceSize::Large,
                button { "按钮1" }
                button { "按钮2" }
                button { "按钮3" }
            }
        }
    }
}

/// 动画效果示例
pub fn animation_example() -> Element {
    let animation_config = SpaceAnimationConfig {
        enabled: true,
        duration: Some("500ms".to_string()),
        easing: Some("ease-in-out".to_string()),
        respect_reduced_motion: true,
    };

    rsx! {
        div {
            h3 { "动画效果" }
            Space {
                animation_config: animation_config,
                size: SpaceSize::Middle,
                button { "按钮1" }
                button { "按钮2" }
                button { "按钮3" }
            }
        }
    }
}

/// 性能优化示例
pub fn performance_example() -> Element {
    let performance_config = SpacePerformanceConfig {
        virtual_scroll: true,
        lazy_loading: true,
        memo_children: true,
    };

    rsx! {
        div {
            h3 { "性能优化" }
            Space {
                performance_config: performance_config,
                direction: SpaceDirection::Vertical,
                // 大量子元素示例
                for i in 0..20 {
                    div {
                        key: "{i}",
                        "项目 {i + 1}"
                    }
                }
            }
        }
    }
}

/// 国际化示例
pub fn i18n_example() -> Element {
    let rtl_config = SpaceI18nConfig {
        rtl: true,
        auto_direction: false,
        locale: Some("ar".to_string()),
    };

    rsx! {
        div {
            h3 { "RTL 布局" }
            Space {
                i18n_config: rtl_config,
                button { "زر 1" }
                button { "زر 2" }
                button { "زر 3" }
            }
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

/// Space.Compact 紧凑模式示例
pub fn space_compact_example() -> Element {
    rsx! {
        div {
            h3 { "紧凑模式" }
            
            // 基础紧凑模式
            div {
                h4 { "基础紧凑模式" }
                SpaceCompact {
                    Button { "按钮1" }
                    Button { "按钮2" }
                    Button { "按钮3" }
                }
            }
            
            // 不同尺寸的紧凑模式
            div {
                h4 { "不同尺寸" }
                SpaceCompact {
                    size: SpaceCompactSize::Small,
                    Button { "小间距" }
                    Button { "小间距" }
                }
                br {}
                SpaceCompact {
                    size: SpaceCompactSize::Middle,
                    Button { "中间距" }
                    Button { "中间距" }
                }
                br {}
                SpaceCompact {
                    size: SpaceCompactSize::Large,
                    Button { "大间距" }
                    Button { "大间距" }
                }
            }
            
            // 垂直紧凑模式
            div {
                h4 { "垂直紧凑模式" }
                SpaceCompact {
                    direction: SpaceDirection::Vertical,
                    Button { "按钮1" }
                    Button { "按钮2" }
                    Button { "按钮3" }
                }
            }
            
            // 块级紧凑模式
            div {
                h4 { "块级紧凑模式" }
                SpaceCompact {
                    block: true,
                    Button { "全宽按钮1" }
                    Button { "全宽按钮2" }
                }
            }
        }
    }
}

/// 数组尺寸示例
pub fn array_size_example() -> Element {
    rsx! {
        div {
            h3 { "数组尺寸" }
            
            // 不同水平和垂直间距
            div {
                h4 { "不同水平和垂直间距" }
                Space {
                    size: SpaceSize::Array(vec![SpaceSize::Large, SpaceSize::Small]),
                    wrap: true,
                    Button { "按钮1" }
                    Button { "按钮2" }
                    Button { "按钮3" }
                    Button { "按钮4" }
                    Button { "按钮5" }
                    Button { "按钮6" }
                }
            }
            
            // 自定义数组尺寸
            div {
                h4 { "自定义数组尺寸" }
                Space {
                    size: SpaceSize::Array(vec![SpaceSize::Custom(32), SpaceSize::Custom(8)]),
                    wrap: true,
                    Button { "大水平间距" }
                    Button { "小垂直间距" }
                    Button { "按钮3" }
                    Button { "按钮4" }
                }
            }
            
            // 混合尺寸数组
            div {
                h4 { "混合尺寸数组" }
                Space {
                    size: SpaceSize::Array(vec![SpaceSize::Large, SpaceSize::Middle]),
                    direction: SpaceDirection::Vertical,
                    Button { "垂直布局" }
                    Button { "大水平间距" }
                    Button { "中等垂直间距" }
                }
            }
        }
    }
}

/// 增强分割元素示例
pub fn enhanced_split_example() -> Element {
    rsx! {
        div {
            h3 { "增强分割元素" }
            
            // 自定义分割元素样式
            div {
                h4 { "自定义分割元素样式" }
                Space {
                    split_config: SpaceSplit {
                        element: rsx! { span { "|" } },
                        style: Some("color: red; font-weight: bold;".to_string()),
                        class: Some("custom-divider".to_string()),
                        visible: true,
                    },
                    Button { "按钮1" }
                    Button { "按钮2" }
                    Button { "按钮3" }
                }
            }
            
            // 可控制显示的分割元素
            div {
                h4 { "可控制显示的分割元素" }
                Space {
                    split_config: SpaceSplit {
                        element: rsx! { span { "•" } },
                        style: Some("color: blue;".to_string()),
                        class: None,
                        visible: true,
                    },
                    Button { "按钮1" }
                    Button { "按钮2" }
                    Button { "按钮3" }
                }
            }
            
            // 复杂分割元素
            div {
                h4 { "复杂分割元素" }
                Space {
                    split_config: SpaceSplit {
                        element: rsx! {
                            div {
                                style: "width: 2px; height: 20px; background: linear-gradient(to bottom, #ff0000, #0000ff);",
                            }
                        },
                        style: None,
                        class: Some("gradient-divider".to_string()),
                        visible: true,
                    },
                    Button { "按钮1" }
                    Button { "按钮2" }
                    Button { "按钮3" }
                }
            }
        }
    }
}

/// 语义化DOM示例
pub fn semantic_dom_example() -> Element {
    rsx! {
        div {
            h3 { "语义化DOM" }
            
            // 语义化类名
            div {
                h4 { "语义化类名" }
                Space {
                    class_names: {
                        let mut map = std::collections::HashMap::new();
                        map.insert("container".to_string(), "semantic-space-container".to_string());
                        map.insert("wrapper".to_string(), "semantic-space-wrapper".to_string());
                        Some(map)
                    },
                    Button { "按钮1" }
                    Button { "按钮2" }
                    Button { "按钮3" }
                }
            }
            
            // 语义化样式
            div {
                h4 { "语义化样式" }
                Space {
                    styles: {
                        let mut map = std::collections::HashMap::new();
                        map.insert("container".to_string(), "border: 2px dashed #ccc; padding: 10px;".to_string());
                        map.insert("item".to_string(), "background-color: #f0f0f0; border-radius: 4px;".to_string());
                        Some(map)
                    },
                    Button { "按钮1" }
                    Button { "按钮2" }
                    Button { "按钮3" }
                }
            }
            
            // 组合使用
            div {
                h4 { "组合使用" }
                Space {
                    class_names: {
                        let mut map = std::collections::HashMap::new();
                        map.insert("container".to_string(), "semantic-container".to_string());
                        Some(map)
                    },
                    styles: {
                        let mut map = std::collections::HashMap::new();
                        map.insert("container".to_string(), "background: linear-gradient(45deg, #f0f0f0, #e0e0e0);".to_string());
                        Some(map)
                    },
                    split_config: SpaceSplit {
                        element: rsx! { span { "→" } },
                        style: Some("color: #666;".to_string()),
                        class: Some("arrow-divider".to_string()),
                        visible: true,
                    },
                    Button { "开始" }
                    Button { "处理" }
                    Button { "完成" }
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
            
            section {
                h2 { "紧凑模式" }
                {space_compact_example()}
            }
            
            section {
                h2 { "数组尺寸" }
                {array_size_example()}
            }
            
            section {
                h2 { "增强分割元素" }
                {enhanced_split_example()}
            }
            
            section {
                h2 { "语义化DOM" }
                {semantic_dom_example()}
            }
        }
    }
}
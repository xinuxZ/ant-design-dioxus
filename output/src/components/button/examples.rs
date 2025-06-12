//! Button 组件使用示例
//!
//! 展示 Button 和 ButtonGroup 组件的各种使用方法

#[cfg(feature = "examples")]
mod examples {
    use dioxus::prelude::*;
    use super::*;

    /// 基础按钮示例
    pub fn basic_button_example() -> Element {
        rsx! {
            div {
                style: "padding: 20px; display: flex; gap: 10px; flex-wrap: wrap;",
                
                h3 { "基础按钮" }
                div {
                    style: "display: flex; gap: 10px; margin-bottom: 20px;",
                    Button {
                        button_type: ButtonType::Primary,
                        "主要按钮"
                    }
                    Button {
                        button_type: ButtonType::Default,
                        "默认按钮"
                    }
                    Button {
                        button_type: ButtonType::Dashed,
                        "虚线按钮"
                    }
                    Button {
                        button_type: ButtonType::Text,
                        "文本按钮"
                    }
                    Button {
                        button_type: ButtonType::Link,
                        "链接按钮"
                    }
                }
            }
        }
    }

    /// 按钮尺寸示例
    pub fn button_sizes_example() -> Element {
        rsx! {
            div {
                style: "padding: 20px;",
                
                h3 { "按钮尺寸" }
                div {
                    style: "display: flex; gap: 10px; align-items: center; margin-bottom: 20px;",
                    Button {
                        button_type: ButtonType::Primary,
                        size: ButtonSize::Large,
                        "大按钮"
                    }
                    Button {
                        button_type: ButtonType::Primary,
                        size: ButtonSize::Middle,
                        "中按钮"
                    }
                    Button {
                        button_type: ButtonType::Primary,
                        size: ButtonSize::Small,
                        "小按钮"
                    }
                }
            }
        }
    }

    /// 按钮形状示例
    pub fn button_shapes_example() -> Element {
        rsx! {
            div {
                style: "padding: 20px;",
                
                h3 { "按钮形状" }
                div {
                    style: "display: flex; gap: 10px; align-items: center; margin-bottom: 20px;",
                    Button {
                        button_type: ButtonType::Primary,
                        shape: ButtonShape::Default,
                        "默认形状"
                    }
                    Button {
                        button_type: ButtonType::Primary,
                        shape: ButtonShape::Round,
                        "圆角按钮"
                    }
                    Button {
                        button_type: ButtonType::Primary,
                        shape: ButtonShape::Circle,
                        "圆"
                    }
                }
            }
        }
    }

    /// 危险按钮示例
    pub fn danger_buttons_example() -> Element {
        rsx! {
            div {
                style: "padding: 20px;",
                
                h3 { "危险按钮" }
                div {
                    style: "display: flex; gap: 10px; margin-bottom: 20px;",
                    Button {
                        button_type: ButtonType::Primary,
                        danger: true,
                        "危险主按钮"
                    }
                    Button {
                        button_type: ButtonType::Default,
                        danger: true,
                        "危险默认按钮"
                    }
                    Button {
                        button_type: ButtonType::Dashed,
                        danger: true,
                        "危险虚线按钮"
                    }
                    Button {
                        button_type: ButtonType::Text,
                        danger: true,
                        "危险文本按钮"
                    }
                }
            }
        }
    }

    /// 幽灵按钮示例
    pub fn ghost_buttons_example() -> Element {
        rsx! {
            div {
                style: "padding: 20px; background: #001529;",
                
                h3 { 
                    style: "color: white;",
                    "幽灵按钮（深色背景）" 
                }
                div {
                    style: "display: flex; gap: 10px; margin-bottom: 20px;",
                    Button {
                        button_type: ButtonType::Primary,
                        ghost: true,
                        "主要幽灵按钮"
                    }
                    Button {
                        button_type: ButtonType::Default,
                        ghost: true,
                        "默认幽灵按钮"
                    }
                    Button {
                        button_type: ButtonType::Dashed,
                        ghost: true,
                        "虚线幽灵按钮"
                    }
                }
            }
        }
    }

    /// 加载状态按钮示例
    pub fn loading_buttons_example() -> Element {
        rsx! {
            div {
                style: "padding: 20px;",
                
                h3 { "加载状态按钮" }
                div {
                    style: "display: flex; gap: 10px; margin-bottom: 20px;",
                    Button {
                        button_type: ButtonType::Primary,
                        loading: true,
                        "加载中"
                    }
                    Button {
                        button_type: ButtonType::Default,
                        loading: true,
                        "加载中"
                    }
                    Button {
                        button_type: ButtonType::Primary,
                        loading: true,
                        size: ButtonSize::Small,
                        "加载中"
                    }
                }
            }
        }
    }

    /// 禁用状态按钮示例
    pub fn disabled_buttons_example() -> Element {
        rsx! {
            div {
                style: "padding: 20px;",
                
                h3 { "禁用状态按钮" }
                div {
                    style: "display: flex; gap: 10px; margin-bottom: 20px;",
                    Button {
                        button_type: ButtonType::Primary,
                        disabled: true,
                        "禁用主按钮"
                    }
                    Button {
                        button_type: ButtonType::Default,
                        disabled: true,
                        "禁用默认按钮"
                    }
                    Button {
                        button_type: ButtonType::Dashed,
                        disabled: true,
                        "禁用虚线按钮"
                    }
                    Button {
                        button_type: ButtonType::Text,
                        disabled: true,
                        "禁用文本按钮"
                    }
                }
            }
        }
    }

    /// 块级按钮示例
    pub fn block_buttons_example() -> Element {
        rsx! {
            div {
                style: "padding: 20px; width: 400px;",
                
                h3 { "块级按钮" }
                div {
                    style: "display: flex; flex-direction: column; gap: 10px; margin-bottom: 20px;",
                    Button {
                        button_type: ButtonType::Primary,
                        block: true,
                        "主要块级按钮"
                    }
                    Button {
                        button_type: ButtonType::Default,
                        block: true,
                        "默认块级按钮"
                    }
                    Button {
                        button_type: ButtonType::Dashed,
                        block: true,
                        "虚线块级按钮"
                    }
                }
            }
        }
    }

    /// 按钮组示例
    pub fn button_group_example() -> Element {
        rsx! {
            div {
                style: "padding: 20px;",
                
                h3 { "按钮组" }
                div {
                    style: "display: flex; flex-direction: column; gap: 20px;",
                    
                    // 基础按钮组
                    div {
                        h4 { "基础按钮组" }
                        ButtonGroup {
                            Button { "左" }
                            Button { "中" }
                            Button { "右" }
                        }
                    }
                    
                    // 大尺寸按钮组
                    div {
                        h4 { "大尺寸按钮组" }
                        ButtonGroup {
                            size: ButtonGroupSize::Large,
                            Button { "左" }
                            Button { "中" }
                            Button { "右" }
                        }
                    }
                    
                    // 小尺寸按钮组
                    div {
                        h4 { "小尺寸按钮组" }
                        ButtonGroup {
                            size: ButtonGroupSize::Small,
                            Button { "左" }
                            Button { "中" }
                            Button { "右" }
                        }
                    }
                    
                    // 禁用按钮组
                    div {
                        h4 { "禁用按钮组" }
                        ButtonGroup {
                            disabled: true,
                            Button { "左" }
                            Button { "中" }
                            Button { "右" }
                        }
                    }
                }
            }
        }
    }

    /// 交互式按钮示例
    pub fn interactive_buttons_example() -> Element {
        let mut count = use_signal(|| 0);
        let mut loading = use_signal(|| false);
        
        rsx! {
            div {
                style: "padding: 20px;",
                
                h3 { "交互式按钮" }
                div {
                    style: "display: flex; flex-direction: column; gap: 20px;",
                    
                    // 计数器
                    div {
                        h4 { "计数器: {count}" }
                        div {
                            style: "display: flex; gap: 10px;",
                            Button {
                                button_type: ButtonType::Primary,
                                onclick: move |_| count += 1,
                                "增加"
                            }
                            Button {
                                button_type: ButtonType::Default,
                                onclick: move |_| count -= 1,
                                "减少"
                            }
                            Button {
                                button_type: ButtonType::Dashed,
                                onclick: move |_| count.set(0),
                                "重置"
                            }
                        }
                    }
                    
                    // 异步加载
                    div {
                        h4 { "异步加载" }
                        Button {
                            button_type: ButtonType::Primary,
                            loading: loading(),
                            onclick: move |_| {
                                if !loading() {
                                    loading.set(true);
                                    // 模拟异步操作
                                    spawn(async move {
                                        gloo_timers::future::TimeoutFuture::new(2000).await;
                                        loading.set(false);
                                    });
                                }
                            },
                            if loading() { "加载中..." } else { "开始加载" }
                        }
                    }
                }
            }
        }
    }

    /// 完整示例应用
    pub fn complete_example() -> Element {
        rsx! {
            div {
                style: "font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;",
                
                h1 { "Button 组件示例" }
                
                {basic_button_example()}
                {button_sizes_example()}
                {button_shapes_example()}
                {danger_buttons_example()}
                {ghost_buttons_example()}
                {loading_buttons_example()}
                {disabled_buttons_example()}
                {block_buttons_example()}
                {button_group_example()}
                {interactive_buttons_example()}
            }
        }
    }
}

#[cfg(feature = "examples")]
pub use examples::*;
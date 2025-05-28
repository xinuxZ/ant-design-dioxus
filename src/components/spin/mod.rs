//! Spin 加载中组件
//!
//! 用于页面和区块的加载中状态。
//!
//! ## 何时使用
//!
//! 页面局部处于等待异步数据或正在渲染过程时，合适的加载动效会有效缓解用户的焦虑。
//!
//! ## 代码演示
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::Spin;
//!
//! #[component]
//! fn App() -> Element {
//!     rsx! {
//!         Spin {
//!             tip: "加载中...",
//!             div {
//!                 style: "height: 200px; background: #f0f0f0; padding: 20px;",
//!                 "内容区域"
//!             }
//!         }
//!     }
//! }
//! ```

use dioxus::prelude::*;

const SPIN_STYLES: &str = include_str!("./style.css");

/// Spin 组件尺寸
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SpinSize {
    /// 小尺寸
    Small,
    /// 默认尺寸
    Default,
    /// 大尺寸
    Large,
}

impl Default for SpinSize {
    fn default() -> Self {
        Self::Default
    }
}

impl SpinSize {
    /// 获取尺寸对应的CSS类名
    pub fn to_class(&self) -> &'static str {
        match self {
            SpinSize::Small => "ant-spin-sm",
            SpinSize::Default => "",
            SpinSize::Large => "ant-spin-lg",
        }
    }

    /// 获取指示器尺寸
    pub fn indicator_size(&self) -> u32 {
        match self {
            SpinSize::Small => 14,
            SpinSize::Default => 20,
            SpinSize::Large => 32,
        }
    }
}

/// Spin 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct SpinProps {
    /// 延迟显示加载效果的时间（防止闪烁），单位：毫秒
    #[props(default = 0)]
    pub delay: u32,

    /// 自定义指示符
    #[props(default)]
    pub indicator: Option<Element>,

    /// 组件大小
    #[props(default)]
    pub size: SpinSize,

    /// 是否为加载中状态
    #[props(default = true)]
    pub spinning: bool,

    /// 当作为包裹元素时，可以自定义描述文案
    #[props(default)]
    pub tip: Option<String>,

    /// 包裹的内容
    #[props(default)]
    pub children: Option<Element>,

    /// 自定义样式类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,
}

/// 默认的加载指示器组件
///
/// 渲染一个旋转的圆形指示器
#[component]
fn DefaultIndicator(size: SpinSize) -> Element {
    let indicator_size = size.indicator_size();

    rsx! {
        span {
            class: "ant-spin-dot ant-spin-dot-spin",
            style: format!("width: {}px; height: {}px;", indicator_size, indicator_size),
            i { class: "ant-spin-dot-item" }
            i { class: "ant-spin-dot-item" }
            i { class: "ant-spin-dot-item" }
            i { class: "ant-spin-dot-item" }
        }
    }
}

/// Spin 加载中组件
///
/// 用于页面和区块的加载中状态，提供了一个简单的加载动画
#[component]
pub fn Spin(props: SpinProps) -> Element {
    let mut is_visible = use_signal(|| props.spinning);

    // 处理延迟显示逻辑
    use_effect(move || {
        if props.delay > 0 && props.spinning {
            is_visible.set(false);
            let delay = props.delay;
            spawn(async move {
                gloo_timers::future::TimeoutFuture::new(delay).await;
                is_visible.set(true);
            });
        } else {
            is_visible.set(props.spinning);
        }
    });

    let spin_class = {
        let mut classes = vec!["ant-spin"];

        if is_visible() {
            classes.push("ant-spin-spinning");
        }

        let size_class = props.size.to_class();
        if !size_class.is_empty() {
            classes.push(size_class);
        }

        if props.tip.is_some() {
            classes.push("ant-spin-show-text");
        }

        if let Some(ref class) = props.class {
            classes.push(class);
        }

        classes.join(" ")
    };

    // 如果有包裹内容，渲染为容器模式
    if let Some(children) = props.children {
        rsx! {
            style { {SPIN_STYLES} }

            div {
                class: "ant-spin-container",
                style: props.style,
                if is_visible() {
                    div {
                        class: "ant-spin-blur",
                        {children}
                    }
                } else {
                    {children}
                }
                if is_visible() {
                    div {
                        class: spin_class,
                        div {
                            class: "ant-spin-dot-holder",
                            if let Some(indicator) = props.indicator {
                                {indicator}
                            } else {
                                DefaultIndicator { size: props.size }
                            }
                        }
                        if let Some(ref tip) = props.tip {
                            div {
                        class: "ant-spin-text",
                        {tip.clone()}
                    }
                        }
                    }
                }
            }
        }
    } else {
        // 独立的加载指示器
        if is_visible() {
            rsx! {
                style { {SPIN_STYLES} }

                div {
                    class: spin_class,
                    style: props.style,
                    div {
                        class: "ant-spin-dot-holder",
                        if let Some(indicator) = props.indicator {
                            {indicator}
                        } else {
                            DefaultIndicator { size: props.size }
                        }
                    }
                    if let Some(ref tip) = props.tip {
                        div {
                            class: "ant-spin-text",
                            {tip.clone()}
                        }
                    }
                }
            }
        } else {
            rsx! { div {} }
        }
    }
}

// 组件已通过#[component]宏自动导出
// 无需重新导出

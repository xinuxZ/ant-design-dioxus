//! Carousel 走马灯组件
//!
//! 旋转木马，一组轮播的区域。
//!
//! ## 何时使用
//!
//! - 当有一组平级的内容。
//! - 当内容空间不足时，可以用走马灯的形式进行收纳，进行轮播展现。
//! - 常用于一组图片或卡片轮播。

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

/// Carousel 走马灯组件属性
#[derive(Props, Clone, PartialEq)]
pub struct CarouselProps {
    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 是否自动切换
    #[props(default = true)]
    pub autoplay: bool,

    /// 自动切换的时间间隔，单位为毫秒
    #[props(default = 3000)]
    pub autoplay_speed: u32,

    /// 指示器位置
    #[props(default = DotPosition::Bottom)]
    pub dot_position: DotPosition,

    /// 是否显示指示器
    #[props(default = true)]
    pub dots: bool,

    /// 切换效果
    #[props(default = Effect::ScrollX)]
    pub effect: Effect,

    /// 是否显示左右切换箭头
    #[props(default = false)]
    pub arrows: bool,

    /// 是否支持手势滑动切换
    #[props(default = true)]
    pub swipe_to_slide: bool,

    /// 切换面板的回调
    #[props(default)]
    pub on_change: Option<EventHandler<usize>>,

    /// 子元素
    pub children: Element,
}

/// 指示器位置
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum DotPosition {
    Top,
    Bottom,
    Left,
    Right,
}

impl Default for DotPosition {
    fn default() -> Self {
        Self::Bottom
    }
}

/// 切换效果
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum Effect {
    ScrollX,
    Fade,
}

impl Default for Effect {
    fn default() -> Self {
        Self::ScrollX
    }
}

/// Carousel 走马灯组件
#[component]
pub fn Carousel(props: CarouselProps) -> Element {
    let mut current_index = use_signal(|| 0usize);
    let mut is_playing = use_signal(|| props.autoplay);

    // 获取子元素数量
    let children_count = props.children.as_ref().map_or(0, |_children| {
        // In Dioxus 0.6, Element is an opaque type
        // We'll assume single element for now
        // For multiple children, they would be wrapped in a fragment automatically
        1
    });

    // 自动播放逻辑
    use_effect(move || {
        if props.autoplay && is_playing() {
            let interval = gloo_timers::callback::Interval::new(props.autoplay_speed, move || {
                let new_index = (current_index() + 1) % children_count;
                current_index.set(new_index);
                if let Some(on_change) = &props.on_change {
                    on_change.call(new_index);
                }
            });
            interval.forget();
        }
    });

    // 切换到指定索引
    let mut go_to = {
        let mut current_index = current_index.clone();
        let on_change = props.on_change.clone();
        move |index: usize| {
            if index < children_count {
                current_index.set(index);
                if let Some(on_change) = &on_change {
                    on_change.call(index);
                }
            }
        }
    };

    // 上一张
    let prev = move |_| {
        let new_index = if current_index() == 0 {
            children_count - 1
        } else {
            current_index() - 1
        };
        go_to(new_index);
    };

    // 下一张
    let next = move |_| {
        let new_index = (current_index() + 1) % children_count;
        go_to(new_index);
    };

    // 鼠标进入暂停自动播放
    let on_mouse_enter = move |_| {
        if props.autoplay {
            is_playing.set(false);
        }
    };

    // 鼠标离开恢复自动播放
    let on_mouse_leave = move |_| {
        if props.autoplay {
            is_playing.set(true);
        }
    };

    let class_name = format!(
        "ant-carousel ant-carousel-{} {}",
        match props.dot_position {
            DotPosition::Top => "dots-top",
            DotPosition::Bottom => "dots-bottom",
            DotPosition::Left => "dots-left",
            DotPosition::Right => "dots-right",
        },
        props.class
    );

    rsx! {
        div {
            class: "{class_name}",
            style: "{props.style}",
            onmouseenter: on_mouse_enter,
            onmouseleave: on_mouse_leave,

            div {
                class: "ant-carousel-inner",
                style: format!(
                    "transform: translateX(-{}%); transition: transform 0.5s ease;",
                    current_index() * 100
                ),

                {props.children}
            }

            // 左右箭头
            if props.arrows {
                button {
                    class: "ant-carousel-arrow ant-carousel-arrow-left",
                    onclick: prev,
                    "‹"
                }
                button {
                    class: "ant-carousel-arrow ant-carousel-arrow-right",
                    onclick: next,
                    "›"
                }
            }

            // 指示器
            if props.dots && children_count > 1 {
                ul {
                    class: "ant-carousel-dots",

                    for i in 0..children_count {
                        li {
                            key: "{i}",
                            class: if i == current_index() { "ant-carousel-dot ant-carousel-dot-active" } else { "ant-carousel-dot" },
                            onclick: {
                                let mut go_to = go_to.clone();
                                move |_| go_to(i)
                            },

                            button {
                                class: "ant-carousel-dot-button"
                            }
                        }
                    }
                }
            }
        }

        style { {include_str!("./style.css")} }
    }
}

/// CarouselItem 走马灯项组件属性
#[derive(Props, Clone, PartialEq)]
pub struct CarouselItemProps {
    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,

    /// 子元素
    pub children: Element,
}

/// CarouselItem 走马灯项组件
#[component]
pub fn CarouselItem(props: CarouselItemProps) -> Element {
    let class_name = format!("ant-carousel-item {}", props.class);

    rsx! {
        div {
            class: "{class_name}",
            style: "{props.style}",

            {props.children}
        }
    }
}
